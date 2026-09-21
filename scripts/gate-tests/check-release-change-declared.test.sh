#!/usr/bin/env bash
# Behavioral test of check-release-change-declared — the arms the one good/bad pair
# cannot hold. The pair runs argument mode over a canned dump; this drives the live
# reading against real tags and a real index in scratch repositories, so the base
# resolution, the dormant verdict, the index read that lets the deleting commit red
# at its own pre-commit, and the rename-as-deletion reading are each proven rather
# than emulated, and it covers every refusal.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GATES_DIR="$ROOT/scripts"
FIXTURES="$ROOT/scripts/gate-tests/check-release-change-declared"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# $1=label $2=dir $3=want-rc $4=want-substring [$5=forbidden-substring] [gate args...]
check_case() {
    local label="$1" dir="$2" want="$3" has="$4" hasnt="${5:-}" out rc
    shift 5
    out="$(cd "$dir" && GATE_SDK_KIT_DIRS=alpha-kit gate_run check-release-change-declared "$GATES_DIR" "$@" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$has" ]] && ! grep -qF -- "$has" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$has': $out"; fails=$((fails + 1))
    fi
    if [[ -n "$hasnt" ]] && grep -qF -- "$hasnt" <<<"$out"; then
        echo "  FAIL [$label]: output carries '$hasnt', which must not be reported: $out"; fails=$((fails + 1))
    fi
}

# $1=dir: a repository with one kit root carrying a tool and a claimed template, and a surface
mkrepo() {
    mkdir -p "$1/alpha-kit/bin" "$1/alpha-kit/templates" "$1/.workflow"
    git -C "$1" init -q 2>/dev/null
    git -C "$1" config user.email t@example.invalid
    git -C "$1" config user.name t
    printf '#!/usr/bin/env bash\n' >"$1/alpha-kit/bin/tool.sh"
    printf '# alpha knobs\nALPHA_KIT_MODE = strict\n' >"$1/alpha-kit/templates/alpha-config.knobs"
    printf '# contract: gate-sdk/SPEC.md §upgrade-smoke — scratch surface.\n' >"$1/.workflow/release-declarations.md"
    git -C "$1" add -A
    git -C "$1" commit -qm seed
}

# A — no v* tag reachable: dormant, and the clean line says nothing was checked, even
# with an undeclared deletion staged. A non-v tag is not a base.
a="$tmp/dormant"
mkrepo "$a"
git -C "$a" tag x1
git -C "$a" rm -q alpha-kit/bin/tool.sh
check_case "no-tag-dormant" "$a" 0 "RELEASE-CHANGE-DECLARED: clean (dormant" ""

# B — a tool deleted in the index and not yet committed: red at its own pre-commit,
# naming the base the nearest v* tag resolves.
b="$tmp/staged"
mkrepo "$b"
git -C "$b" tag -a v9.1.0 -m v9.1.0
git -C "$b" rm -q alpha-kit/bin/tool.sh
check_case "staged-deletion-reds" "$b" 1 "alpha-kit/bin/tool.sh — class R" ""

# C — the same deletion with its bullet staged beside it: clean.
printf '\n## Behavior changes\n\n- **`alpha-kit/bin/tool.sh`** — deleted; run the arm.\n' >>"$b/.workflow/release-declarations.md"
git -C "$b" add .workflow/release-declarations.md
check_case "declared-deletion-clean" "$b" 0 "1 removed kit tool path(s)" ""

# D — the bullet in the working tree but not staged: the index is what the commit
# carries, so the gate still reds.
d="$tmp/unstaged"
mkrepo "$d"
git -C "$d" tag -a v9.1.0 -m v9.1.0
git -C "$d" rm -q alpha-kit/bin/tool.sh
printf '\n## Behavior changes\n\n- **`alpha-kit/bin/tool.sh`** — deleted.\n' >>"$d/.workflow/release-declarations.md"
check_case "unstaged-surface-reds" "$d" 1 "class R" ""

# E — a rename is a deletion of its old path.
e="$tmp/rename"
mkrepo "$e"
git -C "$e" tag -a v9.1.0 -m v9.1.0
git -C "$e" mv alpha-kit/bin/tool.sh alpha-kit/bin/renamed.sh
check_case "rename-is-deletion" "$e" 1 "alpha-kit/bin/tool.sh — class R" "renamed.sh"

# F — a claimed template: a comment-only edit is no change, a data edit is class T.
f="$tmp/template"
mkrepo "$f"
git -C "$f" tag -a v9.1.0 -m v9.1.0
printf '# alpha knobs, reworded\nALPHA_KIT_MODE = strict\n' >"$f/alpha-kit/templates/alpha-config.knobs"
git -C "$f" add -A
check_case "comment-only-template-clean" "$f" 0 "0 changed claimed template(s)" ""
printf '# alpha knobs\nALPHA_KIT_MODE = lax\n' >"$f/alpha-kit/templates/alpha-config.knobs"
git -C "$f" add -A
check_case "data-template-reds" "$f" 1 "alpha-kit/templates/alpha-config.knobs — class T" ""

# G — the dir-and-basename form covers the path it names and no sibling.
check_case "dir-and-basename-covers" "$FIXTURES/bad" 1 "alpha-kit/bin/two.sh" "alpha-kit/bin/one.sh" \
    release-declarations.md changes.txt base

# --- refusals: exit 2, never a pass ------------------------------------------
h="$tmp/noheader"
mkrepo "$h"
git -C "$h" tag -a v9.1.0 -m v9.1.0
printf '## Behavior changes\n' >"$h/.workflow/release-declarations.md"
git -C "$h" add -A
check_case "surface-missing-header" "$h" 2 "lacks its \`# contract:\` header" ""

i="$tmp/nosurface"
mkrepo "$i"
git -C "$i" rm -q .workflow/release-declarations.md
check_case "surface-absent" "$i" 2 "is absent" ""

j="$tmp/nobase"
mkdir -p "$j"
cp "$FIXTURES/good/release-declarations.md" "$j/"
printf 'D\talpha-kit/bin/x.sh\n' >"$j/changes.txt"
mkdir -p "$j/alpha-kit/bin"
check_case "dump-without-base" "$j" 2 "base <tag>" "" release-declarations.md changes.txt base

check_case "dump-without-base-dir" "$j" 2 "together" "" release-declarations.md changes.txt

k="$tmp/notrepo"
mkdir -p "$k/alpha-kit/bin" "$k/.workflow"
check_case "outside-a-repository" "$k" 2 "" ""

if [[ "$fails" -gt 0 ]]; then
    echo "check-release-change-declared.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-release-change-declared.test: ok (no reachable v* tag is dormant and says so; a staged deletion reds at its own pre-commit and a staged bullet clears it while an unstaged one does not; a rename reds its old path; a comment-only claimed-template edit passes and a data edit is class T; the dir-and-basename form covers only the basename it names; a headerless or absent surface, a dump with no base line or no base dir, and a tree outside a repository each fail closed)"
exit 0
