#!/bin/bash

# Demonstration of match-runtime invalidation fix

echo "=== Demonstrating match-runtime build dependency invalidation ==="
echo ""
echo "This script shows how packages with match-runtime build dependencies"
echo "are now automatically rebuilt when the locked version changes."
echo ""

# Create a temporary directory
TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR"
echo "Working in: $TEMP_DIR"
echo ""

# Create child package
mkdir -p child/child
cat > child/child/__init__.py << 'EOF'
# Child package
EOF

cat > child/pyproject.toml << 'EOF'
[project]
name = "child"
version = "0.1.0"
requires-python = ">=3.12"

[build-system]
requires = ["hatchling", "anyio"]
build-backend = "hatchling.build"
EOF

# Create parent package
mkdir -p parent
cat > parent/__init__.py << 'EOF'
# Parent package
EOF

cat > pyproject.toml << 'EOF'
[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==4.0.0", "child"]

[tool.uv.sources]
child = { path = "child" }

[tool.uv.extra-build-dependencies]
child = [{ requirement = "anyio", match-runtime = true }]
EOF

echo "Step 1: Initial sync with anyio==4.0.0"
echo "----------------------------------------"
uv sync --preview-features extra-build-dependencies
echo ""

echo "Step 2: Change parent to use anyio==3.7.1"
echo "----------------------------------------"
cat > pyproject.toml << 'EOF'
[project]
name = "parent"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["anyio==3.7.1", "child"]

[tool.uv.sources]
child = { path = "child" }

[tool.uv.extra-build-dependencies]
child = [{ requirement = "anyio", match-runtime = true }]
EOF

echo "Step 3: Sync again (without --reinstall flag)"
echo "----------------------------------------"
echo "Notice: child package is automatically rebuilt with the new anyio version!"
echo ""
uv sync --preview-features extra-build-dependencies

echo ""
echo "=== Demo complete ==="
echo "The child package was automatically invalidated and rebuilt when"
echo "the match-runtime dependency (anyio) changed versions."
echo ""
echo "Before the fix, you would have needed to use --reinstall-package child"
echo "to force the rebuild."