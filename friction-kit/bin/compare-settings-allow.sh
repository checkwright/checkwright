#!/usr/bin/env bash
# spec: friction-kit/SPEC.md §compare-settings-allow — list local allow entries
# already granted by a committed glob (the deterministic prune-candidate set).
#
# Advisory, read-only — reports candidates, never mutates. A committed pattern
# subsumes a local entry when the local string matches it under shell-glob
# semantics; the harness `:*` prefix idiom (`Bash(printf:*)` ≡ any `printf …`)
# is normalized to a trailing `*` so one glob test covers both forms. It is the
# detector, not the policy: a non-redundant local entry can still be one-off
# junk worth pruning by judgment. `--count` emits the bare count.
set -uo pipefail

BIN="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../lib/guard.sh
source "$BIN/../lib/guard.sh"

COMMITTED="$FRICTION_KIT_SETTINGS"
LOCAL="$FRICTION_KIT_SETTINGS_LOCAL"

COUNT=0
case "${1:-}" in
    --count) COUNT=1 ;;
    "") ;;
    *) echo "usage: compare-settings-allow.sh [--count]" >&2; exit 2 ;;
esac

if [[ ! -f "$LOCAL" ]]; then
    [[ "$COUNT" -eq 1 ]] && { echo "0"; exit 0; }
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

if [[ "$COUNT" -eq 1 ]]; then
    echo "${#redundant[@]}"
    exit 0
fi

echo "=== settings allowlist redundancy (advisory — prune candidates) ==="
if [[ ${#redundant[@]} -eq 0 ]]; then
    echo "no redundant local entries (every $LOCAL allow entry adds coverage)"
    exit 0
fi

echo "${#redundant[@]} local allow entr(ies) already granted by a committed glob — safe to prune from $LOCAL:"
echo
printf '  %s\n' "${redundant[@]}"
echo
echo "help: remove each listed entry from $LOCAL — the committed pattern on the"
echo "      right already grants it (run at close, triage step 4)."
