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

# comment-tier-exempt: evidence-kit's baseline + manifest are data files whose leading '#' line is a wire-format contract header, not a source-comment directive or spec pointer — check-evidence-baseline/manifest own their grammar
# shellcheck disable=SC2034  # consumed by spec-kit/lib/spec.sh after sourcing
SPEC_KIT_COMMENT_WHITELIST=(
    ".workflow/validate-baseline.txt"
    ".workflow/validate-evidence.txt"
)

# comment-tier-exempt: the docs/ site joins the governed manifest set by explicit wiring (SPEC-docs-site.md) — this knob replaces the default set, so it enumerates the prior default (CLAUDE.md, README.md at any depth, kit SPEC.md/README.md — single-level globs skip the gate-tests/ fixtures the finder pruned) plus the docs/ living pages and dated posts
# shellcheck disable=SC2034  # consumed by spec-kit/lib/spec.sh after sourcing
SPEC_KIT_MANIFEST_FILES=(
    "CLAUDE.md"
    "README.md"
    "*/SPEC.md"
    "*/README.md"
    "reserve/*/README.md"
    "docs/*.md"
    "docs/*/index.md"
    "docs/posts/*.md"
)

# comment-tier-exempt: dated posts under docs/posts/ are immutable published artifacts whose dated narrative is their nature — a heading name cannot address a whole-file class, so the path valve exempts them from temporal-narration governance while link and command resolution still apply (SPEC-docs-site.md)
# shellcheck disable=SC2034  # consumed by spec-kit/lib/spec.sh after sourcing
SPEC_KIT_TEMPORAL_EXEMPT_PATHS=("docs/posts/*")
