#!/usr/bin/env bash
# spec: canon-kit/SPEC.md §check-measured-claim — this repo's measured-claim oracle: one <key><TAB><value> line per fact a governed sentence is allowed to state, recomputed off the tree so the sentence that cites it cannot go stale silently. A value is a cardinal or an extent, since the sentences that most need an oracle carry no number. A key joins here before a marker names it; a marker naming a key absent from this roster fails the gate closed.
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
_mc_shell=0
for _mc_m in "${_mc_members[@]}"; do
    _mc_src="$(gate_resolve "$_mc_m" "${_mc_dirs[@]}")" \
        || { echo "measured-claims: registered gate $_mc_m resolves to no declaration path" >&2; exit 2; }
    if [[ "$_mc_src" == *.gate ]]; then _mc_ported=$((_mc_ported + 1)); else _mc_shell=$((_mc_shell + 1)); fi
done
printf 'ported-gate-members\t%s\n' "$_mc_ported"

# spec: canon-kit/SPEC.md §check-measured-claim — the live substrate set the enforcement core
# runs on, as an extent rather than a count, so a sentence carrying no cardinal is still
# self-correcting
_mc_sub=""
(( _mc_ported > 0 )) && _mc_sub="native"
(( _mc_shell > 0 )) && _mc_sub="${_mc_sub:+$_mc_sub+}shell"
[[ -n "$_mc_sub" ]] || { echo "measured-claims: the registry resolved no member to either substrate" >&2; exit 2; }
printf 'gate-substrates\t%s\n' "$_mc_sub"

# spec: canon-kit/SPEC.md §check-measured-claim — the directive's completion predicate as a number,
# read off port-blockers' --tree trailer rather than re-derived, and deliberately not the
# ported-gate-members key widened (gate-sdk/SPEC.md §port-blockers)
_mc_tree="$(bash "$REPO/gate-sdk/bin/port-blockers.sh" --tree | tail -1)"
_mc_owed="${_mc_tree##*, }"
_mc_owed="${_mc_owed% owed}"
[[ "$_mc_owed" =~ ^[0-9]+$ ]] || { echo "measured-claims: port-blockers --tree did not report an owed count: $_mc_tree" >&2; exit 2; }
printf 'tree-shell-owed\t%s\n' "$_mc_owed"
