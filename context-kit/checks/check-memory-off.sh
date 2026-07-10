#!/usr/bin/env bash
# graph: couples=scripts/settings-pins.conf dir=one valve=none tier=precommit trigger=*
# spec: context-kit/SPEC.md §check-memory-off — local-environment scan: the per-project harness memory dir stays empty and no local settings override re-enables a pinned key
#
# usage: check-memory-off.sh [--fixture <dir>]
#   live: scans CONTEXT_KIT_MEMORY_DIRS and the untracked local settings file;
#   --fixture <dir> scans <dir>/memory, <dir>/settings.local.json, <dir>/settings-pins.conf
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

_ck_cfg="${CONTEXT_KIT_CONFIG_FILE:-${GATE_SDK_GATES_DIR:-scripts}/context-config.sh}"
if [[ -f "$_ck_cfg" ]]; then
    # shellcheck source=/dev/null  # consumer config path is resolved at runtime
    source "$_ck_cfg"
fi
unset _ck_cfg

# spec: context-kit/SPEC.md §Layout and configuration — the harness names each project's dir by its absolute path with '/' and '.' folded to '-'; a knob because the layout moves (the plugin-marketplace ruling)
memory_dir_default() {
    local top
    top="$(git rev-parse --show-toplevel 2>/dev/null)" || return 0
    [[ -n "$top" ]] || return 0
    printf '%s/.claude/projects/%s/memory\n' "$HOME" "$(printf '%s' "$top" | tr '/.' '-')"
}

MODE=live
FIXTURE_DIR=""
while [[ $# -gt 0 ]]; do
    case "$1" in
        --fixture) MODE=fixture; FIXTURE_DIR="${2:-}"; shift 2 ;;
        -*) echo "check-memory-off: unknown argument: $1" >&2; exit 2 ;;
        *)  echo "check-memory-off: unexpected argument: $1" >&2; exit 2 ;;
    esac
done

declare -a MEM_DIRS=()
if [[ "$MODE" == fixture ]]; then
    [[ -d "$FIXTURE_DIR" ]] || { echo "check-memory-off: fixture dir not found: $FIXTURE_DIR" >&2; exit 2; }
    MEM_DIRS=("$FIXTURE_DIR/memory")
    PINS_FILE="$FIXTURE_DIR/settings-pins.conf"
    LOCAL_SETTINGS="$FIXTURE_DIR/settings.local.json"
else
    : "${CONTEXT_KIT_SETTINGS_FILE:=.claude/settings.json}"
    : "${CONTEXT_KIT_SETTINGS_PINS:=${GATE_SDK_GATES_DIR:-scripts}/settings-pins.conf}"
    PINS_FILE="$CONTEXT_KIT_SETTINGS_PINS"
    LOCAL_SETTINGS="${CONTEXT_KIT_SETTINGS_FILE%.json}.local.json"
    memdirs="${CONTEXT_KIT_MEMORY_DIRS:-$(memory_dir_default)}"
    for pat in $memdirs; do
        for d in $pat; do
            [[ -e "$d" ]] && MEM_DIRS+=("$d")
        done
    done
fi

polluted=(); overrides=()

# spec: context-kit/SPEC.md §check-memory-off — content is any regular file that is not the dir-preserving .gitkeep
for d in "${MEM_DIRS[@]+"${MEM_DIRS[@]}"}"; do
    [[ -d "$d" ]] || continue
    files="$(find "$d" -type f ! -name .gitkeep 2>/dev/null)"; st=$?
    fail_closed "$st" check-memory-off find
    [[ -n "$files" ]] && polluted+=("$d")
done

# spec: context-kit/SPEC.md §check-memory-off — the untracked local settings file can re-enable what the tracked pin disabled; the hermetic gate cannot see it, this one can
if [[ -f "$LOCAL_SETTINGS" && -e "$PINS_FILE" && -r "$PINS_FILE" ]]; then
    command -v jq >/dev/null 2>&1 || { echo "check-memory-off: jq not found — cannot read $LOCAL_SETTINGS to verify no pinned key is overridden" >&2; exit 2; }
    jq -e . "$LOCAL_SETTINGS" >/dev/null 2>&1 || { echo "check-memory-off: $LOCAL_SETTINGS is not valid JSON" >&2; exit 2; }
    while IFS= read -r line; do
        path="${line%%=*}"; path="${path#"${path%%[![:space:]]*}"}"; path="${path%"${path##*[![:space:]]}"}"
        expected="${line#*=}"; expected="${expected#"${expected%%[![:space:]]*}"}"; expected="${expected%"${expected##*[![:space:]]}"}"
        [[ "$line" == *=* && -n "$path" && -n "$expected" && "$path" == .* ]] || continue
        actual="$(jq -c "$path" "$LOCAL_SETTINGS" 2>/dev/null)"; st=$?
        [[ "$st" -eq 0 ]] || continue
        if [[ "$actual" != "null" && "$actual" != "$expected" ]]; then
            overrides+=("$path locally set to $actual (pin expects $expected)")
        fi
    done < <(gates_list_members "$PINS_FILE")
fi

if [[ ${#polluted[@]} -gt 0 || ${#overrides[@]} -gt 0 ]]; then
    echo "check-memory-off: the harness memory posture is not clean on this clone:"
    for d in "${polluted[@]+"${polluted[@]}"}"; do
        echo "  memory dir holds content: $d"
    done
    for o in "${overrides[@]+"${overrides[@]}"}"; do
        echo "  local settings override: $o"
    done
    echo "  help: durable facts belong in tracked surfaces (the knowledge-friction loop,"
    echo "        the lesson channels, or the operator's local brief), not the harness"
    echo "        memory dir — empty it; and drop any memory-re-enabling key from"
    echo "        $LOCAL_SETTINGS. See context-kit/SPEC.md §The memory-off doctrine."
    exit 1
fi

# spec: context-kit/SPEC.md §check-memory-off — CI-neutral: an absent dir proves nothing about another clone (fail-open on absent, stated here)
echo "MEMORY-OFF: clean (${#MEM_DIRS[@]} memory dir(s) present, all empty; an absent dir proves nothing about another clone)"
exit 0
