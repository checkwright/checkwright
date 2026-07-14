#!/usr/bin/env bash
# graph: couples=kit:checks/*.sh,kit:kpis/*.sh,kit:bin/*.sh,scripts/check-*.sh,scripts/kpi-*.sh dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-exec-bit — every tracked *.sh matching an exec-glob carries git index mode 100755, or a by-path-invoked kit script degrades silently to a skipped check / failed plugin in a fresh clone
#
# usage: check-exec-bit.sh [ls-files-dump]
#   No argument: reads `git ls-files -s` from the repo root (the index mode a
#   clone gets). With an argument: lints that canned dump (hermetic fixtures).
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

if [[ -n "${GATE_SDK_EXEC_GLOBS:-}" ]]; then
    read -r -a EXEC_GLOBS <<<"$GATE_SDK_EXEC_GLOBS"
else
    gd="$(gate_sdk_gates_dir)"
    EXEC_GLOBS=('*/checks/*.sh' '*/kpis/*.sh' '*/bin/*.sh' "$gd/check-*.sh" "$gd/kpi-*.sh")
fi
if [[ -n "${GATE_SDK_EXEC_PRUNE:-}" ]]; then
    read -r -a EXEC_PRUNE <<<"$GATE_SDK_EXEC_PRUNE"
else
    EXEC_PRUNE=(gate-tests fixtures templates smoke)
fi

if [[ $# -gt 0 ]]; then
    [[ -r "$1" ]] || { echo "check-exec-bit: ls-files dump not readable: $1" >&2; exit 2; }
    listing="$(cat -- "$1")"; st=$?
    fail_closed "$st" EXEC-BIT cat
else
    git rev-parse --git-dir >/dev/null 2>&1 || {
        echo "check-exec-bit: not a git repository — cannot read index modes" >&2; exit 2; }
    listing="$(git ls-files -s)"; st=$?
    fail_closed "$st" EXEC-BIT git-ls-files
fi

is_pruned() {
    local p="$1" seg pr
    local -a segs
    IFS='/' read -ra segs <<<"$p"
    for seg in "${segs[@]}"; do
        for pr in "${EXEC_PRUNE[@]}"; do
            [[ "$seg" == "$pr" ]] && return 0
        done
    done
    return 1
}

bad=(); count=0
while IFS= read -r line; do
    [[ -n "$line" ]] || continue
    mode="${line%% *}"
    path="${line#*$'\t'}"
    [[ "$path" == *.sh ]] || continue
    matched=0
    for g in "${EXEC_GLOBS[@]}"; do
        # shellcheck disable=SC2053  # g is the glob, deliberately unquoted
        [[ "$path" == $g ]] && { matched=1; break; }
    done
    [[ "$matched" == 1 ]] || continue
    is_pruned "$path" && continue
    count=$((count + 1))
    [[ "$mode" == 100755 ]] || bad+=("$path (index mode $mode)")
done <<< "$listing"

if [[ ${#bad[@]} -gt 0 ]]; then
    echo "check-exec-bit: by-path-invoked script(s) not committed executable (mode 100755) — a"
    echo "100644 script degrades silently in a fresh clone (a KPI plugin to 'n/a (plugin"
    echo "failed)', a runner-invoked preflight to a skipped check):"
    printf '  %s\n' "${bad[@]}"
    echo "  help: git update-index --chmod=+x <path> (and chmod +x locally), then recommit."
    exit 1
fi

echo "EXEC-BIT: clean ($count by-path-invoked script(s) at index mode 100755)"
exit 0
