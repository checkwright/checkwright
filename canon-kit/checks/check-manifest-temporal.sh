#!/usr/bin/env bash
# graph: couples=*SPEC*.md,*README.md,CLAUDE.md dir=one valve=none tier=precommit
# spec: canon-kit/SPEC.md §check-manifest-temporal — no temporal-narration marker in governed manifest prose outside an exempt site
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

ROOT="${1:-.}"
[[ -d "$ROOT" ]] || { echo "check-manifest-temporal: not a directory: $ROOT" >&2; exit 2; }

mapfile -t manifests < <(spec_manifest_files "$ROOT" | sed 's#^\./##' | sort -u)
[[ ${#manifests[@]} -eq 0 ]] && { echo "MANIFEST-TEMPORAL: clean (0 manifest file(s) found)"; exit 0; }

# spec: canon-kit/SPEC.md §check-manifest-temporal — path valve: a whole file
# whose immutable dated narrative a heading name cannot address (dated posts).
exempt_n=0
if [[ ${#CANON_KIT_TEMPORAL_EXEMPT_PATHS[@]} -gt 0 ]]; then
    kept=()
    for _m in "${manifests[@]}"; do
        _ex=0
        for _g in "${CANON_KIT_TEMPORAL_EXEMPT_PATHS[@]}"; do
            # shellcheck disable=SC2053  # $_g is the exempt path glob, matched unquoted on purpose
            [[ "$_m" == $_g ]] && { _ex=1; break; }
        done
        if [[ $_ex -eq 1 ]]; then exempt_n=$((exempt_n + 1)); else kept+=("$_m"); fi
    done
    manifests=("${kept[@]+"${kept[@]}"}")
    [[ ${#manifests[@]} -eq 0 ]] && { echo "MANIFEST-TEMPORAL: clean ($exempt_n path-exempt, no other manifest)"; exit 0; }
fi

markerlist="$(printf '%s\n' "${CANON_KIT_TEMPORAL_MARKERS[@]}")"
exemptset=$'\x01'
for _s in "${CANON_KIT_TEMPORAL_EXEMPT_SECTIONS[@]+"${CANON_KIT_TEMPORAL_EXEMPT_SECTIONS[@]}"}"; do
    exemptset+="$(printf '%s' "$_s" | tr '[:upper:]' '[:lower:]')"$'\x01'
done

out="$(awk -v markerlist="$markerlist" -v exemptset="$exemptset" '
    function heading_level(s,   n) { n = 0; while (substr(s, n + 1, 1) == "#") n++; return n }
    BEGIN { nm = split(markerlist, markers, "\n") }
    FNR == 1 { in_fence = 0; exempt = 0; exempt_level = 0; prev = "" }
    {
        raw = $0
        if (raw ~ /^[[:space:]]*```/) { in_fence = !in_fence; prev = raw; next }
        if (in_fence) { prev = raw; next }
        if (raw ~ /^#{1,6}[[:space:]]/) {
            lvl = heading_level(raw)
            if (exempt && lvl <= exempt_level) { exempt = 0; exempt_level = 0 }
            h = raw; sub(/^#{1,6}[[:space:]]+/, "", h); sub(/[[:space:]]+$/, "", h)
            if (index(exemptset, "\x01" tolower(h) "\x01") > 0) { exempt = 1; exempt_level = lvl }
            prev = raw; next
        }
        if (exempt) { prev = raw; next }
        if (raw ~ /manifest-temporal-exempt:/ || prev ~ /manifest-temporal-exempt:/) { prev = raw; next }
        scan = raw
        gsub(/`[^`]*`/, "", scan)   # a marker named in inline code is a meta-reference, not narration
        low = tolower(scan)
        for (i = 1; i <= nm; i++) {
            if (markers[i] != "" && low ~ markers[i]) {
                printf "  %s:%d  temporal-narration marker: %s\n", FILENAME, FNR, markers[i]
                break
            }
        }
        prev = raw
    }
' "${manifests[@]}")"; st=$?
fail_closed "$st" check-manifest-temporal awk

if [[ -n "$out" ]]; then
    echo "check-manifest-temporal: temporal-narration marker(s) in manifest prose — a manifest states current behavior; history is derivable from git:"
    echo ""
    echo "$out"
    echo "  help: reword to state the current behavior only (drop the 'formerly…' framing); if the line is legitimately about the past, add a '<!-- manifest-temporal-exempt: <reason> -->' comment on it or the line directly above; a whole provenance section rides CANON_KIT_TEMPORAL_EXEMPT_SECTIONS"
    exit 1
fi
suffix=""
[[ "$exempt_n" -gt 0 ]] && suffix=", $exempt_n path-exempt"
echo "MANIFEST-TEMPORAL: clean (${#manifests[@]} manifest file(s)${suffix}; no temporal-narration marker in governed prose outside an exempt site)"
exit 0
