#!/usr/bin/env bash
# Behavioral test of check-lesson-disposition — the scenarios the one
# good/bad pair cannot hold: (A) a malformed disposition line reds on grammar;
# (B) a Lessons entry still present in the worktree is not a removal, so it
# needs no stamp; (C) a stored prefix matches a longer lead line (prefix join);
# (D-F) bare mode is clean outside a repository and exit 2 in one git refuses,
# a nested one git skips included.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4=head  $5=work  $6=evid
    local out rc
    out="$(gate_run check-lesson-disposition "$DIR/checks" "$4" "$5" "$6" 2>&1)"; rc=$?
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

bare_case() {  # $1=label  $2=sandbox cwd  $3=want-rc  $4=want-substring
    local out rc
    out="$(cd "$2" && gate_run check-lesson-disposition "$DIR/checks" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$3" ]] || ! grep -qF -- "$4" <<<"$out"; then
        echo "  FAIL [$1]: want exit $3 carrying '$4', got $rc -- $out"; fails=$((fails + 1))
    fi
}

# D: bare mode outside a repository has no HEAD baseline — clean.
mkdir -p "$SANDBOX/none"
bare_case "D no-repository" "$SANDBOX/none" 0 "no git repository"

# E: bare mode in a repository git refuses is the crosser's refusal, never that clean answer.
mkdir -p "$SANDBOX/refused"
printf 'gitdir: %s/absent\n' "$SANDBOX/refused" >"$SANDBOX/refused/.git"
bare_case "E refused-repository" "$SANDBOX/refused" 2 "marks a repository"

# F: a nested repository git skips is refused too, though git answers the enclosing one at exit 0.
git init -q "$SANDBOX/outer"
mkdir -p "$SANDBOX/outer/inner/.git"
printf 'garbage\n' >"$SANDBOX/outer/inner/.git/HEAD"
bare_case "F nested-skipped-repository" "$SANDBOX/outer/inner" 2 "marks a repository beneath it"

if [[ "$fails" -gt 0 ]]; then
    echo "check-lesson-disposition.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-lesson-disposition.test.sh: clean (malformed-grammar + still-present + prefix-join + no-repository + refused + nested-skipped, 6 cases)"
exit 0
