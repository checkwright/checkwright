#!/usr/bin/env bash
# graph: couples=kit:*.sh,kit:SPEC.md dir=one valve=none tier=precommit
# spec: canon-kit/SPEC.md §check-knob-default-coupling — every literal kit-knob default in kit source agrees across its sites and with the default the owning SPEC states
#
# usage: check-knob-default-coupling.sh   (no args — the roster is gate_kit_roots)
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

# spec: canon-kit/SPEC.md §check-knob-default-coupling — the knob prefix is derived, never listed: one SCREAMING_SNAKE form per kit root (dir uppercased, hyphens to underscores), so the gate ships no term list and the provenance seam holds
pairs="$(
    while IFS= read -r kr; do
        kr="${kr%/}"; [[ -n "$kr" ]] || continue
        b="${kr##*/}"
        a="${b^^}"; a="${a//-/_}_"
        printf '%s\t%s\n' "$a" "$kr"
    done < <(gate_kit_roots_rel)
)"
[[ -n "$pairs" ]] || { echo "check-knob-default-coupling: no kit roots enumerated" >&2; exit 2; }

mapfile -t sources < <(
    while IFS= read -r kr; do
        kr="${kr%/}"; [[ -n "$kr" ]] || continue
        gate_find "$kr" -name '*.sh' -type f 2>/dev/null | grep -v '/templates/' || true
    done < <(gate_kit_roots_rel) | sed 's#^\./##' | sort -u
)
[[ ${#sources[@]} -eq 0 ]] && { echo "KNOB-DEFAULT-COUPLING: clean (0 kit source file(s) found)"; exit 0; }

# spec: canon-kit/SPEC.md §check-knob-default-coupling — one record per default site: "<class>\t<knob>\t<kit>\t<value>\t<file>\t<lno>", class literal|skip. A ${…:-v} fallback and the guarded-assignment idiom are the two sites; an array (…), an empty fallback, or a value carrying an expansion/substitution ($ or backtick) is a computed default with no single literal — class skip, counted not coupled.
read -r -d '' EXTRACT <<'AWK' || true
BEGIN {
    n = split(PAIRS, _rows, "\n"); j = 0
    for (i = 1; i <= n; i++) {
        if (_rows[i] == "") continue
        t = index(_rows[i], "\t")
        j++; PFX[j] = substr(_rows[i], 1, t - 1); PKIT[j] = substr(_rows[i], t + 1)
    }
    NPFX = j
}
function knob_owner(knob,   i) {
    for (i = 1; i <= NPFX; i++) if (index(knob, PFX[i]) == 1) return PKIT[i]
    return ""
}
function classify(v) {
    if (v == "") return "skip"
    if (substr(v, 1, 1) == "(") return "skip"      # array literal — no single value to couple
    if (v ~ /[$`]/) return "skip"                  # expansion / substitution / command — computed
    return "literal"
}
function emit(knob, val, kit,   cls) {
    kit = knob_owner(knob)
    if (kit == "") return
    cls = classify(val)
    printf "%s\t%s\t%s\t%s\t%s\t%d\n", cls, knob, kit, val, FILENAME, FNR
}
function strip_quotes(v) {
    if (v ~ /^".*"$/) return substr(v, 2, length(v) - 2)
    if (v ~ /^'.*'$/) return substr(v, 2, length(v) - 2)
    return v
}
{
    line = $0
    # idiom 1: ${PREFIX_KNOB:-value} fallback
    rest = line; base = 0
    while (match(rest, /\$\{[A-Za-z_][A-Za-z0-9_]*:-[^}]*\}/) > 0) {
        seg = substr(rest, RSTART, RLENGTH)
        inner = substr(seg, 3, length(seg) - 3)        # drop ${ and }
        c = index(inner, ":-")
        knob = substr(inner, 1, c - 1)
        val = substr(inner, c + 2)
        emit(knob, val)
        base += RSTART + RLENGTH - 1
        rest = substr(rest, RSTART + RLENGTH)
    }
    # idiom 2: guarded assignment — [[ -v KNOB ]] || KNOB=value / declare -p KNOB &>/dev/null || KNOB=(…)
    if (line ~ /(\]|&>\/dev\/null)[ \t]*\|\|[ \t]*[A-Za-z_][A-Za-z0-9_]*=/) {
        tail = line
        sub(/^.*\|\|[ \t]*/, "", tail)
        if (match(tail, /^[A-Za-z_][A-Za-z0-9_]*=/) > 0) {
            knob = substr(tail, 1, RLENGTH - 1)
            val = substr(tail, RLENGTH + 1)
            sub(/[ \t]+$/, "", val)
            emit(knob, strip_quotes(val))
        }
    }
}
AWK

records="$(awk -v PAIRS="$pairs" "$EXTRACT" "${sources[@]}")"; st=$?
fail_closed "$st" check-knob-default-coupling "awk extract"

skipped="$(grep -c '^skip'$'\t' <<<"$records" 2>/dev/null || true)"; skipped="${skipped:-0}"

findings=()
described=0

# spec: canon-kit/SPEC.md §check-knob-default-coupling — assertion 1: every literal site for one knob carries the same literal, else the source disagrees with itself before any SPEC is read (and the knob's assertion-2 check is suppressed)
declare -A knob_val knob_first knob_kit knob_conflict
while IFS=$'\t' read -r cls knob kit val file lno; do
    [[ "$cls" == "literal" ]] || continue
    if [[ -z "${knob_val[$knob]+x}" ]]; then
        knob_val[$knob]="$val"; knob_first[$knob]="$file:$lno"; knob_kit[$knob]="$kit"
    elif [[ "${knob_val[$knob]}" != "$val" ]]; then
        knob_conflict[$knob]=1
        findings+=("  $file:$lno  $knob default '$val' disagrees with '${knob_val[$knob]}' at ${knob_first[$knob]} — a knob's default has one literal across its sites")
    fi
done < <(printf '%s\n' "$records")

# spec: canon-kit/SPEC.md §check-knob-default-coupling — assertion 2: the owning kit's SPEC states that same literal, read through the shared default-statement grammar (§lib/spec.sh); a descriptively-stated default carries no literal and is skipped
SPECGRAMMAR="$(spec_default_grammar_awk)"
read -r -d '' SPECSCAN <<'AWK' || true
BEGIN {
    n = split(PAIRS, _rows, "\n"); j = 0
    for (i = 1; i <= n; i++) {
        if (_rows[i] == "") continue
        t = index(_rows[i], "\t")
        j++; PFX[j] = substr(_rows[i], 1, t - 1)
    }
    NPFX = j
    m = split(SUBSET, _s, "\n"); NS = 0
    for (i = 1; i <= m; i++) {
        if (_s[i] == "") continue
        t = index(_s[i], "\t")
        NS++; K[NS] = substr(_s[i], 1, t - 1); L[NS] = substr(_s[i], t + 1)
    }
}
function sk_is_knobname(tok,   i) {
    for (i = 1; i <= NPFX; i++) if (index(tok, PFX[i]) == 1) return 1
    return 0
}
# reduce every ${VAR:-tail} deferral expression in a window to its tail literal, so
# a SPEC default stated as the same deferral the source uses (a cross-kit knob
# inheriting another's default) compares tail-to-tail, not expression-to-literal.
function reduce_deferrals(s,   out, m, inner, c) {
    out = ""
    while (match(s, /\$\{[A-Za-z_][A-Za-z0-9_]*:-[^}]*\}/) > 0) {
        m = substr(s, RSTART, RLENGTH)
        inner = substr(m, 3, length(m) - 3)
        c = index(inner, ":-")
        out = out substr(s, 1, RSTART - 1) substr(inner, c + 2)
        s = substr(s, RSTART + RLENGTH)
    }
    return out s
}
# v appears in win as a full delimited token: bounded on both sides by a non-value
# char (not alnum, _, ., /, *, or -), so a suffix of a longer literal never matches.
function fulltoken(win, v,   L, start, p, bpos, b, a) {
    L = length(v); start = 1
    while ((p = index(substr(win, start), v)) > 0) {
        bpos = start + p - 1
        b = (bpos > 1) ? substr(win, bpos - 1, 1) : " "
        a = substr(win, bpos + L, 1); if (a == "") a = " "
        if (b !~ /[A-Za-z0-9_.\/*-]/ && a !~ /[A-Za-z0-9_.\/*-]/) return 1
        start = bpos + L
    }
    return 0
}
# truncate s at the first kit-knob token, so a knob's default window never bleeds
# into the next knob's statement (an inline paragraph lists many knob defaults).
function bound_next_knob(s,   i, pos, best) {
    best = length(s) + 1
    for (i = 1; i <= NPFX; i++) { pos = index(s, PFX[i]); if (pos > 0 && pos < best) best = pos }
    return substr(s, 1, best - 1)
}
{ blob = (blob == "" ? $0 : blob " " $0) }
END {
    for (s = 1; s <= NS; s++) {
        found = 0; any_disagree = 0; any_described = 0; disagree_lit = ""
        start = 1
        while ((p = index(substr(blob, start), K[s])) > 0) {
            kpos = start + p - 1
            after = substr(blob, kpos + length(K[s]), 1)
            if (after ~ /[A-Za-z0-9_]/) { start = kpos + length(K[s]); continue }  # a longer knob, not this one
            win = reduce_deferrals(substr(blob, kpos, length(K[s]) + 400))
            haskw = index(tolower(win), "default")
            slit = ""
            if (haskw) {
                # the literal the knob's own (first) "default" binds, bounded before the next knob
                slit = sk_literal_at(bound_next_knob(substr(win, haskw + 7)))
            }
            if (slit == L[s] || (haskw && fulltoken(win, L[s]))) { found = 1; break }
            if (slit != "") { any_disagree = 1; disagree_lit = slit }   # a default literal, but not this one
            else if (haskw) any_described = 1                           # descriptively stated — no literal to couple
            start = kpos + length(K[s])
        }
        if (!found) {
            if (any_disagree) printf "disagree\t%s\t%s\n", K[s], L[s]
            else if (any_described) printf "described\t%s\t%s\n", K[s], L[s]
            else printf "absent\t%s\t%s\n", K[s], L[s]
        }
    }
}
AWK

declare -A kit_subset
for knob in "${!knob_val[@]}"; do
    [[ -n "${knob_conflict[$knob]+x}" ]] && continue     # source disagrees first — fix that before the SPEC
    kit="${knob_kit[$knob]}"
    kit_subset[$kit]+="$knob"$'\t'"${knob_val[$knob]}"$'\n'
done

for kit in "${!kit_subset[@]}"; do
    spec="$kit/$CANON_KIT_SPEC_NAME"
    if [[ ! -f "$spec" ]]; then
        while IFS=$'\t' read -r knob val; do
            [[ -n "$knob" ]] || continue
            findings+=("  $spec  $knob default \`$val\` has no owning SPEC to state it — the SPEC owns knob defaults")
        done < <(printf '%s' "${kit_subset[$kit]}")
        continue
    fi
    out="$(awk -v PAIRS="$pairs" -v SUBSET="${kit_subset[$kit]}" "$SPECGRAMMAR
$SPECSCAN" "$spec")"; st=$?
    fail_closed "$st" check-knob-default-coupling "awk spec-scan($spec)"
    while IFS=$'\t' read -r verdict knob val; do
        [[ -n "$verdict" ]] || continue
        case "$verdict" in
            disagree) findings+=("  $spec  $knob — source default \`$val\` but the SPEC states a different default") ;;
            absent)   findings+=("  $spec  $knob — source default \`$val\` is stated nowhere in the owning SPEC") ;;
            described) described=$((described + 1)) ;;   # SPEC states the default descriptively — no literal to couple
        esac
    done <<< "$out"
done

if [[ ${#findings[@]} -gt 0 ]]; then
    echo "check-knob-default-coupling: kit-knob default(s) drift between source sites or from the owning SPEC — a default has one home, and a divergent copy is a silent regression:"
    echo ""
    printf '%s\n' "${findings[@]}" | sort
    echo "  help: make every fallback site for the knob carry the same literal, and state that literal as the knob's default in the owning kit's SPEC (the default-statement grammar check-knob-citation reads); a computed default belongs in the SPEC as prose, not a coupled literal"
    exit 1
fi

lit_count="$(grep -c '^literal'$'\t' <<<"$records" 2>/dev/null || true)"; lit_count="${lit_count:-0}"
echo "KNOB-DEFAULT-COUPLING: clean (${#sources[@]} kit source file(s); $lit_count literal default site(s) agree across sites and with the owning SPEC; $skipped computed/array/empty + $described descriptively-stated default(s) skipped-and-counted)"
exit 0
