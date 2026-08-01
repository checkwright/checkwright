#!/usr/bin/env bash
# Behavioral test of gate-sdk/checks/check-action-run-shell.sh — the fidelity
# limit's refused class, which the one good/bad pair cannot hold: the pair models
# exit 0 and exit 1, and every refusal here is exit 2 by the fail-closed contract.
# Each case asserts the exit status AND the construct named in the message, so the
# boundary is proved by the oracle rather than described in prose.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GATE="$ROOT/gate-sdk/checks/check-action-run-shell.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# $1=label  $2=run: value + body (verbatim workflow tail)  $3=want-rc  $4=want-substring
check_case() {
    local label="$1" tail="$2" want="$3" substr="$4" dir out rc
    dir="$tmp/$label"
    mkdir -p "$dir/tree"
    {
        printf 'name: %s\n\njobs:\n  j:\n    runs-on: ubuntu-latest\n    steps:\n' "$label"
        printf '%s\n' "$tail"
    } > "$dir/tree/wf.yml"
    out="$( cd "$dir" && "$GATE" tree 2>&1 )"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$substr" ]] && ! grep -qF -- "$substr" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$substr': $out"; fails=$((fails + 1))
    fi
}

# A — a folded block scalar. Reassembling folded lines needs YAML's folding rules,
# and mis-folding manufactures findings; refusing it also makes the literal form a
# conformance requirement inside the gate's own subject.
check_case folded '      - run: >
          echo folded
          across lines' 2 "a folded block scalar (run: >)"

check_case folded_chomped '      - run: >-
          echo folded
          across lines' 2 "a folded block scalar (run: >-)"

# B — an explicit block-scalar indentation indicator can contradict the indent the
# extractor derives from the first body line.
check_case indicator '      - run: |2
            echo indicated' 2 "an explicit block-scalar indentation indicator (run: |2)"

# C — a YAML alias as the run: value. No anchor resolution is attempted.
check_case alias '      - run: *setup' 2 "a YAML alias as the run: value (run: *setup)"

# C2 — the anchor half of the same rule. This fell through every refusal arm and
# landed on the plain-scalar counter, so an anchored body was reported skipped and
# never linted: a silent fail-open in the gate whose whole job is linting this
# shell. The body below is ShellCheck-dirty, so a regression here reads as clean.
check_case anchor '      - run: &setup |
          echo "$undefined_var"
          unused=1' 2 "a YAML anchor on the run: value (run: &setup |)"

# C3 — an anchor on a plain scalar refuses too: the anchor is what we cannot
# resolve, independent of the scalar style it decorates.
check_case anchor_plain '      - run: &setup echo hi' 2 "a YAML anchor on the run: value (run: &setup echo hi)"

# C4 — a quoted value merely beginning with & is an ordinary string, not an anchor.
check_case anchor_quoted '      - run: "echo && echo"' 0 "0 run: block(s) linted"

# D — an unbalanced GitHub expression on a body line: substituting it would mangle
# the fragment, so the line is refused rather than linted wrong.
check_case unbalanced '      - run: |
          echo "${{ github.ref_name"
          echo tail' 2 "an unbalanced GitHub expression"

# E — the chomping indicators are ordinary spellings, handled rather than refused.
# An author reaches for |- by habit, so silently skipping these would be the worst
# hole of the set.
check_case chomp_dash '      - run: |-
          echo kept' 0 "1 run: block(s) linted"
check_case chomp_plus '      - run: |+
          echo kept' 0 "1 run: block(s) linted"

# F — every refusal fires only inside the Actions-shape subject. The same folded
# scalar in a file carrying no top-level jobs:/runs: key is skipped and counted,
# never refused: the gate did not read it as shell.
mkdir -p "$tmp/outside/tree"
printf 'version: 2\n\nworkflows:\n  b:\n    steps:\n      - run: >\n          echo folded\n' \
    > "$tmp/outside/tree/foreign.yml"
out="$( cd "$tmp/outside" && "$GATE" tree 2>&1 )"; rc=$?
if [[ "$rc" -ne 0 ]]; then
    echo "  FAIL [refusal-is-subject-bound]: want exit 0, got $rc -- $out"; fails=$((fails + 1))
elif ! grep -qF -- "1 file(s) skipped by the Actions-shape predicate" <<<"$out"; then
    echo "  FAIL [refusal-is-subject-bound]: the foreign file was not counted as skipped: $out"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-action-run-shell.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-action-run-shell.test: ok (folded scalars, an explicit indentation indicator, a YAML alias, a YAML anchor on either scalar style and an unbalanced GitHub expression each refuse at exit 2 naming the construct; |- and |+ extract normally; a quoted value merely starting with & is not an anchor; a refusable construct outside the Actions-shape subject is skipped and counted rather than refused)"
exit 0
