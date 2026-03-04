#!/usr/bin/env bash
#
# Manually perform a GitHub release for a broken automated release that otherwise went out.
#
# PLEASE USE WITH CAUTION.
#
# It can take a while to download all the artifacts.
#
# Requires the `gh` CLI.

set -euo pipefail

if [ ! -n "$COMMIT" ]; then
    echo "COMMIT is required."
    exit 1
fi

if [ ! -n "$RUN_ID" ]; then
    echo "RUN_ID is required."
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RELEASE_DIR="release_$RUN_ID"

# Download artifacts using the shared script.
"$SCRIPT_DIR/download-release-artifacts.sh" "$RUN_ID" "$RELEASE_DIR"

cd "$RELEASE_DIR"

REPO=$(gh repo view --json nameWithOwner | jq .nameWithOwner -r)

MANIFEST="artifacts-dist-manifest/dist-manifest.json"

# Extract values from manifest
TAG=$(jq -r '.announcement_tag // .tag' "$MANIFEST")
TITLE=$(jq -r '.announcement_title' "$MANIFEST")
BODY=$(jq -r '.announcement_github_body' "$MANIFEST")
PRERELEASE=$(jq -r '.announcement_is_prerelease' "$MANIFEST")

# Write body to temp file
echo "$BODY" > /tmp/notes.txt

# Merge artifacts-* directories into artifacts/ (like CI does)
mkdir -p artifacts
cp -r artifacts-*/* artifacts/

# Remove the granular manifests (like CI does)
rm -f artifacts/*-dist-manifest.json

# Create release
gh release create "$TAG" \
    --target "$COMMIT" \
    --title "$TITLE" \
    --notes-file /tmp/notes.txt \
    "$([ "$PRERELEASE" = "true" ] && echo "--prerelease")" \
    artifacts/*
