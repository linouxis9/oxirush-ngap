#!/usr/bin/env python3
"""Write the revisions of build/revisions wherever the repository takes oxirush-aper
or oxirush-utils from its repository.

    python3 build/pin.py

Neither crate is on crates.io yet. To move to another revision of one, write the name
of its commit in build/revisions, run this, regenerate the bindings as the README
says, and commit the three together: tests/revisions.rs fails while a place names
another revision, and the Reproduce workflow while the generator at the revision of
oxirush-aper writes other bindings.
"""
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parent.parent
PLACES = [
    ".github/workflows/ci.yml",
    ".github/workflows/reproduce.yml",
    "README.md",
    "benches/Cargo.toml",
    "fuzz/Cargo.toml",
    "oracle/Cargo.toml",
]

recorded = {}
for line in (ROOT / "build/revisions").read_text().splitlines():
    if line and not line.startswith("#"):
        name, revision = line.split()
        recorded[name] = revision

for place in PLACES:
    path = ROOT / place
    lines = path.read_text().split("\n")
    for number, line in enumerate(lines):
        for name, revision in recorded.items():
            if f'"https://github.com/linouxis9/{name}"' not in line:
                continue
            # In a manifest the revision is on the line, and for cargo's `--config` it
            # is the next option, on the next line.
            for at in (number, number + 1):
                lines[at], written = re.subn(
                    r'(\brev ?= ?")[0-9a-f]{40}(")', rf"\g<1>{revision}\g<2>", lines[at]
                )
                if written:
                    break
            else:
                raise SystemExit(f"{place}:{number + 1}: {name} has no revision to write")
    path.write_text("\n".join(lines))
print("\n".join(f"{name} {revision}" for name, revision in recorded.items()))
