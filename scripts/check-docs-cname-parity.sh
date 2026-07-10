#!/usr/bin/env bash
# graph: couples=docs/CNAME dir=one valve=none tier=precommit trigger=*
# spec: CLAUDE.md §Housekeeping — docs/CNAME is the single gated source of truth for the docs host; no tracked file names a project host alias other than that host in a URL
#
# usage: check-docs-cname-parity.sh [scan-root] [cname-file]
#   scan-root via git ls-files (default '.'), cname-file supplies H (default
#   docs/CNAME); fixtures pass both. Exempt: docs/posts/*, */gate-tests/*.
set -uo pipefail

SDK="${GATE_SDK_ROOT:-"${BASH_SOURCE[0]%/*}/../gate-sdk"}"
# shellcheck source=../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

SCANROOT="${1:-.}"; SCANROOT="${SCANROOT%/}"
CNAME="${2:-docs/CNAME}"

git rev-parse --git-dir >/dev/null 2>&1 || {
    echo "check-docs-cname-parity: not a git repository — cannot enumerate tracked files" >&2; exit 2; }
[[ -f "$CNAME" ]] || { echo "check-docs-cname-parity: CNAME not found: $CNAME" >&2; exit 2; }

mapfile -t hlines < <(grep -v '^[[:space:]]*$' "$CNAME")
[[ ${#hlines[@]} -eq 1 ]] || {
    echo "check-docs-cname-parity: $CNAME must hold exactly one host line (found ${#hlines[@]})" >&2; exit 2; }
H="${hlines[0]//[[:space:]]/}"
[[ -n "$H" ]] || { echo "check-docs-cname-parity: empty host in $CNAME" >&2; exit 2; }

# exception-list: this repo's own project host aliases, legitimately hardcoded in a
#   consumer gate — the provenance seam bars a kit literal from carrying rule content,
#   never a scripts/ gate; each stays until a rename retires it, hence # permanent:.
ALIASES=(
    checkwright.dev        # permanent: canonical apex; equals H in this clone, skipped at compare
    www.checkwright.dev    # permanent: www subdomain alias, never the cited docs host
    checkwright.com        # permanent: .com redirect host, reachable but never cited
    www.checkwright.com    # permanent: www .com alias, never cited
    checkwright.github.io  # permanent: pre-CNAME Pages default host
)
declare -A IS_ALIAS=()
for a in "${ALIASES[@]}"; do IS_ALIAS["$a"]=1; done

listing="$(git ls-files -- "$SCANROOT")"; st=$?
fail_closed "$st" DOCS-CNAME-PARITY git-ls-files

files=()
while IFS= read -r path; do
    [[ -n "$path" ]] || continue
    gate_path_pruned "$path" && continue
    case "$path" in
        */gate-tests/*|*docs/posts/*) continue ;;
    esac
    [[ -f "$path" ]] && files+=("$path")
done <<< "$listing"

if [[ ${#files[@]} -eq 0 ]]; then
    echo "DOCS-CNAME-PARITY: clean (0 tracked file(s) under $SCANROOT; docs host is '$H')"
    exit 0
fi

matches="$(grep -IHnoE '://[A-Za-z0-9.-]+' -- "${files[@]}")"; gst=$?
[[ "$gst" -le 1 ]] || fail_closed "$gst" DOCS-CNAME-PARITY grep

bad=()
while IFS= read -r m; do
    [[ -n "$m" ]] || continue
    host="${m##*//}"
    [[ -n "${IS_ALIAS[$host]:-}" && "$host" != "$H" ]] || continue
    bad+=("${m%:://*}: alias '$host' — docs host is '$H'")
done <<< "$matches"

if [[ ${#bad[@]} -gt 0 ]]; then
    echo "check-docs-cname-parity: tracked file(s) cite a project host alias other than the docs/CNAME host:"
    printf '  %s\n' "${bad[@]}"
    echo "  help: point the URL at the docs/CNAME host '$H' (a rename is a one-file edit to docs/CNAME"
    echo "        that this gate then enumerates); docs/posts/* and */gate-tests/* are exempt."
    exit 1
fi
echo "DOCS-CNAME-PARITY: clean (${#files[@]} tracked file(s) under $SCANROOT; no alias but the docs host '$H' cited in a URL)"
exit 0
