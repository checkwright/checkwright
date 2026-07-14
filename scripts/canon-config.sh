# shellcheck shell=bash
# spec: canon-kit/SPEC.md §Layout and configuration — this repo's canon-kit consumer config

# comment-tier-exempt: this repo's component specs ARE the kit SPECs (a reference-spec corpus with no Definition-of-Done), so DoD-singleton runs at-most-one
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_DOD_MODE=at-most-one

# comment-tier-exempt: the kits are this repo's own first-party components, so the spec finders scan their SPEC.md rather than prune them as vendored roots
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_SCAN_KIT_ROOTS=1

# comment-tier-exempt: deliberate-absence rulings may narrate what the kit excludes — an "Out of scope" section states what a kit does not carry and why, so the whole section is exempt from the temporal-narration gate
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_TEMPORAL_EXEMPT_SECTIONS=("Out of scope")

# comment-tier-exempt: evidence-kit's baseline + manifest are data files whose leading '#' line is a wire-format contract header, not a source-comment directive or spec pointer — check-evidence-baseline/manifest own their grammar
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_COMMENT_WHITELIST=(
    ".workflow/validate-baseline.txt"
    ".workflow/validate-evidence.txt"
)

# comment-tier-exempt: the docs/ site joins the governed manifest set by explicit wiring — this knob replaces the default set, so it enumerates the prior default (CLAUDE.md, README.md at any depth, kit SPEC.md/README.md — single-level globs skip the gate-tests/ fixtures the finder pruned) plus the docs/ living pages, the dated posts, the root contribution and release surfaces (CONTRIBUTING.md, RELEASING.md), and the doctrine deliverable (doctrine-kit/DOCTRINE.md) so its links and commands resolve under the doc gates
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_MANIFEST_FILES=(
    "CLAUDE.md"
    "README.md"
    "CONTRIBUTING.md"
    "RELEASING.md"
    "*/SPEC.md"
    "*/README.md"
    "reserve/*/README.md"
    "doctrine-kit/DOCTRINE.md"
    "docs/*.md"
    "docs/*/index.md"
    "docs/posts/*.md"
)

# comment-tier-exempt: dated posts under docs/posts/ are immutable published artifacts whose dated narrative is their nature — a heading name cannot address a whole-file class, so the path valve exempts them from temporal-narration governance while link and command resolution still apply
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_TEMPORAL_EXEMPT_PATHS=("docs/posts/*")

# comment-tier-exempt: the enum-set emitter derives the queue-tag set from queue-kit's own tag parser plus this repo's lesson tags, so a prose enumeration of the tag vocabulary that drops a member is caught rather than trusted
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_ENUM_SETS_CMD="bash scripts/enum-sets.sh"

# comment-tier-exempt: the slot-free kit-template and agent-definition surfaces this repo governs — the finder includes a candidate iff it bears no binding slot, so slot-bearing templates (lead.md, the stage skills, agent-execution.md) self-exclude; the stage-skill shims under .claude/commands/ stay out (they are consumer bindings, governed by check-skill-binding/check-shim-restatement)
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_PROSE_SURFACE_GLOBS=(
    "*/templates/*.md"
    ".claude/agents/*.md"
)
