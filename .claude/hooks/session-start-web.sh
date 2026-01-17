#!/bin/bash
set -euo pipefail

# Install `gh`
if ! command -v gh &> /dev/null; then
    GH_VERSION="2.83.2"
    mkdir -p ~/.local/bin
    curl -sL "https://github.com/cli/cli/releases/download/v${GH_VERSION}/gh_${GH_VERSION}_linux_amd64.tar.gz" -o /tmp/gh.tar.gz
    tar -xzf /tmp/gh.tar.gz -C /tmp
    mv /tmp/gh_${GH_VERSION}_linux_amd64/bin/gh ~/.local/bin/
    rm -rf /tmp/gh.tar.gz /tmp/gh_${GH_VERSION}_linux_amd64
fi

# Install clippy and rustfmt for the active toolchain.
rustup component add clippy rustfmt

# Clone or update rendered GitHub issues and PRs
CACHE_REPO="/tmp/.unmaintain-rendered-cache"
if [ -d "$CACHE_REPO/.git" ]; then
    # Already cloned, just update
    git -C "$CACHE_REPO" pull --depth 1 origin main
else
    # Sparse shallow clone with only astral-sh/uv
    git clone --depth 1 --filter=blob:none --sparse \
        https://github.com/zaniebot/unmaintain-rendered.git "$CACHE_REPO"
    git -C "$CACHE_REPO" sparse-checkout set astral-sh/uv
fi

# Copy to .github/issues and .github/pulls
rm -rf .github/issues .github/pulls
cp -r "$CACHE_REPO/astral-sh/uv/issues" .github/issues
cp -r "$CACHE_REPO/astral-sh/uv/pulls" .github/pulls
