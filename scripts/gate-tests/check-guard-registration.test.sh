#!/usr/bin/env bash
# Behavioral test of check-guard-registration — the fail-closed arms the one
# good/bad pair cannot hold, because a rejection case is exit 1 by contract and
# each of these is a refusal at exit 2 (guard-kit/SPEC.md §check-guard-registration).
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GATES_DIR="$ROOT/scripts"
GOOD="$ROOT/scripts/gate-tests/check-guard-registration/good"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# The corpus is `git ls-files` from the working directory, so every case runs in
# the good case dir, whose tracked files are the good corpus.
# $1=label $2=spec $3=table $4=want-rc $5=want-substring
check_case() {
    local out rc
    out="$(cd "$GOOD" && gate_run check-guard-registration "$GATES_DIR" "$2" "$3" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$4" ]]; then
        echo "  FAIL [$1]: want exit $4, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if ! grep -qF -- "$5" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$5': $out"; fails=$((fails + 1))
    fi
}

# The good pair is clean through this harness too, so every refusal below is the
# one change its case makes.
check_case "baseline-clean" "guard-kit/SPEC.md" "table.tsv" 0 "GUARD-REGISTRATION: clean (3 rule(s)"

printf '# Fixture\n\n## Some other section\n\n- **Rule** (`alpha`) — Declares `raw`.\n' >"$tmp/no-section.md"
check_case "roster-absent" "$tmp/no-section.md" "table.tsv" 2 'no "The rule roster" section heading'

printf '# Fixture\n\n### The rule roster\n\nProse only.\n' >"$tmp/no-items.md"
check_case "roster-without-items" "$tmp/no-items.md" "table.tsv" 2 "carries no \`- \` item"

printf '### The rule roster\n\n- **Rule** (`alpha`) — Declares `sq dq hd`.\n- **Rule** (`beta`) — Declares `dq`.\n' >"$tmp/bad-view.md"
check_case "declaration-unknown-view" "$tmp/bad-view.md" "table.tsv" 2 "a declaration clause the grammar cannot read"

printf '### The rule roster\n\n- **Rule** (`alpha`) — Declares `raw, and nothing closes it.\n' >"$tmp/unclosed.md"
check_case "declaration-unclosed" "$tmp/unclosed.md" "table.tsv" 2 "a declaration clause the grammar cannot read"

printf 'alpha sq dq hd\n' >"$tmp/no-tab.tsv"
check_case "table-line-without-tab" "guard-kit/SPEC.md" "$tmp/no-tab.tsv" 2 "a table line of no known shape"

printf 'alpha\tdq\n' >"$tmp/bad-view.tsv"
check_case "table-line-unknown-view" "guard-kit/SPEC.md" "$tmp/bad-view.tsv" 2 "a table line of no known shape"

check_case "spec-unreadable" "$tmp/absent.md" "table.tsv" 2 "cannot read"
check_case "table-unreadable" "guard-kit/SPEC.md" "$tmp/absent.tsv" 2 "cannot read"

if [[ "$fails" -gt 0 ]]; then
    echo "check-guard-registration.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-guard-registration.test: ok (the good pair is clean here too; an absent roster, a roster with no item, a declaration naming an unknown view or left unclosed, a table line with no tab or an unknown view, and an unreadable spec or table each refuse at exit 2 naming what could not be read)"
exit 0
