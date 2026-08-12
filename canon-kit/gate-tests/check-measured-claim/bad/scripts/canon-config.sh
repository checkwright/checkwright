# shellcheck shell=bash
# spec: canon-kit/SPEC.md §check-measured-claim — fixture consumer config: the good pair's oracle unchanged, so the bad case differs only in what the prose claims
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_MEASURED_CLAIMS_CMD='printf "gate-total\t7\nsupported-hosts\tlinux,macos\n"'
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_MEASURED_SURFACE_GLOBS=("*.md")
