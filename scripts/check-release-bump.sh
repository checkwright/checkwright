#!/usr/bin/env bash
# graph: couples=docs/posts/*.md,docs/install.md,.workflow/release-disposition.txt dir=one valve=none tier=precommit
# spec: docs/install.md §Versioning — the derivable bump floor: a release note declaring tightened gates or renamed knobs, or inheriting an outstanding deferred release's floor, may not ride a patch-only bump over its predecessor
#
# usage: check-release-bump.sh [posts-dir [disposition-file]]
#   Parses the `release:` front-matter keys under the posts dir (default
#   docs/posts), orders the versions, and asserts the floor on the newest note.
set -uo pipefail

SDK="${GATE_SDK_ROOT:-"${BASH_SOURCE[0]%/*}/../gate-sdk"}"
# shellcheck source=../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

POSTS_DIR="${1:-docs/posts}"
DISPOSITION_FILE="${2:-.workflow/release-disposition.txt}"
[[ -d "$POSTS_DIR" ]] || { echo "check-release-bump: posts dir not found: $POSTS_DIR" >&2; exit 2; }

rows=()
shopt -s nullglob
for f in "$POSTS_DIR"/*.md; do
    v="$(awk '/^---[[:space:]]*$/ { fm++; next } fm == 1 && /^release:/ { sub(/^release:[[:space:]]*/, ""); print; exit }' "$f")"; st=$?
    fail_closed "$st" RELEASE-BUMP awk
    [[ -n "$v" ]] && rows+=("${v#v}"$'\t'"$f")
done
shopt -u nullglob

# spec: lifecycle-kit/SPEC.md §templates/skills/ — history ∪ live, the reader every
# truncated evidence file needs.
collect_dispositions() {
    git log --reverse --format='%H' -p -U0 -- "$DISPOSITION_FILE" 2>/dev/null \
        | sed -n -E 's/^\+([a-z0-9][a-z0-9-]* release .*)$/\1/p'
    [[ -f "$DISPOSITION_FILE" ]] && grep -E '^[a-z0-9][a-z0-9-]* release ' "$DISPOSITION_FILE"
    return 0
}

# spec: lifecycle-kit/SPEC.md §templates/skills/ — a deferral is outstanding
# until a disposition line releases at or above its version; nothing tracks discharge.
deferred=(); released=()
while read -r _iter _kw value _rest; do
    case "$value" in
        deferred:v*) deferred+=("${value#deferred:v}") ;;
        v*)          released+=("${value#v}") ;;
    esac
done < <(collect_dispositions | sort -u)

deferred_floor=""
for d in "${deferred[@]:-}"; do
    [[ -n "$d" ]] || continue
    discharged=0
    for r in "${released[@]:-}"; do
        [[ -n "$r" ]] || continue
        [[ "$(printf '%s\n%s\n' "$d" "$r" | sort -V | tail -n1)" == "$r" ]] && discharged=1
    done
    [[ "$discharged" -eq 1 ]] && continue
    if [[ -z "$deferred_floor" || "$(printf '%s\n%s\n' "$deferred_floor" "$d" | sort -V | tail -n1)" == "$d" ]]; then
        deferred_floor="$d"
    fi
done

if [[ ${#rows[@]} -lt 2 ]]; then
    if [[ -n "$deferred_floor" ]]; then
        echo "check-release-bump: an outstanding deferred release (v$deferred_floor, $DISPOSITION_FILE) floors the newest note, and a single-note tree cannot ride it out:"
        echo "  help: cut the note at v$deferred_floor or above, or discharge the deferral with a disposition line releasing at or above it."
        exit 1
    fi
    echo "RELEASE-BUMP: clean (${#rows[@]} release note(s) under $POSTS_DIR — no predecessor to derive a floor against)"
    exit 0
fi

sorted="$(printf '%s\n' "${rows[@]}" | sort -V)"; st=$?
fail_closed "$st" RELEASE-BUMP sort
newest="$(tail -n1 <<<"$sorted")"
prev="$(tail -n2 <<<"$sorted" | head -n1)"
newest_v="${newest%%$'\t'*}"; newest_f="${newest#*$'\t'}"
prev_v="${prev%%$'\t'*}"

# spec: docs/install.md §The upgrade contract — all three fixed sections must be present; non-empty = at least one bullet
section_bullets() {  # $1=file  $2=section name; emits the bullet count, status 1 when the section is absent
    awk -v sec="$2" '
        /^## / { insec = (substr($0, 4) == sec); if (insec) found = 1; next }
        insec && /^- / { n++ }
        END { if (!found) exit 1; print n + 0 }
    ' "$1"
}
tg="$(section_bullets "$newest_f" "Tightened gates")" \
    || { echo "check-release-bump: newest note $newest_f has no 'Tightened gates' section — the floor cannot be derived (docs/install.md §The upgrade contract owns the note grammar)" >&2; exit 2; }
rk="$(section_bullets "$newest_f" "Renamed knobs")" \
    || { echo "check-release-bump: newest note $newest_f has no 'Renamed knobs' section — the floor cannot be derived (docs/install.md §The upgrade contract owns the note grammar)" >&2; exit 2; }
bc="$(section_bullets "$newest_f" "Behavior changes")" \
    || { echo "check-release-bump: newest note $newest_f has no 'Behavior changes' section — the floor cannot be derived (docs/install.md §The upgrade contract owns the note grammar)" >&2; exit 2; }

IFS=. read -r nmaj nmin _ <<<"$newest_v"
IFS=. read -r pmaj pmin _ <<<"$prev_v"
patch_only=0
[[ "$nmaj" == "$pmaj" && "$nmin" == "$pmin" ]] && patch_only=1

if [[ "$patch_only" -eq 1 && ( "$tg" -gt 0 || "$rk" -gt 0 || "$bc" -gt 0 || -n "$deferred_floor" ) ]]; then
    echo "check-release-bump: v$newest_v is a patch-only bump over v$prev_v, but its note carries phase-B work (docs/install.md §Versioning — the floor is minor):"
    [[ "$tg" -gt 0 ]] && echo "  $newest_f: $tg tightened-gate bullet(s)"
    [[ "$rk" -gt 0 ]] && echo "  $newest_f: $rk renamed-knob bullet(s)"
    [[ "$bc" -gt 0 ]] && echo "  $newest_f: $bc behavior-change bullet(s)"
    [[ -n "$deferred_floor" ]] && echo "  $DISPOSITION_FILE: an outstanding deferred release (v$deferred_floor) whose unconsumed criteria this note inherits"
    echo "  help: bump the minor instead (re-key the note's 'release:' and re-tag the plan), or move the declared work out of this release's note."
    exit 1
fi

echo "RELEASE-BUMP: clean (newest note v$newest_v holds the derivable floor over v$prev_v${deferred_floor:+, inheriting outstanding deferral v$deferred_floor}; ${#rows[@]} note(s))"
exit 0
