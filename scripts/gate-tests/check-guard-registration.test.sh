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

# $1=label $2=spec $3=lib $4=want-rc $5=want-substring
check_case() {
    local out rc
    out="$(gate_run check-guard-registration "$GATES_DIR" "$2" "$3" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$4" ]]; then
        echo "  FAIL [$1]: want exit $4, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if ! grep -qF -- "$5" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$5': $out"; fails=$((fails + 1))
    fi
}

# The good pair is clean through this harness too, so every refusal below is the
# one change its case makes.
check_case "baseline-clean" "$GOOD/SPEC.md" "$GOOD/guard.sh" 0 "GUARD-REGISTRATION: clean (3 rule(s)"

printf '# Fixture\n\n## Some other section\n\n1. **Rule** (`guard_rule_alpha`)\n' >"$tmp/no-section.md"
check_case "section-absent" "$tmp/no-section.md" "$GOOD/guard.sh" 2 'no "The generic ruleset" section heading'

printf '# Fixture\n\n## The generic ruleset\n\nProse only.\n' >"$tmp/no-items.md"
check_case "section-without-items" "$tmp/no-items.md" "$GOOD/guard.sh" 2 "carries no numbered item"

printf 'guard_rule_alpha() {\n    return 0\n}\n' >"$tmp/no-dispatcher.sh"
check_case "dispatcher-absent" "$GOOD/SPEC.md" "$tmp/no-dispatcher.sh" 2 "no \`guard_generic_rules() {\` definition"

printf 'guard_generic_rules() {\n    local cmd="$1"\n    guard_rule_alpha "$cmd"\n' >"$tmp/unterminated.sh"
check_case "dispatcher-unterminated" "$GOOD/SPEC.md" "$tmp/unterminated.sh" 2 "is never closed"

printf 'guard_generic_rules() {\n    local cmd="$1"\n    guard_rule_alpha "$cmd" || return\n}\n' >"$tmp/foreign.sh"
check_case "dispatcher-foreign-line" "$GOOD/SPEC.md" "$tmp/foreign.sh" 2 "a guard_generic_rules line of no known shape"

check_case "spec-unreadable" "$tmp/absent.md" "$GOOD/guard.sh" 2 "cannot read"

if [[ "$fails" -gt 0 ]]; then
    echo "check-guard-registration.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-guard-registration.test: ok (the good pair is clean here too; an absent section, a section with no numbered item, an absent dispatcher, an unterminated dispatcher, a dispatcher line of a foreign shape and an unreadable spec each refuse at exit 2 naming what could not be read)"
exit 0
