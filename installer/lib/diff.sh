#!/usr/bin/env bash
# spec: installer/README.md §diff — classifies every files[] entry against the tree using lock_hash, the same comparison claim() makes: unchanged is counted only, changed and missing are each named apart because a deletion and an edit have different remedies. Exit status is the verdict: 0 every recorded entry matches, 1 at least one is changed or missing
#
# usage: checkwright diff
#   No --dry-run: diff writes nothing, so it has no mutating form to guard.
set -uo pipefail

INSTALLER="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=./common/lock.sh
source "$INSTALLER/lib/common/lock.sh"

case "${1:-}" in
    '') ;;
    -h|--help)
        printf 'usage: checkwright diff\n\n'
        printf 'Compares the tree against what init recorded in %s, using the same\n' "$CHECKWRIGHT_LOCK_FILE"
        printf 'hash comparison claim() makes. Exit status is the verdict: 0 every\n'
        printf 'recorded file matches, 1 at least one has changed or gone missing.\n'
        exit 0 ;;
    *) printf 'checkwright diff: unknown argument: %s\n' "$1" >&2; exit 2 ;;
esac

die() { printf 'checkwright diff: %s\n' "$1" >&2; [[ -n "${2:-}" ]] && printf '  help: %s\n' "$2" >&2; exit "${3:-2}"; }

# spec: installer/README.md §init — every precondition refuses rather than warns, and is checked before anything is compared: diff's subject is exactly the roster init recorded, so it needs the repository that roster is in
ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" \
    || die "not inside a git work tree" \
       "diff compares the tree against the manifest init committed there, so it needs the repository that manifest is in."

LOCK="$(lock_path "$ROOT")"
[[ -f "$LOCK" ]] || die "no $CHECKWRIGHT_LOCK_FILE at $ROOT" \
    "init is the verb that makes an install, and the manifest it writes is what diff compares against. Without one there is nothing here to compare."
lock_schema_ok "$LOCK" || die "$CHECKWRIGHT_LOCK_FILE carries a schema this build does not know" \
    "this manifest was written by a different Checkwright release. Upgrade the installer rather than letting it guess at a shape it was not built for."

# spec: installer/README.md §diff — a recorded path already off the tree is reported apart from one whose content differs: init's roster exit rule means the next init would silently drop a missing path and rewrite it fresh, which is worth a warning before it happens rather than after
CHANGED=(); MISSING=(); SAME=0
while IFS=$'\t' read -r p h; do
    [[ -n "$p" ]] || continue
    if [[ ! -f "$ROOT/$p" ]]; then
        MISSING+=("$p")
    elif [[ "$(lock_hash "$ROOT/$p")" == "$h" ]]; then
        SAME=$(( SAME + 1 ))
    else
        CHANGED+=("$p")
    fi
done < <(jq -r 'if (.files | type) == "object" then (.files | to_entries[] | "\(.key)\t\(.value)") else empty end' "$LOCK")

printf 'checking %s against the tree\n' "$CHECKWRIGHT_LOCK_FILE"
if [[ ${#CHANGED[@]} -gt 0 ]]; then
    printf '\nchanged (%d) — content differs from what init wrote:\n' "${#CHANGED[@]}"
    printf '  %s\n' "${CHANGED[@]}"
fi
if [[ ${#MISSING[@]} -gt 0 ]]; then
    printf '\nmissing (%d) — recorded by init but no longer on disk; the next init will silently drop these from the roster and rewrite them fresh:\n' "${#MISSING[@]}"
    printf '  %s\n' "${MISSING[@]}"
fi

if [[ ${#CHANGED[@]} -eq 0 && ${#MISSING[@]} -eq 0 ]]; then
    printf '\nDIFF: clean — %d file(s) match what init wrote.\n' "$SAME"
    exit 0
fi
printf '\nDIFF: %d changed, %d missing, %d unchanged\n' "${#CHANGED[@]}" "${#MISSING[@]}" "$SAME"
exit 1
