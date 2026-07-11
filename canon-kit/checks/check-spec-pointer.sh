#!/usr/bin/env bash
# graph: couples=*SPEC*.md,*README.md,CLAUDE.md,scripts/*.sh,kit:*.sh,.workflow/*.txt dir=one valve=none tier=precommit
# spec: canon-kit/SPEC.md §check-spec-pointer — every spec:/contract: directive on a governed source and every free-prose <path>.md §<heading> citation on a governed manifest resolves: the target file is tracked and a §heading fragment names a heading the file carries
#
# usage: check-spec-pointer.sh [scan-root]   (default '.')
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

ROOT="${1:-.}"
[[ -d "$ROOT" ]] || { echo "check-spec-pointer: not a directory: $ROOT" >&2; exit 2; }
git -C "$ROOT" rev-parse --git-dir >/dev/null 2>&1 || {
    echo "check-spec-pointer: $ROOT is not a git repository — cannot verify tracked targets" >&2; exit 2; }

# spec: canon-kit/SPEC.md §check-spec-pointer — shape-only extraction of the
# spec:/contract: directive lines across the hash (# / .txt) and slash (// , block
# *) comment surfaces; resolution happens in the shell below.
read -r -d '' EXTRACT <<'AWK' || true
{
    L = $0
    # the colon needs trailing whitespace (a real "<keyword>: <path>" directive) or
    # end-of-line (a degenerate empty directive, flagged downstream); this parts a
    # pointer from prose that merely opens with "spec:/contract:".
    if      (L ~ /^[ \t]*#+[ \t]*(spec|contract):([ \t]|$)/)   sub(/^[ \t]*#+[ \t]*/, "", L)
    else if (L ~ /^[ \t]*\/\/+[ \t]*(spec|contract):([ \t]|$)/) sub(/^[ \t]*\/\/+[ \t]*/, "", L)
    else if (L ~ /^[ \t]*\*[ \t]*(spec|contract):([ \t]|$)/)    sub(/^[ \t]*\*[ \t]*/, "", L)
    else next
    print FILENAME "\t" FNR "\t" L
}
AWK

heading_present() {  # $1=target file  $2=heading fragment  [$3=exact(default)|prefix] -> 0 when a heading matches
    local file="$1" frag="$2" mode="${3:-exact}" res st
    # spec: canon-kit/SPEC.md §check-spec-pointer — one resolver, two callers:
    # the directive pass matches the fragment whole (exact), the prose pass as
    # a boundary-anchored prefix (prefix); both tolerate a trailing "(qualifier)".
    res="$(awk -v a="$frag" -v mode="$mode" '
        function strippar(s) { sub(/[[:space:]]*\([^)]*\)[[:space:]]*$/, "", s); return s }
        function isprefix(text, h,   L, nc) {
            L = length(h)
            if (substr(text, 1, L) != h) return 0
            if (length(text) == L) return 1
            nc = substr(text, L + 1, 1)
            return (nc !~ /[[:alnum:]]/)
        }
        BEGIN { as = strippar(a) }
        /^#{1,6}[[:space:]]/ {
            h = $0; sub(/^#{1,6}[[:space:]]+/, "", h); sub(/[[:space:]]+$/, "", h)
            hs = strippar(h)
            if (mode == "prefix") {
                if (isprefix(a, h) || isprefix(a, hs)) found = 1
            } else if (h == a || hs == a || h == as || hs == as) {
                found = 1
            }
        }
        END { print found + 0 }
    ' "$file")"; st=$?
    fail_closed "$st" SPEC-POINTER "awk heading match"
    [[ "$res" == "1" ]]
}

# spec: canon-kit/SPEC.md §check-spec-pointer — shape-only extraction of prose
# <path>.md § citations over the blank-line paragraph join; fenced code skipped,
# resolution and the tracked-file carve-out happen in the shell below.
read -r -d '' PROSE_EXTRACT <<'AWK' || true
function flush(   i, joined, mstart, mend, seg, path, post, li, s) {
    if (np == 0) return
    joined = ""
    for (i = 1; i <= np; i++) { lstart[i] = length(joined) + 1; joined = joined (i > 1 ? " " : "") ptext[i] }
    scanpos = 1
    while (1) {
        s = substr(joined, scanpos)
        if (match(s, /[A-Za-z0-9._\/-]+\.md[[:space:]]*§/) == 0) break
        mstart = scanpos + RSTART - 1
        mend = mstart + RLENGTH - 1
        seg = substr(joined, mstart, RLENGTH)
        path = seg; sub(/[[:space:]]*§$/, "", path)
        post = substr(joined, mend + 1)
        li = 1
        for (i = 1; i <= np; i++) if (lstart[i] <= mstart) li = i
        printf "%s\t%d\t%s\t%s\n", cf, pfnr[li], path, post
        scanpos = mend + 1
    }
    np = 0
}
FNR == 1 { flush(); fence = 0 }
{
    cf = FILENAME
    if ($0 ~ /^[[:space:]]*```/) { flush(); fence = !fence; next }
    if (fence) { flush(); next }
    if ($0 ~ /^[[:space:]]*$/) { flush(); next }
    np++; pfnr[np] = FNR; ptext[np] = $0
}
END { flush() }
AWK

errors=()
scanned=0
pointers=0
while IFS= read -r f; do
    [[ -n "$f" ]] || continue
    rel="${f#"$ROOT"/}"; rel="${rel#./}"
    spec_comment_whitelisted "$rel" && continue
    scanned=$((scanned + 1))
    out="$(awk "$EXTRACT" "$f")"; st=$?
    fail_closed "$st" SPEC-POINTER "awk extract ($rel)"
    while IFS=$'\t' read -r _file lineno body; do
        [[ -n "$body" ]] || continue
        pointers=$((pointers + 1))
        if [[ "$body" == spec:* ]]; then after="${body#spec:}"; else after="${body#contract:}"; fi
        # comment-tier-exempt: a § inside the em-dash prose tail is not a heading marker
        sig="${after%% — *}"; sig="${sig%% -- *}"
        read -r path prest <<< "$sig"
        frag=""
        if [[ "$path" == *§* ]]; then
            frag="${path#*§}"; path="${path%%§*}"
        elif [[ "$prest" == *§* ]]; then
            frag="${prest#*§}"
        fi
        frag="${frag#"${frag%%[![:space:]]*}"}"
        frag="${frag%"${frag##*[![:space:]]}"}"

        if [[ -z "$path" || "$path" == §* ]]; then
            errors+=("$rel:$lineno: pointer directive carries no target path: $body")
            continue
        fi
        if [[ ! -f "$ROOT/$path" ]]; then
            errors+=("$rel:$lineno: target file not found: $path")
            continue
        fi
        if ! git -C "$ROOT" ls-files --error-unmatch -- "$path" >/dev/null 2>&1; then
            errors+=("$rel:$lineno: target file untracked: $path")
            continue
        fi
        if [[ -n "$frag" ]] && ! heading_present "$ROOT/$path" "$frag"; then
            errors+=("$rel:$lineno: §heading not found in $path: §$frag")
        fi
    done <<< "$out"
done < <(spec_comment_surface "$ROOT")

# spec: canon-kit/SPEC.md §check-spec-pointer — the prose-citation pass:
# heading_present in prefix mode over the manifest set; an untracked cited path
# is out of scope (path liveness stays with check-md-refs / check-kit-ref-liveness).
mapfile -t manifest_files < <(spec_manifest_files "$ROOT")
prose_cites=0
manifests=0
if [[ ${#manifest_files[@]} -gt 0 ]]; then
    manifests=${#manifest_files[@]}
    prose_out="$(awk "$PROSE_EXTRACT" "${manifest_files[@]}")"; st=$?
    fail_closed "$st" SPEC-POINTER "awk prose extract"
    while IFS=$'\t' read -r pfile plineno ppath pfrag; do
        [[ -n "$ppath" ]] || continue
        [[ -f "$ROOT/$ppath" ]] || continue
        git -C "$ROOT" ls-files --error-unmatch -- "$ppath" >/dev/null 2>&1 || continue
        prose_cites=$((prose_cites + 1))
        [[ -n "$pfrag" ]] || continue
        if ! heading_present "$ROOT/$ppath" "$pfrag" prefix; then
            prel="${pfile#"$ROOT"/}"; prel="${prel#./}"
            errors+=("$prel:$plineno: §heading not found in $ppath: §${pfrag:0:50}")
        fi
    done <<< "$prose_out"
fi

if [[ ${#errors[@]} -gt 0 ]]; then
    echo "SPEC-POINTER: ${#errors[@]} dangling pointer(s):"
    printf '  %s\n' "${errors[@]}"
    echo "  help: a spec:/contract: directive and a free-prose <path>.md §<heading> citation each bind a site to the requirement that governs it — the binding is only live if it resolves. Fix the <path> (repo-relative, tracked) or the §<heading> to name the current target, or drop the § fragment for a file-only pointer. A renamed heading updates every inbound pointer and citation in the same commit."
    exit 1
fi
echo "SPEC-POINTER: clean ($pointers directive pointer(s) across $scanned governed source(s); $prose_cites prose citation(s) across $manifests manifest file(s); every target file tracked and named §heading present)"
exit 0
