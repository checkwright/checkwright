# shellcheck shell=bash
# spec: drift-kit/SPEC.md §lib/drift.sh — sourced resolution for drift-kit's registry knobs, never tool structure; the config bridge sources this to resolve a compiled member's declared knobs

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
