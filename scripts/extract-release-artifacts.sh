#!/usr/bin/env bash
## Extract release artifact archives in place.
##
## Walks each subdirectory of the given artifacts directory and extracts any
## tar.gz, zip, or whl files found there.
##
## Usage:
##   scripts/extract-release-artifacts.sh <artifacts-dir>
##
## Typically used after downloading artifacts with either
## `scripts/download-release-artifacts.sh` or `actions/download-artifact`.

set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: $0 <artifacts-dir>" >&2
    exit 1
fi

ARTIFACTS_DIR="$1"

if [ ! -d "$ARTIFACTS_DIR" ]; then
    echo "error: not a directory: $ARTIFACTS_DIR" >&2
    exit 1
fi

for dir in "$ARTIFACTS_DIR"/*/; do
    [ -d "$dir" ] || continue

    for tarball in "$dir"*.tar.gz; do
        [ -f "$tarball" ] || continue
        tar xzf "$tarball" -C "$dir"
    done

    for zip in "$dir"*.zip "$dir"*.whl; do
        [ -f "$zip" ] || continue
        unzip -qo "$zip" -d "$dir"
    done
done
