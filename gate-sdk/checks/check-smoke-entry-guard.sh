#!/usr/bin/env bash
# graph: couples=kit:smoke/install.sh,kit:smoke/violation.sh dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-smoke-entry-guard — every mutating smoke script (install.sh, violation.sh) carries the ${SMOKE_KIT_ROOT:?} entry-point guard so a bare run refuses instead of mutating the caller's tree
#
# usage: check-smoke-entry-guard.sh [root]
#   bare: sweep gate_kit_roots against the git toplevel; root: resolve relative
#   kit roots against a fixture tree (the case dir's gate-sdk-config.sh names them).
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

ROOT="${1:-}"
if [[ -z "$ROOT" ]]; then
    ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" \
        || { echo "check-smoke-entry-guard: not a git repository and no root argument" >&2; exit 2; }
fi
[[ -d "$ROOT" ]] || { echo "check-smoke-entry-guard: root not found: $ROOT" >&2; exit 2; }

GUARD='${SMOKE_KIT_ROOT:?'

mapfile -t KIT_ROOTS < <(gate_kit_roots)
[[ ${#KIT_ROOTS[@]} -gt 0 ]] || { echo "check-smoke-entry-guard: no kit roots enumerated" >&2; exit 2; }

findings=()
swept=0
for r in "${KIT_ROOTS[@]}"; do
    r="${r%/}"
    abs="$r"
    [[ "$abs" == /* ]] || abs="$ROOT/$r"
    kit="${r##*/}"
    [[ -d "$abs/smoke" ]] || continue
    for name in install.sh violation.sh; do
        f="$abs/smoke/$name"
        [[ -e "$f" ]] || continue
        [[ -r "$f" ]] || { echo "check-smoke-entry-guard: unreadable smoke script: $kit/smoke/$name" >&2; exit 2; }
        swept=$((swept + 1))
        grep -qF -- "$GUARD" "$f" || findings+=("$kit/smoke/$name: no \${SMOKE_KIT_ROOT:?…} entry-point guard")
    done
done

if [[ ${#findings[@]} -gt 0 ]]; then
    echo "check-smoke-entry-guard: mutating smoke script(s) missing the entry-point guard:"
    for f in "${findings[@]}"; do echo "  $f"; done
    echo "  help: add ': \"\${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}\"' right after"
    echo "        'set -euo pipefail' and before the first mutating command, so a bare run"
    echo "        refuses instead of writing into the caller's repo (gate-sdk/SPEC.md §Consumer smoke)."
    exit 1
fi

echo "SMOKE-ENTRY-GUARD: clean ($swept mutating smoke script(s) carry the entry-point guard)"
exit 0
