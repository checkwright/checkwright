#!/usr/bin/env bash
set -uo pipefail
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT" || exit 1
echo "$REPO_ROOT/sub"
