#!/usr/bin/env bash
# Behavioral test of the ceiling arms the one-pair good/bad harness cannot hold:
# equal, above, below, a missing row, a stray row, a duplicate row and an absent
# ceiling file, plus the emptied-knob worklist.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # canon-kit/
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

cat >"$SANDBOX/a.md" <<'EOF'
# A

This sentence runs on past the bound on purpose, because the ceiling cases below need one finding to count, and so it keeps going with more words than the gate allows for any single sentence in a governed file here.
EOF
cat >"$SANDBOX/b.md" <<'EOF'
# B

A short, bounded paragraph.
EOF
cat >"$SANDBOX/gates.knobs" <<'EOF'
CANON_KIT_PROSE_BOUND_GLOBS[] = a.md
CANON_KIT_PROSE_BOUND_GLOBS[] = b.md
CANON_KIT_PROSE_BOUND_CEILING_FILE = ceiling.txt
CANON_KIT_PROSE_BOUND_SENTENCE_MAX = 20
EOF

ceiling() {  # $@ = rows
    { echo "# contract: canon-kit/SPEC.md §check-prose-bounds — <findings> <path>"; printf '%s\n' "$@"; } >"$SANDBOX/ceiling.txt"
}

run() {  # $1..=extra env assignments; prints output, returns rc
    (cd "$SANDBOX" && gate_env CANON_KIT_KNOB_FILE="$SANDBOX/gates.knobs" "$@" \
        && gate_run check-prose-bounds "$DIR/checks" 2>&1)
}

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4=output  $5=rc
    if [[ "$5" -ne "$2" ]]; then
        echo "  FAIL [$1]: want exit $2, got $5 -- $4"; fails=$((fails + 1)); return
    fi
    if [[ -n "$3" ]] && ! grep -qF -- "$3" <<<"$4"; then
        echo "  FAIL [$1]: exit $5 OK but output lacks '$3':"; printf '    %s\n' "$4"
        fails=$((fails + 1))
    fi
}

ceiling "1 a.md"
out="$(run)"; rc=$?
check_case "equal" 0 "1 finding(s) held at the ceiling rows of 1 file(s)" "$out" "$rc"

ceiling "1 a.md" "4 gone.md"
out="$(run)"; rc=$?
check_case "stray-row" 0 "PROSE-BOUNDS: clean" "$out" "$rc"

cat >>"$SANDBOX/b.md" <<'EOF'

This second sentence also runs on past the bound on purpose, so that the file without a row now carries a finding of its own and the gate must name the file it has no ceiling row for at all.
EOF
out="$(run)"; rc=$?
check_case "missing-row" 1 "b.md: 1 finding(s) and no ceiling row" "$out" "$rc"

ceiling "1 a.md" "1 b.md"
out="$(run)"; rc=$?
check_case "equal-two" 0 "held at the ceiling rows of 2 file(s)" "$out" "$rc"

cat >>"$SANDBOX/a.md" <<'EOF'

A third sentence runs on past the bound on purpose as well, so that the first file carries two findings against its row of one and the gate reads that as a regression the author must shorten rather than absorb.
EOF
out="$(run)"; rc=$?
check_case "above" 1 "a.md: 2 finding(s) against a ceiling row of 1 — shorten the prose" "$out" "$rc"

ceiling "3 a.md" "1 b.md"
out="$(run)"; rc=$?
check_case "below" 1 "lower the row to '2 a.md'" "$out" "$rc"

ceiling "2 a.md" "1 b.md" "1 b.md"
out="$(run)"; rc=$?
check_case "duplicate-row" 2 "a second row for 'b.md'" "$out" "$rc"

rm -f "$SANDBOX/ceiling.txt"
out="$(run)"; rc=$?
check_case "absent-file" 2 "cannot be read" "$out" "$rc"

out="$(run CANON_KIT_PROSE_BOUND_CEILING_FILE=)"; rc=$?
check_case "worklist" 1 "3 finding(s) across 2 governed file(s)" "$out" "$rc"

if [[ "$fails" -gt 0 ]]; then
    echo "check-prose-bounds.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-prose-bounds.test.sh: clean (ceiling equal + stray row + missing row + above + below + duplicate row + absent file + emptied-knob worklist)"
exit 0
