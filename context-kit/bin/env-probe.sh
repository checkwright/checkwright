#!/usr/bin/env bash
# spec: context-kit/SPEC.md §bin/env-probe — derives the marker-bounded machine profile into the consumer-local, gitignored profile file; hand-authored gotchas live outside the markers and survive every re-probe
# usage: env-probe.sh   (rewrites the generated block in $CONTEXT_KIT_ENV_PROFILE_FILE, seeding the file with a gotchas scaffold when absent)
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT" 2>/dev/null || { echo "env-probe: cannot enter repo root" >&2; exit 2; }
# shellcheck source=../../gate-sdk/lib/inject.sh
source "$SDK/lib/inject.sh"

_ck_cfg="${CONTEXT_KIT_CONFIG_FILE:-}"
if [[ -n "$_ck_cfg" ]]; then
    [[ -f "$_ck_cfg" ]] || {
        echo "context-kit: CONTEXT_KIT_CONFIG_FILE not found: $_ck_cfg" >&2
        exit 2
    }
    # shellcheck source=/dev/null  # consumer config path is resolved at runtime
    source "$_ck_cfg"
else
    _ck_cfg="${GATE_SDK_GATES_DIR:-scripts}/context-config.sh"
    if [[ -f "$_ck_cfg" ]]; then
        # shellcheck source=/dev/null  # consumer config path is resolved at runtime
        source "$_ck_cfg"
    fi
fi
unset _ck_cfg

: "${CONTEXT_KIT_ENV_PROFILE_FILE:=ENV.local.md}"
BEGIN="<!-- context-kit:env:begin -->"
END="<!-- context-kit:env:end -->"

# spec: context-kit/SPEC.md §bin/env-probe — the probe set: the session's own floor plus the kit tools' shared dependency (shellcheck, the gate battery's linter)
PROBE_SET=(bash git jq awk shellcheck)
# spec: context-kit/SPEC.md §bin/env-probe — package-manager detection walk; first present wins, ordered widest-family first
PM_CANDIDATES=(apt-get dnf yum pacman emerge zypper apk brew nix-env)

probe_version() {
    local tool="$1" raw="" out=""
    command -v "$tool" >/dev/null 2>&1 || return 1
    raw="$("$tool" --version 2>/dev/null)"
    [[ -n "$raw" ]] || raw="$("$tool" -V 2>/dev/null)"
    # spec: context-kit/SPEC.md §bin/env-probe — prefer the first line bearing an N.N version token (shellcheck buries it past a banner), else the first line, else the resolved path
    out="$(printf '%s\n' "$raw" | grep -m1 -E '[0-9]+\.[0-9]+')"
    [[ -n "$out" ]] || out="$(printf '%s\n' "$raw" | head -1)"
    [[ -n "$out" ]] || out="present ($(command -v "$tool"))"
    printf '%s' "$(printf '%s' "$out" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')"
}

os_line="$(uname -s -r -m 2>/dev/null || echo unknown)"
distro=""
if [[ -r /etc/os-release ]]; then
    distro="$(. /etc/os-release 2>/dev/null && printf '%s' "${PRETTY_NAME:-${ID:-}}")"
fi
[[ -n "$distro" ]] && os_line="$os_line — $distro"

pm="none detected"
for _pm in "${PM_CANDIDATES[@]}"; do
    if command -v "$_pm" >/dev/null 2>&1; then
        pm="$_pm ($(command -v "$_pm"))"
        break
    fi
done

tool_lines=""
absent=()
for _t in "${PROBE_SET[@]}"; do
    if ver="$(probe_version "$_t")"; then
        tool_lines+="  - \`$_t\` — $ver"$'\n'
    else
        absent+=("$_t")
    fi
done
absent_line="none"
[[ ${#absent[@]} -gt 0 ]] && absent_line="$(printf '`%s` ' "${absent[@]}")"

if [[ ! -f "$CONTEXT_KIT_ENV_PROFILE_FILE" ]]; then
    # spec: context-kit/SPEC.md §bin/env-probe — seed the gotchas scaffold once (outside the markers); every re-probe replaces only the block
    cat > "$CONTEXT_KIT_ENV_PROFILE_FILE" <<'SEED'
# Local environment profile

Hand-authored gotchas go here, outside the generated block below, and survive
every re-probe — the facts a probe cannot know. For example: no `dig`/`host` on
this box; resolve names with `getent hosts` or a DoH `curl`.

SEED
fi

new_body="$(
    printf '_Probed %s by context-kit env-probe — do not hand-edit inside the markers._\n\n' "$(date +%F)"
    printf -- '- **OS:** %s\n' "$os_line"
    printf -- '- **Package manager:** %s\n' "$pm"
    printf -- '- **Toolchain:**\n%s' "$tool_lines"
    printf -- '- **Absent:** %s\n' "$absent_line"
)"

# spec: context-kit/SPEC.md §bin/env-probe — change-detection: rewrite the block only when the probed content differs from disk, comparing every line but the derived `Probed <date>` line, so an unchanged box writes nothing and the date stays a last-changed signal
new_cmp="$(printf '%s\n' "$new_body" | grep -v '^_Probed ')"
if grep -qF -- "$BEGIN" "$CONTEXT_KIT_ENV_PROFILE_FILE" 2>/dev/null; then
    old_cmp="$(awk -v b="$BEGIN" -v e="$END" '$0==b{i=1;next} $0==e{i=0;next} i' "$CONTEXT_KIT_ENV_PROFILE_FILE" | grep -v '^_Probed ')"
    if [[ "$new_cmp" == "$old_cmp" ]]; then
        echo "env-probe: env profile block unchanged in $CONTEXT_KIT_ENV_PROFILE_FILE (Probed date preserved)"
        exit 0
    fi
fi

action="$(printf '%s\n' "$new_body" | inject_marker_block "$CONTEXT_KIT_ENV_PROFILE_FILE" "$BEGIN" "$END")" \
    || { echo "env-probe: failed to write profile block" >&2; exit 2; }

echo "env-probe: $action the env profile block in $CONTEXT_KIT_ENV_PROFILE_FILE"
