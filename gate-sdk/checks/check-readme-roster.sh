#!/usr/bin/env bash
# graph: couples=kit:README.md,kit:checks/* dir=bi valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-readme-roster — every kit README's gate-roster marker block holds name-set parity with the kit's shipped checks/ basenames, both directions
#
# usage: check-readme-roster.sh [root]
#   bare: sweep gate_kit_roots against the git toplevel; root: resolve relative
#   kit roots against a fixture tree (the case dir's gate-sdk-config.sh names them).
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

ROOT="${1:-}"
if [[ -z "$ROOT" ]]; then
    ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" \
        || { echo "check-readme-roster: not a git repository and no root argument" >&2; exit 2; }
fi
[[ -d "$ROOT" ]] || { echo "check-readme-roster: root not found: $ROOT" >&2; exit 2; }

BEGIN="<!-- gate-roster:begin -->"
END="<!-- gate-roster:end -->"

mapfile -t KIT_ROOTS < <(gate_kit_roots)
[[ ${#KIT_ROOTS[@]} -gt 0 ]] || { echo "check-readme-roster: no kit roots enumerated" >&2; exit 2; }

findings=()
help_block=0
help_parity=0
swept=0
skipped=0
for r in "${KIT_ROOTS[@]}"; do
    r="${r%/}"
    abs="$r"
    [[ "$abs" == /* ]] || abs="$ROOT/$r"
    kit="${r##*/}"
    [[ -d "$abs/checks" ]] || { skipped=$((skipped + 1)); continue; }
    swept=$((swept + 1))

    shipped=""
    shopt -s nullglob
    for f in "$abs/checks/"*.sh; do
        bn="${f##*/}"
        shipped+="${bn%.sh}"$'\n'
    done
    shopt -u nullglob

    readme="$abs/README.md"
    if [[ ! -f "$readme" || -z "$(grep -F -- "$BEGIN" "$readme" 2>/dev/null)" ]]; then
        findings+=("$kit: README.md has no gate-roster marker block beside checks/")
        help_block=1
        continue
    fi

    roster="$(awk -v b="$BEGIN" -v e="$END" '
        { line = $0; gsub(/^[ \t]+|[ \t\r]+$/, "", line) }
        line == b { inb = 1; next }
        line == e { inb = 0; next }
        inb {
            for (i = 1; i <= NF; i++)
                if ($i ~ /^check-/) {
                    match($i, /^check-[[:alnum:]_-]+/)
                    print substr($i, RSTART, RLENGTH)
                    break
                }
        }
    ' "$readme")"; st=$?
    fail_closed "$st" check-readme-roster awk

    shipped_sorted="$(printf '%s' "$shipped" | sort -u)"
    roster_sorted="$(printf '%s\n' "$roster" | sort -u | grep -v '^$' || true)"

    # assertion A: every shipped check appears in the README's roster block
    missing="$(comm -23 <(printf '%s\n' "$shipped_sorted") <(printf '%s\n' "$roster_sorted"))"
    # assertion B: every roster name resolves to a shipped check
    extra="$(comm -13 <(printf '%s\n' "$shipped_sorted") <(printf '%s\n' "$roster_sorted"))"
    while IFS= read -r n; do
        [[ -n "$n" ]] && { findings+=("$kit: shipped check absent from the README roster: $n"); help_parity=1; }
    done <<<"$missing"
    while IFS= read -r n; do
        [[ -n "$n" ]] && { findings+=("$kit: roster names no shipped check: $n"); help_parity=1; }
    done <<<"$extra"
done

if [[ ${#findings[@]} -gt 0 ]]; then
    echo "check-readme-roster: kit README gate roster(s) out of parity with checks/:"
    for f in "${findings[@]}"; do echo "  $f"; done
    [[ "$help_block" -eq 1 ]] && {
        echo "  help: wrap the kit README's register-the-gates block in"
        echo "        '$BEGIN' / '$END' markers — a kit"
        echo "        shipping checks/ registers them (gate-sdk/SPEC.md §Consumer smoke)."
    }
    [[ "$help_parity" -eq 1 ]] && {
        echo "  help: keep the marker block's check-* names in name-set parity with the"
        echo "        kit's checks/ script basenames — add the missing roster line or drop"
        echo "        the stale one."
    }
    exit 1
fi

echo "README-ROSTER: clean ($swept kit README roster(s) in name-set parity with checks/; $skipped root(s) without checks/ skipped)"
exit 0
