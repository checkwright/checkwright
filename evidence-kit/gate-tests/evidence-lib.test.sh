#!/usr/bin/env bash
# Behavioral test of lib/evidence.sh — the per-suite parser dispatch
# (ek_parser_for / ek_parse) and ek_diff's absent-from-baseline triple, neither
# of which any gate fixture pair can hold: both are library adapters, not gates.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # evidence-kit/
LIB="$DIR/lib/evidence.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/empty"

# The lib exits 2 on a malformed machine, so every load runs in a subshell with
# an empty gates dir — no consumer config on the lookup path.
_load() { env -u EVIDENCE_KIT_CONFIG_FILE GATE_SDK_GATES_DIR="$tmp/empty" bash -c "source '$LIB'; $1" 2>&1; }

# A — an unset suite falls through to the global knob; an override wins for its
#     own suite only, leaving a sibling on the global.
out="$(_load 'EVIDENCE_KIT_PARSER=exit-code
EVIDENCE_KIT_PARSER_gates="bash /dev/null"
printf "gates=%s sibling=%s\n" "$(ek_parser_for gates)" "$(ek_parser_for other)"')"
if [[ "$out" != "gates=bash /dev/null sibling=exit-code" ]]; then
    echo "  FAIL: per-suite parser resolution wrong: $out"; fails=$((fails + 1))
fi

# B — the dispatch reaches ek_parse: the overridden suite runs its consumer
#     command while the sibling keeps the built-in exit-code adapter.
printf '#!/usr/bin/env bash\nprintf "from-override pass\\n"\n' >"$tmp/stub.sh"
out="$(_load "EVIDENCE_KIT_PARSER=exit-code
EVIDENCE_KIT_PARSER_gates='bash $tmp/stub.sh'
: >'$tmp/log'
ek_parse gates '$tmp/log' 0
ek_parse other '$tmp/log' 1")"
if [[ "$out" != "from-override pass
other fail" ]]; then
    echo "  FAIL: ek_parse did not dispatch per-suite (override for gates, global for the sibling): $out"; fails=$((fails + 1))
fi

# C/D/E — the absent-from-baseline triple. The baseline carries a row for a
#     different scenario, so 'newcomer' is absent from it in every case.
printf '# fixture\ns known pass\n' >"$tmp/base.txt"
_absent() {
    printf 'known pass\nnewcomer %s\n' "$1" >"$tmp/obs.txt"
    _load "ek_diff '$tmp/base.txt' s '$tmp/obs.txt'; printf 'rc=%s\n' \"\$?\""
}

# C — an observed failure absent from the baseline is a new failure (the
#     §Baseline manifest fail-closed sentence ek_diff converges on).
out="$(_absent fail)"
if [[ "$out" != "new-failure s newcomer
rc=1" ]]; then
    echo "  FAIL: observed fail absent from the baseline is not a new-failure: $out"; fails=$((fails + 1))
fi

# D — an observed pass absent from the baseline is the SPEC's stated
#     classification cost: no line, no red.
out="$(_absent pass)"
if [[ "$out" != "rc=0" ]]; then
    echo "  FAIL: observed pass absent from the baseline must be silent (classification cost, not a red): $out"; fails=$((fails + 1))
fi

# E — the ruled edge. The rule is 'fail', never non-pass: an ignored test is a
#     non-verdict with no spec sentence to converge on, so an absent 'ignore' is
#     silent. Widening to non-pass would red a libtest consumer's new #[ignore]
#     under a debt-convergence banner. 'non-pass' is the intuitive misreading and
#     was the amendment's own first wording, so the edge is pinned here.
out="$(_absent ignore)"
if [[ "$out" != "rc=0" ]]; then
    echo "  FAIL: observed ignore absent from the baseline must stay silent — the rule is 'fail', not non-pass: $out"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "evidence-lib.test: $fails assertion(s) failed"
    exit 1
fi
echo "evidence-lib.test: ok (per-suite parser dispatch with global fall-through; absent-from-baseline fail reds while pass and ignore stay silent)"
exit 0
