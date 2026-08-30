#!/usr/bin/env bash
set -uo pipefail
cd "$(git rev-parse --show-toplevel 2>/dev/null)" || exit 1
ROOT="$(pwd)"
echo "$ROOT/sub"
