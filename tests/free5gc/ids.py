#!/usr/bin/env python3
"""The IE of each field of the messages of free5GC's NGAP library, from its source, for
tests/free5gc/main.go: a row a field, by tabs, the type of the message, the field and the
id of its IE.

    python3 tests/free5gc/ids.py DIR OUT.tsv

DIR is the directory of the module github.com/free5gc/ngap, as `go mod download -json`
gives it. The MarshalBinary of a message writes each field with the constant of its id,
which ie/protocol_ie.go gives: by the field's WriteIE, or by ie.WriteOctetStringIE for a
field of an OCTET STRING.
"""
import os
import re
import sys

if len(sys.argv) != 3:
    sys.exit(__doc__)
module = sys.argv[1]
constants = dict(re.findall(r'^\s*(ProtocolIEID\w+)\s+int64\s*=\s*(\d+)', open(os.path.join(module, 'ie', 'protocol_ie.go'), encoding='utf-8').read(), re.M))
rows = []
directory = os.path.join(module, 'message')
for name in sorted(os.listdir(directory)):
    if not name.endswith('.go') or name.endswith('_test.go'):
        continue
    source = open(os.path.join(directory, name), encoding='utf-8').read()
    for kind, body in re.findall(r'^func \(x \*(\w+)\) MarshalBinary\(\).*?\n(.*?)^}', source, re.M | re.S):
        for field, constant in re.findall(r'x\.(\w+)\.WriteIE\(pdOpenType,\s*ie\.ProtocolIEID\{Value: ie\.(ProtocolIEID\w+)\}', body):
            rows.append((kind, field, constants[constant]))
        # A field of an OCTET STRING, which ie.WriteOctetStringIE writes
        for constant, field in re.findall(r'ie\.WriteOctetStringIE\(pdOpenType,\s*ie\.ProtocolIEID\{Value: ie\.(ProtocolIEID\w+)\},\s*ie\.ProtocolIECriticality\{[^}]*\},\s*\*\(x\.(\w+)\)\)', body):
            rows.append((kind, field, constants[constant]))
if not rows:
    sys.exit(f'no IE in {directory}')
with open(sys.argv[2], 'w', encoding='utf-8') as out:
    for row in rows:
        out.write('\t'.join(row) + '\n')
print(len(rows), 'IEs of', len({row[0] for row in rows}), 'messages')
