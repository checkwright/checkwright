#!/usr/bin/env bash
# Behavioral test of checks/check-tracking-claim.sh over real git repos — the
# ignore-rule cases the good/bad pair cannot reach. A fixture case dir cannot
# ship a gitignored member (git never reports a tracked path as ignored, and a
# fixture file must be tracked to survive a clone), and the regression this
# guards is a claim that verifies with the runtime file on disk but reds in a
# clean checkout where it is absent. So the rule-based ignored side is exercised
# in sandbox repos here, the check-close-surfaces precedent. The pair covers the
# tracked/near-miss cases.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # canon-kit/
GATE="$DIR/checks/check-tracking-claim.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

check_case() {  # $1=label  $2=dir  $3=want-rc  $4=want-substring
    local out rc
    out="$("$GATE" "$2" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$3" ]]; then
        echo "  FAIL [$1]: want exit $3, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$4" ]] && ! grep -qF -- "$4" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$4':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

seed() {  # $1=dir — an initialized empty repo the caller then populates
    mkdir -p "$1"
    git -C "$1" init -q
    git -C "$1" config user.email t@t.invalid
    git -C "$1" config user.name t
}

# --- gitignored, absent from the checkout (the CI-reproducing shape) ---
# `cache/` matches an ignore rule but no file exists on disk — exactly the case
# a presence read (`ls-files --others --ignored`) reported as unverifiable while
# the rule holds. The fix reads the rule, so the claim verifies with no file.
a="$SANDBOX/gitignored-absent"
seed "$a"
printf 'cache/\n' >"$a/.gitignore"
printf '# doc\n\n`cache/` is gitignored.\n' >"$a/README.md"
git -C "$a" add -A && git -C "$a" commit -q -m base
check_case "gitignored-absent" "$a" 0 "TRACKING-CLAIM: clean"

# --- two-tier, force-added member (the deterministic two-tier shape) ---
# `data/` is matched by a whole-dir ignore rule AND carries a force-added tracked
# member; `check-ignore --no-index` confirms the rule match despite the tracked
# member, so two-tier holds with no ignored file on disk.
b="$SANDBOX/two-tier"
seed "$b"
printf 'data/\n' >"$b/.gitignore"
mkdir "$b/data"; printf 'k\n' >"$b/data/keep.txt"
printf '# doc\n\n`data/` is two-tier.\n' >"$b/README.md"
git -C "$b" add -f data/keep.txt
git -C "$b" add .gitignore README.md
git -C "$b" commit -q -m base
check_case "two-tier-forceadded" "$b" 0 "TRACKING-CLAIM: clean"

# --- file-pattern shape has no rule-based two-tier proof (must red) ---
# `logs/` itself matches no rule (only `logs/*.log` does), so a present ignored
# member cannot be leaned on: the gate reports the claim false rather than
# passing on file presence. This is the `.workflow/` shape, filed as the
# runtime-dir-two-tier-detector debt.
c="$SANDBOX/filepattern-two-tier"
seed "$c"
printf 'logs/*.log\n' >"$c/.gitignore"
mkdir "$c/logs"; printf 'x\n' >"$c/logs/x.log"; printf 'n\n' >"$c/logs/note.md"
printf '# doc\n\n`logs/` is two-tier.\n' >"$c/README.md"
git -C "$c" add -f logs/note.md
git -C "$c" add .gitignore README.md
git -C "$c" commit -q -m base
check_case "filepattern-not-two-tier" "$c" 1 "is two-tier' is false"

if [[ "$fails" -gt 0 ]]; then
    echo "check-tracking-claim.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-tracking-claim.test.sh: clean (gitignored-absent, two-tier force-added, file-pattern-not-two-tier, 3 cases)"
exit 0
