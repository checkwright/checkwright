#!/usr/bin/env bash
# Behavioral test of check-gate-tamper's LIVE arm — the one the good/bad pair cannot reach.
# The pair injects both lists through --fixture, so the function that reads a staged gate
# file's bytes out of the object store (`git show :<path>` against `git show HEAD:<path>`) is
# exercised by no fixture case at all, and assertion B's added-exemption derivation with it.
#
# Each case builds a throwaway git repo, stages a commit shape, and runs the gate inside it, so
# the corpus is a real index rather than an injected list.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # delegation-kit/
CHECKS="$DIR/checks"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# new_repo <name> — a git repo whose HEAD carries one gate file with one exemption element
new_repo() {
    local d="$tmp/$1"
    mkdir -p "$d/scripts" "$d/product"
    git -C "$d" init -q
    git -C "$d" config user.email tamper@example.invalid
    git -C "$d" config user.name tamper
    printf '#!/usr/bin/env bash\n# exception-list: the gate this repo already had\nEXEMPT=(\n    "product/*.txt"\n)\n' \
        >"$d/scripts/check-sample.sh"
    printf 'seed\n' >"$d/README.md"
    git -C "$d" add -A
    git -C "$d" commit -q -m seed
    printf '%s\n' "$d"
}

# case_run <name> <want> <expect>  — the repo's index is already staged by the caller
case_run() {
    local d="$1" name="$2" want="$3" expect="$4" out rc
    out="$( cd "$d" && gate_run check-gate-tamper "$CHECKS" 2>&1 )"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL: $name expected exit $want, got $rc: $out"; fails=$((fails + 1)); return
    fi
    if ! grep -qF -- "$expect" <<<"$out"; then
        echo "  FAIL: $name exit OK but output lacks '$expect': $out"; fails=$((fails + 1))
    fi
}

# A — a gate file staged with meta-layer paths only is clean, and the exemption its HEAD
#     version already carries is not "added" even though a co-staged file would match it.
d="$(new_repo head-resident)"
printf 'x\n' >"$d/product/thing.txt"
printf 'note\n' >"$d/scripts/note.md"
git -C "$d" add scripts/note.md
case_run "$d" head-resident-exemption-is-not-added 0 "GATE-TAMPER: clean"

# B — the same repo, now staging the product file the HEAD-resident exemption matches: the
#     exemption is still not new, so assertion B stays silent and only assertion A speaks.
d="$(new_repo a-not-b)"
printf 'x\n' >"$d/product/thing.txt"
printf '# edited\n' >>"$d/scripts/check-sample.sh"
git -C "$d" add product/thing.txt scripts/check-sample.sh
out="$( cd "$d" && gate_run check-gate-tamper "$CHECKS" 2>&1 )"
if ! grep -qF "gate edit not isolated" <<<"$out"; then
    echo "  FAIL: a-not-b did not report the isolation violation: $out"; fails=$((fails + 1))
fi
if grep -qF "self-serving exemption" <<<"$out"; then
    echo "  FAIL: a-not-b reported a HEAD-resident exemption as newly added: $out"; fails=$((fails + 1))
fi

# C — the byte reader proper: a *new* element appended to the staged blob of a gate file,
#     matching a file staged in the same commit. Only `git show :<path>` sees it.
d="$(new_repo self-serving)"
printf '#!/usr/bin/env bash\n# exception-list: the gate this repo already had\nEXEMPT=(\n    "product/*.txt"\n    "product/*.md"\n)\n' \
    >"$d/scripts/check-sample.sh"
printf 'y\n' >"$d/product/excused.md"
git -C "$d" add scripts/check-sample.sh product/excused.md
case_run "$d" a-newly-added-exemption-excusing-a-co-staged-file 1 "product/*.md -> product/excused.md"

# D — the same added element with nothing co-staged that it matches: assertion B is silent, so
#     the derivation is not simply reporting every element it can parse.
d="$(new_repo added-but-unmatched)"
printf '#!/usr/bin/env bash\n# exception-list: the gate this repo already had\nEXEMPT=(\n    "product/*.txt"\n    "nowhere/*.md"\n)\n' \
    >"$d/scripts/check-sample.sh"
git -C "$d" add scripts/check-sample.sh
out="$( cd "$d" && gate_run check-gate-tamper "$CHECKS" 2>&1 )"
if grep -qF "self-serving exemption" <<<"$out"; then
    echo "  FAIL: added-but-unmatched fired on an exemption matching nothing staged: $out"
    fails=$((fails + 1))
fi

# E — an index with no gate file staged: assertion A never opens, so a non-meta path is fine.
d="$(new_repo no-gate-file)"
printf 'z\n' >"$d/product/thing.txt"
git -C "$d" add product/thing.txt
case_run "$d" a-commit-touching-no-gate-file 0 "1 staged path(s)"

# F — a staged *deletion* of a gate file: the staged blob does not exist, and the reader must
#     treat that as no exemptions rather than as a failure to read.
d="$(new_repo staged-deletion)"
git -C "$d" rm -q scripts/check-sample.sh
case_run "$d" a-staged-gate-file-deletion 0 "GATE-TAMPER: clean"

if [[ "$fails" -gt 0 ]]; then
    echo "check-gate-tamper.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-gate-tamper.test: ok (live arm: HEAD-resident exemptions excluded, a newly added one excusing a co-staged file rejected, an unmatched one silent, a gate-file deletion read as empty)"
exit 0
