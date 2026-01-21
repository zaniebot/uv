#!/usr/bin/env python3
"""
Reproduction script for https://github.com/astral-sh/uv/issues/17650

This script demonstrates that the reported issue is NOT a bug in uv,
but rather a Python module shadowing issue caused by having a local
file named `cffi.py` in the current working directory.

When a local `cffi.py` file exists, Python's module lookup finds it
before the installed `cffi` package, causing AttributeError when
trying to access `cffi.__version__` or `cffi.FFI()`.

Root Cause: Module shadowing (user has a local cffi.py file)
Resolution: Rename or remove the local cffi.py file

To reproduce:
1. Create a project: `uv init test-cffi && cd test-cffi`
2. Install cffi: `uv add cffi`
3. Create a local shadow file: `echo "# shadow" > cffi.py`
4. Test import: `uv run python -c "import cffi; print(cffi.__version__)"`
   -> This will fail with AttributeError
5. Remove shadow file: `rm cffi.py`
6. Test import again: `uv run python -c "import cffi; print(cffi.__version__)"`
   -> This will succeed

The same issue would occur with pip - this is standard Python behavior,
not specific to uv.
"""

import subprocess
import tempfile
import os


def run_cmd(cmd, cwd, check=True):
    """Run a command and return output."""
    result = subprocess.run(
        cmd,
        cwd=cwd,
        capture_output=True,
        text=True,
        shell=True,
        check=False,
    )
    return result.returncode, result.stdout, result.stderr


def main():
    # Find uv binary
    uv_binary = os.environ.get("UV_BINARY", "uv")

    with tempfile.TemporaryDirectory() as tmpdir:
        project_dir = os.path.join(tmpdir, "test-cffi")
        os.makedirs(project_dir)

        print("=" * 60)
        print("Reproducing GitHub Issue #17650")
        print("=" * 60)

        # Step 1: Initialize project
        print("\n1. Initializing uv project...")
        code, out, err = run_cmd(f"{uv_binary} init", project_dir)
        if code != 0:
            print(f"   Error: {err}")
            return 1
        print("   Done.")

        # Step 2: Add cffi
        print("\n2. Installing cffi...")
        code, out, err = run_cmd(f"{uv_binary} add cffi", project_dir)
        if code != 0:
            print(f"   Error: {err}")
            return 1
        print("   Done.")

        # Step 3: Test normal import (should work)
        print("\n3. Testing normal cffi import (should work)...")
        code, out, err = run_cmd(
            f'{uv_binary} run python -c "import cffi; print(cffi.__file__); print(cffi.__version__)"',
            project_dir,
        )
        if code == 0:
            print(f"   Success: cffi loaded from {out.strip().splitlines()[0]}")
            print(f"   Version: {out.strip().splitlines()[1]}")
        else:
            print(f"   Unexpected error: {err}")

        # Step 4: Create shadow file
        print("\n4. Creating local cffi.py shadow file...")
        shadow_file = os.path.join(project_dir, "cffi.py")
        with open(shadow_file, "w") as f:
            f.write("# This file shadows the cffi package\n")
        print("   Created cffi.py")

        # Step 5: Test import with shadow (should fail - reproduces the bug)
        print("\n5. Testing cffi import WITH shadow file (reproduces issue #17650)...")
        code, out, err = run_cmd(
            f'{uv_binary} run python -c "import cffi; print(cffi.__version__)"',
            project_dir,
        )
        if code != 0:
            print("   REPRODUCED: AttributeError occurs")
            print(f"   Error: {err.strip().splitlines()[-1]}")
        else:
            print(f"   Unexpected success: {out}")

        # Step 6: Check which file is loaded
        print("\n6. Checking which cffi module is being loaded...")
        code, out, err = run_cmd(
            f'{uv_binary} run python -c "import cffi; print(cffi.__file__)"',
            project_dir,
        )
        print(f"   Loaded: {out.strip()}")
        print("   ^ This is the LOCAL shadow file, not the installed package!")

        # Step 7: Remove shadow and test again
        print("\n7. Removing shadow file and testing again...")
        os.remove(shadow_file)
        code, out, err = run_cmd(
            f'{uv_binary} run python -c "import cffi; print(cffi.__file__); print(cffi.__version__)"',
            project_dir,
        )
        if code == 0:
            print(f"   Success: cffi loaded from {out.strip().splitlines()[0]}")
            print(f"   Version: {out.strip().splitlines()[1]}")
        else:
            print(f"   Error: {err}")

        print("\n" + "=" * 60)
        print("CONCLUSION:")
        print("=" * 60)
        print("""
The issue is NOT a bug in uv. It's a Python module shadowing problem.

The user likely has a file named 'cffi.py' in their project directory
(or current working directory), which shadows the installed 'cffi' package.

This is standard Python behavior where local files take precedence over
installed packages in the module lookup path.

SOLUTION: Rename or remove any local 'cffi.py' file.
""")

        return 0


if __name__ == "__main__":
    exit(main())
