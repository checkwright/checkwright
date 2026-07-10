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

# spec: spec-kit/SPEC.md §check-manifest-count — prose walk only: the shared driver gates fences and per-site markers, these hooks judge the line (sk_count_hit) and the wrapped paragraph (sk_para_wrapped)
read -r -d '' HOOKS <<'AWK' || true
function sk_on_line(file, fnr, raw,   hit) {
    hit = sk_count_hit(raw)
    if (hit != "") printf "  %s:%d  restated collection total: %s\n", file, fnr, hit
}
function sk_on_pflush() {
    if (sk_para_wrapped()) printf "  %s:%d  restated collection total: %s\n", sk_curfile, SK_WRAP_FNR, SK_WRAP_SPAN
}
AWK

AWKSRC="$(spec_para_accum_awk)
$(spec_count_awk_lib)
$(spec_manifest_walk_awk)
$HOOKS"

out="$(awk -v SK_QRE="$(spec_count_quantifier_re)" -v SK_RRE="$(spec_count_range_re)" -v SK_PHRASES="$(spec_count_phraselist)" -v SK_EXEMPT="manifest-count-exempt:" "$AWKSRC" "${manifests[@]}")"; st=$?
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
