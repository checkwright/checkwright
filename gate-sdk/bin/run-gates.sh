#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §run-gates — aggregate gate runner; runs every gates.list member in one shot
#
# usage: run-gates.sh [gates-dir]
#   gates-dir defaults to $GATE_SDK_GATES_DIR (default: scripts), relative to
#   the repo root. Members resolve against gates-dir first, then each vendored
#   kit's checks/ directory (gate_kit_roots). Per-gate + total timings land in
#   $GATE_SDK_TMP_DIR/gate-timings.txt (default: .tmp/) — a measurement, not
#   state; never committed.
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

REPO_ROOT="$(git rev-parse --show-toplevel)" || {
    echo "run-gates: not inside a git repository" >&2
    exit 2
}
cd "$REPO_ROOT" || exit 2

GATES_DIR="${1:-$(gate_sdk_gates_dir)}"
LIST="$GATES_DIR/gates.list"
[[ -f "$LIST" ]] || { echo "run-gates: no registry at $LIST" >&2; exit 2; }

mapfile -t MEMBERS < <(gates_list_members "$LIST")
[[ ${#MEMBERS[@]} -gt 0 ]] || { echo "run-gates: $LIST names no gates" >&2; exit 2; }

RESOLVE_DIRS=("$GATES_DIR")
while IFS= read -r k; do RESOLVE_DIRS+=("$k/checks"); done < <(gate_kit_roots)

failed=()
TIMINGS="${GATE_SDK_TMP_DIR:-.tmp}/gate-timings.txt"
mkdir -p "$(dirname "$TIMINGS")" && : > "$TIMINGS"
total_ms=0
for c in "${MEMBERS[@]}"; do
    printf '\n===== %s =====\n' "$c"
    start_ns=$(date +%s%N)
    if gate_path="$(gate_resolve "$c" "${RESOLVE_DIRS[@]}")"; then
        if "$gate_path"; then
            printf '  PASS: %s\n' "$c"
        else
            rc=$?
            printf '  FAIL: %s (exit %d)\n' "$c" "$rc"
            failed+=("$c")
        fi
    else
        printf '  FAIL: %s (listed in %s but resolves in none of: %s)\n' \
            "$c" "$LIST" "${RESOLVE_DIRS[*]}"
        failed+=("$c")
    fi
    elapsed_ms=$(( ($(date +%s%N) - start_ns) / 1000000 ))
    printf '%s %d\n' "$c" "$elapsed_ms" >> "$TIMINGS"
    total_ms=$(( total_ms + elapsed_ms ))
done
printf 'TOTAL %d\n' "$total_ms" >> "$TIMINGS"

printf '\n===== gates summary =====\n'
if [[ ${#failed[@]} -eq 0 ]]; then
    printf 'All %d gates passed.\n' "${#MEMBERS[@]}"
    exit 0
fi
printf '%d of %d gates FAILED: %s\n' "${#failed[@]}" "${#MEMBERS[@]}" "${failed[*]}"
exit 1
