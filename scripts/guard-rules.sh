#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §Consumer rules — this repo's consumer rule command, run by the shell guard ahead of the generic ruleset: the hook payload on stdin, a block as exit 2 with its message on stderr, nothing printed when no rule fires
# no-port: CLAUDE.md §The provenance seam (never cross it) — this file is this project's own blocking rule content, whose messages name this repo's own instructions, which a kit is forbidden to hold: porting it into native/ would ship it in every adopter's binary. The ground its predecessor, the copied bash guard, declared. Structural, not a sizing judgment.
set -uo pipefail

# spec: gate-sdk/SPEC.md §lib/gate.sh — the binary a harness-integration arm runs, so a linked worktree reaches the main checkout's
# shellcheck source=../gate-sdk/lib/gate.sh
source "$(dirname "${BASH_SOURCE[0]}")/../gate-sdk/lib/gate.sh"
bin="$(gate_harness_bin)"

block() {
    printf 'guard-rules: %s\n' "$1" >&2
    exit 2
}

# spec: guard-kit/SPEC.md §Consumer rules — every rule matches the 'sq dq hd' view: none tests for an expansion, and a heredoc body naming a bypass flag or a scratchpad path is prose, not the executable command
cmd="$("$bin" --guard-json view sq dq hd)" || {
    printf 'guard-rules: cannot read the command view through %s\n' "$bin" >&2
    exit 1
}

# spec: CLAUDE.md §This repo is governed by its own kits — a hook bypass is a one-off with cause, so it must stay visible: the allowlisted 'git commit -m *' glob would otherwise auto-allow a trailing bypass flag
case " $cmd " in
    *" git commit "*"--no-verify"*|*" git commit -n "*)
        block "a hook bypass (--no-verify/-n) is a one-off with cause, never auto-allowed — fix the red gate instead, or run the bypass yourself with !<command> so the cause is on record."
        ;;
esac
# spec: CLAUDE.md §Housekeeping — .tmp/ is this repo's disposable scratch; the harness's per-session /tmp scratchpad leaks session work outside it, so the path prefix is blocked. Prefix-only match: kit mechanism legitimately uses TMPDIR (the hermetic bootstrap).
if [[ "$cmd" == *"/tmp/claude-"* ]]; then
    block "the harness per-session scratchpad (/tmp/claude-...) is not this repo's scratch home — use repo-local .tmp/ instead (CLAUDE.md §Housekeeping): it survives crashes in-tree and is wiped at the scope boundary. If you genuinely need the harness path, run it yourself with !<command>."
fi
# spec: CLAUDE.md §Housekeeping — .metric/ is gitignored persistent measurement trends and .tmp/ crash-recovery scratch; git clean -x/-X wipes both, so steer to the !<command> escape rather than let the destructive form auto-run
if [[ " $cmd " == *" git clean "* && " $cmd " =~ [[:space:]]-[A-Za-z]*[xX] ]]; then
    block "git clean -x/-X wipes gitignored state — the irreplaceable measurement trends under .metric/ and crash-recovery resume journals under .tmp/. If you mean to discard them, run it yourself with !<command> so the intent is on record."
fi
exit 0
