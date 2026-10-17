#!/usr/bin/env python3
# OxiRush
# Copyright 2025-2026 Valentin D'Emmanuele
#
# Licensed under the Apache License, Version 2.0 (the "License"); you may not use this
# file except in compliance with the License. You may obtain a copy of the License at
# http://www.apache.org/licenses/LICENSE-2.0. Unless required by applicable law or agreed
# to in writing, software distributed under the License is distributed on an "AS IS"
# BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
# License for the specific language governing permissions and limitations under the
# License.

"""The comparison of the README: the crate and other implementations of its protocol,
each decoding and encoding the PDUs of pdus.tsv beside this file.

    python3 benches/compare/compare.py [--work DIR] [--core N] [--perf COMMAND]

Run it from the root of the repository. It fetches each other implementation at the
version that is pinned here, builds it in the work directory (benches/compare/work by
default, which git ignores), and prints the tables of the README, which it also writes
to results.md in the work directory, with every figure in results.json:

- the crate, and rasn 0.29.0 in the bindings of oracle/ (rasn-compiler 0.16, with the
  SEQUENCE codec of oracle/src/per.rs in the place of rasn's derive);
- hampi 0.7.2, its asn1-compiler on the ASN.1 of the specification, which
  build/extract_asn1.py takes out of the archive of 3GPP;
- the asn1c code of Open5GS v2.8.0, built with meson from its repository;
- the ngap package v1.2.0 of free5GC, for NGAP (free5GC has no S1AP);
- pycrate 0.8.1, in a virtual environment.

It needs cargo, git, curl, a C compiler, meson, ninja and the libraries that Open5GS is
built with, Go 1.26 for free5GC, and Python 3 with venv. Each run of an operation is on
one core (--core, 2 by default), with --perf for the cycles and the instructions: `perf`
by default, or `sudo -n perf` where perf_event_paranoid keeps them from a user, in which
case the program runs as the user again under perf. The other implementations are not
copied into the repository: Open5GS is under the AGPL.
"""

import argparse
import json
import os
import pathlib
import re
import shutil
import subprocess
import sys

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parent.parent
OPEN5GS = "v2.8.0"
PYCRATE = "0.8.1"
ARCHIVES = {
    "ngap": "https://www.3gpp.org/ftp/Specs/archive/38_series/38.413/38413-j40.zip",
    "s1ap": "https://www.3gpp.org/ftp/Specs/archive/36_series/36.413/36413-j20.zip",
}
# Each codec, with what it is called in the tables.
NAMES = {
    "ours": "this crate",
    "rasn": "rasn 0.29",
    "hampi": "hampi 0.7.2",
    "asn1c": "asn1c (Open5GS 2.8.0)",
    "free5gc": "free5GC ngap 1.2.0",
    "pycrate": "pycrate 0.8.1",
}


def run(command, cwd=ROOT, env=None, capture=False):
    """Run a command of the setup, which stops the comparison when it fails."""
    print("+", " ".join(str(part) for part in command), file=sys.stderr)
    done = subprocess.run(command, cwd=cwd, env=env, check=True, text=True,
                          stdout=subprocess.PIPE if capture else None)
    return done.stdout


class Codec:
    """An implementation that compare.py runs: the command of one operation."""

    def __init__(self, name, command, operations, env=None):
        self.name, self.command, self.operations, self.env = name, command, operations, env

    def argv(self, operation, iterations, octets):
        return [*self.command, operation, str(iterations), octets]


def build(work, protocol):
    """Fetch and build each implementation: the codecs, by name."""
    target = work / "target"
    environment = dict(os.environ, CARGO_TARGET_DIR=str(target))
    run(["cargo", "build", "--release", "--manifest-path", "benches/Cargo.toml",
         "--bin", "compare"], env=environment)
    compare = str(target / "release" / "compare")
    codecs = {
        "ours": Codec("ours", [compare, "ours"], ["envelope", "decode", "encode"]),
        "rasn": Codec("rasn", [compare, "rasn"], ["decode", "encode"]),
    }

    # hampi, on the ASN.1 of the archive of the specification.
    archive = work / pathlib.Path(ARCHIVES[protocol]).name
    if not archive.exists():
        run(["curl", "-sS", "--fail", "-A", "Mozilla/5.0", "-o", str(archive),
             ARCHIVES[protocol]])
    run([sys.executable, "build/extract_asn1.py", str(archive)])
    hampi = work / f"target-hampi-{protocol}"
    environment = dict(os.environ, CARGO_TARGET_DIR=str(hampi), PROTOCOL=protocol,
                       ASN1=str(ROOT / protocol))
    run(["cargo", "build", "--release", "--manifest-path",
         str(HERE / "hampi" / "Cargo.toml")], env=environment)
    codecs["hampi"] = Codec("hampi", [str(hampi / "release" / "oxirush-compare-hampi")],
                            ["decode", "encode"])

    # The asn1c code of Open5GS, at its tag.
    source = work / "open5gs"
    if not source.exists():
        run(["git", "clone", "--quiet", "--depth", "1", "--branch", OPEN5GS,
             "https://github.com/open5gs/open5gs", str(source)])
    described = run(["git", "-C", str(source), "describe", "--tags"], capture=True).strip()
    assert described == OPEN5GS, f"{source} is {described}, not {OPEN5GS}"
    built = source / "build"
    if not built.exists():
        run(["meson", "setup", "--buildtype=release", str(built)], cwd=source)
    run(["meson", "compile", "-C", str(built), f"ogsasn1c-{protocol}"], cwd=source)
    program = work / f"asn1c-{protocol}"
    includes = [source / "lib" / "asn1c" / protocol, source / "lib" / "asn1c" / "common",
                source / "lib" / "asn1c" / "util", source / "lib" / "core", source / "lib",
                built / "lib" / "core", built / "lib", built]
    libraries = [built / "lib" / "asn1c" / protocol, built / "lib" / "asn1c" / "common",
                 built / "lib" / "asn1c" / "util", built / "lib" / "core"]
    run(["cc", "-O2", f"-DPROTOCOL_{protocol.upper()}", str(HERE / "asn1c.c"),
         *[f"-I{path}" for path in includes], *[f"-L{path}" for path in libraries],
         f"-logsasn1c-{protocol}", "-logsasn1c-util", "-logsasn1c-common", "-logscore",
         "-o", str(program)])
    path = ":".join(str(path) for path in libraries)
    codecs["asn1c"] = Codec("asn1c", [str(program)], ["decode", "encode"],
                            env=dict(os.environ, LD_LIBRARY_PATH=path))

    # The ngap package of free5GC, at the version of go.mod.
    if protocol == "ngap":
        program = work / "free5gc"
        run(["go", "build", "-buildvcs=false", "-o", str(program), "."],
            cwd=HERE / "free5gc")
        codecs["free5gc"] = Codec("free5gc", [str(program)], ["decode", "encode"])

    # pycrate, in a virtual environment.
    venv = work / "venv"
    if not venv.exists():
        run([sys.executable, "-m", "venv", str(venv)])
    run([str(venv / "bin" / "pip"), "install", "--quiet", f"pycrate=={PYCRATE}"])
    codecs["pycrate"] = Codec("pycrate", [str(venv / "bin" / "python"), "-I",
                                          str(HERE / "pycrate.py"), protocol],
                              ["decode", "encode"])
    return codecs


def measure(codec, operation, octets, core, perf):
    """The figures of one operation of a codec on one PDU: a call in ns, cycles and
    instructions, the allocations and the octets that it asks for, and the most memory
    that the program holds."""
    def once(iterations, counted):
        argv = codec.argv(operation, iterations, octets)
        argv = ["taskset", "-c", str(core), *argv]
        if counted:
            # As the user again where perf runs as root, which sudo runs without the
            # variables of the environment that the program needs.
            if perf[0] == "sudo":
                variables = [f"{key}={value}" for key, value in (codec.env or {}).items()
                             if key == "LD_LIBRARY_PATH"]
                argv = ["setpriv", f"--reuid={os.getuid()}", f"--regid={os.getgid()}",
                        "--clear-groups", "env", *variables, *argv]
            argv = [*perf, "stat", "-x,", "-e", "cycles,instructions", "--", *argv]
        done = subprocess.run(argv, env=codec.env, text=True, capture_output=True)
        if done.returncode != 0:
            raise RuntimeError(f"{codec.name} {operation}: {done.stderr.strip()[-400:]}")
        stdout, stderr = done.stdout, done.stderr
        figures = json.loads(stdout.strip().splitlines()[-1])
        counts = {}
        for line in stderr.splitlines():
            fields = line.split(",")
            if len(fields) > 2 and fields[2] in ("cycles", "instructions"):
                counts[fields[2]] = float(fields[0]) if fields[0][:1].isdigit() else None
        return figures, counts

    # About a fifth of a second for each of the four times that a program runs the call.
    probe, _ = once(100, False)
    iterations = max(100, int(2e8 / max(probe["ns"], 1.0)))
    result, _ = once(iterations, False)
    if perf:
        try:
            _, first = once(iterations, True)
            _, second = once(2 * iterations, True)
            # The second run makes four times the iterations more calls.
            for name in ("cycles", "instructions"):
                if first.get(name) is not None and second.get(name) is not None:
                    result[name] = (second[name] - first[name]) / (4 * iterations)
        except (RuntimeError, OSError) as error:
            print(f"perf: {error}", file=sys.stderr)
    return result


def ratio(theirs, ours):
    return f"{theirs / ours:.1f}x" if theirs and ours else ""


def tables(protocol, pdus, results, codecs):
    """The tables of the README, in Markdown."""
    others = [name for name in NAMES if name in codecs and name != "ours"]
    lines = []
    for operation, title in (("decode", "Decode, every IE of its type"),
                             ("encode", "Encode, from the typed values")):
        lines.append(f"{title}, ns per call (the ratio to this crate):\n")
        header = ["PDU (octets)"]
        if operation == "decode":
            header.append("this crate, the envelope alone")
        header += [NAMES["ours"], *[NAMES[name] for name in others]]
        lines.append("| " + " | ".join(header) + " |")
        lines.append("|" + "---|" * len(header))
        for name, octets in pdus:
            ours = results[name]["ours"][operation]["ns"]
            row = [f"{name} ({len(octets) // 2})"]
            if operation == "decode":
                row.append(f"{results[name]['ours']['envelope']['ns']:,.0f}")
            row.append(f"{ours:,.0f}")
            for other in others:
                figures = results[name].get(other, {}).get(operation)
                row.append(f"{figures['ns']:,.0f} ({ratio(figures['ns'], ours)})"
                           if figures else "does not take it")
            lines.append("| " + " | ".join(row) + " |")
        lines.append("")
    # Every figure for one PDU.
    name = max(pdus, key=lambda pdu: len(pdu[1]))[0]
    lines.append(f"Every figure for {name}, per call:\n")
    header = ["", "decode ns", "cycles", "instructions", "allocations", "octets allocated",
              "encode ns", "cycles", "instructions", "allocations", "octets allocated",
              "most memory held, KiB"]
    lines.append("| " + " | ".join(header) + " |")
    lines.append("|" + "---|" * len(header))
    ours = results[name]["ours"]

    def cell(figures, key, base=None):
        value = figures.get(key)
        if value is None:
            return ""
        text = f"{value:,.0f}"
        if base is not None and base.get(key):
            text += f" ({ratio(value, base[key])})"
        return text

    for other in ["ours", *others]:
        figures = results[name].get(other)
        if not figures:
            continue
        row = [NAMES[other]]
        for operation in ("decode", "encode"):
            done, base = figures[operation], (None if other == "ours" else ours[operation])
            row += [cell(done, "ns", base), cell(done, "cycles", base),
                    cell(done, "instructions", base), cell(done, "allocations"),
                    cell(done, "bytes")]
        row.append(cell(figures["decode"], "held"))
        lines.append("| " + " | ".join(row) + " |")
    return "\n".join(lines) + "\n"


def load():
    """The load average of the last minute."""
    return pathlib.Path("/proc/loadavg").read_text().split()[0]


def machine():
    """The model of the processor."""
    for line in pathlib.Path("/proc/cpuinfo").read_text().splitlines():
        if line.startswith(("model name", "Model")):
            return line.split(":", 1)[1].strip()
    return os.uname().machine


def main():
    arguments = argparse.ArgumentParser()
    arguments.add_argument("--work", default=str(HERE / "work"))
    arguments.add_argument("--core", type=int, default=2)
    arguments.add_argument("--perf", default="perf",
                           help="the perf command, or '' for none")
    arguments = arguments.parse_args()
    manifest = (ROOT / "Cargo.toml").read_text()
    protocol = re.search(r'^name = "oxirush-(\w+)"', manifest, re.M).group(1)
    work = pathlib.Path(arguments.work)
    work.mkdir(parents=True, exist_ok=True)
    perf = arguments.perf.split()
    if perf and shutil.which(perf[-1]) is None:
        perf = []
    pdus = [line.split("\t") for line in (HERE / "pdus.tsv").read_text().splitlines()
            if line and not line.startswith("#")]
    codecs = build(work, protocol)
    before = load()
    results = {}
    for name, octets in pdus:
        results[name] = {}
        for codec in codecs.values():
            results[name][codec.name] = {}
            for operation in codec.operations:
                try:
                    figures = measure(codec, operation, octets, arguments.core, perf)
                except RuntimeError as error:
                    print(error, file=sys.stderr)
                    continue
                results[name][codec.name][operation] = figures
                print(f"{name} {codec.name} {operation}: {figures}", file=sys.stderr)
    (work / "results.json").write_text(json.dumps(results, indent=1))
    text = tables(protocol, pdus, results, codecs)
    text += (f"\nOn {machine()}, core {arguments.core}, with a load average of {before} "
             f"before and {load()} after.\n")
    (work / "results.md").write_text(text)
    print(text)


main()
