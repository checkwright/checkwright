#!/usr/bin/env bash
# graph: couples=scripts/msg-patterns.list dir=one valve=none tier=commit-msg
# spec: gate-sdk/SPEC.md §check-commit-msg — the commit message matches no banned pattern (the leak guard for the message surface the pre-commit hook never sees)
#
# usage: check-commit-msg.sh <message-file> [pattern-file...]
#   message-file: the commit-msg hook's $1. pattern-file args (grep -E, one per
#   line) override GATE_SDK_MSG_PATTERN_FILES (+ _LOCAL); default when absent.
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

# spec: gate-sdk/SPEC.md §check-commit-msg — no-arg is a clean skip: the message is not a whole-tree surface and a history scan is deferred
if [[ $# -eq 0 ]]; then
    echo "COMMIT-MSG: clean (no message file argument — the commit-msg hook surface is not a whole-tree target; skipped)"
    exit 0
fi
MSG="$1"
[[ -f "$MSG" ]] || { echo "check-commit-msg: message file not found: $MSG" >&2; exit 2; }
shift

pat_list="$(gate_msg_pattern_files "$@")"; st=$?
fail_closed "$st" COMMIT-MSG pattern-files
mapfile -t PATS < <(printf '%s' "$pat_list")

patterns="$(grep -hEv '^[[:space:]]*(#|$)' "${PATS[@]+"${PATS[@]}"}")"; gst=$?
[[ "$gst" -le 1 ]] || fail_closed "$gst" COMMIT-MSG grep-patterns

npat=0
[[ -n "$patterns" ]] && npat="$(grep -c '' <<<"$patterns")"
if [[ "$npat" -eq 0 ]]; then
    echo "COMMIT-MSG: clean (0 banned pattern(s) configured; message unchecked)"
    exit 0
fi

hits="$(grep -EnHf <(printf '%s\n' "$patterns") "$MSG")"; hst=$?
[[ "$hst" -le 1 ]] || fail_closed "$hst" COMMIT-MSG grep

if [[ -n "$hits" ]]; then
    echo "check-commit-msg: commit message matches a banned pattern (leaked local/private term):"
    echo "$hits"
    echo "  help: rewrite the message to remove the leaked term; the pattern set is"
    echo "        GATE_SDK_MSG_PATTERN_FILES (+ the local list). The Co-Authored-By"
    echo "        trailer is a footer convention, not a leak — do not ban it."
    exit 1
fi

echo "COMMIT-MSG: clean (message matches none of $npat banned pattern(s))"
exit 0
