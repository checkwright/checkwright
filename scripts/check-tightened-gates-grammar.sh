#!/usr/bin/env bash
# graph: couples=docs/posts/*.md,docs/install.md dir=one valve=none tier=precommit
# spec: docs/install.md §The upgrade contract — every release note's Tightened-gates section resolves to an explicit `None` or to a non-empty set of backticked bare gate names; a non-`none` section yielding no tokens is the silently-empty declaration the smoke cannot see
#
# usage: check-tightened-gates-grammar.sh [posts-dir]   (default docs/posts)
set -uo pipefail

SDK="${GATE_SDK_ROOT:-"${BASH_SOURCE[0]%/*}/../gate-sdk"}"
# shellcheck source=../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../gate-sdk/lib/declaration.sh
source "$SDK/lib/declaration.sh"

POSTS_DIR="${1:-docs/posts}"
SECTION="Tightened gates"
[[ -d "$POSTS_DIR" ]] || { echo "check-tightened-gates-grammar: posts dir not found: $POSTS_DIR" >&2; exit 2; }

errors=()
notes=0
tokens=0
none=0

shopt -s nullglob
for f in "$POSTS_DIR"/*.md; do
    grep -qE '^release:[[:space:]]+v' "$f" || continue
    notes=$((notes + 1))
    out="$(decl_section_tokens "$f" "$SECTION")"; st=$?
    case "$st" in
        0)
            if [[ -z "$out" ]]; then none=$((none + 1)); else
                tokens=$((tokens + $(grep -c . <<<"$out")))
            fi
            ;;
        1)
            if [[ -z "$out" ]]; then
                errors+=("$f: '$SECTION' is not \`None\` and yields no lead token at all — the parse resolves to an empty allowed-red set the note contradicts")
            else
                while IFS= read -r line; do
                    errors+=("$f: '$SECTION' bullet's lead token is unreadable: ${line:0:72}")
                done <<<"$out"
            fi
            ;;
        *)
            errors+=("$f: no '$SECTION' section — every release note carries the fixed sections its note grammar rosters")
            ;;
    esac
done
shopt -u nullglob

if [[ ${#errors[@]} -gt 0 ]]; then
    echo "check-tightened-gates-grammar: ${#errors[@]} unreadable tightened-gates declaration(s):"
    printf '  %s\n' "${errors[@]}"
    echo "  help: a Tightened-gates bullet's lead token is a backticked, unbolded bare gate name directly after the bullet marker (- \`check-foo\` — …); strip any bold emphasis and add the backticks. A release that tightened nothing states a bare \"None.\" body instead. docs/install.md §The upgrade contract owns the grammar; a mechanical consumer reads these tokens as the release's allowed-red set, so a section that parses to nothing disarms it silently."
    exit 1
fi
echo "TIGHTENED-GATES-GRAMMAR: clean ($notes release note(s) under $POSTS_DIR; $none declare \`None\`, the rest resolve $tokens lead token(s))"
exit 0
