#!/usr/bin/env bash
# Behavioral test of the refusal axis the good/bad pair cannot hold: the pair
# fixes the over-vs-under axis (exit 1 vs exit 0) and always supplies a
# well-formed ceiling file, so neither case can express an absent file or an
# unparsable row. Both are exit 2 — a ratchet with no readable ceilings is a
# broken machine, not a clean tree — which run-gate-tests reads as a harness
# error from a bad/ tree, so it lives here. The sandbox is outside any git
# repository, so every case leaves CONTEXT_KIT_RATCHET_PATHS empty; the git arm
# is what the pair exercises.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # context-kit/
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

printf 'one\ntwo\n' >"$SANDBOX/CLAUDE.md"

cfg() {  # $1=name  $2=ceiling-file body writer already run
    cat >"$SANDBOX/$1.sh" <<EOF
CONTEXT_KIT_SURFACES=("$SANDBOX/CLAUDE.md")
CONTEXT_KIT_CEILING_FILE="$SANDBOX/$1.txt"
EOF
}

cfg absent            # no .txt written at all
cfg unparsable && printf '# contract: x\nnot-a-number CLAUDE.md\n' >"$SANDBOX/unparsable.txt"
cfg sized && printf '# contract: x\n2 %s\n' "$SANDBOX/CLAUDE.md" >"$SANDBOX/sized.txt"
cfg narrowed && printf '# contract: x\n2 %s\n99 %s/gone.md\n' "$SANDBOX/CLAUDE.md" "$SANDBOX" >"$SANDBOX/narrowed.txt"

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4=config-name
    local label="$1" want="$2" sub="$3" name="$4"
    local out rc
    # spec: gate-sdk/SPEC.md §lib/gate.sh — the override is exported rather than passed through
    # `env`, because the config bridge resolves this member's knobs before the argv it builds runs
    out="$(export CONTEXT_KIT_CONFIG_FILE="$SANDBOX/$name.sh"; gate_run check-surface-ratchet "$DIR/checks" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# A ceiling file that is not there disarms the ratchet entirely, so it refuses
# rather than reporting a governed set nothing was compared against.
check_case "absent-ceiling-file-fails-closed" 2 "cannot read the ceiling file" absent

# A row the reader cannot parse is refused for the same reason one row down:
# dropping it would leave exactly that surface silently ungoverned.
check_case "unparsable-row-fails-closed" 2 "unparsable ceiling row" unparsable

# The floor the two refusals sit on: a well-formed file over a surface at its
# ceiling is clean, so the exit 2s above are the grammar failing and not the rule.
check_case "well-formed-ceiling-is-clean" 0 "SURFACE-RATCHET: clean (1 governed file" sized

# A row whose file is gone is a narrowing, and a ratchet that redded on a
# deleted surface would add a violation by removing one.
check_case "row-for-an-absent-file-is-ignored" 0 "SURFACE-RATCHET: clean (1 governed file" narrowed

if [[ "$fails" -gt 0 ]]; then
    echo "check-surface-ratchet.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-surface-ratchet.test.sh: clean (absent file + unparsable row exit 2, well-formed file clean, stale row ignored, 4 cases)"
exit 0
