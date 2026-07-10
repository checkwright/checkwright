#!/usr/bin/env bash
# graph: couples=docs/CNAME dir=one valve=none tier=precommit trigger=*
# spec: site-kit/SPEC.md §check-docs-cname-parity — the docs/CNAME host is the single gated source of truth for the docs host; no tracked file names a configured host alias other than that host in a URL
#
# usage: check-docs-cname-parity.sh [scan-root] [cname-file] [config-file]
#   defaults SITE_KIT_SCAN_ROOT / SITE_KIT_CNAME; config-file overrides
#   SITE_KIT_CONFIG_FILE so a fixture supplies its own SITE_KIT_ALIASES.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

SCANARG="${1:-}"; CNAMEARG="${2:-}"; CONFIGARG="${3:-}"
[[ -n "$CONFIGARG" ]] && export SITE_KIT_CONFIG_FILE="$CONFIGARG"
# shellcheck source=../lib/site.sh
source "$KIT/lib/site.sh"

SCANROOT="${SCANARG:-$SITE_KIT_SCAN_ROOT}"; SCANROOT="${SCANROOT%/}"
CNAME="${CNAMEARG:-$SITE_KIT_CNAME}"

git rev-parse --git-dir >/dev/null 2>&1 || {
    echo "check-docs-cname-parity: not a git repository — cannot enumerate tracked files" >&2; exit 2; }
[[ -f "$CNAME" ]] || { echo "check-docs-cname-parity: CNAME not found: $CNAME" >&2; exit 2; }

mapfile -t hlines < <(grep -v '^[[:space:]]*$' "$CNAME")
[[ ${#hlines[@]} -eq 1 ]] || {
    echo "check-docs-cname-parity: $CNAME must hold exactly one host line (found ${#hlines[@]})" >&2; exit 2; }
H="${hlines[0]//[[:space:]]/}"
[[ -n "$H" ]] || { echo "check-docs-cname-parity: empty host in $CNAME" >&2; exit 2; }

declare -A IS_ALIAS=()
for a in "${SITE_KIT_ALIASES[@]+"${SITE_KIT_ALIASES[@]}"}"; do IS_ALIAS["$a"]=1; done

exempt_path() {  # $1=path — matches any SITE_KIT_EXEMPT_PATHS glob
    local p="$1" g
    for g in "${SITE_KIT_EXEMPT_PATHS[@]+"${SITE_KIT_EXEMPT_PATHS[@]}"}"; do
        # shellcheck disable=SC2053  # $g is the exempt glob, matched unquoted on purpose
        [[ "$p" == $g ]] && return 0
    done
    return 1
}

listing="$(git ls-files -- "$SCANROOT")"; st=$?
fail_closed "$st" DOCS-CNAME-PARITY git-ls-files

files=()
while IFS= read -r path; do
    [[ -n "$path" ]] || continue
    gate_path_pruned "$path" && continue
    exempt_path "$path" && continue
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
    echo "check-docs-cname-parity: tracked file(s) cite a configured host alias other than the docs/CNAME host:"
    printf '  %s\n' "${bad[@]}"
    echo "  help: point the URL at the docs/CNAME host '$H' (a rename is a one-file edit to the CNAME"
    echo "        that this gate then enumerates); SITE_KIT_EXEMPT_PATHS sites are exempt."
    exit 1
fi
echo "DOCS-CNAME-PARITY: clean (${#files[@]} tracked file(s) under $SCANROOT; no alias but the docs host '$H' cited in a URL)"
exit 0
