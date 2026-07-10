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

# spec: spec-kit/SPEC.md §check-manifest-count — prose walk only: fences and per-site markers gate the line, the shared adapter judges it
read -r -d '' SCAN <<'AWK' || true
function pflush() {
    if (sk_para_wrapped()) printf "  %s:%d  restated collection total: %s\n", curfile, SK_WRAP_FNR, SK_WRAP_SPAN
    sk_para_reset()
}
FNR == 1 { pflush(); in_fence = 0; prev = "" }
{
    curfile = FILENAME
    raw = $0
    if (raw ~ /^[[:space:]]*```/) { pflush(); in_fence = !in_fence; prev = raw; next }
    if (in_fence) { pflush(); prev = raw; next }
    if (raw ~ /manifest-count-exempt:/ || prev ~ /manifest-count-exempt:/) { pflush(); prev = raw; next }
    if (raw ~ /^[[:space:]]*$/) { pflush(); prev = raw; next }   # a blank line ends the paragraph
    hit = sk_count_hit(raw)
    if (hit != "") printf "  %s:%d  restated collection total: %s\n", FILENAME, FNR, hit
    sk_para_add(FNR, raw)
    prev = raw
}
END { pflush() }
AWK

AWKSRC="$(spec_count_awk_lib)
$SCAN"

out="$(awk -v SK_QRE="$(spec_count_quantifier_re)" -v SK_RRE="$(spec_count_range_re)" -v SK_PHRASES="$(spec_count_phraselist)" "$AWKSRC" "${manifests[@]}")"; st=$?
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
