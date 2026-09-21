#!/usr/bin/env bash
set -eu
members=(a b c)
if printf '%s\n' "${members[@]}" | grep -qxF "$1"; then
    echo present
fi
