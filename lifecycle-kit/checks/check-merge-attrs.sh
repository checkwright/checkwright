#!/usr/bin/env bash
# graph: couples=.gitattributes,lifecycle-kit/lib/stages.sh dir=one valve=none tier=precommit
# spec: lifecycle-kit/SPEC.md §check-merge-attrs — bidirectional parity between the derived iteration-scoped supersede set and the merge=iteration-scoped lines in .gitattributes (a smuggled ours-driver attribute on a path outside the set is the reverse-direction safety edge), plus forward-only parity for the derived union set (the gap inbox must carry merge=union; the git-native union driver is legitimate consumer usage elsewhere, so no reverse edge)
#
# usage: check-merge-attrs.sh [gitattributes-file]
#   path resolves relative to cwd (= repo root in a battery run); the default
#   is .gitattributes (repo root).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"

ATTRS="${1:-.gitattributes}"

derived="$(lifecycle_supersede_set | sort -u | grep -v '^$')"
[[ -n "$derived" ]] \
    || { echo "check-merge-attrs: the derived iteration-scoped supersede set is empty — the state machine names no boundary-truncated surface (a lifecycle always owns at least its state + lesson-evidence files)" >&2; exit 2; }  # exit 2: fail-closed
union_derived="$(lifecycle_union_set | sort -u | grep -v '^$')"
[[ -n "$union_derived" ]] \
    || { echo "check-merge-attrs: the derived union-merge set is empty — the state machine names no append-only union surface (a lifecycle always owns at least its gap inbox)" >&2; exit 2; }  # exit 2: fail-closed

if [[ -e "$ATTRS" ]]; then
    [[ -r "$ATTRS" ]] \
        || { echo "check-merge-attrs: $ATTRS exists but is not readable — the parity cannot be checked" >&2; exit 2; }  # exit 2: fail-closed
    attributed="$(awk '
        /^[[:space:]]*#/ { next }
        { for (i = 2; i <= NF; i++) if ($i == "merge=iteration-scoped") { print $1; break } }
    ' "$ATTRS")"; st=$?
    fail_closed "$st" check-merge-attrs awk
    attributed="$(printf '%s\n' "$attributed" | sort -u | grep -v '^$' || true)"
    union_attributed="$(awk '
        /^[[:space:]]*#/ { next }
        { for (i = 2; i <= NF; i++) if ($i == "merge=union") { print $1; break } }
    ' "$ATTRS")"; st=$?
    fail_closed "$st" check-merge-attrs awk
    union_attributed="$(printf '%s\n' "$union_attributed" | sort -u | grep -v '^$' || true)"
else
    attributed=""
    union_attributed=""
fi

missing="$(comm -23 <(printf '%s\n' "$derived") <(printf '%s\n' "$attributed" | grep -v '^$'))"
extra="$(comm -13 <(printf '%s\n' "$derived") <(printf '%s\n' "$attributed" | grep -v '^$'))"
# spec: lifecycle-kit/SPEC.md §check-merge-attrs — forward only: a union member with no merge=union line. The reverse edge is deliberately absent — merge=union is git-native and a consumer's own append log legitimately carries it (§Multi-operator semantics).
union_missing="$(comm -23 <(printf '%s\n' "$union_derived") <(printf '%s\n' "$union_attributed" | grep -v '^$'))"

if [[ -n "$missing" || -n "$extra" || -n "$union_missing" ]]; then
    echo "check-merge-attrs: $ATTRS merge attributes are out of parity with the derived supersede/union sets:"
    while IFS= read -r p; do
        [[ -n "$p" ]] && echo "  supersede path with no merge=iteration-scoped attribute (the merge-supersede rule is unmechanized for it): $p"
    done <<<"$missing"
    while IFS= read -r p; do
        [[ -n "$p" ]] && echo "  merge=iteration-scoped on a path outside the derived set (a smuggled ours-driver silently discards merge content on a real surface): $p"
    done <<<"$extra"
    while IFS= read -r p; do
        [[ -n "$p" ]] && echo "  union-merge path with no merge=union attribute (a gap filed on either side of a concurrent merge would be silently dropped): $p"
    done <<<"$union_missing"
    echo "  help: regenerate the marker block — bash lifecycle-kit/bin/install-lifecycle.sh — which writes one 'merge=iteration-scoped' line per boundary-truncated surface (LIFECYCLE_KIT_STATE_FILE, LIFECYCLE_KIT_LESSON_EVIDENCE_FILE, and each LIFECYCLE_KIT_BOUNDARY_TRUNCATE member) and one 'merge=union' line per union surface (LIFECYCLE_KIT_GAP_INBOX_FILE). Remove any hand-added merge=iteration-scoped attribute on a path outside the supersede set."
    exit 1
fi

dcount="$(grep -vc '^$' <<<"$derived")"
ucount="$(grep -vc '^$' <<<"$union_derived")"
echo "MERGE-ATTRS: clean ($ATTRS carries a merge=iteration-scoped line for each of the $dcount derived iteration-scoped surface(s) and no others, and a merge=union line for each of the $ucount union surface(s))"
exit 0
