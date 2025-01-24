#!/usr/bin/env bash
# Run a simple set of operations to ensure uv is working.
# This will mutate the system and should generally be used in disposable environments.

set -e

if [[ "x$1" != "x"  ]]; then
    target_uv="$(realpath "$1")"
else
    target_uv="uv"
fi

tmpdir=$(mktemp -d)
cd "$tmpdir"

echo "Using uv at $(which "$target_uv")"
echo "Running in $(pwd)"

uv() {
    # Display the command before runing
    echo ""
    echo "$ uv $*"
    # Always enable verbose mode
    "$target_uv" -v "$@"
}

uv --version
uv --help
uv help --no-pager

uv venv
uv venv -p 3.13
uv venv --python-preference only-managed

uv pip install ruff
uv run ruff --version
if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    .venv/Scripts/ruff --version
    source .venv/Scripts/activate
else
    .venv/bin/ruff --version
    source .venv/bin/activate
fi

uv python install 3.12 --preview

uvx ruff --version

uv tool install ruff
ruff --version
