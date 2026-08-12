#!/usr/bin/env bash
# spec: canon-kit/SPEC.md §check-measured-claim — this repo's measured-claim oracle: one <key><TAB><value> line per fact a governed sentence is allowed to state as a number, each recomputed off the tree so the sentence that cites it cannot go stale silently. A key joins here before a marker names it; a marker naming a key absent from this roster fails the gate closed.
set -uo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../gate-sdk/lib/gate.sh
source "$REPO/gate-sdk/lib/gate.sh"
cd "$REPO" || exit 2

# spec: canon-kit/SPEC.md §check-measured-claim — how much of the battery dispatches to the
# binary, read through the registry and the resolution order the battery itself runs on
# rather than by counting descriptors, so a consumer's `.sh` shadow is respected
mapfile -t _mc_members < <(gates_list_members "$(gate_sdk_gates_dir)/gates.list")
(( ${#_mc_members[@]} > 0 )) || { echo "measured-claims: the gate registry lists no member" >&2; exit 2; }
mapfile -t _mc_dirs < <(gate_check_dirs)
_mc_ported=0
for _mc_m in "${_mc_members[@]}"; do
    _mc_src="$(gate_resolve "$_mc_m" "${_mc_dirs[@]}")" \
        || { echo "measured-claims: registered gate $_mc_m resolves to no declaration path" >&2; exit 2; }
    [[ "$_mc_src" == *.gate ]] && _mc_ported=$((_mc_ported + 1))
done
printf 'ported-gate-members\t%s\n' "$_mc_ported"
