#!/usr/bin/env bash
# graph: couples=kit:checks/check-*.sh,kit:checks/check-*.gate,kit:smoke/install.sh,installer/lib/common/recipe.sh dir=one valve=none tier=precommit
# install: zero-config
# spec: gate-sdk/SPEC.md §check-install-disposition — every shipped gate declares one install disposition, every zero-config gate is registrable in its kit's smoke, and the installer keeps no second copy of the roster
#
# usage: check-install-disposition.sh [root]
#   bare: sweep gate_kit_roots against the git toplevel; root: resolve relative
#   kit roots against a fixture tree (the case dir's gate-sdk-config.sh names them).
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

ROOT="${1:-}"
if [[ -z "$ROOT" ]]; then
    ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" \
        || { echo "check-install-disposition: not a git repository and no root argument" >&2; exit 2; }
fi
[[ -d "$ROOT" ]] || { echo "check-install-disposition: root not found: $ROOT" >&2; exit 2; }

# spec: gate-sdk/SPEC.md §The install disposition — the vocabulary is closed, so an unrecognised value is a finding rather than a pass-through: it names install-time reachability and nothing else
VOCAB='zero-config|on-surface|never'
RECIPE="installer/lib/common/recipe.sh"

mapfile -t KIT_ROOTS < <(gate_kit_roots)
[[ ${#KIT_ROOTS[@]} -gt 0 ]] || { echo "check-install-disposition: no kit roots enumerated" >&2; exit 2; }

findings=()
declared=0
zeroconf=0
for r in "${KIT_ROOTS[@]}"; do
    r="${r%/}"
    abs="$r"
    [[ "$abs" == /* ]] || abs="$ROOT/$r"
    kit="${r##*/}"
    [[ -d "$abs/checks" ]] || continue
    smoke="$abs/smoke/install.sh"
    shopt -s nullglob
    for f in "$abs"/checks/check-*.sh "$abs"/checks/check-*.gate; do
        name="${f##*/}"; name="${name%.*}"
        [[ -r "$f" ]] || { echo "check-install-disposition: unreadable gate header: $kit/checks/${f##*/}" >&2; exit 2; }
        mapfile -t lines < <(grep '^# install:' "$f")
        if [[ ${#lines[@]} -ne 1 ]]; then
            findings+=("$kit/checks/${f##*/}: ${#lines[@]} '# install:' line(s) where a gate declares exactly one")
            continue
        fi
        value="$(awk 'sub(/^# install:[[:space:]]+/, "") { print $1; exit }' "$f")"; st=$?
        fail_closed "$st" check-install-disposition awk
        if [[ ! "$value" =~ ^($VOCAB)$ ]]; then
            findings+=("$kit/checks/${f##*/}: install disposition '${value:-<empty>}' is outside the closed vocabulary")
            continue
        fi
        declared=$((declared + 1))
        [[ "$value" == zero-config ]] || continue
        zeroconf=$((zeroconf + 1))
        # spec: gate-sdk/SPEC.md §check-install-disposition — the direction is the census's finding made mechanical: the smoke's tree is a superset of the tree init makes, so a gate the installer registers must be registrable there too. The converse is deliberately not asserted
        if [[ ! -f "$smoke" ]]; then
            findings+=("$kit/checks/${f##*/}: declares zero-config where $kit ships no smoke/install.sh to register it")
        elif ! grep -qxF "$name" "$smoke"; then
            findings+=("$kit/checks/${f##*/}: declares zero-config but $kit/smoke/install.sh does not register it")
        fi
    done
    shopt -u nullglob
done
[[ "$declared" -gt 0 || ${#findings[@]} -gt 0 ]] \
    || { echo "check-install-disposition: kit roots enumerated but no gate found under any checks/" >&2; exit 2; }

# spec: gate-sdk/SPEC.md §check-install-disposition — assertion C holds the de-literalization going forward rather than only at the commit that lands it; the file is absent in a vendored consumer, which has no installer, so its absence is a skip and never a finding. A §-prefixed occurrence is a spec-section citation rather than a roster member and is stripped before the match: a section reference registers nothing, which is the only thing this assertion is about
recipe_checked=no
if [[ -f "$ROOT/$RECIPE" ]]; then
    recipe_checked=yes
    literals="$(awk '{ l=$0; gsub(/§[A-Za-z0-9_-]+/, "", l); if (l ~ /check-[a-z0-9]+(-[a-z0-9]+)*/) print FNR }' "$ROOT/$RECIPE")"; st=$?
    fail_closed "$st" check-install-disposition awk
    while IFS= read -r hit; do
        [[ -n "$hit" ]] || continue
        findings+=("$RECIPE:$hit: literal gate name — the roster is derived from each gate's declaration, never listed here")
    done <<<"$literals"
fi

if [[ ${#findings[@]} -gt 0 ]]; then
    echo "check-install-disposition: install-disposition finding(s):"
    for f in "${findings[@]}"; do echo "  $f"; done
    echo "  help: give every gate exactly one '# install: <$VOCAB>' line beside its"
    echo "        '# graph:' directive; register each zero-config gate in its kit's"
    echo "        smoke/install.sh; and keep $RECIPE free of literal"
    echo "        gate names — it derives the roster (gate-sdk/SPEC.md §The install disposition)."
    exit 1
fi

echo "INSTALL-DISPOSITION: clean ($declared gate(s) declared, $zeroconf zero-config and registrable in their kit's smoke; recipe de-literalization checked: $recipe_checked)"
exit 0
