#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §Out of scope — kpi-deprecated-surface (example template): live deprecation-marker count over the spec-kit roster, trending the between-major backlog
set -uo pipefail

CFG="${SPEC_KIT_CONFIG_FILE:-${GATE_SDK_GATES_DIR:-scripts}/spec-config.sh}"
# shellcheck source=/dev/null  # consumer config, path is config
[[ -f "$CFG" ]] && source "$CFG"

if ! declare -p SPEC_KIT_DEPRECATION_MARKERS &>/dev/null || [[ ${#SPEC_KIT_DEPRECATION_MARKERS[@]} -eq 0 ]]; then
    [[ "${1:-}" == "--trend" ]] || printf 'lead\tdeprecated surface\tn/a (no SPEC_KIT_DEPRECATION_MARKERS roster)\n'
    exit 0
fi

marker_re="$(printf '%s|' "${SPEC_KIT_DEPRECATION_MARKERS[@]}")"; marker_re="${marker_re%|}"

files=()
shopt -s nullglob globstar
for g in "${SPEC_KIT_COMMENT_SURFACE[@]:-**/*.sh}"; do
    # shellcheck disable=SC2086  # $g is a surface glob, expansion intended
    for f in $g; do [[ -f "$f" ]] && files+=("$f"); done
done
shopt -u nullglob globstar

count=0
[[ ${#files[@]} -gt 0 ]] && count="$(grep -hE -- "$marker_re" "${files[@]}" 2>/dev/null | awk 'END { print NR }')"

if [[ "${1:-}" == "--trend" ]]; then
    [[ "$count" -gt 0 ]] && printf 'deprecated-surface %d\n' "$count"
    exit 0
fi
printf 'lead\tdeprecated surface\t%d live marker(s) — decommission or re-justify at the next major\n' "$count"
exit 0
