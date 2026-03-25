$ErrorActionPreference = "Stop"

# Test that uv can install packages with entrypoints on Windows NanoServer.
#
# Windows NanoServer does not support the `BeginUpdateResourceW`,
# `UpdateResourceW`, and `EndUpdateResourceW` APIs that are used to embed
# metadata in trampoline executables. uv must fall back to the legacy
# trampoline format on this platform.
#
# See: https://github.com/astral-sh/uv/issues/18663

Write-Host "Testing uv on Windows NanoServer"

# Copy the mounted binary into a writable location on PATH
Copy-Item C:\uv\uv.exe C:\Windows\System32\uv.exe

# Verify uv is available
uv --version
if ($LASTEXITCODE -ne 0) { throw "uv --version failed" }

# Create a temporary project
uv init C:\test-project
if ($LASTEXITCODE -ne 0) { throw "uv init failed" }

# Install a package that has console_scripts entry points.
# `docxcompose` is the package from the original bug report.
uv --directory C:\test-project add docxcompose
if ($LASTEXITCODE -ne 0) { throw "uv add docxcompose failed" }

Write-Host "Successfully installed package with entrypoints on Windows NanoServer"
