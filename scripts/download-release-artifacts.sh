#!/usr/bin/env bash
## Download release artifacts from a GitHub Actions workflow run.
##
## Downloads artifact archives (artifacts-* and wheels_uv-*) and extracts
## their contents (tar.gz, zip, whl) in place.
##
## Requires:
##   gh (GitHub CLI)
##
## Usage:
##   scripts/download-release-artifacts.sh [--platform <filter>] <run-id> [output-dir]
##
## Options:
##   --platform <filter>  Only download artifacts matching the given substring.
##                         Can be specified multiple times.
##                         Examples: --platform apple-darwin --platform windows
##
## If output-dir is omitted, artifacts are downloaded to ./artifacts-<run-id>.

set -euo pipefail

PLATFORMS=()
while [[ $# -gt 0 ]]; do
    case "$1" in
        --platform)
            PLATFORMS+=("$2")
            shift 2
            ;;
        --platform=*)
            PLATFORMS+=("${1#--platform=}")
            shift
            ;;
        -*)
            echo "error: unknown option: $1" >&2
            exit 1
            ;;
        *)
            break
            ;;
    esac
done

if [ $# -lt 1 ]; then
    echo "Usage: $0 [--platform <filter>] <github-actions-run-id> [output-dir]" >&2
    exit 1
fi

command -v gh >/dev/null 2>&1 || {
    echo "error: missing required tool: gh" >&2
    exit 1
}

RUN_ID="$1"
OUTPUT_DIR="${2:-artifacts-$RUN_ID}"

matches_platform() {
    local name="$1"
    if [ ${#PLATFORMS[@]} -eq 0 ]; then
        return 0
    fi
    for platform in "${PLATFORMS[@]}"; do
        if [[ "$name" == *"$platform"* ]]; then
            return 0
        fi
    done
    return 1
}

mkdir -p "$OUTPUT_DIR"

echo "Fetching artifact list for run $RUN_ID..."
ALL_ARTIFACTS=$(gh api "repos/{owner}/{repo}/actions/runs/$RUN_ID/artifacts" \
    --paginate --jq '.artifacts[].name')

for artifact in $ALL_ARTIFACTS; do
    case "$artifact" in
        artifacts-*|wheels_uv-*) ;;
        *) continue ;;
    esac

    if ! matches_platform "$artifact"; then
        continue
    fi

    dest="$OUTPUT_DIR/$artifact"
    mkdir -p "$dest"
    echo "Downloading $artifact..."
    if ! gh run download "$RUN_ID" -n "$artifact" -D "$dest"; then
        echo "warning: failed to download $artifact" >&2
        continue
    fi

    # Extract tar.gz archives.
    for tarball in "$dest"/*.tar.gz; do
        [ -f "$tarball" ] || continue
        tar xzf "$tarball" -C "$dest"
    done

    # Extract zip archives and wheels.
    for zip in "$dest"/*.zip "$dest"/*.whl; do
        [ -f "$zip" ] || continue
        unzip -qo "$zip" -d "$dest"
    done
done

echo ""
echo "Artifacts downloaded to $OUTPUT_DIR"
