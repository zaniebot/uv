# Virtual environments

Each Python installation has an environment that is active when Python is used. Packages can be
installed into an environment to make their modules available from your Python scripts. Generally,
it is considered best practice not to modify a Python installation's environment. This is especially
important for Python installations that come with the operating system which often manage the
packages themselves. A virtual environment is a lightweight way to isolate packages from a Python
installation's environment.

When using uv's primary interfaces (e.g., for [projects](../projects/index.md),
[tools](../tools.md), or [scripts](../../guides/scripts.md)), uv manages virtual environments for
you. The virtual environments are automatically created and updated when necessary. However, uv also
provides support for low-level workflows with drop-in replacements for common `pip`, `pip-tools`,
and `virtualenv` commands. These commands work directly with the virtual environment and are
available in the `uv pip` namespace. The `uv pip` interface exposes the speed and functionality of
uv to power users and projects that are not ready to transition away from `pip` and `pip-tools`.

The following sections discuss manually managing virtual environments:

- [Creating and using environments](./environments.md)
- [Installing and managing packages](./packages.md)
- [Inspecting environments and packages](./inspection.md)
- [Declaring package dependencies](./dependencies.md)
- [Locking and syncing environments](./compile.md)

Please note these commands do not _exactly_ implement the interfaces and behavior of the tools they
are based on. The further you stray from common workflows, the more likely you are to encounter
differences. Consult the [pip-compatibility guide](../../reference/compatibility.md) for details.

!!! important

    uv does not rely on or invoke pip. The pip interface is named as such to highlight its dedicated
    purpose of providing low-level commands that match pip's interface and to separate it from the
    rest of uv's commands which operate at a higher level of abstraction.
