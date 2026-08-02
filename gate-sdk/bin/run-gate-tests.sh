#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §run-gate-tests — golden-fixture test runner for the check-* gate family
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

GATES_DIR_DEFAULT="$(gate_sdk_gates_dir)"
TESTS_DIR="${1:-${GATE_SDK_TESTS_DIR:-$GATES_DIR_DEFAULT/gate-tests}}"
if [[ $# -gt 1 ]]; then
    GATE_DIRS=("${@:2}")
else
    mapfile -t GATE_DIRS < <(gate_check_dirs)
fi
resolved=()
for d in "${GATE_DIRS[@]}"; do
    [[ -d "$d" ]] && resolved+=("$(cd "$d" && pwd)")
done
GATE_DIRS=("${resolved[@]+"${resolved[@]}"}")

[[ -d "$TESTS_DIR" ]] || { echo "run-gate-tests: no fixture tree at $TESTS_DIR" >&2; exit 2; }

pairs=0
logic_fail=0
harness_fail=0

# spec: gate-sdk/SPEC.md §run-gate-tests — the case runs whatever the member dispatches to: gate_command yields the shell script's one-element argv or the binary's two-element `<binary> <name>`, so the fixture pair is the parity oracle across both substrates rather than a shell-only one. argv[0] is absolutized here because the case runs after a `cd` into its own dir and the binary knob's default is deliberately a repo-relative path (§lib/gate.sh).
run_case() {
    local gate="$1" casedir="$2" want="$3" expect="$4"
    local -a argv=()
    if ! mapfile -t argv < <(gate_command "$gate" "${GATE_DIRS[@]+"${GATE_DIRS[@]}"}"); then
        echo "  HARNESS: $gate resolves in none of: ${GATE_DIRS[*]}"
        return 2
    fi
    if [[ ${#argv[@]} -eq 0 ]]; then
        echo "  HARNESS: $gate resolves in none of: ${GATE_DIRS[*]}"
        return 2
    fi
    if [[ ! -x "${argv[0]}" ]]; then
        echo "  HARNESS: ${argv[0]} is not executable"
        return 2
    fi
    if [[ "${argv[0]}" != /* ]]; then
        argv[0]="$(cd "$(dirname "${argv[0]}")" && pwd)/$(basename "${argv[0]}")"
    fi

    local -a args=()
    if [[ -f "$casedir/args" ]]; then
        # shellcheck disable=SC2207
        args=($(grep -v '^#' "$casedir/args"))
    fi

    local out rc
    out="$( cd "$casedir" && "${argv[@]}" "${args[@]+"${args[@]}"}" 2>&1 )"
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
    # Every non-blank expect line is its own assertion: grep -F would read a
    # multi-line pattern as alternatives and pass on any one of them.
    local -a missing=()
    local want_line
    while IFS= read -r want_line; do
        [[ -z "${want_line//[[:space:]]/}" ]] && continue
        grep -qF -- "$want_line" <<<"$out" || missing+=("$want_line")
    done <<<"$expect"
    if [[ "${#missing[@]}" -gt 0 ]]; then
        echo "  FAIL: $gate $(basename "$casedir") exit $rc OK but output lacks expected line(s):"
        printf '        missing: %s\n' "${missing[@]}"
        printf '    %s\n' "$out"
        return 1
    fi

    # spec: gate-sdk/SPEC.md §Output contract — asserted at runtime, both cases
    if [[ "$want" -eq 0 ]]; then
        if ! grep -qE '^[A-Z][A-Z0-9-]*: clean \(.*\)$' <<<"$out"; then
            echo "  FAIL: $gate $(basename "$casedir") exited 0 but emitted no '<NAME>: clean (…)' line"
            printf '    %s\n' "$out"
            return 1
        fi
    else
        if ! grep -qE '(^|[[:space:]])help:' <<<"$out"; then
            echo "  FAIL: $gate $(basename "$casedir") reported a violation with no 'help:' remedy line"
            printf '    %s\n' "$out"
            return 1
        fi
    fi
    return 0
}

shopt -s nullglob
gate_dirs=("$TESTS_DIR"/*/)
unit_tests=("$TESTS_DIR"/*.test.sh)
shopt -u nullglob
if [[ ${#gate_dirs[@]} -eq 0 && ${#unit_tests[@]} -eq 0 ]]; then
    echo "run-gate-tests: no gate fixture dirs and no *.test.sh under $TESTS_DIR" >&2
    exit 2
fi

for d in "${gate_dirs[@]+"${gate_dirs[@]}"}"; do
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
for t in "${unit_tests[@]+"${unit_tests[@]}"}"; do
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
