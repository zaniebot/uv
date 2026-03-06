#!/usr/bin/env bash
set -euxo pipefail

# Install uv into the Termux prefix
cp /uv /data/data/com.termux/files/usr/bin/uv
chmod +x /data/data/com.termux/files/usr/bin/uv

# Update package index to avoid version skew between the base image and the repo
pkg update -y

# Install Python and its transitive dependencies at pinned versions.
# This avoids flakes from Termux's rolling repo introducing broken dependency
# combinations. To update these pins, run:
#   curl -s "https://packages.termux.dev/apt/termux-main/dists/stable/main/binary-x86_64/Packages" \
#     | python3 scripts/resolve_termux_deps.py python
pkg install -y \
    ca-certificates=1:2025.12.02 \
    gdbm=1.26-1 \
    libandroid-posix-semaphore=0.1-4 \
    libandroid-support=29-1 \
    libbz2=1.0.8-8 \
    libcrypt=0.2-6 \
    libexpat=2.7.4 \
    libffi=3.4.7-1 \
    liblzma=5.8.2 \
    libsqlite=3.51.2 \
    ncurses=6.6.20260124+really6.5.20250830 \
    ncurses-ui-libs=6.6.20260124+really6.5.20250830 \
    openssl=1:3.6.1 \
    python=3.13.12-2 \
    readline=8.3.1-2 \
    zlib=1.3.2

# Test uv
uv --version

# Termux uses Bionic libc (not glibc or musl), so uv cannot discover
# managed Python installations. Use only-system to skip that check.
export UV_PYTHON_PREFERENCE=only-system
uv python find
uv run -- python --version
