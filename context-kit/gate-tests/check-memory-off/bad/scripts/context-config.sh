# shellcheck shell=bash
# comment-tier-exempt: the case's own config seam — a knob is how a case reaches this gate now that the `--fixture` arm is gone, and it is what makes the pair drive the code path the live battery drives
# shellcheck disable=SC2034  # consumed by check-memory-off through the config bridge after sourcing
CONTEXT_KIT_MEMORY_DIRS="memory"
