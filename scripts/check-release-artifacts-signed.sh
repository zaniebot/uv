#!/usr/bin/env bash
## Check that release artifacts are code-signed.
##
## Examines macOS and Windows artifacts and wheels, extracts binaries, and
## verifies:
##   - macOS:   codesign identity signature (not ad-hoc)
##   - Windows: Authenticode signature present
##
## Only artifacts whose required tool is available are checked. Artifacts that
## are skipped due to a missing tool are reported as SKIP. If no artifacts are
## checked at all, the script exits with an error.
##
## Tools:
##   macOS binaries:   codesign  (requires macOS)
##   Windows binaries: osslsigncode  (apt install osslsigncode / brew install osslsigncode)
##
## Usage:
##   scripts/check-release-artifacts-signed.sh <artifacts-dir>
##
## The artifacts directory should contain extracted artifact subdirectories
## (e.g., as produced by scripts/download-release-artifacts.sh).

set -euo pipefail

if [ $# -lt 1 ]; then
    echo "Usage: $0 <artifacts-dir>" >&2
    exit 1
fi

ARTIFACTS_DIR="$1"

if [ ! -d "$ARTIFACTS_DIR" ]; then
    echo "error: not a directory: $ARTIFACTS_DIR" >&2
    exit 1
fi

HAS_CODESIGN=false
HAS_OSSLSIGNCODE=false
command -v codesign >/dev/null 2>&1 && HAS_CODESIGN=true
command -v osslsigncode >/dev/null 2>&1 && HAS_OSSLSIGNCODE=true

PASS=0
FAIL=0
SKIP=0

pass() { echo "PASS $1"; PASS=$((PASS + 1)); }
fail() { echo "FAIL $1"; FAIL=$((FAIL + 1)); }
skip() { echo "SKIP $1"; SKIP=$((SKIP + 1)); }

check_macos() {
    local binary="$1"
    local label="$2"

    if ! $HAS_CODESIGN; then
        skip "$label (codesign not available)"
        return
    fi

    local info
    info=$(codesign -dv "$binary" 2>&1) || true
    if echo "$info" | grep -q "Signature=adhoc"; then
        fail "$label (ad-hoc, not identity-signed)"
    elif sig_size=$(echo "$info" | grep "Signature size=" | sed 's/.*Signature size=//'); then
        pass "$label (identity-signed, size=$sig_size)"
    else
        fail "$label (not signed)"
    fi
}

check_windows() {
    local binary="$1"
    local label="$2"

    if ! $HAS_OSSLSIGNCODE; then
        skip "$label (osslsigncode not available)"
        return
    fi

    local output
    output=$(osslsigncode verify -in "$binary" 2>&1) || true
    if echo "$output" | grep -q "Signer's certificate:"; then
        local subject
        subject=$(echo "$output" | grep "Subject:" | head -1 | sed 's/.*Subject: //')
        pass "$label (Authenticode, $subject)"
    else
        fail "$label (not Authenticode signed)"
    fi
}

for artifact_dir in "$ARTIFACTS_DIR"/artifacts-* "$ARTIFACTS_DIR"/wheels_uv-*; do
    [ -d "$artifact_dir" ] || continue

    artifact=$(basename "$artifact_dir")

    # Only check macOS and Windows artifacts.
    case "$artifact" in
        artifacts-*apple-darwin*|artifacts-macos-*)  check=check_macos ;;
        artifacts-*windows*|artifacts-*win*)         check=check_windows ;;
        wheels_uv-*apple-darwin*|wheels_uv-macos-*)  check=check_macos ;;
        wheels_uv-*windows*|wheels_uv-*win*)         check=check_windows ;;
        *) continue ;;
    esac

    # Find the archive or wheel name for labeling.
    archive=""
    for f in "$artifact_dir"/*.tar.gz "$artifact_dir"/*.zip "$artifact_dir"/*.whl; do
        [ -f "$f" ] && archive=$(basename "$f") && break
    done

    # Check each binary.
    while IFS= read -r binary; do
        bin_name=$(basename "$binary")
        $check "$binary" "${archive:-$artifact} / $bin_name"
    done < <(find "$artifact_dir" \( -name "uv" -o -name "uvx" -o -name "uv.exe" -o -name "uvx.exe" -o -name "uvw.exe" \) -type f ! -name "*.sha256")
done

echo ""
TOTAL=$((PASS + FAIL + SKIP))
echo "PASS $PASS / FAIL $FAIL / SKIP $SKIP"

if [ "$TOTAL" -eq 0 ]; then
    echo "error: no artifacts were checked" >&2
    exit 1
fi

if [ "$FAIL" -gt 0 ]; then
    exit 1
fi
