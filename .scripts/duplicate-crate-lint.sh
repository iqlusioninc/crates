#!/bin/bash
#
# Locate and warn for duplicated crates (i.e. multiple versions of the same
# crate) in the build

set -e
set -o pipefail

# Maximum number of duplicated crates allowed
MAX_MULTIVERSION_CRATES=1

DUPLICATED_CRATES=$(
    grep "^\"checksum" Cargo.lock |
    cut -d " " -f 2 |
    sort |
    uniq -c |
    grep -v "^\s*1 "
)

if [[ $(echo "$DUPLICATED_CRATES" | wc -l) -gt $MAX_MULTIVERSION_CRATES ]]; then
    echo "*** ERROR: Crates with multiple versions in Cargo.lock!"
    echo
    echo "$DUPLICATED_CRATES"
fi
