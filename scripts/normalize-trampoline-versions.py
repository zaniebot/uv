"""Normalize workspace member crate versions to 0.0.0 for reproducible trampoline builds.

Rust embeds crate versions in -Cmetadata hashes that feed into symbol mangling.
Even with strip = true the different hashes change linker layout, so a
version-only bump (e.g. 0.0.35 -> 0.0.36) produces a different binary.
Pinning every workspace member version to 0.0.0 keeps the trampoline build
output stable across release version bumps.

The script rewrites Cargo.toml files **in-place** inside the directory tree
rooted at the given path.  It is meant to run on a disposable copy of the
workspace (e.g. inside the Docker build container).
"""

# /// script
# requires-python = ">=3.12"
# ///

from __future__ import annotations

import re
import sys
from pathlib import Path

NORMALIZED_VERSION = "0.0.0"


def normalize_member_manifest(path: Path) -> None:
    """Replace the ``version = "..."`` field in a crate manifest."""
    text = path.read_text()
    # Match the first `version = "..."` which is always the package version.
    new_text = re.sub(
        r'^(version\s*=\s*)"[^"]*"',
        rf'\g<1>"{NORMALIZED_VERSION}"',
        text,
        count=1,
        flags=re.MULTILINE,
    )
    if new_text != text:
        path.write_text(new_text)


def normalize_workspace_root(path: Path) -> None:
    """Normalize all workspace member version pins in the root manifest.

    Targets lines like:
        uv-foo = { version = "0.0.35", path = "crates/uv-foo" }

    The ``path =`` anchor ensures we only touch workspace members (path
    dependencies), not third-party registry dependencies.
    """
    text = path.read_text()
    new_text = re.sub(
        r'(version\s*=\s*)"[^"]*"(\s*,\s*path\s*=)',
        rf'\g<1>"{NORMALIZED_VERSION}"\2',
        text,
    )
    if new_text != text:
        path.write_text(new_text)


def main(workspace_root: Path) -> None:
    root_manifest = workspace_root / "Cargo.toml"
    if not root_manifest.exists():
        print(f"error: {root_manifest} not found", file=sys.stderr)
        sys.exit(1)

    # Normalize each member crate manifest under crates/
    crates_dir = workspace_root / "crates"
    count = 0
    for manifest in sorted(crates_dir.glob("*/Cargo.toml")):
        normalize_member_manifest(manifest)
        count += 1

    # Normalize the workspace dependency pins in the root manifest
    normalize_workspace_root(root_manifest)

    print(f"Normalized {count} crate versions to {NORMALIZED_VERSION}")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(f"usage: {sys.argv[0]} <workspace-root>", file=sys.stderr)
        sys.exit(1)
    main(Path(sys.argv[1]))
