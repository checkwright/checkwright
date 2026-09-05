#!/usr/bin/env bash
# spec: context-kit/SPEC.md §The always-loaded meter — standing per-session surface vs committed baseline
# usage: always-loaded.sh [--update-baseline|--growth]   (bare: total/per-part/delta; --update-baseline: rewrite it; --growth: per-file net growth of every governed prose file since the baseline commit)
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$(git rev-parse --show-toplevel 2>/dev/null || pwd)" 2>/dev/null || { echo "always-loaded: cannot enter repo root" >&2; exit 2; }

UPDATE=0; GROWTH=0
[[ "${1:-}" == "--update-baseline" ]] && UPDATE=1
[[ "${1:-}" == "--growth" ]] && GROWTH=1

# shellcheck source=../lib/context.sh
source "$KIT/lib/context.sh"

# spec: context-kit/SPEC.md §The always-loaded meter — the default resolves the battery runner's --emit front-end, consumer-first, keeping the two-candidate shape and the empty-on-unresolvable behaviour: the front-end sources the shell library and bridges the arm's knobs, so a consumer's overrides reach the rendering this meter counts (gate-sdk/SPEC.md §The non-gate arm)
if [[ -z "${CONTEXT_KIT_HOOK_CMD+x}" ]]; then
    CONTEXT_KIT_HOOK_CMD=""
    for _qi in "${GATE_SDK_GATES_DIR:-scripts}/run-gates.sh" "$KIT/../gate-sdk/bin/run-gates.sh"; do
        if [[ -f "$_qi" ]]; then
            CONTEXT_KIT_HOOK_CMD="bash $_qi --emit queue-index --collapse-deferred"
            break
        fi
    done
    unset _qi
fi

surface=0
for f in "${CONTEXT_KIT_SURFACES[@]}"; do
    [[ -f "$f" ]] || continue
    n="$(wc -l < "$f" 2>/dev/null | tr -d ' ')"
    [[ "$n" =~ ^[0-9]+$ ]] && surface=$(( surface + n ))
done

hook=0
if [[ -n "$CONTEXT_KIT_HOOK_CMD" ]]; then
    hook_out="$(bash -c "$CONTEXT_KIT_HOOK_CMD" 2>/dev/null || true)"
    if [[ -n "$hook_out" ]]; then
        hook="$(printf '%s\n' "$hook_out" | wc -l | tr -d ' ')"
        [[ "$hook" =~ ^[0-9]+$ ]] || hook=0
    fi
fi

total=$(( surface + hook ))

base_total=""; base_commit=""; base_extra=""
if [[ -f "$CONTEXT_KIT_BASELINE_FILE" ]]; then
    read -r base_total _ base_commit base_extra < <(
        grep -vE '^[[:space:]]*(#|$)' "$CONTEXT_KIT_BASELINE_FILE" 2>/dev/null | head -1)
fi

# spec: context-kit/SPEC.md §The always-loaded meter — the growth arm: no governed file is exempt from the brevity pass, so the worklist is every file that grew net since the baseline commit, largest first
if [[ "$GROWTH" -eq 1 ]]; then
    if [[ -z "$base_commit" ]] || ! git rev-parse -q --verify "$base_commit^{commit}" >/dev/null 2>&1; then
        echo "growth: no resolvable baseline commit in $CONTEXT_KIT_BASELINE_FILE"
        exit 0
    fi
    rows="$(git diff --numstat "$base_commit" -- "${CONTEXT_KIT_GROWTH_PATHS[@]}" 2>/dev/null \
        | awk -F'\t' '$1 != "-" && $1 - $2 > 0 { printf "%d\t%s\n", $1 - $2, $3 }' | sort -rn)"
    n=0; net=0
    while IFS=$'\t' read -r d f; do [[ -n "$f" ]] || continue; n=$(( n + 1 )); net=$(( net + d )); done <<<"$rows"
    echo "growth since ${base_commit:0:8}: $n file(s) grew, +$net net line(s)"
    [[ -n "$rows" ]] && printf '%s\n' "$rows" | awk -F'\t' '{ printf "  +%d\t%s\n", $1, $2 }'
    exit 0
fi

if [[ "$UPDATE" -eq 1 ]]; then
    commit="$(git rev-parse HEAD 2>/dev/null || echo unknown)"
    line="$total $surface $commit"
    [[ -n "$base_extra" ]] && line="$line $base_extra"
    {
        echo "# contract: context-kit/SPEC.md §The always-loaded meter"
        echo "$line"
    } > "$CONTEXT_KIT_BASELINE_FILE"
    echo "always-loaded baseline updated: ${total}l (surfaces $surface · hook $hook) @ ${commit:0:8}"
    exit 0
fi

line="always-loaded: ${total}l (surfaces $surface · hook $hook)"
if [[ "$base_total" =~ ^[0-9]+$ ]]; then
    delta=$(( total - base_total ))
    line="$line  $(printf '%+d' "$delta") since ${base_commit:0:8}"
fi
echo "$line"
exit 0
