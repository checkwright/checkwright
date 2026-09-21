#!/usr/bin/env bash
set -euo pipefail
members=(a b c)
found=0
for m in "${members[@]}"; do
    [[ "$m" == "$1" ]] && found=1
done
echo "$found"
