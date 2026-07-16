#!/usr/bin/env bash
# graph: couples=docs/*.md dir=one valve=none tier=precommit
# spec: canon-kit/SPEC.md §check-prose-tells — the mechanical AI-prose tells over the consumer-configured prose surfaces, each threshold-gated, with the prose-tell-exempt valve
#
# usage: check-prose-tells.sh [scan-root]   (default '.')
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

ROOT="${1:-.}"
[[ -d "$ROOT" ]] || { echo "check-prose-tells: not a directory: $ROOT" >&2; exit 2; }

# spec: canon-kit/SPEC.md §check-prose-tells — the scanned surfaces are the consumer's CANON_KIT_PROSE_TELL_GLOBS; empty ⇒ nothing scanned, a clean pass (the unconfigured-consumer no-op)
declare -a FILES=()
if [[ ${#CANON_KIT_PROSE_TELL_GLOBS[@]} -gt 0 ]]; then
    shopt -s nullglob globstar
    for g in "${CANON_KIT_PROSE_TELL_GLOBS[@]}"; do
        for f in "$ROOT"/$g; do [[ -f "$f" ]] && FILES+=("$f"); done
    done
    shopt -u nullglob globstar
fi
if [[ ${#FILES[@]} -eq 0 ]]; then
    echo "PROSE-TELLS: clean (0 configured surface(s); nothing scanned)"
    exit 0
fi
mapfile -t FILES < <(printf '%s\n' "${FILES[@]}" | sort -u)

PHRASES_NL="$(printf '%s\n' "${CANON_KIT_PROSE_TELL_PHRASES[@]}")"
ABBR_NL="$(printf '%s\n' "${CANON_KIT_PROSE_TELL_ABBR_ALLOW[@]}")"

read -r -d '' HOOKS <<'AWK' || true
BEGIN {
    n = split(PHRASES, _ph, "\n"); NPH = 0
    for (i = 1; i <= n; i++) { if (_ph[i] == "") continue; NPH++; PHO[NPH] = _ph[i]; PHL[NPH] = tolower(_ph[i]) }
    n = split(ABBR, _ab, "\n")
    for (i = 1; i <= n; i++) { if (_ab[i] == "") continue; ALLOWSET[_ab[i]] = 1 }
    sec_startline = 1; sec_buf = ""; file_buf = ""; in_gen = 0
}
function emit(ln, code, msg) { print FILENAME ":" ln ": [" code "] " msg }

# what the tells measure is authored reader-facing prose, so three non-prose
# surfaces are held out: inline `code` spans (identifiers, filenames, env-vars),
# markdown table rows (data layout), and generated <!-- x:begin -->..<!-- x:end -->
# regions (byte-gated elsewhere; a prose gate that forced edits to generated
# content would contradict its generation, the docs/posts rationale).
function strip_code(s,   t) { t = s; gsub(/`[^`]*`/, " ", t); return t }
function is_table(s)  { return (s ~ /^[ \t]*\|/) }
# a markdown list item (- / * / + / N.) is its own unit, not flowing prose: a
# definition list carrying one em-dash per item is well-formed markdown, so the
# paragraph/section assertions must not lump items into one blank-line block. In
# the paragraph buffer each item flushes A/B/E on its own span (sk_on_pflush); in
# the section buffer a leading "." boundary keeps the C/F regexes from matching
# across two items.
function is_list_item(s) { return (s ~ /^[ \t]*([-*+]|[0-9]+\.)[ \t]/) }
function is_gen_begin(s) { return (s ~ /<!--[ \t]*[A-Za-z0-9_-]+:begin[ \t]*-->/) }
function is_gen_end(s)   { return (s ~ /<!--[ \t]*[A-Za-z0-9_-]+:end[ \t]*-->/) }

# the walk driver feeds prose lines here (exempt/fenced/blank already dropped);
# a heading is never prose — a level-1/2 head closes the section span (F, C),
# any head is kept out of the paragraph (A, E) and file (D) buffers.
function sk_on_line(file, fnr, raw,   clean) {
    if (is_gen_begin(raw)) { in_gen = 1; return }
    if (is_gen_end(raw))   { in_gen = 0; return }
    if (in_gen) return
    if (raw ~ /^[ \t]*#{1,6}[ \t]/) {
        if (raw ~ /^[ \t]*#{1,2}[ \t]/) { flush_section(); sec_startline = fnr }
        return
    }
    if (is_table(raw)) return
    clean = strip_code(raw)
    if (is_list_item(raw)) { sec_buf = sec_buf " ."; file_buf = file_buf " ." }
    sec_buf = sec_buf " " clean
    file_buf = file_buf " " clean
}
# each blank-line paragraph is unitized: the run before the first list item and
# every list item (with its continuation lines) is a separate A/B/E unit.
function sk_on_pflush(   first, i, ustart) {
    if (sk_pn == 0 || in_gen) return
    first = sk_pline[1]
    if (first ~ /^[ \t]*#{1,6}[ \t]/ || is_table(first)) return
    ustart = 1
    for (i = 2; i <= sk_pn; i++)
        if (is_list_item(sk_pline[i])) { pflush_unit(ustart, i - 1); ustart = i }
    pflush_unit(ustart, sk_pn)
}
function pflush_unit(lo, hi,   para) {
    para = strip_code(_sk_join(lo, hi))
    a_emdash(para, sk_pfnr[lo])
    b_phrases(para, sk_pfnr[lo])
    e_rhythm(para, sk_pfnr[lo])
}

# assertion A: em-dash density — a paragraph over the em-dash cap
function a_emdash(para, ln,   n) {
    n = gsub(EMD, "", para)
    if (n > EMDASH_MAX) emit(ln, "A", "em-dash density (" n " em-dashes > " EMDASH_MAX ") in a paragraph")
}
# assertion B: throat-clearing phrases — any bundled/consumer phrase, case-insensitive
function b_phrases(para, ln,   i, lc) {
    lc = tolower(para)
    for (i = 1; i <= NPH; i++)
        if (index(lc, PHL[i]) > 0) emit(ln, "B", "throat-clearing phrase \"" PHO[i] "\"")
}
# assertion E: sentence-rhythm variance — >= min sentences, word-count CV below the floor
function e_rhythm(para, ln,   ns, i, sum, mean, sq, sd, cv) {
    ns = split_sentences(para)
    if (ns < RHYTHM_MIN) return
    sum = 0
    for (i = 1; i <= ns; i++) sum += SW[i]
    mean = sum / ns
    if (mean <= 0) return
    sq = 0
    for (i = 1; i <= ns; i++) sq += (SW[i] - mean) * (SW[i] - mean)
    sd = sqrt(sq / ns)
    cv = sd / mean
    if (cv < RHYTHM_CV_MIN)
        emit(ln, "E", sprintf("metronomic rhythm (%d sentences, word-count CV %.3f < %.2f)", ns, cv, RHYTHM_CV_MIN))
}
function split_sentences(p,   tmp, parts, m, k, seg, cnt) {
    tmp = p
    gsub(/[.!?]+[ \t]/, "\001", tmp)
    sub(/[.!?]+[ \t]*$/, "\001", tmp)
    m = split(tmp, parts, /\001/)
    cnt = 0
    for (k = 1; k <= m; k++) {
        seg = parts[k]; gsub(/^[ \t]+|[ \t]+$/, "", seg)
        if (seg == "") continue
        cnt++; SW[cnt] = wc(seg)
    }
    return cnt
}
function wc(s,   a) { if (s == "") return 0; return split(s, a, /[ \t]+/) }

# section-span assertions (C, F): flushed at each level-1/2 heading and at END
function flush_section(   lc, sec, n) {
    if (sec_buf == "") return
    lc = tolower(sec_buf)
    # assertion C: contrast cadence — the "not X — it's Y" shape over the cap per section
    n = gsub(/not[^.]*(—|, but)[^.]*it('s| is)/, "", lc)
    if (n > CONTRAST_MAX) emit(sec_startline, "C", "contrast cadence (" n " \"not X, it's Y\" turns > " CONTRAST_MAX ") in a section")
    sec = sec_buf
    # assertion F: tricolon density — "A, B, and C" lists of three over the cap per section
    n = gsub(/[A-Za-z][^,.]*, [^,.]*, and [^,.]*[A-Za-z]/, "", sec)
    if (n > TRICOLON_MAX) emit(sec_startline, "F", "tricolon density (" n " \"A, B, and C\" triples > " TRICOLON_MAX ") in a section")
    sec_buf = ""
}

# assertion D: undefined abbreviations — an all-caps token (>=3) with no parenthesized expansion anywhere in the file and absent from the allow-list
function flush_file(   s, tok, st_, len_, before) {
    s = file_buf
    delete SEEN
    while (match(s, /[A-Z][A-Z0-9][A-Z0-9]+/)) {
        st_ = RSTART; len_ = RLENGTH
        before = (st_ > 1) ? substr(s, st_ - 1, 1) : " "
        tok = substr(s, st_, len_)
        s = substr(s, st_ + len_)
        if (before ~ /[A-Za-z0-9]/) continue
        if (tok in SEEN) continue
        SEEN[tok] = 1
        if (tok in ALLOWSET) continue
        if (is_defined(tok)) continue
        emit(1, "D", "undefined abbreviation \"" tok "\" (never expanded in-file, not in the allow-list)")
    }
}
function is_defined(tok,   re) { re = "\\(" tok "|" tok "[ ]*\\("; return (file_buf ~ re) }

END { flush_section(); flush_file() }
AWK

AWKSRC="$(spec_para_accum_awk)
$(spec_manifest_walk_awk)
$HOOKS"

findings_raw=""
scanned=0
for f in "${FILES[@]}"; do
    out="$(awk \
        -v EMD="—" \
        -v EMDASH_MAX="$CANON_KIT_PROSE_TELL_EMDASH_MAX" \
        -v CONTRAST_MAX="$CANON_KIT_PROSE_TELL_CONTRAST_MAX" \
        -v RHYTHM_MIN="$CANON_KIT_PROSE_TELL_RHYTHM_MIN_SENTENCES" \
        -v RHYTHM_CV_MIN="$CANON_KIT_PROSE_TELL_RHYTHM_CV_MIN" \
        -v TRICOLON_MAX="$CANON_KIT_PROSE_TELL_TRICOLON_MAX" \
        -v PHRASES="$PHRASES_NL" \
        -v ABBR="$ABBR_NL" \
        -v SK_EXEMPT="prose-tell-exempt:" \
        "$AWKSRC" "$f")"; st=$?
    fail_closed "$st" check-prose-tells "awk ($f)"
    [[ -n "$out" ]] && findings_raw+="$out"$'\n'
    scanned=$((scanned + 1))
done

findings="$(printf '%s' "$findings_raw" | grep -v '^[[:space:]]*$' | sort -u || true)"
if [[ -n "$findings" ]]; then
    count="$(printf '%s\n' "$findings" | grep -c .)"
    echo "check-prose-tells: $count mechanical AI-prose tell(s) across $scanned configured surface(s):"
    printf '%s\n' "$findings"
    echo "  help: rewrite the flagged prose — break the em-dash-dense paragraph, cut the throat-clearing opener, vary sentence length, spell out the abbreviation once, thin the contrast/tricolon cadence — or, for a deliberate keep, tag '<!-- prose-tell-exempt: <reason> -->' on the flagged line or directly above it (a reason is mandatory). Thresholds are the CANON_KIT_PROSE_TELL_* knobs (canon-kit/SPEC.md §Layout and configuration)."
    exit 1
fi
echo "PROSE-TELLS: clean ($scanned configured surface(s); no mechanical AI-prose tell tripped)"
exit 0
