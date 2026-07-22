#!/usr/bin/env bash
# graph: couples=.workflow/*,kit:SPEC.md,.claude/commands/*.md,.gitignore dir=one valve=none tier=precommit
# spec: lifecycle-kit/SPEC.md §check-close-surfaces — the derived close-surface roster is complete and moded: no undeclared capture surface, every declaration carries a mode with a well-formed forced= citation, every capture-tier declaration names a reclaim command
#
# usage: check-close-surfaces.sh [scan-root]   (derives through bin/close-surfaces.sh)
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

declare -a ROOTARG=()
if [[ -n "${1:-}" ]]; then
    cd "$1" || { echo "check-close-surfaces: not a directory: $1" >&2; exit 2; }
    ROOTARG=(.)
fi

roster="$(bash "$KIT/bin/close-surfaces.sh" ${ROOTARG[@]+"${ROOTARG[@]}"})"; st=$?
fail_closed "$st" CLOSE-SURFACES "bin/close-surfaces.sh"

WF="${GATE_SDK_WORKFLOW_DIR:-.workflow}"
FORCED_RE='^forced=[A-Za-z0-9._/-]+\.md[[:space:]]+§[^[:space:]]'

errors=()
declarations=0
captures=0
while IFS=$'\t' read -r path mode reclaim owner; do
    [[ -n "$path" ]] || continue

    # assertion A: no undeclared capture surface
    if [[ "$mode" == "(undeclared)" ]]; then
        captures=$((captures + 1))
        errors+=("$path: capture-tier workflow member with no 'close-surface:' declaration — close would read it only by luck")
        continue
    fi
    declarations=$((declarations + 1))

    # assertion B: every declaration carries a mode; a forced= citation is well-formed
    if [[ -z "$mode" ]]; then
        errors+=("$owner: 'close-surface: $path' carries no mode — say 'advisory' or 'forced=<owner-path>.md §<section>'")
    elif [[ "$mode" != "advisory" ]]; then
        if [[ ! "$mode" =~ $FORCED_RE ]]; then
            errors+=("$owner: 'close-surface: $path' mode is neither 'advisory' nor a well-formed 'forced=<owner-path>.md §<section>': $mode")
        fi
    fi

    # assertion C: a capture-tier declaration names its reclaim command
    if [[ "$path" == "$WF"/* ]] && git check-ignore -q -- "$path" 2>/dev/null; then
        captures=$((captures + 1))
        [[ "$reclaim" == "-" || -z "$reclaim" ]] \
            && errors+=("$owner: 'close-surface: $path' is capture-tier (gitignored) and names no reclaim= command")
    fi
done <<<"$roster"

if [[ ${#errors[@]} -gt 0 ]]; then
    echo "check-close-surfaces: ${#errors[@]} close-surface roster violation(s):"
    printf '  %s\n' "${errors[@]}"
    echo "  help: declare the surface with a full-line 'close-surface: <path> <mode> [reclaim=<command>]' directive in the SPEC section that already owns it — never a central list. <mode> is 'advisory' (no forcing function; a skip is a visible judgment) or 'forced=<owner-path>.md §<section>' naming the structural forcing function. A gitignored capture surface names the drain that empties it as the trailing reclaim=<command>."
    exit 1
fi
echo "CLOSE-SURFACES: clean ($declarations declared surface(s), $captures capture-tier; every capture member declared, every declaration moded, every capture-tier declaration reclaimed)"
exit 0
