#!/usr/bin/env bash
# Behavioral test of checks/check-shim-restatement.sh — the scenarios the one
# good/bad pair cannot hold. The pair covers the plain restatement/clean split;
# this drives: a shim with no binding directive is skipped (not a shim), a
# resolved corpus that yields no n-grams is fail-closed (never a false clean —
# the guard against the "corpus never read" regression), and a paraphrase below
# N words passes (the acknowledged honest limit).
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
GATE="$DIR/checks/check-shim-restatement.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

check_case() {  # $1=label  $2=sandbox-dir  $3=want-rc  $4=want-substring  $5..=gate args
    local label="$1" dir="$2" want="$3" sub="$4"; shift 4
    local out rc
    out="$(cd "$dir" && "$GATE" "$@" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# --- skip: a file with no binding directive is not a shim; a restating one IS ---
s="$SANDBOX/skip"
mkdir -p "$s/commands"
cat >"$s/corpus.md" <<'EOF'
The generated pre-commit hook is the faithful projection of every gate manifest.
EOF
cat >"$s/commands/notes.md" <<'EOF'
The generated pre-commit hook is the faithful projection of every gate manifest.
EOF
check_case "no-directive-skipped" "$s" 0 "SHIM-RESTATEMENT: clean" commands corpus.md

# --- fail-closed: a corpus too short to yield an N-gram must not read as clean ---
e="$SANDBOX/empty"
mkdir -p "$e/commands"
cat >"$e/corpus.md" <<'EOF'
too short
EOF
cat >"$e/commands/build.md" <<'EOF'
Execute the template at templates/build.md, applying the bindings below.

## Bindings

**ritual** — pick the first task.
EOF
check_case "short-corpus-fail-closed" "$e" 2 "no" commands corpus.md

# --- honest limit: a paraphrase below N words passes (a genuine short citation) ---
p="$SANDBOX/paraphrase"
mkdir -p "$p/commands"
cat >"$p/corpus.md" <<'EOF'
The pre-commit hook is generated and must never be hand-edited by any session
under any circumstances at all, so edit the manifest and regenerate it instead.
EOF
cat >"$p/commands/build.md" <<'EOF'
Execute the template at templates/build.md, applying the bindings below.

## Bindings

**ritual** — the hook is generated (see the corpus); do not hand-edit it here.
EOF
check_case "short-overlap-passes" "$p" 0 "SHIM-RESTATEMENT: clean" commands corpus.md

if [[ "$fails" -gt 0 ]]; then
    echo "check-shim-restatement.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-shim-restatement.test.sh: clean (no-directive-skip + short-corpus-fail-closed + short-overlap-passes, 3 cases)"
exit 0
