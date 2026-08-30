#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §Consumer rules — PreToolUse(Bash) hook (consumer copy): block, steer, or auto-allow
# no-port: CLAUDE.md §The provenance seam (never cross it) — the class ruling of 2026-08-30 at gate-sdk/SPEC.md §The harness-template port disposition, reached by ground rather than by scope: this copy carries this project's own blocking rules and the only # copy-divergence: declarations in the corpus, which is rule content a kit is forbidden to hold — the same material gate-sdk/SPEC.md §check-template-copy-parity's case: class discards rather than reads. It also takes its template's disposition under that section's bidirectional parity, so the ground here stands whatever becomes of the template's. Structural, not a sizing judgment.
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

# spec: guard-kit/SPEC.md §Consumer rules — project block/steer/allow rules go here, before the generic ruleset
# copy-divergence: guard_block — the template ships the allow/steer skeleton only; this repo's project rules are blocking ones, so the copy calls guard_block where the template calls neither
# copy-divergence: guard_skeleton — the template ships no project rule, so it needs no lexical view of its own; every rule this copy adds matches on one, and taking it from the normalizer is what keeps the copy off a sixth private stripping dialect
# spec: guard-kit/SPEC.md §The guard framework — the project rules take their lexical view from the one normalizer, declaring 'sq dq hd': none of them tests for an expansion, and a heredoc body naming a bypass flag or a scratchpad path is prose, not the executable command
cmd_unquoted="$(guard_skeleton "$cmd" sq dq hd)"
# spec: CLAUDE.md §This repo is governed by its own kits — a hook bypass is a one-off with cause, so it must stay visible: the allowlisted 'git commit -m *' glob would otherwise auto-allow a trailing bypass flag
case " $cmd_unquoted " in
    *" git commit "*"--no-verify"*|*" git commit -n "*)
        guard_block "a hook bypass (--no-verify/-n) is a one-off with cause, never auto-allowed — fix the red gate instead, or run the bypass yourself with !<command> so the cause is on record."
        ;;
esac
# spec: CLAUDE.md §Housekeeping — .tmp/ is this repo's disposable scratch; the harness's
# per-session /tmp scratchpad leaks session work outside it, so the path prefix is blocked.
# Prefix-only match: kit mechanism legitimately uses TMPDIR (the hermetic bootstrap).
if [[ "$cmd_unquoted" == *"/tmp/claude-"* ]]; then
    guard_block "the harness per-session scratchpad (/tmp/claude-...) is not this repo's scratch home — use repo-local .tmp/ instead (CLAUDE.md §Housekeeping): it survives crashes in-tree and is wiped at the scope boundary. If you genuinely need the harness path, run it yourself with !<command>."
fi
# spec: CLAUDE.md §Housekeeping — .metric/ is gitignored persistent measurement trends and .tmp/ crash-recovery scratch; git clean -x/-X wipes both, so steer to the !<command> escape rather than let the destructive form auto-run
if [[ " $cmd_unquoted " == *" git clean "* && " $cmd_unquoted " =~ [[:space:]]-[A-Za-z]*[xX] ]]; then
    guard_block "git clean -x/-X wipes gitignored state — the irreplaceable measurement trends under .metric/ and crash-recovery resume journals under .tmp/. If you mean to discard them, run it yourself with !<command> so the intent is on record."
fi
guard_generic_rules "$cmd"
guard_log_fallthrough "$cmd"
exit 0
