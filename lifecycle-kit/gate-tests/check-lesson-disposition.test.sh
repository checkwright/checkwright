#!/usr/bin/env bash
# Behavioral test of checks/check-lesson-disposition.sh — the scenarios the one
# good/bad pair cannot hold: (A) a malformed disposition line reds on grammar;
# (B) a Lessons entry still present in the worktree is not a removal, so it
# needs no stamp; (C) a stored prefix matches a longer lead line (prefix join).
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
GATE="$DIR/checks/check-lesson-disposition.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4=head  $5=work  $6=evid
    local out rc
    out="$("$GATE" "$4" "$5" "$6" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$2" ]]; then
        echo "  FAIL [$1]: want exit $2, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$3" ]] && ! grep -qF -- "$3" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$3':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

cat >"$SANDBOX/head.md" <<'EOF'
## Lessons Learned

- **alpha-lesson** [attend] — a long lead line the stamp prefixes
EOF

# A: malformed evidence line (no ' — ' separator) reds on grammar.
printf '# contract: lesson-disposition v1\ndemo lesson discard just-because\n' >"$SANDBOX/evid-bad.txt"
check_case "A malformed-grammar" 1 "malformed" "$SANDBOX/head.md" "$SANDBOX/head.md" "$SANDBOX/evid-bad.txt"

# B: entry still present in the worktree (head == work) is no removal — clean, no stamp needed.
printf '# contract: lesson-disposition v1\n' >"$SANDBOX/evid-empty.txt"
check_case "B still-present-not-removed" 0 "clean" "$SANDBOX/head.md" "$SANDBOX/head.md" "$SANDBOX/evid-empty.txt"

# C: stored prefix is a true prefix of the removed entry's longer lead line.
cat >"$SANDBOX/work-cleared.md" <<'EOF'
## Lessons Learned
EOF
printf '# contract: lesson-disposition v1\ndemo lesson task new-slug — **alpha-lesson**\n' >"$SANDBOX/evid-prefix.txt"
check_case "C prefix-join" 0 "clean" "$SANDBOX/head.md" "$SANDBOX/work-cleared.md" "$SANDBOX/evid-prefix.txt"

if [[ "$fails" -gt 0 ]]; then
    echo "check-lesson-disposition.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-lesson-disposition.test.sh: clean (malformed-grammar + still-present + prefix-join, 3 cases)"
exit 0
