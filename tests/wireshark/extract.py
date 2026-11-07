#!/usr/bin/env python3
"""Collect PDUs of NGAP (SCTP payload protocol 60) or S1AP (18) for the Wireshark differential.

Sources, any number of each:
  --pcap PATH          a capture, or a directory searched for *.pcap and *.pcapng: the payloads
                       of the SCTP DATA chunks of the protocol, read by tshark from standard input
  --tsv PATH[:N]       rows of a TSV whose column N, or else its last column of hexadecimal, is a
                       PDU (comments with #)

Rows out, separated by tabs: source, octets. A PDU appears once, by its octets; `source` is
where it was first seen.
"""

import argparse
import os
import subprocess
import sys
from pathlib import Path

PPID = {"ngap": "60", "s1ap": "18"}


def captures(path):
    if path.is_dir():
        for root, _, files in os.walk(path):
            for name in sorted(files):
                if name.endswith((".pcap", ".pcapng")):
                    yield Path(root) / name
    else:
        yield path


def from_capture(protocol, path):
    # The capture goes through a pipe: tshark may refuse a path where the file is readable.
    # With the AP dissectors off, the payload of a chunk is its data.
    result = subprocess.run(
        ["tshark", "-n", "-r", "-", "--disable-protocol", "ngap,s1ap", "-T", "fields",
         "-E", "occurrence=a", "-E", "aggregator=|", "-e", "frame.number",
         "-e", "sctp.data_payload_proto_id", "-e", "data.data"],
        input=path.read_bytes(), capture_output=True)
    if result.returncode != 0:
        print(f"{path}: tshark failed: {result.stderr.decode().strip()}", file=sys.stderr)
        return
    for line in result.stdout.decode().splitlines():
        frame, ppids, payloads = (line.split("\t") + ["", ""])[:3]
        for ppid, payload in zip(ppids.split("|"), payloads.split("|")):
            if ppid == PPID[protocol] and payload:
                yield f"{path}#{frame}", bytes.fromhex(payload)


def from_tsv(spec):
    path, _, index = spec.partition(":")
    path = Path(path)
    for number, line in enumerate(path.read_text().splitlines(), 1):
        if not line.strip() or line.startswith("#"):
            continue
        columns = line.split("\t")
        if index:
            columns = columns[int(index) - 1:int(index)]
        for column in reversed(columns):
            column = column.strip()
            if column and len(column) % 2 == 0 and all(c in "0123456789abcdefABCDEF" for c in column):
                yield f"{path}:{number}", bytes.fromhex(column)
                break


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--protocol", choices=PPID, required=True)
    parser.add_argument("--pcap", type=Path, action="append", default=[])
    parser.add_argument("--tsv", action="append", default=[])
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    seen = {}
    total = 0
    for path in args.pcap:
        for capture in captures(path):
            for source, octets in from_capture(args.protocol, capture):
                total += 1
                seen.setdefault(octets, source)
    for spec in args.tsv:
        for source, octets in from_tsv(spec):
            total += 1
            seen.setdefault(octets, source)
    with args.output.open("w") as output:
        output.write("# source\toctets\n")
        for octets, source in seen.items():
            output.write(f"{source}\t{octets.hex()}\n")
        output.flush()
        os.fsync(output.fileno())
    print(f"{args.protocol}: {total} seen, {len(seen)} distinct")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
