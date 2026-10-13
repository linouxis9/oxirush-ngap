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
# and UTILS name, and without from their repositories, at the revisions of
# build/revisions, as neither is on crates.io yet.
set -eu
directory=$1
here=$(cd "$(dirname "$0")/.." && pwd)
mkdir "$directory"
directory=$(cd "$directory" && pwd)
# The options of cargo that say where a crate is: a directory, or its repository at
# the revision that is recorded.
set --
for crate in oxirush-aper oxirush-utils; do
    case "$crate" in
        oxirush-aper) directory_of=${APER:-} ;;
        *) directory_of=${UTILS:-} ;;
    esac
    if [ -n "$directory_of" ]; then
        set -- "$@" --config "patch.crates-io.$crate.path=\"$(cd "$directory_of" && pwd)\""
    else
        revision=$(awk -v name="$crate" '$1 == name { print $2 }' "$here/build/revisions")
        set -- "$@" --config "patch.crates-io.$crate.git=\"https://github.com/linouxis9/$crate\""
        set -- "$@" --config "patch.crates-io.$crate.rev=\"$revision\""
    fi
done
cd "$here"
cargo package "$@" --no-verify --allow-dirty --target-dir "$directory/target"
package=$(ls "$directory/target/package/"*.crate)
tar -xzf "$package" -C "$directory"
cd "$directory/$(basename "$package" .crate)"
echo "$(basename "$package"): $(wc -c < "$package") octets, $(find . -type f | wc -l) files"
export CARGO_TARGET_DIR="$directory/target"
cargo test "$@" --all-features
cargo test "$@"
