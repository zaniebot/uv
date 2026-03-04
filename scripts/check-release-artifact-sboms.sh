#!/usr/bin/env bash
## Verify that all release artifacts contain cargo-auditable SBOM data.
##
## Requires:
##   cargo install rust-audit-info --locked
##
## Usage:
##   scripts/check-release-artifact-sboms.sh <artifacts-dir>
##
## The artifacts directory should contain extracted artifact subdirectories
## (e.g., as produced by scripts/download-release-artifacts.sh).

set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: $0 <artifacts-dir>" >&2
    exit 1
fi

command -v rust-audit-info >/dev/null 2>&1 || {
    echo "error: missing required tool: rust-audit-info" >&2
    echo "  cargo install rust-audit-info --locked" >&2
    exit 1
}

ARTIFACTS_DIR="$1"

if [ ! -d "$ARTIFACTS_DIR" ]; then
    echo "error: not a directory: $ARTIFACTS_DIR" >&2
    exit 1
fi

PASS=0
FAIL=0

pass() { echo "PASS $1"; PASS=$((PASS + 1)); }
fail() { echo "FAIL $1"; FAIL=$((FAIL + 1)); }

check() {
    local binary="$1"
    local label="$2"
    if rust-audit-info "$binary" >/dev/null 2>&1; then
        pass "$label"
    else
        fail "$label"
    fi
}

for artifact_dir in "$ARTIFACTS_DIR"/artifacts-*/; do
    [ -d "$artifact_dir" ] || continue

    artifact=$(basename "$artifact_dir")

    # Find the archive name for labeling.
    archive=""
    for f in "$artifact_dir"/*.tar.gz "$artifact_dir"/*.zip; do
        [ -f "$f" ] && archive=$(basename "$f") && break
    done

    # Check uv and uvx binaries.
    for bin in uv uvx; do
        binary=$(find "$artifact_dir" \( -name "$bin" -o -name "$bin.exe" \) -type f | head -1)
        if [ -n "$binary" ]; then
            check "$binary" "${archive:-$artifact} / $(basename "$binary")"
        fi
    done
done

echo ""
echo "PASS $PASS / FAIL $FAIL"

if [ "$FAIL" -gt 0 ]; then
    exit 1
fi
