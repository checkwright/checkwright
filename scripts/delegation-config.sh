# shellcheck shell=bash
# Checkwright's own delegation-kit config (delegation-kit/SPEC.md §Layout and
# configuration). This repo is a monorepo of kits: its gate files live in each
# kit's checks/ dir (plus gate-sdk's sourced lib and fixture runner), and every
# kit dir is meta-layer. The budget knobs keep the kit defaults.

# Gate files: any kit's checks/ gate, plus the gate-sdk lib and fixture runner
# an agent could weaken. `*` spans '/' in a bash [[ == ]] glob.
# shellcheck disable=SC2034  # consumed by delegation-kit/lib/delegation.sh after sourcing
DELEGATION_KIT_GATE_FILES=(
    "*/checks/*.sh"
    "gate-sdk/lib/gate.sh"
    "gate-sdk/bin/run-gate-tests.sh"
)

# Meta-layer prefixes: the gates dir, the workflow projections, the harness
# config, and every kit directory (a gate edit legitimately rides with its
# kit's SPEC.md, README, and fixtures). Root-level *.md is always meta.
# shellcheck disable=SC2034  # consumed by delegation-kit/lib/delegation.sh after sourcing
DELEGATION_KIT_META_PATHS=(
    scripts/ .workflow/ .claude/
    gate-sdk/ lifecycle-kit/ queue-kit/ spec-kit/ friction-kit/ delegation-kit/ context-kit/
)
