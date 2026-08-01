#!/usr/bin/env bash
# graph: couples=kit:templates/*.list,kit:*/*.sh dir=bi valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-template-registry-parity — a kit's shipped `.list` registry template names exactly the artifacts of its sibling directory, both directions
#
# usage: check-template-registry-parity.sh [root]
#   bare: sweep gate_kit_roots against the git toplevel; root: resolve relative
#   kit roots against a fixture tree (the case dir's gate-sdk-config.sh names them).
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

ROOT="${1:-}"
if [[ -z "$ROOT" ]]; then
    ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" \
        || { echo "check-template-registry-parity: not a git repository and no root argument" >&2; exit 2; }
fi
[[ -d "$ROOT" ]] || { echo "check-template-registry-parity: root not found: $ROOT" >&2; exit 2; }

mapfile -t KIT_ROOTS < <(gate_kit_roots)
[[ ${#KIT_ROOTS[@]} -gt 0 ]] || { echo "check-template-registry-parity: no kit roots enumerated" >&2; exit 2; }

findings=()
registries=0
skipped=0

for r in "${KIT_ROOTS[@]}"; do
    r="${r%/}"
    abs="$r"
    [[ "$abs" == /* ]] || abs="$ROOT/$r"
    kit="${r##*/}"

    shopt -s nullglob
    templates=("$abs"/templates/*.list)
    shopt -u nullglob

    for tpl in "${templates[@]}"; do
        base="${tpl##*/}"
        dir="$abs/${base%.list}"
        # spec: gate-sdk/SPEC.md §check-template-registry-parity — the population predicate: a `.list` template enters only beside a sibling directory of kit-shipped artifacts, so a template of consumer rule content is skipped-and-counted by construction rather than by an exception naming it
        [[ -d "$dir" ]] || { skipped=$((skipped + 1)); continue; }
        [[ -r "$tpl" ]] || { echo "check-template-registry-parity: template not readable: $tpl" >&2; exit 2; }
        [[ -r "$dir" && -x "$dir" ]] || { echo "check-template-registry-parity: sibling directory not readable: $dir" >&2; exit 2; }
        registries=$((registries + 1))
        rel_tpl="${tpl#"$ROOT"/}"

        listing="$(git -C "$dir" ls-files -- '*.sh')"; st=$?
        [[ "$st" -eq 0 ]] || { echo "check-template-registry-parity: git ls-files failed under $dir" >&2; exit 2; }
        shipped="$(
            while IFS= read -r f; do
                [[ -n "$f" && "$f" != */* ]] || continue
                printf '%s\n' "${f%.sh}"
            done <<<"$listing" | sort -u
        )"
        registered="$(gates_list_members "$tpl" | sort -u)"

        # assertion A: every shipped artifact is registered
        while IFS= read -r m; do
            [[ -n "$m" ]] && findings+=("$kit: $rel_tpl does not register shipped artifact: $m")
        done < <(comm -23 <(printf '%s\n' "$shipped") <(printf '%s\n' "$registered"))

        # assertion B: every registry line resolves to a shipped artifact
        while IFS= read -r m; do
            [[ -n "$m" ]] && findings+=("$kit: $rel_tpl registers a name no shipped artifact answers: $m")
        done < <(comm -13 <(printf '%s\n' "$shipped") <(printf '%s\n' "$registered"))
    done
done

if [[ ${#findings[@]} -gt 0 ]]; then
    echo "check-template-registry-parity: a kit's shipped registry template is out of parity with the directory it registers:"
    printf '  %s\n' "${findings[@]}"
    echo "  help: a shipped registry template names the kit's whole bundled set — add the"
    echo "        missing line (the consumer prunes its own copy, the kit ships all of it),"
    echo "        or drop the line whose artifact the kit no longer ships. An untracked file"
    echo "        is not shipped and forces nothing: commit it first."
    exit 1
fi

echo "TEMPLATE-REGISTRY-PARITY: clean ($registries shipped registry template(s) in name-set parity with the sibling directory each registers; $skipped .list template(s) with no such sibling skipped)"
exit 0
