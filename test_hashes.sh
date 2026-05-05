#!/bin/bash

# Create a temporary directory for testing
TMPDIR=$(mktemp -d)
cd "$TMPDIR"

# Create a requirements file
cat > requirements.in << EOF
numpy==1.26.4
requests==2.31.0
EOF

echo "Testing hash modes with uv pip compile..."
echo

# Test with --hashes all (default)
echo "=== Testing --hashes all ==="
UV_BIN="${1:-uv}"
"$UV_BIN" pip compile requirements.in --generate-hashes --hashes all -o requirements-all.txt
ALL_HASHES=$(grep -c "^    --hash=" requirements-all.txt)
echo "Total hashes with --hashes all: $ALL_HASHES"
echo

# Test with --hashes compatible
echo "=== Testing --hashes compatible ==="
"$UV_BIN" pip compile requirements.in --generate-hashes --hashes compatible -o requirements-compatible.txt
COMPATIBLE_HASHES=$(grep -c "^    --hash=" requirements-compatible.txt)
echo "Total hashes with --hashes compatible: $COMPATIBLE_HASHES"
echo

# Compare the results
echo "=== Comparison ==="
echo "Hashes with 'all' mode: $ALL_HASHES"
echo "Hashes with 'compatible' mode: $COMPATIBLE_HASHES"
DIFF=$((ALL_HASHES - COMPATIBLE_HASHES))
echo "Difference: $DIFF fewer hashes in compatible mode"
echo

# Show a sample of the output
echo "=== Sample output from compatible mode ==="
head -20 requirements-compatible.txt

# Cleanup
cd ..
rm -rf "$TMPDIR"