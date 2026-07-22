#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §The close-surface roster — the derivation: every close-surface: declaration, unioned with the workflow directory's capture tier so an undeclared capture surface is reported rather than missing
# usage: close-surfaces.sh [scan-root]   (default: the repo toplevel)
#   prints one tab-separated '<path> <mode> <reclaim> <owner>' row per surface,
#   sorted by path; exit 2 on an unreadable configuration or declaration surface
set -uo pipefail

ROOT="${1:-}"
KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
[[ -n "$ROOT" ]] && { cd "$ROOT" || exit 2; }
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"

if [[ -z "$ROOT" ]]; then
    REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || {
        echo "close-surfaces: not a git repository — the capture tier is underivable" >&2; exit 2; }
    cd "$REPO_ROOT" || exit 2
fi
git rev-parse --git-dir >/dev/null 2>&1 || {
    echo "close-surfaces: not a git repository — the capture tier is underivable" >&2; exit 2; }

declare -a SURFACES=()
_add_surface() {
    local f s
    f="${1#./}"
    for s in ${SURFACES[@]+"${SURFACES[@]}"}; do [[ "$s" == "$f" ]] && return 0; done
    SURFACES+=("$f")
}

while IFS= read -r r; do
    [[ -n "$r" ]] || continue
    [[ -f "${r%/}/$LIFECYCLE_KIT_ROSTER_BASENAME" ]] && _add_surface "${r%/}/$LIFECYCLE_KIT_ROSTER_BASENAME"
done < <(gate_kit_roots_rel)

shopt -s nullglob globstar
for g in ${LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS[@]+"${LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS[@]}"}; do
    for f in $g; do [[ -f "$f" ]] && _add_surface "$f"; done
done
shopt -u nullglob globstar

# spec: lifecycle-kit/SPEC.md §The close-surface roster — fenced blocks are skipped, so the directive's own grammar is quotable where it is specified (check-spec-pointer's carve-out, same reason)
read -r -d '' EXTRACT <<'AWK' || true
/^[[:space:]]*```/ { fence = !fence; next }
fence { next }
/^[[:space:]]*close-surface:[[:space:]]/ { print }
AWK

declare -a ROWS=()
declare -a DECLARED=()
for s in ${SURFACES[@]+"${SURFACES[@]}"}; do
    [[ -r "$s" ]] || { echo "close-surfaces: declaration surface not readable: $s" >&2; exit 2; }
    while IFS= read -r line; do
        [[ -n "$line" ]] || continue
        body="${line#"${line%%close-surface:*}"}"
        body="${body#close-surface:}"
        body="${body#"${body%%[![:space:]]*}"}"
        body="${body%"${body##*[![:space:]]}"}"
        reclaim="-"
        if [[ "$body" == *" reclaim="* ]]; then
            reclaim="${body#* reclaim=}"
            body="${body%% reclaim=*}"
        fi
        path="${body%%[[:space:]]*}"
        mode="${body#"$path"}"
        mode="${mode#"${mode%%[![:space:]]*}"}"
        mode="${mode%"${mode##*[![:space:]]}"}"
        ROWS+=("$path	$mode	$reclaim	$s")
        DECLARED+=("$path")
    done < <(awk "$EXTRACT" "$s")
done

WF="${GATE_SDK_WORKFLOW_DIR:-.workflow}"
if [[ -d "$WF" ]]; then
    shopt -s nullglob dotglob
    for m in "$WF"/*; do
        [[ -f "$m" ]] || continue
        git check-ignore -q -- "$m"; st=$?
        case "$st" in
            0) ;;
            1) continue ;;
            *) echo "close-surfaces: git check-ignore exited $st on $m" >&2; exit 2 ;;
        esac
        seen=0
        for d in ${DECLARED[@]+"${DECLARED[@]}"}; do [[ "$d" == "$m" ]] && { seen=1; break; }; done
        [[ "$seen" == 1 ]] || ROWS+=("$m	(undeclared)	-	-")
    done
    shopt -u nullglob dotglob
fi

[[ ${#ROWS[@]} -eq 0 ]] && exit 0
printf '%s\n' "${ROWS[@]}" | LC_ALL=C sort -t'	' -k1,1
