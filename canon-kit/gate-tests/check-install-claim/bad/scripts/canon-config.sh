# shellcheck shell=bash
# spec: canon-kit/SPEC.md §check-install-claim — fixture consumer config: two transports, an anchored install-section regex, and the published-notes valve the pair proves is load-bearing
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_MANIFEST_FILES=("*.md" "posts/*.md")
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_INSTALL_TRANSPORTS_CMD="bash scripts/install-transports.sh"
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_INSTALL_SECTION_RE='^(Quick start|Install)'
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_INSTALL_CLAIM_EXCLUDE=("posts/*")
