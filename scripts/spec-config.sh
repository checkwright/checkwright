# shellcheck shell=bash
# spec: spec-kit/SPEC.md §Layout and configuration — this repo's spec-kit consumer config

# comment-tier-exempt: this repo's component specs ARE the kit SPECs (a reference-spec corpus with no Definition-of-Done), so DoD-singleton runs at-most-one
# shellcheck disable=SC2034  # consumed by spec-kit/lib/spec.sh after sourcing
SPEC_KIT_DOD_MODE=at-most-one

# comment-tier-exempt: the kits are this repo's own first-party components, so the spec finders scan their SPEC.md rather than prune them as vendored roots
# shellcheck disable=SC2034  # consumed by spec-kit/lib/spec.sh after sourcing
SPEC_KIT_SCAN_KIT_ROOTS=1

# comment-tier-exempt: deliberate extraction provenance is this repo's convention, not kit mechanism — a "What stayed on the platform" section narrates what the seam left behind, so the whole section is exempt from the temporal-narration gate
# shellcheck disable=SC2034  # consumed by spec-kit/lib/spec.sh after sourcing
SPEC_KIT_TEMPORAL_EXEMPT_SECTIONS=("What stayed on the platform")
