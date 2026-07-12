# shellcheck shell=bash
# The candidate glob the finder discriminates: the slot-bearing cand-tmpl.md is
# excluded, so its dangling citation is never scanned.
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_PROSE_SURFACE_GLOBS=("cand-*.md")
