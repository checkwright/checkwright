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
# spec: context-kit/SPEC.md §Layout and configuration — the settings knob tells two adoption modes apart by *set-ness*, and a guarded default erases that the moment the value crosses the config bridge: a compiled reader sees one path string and cannot tell an explicitly-misconfigured file (refuse) from an unadopted one (degrade). The refusal is therefore taken *here*, where set-ness is still visible — an explicitly-set path that does not exist is adopted-but-broken and exits 2 — while an unset knob keeps the plain default, and a reader that cannot open *that* path is in the not-adopted case and degrades. The empty-string signal drift-kit's counterpart uses is unavailable here: the validation below rejects an empty value as a malformed config, and that invariant is older than this bridge.
[[ -v CONTEXT_KIT_SETTINGS_FILE ]] \
    && { [[ -f "$CONTEXT_KIT_SETTINGS_FILE" ]] || {
             echo "context-kit: CONTEXT_KIT_SETTINGS_FILE not found: $CONTEXT_KIT_SETTINGS_FILE" >&2
             exit 2
         }; } \
    || CONTEXT_KIT_SETTINGS_FILE=".claude/settings.json"
[[ -v CONTEXT_KIT_SETTINGS_PINS ]] || CONTEXT_KIT_SETTINGS_PINS="${GATE_SDK_GATES_DIR:-scripts}/settings-pins.conf"

# spec: context-kit/SPEC.md §check-memory-off — empty means "derive it", not "no dir": the harness names each project's dir from an absolute path this library must not compute on every knob resolution, so the default stays lazy and its owner is the gate that reads it
[[ -v CONTEXT_KIT_MEMORY_DIRS ]] || CONTEXT_KIT_MEMORY_DIRS=""

[[ -v CONTEXT_KIT_BREVITY_FILE ]] || CONTEXT_KIT_BREVITY_FILE="CLAUDE.md"
[[ -v CONTEXT_KIT_BREVITY_SECTION ]] || CONTEXT_KIT_BREVITY_SECTION="## Shared conventions"
[[ -v CONTEXT_KIT_BREVITY_BUDGET ]] || CONTEXT_KIT_BREVITY_BUDGET=4
[[ -v CONTEXT_KIT_BREVITY_POINTER_RE ]] || CONTEXT_KIT_BREVITY_POINTER_RE="§"

declare -p CONTEXT_KIT_SURFACES >/dev/null 2>&1 || CONTEXT_KIT_SURFACES=("CLAUDE.md")

[[ -v CONTEXT_KIT_BASELINE_FILE ]] || CONTEXT_KIT_BASELINE_FILE="${GATE_SDK_WORKFLOW_DIR:-.workflow}/always-loaded-baseline.txt"

[[ -v CONTEXT_KIT_ENV_PROFILE_FILE ]] || CONTEXT_KIT_ENV_PROFILE_FILE="ENV.local.md"

# spec: context-kit/SPEC.md §Index-first reading — the kit's one traversal-exclusion set, matched on the leaf basename; the default is the union of the two literals the index tools carried privately plus the worktrees leaf, and .tmp / gate-tests are deliberately absent
declare -p CONTEXT_KIT_PRUNE_DIRS >/dev/null 2>&1 \
    || CONTEXT_KIT_PRUNE_DIRS=(.git node_modules target dist build worktrees)

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
