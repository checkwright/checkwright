#!/usr/bin/env bash
# graph: couples=*SPEC*.md,*README.md,CLAUDE.md dir=one valve=none tier=precommit
# spec: spec-kit/SPEC.md §check-manifest-count — no bare cardinal quantifying a governed collection noun in manifest prose outside an exempt site
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

ROOT="${1:-.}"
[[ -d "$ROOT" ]] || { echo "check-manifest-count: not a directory: $ROOT" >&2; exit 2; }

mapfile -t manifests < <(spec_manifest_files "$ROOT" | sed 's#^\./##' | sort -u)
[[ ${#manifests[@]} -eq 0 ]] && { echo "MANIFEST-COUNT: clean (0 manifest file(s) found)"; exit 0; }

phraselist=""
if [[ ${#SPEC_KIT_COUNT_ALLOWED_PHRASES[@]} -gt 0 ]]; then
    phraselist="$(printf '%s\n' "${SPEC_KIT_COUNT_ALLOWED_PHRASES[@]}" | tr '[:upper:]' '[:lower:]')"
fi

out="$(awk -v qre="$(spec_count_quantifier_re)" -v rre="$(spec_count_range_re)" -v phraselist="$phraselist" '
    function phrase_exempt(low, ms, me,   i, p, lp, start, idx, pp) {
        for (i = 1; i <= np; i++) {
            p = phrases[i]; if (p == "") continue
            lp = length(p); start = 1
            while (1) {
                idx = index(substr(low, start), p)
                if (idx == 0) break
                pp = start + idx - 1
                if (ms >= pp && me <= pp + lp - 1) return 1
                start = pp + 1
            }
        }
        return 0
    }
    function report(low, scan, re, quantifier,   rest, off, ms, ml, me, bc, ac, ok, m, prefix, suffix) {
        rest = low; off = 0
        while (match(rest, re) > 0) {
            ms = off + RSTART; ml = RLENGTH; me = ms + ml - 1
            bc = (ms > 1) ? substr(low, ms - 1, 1) : " "
            ac = (me < length(low)) ? substr(low, me + 1, 1) : " "
            ok = 1
            if (bc ~ /[[:alnum:]]/) ok = 0          # match glued to a preceding word or number
            if (ac ~ /[[:alnum:]-]/) ok = 0         # noun glued to a following word (e.g. gatekeepers)
            if (ok) {
                m = substr(low, ms, ml)
                prefix = substr(low, 1, ms - 1)
                suffix = substr(low, me + 1)
                if (phrase_exempt(low, ms, me)) ok = 0
                else if (quantifier) {
                    if (prefix ~ /(≥|≤|>|<|at least|at most|up to|more than|fewer than)[[:space:]]*$/) ok = 0
                    else if (prefix ~ /all but[[:space:]]*$/) ok = 0
                    else if (prefix ~ /(^|[^[:alnum:]])of[[:space:]]+(the[[:space:]]+)?$/) ok = 0
                    else if (m ~ /(^|[[:space:]])of([[:space:]]|$)/) ok = 0
                    else if (suffix ~ /^[[:space:]]+per([[:space:]]|$)/) ok = 0
                }
            }
            if (ok) {
                printf "  %s:%d  restated collection total: %s\n", FILENAME, FNR, substr(scan, ms, ml)
                return 1
            }
            off = ms; rest = substr(low, ms + 1)
        }
        return 0
    }
    BEGIN { np = split(phraselist, phrases, "\n") }
    FNR == 1 { in_fence = 0; prev = "" }
    {
        raw = $0
        if (raw ~ /^[[:space:]]*```/) { in_fence = !in_fence; prev = raw; next }
        if (in_fence) { prev = raw; next }
        if (raw ~ /manifest-count-exempt:/ || prev ~ /manifest-count-exempt:/) { prev = raw; next }
        scan = raw
        gsub(/`[^`]*`/, "", scan)   # a cardinal in inline code is a meta-reference, not a restated total
        low = tolower(scan)
        if (!report(low, scan, qre, 1)) report(low, scan, rre, 0)
        prev = raw
    }
' "${manifests[@]}")"; st=$?
fail_closed "$st" check-manifest-count awk

if [[ -n "$out" ]]; then
    echo "check-manifest-count: bare count(s) quantifying a governed collection in manifest prose — the count's owner is the collection, a restated total drifts silently:"
    echo ""
    echo "$out"
    echo "  help: reword to cite the owning collection (e.g. 'the gates in gates.list') rather than pin a total; a genuinely fixed named set joins SPEC_KIT_COUNT_ALLOWED_PHRASES; a threshold/rate/partition/proportion is already exempt; else add a 'manifest-count-exempt: <reason>' comment on the line or the one above"
    exit 1
fi
echo "MANIFEST-COUNT: clean (${#manifests[@]} manifest file(s); no bare cardinal quantifying a governed collection in prose outside an exempt site)"
exit 0
