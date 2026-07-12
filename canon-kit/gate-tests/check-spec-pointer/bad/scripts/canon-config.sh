# shellcheck shell=bash
# The candidate glob the finder discriminates: the slot-free cand-tmpl.md is
# included, so its dangling citation is scanned and reddens.
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_PROSE_SURFACE_GLOBS=("cand-*.md")
