#!/usr/bin/env bash
# graph: couples=docs/footprint.md,CLAUDE.md,kit:templates dir=one valve=none tier=precommit trigger=docs/footprint.md,CLAUDE.md,kit:templates
# spec: context-kit/SPEC.md §check-footprint-fresh — docs/footprint.md is the byte-fresh projection of footprint.sh --emit
#
# usage: check-footprint-fresh.sh [projection-file] [emit-file]
#   bare: compare docs/footprint.md against `footprint.sh --emit`.
#   two args: compare projection-file to a pre-baked emit-file (hermetic fixture).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

PROJECTION="${1:-docs/footprint.md}"
EMIT_SRC="${2:-}"

[[ -f "$PROJECTION" ]] || { echo "check-footprint-fresh: projection not found: $PROJECTION" >&2; exit 2; }

if [[ -n "$EMIT_SRC" ]]; then
    [[ -f "$EMIT_SRC" ]] || { echo "check-footprint-fresh: emit source not found: $EMIT_SRC" >&2; exit 2; }
    emitted="$(cat "$EMIT_SRC")"; st=$?
    fail_closed "$st" check-footprint-fresh cat
else
    FOOT="$KIT/bin/footprint.sh"
    [[ -x "$FOOT" ]] || { echo "check-footprint-fresh: emitter not found: $FOOT" >&2; exit 2; }
    emitted="$(bash "$FOOT" --emit)"; st=$?
    fail_closed "$st" check-footprint-fresh footprint
fi

if [[ "$emitted" != "$(cat "$PROJECTION")" ]]; then
    echo "check-footprint-fresh: $PROJECTION is stale vs the footprint emitter:"
    diff <(printf '%s\n' "$emitted") "$PROJECTION" | head -20 || true
    echo "  help: regenerate — bash context-kit/bin/footprint.sh --emit > docs/footprint.md"
    exit 1
fi
echo "FOOTPRINT-FRESH: clean ($PROJECTION byte-matches the footprint emitter)"
exit 0
