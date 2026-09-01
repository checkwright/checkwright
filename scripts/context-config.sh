# shellcheck shell=bash
# spec: context-kit/SPEC.md §Layout and configuration — this repo's context-kit consumer config
# no-port: gate-sdk/SPEC.md §The config-seam port disposition — the class ruling of 2026-08-30, reached by ground rather than by scope: this is the seeded-copy side of context-kit's config seam, the file this repo actually edits and the more edited of the two, so porting it deletes the seam outright. Every knob here is this repo's own layout — an always-loaded section heading, a prune set deliberately unequal to the gate library's, a hook command naming where gate-sdk sits in this monorepo — which is the adopter half of the seam by definition. The 2026-08-24 vocabulary ruling left it owed deliberately — a heading, a generic prune set and a hook command, read as layout — which answers "does this hold private vocabulary?" and never "is this an edit seam?"; that verdict is untouched here and the two grounds are cumulative. Structural, not a sizing judgment.

# comment-tier-exempt: this repo budgets the gate-sdk conventions roster as its always-loaded section, not the platform default "## Shared conventions"
# shellcheck disable=SC2034  # consumed by check-brevity through the config bridge after sourcing
CONTEXT_KIT_BREVITY_SECTION="## Conventions established in gate-sdk (keep every kit consistent)"

# comment-tier-exempt: assigned explicitly rather than derived from GATE_PRUNE_DIRS — the two sets are deliberately not identical (this one omits .tmp and gate-tests, and carries dist and build), and deriving one from the other would make context-kit's own exclusion rule a gate-sdk read
# shellcheck disable=SC2034  # consumed by the md-index and pub-index arms across the config bridge after sourcing
CONTEXT_KIT_PRUNE_DIRS=(.git node_modules target dist build worktrees)

# comment-tier-exempt: gate-sdk lives at gate-sdk/bin/ in this monorepo, so name the front-end explicitly rather than lean on the default gates-dir probe
# shellcheck disable=SC2034  # consumed by context-kit/bin/always-loaded.sh after sourcing
CONTEXT_KIT_HOOK_CMD="bash gate-sdk/bin/run-gates.sh --emit queue-index --collapse-deferred"
