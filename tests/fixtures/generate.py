#!/usr/bin/env python3
"""Write the vectors of `tests/fixtures`, which pycrate encodes from the ASN.1 of the
specification.

    python3 -m venv venv && venv/bin/pip install pycrate==0.8.1
    venv/bin/python tests/fixtures/generate.py           # write the files
    venv/bin/python tests/fixtures/generate.py --check   # compare them with what is there

Run it from the directory of the crate, with the ASN.1 modules where
`build/asn1.sha256` lists them: `build/extract_asn1.py` takes them out of the archive
of the specification. Nothing of the crate takes part. pycrate compiles the modules
with its own compiler and encodes the values with its own codec, in aligned PER. The
values are made here from the types of the compiled modules, with a random generator
whose seed is the digest of the name of the vector: the files come out the same octets
every time.

`messages.tsv` has one message for each message of each procedure, with the IEs and
the members that are mandatory and nothing else.

`vectors.tsv` has messages with content:

- `cover-N`: each message with every IE of its object set and every member, again and
  again. The pass N gives a CHOICE the alternative, an ENUMERATED the value and a single
  container the IE that no vector has yet. It gives a number and a size the lower bound,
  the upper bound or a value between them, and once the value after the upper bound
  when the constraint is extensible. A string has once the largest size of its type,
  and once a size of 128 or more. One pass in four leaves out members and IEs that may
  be absent. A vector is kept when it holds what no vector before it holds, and the
  passes end after eight that keep nothing.
- `length-N`: a NAS-PDU of N octets, where N, the open type of the IE that holds it or
  the open type of the message has a length around a form of the length determinant:
  one octet up to 127, two up to 16383, then fragments of 16384 to 65536 (ITU-T X.691
  (02/2021) clause 11.9).
- `count-N` and `bits-N`: a list of N entries and a bit string of N bits, around the
  same forms.
- `largest-TYPE`: a list or a string of the largest size that its type has.

With `--values FILE` the script also writes each vector of `vectors.tsv` as the JSON
view of its PDU, a line for each with its name: the values in the JSON encoding of
ASN.1, as pycrate has them. A test of `tests/vectors.rs` writes a PDU from each view
and compares its octets with the vector.
"""

import argparse
import hashlib
import importlib
import inspect
import json
import pathlib
import random
import re
import sys
import tempfile
import textwrap

# How far over its lower bound the size of a list and of a string goes, but for the
# sizes that a vector is made for.
LIST_CAP = 3
STRING_CAP = 12
# How deep a value has the members that may be absent and more entries than a list
# needs.
DEPTH = 40
PRINTABLE = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789 '()+,-./:=?"
VISIBLE = "".join(chr(c) for c in range(32, 127))
STRINGS = ("PrintableString", "VisibleString", "UTF8String")
ALTERNATIVES = ["initiatingMessage", "successfulOutcome", "unsuccessfulOutcome"]
# The lengths around which a length determinant changes its form.
FORMS = {127, 128, 16383, 16384}
FRAGMENTS = (127, 128, 16383, 16384, 16385, 32768, 49153, 65536, 65537, 81920, 131072)

# What is particular to a protocol.
#
# `nas` is a NAS message, `carried_message` the IE that carries a PDU, and `handover` a
# type of handover, with the types that the octets of its containers hold. `other` is
# what an OCTET STRING holds of the other protocol, in aligned PER, each as pycrate
# encodes a value of it: the E-UTRA MDT mode and the SON configuration transfer to an
# en-gNB of TS 36.413 in NGAP, and the MDT configuration and the SON configuration
# transfer of TS 38.413 in S1AP.
#
# The rest says where the vectors of a length and of a size are. `length` is a message
# that has a NAS-PDU. `count` and `bits` are a message, a type, its sizes, the
# alternatives that lead to the type and the members and the IEs that do, each with the
# name of its SEQUENCE. Each `largest` is the same with one size.
PROTOCOLS = {
    "ngap": {
        "specification": "3GPP TS 38.413 v19.4.0",
        "nas": "7e006416",
        "carried_message": 42,
        "handover": "intra5gs",
        "containers": {
            "sourcetotargettransparentcontainer": "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer",
            "targettosourcetransparentcontainer": "TargetNGRANNode-ToSourceNGRANNode-TransparentContainer",
            "targettosourcefailuretransparentcontainer": "TargetNGRANNode-ToSourceNGRANNode-FailureTransparentContainer",
        },
        "other": {
            "mdtmodeeutra": "1fd4000000000800ab40010000ac400940000000015a40010000ad400940800000015b4001"
            "0000ae4001c300dc400960080000015c40010000dd400940000000015d400100011c40046000fe00011d40"
            "0470007400015940024000",
            "endcsonconfigurationtransfer": "438002f83900a1eaf002f839dae80002f83900f0213402f839753c"
            "0002f839408ddedb7002f839dd620002f83988560002f839750285083ec0268a8d00010099400d060f8086"
            "e7ca7500f8a761bc3b00c1400607c0f3e9f37d",
        },
        "length": "DownlinkNASTransport",
        "count": (
            "NGReset",
            "UE-associatedLogicalNG-connectionList",
            (127, 128, 16383, 16384, 16385, 65536),
            {"ResetType": "partOfNG-Interface"},
            (),
        ),
        "bits": (
            "UplinkRANStatusTransfer",
            "receiveStatusOfUL-PDCP-SDUs",
            FRAGMENTS,
            {"DRBStatusUL": "dRBStatusUL18"},
            [("DRBStatusUL18", "receiveStatusOfUL-PDCP-SDUs")],
        ),
        "largest": [
            (
                "NGSetupResponse",
                "ExtendedSliceSupportList",
                65535,
                {},
                [("PLMNSupportItem", "iE-Extensions"), ("PLMNSupportItem", "ExtendedSliceSupportList")],
            ),
            (
                "DownlinkNASTransport",
                "ForbiddenTACs",
                4096,
                {},
                [
                    ("DownlinkNASTransport", "MobilityRestrictionList"),
                    ("MobilityRestrictionList", "forbiddenAreaInformation"),
                ],
            ),
            (
                "WriteReplaceWarningRequest",
                "WarningMessageContents",
                9600,
                {},
                [("WriteReplaceWarningRequest", "WarningMessageContents")],
            ),
        ],
    },
    "s1ap": {
        "specification": "3GPP TS 36.413 v19.2.0",
        "nas": "076002",
        "carried_message": 225,
        "handover": "intralte",
        "containers": {
            "sourcetotargettransparentcontainer": "SourceeNB-ToTargeteNB-TransparentContainer",
            "targettosourcetransparentcontainer": "TargeteNB-ToSourceeNB-TransparentContainer",
        },
        "other": {
            "mdtconfigurationnr": "46000002f839de28f8dd31ff6fb80008000000011d400100000101544001000169"
            "4002600040000000015040010040800000015140010040800001015240010001554004000000004000000001"
            "534001006000007638009d0f54000002f83900010199400a00000002f839d400763901bf400b0002f8390000"
            "1770723658",
            "intersystemsonconfigurationtransfer": "680002f83900d79bf402f83936a5ea0000002d40090002f8"
            "39e7eba149101002f83900edd21002f839884b07081fa2a92ddf070f809dc70a9d00f8b2c8c24d000000ad40"
            "0607c0dc2c2cc3000001bd40040002f839",
        },
        "length": "DownlinkNASTransport",
        "count": (
            "S1SetupResponse",
            "ServedGroupIDs",
            (127, 128, 255, 256, 257, 16383, 16384),
            {},
            (),
        ),
        "bits": (
            "ENBStatusTransfer",
            "ReceiveStatusOfULPDCPSDUsPDCP-SNlength18",
            FRAGMENTS,
            {},
            [
                ("Bearers-SubjectToStatusTransfer-Item", "iE-Extensions"),
                ("Bearers-SubjectToStatusTransfer-Item", "ReceiveStatusOfULPDCPSDUsPDCP-SNlength18"),
            ],
        ),
        "largest": [
            ("S1SetupResponse", "ServedGroupIDs", 65535, {}, ()),
            (
                "DownlinkNASTransport",
                "ForbiddenTACs",
                4096,
                {},
                [
                    ("DownlinkNASTransport", "HandoverRestrictionList"),
                    ("HandoverRestrictionList", "forbiddenTAs"),
                ],
            ),
            (
                "WriteReplaceWarningRequest",
                "WarningMessageContents",
                9600,
                {},
                [("WriteReplaceWarningRequest", "WarningMessageContents")],
            ),
        ],
    },
}

# Whether the last value under an alternative, a value of an ENUMERATED or an IE held
# something that no vector had: a pass goes back to it until it holds nothing new.
FRUITFUL = {}


def canonical(name):
    return re.sub(r"[^a-z0-9]", "", name.lower())


def asn_name(obj):
    """The name of the type of an object: that of the type that it refers to, or its own."""
    return obj._tr._name if obj._tr is not None else obj._name


def table_rows(table):
    return list(table._val.root) + list(table._val.ext or [])


def ie_rows(obj):
    """The object set of a field of a container of IEs."""
    if obj.TYPE != "SEQUENCE":
        return []
    identifier = obj._cont["id"] if "id" in obj._cont else None
    table = getattr(identifier, "_const_tab", None)
    return table_rows(table) if table is not None else []


def empty_table(obj):
    """Whether an object is a container, or a single IE, of an object set that is empty."""
    if obj.TYPE == "SEQUENCE OF":
        obj = obj._cont
    if obj.TYPE != "SEQUENCE":
        return False
    for component in obj._cont.values():
        table = getattr(component, "_const_tab", None)
        if table is not None and getattr(component, "_const_tab_at", None) is None:
            return not list(table._val.root)
    return False


def bounds(constraint, default_lower=0):
    """The root of a size constraint, and whether it is extensible."""
    if constraint is None:
        return default_lower, None, False
    root = constraint.root[0]
    lower, upper = (root, root) if isinstance(root, int) else (root.lb, root.ub)
    return lower, upper, constraint.ext is not None


def seed(protocol, name, variant):
    digest = hashlib.sha256(f"oxirush-aper-v1/{protocol}/{name}/{variant}".encode()).digest()
    return int.from_bytes(digest[:8], "big")


class Values:
    """Values of the types of a compiled module.

    In the mode `min` a value has what is mandatory and nothing else, at the lower
    bounds. In the mode `cover` it has what the pass `turn` gives it, and takes what
    `seen` does not have: `keys` collects what it holds.
    """

    def __init__(self, protocol, module, mode, name, variant, seen=frozenset(), turn=0):
        self.protocol = protocol
        self.module = module
        self.mode = mode
        self.rng = random.Random(seed(protocol, name, variant))
        self.seen = seen
        self.keys = set()
        self.fresh = 0
        self.turn = turn
        self.depth = 0
        # The name of the SEQUENCE or of the CHOICE that a value is in.
        self.parent = None
        # What a vector of a size asks for, each by the name of its type: the size of a
        # type, the alternative of a CHOICE, and the members and the IEs that are there.
        self.sizes = {}
        self.alternatives = {}
        self.members = set()

    # What a vector holds, and what no vector holds yet.

    def named(self, obj):
        """The name of a type among the types of the module: a type that has no name
        of its own is told by the member that it is the type of."""
        return asn_name(obj) if obj._tr is not None else f"{self.parent}.{obj._name}"

    def new(self, key):
        return key not in self.seen and key not in self.keys

    def cover(self, key):
        self.fresh += self.new(key)
        self.keys.add(key)

    def first_new(self, kind, name, candidates):
        """The first of `candidates` that no vector has, then the first under which
        the last value held something that no vector had, or any of them."""
        for candidate in candidates:
            if self.new(f"{kind}:{name}.{candidate}"):
                return candidate
        for candidate in candidates:
            if FRUITFUL.get(f"{kind}:{name}.{candidate}"):
                return candidate
        return self.rng.choice(candidates)

    def under(self, key, value):
        """Make a value under an alternative, a value of an ENUMERATED or an IE, and
        remember whether it held something that no vector had."""
        before = self.fresh
        self.cover(key)
        made = value()
        FRUITFUL[key] = self.fresh > before
        return made

    def leaves_out(self, there, absent=None):
        """Whether a pass leaves out a member or an IE that may be absent. One pass in
        four does, for what a vector has already: more often what was always there."""
        if self.turn % 4 != 3 or self.new(there):
            return False
        return self.rng.random() < (0.6 if absent and self.new(absent) else 0.2)

    def namespace(self, suffix):
        return getattr(self.module, f"{self.protocol.upper()}_{suffix}")

    # Sizes.

    def size(self, obj, strings=False, default_lower=0):
        """The size of a list or, with `strings`, of a string."""
        if asn_name(obj) in self.sizes:
            return self.sizes[asn_name(obj)]
        lower, upper, extensible = bounds(obj._const_sz, default_lower)
        if self.mode == "min":
            return lower
        name = self.named(obj)
        if extensible and upper is not None and upper < 4096 and self.new(f"Z:{name}"):
            # One more than the root of an extensible size has.
            self.cover(f"Z:{name}")
            return upper + 1
        if strings and upper is not None and lower < upper <= 16384 and self.new(f"M:{name}"):
            # The largest size of a string.
            self.cover(f"M:{name}")
            return upper
        if strings and (upper is None or upper - lower >= 128) and self.new(f"L:{name}"):
            # A size that has a length determinant of two octets.
            self.cover(f"L:{name}")
            return max(lower, 128) + self.rng.randint(0, 40)
        if asn_name(obj) == "TransportLayerAddress":
            # An IPv4 address, an IPv6 address, and the two.
            return (32, 128, 160)[self.turn % 3]
        cap = STRING_CAP if strings else LIST_CAP
        highest = lower + cap if upper is None else min(upper, lower + cap)
        least = min(max(lower, 1), highest)
        if not strings and self.turn != 1:
            # A list has one entry or two, but in the second pass, where it has as many
            # as the cap lets it, and in the passes that leave members out, where it
            # may be empty.
            if self.turn % 4 == 3:
                return self.rng.randint(lower, min(least + 1, highest))
            return min(least + self.turn % 2, highest)
        if self.turn % 3 == 0:
            return least
        if self.turn % 3 == 1:
            return highest
        return self.rng.randint(least, highest)

    # One value of each kind of type.

    def gen(self, obj):
        protocol = PROTOCOLS[self.protocol]
        name = canonical(asn_name(obj))
        octets = obj.TYPE == "OCTET STRING" and asn_name(obj) not in self.sizes
        if octets and asn_name(obj) in ("NAS-PDU", "NASPDU"):
            return bytes.fromhex(protocol["nas"])
        if octets and name == "plmnidentity":
            return bytes.fromhex("02f839")
        if octets and name in ("ngrantraceid", "eutrantraceid"):
            return bytes.fromhex("02f8390000010001")
        if octets and name == "imsi":
            return bytes.fromhex("029813" if self.mode == "min" else "02980300000000f1")
        if self.protocol == "s1ap" and octets and name == "coarseuelocation":
            # TS 36.413 9.2.1.167: an Ellipsoid-Point in unaligned PER.
            return bytes(6)
        if octets and name == "warningmessagecontents":
            pages = 1 if self.mode == "min" else 1 + self.turn % 3
            return bytes([pages]) + (bytes(82) + bytes([82])) * pages
        if obj.TYPE == "BIT STRING" and name == "datacodingscheme":
            return 0x0F, 8
        if octets and name in protocol["other"]:
            return bytes.fromhex(protocol["other"][name])
        if octets and name in protocol["containers"]:
            contained = protocol["containers"][name].replace("-", "_")
            contained = getattr(self.namespace("IEs"), contained)
            contained.set_val(self.gen(contained))
            return contained.to_aper()
        self.depth += 1
        try:
            return getattr(self, "g_" + obj.TYPE.replace(" ", "_").replace("-", "_"))(obj)
        finally:
            self.depth -= 1

    def g_NULL(self, obj):
        return 0

    def g_BOOLEAN(self, obj):
        return self.rng.random() < 0.5

    def g_INTEGER(self, obj):
        constraint = obj._const_val
        if constraint is None or not constraint.root:
            lower, upper = -(2**40), 2**40
            if self.mode == "min":
                return lower
            return self.rng.choice([lower, upper, self.rng.randint(lower, upper)])
        ranges = [
            (value, value) if isinstance(value, int) else (value.lb, value.ub)
            for value in constraint.root
        ]
        lower = min(low if low is not None else -(2**40) for low, _ in ranges)
        upper = max(high if high is not None else 2**40 for _, high in ranges)
        if self.mode == "min":
            return lower
        key = f"X:{self.named(obj)}"
        if constraint.ext is not None and self.new(key):
            # One more than the root of an extensible constraint has.
            self.cover(key)
            return upper + 1
        if self.turn % 3 == 0:
            return lower
        if self.turn % 3 == 1:
            return upper
        # A value of one of the ranges of the root: a number between two of them is in
        # the range of the root and is none of its values.
        low, high = self.rng.choice(ranges)
        return self.rng.randint(lower if low is None else low, upper if high is None else high)

    def g_ENUMERATED(self, obj):
        if canonical(asn_name(obj)) == "handovertype":
            # The type of handover that the containers of the vectors are of.
            return PROTOCOLS[self.protocol]["handover"]
        root = list(obj._root)
        if self.mode == "min":
            return root[0]
        name = self.named(obj)
        value = self.first_new("E", name, root + list(obj._ext or []))
        return self.under(f"E:{name}.{value}", lambda: value)

    def g_BIT_STRING(self, obj):
        bits = self.size(obj, strings=True)
        if asn_name(obj) in self.sizes:
            # A pattern, for a string of a size that a vector is made for.
            value = int.from_bytes(bytes([0xA5]) * ((bits + 7) // 8), "big") >> (-bits % 8)
        elif self.mode == "min":
            return 0, bits
        else:
            value = self.rng.getrandbits(bits) if bits else 0
        if bits:
            # Its first and its last bit are set: a string that is read from another
            # place, or to another length, is another one.
            value |= 1 << (bits - 1) | 1
        return value, bits

    def g_OCTET_STRING(self, obj):
        contained = getattr(obj, "_const_cont", None)
        if contained is not None:
            name = asn_name(contained) if contained._tr is not None else contained.TYPE
            return name, self.gen(contained)
        octets = self.size(obj, strings=True)
        if asn_name(obj) in self.sizes:
            return bytes(index % 251 for index in range(octets))
        return bytes(self.rng.getrandbits(8) for _ in range(octets)) if octets else b""

    def string(self, obj, alphabet):
        length = self.size(obj, strings=True, default_lower=1)
        return "".join(self.rng.choice(alphabet) for _ in range(length))

    def g_PrintableString(self, obj):
        return self.string(obj, PRINTABLE)

    def g_VisibleString(self, obj):
        return self.string(obj, VISIBLE)

    def g_UTF8String(self, obj):
        return self.string(obj, PRINTABLE + "éß€")

    def g_OBJECT_IDENTIFIER(self, obj):
        return 1, 2, 3, self.rng.randint(0, 100000)

    def g_CHOICE(self, obj):
        name = self.named(obj)
        root = list(obj._root)
        root = [alternative for alternative in root if not empty_table(obj._cont[alternative])] or root
        if asn_name(obj) in self.alternatives:
            alternative = self.alternatives[asn_name(obj)]
        elif self.mode == "min":
            alternative = root[0]
        else:
            alternative = self.first_new("C", name, root + list(obj._ext or []))
        outer, self.parent = self.parent, name
        try:
            value = lambda: self.component(obj._cont[alternative], None)
            if self.mode == "min":
                return alternative, value()
            return alternative, self.under(f"C:{name}.{alternative}", value)
        finally:
            self.parent = outer

    def g_SEQUENCE_OF(self, obj):
        rows = ie_rows(obj._cont)
        if rows and "presence" in rows[0]:
            return self.container(obj, rows)
        count = self.size(obj)
        if asn_name(obj) in self.sizes:
            # A list of a size that a vector is made for: seven entries, again and again.
            entries = [self.component(obj._cont, None) for _ in range(min(count, 7))]
            return [entries[index % 7] for index in range(count)]
        if self.depth > DEPTH:
            count = bounds(obj._const_sz)[0]
        return [self.component(obj._cont, None) for _ in range(count)]

    def container(self, obj, rows):
        """A container of IEs: those of its object set that are mandatory, and in the
        mode `cover` the others too, in the order of the set."""
        chosen = []
        for row in rows:
            key = f"I:{self.parent}.{row['id']}"
            if row["presence"] != "mandatory":
                value = row.get("Value") or row["Extension"]
                if self.mode == "min" and (self.parent, asn_name(value)) not in self.members:
                    continue
                if self.mode != "min" and self.leaves_out(key):
                    continue
            if self.mode != "min":
                self.cover(key)
            chosen.append(row)
        if not chosen:
            chosen = rows[: bounds(obj._const_sz)[0]]
        return [
            {member: self.component(component, row) for member, component in obj._cont._cont.items()}
            for row in chosen
        ]

    def g_SEQUENCE(self, obj):
        name = self.named(obj)
        value = {}
        row = None
        # The object of the set that the identifier of an IE selects, for a single IE.
        keys = {
            component._const_tab_id
            for component in obj._cont.values()
            if getattr(component, "_const_tab", None) is not None
        }
        for component in obj._cont.values():
            table = getattr(component, "_const_tab", None)
            if table is not None and getattr(component, "_const_tab_at", None) is None:
                rows = [entry for entry in table_rows(table) if keys <= set(entry)]
                if rows:
                    row = self.row(rows)
        outer, self.parent = self.parent, name
        try:
            for member, component in obj._cont.items():
                if member not in obj._root_mand:
                    if empty_table(component) or not self.present(asn_name(obj), member):
                        continue
                value[member] = self.component(component, row)
        finally:
            self.parent = outer
        return value

    def present(self, name, member):
        """Whether a member that may be absent is there, in the SEQUENCE `self.parent`."""
        if (name, member) in self.members:
            return True
        if self.mode == "min" or self.depth > DEPTH:
            return False
        there, absent = f"S:{self.parent}.{member}", f"A:{self.parent}.{member}"
        if self.leaves_out(there, absent):
            self.cover(absent)
            return False
        self.cover(there)
        return True

    def row(self, rows):
        """The object of a set for a single IE."""
        if self.mode == "min":
            return rows[0]
        identifier = self.first_new("I", self.parent, [row["id"] for row in rows])
        self.cover(f"I:{self.parent}.{identifier}")
        return next(row for row in rows if row["id"] == identifier)

    def component(self, component, row):
        if row is not None and component.TYPE == "OPEN_TYPE":
            if self.protocol == "ngap" and row.get("id") == 37:
                # NASC, the value of TS 24.501 9.11.2.6 without its identifier and its
                # length.
                return "NAS-PDU", bytes.fromhex("55667788331265")
            if row.get("id") == PROTOCOLS[self.protocol]["carried_message"]:
                return "OCTET STRING", self.carried_message()
        table = getattr(component, "_const_tab", None)
        if table is not None:
            if row is None:
                rows = list(table._val.root)
                if not rows:
                    return self.unconstrained(component)
                row = self.row(rows)
            field = row[component._const_tab_id]
            if component.TYPE == "OPEN_TYPE":
                return asn_name(field) if field._tr is not None else field.TYPE, self.gen(field)
            return field
        if component.TYPE == "OPEN_TYPE":
            return self.unconstrained(component)
        return self.gen(component)

    def unconstrained(self, component):
        if component.TYPE == "OPEN_TYPE":
            count = self.rng.randint(1, 5)
            return "_unk_004", bytes(self.rng.getrandbits(8) for _ in range(count))
        return self.gen(component)

    def carried_message(self):
        """The PDU of an INITIAL UE MESSAGE, which a rerouted NAS message carries."""
        known = messages(self.protocol, self.module)
        name, procedure, alternative, obj = next(
            message for message in known if message[0] == "InitialUEMessage"
        )
        values = Values(self.protocol, self.module, "min", "nestedInitialUEMessage", "min")
        pdu = pdu_type(self.protocol, self.module)
        pdu.set_val(pdu_value(procedure, alternative, name, values.gen(obj)))
        return pdu.to_aper()


def compile_modules(protocol, directory):
    """The modules of `build/asn1.sha256`, compiled by pycrate into a module of Python."""
    from pycrate_asn1c.asnproc import PycrateGenerator, compile_text, generate_modules
    from pycrate_asn1rt import codecs
    from pycrate_asn1rt.codecs import ASN1CodecPER

    # pycrate 0.8.1 does not write two values in fragments (ITU-T X.691 (02/2021)
    # clause 11.9.3.8), and both are repaired here. For a list of 16384 entries or more
    # it calls `encode_pas`, which it does not have: it is `encode_pad`, as it is two
    # lines further. For a bit string it takes the octets of each fragment after the
    # first up to `off+fs>>3`, which is `(off+fs)>>3`, where it means `off+(fs>>3)`.
    if not hasattr(ASN1CodecPER, "encode_pas"):
        ASN1CodecPER.encode_pas = ASN1CodecPER.encode_pad
    source = textwrap.dedent(inspect.getsource(ASN1CodecPER.encode_fragbytes))
    repaired = {}
    exec(source.replace("off+fs>>3", "off+(fs>>3)"), vars(codecs), repaired)
    ASN1CodecPER.encode_fragbytes = repaired["encode_fragbytes"]
    texts = []
    for entry in pathlib.Path("build/asn1.sha256").read_text().splitlines():
        digest, path = entry.split()
        octets = pathlib.Path(path).read_bytes()
        if hashlib.sha256(octets).hexdigest() != digest:
            sys.exit(f"{path} is not the module that build/asn1.sha256 records")
        texts.append(octets.decode())
    compile_text(texts)
    generate_modules(PycrateGenerator, str(directory / f"{protocol}_py.py"))
    sys.path.insert(0, str(directory))
    return importlib.import_module(f"{protocol}_py")


def pdu_type(protocol, module):
    descriptions = getattr(module, protocol.upper() + "_PDU_Descriptions")
    return getattr(descriptions, protocol.upper() + "_PDU")


def messages(protocol, module):
    """Each message of each procedure: its name, the row of its procedure, the
    position of its alternative in the PDU and its type."""
    descriptions = getattr(module, protocol.upper() + "_PDU_Descriptions")
    for procedure in table_rows(descriptions.InitiatingMessage._cont["procedureCode"]._const_tab):
        for alternative, direction in enumerate(ALTERNATIVES):
            direction = direction[0].upper() + direction[1:]
            if direction in procedure:
                obj = procedure[direction]
                yield asn_name(obj), procedure, alternative, obj


def pdu_value(procedure, alternative, name, value):
    """The value of the PDU of a message."""
    members = {
        "procedureCode": procedure["procedureCode"],
        "criticality": procedure["criticality"],
        "value": (name, value),
    }
    return ALTERNATIVES[alternative], members


def minimal(protocol, module):
    """`messages.tsv`."""
    pdu = pdu_type(protocol, module)
    lines = [
        f"# {PROTOCOLS[protocol]['specification']}; independent pycrate 0.8.1 vectors.",
        "# Message\twire bytes\tprocedure code\tPDU alternative",
    ]
    for name, procedure, alternative, obj in messages(protocol, module):
        values = Values(protocol, module, "min", name, "min")
        pdu.set_val(pdu_value(procedure, alternative, name, values.gen(obj)))
        wire = pdu.to_aper()
        lines.append(f"{name}\t{wire.hex()}\t{procedure['procedureCode']}\t{alternative}")
    return "\n".join(lines) + "\n"


def covering(protocol, module, seen):
    """The `cover-N` vectors, each as its name and the value of its PDU."""
    idle, turn = 0, 0
    while idle < 8:
        kept = 0
        for name, procedure, alternative, obj in messages(protocol, module):
            variant = f"cover-{turn}"
            values = Values(protocol, module, "cover", name, variant, seen, turn)
            value = values.gen(obj)
            if values.keys <= seen:
                continue
            seen |= values.keys
            kept += 1
            yield f"{name}/{variant}", pdu_value(procedure, alternative, name, value)
        idle = 0 if kept else idle + 1
        turn += 1


def open_type(octets, at):
    """The octets of an open type at `at`, and the place after it: a length determinant
    of one octet, of two, or one for each fragment."""
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


def open_types(wire):
    """The lengths of the open types of a PDU: that of its message, then that of the
    value of each IE."""
    message, _ = open_type(wire, 3)
    lengths = [len(message)]
    at = 3
    for _ in range(int.from_bytes(message[1:3], "big")):
        value, at = open_type(message, at + 3)
        lengths.append(len(value))
    return lengths


def sized(protocol, module):
    """The vectors of a length and of a size, each as its name and the value of its PDU."""
    known = {message[0]: message for message in messages(protocol, module)}
    pdu = pdu_type(protocol, module)

    def vector(message, variant, sizes, alternatives=(), members=()):
        name, procedure, alternative, obj = known[message]
        values = Values(protocol, module, "min", name, variant)
        values.sizes, values.alternatives, values.members = sizes, dict(alternatives), set(members)
        return f"{name}/{variant}", pdu_value(procedure, alternative, name, values.gen(obj))

    # A NAS-PDU of each size at which its length, that of its IE or that of the
    # message is around a form of the length determinant, and one of 64K and an octet.
    message = PROTOCOLS[protocol]["length"]
    for octets in [*range(100, 140), *range(16340, 16390), 65537]:
        made = vector(message, f"length-{octets}", {"NAS-PDU": octets})
        pdu.set_val(made[1])
        if octets == 65537 or {octets, *open_types(pdu.to_aper())} & FORMS:
            yield made
    for kind in ("count", "bits"):
        message, sized_type, sizes, alternatives, members = PROTOCOLS[protocol][kind]
        for size in sizes:
            yield vector(message, f"{kind}-{size}", {sized_type: size}, alternatives, members)
    for message, sized_type, size, alternatives, members in PROTOCOLS[protocol]["largest"]:
        yield vector(message, f"largest-{sized_type}", {sized_type: size}, alternatives, members)


def to_json(obj, value):
    """A value in the JSON encoding of ASN.1 (ITU-T X.697), as a view takes it."""
    kind = obj.TYPE
    if kind == "NULL":
        return None
    if kind in ("BOOLEAN", "INTEGER", "ENUMERATED", *STRINGS):
        return value
    if kind == "OBJECT IDENTIFIER":
        return ".".join(str(arc) for arc in value)
    if kind == "BIT STRING":
        number, bits = value
        octets = (number << (-bits % 8)).to_bytes((bits + 7) // 8, "big").hex().upper()
        lower, upper, extensible = bounds(obj._const_sz)
        return octets if lower == upper and not extensible else {"value": octets, "length": bits}
    if kind == "OCTET STRING":
        if isinstance(value, tuple):
            # The octets hold a type: its value.
            return to_json(obj._const_cont, value[1])
        return value.hex().upper()
    if kind == "CHOICE":
        alternative, inner = value
        return {alternative: member_to_json(obj._cont[alternative], inner, None)}
    if kind == "SEQUENCE OF":
        return [member_to_json(obj._cont, entry, None) for entry in value]
    if kind == "SEQUENCE":
        # The object of the set that the identifier of an IE selects.
        row = None
        for member, component in obj._cont.items():
            table = getattr(component, "_const_tab", None)
            if table is not None and getattr(component, "_const_tab_at", None) is None:
                key = component._const_tab_id
                rows = [row for row in table_rows(table) if row[key] == value[member]]
                row = rows[0] if rows else None
        return {
            member: member_to_json(obj._cont[member], inner, row) for member, inner in value.items()
        }
    raise ValueError(f"{kind} has no JSON here")


def member_to_json(component, value, row):
    if component.TYPE != "OPEN_TYPE":
        return to_json(component, value)
    name, inner = value
    if isinstance(inner, bytes):
        return inner.hex().upper()
    key = component._const_tab_id
    if row is None:
        # The message of a PDU, by its name.
        rows = table_rows(component._const_tab)
        row = next(row for row in rows if key in row and asn_name(row[key]) == name)
    return to_json(row[key], inner)


def view(protocol, module, value):
    """The view of a PDU: its procedure code, its direction, its criticality and its
    message."""
    alternative, members = value
    obj = pdu_type(protocol, module)._cont[alternative]
    return {
        "procedure_code": members["procedureCode"],
        "direction": alternative[0].upper() + alternative[1:],
        "criticality": members["criticality"],
        "message": member_to_json(obj._cont["value"], members["value"], None),
    }


def summary(keys):
    """What the vectors of the passes hold."""
    count = lambda kind: sum(key.startswith(kind + ":") for key in keys)
    return [
        f"# {count('I')} IEs of object sets, {count('S')} members that may be absent and are",
        f"# there and {count('A')} that are absent, {count('C')} alternatives of CHOICE types,",
        f"# {count('E')} values of ENUMERATED types, {count('X')} numbers and {count('Z')} sizes after the",
        f"# root of an extensible constraint, and {count('M')} strings of the largest size of their type.",
    ]


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--check", action="store_true", help="compare, and write nothing")
    parser.add_argument("--values", help="write the view of each vector into this file")
    arguments = parser.parse_args()
    protocol = next(name for name in PROTOCOLS if pathlib.Path(name).is_dir())
    with tempfile.TemporaryDirectory() as directory:
        module = compile_modules(protocol, pathlib.Path(directory))
        pdu = pdu_type(protocol, module)
        files = {"messages.tsv": minimal(protocol, module)}
        keys, lines, views = set(), [], []
        for made in (covering(protocol, module, keys), sized(protocol, module)):
            for name, value in made:
                pdu.set_val(value)
                code, alternative = value[1]["procedureCode"], ALTERNATIVES.index(value[0])
                lines.append(f"{name}\t{pdu.to_aper().hex()}\t{code}\t{alternative}")
                if arguments.values:
                    views.append(f"{name}\t{json.dumps(view(protocol, module, value))}")
        header = [
            f"# {PROTOCOLS[protocol]['specification']}; pycrate 0.8.1 vectors with content, which",
            "# tests/fixtures/generate.py writes. The vectors of the passes hold",
            *summary(keys),
            "# Message/variant\twire bytes\tprocedure code\tPDU alternative",
        ]
        files["vectors.tsv"] = "\n".join(header + lines) + "\n"
    failed = False
    for name, text in files.items():
        path = pathlib.Path("tests/fixtures") / name
        if arguments.check:
            same = path.exists() and path.read_text() == text
            failed |= not same
            print(f"{path}: {'as it is written again' if same else 'NOT what is written again'}")
        else:
            path.write_text(text)
            print(f"{path}: {len(text.splitlines())} lines, {len(text)} octets")
    if arguments.values:
        pathlib.Path(arguments.values).write_text("\n".join(views) + "\n")
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
