#!/usr/bin/env bash
#
# Remote Docker build helper for developing/debugging uv on Linux from macOS.
#
# Usage:
#   ./scripts/remote-build.sh setup   <ssh-host>        # Create remote buildx builder
#   ./scripts/remote-build.sh image                     # Build the dev Docker image on remote
#   ./scripts/remote-build.sh build   [cargo args...]   # Build uv on remote host
#   ./scripts/remote-build.sh shell   [ssh-host]        # Interactive shell in remote container
#   ./scripts/remote-build.sh run     [uv args...]      # Run uv on remote Linux
#   ./scripts/remote-build.sh clean                     # Remove remote builder
#
# Environment:
#   REMOTE_HOST   - SSH host (e.g., user@server). Used as default for all commands.
#   BUILDER_NAME  - buildx builder name (default: uv-remote)
#   IMAGE_NAME    - Docker image name (default: uv-dev)

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BUILDER_NAME="${BUILDER_NAME:-uv-remote}"
IMAGE_NAME="${IMAGE_NAME:-uv-dev}"
REMOTE_HOST="${REMOTE_HOST:-}"

usage() {
    sed -n '3,13p' "$0" | sed 's/^# \?//'
    exit 1
}

sync_source() {
    local host="$1"
    echo "Syncing source to remote..."
    rsync -az --delete \
        --exclude target/ \
        --exclude .git/ \
        --exclude '*.pyc' \
        --filter=':- .gitignore' \
        "$REPO_ROOT/" "$host:~/uv-build/"
}

docker_run() {
    local host="$1"
    shift
    # shellcheck disable=SC2029
    ssh "$host" "cd ~/uv-build && docker run --rm \
        -v \"\$(pwd):/src\" \
        -v uv-cargo-registry:/usr/local/cargo/registry \
        -v uv-cargo-target:/src/target \
        -w /src \
        $IMAGE_NAME \
        -c \"$*\""
}

cmd_setup() {
    local host="${1:-$REMOTE_HOST}"
    if [[ -z "$host" ]]; then
        echo "error: provide ssh host as argument or set REMOTE_HOST" >&2
        exit 1
    fi

    echo "Creating remote buildx builder '$BUILDER_NAME' on $host..."
    docker buildx create \
        --name "$BUILDER_NAME" \
        --driver docker-container \
        --platform linux/amd64 \
        "ssh://$host"

    echo "Bootstrapping builder..."
    docker buildx inspect --builder "$BUILDER_NAME" --bootstrap

    echo ""
    echo "Builder '$BUILDER_NAME' is ready."
    echo "Set REMOTE_HOST=$host to use as default for other commands."
}

cmd_image() {
    local host="${REMOTE_HOST:?Set REMOTE_HOST to your ssh host}"

    sync_source "$host"

    echo "Building dev image '$IMAGE_NAME' on remote..."
    # shellcheck disable=SC2029
    ssh "$host" "cd ~/uv-build && docker build -t $IMAGE_NAME -f Dockerfile.dev ."
}

cmd_build() {
    local cargo_args=("$@")
    if [[ ${#cargo_args[@]} -eq 0 ]]; then
        cargo_args=(--bin uv --bin uvx)
    fi

    local host="${REMOTE_HOST:?Set REMOTE_HOST to your ssh host}"

    sync_source "$host"

    echo "Building on remote: cargo build ${cargo_args[*]}"
    docker_run "$host" cargo build "${cargo_args[*]}"
}

cmd_shell() {
    local host="${1:-${REMOTE_HOST:?Set REMOTE_HOST to your ssh host}}"

    sync_source "$host"

    echo "Starting interactive shell on $host..."
    # shellcheck disable=SC2029
    ssh -t "$host" "cd ~/uv-build && docker run --rm -it \
        -v \"\$(pwd):/src\" \
        -v uv-cargo-registry:/usr/local/cargo/registry \
        -v uv-cargo-target:/src/target \
        -w /src \
        $IMAGE_NAME \
        bash"
}

cmd_run() {
    local host="${REMOTE_HOST:?Set REMOTE_HOST to your ssh host}"
    local uv_args=("$@")

    docker_run "$host" cargo run --bin uv -- "${uv_args[*]}"
}

cmd_clean() {
    echo "Removing builder '$BUILDER_NAME'..."
    docker buildx rm "$BUILDER_NAME" 2>/dev/null || true
    echo "Done."
}

case "${1:-}" in
    setup) shift; cmd_setup "$@" ;;
    image) shift; cmd_image "$@" ;;
    build) shift; cmd_build "$@" ;;
    shell) shift; cmd_shell "$@" ;;
    run)   shift; cmd_run "$@" ;;
    clean) shift; cmd_clean "$@" ;;
    *)     usage ;;
esac
