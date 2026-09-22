#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — --dispatch and --dispatch-withdraw end-to-end through a sandboxed enter-stage: the declaration after a clear pre-flight, a refused pre-flight declaring nothing, the stamp's discharge of one line and the emptied marker's removal, the withdrawal and its no-op, and the --simulate and surplus refusals, none writing the state file
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the second sanctioned caller: a non-git sandbox
# cwd, so the binary is resolved through gate_arm_run rather than through the front-end.
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

seed() {  # $1=sandbox subdir; the cursor is 'build'
    local sb="$1"
    mkdir -p "$sb/.workflow"
    printf '# TASK-QUEUE.md\n\n## Iteration: demo\n\n---\n\n## New Features\n\n## Technical Debt\n\n## Done\n' >"$sb/TASK-QUEUE.md"
    printf '# contract: lifecycle-kit/SPEC.md §check-stage-evidence\n\n---\n\ndemo scope aaaaaaaa 2026-06-01 none\ndemo build bbbbbbbb 2026-06-02 none\n' \
        >"$sb/.workflow/WORKFLOW-STATE.txt"
    : >"$sb/lifecycle-config.knobs"
}

run_enter() {  # $1=sandbox, rest = argv after the tool name
    local sb="$1"; shift
    ( cd "$sb" && gate_env LIFECYCLE_KIT_KNOB_FILE="$sb/lifecycle-config.knobs" \
                           GATE_SDK_TMP_DIR=scratch LIFECYCLE_KIT_SESSION_ID=deadbeef01 \
        && gate_arm_run --enter-stage "$@" 2>&1 )
}

state_of() { cat "$1/.workflow/WORKFLOW-STATE.txt"; }

# --- a clear pre-flight declares one line, and a second declaration appends a second ---
sb="$SANDBOX/declare"; seed "$sb"
before="$(state_of "$sb")"
out="$(run_enter "$sb" --dispatch validate)"; rc=$?
[[ "$rc" -eq 0 ]] || note declare "want exit 0, got $rc -- $out"
[[ "$(cat "$sb/scratch/stage-dispatch.txt" 2>/dev/null)" == "validate" ]] \
    || note declare-line "the marker is not the one declared stage: $(cat "$sb/scratch/stage-dispatch.txt" 2>/dev/null)"
[[ "$(state_of "$sb")" == "$before" ]] || note declare-state "--dispatch wrote the state file"
run_enter "$sb" --dispatch validate >/dev/null
[[ "$(grep -c . "$sb/scratch/stage-dispatch.txt")" -eq 2 ]] || note declare-two "a second declaration did not append"

# --- the stamp discharges one line; the next stamp empties the marker and removes it ---
out="$(run_enter "$sb" validate)"; rc=$?
[[ "$rc" -eq 0 ]] || note discharge "want exit 0 from the stamp, got $rc -- $out"
[[ "$(grep -c . "$sb/scratch/stage-dispatch.txt")" -eq 1 ]] || note discharge-one "the stamp did not remove exactly one line"
grep -qF 'discharged' <<<"$out" || note discharge-report "the stamp report does not note the discharge: $out"
( cd "$sb" && gate_env LIFECYCLE_KIT_KNOB_FILE="$sb/lifecycle-config.knobs" GATE_SDK_TMP_DIR=scratch \
    LIFECYCLE_KIT_SESSION_ID=cafef00d02 && gate_arm_run --enter-stage validate >/dev/null 2>&1 )
[[ -e "$sb/scratch/stage-dispatch.txt" ]] && note discharge-empty "an emptied marker was left on disk"

# --- a refused pre-flight declares nothing ---
sb="$SANDBOX/refuse"; seed "$sb"
out="$(run_enter "$sb" --dispatch close)"; rc=$?
[[ "$rc" -eq 1 ]] || note refuse "want exit 1 for a refused pre-flight, got $rc -- $out"
[[ -e "$sb/scratch/stage-dispatch.txt" ]] && note refuse-write "a refused dispatch wrote the marker"
grep -qF 'nothing declared' <<<"$out" || note refuse-report "the refusal does not say nothing was declared: $out"

# --- withdrawal removes one line; with none it is a reported no-op ---
sb="$SANDBOX/withdraw"; seed "$sb"
run_enter "$sb" --dispatch validate >/dev/null
out="$(run_enter "$sb" --dispatch-withdraw validate)"; rc=$?
[[ "$rc" -eq 0 ]] || note withdraw "want exit 0, got $rc -- $out"
[[ -e "$sb/scratch/stage-dispatch.txt" ]] && note withdraw-empty "the withdrawn marker was left on disk"
out="$(run_enter "$sb" --dispatch-withdraw validate)"; rc=$?
[[ "$rc" -eq 0 ]] || note withdraw-noop "want exit 0 for a no-op withdrawal, got $rc -- $out"
grep -qF 'no-op' <<<"$out" || note withdraw-noop-report "the no-op is not reported: $out"

# --- --simulate, a surplus argument and an unknown stage are usage errors writing nothing ---
sb="$SANDBOX/usage"; seed "$sb"
for argv in "--simulate --dispatch validate" "--dispatch validate extra" "--dispatch" "--dispatch nosuchstage" "--dispatch-withdraw"; do
    # shellcheck disable=SC2086
    out="$(run_enter "$sb" $argv)"; rc=$?
    [[ "$rc" -eq 2 ]] || note "usage [$argv]" "want exit 2, got $rc -- $out"
done
[[ -e "$sb/scratch/stage-dispatch.txt" ]] && note usage-write "a refused form wrote the marker"

[[ "$fails" -eq 0 ]] || { echo "dispatch-marker.test: $fails assertion(s) failed"; exit 1; }
echo "dispatch-marker.test: clean (--dispatch declares after a clear pre-flight and never after a refused one, each stamp discharges one line and an emptied marker is removed, --dispatch-withdraw removes one or no-ops, and --simulate, surplus and unknown-stage forms are usage errors writing nothing)"
exit 0
