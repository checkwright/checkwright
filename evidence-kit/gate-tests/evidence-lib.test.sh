#!/usr/bin/env bash
# Behavioral test of the library's per-suite parser dispatch and its absent-from-baseline triple,
# neither of which any gate fixture pair can hold: both are adapters, not gates.
#
# spec: evidence-kit/SPEC.md §lib/evidence.sh — the adapters are the COMPILED twins since the
# 2026-09-04 diff cut retired their shell forms, so this suite drives them through the front end's
# `--diff-baseline` arm rather than by sourcing the library. What it asserts is unchanged: the
# subject moved substrate, and the assertions moved to the surviving implementation.
#
# Every scenario is read out of `--diff-baseline`'s FINDINGS rather than out of the parsed lines,
# which is what a caller can observe: a baseline row is the probe, and a scenario the parser failed
# to produce reds as an absent one.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # evidence-kit/
FE="$(cd "$DIR/../gate-sdk/bin" && pwd)/run-gates.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/empty" "$tmp/scratch"
: >"$tmp/log"

# The hermetic preamble pins every kit's config at an empty file; the empty gates dir keeps a
# consumer config off the lookup path too, and every knob below is the fixture's own.
_diff() {   # $1 = baseline file, $2.. = argv groups; extra knobs come from the caller's env
    env -u EVIDENCE_KIT_CONFIG_FILE \
        GATE_SDK_GATES_DIR="$tmp/empty" \
        EVIDENCE_KIT_BASELINE_FILE="$1" \
        EVIDENCE_KIT_SKIP_FILE="$tmp/no-skips.txt" \
        EVIDENCE_KIT_TMP_DIR="$tmp/scratch" \
        bash "$FE" --diff-baseline "${@:2}" 2>&1
    printf 'rc=%s\n' "$?"
}

# A/B — an unset suite falls through to the global knob while an override wins for its own suite,
#       and the dispatch reaches the parser: `gates` runs the consumer command, `other` keeps the
#       built-in exit-code adapter. The baseline names the scenario ONLY the override can produce,
#       so a resolution that fell through would red it as absent instead of staying silent.
printf '#!/usr/bin/env bash\nprintf "from-override pass\\n"\n' >"$tmp/stub.sh"
printf '# fixture\ngates from-override pass\nother other pass\n' >"$tmp/ab.txt"
out="$(EVIDENCE_KIT_PARSER=exit-code EVIDENCE_KIT_PARSER_gates="bash $tmp/stub.sh" \
    _diff "$tmp/ab.txt" gates "$tmp/log" 0 other "$tmp/log" 1)"
if ! grep -qx 'new-failure other other' <<<"$out" \
    || grep -q 'new-failure gates' <<<"$out" \
    || ! grep -qx 'rc=1' <<<"$out"; then
    echo "  FAIL: per-suite parser dispatch wrong — the override must own 'gates' while 'other' keeps the global exit-code adapter: $out"; fails=$((fails + 1))
fi

# C/D/E — the absent-from-baseline triple. The baseline carries a row for a different scenario, so
#       'newcomer' is absent from it in every case and only its observed status decides.
printf '# fixture\ns known pass\n' >"$tmp/base.txt"
_absent() {
    printf '#!/usr/bin/env bash\nprintf "known pass\\nnewcomer %s\\n"\n' "$1" >"$tmp/triple.sh"
    EVIDENCE_KIT_PARSER=exit-code EVIDENCE_KIT_PARSER_s="bash $tmp/triple.sh" \
        _diff "$tmp/base.txt" s "$tmp/log" 0
}

# C — an observed failure absent from the baseline is a new failure (the §Baseline manifest
#     fail-closed sentence the diff converges on).
out="$(_absent fail)"
if ! grep -qx 'new-failure s newcomer' <<<"$out" || ! grep -qx 'rc=1' <<<"$out"; then
    echo "  FAIL: observed fail absent from the baseline is not a new-failure: $out"; fails=$((fails + 1))
fi

# D — an observed pass absent from the baseline is the SPEC's stated classification cost: no
#     finding, no red.
out="$(_absent pass)"
if grep -q '^new-failure' <<<"$out" || ! grep -qx 'rc=0' <<<"$out"; then
    echo "  FAIL: observed pass absent from the baseline must be silent (classification cost, not a red): $out"; fails=$((fails + 1))
fi

# E — the ruled edge. The rule is 'fail', never non-pass: an ignored test is a non-verdict with no
#     spec sentence to converge on, so an absent 'ignore' is silent. Widening to non-pass would red
#     a libtest consumer's new #[ignore] under a debt-convergence banner. 'non-pass' is the
#     intuitive misreading and was the amendment's own first wording, so the edge is pinned here.
out="$(_absent ignore)"
if grep -q '^new-failure' <<<"$out" || ! grep -qx 'rc=0' <<<"$out"; then
    echo "  FAIL: observed ignore absent from the baseline must stay silent — the rule is 'fail', not non-pass: $out"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "evidence-lib.test: $fails assertion(s) failed"
    exit 1
fi
echo "evidence-lib.test: ok (per-suite parser dispatch with global fall-through; absent-from-baseline fail reds while pass and ignore stay silent)"
exit 0
