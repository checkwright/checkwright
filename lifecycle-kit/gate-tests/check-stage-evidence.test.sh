#!/usr/bin/env bash
# Behavioral test of check-stage-evidence — the sentinel-scoping
# guards the one-pair good/bad harness cannot hold ('—' legal only at the
# first stage; a '—' stamp legal only while the header is also unnamed), plus
# the waiver-token grammar allowance. The regression lives in the interplay of
# the header guard and the per-stamp loop allowance, so the cases drive the
# whole gate on crafted input via its $1/$2 argument mode.
#
# spec: lifecycle-kit/SPEC.md §check-stage-evidence — the stamp-provenance and
# stamp-commit-purity assertions live here too, and nowhere else: each reads a real
# HEAD and a real staged path set, so a static good/bad fixture pair cannot carry
# one. Part 2 below builds a throwaway git repo per case for exactly that.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# case <name> <header-line> <stamp-lines> <want-exit> <expect-substring>
case_run() {
    local name="$1" hdr="$2" stamp="$3" want="$4" expect="$5" out rc
    printf '%s\n' "$hdr" >"$tmp/TASK-QUEUE.md"
    printf 'header prose\n---\n%b' "$stamp" >"$tmp/WORKFLOW-STATE.txt"
    out="$(gate_run check-stage-evidence "$DIR/checks" \
        "$tmp/TASK-QUEUE.md" "$tmp/WORKFLOW-STATE.txt" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL: $name expected exit $want, got $rc: $out"; fails=$((fails + 1)); return
    fi
    if ! grep -qF -- "$expect" <<<"$out"; then
        echo "  FAIL: $name exit OK but output lacks '$expect': $out"; fails=$((fails + 1))
    fi
}

# A — an unnamed iteration past the first stage must FAIL (without the header
#     guard a '—' header matched a '— validate' stamp and passed clean).
case_run "unnamed-past-first-stage" \
    '## Iteration: —' \
    '— validate s1 2026-06-12 none\n' \
    1 "still unnamed ('—') at stage 'validate'"

# B — a named header with a leftover '—' bootstrap stamp is stale and must FAIL.
case_run "stale-bootstrap-under-named" \
    '## Iteration: demo-iteration' \
    '— scope s1 2026-06-12 none\n' \
    1 "legal '—' bootstrap"

# C — the legitimate pre-naming first-stage bootstrap is CLEAN.
case_run "unnamed-first-stage-bootstrap" \
    '## Iteration: —' \
    '— scope s1 2026-06-12 none\n' \
    0 "clean"

# D — a properly named iteration with a matching stamp at a later stage is CLEAN.
case_run "named-at-validate" \
    '## Iteration: demo-iteration' \
    'demo-iteration scope s1 2026-06-12 none\ndemo-iteration validate s2 2026-06-12 none\n' \
    0 "clean"

# E — a waiver line (check-stage-entry assertion C's recorded waiver) is a
#     well-formed stamp token: the grammar accepts it, and it never satisfies
#     the current-stage match (here the build stamp does), so the header is CLEAN.
#     It also shares its id with build here — a waiver stamp is exempt from the
#     stage-distinctness pass, so that reuse does not fire.
case_run "waiver-token-accepted" \
    '## Iteration: demo-iteration' \
    'demo-iteration scope s1 2026-06-12 none\ndemo-iteration align-waived s3 2026-06-12 none\ndemo-iteration build s3 2026-06-12 none\n' \
    0 "clean"

# F — two distinct stages sharing one session id must FAIL: a stage flip is a
#     context boundary, so scope and build cannot both be session s1.
case_run "shared-session-across-stages" \
    '## Iteration: demo-iteration' \
    'demo-iteration scope s1 2026-06-12 none\ndemo-iteration build s1 2026-06-13 none\n' \
    1 "is shared by stages"

# G — a multi-session build (same stage, two different ids) is CLEAN: same-stage
#     re-entries may rotate the id freely.
case_run "same-stage-multi-session" \
    '## Iteration: demo-iteration' \
    'demo-iteration scope s1 2026-06-12 none\ndemo-iteration build s2 2026-06-13 none\ndemo-iteration build s3 2026-06-14 none\n' \
    0 "clean"

# G2 — a state file present but carrying no stamp (the no-cursor window) must
#      FAIL. Once the stage axis moved off the header this is the shape that
#      would otherwise go vacuous: with no stamps there is nothing for the
#      grammar or staleness passes to reject, so an unstamped file would read
#      as clean — exactly what this gate exists to catch.
case_run "no-cursor-unstamped-state" \
    '## Iteration: demo-iteration' \
    '' \
    1 "carries no stamp"

# H — case F's shared-id input greens under the 'iteration' posture: the knob
#     skips only the cross-stage distinctness map (attribution still stamps).
printf '%s\n' '## Iteration: demo-iteration' >"$tmp/TASK-QUEUE.md"
printf 'header prose\n---\ndemo-iteration scope s1 2026-06-12 none\ndemo-iteration build s1 2026-06-13 none\n' >"$tmp/WORKFLOW-STATE.txt"
out="$(gate_env LIFECYCLE_KIT_SESSION_BOUNDARY=iteration
       gate_run check-stage-evidence "$DIR/checks" \
           "$tmp/TASK-QUEUE.md" "$tmp/WORKFLOW-STATE.txt" 2>&1)"; rc=$?
if [[ "$rc" -ne 0 ]] || ! grep -qF "clean" <<<"$out"; then
    echo "  FAIL: shared-session-iteration-posture expected clean exit 0, got $rc: $out"; fails=$((fails + 1))
fi

# I — a bad posture value must exit 2 (the loader's fail-closed machine check).
printf '%s\n' '## Iteration: demo-iteration' >"$tmp/TASK-QUEUE.md"
printf 'header prose\n---\ndemo-iteration build s1 2026-06-13 none\n' >"$tmp/WORKFLOW-STATE.txt"
out="$(gate_env LIFECYCLE_KIT_SESSION_BOUNDARY=bogus
       gate_run check-stage-evidence "$DIR/checks" \
           "$tmp/TASK-QUEUE.md" "$tmp/WORKFLOW-STATE.txt" 2>&1)"; rc=$?
if [[ "$rc" -ne 2 ]] || ! grep -qF "neither 'stage' nor 'iteration'" <<<"$out"; then
    echo "  FAIL: bogus-posture expected exit 2 with the loader finding, got $rc: $out"; fails=$((fails + 1))
fi

# --- Part 2: the provenance + purity assertions, each on a throwaway git repo ---
# spec: lifecycle-kit/SPEC.md §check-stage-evidence — the assertions read HEAD and the staged
# path set, so every case owns a repo: a shared one would let an earlier case's commit decide a
# later case's verdict, which is the coupling these assertions exist to detect.

repo_new() {   # $1 = case name; echoes the repo path, seeded with a committed 4-field stamp
    local r="$tmp/repo-$1"
    mkdir -p "$r/.workflow"
    git -C "$r" init -q
    git -C "$r" config user.email test@example.invalid
    git -C "$r" config user.name test
    printf '## Iteration: demo-iteration\n' >"$r/TASK-QUEUE.md"
    printf 'header prose\n---\ndemo-iteration scope s1 2026-06-12 none\n' >"$r/.workflow/WORKFLOW-STATE.txt"
    git -C "$r" add -A
    git -C "$r" commit -q -m seed
    printf '%s\n' "$r"
}

repo_head() { git -C "$1" rev-parse --short HEAD; }

repo_run() {   # $1 = repo; runs the gate from inside it on the configured paths
    ( cd "$1" && gate_run check-stage-evidence "$DIR/checks" \
        TASK-QUEUE.md .workflow/WORKFLOW-STATE.txt 2>&1 )
}

repo_case() {  # $1=name $2=repo $3=want-exit $4=expect-substring
    local out rc
    out="$(repo_run "$2")"; rc=$?
    if [[ "$rc" -ne "$3" ]]; then
        echo "  FAIL: $1 expected exit $3, got $rc: $out"; fails=$((fails + 1)); return
    fi
    if ! grep -qF -- "$4" <<<"$out"; then
        echo "  FAIL: $1 exit OK but output lacks '$4': $out"; fails=$((fails + 1))
    fi
}

# J — a newly introduced stamp naming the current HEAD is CLEAN, and stays clean with the
#     state file staged alone (the stamp commit's own shape).
r="$(repo_new current)"
printf 'demo-iteration build s2 2026-06-13 %s\n' "$(repo_head "$r")" >>"$r/.workflow/WORKFLOW-STATE.txt"
repo_case "provenance-head-is-current" "$r" 0 "clean"
git -C "$r" add .workflow/WORKFLOW-STATE.txt
repo_case "purity-state-file-alone" "$r" 0 "clean"

# K — the attested defect: the stamp was written, then work commits landed, then the stamp is
#     committed. HEAD moved under it, so the recorded head is stale and the gate must RED.
r="$(repo_new stale)"
printf 'demo-iteration build s2 2026-06-13 %s\n' "$(repo_head "$r")" >>"$r/.workflow/WORKFLOW-STATE.txt"
printf 'work\n' >"$r/work.txt"
git -C "$r" add work.txt
git -C "$r" commit -q -m "work that landed under an uncommitted stamp"
repo_case "provenance-head-went-stale" "$r" 1 "HEAD is now"

# L — 'none' is the sentinel for a tree with no commit to name; on the live file inside a work
#     tree it is a RED, which is what stops the inertness conditions from being a disarm.
r="$(repo_new sentinel)"
printf 'demo-iteration build s2 2026-06-13 none\n' >>"$r/.workflow/WORKFLOW-STATE.txt"
repo_case "provenance-none-inside-a-work-tree" "$r" 1 "records head 'none'"

# M — the purity assertion: a commit that introduces a stamp and also stages work must RED,
#     which is the one-commit shape provenance alone cannot see (HEAD never moved).
r="$(repo_new purity)"
printf 'demo-iteration build s2 2026-06-13 %s\n' "$(repo_head "$r")" >>"$r/.workflow/WORKFLOW-STATE.txt"
printf 'work\n' >"$r/work.txt"
git -C "$r" add -A
repo_case "purity-work-rides-the-stamp-commit" "$r" 1 "also stages 'work.txt'"

# N — the first stage's boundary reset legitimately writes the queue and the boundary surfaces
#     in the same motion, so its stamp commit is exempt for exactly those paths.
r="$(repo_new boundary)"
printf 'demo-iteration scope s2 2026-06-13 %s\n' "$(repo_head "$r")" >>"$r/.workflow/WORKFLOW-STATE.txt"
printf '# contract\n' >"$r/.workflow/survey-record.md"
printf '# contract\n' >"$r/.workflow/gap-inbox.md"
printf '## Iteration: demo-iteration\n\nnamed\n' >"$r/TASK-QUEUE.md"
git -C "$r" add -A
repo_case "purity-boundary-reset-surfaces-exempt" "$r" 0 "clean"

# N2 — and the exemption is the FIRST STAGE's alone: the identical staged set under a later
#      stage's stamp reds, which is what makes case N an exemption rather than a blanket pass.
r="$(repo_new boundary-later-stage)"
printf 'demo-iteration build s2 2026-06-13 %s\n' "$(repo_head "$r")" >>"$r/.workflow/WORKFLOW-STATE.txt"
printf '## Iteration: demo-iteration\n\nedited\n' >"$r/TASK-QUEUE.md"
git -C "$r" add -A
repo_case "purity-boundary-exemption-is-first-stage-only" "$r" 1 "also stages 'TASK-QUEUE.md'"

# N3 — the valve ledger is exempt at ANY stage, which is the predicate the exemption always had
#      stated as itself: bin/enter-stage.sh writes the ledger and the stamp in one motion at an
#      admitting entry, and that entry is never the first stage's. The stage restriction rides
#      membership — a non-admitting entry leaves the ledger unstaged — so no stage name gates it.
r="$(repo_new valve-ledger)"
printf 'demo-iteration close s2 2026-06-13 %s\n' "$(repo_head "$r")" >>"$r/.workflow/WORKFLOW-STATE.txt"
printf '# contract\ndemo-iteration close used the accepted red this entry was admitted on\n' \
    >"$r/.workflow/preflight-valve.txt"
git -C "$r" add -A
out="$( cd "$r" && gate_env LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE=.workflow/preflight-valve.txt
        gate_run check-stage-evidence "$DIR/checks" \
            TASK-QUEUE.md .workflow/WORKFLOW-STATE.txt 2>&1 )"; rc=$?
if [[ "$rc" -ne 0 ]] || ! grep -qF "clean" <<<"$out"; then
    echo "  FAIL: purity-valve-ledger-exempt-at-any-stage expected clean exit 0, got $rc: $out"
    fails=$((fails + 1))
fi

# N4 — and the widening is exactly one path: the identical commit with the knob unset reds, so the
#      exemption follows the configuration rather than the filename.
out="$(repo_run "$r")"; rc=$?
if [[ "$rc" -ne 1 ]] || ! grep -qF "also stages '.workflow/preflight-valve.txt'" <<<"$out"; then
    echo "  FAIL: purity-valve-ledger-needs-the-knob expected exit 1 naming the ledger, got $rc: $out"
    fails=$((fails + 1))
fi

# O — the migration clause: rewriting the file's existing four-field stamps to five fields is a
#     REWRITE, not an introduction. Without it the one-time migration — and every consumer's own
#     recovery, which the spec names as the mitigation — would red against an assertion no
#     historical stamp can satisfy.
r="$(repo_new migration)"
printf 'header prose\n---\ndemo-iteration scope s1 2026-06-12\n' >"$r/.workflow/WORKFLOW-STATE.txt"
git -C "$r" add -A
git -C "$r" commit -q -m "a pre-upgrade four-field state file"
printf 'header prose\n---\ndemo-iteration scope s1 2026-06-12 aaaaaaa\n' >"$r/.workflow/WORKFLOW-STATE.txt"
git -C "$r" add -A
repo_case "migration-of-four-field-stamps-is-not-an-introduction" "$r" 0 "clean"

# P — --rename rewrites column 1 of every data line, which must not read as re-introducing all
#     of them: identity is the (session-id, head) pair, not the whole line.
r="$(repo_new rename)"
sed -i 's/^demo-iteration /renamed-iteration /' "$r/.workflow/WORKFLOW-STATE.txt"
printf '## Iteration: renamed-iteration\n' >"$r/TASK-QUEUE.md"
git -C "$r" add -A
repo_case "rename-of-column-1-is-not-an-introduction" "$r" 0 "clean"

if [[ "$fails" -gt 0 ]]; then
    echo "check-stage-evidence.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-stage-evidence.test: ok (unnamed past first stage + stale bootstrap + shared-session-across-stages + unstamped no-cursor state + bogus posture rejected; bootstrap + named later stage + waiver token + multi-session build + iteration-posture shared id accepted; a stale head, a 'none' inside a work tree, work riding the stamp commit and an unconfigured valve ledger rejected; a current head, a lone staged state file, the boundary reset's surfaces, a configured valve ledger at a non-first stage, the four-field migration and a column-1 rename accepted)"
exit 0
