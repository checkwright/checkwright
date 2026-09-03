#!/usr/bin/env bash
# Cross-implementation parity for the floor predicate context-kit holds twice after the env-probe
# cut: `tool_floor_parse` and `tool_floor_check` in context-kit/lib/toolfloor.sh, and their
# compiled counterparts in native/src/toolfloor.rs. `installer/lib/doctor.sh` still calls the
# library off its own payload copy, so the shell caller set does not empty and the duplication is
# permanent — criterion 6's *unless* clause and its machine-held disposition rather than the
# deletion one (gate-sdk/SPEC.md §The port-candidate criteria, criterion 6).
#
# What is compared is *classification* over one canned corpus: the parse's four fields and the
# verdict's own words, A against B directly with no committed expected file. The kit's existing
# golden (index-tests/toolfloor-cases.sh) stays the *shell* holder's oracle; pointing a crate arm
# at it would make the verdict set a third copy to drift.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$GATE_SDK_TEST_LIB_DIR/gate.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # context-kit/
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
checks=0

# spec: gate-sdk/SPEC.md §check-gate-binary-fresh — a declaration is not a dispatch: a consumer on
# an uncovered platform vendors the shell library with no artifact behind it, and a parity
# assertion there would be vacuous rather than true. A binary that is present and refuses the arm
# is a stale binary, so it fails here.
BIN="$(gate_native_bin)"
if [[ ! -x "$BIN" ]]; then
    echo "toolfloor-parity.test: ok (0 assertions; skipped — no gate binary at $BIN, so nothing dispatches to the compiled twin)"
    exit 0
fi
[[ "$BIN" == /* ]] || BIN="$(cd "$(dirname "$BIN")" && pwd)/$(basename "$BIN")"

# The parse corpus reaches every spelling of the positional grammar: the three empty-field forms
# that are one member, an implementation constraint with no floor, an audience with and without a
# floor, and a fifth field the grammar drops rather than folding into the audience.
ELEMENTS=(
    bash:4.3
    jq
    jq:
    jq::
    jq:::
    awk::GNU
    sort::coreutils
    cargo:1.71::contributor
    cargo:::contributor
    'awk::GNU:contributor'
    'awk::GNU:contributor:dropped'
    ':4.3'
)

# The check corpus reaches every arm of the closed verdict set, including the fail-closed one on a
# banner carrying no dotted token, and both directions of the floor comparison.
PAIRS=(
    bash:4.3 ''
    bash:4.3 'GNU bash, version 5.2.37(1)-release (x86_64-pc-linux-gnu)'
    bash:4.3 'GNU bash, version 4.3.0(1)-release'
    bash:4.3 'GNU bash, version 3.2.57(1)-release (x86_64-apple-darwin20)'
    bash:4.3 'GNU bash, no version here'
    awk::GNU 'GNU Awk 5.3.1, API 4.0'
    awk::GNU 'mawk 1.3.4 20240905'
    sort::coreutils 'sort (GNU coreutils) 9.5'
    sort::coreutils 'present (/usr/bin/sort)'
    'bash:4.0:GNU' 'GNU bash, version 5.2.37(1)-release'
    'bash:4.0:GNU' 'bosh, version 3.1'
    cargo:1.71::contributor 'cargo 1.86.0 (adbf5df3f 2026-01-01)'
    cargo:1.71::contributor 'cargo 1.40.0'
    jq 'jq-1.8.2'
    git ''
    'x:1.0' '12 items, 3.4 left'
)

shell_side() {
    (
        # shellcheck source=../lib/toolfloor.sh
        source "$DIR/lib/toolfloor.sh"
        local e i
        for e in "${ELEMENTS[@]}"; do
            tool_floor_parse "$e"
            printf 'parse\t%s\t%s\t%s\t%s\t%s\n' \
                "$e" "$TOOL_FLOOR_NAME" "$TOOL_FLOOR_MIN" "$TOOL_FLOOR_IMPL" "$TOOL_FLOOR_AUDIENCE"
        done
        for ((i = 0; i < ${#PAIRS[@]}; i += 2)); do
            printf 'check\t%s\t%s\t%s\n' \
                "${PAIRS[i]}" "${PAIRS[i + 1]}" "$(tool_floor_check "${PAIRS[i]}" "${PAIRS[i + 1]}")"
        done
    )
}

# The compiled side is reached through the same binary a dispatched gate reaches, and the arm
# reports classification rather than an internal representation — `--queue-parity`'s own rule.
native_side() {
    "$BIN" --toolfloor-parity parse "${ELEMENTS[@]}" || return $?
    "$BIN" --toolfloor-parity check "${PAIRS[@]}"
}

checks=$((checks + 1))
a="$(shell_side)"; arc=$?
b="$(native_side)"; brc=$?
if [[ "$arc" -ne 0 || "$brc" -ne 0 ]]; then
    echo "  FAIL: a side could not report (shell exit $arc, binary exit $brc)"
    fails=$((fails + 1))
elif [[ -z "$a" ]]; then
    echo "  FAIL: the shell side classified nothing — a vacuous agreement, not a parity hold"
    fails=$((fails + 1))
elif [[ "$a" != "$b" ]]; then
    echo "  FAIL: the two implementations disagree about the same corpus:"
    diff <(printf '%s\n' "$a") <(printf '%s\n' "$b") | sed 's/^/    /'
    fails=$((fails + 1))
fi

# The corpus must actually reach the branches the comparison is bought for: an agreement over a
# corpus that classifies nothing is the vacuity this lane exists to end, arriving one layer up.
# Each is read off the shell side, the one that owns the predicate under obligation.
T=$'\t'
have() {   # $1=label $2=grep -E pattern
    checks=$((checks + 1))
    grep -qE "$2" <<<"$a" || {
        echo "  FAIL [$1]: the corpus no longer exercises this branch"
        fails=$((fails + 1))
    }
}
have "verdict-ok"           "${T}ok\$"
have "verdict-absent"       "${T}absent\$"
have "verdict-below"        "${T}below 3\.2\.57 4\.3\$"
have "verdict-wrong-impl"   "${T}wrong-impl mawk\$"
have "verdict-uncomparable" "^check${T}bash:4\.3${T}GNU bash, no version here${T}uncomparable\$"
have "parse-empty-fields"   "^parse${T}jq:::${T}jq${T}${T}${T}\$"
have "parse-audience"       "^parse${T}cargo:1\.71::contributor${T}cargo${T}1\.71${T}${T}contributor\$"
have "parse-fifth-dropped"  "^parse${T}awk::GNU:contributor:dropped${T}awk${T}${T}GNU${T}contributor\$"

# `uncomparable`'s *second* cause, and the whole reason the compiled holder keeps `sort -V` as a
# spawn rather than comparing natively: a `sort` that rejects `-V` must read as unverified on both
# sides. A native comparator cannot reach this condition, so this case would silently pass on one
# holder and be unreachable on the other — which is what a canned corpus alone cannot express.
mkdir -p "$SANDBOX/bin"
printf '#!/bin/sh\nexit 1\n' > "$SANDBOX/bin/sort"
chmod +x "$SANDBOX/bin/sort"
checks=$((checks + 1))
s_out="$(PATH="$SANDBOX/bin:$PATH" bash -c 'source "$1/lib/toolfloor.sh"; tool_floor_check bash:4.3 "GNU bash, version 5.2.37(1)-release"' _ "$DIR")"
n_out="$(PATH="$SANDBOX/bin:$PATH" "$BIN" --toolfloor-parity check bash:4.3 'GNU bash, version 5.2.37(1)-release' | cut -f4)"
if [[ "$s_out" != uncomparable || "$n_out" != uncomparable ]]; then
    echo "  FAIL [sort-without-V]: a sort rejecting -V must read as unverified on both holders (shell '$s_out', binary '$n_out')"
    echo "         a compiled holder that answers 'ok' here has replaced the spawn with a native"
    echo "         comparison and lost one of the two causes the uncomparable arm exists for"
    fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "toolfloor-parity.test: $fails of $checks assertion(s) failed"
    exit 1
fi
echo "toolfloor-parity.test: ok ($checks assertions; tool_floor_parse and tool_floor_check held to their compiled twins over ${#ELEMENTS[@]} elements and $((${#PAIRS[@]} / 2)) banner pairs)"
exit 0
