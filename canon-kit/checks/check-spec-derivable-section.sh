#!/usr/bin/env bash
# graph: couples=*SPEC*.md dir=one valve=none tier=align-only
# spec: canon-kit/SPEC.md §check-spec-derivable-section — a banned-heading section may not be a fenced code dump above the density budget
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

ROOT="${1:-.}"
[[ -d "$ROOT" ]] || { echo "check-spec-derivable-section: not a directory: $ROOT" >&2; exit 2; }

mapfile -t specs < <(spec_canonical_specs "$ROOT" | sort)
[[ ${#specs[@]} -eq 0 ]] && { echo "SPEC-DERIVABLE-SECTION: clean (0 $CANON_KIT_SPEC_NAME found)"; exit 0; }

banned=""
for h in "${CANON_KIT_BANNED_HEADINGS[@]}"; do
    banned+=$'\x01'"$(printf '%s' "$h" | tr '[:upper:]' '[:lower:]')"
done
banned+=$'\x01'

out="$(awk -v banned="$banned" -v density="$CANON_KIT_DERIVABLE_DENSITY" \
           -v pointer="$CANON_KIT_DERIVABLE_POINTER_REGEX" '
    function flush(   dens) {
        if (cur_heading == "") return
        if (is_banned && !has_pointer && body_nonblank > 0) {
            dens = int(fenced_nonblank * 100 / body_nonblank)
            if (fenced_nonblank * 100 > density * body_nonblank)
                printf "%s: section \"%s\" (line %d) is %d%% fenced (banned heading, budget %d%%) — shed to a one-line index pointer\n", \
                    cur_file, cur_heading, cur_line, dens, density
        }
        cur_heading = ""; is_banned = 0; has_pointer = 0; body_nonblank = 0; fenced_nonblank = 0
    }
    FNR == 1 { flush(); in_fence = 0 }
    {
        if ($0 ~ /^```/) {
            in_fence = !in_fence
            if (cur_heading != "") { body_nonblank++; fenced_nonblank++ }
            next
        }
        if (!in_fence && $0 ~ /^#{1,6}[[:space:]]/) {
            flush()
            h = $0; sub(/^#{1,6}[[:space:]]+/, "", h); sub(/[[:space:]]+$/, "", h)
            cur_heading = h; cur_file = FILENAME; cur_line = FNR
            is_banned = (index(banned, "\x01" tolower(h) "\x01") > 0)
            next
        }
        if (cur_heading != "" && $0 ~ /[^[:space:]]/) {
            body_nonblank++
            if (in_fence) fenced_nonblank++
            if ($0 ~ pointer) has_pointer = 1
        }
    }
    END { flush() }
' "${specs[@]}")"; st=$?
fail_closed "$st" check-spec-derivable-section awk

if [[ -n "$out" ]]; then
    n="$(printf '%s\n' "$out" | grep -c .)"
    echo "SPEC-DERIVABLE-SECTION: ${n} violation(s):"
    printf '%s\n' "$out" | sed 's/^/  /'
    echo "  help: a banned-heading section (${CANON_KIT_BANNED_HEADINGS[*]}) that is mostly a code dump drifts — shed the body to a one-line index pointer (cite the code), keeping the prose that owns semantics"
    exit 1
fi
echo "SPEC-DERIVABLE-SECTION: clean (${#specs[@]} $CANON_KIT_SPEC_NAME, no banned-heading section exceeds the ${CANON_KIT_DERIVABLE_DENSITY}% fenced budget)"
exit 0
