#!/usr/bin/env bash
# graph: couples=docs/enforcement.md,scripts/gates.list,scripts/*.sh,kit:*.sh,scripts/kpis.list,.claude/settings.json,.github/workflows/*.yml dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-enforcement-fresh — docs/enforcement.md is the byte-fresh projection of enforcement-map.sh --emit
#
# usage: check-enforcement-fresh.sh [projection-file] [emit-file]
#   bare: compare docs/enforcement.md against `enforcement-map.sh --emit`.
#   two args: compare projection-file to a pre-baked emit-file (hermetic fixture).
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

PROJECTION="${1:-docs/enforcement.md}"
EMIT_SRC="${2:-}"

[[ -f "$PROJECTION" ]] || { echo "check-enforcement-fresh: projection not found: $PROJECTION" >&2; exit 2; }

if [[ -n "$EMIT_SRC" ]]; then
    [[ -f "$EMIT_SRC" ]] || { echo "check-enforcement-fresh: emit source not found: $EMIT_SRC" >&2; exit 2; }
    emitted="$(cat "$EMIT_SRC")"; st=$?
    fail_closed "$st" check-enforcement-fresh cat
else
    EMITTER="$SDK/bin/enforcement-map.sh"
    [[ -x "$EMITTER" ]] || { echo "check-enforcement-fresh: emitter not found: $EMITTER" >&2; exit 2; }
    emitted="$(bash "$EMITTER" --emit)"; st=$?
    fail_closed "$st" check-enforcement-fresh enforcement-map
fi

if [[ "$emitted" != "$(cat "$PROJECTION")" ]]; then
    echo "check-enforcement-fresh: $PROJECTION is stale vs the enforcement-map emitter:"
    diff <(printf '%s\n' "$emitted") "$PROJECTION" | head -20 || true
    echo "  help: regenerate — bash gate-sdk/bin/enforcement-map.sh --emit > docs/enforcement.md"
    exit 1
fi
echo "ENFORCEMENT-FRESH: clean ($PROJECTION byte-matches the enforcement-map emitter)"
exit 0
