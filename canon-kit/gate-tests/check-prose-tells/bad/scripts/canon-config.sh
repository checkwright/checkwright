# shellcheck shell=bash
# spec: canon-kit/SPEC.md §check-prose-tells — the fixture opts in one prose surface so the gate has something to scan
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_PROSE_TELL_GLOBS=("prose.md")

# comment-tier-exempt: the base array stays at its bundled default so the trip proves the union, not a replacement
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_PROSE_TELL_PHRASES_EXTRA=("Suffice it to say")
