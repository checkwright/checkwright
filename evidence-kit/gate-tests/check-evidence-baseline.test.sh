#!/usr/bin/env bash
# Behavioral test of check-evidence-baseline — the slug-liveness and
# scenario-coverage branches the one good/bad pair (grammar) cannot hold: a Done
# slug is stale-red, an unknown slug is red, a permanent marker is accepted, a
# configured scenario glob asserts manifest↔disk set equality both ways, and the
# flip assertion judges reproduces-at against a scratch repository's iteration start.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
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
    out="$(gate_run check-evidence-baseline "$DIR/checks" "$tmp/base.txt" "$tmp/queue.md" "$tmp/no-state.txt" 2>&1)"; rc=$?
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
    printf 'EVIDENCE_KIT_PERMANENT_SLUGS[] = forever\n' >"$d/scripts/evidence-config.knobs"
    printf '# fixture\nu a ignore forever\n' >"$d/base.txt"
    printf '## New Features\n- **unrelated** — x\n' >"$d/queue.md"
    ( cd "$d" && unset EVIDENCE_KIT_KNOB_FILE \
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
    printf 'EVIDENCE_KIT_SCENARIO_GLOBS[sx] = scen/*.txt\n' >"$d/scripts/evidence-config.knobs"
    : >"$d/scen/a.txt"; : >"$d/scen/b.txt"
    printf '# fixture\nsx a.txt pass\n' >"$d/base.txt"
    ( cd "$d" && unset EVIDENCE_KIT_KNOB_FILE \
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
    printf 'EVIDENCE_KIT_SUITES =\n' >"$d/scripts/evidence-config.knobs"
    printf '# fixture\nu a pass\n' >"$d/base.txt"
    ( cd "$d" && unset EVIDENCE_KIT_KNOB_FILE \
        && gate_env GATE_SDK_GATES_DIR=scripts \
        && gate_run check-evidence-baseline "$DIR/checks" base.txt 2>&1 )
}
if ! out="$(_nosuites_cfg)" || ! grep -qF "0 configured suite(s)" <<<"$out"; then
    echo "  FAIL: an empty suite roster did not disarm cleanly at the declared early-out: $out"; fails=$((fails + 1))
fi

# H — a suite roster the knob file cannot resolve is exit 2, never a clean run.
_unresolvable() {
    local d="$tmp/unres"; mkdir -p "$d/scripts"
    printf 'EVIDENCE_KIT_SUITES = gates\n' >"$d/scripts/evidence-config.knobs"
    printf '# fixture\ngates gates pass\n' >"$d/base.txt"
    ( cd "$d" && unset EVIDENCE_KIT_KNOB_FILE \
        && gate_env GATE_SDK_GATES_DIR=scripts \
        && gate_run check-evidence-baseline "$DIR/checks" base.txt 2>&1 )
}
out="$(_unresolvable)"; rc=$?
if [[ "$rc" -ne 2 ]]; then
    echo "  FAIL: an unresolvable suite roster exited $rc, want 2 (fail-closed): $out"; fails=$((fails + 1))
elif ! grep -qF "EVIDENCE_KIT_SUITES is declared indexed" <<<"$out"; then
    echo "  FAIL: the fail-closed refusal did not name the knob it could not resolve: $out"; fails=$((fails + 1))
fi

# I — the flip assertion, in a scratch repository whose iteration started at $start with
#     both rows passing; $inside is a commit the iteration made after that.
flip="$tmp/flip"; mkdir -p "$flip/scripts"
_git() { git -C "$flip" -c user.name=t -c user.email=t@t -c core.hooksPath=/dev/null "$@"; }
_git init -q .
printf '# fixture\nu a pass\nu b pass\n' >"$flip/base.txt"
_git add base.txt && _git commit -qm start
start="$(_git rev-parse HEAD)"
_git commit -qm inside --allow-empty
inside="$(_git rev-parse HEAD)"
printf '# state\n---\nit scope sid 2026-01-01 %s\n' "$start" >"$flip/state.txt"
printf '## New Features\n- **live** — x\n' >"$flip/queue.md"

# flip_case <name> <baseline-file> <baseline-body> <state> <want-exit> <expect-substring>
flip_case() {
    local name="$1" file="$2" body="$3" state="$4" want="$5" expect="$6" out rc
    printf '# fixture\n%b' "$body" >"$flip/$file"
    out="$( cd "$flip" && gate_run check-evidence-baseline "$DIR/checks" "$file" queue.md "$state" 2>&1 )"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL: $name expected exit $want, got $rc: $out"; fails=$((fails + 1)); return
    fi
    if ! grep -qF -- "$expect" <<<"$out"; then
        echo "  FAIL: $name exit OK but output lacks '$expect': $out"; fails=$((fails + 1))
    fi
}

flip_case "flip-untokened" base.txt 'u a fail live\nu b pass\n' state.txt \
    1 "passed at the iteration start $start and is now fail"
flip_case "flip-at-start" base.txt "u a fail live reproduces-at=${start:0:12}\nu b pass\n" state.txt \
    0 "flip causation judged against the iteration start $start"
flip_case "flip-inside" base.txt "u a ignore live reproduces-at=$inside\nu b pass\n" state.txt \
    1 "names a commit inside the iteration"
flip_case "token-unresolvable" base.txt 'u a pass\nu b fail live reproduces-at=deadbeef00\n' state.txt \
    1 "resolves to no commit"
flip_case "token-on-pass" base.txt "u a pass live reproduces-at=$start\nu b pass\n" state.txt \
    1 "takes no reproduces-at token"
flip_case "token-malformed" base.txt 'u a fail live reproduces-at=XYZ\nu b pass\n' state.txt \
    1 "the only fifth field is reproduces-at=<rev>"
flip_case "new-scenario-not-a-flip" base.txt 'u a pass\nu b pass\nu c fail live\n' state.txt \
    0 "judged against the iteration start"
flip_case "prior-path-absent" later.txt 'u a fail live\nu b pass\n' state.txt \
    0 "judged against the iteration start"
flip_case "no-iteration-start" base.txt 'u a fail live\nu b pass\n' no-state.txt \
    0 "flip causation off: no iteration-start commit"

# J — the state file resolves through EVIDENCE_KIT_STATE_FILE when no third positional is given.
printf 'EVIDENCE_KIT_STATE_FILE = state.txt\n' >"$flip/scripts/evidence-config.knobs"
printf '# fixture\nu a fail live\nu b pass\n' >"$flip/base.txt"
out="$( cd "$flip" && unset EVIDENCE_KIT_KNOB_FILE && gate_env GATE_SDK_GATES_DIR=scripts \
    && gate_run check-evidence-baseline "$DIR/checks" base.txt queue.md 2>&1 )"; rc=$?
if [[ "$rc" -ne 1 ]] || ! grep -qF "passed at the iteration start" <<<"$out"; then
    echo "  FAIL: the knob-resolved state file did not arm the flip assertion (exit $rc): $out"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-evidence-baseline.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-evidence-baseline.test: ok (done-stale + unknown + pass-with-slug + coverage-gap rejected; live-slug + permanent-marker accepted; no-suites disarms, unresolvable suites fail closed; flip assertion over positional and knob state)"
exit 0
