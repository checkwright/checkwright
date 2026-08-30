#!/usr/bin/env bash
set -uo pipefail
COMMON="$(git rev-parse --git-common-dir 2>/dev/null)"
echo "${COMMON%/*}/objects"
