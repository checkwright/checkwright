# shellcheck shell=bash
# context-kit consumer config (context-kit/SPEC.md §Layout and configuration).
# Copy into your gates dir (default scripts/), or point CONTEXT_KIT_CONFIG_FILE
# at it, and uncomment any knob you want to override. always-loaded.sh and
# check-brevity.sh source this first, then fill every unset knob with the
# platform-value default shown here.

# ---- the always-loaded meter (bin/always-loaded.sh) -------------------------

# Always-loaded surface files — the standing per-session cost the meter sums.
# CONTEXT_KIT_SURFACES=("CLAUDE.md")

# Command whose output line count approximates the steady-state session-start
# hook body. Default: queue-kit's collapsed queue index when the script
# resolves (consumer gates dir, then a sibling queue-kit/bin), else empty
# (surfaces only). NEVER point this at the session-context hook — that hook
# emits the meter's own line, so self-measurement would recurse and inflate.
# CONTEXT_KIT_HOOK_CMD="bash scripts/queue-index.sh --collapse-deferred"

# Committed baseline the meter deltas against; --update-baseline rewrites it.
# CONTEXT_KIT_BASELINE_FILE="${GATE_SDK_WORKFLOW_DIR:-.workflow}/always-loaded-baseline.txt"

# ---- the brevity gate (checks/check-brevity.sh) -----------------------------

# The always-loaded file the gate scans, and the budgeted bullet section in it.
# CONTEXT_KIT_BREVITY_FILE="CLAUDE.md"
# CONTEXT_KIT_BREVITY_SECTION="## Shared conventions"

# Lines allowed per `- **name:**` bullet before it is over budget.
# CONTEXT_KIT_BREVITY_BUDGET=4

# The "cites a deeper doc" pattern (a bash regex): an over-budget bullet is
# flagged only when its body matches this. Default `§` (any section pointer).
# CONTEXT_KIT_BREVITY_POINTER_RE="§"
