#!/usr/bin/env bash
## Verify that all release wheels contain valid CycloneDX SBOM data.
##
## Checks that each .whl file in the artifacts directory contains a
## CycloneDX SBOM (*.cyclonedx.json) with a valid structure and at least
## one component.
##
## Requires:
##   python3
##
## Usage:
##   scripts/check-release-artifact-wheel-sboms.sh <artifacts-dir>
##
## The artifacts directory should contain extracted artifact subdirectories
## (e.g., as produced by scripts/download-release-artifacts.sh).

set -euo pipefail

if [ $# -ne 1 ]; then
    echo "Usage: $0 <artifacts-dir>" >&2
    exit 1
fi

command -v python3 >/dev/null 2>&1 || {
    echo "error: missing required tool: python3" >&2
    exit 1
}

ARTIFACTS_DIR="$1"

if [ ! -d "$ARTIFACTS_DIR" ]; then
    echo "error: not a directory: $ARTIFACTS_DIR" >&2
    exit 1
fi

PASS=0
FAIL=0

pass() { echo "PASS $1"; PASS=$((PASS + 1)); }
fail() { echo "FAIL $1: $2"; FAIL=$((FAIL + 1)); }

check_wheel() {
    local wheel="$1"
    local label="$2"

    # Use python3 to extract and validate the CycloneDX SBOM from the wheel.
    result=$(python3 -c "
import json, re, sys, zipfile

wheel_path = sys.argv[1]

with zipfile.ZipFile(wheel_path) as whl:
    sbom_entries = [n for n in whl.namelist() if n.endswith('.cyclonedx.json')]

    if not sbom_entries:
        print('no CycloneDX SBOM found in wheel')
        sys.exit(1)

    for entry in sbom_entries:
        data = whl.read(entry)
        try:
            sbom = json.loads(data)
        except json.JSONDecodeError as exc:
            print(f'{entry}: invalid JSON: {exc}')
            sys.exit(1)

        errors = []
        if sbom.get('bomFormat') != 'CycloneDX':
            errors.append(f'bomFormat: expected CycloneDX, got {sbom.get(\"bomFormat\")!r}')

        spec_version = sbom.get('specVersion')
        if not isinstance(spec_version, str) or not spec_version:
            errors.append(f'specVersion: missing or empty')

        components = sbom.get('components')
        if not isinstance(components, list):
            errors.append(f'components: expected a list, got {type(components).__name__}')
        elif len(components) == 0:
            errors.append('components: empty (expected at least one)')

        if errors:
            print(f'{entry}: ' + '; '.join(errors))
            sys.exit(1)

        print(f'{entry} ({len(components)} components)')
" "$wheel" 2>&1)

    if [ $? -eq 0 ]; then
        pass "$label: $result"
    else
        fail "$label" "$result"
    fi
}

for wheel_dir in "$ARTIFACTS_DIR"/wheels_uv*/; do
    [ -d "$wheel_dir" ] || continue

    for wheel in "$wheel_dir"*.whl; do
        [ -f "$wheel" ] || continue
        check_wheel "$wheel" "$(basename "$wheel")"
    done
done

echo ""
echo "PASS $PASS / FAIL $FAIL"

if [ "$FAIL" -gt 0 ]; then
    exit 1
fi
