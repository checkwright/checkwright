# shellcheck shell=bash
# spec: canon-kit/SPEC.md §check-payload-claim — fixture consumer config: two disclosure classes and the published-notes valve the pair proves is load-bearing
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_MANIFEST_FILES=("*.md" "posts/*.md")
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_PAYLOAD_CLAIMS_CMD="bash scripts/payload-claims.sh"
# shellcheck disable=SC2034  # consumed by canon-kit/lib/spec.sh after sourcing
CANON_KIT_PAYLOAD_CLAIM_EXCLUDE=("posts/*")
