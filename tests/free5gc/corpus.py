#!/usr/bin/env python3
"""The corpus of the test `free5gc`: a PDU a line, its name and its octets in hexadecimal,
each PDU once, from the corpus of the comparison with Wireshark, the fixtures, and the files
of each DIR.

    python3 tests/free5gc/corpus.py OUT.tsv [DIR...]

A row of a fixture is a PDU by the first field of hexadecimal after its name; a DIR is a
directory of PDUs, one a file, as the corpus of a fuzz target is.
"""
import os
import re
import sys

if len(sys.argv) < 2:
    sys.exit(__doc__)
HERE = os.path.dirname(os.path.abspath(__file__))
out, directories = sys.argv[1], sys.argv[2:]
seen, rows = set(), []


def add(name, octets):
    if octets and octets not in seen:
        seen.add(octets)
        rows.append((name, octets.hex()))


for name in ('../wireshark/corpus.tsv', '../fixtures/messages.tsv', '../fixtures/vectors.tsv',
             '../fixtures/captured.tsv', '../fixtures/later.tsv', '../fixtures/fuzzed.tsv'):
    path = os.path.normpath(os.path.join(HERE, name))
    for line in open(path, encoding='utf-8'):
        if line.startswith('#') or not line.strip():
            continue
        fields = line.rstrip('\n').split('\t')
        octets = next((f for f in fields[1:] if len(f) % 2 == 0 and re.fullmatch(r'[0-9a-fA-F]+', f)), None)
        if octets:
            add(f'{os.path.basename(path)}/{fields[0]}', bytes.fromhex(octets))
for directory in directories:
    label = os.path.normpath(directory)
    for name in sorted(os.listdir(directory)):
        with open(os.path.join(directory, name), 'rb') as f:
            add(f'{label}/{name}', f.read())
with open(out, 'w', encoding='utf-8') as f:
    for name, octets in rows:
        f.write(f'{name}\t{octets}\n')
print(len(rows), 'PDUs')
