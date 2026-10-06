#!/usr/bin/env bash
# Behavioral test of check-stage-entry assertions B, C, D and E — the
# scenarios the one-pair good/bad harness cannot hold. The good/bad fixture
# pair (--run-gate-tests) covers assertion A (prerequisite-stamp ordering: a
# close cursor with no validate stamp), and the harness admits only one bad/
# dir. Each scenario's label below is its roster entry.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
SANDBOX="$(mktemp -d)"
mkdir -p "$SANDBOX/.workflow"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

cat >"$SANDBOX/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo-iteration

---

## New Features

### unfinished-feature

still in the build queue, its spec `SPEC-demo.md`

## Technical Debt

## Done
EOF

cat >"$SANDBOX/.workflow/WORKFLOW-STATE.txt" <<'EOF'
---

demo-iteration scope aaaaaaaa 2026-06-01 none
demo-iteration build bbbbbbbb 2026-06-02 none
demo-iteration validate cccccccc 2026-06-03 none
EOF

out="$(cd "$SANDBOX" && gate_run check-stage-entry "$DIR/checks" 2>&1)"; rc=$?
if [[ "$rc" -ne 1 ]]; then
    echo "  FAIL [validate-non-empty-queue]: want exit 1, got $rc -- $out"
    fails=$((fails + 1))
elif ! grep -qF 'active queue is non-empty' <<<"$out"; then
    echo "  FAIL [validate-non-empty-queue]: exit 1 OK but output lacks 'active queue is non-empty':"
    printf '    %s\n' "$out"
    fails=$((fails + 1))
fi

# --- assertion B, drain-exempt model: tag skips at drain entry, never at successor entry ---

state_through_validate() {  # stamps scope..validate into $1/.workflow — the last stamp IS the entered stage
    mkdir -p "$1/.workflow"
    cat >"$1/.workflow/WORKFLOW-STATE.txt" <<'EOF'
---

demo-iteration scope aaaaaaaa 2026-06-01 none
demo-iteration build bbbbbbbb 2026-06-02 none
demo-iteration validate cccccccc 2026-06-03 none
EOF
}

check_case() {  # $1=label  $2=sandbox-dir  $3=want-rc  $4=want-substring
    local out rc
    out="$(cd "$2" && gate_run check-stage-entry "$DIR/checks" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$3" ]]; then
        echo "  FAIL [$1]: want exit $3, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$4" ]] && ! grep -qF -- "$4" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$4':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# B2 (good): tagged residue at drain entry — skipped, reason echoed in the clean detail.
b2="$SANDBOX/b2"
mkdir -p "$b2"
cat >"$b2/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo-iteration

---

## New Features

### spanning-feature

[drain-exempt: validate-half is validate work]

build half shipped

## Technical Debt

## Done
EOF
state_through_validate "$b2"
check_case "B2 tagged-residue-drain-entry" "$b2" 0 "validate-half is validate work"

# B3 (bad): an empty reason is malformed — the exemption does not hold.
b3="$SANDBOX/b3"
mkdir -p "$b3"
cat >"$b3/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo-iteration

---

## New Features

### spanning-feature

[drain-exempt: ]

no reason recorded

## Technical Debt

## Done
EOF
state_through_validate "$b3"
check_case "B3 empty-reason-malformed" "$b3" 1 "empty reason is malformed"

# B4 (bad): tagged residue at the drain successor's entry — the backstop runs
# with no exemption; nothing may remain active past the drain stage.
b4="$SANDBOX/b4"
mkdir -p "$b4"
cat >"$b4/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo-iteration

---

## New Features

### spanning-feature

[drain-exempt: validate-half is validate work]

never drained

## Technical Debt

## Done
EOF
mkdir -p "$b4/.workflow"
cat >"$b4/.workflow/WORKFLOW-STATE.txt" <<'EOF'
---

demo-iteration scope aaaaaaaa 2026-06-01 none
demo-iteration build bbbbbbbb 2026-06-02 none
demo-iteration validate dddddddd 2026-06-03 none
demo-iteration close eeeeeeee 2026-06-04 none
EOF
check_case "B4 tagged-residue-successor-entry" "$b4" 1 "[drain-exempt:] included"

# B5 (bad): an [observed-by:] entry blocking the drain entry is named separately,
# because landing its work does not drain it — the refusal cites its own remedy.
b5="$SANDBOX/b5"
mkdir -p "$b5"
cat >"$b5/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo-iteration

---

## New Features

### observed-feature

[observed-by: ci-run]

work landed; the run is unread

## Technical Debt

## Done
EOF
state_through_validate "$b5"
check_case "B5 observed-by-residue-drain-entry" "$b5" 1 "observation of a remote run rather than a tree state"
check_case "B5 observed-by-names-its-remedy" "$b5" 1 "splits at scope into a produce half and an observe half"

# --- assertion C: a cross-component build entry demands an align (or waiver) stamp ---

build_queue() {  # writes the name-axis-only TASK-QUEUE.md the build-entry cases share into $1
    mkdir -p "$1"
    cat >"$1/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo-iteration

---

## New Features

## Technical Debt

## Done
EOF
}

# C1 (bad): amendments in two component dirs, no align stamp, no waiver — signal 1.
c1="$SANDBOX/c1"
build_queue "$c1"
mkdir -p "$c1/.workflow" "$c1/widget-service" "$c1/panel-facade"
: >"$c1/widget-service/SPEC.md"; : >"$c1/widget-service/SPEC-foo.md"
: >"$c1/panel-facade/SPEC.md"; : >"$c1/panel-facade/SPEC-bar.md"
cat >"$c1/.workflow/WORKFLOW-STATE.txt" <<'EOF'
---

demo-iteration scope aaaaaaaa 2026-06-01 none
demo-iteration build bbbbbbbb 2026-06-02 none
EOF
check_case "C1 two-dir-no-align" "$c1" 1 "cross-component amendment signal"

# C2 (good): same two-dir amendments, with an explicit align-waived waiver.
c2="$SANDBOX/c2"
cp -r "$c1" "$c2"
cat >"$c2/.workflow/WORKFLOW-STATE.txt" <<'EOF'
---

demo-iteration scope aaaaaaaa 2026-06-01 none
demo-iteration align-waived cccccccc 2026-06-03 none
demo-iteration build bbbbbbbb 2026-06-02 none
EOF
check_case "C2 two-dir-waiver" "$c2" 0 "STAGE-ENTRY: clean"

# C3 (bad): a single amendment whose body names a second component — signal 2.
c3="$SANDBOX/c3"
build_queue "$c3"
mkdir -p "$c3/.workflow" "$c3/widget-service" "$c3/panel-facade"
: >"$c3/widget-service/SPEC.md"; : >"$c3/panel-facade/SPEC.md"
printf 'delta folds the wire change into panel-facade/SPEC.md\n' >"$c3/widget-service/SPEC-foo.md"
cat >"$c3/.workflow/WORKFLOW-STATE.txt" <<'EOF'
---

demo-iteration scope aaaaaaaa 2026-06-01 none
demo-iteration build bbbbbbbb 2026-06-02 none
EOF
check_case "C3 single-amendment-cross-component" "$c3" 1 "cross-component amendment signal"
check_case "C3 help-respell-not-a-remedy" "$c3" 1 "Respelling a body token so it stops resolving is not a remedy: the token is the amendment's reach. A generated mirror of a roster file is declared with LIFECYCLE_KIT_MIRROR_ROOT, which counts it as the component it mirrors"

# C4 (good): a single-component amendment (own dir only) — no signal.
c4="$SANDBOX/c4"
build_queue "$c4"
mkdir -p "$c4/.workflow" "$c4/widget-service"
: >"$c4/widget-service/SPEC.md"
printf 'delta stays within widget-service/SPEC.md\n' >"$c4/widget-service/SPEC-foo.md"
cat >"$c4/.workflow/WORKFLOW-STATE.txt" <<'EOF'
---

demo-iteration scope aaaaaaaa 2026-06-01 none
demo-iteration build bbbbbbbb 2026-06-02 none
EOF
check_case "C4 single-component" "$c4" 0 "STAGE-ENTRY: clean"

# C5 (good): a single real amendment (own dir) plus a templates/ stub amendment
# in another dir — the stub is a copyable skeleton, not a live amendment, so it
# is excluded from the scan and does not fabricate a second component.
c5="$SANDBOX/c5"
build_queue "$c5"
mkdir -p "$c5/.workflow" "$c5/widget-service" "$c5/some-kit/templates"
: >"$c5/widget-service/SPEC.md"
printf 'delta stays within widget-service/SPEC.md\n' >"$c5/widget-service/SPEC-foo.md"
: >"$c5/some-kit/templates/SPEC-amendment.md"
cat >"$c5/.workflow/WORKFLOW-STATE.txt" <<'EOF'
---

demo-iteration scope aaaaaaaa 2026-06-01 none
demo-iteration build bbbbbbbb 2026-06-02 none
EOF
check_case "C5 templates-stub-not-counted" "$c5" 0 "STAGE-ENTRY: clean"

# C6 (bad): an amendment glob carrying `/` matches by path — two in-flight deltas
# under one change, in two directories, are two components.
c6="$SANDBOX/c6"
build_queue "$c6"
mkdir -p "$c6/.workflow" "$c6/changes/c1/specs/alpha" "$c6/changes/c1/specs/beta"
: >"$c6/changes/c1/specs/alpha/delta.md"; : >"$c6/changes/c1/specs/beta/delta.md"
cp "$c5/.workflow/WORKFLOW-STATE.txt" "$c6/.workflow/"
export LIFECYCLE_KIT_AMENDMENT_GLOB='changes/*/specs/*/delta.md'
check_case "C6 path-glob-two-dirs" "$c6" 1 "changes/c1/specs/alpha changes/c1/specs/beta"

# C7 (good): the same glob does not reach a delta one level deeper, so an
# archived delta beside one in-flight delta is no second component.
c7="$SANDBOX/c7"
build_queue "$c7"
mkdir -p "$c7/.workflow" "$c7/changes/c1/specs/alpha" "$c7/changes/archive/c0/specs/beta"
: >"$c7/changes/c1/specs/alpha/delta.md"; : >"$c7/changes/archive/c0/specs/beta/delta.md"
cp "$c5/.workflow/WORKFLOW-STATE.txt" "$c7/.workflow/"
check_case "C7 path-glob-archived-not-counted" "$c7" 0 "STAGE-ENTRY: clean"
unset LIFECYCLE_KIT_AMENDMENT_GLOB

# C8–C12: the mirror fold. One tree — roster dirs a/ and b/, their mirrors site/a/
# and site/b/, a roster dir site/extra/ mirroring nothing, one amendment in a/.
mirror_case() {  # $1=label  $2=knob value ("" = unset)  $3=body token  $4=want-rc  $5=want-substring
    local d="$SANDBOX/m" out rc
    rm -rf "$d"; build_queue "$d"
    mkdir -p "$d/.workflow" "$d/a" "$d/b" "$d/site/a" "$d/site/b" "$d/site/extra"
    : >"$d/a/SPEC.md"; : >"$d/b/SPEC.md"
    : >"$d/site/a/SPEC.md"; : >"$d/site/b/SPEC.md"; : >"$d/site/extra/SPEC.md"
    printf 'regenerate %s after the merge\n' "$3" >"$d/a/SPEC-foo.md"
    cp "$c5/.workflow/WORKFLOW-STATE.txt" "$d/.workflow/"
    if [[ -n "$2" ]]; then
        out="$(cd "$d" && gate_env LIFECYCLE_KIT_MIRROR_ROOT="$2" && gate_run check-stage-entry "$DIR/checks" 2>&1)"; rc=$?
    else
        out="$(cd "$d" && gate_run check-stage-entry "$DIR/checks" 2>&1)"; rc=$?
    fi
    if [[ "$rc" -ne "$4" ]]; then
        echo "  FAIL [$1]: want exit $4, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if ! grep -qF -- "$5" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$5':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}
mirror_case "C8 mirror-undeclared-is-a-component" "" site/a/SPEC.md 1 "references 2 components: a site/a"
mirror_case "C9 mirror-declared-folds" site site/a/SPEC.md 0 "STAGE-ENTRY: clean"
mirror_case "C10 mirror-root-trailing-slash" site/ site/a/SPEC.md 0 "STAGE-ENTRY: clean"
mirror_case "C11 mirror-of-another-component-adds-it" site site/b/SPEC.md 1 "references 2 components: a b"
mirror_case "C12 unmirrored-dir-under-root-stays" site site/extra/SPEC.md 1 "references 2 components: a site/extra"

# --- assertion D: a build entry refuses an amendment still carrying an unrun inferred-claim marker ---

d_sandbox() {  # $1=dir  $2=amendment body — a single-component amendment at build entry
    build_queue "$1"
    mkdir -p "$1/.workflow" "$1/widget-service"
    : >"$1/widget-service/SPEC.md"
    printf '%s\n' "$2" >"$1/widget-service/SPEC-foo.md"
    cat >"$1/.workflow/WORKFLOW-STATE.txt" <<'EOF'
---

demo-iteration scope aaaaaaaa 2026-06-01 none
demo-iteration build bbbbbbbb 2026-06-02 none
EOF
}

# D1 (bad): a not-run marker — refused, naming the file and line.
d_sandbox "$SANDBOX/d1" $'intro\n**Inferred, not run:** the arm refuses — `run-gates.sh --only x`'
check_case "D1 not-run-marker" "$SANDBOX/d1" 1 "widget-service/SPEC-foo.md:2: **Inferred, not run:**"
check_case "D1 help-keep-honest-limit" "$SANDBOX/d1" 1 "A claim no delta rests on keeps its honest limit, stated in prose, when the marker goes: never delete the limit with the marker"

# D2 (good): a cannot-run marker with a reason — clean, the count in the detail.
d_sandbox "$SANDBOX/d2" '- **Inferred, cannot run before build:** the gate reds — the gate is unwritten'
check_case "D2 cannot-run-with-reason" "$SANDBOX/d2" 0 "1 cannot-run claim(s) carried"

# D4 (good): a marker inside a fence, a backticked mid-line mention in prose, and a templates/ stub — none a marker.
d_sandbox "$SANDBOX/d4" $'```\n**Inferred, not run:** fenced — `x`\n```\nprose naming `**Inferred, not run:**` mid-line'
mkdir -p "$SANDBOX/d4/some-kit/templates"
printf '**Inferred, not run:** stub — `x`\n' >"$SANDBOX/d4/some-kit/templates/SPEC-amendment.md"
check_case "D4 fence-prose-stub-not-markers" "$SANDBOX/d4" 0 "STAGE-ENTRY: clean"

d_queue() {  # $1=dir  $2=debt-section body  $3=deferred-section body — rewrites the queue D5/D6 share
    cat >"$1/TASK-QUEUE.md" <<EOF
# TASK-QUEUE.md

## Iteration: demo-iteration

---

## New Features

## Technical Debt
$2
## Deferred
$3
## Done
EOF
}
marked_entry=$'### marked-entry\n\na premise the filer did not run\n\n**Inferred, not run:** the arm refuses — `run-gates.sh --only x`'

# D5 (bad): a not-run marker on an active debt entry — refused, naming the queue line.
d_sandbox "$SANDBOX/d5" 'no marker here'
d_queue "$SANDBOX/d5" "$marked_entry" ""
check_case "D5 active-queue-entry-marker" "$SANDBOX/d5" 1 "TASK-QUEUE.md:14: **Inferred, not run:**"

# D6 (good): the same marker on a deferred entry — the pool is not read.
d_sandbox "$SANDBOX/d6" 'no marker here'
d_queue "$SANDBOX/d6" "" "$marked_entry"
check_case "D6 deferred-queue-entry-marker-unread" "$SANDBOX/d6" 0 "STAGE-ENTRY: clean"

# --- assertion E: the marker grammar is held whatever the cursor ---

e_sandbox() {  # $1=dir  $2=amendment body — d_sandbox with the cursor at align, away from audit-entry
    d_sandbox "$1" "$2"
    cat >"$1/.workflow/WORKFLOW-STATE.txt" <<'EOF'
---

demo-iteration scope aaaaaaaa 2026-06-01 none
demo-iteration align bbbbbbbb 2026-06-02 none
EOF
}

# E1 (bad): a cannot-run marker with an empty reason — malformed at an align cursor.
e_sandbox "$SANDBOX/e1" '**Inferred, cannot run before build:** the gate reds'
check_case "E1 cannot-run-empty-reason" "$SANDBOX/e1" 1 "SPEC-foo.md:1: malformed marker: **Inferred, cannot run before build:**"

# E2 (bad): a not-run marker with no command — malformed at an align cursor.
e_sandbox "$SANDBOX/e2" '**Inferred, not run:** the arm refuses. run-gates.sh --only x'
check_case "E2 not-run-no-command" "$SANDBOX/e2" 1 "SPEC-foo.md:1: malformed marker: **Inferred, not run:**"

# E3 (bad): a bold spelling mid-line, and one a hard wrap splits — each misplaced at an align cursor.
e_sandbox "$SANDBOX/e3" $'prose naming **Inferred, not run:** mid-line\na claim **Inferred,\nnot run:** wrapped — `x`'
check_case "E3 mid-line-misplaced" "$SANDBOX/e3" 1 "SPEC-foo.md:1: misplaced marker: prose naming"
check_case "E3 split-misplaced" "$SANDBOX/e3" 1 "SPEC-foo.md:2: misplaced marker: a claim"

# E4 (good): a well-formed not-run marker at an align cursor — the residue is not yet due.
e_sandbox "$SANDBOX/e4" $'intro\n**Inferred, not run:** the arm refuses — `run-gates.sh --only x`'
check_case "E4 residue-not-due" "$SANDBOX/e4" 0 "STAGE-ENTRY: clean"

# E5 (bad): a malformed marker at build entry is one finding — E's, never also D's residue.
d_sandbox "$SANDBOX/e5" '**Inferred, not run:** the arm refuses'
check_case "E5 malformed-at-audit-entry" "$SANDBOX/e5" 1 "STAGE-ENTRY: 1 prior-stage readiness issue(s)"
check_case "E5 reported-as-malformed" "$SANDBOX/e5" 1 "SPEC-foo.md:1: malformed marker:"

if [[ "$fails" -gt 0 ]]; then
    echo "check-stage-entry.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-stage-entry.test.sh: clean (assertion B queue-empty + drain-exempt model + observed-by refusal branch + assertion C cross-component align/waiver + templates-stub exclusion + path-shaped amendment glob + mirror fold + assertion D inferred-claim residue over amendments and active queue entries + assertion E marker grammar at every cursor, 27 scenarios)"
exit 0
