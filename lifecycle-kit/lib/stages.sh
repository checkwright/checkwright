# shellcheck shell=bash
# spec: lifecycle-kit/SPEC.md §lib/stages.sh — the stage machine as config: platform defaults, consumer overrides
# no-port: gate-sdk/SPEC.md §The kit-library port disposition — the class ruling of 2026-08-30, reached by ground rather than by scope. This library is the config bridge's sole resolver for the LIFECYCLE_KIT_* knobs: gate-sdk/SPEC.md §lib/gate.sh rules exactly one place a knob's value is computed, and the bridge computes it by sourcing this file, so a crate-side resolver would be the second producer criterion 6 refuses. LIFECYCLE_KIT_PREDECESSOR is the tree's live keyed-knob instance and crosses the bridge from here, which is this member's sharpest form of the ground. Structural, not a sizing judgment.
_lc_cfg="${LIFECYCLE_KIT_CONFIG_FILE:-}"
if [[ -n "$_lc_cfg" ]]; then
    [[ -f "$_lc_cfg" ]] || {
        echo "lifecycle-kit: LIFECYCLE_KIT_CONFIG_FILE not found: $_lc_cfg" >&2
        exit 2
    }
    # shellcheck disable=SC1090  # consumer-supplied config, path is config
    source "$_lc_cfg"
else
    _lc_cfg="${GATE_SDK_GATES_DIR:-scripts}/lifecycle-config.sh"
    if [[ -f "$_lc_cfg" ]]; then
        # shellcheck disable=SC1090  # consumer-supplied config, path is config
        source "$_lc_cfg"
    fi
fi
unset _lc_cfg

declare -p LIFECYCLE_KIT_STAGES &>/dev/null || LIFECYCLE_KIT_STAGES=(scope align build validate close)

if ! declare -p LIFECYCLE_KIT_PREDECESSOR &>/dev/null; then
    declare -A LIFECYCLE_KIT_PREDECESSOR=([align]=scope [build]=scope [validate]=build [close]=validate)
fi

[[ -v LIFECYCLE_KIT_FIRST_STAGE ]] || LIFECYCLE_KIT_FIRST_STAGE=scope

[[ -v LIFECYCLE_KIT_DRAIN_STAGE ]] || LIFECYCLE_KIT_DRAIN_STAGE=validate

declare -p LIFECYCLE_KIT_ACTIVE_SECTIONS &>/dev/null || LIFECYCLE_KIT_ACTIVE_SECTIONS=("New Features" "Technical Debt")

[[ -v LIFECYCLE_KIT_AUDIT_STAGE ]] || LIFECYCLE_KIT_AUDIT_STAGE=align
[[ -v LIFECYCLE_KIT_AUDIT_ENTRY_STAGE ]] || LIFECYCLE_KIT_AUDIT_ENTRY_STAGE="${LIFECYCLE_KIT_AUDIT_STAGE:+build}"

[[ -v LIFECYCLE_KIT_WAIVER_TOKEN ]] || LIFECYCLE_KIT_WAIVER_TOKEN="${LIFECYCLE_KIT_AUDIT_STAGE:+${LIFECYCLE_KIT_AUDIT_STAGE}-waived}"

[[ -v LIFECYCLE_KIT_AMENDMENT_GLOB ]] || LIFECYCLE_KIT_AMENDMENT_GLOB='SPEC-*.md'
[[ -v LIFECYCLE_KIT_ROSTER_BASENAME ]] || LIFECYCLE_KIT_ROSTER_BASENAME='SPEC.md'
declare -p LIFECYCLE_KIT_CONTRACT_TOKENS &>/dev/null || LIFECYCLE_KIT_CONTRACT_TOKENS=("SPEC.md" "proto/")

[[ -v LIFECYCLE_KIT_SKILLS_DIR ]] || LIFECYCLE_KIT_SKILLS_DIR=".claude/commands"

[[ -v LIFECYCLE_KIT_SESSION_BOUNDARY ]] || LIFECYCLE_KIT_SESSION_BOUNDARY=stage

[[ -v LIFECYCLE_KIT_AGENT_FILE ]] || LIFECYCLE_KIT_AGENT_FILE="CLAUDE.md"

[[ -v LIFECYCLE_KIT_SHIM_NGRAM ]] || LIFECYCLE_KIT_SHIM_NGRAM=9
declare -p LIFECYCLE_KIT_SHIM_DEDUP_CORPUS &>/dev/null || LIFECYCLE_KIT_SHIM_DEDUP_CORPUS=()

[[ -v LIFECYCLE_KIT_QUEUE_FILE ]] || LIFECYCLE_KIT_QUEUE_FILE="${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}"
[[ -v LIFECYCLE_KIT_STATE_FILE ]] || LIFECYCLE_KIT_STATE_FILE="${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt"
[[ -v LIFECYCLE_KIT_LESSON_EVIDENCE_FILE ]] || LIFECYCLE_KIT_LESSON_EVIDENCE_FILE="${GATE_SDK_WORKFLOW_DIR:-.workflow}/lesson-evidence.txt"
[[ -v LIFECYCLE_KIT_GAP_INBOX_FILE ]] || LIFECYCLE_KIT_GAP_INBOX_FILE="${GATE_SDK_WORKFLOW_DIR:-.workflow}/gap-inbox.md"
[[ -v LIFECYCLE_KIT_SURVEY_RECORD_FILE ]] || LIFECYCLE_KIT_SURVEY_RECORD_FILE="${GATE_SDK_WORKFLOW_DIR:-.workflow}/survey-record.md"

[[ -v LIFECYCLE_KIT_RECURRENCE_THRESHOLD ]] || LIFECYCLE_KIT_RECURRENCE_THRESHOLD=2

# spec: lifecycle-kit/SPEC.md §The state machine — the stage journal's path is derived from the stage, never invented per dispatch; the default defers to the scratch dir's own knob rather than restating its literal
[[ -v LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN ]] \
    || LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN="${GATE_SDK_TMP_DIR:-.tmp}/<stage>-journal.md"
[[ -v LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE ]] || LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE=0

# spec: lifecycle-kit/SPEC.md §The state machine — the one expansion every reader shares: a dispatcher granting the path, a stage session writing it, and the entry asserting its predecessor's must name one file or the assertion reads a file nobody wrote
lifecycle_stage_journal() {
    printf '%s\n' "${LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN//<stage>/$1}"
}

# spec: lifecycle-kit/SPEC.md §The state machine — the opening line's fixed lead, spelled once because two readers must agree on it by construction: the opener writes it and the entry assertion tells the tool's own bytes from a session's by it
LIFECYCLE_STAGE_JOURNAL_MARK='# stage-journal '

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the opener: append, never overwrite, so a stage running several sessions accumulates a heading per session instead of losing its predecessors' work. The heading carries the stamp's own five fields, so an unwritten journal names who owed it and when. Prints the path it wrote, which is the entering session's only source for it.
lifecycle_stage_journal_open() { # <stage> <iteration> <session-id> <date> <head>
    local p dir
    p="$(lifecycle_stage_journal "$1")"
    printf '%s\n' "$p"
    dir="$(dirname "$p")"
    [[ -d "$dir" ]] || mkdir -p "$dir" || return 1
    [[ -s "$p" ]] && printf '\n' >> "$p"
    printf '%s%s — %s %s %s %s\n' "$LIFECYCLE_STAGE_JOURNAL_MARK" "$1" "$2" "$3" "$4" "$5" >> "$p"
}

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the predicate the entry assertion reads. Non-emptiness went vacuous the moment the opener started writing the file, so "the owing session wrote something" is a line that is neither blank nor an opener heading — the same assertion as before, restated against a file the tool itself creates.
lifecycle_stage_journal_written() { # <path>
    [[ -s "$1" ]] || return 1
    grep -qv -e '^[[:space:]]*$' -e "^$LIFECYCLE_STAGE_JOURNAL_MARK" "$1"
}

declare -p LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS &>/dev/null || LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS=("*/SPEC.md")

# spec: lifecycle-kit/SPEC.md §The survey record — the surfaces held to the no-retrieval-pointer rule; the queue file alone by default because it is the one permanent surface this kit owns and where both attested firings landed, so the default is non-vacuous in every consumer and over-reaches in none
declare -p LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS &>/dev/null \
    || LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS=("$LIFECYCLE_KIT_QUEUE_FILE")

declare -p LIFECYCLE_KIT_BOUNDARY_TRUNCATE &>/dev/null || LIFECYCLE_KIT_BOUNDARY_TRUNCATE=()

declare -p LIFECYCLE_KIT_BOUNDARY_REQUIRE &>/dev/null || LIFECYCLE_KIT_BOUNDARY_REQUIRE=()

declare -p LIFECYCLE_KIT_BOUNDARY_PRESERVE &>/dev/null || LIFECYCLE_KIT_BOUNDARY_PRESERVE=()

[[ -v LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK ]] || LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK=1

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the lock-reason pattern is consumer vocabulary, so the kit default is empty and an unconfigured consumer classifies nothing; a kit literal spelling one harness's lock reason would publish it, the same seam the residue-directory omission one knob up already takes
[[ -v LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE ]] || LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE=''

declare -p LIFECYCLE_KIT_ENTRY_PREFLIGHT &>/dev/null || LIFECYCLE_KIT_ENTRY_PREFLIGHT=()

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the one-shot pre-flight valve's ledger path; the default is empty because a ledger path is one consumer's workflow-directory layout and a kit literal spelling it would ship that layout to every adopter, the same seam the lock-reason pattern one knob up already takes. Empty reads as no valve, which is the unconditional refusal every consumer sees today. No shape arm here: the value is a path that need not exist (header-only is the ledger's resting state), and the ledger's own two fail-closed refusals are the writer's, asserted where the file is read rather than where the knob is resolved.
[[ -v LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE ]] || LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE=''

lifecycle_header() {
    grep -m1 '^## Iteration:' "$1" 2>/dev/null || true
}

# spec: lifecycle-kit/SPEC.md §lib/stages.sh — the trailing-bracket strip survives the cursor extraction as residual-field healing: a pre-upgrade header still carrying [stage:] yields the bare name, so a consumer upgrades mid-iteration without a red
lifecycle_header_iter() {
    sed -E 's/^## Iteration:[[:space:]]*//; s/[[:space:]]*\[stage:.*$//' <<<"$1"
}

# spec: lifecycle-kit/SPEC.md §lib/stages.sh — the cursor: the state file's last data line's stage token, the one derivation every lifecycle reader shares. Empty with status 0 for both no-cursor shapes (absent file, no data line yet) — "no cursor" is a legitimate state, not an error, and each caller decides what it means.
lifecycle_current_stage() {
    local s="${1:-$LIFECYCLE_KIT_STATE_FILE}" last
    [[ -f "$s" ]] || return 0
    last="$(awk '/^---[[:space:]]*$/ { f = 1; next } f && NF { l = $0 } END { print l }' "$s")"
    [[ -n "$last" ]] || return 0
    awk '{ print $2 }' <<<"$last"
}

# spec: lifecycle-kit/SPEC.md §lib/stages.sh — the closing-stage predicate: success when the cursor equals the last configured stage. Hoisted because two tools must agree by construction rather than by lookalike — the filer warned at capture that no stage is left to drain a bullet is warned by the same test that later admits it at the boundary.
lifecycle_closing_stage_reached() {
    [[ "$(lifecycle_current_stage "$@")" == "${LIFECYCLE_KIT_STAGES[-1]}" ]]
}

lifecycle_stage_known() {
    local s
    for s in "${LIFECYCLE_KIT_STAGES[@]}"; do
        [[ "$1" == "$s" ]] && return 0
    done
    return 1
}

# spec: lifecycle-kit/SPEC.md §Multi-operator semantics — the iteration-scoped supersede set: exactly the surfaces enter-stage.sh truncates at the iteration boundary (the state file, the two kit-owned built-ins, and every LIFECYCLE_KIT_BOUNDARY_TRUNCATE member). Derived here so the installer's .gitattributes block and check-merge-attrs's parity check read one set and cannot drift.
lifecycle_supersede_set() {
    printf '%s\n' "$LIFECYCLE_KIT_STATE_FILE" "$LIFECYCLE_KIT_LESSON_EVIDENCE_FILE" "$LIFECYCLE_KIT_SURVEY_RECORD_FILE"
    local m
    for m in ${LIFECYCLE_KIT_BOUNDARY_TRUNCATE[@]+"${LIFECYCLE_KIT_BOUNDARY_TRUNCATE[@]}"}; do
        printf '%s\n' "$m"
    done
}

# spec: lifecycle-kit/SPEC.md §The committed gap inbox — the union-merge set: append-only surfaces where a bullet filed on either side of a concurrent merge must survive (the gap inbox), git-native `merge=union` so no per-clone driver registration. Distinct from the keep-ours iteration-scoped supersede set above.
lifecycle_union_set() {
    printf '%s\n' "$LIFECYCLE_KIT_GAP_INBOX_FILE"
}

# spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — the shell holder of the .gitattributes merge-driver lines: one `<path> merge=iteration-scoped` per supersede member (keep-ours) then one `<path> merge=union` per union member (git-native). The `--install-lifecycle` arm emits these and check-merge-attrs verifies them, both in-crate since the 2026-09-03 port; this holder has no caller in this tree and its disposition is §lib/stages.sh's.
lifecycle_merge_attrs_block() {
    local p
    while IFS= read -r p; do
        printf '%s merge=iteration-scoped\n' "$p"
    done < <(lifecycle_supersede_set)
    while IFS= read -r p; do
        printf '%s merge=union\n' "$p"
    done < <(lifecycle_union_set)
}

# spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — the shell holder of the resident registration block, rendered from the live config; the roster is the stage set as skill invocations, never hand-listed. The `--install-lifecycle` arm and check-lifecycle-registration derive that one text in-crate since the 2026-09-03 port, so this holder has no caller in this tree either.
lifecycle_registration_block() {
    local roster="" s
    for s in "${LIFECYCLE_KIT_STAGES[@]}"; do
        roster+="\`/$s\` "
    done
    roster="${roster% }"
    cat <<EOF
The repo runs lifecycle-kit's iteration state machine on \`$LIFECYCLE_KIT_QUEUE_FILE\` — one
stage session per stage, each invoking its skill:
$roster.
The state machine, its stamp protocol, and the per-stage contracts:
[lifecycle-kit/SPEC.md](lifecycle-kit/SPEC.md).
EOF
}

_lc_errs=()
[[ ${#LIFECYCLE_KIT_STAGES[@]} -gt 0 ]] || _lc_errs+=("LIFECYCLE_KIT_STAGES is empty")
[[ -n "$LIFECYCLE_KIT_SKILLS_DIR" ]] || _lc_errs+=("LIFECYCLE_KIT_SKILLS_DIR is empty")
[[ -n "$LIFECYCLE_KIT_LESSON_EVIDENCE_FILE" ]] || _lc_errs+=("LIFECYCLE_KIT_LESSON_EVIDENCE_FILE is empty")
[[ -n "$LIFECYCLE_KIT_GAP_INBOX_FILE" ]] || _lc_errs+=("LIFECYCLE_KIT_GAP_INBOX_FILE is empty")
[[ -n "$LIFECYCLE_KIT_SURVEY_RECORD_FILE" ]] || _lc_errs+=("LIFECYCLE_KIT_SURVEY_RECORD_FILE is empty")
[[ "$LIFECYCLE_KIT_SHIM_NGRAM" =~ ^[1-9][0-9]*$ ]] \
    || _lc_errs+=("LIFECYCLE_KIT_SHIM_NGRAM '$LIFECYCLE_KIT_SHIM_NGRAM' is not a positive integer")
[[ "$LIFECYCLE_KIT_RECURRENCE_THRESHOLD" =~ ^[1-9][0-9]*$ ]] \
    || _lc_errs+=("LIFECYCLE_KIT_RECURRENCE_THRESHOLD '$LIFECYCLE_KIT_RECURRENCE_THRESHOLD' is not a positive integer")
[[ "$LIFECYCLE_KIT_SESSION_BOUNDARY" == stage || "$LIFECYCLE_KIT_SESSION_BOUNDARY" == iteration ]] \
    || _lc_errs+=("LIFECYCLE_KIT_SESSION_BOUNDARY '$LIFECYCLE_KIT_SESSION_BOUNDARY' is neither 'stage' nor 'iteration'")
lifecycle_stage_known "$LIFECYCLE_KIT_FIRST_STAGE" \
    || _lc_errs+=("LIFECYCLE_KIT_FIRST_STAGE '$LIFECYCLE_KIT_FIRST_STAGE' is not in LIFECYCLE_KIT_STAGES")
for _lc_k in "${!LIFECYCLE_KIT_PREDECESSOR[@]}"; do
    lifecycle_stage_known "$_lc_k" \
        || _lc_errs+=("LIFECYCLE_KIT_PREDECESSOR key '$_lc_k' is not in LIFECYCLE_KIT_STAGES")
    lifecycle_stage_known "${LIFECYCLE_KIT_PREDECESSOR[$_lc_k]}" \
        || _lc_errs+=("LIFECYCLE_KIT_PREDECESSOR[$_lc_k]='${LIFECYCLE_KIT_PREDECESSOR[$_lc_k]}' is not in LIFECYCLE_KIT_STAGES")
done
[[ -z "$LIFECYCLE_KIT_DRAIN_STAGE" ]] || lifecycle_stage_known "$LIFECYCLE_KIT_DRAIN_STAGE" \
    || _lc_errs+=("LIFECYCLE_KIT_DRAIN_STAGE '$LIFECYCLE_KIT_DRAIN_STAGE' is not in LIFECYCLE_KIT_STAGES")
if [[ -n "$LIFECYCLE_KIT_DRAIN_STAGE" ]]; then
    _lc_succ=0
    for _lc_k in "${!LIFECYCLE_KIT_PREDECESSOR[@]}"; do
        [[ "${LIFECYCLE_KIT_PREDECESSOR[$_lc_k]}" == "$LIFECYCLE_KIT_DRAIN_STAGE" ]] && _lc_succ=1
    done
    # spec: lifecycle-kit/SPEC.md §check-stage-entry — a terminal drain stage is fail-closed config: a [drain-exempt:] tag with no reachable successor backstop would be a permanent exemption
    [[ "$_lc_succ" == 1 ]] \
        || _lc_errs+=("LIFECYCLE_KIT_DRAIN_STAGE '$LIFECYCLE_KIT_DRAIN_STAGE' is terminal (no LIFECYCLE_KIT_PREDECESSOR entry names it) — the drain-exempt backstop would never run")
fi
[[ -z "$LIFECYCLE_KIT_AUDIT_STAGE" ]] || lifecycle_stage_known "$LIFECYCLE_KIT_AUDIT_STAGE" \
    || _lc_errs+=("LIFECYCLE_KIT_AUDIT_STAGE '$LIFECYCLE_KIT_AUDIT_STAGE' is not in LIFECYCLE_KIT_STAGES")
[[ -z "$LIFECYCLE_KIT_AUDIT_ENTRY_STAGE" ]] || lifecycle_stage_known "$LIFECYCLE_KIT_AUDIT_ENTRY_STAGE" \
    || _lc_errs+=("LIFECYCLE_KIT_AUDIT_ENTRY_STAGE '$LIFECYCLE_KIT_AUDIT_ENTRY_STAGE' is not in LIFECYCLE_KIT_STAGES")
if [[ -n "$LIFECYCLE_KIT_WAIVER_TOKEN" ]] && lifecycle_stage_known "$LIFECYCLE_KIT_WAIVER_TOKEN"; then
    _lc_errs+=("LIFECYCLE_KIT_WAIVER_TOKEN '$LIFECYCLE_KIT_WAIVER_TOKEN' collides with a stage name")
fi
for _lc_pf in ${LIFECYCLE_KIT_ENTRY_PREFLIGHT[@]+"${LIFECYCLE_KIT_ENTRY_PREFLIGHT[@]}"}; do
    if [[ "$_lc_pf" != *=* ]]; then
        _lc_errs+=("LIFECYCLE_KIT_ENTRY_PREFLIGHT entry '$_lc_pf' lacks the '<stage>=<command>' shape")
    elif ! lifecycle_stage_known "${_lc_pf%%=*}"; then
        _lc_errs+=("LIFECYCLE_KIT_ENTRY_PREFLIGHT stage key '${_lc_pf%%=*}' is not in LIFECYCLE_KIT_STAGES")
    fi
done
[[ "$LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK" == "0" || "$LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK" == "1" ]] \
    || _lc_errs+=("LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK must be 0|1 (got '$LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK')")
# spec: lifecycle-kit/SPEC.md §lib/stages.sh — a stage-journal pattern with no <stage> placeholder would name one file for every stage, so the entry assertion would read the wrong session's journal and pass on it; fail-closed rather than silently mis-asserting
[[ "$LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN" == *'<stage>'* ]] \
    || _lc_errs+=("LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN '$LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN' carries no '<stage>' placeholder")
[[ "$LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE" == "0" || "$LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE" == "1" ]] \
    || _lc_errs+=("LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE must be 0|1 (got '$LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE')")
# spec: lifecycle-kit/SPEC.md §lib/stages.sh — a malformed lock-reason pattern is a fail-closed config refusal, never a silent everything-unclassified: bash returns 2 from [[ =~ ]] on a pattern it cannot compile (0 and 1 both meaning it compiled), and a pattern that declares no group would classify every match as pid-less
if [[ -n "$LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE" ]]; then
    # spec: lifecycle-kit/SPEC.md §lib/stages.sh — the status is captured through a `||` so the probe sits in a condition context: a bare subshell whose non-match returns 1 aborts every `set -e` caller sourcing this loader, which is what a probe designed to fail routinely must never do
    _lc_re_rc=0
    ( [[ "" =~ $LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE ]] ) 2>/dev/null || _lc_re_rc=$?
    if [[ "$_lc_re_rc" -gt 1 ]]; then
        _lc_errs+=("LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE '$LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE' is not a valid POSIX ERE")
    elif [[ "$(sed -E 's/\\./X/g' <<<"$LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE")" != *'('* ]]; then
        _lc_errs+=("LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE '$LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE' declares no capture group — the group is the holder's pid")
    fi
fi
if [[ ${#_lc_errs[@]} -gt 0 ]]; then
    printf 'lifecycle-kit: malformed stage-machine config — the gates cannot run:\n' >&2
    printf '  %s\n' "${_lc_errs[@]}" >&2
    exit 2
fi
unset _lc_errs _lc_k _lc_pf _lc_succ _lc_re_rc
