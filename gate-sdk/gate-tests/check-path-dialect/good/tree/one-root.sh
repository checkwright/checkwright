#!/usr/bin/env bash
set -uo pipefail
SDK="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"
echo "$SDK/lib" "${SDK%/*}"
