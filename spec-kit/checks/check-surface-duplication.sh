#!/usr/bin/env bash
# graph: couples=GLOSSARY.md,VISION.md,*SPEC*.md dir=bi valve=none tier=align-only
# spec: spec-kit/SPEC.md §check-surface-duplication — a non-glossary surface may not carry a glossary term's bold-lead-in definition
#
# usage: check-surface-duplication.sh [scan-root]
#   Scans the configured non-glossary surfaces (SPEC_KIT_DUP_SURFACES + every
#   component spec) under the root (default '.') for foreign bold-lead-in
#   definitions of glossary terms. Exits 2 when the glossary is absent — register
#   the gate only where the topology exists.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

ROOT="${1:-.}"
[[ -d "$ROOT" ]] || { echo "check-surface-duplication: not a directory: $ROOT" >&2; exit 2; }
GLOSS="$ROOT/$SPEC_KIT_GLOSSARY_FILE"
[[ -f "$GLOSS" ]] || { echo "check-surface-duplication: no $SPEC_KIT_GLOSSARY_FILE at $GLOSS (register the gate only where the glossary topology exists)" >&2; exit 2; }

# Surfaces to scan: the configured non-glossary surfaces present at the root,
# each tagged with its introduce-valve (a spec surface uses spec-introduces,
# anything else vision-introduces), plus every component spec under the root.
declare -a SURFACES=()
COMPONENTS=""
for s in "${SPEC_KIT_DUP_SURFACES[@]}"; do
    [[ -f "$ROOT/$s" ]] || continue
    if [[ "$(basename "$s")" == "$SPEC_KIT_SPEC_NAME" ]]; then
        SURFACES+=("$ROOT/$s"$'\t'spec-introduces)
    else
        SURFACES+=("$ROOT/$s"$'\t'vision-introduces)
    fi
done
while IFS= read -r f; do
    [[ -n "$f" ]] || continue
    SURFACES+=("$f"$'\t'spec-introduces)
    comp="$(basename "$(dirname "$f")")"; [[ "$comp" == "." ]] && comp="$(basename "$(cd "$ROOT" && pwd)")"
    COMPONENTS+=$'\n'"${comp,,}"
done < <(spec_canonical_specs "$ROOT" | sort)

[[ ${#SURFACES[@]} -gt 0 ]] || { echo "SURFACE-DUPLICATION: clean (no configured surface present under $ROOT)"; exit 0; }

# The glossary term set: the Quick-reference table's canonical column and every
# bold-lead-in entry head.
terms="$(awk '
    /^## Quick reference/ { inqr = 1 }
    inqr && /^---/ { inqr = 0 }
    inqr && /^\|/ && !/^\| *Canonical/ && !/^\|[-| ]*$/ {
        n = split($0, c, "|"); cell = c[2]
        gsub(/`/, "", cell); gsub(/\([^)]*\)/, "", cell)
        m = split(cell, p, "/")
        for (i = 1; i <= m; i++) { t = p[i]; gsub(/^[[:space:]]+|[[:space:]]+$/, "", t); if (t != "") print tolower(t) }
        next
    }
    /^\*\*[^*]+\*\*/ {
        h = $0; sub(/^\*\*/, "", h); sub(/\*\*.*/, "", h)
        gsub(/`/, "", h); sub(/\.$/, "", h); sub(/[[:space:]]*$/, "", h)
        if (h != "") print tolower(h)
    }
' "$GLOSS" | sort -u)"; st=$?
fail_closed "$st" SURFACE-DUPLICATION "awk glossary term-set scan"
tcount="$(grep -c . <<< "$terms")"

scan='
    function valve_term(v,   t) {
        if (v == "") return ""
        t = v; sub(".*" vk ":[[:space:]]*", "", t); sub(/[[:space:]]*-->.*/, "", t)
        return tolower(t)
    }
    {
        line = $0
        probe = line
        sub(/^[[:space:]]*[-*][[:space:]]+/, "", probe)
        gsub(/<!--[^>]*-->/, "", probe)
        sub(/^[[:space:]]+/, "", probe)
        is_block = (prev ~ /^[[:space:]]*$/) || (line ~ /^[[:space:]]*[-*][[:space:]]/) || (prev ~ /^#/)
        if (is_block && (probe ~ /^\*\*[^*]+\.\*\*/ || probe ~ /^\*\*[^*]+\*\* —/ || probe ~ /^\*\*[^*]+\*\* -/)) {
            h = probe; sub(/^\*\*/, "", h); sub(/\*\*.*/, "", h)
            gsub(/`/, "", h); sub(/\.$/, "", h); sub(/[[:space:]]*$/, "", h)
            vt = ""
            if (line ~ vk ":") vt = valve_term(line)
            else if (prev ~ vk ":") vt = valve_term(prev)
            print tolower(h) "\t" NR "\t" vt
        }
        prev = line
    }'

errors=()
nsurf=0
for entry in "${SURFACES[@]}"; do
    file="${entry%%$'\t'*}"
    vk="${entry##*$'\t'}"
    nsurf=$((nsurf + 1))
    out="$(awk -v vk="$vk" "$scan" "$file")"; st=$?
    fail_closed "$st" SURFACE-DUPLICATION "awk lead-in scan ($file)"
    rel="${file#"$ROOT"/}"
    while IFS=$'\t' read -r head lineno valve; do
        [[ -z "$head" ]] && continue
        grep -qxF "$head" <<< "$terms" || continue
        grep -qxF "$head" <<< "$COMPONENTS" && continue
        [[ "$valve" == "$head" ]] && continue
        errors+=("$rel:$lineno: bold-lead-in definition of glossary term '$head' — this surface may name it and explain *why*/*how-it-joins*, not carry the canonical definition (that lives in $SPEC_KIT_GLOSSARY_FILE). Reword to narration + a pointer, or tag the line '<!-- $vk: $head -->' if this surface legitimately introduces it")
    done <<< "$out"
done

if (( ${#errors[@]} )); then
    echo "SURFACE-DUPLICATION: ${#errors[@]} violation(s):"
    printf '  %s\n' "${errors[@]}"
    echo "  help: $SPEC_KIT_GLOSSARY_FILE owns the definition; another surface owns the *why* and the local mechanism — reword a restated definition to narration + a pointer, or tag the line '<!-- vision-introduces: <term> -->' / '<!-- spec-introduces: <term> -->'"
    exit 1
fi
echo "SURFACE-DUPLICATION: clean ($nsurf surfaces, $tcount terms)"
exit 0
