# Run Modules

Tests for running Python modules with `uv run -m`.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Running modules

### Run built-in module

<!-- from run.rs::run_module -->

Run a built-in Python module.

```console
$ uv run -m __hello__
success: true
exit_code: 0
----- stdout -----
Hello world!

----- stderr -----
```

### Run http.server module

<!-- from run.rs::run_module -->

Run a standard library module with arguments.

```console
$ uv run -m http.server -h
success: true
exit_code: 0
----- stdout -----
usage: server.py [-h] [--cgi] [-b ADDRESS] [-d DIRECTORY] [-p VERSION] [port]

positional arguments:
  port                  bind to this port (default: 8000)

options:
  -h, --help            show this help message and exit
  --cgi                 run as CGI server
  -b ADDRESS, --bind ADDRESS
                        bind to this address (default: all interfaces)
  -d DIRECTORY, --directory DIRECTORY
                        serve this directory (default: current directory)
  -p VERSION, --protocol VERSION
                        conform to this HTTP version (default: HTTP/1.0)

----- stderr -----
```

## Error handling

### Cannot run module from stdin

<!-- from run.rs::run_module_stdin -->

Running a module from stdin is not supported.

```console
$ uv run -m -
success: false
exit_code: 2
----- stdout -----

----- stderr -----
error: Cannot run a Python module from stdin
```
