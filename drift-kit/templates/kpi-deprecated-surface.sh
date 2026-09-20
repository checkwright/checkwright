#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §Out of scope — kpi-deprecated-surface (example template): live deprecation-marker count over the canon-kit roster, trending the between-major backlog
# no-port: CLAUDE.md §The provenance seam (never cross it) — the seam's **worked example**. It ships as a template rather than as a bundled member precisely because its marker spelling is a consumer literal and it reads consumer array knobs through the knob-values arm; porting it would publish consumer rule content as kit mechanism in every adopter's binary. Structural, not a sizing judgment.
set -uo pipefail

trend=0
[[ "${1:-}" == "--trend" ]] && trend=1

# spec: drift-kit/SPEC.md §Out of scope — the handoff's `gate-sdk` member locates the LIBRARY, and the library spells the door: gate_native_bin_spelled resolves GATE_SDK_NATIVE_BIN's own precedence, so this plugin carries no second copy of that order and no path of its own to the binary
door=""
while IFS= read -r root; do
    # shellcheck source=/dev/null
    [[ "${root##*/}" == gate-sdk && -f "$root/lib/gate.sh" ]] && source "$root/lib/gate.sh"
done <<< "${DRIFT_KIT_KIT_ROOTS:-}"
declare -F gate_native_bin_spelled >/dev/null && door="$(gate_native_bin_spelled)"

markers=()
surface=()
if [[ ! -x "$door" ]] || ! values="$("$door" --emit knob-values CANON_KIT_DEPRECATION_MARKERS CANON_KIT_COMMENT_SURFACE 2>/dev/null)"; then
    [[ "$trend" -eq 1 ]] || printf 'lead\tdeprecated surface\tn/a (knob read failed)\n'
    exit 0
fi
while IFS=$'\t' read -r name _ value; do
    [[ -n "$value" ]] || continue
    case "$name" in
        CANON_KIT_DEPRECATION_MARKERS) markers+=("$value") ;;
        CANON_KIT_COMMENT_SURFACE) surface+=("$value") ;;
    esac
done <<< "$values"

if [[ ${#markers[@]} -eq 0 ]]; then
    [[ "$trend" -eq 1 ]] || printf 'lead\tdeprecated surface\tn/a (no CANON_KIT_DEPRECATION_MARKERS roster)\n'
    exit 0
fi

marker_re="$(printf '%s|' "${markers[@]}")"; marker_re="${marker_re%|}"

files=()
shopt -s nullglob globstar
for g in "${surface[@]:-**/*.sh}"; do
    # shellcheck disable=SC2086  # $g is a surface glob, expansion intended
    for f in $g; do [[ -f "$f" ]] && files+=("$f"); done
done
shopt -u nullglob globstar

count=0
[[ ${#files[@]} -gt 0 ]] && count="$(( $(grep -hE -- "$marker_re" "${files[@]}" 2>/dev/null | wc -l) ))"

if [[ "$trend" -eq 1 ]]; then
    [[ "$count" -gt 0 ]] && printf 'deprecated-surface %d\n' "$count"
    exit 0
fi
printf 'lead\tdeprecated surface\t%d live marker(s) — decommission or re-justify at the next major\n' "$count"
exit 0
