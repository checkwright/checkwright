#!/usr/bin/env bash
# Behavioral test of check-test-control — the branches the one good/bad pair cannot hold: the
# empty-controls not-applicable line, a malformed declaration suppressing (B) and (C), the git read
# of the baseline at the iteration start, the EVIDENCE_KIT_CONTROL_SINCE fall-through and its
# unresolvable-revision exit 2, (B)'s disarmed early-out, and (C) against a scratch manifest.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # evidence-kit/

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

H64="$(printf 'b%.0s' {1..64})"

# A scratch repository whose iteration started at $start with one baselined test.
repo="$tmp/repo"; mkdir -p "$repo/scripts"
_git() { git -C "$repo" -c user.name=t -c user.email=t@t -c core.hooksPath=/dev/null "$@"; }
_git init -q .
printf '# fixture\nunit t_old pass\n' >"$repo/base.txt"
_git add base.txt && _git commit -qm start
start="$(_git rev-parse HEAD)"
printf '# state\n---\nit scope sid 2026-01-01 %s\n' "$start" >"$repo/state.txt"
printf '## Iteration: it\n' >"$repo/queue.md"
printf '# contract: evidence-manifest v2\n' >"$repo/manifest.txt"

# knobs <controls-lines…> — the scratch repo's knob file: suite `unit` plus the given lines
knobs() {
    { printf 'EVIDENCE_KIT_SUITES[] = unit\n'; printf '%s\n' "$@"; } >"$repo/scripts/evidence-config.knobs"
}

# case_run <name> <want-exit> <expect-substring> <positional…>
case_run() {
    local name="$1" want="$2" expect="$3" out rc; shift 3
    out="$( cd "$repo" && unset EVIDENCE_KIT_KNOB_FILE && gate_env GATE_SDK_GATES_DIR=scripts \
        && gate_run check-test-control "$DIR/checks" "$@" 2>&1 )"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL: $name expected exit $want, got $rc: $out"; fails=$((fails + 1)); return
    fi
    if ! grep -qF -- "$expect" <<<"$out"; then
        echo "  FAIL: $name exit OK but output lacks '$expect': $out"; fails=$((fails + 1))
    fi
}

CTL=('EVIDENCE_KIT_CONTROLS[] = nullguard' 'EVIDENCE_KIT_CONTROL_nullguard = unit t_null BREAK_NULL=1')

# 1 — no control declared: nothing asserted, the not-applicable line, however red the tree.
knobs
printf '# fixture\nunit t_old pass\nunit t_new pass\n' >"$repo/base.txt"
case_run "empty-controls" 0 "clean (not applicable: EVIDENCE_KIT_CONTROLS is empty)" base.txt "" manifest.txt queue.md state.txt

# 2 — (A): a malformed declaration, a control named SINCE and a duplicate each red, and (B) is
#     suppressed: the uncontrolled new test above goes unreported.
knobs 'EVIDENCE_KIT_CONTROLS[] = a' 'EVIDENCE_KIT_CONTROLS[] = b' 'EVIDENCE_KIT_CONTROLS[] = c' 'EVIDENCE_KIT_CONTROLS[] = SINCE' \
    'EVIDENCE_KIT_CONTROL_a = unit t_null BREAK=1' 'EVIDENCE_KIT_CONTROL_b = unit t_null BREAK=2' 'EVIDENCE_KIT_CONTROL_c = ghost t X=1'
out="$( cd "$repo" && unset EVIDENCE_KIT_KNOB_FILE && gate_env GATE_SDK_GATES_DIR=scripts \
    && gate_run check-test-control "$DIR/checks" base.txt "" manifest.txt queue.md state.txt 2>&1 )"; rc=$?
for want in "both name unit t_null" "names suite 'ghost'" "collides with EVIDENCE_KIT_CONTROL_SINCE"; do
    grep -qF -- "$want" <<<"$out" || { echo "  FAIL: (A) lacks '$want': $out"; fails=$((fails + 1)); }
done
if [[ "$rc" -ne 1 ]] || grep -qF "t_new" <<<"$out"; then
    echo "  FAIL: (A) exited $rc or did not suppress (B): $out"; fails=$((fails + 1))
fi

# 3 — (B) through git: t_new was baselined after the iteration start and no control names it.
knobs "${CTL[@]}"
printf '# fixture\nunit t_old pass\nunit t_null pass\nunit t_new pass\n' >"$repo/base.txt"
case_run "new-test-at-iteration-start" 1 "new test 'unit t_new' (baselined pass since the iteration start $start)" \
    base.txt "" manifest.txt queue.md state.txt

# 4 — a baseline absent at the since commit makes every pass row new.
printf '# fixture\nunit t_null pass\n' >"$repo/later.txt"
case_run "absent-at-since" 0 "1 new test(s) judged against the iteration start $start" \
    later.txt "" manifest.txt queue.md state.txt

# 5 — with no iteration start, EVIDENCE_KIT_CONTROL_SINCE names the since commit.
knobs "${CTL[@]}" "EVIDENCE_KIT_CONTROL_SINCE = $start"
case_run "since-fall-through" 1 "baselined pass since EVIDENCE_KIT_CONTROL_SINCE $start" \
    base.txt "" manifest.txt queue.md no-state.txt

# 6 — an unresolvable EVIDENCE_KIT_CONTROL_SINCE is exit 2 naming it, never clean or disarmed.
knobs "${CTL[@]}" "EVIDENCE_KIT_CONTROL_SINCE = no-such-rev"
case_run "since-unresolvable" 2 "EVIDENCE_KIT_CONTROL_SINCE 'no-such-rev' resolves to no commit" \
    base.txt "" manifest.txt queue.md no-state.txt

# 7 — neither an iteration start nor a since revision disarms (B) at its declared early-out.
knobs "${CTL[@]}"
case_run "since-disarmed" 0 "new-test check off: no iteration-start commit and EVIDENCE_KIT_CONTROL_SINCE is empty" \
    base.txt "" manifest.txt queue.md no-state.txt

# 8 — (C) on a close cursor: judged per control against the suite line it follows.
printf '# fixture\nunit t_old pass\nunit t_null pass\n' >"$repo/base.txt"
printf '# state\n---\nit validate sid 2026-01-02 %s\nit close sid 2026-01-03 %s\n' "$start" "$start" >"$repo/close.txt"
suite="it unit sha256=$H64 pass=2 fail=0 ignore=0 verdict=clean 2026-01-02 commit=$start tree=clean cmd=$H64 origin=local"
# The witness's assign= is the SHA-256 of nullguard's one assignment, 'BREAK_NULL=1' and a newline.
witness="it unit control=nullguard scenario=t_null witnessed=red 2026-01-02 commit=$start cmd=$H64 assign=ffc2bf120f95ff83431fa8cb7d89902ed9cdf9a3c62dd02a78a7fc2c0d3d2ff3"
# close_case <name> <want-exit> <expect> <manifest-lines…>
close_case() {
    local name="$1" want="$2" expect="$3"; shift 3
    { printf '# contract: evidence-manifest v2\n'; printf '%s\n' "$@"; } >"$repo/manifest.txt"
    case_run "$name" "$want" "$expect" base.txt "" manifest.txt queue.md close.txt
}
close_case "close-witnessed" 0 "every control witnessed red for 'it' at stage 'close'" "$suite" "$witness"
close_case "close-unwitnessed" 1 "control 'nullguard' has no witness" "$suite"
close_case "close-green" 1 "control 'nullguard' witnessed=green" "$suite" "${witness/witnessed=red/witnessed=green}"
close_case "close-other-scenario" 1 "not its declared scenario t_null" "$suite" "${witness/scenario=t_null/scenario=t_old}"
close_case "close-other-command" 1 "not the commit and command of the suite line it follows" "$suite" "${witness/cmd=$H64/cmd=${H64//b/c}}"
close_case "close-stale" 1 "before the earliest validate stamp 2026-01-02" "$suite" "${witness/2026-01-02/2026-01-01}"
close_case "close-imported-after" 1 "control 'nullguard' has no witness" "$suite" "$witness" "${suite/origin=local/origin=imported}"
knobs 'EVIDENCE_KIT_CONTROLS[] = nullguard' 'EVIDENCE_KIT_CONTROL_nullguard = unit t_null BREAK_NULL=2'
close_case "close-other-assignments" 1 "control 'nullguard' was witnessed under other assignments than its declaration's" "$suite" "$witness"

if [[ "$fails" -gt 0 ]]; then
    echo "check-test-control.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-test-control.test: ok (empty controls not applicable; (A) reds and suppresses (B); (B) through git at the iteration start, an absent prior baseline, the since fall-through, its unresolvable exit 2 and the disarmed early-out; (C) witnessed, unwitnessed, green, other scenario, other command, stale, import-superseded and other assignments)"
exit 0
