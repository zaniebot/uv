# Script Dependencies

Tests for adding and removing dependencies in PEP 723 scripts.

```toml
# mdtest

[environment]
python-version = "3.12"
required-features = "pypi"
```

## Adding dependencies to scripts

### Add dependency to script

<!-- from edit.rs::add_script -->

`uv add --script` adds dependencies to PEP 723 scripts.

```python script.py
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "requests<3",
#   "rich",
# ]
# ///

import requests
from rich.pretty import pprint

resp = requests.get("https://peps.python.org/api/peps.json")
data = resp.json()
pprint([(k, v["title"]) for k, v in data.items()][:10])
```

```console
$ uv add anyio --script script.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 11 packages in [TIME]
```

The dependency is added to the script:

```python title="script.py" snapshot=true
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "anyio>=4.3.0",
#   "requests<3",
#   "rich",
# ]
# ///

import requests
from rich.pretty import pprint

resp = requests.get("https://peps.python.org/api/peps.json")
data = resp.json()
pprint([(k, v["title"]) for k, v in data.items()][:10])
```

### Add to script without metadata

<!-- from edit.rs::add_script_without_metadata_table -->

Adding to a script without existing metadata creates it.

```python hello.py
print("Hello, world!")
```

```console
$ uv add anyio --script hello.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 3 packages in [TIME]
```

The script now has PEP 723 metadata:

```python title="hello.py" snapshot=true
# /// script
# requires-python = ">=3.12"
# dependencies = [
#     "anyio>=4.3.0",
# ]
# ///

print("Hello, world!")
```

## Removing dependencies from scripts

### Remove from script

<!-- from edit.rs::remove_script -->

`uv remove --script` removes dependencies from PEP 723 scripts.

```python script.py
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "requests<3",
#   "rich",
# ]
# ///

import requests
from rich.pretty import pprint

resp = requests.get("https://peps.python.org/api/peps.json")
data = resp.json()
pprint([(k, v["title"]) for k, v in data.items()][:10])
```

```console
$ uv remove rich --script script.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Resolved 5 packages in [TIME]
```

The dependency is removed:

```python title="script.py" snapshot=true
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "requests<3",
# ]
# ///

import requests
from rich.pretty import pprint

resp = requests.get("https://peps.python.org/api/peps.json")
data = resp.json()
pprint([(k, v["title"]) for k, v in data.items()][:10])
```

### Remove last dependency from script

<!-- from edit.rs::remove_last_dep_script -->

Removing the last dependency from a script leaves the metadata intact.

```python script.py
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "rich",
# ]
# ///

from rich.pretty import pprint

pprint("Hello, world!")
```

```console
$ uv remove rich --script script.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
Updated `script.py`
```

### Add dependency with bounds to script

<!-- from edit.rs::add_script_bounds -->

The `--bounds` flag applies version constraints when adding to a script.

```python script.py
print("Hello, world!")
```

```console
$ uv add anyio --bounds minor --script script.py
success: true
exit_code: 0
----- stdout -----

----- stderr -----
warning: The `bounds` option is in preview and may change in any future release. Pass `--preview-features add-bounds` to disable this warning.
Resolved 3 packages in [TIME]
```
