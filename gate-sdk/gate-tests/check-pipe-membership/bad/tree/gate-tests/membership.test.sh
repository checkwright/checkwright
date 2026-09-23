#!/usr/bin/env bash
set -euo pipefail
cases=(a b c)
if printf '%s\n' "${cases[@]}" | grep -qxF "$1"; then
    echo ran
fi
