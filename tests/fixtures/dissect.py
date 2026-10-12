#!/usr/bin/env python3
"""Give the vectors of `tests/fixtures` to tshark, and compare what it reads with them.

    python3 tests/fixtures/dissect.py

Run it from the directory of the crate, with `tshark` and `text2pcap` of Wireshark on
the path. Nothing of the crate takes part, and nothing of pycrate, which encoded the
vectors: this is a third reading of the octets.

For each vector of `messages.tsv`, `vectors.tsv` and `later.tsv`, tshark must read the
procedure code and the alternative of the PDU that the file says, and the identifiers of
the IEs of the message in their order, as this script reads them from the layout of the
octets. In a
vector that is made for a size, it must read a string of that many octets or bits, or a
list of that many entries. The script fails when one of these differs.

What else tshark says of a vector is counted and printed, and is no failure:

- a list in fragments, which Wireshark does not read ("10.9.3.8.1");
- the octets that hold a value of another specification, as a message of RRC or a
  classmark, which the vectors fill with random octets and Wireshark reads as that
  value: it stops at the first that is malformed, so the IEs after it are not compared;
- the open type of a private IE, which has no type;
- the additions of a later release, in `later.tsv`, which it says that it does not know.
"""

import collections
import pathlib
import re
import subprocess
import sys
import tempfile
import xml.etree.ElementTree as ET

# The dissectors of what the octets of a value hold, where a preference or a protocol
# switches them off: the vectors have random octets there.
OTHERS = (
    "nas-eps,nas-5gs,lte_rrc,nr-rrc,lppa,nrppa,ranap,rrc,gsm_a.bssmap,cdma2k,gcsna,bssgp,"
    "a21,x2ap,xnap,lpp,gsm_a.dtap,gsm_sms,gsm_map"
)


def open_type(octets, at):
    """The octets of an open type at `at`, and the place after it."""
    content = b""
    while True:
        first = octets[at]
        if first < 0x80:
            length, at = first, at + 1
        elif first < 0xC0:
            length, at = (first & 0x3F) << 8 | octets[at + 1], at + 2
        else:
            length, at = (first & 0x3F) * 16384, at + 1
        content += octets[at : at + length]
        at += length
        if first < 0xC0:
            return content, at


def identifiers(wire):
    """The identifiers of the IEs of the message of a PDU, from its layout."""
    message, _ = open_type(wire, 3)
    found, at = [], 3
    for _ in range(int.from_bytes(message[1:3], "big")):
        found.append(int.from_bytes(message[at : at + 2], "big"))
        _, at = open_type(message, at + 3)
    return found


def dissect(protocol, rows, directory):
    """What tshark reads in each vector: a `packet` element of its PDML."""
    dump = directory / "vectors.txt"
    with dump.open("w") as text:
        for name, wire, *_ in rows:
            octets = bytes.fromhex(wire)
            text.write(f"# {name}\n")
            for at in range(0, len(octets), 32):
                text.write(f"{at:06x} {octets[at:at + 32].hex(' ')}\n")
            text.write("\n")
    # Each PDU is a frame of its own link type, with no SCTP and no IP around it: a
    # PDU may be longer than an IP packet.
    capture = directory / "vectors.pcapng"
    command = ["text2pcap", "-q", "-l", "147", str(dump), str(capture)]
    subprocess.run(command, check=True, capture_output=True)
    command = ["tshark", "-n", "-r", str(capture), "-T", "pdml"]
    command += ["-o", f'uat:user_dlts:"User 0 (DLT=147)","{protocol}","0","","0",""']
    command += ["-o", f"{protocol}.dissect_container:FALSE", "--disable-protocol", OTHERS]
    read = subprocess.run(command, check=True, capture_output=True)
    return ET.fromstring(read.stdout).findall("packet")


def compare(protocol, row, packet):
    """Why what tshark reads is not the vector, and what else it says of it."""
    name, wire, code, alternative = row[:4]
    fields = list(packet.iter("field"))
    shown = lambda field: next((f.get("show") for f in fields if f.get("name") == field), None)
    remarks = [f.get("show") for f in fields if f.get("name") == "_ws.expert.message"]
    remarks = [re.sub(r"\d+", "N", remark) for remark in remarks]
    stopped = any(proto.get("name") == "_ws.malformed" for proto in packet.iter("proto"))
    failures = []
    if shown(f"{protocol}.procedureCode") != code:
        failures.append(f"procedure code {shown(f'{protocol}.procedureCode')}")
    if shown(f"{protocol}.{protocol.upper()}_PDU") != alternative:
        failures.append(f"alternative {shown(f'{protocol}.{protocol.upper()}_PDU')}")
    # The IEs of the message: all of them, or those before the place where it stopped.
    container = next((f for f in fields if f.get("name") == f"{protocol}.protocolIEs"), None)
    if container is not None:
        read = []
        for entry in container.findall("field"):
            ids = [f for f in entry.iter("field") if f.get("name") == f"{protocol}.id"]
            read.extend(int(f.get("show")) for f in ids[:1])
        sent = identifiers(bytes.fromhex(wire))
        if read != (sent[: len(read)] if stopped else sent):
            failures.append(f"IEs {read} for {sent}")
    # The string or the list of a size.
    sized = re.fullmatch(r".*/(length|count|bits|largest)-(.+)", name)
    if sized and not failures and not stopped:
        kind, size = sized.groups()
        octets = [len(f.get("value", "")) // 2 for f in fields]
        entries = [re.search(r": (\d+) items?$", f.get("showname", "")) for f in fields]
        entries = [int(found.group(1)) for found in entries if found]
        if kind == "length" and int(size) not in octets:
            failures.append(f"no string of {size} octets")
        if kind == "bits" and (int(size) + 7) // 8 not in octets:
            failures.append(f"no string of {size} bits")
        if kind == "count" and int(size) not in entries:
            failures.append(f"no list of {size} entries")
        if kind == "largest" and max(octets + entries) < 4096:
            failures.append("no list and no string of the largest size")
    return failures, remarks


def main():
    protocol = next(name for name in ("ngap", "s1ap") if pathlib.Path(f"src/{name}.rs").exists())
    version = subprocess.run(["tshark", "--version"], capture_output=True, text=True).stdout
    print(version.splitlines()[0])
    failed = False
    for file in ("messages.tsv", "vectors.tsv", "later.tsv"):
        lines = pathlib.Path("tests/fixtures", file).read_text().splitlines()
        rows = [line.split("\t") for line in lines if not line.startswith("#")]
        with tempfile.TemporaryDirectory() as directory:
            packets = dissect(protocol, rows, pathlib.Path(directory))
        assert len(packets) == len(rows), f"{len(packets)} packets for {len(rows)} vectors"
        without, counted = 0, collections.Counter()
        for row, packet in zip(rows, packets):
            failures, remarks = compare(protocol, row, packet)
            for failure in failures:
                failed = True
                print(f"{file}: {row[0]}: {failure}")
            without += not remarks
            counted.update(set(remarks))
        print(f"{file}: {len(rows)} vectors, {without} that tshark reads without a remark")
        for remark, count in counted.most_common():
            print(f"    {count:4} {remark}")
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
