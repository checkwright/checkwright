#!/usr/bin/env bash
# spec: CLAUDE.md §Housekeeping — the published activation surface's entry point; npm is the delivery vehicle and bash is the implementation, so what an adopter is about to run is the one language the rest of the tree is written in
#
# usage: checkwright <verb> [args...]
#   checkwright --help    list the verbs this package carries
set -uo pipefail

INSTALLER="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# spec: CLAUDE.md §Housekeeping — the verb roster is lib/ itself, never a list beside it: a verb is advertised because its implementation is present, so the help text cannot promise a verb the package does not carry
verbs() {
    local f
    shopt -s nullglob
    for f in "$INSTALLER"/lib/*.sh; do f="${f##*/}"; printf '%s\n' "${f%.sh}"; done
    shopt -u nullglob
}

usage() {
    printf 'usage: checkwright <verb> [args...]\n\n'
    printf 'Vendors pinned Checkwright kit source into your repository and commits it.\n'
    printf 'Nothing is fetched after this package itself.\n\n'
    printf 'verbs:\n'
    local v found=0
    while IFS= read -r v; do
        [[ -n "$v" ]] || continue
        found=1
        printf '  %s\n' "$v"
    done < <(verbs)
    (( found )) || printf '  (none — this build carries no verbs)\n'
    printf '\nhttps://checkwright.dev\n'
}

VERB="${1:-}"
case "$VERB" in
    ''|-h|--help) usage; exit 0 ;;
esac
shift

IMPL="$INSTALLER/lib/$VERB.sh"
if [[ ! -f "$IMPL" ]]; then
    printf 'checkwright: unknown verb: %s\n\n' "$VERB" >&2
    usage >&2
    exit 2
fi

exec bash "$IMPL" "$@"
