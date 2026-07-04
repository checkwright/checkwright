#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §run-gate-tests — golden-fixture test runner for the check-* gate family
#
# usage: run-gate-tests.sh [tests-dir [gate-dir...]]
#   tests-dir defaults to $GATE_SDK_TESTS_DIR (default: <gates-dir>/gate-tests).
#   Each <tests-dir>/<gate>/ holds a good/ + bad/ case pair; the gate under
#   test resolves against the given gate-dirs (default: the consumer gates dir,
#   then the kit's checks/). <tests-dir>/*.test.sh unit tests also run.
#
# The kit's own fixtures run as:
#   gate-sdk/bin/run-gate-tests.sh gate-sdk/gate-tests gate-sdk/checks
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

GATES_DIR_DEFAULT="$(gate_sdk_gates_dir)"
TESTS_DIR="${1:-${GATE_SDK_TESTS_DIR:-$GATES_DIR_DEFAULT/gate-tests}}"
if [[ $# -gt 1 ]]; then
    GATE_DIRS=("${@:2}")
else
    GATE_DIRS=("$GATES_DIR_DEFAULT" "$SDK/checks")
fi
# Absolute-ize so resolution survives the per-case cd.
resolved=()
for d in "${GATE_DIRS[@]}"; do
    [[ -d "$d" ]] && resolved+=("$(cd "$d" && pwd)")
done
GATE_DIRS=("${resolved[@]+"${resolved[@]}"}")

[[ -d "$TESTS_DIR" ]] || { echo "run-gate-tests: no fixture tree at $TESTS_DIR" >&2; exit 2; }

pairs=0
logic_fail=0
harness_fail=0

run_case() {
    local gate="$1" casedir="$2" want="$3" expect="$4"
    local gate_path
    if ! gate_path="$(gate_resolve "$gate" "${GATE_DIRS[@]+"${GATE_DIRS[@]}"}")"; then
        echo "  HARNESS: $gate.sh resolves in none of: ${GATE_DIRS[*]}"
        return 2
    fi
    if [[ ! -x "$gate_path" ]]; then
        echo "  HARNESS: $gate_path is not executable"
        return 2
    fi

    local -a args=()
    if [[ -f "$casedir/args" ]]; then
        # shellcheck disable=SC2207
        args=($(grep -v '^#' "$casedir/args"))
    fi

    local out rc
    out="$( cd "$casedir" && "$gate_path" "${args[@]+"${args[@]}"}" 2>&1 )"
    rc=$?

    if [[ "$rc" -eq 2 ]]; then
        echo "  HARNESS: $gate $casedir exited 2 (gate could not run / malformed fixture):"
        printf '    %s\n' "$out"
        return 2
    fi
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL: $gate $(basename "$casedir") expected exit $want, got $rc"
        printf '    %s\n' "$out"
        return 1
    fi
    if [[ -n "$expect" ]] && ! grep -qF -- "$expect" <<<"$out"; then
        echo "  FAIL: $gate $(basename "$casedir") exit $rc OK but output lacks expected substring:"
        echo "        want: $expect"
        printf '    %s\n' "$out"
        return 1
    fi
    return 0
}

shopt -s nullglob
gate_dirs=("$TESTS_DIR"/*/)
shopt -u nullglob
if [[ ${#gate_dirs[@]} -eq 0 ]]; then
    echo "run-gate-tests: no gate fixture dirs under $TESTS_DIR" >&2
    exit 2
fi

for d in "${gate_dirs[@]}"; do
    gate="$(basename "$d")"
    good="$d/good"
    bad="$d/bad"
    if [[ ! -d "$good" || ! -d "$bad" ]]; then
        echo "  HARNESS: $gate is missing a good/ or bad/ case dir"
        harness_fail=$((harness_fail + 1))
        continue
    fi
    if [[ ! -f "$bad/expect.txt" ]]; then
        echo "  HARNESS: $gate bad/ has no expect.txt (a rejection substring is required)"
        harness_fail=$((harness_fail + 1))
        continue
    fi

    good_expect=""
    [[ -f "$good/expect.txt" ]] && good_expect="$(cat "$good/expect.txt")"
    bad_expect="$(cat "$bad/expect.txt")"

    pairs=$((pairs + 1))

    run_case "$gate" "$good" 0 "$good_expect"; gc=$?
    run_case "$gate" "$bad" 1 "$bad_expect"; bc=$?
    [[ "$gc" -eq 1 || "$bc" -eq 1 ]] && logic_fail=$((logic_fail + 1))
    [[ "$gc" -eq 2 || "$bc" -eq 2 ]] && harness_fail=$((harness_fail + 1))
done

unit=0
unit_fail=0
shopt -s nullglob
unit_tests=("$TESTS_DIR"/*.test.sh)
shopt -u nullglob
for t in "${unit_tests[@]}"; do
    unit=$((unit + 1))
    if out="$(bash "$t" 2>&1)"; then
        :
    else
        echo "  FAIL: $(basename "$t")"
        printf '    %s\n' "$out"
        unit_fail=$((unit_fail + 1))
    fi
done

echo
if [[ "$harness_fail" -gt 0 ]]; then
    echo "GATE-TESTS: $harness_fail harness/fixture error(s) (malformed fixtures — could not test)"
    exit 2
fi
if [[ "$logic_fail" -gt 0 || "$unit_fail" -gt 0 ]]; then
    echo "GATE-TESTS: $logic_fail of $pairs gate(s) + $unit_fail of $unit unit test(s) misbehaved"
    exit 1
fi
echo "GATE-TESTS: clean ($pairs pairs, $unit unit tests)"
exit 0
