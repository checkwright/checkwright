#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §Consumer rules — consumer-copy PreToolUse(Bash) hook: project block/steer/allow rules before guard-kit's generic ruleset; wire via templates/settings-hooks.json
# no-port: gate-sdk/SPEC.md §The harness-template port disposition — the class ruling of 2026-08-30, reached by ground rather than by scope: this file's whole body is a resolve-source-read preamble around one [EDIT ME] gap, so the gap IS the file and porting it deletes the extension point there is to fill. guard-kit/SPEC.md §Consumer rules owns the placement — guard-kit ships no consumer rule and names none, and what a project blocks or steers stays in its copy — so a compiled form would have nothing to compile. Structural, not a sizing judgment.
set -uo pipefail

# shellcheck disable=SC2034  # consumed by the sourced lib/guard.sh (guard_block et al.)
GUARD_NAME="bash-guard"
GUARD_KIT_LIB="${GUARD_KIT_LIB:-guard-kit/lib/guard.sh}"
[[ -f "$GUARD_KIT_LIB" ]] || exit 0
# shellcheck source=/dev/null  # vendored lib path is resolved at runtime; fail-open above if absent, but the lib's own exit 2 (set-but-missing config) must stay loud
source "$GUARD_KIT_LIB"

# spec: guard-kit/SPEC.md §The guard framework — cache the payload before the first field is read, so a rule needing a tool-input field beyond the command can reach one
guard_read_input || exit 0
cmd="$(guard_read_command)" || exit 0

# spec: guard-kit/SPEC.md §Consumer rules — add project block/steer/allow rules here (compose guard.sh primitives), before the generic ruleset [EDIT ME]

guard_generic_rules "$cmd"
guard_log_fallthrough "$cmd"
exit 0
