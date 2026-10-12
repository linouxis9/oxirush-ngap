#!/usr/bin/env python3
"""Take the ASN.1 modules of the bindings out of the archive of their specification.

    python3 build/extract_asn1.py 38413-j40.zip

Run it from the directory of the crate, with the archive as 3GPP publishes it:

    https://www.3gpp.org/ftp/Specs/archive/38_series/38.413/38413-j40.zip
    https://www.3gpp.org/ftp/Specs/archive/36_series/36.413/36413-j20.zip

The archive holds one document. Each module is the paragraphs of that document between
a line `-- ASN1START` and a line `-- ASN1STOP`. `build/asn1.sha256` lists the modules
that the bindings are generated from, each with its SHA-256: this script writes each of
them where the list says, and fails if one does not have its digest. It needs Python 3
alone.

A module is written as the bindings were generated from it:

- a first line that says where it comes from;
- a tab for each `w:tab` of a paragraph, among them the tab stops that the paragraph
  defines, which are white space before its text;
- a space for a no-break space, and no white space at the end of a line;
- no empty line at its end.
"""

import hashlib
import io
import pathlib
import re
import sys
import xml.etree.ElementTree as ET
import zipfile

WORD = "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}"


def text(paragraph):
    """The text of a paragraph of the document."""
    parts = []
    for element in paragraph.iter():
        if element.tag == WORD + "t":
            parts.append(element.text or "")
        elif element.tag == WORD + "tab":
            parts.append("\t")
    return "".join(parts).replace(" ", " ").rstrip()


def modules(archive):
    """The lines of each module of the document of an archive, by the name of the module."""
    with zipfile.ZipFile(archive) as outer:
        (name,) = [name for name in outer.namelist() if name.endswith(".docx")]
        with zipfile.ZipFile(io.BytesIO(outer.read(name))) as document:
            body = ET.fromstring(document.read("word/document.xml"))
    found, lines = {}, None
    for paragraph in body.iter(WORD + "p"):
        line = text(paragraph)
        if line == "-- ASN1START":
            lines = []
        elif line == "-- ASN1STOP":
            while lines and not lines[-1]:
                lines.pop()
            # The name of a module starts the first line that is no comment.
            first = next(line for line in lines if line and not line.startswith("--"))
            found[re.match(r"[A-Za-z0-9-]+", first).group()] = lines
            lines = None
        elif lines is not None:
            lines.append(line)
    return found


def main():
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    archive = pathlib.Path(sys.argv[1])
    number = re.fullmatch(r"(\d\d)(\d\d\d)-\w+\.zip", archive.name)
    if number is None:
        sys.exit(f"{archive.name} is not the name of an archive, as 38413-j40.zip")
    series, specification = number.groups()
    url = (
        f"https://www.3gpp.org/ftp/Specs/archive/{series}_series/"
        f"{series}.{specification}/{archive.name}"
    )
    found = modules(archive)
    failed = False
    for entry in pathlib.Path("build/asn1.sha256").read_text().splitlines():
        digest, path = entry.split()
        path = pathlib.Path(path)
        if path.stem not in found:
            sys.exit(f"{archive.name} has no module {path.stem}")
        module = f"-- {path.name}, extracted from {url}\n" + "\n".join(found[path.stem]) + "\n"
        octets = module.encode()
        path.parent.mkdir(exist_ok=True)
        path.write_bytes(octets)
        same = hashlib.sha256(octets).hexdigest() == digest
        failed |= not same
        print(f"{path}: {'as recorded' if same else 'NOT the module that is recorded'}")
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
