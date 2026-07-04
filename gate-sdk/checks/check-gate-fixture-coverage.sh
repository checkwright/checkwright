#!/usr/bin/env bash
# graph: couples=scripts/gates.list,scripts/*.sh,scripts/gate-tests/*,gate-sdk/*.sh,lifecycle-kit/*.sh,queue-kit/*.sh dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-gate-fixture-coverage — every gates.list member has a fixture pair or a no-fixture opt-out
#
# usage: check-gate-fixture-coverage.sh [gates-dir [tests-dir...]]
#   Fixture pairs are searched across the given tests dirs. Default:
#   <gates-dir>/gate-tests plus each vendored kit's own gate-tests/ (a kit's
#   shipped gates carry their pairs there).
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

DIR="${1:-$(gate_sdk_gates_dir)}"
if [[ $# -gt 1 ]]; then
    TESTS_DIRS=("${@:2}")
else
    TESTS_DIRS=("${GATE_SDK_TESTS_DIR:-$DIR/gate-tests}")
    while IFS= read -r k; do TESTS_DIRS+=("$k/gate-tests"); done < <(gate_kit_roots)
fi
RESOLVE_DIRS=("$DIR")
while IFS= read -r k; do RESOLVE_DIRS+=("$k/checks"); done < <(gate_kit_roots)
LIST="$DIR/gates.list"
[[ -f "$LIST" ]] || { echo "check-gate-fixture-coverage: no registry at $LIST" >&2; exit 2; }

members="$(gates_list_members "$LIST")"
[[ -n "$members" ]] || { echo "check-gate-fixture-coverage: no members parsed from $LIST" >&2; exit 2; }

fixture_dir_for() {
    local m="$1" t
    for t in "${TESTS_DIRS[@]}"; do
        [[ -d "$t/$m" ]] && { printf '%s\n' "$t/$m"; return 0; }
    done
    return 1
}

neither=()
halfpair=()
fixtured=0
optout=0
total=0
while IFS= read -r m; do
    [[ -n "$m" ]] || continue
    total=$((total + 1))
    if gd="$(fixture_dir_for "$m")"; then
        if [[ -d "$gd/good" && -d "$gd/bad" ]]; then
            fixtured=$((fixtured + 1))
        elif [[ -d "$gd/good" ]]; then
            halfpair+=("$m ($gd/ has good/ but no bad/)")
        else
            halfpair+=("$m ($gd/ has bad/ but no good/)")
        fi
        continue
    fi
    if ! src="$(gate_resolve "$m" "${RESOLVE_DIRS[@]}")"; then
        neither+=("$m (no fixture pair, and source resolves in none of: ${RESOLVE_DIRS[*]})")
        continue
    fi
    if grep -Eq '^# no-fixture:' "$src"; then
        optout=$((optout + 1))
    else
        neither+=("$m (no fixture pair under: ${TESTS_DIRS[*]}; no '# no-fixture:' opt-out)")
    fi
done <<< "$members"

if [[ ${#neither[@]} -gt 0 || ${#halfpair[@]} -gt 0 ]]; then
    if [[ ${#neither[@]} -gt 0 ]]; then
        echo "check-gate-fixture-coverage: gates.list member(s) with neither a fixture pair"
        echo "nor a '# no-fixture:' opt-out (gate-sdk/SPEC.md §Fixture-pair discipline — a"
        echo "gate ships a good/bad fixture pair on write):"
        echo ""
        for m in "${neither[@]}"; do echo "  $m"; done
        echo ""
        echo "  help: add <tests-dir>/<gate>/{good,bad}/ (run by run-gate-tests.sh), OR"
        echo "        a '# no-fixture: <why>' header line for a whole-tree scanner with"
        echo "        no synthetic-dir mode (stopgap on a fixture-capable gate -> file"
        echo "        a fixture-backfill debt task and list it there)."
    fi
    if [[ ${#halfpair[@]} -gt 0 ]]; then
        [[ ${#neither[@]} -gt 0 ]] && echo ""
        echo "check-gate-fixture-coverage: gates.list member(s) with a half-built fixture dir"
        echo "(a pair needs both good/ and bad/):"
        echo ""
        for m in "${halfpair[@]}"; do echo "  $m"; done
        echo ""
        echo "  help: add the missing half under <tests-dir>/<gate>/, or remove"
        echo "        the partial dir and add a '# no-fixture:' opt-out instead."
    fi
    exit 1
fi

echo "GATE-FIXTURE-COVERAGE: clean ($total members: $fixtured fixtured, $optout opted-out)"
exit 0
