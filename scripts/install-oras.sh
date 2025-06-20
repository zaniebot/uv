#!/usr/bin/env bash
#
# Install the ORAS CLI.
#
# Usage:
#
#   ./dev/install-oras.sh [<version>]
#
# If no version is specified, the latest version will be installed.
#
# The installation path can be set using the `ORAS_INSTALL_DIR` environment variable, but defaults
# to /usr/local/bin.

set -e

install_oras_cli() {
    ORAS_INSTALL_DIR="${ORAS_INSTALL_DIR:-/usr/local/bin}"
    if [[ ! -d "${ORAS_INSTALL_DIR}" ]]; then
        echo "Installation directory ${ORAS_INSTALL_DIR} not found"
        exit 1
    fi
    echo "Installing to ${ORAS_INSTALL_DIR}"
    
    VERSION="$1"

    if [ -z "$VERSION" ]; then
        # Fetch the latest version
        # Strip the leading 'v' from the version number
        VERSION=$(curl -sSfL https://api.github.com/repos/oras-project/oras/releases/latest | jq -r .tag_name | sed 's/^v//')
        echo "Found latest version ${VERSION}"
    fi

    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        OS="linux"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        OS="darwin"
    else
        echo "Operating system not supported yet for this installer: $OSTYPE."
        exit 1
    fi

    ARCH=$(uname -m)
    if [[ "$ARCH" == "x86_64" ]]; then
        ARCH="amd64"
    elif [[ "$ARCH" == "aarch64" ]]; then
        ARCH="arm64"
    fi

    if [[ "$ARCH" != "amd64" ]] && [[ "$ARCH" != "arm64" ]]; then
        echo "Architecture not supported yet for this installer: $ARCH."
        exit 1
    fi
    
    URL="https://github.com/oras-project/oras/releases/download/v${VERSION}/oras_${VERSION}_${OS}_${ARCH}.tar.gz"
    echo "Downloading from ${URL}"
    curl -sSfLo oras.tar.gz ${URL}

    # Create temporary directory for extraction
    TEMP_DIR=$(mktemp -d)
    tar -xzf oras.tar.gz -C "${TEMP_DIR}"
    
    # Move the oras binary to installation directory
    mv "${TEMP_DIR}/oras" "${ORAS_INSTALL_DIR}"
    
    # Cleanup
    rm -rf oras.tar.gz "${TEMP_DIR}"

    echo "Installed oras"
}

install_oras_cli $1
