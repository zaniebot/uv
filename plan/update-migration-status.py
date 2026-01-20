#!/usr/bin/env python3
"""
Comprehensively update MIGRATION.md based on actual migrated tests in mdtest files.

Extracts test references from:
1. <!-- from file.rs::test_name --> comments
2. <!-- Derived from [`file::test_name`] --> comments

Then marks corresponding tests as [x] in MIGRATION.md
"""

import re
import subprocess
from pathlib import Path

# Find all mdtest files
mdtest_dir = Path("/Users/zb/workspace/uv/test/uv")
md_files = list(mdtest_dir.glob("**/*.md"))
md_files = [f for f in md_files if f.name != "readme.md"]

print(f"Found {len(md_files)} mdtest files")

# Extract test references
migrated_tests = set()

for md_file in md_files:
    content = md_file.read_text()

    # Pattern 1: <!-- from file.rs::test_name -->
    for match in re.finditer(r"<!-- from ([^:]+)::([^\s]+)", content):
        file_part = match.group(1).strip()
        test_part = match.group(2).strip()
        # Normalize to file.rs::test_name format
        if not file_part.endswith(".rs"):
            file_part += ".rs"
        test_ref = f"{file_part}::{test_part}"
        migrated_tests.add(test_ref)

    # Pattern 2: Derived from [`file::test_name`]
    for match in re.finditer(r"\[`([^:]+)::([^`]+)`\]", content):
        file_part = match.group(1).strip()
        test_part = match.group(2).strip()
        # Normalize to file.rs::test_name format
        if not file_part.endswith(".rs"):
            file_part += ".rs"
        test_ref = f"{file_part}::{test_part}"
        migrated_tests.add(test_ref)

print(f"\nExtracted {len(migrated_tests)} unique test references")

# Show breakdown by file
from collections import Counter

file_counts = Counter()
for test in migrated_tests:
    file_name = test.split("::")[0]
    file_counts[file_name] += 1

print("\nTop 20 files with migrated tests:")
for file_name, count in file_counts.most_common(20):
    print(f"  {file_name}: {count} tests")

# Read MIGRATION.md
migration_file = Path("/Users/zb/workspace/uv/plan/MIGRATION.md")
lines = migration_file.read_text().splitlines(keepends=True)

# Update lines
updated_count = 0
already_marked = 0

for i, line in enumerate(lines):
    # Match lines like "- [ ] file.rs::test_name" or "- [x] file.rs::test_name"
    match = re.match(r"^- \[(.)\] ([^:]+::[^\s]+)", line)
    if match:
        status = match.group(1)
        test_name = match.group(2).strip()

        if test_name in migrated_tests:
            if status == " ":
                # Mark as migrated
                lines[i] = line.replace("- [ ]", "- [x]", 1)
                updated_count += 1
            else:
                # Already marked
                already_marked += 1

print(f"\nUpdating MIGRATION.md:")
print(f"  Already marked: {already_marked}")
print(f"  Newly marked: {updated_count}")
print(f"  Total migrated: {already_marked + updated_count}")

# Write back
migration_file.write_text("".join(lines))

print("\nMIGRATION.md updated successfully!")

# Calculate new progress
total_tests = len([l for l in lines if re.match(r"^- \[[x ]\]", l)])
completed_tests = len([l for l in lines if re.match(r"^- \[x\]", l)])
print(
    f"\nNew progress: {completed_tests} / {total_tests} ({100 * completed_tests / total_tests:.1f}%)"
)
