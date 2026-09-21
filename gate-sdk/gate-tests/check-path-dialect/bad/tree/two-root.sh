#!/usr/bin/env bash
set -uo pipefail
KIT="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"
REPO="$(cd "$(git rev-parse --show-toplevel 2>/dev/null)" && pwd -P)"
echo "$REPO/${KIT#"$REPO"/}"
