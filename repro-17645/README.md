# Reproduction for uv issue #17645

This directory contains a minimal reproduction for https://github.com/astral-sh/uv/issues/17645

## Problem

When switching extras with `uv sync`, packages that provide conflicting files (like `typer` and
`typer-slim`) can corrupt the virtual environment. The tool's metadata incorrectly shows a package
as installed while its actual files have been deleted.

## Steps to reproduce

```bash
cd repro-17645

# Clean start
rm -rf .venv uv.lock

# Step 1: Initial sync (works)
uv sync
.venv/bin/python test_typer.py  # Success: "typer import successful!"

# Step 2: Sync with extra1 (works)
uv sync --extra extra1
.venv/bin/python test_typer.py  # Success: "typer import successful!"

# Step 3: Sync with extra2 (FAILS - this is the bug)
uv sync --extra extra2
.venv/bin/python test_typer.py  # FAILS: ModuleNotFoundError: No module named 'typer'

# Verify corrupted state:
uv pip list | grep typer  # Shows typer==0.21.1 as "installed"
ls .venv/lib/python*/site-packages/ | grep typer  # Only shows typer-*.dist-info, no typer/ directory
```

## Root Cause

1. Initial sync installs `typer` (which depends on `typer-slim`)
2. `uv sync --extra extra1` installs `typer-slim`, which overwrites files in the `typer/` directory
3. `uv sync --extra extra2` uninstalls `typer-slim` (since extra1 is no longer active), deleting the
   shared `typer/` directory
4. Result: `typer` metadata still shows it's installed, but the actual module files are gone

## Workaround

```bash
uv sync --extra extra2 --reinstall
```
