# shellcheck shell=bash
# spec: lifecycle-kit/SPEC.md §lib/stages.sh — the stage machine as config: platform defaults, consumer overrides
#
# Sourced by the lifecycle gates. The extracted platform's five-stage
# lifecycle is the default; a consumer overrides any knob by dropping
# <gates-dir>/lifecycle-stages.sh (copy templates/lifecycle-stages.sh), or by
# pointing LIFECYCLE_KIT_STAGES_FILE at its config file. The stage names are
# mechanism defaults, not rule content — the consumer's own vocabulary lives
# in its config file, never here.

# Source the consumer config first so its assignments win; the defaults below
# fill only what it left unset (an explicitly empty value disables the knob).
_lc_cfg="${LIFECYCLE_KIT_STAGES_FILE:-}"
if [[ -n "$_lc_cfg" ]]; then
    [[ -f "$_lc_cfg" ]] || {
        echo "lifecycle-kit: LIFECYCLE_KIT_STAGES_FILE not found: $_lc_cfg" >&2
        exit 2
    }
    # shellcheck disable=SC1090  # consumer-supplied config, path is config
    source "$_lc_cfg"
else
    _lc_cfg="${GATE_SDK_GATES_DIR:-scripts}/lifecycle-stages.sh"
    if [[ -f "$_lc_cfg" ]]; then
        # shellcheck disable=SC1090  # consumer-supplied config, path is config
        source "$_lc_cfg"
    fi
fi
unset _lc_cfg

# --- defaults (the platform's lifecycle) -------------------------------------

# The legal [stage:] header values.
declare -p LIFECYCLE_STAGES &>/dev/null || LIFECYCLE_STAGES=(scope align build validate close)

# Mandatory predecessor per stage (a stage absent from the map has none).
# Trigger-gated stages (align) are deliberately nobody's predecessor: build's
# prior is scope whether or not align ran.
if ! declare -p LIFECYCLE_PREDECESSOR &>/dev/null; then
    declare -A LIFECYCLE_PREDECESSOR=([align]=scope [build]=scope [validate]=build [close]=validate)
fi

# The iteration-boundary stage: the only stage at which the unnamed-iteration
# sentinel '—' is legal, in the header and in a stamp.
[[ -v LIFECYCLE_FIRST_STAGE ]] || LIFECYCLE_FIRST_STAGE=scope

# Entering this stage requires the active queue drained (empty disables).
[[ -v LIFECYCLE_DRAIN_STAGE ]] || LIFECYCLE_DRAIN_STAGE=validate

# The queue sections whose top-level '- ' entries constitute the active queue
# (plain text, matched literally inside a '^## <name>$' heading regex).
declare -p LIFECYCLE_ACTIVE_SECTIONS &>/dev/null || LIFECYCLE_ACTIVE_SECTIONS=("New Features" "Technical Debt")

# The trigger-gated audit stage and the stage whose entry demands it when the
# cross-component amendment signal fires (empty LIFECYCLE_AUDIT_STAGE disables
# the signal check entirely, entry-stage and waiver knobs included).
[[ -v LIFECYCLE_AUDIT_STAGE ]] || LIFECYCLE_AUDIT_STAGE=align
[[ -v LIFECYCLE_AUDIT_ENTRY_STAGE ]] || LIFECYCLE_AUDIT_ENTRY_STAGE="${LIFECYCLE_AUDIT_STAGE:+build}"

# The recorded-waiver stamp token: a legal stamp line, never a header stage.
[[ -v LIFECYCLE_WAIVER_TOKEN ]] || LIFECYCLE_WAIVER_TOKEN="${LIFECYCLE_AUDIT_STAGE:+${LIFECYCLE_AUDIT_STAGE}-waived}"

# Amendment-file shape, component-roster marker, and the path tokens whose
# mention in an amendment body counts as naming a component's contract surface.
[[ -v LIFECYCLE_AMENDMENT_GLOB ]] || LIFECYCLE_AMENDMENT_GLOB='SPEC-*.md'
[[ -v LIFECYCLE_ROSTER_BASENAME ]] || LIFECYCLE_ROSTER_BASENAME='SPEC.md'
declare -p LIFECYCLE_CONTRACT_TOKENS &>/dev/null || LIFECYCLE_CONTRACT_TOKENS=("SPEC.md" "proto/")

# Governed files (repo-root-relative; gates also take them as $1/$2).
[[ -v LIFECYCLE_QUEUE_FILE ]] || LIFECYCLE_QUEUE_FILE="${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}"
[[ -v LIFECYCLE_STATE_FILE ]] || LIFECYCLE_STATE_FILE="${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt"

# --- shared header adapters (both gates must parse identically) --------------

# lifecycle_header <queue-file> — print the first '## Iteration:' line, if any.
lifecycle_header() {
    grep -m1 '^## Iteration:' "$1" 2>/dev/null || true
}

# lifecycle_header_iter / lifecycle_header_stage <header-line>
lifecycle_header_iter() {
    sed -E 's/^## Iteration:[[:space:]]*//; s/[[:space:]]*\[stage:.*$//' <<<"$1"
}
lifecycle_header_stage() {
    sed -E 's/.*\[stage:[[:space:]]*//; s/[[:space:]]*\].*$//' <<<"$1"
}

# lifecycle_stage_known <token> — 0 iff the token is a configured stage.
lifecycle_stage_known() {
    local s
    for s in "${LIFECYCLE_STAGES[@]}"; do
        [[ "$1" == "$s" ]] && return 0
    done
    return 1
}

# --- config validation (a malformed machine must not gate anything) ----------

_lc_errs=()
[[ ${#LIFECYCLE_STAGES[@]} -gt 0 ]] || _lc_errs+=("LIFECYCLE_STAGES is empty")
lifecycle_stage_known "$LIFECYCLE_FIRST_STAGE" \
    || _lc_errs+=("LIFECYCLE_FIRST_STAGE '$LIFECYCLE_FIRST_STAGE' is not in LIFECYCLE_STAGES")
for _lc_k in "${!LIFECYCLE_PREDECESSOR[@]}"; do
    lifecycle_stage_known "$_lc_k" \
        || _lc_errs+=("LIFECYCLE_PREDECESSOR key '$_lc_k' is not in LIFECYCLE_STAGES")
    lifecycle_stage_known "${LIFECYCLE_PREDECESSOR[$_lc_k]}" \
        || _lc_errs+=("LIFECYCLE_PREDECESSOR[$_lc_k]='${LIFECYCLE_PREDECESSOR[$_lc_k]}' is not in LIFECYCLE_STAGES")
done
[[ -z "$LIFECYCLE_DRAIN_STAGE" ]] || lifecycle_stage_known "$LIFECYCLE_DRAIN_STAGE" \
    || _lc_errs+=("LIFECYCLE_DRAIN_STAGE '$LIFECYCLE_DRAIN_STAGE' is not in LIFECYCLE_STAGES")
[[ -z "$LIFECYCLE_AUDIT_STAGE" ]] || lifecycle_stage_known "$LIFECYCLE_AUDIT_STAGE" \
    || _lc_errs+=("LIFECYCLE_AUDIT_STAGE '$LIFECYCLE_AUDIT_STAGE' is not in LIFECYCLE_STAGES")
[[ -z "$LIFECYCLE_AUDIT_ENTRY_STAGE" ]] || lifecycle_stage_known "$LIFECYCLE_AUDIT_ENTRY_STAGE" \
    || _lc_errs+=("LIFECYCLE_AUDIT_ENTRY_STAGE '$LIFECYCLE_AUDIT_ENTRY_STAGE' is not in LIFECYCLE_STAGES")
if [[ -n "$LIFECYCLE_WAIVER_TOKEN" ]] && lifecycle_stage_known "$LIFECYCLE_WAIVER_TOKEN"; then
    _lc_errs+=("LIFECYCLE_WAIVER_TOKEN '$LIFECYCLE_WAIVER_TOKEN' collides with a stage name")
fi
if [[ ${#_lc_errs[@]} -gt 0 ]]; then
    printf 'lifecycle-kit: malformed stage-machine config — the gates cannot run:\n' >&2
    printf '  %s\n' "${_lc_errs[@]}" >&2
    exit 2
fi
unset _lc_errs _lc_k
