# shellcheck shell=bash
# spec: canon-kit/SPEC.md §check-unmarked-claim — fixture consumer config: one declared claim class and the shared measured surface, so the pair exercises the gate the empty defaults would clean-skip
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_CLAIM_CLASSES_CMD='printf "engine-substrate\tthe engine is (a|one) (small )?(shell script|shell program)\n"'
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_MEASURED_SURFACE_GLOBS=("*.md")
