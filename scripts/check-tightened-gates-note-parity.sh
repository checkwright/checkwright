#!/usr/bin/env bash
# graph: couples=docs/posts/*.md,.workflow/tightened-gates.txt dir=one valve=none tier=precommit
# spec: docs/install.md §The upgrade contract — while a release note is under composition its Tightened-gates token set equals the declaration surface it was composed from, both directions
#
# usage: check-tightened-gates-note-parity.sh [posts-dir [declaration-file]]
set -uo pipefail

SDK="${GATE_SDK_ROOT:-"${BASH_SOURCE[0]%/*}/../gate-sdk"}"
# shellcheck source=../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../gate-sdk/lib/declaration.sh
source "$SDK/lib/declaration.sh"

POSTS_DIR="${1:-docs/posts}"
DECL_FILE="${2:-.workflow/tightened-gates.txt}"
[[ -d "$POSTS_DIR" ]] || { echo "check-tightened-gates-note-parity: posts dir not found: $POSTS_DIR" >&2; exit 2; }

# spec: gate-sdk/SPEC.md §lib/declaration.sh — the note set is docs/posts/ filtered by the release: key; the announcement post carries no front matter and is not a note
untagged=()
shopt -s nullglob
for f in "$POSTS_DIR"/*.md; do
    v="$(awk '/^---[[:space:]]*$/ { fm++; next } fm == 1 && /^release:/ { sub(/^release:[[:space:]]*/, ""); print; exit }' "$f")"; st=$?
    fail_closed "$st" TIGHTENED-GATES-NOTE-PARITY awk
    [[ -n "$v" ]] || continue
    git rev-parse -q --verify "refs/tags/${v}" >/dev/null 2>&1 || untagged+=("${v}"$'\t'"$f")
done
shopt -u nullglob

if [[ "${#untagged[@]}" -gt 1 ]]; then
    echo "check-tightened-gates-note-parity: $POSTS_DIR carries more than one untagged release note, a state the release choreography does not admit:" >&2
    printf '  %s\n' "${untagged[@]}" >&2
    echo "  help: exactly one note is in flight at a time — tag the released one or remove the stray note." >&2
    exit 2
fi

if [[ "${#untagged[@]}" -eq 0 ]]; then
    echo "TIGHTENED-GATES-NOTE-PARITY: dormant (every release note under $POSTS_DIR is tagged, so the surface has been drained by contract and there is nothing to compare)"
    exit 0
fi

note_v="${untagged[0]%%$'\t'*}"
note_f="${untagged[0]#*$'\t'}"

head -n1 "$DECL_FILE" 2>/dev/null | grep -q '^#' \
    || { echo "check-tightened-gates-note-parity: $DECL_FILE is missing its required header line, so the declaration surface cannot be established (gate-sdk/SPEC.md §upgrade-smoke owns its contract)" >&2; exit 2; }

note_tokens="$(decl_section_tokens "$note_f" "Tightened gates")"; st=$?
if [[ "$st" -eq 2 ]]; then
    echo "check-tightened-gates-note-parity: note $note_f has no 'Tightened gates' section, so there is nothing to hold against the surface (docs/install.md §The upgrade contract owns the note grammar)" >&2
    exit 2
fi
if [[ "$st" -ne 0 ]]; then
    echo "check-tightened-gates-note-parity: note $note_f's 'Tightened gates' section does not parse, so it would compare as a silently empty set:" >&2
    [[ -n "$note_tokens" ]] && printf '  %s\n' "$note_tokens" >&2
    exit 2
fi

decl_tokens="$(decl_record_tokens "$DECL_FILE")"; st=$?
if [[ "$st" -ne 0 ]]; then
    echo "check-tightened-gates-note-parity: $DECL_FILE carries malformed data line(s), so the surface would compare as a silently wrong set:" >&2
    [[ -n "$decl_tokens" ]] && printf '  %s\n' "$decl_tokens" >&2
    exit 2
fi

only_surface="$(comm -23 <(sort -u <<<"$decl_tokens" | grep -v '^$') <(sort -u <<<"$note_tokens" | grep -v '^$'))"
only_note="$(comm -13 <(sort -u <<<"$decl_tokens" | grep -v '^$') <(sort -u <<<"$note_tokens" | grep -v '^$'))"

if [[ -n "$only_surface" || -n "$only_note" ]]; then
    echo "check-tightened-gates-note-parity: note $note_f (v${note_v#v}, under composition) and $DECL_FILE declare different gate sets:"
    if [[ -n "$only_surface" ]]; then
        echo "  on the surface, missing from the note — tightened and shipping undeclared, which licenses a red the upgrade smoke would wave through:"
        printf '    %s\n' $only_surface
    fi
    if [[ -n "$only_note" ]]; then
        echo "  in the note, missing from the surface — declares a gate that never tightened, sending consumers hunting a reconcile that does not exist:"
        printf '    %s\n' $only_note
    fi
    echo "  help: the note's Tightened-gates bullets are composed from the surface's data lines — bring the two into agreement before the drain-and-stamp commit."
    exit 1
fi

n="$(grep -c . <<<"$note_tokens")"
[[ -n "$note_tokens" ]] || n=0
echo "TIGHTENED-GATES-NOTE-PARITY: clean (note $note_f is under composition and its Tightened-gates set equals $DECL_FILE, both directions; $n token(s))"
exit 0
