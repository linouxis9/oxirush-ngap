#!/usr/bin/env python3
"""The time and the peak memory of a clean build of the crate.

    python3 benches/build_cost.py                          # cargo build
    python3 benches/build_cost.py --features inspect
    python3 benches/build_cost.py +1.88.0 --features inspect --release
    CARGO_PROFILE_DEV_DEBUG=0 python3 benches/build_cost.py --features inspect

The arguments are those of `cargo build`, after the toolchain when one is
named. The files that Git tracks are copied to a temporary directory, under
`TMPDIR`, and built there with a target directory of its own: no workspace
that the crate is in takes part, and nothing is compiled already. The
dependencies are fetched before the build is timed, with the `--config KEY=VALUE`
of the arguments: until oxirush-aper and oxirush-utils are on crates.io, one
for each says where it is.

    APER='patch.crates-io.oxirush-aper.git="https://github.com/linouxis9/oxirush-aper"'
    UTILS='patch.crates-io.oxirush-utils.git="https://github.com/linouxis9/oxirush-utils"'
    python3 benches/build_cost.py --config "$APER" --config "$UTILS" --features inspect

The peak is the summed resident memory of cargo and of the compilers that it
runs, sampled four times a second from `/proc`, so it needs Linux. On a
machine that does something else the time is longer and the peak is the same.
"""
import os
import shutil
import subprocess
import sys
import tempfile
import time

CRATE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def descendants(root):
    """The process `root` and those that it started, by their identifiers."""
    children = {}
    for entry in os.listdir("/proc"):
        if entry.isdigit():
            try:
                with open(f"/proc/{entry}/stat") as stat:
                    parent = int(stat.read().rsplit(")", 1)[1].split()[1])
                children.setdefault(parent, []).append(int(entry))
            except OSError:
                pass
    found, pending = [], [root]
    while pending:
        process = pending.pop()
        found.append(process)
        pending += children.get(process, [])
    return found


def resident(process):
    """The resident memory of a process, in kibibytes."""
    try:
        with open(f"/proc/{process}/status") as status:
            for line in status:
                if line.startswith("VmRSS:"):
                    return int(line.split()[1])
    except OSError:
        pass
    return 0


def main():
    arguments = sys.argv[1:]
    toolchain = [arguments.pop(0)] if arguments and arguments[0].startswith("+") else []
    cargo = ["cargo", *toolchain]
    with tempfile.TemporaryDirectory(prefix="build-cost-") as directory:
        copy = os.path.join(directory, "crate")
        tracked = subprocess.run(
            ["git", "-C", CRATE, "ls-files", "-z"], check=True, capture_output=True
        ).stdout.decode().split("\0")
        for path in filter(None, tracked):
            target = os.path.join(copy, path)
            os.makedirs(os.path.dirname(target), exist_ok=True)
            shutil.copy2(os.path.join(CRATE, path), target)
        environment = dict(os.environ, CARGO_TARGET_DIR=os.path.join(directory, "target"))
        # A configuration of cargo holds for the fetch as for the build.
        pairs = zip(arguments, arguments[1:])
        config = [word for pair in pairs if pair[0] == "--config" for word in pair]
        fetch = [*cargo, "fetch", "--quiet", *config]
        subprocess.run(fetch, cwd=copy, env=environment, check=True)

        started = time.monotonic()
        build = subprocess.Popen([*cargo, "build", "--quiet", *arguments], cwd=copy, env=environment)
        peak = 0
        while build.poll() is None:
            peak = max(peak, sum(resident(process) for process in descendants(build.pid)))
            time.sleep(0.25)
        seconds = time.monotonic() - started
    if build.returncode != 0:
        sys.exit(build.returncode)
    print(f"{seconds:.0f} s, peak {peak / 1048576:.2f} GiB: cargo {' '.join([*toolchain, 'build', *arguments])}")


if __name__ == "__main__":
    main()
