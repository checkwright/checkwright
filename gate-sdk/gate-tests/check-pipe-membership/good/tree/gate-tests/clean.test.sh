#!/usr/bin/env bash
set -euo pipefail
cases=(a b c)
found=0
for c in "${cases[@]}"; do
    [[ "$c" == "$1" ]] && found=1
done
echo "$found"
