#!/usr/bin/env bash
set -uo pipefail
LIB="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"
cd "$(pwd -P)"
TOP="$(cd "$LIB/.." && pwd)"
LIB_UP="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"
echo "${LIB%/lib}" "$LIB_UP/x" "$TOP"
