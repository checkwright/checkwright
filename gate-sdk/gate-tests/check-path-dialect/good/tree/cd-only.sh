#!/usr/bin/env bash
# A site that means to BE at the root and binds none. The cd consumes the produced value and
# nothing downstream reads a root, so no read-back is owed and none is written.
set -uo pipefail

cd "$(git rev-parse --show-toplevel 2>/dev/null || pwd)" 2>/dev/null || exit 0
ls
