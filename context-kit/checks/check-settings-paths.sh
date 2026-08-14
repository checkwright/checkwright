#!/usr/bin/env bash
# graph: couples=.claude/settings.json,kit:checks/*.sh,scripts/check-*.sh dir=one valve=none tier=precommit
# install: on-surface
# spec: context-kit/SPEC.md §check-settings-paths — every committed allow-list entry whose command token is a literal repo-relative .sh path resolves in the working tree
#
# usage: check-settings-paths.sh [--fixture <dir>]
#   live: reads CONTEXT_KIT_SETTINGS_FILE;
#   --fixture <dir> reads <dir>/settings.json and resolves paths against <dir>
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

_ck_cfg="${CONTEXT_KIT_CONFIG_FILE:-}"
if [[ -n "$_ck_cfg" ]]; then
    [[ -f "$_ck_cfg" ]] || {
        echo "context-kit: CONTEXT_KIT_CONFIG_FILE not found: $_ck_cfg" >&2
        exit 2
    }
    # shellcheck source=/dev/null  # consumer config path is resolved at runtime
    source "$_ck_cfg"
else
    _ck_cfg="${GATE_SDK_GATES_DIR:-scripts}/context-config.sh"
    if [[ -f "$_ck_cfg" ]]; then
        # shellcheck source=/dev/null  # consumer config path is resolved at runtime
        source "$_ck_cfg"
    fi
fi
unset _ck_cfg

FIXTURE_DIR=""
while [[ $# -gt 0 ]]; do
    case "$1" in
        --fixture) FIXTURE_DIR="${2:-}"; shift 2 ;;
        -*) echo "check-settings-paths: unknown argument: $1" >&2; exit 2 ;;
        *)  echo "check-settings-paths: unexpected argument: $1" >&2; exit 2 ;;
    esac
done

if [[ -n "$FIXTURE_DIR" ]]; then
    [[ -d "$FIXTURE_DIR" ]] || { echo "check-settings-paths: fixture dir not found: $FIXTURE_DIR" >&2; exit 2; }
    SETTINGS_FILE="$FIXTURE_DIR/settings.json"
    ROOT="$FIXTURE_DIR"
else
    : "${CONTEXT_KIT_SETTINGS_FILE:=.claude/settings.json}"
    SETTINGS_FILE="$CONTEXT_KIT_SETTINGS_FILE"
    ROOT="."
fi

command -v jq >/dev/null 2>&1 || { echo "check-settings-paths: jq not found — cannot read $SETTINGS_FILE" >&2; exit 2; }
# spec: context-kit/SPEC.md §check-settings-paths — the settings file is the sole subject and the sibling gate reads it on the same terms, so an absent or unparseable one is fail-closed here too, never a clean skip
[[ -r "$SETTINGS_FILE" ]] || { echo "check-settings-paths: settings file not readable: $SETTINGS_FILE" >&2; exit 2; }
jq -e . "$SETTINGS_FILE" >/dev/null 2>&1 || { echo "check-settings-paths: $SETTINGS_FILE is not valid JSON" >&2; exit 2; }

entries="$(jq -r '.permissions.allow[]? | select(type == "string")' "$SETTINGS_FILE")"; st=$?
fail_closed "$st" check-settings-paths jq

dead=(); checked=0
while IFS= read -r entry; do
    [[ -n "$entry" ]] || continue
    [[ "$entry" == 'Bash('*')' ]] || continue
    inner="${entry#Bash(}"; inner="${inner%)}"
    # spec: context-kit/SPEC.md §check-settings-paths — `read -ra` splits without pathname
    # expansion: an unquoted array assignment would expand a pattern grant against the tree
    # and check an arbitrary first match, which greens the glob class instead of skipping it
    read -r -a tok <<<"$inner"
    i=0
    # spec: context-kit/SPEC.md §check-settings-paths — the command token is not always argv[0]: a grant may lead with `env NAME=VALUE ...` before the interpreter, and one on this tree does
    if [[ "${tok[0]:-}" == env ]]; then
        i=1
        while [[ "$i" -lt "${#tok[@]}" && "${tok[$i]}" == [A-Za-z_]*=* ]]; do i=$((i + 1)); done
    fi
    if [[ "${tok[$i]:-}" == bash || "${tok[$i]:-}" == sh ]]; then i=$((i + 1)); fi
    cand="${tok[$i]:-}"
    [[ "$cand" == *.sh ]] || continue
    # spec: context-kit/SPEC.md §check-settings-paths — a `*` in the command token makes it a pattern, intentionally polymorphic over files that need not exist today; the `*` twin of a literal grant is a separate token and stays in scope
    [[ "$cand" == *'*'* ]] && continue
    checked=$((checked + 1))
    [[ -f "$ROOT/$cand" ]] || dead+=("$entry — no such file: $cand")
done <<<"$entries"

if [[ ${#dead[@]} -gt 0 ]]; then
    echo "check-settings-paths: $SETTINGS_FILE grants a path that does not resolve in the tree:"
    printf '  %s\n' "${dead[@]}"
    echo "  help: repoint each entry at the path that replaced it, or drop the entry if the"
    echo "        grant is spent — a port that replaces checks/<gate>.sh with <gate>.gate"
    echo "        strands both the bare form and its '*' twin."
    exit 1
fi

echo "SETTINGS-PATHS: clean ($checked literal .sh grant(s) in $SETTINGS_FILE resolve)"
exit 0
