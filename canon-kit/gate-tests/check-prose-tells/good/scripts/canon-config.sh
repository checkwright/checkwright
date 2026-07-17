# shellcheck shell=bash
# spec: canon-kit/SPEC.md §check-prose-tells — the fixture opts in one prose surface so the gate has something to scan
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_PROSE_TELL_GLOBS=("prose.md")

# comment-tier-exempt: the base array is left at its bundled default on purpose — the _EXTRA token must clear assertion D by union, which a restated base set would prove nothing about
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_PROSE_TELL_ABBR_ALLOW_EXTRA=("ZFS")
