#!/usr/bin/env bash
# graph: couples=docs/*.md,docs/*/index.md,docs/*/SPEC.md,docs/*/README.md,docs/doctrine-kit/DOCTRINE.md,docs/posts/*.md,scripts/docs-offnav.list dir=one valve=none tier=precommit
# spec: CLAUDE.md §Housekeeping — every tracked docs page carries a title front-matter block and is reachable from the rendered nav (a nav slot, or a relative-link walk seeded from the nav set), or is listed in the off-nav allowlist
#
# usage: check-docs-nav-reachable.sh [docs-root] [allowlist]
#   defaults: docs, scripts/docs-offnav.list — the optional args point the
#   fixture pair at a synthetic docs tree and its own allowlist.
set -uo pipefail

SDK="${GATE_SDK_ROOT:-"${BASH_SOURCE[0]%/*}/../gate-sdk"}"
# shellcheck source=../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

ROOT="${1:-docs}"; ROOT="${ROOT%/}"
ALLOWLIST="${2:-scripts/docs-offnav.list}"
[[ -d "$ROOT" ]] || { echo "check-docs-nav-reachable: not a directory: $ROOT" >&2; exit 2; }

mapfile -t pages < <(find "$ROOT" -type d -name '_*' -prune -o -type f -name '*.md' -print | sort)
[[ ${#pages[@]} -eq 0 ]] && { echo "DOCS-NAV-REACHABLE: clean (0 docs page(s) found under $ROOT)"; exit 0; }

declare -A ALLOW=()
if [[ -f "$ALLOWLIST" ]]; then
    while IFS= read -r line; do
        line="${line%%#*}"; line="${line//[[:space:]]/}"
        [[ -n "$line" ]] && ALLOW["$line"]=1
    done < "$ALLOWLIST"
fi

# spec: CLAUDE.md §Housekeeping — front-matter facts per page: does it open with a
# block carrying title:, does it hold a top-level nav_order slot, and its nav_id /
# nav_parent (the include's one-level parent/child keys). Resolution is in the shell.
read -r -d '' FM_EXTRACT <<'AWK' || true
NR==1 { if ($0 != "---") { print "0 0 - -"; exit } fm=1; next }
fm && $0 == "---" { fm=0 }
fm && /^title:[[:space:]]*[^[:space:]]/ { t=1 }
fm && /^nav_order:[[:space:]]*[0-9]/ { o=1 }
fm && /^nav_id:[[:space:]]*[^[:space:]]/ { id=$2 }
fm && /^nav_parent:[[:space:]]*[^[:space:]]/ { par=$2 }
END { print t+0, o+0, (id=="" ? "-" : id), (par=="" ? "-" : par) }
AWK

declare -A TITLE=() ORDER=() PARENT=() INSCOPE=() TOPNAVID=()
for p in "${pages[@]}"; do
    fm="$(awk "$FM_EXTRACT" "$p")"; st=$?
    fail_closed "$st" DOCS-NAV-REACHABLE "awk front-matter ($p)"
    read -r t o id par <<< "$fm"
    TITLE[$p]="$t"; ORDER[$p]="$o"; PARENT[$p]="$par"; INSCOPE[$p]=1
    [[ "$o" == "1" && "$id" != "-" ]] && TOPNAVID[$id]=1
done

links_of() {  # $1=in-scope page — prints each in-tree relative .md link target (cwd-relative)
    local f="$1" base raw tgt path p
    base="$(dirname "$f")"
    while IFS= read -r hit; do
        raw="${hit#*:}"; tgt="${raw#*](}"; tgt="${tgt%\)}"; tgt="${tgt%% *}"
        [[ -n "$tgt" ]] || continue
        [[ "$tgt" == *"://"* || "$tgt" == mailto:* ]] && continue
        path="${tgt%%#*}"
        [[ -n "$path" && "$path" == *.md ]] || continue
        p="$(realpath -m --relative-to=. -- "$base/$path" 2>/dev/null)"
        [[ -n "$p" && -f "$p" ]] && printf '%s\n' "$p"
    done < <(grep -noE '\]\([^)]+\)' "$f")
}

declare -A REACH=()
queue=()
for p in "${pages[@]}"; do
    if [[ "${ORDER[$p]}" == "1" ]] || { [[ "${PARENT[$p]}" != "-" ]] && [[ -n "${TOPNAVID[${PARENT[$p]}]:-}" ]]; }; then
        REACH[$p]=1; queue+=("$p")
    fi
done

while [[ ${#queue[@]} -gt 0 ]]; do
    cur="${queue[0]}"; queue=("${queue[@]:1}")
    while IFS= read -r tgt; do
        [[ -n "$tgt" && -n "${INSCOPE[$tgt]:-}" && -z "${REACH[$tgt]:-}" ]] || continue
        REACH[$tgt]=1; queue+=("$tgt")
    done < <(links_of "$cur")
done

bad=()
for p in "${pages[@]}"; do
    [[ -n "${ALLOW[$p]:-}" ]] && continue
    [[ "${TITLE[$p]}" == "1" ]] || bad+=("$p: no title front-matter block — it renders without a nav slot and joins no nav")
    [[ -n "${REACH[$p]:-}" ]] || bad+=("$p: not reachable from the rendered nav — no nav slot and no link walk reaches it")
done

if [[ ${#bad[@]} -gt 0 ]]; then
    echo "check-docs-nav-reachable: docs page(s) fall outside the site nav:"
    printf '  %s\n' "${bad[@]}"
    echo "  help: open the page with a front-matter block carrying 'title:', and give it a nav slot"
    echo "        ('nav_order: <n>' top-level, or 'nav_parent: <id>' under one), or link it from a nav"
    echo "        page. A page off-nav by design (an embedded fragment) goes in $ALLOWLIST."
    exit 1
fi
echo "DOCS-NAV-REACHABLE: clean (${#pages[@]} docs page(s) under $ROOT; each carries a title block and sits in the rendered nav or its link walk, or is allowlisted off-nav)"
exit 0
