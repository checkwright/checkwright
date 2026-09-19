#!/usr/bin/env bash
# Hermetic fixture: a shared build body emitting one digest through whichever hasher the host carries.
set -euo pipefail
out="$2/$1"
if command -v sha256sum >/dev/null 2>&1; then
    ( cd "$out" && sha256sum gate > gate.sha256 )
else
    ( cd "$out" && shasum -a 256 gate > gate.sha256 )
fi
