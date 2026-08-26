#!/usr/bin/env bash
# Behavioral test of check-evidence-baseline — the slug-liveness and
# scenario-coverage branches the one good/bad pair (grammar) cannot hold: a Done
# slug is stale-red, an unknown slug is red, a permanent marker is accepted, and
# a configured scenario glob asserts manifest↔disk set equality both ways.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # evidence-kit/

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# case <name> <baseline-body> <queue-body> <want-exit> <expect-substring>
case_run() {
    local name="$1" base="$2" queue="$3" want="$4" expect="$5" out rc
    printf '# fixture\n%b' "$base" >"$tmp/base.txt"
    printf '%b' "$queue" >"$tmp/queue.md"
    out="$(gate_run check-evidence-baseline "$DIR/checks" "$tmp/base.txt" "$tmp/queue.md" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL: $name expected exit $want, got $rc: $out"; fails=$((fails + 1)); return
    fi
    if ! grep -qF -- "$expect" <<<"$out"; then
        echo "  FAIL: $name exit OK but output lacks '$expect': $out"; fails=$((fails + 1))
    fi
}

# A — a fail slug that resolves to a live task is CLEAN.
case_run "live-slug-clean" \
    'u a fail live-one\n' '## New Features\n- **live-one** — x\n' \
    0 "clean"

# B — a fail slug that is a Done task is stale-red.
case_run "done-slug-stale" \
    'u a fail gone-task\n' '## Done\n- **gone-task** — x\n' \
    1 "is a Done task"

# C — a fail slug that resolves nowhere is red.
case_run "unknown-slug" \
    'u a fail nowhere\n' '## New Features\n- **other** — x\n' \
    1 "resolves to no live task"

# D — a 'pass' line carrying a slug is red (a pass takes no blocking slug).
case_run "pass-with-slug" \
    'u a pass stray\n' '## New Features\n- **stray** — x\n' \
    1 "takes no blocking slug"

# E — a permanent marker satisfies liveness without a queue task.
_perm_cfg() {
    local d="$tmp/perm"; mkdir -p "$d/scripts"
    printf 'EVIDENCE_KIT_PERMANENT_SLUGS=(forever)\n' >"$d/scripts/evidence-config.sh"
    printf '# fixture\nu a ignore forever\n' >"$d/base.txt"
    printf '## New Features\n- **unrelated** — x\n' >"$d/queue.md"
    ( cd "$d" && unset EVIDENCE_KIT_CONFIG_FILE \
        && gate_env GATE_SDK_GATES_DIR=scripts \
        && gate_run check-evidence-baseline "$DIR/checks" base.txt queue.md 2>&1 )
}
if ! out="$(_perm_cfg)" || ! grep -qF "clean" <<<"$out"; then
    echo "  FAIL: permanent-marker not accepted: $out"; fails=$((fails + 1))
fi

# F — a configured scenario glob asserts manifest↔disk equality; an on-disk
#     scenario with no baseline line is red.
_cov_cfg() {
    local d="$tmp/cov"; mkdir -p "$d/scripts" "$d/scen"
    printf 'declare -A EVIDENCE_KIT_SCENARIO_GLOBS=([sx]="scen/*.txt")\n' >"$d/scripts/evidence-config.sh"
    : >"$d/scen/a.txt"; : >"$d/scen/b.txt"
    printf '# fixture\nsx a.txt pass\n' >"$d/base.txt"
    ( cd "$d" && unset EVIDENCE_KIT_CONFIG_FILE \
        && gate_env GATE_SDK_GATES_DIR=scripts \
        && gate_run check-evidence-baseline "$DIR/checks" base.txt 2>&1 )
}
if out="$(_cov_cfg)"; then
    echo "  FAIL: coverage gap (extra on-disk scenario) did not redden: $out"; fails=$((fails + 1))
elif ! grep -qF "no baseline line" <<<"$out"; then
    echo "  FAIL: coverage gap wrong finding: $out"; fails=$((fails + 1))
fi

# G — a consumer configuring NO suites disarms the suite-coverage arm at a
#     declared early-out, rather than falling through the live assertions.
_nosuites_cfg() {
    local d="$tmp/nosuites"; mkdir -p "$d/scripts"
    printf 'EVIDENCE_KIT_SUITES=()\n' >"$d/scripts/evidence-config.sh"
    printf '# fixture\nu a pass\n' >"$d/base.txt"
    ( cd "$d" && unset EVIDENCE_KIT_CONFIG_FILE \
        && gate_env GATE_SDK_GATES_DIR=scripts \
        && gate_run check-evidence-baseline "$DIR/checks" base.txt 2>&1 )
}
if ! out="$(_nosuites_cfg)" || ! grep -qF "0 configured suite(s)" <<<"$out"; then
    echo "  FAIL: an empty suite roster did not disarm cleanly at the declared early-out: $out"; fails=$((fails + 1))
fi

# H — a suite roster the bridge could not carry is exit 2, never a clean run:
#     the argv the bridge built, minus that one assignment, is exactly that state.
_unresolvable() {
    local d="$tmp/unres"; mkdir -p "$d/scripts"
    printf 'EVIDENCE_KIT_SUITES=(gates)\n' >"$d/scripts/evidence-config.sh"
    printf '# fixture\ngates gates pass\n' >"$d/base.txt"
    ( cd "$d" && unset EVIDENCE_KIT_CONFIG_FILE \
        && gate_env GATE_SDK_GATES_DIR=scripts \
        && source "$GATE_SDK_TEST_LIB_DIR/gate.sh" \
        && mapfile -t argv < <(gate_command check-evidence-baseline "$DIR/checks") \
        && kept=() \
        && for a in "${argv[@]}"; do
               [[ "$a" == GATE_SDK_KNOB_EVIDENCE_KIT_SUITES=* ]] || kept+=("$a")
           done \
        && "${kept[@]}" base.txt 2>&1 )
}
out="$(_unresolvable)"; rc=$?
if [[ "$rc" -ne 2 ]]; then
    echo "  FAIL: an unresolvable suite roster exited $rc, want 2 (fail-closed): $out"; fails=$((fails + 1))
elif ! grep -qF "could not run" <<<"$out"; then
    echo "  FAIL: the fail-closed refusal did not name itself as a non-run: $out"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-evidence-baseline.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-evidence-baseline.test: ok (done-stale + unknown + pass-with-slug + coverage-gap rejected; live-slug + permanent-marker accepted; no-suites disarms, unresolvable suites fail closed)"
exit 0
