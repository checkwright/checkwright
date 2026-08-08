#!/usr/bin/env bash
# graph: couples=scripts/msg-patterns.list dir=one valve=none tier=precommit trigger=*
# install: zero-config
# spec: gate-sdk/SPEC.md §check-tree-terms — no tracked file matches the banned-pattern set (the tracked-files half of the leak guard; the pattern files and their templates are self-exempt)
#
# usage: check-tree-terms.sh [scan-root] [pattern-file...]
#   scan-root enumerated via git ls-files (default '.'); pattern-file args
#   override GATE_SDK_MSG_PATTERN_FILES (+ _LOCAL). Skips per spec: above.
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

SCANROOT="${1:-.}"
shift || true

git rev-parse --git-dir >/dev/null 2>&1 || {
    echo "check-tree-terms: not a git repository — cannot enumerate tracked files" >&2; exit 2; }

pat_list="$(gate_msg_pattern_files "$@")"; st=$?
fail_closed "$st" TREE-TERMS pattern-files
mapfile -t PATS < <(printf '%s' "$pat_list")

patterns="$(grep -hEv '^[[:space:]]*(#|$)' "${PATS[@]+"${PATS[@]}"}")"; gst=$?
[[ "$gst" -le 1 ]] || fail_closed "$gst" TREE-TERMS grep-patterns

npat=0
[[ -n "$patterns" ]] && npat="$(grep -c '' <<<"$patterns")"
if [[ "$npat" -eq 0 ]]; then
    echo "TREE-TERMS: clean (0 banned pattern(s) configured; tree unchecked)"
    exit 0
fi

listing="$(git ls-files -- "$SCANROOT")"; st=$?
fail_closed "$st" TREE-TERMS git-ls-files

# spec: gate-sdk/SPEC.md §check-tree-terms — the filter pass (pruned dirs, the pattern-file's own basename, non-regular files) stays a per-path loop with no fork in it; the grep itself batches every surviving path into one process instead of re-materialising the pattern file and re-execing grep per file
paths=()
while IFS= read -r path; do
    [[ -n "$path" ]] || continue
    gate_path_pruned "$path" && continue
    base="${path##*/}"
    [[ "$base" == msg-patterns* ]] && continue
    [[ -f "$path" ]] || continue
    paths+=("$path")
done <<< "$listing"
scanned="${#paths[@]}"

hits=""
if [[ "$scanned" -gt 0 ]]; then
    m="$(grep -EnHf <(printf '%s\n' "$patterns") "${paths[@]}")"; mst=$?
    [[ "$mst" -le 1 ]] || fail_closed "$mst" TREE-TERMS grep
    [[ -n "$m" ]] && hits="$m"$'\n'
fi

if [[ -n "$hits" ]]; then
    echo "check-tree-terms: tracked file(s) match a banned pattern (leaked local/private term):"
    printf '%s' "$hits"
    echo "  help: remove the leaked term from the tracked file; private term lists"
    echo "        belong in the gitignored local pattern file, never in a tracked one."
    exit 1
fi

echo "TREE-TERMS: clean ($scanned tracked file(s) scanned under $SCANROOT; none match the $npat banned pattern(s))"
exit 0
