#!/usr/bin/env bash
# Behavioral test of check-stage-skill-coverage — the directions the one-pair
# good/bad harness cannot hold. The pair drives the journal-citation direction;
# the harness admits one bad/ dir, so this drives the forward direction (a stage
# with no skill), the reverse direction (a skill invoking a retired stage), and
# the executed-surface resolution the citation direction rests on: a bound
# template is the surface read, and the finding names the template rather than
# the shim that named it.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

CITE='lifecycle-kit/SPEC.md §The state machine'
fails=0

seed() {  # $1=sandbox-dir; the default stage roster, every skill carrying the citation
    local sb="$1" s
    mkdir -p "$sb/commands"
    for s in scope align build validate close; do
        printf '# %s\n`bash lifecycle-kit/bin/enter-stage.sh %s`\n\nLast step (%s).\n' \
            "$s" "$s" "$CITE" >"$sb/commands/$s.md"
    done
}

check_case() {  # $1=label  $2=sandbox-dir  $3=want-rc  $4=want-substring
    local out rc
    out="$(cd "$2" && gate_run check-stage-skill-coverage "$DIR/checks" commands 2>&1)"; rc=$?
    if [[ "$rc" -ne "$3" ]]; then
        echo "  FAIL [$1]: want exit $3, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$4" ]] && ! grep -qF -- "$4" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$4':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# --- forward: a configured stage with no skill cannot be entered ---
f="$SANDBOX/forward"; seed "$f"; rm "$f/commands/close.md"
check_case "missing-skill" "$f" 1 "no skill for stage: close"

# --- reverse: a retired stage's orphan skill is a dead entry point ---
r="$SANDBOX/reverse"; seed "$r"
printf '# polish\n`bash lifecycle-kit/bin/enter-stage.sh polish`\n' >"$r/commands/polish.md"
check_case "orphan-skill" "$r" 1 "not a lifecycle stage"

# --- a non-stage skill invoking nothing is not flagged by either direction ---
n="$SANDBOX/neutral"; seed "$n"
printf '# agent-execution\nA skill that enters no stage at all.\n' >"$n/commands/agent-execution.md"
check_case "non-stage-skill-clean" "$n" 0 "STAGE-SKILL-COVERAGE: clean"

# --- executed surface: a bound template carries the citation on the shim's behalf ---
b="$SANDBOX/bound"; seed "$b"
mkdir -p "$b/templates"
printf 'The build stage. Last step (%s).\n' "$CITE" >"$b/templates/build.md"
printf 'Execute the template at templates/build.md, applying the bindings below.\n\n## Bindings\n' \
    >"$b/commands/build.md"
check_case "bound-template-clean" "$b" 0 "STAGE-SKILL-COVERAGE: clean"

# --- and an uncited bound template reds, named as the template rather than the shim ---
printf 'The build stage, with no last step at all.\n' >"$b/templates/build.md"
check_case "bound-template-uncited" "$b" 1 "no journal step:    templates/build.md"

if [[ "$fails" -gt 0 ]]; then
    echo "check-stage-skill-coverage.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-stage-skill-coverage.test.sh: clean (missing-skill + orphan-skill + non-stage-skill + bound-template cited and uncited, 5 cases)"
exit 0
