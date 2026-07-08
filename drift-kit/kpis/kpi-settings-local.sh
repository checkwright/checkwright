#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §Bundled KPIs — kpi-settings-local: entry count of the untracked local permission overlay
set -uo pipefail

LOCAL="${GUARD_KIT_SETTINGS_LOCAL:-.claude/settings.local.json}"

na() { [[ "${1:-}" == "--trend" ]] && exit 0; printf 'lead\tsettings.local\tn/a (%s)\n' "$2"; exit 0; }

command -v jq >/dev/null 2>&1 || na "${1:-}" "jq absent"
[[ -f "$LOCAL" ]] || na "${1:-}" "no local overlay"

n="$(jq '[.permissions.allow // [], .permissions.deny // [], .permissions.ask // []] | add | length' "$LOCAL" 2>/dev/null)" \
    || na "${1:-}" "unreadable overlay"
[[ "$n" =~ ^[0-9]+$ ]] || na "${1:-}" "unreadable overlay"

if [[ "${1:-}" == "--trend" ]]; then
    [[ "$n" -gt 0 ]] && printf 'local %d\n' "$n"
    exit 0
fi
printf 'lead\tsettings.local\t%d local override(s)\n' "$n"
exit 0
