#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §The turn-end liveness hook (template) — this repo's reader for
# DELEGATION_KIT_LIVENESS_CMD, a path run with the scratch dir as its only argument; the gate is
# name-addressed, so the adapter is consumer-side (evidence-kit/SPEC.md §check-evidence-manifest)
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../gate-sdk/lib/gate.sh
source "$HERE/../gate-sdk/lib/gate.sh"

# spec: delegation-kit/SPEC.md §The turn-end liveness hook (template) — a linked worktree carries
# no build output, so the configured binary is absent there and this reader would refuse before
# reading a record; resolve it against the main checkout, and only in that one shape
if [[ ! -x "$GATE_SDK_NATIVE_BIN" && "$GATE_SDK_NATIVE_BIN" != /* ]]; then
    _plr_dir="$( { cd "$(git rev-parse --git-dir 2>/dev/null)" && pwd -P; } 2>/dev/null)"
    _plr_common="$( { cd "$(git rev-parse --git-common-dir 2>/dev/null)" && pwd -P; } 2>/dev/null)"
    # comment-tier-exempt: the two answers are the same directory spelled differently from a main-checkout SUBdir (`/abs/.git` vs `../.git`), which is why both are resolved with `pwd -P` before the compare rather than string-matched — probed, not assumed
    if [[ -n "$_plr_common" && "$_plr_dir" != "$_plr_common" ]]; then
        _plr_main="${_plr_common%/*}"
        [[ -x "$_plr_main/$GATE_SDK_NATIVE_BIN" ]] \
            && export GATE_SDK_NATIVE_BIN="$_plr_main/$GATE_SDK_NATIVE_BIN"
    fi
    unset _plr_dir _plr_common _plr_main
fi

exec bash "$HERE/gate-exec.sh" check-producer-liveness "$@"
