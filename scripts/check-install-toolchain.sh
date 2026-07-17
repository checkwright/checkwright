#!/usr/bin/env bash
# graph: couples=docs/install.md,context-kit/bin/env-probe.sh dir=bi valve=none tier=precommit
# spec: docs/site-architecture.md §Generated projections and their freshness gates — docs/install.md's Requirements toolchain list holds name-set parity with context-kit/bin/env-probe.sh's PROBE_SET array, both directions
#
# usage: check-install-toolchain.sh [install-md] [probe-script]
#   bare: parity between docs/install.md's toolchain marker block and env-probe's PROBE_SET.
#   two args: steer onto hermetic fixture copies of each surface.
set -uo pipefail

SDK="${GATE_SDK_ROOT:-"${BASH_SOURCE[0]%/*}/../gate-sdk"}"
# shellcheck source=../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

INSTALL_MD="${1:-docs/install.md}"
PROBE="${2:-context-kit/bin/env-probe.sh}"
BEGIN="<!-- toolchain:begin -->"
END="<!-- toolchain:end -->"

[[ -f "$INSTALL_MD" ]] || { echo "check-install-toolchain: install page not found: $INSTALL_MD" >&2; exit 2; }
[[ -f "$PROBE" ]] || { echo "check-install-toolchain: probe script not found: $PROBE" >&2; exit 2; }
grep -qF -- "$BEGIN" "$INSTALL_MD" || { echo "check-install-toolchain: no toolchain marker block ($BEGIN) in $INSTALL_MD" >&2; exit 2; }

listed="$(awk -v b="$BEGIN" -v e="$END" '
    $0 == b { inb = 1; next }
    $0 == e { inb = 0; next }
    inb && /^- `/ { if (match($0, /`[^`]+`/)) print substr($0, RSTART + 1, RLENGTH - 2) }
' "$INSTALL_MD")"; st=$?
fail_closed "$st" INSTALL-TOOLCHAIN awk
[[ -n "$listed" ]] || { echo "check-install-toolchain: marker block present but no '- \`tool\`' bullets in $INSTALL_MD" >&2; exit 2; }

probe_line="$(grep -m1 -E '^PROBE_SET=\(' "$PROBE")" || probe_line=""
[[ -n "$probe_line" ]] || { echo "check-install-toolchain: no PROBE_SET=(...) array in $PROBE" >&2; exit 2; }
probe_inner="${probe_line#*(}"
probe_inner="${probe_inner%%)*}"
read -r -a probe_arr <<<"$probe_inner"
[[ ${#probe_arr[@]} -gt 0 ]] || { echo "check-install-toolchain: PROBE_SET array is empty in $PROBE" >&2; exit 2; }

probe_sorted="$(printf '%s\n' "${probe_arr[@]}" | sort -u)"; st=$?
fail_closed "$st" INSTALL-TOOLCHAIN sort
listed_sorted="$(printf '%s\n' "$listed" | sort -u)"; st=$?
fail_closed "$st" INSTALL-TOOLCHAIN sort

missing="$(comm -23 <(printf '%s\n' "$probe_sorted") <(printf '%s\n' "$listed_sorted"))"
extra="$(comm -13 <(printf '%s\n' "$probe_sorted") <(printf '%s\n' "$listed_sorted"))"

if [[ -n "$missing" || -n "$extra" ]]; then
    echo "check-install-toolchain: $INSTALL_MD toolchain list and $PROBE PROBE_SET disagree:"
    while IFS= read -r t; do [[ -n "$t" ]] && echo "  probed but not listed: $t"; done <<<"$missing"
    while IFS= read -r t; do [[ -n "$t" ]] && echo "  listed but not probed: $t"; done <<<"$extra"
    echo "  help: keep docs/install.md's toolchain bullets in name-set parity with"
    echo "        context-kit/bin/env-probe.sh's PROBE_SET array (the single owner) — add"
    echo "        the missing tool's bullet or drop the stale one."
    exit 1
fi

count="$(printf '%s\n' "$probe_sorted" | grep -c .)"
echo "INSTALL-TOOLCHAIN: clean ($count tool name(s) in parity between $INSTALL_MD and $PROBE PROBE_SET)"
exit 0
