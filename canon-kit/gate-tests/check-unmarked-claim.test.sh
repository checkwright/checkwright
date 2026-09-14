#!/usr/bin/env bash
# Behavioral test of the arms the one-pair good/bad harness cannot hold: every
# fail-closed exit, and the two clean skips a pair cannot tell apart. A pair
# asserts one exit code per case dir, so a vocabulary that errors, a line the
# loader refuses, and a pattern that does not compile have no pair spelling; the
# inactive-by-default posture and the declared-nothing skip need configs the
# pair's own canon-config.knobs cannot also be.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # canon-kit/
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

cat >"$SANDBOX/SPEC.md" <<'EOF'
# consumer — SPEC

The engine is a small shell script, a claim in the class with no oracle behind
it.
EOF

emit() {  # $1=basename  $2..=the emitter argv, one knob-file element per word
    local f="$SANDBOX/$1"; shift
    printf 'CANON_KIT_CLAIM_CLASSES_CMD[] = %s\n' "$@" >"$f"
    printf '%s\n' 'CANON_KIT_MEASURED_SURFACE_GLOBS[] = *.md' >>"$f"
}

emit ok.knobs      printf 'engine-substrate\tthe engine is a small shell script\n'
emit err.knobs     bash -c 'exit 3'
emit notab.knobs   printf 'engine-substrate the engine is a small shell script\n'
emit badid.knobs   printf 'Engine_Substrate\tthe engine is a small shell script\n'
emit dup.knobs     printf 'engine\tone\nengine\ttwo\n'
emit extratab.knobs printf 'engine\tone\ttwo\n'
emit uncompilable.knobs printf 'engine\tthe engine is (unclosed\n'
emit nothing.knobs true
printf '%s\n' 'CANON_KIT_MEASURED_SURFACE_GLOBS[] = *.md' >"$SANDBOX/off.knobs"

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4=config
    local label="$1" want="$2" sub="$3" cfg="$4"
    local out rc
    out="$(cd "$SANDBOX" && gate_env CANON_KIT_KNOB_FILE="$SANDBOX/$cfg" \
        && gate_run check-unmarked-claim "$DIR/checks" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# The configured baseline: with a class declared, the sentence above is red.
# Every fail-closed case below differs from this one only in the vocabulary, so
# a case that exits 2 proves the loader refused rather than that nothing matched.
check_case "declared-class-is-red" 1 "falls in claim class 'engine-substrate'" ok.knobs

# Arm B, inherited whole from the shared command adapter: a roster that cannot be
# read leaves the gate holding no class, and a gate holding no class must not
# report the clean it did not earn.
check_case "emitter-error-fails-closed"  2 "exited 3"                    err.knobs
check_case "no-tab-fails-closed"         2 "line has no tab"             notab.knobs
check_case "non-slug-id-fails-closed"    2 "id is not slug-shaped"       badid.knobs
check_case "duplicate-id-fails-closed"   2 "duplicate id"                dup.knobs
check_case "extra-tab-fails-closed"      2 "extra tab in line"           extratab.knobs

# The pattern is consumer-authored, so an ERE that does not compile is the same
# unreadable-roster failure one step later, and takes the same exit.
check_case "uncompilable-ere-fails-closed" 2 "does not compile" uncompilable.knobs

# The two clean skips a fixture pair cannot tell apart: no vocabulary configured
# at all, and one configured that declared nothing. Both are green and they are
# green for different reasons, which is why the reader checks the command knob
# before it spawns the command.
check_case "no-vocabulary-is-clean"      0 "CANON_KIT_CLAIM_CLASSES_CMD unset" off.knobs
check_case "declared-nothing-is-clean"   0 "declared no claim classes"         nothing.knobs

if [[ "$fails" -gt 0 ]]; then
    echo "check-unmarked-claim.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-unmarked-claim.test.sh: clean (every unreadable roster fails closed; an unset and an empty vocabulary are two distinct clean skips)"
exit 0
