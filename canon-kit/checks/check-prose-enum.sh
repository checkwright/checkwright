#!/usr/bin/env bash
# graph: couples=*SPEC*.md,*README.md,CLAUDE.md,scripts/*.sh,kit:*.sh dir=one valve=none tier=precommit
# spec: canon-kit/SPEC.md §check-prose-enum — within one manifest-prose paragraph, naming two or more members of a declared governed set must name every member, unless an exempt site holds
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

ROOT="${1:-.}"
[[ -d "$ROOT" ]] || { echo "check-prose-enum: not a directory: $ROOT" >&2; exit 2; }

if [[ -z "$CANON_KIT_ENUM_SETS_CMD" ]]; then
    echo "PROSE-ENUM: clean (CANON_KIT_ENUM_SETS_CMD unset — no declared sets to check)"
    exit 0
fi

sets="$(spec_enum_sets)"; st=$?
fail_closed "$st" check-prose-enum "CANON_KIT_ENUM_SETS_CMD"
[[ -n "$sets" ]] || { echo "PROSE-ENUM: clean (CANON_KIT_ENUM_SETS_CMD declared no members)"; exit 0; }

mapfile -t manifests < <(spec_manifest_files "$ROOT" | sed 's#^\./##' | sort -u)
[[ ${#manifests[@]} -eq 0 ]] && { echo "PROSE-ENUM: clean (0 manifest file(s) found)"; exit 0; }

# spec: canon-kit/SPEC.md §check-prose-enum — the enum matcher and hooks: word-bounded member presence (bracketed or bare, neither alnum nor hyphen abutting), the delimited-adjacency test that separates a hand list from scattered mentions, the subset/partitive exempt escapes, and the per-paragraph verdict the shared driver flushes
read -r -d '' MATCH <<'AWK' || true
BEGIN {
    _ns = split(SK_SETS, _sl, "\n")
    _nset = 0
    for (_i = 1; _i <= _ns; _i++) {
        if (_sl[_i] == "") continue
        _tp = index(_sl[_i], "\t")
        _sn = substr(_sl[_i], 1, _tp - 1)
        _mb = substr(_sl[_i], _tp + 1)
        if (!(_sn in setidx)) { setidx[_sn] = ++_nset; setname[_nset] = _sn }
        _c = ++mcount[_sn]
        morig[_sn, _c] = _mb
        mlow[_sn, _c] = tolower(_mb)
    }
    SK_SUBSET_RE = "e\\.g\\.|such as|among them"
    SK_PARTITIVE_RE = "(one|some|several|any|each|few|many)[[:space:]]+of[[:space:]]*$|of[[:space:]]+(the[[:space:]]+)?(set|these|those|them)[[:space:]]*$"
}
function _sk_present(low, m,   lm, start, idx, pp, bc, ac) {
    lm = length(m); start = 1
    while (1) {
        idx = index(substr(low, start), m)
        if (idx == 0) return 0
        pp = start + idx - 1
        bc = (pp > 1) ? substr(low, pp - 1, 1) : " "
        ac = (pp + lm <= length(low)) ? substr(low, pp + lm, 1) : " "
        if (bc !~ /[[:alnum:]-]/ && ac !~ /[[:alnum:]-]/) return pp
        start = pp + 1
    }
}
function _sk_adjacent(gap,   g) {
    if (length(gap) <= 8) { g = gap; gsub(/[][ \t,\/:().`|]/, "", g); if (g == "") return 1 }
    if (length(gap) <= 16 && gap ~ /^[^[:alpha:]]*(and|or)[^[:alpha:]]*$/) return 1
    return 0
}
function sk_on_line(file, fnr, raw) { }
function sk_on_pflush(   text, low, si, sn, k, pos, npres, ppos, plen, missn, miss, msg, pre, i, j, v, w, run, maxrun, gs, ge, gap) {
    if (sk_pn < 1) return
    text = _sk_join(1, sk_pn)
    low = tolower(text)
    for (si = 1; si <= _nset; si++) {
        sn = setname[si]
        npres = 0; missn = 0; delete ppos; delete plen; delete miss
        for (k = 1; k <= mcount[sn]; k++) {
            pos = _sk_present(low, mlow[sn, k])
            if (pos > 0) { npres++; ppos[npres] = pos; plen[npres] = length(mlow[sn, k]) }
            else miss[++missn] = morig[sn, k]
        }
        if (npres < 2 || missn == 0) continue
        for (i = 2; i <= npres; i++) {            # insertion-sort the match positions ascending
            v = ppos[i]; w = plen[i]; j = i - 1
            while (j >= 1 && ppos[j] > v) { ppos[j + 1] = ppos[j]; plen[j + 1] = plen[j]; j-- }
            ppos[j + 1] = v; plen[j + 1] = w
        }
        maxrun = 1; run = 1                        # longest run of members chained by list separators
        for (i = 2; i <= npres; i++) {
            gs = ppos[i - 1] + plen[i - 1]; ge = ppos[i] - 1
            gap = (ge >= gs) ? substr(low, gs, ge - gs + 1) : ""
            if (_sk_adjacent(gap)) { run++; if (run > maxrun) maxrun = run } else run = 1
        }
        if (maxrun < 2) continue                   # scattered mentions, not a hand list
        if (low ~ SK_SUBSET_RE) continue
        pre = substr(low, (ppos[1] > 32 ? ppos[1] - 32 : 1), (ppos[1] > 32 ? 32 : ppos[1] - 1))
        if (pre ~ SK_PARTITIVE_RE) continue
        msg = miss[1]
        for (k = 2; k <= missn; k++) msg = msg ", " miss[k]
        printf "  %s:%d  set '%s' lists %d of %d member(s) but omits: %s\n", \
            sk_curfile, sk_pfnr[1], sn, npres, npres + missn, msg
    }
}
AWK

AWKSRC="$(spec_para_accum_awk)
$(spec_manifest_walk_awk)
$MATCH"

out="$(awk -v SK_SETS="$sets" -v SK_EXEMPT="prose-enum-exempt:" "$AWKSRC" "${manifests[@]}")"; st=$?
fail_closed "$st" check-prose-enum awk

if [[ -n "$out" ]]; then
    echo "check-prose-enum: incomplete prose enumeration of a governed set — naming a subset drifts silently when the set grows:"
    echo ""
    echo "$out"
    echo "  help: cite the owning set by name, or complete the enumeration — never trim to a silent subset. A genuinely illustrative list marks itself ('e.g.', 'such as', 'among them') or is partitive ('some of the set'); a legitimately partial site takes a 'prose-enum-exempt: <reason>' comment on the line or the one above"
    exit 1
fi
echo "PROSE-ENUM: clean (${#manifests[@]} manifest file(s); every prose paragraph naming 2+ members of a declared set names them all or is exempt)"
exit 0
