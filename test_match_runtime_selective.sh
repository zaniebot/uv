#!/bin/bash

# Demonstration of selective match-runtime invalidation

echo "=== Demonstrating selective match-runtime invalidation ==="
echo ""
echo "This script shows how packages with match-runtime build dependencies"
echo "are only rebuilt when the locked versions actually change."
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
echo "========================================="
uv sync --preview-features extra-build-dependencies
echo ""

echo "Step 2: Sync again with NO changes"
echo "========================================="
echo "Expected: Child package should NOT be rebuilt (just audited)"
uv sync --preview-features extra-build-dependencies
echo ""

echo "Step 3: Change parent to use anyio==3.7.1"
echo "========================================="
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

echo "Step 4: Sync with changed dependency"
echo "========================================="
echo "Expected: Child package SHOULD be rebuilt automatically"
uv sync --preview-features extra-build-dependencies
echo ""

echo "Step 5: Sync again with NO changes"
echo "========================================="
echo "Expected: Child package should NOT be rebuilt (just audited)"
uv sync --preview-features extra-build-dependencies

echo ""
echo "=== Demo complete ==="
echo ""
echo "Summary:"
echo "- Child package was rebuilt ONLY when anyio version changed"
echo "- No unnecessary rebuilds when dependencies are unchanged"
echo "- No need for manual --reinstall flags"