#!/usr/bin/env bash
# spec: context-kit/SPEC.md §Index-first reading — compact public API surface, a dispatcher over per-language extractors (ships rust, ts)
# usage: pub-index.sh [paths…]
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"

_ck_cfg="${CONTEXT_KIT_CONFIG_FILE:-${GATE_SDK_GATES_DIR:-scripts}/context-config.sh}"
if [[ -f "$_ck_cfg" ]]; then
    # shellcheck source=/dev/null  # consumer config path is resolved at runtime
    source "$_ck_cfg"
fi
unset _ck_cfg

TARGETS=("$@")
[[ ${#TARGETS[@]} -eq 0 ]] && TARGETS=("$REPO_ROOT")

# spec: context-kit/SPEC.md §Index-first reading — extractor resolution: the consumer dir shadows the kit's shipped lib/pub-lang (the gates.list consumer-first precedent)
KIT_LANG_DIR="$KIT/lib/pub-lang"
LANG_DIR="${CONTEXT_KIT_PUB_LANG_DIR:-${GATE_SDK_GATES_DIR:-scripts}/pub-lang}"

resolve_extractor() {   # $1 = lang -> prints extractor path, or returns 1
    [[ -f "$LANG_DIR/$1.sh" ]] && { printf '%s' "$LANG_DIR/$1.sh"; return 0; }
    [[ -f "$KIT_LANG_DIR/$1.sh" ]] && { printf '%s' "$KIT_LANG_DIR/$1.sh"; return 0; }
    return 1
}

# spec: context-kit/SPEC.md §Index-first reading — the enabled set is CONTEXT_KIT_PUB_LANGS; its default is the shipped kit roster, derived at run time (never a maintained list)
if declare -p CONTEXT_KIT_PUB_LANGS >/dev/null 2>&1; then
    LANGS=("${CONTEXT_KIT_PUB_LANGS[@]}")
else
    LANGS=()
    if [[ -d "$KIT_LANG_DIR" ]]; then
        while IFS= read -r f; do LANGS+=("$(basename "$f" .sh)"); done \
            < <(find "$KIT_LANG_DIR" -maxdepth 1 -name '*.sh' -type f | sort)
    fi
fi

PRUNE=(-not -path '*/.git/*' -not -path '*/target/*' -not -path '*/node_modules/*' -not -path '*/dist/*' -not -path '*/build/*')

emit_lang() {   # $1 = extractor path; prints per-file blocks for this language
    (
        # shellcheck source=/dev/null  # extractor path is resolved at runtime
        source "$1"
        local nameexpr=() g
        for g in "${PUB_LANG_GLOBS[@]}"; do
            [[ ${#nameexpr[@]} -gt 0 ]] && nameexpr+=(-o)
            nameexpr+=(-name "$g")
        done
        local file raw rows rel count
        while IFS= read -r -d '' file; do
            raw="$(pub_lang_extract "$file" || true)"
            [[ -n "$raw" ]] || continue
            rows="$(printf '%s\n' "$raw" | LC_ALL=C sort -k1,1 -k2,2)"
            rel="${file#"$REPO_ROOT"/}"
            count="$(printf '%s\n' "$rows" | wc -l | tr -d ' ')"
            printf '%s  (%s)\n' "$rel" "$count"
            printf '%s\n' "$rows" | awk '{ printf "  %-8s %s :%s\n", $1, $2, $3 }'
            printf '\n'
        done < <(find "${TARGETS[@]}" \( "${nameexpr[@]}" \) "${PRUNE[@]}" -print0 2>/dev/null | sort -z)
    )
}

FOUND=0
for lang in "${LANGS[@]}"; do
    extractor="$(resolve_extractor "$lang")" || {
        echo "pub-index: no extractor for language '$lang' in $LANG_DIR or $KIT_LANG_DIR" >&2
        exit 2
    }
    block="$(emit_lang "$extractor")"
    if [[ -n "$block" ]]; then
        printf '%s\n' "$block"
        FOUND=1
    fi
done

if [[ "$FOUND" -eq 0 ]]; then
    echo "No public items found in ${TARGETS[*]}"
fi
