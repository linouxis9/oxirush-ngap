#!/usr/bin/env python3
"""Compare what the crate and free5GC's NGAP library read of the same PDUs, for
tests/free5gc.rs.

    python3 -I tests/free5gc/compare.py --crate crate.jsonl --free5gc free5gc.jsonl \\
        --known tests/free5gc/known.tsv --output DIR

A line of each file is a PDU, in the same order. The crate reads a PDU when it decodes it,
its container of IEs and the value of each IE (check() finds no IE that does not decode),
or, of a message of no container of protocol IEs, its type;
of a PDU that both read, the type of the PDU and the procedure code, and the value of each
IE by its id, are compared. Each difference is matched with the rows of known.tsv; one that
no row explains fails the run, as a run that compares nothing does. DIR has report.json,
each PDU with its differences, and summary.txt.
"""
import argparse
import collections
import json
import os
import re
import sys

UNDECODED = ('UndecodableIe',)


def refused(ours):
    """Why the crate does not read the PDU, or None."""
    if 'error' in ours:
        return ours['error']
    for deviation in ours.get('check', []):
        if deviation['kind'] in UNDECODED:
            return deviation['text']
    return None


def differences(ours, theirs):
    """What the two read otherwise of one PDU, as (kind, text) pairs."""
    why = refused(ours)
    if why is not None and not theirs['accepted']:
        return []
    if why is not None:
        return [('crate-refuses', f'the crate refuses it: {why}')]
    if not theirs['accepted']:
        if panicked(theirs['error']):
            return []
        return [('free5gc-refuses', f'{ours["message"]}: free5GC refuses it: {theirs["error"]}')]
    if (ours['pdu'], ours['procedure']) != (theirs['pdu'], theirs['procedure']):
        return [('message', f'message: crate {ours["message"]} ({ours["pdu"]}, {ours["procedure"]}), free5GC {theirs["type"]} ({theirs["pdu"]}, {theirs["procedure"]})')]
    found = []
    name = ours['message']
    if 'container' in ours:
        # A message of no container of protocol IEs: its type and procedure code alone
        return found
    mine = collections.defaultdict(list)
    for ie in ours.get('ies', []):
        mine[ie['id']].append(ie['value'])
    theirs_by_id = {ie['id']: ie for ie in theirs.get('ies', [])}
    for id_, values in mine.items():
        ie = theirs_by_id.get(id_)
        if ie is None:
            found.append(('ie', f'{name} IE {id_}: only the crate has it'))
        elif ie.get('value') != values[0]:
            value = ie.get('value')
            last = ''
            if value and len(value) == len(values[0]) and value[:-2] == values[0][:-2]:
                last = ' (in the last octet alone)'
            found.append(('value', f'{name} IE {id_} ({ie["field"]}): crate {values[0]}, free5GC {value or ie.get("error")}{last}'))
    for id_, ie in theirs_by_id.items():
        if id_ not in mine:
            found.append(('ie', f'{name} IE {id_} ({ie["field"]}): only free5GC has it'))
    return found


def panicked(error):
    """Whether free5GC refused the octets by a panic, which the program of tests/free5gc/
    catches and gives as its error: a refusal, which needs no row."""
    return 'panic: ' in error


def pattern(text):
    """The class of a difference: its numbers and octets as #."""
    return re.sub(r'\b[0-9a-f]{2,}\b|\d+', '#', text)


class Known:
    def __init__(self, path):
        self.rows = []
        for line in open(path, encoding='utf-8'):
            if line.startswith('#') or not line.strip():
                continue
            kind, where, verdict, reference = line.rstrip('\n').split('\t')
            self.rows.append((kind, re.compile(where), verdict, reference))

    def explain(self, kind, text):
        for row in self.rows:
            if row[0] in ('*', kind) and row[1].search(text):
                return row
        return None


def main():
    parser = argparse.ArgumentParser(description=__doc__.split('\n\n')[0])
    for name in ('crate', 'free5gc', 'known', 'output'):
        parser.add_argument('--' + name, required=True)
    args = parser.parse_args()
    known = Known(args.known)
    os.makedirs(args.output, exist_ok=True)
    counts = collections.Counter()
    explained = collections.Counter()
    unknown = collections.OrderedDict()
    report = []
    with open(args.crate, encoding='utf-8') as crate, open(args.free5gc, encoding='utf-8') as free5gc:
        for line, other in zip(crate, free5gc, strict=True):
            ours, theirs = json.loads(line), json.loads(other)
            if ours['octets'] != theirs['octets']:
                sys.exit(f'the two files differ in their PDUs at {ours["octets"]}')
            found = differences(ours, theirs)
            if refused(ours) is not None and not theirs['accepted']:
                counts['both-refuse'] += 1
            elif not found:
                counts['agree'] += 1
            else:
                counts[found[0][0] if found[0][0].endswith('refuses') else 'differs'] += 1
            entry = {'octets': ours['octets'], 'differences': []}
            for kind, text in found:
                row = known.explain(kind, text)
                entry['differences'].append({'kind': kind, 'text': text, 'known': row and row[3]})
                if row:
                    explained[f'{row[2]}: {row[3]}'] += 1
                else:
                    unknown.setdefault((kind, pattern(text)), (text, ours['octets']))
            report.append(entry)
    compared = sum(counts.values())
    lines = [f'free5GC ngap v1.2.0, {compared} PDUs compared']
    for kind in ('agree', 'differs', 'both-refuse', 'crate-refuses', 'free5gc-refuses'):
        lines.append(f'{counts[kind]:7}  {kind}')
    lines.append('explained differences:')
    lines += [f'{n:7}  {what}' for what, n in explained.most_common()]
    lines.append(f'{len(unknown)} differences without a row of known.tsv')
    lines += [f'  {kind}\t{text}\t(first: {octets})' for (kind, _), (text, octets) in unknown.items()]
    summary = '\n'.join(lines) + '\n'
    with open(os.path.join(args.output, 'report.json'), 'w', encoding='utf-8') as f:
        json.dump(report, f, indent=1)
    print(summary, end='')
    if compared == 0:
        sys.exit('nothing was compared')
    with open(os.path.join(args.output, 'summary.txt'), 'w', encoding='utf-8') as f:
        f.write(summary)
    sys.exit(1 if unknown else 0)


if __name__ == '__main__':
    main()
