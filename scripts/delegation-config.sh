# shellcheck shell=bash
# spec: delegation-kit/SPEC.md §Layout and configuration — this repo's delegation-kit consumer config

# comment-tier-exempt: `*` spans '/' in a bash [[ == ]] glob, so */checks/*.sh reaches every kit's gates
# shellcheck disable=SC2034  # consumed by delegation-kit/lib/delegation.sh after sourcing
DELEGATION_KIT_GATE_FILES=(
    "*/checks/*.sh"
    "gate-sdk/lib/gate.sh"
    "gate-sdk/bin/run-gate-tests.sh"
)

# comment-tier-exempt: the kit roots are auto-unioned by delegation.sh (a vendored kit's edits are meta-layer by definition), so only the non-kit prefixes are declared here
# shellcheck disable=SC2034  # consumed by delegation-kit/lib/delegation.sh after sourcing
DELEGATION_KIT_META_PATHS=(
    scripts/ .workflow/ .claude/
)
