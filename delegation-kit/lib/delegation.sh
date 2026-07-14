# shellcheck shell=bash
# spec: delegation-kit/SPEC.md §Layout and configuration — sourced config loader for usage-verdict + check-gate-tamper, values only
_dk_cfg="${DELEGATION_KIT_CONFIG_FILE:-}"
if [[ -n "$_dk_cfg" ]]; then
    [[ -f "$_dk_cfg" ]] || {
        echo "delegation-kit: DELEGATION_KIT_CONFIG_FILE not found: $_dk_cfg" >&2
        exit 2
    }
    # shellcheck disable=SC1090  # consumer-supplied config, path is config
    source "$_dk_cfg"
else
    _dk_cfg="${GATE_SDK_GATES_DIR:-scripts}/delegation-config.sh"
    if [[ -f "$_dk_cfg" ]]; then
        # shellcheck disable=SC1090  # consumer-supplied config, path is config
        source "$_dk_cfg"
    fi
fi
unset _dk_cfg

[[ -v DELEGATION_KIT_USAGE_FILE ]] || DELEGATION_KIT_USAGE_FILE="${CLAUDE_CONFIG_DIR:-$HOME/.claude}/usage.txt"
[[ -v DELEGATION_KIT_CRED_FILE ]] || DELEGATION_KIT_CRED_FILE="${DELEGATION_KIT_USAGE_FILE%/*}/.credentials.json"
[[ -v DELEGATION_KIT_PAUSE_PCT ]] || DELEGATION_KIT_PAUSE_PCT=80
[[ -v DELEGATION_KIT_PAUSE_PCT_7D ]] || DELEGATION_KIT_PAUSE_PCT_7D=95
[[ -v DELEGATION_KIT_STALE_AGE ]] || DELEGATION_KIT_STALE_AGE=600
[[ -v DELEGATION_KIT_LOGIN_WINDOW ]] || DELEGATION_KIT_LOGIN_WINDOW=600
[[ -v DELEGATION_KIT_USAGE_HISTORY ]] || DELEGATION_KIT_USAGE_HISTORY=""
[[ -v DELEGATION_KIT_FAN_WIDTH ]] || DELEGATION_KIT_FAN_WIDTH=2

declare -p DELEGATION_KIT_GATE_FILES &>/dev/null || DELEGATION_KIT_GATE_FILES=(
    "${GATE_SDK_GATES_DIR:-scripts}/check-*.sh"
    "${GATE_SDK_GATES_DIR:-scripts}/lib/gate.sh"
    "${GATE_SDK_GATES_DIR:-scripts}/run-gate-tests.sh"
)
declare -p DELEGATION_KIT_META_PATHS &>/dev/null || DELEGATION_KIT_META_PATHS=(
    "${GATE_SDK_GATES_DIR:-scripts}/"
    "${GATE_SDK_WORKFLOW_DIR:-.workflow}/"
    ".claude/"
)

# spec: delegation-kit/SPEC.md §Layout and configuration — a vendored kit's edits are meta-layer by definition; when gate.sh resolves, union every kit root into META_PATHS (additive, never a filter, so a declared prefix cannot be lost)
_dk_gate_lib="${GATE_SDK_LIB:-${BASH_SOURCE[0]%/*}/../../gate-sdk/lib/gate.sh}"
if [[ -f "$_dk_gate_lib" ]]; then
    # shellcheck source=../../gate-sdk/lib/gate.sh
    source "$_dk_gate_lib"
    if declare -F gate_kit_roots_rel >/dev/null; then
        while IFS= read -r _dk_root; do
            _dk_root="${_dk_root%/}/"
            _dk_seen=0
            for _dk_p in "${DELEGATION_KIT_META_PATHS[@]}"; do
                [[ "$_dk_p" == "$_dk_root" ]] && { _dk_seen=1; break; }
            done
            [[ "$_dk_seen" -eq 0 ]] && DELEGATION_KIT_META_PATHS+=("$_dk_root")
        done < <(gate_kit_roots_rel)
    fi
fi
unset _dk_gate_lib _dk_root _dk_seen _dk_p

_dk_errs=()
[[ -n "$DELEGATION_KIT_USAGE_FILE" ]] || _dk_errs+=("DELEGATION_KIT_USAGE_FILE is empty")
[[ "$DELEGATION_KIT_PAUSE_PCT" =~ ^[0-9]+(\.[0-9]+)?$ ]] \
    || _dk_errs+=("DELEGATION_KIT_PAUSE_PCT must be numeric (got '$DELEGATION_KIT_PAUSE_PCT')")
[[ "$DELEGATION_KIT_PAUSE_PCT_7D" =~ ^[0-9]+(\.[0-9]+)?$ ]] \
    || _dk_errs+=("DELEGATION_KIT_PAUSE_PCT_7D must be numeric (got '$DELEGATION_KIT_PAUSE_PCT_7D')")
[[ "$DELEGATION_KIT_STALE_AGE" =~ ^[0-9]+$ ]] \
    || _dk_errs+=("DELEGATION_KIT_STALE_AGE must be a non-negative integer (got '$DELEGATION_KIT_STALE_AGE')")
[[ "$DELEGATION_KIT_LOGIN_WINDOW" =~ ^[0-9]+$ ]] \
    || _dk_errs+=("DELEGATION_KIT_LOGIN_WINDOW must be a non-negative integer (got '$DELEGATION_KIT_LOGIN_WINDOW')")
[[ "$DELEGATION_KIT_FAN_WIDTH" =~ ^[0-9]+$ && "$DELEGATION_KIT_FAN_WIDTH" -gt 0 ]] \
    || _dk_errs+=("DELEGATION_KIT_FAN_WIDTH must be a positive integer (got '$DELEGATION_KIT_FAN_WIDTH')")
[[ ${#DELEGATION_KIT_GATE_FILES[@]} -gt 0 ]] || _dk_errs+=("DELEGATION_KIT_GATE_FILES is empty")
[[ ${#DELEGATION_KIT_META_PATHS[@]} -gt 0 ]] || _dk_errs+=("DELEGATION_KIT_META_PATHS is empty")
if [[ ${#_dk_errs[@]} -gt 0 ]]; then
    printf 'delegation-kit: malformed delegation config — the tools cannot run:\n' >&2
    printf '  %s\n' "${_dk_errs[@]}" >&2
    exit 2
fi
unset _dk_errs
