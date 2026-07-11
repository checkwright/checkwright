#!/usr/bin/env bash
# graph: couples=*SPEC*.md dir=one valve=none tier=align-only
# spec: canon-kit/SPEC.md §check-spec-dod-singleton — a canonical spec carries the Definition-of-Done heading the configured number of times
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

ROOT="${1:-.}"
[[ -d "$ROOT" ]] || { echo "check-spec-dod-singleton: not a directory: $ROOT" >&2; exit 2; }

mapfile -t specs < <(spec_canonical_specs "$ROOT" | sort)
[[ ${#specs[@]} -eq 0 ]] && { echo "SPEC-DOD-SINGLETON: clean (0 $CANON_KIT_SPEC_NAME found)"; exit 0; }

dod_lc="$(printf '%s' "$CANON_KIT_DOD_HEADING" | tr '[:upper:]' '[:lower:]')"

errors=()
for f in "${specs[@]}"; do
    n="$(awk -v want="$dod_lc" '
        /^#{2,4}[[:space:]]/ {
            h = tolower($0); sub(/^#{2,4}[[:space:]]+/, "", h)
            if (index(h, want) > 0) c++
        }
        END { print c + 0 }
    ' "$f")"; st=$?
    fail_closed "$st" check-spec-dod-singleton awk
    if [[ "$CANON_KIT_DOD_MODE" == "exactly-one" && "$n" -ne 1 ]]; then
        errors+=("$f has $n \"$CANON_KIT_DOD_HEADING\" heading(s) (need exactly 1)")
    elif [[ "$CANON_KIT_DOD_MODE" == "at-most-one" && "$n" -gt 1 ]]; then
        errors+=("$f has $n \"$CANON_KIT_DOD_HEADING\" heading(s) (need at most 1)")
    fi
done

if [[ ${#errors[@]} -gt 0 ]]; then
    echo "SPEC-DOD-SINGLETON: ${#errors[@]} violation(s) ($CANON_KIT_DOD_MODE):"
    printf '  %s\n' "${errors[@]}"
    echo "  help: a duplicate Definition-of-Done checklist is two sources on the completion contract — fold the doubled/appended one into the canonical '## $CANON_KIT_DOD_HEADING' heading${CANON_KIT_DOD_MODE:+ (add one if missing under exactly-one)}"
    exit 1
fi
echo "SPEC-DOD-SINGLETON: clean (${#specs[@]} $CANON_KIT_SPEC_NAME scanned, $CANON_KIT_DOD_MODE)"
exit 0
