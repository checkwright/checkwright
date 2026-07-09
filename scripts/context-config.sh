# shellcheck shell=bash
# spec: context-kit/SPEC.md §Layout and configuration — this repo's context-kit consumer config

# comment-tier-exempt: this repo budgets the gate-sdk conventions roster as its always-loaded section, not the platform default "## Shared conventions"
# shellcheck disable=SC2034  # consumed by context-kit/checks/check-brevity.sh after sourcing
CONTEXT_KIT_BREVITY_SECTION="## Conventions established in gate-sdk (keep every kit consistent)"

# comment-tier-exempt: queue-kit lives at queue-kit/bin/ in this monorepo, so name the hook body explicitly rather than lean on the default gates-dir probe
# shellcheck disable=SC2034  # consumed by context-kit/bin/always-loaded.sh after sourcing
CONTEXT_KIT_HOOK_CMD="bash queue-kit/bin/queue-index.sh --collapse-deferred"
