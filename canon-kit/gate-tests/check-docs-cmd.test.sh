#!/usr/bin/env bash
# spec: canon-kit/SPEC.md §check-docs-cmd — assertion (C) reads the retired set out of the history the tree holds, which a static case dir cannot carry: the pair runs inside whatever repo vendored it, so a pair case that reds on a retirement reds only where that repo's history happens to hold the deletion. Every retirement-dependent case builds its own history here instead.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # canon-kit/
sb="$(mktemp -d)"
trap 'rm -rf "$sb"' EXIT

git -C "$sb" init -q
git -C "$sb" config user.email t@example.invalid
git -C "$sb" config user.name t
mkdir -p "$sb/bin" "$sb/comp"
printf '#!/usr/bin/env bash\n' >"$sb/bin/old.sh"
printf '# SPEC amendment: gone\n' >"$sb/comp/SPEC-gone.md"
printf '#!/usr/bin/env bash\n' >"$sb/bin/staged.sh"
printf '# doc\n' >"$sb/doc.md"
git -C "$sb" add -A
git -C "$sb" commit -qm seed
git -C "$sb" rm -q bin/old.sh comp/SPEC-gone.md
git -C "$sb" commit -qm retire

fails=0
run() { ( cd "$sb" && gate_run check-docs-cmd "$DIR/checks" doc.md 2>&1 ); }
expect() {  # $1=case $2=want-exit $3=want-line
    local out rc
    out="$(run)"; rc=$?
    if [[ "$rc" -ne "$2" ]]; then
        echo "  FAIL [$1]: want exit $2, got $rc -- $out"; fails=$((fails + 1))
    elif ! grep -qF "$3" <<<"$out"; then
        echo "  FAIL [$1]: exit $2 but output lacks '$3': $out"; fails=$((fails + 1))
    fi
}

printf '# doc\n\nThe deleted `bin/old.sh`.\n' >"$sb/doc.md"
expect committed-deletion 1 "cited path 'bin/old.sh' was retired"

git -C "$sb" rm -q bin/staged.sh
printf '# doc\n\nThe staged-deleted `bin/staged.sh`.\n' >"$sb/doc.md"
expect staged-deletion 1 "cited path 'bin/staged.sh' was retired"
git -C "$sb" reset -q -- bin/staged.sh
git -C "$sb" checkout -q -- bin/staged.sh

printf '# doc\n\nThe deleted `bin/old.sh`, as history. <!-- manifest-temporal-exempt: fixture history -->\n' >"$sb/doc.md"
expect history-valve 0 "DOCS-CMD: clean"

printf '# doc\n\nA never-tracked `bin/never.sh` and the tracked `bin/staged.sh`.\n' >"$sb/doc.md"
expect never-tracked 0 "DOCS-CMD: clean"

printf '# doc\n\nThe merged `SPEC-gone.md` amendment.\n' >"$sb/doc.md"
expect retired-amendment-basename 1 "cited amendment 'SPEC-gone.md' was retired"

printf '# doc\n\nAn illustrative `SPEC-never.md` amendment.\n' >"$sb/doc.md"
expect never-tracked-amendment 0 "DOCS-CMD: clean"

if [[ "$fails" -gt 0 ]]; then
    echo "check-docs-cmd.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-docs-cmd.test.sh: clean (retired set from the scratch repo's own history: committed and staged deletions red, the history valve and a never-tracked path clear, a retired amendment basename reds and a never-tracked one clears, 6 cases)"
exit 0
