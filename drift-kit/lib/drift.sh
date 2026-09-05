# shellcheck shell=bash
# no-port: drift-kit/SPEC.md §lib/drift.sh — the config bridge's **sole resolver** for the DRIFT_KIT_* knobs. Resolving a declared knob means sourcing the owning kit's lib/*.sh (gate-sdk/SPEC.md §lib/gate.sh), and three already-compiled arms plus the collator resolve through this one, so a crate-side resolver would be the second producer criterion 6 refuses. Structural, not a sizing judgment. The ground is stated rather than cited because the class of kit lib/*.sh files has never been swept, and a cohort inherits a stated reason where it cannot inherit a precedent-by-example.
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

# spec: drift-kit/SPEC.md §The report skeleton — the report's own knobs resolve *here* because the config bridge sources this library alone: a default left in the collator would resolve to nothing once the collator is a compiled arm, and a default duplicated on the crate side would be the second producer the port-candidate criteria refuse.
: "${DRIFT_KIT_QUEUE_FILE:=${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}}"
: "${DRIFT_KIT_KNOWLEDGE_LOG:=${GATE_SDK_WORKFLOW_DIR:-.workflow}/knowledge-friction.log}"
: "${DRIFT_KIT_TIMINGS_FILE:=${GATE_SDK_TMP_DIR:-.tmp}/gate-timings.txt}"
: "${DRIFT_KIT_TMP_DIR:=${GATE_SDK_TMP_DIR:-.tmp}}"
: "${DRIFT_KIT_METRIC_DIR:=.metric}"
: "${DRIFT_KIT_DONE_SECTION:=Done}"
: "${DRIFT_KIT_DEFERRED_SECTION:=Deferred}"
: "${DRIFT_KIT_ICEBOX_SECTION:=}"
# spec: drift-kit/SPEC.md §Layout and configuration — DRIFT_KIT_SESSIONS_DIR is declared **empty** here so the config bridge's `declare -p` can find it: the meter that reads it is a compiled arm, and a knob no kit library defines is the bridge's undeclared-knob refusal on every invocation, which would break the bare no-override run that is the meter's primary use. It computes nothing itself — the `<config-home>/projects/<cwd-slug>` fallback lives in the crate's shared session derivation, and a second copy in shell would be exactly the divergence the meters' port removed. Empty means "derive it", never "no value" (the GATE_SDK_UPGRADE_FROM shape).
: "${DRIFT_KIT_SESSIONS_DIR:=}"
: "${DRIFT_KIT_OVERHEAD_LOG:=$DRIFT_KIT_METRIC_DIR/overhead-log.txt}"

# spec: drift-kit/SPEC.md §Layout and configuration — the stage-economics meter's four remaining knobs, resolved here rather than inline in the tool that used to hold them: that tool is a compiled arm now, and the config bridge finds a default only through `declare -p`, so a value defaulted at a use site is its undeclared-knob refusal on every invocation. Neither the names nor the values are new — each already governed the shell tool's behavior; what moves is the default's resolution site, which is the same relocation §Layout records for DRIFT_KIT_PRICE_TABLE.
: "${DRIFT_KIT_STAGE_ECONOMICS_LOG:=$DRIFT_KIT_METRIC_DIR/stage-economics-log.txt}"
: "${DRIFT_KIT_STATE_FILE:=${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt}"
: "${DRIFT_KIT_SUPERVISION_LABEL:=supervision}"
: "${DRIFT_KIT_FANOUT_SUFFIX:=+fanout}"

# spec: drift-kit/SPEC.md §Bundled KPIs — kpi-price-table-age and bin/stage-economics.sh read one table; resolving the default here makes this library its single producer, which is what the substrate move converts the former in-substrate restatement into rather than a cross-substrate one.
: "${DRIFT_KIT_PRICE_TABLE:=${GATE_SDK_GATES_DIR:-scripts}/price-table.tsv}"

# spec: drift-kit/SPEC.md §The KPI plugin contract — DRIFT_KIT_KPI_DIRS is the extension point's first resolution tier and is consumer-first by construction: the adopter's own gates dir, before any kit's members.
declare -p DRIFT_KIT_KPI_DIRS >/dev/null 2>&1 || DRIFT_KIT_KPI_DIRS=("${GATE_SDK_GATES_DIR:-scripts}")
