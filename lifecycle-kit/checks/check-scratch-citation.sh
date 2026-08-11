#!/usr/bin/env bash
# graph: couples=TASK-QUEUE.md,*/SPEC.md,kit:lib/stages.sh dir=one valve=none tier=precommit
# install: on-surface
# spec: lifecycle-kit/SPEC.md §check-scratch-citation — no permanent surface carries a retrieval pointer into a boundary-truncated one, because that pointer resolves to nothing at the next iteration boundary
#
# usage: check-scratch-citation.sh [surface-glob…]
#   with no argument the surfaces come from LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS;
#   the forbidden targets are always lifecycle_supersede_set()'s, never a second roster.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"

if [[ $# -gt 0 ]]; then
    GLOBS=("$@")
else
    GLOBS=(${LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS[@]+"${LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS[@]}"})
fi

# spec: lifecycle-kit/SPEC.md §check-scratch-citation — the forbidden-target set is lifecycle_supersede_set()'s third reader, so a consumer adding a LIFECYCLE_KIT_BOUNDARY_TRUNCATE member gets citation enforcement over it with no second roster to update
targets="$(lifecycle_supersede_set | sort -u | grep -v '^$')"
[[ -n "$targets" ]] \
    || { echo "check-scratch-citation: the derived boundary-truncated set is empty — the state machine names no such surface (a lifecycle always owns at least its state + lesson-evidence files)" >&2; exit 2; }  # exit 2: fail-closed

shopt -s nullglob globstar
files=()
for g in ${GLOBS[@]+"${GLOBS[@]}"}; do
    for f in $g; do
        [[ -f "$f" ]] && files+=("$f")
    done
done
shopt -u nullglob globstar

if [[ "${#files[@]}" -eq 0 ]]; then
    echo "SCRATCH-CITATION: clean (no permanent surface configured — LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS matched nothing)"
    exit 0
fi

# spec: lifecycle-kit/SPEC.md §check-scratch-citation — the blank-line paragraph join canon-kit/checks/check-spec-pointer.sh's PROSE_EXTRACT already uses for a citation-liveness scan: a wrapped bullet routinely splits the colon from the path it introduces, so a scanner reading physical lines in isolation is silently blind on exactly the case this gate was written for. Per-line start offsets map a hit back to its physical line.
read -r -d '' SCAN <<'AWK' || true
function flush(   i, joined, mstart, mend, li, s, t, pre, before, after, hitline) {
    if (np == 0) return
    joined = ""
    for (i = 1; i <= np; i++) { lstart[i] = length(joined) + 1; joined = joined (i > 1 ? " " : "") ptext[i] }
    for (t in target) {
        scanpos = 1
        while (1) {
            s = substr(joined, scanpos)
            if (index(s, t) == 0) break
            mstart = scanpos + index(s, t) - 1
            mend = mstart + length(t) - 1
            li = 1
            for (i = 1; i <= np; i++) if (lstart[i] <= mstart) li = i
            hitline = pfnr[li]
            before = substr(joined, 1, mstart - 1)
            after = substr(joined, mend + 1)
            pre = before
            sub(/`$/, "", pre)
            if (after ~ /^`?\)/ && before ~ /\]\($/) {
                printf "%s\t%d\tmarkdown link target\t%s\n", cf, hitline, t
            } else if (pre ~ /:[[:space:]]+$/ && after ~ /^`?([[:space:]]*$|[".,)])/) {
                printf "%s\t%d\tcolon-introduced citation\t%s\n", cf, hitline, t
            }
            scanpos = mend + 1
        }
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

hits=""
for f in "${files[@]}"; do
    out="$(awk -v tlist="$targets" '
        BEGIN { n = split(tlist, a, "\n"); for (i = 1; i <= n; i++) if (a[i] != "") target[a[i]] = 1 }
        '"$SCAN"'
    ' "$f")"; st=$?
    fail_closed "$st" check-scratch-citation awk
    [[ -n "$out" ]] && hits+="$out"$'\n'
done

# spec: lifecycle-kit/SPEC.md §check-scratch-citation — the escape hatch is checked on the line before the hit's physical line, the repo's established per-line opt-out shape, for a surface that must quote a dead citation verbatim in order to describe it
findings=""
while IFS=$'\t' read -r file lineno kind path; do
    [[ -n "$file" ]] || continue
    prev=""
    [[ "$lineno" -gt 1 ]] && prev="$(awk -v n="$((lineno - 1))" 'FNR == n { print; exit }' "$file")"
    [[ "$prev" == *scratch-citation-exempt:* ]] && continue
    findings+="  $file:$lineno: $kind into the boundary-truncated $path"$'\n'
done <<<"$hits"

if [[ -n "$findings" ]]; then
    echo "check-scratch-citation: permanent surface(s) point a reader into per-iteration scratch:"
    printf '%s' "$findings"
    echo "  help: a boundary-truncated surface is emptied by the next enter-stage.sh boundary reset, so the"
    echo "        pointer resolves to nothing one iteration after it is written. Inline the finding instead —"
    echo "        bash lifecycle-kit/bin/cite-survey.sh \"<heading-substring>\" emits the block's heading and"
    echo "        all four witness fields, which is what keeps it re-usable rather than merely readable."
    echo "        A surface that must quote a dead citation verbatim tags the line above it"
    echo "        'scratch-citation-exempt: <reason>'."
    exit 1
fi

echo "SCRATCH-CITATION: clean (${#files[@]} permanent surface(s) carry no retrieval pointer into the derived boundary-truncated set)"
exit 0
