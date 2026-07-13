#!/usr/bin/env bash
# graph: couples=scripts/root-allowlist.list dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-root-tiering — the repo root holds only allowlisted orientation entries; workflow machinery stays under the configured dirs
#
# usage: check-root-tiering.sh [allowlist-file] [scan-root]
#   allowlist-file: GATE_SDK_ROOT_ALLOWLIST (default root-allowlist.list; absent
#   → the built-in minimal set). scan-root: the checked tree (default '.').
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

ALLOWFILE="${1:-${GATE_SDK_ROOT_ALLOWLIST:-$(gate_sdk_gates_dir)/root-allowlist.list}}"
SCANROOT="${2:-.}"

git rev-parse --git-dir >/dev/null 2>&1 || {
    echo "check-root-tiering: not a git repository — cannot enumerate tracked root entries" >&2; exit 2; }

if [[ -f "$ALLOWFILE" ]]; then
    [[ -r "$ALLOWFILE" ]] || { echo "check-root-tiering: allowlist not readable: $ALLOWFILE" >&2; exit 2; }
    mapfile -t allow < <(gates_list_members "$ALLOWFILE")
    src="$ALLOWFILE"
else
    allow=(README.md LICENSE "${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}" "${GATE_SDK_AGENT_FILE:-CLAUDE.md}" .gitignore 'SPEC-*.md')
    src="built-in minimal orientation set"
fi

covered() {
    local e="$1" p
    for p in "${allow[@]}"; do
        # shellcheck disable=SC2053  # $p is the allowlist glob, matched unquoted on purpose
        [[ "$e" == $p ]] && return 0
    done
    return 1
}

prefix=""
[[ "$SCANROOT" != "." ]] && prefix="${SCANROOT%/}/"

listing="$(git ls-files -- "$SCANROOT")"; st=$?
fail_closed "$st" ROOT-TIERING git-ls-files

declare -A seen=()
stray=(); count=0
while IFS= read -r path; do
    [[ -n "$path" ]] || continue
    entry="${path#"$prefix"}"; entry="${entry%%/*}"
    [[ -n "${seen[$entry]:-}" ]] && continue
    seen[$entry]=1
    count=$((count + 1))
    covered "$entry" || stray+=("$entry")
done <<< "$listing"

if [[ ${#stray[@]} -gt 0 ]]; then
    echo "check-root-tiering: tracked top-level entry not in the allowlist ($src) —"
    echo "the repo root is the orientation surface; workflow machinery belongs under the"
    echo "configured workflow/gates dirs, not scattered at root:"
    printf '  %s\n' "${stray[@]}"
    echo "  help: move the entry under an existing dir, or — if it is a deliberate new"
    echo "        root surface — add it to $ALLOWFILE in the same commit."
    exit 1
fi

echo "ROOT-TIERING: clean ($count tracked top-level entr(y|ies) all allowlisted via $src)"
exit 0
