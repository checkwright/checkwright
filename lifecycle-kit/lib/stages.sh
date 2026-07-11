# shellcheck shell=bash
# spec: lifecycle-kit/SPEC.md §lib/stages.sh — the stage machine as config: platform defaults, consumer overrides
_lc_cfg="${LIFECYCLE_KIT_CONFIG_FILE:-}"
if [[ -n "$_lc_cfg" ]]; then
    [[ -f "$_lc_cfg" ]] || {
        echo "lifecycle-kit: LIFECYCLE_KIT_CONFIG_FILE not found: $_lc_cfg" >&2
        exit 2
    }
    # shellcheck disable=SC1090  # consumer-supplied config, path is config
    source "$_lc_cfg"
else
    _lc_cfg="${GATE_SDK_GATES_DIR:-scripts}/lifecycle-config.sh"
    if [[ -f "$_lc_cfg" ]]; then
        # shellcheck disable=SC1090  # consumer-supplied config, path is config
        source "$_lc_cfg"
    fi
fi
unset _lc_cfg

declare -p LIFECYCLE_KIT_STAGES &>/dev/null || LIFECYCLE_KIT_STAGES=(scope align build validate close)

if ! declare -p LIFECYCLE_KIT_PREDECESSOR &>/dev/null; then
    declare -A LIFECYCLE_KIT_PREDECESSOR=([align]=scope [build]=scope [validate]=build [close]=validate)
fi

[[ -v LIFECYCLE_KIT_FIRST_STAGE ]] || LIFECYCLE_KIT_FIRST_STAGE=scope

[[ -v LIFECYCLE_KIT_DRAIN_STAGE ]] || LIFECYCLE_KIT_DRAIN_STAGE=validate

declare -p LIFECYCLE_KIT_ACTIVE_SECTIONS &>/dev/null || LIFECYCLE_KIT_ACTIVE_SECTIONS=("New Features" "Technical Debt")

[[ -v LIFECYCLE_KIT_AUDIT_STAGE ]] || LIFECYCLE_KIT_AUDIT_STAGE=align
[[ -v LIFECYCLE_KIT_AUDIT_ENTRY_STAGE ]] || LIFECYCLE_KIT_AUDIT_ENTRY_STAGE="${LIFECYCLE_KIT_AUDIT_STAGE:+build}"

[[ -v LIFECYCLE_KIT_WAIVER_TOKEN ]] || LIFECYCLE_KIT_WAIVER_TOKEN="${LIFECYCLE_KIT_AUDIT_STAGE:+${LIFECYCLE_KIT_AUDIT_STAGE}-waived}"

[[ -v LIFECYCLE_KIT_AMENDMENT_GLOB ]] || LIFECYCLE_KIT_AMENDMENT_GLOB='SPEC-*.md'
[[ -v LIFECYCLE_KIT_ROSTER_BASENAME ]] || LIFECYCLE_KIT_ROSTER_BASENAME='SPEC.md'
declare -p LIFECYCLE_KIT_CONTRACT_TOKENS &>/dev/null || LIFECYCLE_KIT_CONTRACT_TOKENS=("SPEC.md" "proto/")

[[ -v LIFECYCLE_KIT_SKILLS_DIR ]] || LIFECYCLE_KIT_SKILLS_DIR=".claude/commands"

[[ -v LIFECYCLE_KIT_SHIM_NGRAM ]] || LIFECYCLE_KIT_SHIM_NGRAM=9
declare -p LIFECYCLE_KIT_SHIM_DEDUP_CORPUS &>/dev/null || LIFECYCLE_KIT_SHIM_DEDUP_CORPUS=()

[[ -v LIFECYCLE_KIT_QUEUE_FILE ]] || LIFECYCLE_KIT_QUEUE_FILE="${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}"
[[ -v LIFECYCLE_KIT_STATE_FILE ]] || LIFECYCLE_KIT_STATE_FILE="${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt"
[[ -v LIFECYCLE_KIT_LESSON_EVIDENCE_FILE ]] || LIFECYCLE_KIT_LESSON_EVIDENCE_FILE="${GATE_SDK_WORKFLOW_DIR:-.workflow}/lesson-evidence.txt"

declare -p LIFECYCLE_KIT_BOUNDARY_TRUNCATE &>/dev/null || LIFECYCLE_KIT_BOUNDARY_TRUNCATE=()

declare -p LIFECYCLE_KIT_ENTRY_PREFLIGHT &>/dev/null || LIFECYCLE_KIT_ENTRY_PREFLIGHT=()

lifecycle_header() {
    grep -m1 '^## Iteration:' "$1" 2>/dev/null || true
}

lifecycle_header_iter() {
    sed -E 's/^## Iteration:[[:space:]]*//; s/[[:space:]]*\[stage:.*$//' <<<"$1"
}
lifecycle_header_stage() {
    sed -E 's/.*\[stage:[[:space:]]*//; s/[[:space:]]*\].*$//' <<<"$1"
}

lifecycle_stage_known() {
    local s
    for s in "${LIFECYCLE_KIT_STAGES[@]}"; do
        [[ "$1" == "$s" ]] && return 0
    done
    return 1
}

_lc_errs=()
[[ ${#LIFECYCLE_KIT_STAGES[@]} -gt 0 ]] || _lc_errs+=("LIFECYCLE_KIT_STAGES is empty")
[[ -n "$LIFECYCLE_KIT_SKILLS_DIR" ]] || _lc_errs+=("LIFECYCLE_KIT_SKILLS_DIR is empty")
[[ -n "$LIFECYCLE_KIT_LESSON_EVIDENCE_FILE" ]] || _lc_errs+=("LIFECYCLE_KIT_LESSON_EVIDENCE_FILE is empty")
[[ "$LIFECYCLE_KIT_SHIM_NGRAM" =~ ^[1-9][0-9]*$ ]] \
    || _lc_errs+=("LIFECYCLE_KIT_SHIM_NGRAM '$LIFECYCLE_KIT_SHIM_NGRAM' is not a positive integer")
lifecycle_stage_known "$LIFECYCLE_KIT_FIRST_STAGE" \
    || _lc_errs+=("LIFECYCLE_KIT_FIRST_STAGE '$LIFECYCLE_KIT_FIRST_STAGE' is not in LIFECYCLE_KIT_STAGES")
for _lc_k in "${!LIFECYCLE_KIT_PREDECESSOR[@]}"; do
    lifecycle_stage_known "$_lc_k" \
        || _lc_errs+=("LIFECYCLE_KIT_PREDECESSOR key '$_lc_k' is not in LIFECYCLE_KIT_STAGES")
    lifecycle_stage_known "${LIFECYCLE_KIT_PREDECESSOR[$_lc_k]}" \
        || _lc_errs+=("LIFECYCLE_KIT_PREDECESSOR[$_lc_k]='${LIFECYCLE_KIT_PREDECESSOR[$_lc_k]}' is not in LIFECYCLE_KIT_STAGES")
done
[[ -z "$LIFECYCLE_KIT_DRAIN_STAGE" ]] || lifecycle_stage_known "$LIFECYCLE_KIT_DRAIN_STAGE" \
    || _lc_errs+=("LIFECYCLE_KIT_DRAIN_STAGE '$LIFECYCLE_KIT_DRAIN_STAGE' is not in LIFECYCLE_KIT_STAGES")
[[ -z "$LIFECYCLE_KIT_AUDIT_STAGE" ]] || lifecycle_stage_known "$LIFECYCLE_KIT_AUDIT_STAGE" \
    || _lc_errs+=("LIFECYCLE_KIT_AUDIT_STAGE '$LIFECYCLE_KIT_AUDIT_STAGE' is not in LIFECYCLE_KIT_STAGES")
[[ -z "$LIFECYCLE_KIT_AUDIT_ENTRY_STAGE" ]] || lifecycle_stage_known "$LIFECYCLE_KIT_AUDIT_ENTRY_STAGE" \
    || _lc_errs+=("LIFECYCLE_KIT_AUDIT_ENTRY_STAGE '$LIFECYCLE_KIT_AUDIT_ENTRY_STAGE' is not in LIFECYCLE_KIT_STAGES")
if [[ -n "$LIFECYCLE_KIT_WAIVER_TOKEN" ]] && lifecycle_stage_known "$LIFECYCLE_KIT_WAIVER_TOKEN"; then
    _lc_errs+=("LIFECYCLE_KIT_WAIVER_TOKEN '$LIFECYCLE_KIT_WAIVER_TOKEN' collides with a stage name")
fi
for _lc_pf in ${LIFECYCLE_KIT_ENTRY_PREFLIGHT[@]+"${LIFECYCLE_KIT_ENTRY_PREFLIGHT[@]}"}; do
    if [[ "$_lc_pf" != *=* ]]; then
        _lc_errs+=("LIFECYCLE_KIT_ENTRY_PREFLIGHT entry '$_lc_pf' lacks the '<stage>=<command>' shape")
    elif ! lifecycle_stage_known "${_lc_pf%%=*}"; then
        _lc_errs+=("LIFECYCLE_KIT_ENTRY_PREFLIGHT stage key '${_lc_pf%%=*}' is not in LIFECYCLE_KIT_STAGES")
    fi
done
if [[ ${#_lc_errs[@]} -gt 0 ]]; then
    printf 'lifecycle-kit: malformed stage-machine config — the gates cannot run:\n' >&2
    printf '  %s\n' "${_lc_errs[@]}" >&2
    exit 2
fi
unset _lc_errs _lc_k _lc_pf
