# shellcheck shell=bash
# spec: canon-kit/SPEC.md §check-deprecation-task — the marker literal is spelled
#   in two pieces because the default corpus derivation scans this config file
#   too, and a contiguous spelling here would be an occurrence of the vocabulary
#   it declares
CANON_KIT_DEPRECATION_MARKERS=("@dep"'recated')
