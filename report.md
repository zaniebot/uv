

## terminology and concepts

Package: A deliverable unit of reusable code
Package resolution: Determining a set of compatibile packages
Package installation: Downloading and adding package code to the user's environment
Environment: A place for executable Python code
Virtual environment: An isolated environment for executable Python code, i.e., you can have more
than one
Package manager: A tool for resolving and installing packages
Project: A local unit of code and an enumeration of the packages it depends on
Project manager: A tool for

## pip

pip is written in Python and is the only package manager included in Python itself. It is
consequently the default package manager for many users. 

It defines expectations for other packaging tools in the space, i.e., there are many behaviors not
specified in Python packaging standards and other tools mimic pip to ensure they are meeting user
expectations. All new Python packaging standards require an integration plan with pip. 

It provides low-level utilities not included in uv, e.g., for bulk downloads of packages.

It only supports resolving and installing for a single platform (operating system, architecture,
etc.) at a time.

A significant portion of uv's adoption is just a faster interface matching pip's. uv success has
encouraged their team to invest more in performance and they're making tangible performance
improvements. The bulk of uv's performance improvements do not require Rust, and are rather design
and architectural decisions that could be adopted by pip.

It cannot create "virtual environments", but Python comes with a command to do so, e.g., `python -m
venv`.

It can install into arbitrary environments, but is most commonly used to target its own environment.

It only supports rudimentary "locking" of dependencies.

It does not support "project management" features like modifying the `pyproject.toml` and
automatically running commands in the context of an environment

It is useful for integration with other tools, because its presence can be relied on. It avoids
complex or hidden effects, which makes its behavior easy to reason about. It is often the first and
only packaging tool used by Python beginners. It is the defacto-standard of package management.

## pip-tools

pip-tools builds on top of pip to provide single-platform dependency locking and exact sync
installations. The ubiquitous `requirements.txt` format is based on pip and pip-tools. The
single-platform resolution can be limiting, requiring a requirements file for each platform,
however, it also simplifies interactions. Similar to pip, it does not attempt to support "project
management" features.

Similar to pip, this tool is easy to understand and has been central to Python package management
for a long time. It is very commonly used alongside pip.

## pipenv

Pipenv extends the concepts from `pip-tools` to include project management features. It has
dedicated formats for declaring (Pipfile) and locking (Pipfile.lock) dependencies. 

## poetry

## pipx

## pyenv

## virtualenv

## twine

## conda

## mamba

## pixi

## hatch

## pdm
