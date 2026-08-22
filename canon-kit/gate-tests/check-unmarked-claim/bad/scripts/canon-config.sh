# shellcheck shell=bash
# spec: canon-kit/SPEC.md §check-unmarked-claim — fixture consumer config: the good pair's class and surface unchanged, so the bad case differs only in whether the prose carries an oracle
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_CLAIM_CLASSES_CMD='printf "engine-substrate\tthe engine is (a|one) (small )?(shell script|shell program)\n"'
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_MEASURED_SURFACE_GLOBS=("*.md")
