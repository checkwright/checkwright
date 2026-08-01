#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §compare-settings-allow — the prune-candidate set and the narrowing-candidate set
# usage: compare-settings-allow.sh [--count]
set -uo pipefail

BIN="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../lib/guard.sh
source "$BIN/../lib/guard.sh"

COMMITTED="$GUARD_KIT_SETTINGS"
LOCAL="$GUARD_KIT_SETTINGS_LOCAL"

COUNT=0
case "${1:-}" in
    --count) COUNT=1 ;;
    "") ;;
    *) echo "usage: compare-settings-allow.sh [--count]" >&2; exit 2 ;;
esac

if [[ ! -f "$LOCAL" ]]; then
    [[ "$COUNT" -eq 1 ]] && { echo "0 0"; exit 0; }
    echo "=== settings allowlist redundancy (advisory — prune candidates) ==="
    echo "no $LOCAL — nothing to compare"
    exit 0
fi

mapfile -t COMMITTED_ALLOW < <(jq -r '.permissions.allow[]?' "$COMMITTED" 2>/dev/null || true)
mapfile -t LOCAL_ALLOW     < <(jq -r '.permissions.allow[]?' "$LOCAL" 2>/dev/null || true)

redundant=()
for entry in "${LOCAL_ALLOW[@]}"; do
    [[ -z "$entry" ]] && continue
    for pat in "${COMMITTED_ALLOW[@]}"; do
        [[ -z "$pat" ]] && continue
        if guard_allow_match "$entry" "$pat"; then
            redundant+=("$entry  ⊆  $pat")
            break
        fi
    done
done

# spec: guard-kit/SPEC.md §compare-settings-allow — the breadth question: guard_allow_match with the
# arguments swapped, asking whether a local glob would auto-allow a probe rather than whether a
# committed glob already grants a local entry
too_broad=()
for entry in "${LOCAL_ALLOW[@]}"; do
    [[ -z "$entry" ]] && continue
    for probe in "${GUARD_KIT_BREADTH_PROBES[@]}"; do
        [[ -z "$probe" ]] && continue
        if guard_allow_match "$probe" "$entry"; then
            too_broad+=("$entry  ⊇  $probe")
            break
        fi
    done
done

if [[ "$COUNT" -eq 1 ]]; then
    echo "${#redundant[@]} ${#too_broad[@]}"
    exit 0
fi

echo "=== settings allowlist redundancy (advisory — prune candidates) ==="
if [[ ${#redundant[@]} -eq 0 ]]; then
    echo "no redundant local entries (every $LOCAL allow entry adds coverage)"
else
    echo "${#redundant[@]} local allow entr(ies) already granted by a committed glob — safe to prune from $LOCAL:"
    echo
    printf '  %s\n' "${redundant[@]}"
    echo
    echo "help: remove each listed entry from $LOCAL — the committed pattern on the"
    echo "      right already grants it (run at close, triage step 4)."
fi

# spec: guard-kit/SPEC.md §Layout and configuration — an empty probe set omits the section entirely
[[ ${#GUARD_KIT_BREADTH_PROBES[@]} -eq 0 ]] && exit 0

echo
echo "=== settings allowlist breadth (advisory — narrowing candidates) ==="
if [[ ${#too_broad[@]} -eq 0 ]]; then
    echo "no over-broad local entries (no configured probe is auto-allowed by a $LOCAL glob)"
    exit 0
fi

echo "${#too_broad[@]} local allow entr(ies) auto-allow a configured probe — candidates to narrow in $LOCAL:"
echo
printf '  %s\n' "${too_broad[@]}"
echo
echo "help: narrow each listed glob on the left, or record that the breadth is"
echo "      intended — the probe on the right witnesses what it auto-allows."
echo "      Probes are witnesses, not a roster: no completeness is claimed, so an"
echo "      empty report is not a proof that every local glob is narrow enough."
