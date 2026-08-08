#!/usr/bin/env bash
# graph: couples=*SPEC*.md,*README.md,CLAUDE.md,docs/*.md,scripts/*.sh dir=one valve=none tier=precommit
# install: on-surface
# spec: canon-kit/SPEC.md §check-install-claim — exactly one governed doc declares the primary install transport, and no scanned install section leads with a different one
#
# usage: check-install-claim.sh [scan-root]   (default '.')
#   Scans the manifest set (lib/spec.sh) minus CANON_KIT_MDREF_EXCLUDE and
#   CANON_KIT_INSTALL_CLAIM_EXCLUDE. Unconfigured transports or section regex: no-op.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

ROOT="${1:-.}"
[[ -d "$ROOT" ]] || { echo "check-install-claim: not a directory: $ROOT" >&2; exit 2; }

# spec: canon-kit/SPEC.md §check-install-claim — the transport vocabulary and the install-section regex are consumer config with no kit default, so either left empty is the clean skip for a consumer that documents no install path
if [[ -z "$CANON_KIT_INSTALL_TRANSPORTS_CMD" || -z "$CANON_KIT_INSTALL_SECTION_RE" ]]; then
    echo "INSTALL-CLAIM: clean (no transport vocabulary or install-section regex configured — nothing to hold)"
    exit 0
fi

transports="$(spec_install_transports)"; st=$?
fail_closed "$st" check-install-claim "CANON_KIT_INSTALL_TRANSPORTS_CMD"
[[ -n "$transports" ]] || { echo "INSTALL-CLAIM: clean (CANON_KIT_INSTALL_TRANSPORTS_CMD declared no transports)"; exit 0; }

excluded() {
    local rel="$1" g
    for g in "${CANON_KIT_MDREF_EXCLUDE[@]+"${CANON_KIT_MDREF_EXCLUDE[@]}"}" \
             "${CANON_KIT_INSTALL_CLAIM_EXCLUDE[@]+"${CANON_KIT_INSTALL_CLAIM_EXCLUDE[@]}"}"; do
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
[[ ${#files[@]} -eq 0 ]] && { echo "INSTALL-CLAIM: clean (0 governed doc(s) found)"; exit 0; }

DECL_RE='^[[:space:]]*<!--[[:space:]]*install-primary:[[:space:]]*([a-z0-9][a-z0-9-]*)[[:space:]]*-->[[:space:]]*$'

decls="$(grep -nE "$DECL_RE" -- "${files[@]}" /dev/null)"; st=$?
[[ $st -le 1 ]] || { echo "check-install-claim: grep failed scanning the governed doc set" >&2; exit 2; }

ndecl=0
[[ -n "$decls" ]] && ndecl="$(printf '%s\n' "$decls" | wc -l)"

if [[ "$ndecl" -eq 0 ]]; then
    echo "check-install-claim: no governed doc declares the primary install transport:"
    echo "  0 'install-primary:' declarations across ${#files[@]} governed doc(s)"
    echo "  help: an unowned primary-path claim is how two pages drift into naming different"
    echo "        transports with nothing to catch it. Put one full-line"
    echo "        '<!-- install-primary: <transport-id> -->' in the section that owns the claim,"
    echo "        naming a transport CANON_KIT_INSTALL_TRANSPORTS_CMD emits."
    exit 1
fi
if [[ "$ndecl" -gt 1 ]]; then
    echo "check-install-claim: the primary-install-path claim has $ndecl owners; exactly one is required:"
    printf '%s\n' "$decls" | sed 's/^/  /'
    echo "  help: keep the declaration on the page that owns the claim and delete the others;"
    echo "        two owners is the same unowned-claim defect wearing a different shape."
    exit 1
fi

decl_site="${decls%%:*}"
decl_line="$(printf '%s\n' "$decls" | cut -d: -f2)"
primary="$(printf '%s\n' "$decls" | sed -E "s/.*install-primary:[[:space:]]*([a-z0-9][a-z0-9-]*).*/\1/")"

# spec: canon-kit/SPEC.md §check-install-claim — an id outside the configured vocabulary is fail-closed, not a violation: the gate then has no primary to compare a section's leading transport against, so it must not run rather than pass
if ! printf '%s\n' "$transports" | cut -f1 | grep -qxF -- "$primary"; then
    echo "check-install-claim: declared primary '$primary' ($decl_site:$decl_line) is not a configured transport:" >&2
    printf '%s\n' "$transports" | cut -f1 | sed 's/^/  /' >&2
    exit 2
fi

export SK_TRANSPORTS="$transports"
export SK_SECTION_RE="$CANON_KIT_INSTALL_SECTION_RE"
export SK_PRIMARY="$primary"
export SK_DECL_RE="$DECL_RE"

# spec: canon-kit/SPEC.md §check-install-claim — assertion B: the earliest transport-matching line in a scanned section is that section's leading claim and later matches are never flagged; fenced content is scanned (a recipe is where a transport shows) but never read as a heading, and the declaration line is not evidence for itself
read -r -d '' AWKSRC <<'AWK' || true
BEGIN {
    n = split(ENVIRON["SK_TRANSPORTS"], tl, "\n")
    for (i = 1; i <= n; i++) {
        if (tl[i] == "") continue
        tp = index(tl[i], "\t")
        nt++
        tid[nt] = substr(tl[i], 1, tp - 1)
        tre[nt] = substr(tl[i], tp + 1)
    }
    sectre  = ENVIRON["SK_SECTION_RE"]
    primary = ENVIRON["SK_PRIMARY"]
    declre  = ENVIRON["SK_DECL_RE"]
}
FNR == 1 { fence = 0; inscope = 0; settled = 0 }
{
    if ($0 ~ /^[[:space:]]*(```|~~~)/) { fence = !fence; next }
    if (!fence && match($0, /^#{2,6}[[:space:]]+/)) {
        head = substr($0, RSTART + RLENGTH)
        sub(/[[:space:]]+$/, "", head)
        inscope = (head ~ sectre)
        settled = 0
        if (inscope) { scanned++; sect = head; sectfnr = FNR }
        next
    }
    if (!inscope || settled || $0 ~ declre) next
    led = ""
    for (i = 1; i <= nt; i++) {
        if ($0 !~ tre[i]) continue
        if (tid[i] == primary) { led = primary; break }
        if (led == "") led = tid[i]
    }
    if (led == "") next
    settled = 1
    if (led != primary)
        printf "  %s:%d: section '%s' (line %d) leads with '%s', not the declared primary '%s'\n", \
            FILENAME, FNR, sect, sectfnr, led, primary
}
END { printf "%d sections scanned\n", scanned }
AWK

out="$(awk "$AWKSRC" "${files[@]}")"; st=$?
fail_closed "$st" check-install-claim awk

findings="$(printf '%s\n' "$out" | grep '^  ' || true)"
scanned="$(printf '%s\n' "$out" | tail -n1)"

if [[ -n "$findings" ]]; then
    echo "check-install-claim: an install section leads with a transport the primary-path claim does not name:"
    printf '%s\n' "$findings"
    echo "  help: the first transport a section names is the path it recommends. Lead with the"
    echo "        declared primary and name the others after it, or move the declaration"
    echo "        ($decl_site:$decl_line) to the transport the project actually leads with."
    exit 1
fi

echo "INSTALL-CLAIM: clean (${#files[@]} governed doc(s), $scanned; '$primary' declared primary at $decl_site:$decl_line, and no scanned section leads with another transport)"
exit 0
