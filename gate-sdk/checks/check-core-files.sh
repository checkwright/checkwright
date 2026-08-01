#!/usr/bin/env bash
# graph: couples=scripts/core-files.list,kit:SPEC.md,kit:README.md dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-core-files — every path in the core-files manifest exists in the worktree and is tracked
#
# usage: check-core-files.sh [manifest]
#   default: GATE_SDK_CORE_FILES_FILE (<gates-dir>/core-files.list)
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

MANIFEST="${1:-${GATE_SDK_CORE_FILES_FILE:-$(gate_sdk_gates_dir)/core-files.list}}"

if [[ ! -e "$MANIFEST" ]]; then
    echo "CORE-FILES: clean (no manifest at $MANIFEST — optional consumer config absent)"
    exit 0
fi
[[ -r "$MANIFEST" ]] || { echo "check-core-files: manifest not readable: $MANIFEST" >&2; exit 2; }

mapfile -t lines < <(gates_list_members "$MANIFEST")
if [[ ${#lines[@]} -eq 0 ]]; then
    echo "CORE-FILES: clean (manifest $MANIFEST lists no paths)"
    exit 0
fi

# spec: gate-sdk/SPEC.md §check-core-files — a kit: line derives one path per kit root; the
# wildcard refusal is fail-closed because this reader requires each expanded path to exist,
# which is a different invariant from the glob match a couples= field performs
paths=(); _cf_expanded=""
for line in "${lines[@]}"; do
    if [[ "$line" == kit:* ]]; then
        case "${line#kit:}" in
            *'*'* | *'?'* | *'['*)
                echo "check-core-files: kit: token carries a wildcard in $MANIFEST: $line" >&2
                echo "  help: this manifest requires every expanded path to exist and be tracked," >&2
                echo "        which a wildcard cannot express — name an exact per-kit basename" >&2
                echo "        (e.g. kit:SPEC.md), or hand-list the paths." >&2
                exit 2
                ;;
        esac
        gate_expand_couples_var _cf_expanded "$line"
        IFS=',' read -ra _cf_parts <<<"$_cf_expanded"
        paths+=("${_cf_parts[@]}")
    else
        paths+=("$line")
    fi
done

git rev-parse --git-dir >/dev/null 2>&1 || {
    echo "check-core-files: not a git repository — cannot verify tracked status" >&2; exit 2; }

missing=(); untracked=(); present=0
for p in "${paths[@]}"; do
    if [[ ! -e "$p" ]]; then
        missing+=("$p")
    elif git ls-files --error-unmatch -- "$p" >/dev/null 2>&1; then
        present=$((present + 1))
    else
        untracked+=("$p")
    fi
done

if [[ ${#missing[@]} -gt 0 || ${#untracked[@]} -gt 0 ]]; then
    echo "check-core-files: core file(s) listed in $MANIFEST but missing or untracked"
    echo "(the silent-deletion class downstream gates catch only incidentally):"
    for p in "${missing[@]}"; do echo "  missing:   $p"; done
    for p in "${untracked[@]}"; do echo "  untracked: $p"; done
    echo "  help: restore the file (git checkout / git add), or — if the removal is"
    echo "        intentional — delete its line from $MANIFEST in the same commit."
    exit 1
fi

echo "CORE-FILES: clean ($present manifest path(s) present and tracked in $MANIFEST)"
exit 0
