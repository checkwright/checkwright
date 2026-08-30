#!/usr/bin/env bash
# The cwd-preserving subshell idiom: same crossing, without leaving the caller's directory.
set -uo pipefail

ROOT="$( { cd "$(git rev-parse --show-toplevel 2>/dev/null)" && pwd -P; } 2>/dev/null )"
echo "$ROOT/sub"
