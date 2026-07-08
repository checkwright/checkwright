#!/usr/bin/env bash
# graph: couples=scripts/git-hooks/* dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-hook-exec-bit — every tracked file in the hooks dir carries index mode 100755, or a fresh clone silently skips a non-executable hook
#
# usage: check-hook-exec-bit.sh [hooks-dir]
#   Defaults to GATE_SDK_HOOKS_DIR (<gates-dir>/git-hooks). Reads the git
#   *index* mode — the mode a fresh clone receives — not the worktree bit.
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

DIR="${1:-${GATE_SDK_HOOKS_DIR:-$(gate_sdk_gates_dir)/git-hooks}}"

git rev-parse --git-dir >/dev/null 2>&1 || {
    echo "check-hook-exec-bit: not a git repository — cannot read index modes" >&2; exit 2; }

if [[ ! -d "$DIR" ]]; then
    echo "HOOK-EXEC-BIT: clean (no hooks dir at $DIR — nothing committed to skip)"
    exit 0
fi

listing="$(git ls-files -s -- "$DIR")"; st=$?
fail_closed "$st" HOOK-EXEC-BIT git-ls-files

bad=(); count=0
while IFS= read -r line; do
    [[ -n "$line" ]] || continue
    count=$((count + 1))
    mode="${line%% *}"
    path="${line#*$'\t'}"
    [[ "$mode" == 100755 ]] || bad+=("$path (index mode $mode)")
done <<< "$listing"

if [[ ${#bad[@]} -gt 0 ]]; then
    echo "check-hook-exec-bit: hook file(s) not committed executable (mode 100755) — git"
    echo "silently skips a non-executable hook, disabling the gate battery for a fresh clone:"
    printf '  %s\n' "${bad[@]}"
    echo "  help: git update-index --chmod=+x <path> (and chmod +x locally), then recommit."
    exit 1
fi

echo "HOOK-EXEC-BIT: clean ($count tracked hook file(s) in $DIR at index mode 100755)"
exit 0
