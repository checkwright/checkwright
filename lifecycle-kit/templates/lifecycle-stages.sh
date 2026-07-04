# shellcheck shell=bash
# Consumer stage-machine config for lifecycle-kit (lifecycle-kit/SPEC.md
# §lib/stages.sh). Copy into your gates dir as lifecycle-stages.sh (or point
# LIFECYCLE_KIT_STAGES_FILE at it). Every knob is optional: anything left
# unset keeps the kit default shown here; set a knob to '' to disable it
# where the SPEC says emptiness disables.

# The legal [stage:] header values.
#LIFECYCLE_STAGES=(scope align build validate close)

# Mandatory predecessor per stage — the stamp check-stage-entry demands at the
# flip. A stage absent from the map has no mandatory predecessor. Keep a
# trigger-gated stage (align) out of the map: demanding it before every build
# would false-fire on an iteration that legitimately skipped it.
#declare -A LIFECYCLE_PREDECESSOR=([align]=scope [build]=scope [validate]=build [close]=validate)

# The iteration-boundary stage: the only stage at which the unnamed-iteration
# sentinel '—' is legal (header and stamp).
#LIFECYCLE_FIRST_STAGE=scope

# Entering this stage requires the active queue drained ('' disables).
#LIFECYCLE_DRAIN_STAGE=validate

# The queue sections whose top-level '- ' entries constitute the active queue.
# Plain text — each name is spliced into a '^## (…)$' heading regex, so avoid
# regex metacharacters.
#LIFECYCLE_ACTIVE_SECTIONS=("New Features" "Technical Debt")

# The trigger-gated audit stage, and the stage whose entry demands an audit
# stamp (or a recorded waiver) when the cross-component amendment signal
# fires. LIFECYCLE_AUDIT_STAGE='' disables the signal check entirely.
#LIFECYCLE_AUDIT_STAGE=align
#LIFECYCLE_AUDIT_ENTRY_STAGE=build

# The recorded-waiver stamp token — a legal stamp line, never a header stage.
# Default: '<audit-stage>-waived'.
#LIFECYCLE_WAIVER_TOKEN=align-waived

# Amendment-file shape, the component-roster marker (a dir holding one is a
# component), and the path tokens whose mention in an amendment body counts
# as naming a component's contract surface.
#LIFECYCLE_AMENDMENT_GLOB='SPEC-*.md'
#LIFECYCLE_ROSTER_BASENAME='SPEC.md'
#LIFECYCLE_CONTRACT_TOKENS=("SPEC.md" "proto/")

# Governed files (repo-root-relative; the gates also take them as $1/$2).
#LIFECYCLE_QUEUE_FILE=TASK-QUEUE.md
#LIFECYCLE_STATE_FILE=.workflow/WORKFLOW-STATE.txt
