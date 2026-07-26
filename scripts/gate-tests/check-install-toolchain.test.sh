#!/usr/bin/env bash
# Behavioral test of scripts/check-install-toolchain.sh — the arms the one
# good/bad pair cannot hold. The pair covers whole-element parity and the
# floor-divergence rejection; this covers the two name-set directions the
# widened assertion still owns, the three spellings of an unconstrained member,
# and the implementation-token axis diverging on its own.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GATE="$ROOT/scripts/check-install-toolchain.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# $1=dir  $2=bullet body (one `- ...` line per roster member)  $3=PROBE_SET body
write_case() {
    mkdir -p "$1"
    {
        printf '# Install fixture\n\n<!-- toolchain:begin -->\n\n'
        printf '%s\n' "$2"
        printf '\n<!-- toolchain:end -->\n'
    } >"$1/install.md"
    printf '#!/usr/bin/env bash\nPROBE_SET=(%s)\n' "$3" >"$1/roster.sh"
}

# $1=label $2=dir $3=want-rc $4=want-substring
check_case() {
    local out rc
    out="$(cd "$2" && "$GATE" install.md roster.sh 2>&1)"; rc=$?
    if [[ "$rc" -ne "$3" ]]; then
        echo "  FAIL [$1]: want exit $3, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$4" ]] && ! grep -qF -- "$4" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$4': $out"; fails=$((fails + 1))
    fi
}

# A — a roster member the page never lists: the probed-but-not-listed direction.
write_case "$tmp/a" '- `bash` (≥ 4.0) — runs the battery.' 'bash:4.0 sort::coreutils'
check_case "probed-not-listed" "$tmp/a" 1 "probed but not listed: sort"

# B — a page bullet no roster member backs: the listed-but-not-probed direction.
write_case "$tmp/b" '- `bash` (≥ 4.0) — runs the battery.
- `cmake` — builds nothing here.' 'bash:4.0'
check_case "listed-not-probed" "$tmp/b" 1 "listed but not probed: cmake"

# C — the three spellings of an unconstrained member are one member, so a page of
# bare bullets is in parity with a roster carrying empty trailing fields.
write_case "$tmp/c" '- `awk` — scans lines.
- `git` — reads tracked files.
- `jq` — parses JSON inputs.' 'awk:: git: jq'
check_case "empty-fields-are-unconstrained" "$tmp/c" 0 "INSTALL-TOOLCHAIN: clean"

# D — the implementation axis diverges alone: names and floors agree, the page
# names the wrong family. The reflex `(GNU)` for a coreutils member is exactly
# this red.
write_case "$tmp/d" '- `sort` (GNU) — orders things.' 'sort::coreutils'
check_case "impl-token-mismatch" "$tmp/d" 1 "roster says (coreutils), page says (GNU)"

# E — an unconstrained roster member the page decorates with a floor: the
# divergence pointing the other way from the bad fixture's.
write_case "$tmp/e" '- `jq` (≥ 1.5) — parses JSON inputs.' 'jq'
check_case "page-invents-a-floor" "$tmp/e" 1 "roster says (none), page says (≥ 1.5)"

if [[ "$fails" -gt 0 ]]; then
    echo "check-install-toolchain.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-install-toolchain.test: ok (both name-set directions still red; a bare bullet, a trailing empty field and a doubled empty field are one unconstrained member; the implementation token and an invented floor each red on their own)"
exit 0
