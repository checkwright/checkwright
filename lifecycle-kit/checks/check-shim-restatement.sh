#!/usr/bin/env bash
# graph: couples=.claude/commands/*.md,CLAUDE.md,kit:templates/*.md dir=one valve=none tier=precommit
# spec: lifecycle-kit/SPEC.md §check-shim-restatement — no binding shim shares an >=N-word normalized n-gram with the dedup corpus (CLAUDE.md + every kit's templates)
#
# usage: check-shim-restatement.sh [skills-dir] [corpus-file...]
#   skills-dir defaults to LIFECYCLE_SKILLS_DIR; positional corpus files override
#   the computed default (the hermetic-fixture affordance).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"

DIR="${1:-$LIFECYCLE_SKILLS_DIR}"
[[ -d "$DIR" ]] || { echo "check-shim-restatement: skills dir not found: $DIR" >&2; exit 2; }
[[ $# -gt 0 ]] && shift

N="$LIFECYCLE_SHIM_NGRAM"

# spec: lifecycle-kit/SPEC.md §check-shim-restatement — corpus resolution order
corpus=()
if [[ $# -gt 0 ]]; then
    corpus=("$@")
elif [[ ${#LIFECYCLE_SHIM_DEDUP_CORPUS[@]} -gt 0 ]]; then
    corpus=("${LIFECYCLE_SHIM_DEDUP_CORPUS[@]}")
else
    [[ -f CLAUDE.md ]] && corpus+=(CLAUDE.md)
    while IFS= read -r root; do
        tdir="${root%/}/templates"
        [[ -d "$tdir" ]] || continue
        while IFS= read -r tf; do corpus+=("$tf"); done < <(gate_find "$tdir" -name '*.md' -type f | sort)
    done < <(gate_kit_roots_rel)
fi

# spec: lifecycle-kit/SPEC.md §check-shim-restatement — normalize, then emit every N-word window
emit_ngrams() {
    awk -v N="$N" '
        { line = tolower($0); gsub(/[^a-z0-9]+/, " ", line)
          nf = split(line, a, " ")
          for (i = 1; i <= nf; i++) if (a[i] != "") { c++; w[c] = a[i] } }
        END { for (i = 1; i + N - 1 <= c; i++) { s = w[i]; for (j = 1; j < N; j++) s = s " " w[i + j]; print s } }
    ' "$1"
}

CORPUS_IDX="$(mktemp)"
trap 'rm -f "$CORPUS_IDX"' EXIT
: >"$CORPUS_IDX"
for cf in "${corpus[@]+"${corpus[@]}"}"; do
    [[ -f "$cf" ]] || continue
    cg="$(emit_ngrams "$cf")"; st=$?
    fail_closed "$st" check-shim-restatement "ngram($cf)"
    [[ -n "$cg" ]] || continue
    sort -u <<<"$cg" | awk -v f="$cf" '{print $0 "\t" f}' >>"$CORPUS_IDX"
done
LC_ALL=C sort -t$'\t' -k1,1 -u "$CORPUS_IDX" -o "$CORPUS_IDX"
[[ -s "$CORPUS_IDX" ]] || {
    echo "check-shim-restatement: dedup corpus produced no ${N}-word n-grams (corpus files missing or shorter than N)" >&2
    exit 2
}

findings=()
shims=0
shopt -s nullglob
for f in "$DIR"/*.md; do
    grep -qE '^Execute the template at .+, applying the bindings below\.$' "$f" || continue
    shims=$((shims + 1))
    sg="$(emit_ngrams "$f" | sort -u)"; st=${PIPESTATUS[0]}
    fail_closed "$st" check-shim-restatement "ngram($f)"
    [[ -n "$sg" ]] || continue
    overlaps="$(awk -F'\t' '
        NR == FNR { file[$1] = $2; next }
        ($0 in file) { print file[$0] "\t" $0 }
    ' "$CORPUS_IDX" - <<<"$sg")"; st=$?
    fail_closed "$st" check-shim-restatement "awk overlap($f)"
    [[ -n "$overlaps" ]] || continue
    while IFS=$'\t' read -r cf ng; do
        [[ -n "$ng" ]] || continue
        findings+=("$(basename "$f") shares a ${N}-word phrase with $cf: \"$ng\"")
    done <<<"$overlaps"
done
shopt -u nullglob

if [[ ${#findings[@]} -gt 0 ]]; then
    echo "check-shim-restatement: a binding shim restates the dedup corpus — bind consumer"
    echo "residue and cite kit-owned procedure, never restate it:"
    for m in "${findings[@]}"; do echo "  $m"; done
    echo "  help: delete the restated span from the shim and replace it with a citation"
    echo "        (a path plus a §heading) to the corpus surface that owns it. The n-gram"
    echo "        holds the copy shape only; a paraphrase below ${N} words passes this gate"
    echo "        and is still the same defect to fix on sight."
    exit 1
fi
echo "SHIM-RESTATEMENT: clean ($shims binding-shim(s); no ${N}-word phrase shared with the dedup corpus)"
exit 0
