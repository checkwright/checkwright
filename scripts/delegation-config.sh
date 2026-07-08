# shellcheck shell=bash
# spec: delegation-kit/SPEC.md §Layout and configuration — this repo's delegation-kit consumer config

# comment-tier-exempt: `*` spans '/' in a bash [[ == ]] glob, so */checks/*.sh reaches every kit's gates
# shellcheck disable=SC2034  # consumed by delegation-kit/lib/delegation.sh after sourcing
DELEGATION_KIT_GATE_FILES=(
    "*/checks/*.sh"
    "gate-sdk/lib/gate.sh"
    "gate-sdk/bin/run-gate-tests.sh"
)

# comment-tier-exempt: this repo is a monorepo of kits — every kit dir is meta-layer, since a gate edit legitimately rides with its kit's SPEC.md, README, and fixtures
# shellcheck disable=SC2034  # consumed by delegation-kit/lib/delegation.sh after sourcing
DELEGATION_KIT_META_PATHS=(
    scripts/ .workflow/ .claude/
    gate-sdk/ lifecycle-kit/ queue-kit/ spec-kit/ guard-kit/ delegation-kit/ context-kit/ drift-kit/
)
