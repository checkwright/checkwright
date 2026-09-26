#!/usr/bin/env bash
# Behavioral test of check-install-toolchain — the arms the one
# good/bad pair cannot hold. The pair covers whole-element parity and the
# derived-audience under-declaration; this covers the two name-set directions,
# the three spellings of an unconstrained member, the implementation-token axis
# diverging on its own, the audience axis in parity, diverging each way, and
# spelled with a Needed-by value no audience renders to, the floor divergence,
# both placement directions, and the fail-closed arm of a derived audience with
# no kit root to resolve.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GATES_DIR="$ROOT/scripts"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# $1=dir $2=file $3=rows (one `| ... |` line per member, may be empty)
write_table() {
    {
        printf '# Fixture\n\n<!-- toolchain:begin -->\n\n'
        printf '| Tool | Version | Needed by | Why |\n|---|---|---|---|\n'
        [[ -n "$3" ]] && printf '%s\n' "$3"
        printf '\n<!-- toolchain:end -->\n'
    } >"$1/$2"
}

# $1=dir  $2=install-page rows  $3=PROBE_SET body  $4=CONTRIBUTING.md rows (optional)
write_case() {
    mkdir -p "$1"
    write_table "$1" install.md "$2"
    write_table "$1" CONTRIBUTING.md "${4:-}"
    printf '#!/usr/bin/env bash\nPROBE_SET=(%s)\n' "$3" >"$1/roster.sh"
}

# $1=label $2=dir $3=want-rc $4=want-substring
check_case() {
    local out rc
    out="$(cd "$2" && gate_run check-install-toolchain "$GATES_DIR" install.md CONTRIBUTING.md roster.sh 2>&1)"; rc=$?
    if [[ "$rc" -ne "$3" ]]; then
        echo "  FAIL [$1]: want exit $3, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$4" ]] && ! grep -qF -- "$4" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$4': $out"; fails=$((fails + 1))
    fi
}

# A — a roster member the page never lists: the probed-but-not-listed direction.
write_case "$tmp/a" '| `bash` | ≥ 4.0 | every profile | runs the battery |' 'bash:4.0 sort::coreutils'
check_case "probed-not-listed" "$tmp/a" 1 "probed but not listed: sort"

# B — a page row no roster member backs: the listed-but-not-probed direction.
write_case "$tmp/b" '| `bash` | ≥ 4.0 | every profile | runs the battery |
| `cmake` | — | every profile | builds nothing here |' 'bash:4.0'
check_case "listed-not-probed" "$tmp/b" 1 "listed but not probed: cmake"

# C — the three spellings of an unconstrained member are one member, so a page of
# bare rows is in parity with a roster carrying empty trailing fields.
write_case "$tmp/c" '| `awk` | — | every profile | scans lines |
| `git` | — | every profile | reads tracked files |
| `jq` | — | every profile | parses JSON inputs |' 'awk:: git: jq'
check_case "empty-fields-are-unconstrained" "$tmp/c" 0 "INSTALL-TOOLCHAIN: clean"

# D — the implementation axis diverges alone: names and floors agree, the page
# names the wrong family. The reflex `GNU` for a coreutils member is exactly
# this red.
write_case "$tmp/d" '| `sort` | GNU | every profile | orders things |' 'sort::coreutils'
check_case "impl-token-mismatch" "$tmp/d" 1 "roster says | coreutils | every profile |, page says | GNU | every profile |"

# E — an unconstrained roster member the page decorates with a floor: the
# divergence pointing the other way from the bad fixture's.
write_case "$tmp/e" '| `jq` | ≥ 1.5 | every profile | parses JSON inputs |' 'jq'
check_case "page-invents-a-floor" "$tmp/e" 1 "roster says | — | every profile |, page says | ≥ 1.5 | every profile |"

# F — the audience axis renders and reaches parity: a roster element carrying the
# fourth field is clean only against a row that publishes it, on its own page.
write_case "$tmp/f" '| `git` | — | every profile | reads tracked files |' 'git cargo:1.71::contributor' \
    '| `cargo` | ≥ 1.71 | contributors | builds the crate |'
check_case "audience-in-parity" "$tmp/f" 0 "INSTALL-TOOLCHAIN: clean"

# G — the silent failure this axis makes possible, caught: a member quietly
# marked contributor-only drops out of every consumer's floor, and the page
# saying nothing about it is what would have hidden that.
write_case "$tmp/g" '' 'git:::contributor' '| `git` | — | every profile | reads tracked files |'
check_case "undeclared-audience" "$tmp/g" 1 "roster says | — | contributors |, page says | — | every profile |"

# H — the same divergence pointing the other way: a page that demotes a member
# the roster still holds every audience to.
write_case "$tmp/h" '' 'git' '| `git` | — | contributors | reads tracked files |'
check_case "page-invents-an-audience" "$tmp/h" 1 "roster says | — | every profile |, page says | — | contributors |"

# I — a Needed-by value no audience renders to is read as a kit list, so it reds
# as one rather than being accepted as the audience it was meant to name.
write_case "$tmp/i" '| `git` | — | every profile | reads tracked files |' 'git cargo:1.71::contributor' \
    '| `cargo` | ≥ 1.71 | contributor | builds the crate |'
check_case "unrendered-audience" "$tmp/i" 1 "page says | ≥ 1.71 | contributor |"

# J — the floor divergence: name sets agree exactly, the page states `bash`
# unconstrained while the roster pins a floor.
write_case "$tmp/j" '| `bash` | — | every profile | runs the battery |
| `git` | — | every profile | reads tracked files |' 'bash:4.0 git'
check_case "floor-divergence" "$tmp/j" 1 "roster says | ≥ 4.0 | every profile |, page says | — | every profile |"

# K — placement, one direction: a contributors row on the install page, in full
# parity with the roster, still reds, because an adopter reads that page.
write_case "$tmp/k" '| `cargo` | ≥ 1.71 | contributors | builds the crate |' 'cargo:1.71::contributor'
check_case "contributor-row-on-install-page" "$tmp/k" 1 "a row on the wrong page: cargo belongs in CONTRIBUTING.md, not install.md"

# L — placement, the other direction: an adopter's row kept in CONTRIBUTING.md.
write_case "$tmp/l" '' 'git' '| `git` | — | every profile | reads tracked files |'
check_case "adopter-row-in-contributing" "$tmp/l" 1 "a row on the wrong page: git belongs in install.md, not CONTRIBUTING.md"

# M — no row on either page is a check that could not run.
write_case "$tmp/m" '' 'git'
check_case "no-row-anywhere" "$tmp/m" 2 "no '| \`tool\` | … |' rows"

# N — a `derived` element whose derivation reaches no kit root fails CLOSED rather
# than resolving to an empty audience: an empty one would compare equal to a page
# row that declares none, which is the silent under-declaration this axis exists
# to make impossible. The kit-dirs knob names a directory that is not there, which
# is what an unresolvable derivation looks like from inside the gate.
write_case "$tmp/n" '| `bash` | — | every profile | runs the battery |' 'bash:4.0::derived'
out="$(cd "$tmp/n" \
    && gate_env GATE_SDK_KIT_DIRS=checkwright-no-such-kit \
    && gate_run check-install-toolchain "$GATES_DIR" install.md CONTRIBUTING.md roster.sh 2>&1)"; rc=$?
if [[ "$rc" -ne 2 ]]; then
    echo "  FAIL [underived-audience-fails-closed]: want exit 2, got $rc -- $out"; fails=$((fails + 1))
elif ! grep -qF -- "no kit root under this tree satisfies its predicate" <<<"$out"; then
    echo "  FAIL [underived-audience-fails-closed]: exit 2 OK but output lacks the cause: $out"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-install-toolchain.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-install-toolchain.test: ok (both name-set directions red; a bare row, a trailing empty field and a doubled empty field are one unconstrained member; the implementation token and an invented floor each red on their own; the audience axis reaches parity, reds in both directions, and reds on a Needed-by value no audience renders to; a floor divergence reds; a row on the wrong page reds in both directions; no row anywhere and a derived audience with no kit root each fail closed)"
exit 0
