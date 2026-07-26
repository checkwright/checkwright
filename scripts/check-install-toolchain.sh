#!/usr/bin/env bash
# graph: couples=docs/install.md,context-kit/lib/toolfloor.sh dir=bi valve=none tier=precommit
# spec: docs/site-architecture.md §Generated projections and their freshness gates — docs/install.md's Requirements toolchain list holds whole-element parity (name, version floor, implementation token) with context-kit/lib/toolfloor.sh's PROBE_SET roster, both directions
#
# usage: check-install-toolchain.sh [install-md] [roster-file]
#   bare: parity between docs/install.md's toolchain marker block and the PROBE_SET roster.
#   two args: steer onto hermetic fixture copies of each surface.
set -uo pipefail

SDK="${GATE_SDK_ROOT:-"${BASH_SOURCE[0]%/*}/../gate-sdk"}"
# shellcheck source=../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

INSTALL_MD="${1:-docs/install.md}"
ROSTER="${2:-context-kit/lib/toolfloor.sh}"
BEGIN="<!-- toolchain:begin -->"
END="<!-- toolchain:end -->"
GE="≥"

[[ -f "$INSTALL_MD" ]] || { echo "check-install-toolchain: install page not found: $INSTALL_MD" >&2; exit 2; }
[[ -f "$ROSTER" ]] || { echo "check-install-toolchain: roster file not found: $ROSTER" >&2; exit 2; }
grep -qF -- "$BEGIN" "$INSTALL_MD" || { echo "check-install-toolchain: no toolchain marker block ($BEGIN) in $INSTALL_MD" >&2; exit 2; }

# spec: docs/site-architecture.md §Generated projections and their freshness gates — the bullet's parenthetical carries the roster token verbatim, so each side normalizes to one `name:min:impl` triple and parity is a set comparison rather than a mapping table
listed="$(awk -v b="$BEGIN" -v e="$END" -v ge="$GE" '
    $0 == b { inb = 1; next }
    $0 == e { inb = 0; next }
    inb && /^- `/ {
        if (!match($0, /`[^`]+`/)) next
        name = substr($0, RSTART + 1, RLENGTH - 2)
        rest = substr($0, RSTART + RLENGTH)
        min = ""; impl = ""
        if (match(rest, /^ \([^)]*\)/)) {
            n = split(substr(rest, RSTART + 2, RLENGTH - 3), f, ",")
            for (i = 1; i <= n; i++) {
                gsub(/^[[:space:]]+|[[:space:]]+$/, "", f[i])
                if (f[i] == "") continue
                if (substr(f[i], 1, length(ge)) == ge) {
                    min = substr(f[i], length(ge) + 1)
                    gsub(/^[[:space:]]+|[[:space:]]+$/, "", min)
                } else impl = f[i]
            }
        }
        print name ":" min ":" impl
    }
' "$INSTALL_MD")"; st=$?
fail_closed "$st" INSTALL-TOOLCHAIN awk
[[ -n "$listed" ]] || { echo "check-install-toolchain: marker block present but no '- \`tool\`' bullets in $INSTALL_MD" >&2; exit 2; }

roster_line="$(grep -m1 -E '^PROBE_SET=\(' "$ROSTER")" || roster_line=""
[[ -n "$roster_line" ]] || { echo "check-install-toolchain: no PROBE_SET=(...) array in $ROSTER" >&2; exit 2; }
roster_inner="${roster_line#*(}"
roster_inner="${roster_inner%%)*}"
read -r -a roster_arr <<<"$roster_inner"
[[ ${#roster_arr[@]} -gt 0 ]] || { echo "check-install-toolchain: PROBE_SET array is empty in $ROSTER" >&2; exit 2; }

# spec: docs/site-architecture.md §Generated projections and their freshness gates — parse the roster grammar here rather than sourcing its library: a fixture path is untrusted input, so the reader that lints the array must not be made to execute the file it reads
declare -A roster_by_name=() listed_by_name=()
for _e in "${roster_arr[@]}"; do
    _name="${_e%%:*}"
    _rest="${_e#"$_name"}"; _rest="${_rest#:}"
    _min="${_rest%%:*}"
    _impl=""
    if [[ "$_rest" == *:* ]]; then _rest="${_rest#*:}"; _impl="${_rest%%:*}"; fi
    roster_by_name["$_name"]="$_name:$_min:$_impl"
done
while IFS= read -r _el; do
    [[ -n "$_el" ]] && listed_by_name["${_el%%:*}"]="$_el"
done <<<"$listed"

render() {   # $1 = name:min:impl -> the bullet parenthetical the element demands, "(none)" when unconstrained
    local rest min impl desc=""
    rest="${1#*:}"; min="${rest%%:*}"; impl="${rest#*:}"
    [[ -n "$min" ]] && desc="$GE $min"
    if [[ -n "$impl" ]]; then
        [[ -n "$desc" ]] && desc+=", "
        desc+="$impl"
    fi
    [[ -n "$desc" ]] || { printf '(none)'; return 0; }
    printf '(%s)' "$desc"
}

names="$(printf '%s\n' "${!roster_by_name[@]}" "${!listed_by_name[@]}" | sort -u)"; st=$?
fail_closed "$st" INSTALL-TOOLCHAIN sort

findings=()
while IFS= read -r _n; do
    [[ -n "$_n" ]] || continue
    if [[ -z "${roster_by_name[$_n]:-}" ]]; then
        findings+=("listed but not probed: $_n")
    elif [[ -z "${listed_by_name[$_n]:-}" ]]; then
        findings+=("probed but not listed: $_n")
    elif [[ "${roster_by_name[$_n]}" != "${listed_by_name[$_n]}" ]]; then
        findings+=("constraint mismatch: $_n — roster says $(render "${roster_by_name[$_n]}"), page says $(render "${listed_by_name[$_n]}")")
    fi
done <<<"$names"

if [[ ${#findings[@]} -gt 0 ]]; then
    echo "check-install-toolchain: $INSTALL_MD toolchain list and $ROSTER PROBE_SET disagree:"
    printf '  %s\n' "${findings[@]}"
    echo "  help: each bullet in the toolchain marker block renders its roster element"
    echo "        verbatim — \`- \\\`tool\\\` (${GE} <min-version>, <impl-token>) — …\`, either field"
    echo "        dropped where the element leaves it empty. Add the missing tool's"
    echo "        bullet, drop the stale one, or correct the parenthetical."
    exit 1
fi

echo "INSTALL-TOOLCHAIN: clean (${#roster_by_name[@]} roster element(s) in name+floor+impl parity between $INSTALL_MD and $ROSTER PROBE_SET)"
exit 0
