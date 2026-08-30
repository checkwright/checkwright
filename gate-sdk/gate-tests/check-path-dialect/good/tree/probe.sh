#!/usr/bin/env bash
# Repository-presence probes. Stdout goes to the void, so no value is bound and there is no root
# to cross; only the exit status is read, and an exit status has no dialect.
set -uo pipefail

git rev-parse --git-dir >/dev/null 2>&1 || exit 0
git rev-parse --git-common-dir &>/dev/null || exit 0
echo present
