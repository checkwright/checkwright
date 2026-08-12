# shellcheck shell=bash
# spec: canon-kit/SPEC.md §check-measured-claim — fixture consumer config: a two-key oracle (one cardinal, one extent) and a surface, so the pair exercises the gate the empty defaults would clean-skip
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_MEASURED_CLAIMS_CMD='printf "gate-total\t7\nsupported-hosts\tlinux,macos\n"'
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_MEASURED_SURFACE_GLOBS=("*.md")
