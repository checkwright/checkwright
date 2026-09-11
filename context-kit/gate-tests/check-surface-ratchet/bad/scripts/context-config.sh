# shellcheck shell=bash
# comment-tier-exempt: the case's own config seam — a knob is how a case reaches this gate, and pointing the ceiling file and the pathspecs at the case dir is what makes the pair drive the code path the live battery drives
# shellcheck disable=SC2034  # consumed by check-surface-ratchet through the config bridge after sourcing
CONTEXT_KIT_SURFACES=("CLAUDE.md")
# shellcheck disable=SC2034  # consumed by check-surface-ratchet through the config bridge after sourcing
CONTEXT_KIT_RATCHET_PATHS=('templates/*.md')
# shellcheck disable=SC2034  # consumed by check-surface-ratchet through the config bridge after sourcing
CONTEXT_KIT_CEILING_FILE="surface-ceiling.txt"
