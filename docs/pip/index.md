# The pip interface

uv provides a drop-in replacement for common `pip`, `pip-tools`, and `virtualenv` commands. These
commands work directly with the virtual environment, in contrast to uv's primary interfaces where
the virtual environment is managed automatically. The `uv pip` interface exposes the speed and
functionality of uv to power users and projects that are not ready to transition away from `pip` and
`pip-tools`.

The following sections discuss the basics of using `uv pip`:

- [Creating and using environments](/uv/pip/environments.md)
- [Installing and managing packages](/uv/pip/packages.md)
- [Inspecting environments and packages](/uv/pip/inspection.md)
- [Declaring package dependencies](/uv/pip/dependencies.md)
- [Locking and syncing environments](/uv/pip/compile.md)

Please note these commands do not _exactly_ implement the interfaces and behavior of the tools they
are based on. The further you stray from common workflows, the more likely you are to encounter
differences. Consult the [pip-compatibility guide](/uv/pip/compatibility.md) for details.
