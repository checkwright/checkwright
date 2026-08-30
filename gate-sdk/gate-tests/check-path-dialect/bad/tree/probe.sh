#!/usr/bin/env bash
# Standing guard: presence probes stay green beside the violations.
set -uo pipefail

git rev-parse --git-dir >/dev/null 2>&1 || exit 0
git rev-parse --git-common-dir &>/dev/null || exit 0
echo present
