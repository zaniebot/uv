#!/usr/bin/env python3
"""
Reproduction script for https://github.com/astral-sh/uv/issues/17650

This script demonstrates that the reported issue is NOT a bug in uv,
but rather a Python module shadowing issue. The issue occurs when:

1. A local `cffi.py` file exists in the current working directory, OR
2. A local `cffi/` directory (package) exists in the current working directory

Either of these will shadow the installed `cffi` package, causing AttributeError
when trying to access `cffi.__version__` or `cffi.FFI()`.

Root Cause: Module shadowing (user has a local cffi.py file OR cffi/ directory)
Resolution: Rename or remove the local cffi.py file or cffi/ directory

The same issue would occur with pip - this is standard Python behavior,
not specific to uv.

Usage:
    python reproduce-17650.py           # Run full reproduction test
    python reproduce-17650.py --diagnose # Run diagnostic on current directory
"""

import subprocess
import tempfile
import os
import sys
import argparse


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


def diagnose_cffi_issue():
    """
    Diagnostic function to check for cffi shadowing issues in the current environment.
    Users experiencing the issue can run this to identify the root cause.
    """
    print("=" * 60)
    print("Diagnosing cffi import issue")
    print("=" * 60)

    # Check 1: Look for local cffi.py or cffi/ in current directory
    cwd = os.getcwd()
    print(f"\n1. Checking current directory: {cwd}")

    cffi_py = os.path.join(cwd, "cffi.py")
    cffi_dir = os.path.join(cwd, "cffi")
    cffi_pycache = os.path.join(cwd, "__pycache__")

    issues_found = []

    if os.path.exists(cffi_py):
        print(f"   WARNING: Found cffi.py at {cffi_py}")
        print("   This file will shadow the installed cffi package!")
        issues_found.append(f"Remove or rename: {cffi_py}")

    if os.path.isdir(cffi_dir):
        print(f"   WARNING: Found cffi/ directory at {cffi_dir}")
        print("   This directory will shadow the installed cffi package!")
        issues_found.append(f"Remove or rename: {cffi_dir}")

    if os.path.isdir(cffi_pycache):
        cffi_pyc = None
        for f in os.listdir(cffi_pycache):
            if f.startswith("cffi.") and f.endswith(".pyc"):
                cffi_pyc = os.path.join(cffi_pycache, f)
                break
        if cffi_pyc:
            print(f"   NOTE: Found cached bytecode at {cffi_pyc}")
            print("   (This alone shouldn't cause issues if source is removed)")

    if not issues_found:
        print("   No local cffi.py or cffi/ found in current directory.")

    # Check 2: Check sys.path for potential shadows
    print("\n2. Checking Python path for cffi shadows...")
    for path in sys.path:
        if not path:
            path = cwd
        if not os.path.isdir(path):
            continue
        if "site-packages" in path:
            continue  # Skip site-packages, that's where it should be

        cffi_py_path = os.path.join(path, "cffi.py")
        cffi_dir_path = os.path.join(path, "cffi")

        if os.path.exists(cffi_py_path):
            print(f"   WARNING: Found cffi.py at {cffi_py_path}")
            issues_found.append(f"Remove or rename: {cffi_py_path}")

        if os.path.isdir(cffi_dir_path) and os.path.exists(
            os.path.join(cffi_dir_path, "__init__.py")
        ):
            print(f"   WARNING: Found cffi/ package at {cffi_dir_path}")
            issues_found.append(f"Remove or rename: {cffi_dir_path}")

    if not issues_found:
        print("   No shadows found in Python path.")

    # Check 3: Try to import cffi and show where it comes from
    print("\n3. Testing cffi import...")
    try:
        # Clear any cached import
        if "cffi" in sys.modules:
            del sys.modules["cffi"]

        import cffi

        print(f"   cffi.__file__ = {cffi.__file__}")

        if "site-packages" in str(cffi.__file__):
            print("   cffi is loading from site-packages (correct)")
        else:
            print("   WARNING: cffi is NOT loading from site-packages!")
            print("   This indicates a shadowing issue.")

        print(f"   cffi.__version__ = {cffi.__version__}")
        print("   Import successful!")
    except AttributeError as e:
        print(f"   ERROR: {e}")
        print("   This confirms a shadowing issue!")
    except ImportError as e:
        print(f"   ERROR: Could not import cffi: {e}")

    # Summary
    print("\n" + "=" * 60)
    print("DIAGNOSIS SUMMARY:")
    print("=" * 60)
    if issues_found:
        print("\nIssues found! To fix, run these commands:")
        for fix in issues_found:
            print(f"  - {fix}")
    else:
        print("\nNo obvious shadowing issues found.")
        print("If you're still experiencing issues, check:")
        print("  - PYTHONPATH environment variable")
        print("  - .pth files in site-packages")
        print("  - Virtual environment activation")

    return len(issues_found)


def main():
    parser = argparse.ArgumentParser(
        description="Reproduce or diagnose GitHub issue #17650"
    )
    parser.add_argument(
        "--diagnose",
        action="store_true",
        help="Run diagnostic on current directory instead of reproduction",
    )
    args = parser.parse_args()

    if args.diagnose:
        return diagnose_cffi_issue()

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

        # === Test Case A: Shadow with cffi.py file ===
        print("\n" + "-" * 60)
        print("TEST CASE A: Shadow with cffi.py file")
        print("-" * 60)

        # Step 4a: Create shadow file
        print("\n4a. Creating local cffi.py shadow file...")
        shadow_file = os.path.join(project_dir, "cffi.py")
        with open(shadow_file, "w") as f:
            f.write("# This file shadows the cffi package\n")
        print("   Created cffi.py")

        # Step 5a: Test import with shadow (should fail)
        print("\n5a. Testing cffi import WITH cffi.py shadow file...")
        code, out, err = run_cmd(
            f'{uv_binary} run python -c "import cffi; print(cffi.__version__)"',
            project_dir,
        )
        if code != 0:
            print("   REPRODUCED: AttributeError occurs")
            print(f"   Error: {err.strip().splitlines()[-1]}")
        else:
            print(f"   Unexpected success: {out}")

        # Step 6a: Check which file is loaded
        print("\n6a. Checking which cffi module is being loaded...")
        code, out, err = run_cmd(
            f'{uv_binary} run python -c "import cffi; print(cffi.__file__)"',
            project_dir,
        )
        print(f"   Loaded: {out.strip()}")

        # Step 7a: Remove shadow and test again
        print("\n7a. Removing cffi.py shadow file...")
        os.remove(shadow_file)
        code, out, err = run_cmd(
            f'{uv_binary} run python -c "import cffi; print(cffi.__file__); print(cffi.__version__)"',
            project_dir,
        )
        if code == 0:
            print(f"   Fixed: cffi loaded from {out.strip().splitlines()[0]}")
        else:
            print(f"   Error: {err}")

        # === Test Case B: Shadow with cffi/ directory ===
        print("\n" + "-" * 60)
        print("TEST CASE B: Shadow with cffi/ directory (package)")
        print("-" * 60)

        # Step 4b: Create shadow directory
        print("\n4b. Creating local cffi/ shadow directory...")
        shadow_dir = os.path.join(project_dir, "cffi")
        os.makedirs(shadow_dir)
        with open(os.path.join(shadow_dir, "__init__.py"), "w") as f:
            f.write("# This package shadows the installed cffi\n")
        print("   Created cffi/__init__.py")

        # Step 5b: Test import with shadow (should fail)
        print("\n5b. Testing cffi import WITH cffi/ shadow directory...")
        code, out, err = run_cmd(
            f'{uv_binary} run python -c "import cffi; print(cffi.__version__)"',
            project_dir,
        )
        if code != 0:
            print("   REPRODUCED: AttributeError occurs")
            print(f"   Error: {err.strip().splitlines()[-1]}")
        else:
            print(f"   Unexpected success: {out}")

        # Step 6b: Check which file is loaded
        print("\n6b. Checking which cffi module is being loaded...")
        code, out, err = run_cmd(
            f'{uv_binary} run python -c "import cffi; print(cffi.__file__)"',
            project_dir,
        )
        print(f"   Loaded: {out.strip()}")
        print("   ^ This is the LOCAL shadow directory, not the installed package!")

        # Step 7b: Remove shadow directory and test again
        print("\n7b. Removing cffi/ shadow directory...")
        import shutil

        shutil.rmtree(shadow_dir)
        code, out, err = run_cmd(
            f'{uv_binary} run python -c "import cffi; print(cffi.__file__); print(cffi.__version__)"',
            project_dir,
        )
        if code == 0:
            print(f"   Fixed: cffi loaded from {out.strip().splitlines()[0]}")
        else:
            print(f"   Error: {err}")

        print("\n" + "=" * 60)
        print("CONCLUSION:")
        print("=" * 60)
        print(
            """
The issue is NOT a bug in uv. It's a Python module shadowing problem.

The user likely has one of the following in their project directory:
  1. A file named 'cffi.py', OR
  2. A directory named 'cffi/' with an __init__.py

Either of these will shadow the installed 'cffi' package.

This is standard Python behavior where local files/packages take precedence
over installed packages in the module lookup path.

SOLUTION:
  - Check for and remove/rename any local 'cffi.py' file
  - Check for and remove/rename any local 'cffi/' directory
  - Run: python reproduce-17650.py --diagnose  (for automated diagnosis)
"""
        )

        return 0


if __name__ == "__main__":
    exit(main())
