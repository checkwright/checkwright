#!/usr/bin/env bash
set -euo pipefail
for f in *.txt; do
    printf '%s\n' "$f"
done | head -1
