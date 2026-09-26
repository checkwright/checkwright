#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §check-dispatch-entry — the workflow-state guard's stamp-before-write and superseded-stage rules end-to-end through a sandboxed --hook: a configured stage-session caller with no stamp is blocked, a caller whose stage the cursor has left is blocked outside the scratch dir and allowed inside it, a stamped one holding the cursor, a caller of another type and a top-level caller pass, an empty roster is inert, an unreadable state file declines through the advise envelope, and the state-file rule still blocks a stamped caller and follows a relocated LIFECYCLE_KIT_STATE_FILE
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

sb="$SANDBOX/tree"
mkdir -p "$sb/.workflow"
printf '# contract: lifecycle-kit/SPEC.md §check-stage-evidence\n\n---\n\ndemo scope aaaaaaaa 2026-06-01 none\ndemo build a94d72dc 2026-06-02 none\n' \
    >"$sb/.workflow/WORKFLOW-STATE.txt"
printf 'LIFECYCLE_KIT_STAGE_SESSION_TYPES[] = stage-session\n' >"$sb/roster.knobs"
: >"$sb/empty.knobs"

payload() {  # $1=agent_type (or '-' for none)  $2=agent_id  $3=file_path
    if [[ "$1" == "-" ]]; then
        printf '{"hook_event_name":"PreToolUse","tool_name":"Write","tool_input":{"file_path":"%s"}}' "$3"
    else
        printf '{"hook_event_name":"PreToolUse","tool_name":"Write","agent_type":"%s","agent_id":"%s","tool_input":{"file_path":"%s"}}' "$1" "$2" "$3"
    fi
}

hook() {  # $1=knob file  $2=payload; prints stdout+stderr, returns the hook's exit
    ( cd "$sb" && gate_env LIFECYCLE_KIT_KNOB_FILE="$sb/$1" && gate_arm_run --hook workflow-state-guard <<<"$2" 2>&1 )
}

out="$(hook roster.knobs "$(payload stage-session a94d72dcd2db52dbf notes.md)")"; rc=$?
[[ "$rc" -eq 0 && -z "$out" ]] || note stamped "want a silent exit 0 for a stamped caller, got $rc -- $out"

out="$(hook roster.knobs "$(payload stage-session b1b2b3b4c5c6c7c8d notes.md)")"; rc=$?
[[ "$rc" -eq 2 ]] || note unstamped "want exit 2 for an unstamped stage session, got $rc -- $out"
grep -qF -- '--enter-stage <stage>' <<<"$out" || note unstamped-remedy "the block does not name the remedy: $out"
grep -qF 'b1b2b3b4' <<<"$out" || note unstamped-id "the block does not name the normalized id: $out"

out="$(hook roster.knobs "$(payload Explore b1b2b3b4c5c6c7c8d notes.md)")"; rc=$?
[[ "$rc" -eq 0 && -z "$out" ]] || note other-type "want a silent exit 0 for a caller outside the roster, got $rc -- $out"

out="$(hook roster.knobs "$(payload - - notes.md)")"; rc=$?
[[ "$rc" -eq 0 && -z "$out" ]] || note top-level "want a silent exit 0 for a caller with no agent_type, got $rc -- $out"

out="$(hook empty.knobs "$(payload stage-session b1b2b3b4c5c6c7c8d notes.md)")"; rc=$?
[[ "$rc" -eq 0 && -z "$out" ]] || note inert "want a silent exit 0 under an empty roster, got $rc -- $out"

# the third rule: the scope-stamped caller's stage has been left, so it writes only under the scratch dir
out="$(hook roster.knobs "$(payload stage-session aaaaaaaa00000000 notes.md)")"; rc=$?
[[ "$rc" -eq 2 ]] || note superseded "want exit 2 for a caller whose stage the cursor has left, got $rc -- $out"
grep -qF "entered 'scope', and the cursor has moved to 'build'" <<<"$out" || note superseded-stages "the block does not name both stages: $out"
out="$(hook roster.knobs "$(payload stage-session aaaaaaaa00000000 "$sb/.tmp/journal/stage-journal.md")")"; rc=$?
[[ "$rc" -eq 0 && -z "$out" ]] || note superseded-scratch "want a silent exit 0 for a superseded caller writing under the scratch dir, got $rc -- $out"
out="$(hook roster.knobs "$(payload stage-session aaaaaaaa00000000 "$sb/.tmp/../notes.md")")"; rc=$?
[[ "$rc" -eq 2 ]] || note superseded-climb "want exit 2 for a superseded caller climbing out of the scratch dir, got $rc -- $out"
out="$(hook roster.knobs "$(payload stage-session a94d72dcd2db52dbf notes.md)")"; rc=$?
[[ "$rc" -eq 0 && -z "$out" ]] || note cursor-holder "want a silent exit 0 for the caller holding the cursor, got $rc -- $out"

out="$(hook roster.knobs "$(payload stage-session a94d72dcd2db52dbf .workflow/WORKFLOW-STATE.txt)")"; rc=$?
[[ "$rc" -eq 2 ]] || note first-rule "want exit 2 for a stamped caller writing the state file, got $rc -- $out"

mkdir -p "$sb/stamps"
cp "$sb/.workflow/WORKFLOW-STATE.txt" "$sb/stamps/STATE.txt"
printf 'LIFECYCLE_KIT_STATE_FILE = stamps/STATE.txt\n' >"$sb/relocated.knobs"
out="$(hook relocated.knobs "$(payload - - stamps/STATE.txt)")"; rc=$?
[[ "$rc" -eq 2 ]] || note relocated "want exit 2 for a write to a state file relocated through LIFECYCLE_KIT_STATE_FILE, got $rc -- $out"
out="$(hook relocated.knobs "$(payload - - .workflow/WORKFLOW-STATE.txt)")"; rc=$?
[[ "$rc" -eq 0 && -z "$out" ]] || note relocated-old "want a silent exit 0 for the path the relocated knob left, got $rc -- $out"

printf 'LIFECYCLE_KIT_STAGE_SESSION_TYPES[] = stage-session\nLIFECYCLE_KIT_STATE_FILE = .workflow/absent.txt\n' >"$sb/absent.knobs"
out="$(hook absent.knobs "$(payload stage-session b1b2b3b4c5c6c7c8d notes.md)")"; rc=$?
[[ "$rc" -eq 0 ]] || note decline "want exit 0 for an unreadable state file, got $rc -- $out"
grep -qF '"additionalContext"' <<<"$out" || note decline-envelope "the decline did not write the PreToolUse advise envelope: $out"

[[ "$fails" -eq 0 ]] || { echo "stamp-before-write.test: $fails assertion(s) failed"; exit 1; }
echo "stamp-before-write.test: clean (an unstamped stage-session caller is blocked naming its id and the remedy; a caller whose stage the cursor has left is blocked naming both stages, allowed under the scratch dir and blocked climbing out of it; a stamped one holding the cursor, another type, a top-level caller and an empty roster pass silently; an unreadable state file declines through the advise envelope; the state-file rule still blocks a stamped caller and follows a relocated LIFECYCLE_KIT_STATE_FILE)"
exit 0
