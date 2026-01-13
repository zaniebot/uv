#!/bin/bash
set -euo pipefail

# Sync fork's main branch with upstream and rebase current branch
# Only runs if the repository is a fork

# Check if we're in a git repository
if ! git rev-parse --git-dir &> /dev/null; then
    echo "Not in a git repository, skipping sync"
    exit 0
fi

# Get repo name from origin remote (handles both HTTPS and SSH URLs)
ORIGIN_URL=$(git remote get-url origin 2>/dev/null || echo "")
if [ -z "${ORIGIN_URL}" ]; then
    echo "No origin remote found, skipping sync"
    exit 0
fi

# Extract owner/repo from URL
# Handles: github.com URLs, SSH URLs, and local proxy URLs (/git/owner/repo)
if echo "${ORIGIN_URL}" | grep -q 'github\.com'; then
    REPO_NAME=$(echo "${ORIGIN_URL}" | sed -E 's#.*github\.com[:/]([^/]+/[^/]+?)(\.git)?$#\1#' | sed 's/\.git$//')
elif echo "${ORIGIN_URL}" | grep -q '/git/'; then
    REPO_NAME=$(echo "${ORIGIN_URL}" | sed -E 's#.*/git/([^/]+/[^/]+)$#\1#')
else
    echo "Could not parse repo name from origin URL: ${ORIGIN_URL}, skipping sync"
    exit 0
fi

if [ -z "${REPO_NAME}" ] || ! echo "${REPO_NAME}" | grep -q '/'; then
    echo "Could not parse repo name from origin URL, skipping sync"
    exit 0
fi

# Check if this repo is a fork
if ! gh repo view "${REPO_NAME}" --json isFork --jq '.isFork' 2>/dev/null | grep -q 'true'; then
    echo "Not a fork, skipping sync"
    exit 0
fi

echo "Syncing fork ${REPO_NAME} with upstream..."

# Sync the fork's default branch with upstream
if ! gh repo sync "${REPO_NAME}"; then
    echo "Warning: Failed to sync fork with upstream, continuing anyway"
    exit 0
fi

# Fetch the updated main branch
git fetch origin main

# Rebase current branch onto the updated main
CURRENT_BRANCH=$(git branch --show-current)
if [ -n "${CURRENT_BRANCH}" ] && [ "${CURRENT_BRANCH}" != "main" ]; then
    # Check for uncommitted changes
    if ! git diff --quiet || ! git diff --cached --quiet; then
        echo "Warning: Uncommitted changes detected, skipping rebase"
        exit 0
    fi

    echo "Rebasing ${CURRENT_BRANCH} onto origin/main..."
    if ! git rebase origin/main; then
        echo "Warning: Rebase failed, aborting rebase"
        git rebase --abort 2>/dev/null || true
        exit 0
    fi
fi

echo "Sync complete"
