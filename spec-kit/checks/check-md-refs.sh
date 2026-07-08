#!/usr/bin/env bash
# graph: couples=*SPEC*.md,*README.md,CLAUDE.md dir=one valve=none tier=precommit
# spec: spec-kit/SPEC.md §check-md-refs — every internal markdown link in the governed doc set resolves (relative path to a tracked file/dir, #anchor to a heading slug)
#
# usage: check-md-refs.sh [file...]
#   Defaults to the manifest set (lib/spec.sh) minus SPEC_KIT_MDREF_EXCLUDE.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

git rev-parse --git-dir >/dev/null 2>&1 || {
    echo "check-md-refs: not a git repository — cannot verify tracked targets" >&2; exit 2; }

excluded() {
    local rel="$1" g
    for g in "${SPEC_KIT_MDREF_EXCLUDE[@]+"${SPEC_KIT_MDREF_EXCLUDE[@]}"}"; do
        # shellcheck disable=SC2053  # $g is the exclude glob, matched unquoted on purpose
        [[ "$rel" == $g ]] && return 0
    done
    return 1
}

if [[ $# -gt 0 ]]; then
    files=("$@")
else
    files=()
    while IFS= read -r f; do
        excluded "${f#./}" || files+=("$f")
    done < <(spec_manifest_files ".")
fi

slugify() {
    local s="${1,,}"
    printf '%s' "$s" | sed -E 's/[^a-z0-9 _-]//g; s/ +/-/g'
}

anchor_ok() {  # $1=target file, $2=anchor slug
    local h
    while IFS= read -r h; do
        [[ "$(slugify "$h")" == "$2" ]] && return 0
    done < <(sed -nE 's/^#{1,6}[[:space:]]+(.*[^[:space:]])[[:space:]]*$/\1/p' "$1")
    return 1
}

target_resolves() {  # $1=repo-relative path
    local p="$1"
    [[ "$p" == ..* ]] && return 1
    if [[ -f "$p" ]]; then
        git ls-files --error-unmatch -- "$p" >/dev/null 2>&1 && return 0
        git check-ignore -q -- "$p" && return 0
        return 1
    fi
    [[ -d "$p" && -n "$(git ls-files -- "$p")" ]]
}

bad=(); links=0
for f in "${files[@]}"; do
    [[ -f "$f" ]] || continue
    base="$(dirname "$f")"
    while IFS= read -r raw; do
        tgt="${raw#*](}"; tgt="${tgt%\)}"
        tgt="${tgt%% *}"                       # drop any "title" suffix
        [[ -n "$tgt" ]] || continue
        [[ "$tgt" == *"://"* || "$tgt" == mailto:* ]] && continue
        links=$((links + 1))
        path="${tgt%%#*}"; anchor=""
        [[ "$tgt" == *#* ]] && anchor="${tgt#*#}"
        if [[ -z "$path" ]]; then
            [[ -n "$anchor" ]] && ! anchor_ok "$f" "$anchor" \
                && bad+=("$f: [..](#$anchor) — no heading in this file slugs to '$anchor'")
            continue
        fi
        p="$(realpath -m --relative-to=. -- "$base/$path" 2>/dev/null)"
        if ! target_resolves "$p"; then
            bad+=("$f: link target '$tgt' → $p is not a tracked file or directory")
            continue
        fi
        if [[ -n "$anchor" && -f "$p" ]] && ! anchor_ok "$p" "$anchor"; then
            bad+=("$f: [..]($tgt) — no heading in $p slugs to '$anchor'")
        fi
    done < <(grep -oE '\]\([^)]+\)' "$f")
done

if [[ ${#bad[@]} -gt 0 ]]; then
    echo "check-md-refs: unresolved internal markdown link(s) in the governed doc set:"
    printf '  %s\n' "${bad[@]}"
    echo "  help: fix the path (relative to the linking file), track the target, or fix the"
    echo "        #anchor to a real heading slug. External URLs are out of scope."
    exit 1
fi

echo "MD-REFS: clean (${#files[@]} doc(s), $links internal link(s) all resolve)"
exit 0
