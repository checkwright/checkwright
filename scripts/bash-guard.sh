#!/usr/bin/env bash
# spec: friction-kit/SPEC.md — PreToolUse(Bash) hook: block, steer, or auto-allow.
#
# CONSUMER COPY. Copy this into your gates dir (default scripts/), wire it as
# the PreToolUse(Bash) hook (see templates/settings-hooks.json), and add your
# project rules in the marked section below. The generic ruleset and hook
# primitives live in the vendored friction-kit/lib/guard.sh — override its path
# with FRICTION_KIT_LIB if the kit is vendored elsewhere.
set -uo pipefail

# shellcheck disable=SC2034  # consumed by the sourced lib/guard.sh (guard_block et al.)
GUARD_NAME="bash-guard"
FRICTION_KIT_LIB="${FRICTION_KIT_LIB:-friction-kit/lib/guard.sh}"
# shellcheck source=/dev/null  # vendored lib path is resolved at runtime; fail-open if absent
source "$FRICTION_KIT_LIB" 2>/dev/null || exit 0

cmd="$(guard_read_command)" || exit 0

# ===== consumer rules — edit this section ====================================
# Project-specific block/steer/allow rules go here, BEFORE the generic ruleset.
# Two ordering disciplines carried from the source platform:
#   - a command about to be *blocked* must never first trigger a side-effecting
#     rule — place blocks before any reclaim/cleanup rule;
#   - a steering rule must precede the broader rule that would catch the same
#     command less precisely.
# Compose from the guard.sh primitives, e.g.:
#   grep -qE '<your pattern>' <<<"$cmd" && guard_block "<what> — <corrective form>"
# ===== end consumer rules ====================================================

guard_generic_rules "$cmd"      # rules 1-8 (see friction-kit/SPEC.md)
guard_log_fallthrough "$cmd"    # rule 9: log anything neither blocked nor auto-allowed
exit 0
