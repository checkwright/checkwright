# shellcheck shell=bash
# context-kit consumer config for THIS repo (context-kit/SPEC.md §Layout and
# configuration). always-loaded.sh and check-brevity.sh source it, then fill any
# unset knob with the kit default. Only the knobs this repo diverges on are set.

# This repo's budgeted always-loaded section is the gate-sdk conventions roster,
# not the platform's "## Shared conventions".
# shellcheck disable=SC2034  # consumed by context-kit/checks/check-brevity.sh after sourcing
CONTEXT_KIT_BREVITY_SECTION="## Conventions established in gate-sdk (keep later kits consistent)"

# The steady-state hook body: queue-kit lives at queue-kit/bin/ here (not the
# consumer gates dir), so name it explicitly rather than lean on the default probe.
# shellcheck disable=SC2034  # consumed by context-kit/bin/always-loaded.sh after sourcing
CONTEXT_KIT_HOOK_CMD="bash queue-kit/bin/queue-index.sh --collapse-deferred"
