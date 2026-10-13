#!/bin/sh
# The package that crates.io would hold of the crate, alone: cargo makes it, and its
# tests are built and run from it, outside the repository, with every feature and with
# none. The package has what `include` of the manifest lists, so a test that needs a
# file or a dependency which the package does not have fails here.
#
#     tests/package.sh DIRECTORY
#
# DIRECTORY, which the script makes, has the package as cargo writes it, unpacked, and
# what cargo builds of it. The files of the package are those of the working directory,
# committed or not. oxirush-aper and oxirush-utils come from the directories that APER
# and UTILS name, and from their repositories without, as neither is on crates.io yet.
set -eu
directory=$1
here=$(cd "$(dirname "$0")/.." && pwd)
mkdir "$directory"
directory=$(cd "$directory" && pwd)
aper='git="https://github.com/linouxis9/oxirush-aper"'
utils='git="https://github.com/linouxis9/oxirush-utils"'
if [ -n "${APER:-}" ]; then
    aper="path=\"$(cd "$APER" && pwd)\""
fi
if [ -n "${UTILS:-}" ]; then
    utils="path=\"$(cd "$UTILS" && pwd)\""
fi
set -- --config "patch.crates-io.oxirush-aper.$aper" --config "patch.crates-io.oxirush-utils.$utils"
cd "$here"
cargo package "$@" --no-verify --allow-dirty --target-dir "$directory/target"
package=$(ls "$directory/target/package/"*.crate)
tar -xzf "$package" -C "$directory"
cd "$directory/$(basename "$package" .crate)"
echo "$(basename "$package"): $(wc -c < "$package") octets, $(find . -type f | wc -l) files"
export CARGO_TARGET_DIR="$directory/target"
cargo test "$@" --all-features
cargo test "$@"
