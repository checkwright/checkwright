# shellcheck shell=bash
# spec: delegation-kit/SPEC.md §Layout and configuration — sourced config loader for usage-gate + check-gate-tamper
#
# Extracted from the governance meta-layer of a private production platform;
# the single-operator source path, budget thresholds, gate-file roster, and
# meta-layer prefixes are the defaults below, overridable per consumer — the
# platform's validate battery and shared-file roster stayed behind. This file
# carries values only (gate-sdk's lib/gate.sh rule): the two tools that gate
# delegation share their knobs here.

# Source the consumer config first so its assignments win; the defaults below
# fill only what it left unset. A malformed config exits 2 — a broken machine
# must not gate anything.
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

# --- usage-gate knobs (the platform's single-operator budget verdict) --------

# The usage snapshot the verdict reads (usage.txt contract, SPEC §usage-gate).
# Default the platform's single-operator path; usage-gate's positional $1 wins.
[[ -v DELEGATION_KIT_USAGE_FILE ]] || DELEGATION_KIT_USAGE_FILE="${CLAUDE_CONFIG_DIR:-$HOME/.claude}/usage.txt"

# The credentials file whose mtime dates the last auth event (post-login lag).
# Default the usage file's sibling; usage-gate's positional $2 wins.
[[ -v DELEGATION_KIT_CRED_FILE ]] || DELEGATION_KIT_CRED_FILE="${DELEGATION_KIT_USAGE_FILE%/*}/.credentials.json"

# PAUSE above this percentage of the live 5h window.
[[ -v DELEGATION_KIT_PAUSE_PCT ]] || DELEGATION_KIT_PAUSE_PCT=80

# A reading older than this many seconds is STALE (re-read before trusting).
[[ -v DELEGATION_KIT_STALE_AGE ]] || DELEGATION_KIT_STALE_AGE=600

# A would-be PAUSE within this many seconds of the last auth event routes to
# STALE instead — a fresh /login starts a window the server-fed pct lags.
[[ -v DELEGATION_KIT_LOGIN_WINDOW ]] || DELEGATION_KIT_LOGIN_WINDOW=600

# --- check-gate-tamper knobs (assertion A gate-edit isolation) ---------------

# Globs naming gate files; a commit touching one is constrained to meta paths.
# Default the platform's single-gates-dir layout (check-*.sh plus the sourced
# lib and the fixture runner); a gate-sdk consumer widens this to its kit
# checks/ dirs and the vendored lib/runner (see templates/delegation-config.sh).
declare -p DELEGATION_KIT_GATE_FILES &>/dev/null || DELEGATION_KIT_GATE_FILES=(
    "${GATE_SDK_GATES_DIR:-scripts}/check-*.sh"
    "${GATE_SDK_GATES_DIR:-scripts}/lib/gate.sh"
    "${GATE_SDK_GATES_DIR:-scripts}/run-gate-tests.sh"
)

# Path prefixes counted as meta-layer for assertion A. Root-level *.md is always
# meta (handled in the gate), so it need not be listed. Default the platform's
# governance surfaces; a gate-sdk consumer adds its kit dirs.
declare -p DELEGATION_KIT_META_PATHS &>/dev/null || DELEGATION_KIT_META_PATHS=(
    "${GATE_SDK_GATES_DIR:-scripts}/"
    "${GATE_SDK_WORKFLOW_DIR:-.workflow}/"
    ".claude/"
)

# --- config validation (a malformed machine must not gate anything) ----------

_dk_errs=()
[[ -n "$DELEGATION_KIT_USAGE_FILE" ]] || _dk_errs+=("DELEGATION_KIT_USAGE_FILE is empty")
[[ "$DELEGATION_KIT_PAUSE_PCT" =~ ^[0-9]+(\.[0-9]+)?$ ]] \
    || _dk_errs+=("DELEGATION_KIT_PAUSE_PCT must be numeric (got '$DELEGATION_KIT_PAUSE_PCT')")
[[ "$DELEGATION_KIT_STALE_AGE" =~ ^[0-9]+$ ]] \
    || _dk_errs+=("DELEGATION_KIT_STALE_AGE must be a non-negative integer (got '$DELEGATION_KIT_STALE_AGE')")
[[ "$DELEGATION_KIT_LOGIN_WINDOW" =~ ^[0-9]+$ ]] \
    || _dk_errs+=("DELEGATION_KIT_LOGIN_WINDOW must be a non-negative integer (got '$DELEGATION_KIT_LOGIN_WINDOW')")
[[ ${#DELEGATION_KIT_GATE_FILES[@]} -gt 0 ]] || _dk_errs+=("DELEGATION_KIT_GATE_FILES is empty")
[[ ${#DELEGATION_KIT_META_PATHS[@]} -gt 0 ]] || _dk_errs+=("DELEGATION_KIT_META_PATHS is empty")
if [[ ${#_dk_errs[@]} -gt 0 ]]; then
    printf 'delegation-kit: malformed delegation config — the tools cannot run:\n' >&2
    printf '  %s\n' "${_dk_errs[@]}" >&2
    exit 2
fi
unset _dk_errs
