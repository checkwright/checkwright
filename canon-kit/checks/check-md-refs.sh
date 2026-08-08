#!/usr/bin/env bash
# graph: couples=*SPEC*.md,*README.md,CLAUDE.md dir=one valve=none tier=precommit
# install: on-surface
# spec: canon-kit/SPEC.md §check-md-refs — every internal markdown link in the governed doc set resolves (relative path to a tracked file/dir, #anchor to a heading slug)
#
# usage: check-md-refs.sh [file...]
#   Defaults to the manifest set (lib/spec.sh) minus CANON_KIT_MDREF_EXCLUDE.
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
    for g in "${CANON_KIT_MDREF_EXCLUDE[@]+"${CANON_KIT_MDREF_EXCLUDE[@]}"}"; do
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

# spec: canon-kit/SPEC.md §check-md-refs — the self-repo blob-link prefix, derived from origin through the shared gate.sh adapter (git@ and https forms normalize to one identity); CANON_KIT_DOCS_BLOB_REF is the ref policy, empty ⇒ no origin, the self-repo pass is skipped
self_repo_prefix="$(gate_self_repo_prefix "$CANON_KIT_DOCS_BLOB_REF")"

# spec: canon-kit/SPEC.md §check-md-refs — the tracked-file membership set, filled once from a single git ls-files pass rather than a per-link `git ls-files --error-unmatch` exec; check-ignore and the directory listing stay per-call (the fallback path, not the hot one)
declare -A TRACKED=()
while IFS= read -r -d '' _tf; do TRACKED["$_tf"]=1; done < <(git ls-files -z)

# spec: canon-kit/SPEC.md §check-md-refs — anchor_ok memoizes a target file's slug set on first use: one heading-extraction sed and one bulk slugify sed per file, however many anchors across however many links cite it, instead of re-forking sed per heading per link
declare -A ANCHOR_SLUGS=()
declare -A ANCHOR_SLUGS_DONE=()

anchor_ok() {  # $1=target file, $2=anchor slug
    local file="$1" headings s
    if [[ -z "${ANCHOR_SLUGS_DONE[$file]:-}" ]]; then
        ANCHOR_SLUGS_DONE["$file"]=1
        headings="$(sed -nE 's/^#{1,6}[[:space:]]+(.*[^[:space:]])[[:space:]]*$/\1/p' "$file")"
        if [[ -n "$headings" ]]; then
            headings="${headings,,}"
            ANCHOR_SLUGS["$file"]="$(sed -E 's/[^a-z0-9 _-]//g; s/ +/-/g' <<< "$headings")"
        else
            ANCHOR_SLUGS["$file"]=""
        fi
    fi
    while IFS= read -r s; do
        [[ "$s" == "$2" ]] && return 0
    done <<< "${ANCHOR_SLUGS[$file]}"
    return 1
}

target_resolves() {  # $1=repo-relative path
    local p="$1"
    [[ "$p" == ..* ]] && return 1
    if [[ -f "$p" ]]; then
        [[ -n "${TRACKED[$p]:-}" ]] && return 0
        git check-ignore -q -- "$p" && return 0
        return 1
    fi
    [[ -d "$p" && -n "$(git ls-files -- "$p")" ]]
}

bad=(); links=0; selfrepo=0
for f in "${files[@]}"; do
    [[ -f "$f" ]] || continue
    base="$(dirname "$f")"
    while IFS= read -r raw; do
        tgt="${raw#*](}"; tgt="${tgt%\)}"
        tgt="${tgt%% *}"                       # drop any "title" suffix
        [[ -n "$tgt" ]] || continue
        if [[ -n "$self_repo_prefix" && "$tgt" == "$self_repo_prefix"* ]]; then
            rest="${tgt#"$self_repo_prefix"}"
            links=$((links + 1)); selfrepo=$((selfrepo + 1))
            path="${rest%%#*}"; anchor=""
            [[ "$rest" == *#* ]] && anchor="${rest#*#}"
            if [[ -z "$path" ]]; then
                bad+=("$f: self-repo reference link '$tgt' names no path")
            elif ! target_resolves "$path"; then
                bad+=("$f: self-repo reference link '$tgt' → $path is not a git-tracked file")
            elif [[ -n "$anchor" && -f "$path" ]] && ! anchor_ok "$path" "$anchor"; then
                bad+=("$f: [..]($tgt) — no heading in $path slugs to '$anchor'")
            fi
            continue
        fi
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

echo "MD-REFS: clean (${#files[@]} doc(s), $links internal link(s) all resolve; $selfrepo self-repo reference link(s))"
exit 0
