#!/usr/bin/env bash
# graph: couples=gate-sdk/lib/gate.sh dir=one valve=none tier=commit-msg
# spec: gate-sdk/SPEC.md §check-commit-subject — the subject line parses as <type>(<scope>)?!?: <summary> with <type> in the shared roster, or matches a git-generated carve-out (the parse guarantee under trajectory.sh's feat/debt column)
#
# usage: check-commit-subject.sh <message-file>
#   message-file: the commit-msg hook's $1. Roster is GATE_SDK_COMMIT_TYPES
#   (gate_commit_types, lib/gate.sh); no positional roster override.
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

# spec: gate-sdk/SPEC.md §check-commit-subject — no-arg is a clean skip: the message is not a whole-tree surface
if [[ $# -eq 0 ]]; then
    echo "COMMIT-SUBJECT: clean (no message file argument — the commit-msg hook surface is not a whole-tree target; skipped)"
    exit 0
fi
MSG="$1"
[[ -f "$MSG" ]] || { echo "check-commit-subject: message file not found: $MSG" >&2; exit 2; }

roster="$(gate_commit_types)"; st=$?
fail_closed "$st" COMMIT-SUBJECT roster
types="$(printf '%s' "$roster" | tr ' ' '|')"

subject="$(sed -n '1p' "$MSG")"; sst=$?
fail_closed "$sst" COMMIT-SUBJECT subject

# spec: gate-sdk/SPEC.md §check-commit-subject — conventional grammar (roster type, optional (scope) with token [a-z0-9./-]+, optional ! break marker, ': ' then non-empty summary) OR a git-generated carve-out (Merge/Revert + the fixup!/squash! autosquash forms)
if [[ "$subject" =~ ^($types)(\([a-z0-9./-]+\))?!?:\ .+$ ]] \
   || [[ "$subject" =~ ^(Merge|Revert)\  ]] \
   || [[ "$subject" =~ ^(fixup|squash)!\  ]]; then
    echo "COMMIT-SUBJECT: clean (subject parses against the ${roster// /, } roster or a git-generated carve-out)"
    exit 0
fi

echo "check-commit-subject: subject line does not parse as <type>(<scope>)?!?: <summary>:"
echo "  $subject"
echo "  help: open the subject with a roster type followed by ': ' and a summary"
echo "        (e.g. 'feat(scope): …'); the roster is GATE_SDK_COMMIT_TYPES"
echo "        (default: $roster). git's own Merge/Revert/fixup!/squash! forms are"
echo "        carve-outs — do not reword them."
exit 1
