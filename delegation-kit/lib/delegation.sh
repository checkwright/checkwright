# shellcheck shell=bash
# spec: delegation-kit/SPEC.md §Layout and configuration — sourced config loader for the usage arms + check-gate-tamper, values only
# no-port: gate-sdk/SPEC.md §The kit-library port disposition — the class ruling of 2026-08-30, reached by ground rather than by scope. This library is the config bridge's sole resolver for the DELEGATION_KIT_* knobs: gate-sdk/SPEC.md §lib/gate.sh rules exactly one place a knob's value is computed, and the bridge computes it by sourcing this file, so a crate-side resolver would be the second producer criterion 6 refuses. delegation-kit ships no library section, so delegation-kit/SPEC.md §Layout and configuration records the disposition beside the layout line that names this file. Structural, not a sizing judgment.
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
[[ -v DELEGATION_KIT_REFRESH_CMD ]] || DELEGATION_KIT_REFRESH_CMD=""
[[ -v DELEGATION_KIT_REFRESH_MIN_AGE ]] || DELEGATION_KIT_REFRESH_MIN_AGE=60
[[ -v DELEGATION_KIT_USAGE_HISTORY ]] || DELEGATION_KIT_USAGE_HISTORY=""
[[ -v DELEGATION_KIT_FAN_WIDTH ]] || DELEGATION_KIT_FAN_WIDTH=2
[[ -v DELEGATION_KIT_AGENT_DIR ]] || DELEGATION_KIT_AGENT_DIR=".claude/agents"

# spec: gate-sdk/SPEC.md §The non-gate arm — the defaults the deleted shell drivers held inline,
# moved here in the same cut: the bridge resolves a declared knob by sourcing exactly this
# library, and refuses the whole environment for one it finds undefined
[[ -v DELEGATION_KIT_ACCOUNT_CONFIG ]] || DELEGATION_KIT_ACCOUNT_CONFIG="$HOME/.claude.json"
[[ -v DELEGATION_KIT_USAGE_ENDPOINT ]] || DELEGATION_KIT_USAGE_ENDPOINT="https://api.anthropic.com/api/oauth/usage"
[[ -v DELEGATION_KIT_STOP_LOG ]] || DELEGATION_KIT_STOP_LOG="${GATE_SDK_WORKFLOW_DIR:-.workflow}/subagent-stop-liveness.log"
# spec: delegation-kit/SPEC.md §The turn-end liveness hook — no shipped default: the reader is a path the consumer names, because the gate behind it is name-addressed and this knob is not taught to resolve a name
[[ -v DELEGATION_KIT_LIVENESS_CMD ]] || DELEGATION_KIT_LIVENESS_CMD=""

# spec: delegation-kit/SPEC.md §The delegation model — D2's roster ships empty: a read-only dispatch type is a consumer's own vocabulary, so the kit declares the knob and names no member of it
declare -p DELEGATION_KIT_READONLY_TYPES &>/dev/null || DELEGATION_KIT_READONLY_TYPES=()

declare -p DELEGATION_KIT_GATE_FILES &>/dev/null || DELEGATION_KIT_GATE_FILES=(
    "${GATE_SDK_GATES_DIR:-scripts}/check-*.sh"
    "${GATE_SDK_GATES_DIR:-scripts}/check-*.gate"
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
[[ "$DELEGATION_KIT_REFRESH_MIN_AGE" =~ ^[0-9]+$ ]] \
    || _dk_errs+=("DELEGATION_KIT_REFRESH_MIN_AGE must be a non-negative integer (got '$DELEGATION_KIT_REFRESH_MIN_AGE')")
[[ "$DELEGATION_KIT_FAN_WIDTH" =~ ^[0-9]+$ && "$DELEGATION_KIT_FAN_WIDTH" -gt 0 ]] \
    || _dk_errs+=("DELEGATION_KIT_FAN_WIDTH must be a positive integer (got '$DELEGATION_KIT_FAN_WIDTH')")
[[ -n "$DELEGATION_KIT_AGENT_DIR" ]] || _dk_errs+=("DELEGATION_KIT_AGENT_DIR is empty")
[[ ${#DELEGATION_KIT_GATE_FILES[@]} -gt 0 ]] || _dk_errs+=("DELEGATION_KIT_GATE_FILES is empty")
[[ ${#DELEGATION_KIT_META_PATHS[@]} -gt 0 ]] || _dk_errs+=("DELEGATION_KIT_META_PATHS is empty")
if [[ ${#_dk_errs[@]} -gt 0 ]]; then
    printf 'delegation-kit: malformed delegation config — the tools cannot run:\n' >&2
    printf '  %s\n' "${_dk_errs[@]}" >&2
    exit 2
fi
unset _dk_errs
