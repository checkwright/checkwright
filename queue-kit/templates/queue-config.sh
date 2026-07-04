# shellcheck shell=bash
# Consumer queue config for queue-kit (queue-kit/SPEC.md §Layout and
# configuration). Copy into your gates dir as queue-config.sh (or point
# QUEUE_KIT_CONFIG_FILE at it). Every knob is optional: anything left unset
# keeps the kit default shown here. A malformed config exits 2 — a broken
# grammar must not gate anything.

# The governed queue file (repo-root-relative; every gate also takes it as $1).
#QUEUE_KIT_QUEUE_FILE=TASK-QUEUE.md

# The pickable queue sections, in selection order. Plain text — each name is
# spliced into a '^## (…)$' heading regex, so avoid regex metacharacters.
# Cross-kit note: lifecycle-kit's LIFECYCLE_ACTIVE_SECTIONS carries the same
# default; a consumer renaming its active sections sets both (independent knobs).
#QUEUE_KIT_ACTIVE_SECTIONS=("New Features" "Technical Debt")

# The parked section (excluded from selection) and the completed section.
#QUEUE_KIT_DEFERRED_SECTION=Deferred
#QUEUE_KIT_DONE_SECTION=Done

# The check-queue-wrap gate floor, in columns (the authoring target is ~80).
#QUEUE_KIT_WRAP_BUDGET=100

# Column-0 lead tokens exempt from the hygiene gate's no-prose axis. Whole-line
# lead match, not a substring — a line beginning with the token is allowed.
#QUEUE_KIT_PROSE_LEADS=("Protocol:")

# The forward-precondition trigger set for check-queue-prose-precondition, an
# extended regex matched against lowercased prose. Keep it present-tense and
# forward-looking (past-tense narration is stripped before matching); a broader
# set trades false positives for reach.
#QUEUE_KIT_PRECONDITION_REGEX='revisit when|once [^.]*(lands|ships|is (done|ready|merged))|gated on|contingent on|waiting on|pending [a-z]|blocked on'
