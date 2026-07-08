#!/usr/bin/env bash
# graph: couples=scripts/gates.list,kit:checks/*.sh dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-kit-enum — a literal hand list of >=2 kit roots sharing a glob must name every kit root with matching tracked files; the fix is the kit:<glob> token, not a longer list
#
# usage: check-kit-enum.sh [gates-dir]
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

GATES_DIR="${1:-$(gate_sdk_gates_dir)}"
LIST="$GATES_DIR/gates.list"
[[ -f "$LIST" ]] || { echo "check-kit-enum: no registry at $LIST" >&2; exit 2; }

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" \
    || { echo "check-kit-enum: not a git repository — cannot test tracked kit files" >&2; exit 2; }

mapfile -t KIT_ROOTS < <(gate_kit_roots_rel)
[[ ${#KIT_ROOTS[@]} -gt 0 ]] || { echo "check-kit-enum: no kit roots enumerated" >&2; exit 2; }

RESOLVE_DIRS=("$GATES_DIR")
while IFS= read -r k; do RESOLVE_DIRS+=("$k/checks"); done < <(gate_kit_roots)

is_kit_root() {
    local c="$1" r
    for r in "${KIT_ROOTS[@]}"; do [[ "$c" == "${r%/}" ]] && return 0; done
    return 1
}

root_has_glob() {
    local root="$1" glob="$2" out st
    out="$(git -C "$REPO_ROOT" ls-files -- "${root%/}/$glob" 2>/dev/null)"; st=$?
    fail_closed "$st" check-kit-enum "git ls-files"
    [[ -n "$out" ]]
}

members="$(gates_list_members "$LIST")"
[[ -n "$members" ]] || { echo "check-kit-enum: no members parsed from $LIST" >&2; exit 2; }

violations=()
groups_checked=0
while IFS= read -r m; do
    [[ -n "$m" ]] || continue
    if ! src="$(gate_resolve "$m" "${RESOLVE_DIRS[@]}")"; then
        echo "check-kit-enum: $m in $LIST resolves in none of: ${RESOLVE_DIRS[*]}" >&2
        exit 2
    fi
    man="$(grep -m1 '^# graph: ' "$src" 2>/dev/null || true)"
    [[ -n "$man" ]] || continue  # no manifest is check-graph's finding, not this gate's

    couples=""
    for kv in ${man#\# graph: }; do
        [[ "$kv" == couples=* ]] && { couples="${kv#couples=}"; break; }
    done
    [[ -n "$couples" ]] || continue

    declare -A named=()
    IFS=',' read -ra toks <<<"$couples"
    for t in "${toks[@]}"; do
        [[ "$t" == */* ]] || continue
        root="${t%%/*}"; glob="${t#*/}"
        is_kit_root "$root" || continue
        named["$glob"]+=" $root"
    done

    for glob in "${!named[@]}"; do
        read -ra have <<<"${named[$glob]}"
        [[ ${#have[@]} -ge 2 ]] || continue  # a single named root is not a hand list
        groups_checked=$((groups_checked + 1))
        missing=()
        for r in "${KIT_ROOTS[@]}"; do
            r="${r%/}"
            root_has_glob "$r" "$glob" || continue
            found=0
            for h in "${have[@]}"; do [[ "$h" == "$r" ]] && { found=1; break; }; done
            [[ "$found" -eq 0 ]] && missing+=("$r")
        done
        [[ ${#missing[@]} -gt 0 ]] \
            && violations+=("$m couples a '$glob' hand list naming [${have[*]}] but omits [${missing[*]}]")
    done
    unset named
done <<< "$members"

if [[ ${#violations[@]} -gt 0 ]]; then
    echo "check-kit-enum: gate(s) hand-list kit roots incompletely — the kit set drifted:"
    for v in "${violations[@]}"; do echo "  $v"; done
    echo "  help: replace the per-kit hand list with the 'kit:<glob>' couples token"
    echo "        (lib/gate.sh expands it to every gate_kit_roots member), so a kit"
    echo "        added later cannot silently fall out of the coupling."
    exit 1
fi
echo "KIT-ENUM: clean (${groups_checked} multi-kit hand-list group(s) complete; kit:<glob> keeps them so)"
exit 0
