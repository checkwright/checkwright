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

# shellcheck source=../lib/context.sh
source "$KIT/lib/context.sh"

BEGIN="<!-- context-kit:env:begin -->"
END="<!-- context-kit:env:end -->"

# shellcheck source=../lib/toolfloor.sh
source "$KIT/lib/toolfloor.sh"

# spec: context-kit/SPEC.md §bin/env-probe — package-manager detection walk; first present wins, ordered widest-family first
PM_CANDIDATES=(apt-get dnf yum pacman emerge zypper apk brew nix-env)

probe_version() {
    local tool="$1" raw="" out=""
    command -v "$tool" >/dev/null 2>&1 || return 1
    # spec: context-kit/SPEC.md §bin/env-probe — both version probes read from /dev/null: `-V` prints a banner for most tools but is an ordinary flag for some (GNU sort's version-sort), so a tool rejecting `--version` would otherwise fall through to a `-V` that reads inherited stdin and hangs the probe
    raw="$("$tool" --version 2>/dev/null </dev/null)"
    [[ -n "$raw" ]] || raw="$("$tool" -V 2>/dev/null </dev/null)"
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

# spec: context-kit/SPEC.md §bin/env-probe — the audience marker, spelled once and appended by every line that names a member, so a reader tells a floor that is theirs apart from one they are not on the hook for
audience_mark() {   # $1 = roster element -> the audience word, empty for the every-audience default
    tool_floor_parse "$1"
    [[ -n "$TOOL_FLOOR_AUDIENCE" ]] || return 0
    printf '%s-only' "$TOOL_FLOOR_AUDIENCE"
}

# spec: context-kit/SPEC.md §bin/env-probe — renders the constrained member's parenthetical; an unconstrained member carries none, so the roster's optional axis stays optional on the page too
render_floor() {   # $1 = roster element, $2 = verdict -> the trailing parenthetical, empty when the member is unconstrained
    local desc="" mark
    tool_floor_parse "$1"
    [[ -n "$TOOL_FLOOR_MIN" ]] && desc="floor $TOOL_FLOOR_MIN"
    if [[ -n "$TOOL_FLOOR_IMPL" ]]; then
        [[ -n "$desc" ]] && desc+=", "
        desc+="requires $TOOL_FLOOR_IMPL"
    fi
    mark="$(audience_mark "$1")"
    if [[ -n "$mark" ]]; then
        [[ -n "$desc" ]] && desc+=", "
        desc+="$mark"
    fi
    [[ -n "$desc" ]] || return 0
    case "$2" in
        ok)           printf ' (%s, ok)' "$desc" ;;
        uncomparable) printf ' (%s — unverified)' "$desc" ;;
        *)            printf ' (%s — below contract)' "$desc" ;;
    esac
}

tool_lines=""
absent=()
below=()
for _e in "${PROBE_SET[@]}"; do
    tool_floor_parse "$_e"
    _t="$TOOL_FLOOR_NAME"
    _mark="$(audience_mark "$_e")"
    ver="$(probe_version "$_t")" || ver=""
    verdict="$(tool_floor_check "$_e" "$ver")"
    read -r _kind _found _floor <<<"$verdict"
    case "$_kind" in
        absent) absent+=("\`$_t\`${_mark:+ ($_mark)}"); continue ;;
        below) below+=("\`$_t\` (found $_found, floor $_floor${_mark:+, $_mark})") ;;
        wrong-impl) below+=("\`$_t\` (found $_found, requires $TOOL_FLOOR_IMPL${_mark:+, $_mark})") ;;
        uncomparable) below+=("\`$_t\` (unverified against floor $TOOL_FLOOR_MIN${_mark:+, $_mark})") ;;
    esac
    tool_lines+="  - \`$_t\` — $ver$(render_floor "$_e" "$verdict")"$'\n'
done
absent_line="none"
[[ ${#absent[@]} -gt 0 ]] && absent_line="$(printf '%s ' "${absent[@]}")"
below_line="none"
if [[ ${#below[@]} -gt 0 ]]; then
    below_line="$(printf '%s; ' "${below[@]}")"
    below_line="${below_line%; }"
fi

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
    printf -- '- **Below contract:** %s\n' "$below_line"
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
