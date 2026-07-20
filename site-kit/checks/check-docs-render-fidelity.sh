#!/usr/bin/env bash
# graph: couples=docs/*.md,docs/*/index.md,docs/posts/*.md dir=one valve=none tier=precommit
# spec: site-kit/SPEC.md §check-docs-render-fidelity — every tracked docs markdown page, rendered through the pinned Pages parser, leaks no code-span corruption symptom (a stray backtick or a raw non-HTML-element tag) into text, promotes no code-fenced heading, and renders no fewer tables than its source GFM table starts; a missing renderer fails closed
#
# usage: check-docs-render-fidelity.sh [docs-dir] [config-file]
#   defaults SITE_KIT_DOCS_DIR; config-file overrides SITE_KIT_CONFIG_FILE so a
#   fixture supplies its own SITE_KIT_RENDERER / docs dir.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

DOCSARG="${1:-}"; CONFIGARG="${2:-}"
[[ -n "$CONFIGARG" ]] && export SITE_KIT_CONFIG_FILE="$CONFIGARG"
# shellcheck source=../lib/site.sh
source "$KIT/lib/site.sh"

DOCS="${DOCSARG:-$SITE_KIT_DOCS_DIR}"; DOCS="${DOCS%/}"

git rev-parse --git-dir >/dev/null 2>&1 || {
    echo "check-docs-render-fidelity: not a git repository — cannot enumerate tracked pages" >&2; exit 2; }
[[ -d "$DOCS" ]] || { echo "check-docs-render-fidelity: docs dir not found: $DOCS" >&2; exit 2; }

probe="$(printf '# probe\n' | "${SITE_KIT_RENDERER[@]}" 2>/dev/null)"; pst=$?
if [[ "$pst" -ne 0 || -z "$probe" ]]; then
    echo "check-docs-render-fidelity: renderer '${SITE_KIT_RENDERER[*]}' could not run (exit $pst)" >&2
    echo "  help: install the pinned Pages parser — ruby plus the kramdown-parser-gfm gem — or point" >&2
    echo "        SITE_KIT_RENDERER at a stdin->stdout GFM-to-HTML command" >&2
    exit 2
fi

listing="$(git ls-files -- "$DOCS")"; st=$?
fail_closed "$st" DOCS-RENDER-FIDELITY git-ls-files

pages=()
while IFS= read -r p; do
    [[ -n "$p" ]] || continue
    [[ "$p" == *.md ]] || continue
    case "/$p/" in */_*/*) continue ;; esac   # exclude every underscore-prefixed dir segment (Jekyll internals)
    gate_path_pruned "$p" && continue
    [[ -f "$p" ]] && pages+=("$p")
done <<< "$listing"

if [[ ${#pages[@]} -eq 0 ]]; then
    echo "DOCS-RENDER-FIDELITY: clean (0 tracked markdown page(s) under $DOCS)"
    exit 0
fi

strip_fm='NR==1 && $0=="---" { fm=1; next } fm==1 && $0=="---" { fm=0; next } fm==1 { next } { print }'

rendered_scan='
BEGIN {
    inpre=0; leak=0; h=0; tbl=0; fgn=0
    # spec: site-kit/SPEC.md §check-docs-render-fidelity — the known-HTML-element set is a kit built-in, not a config seam
    split("a abbr address area article aside audio b base bdi bdo blockquote body br button canvas caption cite code col colgroup data datalist dd del details dfn dialog div dl dt em embed fieldset figcaption figure footer form h1 h2 h3 h4 h5 h6 head header hgroup hr html i iframe img input ins kbd label legend li link main map mark menu meta meter nav noscript object ol optgroup option output p param picture pre progress q rp rt ruby s samp script search section select slot small source span strong style sub summary sup table tbody td template textarea tfoot th thead time title tr track u ul var video wbr", A, " ")
    for (i in A) HTMLEL[A[i]]=1
}
{
    t=$0
    while (match(t, /<h[1-6][ >]/)) { h++; t=substr(t, RSTART+RLENGTH) }
    u=$0
    while (match(u, /<table[ >]/)) { tbl++; u=substr(u, RSTART+RLENGTH) }
    if (inpre) { if ($0 ~ /<\/pre>/) inpre=0; next }
    if ($0 ~ /<pre/) { inpre=1; next }
    s=$0
    gsub(/<code[^>]*>[^<]*<\/code>/, "", s)
    # spec: site-kit/SPEC.md §check-docs-render-fidelity — symptom (b), scanned before the tag-strip below so the placeholder tag is still present
    w=s
    while (match(w, /<\/?[A-Za-z][A-Za-z0-9]*[^>]*>/)) {
        nm=substr(w, RSTART, RLENGTH); w=substr(w, RSTART+RLENGTH)
        closing=(nm ~ /^<\//); selfclose=(nm ~ /\/>[ \t]*$/)
        sub(/^<\/?/, "", nm); sub(/[^A-Za-z0-9].*/, "", nm)
        nm=tolower(nm)
        # spec: site-kit/SPEC.md §check-docs-render-fidelity — foreign-content subtree, tracked by depth so an unknown name inside it is legitimate SVG/MathML vocabulary rather than a leaked placeholder
        if (nm == "svg" || nm == "math") {
            if (closing) { if (fgn > 0) fgn-- } else if (!selfclose) fgn++
            continue
        }
        if (fgn > 0) continue
        if (!(nm in HTMLEL)) leak=1
    }
    # spec: site-kit/SPEC.md §check-docs-render-fidelity — symptom (a), any literal backtick, not only a fence run
    gsub(/<[^>]*>/, "", s)
    if (s ~ /`/) leak=1
}
END { print leak+0, h+0, tbl+0 }'

source_headings='
BEGIN { infence=0; fchar=""; flen=0; count=0; prevblank=1; previsatx=0 }
{
    s=$0; n=0
    while (substr(s,1,1)==" ") { s=substr(s,2); n++ }
    isfence=0; fc=""; fl=0
    if (n<=3 && (s ~ /^`{3,}/ || s ~ /^~{3,}/)) {
        fc=substr(s,1,1); m=s
        while (substr(m,1,1)==fc) { fl++; m=substr(m,2) }
        isfence=1
    }
    if (infence) {
        if (isfence && fc==fchar && fl>=flen && s ~ ("^" fchar "{" flen ",}[ \t]*$")) infence=0
        prevblank=($0 ~ /^[ \t]*$/); previsatx=0; next
    }
    if (isfence) { infence=1; fchar=fc; flen=fl; prevblank=0; previsatx=0; next }
    if (n<=3 && (s ~ /^=+[ \t]*$/ || s ~ /^-+[ \t]*$/) && prevblank==0 && previsatx==0) {
        count++; prevblank=1; previsatx=0; next
    }
    if (n<=3 && s ~ /^#{1,6}([ \t]|$)/) { count++; prevblank=0; previsatx=1; next }
    prevblank=($0 ~ /^[ \t]*$/); previsatx=0
}
END { print count+0 }'

# spec: site-kit/SPEC.md §check-docs-render-fidelity — count source GFM table starts (a pipe row followed by a | --- | delimiter row) outside code fences
source_tables='
BEGIN { infence=0; fchar=""; flen=0; count=0; prevpipe=0 }
{
    s=$0; n=0
    while (substr(s,1,1)==" ") { s=substr(s,2); n++ }
    isfence=0; fc=""; fl=0
    if (n<=3 && (s ~ /^`{3,}/ || s ~ /^~{3,}/)) {
        fc=substr(s,1,1); m=s
        while (substr(m,1,1)==fc) { fl++; m=substr(m,2) }
        isfence=1
    }
    if (infence) {
        if (isfence && fc==fchar && fl>=flen && s ~ ("^" fchar "{" flen ",}[ \t]*$")) infence=0
        prevpipe=0; next
    }
    if (isfence) { infence=1; fchar=fc; flen=fl; prevpipe=0; next }
    isdelim = (n<=3 && s ~ /^\|?[ \t:|-]*-[ \t:|-]*$/ && s ~ /\|/)
    if (isdelim && prevpipe==1) { count++; prevpipe=0; next }
    if ($0 ~ /^[ \t]*$/) { prevpipe=0; next }
    prevpipe = (index($0, "|") > 0) ? 1 : 0
}
END { print count+0 }'

findings=()
for page in "${pages[@]}"; do
    body="$(awk "$strip_fm" "$page")"; bst=$?
    fail_closed "$bst" DOCS-RENDER-FIDELITY front-matter-strip
    html="$(printf '%s\n' "$body" | "${SITE_KIT_RENDERER[@]}")"; rst=$?
    fail_closed "$rst" DOCS-RENDER-FIDELITY renderer

    read -r leak rcount rtbl < <(printf '%s\n' "$html" | awk "$rendered_scan"); ast=$?
    fail_closed "$ast" DOCS-RENDER-FIDELITY rendered-scan
    scount="$(printf '%s\n' "$body" | awk "$source_headings")"; sst=$?
    fail_closed "$sst" DOCS-RENDER-FIDELITY source-scan
    stbl="$(printf '%s\n' "$body" | awk "$source_tables")"; tst=$?
    fail_closed "$tst" DOCS-RENDER-FIDELITY source-table-scan

    [[ "$leak" -eq 1 ]] && findings+=("$page: a code-span corruption symptom (a stray backtick, or a raw non-HTML-element tag) leaked into rendered text — a code span or fenced block failed to parse")
    [[ "$rcount" -gt "$scount" ]] && findings+=("$page: $rcount rendered heading(s) exceed $scount source heading(s) outside code — a code-fenced '#' line was promoted")
    [[ "$rtbl" -lt "$stbl" ]] && findings+=("$page: $rtbl rendered table(s) fall short of $stbl source GFM table start(s) — a table collapsed into literal-pipe paragraph text (a row abutting a non-blank line)")
done

if [[ ${#findings[@]} -gt 0 ]]; then
    echo "check-docs-render-fidelity: rendered docs page(s) diverge from source under the pinned Pages parser:"
    printf '  %s\n' "${findings[@]}"
    echo "  help: restructure the offending block so kramdown's GFM parser renders it faithfully — an"
    echo "        indented (4-space) code block avoids the consecutive-fence and unclosed-fence leakage"
    echo "        class; a doubled-backtick code span kept on one line (never split across a newline"
    echo "        before a <word> token) avoids the severed-span class."
    exit 1
fi
echo "DOCS-RENDER-FIDELITY: clean (${#pages[@]} tracked markdown page(s) under $DOCS render with no span-corruption, heading, or table leakage)"
exit 0
