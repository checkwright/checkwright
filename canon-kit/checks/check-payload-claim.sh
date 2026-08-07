#!/usr/bin/env bash
# graph: couples=*SPEC*.md,*README.md,CLAUDE.md,SECURITY.md,docs/*.md,scripts/*.sh dir=one valve=none tier=precommit
# spec: canon-kit/SPEC.md §check-payload-claim — exactly one governed doc declares what a gate on the vendored payload discloses, and no scanned governed doc asserts a different disclosure class
#
# usage: check-payload-claim.sh [scan-root]   (default '.')
#   Scans the manifest set (lib/spec.sh) minus CANON_KIT_MDREF_EXCLUDE and
#   CANON_KIT_PAYLOAD_CLAIM_EXCLUDE. Unconfigured claim vocabulary: no-op.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

ROOT="${1:-.}"
[[ -d "$ROOT" ]] || { echo "check-payload-claim: not a directory: $ROOT" >&2; exit 2; }

# spec: canon-kit/SPEC.md §check-payload-claim — the disclosure vocabulary is consumer config with no kit default, so an empty command is the clean skip for a tree whose payload discloses one thing only
if [[ -z "$CANON_KIT_PAYLOAD_CLAIMS_CMD" ]]; then
    echo "PAYLOAD-CLAIM: clean (no disclosure vocabulary configured — nothing to hold)"
    exit 0
fi

claims="$(spec_claim_vocabulary "$CANON_KIT_PAYLOAD_CLAIMS_CMD" CANON_KIT_PAYLOAD_CLAIMS_CMD)"; st=$?
fail_closed "$st" check-payload-claim "CANON_KIT_PAYLOAD_CLAIMS_CMD"
[[ -n "$claims" ]] || { echo "PAYLOAD-CLAIM: clean (CANON_KIT_PAYLOAD_CLAIMS_CMD declared no disclosure classes)"; exit 0; }

excluded() {
    local rel="$1" g
    for g in "${CANON_KIT_MDREF_EXCLUDE[@]+"${CANON_KIT_MDREF_EXCLUDE[@]}"}" \
             "${CANON_KIT_PAYLOAD_CLAIM_EXCLUDE[@]+"${CANON_KIT_PAYLOAD_CLAIM_EXCLUDE[@]}"}"; do
        # shellcheck disable=SC2053  # $g is the exclude glob, matched unquoted on purpose
        [[ "$rel" == $g ]] && return 0
    done
    return 1
}

files=()
while IFS= read -r f; do
    rel="${f#./}"; rel="${rel#"$ROOT"/}"
    excluded "$rel" || files+=("$f")
done < <(spec_manifest_files "$ROOT" | sort -u)
[[ ${#files[@]} -eq 0 ]] && { echo "PAYLOAD-CLAIM: clean (0 governed doc(s) found)"; exit 0; }

DECL_RE='^[[:space:]]*<!--[[:space:]]*payload-discloses:[[:space:]]*([a-z0-9][a-z0-9-]*)[[:space:]]*-->[[:space:]]*$'

decls="$(grep -nE "$DECL_RE" -- "${files[@]}" /dev/null)"; st=$?
[[ $st -le 1 ]] || { echo "check-payload-claim: grep failed scanning the governed doc set" >&2; exit 2; }

ndecl=0
[[ -n "$decls" ]] && ndecl="$(printf '%s\n' "$decls" | wc -l)"

# assertion A: exactly one governed doc declares the disclosure class
if [[ "$ndecl" -eq 0 ]]; then
    echo "check-payload-claim: no governed doc declares what the vendored payload discloses:"
    echo "  0 'payload-discloses:' declarations across ${#files[@]} governed doc(s)"
    echo "  help: an unowned disclosure claim is how an unbounded number of surfaces drift into"
    echo "        promising the consumer something else, with nothing watching. Put one full-line"
    echo "        '<!-- payload-discloses: <claim-id> -->' in the section that rules the fact,"
    echo "        naming a class CANON_KIT_PAYLOAD_CLAIMS_CMD emits."
    exit 1
fi
if [[ "$ndecl" -gt 1 ]]; then
    echo "check-payload-claim: the payload-disclosure claim has $ndecl owners; exactly one is required:"
    printf '%s\n' "$decls" | sed 's/^/  /'
    echo "  help: keep the declaration in the section that rules the fact and delete the others;"
    echo "        two owners is the same unowned-claim defect wearing a different shape."
    exit 1
fi

decl_site="${decls%%:*}"
decl_line="$(printf '%s\n' "$decls" | cut -d: -f2)"
declared="$(printf '%s\n' "$decls" | sed -E "s/.*payload-discloses:[[:space:]]*([a-z0-9][a-z0-9-]*).*/\1/")"

# spec: canon-kit/SPEC.md §check-payload-claim — an id outside the configured vocabulary is fail-closed, not a violation: with no resolvable declared class the gate holds nothing to compare a line against, so it must not run rather than pass
if ! printf '%s\n' "$claims" | cut -f1 | grep -qxF -- "$declared"; then
    echo "check-payload-claim: declared class '$declared' ($decl_site:$decl_line) is not a configured disclosure class:" >&2
    printf '%s\n' "$claims" | cut -f1 | sed 's/^/  /' >&2
    exit 2
fi

export SK_CLAIMS="$claims"
export SK_DECLARED="$declared"
export SK_DECL_RE="$DECL_RE"

# assertion B: no scanned line asserts a disclosure class other than the declared one
# spec: canon-kit/SPEC.md §check-payload-claim — membership over the whole document rather than position inside a section: a non-declared class is wrong wherever it appears. Fenced content is scanned, because a quoted recipe is exactly where a disclosure claim shows up in passing; the declaration line is skipped, since a claim is not evidence for itself.
read -r -d '' AWKSRC <<'AWK' || true
BEGIN {
    n = split(ENVIRON["SK_CLAIMS"], cl, "\n")
    for (i = 1; i <= n; i++) {
        if (cl[i] == "") continue
        tp = index(cl[i], "\t")
        cid = substr(cl[i], 1, tp - 1)
        if (cid == ENVIRON["SK_DECLARED"]) continue
        nc++
        oid[nc] = cid
        ore[nc] = substr(cl[i], tp + 1)
    }
    declre = ENVIRON["SK_DECL_RE"]
}
{
    if ($0 ~ declre) next
    for (i = 1; i <= nc; i++) {
        if ($0 !~ ore[i]) continue
        printf "  %s:%d: asserts '%s', not the declared '%s'\n", \
            FILENAME, FNR, oid[i], ENVIRON["SK_DECLARED"]
        break
    }
}
END { printf "%d other class(es) held\n", nc }
AWK

out="$(awk "$AWKSRC" "${files[@]}")"; st=$?
fail_closed "$st" check-payload-claim awk

findings="$(printf '%s\n' "$out" | grep '^  ' || true)"
held="$(printf '%s\n' "$out" | tail -n1)"

if [[ -n "$findings" ]]; then
    echo "check-payload-claim: a governed doc asserts a disclosure class the declared claim does not name:"
    printf '%s\n' "$findings"
    echo "  help: correct the sentence to the declared class, or move the declaration"
    echo "        ($decl_site:$decl_line) to the class the payload actually discloses."
    echo "        The declared class is what the payload ships; every other class is wrong"
    echo "        wherever it appears, whichever section it sits in."
    exit 1
fi

echo "PAYLOAD-CLAIM: clean (${#files[@]} governed doc(s), $held; '$declared' declared at $decl_site:$decl_line, and no scanned line asserts another class)"
exit 0
