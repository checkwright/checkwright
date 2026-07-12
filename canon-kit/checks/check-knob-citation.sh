#!/usr/bin/env bash
# graph: couples=*SPEC*.md,*README.md,CLAUDE.md dir=one valve=none tier=precommit
# spec: canon-kit/SPEC.md §check-knob-citation — no kit knob stated with its value in manifest prose outside the owning kit's SPEC
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

ROOT="${1:-.}"
[[ -d "$ROOT" ]] || { echo "check-knob-citation: not a directory: $ROOT" >&2; exit 2; }

# spec: canon-kit/SPEC.md §check-knob-citation — the knob-token vocabulary is derived from gate_kit_roots, never listed: two prefix forms per root map to the owning kit, so the gate ships no term list and the provenance seam is untouched
pairs="$(
    while IFS= read -r kr; do
        kr="${kr%/}"; [[ -n "$kr" ]] || continue
        b="${kr##*/}"
        a="${b^^}"; a="${a//-/_}_"
        printf '%s\t%s\n' "$a" "$kr"
        if [[ "$b" == *-kit ]]; then
            bb="${b%-kit}"; bb="${bb^^}"; bb="${bb//-/_}_"
            printf '%s\t%s\n' "$bb" "$kr"
        fi
    done < <(gate_kit_roots_rel)
)"

mapfile -t manifests < <(spec_manifest_files "$ROOT" | sed 's#^\./##' | sort -u)
[[ ${#manifests[@]} -eq 0 ]] && { echo "KNOB-CITATION: clean (0 manifest file(s) found)"; exit 0; }

# spec: canon-kit/SPEC.md §check-knob-citation — prose walk only via the shared driver (fence-skip, blank-line reset, per-site exempt window); sk_on_line judges the value-statement triad per line, sk_on_pflush is unused
read -r -d '' HOOKS <<'AWK' || true
BEGIN {
    KCN = split(KC_PAIRS, _rows, "\n")
    j = 0
    for (i = 1; i <= KCN; i++) {
        if (_rows[i] == "") continue
        t = index(_rows[i], "\t")
        j++; PFX[j] = substr(_rows[i], 1, t - 1); PKIT[j] = substr(_rows[i], t + 1)
    }
    KCN = j
}
function _kc_norm(f) { sub(/^\.\//, "", f); return f }
function _kc_is_owner(f, kit,   sp) {
    sp = kit "/" KC_SPEC; f = _kc_norm(f)
    return (f == sp || f ~ ("/" sp "$"))
}
function _kc_has_prefix(t,   i) {
    for (i = 1; i <= KCN; i++) if (index(t, PFX[i]) == 1) return 1
    return 0
}
# spec: canon-kit/SPEC.md §check-knob-citation — the shared default-statement grammar's knob-name predicate (§lib/spec.sh): a backticked token that is a knob name is a name citation, not a value literal
function sk_is_knobname(t) { return _kc_has_prefix(t) }
function _kc_token_owner(t, file,   i, matched, self, owner) {
    matched = 0; self = 0; owner = ""
    for (i = 1; i <= KCN; i++) {
        if (index(t, PFX[i]) == 1) {
            matched = 1
            if (owner == "") owner = PKIT[i]
            if (_kc_is_owner(file, PKIT[i])) self = 1
        }
    }
    if (!matched || self) return ""
    return owner
}
function sk_on_line(file, fnr, raw,   line, tokline, defm, pos, n, s, abs, m, before, tok, eq, owner, firstdef) {
    line = raw
    # spec: canon-kit/SPEC.md §check-knob-citation — a knob named inside a ${...} shell expansion is a name citation (another knob's default expression, a fallback source), never a value statement of itself; blank the expansions before the token scan
    tokline = line
    gsub(/\$\{[^}]*\}/, "  ", tokline)
    defm = sk_default_bound(line)
    firstdef = ""
    pos = 1; n = length(tokline)
    while (pos <= n) {
        s = substr(tokline, pos)
        if (match(s, /[A-Z][A-Z0-9_]*=?/) == 0) break
        abs = pos + RSTART - 1
        m = substr(tokline, abs, RLENGTH)
        before = (abs > 1) ? substr(tokline, abs - 1, 1) : " "
        pos = abs + RLENGTH
        if (before ~ /[A-Za-z0-9_]/) continue
        eq = 0; tok = m
        if (substr(m, length(m), 1) == "=") { eq = 1; tok = substr(m, 1, length(m) - 1) }
        owner = _kc_token_owner(tok, file)
        if (owner == "") continue
        if (eq) {
            printf "  %s:%d  %s stated with an '=' value — the value belongs in %s/%s\n", _kc_norm(file), fnr, tok, owner, KC_SPEC
            return
        }
        if (firstdef == "") { firstdef = tok; firstowner = owner }
    }
    if (defm && firstdef != "") {
        printf "  %s:%d  %s stated with a default value — the value belongs in %s/%s\n", _kc_norm(file), fnr, firstdef, firstowner, KC_SPEC
    }
}
function sk_on_pflush() { }
AWK

AWKSRC="$(spec_para_accum_awk)
$(spec_manifest_walk_awk)
$(spec_default_grammar_awk)
$HOOKS"

out="$(awk -v KC_PAIRS="$pairs" -v KC_SPEC="$CANON_KIT_SPEC_NAME" -v SK_EXEMPT="knob-citation-exempt:" "$AWKSRC" "${manifests[@]}")"; st=$?
fail_closed "$st" check-knob-citation awk

if [[ -n "$out" ]]; then
    echo "check-knob-citation: kit knob(s) stated with a value in manifest prose outside the owning SPEC — a knob's value has one home, and a restated copy drifts silently:"
    echo ""
    echo "$out"
    echo "  help: cite the knob by bare name and point at the owning kit's SPEC roster, which owns the value; a genuine local restatement takes a 'knob-citation-exempt: <reason>' comment on the line or the one above"
    exit 1
fi
echo "KNOB-CITATION: clean (${#manifests[@]} manifest file(s); no kit knob stated with a value in prose outside the owning SPEC)"
exit 0
