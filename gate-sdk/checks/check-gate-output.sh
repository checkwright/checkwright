#!/usr/bin/env bash
# graph: couples=scripts/gates.list,scripts/*.sh,gate-sdk/*.sh,lifecycle-kit/*.sh dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-gate-output — every gates.list member emits a machine-keyable success line and a help: remedy
#
# usage: check-gate-output.sh [gates-dir]
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

DIR="${1:-$(gate_sdk_gates_dir)}"
LIST="$DIR/gates.list"
[[ -f "$LIST" ]] || { echo "check-gate-output: no registry at $LIST" >&2; exit 2; }

members="$(gates_list_members "$LIST")"
[[ -n "$members" ]] || { echo "check-gate-output: no members parsed from $LIST" >&2; exit 2; }

RESOLVE_DIRS=("$DIR")
while IFS= read -r k; do RESOLVE_DIRS+=("$k/checks"); done < <(gate_kit_roots)

missing=()
no_help=()
total=0
while IFS= read -r m; do
    [[ -n "$m" ]] || continue
    total=$((total + 1))
    if ! src="$(gate_resolve "$m" "${RESOLVE_DIRS[@]}")"; then
        missing+=("$m (source resolves in none of: ${RESOLVE_DIRS[*]})")
        continue
    fi
    if ! grep -Eq '(echo|printf).*: clean' "$src"; then
        missing+=("$m (no '<NAME>: clean (…)' success line)")
    fi
    if ! grep -Eq '(echo|printf).*help:' "$src"; then
        no_help+=("$m (no 'help:' remedy line on the failure path)")
    fi
done <<< "$members"

if [[ ${#missing[@]} -gt 0 || ${#no_help[@]} -gt 0 ]]; then
    if [[ ${#missing[@]} -gt 0 ]]; then
        echo "check-gate-output: gates.list member(s) with no machine-keyable success line"
        echo "(gate-sdk/SPEC.md §Output contract — success is '^<NAME>: clean (<what>)'):"
        echo ""
        for m in "${missing[@]}"; do echo "  $m"; done
        echo ""
        echo "  help: emit exactly one success line on the exit-0 path —"
        echo "        echo \"<NAME>: clean (<what was checked>)\""
        echo "        where <NAME> is the gate's upper-token id (e.g. KIT-README)."
    fi
    if [[ ${#no_help[@]} -gt 0 ]]; then
        [[ ${#missing[@]} -gt 0 ]] && echo ""
        echo "check-gate-output: gates.list member(s) with no 'help:' remedy line"
        echo "(gate-sdk/SPEC.md §Output contract — every failure path names the fix):"
        echo ""
        for m in "${no_help[@]}"; do echo "  $m"; done
        echo ""
        echo "  help: add a remedy line on the failure path naming the concrete"
        echo "        action — echo \"  help: <do this to fix it>\" (one per failure class)."
    fi
    exit 1
fi

echo "GATE-OUTPUT: clean ($total gates.list members emit a '<NAME>: clean' success line + a 'help:' remedy)"
exit 0
