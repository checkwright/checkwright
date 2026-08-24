# shellcheck shell=bash
# spec: context-kit/SPEC.md §Layout and configuration — this repo's context-kit consumer config

# comment-tier-exempt: this repo budgets the gate-sdk conventions roster as its always-loaded section, not the platform default "## Shared conventions"
# shellcheck disable=SC2034  # consumed by check-brevity through the config bridge after sourcing
CONTEXT_KIT_BREVITY_SECTION="## Conventions established in gate-sdk (keep every kit consistent)"

# comment-tier-exempt: assigned explicitly rather than derived from GATE_PRUNE_DIRS — the two sets are deliberately not identical (this one omits .tmp and gate-tests, and carries dist and build), and a hard read of the gate library would make an advisory bin/ tool fail in a tree that vendored context-kit without gate-sdk
# shellcheck disable=SC2034  # consumed by context-kit/bin/md-index.sh and bin/pub-index.sh after sourcing
CONTEXT_KIT_PRUNE_DIRS=(.git node_modules target dist build worktrees)

# comment-tier-exempt: gate-sdk lives at gate-sdk/bin/ in this monorepo, so name the front-end explicitly rather than lean on the default gates-dir probe
# shellcheck disable=SC2034  # consumed by context-kit/bin/always-loaded.sh after sourcing
CONTEXT_KIT_HOOK_CMD="bash gate-sdk/bin/run-gates.sh --emit queue-index --collapse-deferred"
