#!/usr/bin/env bash
# Behavioral test of checks/check-evidence-baseline.sh — the slug-liveness and
# scenario-coverage branches the one good/bad pair (grammar) cannot hold: a Done
# slug is stale-red, an unknown slug is red, a permanent marker is accepted, and
# a configured scenario glob asserts manifest↔disk set equality both ways.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # evidence-kit/
GATE="$DIR/checks/check-evidence-baseline.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# case <name> <baseline-body> <queue-body> <want-exit> <expect-substring>
case_run() {
    local name="$1" base="$2" queue="$3" want="$4" expect="$5" out rc
    printf '# fixture\n%b' "$base" >"$tmp/base.txt"
    printf '%b' "$queue" >"$tmp/queue.md"
    out="$("$GATE" "$tmp/base.txt" "$tmp/queue.md" 2>&1)"; rc=$?
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
    ( cd "$d" && GATE_SDK_GATES_DIR=scripts "$GATE" base.txt queue.md 2>&1 )
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
    ( cd "$d" && GATE_SDK_GATES_DIR=scripts "$GATE" base.txt 2>&1 )
}
if out="$(_cov_cfg)"; then
    echo "  FAIL: coverage gap (extra on-disk scenario) did not redden: $out"; fails=$((fails + 1))
elif ! grep -qF "no baseline line" <<<"$out"; then
    echo "  FAIL: coverage gap wrong finding: $out"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-evidence-baseline.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-evidence-baseline.test: ok (done-stale + unknown + pass-with-slug + coverage-gap rejected; live-slug + permanent-marker accepted)"
exit 0
