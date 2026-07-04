#!/usr/bin/env bash
# graph: couples=TASK-QUEUE.md dir=one valve=none tier=precommit
# no-fixture: HEAD-vs-worktree diff — a committed fixture has HEAD == worktree, so the loss case (an uncommitted deletion) has no static-fixture representation (queue-kit/SPEC.md §check-task-conservation).
# spec: queue-kit/SPEC.md §check-task-conservation — every live slug present at HEAD is still present (live or done) in the working tree
#
# usage: check-task-conservation.sh [queue-file]
#   Defaults to the configured queue file (QUEUE_KIT_QUEUE_FILE). Diffs
#   `git show HEAD:<queue>` against the worktree; no git baseline ⇒ clean exit.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/queue.sh
source "$KIT/lib/queue.sh"

FILE="${1:-$QUEUE_KIT_QUEUE_FILE}"

git rev-parse --git-dir >/dev/null 2>&1 || {
    echo "TASK-CONSERVATION: clean (no git repository — no HEAD baseline to compare)"; exit 0; }

head_tmp="$(mktemp)"
trap 'rm -f "$head_tmp"' EXIT
if ! git show "HEAD:$FILE" >"$head_tmp" 2>/dev/null; then
    echo "TASK-CONSERVATION: clean ($FILE not at HEAD — no prior live slugs to conserve)"; exit 0
fi
[[ -f "$FILE" ]] || { echo "check-task-conservation: worktree file not found: $FILE" >&2; exit 2; }

# Worktree present set = live ∪ done (a HEAD-live slug is conserved if it is
# still live or has moved to done).
declare -A present=()
while IFS= read -r s; do [[ -n "$s" ]] && present["$s"]=1; done < <(queue_live_slugs "$FILE")
while IFS= read -r s; do [[ -n "$s" ]] && present["$s"]=1; done < <(queue_done_slugs "$FILE")

lost=(); conserved=0
while IFS= read -r s; do
    [[ -n "$s" ]] || continue
    if [[ -n "${present[$s]:-}" ]]; then conserved=$((conserved + 1)); else lost+=("$s"); fi
done < <(queue_live_slugs "$head_tmp")

if [[ ${#lost[@]} -gt 0 ]]; then
    echo "check-task-conservation: live slug(s) present at HEAD but gone from the working"
    echo "tree — neither live nor done (a lost task; the absence class diff-review misses):"
    printf '  %s\n' "${lost[@]}"
    echo "  help: restore the entry, or move its slug to the done section if it completed."
    echo "        A rename must move the old slug to done and sweep every [blocked-by:] ref."
    exit 1
fi

echo "TASK-CONSERVATION: clean ($conserved HEAD live slug(s) all still present in $FILE)"
exit 0
