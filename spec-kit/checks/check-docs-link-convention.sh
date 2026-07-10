#!/usr/bin/env bash
# graph: couples=docs/*.md,docs/*/index.md,docs/posts/*.md dir=one valve=none tier=precommit
# spec: spec-kit/SPEC.md §check-docs-link-convention — docs pages cite downward: no directory-target relative link (name the file), a kit page's back-link to its own README/SPEC carries a #section anchor
#
# usage: check-docs-link-convention.sh [docs-root]   (default SPEC_KIT_LINK_ROOT;
#   the optional arg points the fixture pair at a synthetic docs tree)
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

ROOT="${1:-$SPEC_KIT_LINK_ROOT}"; ROOT="${ROOT%/}"
[[ -d "$ROOT" ]] || { echo "check-docs-link-convention: not a directory: $ROOT" >&2; exit 2; }

mapfile -t pages < <(find "$ROOT" -type f -name '*.md' | sort)
[[ ${#pages[@]} -eq 0 ]] && { echo "DOCS-LINK-CONVENTION: clean (0 docs page(s) found)"; exit 0; }

exempt() {  # $1=file $2=lineno — a docs-link-exempt marker on the hit line or the one above
    local f="$1" n="$2" lo
    lo=$(( n > 1 ? n - 1 : 1 ))
    sed -n "${lo},${n}p" "$f" | grep -q 'docs-link-exempt:'
}

bad=(); links=0
for f in "${pages[@]}"; do
    base="$(dirname "$f")"
    kit=""                                    # a kit page is <root>/<kit>/index.md — its kit is the parent dir name
    [[ "$(basename "$f")" == index.md && "$base" != "$ROOT" && "$(dirname "$base")" == "$ROOT" ]] \
        && kit="$(basename "$base")"
    while IFS= read -r hit; do
        lno="${hit%%:*}"; raw="${hit#*:}"
        tgt="${raw#*](}"; tgt="${tgt%\)}"; tgt="${tgt%% *}"   # strip ]( … ) and any "title" suffix
        [[ -n "$tgt" ]] || continue
        [[ "$tgt" == *"://"* || "$tgt" == mailto:* ]] && continue
        links=$((links + 1))
        path="${tgt%%#*}"; anchor=""
        [[ "$tgt" == *#* ]] && anchor="${tgt#*#}"
        [[ -n "$path" ]] || continue                          # pure #anchor — neither invariant applies

        if [[ "$path" == */ ]]; then
            exempt "$f" "$lno" || bad+=("$f:$lno: directory-target link '$tgt' — name the file (e.g. ${path}index.md), not the directory")
            continue
        fi
        p="$(realpath -m --relative-to=. -- "$base/$path" 2>/dev/null)"
        [[ -n "$p" ]] || continue
        if [[ -d "$p" ]]; then
            exempt "$f" "$lno" || bad+=("$f:$lno: directory-target link '$tgt' → $p/ — name the file (e.g. $tgt/index.md), not the directory")
            continue
        fi
        if [[ -n "$kit" && -z "$anchor" ]]; then
            b="$(basename "$p")"
            [[ ( "$b" == README.md || "$b" == SPEC.md ) && "$(basename "$(dirname "$p")")" == "$kit" ]] \
                && { exempt "$f" "$lno" || bad+=("$f:$lno: anchorless back-link '$tgt' to this kit's $b — a docs page cites downward, name the #section rather than the whole spec"); }
        fi
    done < <(grep -noE '\]\([^)]+\)' "$f")
done

if [[ ${#bad[@]} -gt 0 ]]; then
    echo "check-docs-link-convention: docs page link(s) break a shape convention (resolution is check-md-refs' job; this gate owns shape):"
    printf '  %s\n' "${bad[@]}"
    echo "  help: name the file a directory link points at (kit/index.md, not kit/); give a kit page's"
    echo "        back-link to its own README/SPEC a #section anchor. Per-site valve: a 'docs-link-exempt:"
    echo "        <reason>' HTML comment on the link line or the one above."
    exit 1
fi
echo "DOCS-LINK-CONVENTION: clean (${#pages[@]} docs page(s), $links relative link(s); no directory target, kit back-links anchored)"
exit 0
