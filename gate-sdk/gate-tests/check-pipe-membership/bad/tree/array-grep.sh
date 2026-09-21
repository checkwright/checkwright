#!/usr/bin/env bash
set -euo pipefail
members=(a b c)
if printf '%s\n' "${members[@]}" | grep -qxF "$1"; then
    echo present
fi
