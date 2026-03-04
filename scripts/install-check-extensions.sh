#!/usr/bin/env bash
## Install tools for verifying release artifacts.
##
## Installs:
##   rust-audit-info   - SBOM verification (cargo install, all platforms)
##   osslsigncode      - Windows Authenticode signature verification
##                        (apt on Linux, brew on macOS)
##
## On macOS, the native `codesign` is used for macOS signature verification
## and does not need to be installed.
##
## Usage:
##
##   $ scripts/install-check-extensions.sh
##
## Expected to be used with:
##   scripts/check-release-artifact-sboms.sh
##   scripts/check-release-artifacts-signed.sh

set -euo pipefail

cargo install rust-audit-info --locked

case "$(uname -s)" in
    Linux)
        sudo apt-get update -qq
        sudo apt-get install -y -qq osslsigncode
        ;;
    Darwin)
        brew install osslsigncode
        ;;
    *)
        echo "warning: unsupported platform for osslsigncode install" >&2
        ;;
esac
