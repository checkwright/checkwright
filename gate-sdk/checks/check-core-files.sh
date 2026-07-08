#!/usr/bin/env bash
# graph: couples=scripts/core-files.list dir=one valve=none tier=precommit
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

mapfile -t paths < <(gates_list_members "$MANIFEST")
if [[ ${#paths[@]} -eq 0 ]]; then
    echo "CORE-FILES: clean (manifest $MANIFEST lists no paths)"
    exit 0
fi

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

echo "CORE-FILES: clean ($present listed path(s) present and tracked in $MANIFEST)"
exit 0
