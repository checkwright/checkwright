#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §wakeup-guard — consumer-copy PreToolUse(SendMessage) advisory hook: nudge a headerless escalation to the lead toward the decision shape
set -uo pipefail

# shellcheck disable=SC2034  # GUARD_NAME is read by the advisory prefix below
GUARD_NAME="escalation-guard"

input="$(cat 2>/dev/null || true)"

# spec: guard-kit/SPEC.md §wakeup-guard — advisory posture: never block; an uninspectable payload passes
command -v jq >/dev/null 2>&1 || exit 0

to="$(printf '%s' "$input" | jq -r '.tool_input.to // empty' 2>/dev/null || true)"
message="$(printf '%s' "$input" | jq -r '.tool_input.message // empty' 2>/dev/null || true)"

# spec: guard-kit/SPEC.md §wakeup-guard — only an upward message (to the lead as "main") is an escalation
[[ "$to" == "main" ]] || exit 0

missing=()
for h in Question Options Recommendation Evidence; do
    grep -qiw "$h" <<<"$message" || missing+=("$h")
done
[[ ${#missing[@]} -eq 0 ]] && exit 0

printf '%s' "$GUARD_NAME: this message to the lead is missing the decision-shape header(s): ${missing[*]}. An escalation batches every open question as Question / Options / Recommendation / Evidence so the lead can rule and resume you in place; routine narration belongs in the resume journal, not the message channel. (guard-kit/SPEC.md §wakeup-guard)" \
    | jq -Rc '{hookSpecificOutput:{hookEventName:"PreToolUse",additionalContext:.}}' 2>/dev/null || true
exit 0
