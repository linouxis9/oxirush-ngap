#!/usr/bin/env python3
"""Compare Wireshark's reading of NGAP or S1AP PDUs with this crate's, field by field.

Inputs:
  --protocol ngap or s1ap
  --corpus   rows of ..., octets in hexadecimal, by tabs
  --crate    one line of JSON a row: what the crate read (tests/wireshark.rs): its tree and what
             check() found
  --mapping  mapping.tsv: the names that differ, the fields compared to nothing and why
  --known    known.tsv: the differences that were triaged, which do not fail the run

tshark dissects each PDU as an exported PDU of the dissector of the protocol, each from an
address of its own, and writes PDML. Both readings are made into lists of named values, in the order of the PDU: a field
of Wireshark by its name without the protocol, a member of the crate's tree by its name,
both without case and punctuation; an entry of a SEQUENCE OF that is one value has the name
of the list on both sides. Each name that both have is compared value by value:
a number as a number, an enumeration by its name, an octet string by its octets, a PLMN
identity by its MCC and MNC, a transport layer address by its IP addresses, digits by their
TBCD octets, a bit string by its bits, a SEQUENCE OF by its number of entries. The message,
and the identifiers of its IEs in their order, are compared too. A field that only one side
has is listed in coverage.tsv, compared to nothing with a reason from the mapping or unlisted.

What one side does not read is a difference of its own: an IE or a message that the crate
keeps as octets (with what Wireshark notes in it), an IE or an extension that Wireshark does
not know or leaves undissected, what the crate reads after Wireshark threw in an IE or in the
message, and a fault of the encoding that only check() finds. An error of Wireshark says
where it is: the field that shows it and the field before it, which names the dissector.
What check() finds of the tables of the specification (criticalities, presence, order),
which a dissector does not check, is counted in the summary.

Each PDU is then one of: agree, both-refuse, ws-malformed (the crate decodes it, Wireshark
finds it malformed or reports an expert error), crate-refuses, differs.

Outputs in --output: report.json, summary.txt, coverage.tsv and the PDML. The run fails
when a difference is not in --known, or a tshark other than the pinned one is used without
--any-tshark.
"""

import argparse
import collections
import ipaddress
import json
import re
import subprocess
import sys
import threading
import xml.etree.ElementTree as ET
from pathlib import Path

PINNED = "TShark (Wireshark) 4.6.4"
EXPERT_ERROR = 0x00800000


class Node:
    """A field of PDML: its name, what it shows, its octets and its children."""

    __slots__ = ("name", "show", "showname", "value", "hide", "children")

    def __init__(self, element):
        self.name = element.get("name", "")
        self.show = element.get("show", "")
        self.showname = element.get("showname", "")
        self.value = element.get("value", "")
        self.hide = element.get("hide") == "yes"
        self.children = [Node(child) for child in element if child.tag in ("field", "proto")]

    def label(self):
        """What a coded value means, from `Name: meaning (code)`."""
        text = self.showname.split(": ", 1)[1] if ": " in self.showname else self.showname
        return re.sub(r"\s*\((?:0x)?[0-9a-fA-F]+\)$", "", text).strip()


def norm(name):
    return re.sub(r"[^a-z0-9]", "", name.lower())


def kebab(text):
    return re.sub(r"[^a-z0-9]+", "-", text.lower()).strip("-")


class Mapping:
    """mapping.tsv rows, by tabs: kind, wireshark, crate, note.

      name   a field of Wireshark -> the member of the crate it is (their names differ)
      ie     the name of an IE or an extension in Wireshark -> its name in the crate (another version)
      none   a field compared to nothing (`*` ends a prefix), and why
      value  a coded value: `field=label` -> the crate's name of it
    """

    def __init__(self, path):
        self.names, self.none, self.prefixes, self.values, self.ies = {}, {}, [], {}, {}
        for number, line in enumerate(path.read_text().splitlines(), 1):
            if not line.strip() or line.startswith("#"):
                continue
            kind, wireshark, crate, note = (line.split("\t") + [""] * 4)[:4]
            if kind == "name":
                self.names[wireshark] = norm(crate)
            elif kind == "ie":
                self.ies[norm(wireshark)] = norm(crate)
            elif kind == "none":
                if wireshark.endswith("*"):
                    self.prefixes.append((wireshark[:-1], crate or note))
                else:
                    self.none[wireshark] = crate or note
            elif kind == "value":
                field, _, label = wireshark.partition("=")
                self.values[(field, kebab(label))] = crate
            else:
                raise ValueError(f"{path}:{number}: unknown kind {kind!r}")

    def why_none(self, field):
        if field in self.none:
            return self.none[field]
        return next((why for prefix, why in self.prefixes if field.startswith(prefix)), None)


def feed(pipe, octets):
    with pipe:
        pipe.write(octets)


PPID = {"ngap": 60, "s1ap": 18}
PORT = {"ngap": 38412, "s1ap": 36412}


def capture(protocol, rows):
    """A capture of exported PDUs (link type 252) for the dissector of the protocol, each from
    an address and port of its own: Wireshark keeps the state of an association (the type
    of a handover, which says how to read a container), which a PDU must not take from the
    one before it. A PDU of any length fits, where an SCTP chunk holds 65519 octets."""
    import struct

    def tag(number, value):
        return struct.pack(">HH", number, len(value)) + value + b"\0" * (-len(value) % 4)

    out = [struct.pack("<IHHiIII", 0xA1B2C3D4, 2, 4, 0, 0, 0x7FFFFFFF, 252)]
    for index, octets in enumerate(rows):
        source = bytes([10, (index >> 16) & 255, (index >> 8) & 255, index & 255])
        tags = (tag(12, protocol.encode()) + tag(20, source) + tag(21, bytes([10, 255, 255, 1]))
                + tag(24, struct.pack(">I", 3)) + tag(25, struct.pack(">I", 1024 + index % 60000))
                + tag(26, struct.pack(">I", PORT[protocol])) + struct.pack(">HH", 0, 0))
        packet = tags + octets
        out.append(struct.pack("<IIII", index, 0, len(packet), len(packet)) + packet)
    return b"".join(out)


def dissect(protocol, rows, output):
    """The PDML packet of each row, in order."""
    pcap = output / f"{protocol}.pcap"
    pcap.write_bytes(capture(protocol, rows))
    pdml = output / f"{protocol}.pdml"
    # The capture goes to tshark through a pipe, and its PDML comes back through another:
    # tshark may refuse a path, or a file as its standard input or output, where the file
    # is readable and writable.
    process = subprocess.Popen(["tshark", "-n", "-r", "-", "-T", "pdml"],
                               stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    feeder = threading.Thread(target=feed, args=(process.stdin, pcap.read_bytes()))
    feeder.start()
    errors = []
    reader = threading.Thread(target=lambda: errors.append(process.stderr.read()))
    reader.start()
    with pdml.open("wb") as sink:
        while chunk := process.stdout.read(1 << 20):
            sink.write(chunk)
    feeder.join()
    reader.join()
    if process.wait() != 0:
        raise RuntimeError(f"tshark failed ({process.returncode}): {errors[0].decode().strip()}")
    packets = []
    for _, element in ET.iterparse(pdml, events=("end",)):
        if element.tag != "packet":
            continue
        proto = None
        malformed = stopped = False
        experts = []
        previous = None
        for child in element:
            if child.get("name") == protocol:
                proto = Node(child)
            elif child.get("name") == "_ws.malformed":
                malformed = True
                # right after the tree of the protocol: an exception that ended its dissection
                # (one after the tree of a NAS message did not)
                stopped = stopped or previous == protocol
            previous = child.get("name")
        for expert in element.iter("field"):
            if expert.get("name") == "_ws.expert":
                fields = {f.get("name"): f.get("show", "") for f in expert}
                experts.append({"message": fields.get("_ws.expert.message", ""),
                                "severity": int(fields.get("_ws.expert.severity", "0") or 0)})
        # each error with its place: the field before it, which names the dissector that
        # reported it (`lte-rrc.`, `nas-eps.`)
        packets.append({"proto": proto, "malformed": malformed, "stopped": stopped, "experts": experts,
                        "errors": error_places(element)})
        element.clear()
    if len(packets) != len(rows):
        raise AssertionError(f"{len(rows)} PDUs gave {len(packets)} packets")
    return packets


def error_places(packet):
    """The errors of a packet, each with where it is: an exception as the field that shows
    it and the last field before it (`Malformed Packet in X after Y`), with the note that
    Wireshark added right before it, or for one after the protocols of the packet, its
    protocol (`Malformed Packet: NGAP after Y`); an expert error as its message and the last
    field before it (`message [after Y]`)."""
    places = []
    last = ["", ""]  # the last field, and a note after it

    def walk(element):
        for child in element:
            name = child.get("name", "")
            if name == "_ws.malformed":
                where = (child.get("showname", "").strip("[]") if element is packet
                         else f"Malformed Packet in {element.get('name', '')}")
                places.append(f"{where} after {last[0]}" + (f" ({last[1]})" if last[1] else ""))
            elif name == "_ws.expert":
                fields = {f.get("name"): f.get("show", "") for f in child}
                note = fields.get("_ws.expert.message", "")
                if note == "Malformed Packet (Exception occurred)":
                    continue
                if int(fields.get("_ws.expert.severity", "0") or 0) >= EXPERT_ERROR:
                    places.append(f"{note} [after {last[0]}]")
                last[1] = note
            elif child.tag in ("field", "proto"):
                if name and not name.startswith(("_ws.", "per.")):
                    last[0], last[1] = name, ""
                walk(child)
    walk(packet)
    return places


# ---------------------------------------------------------------------------
# The two readings as named values

STRUCTURE = re.compile(r"_element$|^(protocolIEs|protocolExtensions|iE_Extensions|ProtocolIE_Field|"
                       r"ProtocolExtensionField|ie_field_value|extensionValue|id|ext_id|criticality|"
                       r"NGAP_PDU|S1AP_PDU|initiatingMessagevalue|successfulOutcome_value|"
                       r"successfulOutcomevalue|unsuccessfulOutcome_value|unsuccessfulOutcomevalue|value)$")
META = {"criticality", "procedure_code", "direction"}


class Wireshark:
    """The fields of the protocol in one IE that carry a value, in order, keyed by name.

    The value of an IE, and that of an extension, has the name of the IE (from its
    identifier) where Wireshark names it by its type. A field with a dot in its name past
    the protocol, as `ngap.NrencryptionAlgorithms.nea1`, is a part of a value that Wireshark
    shows, compared as that value. An extension whose identifier Wireshark does not know
    is kept in `undissected` with its identifier, and an IE or an extension whose
    identifier it knows and whose value it shows empty, in `empty` with its name.
    """

    def __init__(self, protocol, mapping, coverage, node, name):
        self.protocol, self.mapping, self.coverage = protocol, mapping, coverage
        self.prefix = protocol + "."
        self.values = []
        self.undissected = []
        self.empty = set()
        self.extensions = set()
        self.walk(node, None, name)

    def label(self, node):
        """The name of the IE or extension whose identifier is a child of node."""
        ident = next((c for c in node.children if c.name in (self.prefix + "id", self.prefix + "ext_id")), None)
        label = norm(ident.label().removeprefix("id-")) if ident is not None else ""
        return self.mapping.ies.get(label, label)

    def scalar(self, item):
        """Whether an entry of a SEQUENCE OF is one value, which Wireshark names by its type."""
        fields = [c for c in item.children if c.name.startswith(self.prefix) and not c.hide]
        # (the parts of a value that Wireshark shows, as the addresses of a transport layer
        # address, are not members of their own)
        return len(fields) == 1 and not any(c.name.startswith(self.prefix) and not self.mapping.why_none(c.name)
                                            and "." not in c.name[len(self.prefix):] for c in fields[0].children)

    def walk(self, node, inside, rename, entries=None):
        for index, child in enumerate(node.children):
            # The MCC and MNC that S1AP shows after a PLMN identity, as NGAP shows them in it
            if "plmn" in child.name.lower() and not any(c.name.startswith("e212.") for c in child.children):
                for following in node.children[index + 1:]:
                    if not following.name.startswith("e212."):
                        break
                    child.children.append(following)
            name = child.name
            if child.hide or name.startswith(("per.", "_ws.")) or name == "":
                # An entry of a SEQUENCE OF that is one value has the name of the list, as
                # in the crate's tree
                if name == "" and entries is not None and child.show.startswith("Item ") and self.scalar(child):
                    self.walk(child, inside, entries)
                else:
                    self.walk(child, inside, rename)
                continue
            if not name.startswith(self.prefix):
                why = self.mapping.why_none(name)
                self.coverage[(inside or "(pdu)", name, "none" if why else "inside a value")] += 1
                continue
            short = name[len(self.prefix):]
            if short in ("ie_field_value_element", "extensionValue_element") and not any(
                    n.name.startswith(self.prefix) and n.name != self.prefix + "extensionValue.data"
                    for n in iter_nodes(child) if n is not child):
                # the value of an IE or an extension that Wireshark shows empty, or as the
                # octets of an extension that it has no dissector for (`ngap.extensionValue.data`)
                label = self.label(node)
                if label and not label.startswith("unknown"):
                    self.empty.add((label, short == "extensionValue_element"))
                continue
            if short.endswith("_element") and child.showname.endswith(": NULL"):
                # a NULL, which Wireshark shows as a structure
                self.values.append((rename or self.mapping.names.get(name, norm(short.removesuffix("_element"))), child))
                continue
            if short in ("ProtocolIE_Field_element", "ProtocolIE_SingleContainer_element") and inside is not None:
                # an IE of a transfer or of a container: its value has the name of the IE
                label = self.label(child) or None
                self.walk(child, inside, label)
                continue
            if short == "ProtocolExtensionField_element":
                ident = next((c for c in child.children if c.name == self.prefix + "ext_id"), None)
                label = self.label(child)
                if not label or label.startswith("unknown"):
                    # an extension that Wireshark does not know: its octets alone
                    self.undissected.append(ident.show if ident is not None else "")
                    continue
                self.extensions.add(label)
                self.walk(child, inside, label)
                continue
            if "." in short or STRUCTURE.search(short):
                if "." in short:
                    self.coverage[(inside or "(pdu)", name, "a part of a value")] += 1
                # The name of an IE is that of its value; a SEQUENCE has no field of
                # its own, and its members keep their names
                keep = rename if short in ("ie_field_value_element", "extensionValue", "extensionValue_element",
                                           "ProtocolExtensionField_element", "ProtocolIE_Field_element",
                                           "value", "value_element") else None
                self.walk(child, inside, keep)
                continue
            key = rename or self.mapping.names.get(name, norm(short))
            self.values.append((key, child))
            self.walk(child, name, None, key)


def crate_values(name, value):
    """[(name, value, within, parent)] of an IE of the crate's tree, in order: each member, a
    list or an object as itself; `within` are the names of the members it is in, itself
    included, each with whether it is an extension; `parent` is the position of the closest
    member it is in."""
    found = []
    parents = [None]

    def walk(key, member, within):
        pushed = key is not None and key not in META
        if pushed:
            found.append((norm(key), member, within, parents[-1]))
            parents.append(len(found) - 1)
        inner_walk(member, key, within)
        if pushed:
            parents.pop()

    def inner_walk(member, key, within):
        if isinstance(member, dict):
            if set(member) <= {"length", "value"} and "value" in member:
                return  # a bit string
            for inner, held in member.items():
                if inner in ("protocolIEs", "iE-Extensions", "protocolExtensions"):
                    for entry in held if isinstance(held, list) else [held]:
                        if isinstance(entry, dict):
                            for ext, ext_value in entry.items():
                                if ext not in META:
                                    walk(ext, ext_value, within + ((norm(ext), inner != "protocolIEs"),))
                elif inner not in META:
                    walk(inner, held, within + ((norm(inner), False),))
        elif isinstance(member, list):
            for entry in member:
                # an entry that is one value has the name of the list; the members of one
                # that is a structure keep theirs
                one = (not isinstance(entry, (dict, list))
                       or isinstance(entry, dict) and set(entry) <= {"length", "value"} and "value" in entry)
                walk(key if one else None, entry, within)
    walk(name, value, ((norm(name), False),))
    return found


PLMN = re.compile(r"^(\d{3})-(\d{2,3})$")


def addresses(text):
    """The IP addresses of a transport layer address as the crate writes it, by commas; None
    for a text that is not one."""
    try:
        return [ipaddress.ip_address(part) for part in text.split(",")]
    except ValueError:
        return None


def tbcd(octets):
    """The digits of TBCD octets (TS 29.002 §17.7.8), up to the filler."""
    digits = ""
    for octet in bytes.fromhex(octets):
        for digit in (octet & 15, octet >> 4):
            if digit == 15:
                return digits
            digits += str(digit) if digit < 10 else "?"
    return digits


FIELD_TYPES = {}


def field_types():
    """The type of each field of Wireshark: FT_UINT32, FT_BYTES..."""
    if not FIELD_TYPES:
        listing = subprocess.run(["tshark", "-G", "fields"], capture_output=True, text=True).stdout
        for line in listing.splitlines():
            columns = line.split("\t")
            if len(columns) > 3 and columns[0] == "F":
                FIELD_TYPES[columns[2]] = columns[3]
    return FIELD_TYPES


def equal(protocol, mapping, node, value):
    """Whether Wireshark's node and the crate's value say the same; else what Wireshark says."""
    if isinstance(value, bool):
        return node.show in (("True", "1") if value else ("False", "0")), node.show
    kind = field_types().get(node.name, "")
    if isinstance(value, int) and re.match(r"FT_U?INT", kind):
        try:
            return int(node.show, 0 if node.show.startswith("0x") else 10) == value, node.show
        except ValueError:
            pass
    if isinstance(value, int):
        # an octet or bit string that the crate reads as a number: its octets, without
        # the padding of a bit string
        octets = node.value if re.fullmatch(r"[0-9a-fA-F]+", node.value or "") else node.show.replace(":", "")
        try:
            number = int(octets, 16)
        except ValueError:
            return False, node.show
        return any(number >> pad == value and number & ((1 << pad) - 1) == 0 for pad in range(8)), node.show
    if isinstance(value, list):
        try:
            return int(node.show, 0) == len(value), node.show
        except ValueError:
            return True, node.show  # a container that Wireshark shows without a count
    if isinstance(value, dict):
        if set(value) <= {"length", "value"} and "value" in value:
            # a bit string: its first `length` bits, which Wireshark shows padded at the end
            shown = node.show.replace(":", "")
            shown = shown if re.fullmatch(r"[0-9a-fA-F]+", shown) else node.value
            written = str(value["value"])
            bits = value.get("length", 4 * len(written))
            if bits == 0:
                return node.show in ("", "<MISSING>") or not node.value, node.show
            if not re.fullmatch(r"[0-9a-fA-F]+", shown) or 4 * len(shown) < bits or 4 * len(written) < bits:
                return False, node.show
            return int(shown, 16) >> (4 * len(shown) - bits) == int(written, 16) >> (4 * len(written) - bits), node.show
        return True, ""  # a structure: its members are compared
    if not isinstance(value, str):
        return value is None, node.show
    match = PLMN.match(value)
    if match:
        mcc = next((c.show for c in node.children if c.name.endswith(".mcc")), None)
        mnc = next((c.show for c in node.children if c.name.endswith(".mnc")), None)
        if mcc is not None:
            return f"{mcc}-{mnc}" == value, f"{mcc}-{mnc}"
    ips = addresses(value) if not re.fullmatch(r"[0-9a-fA-F]+", value) else None
    if ips is not None:
        shown = [c.show for c in iter_nodes(node) if c is not node and re.search(r"ipv[46]", c.name, re.I)]
        return ips == (addresses(",".join(shown)) if shown else []), ",".join(shown)
    if re.fullmatch(r"[0-9a-f]*", value) and node.value and re.fullmatch(r"[0-9a-fA-F]+", node.value):
        if node.value.lower() == value:
            return True, node.value
    if re.fullmatch(r"\d{5,}", value) and re.fullmatch(r"[0-9a-fA-F]+", node.value or "x"):
        # digits that the octets carry in TBCD (an IMSI)
        return tbcd(node.value) == value, tbcd(node.value)
    label = node.label()
    name = mapping.values.get((node.name, kebab(label)), kebab(label))
    if name == kebab(value) or node.show == value:
        return True, label
    return False, label or node.show


def compare_one(protocol, mapping, octets, packet, reading):
    coverage = collections.Counter()
    errors = packet["errors"]
    malformed = packet["malformed"] or bool(errors)
    result = {"octets": octets.hex(), "wireshark-errors": errors, "wireshark-malformed": packet["malformed"],
              "experts": [e["message"] for e in packet["experts"]]}
    if "error" in reading:
        result["crate-error"] = reading["error"]
        result["class"] = "both-refuse" if malformed else "crate-refuses"
        return result, coverage
    differences = []

    def differ(where, what, wireshark, crate):
        differences.append({"where": where, "what": what, "wireshark": wireshark, "crate": crate})

    tree = reading["tree"]
    proto = packet["proto"]
    result["check"] = reading.get("check", [])
    if proto is None:
        differ("", "Wireshark has no tree of the protocol", "", "")
    else:
        # the message, and the identifiers of its IEs in order
        message = next((n for n in iter_nodes(proto) if re.fullmatch(
            rf"{protocol}\.(initiatingMessage|successfulOutcome|unsuccessfulOutcome)_?value_element", n.name)), None)
        typed = message.children[0].name if message is not None and message.children else ""
        typed = typed[len(protocol) + 1:].removesuffix("_element") if typed.startswith(protocol + ".") else ""
        crate_message = tree.get("message", "")
        if typed and norm(typed) != norm(crate_message):
            differ("/message", "message", typed, crate_message)
        top = next((n for n in iter_nodes(proto) if n.name == f"{protocol}.protocolIEs"), None)
        ws_ids = []
        if top is not None:
            for item in top.children:
                ident = next((n for n in iter_nodes(item) if n.name == f"{protocol}.id"), None)
                if ident is not None:
                    label = norm(ident.label().removeprefix("id-"))
                    ws_ids.append(mapping.ies.get(label, label))
        crate_ids = [norm(next(iter(ie))) for ie in tree.get("ies") or [] if isinstance(ie, dict) and ie]
        if top is not None and "ies" in tree:
            if malformed and 0 < len(ws_ids) < len(crate_ids) and crate_ids[:len(ws_ids)] == ws_ids:
                # Wireshark threw before the end of the message
                differ("/ies", "what the crate reads after Wireshark threw", ws_ids[-1],
                       f"{len(crate_ids) - len(ws_ids)} IEs")
            elif len(ws_ids) != len(crate_ids):
                differ("/ies", "the IEs of the message", ws_ids, crate_ids)
            else:
                for ws_id, crate_id in zip(ws_ids, crate_ids):
                    if ws_id.startswith("unknown") and ws_id != crate_id:
                        differ(f"/ies/{crate_id}", "an IE that Wireshark does not know", ws_id, crate_id)
                    elif ws_id != crate_id:
                        differ("/ies", "the IEs of the message", ws_ids, crate_ids)
                        break
        # the values, IE by IE
        items = []
        if top is not None:
            for item in top.children:
                ident = next((n for n in iter_nodes(item) if n.name == f"{protocol}.id"), None)
                value = next((n for n in iter_nodes(item) if n.name == f"{protocol}.ie_field_value_element"), None)
                if ident is not None:
                    label = norm(ident.label().removeprefix("id-"))
                    label = mapping.ies.get(label, label)
                    items.append((f"unknown{ident.show}" if label.startswith("unknown") else label, value, item))
        crate_ies = [(norm(next(iter(ie))), next(iter(ie.values()))) for ie in tree.get("ies") or []
                     if isinstance(ie, dict) and ie]
        if "octets" in tree and "ies" not in tree:
            # a message whose IEs the crate does not read: what Wireshark notes in it
            notes = sorted({n.show for n in iter_nodes(proto) if n.name == "_ws.expert.message"})
            differ("/" + norm(tree.get("message", "")), "a message that the crate keeps as octets", "; ".join(notes),
                   tree.get("message", ""))
        for index, (name, value_node, item) in enumerate(items):
            if index >= len(crate_ies) or crate_ies[index][0] != name or value_node is None:
                continue
            kept = crate_ies[index][1]
            if isinstance(kept, dict) and set(kept) == {"octets"}:
                # an IE that did not decode, or whose value has more than the crate's version
                # of its type: what Wireshark notes in it, and its octets
                notes = sorted({n.show for n in iter_nodes(item) if n.name == "_ws.expert.message"})
                for node in iter_nodes(value_node):
                    if node is not value_node and node.name.startswith(protocol + "."):
                        coverage[("(pdu)", node.name, "inside an IE that the crate keeps as octets")] += 1
                if not notes and not item.value.lower().endswith(kept["octets"]):
                    differ(f"/{name}", "the octets of an IE", item.value, kept["octets"])
                # and what check() finds of it, which says why
                ie_name = next(iter(tree["ies"][index]))
                why = [finding.split(": ", 1)[-1] for finding in result["check"] if finding.startswith((ie_name + " ", f"the value of {ie_name} "))]
                differ(f"/{name}", "an IE that the crate keeps as octets" + (", where Wireshark notes: " + "; ".join(notes)
                                                                           if notes else ", where Wireshark notes nothing")
                       + (f" (check(): {'; '.join(why)})" if why else ""), "; ".join(notes), "octets")
                continue
            if kept is not None and not any(n.name.startswith(protocol + ".") for n in iter_nodes(value_node)
                                            if n is not value_node):
                differ(f"/{name}", "an IE that Wireshark does not dissect", "", name)
                continue
            ws_reading = Wireshark(protocol, mapping, coverage, value_node, name)
            found = crate_values(name, kept)
            unknown, empty, skip = set(), set(), set()
            for position, (key, value, within, parent) in enumerate(found):
                shown = [n for n, extension in within if (n, extension) in ws_reading.empty]
                if shown:
                    # in an IE or an extension whose value Wireshark leaves empty
                    empty.add(shown[0])
                    skip.add(position)
                    continue
                missing = [n for n, extension in within if extension and n not in ws_reading.extensions]
                if missing and ws_reading.undissected:
                    # in an extension that Wireshark does not know
                    unknown.add(missing[0])
                    skip.add(position)
            # What Wireshark reads inside an octet string that the crate's tree keeps as
            # octets (a container of a handover): compared as those octets
            kept_octets = {key for position, (key, value, _, _) in enumerate(found) if position not in skip
                           and isinstance(value, str) and re.fullmatch(r"[0-9a-f]+", value)}
            inside = set()
            for key, node in ws_reading.values:
                # (a SEQUENCE OF, whose entries are values of the crate too, is not one)
                if key in kept_octets and id(node) not in inside and not any(
                        c.name == "" and c.show.startswith("Item ") for c in node.children):
                    inside.update(id(n) for n in iter_nodes(node) if n is not node)
            shown_values = []
            for key, node in ws_reading.values:
                if id(node) in inside:
                    coverage[("(pdu)", node.name, "inside octets that the crate keeps")] += 1
                    continue
                shown_values.append((key, node))
            # the last IE that Wireshark shows of a message whose dissection an exception ended
            last = packet["stopped"] and index == len(items) - 1
            removed = after_throws(name, value_node, shown_values, found, skip, differ, last)
            ws = collections.defaultdict(list)
            for key, node in shown_values:
                ws[key].append(node)
            crate = collections.defaultdict(list)
            for position, (key, value, _, _) in enumerate(found):
                if position not in skip and position not in removed:
                    crate[key].append(value)
            for extension in sorted(unknown):
                differ(f"/{name}/{extension}", "an extension that Wireshark does not know", ",".join(ws_reading.undissected), extension)
            for inner in sorted(empty):
                differ(f"/{name}/{inner}", "an IE or extension that Wireshark does not dissect", "", inner)
            compare_named(protocol, mapping, coverage, differ, name, ws, crate)
    if not malformed:
        # what check() finds of the encoding, where Wireshark notes no error
        for finding in result.get("check", []):
            if finding.startswith("error:") or "does not decode" in finding:
                differ("", "a fault that only check() finds", "", finding)
    if differences:
        result["class"] = "ws-malformed" if malformed else "differs"
    else:
        result["class"] = "ws-malformed" if malformed else "agree"
    result["differences"] = differences
    return result, coverage


def after_throws(name, value_node, shown, found, skip, differ, stopped=False):
    """The positions of the crate's values that Wireshark did not read because it threw: for
    each exception in the IE, those in the member that Wireshark was reading (the field that
    shows the exception, or the closest one around it that is a value) after the last value
    that Wireshark shows in it. Each such member is one difference."""
    parent_of = {}
    for node in iter_nodes(value_node):
        for child in node.children:
            parent_of[id(child)] = node
    at = {id(node): position for position, (_, node) in enumerate(shown)}
    order = [index for index in range(len(found)) if index not in skip]

    def below(top):
        """The positions of the members of the crate within `top` (None: the whole IE)."""
        if top is None:
            return [index for index in order]
        inner = []
        for index in order:
            up = found[index][3]
            while up is not None and up != top:
                up = found[up][3]
            if up == top:
                inner.append(index)
        return inner

    removed = set()
    throws = [parent_of[id(node)] for node in iter_nodes(value_node) if node.name == "_ws.malformed"
              and id(node) in parent_of and parent_of[id(node)].name != "_ws.malformed"]
    if stopped:
        throws.append(value_node)
    for holder in throws:
        while id(holder) not in at and holder is not value_node and id(holder) in parent_of:
            holder = parent_of[id(holder)]
        if id(holder) in at:
            key = shown[at[id(holder)]][0]
            occurrence = sum(1 for k, _ in shown[:at[id(holder)] + 1] if k == key)
            same = [index for index in order if found[index][0] == key]
            if occurrence > len(same):
                continue
            top, where = same[occurrence - 1], f"/{name}/{key}" if key != name else f"/{name}"
        else:
            holder, top, where = value_node, None, f"/{name}"
        region = below(top)
        within = {id(n) for n in iter_nodes(holder) if n is not holder}
        seen = [k for k, n in shown if id(n) in within]
        cut = 0
        if seen:
            last, count = seen[-1], seen.count(seen[-1])
            positions = [i for i, index in enumerate(region) if found[index][0] == last]
            if len(positions) < count:
                continue
            cut = positions[count - 1] + 1
        after = [index for index in region[cut:] if index not in removed]
        if after:
            removed.update(after)
            differ(where, "what the crate reads after Wireshark threw", seen[-1] if seen else "",
                   f"{len(after)} values")
    return removed


def compare_named(protocol, mapping, coverage, differ, ie, ws, crate):
    for key, nodes in ws.items():
        field = nodes[0].name
        where = f"/{ie}/{key}" if key != ie else f"/{ie}"
        if key not in crate:
            why = mapping.why_none(field)
            if why or nodes[0].children and all(c.name.startswith(protocol + ".") or c.hide for c in nodes[0].children):
                # compared to nothing, or a container whose members are compared
                coverage[("(pdu)", field, "none" if why else "a container")] += 1
                continue
            coverage[("(pdu)", field, "unlisted")] += 1
            differ(where, "a field that only Wireshark shows", field, "")
            continue
        values = crate[key]
        coverage[("(pdu)", field, "compared")] += 1
        structure = lambda v: isinstance(v, dict) and not set(v) <= {"length", "value"}
        if len(values) != len(nodes) and all(structure(v) for v in values) and all(
                any(c.name.startswith(protocol + ".") for c in node.children) for node in nodes):
            # structures by one name (a CHOICE and a SEQUENCE, which Wireshark shows as no
            # value of its own): their members are compared
            continue
        if len(values) != len(nodes):
            # a CHOICE that has the name of its alternative: its members are compared
            leaves = [v for v in values if not (isinstance(v, dict) and not set(v) <= {"length", "value"})]
            if len(leaves) == len(nodes):
                values = leaves
            else:
                differ(where, "how many times", len(nodes), len(values))
                continue
        for node, value in zip(nodes, values):
            same, said = equal(protocol, mapping, node, value)
            if not same:
                # an enumerated value that Wireshark has no name for
                what = "a value that Wireshark shows as unknown" if node.label() == "Unknown" else "value"
                differ(where, what, said, value if not isinstance(value, (dict, list)) else json.dumps(value)[:200])
    for key, values in crate.items():
        if key not in ws and not any(isinstance(v, (dict, list)) for v in values):
            differ(f"/{ie}/{key}", "a member that only the crate shows", "", str(values[0])[:200])


def iter_nodes(node):
    yield node
    for child in node.children:
        yield from iter_nodes(child)


# ---------------------------------------------------------------------------
# Known differences

def load_known(path):
    """known.tsv: octets (`*`, or `re:` and a regular expression), where, what, verdict, reference.

    `where` is a regular expression over the path of a difference, `what` over its kind; for
    a PDU whose class is `what` (crate-refuses, ws-malformed), `where` is over the crate's
    error or Wireshark's errors. Such a row explains the differences of the PDU too (what
    Wireshark did not read after its error), unless `what` ends with ` alone`: an error in
    the octets of another protocol, after which Wireshark reads on.
    """
    known = []
    if path is None or not path.exists():
        return known
    for line in path.read_text().splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        octets, where, what, verdict, reference = (line.split("\t") + [""] * 5)[:5]
        known.append({"octets": octets, "where": re.compile(where), "what": what,
                      "verdict": verdict, "reference": reference})
    return known


def explained(known, result, difference):
    """The row of known.tsv that explains a difference; for `difference` None, the row that
    explains the class of the PDU, where each error of Wireshark needs a row (the first is
    the verdict), the generic note of an exception aside, which comes with the place of each."""
    rows = []
    for entry in known:
        octets = entry["octets"]
        if octets.startswith("re:"):
            if not re.search(octets[3:], result["octets"]):
                continue
        elif octets not in ("*", result["octets"]):
            continue
        rows.append(entry)
    if difference is None:
        context = " | " + result.get("crate-error", "") + " | " + "; ".join(result.get("check", []))
        errors = [e for e in dict.fromkeys(result.get("wireshark-errors", []))
                  if e != "Malformed Packet (Exception occurred)"] or [""]
        found = []
        for error in errors:
            row = next((entry for entry in rows if entry["what"].removesuffix(" alone") == result["class"]
                        and entry["where"].search(error + context)), None)
            if row is None:
                return None
            found.append(row)
        # a row that explains the differences too, where one explains Wireshark's stop
        return next((row for row in found if not row["what"].endswith(" alone")), found[0])
    for entry in rows:
        if entry["what"].removesuffix(" alone") in ("crate-refuses", "ws-malformed", "both-refuse"):
            continue
        if (entry["what"] == "*" or re.fullmatch(entry["what"], difference["what"])) and entry["where"].fullmatch(difference["where"]):
            return entry
    return None


RULES = [("the criticality of an IE or of the PDU", "criticality"), ("a mandatory IE missing", "mandatory"),
         ("the order of the IEs", "comes after"), ("an IE repeated", "in the message already"),
         ("an IE not of the message", "not an IE of the message"), ("a procedure not of the protocol", "no message of"),
         ("a value of a later version of its type", "later version")]


def rules(results):
    """The PDUs in which check() finds a deviation from the tables of the specification (the
    criticalities, presence and order of IEs) or a value of a later version: Wireshark reads
    the IEs and does not check them, so these are counted, not compared."""
    found = collections.Counter()
    for result in results:
        for label in {label for finding in result.get("check", []) for label, word in RULES if word in finding}:
            found[label] += 1
    return found


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--protocol", choices=PORT, required=True)
    parser.add_argument("--corpus", type=Path, required=True)
    parser.add_argument("--crate", type=Path, required=True)
    parser.add_argument("--mapping", type=Path, required=True)
    parser.add_argument("--known", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--any-tshark", action="store_true", help="compare with another version than " + PINNED)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)

    version = subprocess.check_output(["tshark", "--version"], text=True).splitlines()[0].rstrip(".")
    if version != PINNED and not args.any_tshark:
        print(f"skipped: {version} is not {PINNED}, which known.tsv was triaged with (--any-tshark compares anyway)")
        return 0

    mapping = Mapping(args.mapping)
    known = load_known(args.known)
    rows = [bytes.fromhex(line.split("\t")[-1]) for line in args.corpus.read_text().splitlines()
            if line.strip() and not line.startswith("#")]
    readings = [json.loads(line) for line in args.crate.read_text().splitlines() if line.strip()]
    if len(readings) != len(rows):
        raise AssertionError(f"{len(rows)} PDUs but {len(readings)} readings of the crate")
    packets = dissect(args.protocol, rows, args.output)

    results, coverage, verdicts = [], collections.Counter(), collections.Counter()
    for octets, packet, reading in zip(rows, packets, readings):
        if reading["octets"] != octets.hex():
            raise AssertionError(f"the readings are not those of the corpus at {octets.hex()}")
        result, seen = compare_one(args.protocol, mapping, octets, packet, reading)
        coverage.update(seen)
        unexplained = []
        whole = None
        if result["class"] in ("crate-refuses", "ws-malformed"):
            whole = explained(known, result, None)
            if whole is not None:
                result["known"] = whole["verdict"] + " " + whole["reference"]
                verdicts[whole["verdict"]] += 1
        covers = whole is not None and not whole["what"].endswith(" alone")
        for difference in result.get("differences", []):
            entry = whole if covers else explained(known, result, difference)
            if entry is None:
                unexplained.append(difference)
            else:
                difference["known"] = entry["verdict"] + " " + entry["reference"]
                if not covers:
                    verdicts[entry["verdict"]] += 1
        if result["class"] in ("crate-refuses", "ws-malformed") and whole is None:
            unexplained.append({"where": "", "what": result["class"], "wireshark": result["wireshark-errors"],
                                "crate": result.get("crate-error", "")})
        result["unexplained"] = len(unexplained)
        results.append(result)

    classes = collections.Counter(r["class"] for r in results)
    what = collections.Counter(d["what"] for r in results for d in r.get("differences", []))
    failing = [r for r in results if r["unexplained"]]
    report = {"wireshark": version, "protocol": args.protocol, "pdus": len(results), "classes": dict(classes),
              "differences": dict(what), "explained": dict(verdicts), "check-only": dict(rules(results)),
              "unexplained": len(failing), "results": results}
    (args.output / "report.json").write_text(json.dumps(report, indent=1) + "\n")
    with (args.output / "coverage.tsv").open("w") as table:
        table.write("# inside\tWireshark field\tstate\tPDUs\n")
        for (inside, field, state), count in sorted(coverage.items()):
            table.write(f"{inside}\t{field}\t{state}\t{count}\n")
    lines = [f"{version}: {len(results)} {args.protocol} PDUs"]
    lines += [f"  {c}: {n}" for c, n in sorted(classes.items())]
    lines.append("differences by kind:")
    lines += [f"  {w}: {n}" for w, n in sorted(what.items(), key=lambda x: -x[1])]
    lines.append("explained by known.tsv:")
    lines += [f"  {v}: {n}" for v, n in sorted(verdicts.items())]
    lines.append("what check() finds that a dissector does not check (PDUs):")
    lines += [f"  {k}: {n}" for k, n in sorted(rules(results).items())]
    lines.append(f"unexplained: {len(failing)} PDUs")
    summary = "\n".join(lines) + "\n"
    (args.output / "summary.txt").write_text(summary)
    print(summary, end="")
    for result in failing[:10]:
        print(f"{result['octets'][:80]}: {result['class']}")
        for difference in result.get("differences", [])[:4]:
            if "known" not in difference:
                print(f"    {difference['where']}: {difference['what']}: wireshark={difference['wireshark']!r} crate={difference['crate']!r}")
    return 1 if failing else 0


if __name__ == "__main__":
    sys.exit(main())
