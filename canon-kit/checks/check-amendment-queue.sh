#!/usr/bin/env bash
# graph: couples=TASK-QUEUE.md,SPEC-*.md,*/SPEC-*.md dir=bi valve=none tier=precommit
# spec: canon-kit/SPEC.md §check-amendment-queue — the Task↔amendment bidirectional rule and spec-readiness
#
# usage: check-amendment-queue.sh [queue-file [scan-root]]
#   Defaults to the configured queue file (CANON_KIT_QUEUE_FILE) and '.' for the
#   amendment-on-disk scan (a fixture case dir carries its own copies).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

QUEUE="${1:-$CANON_KIT_QUEUE_FILE}"
ROOT="${2:-.}"
[[ -f "$QUEUE" ]] || { echo "check-amendment-queue: file not found: $QUEUE" >&2; exit 2; }

errors=""

# (a) feature-section entries missing [spec:], and any [needs-spec] in the
#     active sections (entries, sub-bullets, or prose — a mention masks a
#     missing tag); (b) deferred entries missing [needs-spec], and a deferred
#     entry already carrying [spec:] (promote it). One awk pass over the queue.
qout="$(awk -v featre="$SPEC_FEATURE_RE" -v activere="$SPEC_ACTIVE_RE" \
            -v defre="$SPEC_DEFERRED_RE" -v sectre="$SPEC_SECTION_RE" '
    $0 ~ sectre {
        sec = "other"
        if ($0 ~ featre)        sec = "feature"
        else if ($0 ~ activere) sec = "active"
        else if ($0 ~ defre)    sec = "deferred"
        next
    }
    (sec == "feature" || sec == "active") {
        if ($0 ~ /^- /) {
            if ($0 ~ /\[needs-spec\]/)
                printf "active-needs-spec\t%d\t%s\n", FNR, $0
            else if (sec == "feature" && $0 !~ /\[spec:/)
                printf "missing-spec\t%d\t%s\n", FNR, $0
        } else if ($0 ~ /\[needs-spec\]/) {
            printf "prose-needs-spec\t%d\t%s\n", FNR, $0
        }
        next
    }
    sec == "deferred" && $0 ~ /^- / {
        if ($0 !~ /\[needs-spec\]/)
            printf "deferred-open\t%d\t%s\n", FNR, $0
        else if ($0 ~ /\[spec:/)
            printf "deferred-ready\t%d\t%s\n", FNR, $0
        next
    }
' "$QUEUE")"; st=$?
fail_closed "$st" check-amendment-queue awk

missing=(); an=(); pn=(); dopen=(); dready=()
while IFS=$'\t' read -r class ln text; do
    [[ -n "$class" ]] || continue
    case "$class" in
        missing-spec)      missing+=("$QUEUE:$ln: $text") ;;
        active-needs-spec) an+=("$QUEUE:$ln: $text") ;;
        prose-needs-spec)  pn+=("$QUEUE:$ln: $text") ;;
        deferred-open)     dopen+=("$QUEUE:$ln: $text") ;;
        deferred-ready)    dready+=("$QUEUE:$ln: $text") ;;
    esac
done <<< "$qout"

(( ${#missing[@]} )) && errors+="feature-section entries without [spec:] (spec-writing is scope-stage — write the amendment, then promote):"$'\n'"$(printf '  %s\n' "${missing[@]}")"$'\n'
(( ${#an[@]} ))      && errors+="[needs-spec] tag in an active-queue entry (move it to $CANON_KIT_DEFERRED_SECTION):"$'\n'"$(printf '  %s\n' "${an[@]}")"$'\n'
(( ${#pn[@]} ))      && errors+="[needs-spec] tag in active-queue prose ($CANON_KIT_DEFERRED_SECTION-only tag; say \"needs design\" in prose):"$'\n'"$(printf '  %s\n' "${pn[@]}")"$'\n'
(( ${#dopen[@]} ))   && errors+="$CANON_KIT_DEFERRED_SECTION entries without [needs-spec] (all deferred work is design-pending):"$'\n'"$(printf '  %s\n' "${dopen[@]}")"$'\n'
(( ${#dready[@]} ))  && errors+="$CANON_KIT_DEFERRED_SECTION entries already carrying [spec:] (promote to a feature section):"$'\n'"$(printf '  %s\n' "${dready[@]}")"$'\n'

# (c) bidirectional pairing on disk: every [spec:] ref resolves to a file, and
#     every amendment on disk is referenced by a [spec:] entry (by basename). A
#     ref with a '/' is a repo-relative path (resolved directly); a bare ref is
#     an amendment basename (searched tree-wide).
mapfile -t refs < <(grep -oE '\[spec:[[:space:]]*[^]]+\]' "$QUEUE" 2>/dev/null \
    | sed -E 's/\[spec:[[:space:]]*//; s/[[:space:]]*\]$//' | sort -u)

declare -A ref_bases=()
for r in ${refs[@]+"${refs[@]}"}; do
    [[ -n "$r" ]] || continue
    ref_bases["$(basename "$r")"]=1
    if [[ "$r" == */* ]]; then
        [[ -f "$r" ]] || errors+="queue references [spec: $r] but no such file exists at that path"$'\n'
    else
        hit="$(spec_amendments "$ROOT" | while IFS= read -r f; do [[ "$(basename "$f")" == "$r" ]] && { echo "$f"; break; }; done)"
        [[ -n "$hit" ]] || errors+="queue references [spec: $r] but no amendment file named $r exists on disk"$'\n'
    fi
done

while IFS= read -r f; do
    [[ -n "$f" ]] || continue
    base="$(basename "$f")"
    [[ -n "${ref_bases[$base]:-}" ]] \
        || errors+="amendment on disk with no queue entry: $f (expected a task tagged [spec: $base])"$'\n'
done < <(spec_amendments "$ROOT")

if [[ -n "$errors" ]]; then
    echo "check-amendment-queue: Task↔amendment bidirectional-rule violation(s):"
    echo ""
    printf '%s' "$errors"
    echo "  help: pair every amendment with a [spec: …] queue entry and vice versa; tag every $CANON_KIT_DEFERRED_SECTION entry [needs-spec]; give every feature entry a [spec:] ref"
    exit 1
fi

echo "AMENDMENT-QUEUE: clean (every amendment ↔ a queue entry; feature entries spec-ready; deferred entries design-pending)"
exit 0
