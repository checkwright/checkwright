#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §check-stage-evidence — consumer-copy PreToolUse(Write|Edit) hook: the stage stamp is bin/enter-stage.sh's to write, and every gate that would catch a hand-stamp fires only at commit
set -uo pipefail

# shellcheck disable=SC2034  # consumed by the sourced lib/guard.sh (guard_block)
GUARD_NAME="workflow-state-guard"
GUARD_KIT_LIB="${GUARD_KIT_LIB:-guard-kit/lib/guard.sh}"
[[ -f "$GUARD_KIT_LIB" ]] || exit 0
# shellcheck source=/dev/null  # vendored lib path is resolved at runtime; fail-open above if absent, but the lib's own exit 2 (set-but-missing config) must stay loud
source "$GUARD_KIT_LIB"

STATE_FILE="${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt"

input="$(cat 2>/dev/null || true)"

# spec: guard-kit/SPEC.md §The guard framework — fail-open-but-loud: the rule turns on a payload field and on path resolution, so a call it cannot judge is allowed with an advisory naming the unenforced rule; guard_advise is itself jq-backed, so this branch emits the envelope directly rather than reaching for a primitive its own trigger disabled
if ! command -v jq >/dev/null 2>&1 || ! command -v readlink >/dev/null 2>&1; then
    printf '%s\n' '{"hookSpecificOutput":{"hookEventName":"PreToolUse","additionalContext":"workflow-state-guard: jq or readlink is unavailable, so the direct-edit rule for the lifecycle state file could not be enforced on this call. That file has one sanctioned writer, lifecycle-kit bin/enter-stage.sh."}}'
    exit 0
fi

path="$(guard_read_path <<<"$input")" || exit 0

# spec: lifecycle-kit/SPEC.md §check-stage-evidence — resolved comparison, never textual: an absolute path, a ./ prefix and a path reaching the file through a symlinked directory all name one file, and a textual match catches only the spelling it was written against
_resolve() { readlink -f -- "$1" 2>/dev/null || printf '%s' "${1#./}"; }

[[ "$(_resolve "$path")" == "$(_resolve "$STATE_FILE")" ]] || exit 0

guard_block "$STATE_FILE is written by lifecycle-kit's bin/enter-stage.sh, never by hand — run 'bash lifecycle-kit/bin/enter-stage.sh <stage>' to stamp, or 'bash lifecycle-kit/bin/enter-stage.sh --rename <name>' to rename the iteration (it rewrites the queue header and column 1 of every stamp in one motion, proving columns 2-4 unchanged). The stamp *is* the stage transition, so a hand-written line moves the cursor for every reader for the rest of the session, and every gate that would catch it fires only at commit: an uncommitted hand-stamp is never seen at all. If enter-stage refuses, that refusal is a gate verdict to resolve at its source, not to write around."
