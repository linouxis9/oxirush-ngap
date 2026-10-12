#!/bin/sh
# Start the corpus of each target from the vectors of the crate and from the PDUs
# that other implementations sent: one file for each of tests/fixtures that has no
# more than 4096 octets. Run it from the root of
# the repository.
set -eu
mkdir -p fuzz/corpus/decode fuzz/corpus/view fuzz/corpus/paths
number=0
grep -hv '^#' tests/fixtures/messages.tsv tests/fixtures/vectors.tsv tests/fixtures/later.tsv \
    tests/fixtures/captured.tsv |
    while IFS="$(printf '\t')" read -r name octets rest; do
        [ "${#octets}" -le 8192 ] || continue
        number=$((number + 1))
        printf '%s' "$octets" | xxd -r -p > "fuzz/corpus/decode/$number"
        cp "fuzz/corpus/decode/$number" "fuzz/corpus/view/$number"
        # Before its PDU, the target of the paths takes what is done with it: here
        # `select`, with the first path of the view, whole, and nothing more.
        { printf '\000\000\377\000\000'; cat "fuzz/corpus/decode/$number"; } \
            > "fuzz/corpus/paths/$number"
    done
