#!/bin/sh
# The crate as a member of a larger workspace, which is how a product that keeps a copy
# of it beside its own crates has it: cargo loads the workspace, clippy, which is
# `cargo check` with its lints, accepts every target of every member, and rustdoc
# documents them. With other crates, every target is built too: two members that give
# a target one name write one file, which cargo warns of where it builds them.
#
#     tests/workspace_member.sh DIRECTORY [CRATE...]
#
# DIRECTORY, which the script makes, becomes a workspace whose members are this crate
# and each other CRATE, the directory of one: they are links, and nothing is written
# into them. A member named oxirush-aper or oxirush-utils is the one of the others.
# Without one it comes from its repository, as neither is on crates.io yet.
set -eu
directory=$1
shift
here=$(cd "$(dirname "$0")/.." && pwd)
mkdir "$directory"
members=
aper='{ git = "https://github.com/linouxis9/oxirush-aper" }'
utils='{ git = "https://github.com/linouxis9/oxirush-utils" }'
for crate in "$here" "$@"; do
    crate=$(cd "$crate" && pwd)
    name=$(sed -n 's/^name = "\(.*\)"$/\1/p' "$crate/Cargo.toml" | head -n 1)
    ln -s "$crate" "$directory/$name"
    members="$members\"$name\", "
    if [ "$name" = oxirush-aper ]; then
        aper='{ path = "oxirush-aper" }'
    fi
    if [ "$name" = oxirush-utils ]; then
        utils='{ path = "oxirush-utils" }'
    fi
done
cat > "$directory/Cargo.toml" <<MANIFEST
[workspace]
resolver = "3"
members = [$members]

[patch.crates-io]
oxirush-aper = $aper
oxirush-utils = $utils
MANIFEST
cd "$directory"
cargo metadata --format-version 1 > /dev/null
cargo clippy --workspace --all-targets -- -D warnings
cargo doc --workspace --no-deps
if [ $# -gt 0 ]; then
    cargo build --workspace --all-targets 2> build.log || {
        cat build.log >&2
        exit 1
    }
    if grep -A 4 "output filename collision" build.log >&2; then
        exit 1
    fi
fi
