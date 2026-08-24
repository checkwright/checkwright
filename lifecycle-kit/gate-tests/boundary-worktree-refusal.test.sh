#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the iteration-boundary linked-worktree refusal end-to-end through a sandboxed enter-stage on a real git checkout: a boundary entry with a linked worktree present refuses and writes nothing, --simulate relays the same refusal, the same tree with the worktree reaped enters cleanly, a non-boundary entry ignores the worktree, LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK=0 turns it off, and a non-git tree skips the check rather than failing on it
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
echo "boundary-worktree-refusal.test: clean (the boundary refuses on a linked worktree and writes nothing, --simulate relays it, the knob disables it, a non-boundary entry ignores it, a reaped tree passes, a non-git tree skips)"
exit 0
