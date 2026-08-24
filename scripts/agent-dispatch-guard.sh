#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §The delegation model — PreToolUse(Agent) dispatch-shape guard (consumer copy): D1 fork ban, D2 read-only isolation claim, D3 nested-dispatch advisory; registered under matcher `Agent` beside the budget guard (§Layout and configuration)
set -uo pipefail

# shellcheck disable=SC2034  # consumed by the sourced lib/guard.sh (guard_block/guard_advise)
GUARD_NAME="agent-dispatch-guard"
GUARD_KIT_LIB="${GUARD_KIT_LIB:-guard-kit/lib/guard.sh}"
[[ -f "$GUARD_KIT_LIB" ]] || exit 0
# shellcheck source=/dev/null  # vendored lib path is resolved at runtime; fail-open above if absent, but the lib's own exit 2 (set-but-missing config) must stay loud
source "$GUARD_KIT_LIB"

# spec: delegation-kit/SPEC.md §The delegation model — D2's roster is the only knob read here, so delegation-kit's validating loader is deliberately not sourced: a malformed *unrelated* knob must not wedge every dispatch
_adg_cfg="${DELEGATION_KIT_CONFIG_FILE:-${GATE_SDK_GATES_DIR:-scripts}/delegation-config.sh}"
_adg_roster_unreadable=""
if [[ -n "${DELEGATION_KIT_CONFIG_FILE:-}" && ! -f "$_adg_cfg" ]]; then
    _adg_roster_unreadable="DELEGATION_KIT_CONFIG_FILE names no readable file"
elif [[ -f "$_adg_cfg" ]]; then
    # shellcheck source=/dev/null  # consumer-supplied config, path is config
    source "$_adg_cfg"
fi
declare -p DELEGATION_KIT_READONLY_TYPES >/dev/null 2>&1 || DELEGATION_KIT_READONLY_TYPES=()

# spec: delegation-kit/SPEC.md §The delegation model — fail-open-but-loud: keep every advisory literal below free of any character JSON must escape, because the jq-absent arm hand-writes the envelope with no escaper
_adg_advise() {
    local msg="agent-dispatch-guard: $1"
    command -v jq >/dev/null 2>&1 && guard_advise "$msg"
    printf '{"hookSpecificOutput":{"hookEventName":"PreToolUse","additionalContext":"%s"}}\n' "$msg"
    exit 0
}

_adg_degraded() {
    _adg_advise "allowed this dispatch WITHOUT enforcing the fork ban (D1) or the read-only isolation claim (D2) — $1. Check the dispatch by hand: no fork, and a child claimed read-only takes isolation: worktree (delegation-kit/SPEC.md §The delegation model)."
}

input="$(cat 2>/dev/null || true)"

command -v jq >/dev/null 2>&1 || _adg_degraded "jq is not on PATH, so the payload could not be read"

_adg_fields="$(printf '%s' "$input" | jq -r 'if (.tool_input | type) == "object" then [(.tool_input.subagent_type // ""), (.tool_input.isolation // ""), (if .agent_id then "nested" else "" end)] | @tsv else empty end' 2>/dev/null || true)"
[[ -n "$_adg_fields" ]] || _adg_degraded "the hook payload did not parse, or carried no tool_input object"

# spec: delegation-kit/SPEC.md §The delegation model — split on a non-whitespace separator: bash collapses runs of a *whitespace* IFS, so a tab split would silently shift an absent middle field onto the next name
IFS=$'\037' read -r _adg_type _adg_isolation _adg_nested <<<"${_adg_fields//$'\t'/$'\037'}"

# spec: delegation-kit/SPEC.md §The delegation model — D1 precedes D2 because the fork ban is unconditional and its message is the more specific one for a dispatch violating both
if [[ "$_adg_type" == "fork" ]]; then
    guard_block "a fork inherits the dispatcher's whole context, toolset and model tier and disclaims nothing, so any narrowing this prompt states exists only as a sentence. Two lawful alternatives: dispatch a TYPED agent whose definition carries the narrower authority, brief and tier, so the narrowing is structural rather than requested; or, where the child does the same job at the same authority and you only want parallelism or its own index, dispatch that typed agent with isolation: worktree. There is no per-dispatch override — a knob here would restore the honour system this rule replaced, so the valve is unregistering the hook. The full protocol is /agent-execution (delegation-kit/SPEC.md §The delegation model)."
fi

if [[ -n "$_adg_type" && "$_adg_isolation" != "worktree" && ${#DELEGATION_KIT_READONLY_TYPES[@]} -gt 0 ]]; then
    for _adg_ro in "${DELEGATION_KIT_READONLY_TYPES[@]}"; do
        [[ "$_adg_type" == "$_adg_ro" ]] && guard_block "'$_adg_type' is declared a read-only dispatch type (DELEGATION_KIT_READONLY_TYPES), but this dispatch's shape grants write reach — a subagent inherits its toolset from its type whatever the prompt says, and a type carrying no Edit or Write still reaches git through its shell. Make the claim with the shape: add isolation: worktree, whose commits and index are the child's own, and which the harness best-effort auto-cleans afterwards. If this type is not in fact dispatched read-only, drop it from the roster rather than working around the rule here (delegation-kit/SPEC.md §The delegation model)."
    done
fi

_adg_notes=""
[[ -n "$_adg_roster_unreadable" ]] && _adg_notes="D2 (the read-only isolation claim) went unenforced on this dispatch: $_adg_roster_unreadable. "
[[ "$_adg_nested" == "nested" ]] && _adg_notes="${_adg_notes}you are yourself a dispatched agent, so this call creates a grandchild with no upward channel to you: it cannot message you mid-run, and neither level knows its own address or its parent's. Give it return-value-only work, or grant it a durable path in the main checkout, named absolutely in its prompt, and read that path yourself (delegation-kit/SPEC.md §Operative residency)."

[[ -n "$_adg_notes" ]] || exit 0
_adg_advise "$_adg_notes"
