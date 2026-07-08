# shellcheck shell=bash
# spec: evidence-kit/SPEC.md §lib/evidence.sh — sourced config loader, parser adapters, and baseline/manifest read helpers; values + adapters, never tool structure

# spec: evidence-kit/SPEC.md §lib/evidence.sh — requires gate-sdk vendored beside it, sourced for fail_closed + the prune adapters
_ek_gate_lib="${GATE_SDK_LIB:-${BASH_SOURCE[0]%/*}/../../gate-sdk/lib/gate.sh}"
if [[ -f "$_ek_gate_lib" ]]; then
    # shellcheck source=../../gate-sdk/lib/gate.sh
    source "$_ek_gate_lib"
fi
unset _ek_gate_lib

# spec: evidence-kit/SPEC.md §Evidence manifest — the versioned wire-format token the manifest header carries and check-evidence-manifest asserts; a lib constant, not consumer config
# shellcheck disable=SC2034  # consumed by check-evidence-manifest after sourcing
EVIDENCE_MANIFEST_CONTRACT="evidence-manifest v1"

_ek_cfg="${EVIDENCE_KIT_CONFIG_FILE:-}"
if [[ -n "$_ek_cfg" ]]; then
    [[ -f "$_ek_cfg" ]] || {
        echo "evidence-kit: EVIDENCE_KIT_CONFIG_FILE not found: $_ek_cfg" >&2
        exit 2
    }
    # shellcheck disable=SC1090  # consumer-supplied config, path is config
    source "$_ek_cfg"
else
    _ek_cfg="${GATE_SDK_GATES_DIR:-scripts}/evidence-config.sh"
    if [[ -f "$_ek_cfg" ]]; then
        # shellcheck disable=SC1090  # consumer-supplied config, path is config
        source "$_ek_cfg"
    fi
fi
unset _ek_cfg

declare -p EVIDENCE_KIT_SUITES &>/dev/null || EVIDENCE_KIT_SUITES=()
[[ -v EVIDENCE_KIT_PARSER ]]        || EVIDENCE_KIT_PARSER="exit-code"
[[ -v EVIDENCE_KIT_BASELINE_FILE ]] || EVIDENCE_KIT_BASELINE_FILE="${GATE_SDK_WORKFLOW_DIR:-.workflow}/validate-baseline.txt"
[[ -v EVIDENCE_KIT_MANIFEST_FILE ]] || EVIDENCE_KIT_MANIFEST_FILE="${GATE_SDK_WORKFLOW_DIR:-.workflow}/validate-evidence.txt"
[[ -v EVIDENCE_KIT_SKIP_FILE ]]     || EVIDENCE_KIT_SKIP_FILE="${GATE_SDK_WORKFLOW_DIR:-.workflow}/validate-skips.txt"
[[ -v EVIDENCE_KIT_QUEUE_FILE ]]    || EVIDENCE_KIT_QUEUE_FILE="${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}"
[[ -v EVIDENCE_KIT_STATE_FILE ]]    || EVIDENCE_KIT_STATE_FILE="${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt"
[[ -v EVIDENCE_KIT_TMP_DIR ]]       || EVIDENCE_KIT_TMP_DIR="${GATE_SDK_TMP_DIR:-.tmp}"
[[ -v EVIDENCE_KIT_RUN_ID ]]        || EVIDENCE_KIT_RUN_ID=""
[[ -v EVIDENCE_KIT_PRE_HOOK ]]      || EVIDENCE_KIT_PRE_HOOK=""
declare -p EVIDENCE_KIT_SCENARIO_GLOBS &>/dev/null || declare -A EVIDENCE_KIT_SCENARIO_GLOBS=()
declare -p EVIDENCE_KIT_PERMANENT_SLUGS &>/dev/null || EVIDENCE_KIT_PERMANENT_SLUGS=()

ek_suite_cmd() {
    local var="EVIDENCE_KIT_RUN_$1"
    printf '%s\n' "${!var-}"
}

# spec: evidence-kit/SPEC.md §lib/evidence.sh — the queue header's iteration is the evidence-line key when lifecycle drives the tree; a self-contained reader so the kit needs no lifecycle-kit dependency
ek_queue_iteration() {
    local q="${1:-$EVIDENCE_KIT_QUEUE_FILE}"
    [[ -f "$q" ]] || return 1
    local hdr
    hdr="$(grep -m1 '^## Iteration:' "$q" 2>/dev/null)" || return 1
    [[ -n "$hdr" ]] || return 1
    sed -E 's/^## Iteration:[[:space:]]*//; s/[[:space:]]*\[stage:.*$//' <<<"$hdr"
}

ek_queue_stage() {
    local q="${1:-$EVIDENCE_KIT_QUEUE_FILE}"
    [[ -f "$q" ]] || return 1
    local hdr
    hdr="$(grep -m1 '^## Iteration:' "$q" 2>/dev/null)" || return 1
    [[ "$hdr" == *'[stage:'* ]] || return 1
    sed -E 's/.*\[stage:[[:space:]]*//; s/[[:space:]]*\].*$//' <<<"$hdr"
}

ek_run_key() {
    local iter
    if iter="$(ek_queue_iteration)" && [[ -n "$iter" && "$iter" != "—" ]]; then
        printf '%s\n' "$iter"
        return 0
    fi
    [[ -n "$EVIDENCE_KIT_RUN_ID" ]] || return 1
    printf '%s\n' "$EVIDENCE_KIT_RUN_ID"
}

# spec: evidence-kit/SPEC.md §lib/evidence.sh — parser adapters map a captured log (+ the suite's exit status) to '<scenario> <pass|fail|ignore>' lines; two ship built-in, any other value is a consumer command run on the log
ek_parse() {
    local parser="$1" suite="$2" log="$3" status="$4"
    case "$parser" in
        exit-code)
            if [[ "$status" -eq 0 ]]; then printf '%s pass\n' "$suite"; else printf '%s fail\n' "$suite"; fi
            ;;
        libtest)
            awk '
                /^test .* \.\.\. / {
                    name = $2; res = $NF
                    if (res == "ok")            print name, "pass"
                    else if (res == "FAILED")   print name, "fail"
                    else if (res == "ignored")  print name, "ignore"
                }
            ' "$log"
            ;;
        *)
            # shellcheck disable=SC2086  # a multi-word consumer parser command word-splits by design
            $parser "$log"
            ;;
    esac
}

ek_data_lines() {
    grep -Ev '^[[:space:]]*(#|$)' "$1" 2>/dev/null || true
}

# spec: evidence-kit/SPEC.md §bin/diff-baseline.sh — the per-scenario diff shared by run-validate (verdict) and diff-baseline (findings): a baseline 'pass' scenario red-or-absent is a new-failure; a baseline 'fail'/'ignore' scenario observed green is an unpromoted recovery; a self-skipped scenario is demoted from pass first. Prints 'new-failure <suite> <scenario>' / 'recovery <suite> <scenario>' lines; returns 1 iff a new-failure fired.
ek_diff() {
    local baseline="$1" suite="$2" observed="$3" skipfile="${4:-}"
    declare -A obs=() skip=()
    local sc st f1 f2 bsuite bscen bstat cur
    while read -r sc st _; do
        [[ -n "$sc" ]] && obs["$sc"]="$st"
    done < "$observed"
    if [[ -n "$skipfile" && -f "$skipfile" ]]; then
        while read -r f1 f2 _; do
            [[ "$f1" == "$suite" && -n "$f2" ]] && skip["$f2"]=1
        done < "$skipfile"
    fi
    local rc=0
    while read -r bsuite bscen bstat _; do
        [[ "$bsuite" == "$suite" ]] || continue
        cur="${obs[$bscen]:-absent}"
        [[ -n "${skip[$bscen]:-}" && "$cur" == "pass" ]] && cur="skip"
        if [[ "$bstat" == "pass" ]]; then
            if [[ "$cur" != "pass" ]]; then
                printf 'new-failure %s %s\n' "$suite" "$bscen"
                rc=1
            fi
        else
            [[ "$cur" == "pass" ]] && printf 'recovery %s %s\n' "$suite" "$bscen"
        fi
    done < <(ek_data_lines "$baseline")
    return "$rc"
}

_ek_errs=()
[[ -n "$EVIDENCE_KIT_PARSER" ]] || _ek_errs+=("EVIDENCE_KIT_PARSER is empty")
[[ -n "$EVIDENCE_KIT_BASELINE_FILE" ]] || _ek_errs+=("EVIDENCE_KIT_BASELINE_FILE is empty")
[[ -n "$EVIDENCE_KIT_MANIFEST_FILE" ]] || _ek_errs+=("EVIDENCE_KIT_MANIFEST_FILE is empty")
[[ -n "$EVIDENCE_KIT_QUEUE_FILE" ]] || _ek_errs+=("EVIDENCE_KIT_QUEUE_FILE is empty")
for _ek_s in ${EVIDENCE_KIT_SUITES[@]+"${EVIDENCE_KIT_SUITES[@]}"}; do
    [[ "$_ek_s" =~ ^[A-Za-z_][A-Za-z0-9_]*$ ]] \
        || _ek_errs+=("suite name '$_ek_s' is not a valid EVIDENCE_KIT_RUN_<suite> variable suffix")
done
if [[ ${#_ek_errs[@]} -gt 0 ]]; then
    printf 'evidence-kit: malformed evidence config — the tools cannot run:\n' >&2
    printf '  %s\n' "${_ek_errs[@]}" >&2
    exit 2
fi
unset _ek_errs _ek_s
