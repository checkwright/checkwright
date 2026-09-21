#!/usr/bin/env bash
set -euo pipefail
roster=(x y z)
hit="$(echo "${roster[*]}" | grep -m1 -o "$1")"
echo "$hit"
