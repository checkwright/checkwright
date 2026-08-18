# shellcheck shell=bash
# spec: drift-kit/SPEC.md §lib/drift.sh — sourced resolution for drift-kit's registry knobs, never tool structure; the config bridge sources this to resolve a compiled member's declared knobs

# spec: drift-kit/SPEC.md §lib/drift.sh — the consumer config is sourced *here* now that the trajectory arm reads its knobs across the config bridge: the bridge resolves a declared name by sourcing this library alone, so a knob a consumer config sets would otherwise resolve to the platform default and silently ignore the override. The refusal mode is DRIFT_KIT_KPIS_FILE's, for the same reason: an explicitly-set path that does not exist is adopted-but-broken.
_ds_cfg="${DRIFT_KIT_CONFIG_FILE:-}"
if [[ -n "$_ds_cfg" ]]; then
    [[ -f "$_ds_cfg" ]] || {
        echo "drift-kit: DRIFT_KIT_CONFIG_FILE not found: $_ds_cfg" >&2
        exit 2
    }
    # shellcheck source=/dev/null  # consumer config path is resolved at runtime
    source "$_ds_cfg"
else
    _ds_cfg="${GATE_SDK_GATES_DIR:-scripts}/drift-config.sh"
    if [[ -f "$_ds_cfg" ]]; then
        # shellcheck source=/dev/null  # consumer config path is resolved at runtime
        source "$_ds_cfg"
    else
        _ds_cfg=""
    fi
fi
DRIFT_KIT_CONFIG_FILE="$_ds_cfg"
unset _ds_cfg

# spec: drift-kit/SPEC.md §lib/drift.sh — DRIFT_KIT_KPIS_FILE resolves to the KPI registry path, and the two adoption modes are preserved *here* rather than at the reader, because a guarded default would collapse them: an explicitly-set path that does not exist is adopted-but-broken and refuses (exit 2 naming the knob, which under the config bridge refuses the whole invocation); an unset knob whose default path is absent is not-adopted and resolves to the **empty string**, which a reader takes as "no registry, drop the section". Emptiness is the not-adopted signal, so no reader needs to know the default and none carries one.
if [[ -n "${DRIFT_KIT_KPIS_FILE:-}" ]]; then
    [[ -f "$DRIFT_KIT_KPIS_FILE" ]] || {
        echo "drift-kit: DRIFT_KIT_KPIS_FILE not found: $DRIFT_KIT_KPIS_FILE" >&2
        exit 2
    }
else
    DRIFT_KIT_KPIS_FILE="${GATE_SDK_GATES_DIR:-scripts}/kpis.list"
    [[ -f "$DRIFT_KIT_KPIS_FILE" ]] || DRIFT_KIT_KPIS_FILE=""
fi

# spec: drift-kit/SPEC.md §Layout and configuration — DRIFT_KIT_TRAJECTORY_SURFACES is "<state-file> <evidence-file>", one scalar whose two whitespace-separated fields the reader splits; it keeps that shape across the substrate move so a consumer setting it does not have to learn a second one.
_ds_wf="${GATE_SDK_WORKFLOW_DIR:-.workflow}"
: "${DRIFT_KIT_TRAJECTORY_SURFACES:=$_ds_wf/WORKFLOW-STATE.txt $_ds_wf/validate-evidence.txt}"
unset _ds_wf
: "${DRIFT_KIT_GATES_FILE:=${GATE_SDK_GATES_DIR:-scripts}/gates.list}"

# spec: drift-kit/SPEC.md §Layout and configuration — DRIFT_KIT_STAGES is the ordered stage roster the trajectory table renders; unset falls open to the historical five so standalone/un-upgraded emission stays byte-identical (drift re-derives with its own knob, never sourcing lifecycle — the DRIFT_KIT_STATE_FILE precedent)
declare -p DRIFT_KIT_STAGES &>/dev/null || DRIFT_KIT_STAGES=(scope align build validate close)
