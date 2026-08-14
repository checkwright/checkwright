# shellcheck shell=bash
# spec: context-kit/SPEC.md §lib/context.sh — sourced config loader + the kit's knob defaults, never gate structure; the config bridge sources this to resolve a `.gate`-dispatched member's declared knobs

_ck_cfg="${CONTEXT_KIT_CONFIG_FILE:-}"
if [[ -n "$_ck_cfg" ]]; then
    [[ -f "$_ck_cfg" ]] || {
        echo "context-kit: CONTEXT_KIT_CONFIG_FILE not found: $_ck_cfg" >&2
        exit 2
    }
    # shellcheck disable=SC1090  # consumer-supplied config, path is config
    source "$_ck_cfg"
else
    _ck_cfg="${GATE_SDK_GATES_DIR:-scripts}/context-config.sh"
    if [[ -f "$_ck_cfg" ]]; then
        # shellcheck disable=SC1090  # consumer-supplied config, path is config
        source "$_ck_cfg"
    fi
fi
unset _ck_cfg

# spec: context-kit/SPEC.md §lib/context.sh — every default is repo-relative: a bridged value is baked verbatim into the tracked pre-commit hook, so an absolute path would pin one clone's layout into a committed artifact
[[ -v CONTEXT_KIT_SETTINGS_FILE ]] || CONTEXT_KIT_SETTINGS_FILE=".claude/settings.json"
[[ -v CONTEXT_KIT_SETTINGS_PINS ]] || CONTEXT_KIT_SETTINGS_PINS="${GATE_SDK_GATES_DIR:-scripts}/settings-pins.conf"

# spec: context-kit/SPEC.md §check-memory-off — empty means "derive it", not "no dir": the harness names each project's dir from an absolute path this library must not compute on every knob resolution, so the default stays lazy and its owner is the function below
[[ -v CONTEXT_KIT_MEMORY_DIRS ]] || CONTEXT_KIT_MEMORY_DIRS=""

[[ -v CONTEXT_KIT_BREVITY_FILE ]] || CONTEXT_KIT_BREVITY_FILE="CLAUDE.md"
[[ -v CONTEXT_KIT_BREVITY_SECTION ]] || CONTEXT_KIT_BREVITY_SECTION="## Shared conventions"
[[ -v CONTEXT_KIT_BREVITY_BUDGET ]] || CONTEXT_KIT_BREVITY_BUDGET=4
[[ -v CONTEXT_KIT_BREVITY_POINTER_RE ]] || CONTEXT_KIT_BREVITY_POINTER_RE="§"

declare -p CONTEXT_KIT_SURFACES >/dev/null 2>&1 || CONTEXT_KIT_SURFACES=("CLAUDE.md")

[[ -v CONTEXT_KIT_BASELINE_FILE ]] || CONTEXT_KIT_BASELINE_FILE="${GATE_SDK_WORKFLOW_DIR:-.workflow}/always-loaded-baseline.txt"

[[ -v CONTEXT_KIT_ENV_PROFILE_FILE ]] || CONTEXT_KIT_ENV_PROFILE_FILE="ENV.local.md"

# spec: context-kit/SPEC.md §Layout and configuration — the harness names each project's dir by its absolute path with '/' and '.' folded to '-'; a function because the layout moves (the plugin-marketplace ruling) and because the value costs a subprocess no knob resolution should pay
context_memory_dir_default() {
    local top
    top="$(git rev-parse --show-toplevel 2>/dev/null)" || return 0
    [[ -n "$top" ]] || return 0
    printf '%s/.claude/projects/%s/memory\n' "$HOME" "$(printf '%s' "$top" | tr '/.' '-')"
}

_ck_errs=()
[[ -n "$CONTEXT_KIT_SETTINGS_FILE" ]] || _ck_errs+=("CONTEXT_KIT_SETTINGS_FILE is empty")
[[ -n "$CONTEXT_KIT_SETTINGS_PINS" ]] || _ck_errs+=("CONTEXT_KIT_SETTINGS_PINS is empty")
[[ -n "$CONTEXT_KIT_BREVITY_FILE" ]] || _ck_errs+=("CONTEXT_KIT_BREVITY_FILE is empty")
[[ "$CONTEXT_KIT_BREVITY_BUDGET" =~ ^[0-9]+$ ]] \
    || _ck_errs+=("CONTEXT_KIT_BREVITY_BUDGET must be an integer (got '$CONTEXT_KIT_BREVITY_BUDGET')")
if [[ ${#_ck_errs[@]} -gt 0 ]]; then
    printf 'context-kit: malformed context config — the gates cannot run:\n' >&2
    printf '  %s\n' "${_ck_errs[@]}" >&2
    exit 2
fi
unset _ck_errs
