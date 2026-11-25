#!/usr/bin/env bash
# Behavioral test of check-gate-tamper's LIVE and RANGE arms — the ones the good/bad pair cannot reach.
# The pair injects both lists through --fixture, so the function that reads a staged gate
# file's bytes out of the object store (`git show :<path>` against `git show HEAD:<path>`) is
# exercised by no fixture case at all, and assertion B's added-exemption derivation with it.
#
# Each case builds a throwaway git repo, stages a commit shape (or commits a series and names it
# with --range), and runs the gate inside it, so the corpus is a real index or history rather than
# an injected list.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
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

# case_run <dir> <name> <want> <expect> [gate args…]  — the repo's index or history is already built by the caller
case_run() {
    local d="$1" name="$2" want="$3" expect="$4" out rc
    shift 4
    out="$( cd "$d" && gate_run check-gate-tamper "$CHECKS" "$@" 2>&1 )"; rc=$?
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

# G — a registered gate that no configured glob matches is still a gate file: the reader unions
#     every registered member's declaration, so co-staging product code with it is not isolated.
registered_outside_globs() {
    local d
    d="$(new_repo "$1")"
    printf 'check-sample\n' >"$d/scripts/gates.list"
    printf 'DELEGATION_KIT_GATE_FILES[] = other/check-*.sh\n' >"$d/scripts/delegation-config.knobs"
    git -C "$d" add scripts/gates.list scripts/delegation-config.knobs
    git -C "$d" commit -q -m register
    printf '# edited\n' >>"$d/scripts/check-sample.sh"
    git -C "$d" add scripts/check-sample.sh
    printf '%s\n' "$d"
}
d="$(registered_outside_globs registered-with-product)"
printf 'x\n' >"$d/product/x.txt"
git -C "$d" add product/x.txt
case_run "$d" a-registered-gate-outside-every-glob 1 "gate edit not isolated"

d="$(registered_outside_globs registered-alone)"
case_run "$d" a-registered-gate-outside-every-glob-staged-alone 0 "registered gate file(s) covered"

# H — the install manifest at the toplevel is meta-layer, so an update's commit co-staging it with a
#     rewritten gate file is isolated; a file of that name anywhere below the toplevel is not.
d="$(new_repo manifest-beside-gate)"
printf '# edited\n' >>"$d/scripts/check-sample.sh"
printf '{}\n' >"$d/checkwright.lock"
git -C "$d" add scripts/check-sample.sh checkwright.lock
case_run "$d" the-install-manifest-beside-a-gate-edit 0 "GATE-TAMPER: clean"

d="$(new_repo nested-manifest-beside-gate)"
printf '# edited\n' >>"$d/scripts/check-sample.sh"
printf '{}\n' >"$d/product/checkwright.lock"
git -C "$d" add scripts/check-sample.sh product/checkwright.lock
case_run "$d" a-nested-manifest-namesake-beside-a-gate-edit 1 "  product/checkwright.lock"

# I — a descriptor's `# exception:` line is an exemption element: one added beside a co-staged
#     file it matches reds assertion B, and the same element matching nothing staged is silent.
descriptor_repo() {
    local d
    d="$(new_repo "$1")"
    printf '# graph: dir=one valve=none tier=precommit\n# exception: product/*.txt # permanent: seeded\n' \
        >"$d/scripts/check-described.gate"
    git -C "$d" add scripts/check-described.gate
    git -C "$d" commit -q -m describe
    printf '# exception: %s # until: some-task\n' "$2" >>"$d/scripts/check-described.gate"
    git -C "$d" add scripts/check-described.gate
    printf '%s\n' "$d"
}
d="$(descriptor_repo descriptor-self-serving 'product/*.md')"
printf 'y\n' >"$d/product/excused.md"
git -C "$d" add product/excused.md
case_run "$d" a-descriptor-exemption-excusing-a-co-staged-file 1 "product/*.md -> product/excused.md"

d="$(descriptor_repo descriptor-unmatched 'nowhere/*.md')"
case_run "$d" a-descriptor-exemption-matching-nothing-staged 0 "GATE-TAMPER: clean"

# J — the range arm judges each commit as its own pre-commit would: a gate-only commit beside a
#     product-only commit is clean, where the same two changes in one commit red.
d="$(new_repo range-split)"
printf '# edited\n' >>"$d/scripts/check-sample.sh"
git -C "$d" commit -q -am gate-only
printf 'x\n' >"$d/product/thing.txt"
git -C "$d" add product/thing.txt
git -C "$d" commit -q -m product-only
case_run "$d" a-range-of-shape-holding-commits 0 "2 commit(s) in HEAD~2..HEAD; 0 merge(s) not judged" --range HEAD~2..HEAD

d="$(new_repo range-joined)"
printf '# edited\n' >>"$d/scripts/check-sample.sh"
printf 'x\n' >"$d/product/thing.txt"
git -C "$d" add -A
git -C "$d" commit -q -m joined
joined="$(git -C "$d" rev-parse --short HEAD)"
case_run "$d" the-same-changes-in-one-commit 1 "gate edit not isolated in commit $joined" --range HEAD~1..HEAD

# K — an exemption added beside the file it excuses reds, naming that commit and not its clean
#     predecessor; a root commit is read against the empty tree, so its whole set is added.
d="$(new_repo range-self-serving)"
printf 'ok\n' >"$d/README.md"
git -C "$d" commit -q -am clean-first
printf '#!/usr/bin/env bash\n# exception-list: the gate this repo already had\nEXEMPT=(\n    "product/*.txt"\n    "product/*.md"\n)\n' \
    >"$d/scripts/check-sample.sh"
printf 'y\n' >"$d/product/excused.md"
git -C "$d" add -A
git -C "$d" commit -q -m self-serving
bad="$(git -C "$d" rev-parse --short HEAD)"
case_run "$d" a-range-carrying-a-self-serving-exemption 1 "self-serving exemption in commit $bad" --range HEAD~2..HEAD
out="$( cd "$d" && gate_run check-gate-tamper "$CHECKS" --range HEAD~2..HEAD 2>&1 )"
if grep -qF "in commit $(git -C "$d" rev-parse --short HEAD~1)" <<<"$out"; then
    echo "  FAIL: range-self-serving named its clean predecessor: $out"; fails=$((fails + 1))
fi

d="$(new_repo range-root)"
seed="$(git -C "$d" rev-parse HEAD)"
git -C "$d" checkout -q --orphan unrelated
git -C "$d" rm -rfq .
mkdir -p "$d/scripts" "$d/product"
printf '#!/usr/bin/env bash\n# exception-list: seeded at the root\nEXEMPT=(\n    "product/*.txt"\n)\n' >"$d/scripts/check-sample.sh"
printf 'x\n' >"$d/product/thing.txt"
git -C "$d" add -A
git -C "$d" commit -q -m root
case_run "$d" a-root-commit-read-against-the-empty-tree 1 "product/*.txt -> product/thing.txt" --range "$seed..unrelated"

# L — a merge is counted and not judged, and the commits it brings in are judged one by one.
d="$(new_repo range-merge)"
git -C "$d" checkout -q -b side
printf '# edited\n' >>"$d/scripts/check-sample.sh"
printf 'x\n' >"$d/product/side.txt"
git -C "$d" add -A
git -C "$d" commit -q -m side-joined
side="$(git -C "$d" rev-parse --short HEAD)"
git -C "$d" checkout -q -
mkdir -p "$d/product"
printf 'z\n' >"$d/product/main.txt"
git -C "$d" add -A
git -C "$d" commit -q -m main-product
git -C "$d" merge -q --no-edit side
case_run "$d" a-merge-brings-its-commits-in 1 "gate edit not isolated in commit $side" --range HEAD~1~1..HEAD

d="$(new_repo range-merge-clean)"
git -C "$d" checkout -q -b side
printf 'x\n' >"$d/product/side.txt"
git -C "$d" add -A
git -C "$d" commit -q -m side-product
git -C "$d" checkout -q -
printf '# edited\n' >>"$d/scripts/check-sample.sh"
git -C "$d" commit -q -am main-gate-only
git -C "$d" merge -q --no-edit side
case_run "$d" a-merge-of-two-clean-branches 0 "2 commit(s) in HEAD~1~1..HEAD; 1 merge(s) not judged" --range HEAD~1~1..HEAD

# M — the range spelling's refusals: a malformed range, an unknown revision, --range beside --fixture.
d="$(new_repo range-refusals)"
case_run "$d" a-bare-revision 2 "--range takes <base>..<head>" --range HEAD
case_run "$d" a-symmetric-range 2 "--range takes <base>..<head>" --range HEAD...HEAD
case_run "$d" an-empty-side 2 "--range takes <base>..<head>" --range ..HEAD
case_run "$d" an-unknown-revision 2 "'no-such-rev' names no commit" --range no-such-rev..HEAD
case_run "$d" range-beside-fixture 2 "give one, once" --range HEAD..HEAD --fixture "$tmp"
case_run "$d" an-empty-range 0 "0 commit(s) in HEAD..HEAD" --range HEAD..HEAD

if [[ "$fails" -gt 0 ]]; then
    echo "check-gate-tamper.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-gate-tamper.test: ok (range arm: each commit judged as its own pre-commit, the offending commit named, a root commit read against the empty tree, a merge counted and its commits judged, the range spelling's refusals; live arm: HEAD-resident exemptions excluded, a newly added one excusing a co-staged file rejected, an unmatched one silent, a descriptor's # exception: element read alike, a gate-file deletion read as empty, a registered gate outside every glob covered, the toplevel install manifest meta-layer and a nested namesake not)"
exit 0
