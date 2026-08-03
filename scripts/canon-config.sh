# shellcheck shell=bash
# spec: canon-kit/SPEC.md §Layout and configuration — this repo's canon-kit consumer config

# spec: canon-kit/SPEC.md §Layout and configuration — the design-pending section set is deferred plus icebox; this repo runs the icebox tier, so its canon-kit counterpart of QUEUE_KIT_ICEBOX_SECTION is set here
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_ICEBOX_SECTION=Icebox

# comment-tier-exempt: this repo's component specs ARE the kit SPECs (a reference-spec corpus with no Definition-of-Done), so DoD-singleton runs at-most-one
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_DOD_MODE=at-most-one

# comment-tier-exempt: the kits are this repo's own first-party components, so the spec finders scan their SPEC.md rather than prune them as vendored roots
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_SCAN_KIT_ROOTS=1

# comment-tier-exempt: deliberate-absence rulings may narrate what the kit excludes — an "Out of scope" section states what a kit does not carry and why, so the whole section is exempt from the temporal-narration gate
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_TEMPORAL_EXEMPT_SECTIONS=("Out of scope")

# comment-tier-exempt: the docs/ site joins the governed manifest set by explicit wiring — this knob replaces the default set, so it enumerates the prior default (CLAUDE.md, README.md at any depth, kit SPEC.md/README.md — single-level globs skip the gate-tests/ fixtures the finder pruned) plus the docs/ living pages, the dated posts, the root contribution, conduct, disclosure, release, public-direction and ruling-record surfaces (CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md, RELEASING.md, ROADMAP.md, TRAJECTORY.md), and the doctrine deliverable (doctrine-kit/DOCTRINE.md) so its links and commands resolve under the doc gates
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_MANIFEST_FILES=(
    "CLAUDE.md"
    "README.md"
    "CONTRIBUTING.md"
    "CODE_OF_CONDUCT.md"
    "SECURITY.md"
    "RELEASING.md"
    "ROADMAP.md"
    "TRAJECTORY.md"
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

# comment-tier-exempt: reserve/ is the crates.io name-reservation placeholder, not developed in (CLAUDE.md §Housekeeping) — its lib.rs carries the registry blurb crates.io renders, which is the file's whole purpose and answers to no SPEC section
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_COMMENT_WHITELIST=("reserve/*")

# comment-tier-exempt: this repo's install transports and the sections they are held over — the vocabulary is consumer config because a kit literal spelling a transport would publish one project's distribution model; the section regex is anchored so a heading merely mentioning a script name (### install-hooks) never selects, and the posts valve keeps a published release note immutable the same way CANON_KIT_TEMPORAL_EXEMPT_PATHS already does
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_INSTALL_TRANSPORTS_CMD="bash scripts/install-transports.sh"
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_INSTALL_SECTION_RE='^(Quick start|Install)'
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_INSTALL_CLAIM_EXCLUDE=("docs/posts/*")

# comment-tier-exempt: the enum-set emitter derives the queue-tag set from queue-kit's own tag parser plus this repo's lesson tags, so a prose enumeration of the tag vocabulary that drops a member is caught rather than trusted
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_ENUM_SETS_CMD="bash scripts/enum-sets.sh"

# comment-tier-exempt: the slot-free kit-template and agent-definition surfaces this repo governs — the finder includes a candidate iff it bears no binding slot, so slot-bearing templates (lead.md, the stage skills, agent-execution.md) self-exclude; the stage-skill shims under .claude/commands/ stay out (they are consumer bindings, governed by check-skill-binding/check-shim-restatement)
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_PROSE_SURFACE_GLOBS=(
    "*/templates/*.md"
    ".claude/agents/*.md"
)

# comment-tier-exempt: this repo's reader-facing prose surfaces for check-prose-tells — the hand-authored top-level docs living pages only; the single-level docs/*.md glob deliberately excludes the generated kit mirror (docs/<kit>/) and the immutable dated posts (docs/posts/), since a prose gate forcing edits to generated or immutable pages contradicts them; consumer editorial scope, never a kit literal (the provenance seam)
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_PROSE_TELL_GLOBS=(docs/*.md)

# comment-tier-exempt: tokens this repo's audience reads as jargon-free and so exempt from the undefined-abbreviation tell — this repo's own standard names alongside industry terms nobody expands on an install page; consumer vocabulary, never a kit literal (the provenance seam)
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_PROSE_TELL_ABBR_ALLOW_EXTRA=(
    "SPEC" "KPI" "README" "CNAME" "CLAUDE" "DOCTRINE" "ROADMAP" "GNU" "BSD"
)
