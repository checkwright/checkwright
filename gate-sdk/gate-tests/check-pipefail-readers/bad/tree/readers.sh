#!/usr/bin/env bash
# One site per early-exit reader, under pipefail from the set.
set -euo pipefail
ls | grep -q x
ls | grep -m 1 x
ls | grep -l x
ls | head -n 3
ls | read -r first
ls | sed '/x/q'
ls | awk '/x/ { print; exit }'
ls | while read -r l; do
    [[ "$l" == x ]] && break
done
v="$(ls |
    grep -Fxq x && echo y)"
ls | grep -q x # pipefail-ok:
