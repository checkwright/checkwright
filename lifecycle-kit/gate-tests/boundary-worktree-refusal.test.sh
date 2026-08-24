#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the iteration-boundary linked-worktree refusal end-to-end through a sandboxed enter-stage on a real git checkout: a boundary entry with a linked worktree present refuses and writes nothing, --simulate relays the same refusal, the same tree with the worktree reaped enters cleanly, LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK=0 turns it off, a non-git tree skips the check rather than failing on it; and the liveness classification: each of the four class readings, the empty-knob default that classifies nothing, the loss report on a residue path, the mid-iteration advisory that reports orphans without refusing, and a malformed pattern refused as config
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
ENTER="$DIR/bin/enter-stage.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

seed() {  # $1=sandbox subdir; writes a boundary-ready consumer (no git init)
    local sb="$1"
    mkdir -p "$sb/.workflow" "$sb/scratch"
    cat >"$sb/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo-iteration

---

## New Features

## Technical Debt

## Done
EOF
    cat >"$sb/.workflow/WORKFLOW-STATE.txt" <<'EOF'
# contract: lifecycle-kit/SPEC.md §check-stage-evidence

---

demo-iteration scope aaaaaaaa 2026-06-01 none
demo-iteration build bbbbbbbb 2026-06-02 none
demo-iteration validate cccccccc 2026-06-03 none
demo-iteration close dddddddd 2026-06-04 none
EOF
    : >"$sb/lifecycle-config.sh"
    : >"$sb/scratch/.gitkeep"
}

git_seed() {  # $1=sandbox subdir; seeds and turns it into a real checkout with one commit
    seed "$1"
    git -C "$1" init -q -b main
    git -C "$1" -c user.email=t@example.invalid -c user.name=t add -A
    git -C "$1" -c user.email=t@example.invalid -c user.name=t commit -qm seed
}

run_enter() {  # $1=sandbox subdir  $2...=enter-stage argv
    local sb="$1"; shift
    ( cd "$sb" && env LIFECYCLE_KIT_CONFIG_FILE="$sb/lifecycle-config.sh" \
                      GATE_SDK_TMP_DIR=scratch \
                      LIFECYCLE_KIT_SESSION_ID=deadbeef01 \
                      bash "$ENTER" "$@" 2>&1 )
}

state_of() { cat "$1/.workflow/WORKFLOW-STATE.txt"; }

# --- a boundary entry with a linked worktree present refuses and writes nothing ---
wt="$SANDBOX/with-worktree"
git_seed "$wt"
git -C "$wt" worktree add -q -b agent-branch "$wt/.claude/worktrees/agent-01" HEAD
before="$(state_of "$wt")"

out="$(run_enter "$wt" scope)"; rc=$?
[[ "$rc" -eq 1 ]] || note refuse-status "want exit 1 on a linked worktree, got $rc -- $out"
grep -qF 'refused' <<<"$out"                    || note refuse-text "the refusal does not say refused: $out"
grep -qF '.claude/worktrees/agent-01' <<<"$out" || note refuse-path "the refusal does not name the worktree path: $out"
grep -qF 'branch ref' <<<"$out"                 || note refuse-branch "the refusal guidance does not name the branch half: $out"
[[ "$(state_of "$wt")" == "$before" ]]          || note refuse-nowrite "a refused boundary entry wrote to the state file"

# --- --simulate relays the same refusal and writes nothing either ---
out="$(run_enter "$wt" --simulate scope)"; rc=$?
[[ "$rc" -eq 1 ]] || note sim-status "want exit 1 under --simulate, got $rc -- $out"
grep -qF 'enter-stage (simulate)' <<<"$out"     || note sim-prefix "the simulate transcript is unprefixed: $out"
grep -qF '.claude/worktrees/agent-01' <<<"$out" || note sim-path "the simulate refusal does not name the path: $out"
[[ "$(state_of "$wt")" == "$before" ]]          || note sim-nowrite "--simulate wrote to the state file"

# --- the knob turns it off, and the same entry then proceeds ---
out="$( cd "$wt" && env LIFECYCLE_KIT_CONFIG_FILE="$wt/lifecycle-config.sh" \
                        GATE_SDK_TMP_DIR=scratch LIFECYCLE_KIT_SESSION_ID=deadbeef01 \
                        LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK=0 \
                        bash "$ENTER" --simulate scope 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] || note knob-off "the knob at 0 still refused: $out"

# --- a NON-boundary entry ignores the worktree entirely ---
out="$(run_enter "$wt" build)"; rc=$?
[[ "$rc" -eq 0 ]] || note nonboundary "a non-boundary entry refused on a linked worktree: $out"

# --- the liveness classification ---
# The fixture's lock vocabulary is invented on purpose: the kit ships an empty
# pattern default, so a fixture spelling a real harness's reason would publish
# the vocabulary the seam ruling keeps in consumer config.
RE='^testharness \(pid ([0-9]+)\)$'
DEAD_PID=2147483646   # never a live process; the same probe value producer_liveness.rs uses

run_classified() {  # $1=sandbox  $2=pattern  $3...=argv
    local sb="$1" re="$2"; shift 2
    ( cd "$sb" && env LIFECYCLE_KIT_CONFIG_FILE="$sb/lifecycle-config.sh" \
                      GATE_SDK_TMP_DIR=scratch \
                      LIFECYCLE_KIT_SESSION_ID=deadbeef01 \
                      LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE="$re" \
                      bash "$ENTER" "$@" 2>&1 )
}

seed_wt() {  # $1=name -> echoes the sandbox path with one linked worktree at .claude/worktrees/agent-01
    local sb="$SANDBOX/$1"
    git_seed "$sb"
    git -C "$sb" worktree add -q -b agent-branch "$sb/.claude/worktrees/agent-01" HEAD
    echo "$sb"
}

# locked, reason matches, captured pid alive -> live: named with its pid, told to wait, never offered --force
sb="$(seed_wt class-live)"
git -C "$sb" worktree lock --reason "testharness (pid $$)" "$sb/.claude/worktrees/agent-01"
out="$(run_classified "$sb" "$RE" scope)"; rc=$?
[[ "$rc" -eq 1 ]] || note live-status "want exit 1 with a live worktree, got $rc -- $out"
grep -qE '^live +.*agent-01' <<<"$out" || note live-class "a live worktree was not classified live: $out"
grep -qF "held by pid $$" <<<"$out"    || note live-pid "the live line does not name the holding pid: $out"
grep -qF 'wait for the named pid' <<<"$out" || note live-remedy "the live class was not told to wait: $out"
grep -qF -- '--force --force' <<<"$out"     && note live-force "the live class was offered a force-remove: $out"

# locked, reason matches, captured pid dead -> orphaned: --force --force named here and only here
sb="$(seed_wt class-orphan-locked)"
git -C "$sb" worktree lock --reason "testharness (pid $DEAD_PID)" "$sb/.claude/worktrees/agent-01"
out="$(run_classified "$sb" "$RE" scope)"; rc=$?
[[ "$rc" -eq 1 ]] || note orphan-status "want exit 1 with an orphaned worktree, got $rc -- $out"
grep -qE '^orphaned +.*agent-01' <<<"$out"  || note orphan-class "a dead holder was not classified orphaned: $out"
grep -qF -- '--force --force' <<<"$out"     || note orphan-remedy "the orphaned class was not given the double-force remedy: $out"
grep -qF 'removal is lossless' <<<"$out"    || note orphan-loss "a clean commitless orphan was not reported lossless: $out"
grep -qF 'wait for the named pid' <<<"$out" && note orphan-wait "the orphaned class was told to wait: $out"

# not locked at all, knob set -> orphaned
sb="$(seed_wt class-orphan-unlocked)"
out="$(run_classified "$sb" "$RE" scope)"; rc=$?
[[ "$rc" -eq 1 ]] || note unlocked-status "want exit 1 with an unlocked worktree, got $rc -- $out"
grep -qE '^orphaned +.*agent-01' <<<"$out" || note unlocked-class "an unlocked worktree was not classified orphaned: $out"

# locked, reason does not match the pattern -> unclassified, and today's reap guidance
sb="$(seed_wt class-unclassified)"
git -C "$sb" worktree lock --reason "some other tool is holding this" "$sb/.claude/worktrees/agent-01"
out="$(run_classified "$sb" "$RE" scope)"; rc=$?
[[ "$rc" -eq 1 ]] || note uncl-status "want exit 1 with an unclassified worktree, got $rc -- $out"
grep -qE '^unclassified +.*agent-01' <<<"$out" || note uncl-class "an unmatched lock reason was not unclassified: $out"
grep -qF "git worktree remove <path>" <<<"$out" || note uncl-remedy "the unclassified class lost today's reap guidance: $out"

# the EMPTY-knob default classifies nothing: a live-looking lock still reads unclassified
sb="$(seed_wt class-default)"
git -C "$sb" worktree lock --reason "testharness (pid $$)" "$sb/.claude/worktrees/agent-01"
out="$(run_enter "$sb" scope)"; rc=$?
[[ "$rc" -eq 1 ]] || note default-status "want exit 1 under the empty-knob default, got $rc -- $out"
grep -qE '^unclassified +.*agent-01' <<<"$out" || note default-class "the empty knob classified a worktree: $out"
grep -qE '^live ' <<<"$out" && note default-live "the empty knob produced a live reading: $out"

# the loss report is a real read, not a constant: a commit in the worktree is counted as unreachable
sb="$(seed_wt class-loss)"
git -C "$sb" worktree lock --reason "testharness (pid $DEAD_PID)" "$sb/.claude/worktrees/agent-01"
echo stray > "$sb/.claude/worktrees/agent-01/stray.txt"
git -C "$sb/.claude/worktrees/agent-01" add -A
git -C "$sb/.claude/worktrees/agent-01" -c user.email=t@example.invalid -c user.name=t commit -qm stray
out="$(run_classified "$sb" "$RE" scope)"; rc=$?
grep -qF '1 commit(s) unreachable from HEAD' <<<"$out" || note loss-commits "the loss report did not count the unreachable commit: $out"
grep -qF 'removal is lossless' <<<"$out" && note loss-lossless "a worktree carrying a commit was called lossless: $out"

# mid-iteration: an orphan is ADVISORY, the entry proceeds and the stamp lands
sb="$(seed_wt advisory)"
git -C "$sb" worktree lock --reason "testharness (pid $DEAD_PID)" "$sb/.claude/worktrees/agent-01"
out="$(run_classified "$sb" "$RE" build)"; rc=$?
[[ "$rc" -eq 0 ]] || note advisory-status "the mid-iteration advisory refused the entry: $out"
grep -qF 'advisory' <<<"$out"              || note advisory-text "no mid-iteration advisory for an orphan: $out"
grep -qE '^orphaned +.*agent-01' <<<"$out" || note advisory-path "the advisory does not name the orphan: $out"
grep -qF 'demo-iteration build' "$sb/.workflow/WORKFLOW-STATE.txt" \
    || note advisory-stamp "the advisory suppressed the stamp: $out"

# mid-iteration: a LIVE worktree is reported nowhere — an in-flight dispatch is the normal state
sb="$(seed_wt advisory-live)"
git -C "$sb" worktree lock --reason "testharness (pid $$)" "$sb/.claude/worktrees/agent-01"
out="$(run_classified "$sb" "$RE" build)"; rc=$?
[[ "$rc" -eq 0 ]] || note advisory-live-status "a live worktree refused a non-boundary entry: $out"
grep -qF 'advisory' <<<"$out" && note advisory-live-quiet "a live worktree was reported mid-iteration: $out"

# a malformed pattern is a config refusal, never a silent everything-unclassified
sb="$(seed_wt bad-pattern)"
out="$(run_classified "$sb" '^testharness ((' --simulate scope)"; rc=$?
[[ "$rc" -eq 2 ]] || note bad-ere "an uncompilable pattern was not refused as config (exit $rc): $out"
out="$(run_classified "$sb" '^testharness pid [0-9]+$' --simulate scope)"; rc=$?
[[ "$rc" -eq 2 ]] || note no-group "a pattern with no capture group was not refused as config (exit $rc): $out"

# --- the same tree with the worktree reaped enters cleanly (the refusal's off state) ---
clean="$SANDBOX/reaped"
git_seed "$clean"
git -C "$clean" worktree add -q -b agent-branch "$clean/.claude/worktrees/agent-02" HEAD
git -C "$clean" worktree remove --force "$clean/.claude/worktrees/agent-02"
out="$(run_enter "$clean" --simulate scope)"; rc=$?
[[ "$rc" -eq 0 ]] || note reaped "a reaped tree still refused the boundary entry: $out"

# --- a tree that is no git checkout at all skips the check rather than failing on it ---
nogit="$SANDBOX/nogit"
seed "$nogit"
out="$(run_enter "$nogit" --simulate scope)"; rc=$?
[[ "$rc" -eq 0 ]] || note nogit "a non-git tree failed the worktree check instead of skipping it: $out"

[[ "$fails" -eq 0 ]] || { echo "boundary-worktree-refusal.test: $fails assertion(s) failed"; exit 1; }
echo "boundary-worktree-refusal.test: clean (the boundary refuses on a linked worktree and writes nothing, --simulate relays it, the knob disables it, a reaped tree passes, a non-git tree skips; four class readings, the empty-knob default, the loss report, the mid-iteration advisory live and orphaned, and two malformed patterns refused as config)"
exit 0
