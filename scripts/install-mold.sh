#!/usr/bin/env bash
# Install mold linker and make it the default linker.
#
# Retries on transient HTTP errors (e.g., 500) that the `rui314/setup-mold`
# GitHub Action does not handle.

set -euo pipefail

MOLD_VERSION="${MOLD_VERSION:-2.40.4}"

arch="$(uname -m)"
base_url="https://github.com/rui314/mold/releases/download/v${MOLD_VERSION}"
archive="mold-${MOLD_VERSION}-${arch}-linux.tar.gz"
url="${base_url}/${archive}"
checksums_url="${base_url}/sha256sum.txt"

if [ "$(whoami)" = root ]; then
    SUDO=""
else
    SUDO="sudo"
fi

echo "Installing mold ${MOLD_VERSION} (${arch})..."

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

wget -O "$tmpdir/$archive" \
    --timeout=10 \
    --tries=5 \
    --waitretry=3 \
    --retry-connrefused \
    --retry-on-http-error=429,500,502,503,504 \
    --progress=dot:mega \
    "$url"

wget -O "$tmpdir/sha256sum.txt" \
    --timeout=10 \
    --tries=5 \
    --waitretry=3 \
    --retry-connrefused \
    --retry-on-http-error=429,500,502,503,504 \
    --progress=dot:mega \
    "$checksums_url"

if ! grep -F "  $archive" "$tmpdir/sha256sum.txt" >"$tmpdir/sha256sum.entry"; then
    echo "Checksum entry not found for $archive" >&2
    exit 1
fi

(cd "$tmpdir" && sha256sum --check --status sha256sum.entry)

$SUDO tar -C /usr/local --strip-components=1 --no-overwrite-dir -xzf "$tmpdir/$archive"

# Make mold the default linker
current_ld="$(realpath /usr/bin/ld)"
if [ "$current_ld" != /usr/local/bin/mold ]; then
    $SUDO ln -sf /usr/local/bin/mold "$current_ld"
fi

echo "mold ${MOLD_VERSION} installed successfully"
mold --version
