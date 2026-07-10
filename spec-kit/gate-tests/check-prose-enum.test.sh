#!/usr/bin/env bash
# Behavioral test of the config-driven paths the one-pair good/bad harness
# cannot hold: the empty-default clean skip, the fail-closed escapes (a sets
# command that errors, a line that does not parse), bracketed-vs-bare member
# matching, multi-set independence, and the subset/partitive/per-site exempt
# escapes. The good/bad pair covers a bare comma hand list dropping a member
# and the marked-subset good cases with the stock printf set; these cases reach
# what a single set command and manifest cannot.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # spec-kit/
GATE="$DIR/checks/check-prose-enum.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4..=env assignments
    local label="$1" want="$2" sub="$3"; shift 3
    local out rc
    out="$(cd "$SANDBOX" && env "$@" "$GATE" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# The empty default is a clean skip: no declared sets, nothing to check.
check_case "empty-default-skip" 0 "SPEC_KIT_ENUM_SETS_CMD unset"

cat >"$SANDBOX/three.sh" <<'EOF'
SPEC_KIT_ENUM_SETS_CMD='printf "sig\talpha\nsig\tbeta\nsig\tgamma\n"'
EOF

# Fail-closed: a sets command that errors is treated as failure, never a false clean.
cat >"$SANDBOX/boom.sh" <<'EOF'
SPEC_KIT_ENUM_SETS_CMD='echo hi; exit 4'
EOF
check_case "cmd-error-fail-closed" 2 "" SPEC_KIT_CONFIG_FILE="$SANDBOX/boom.sh"

# Fail-closed: an emitted line with no tab does not parse.
cat >"$SANDBOX/notab.sh" <<'EOF'
SPEC_KIT_ENUM_SETS_CMD='printf "sig-no-tab-here\n"'
EOF
check_case "cmd-unparsable-fail-closed" 2 "" SPEC_KIT_CONFIG_FILE="$SANDBOX/notab.sh"

# Bracketed matching: the same member reads inside [alpha] tag syntax, so a
# bracketed hand list that drops gamma trips just as a bare list does — the path
# the bare-form pair never exercises.
cat >"$SANDBOX/SPEC.md" <<'EOF'
# bracketed

The `[alpha]`/`[beta]` tags are the pair, a bracketed hand list.
EOF
check_case "bracketed-incomplete-trips" 1 "but omits: gamma" SPEC_KIT_CONFIG_FILE="$SANDBOX/three.sh"

# A complete bracketed hand list names every member, so it is clean.
cat >"$SANDBOX/SPEC.md" <<'EOF'
# bracketed complete

The `[alpha]`, `[beta]`, `[gamma]` tags are the whole set.
EOF
check_case "bracketed-complete-clean" 0 "PROSE-ENUM: clean" SPEC_KIT_CONFIG_FILE="$SANDBOX/three.sh"

# Scattered mentions are not a hand list: two members far apart in one paragraph
# with non-separator prose between them never engage.
cat >"$SANDBOX/SPEC.md" <<'EOF'
# scattered

alpha opens the run; only much later, after unrelated prose, does beta close it.
EOF
check_case "scattered-mentions-clean" 0 "PROSE-ENUM: clean" SPEC_KIT_CONFIG_FILE="$SANDBOX/three.sh"

# Multi-set independence: one complete set is clean while another incomplete set
# in the same paragraph trips, and the finding names only the drifted set.
cat >"$SANDBOX/two.sh" <<'EOF'
SPEC_KIT_ENUM_SETS_CMD='printf "col\tred\ncol\tgreen\nsig\talpha\nsig\tbeta\nsig\tgamma\n"'
EOF
cat >"$SANDBOX/SPEC.md" <<'EOF'
# two sets

The colors red, green are complete; the signals alpha, beta drop one.
EOF
check_case "multiset-only-drifted-trips" 1 "set 'sig' lists 2 of 3" SPEC_KIT_CONFIG_FILE="$SANDBOX/two.sh"
out="$(cd "$SANDBOX" && env SPEC_KIT_CONFIG_FILE="$SANDBOX/two.sh" "$GATE" 2>&1)"
if grep -qF -- "set 'col'" <<<"$out"; then
    echo "  FAIL [multiset-complete-quiet]: the complete 'col' set was flagged:"
    printf '    %s\n' "$out"; fails=$((fails + 1))
fi

# The subset marker escapes: an explicitly illustrative list is not a drifted one.
cat >"$SANDBOX/SPEC.md" <<'EOF'
# subset marked

Common signals such as alpha, beta cover most runs.
EOF
check_case "subset-marker-escape" 0 "PROSE-ENUM: clean" SPEC_KIT_CONFIG_FILE="$SANDBOX/three.sh"

# The partitive marker escapes: a selection from the set is not an enumeration of it.
cat >"$SANDBOX/SPEC.md" <<'EOF'
# partitive

On startup any of alpha, beta may fire before the rest.
EOF
check_case "partitive-marker-escape" 0 "PROSE-ENUM: clean" SPEC_KIT_CONFIG_FILE="$SANDBOX/three.sh"

# The per-site marker escapes on the line above the offending list.
cat >"$SANDBOX/SPEC.md" <<'EOF'
# per-site exempt

<!-- prose-enum-exempt: alpha and beta are the fast pair, gamma the fallback -->
The fast pair alpha/beta short-circuits.
EOF
check_case "per-site-exempt-escape" 0 "PROSE-ENUM: clean" SPEC_KIT_CONFIG_FILE="$SANDBOX/three.sh"

if [[ "$fails" -gt 0 ]]; then
    echo "check-prose-enum.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-prose-enum.test.sh: clean (empty-skip + fail-closed error/parse + bracketed match + scattered non-engage + multiset independence + subset/partitive/per-site escapes)"
exit 0
