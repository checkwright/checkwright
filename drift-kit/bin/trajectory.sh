#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §The published-evidence extractor — governed-trajectory table, pure function of committed history
# usage: trajectory.sh [--emit]   (bare: human header + table; --emit: table only, for the committed projection)
#   advisory by construction: exit is always 0, never joins gates.list; the
#   consumer freshness gate (check-trajectory-fresh) is what blocks a stale emission.
set -uo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT" 2>/dev/null || exit 0

_ds_cfg="${DRIFT_KIT_CONFIG_FILE:-}"
if [[ -n "$_ds_cfg" ]]; then
    [[ -f "$_ds_cfg" ]] || {
        echo "drift-kit: DRIFT_KIT_CONFIG_FILE not found: $_ds_cfg" >&2
        exit 2
    }
    # shellcheck source=/dev/null  # consumer config path is resolved at runtime
    source "$_ds_cfg"
else
    _ds_cfg="${GATE_SDK_GATES_DIR:-scripts}/drift-config.sh"
    if [[ -f "$_ds_cfg" ]]; then
        # shellcheck source=/dev/null  # consumer config path is resolved at runtime
        source "$_ds_cfg"
    fi
fi
unset _ds_cfg

# spec: drift-kit/SPEC.md §Layout and configuration — DRIFT_KIT_TRAJECTORY_SURFACES is "<state-file> <evidence-file>"
_wf="${GATE_SDK_WORKFLOW_DIR:-.workflow}"
: "${DRIFT_KIT_TRAJECTORY_SURFACES:=$_wf/WORKFLOW-STATE.txt $_wf/validate-evidence.txt}"
: "${DRIFT_KIT_GATES_FILE:=${GATE_SDK_GATES_DIR:-scripts}/gates.list}"
unset _wf
read -r STATE_FILE EVIDENCE_FILE _ <<<"$DRIFT_KIT_TRAJECTORY_SURFACES"

# spec: drift-kit/SPEC.md §Layout and configuration — DRIFT_KIT_STAGES is the ordered stage roster the trajectory table renders; unset falls open to the historical five so standalone/un-upgraded emission stays byte-identical (drift re-derives with its own knob, never sourcing lifecycle — the DRIFT_KIT_STATE_FILE precedent)
declare -p DRIFT_KIT_STAGES &>/dev/null || DRIFT_KIT_STAGES=(scope align build validate close)

# spec: drift-kit/SPEC.md §The published-evidence extractor — each stage's slot label is its shortest prefix unique among the roster; header legend and cells read this one map so they cannot drift, and a non-colliding roster reduces every label to its single letter (five-stage default byte-identical)
declare -A STAGE_ABBR
for _s in "${DRIFT_KIT_STAGES[@]}"; do
    _len=1
    while (( _len < ${#_s} )); do
        _pfx="${_s:0:_len}"
        _collide=0
        for _o in "${DRIFT_KIT_STAGES[@]}"; do
            [[ "$_o" == "$_s" ]] && continue
            [[ "${_o:0:_len}" == "$_pfx" ]] && { _collide=1; break; }
        done
        (( _collide )) || break
        _len=$(( _len + 1 ))
    done
    STAGE_ABBR[$_s]="${_s:0:_len}"
done
_stage_legend=""
for _s in "${DRIFT_KIT_STAGES[@]}"; do
    _stage_legend="$_stage_legend${_stage_legend:+ }${STAGE_ABBR[$_s]}"
done
unset _s _o _len _pfx _collide

# spec: drift-kit/SPEC.md §The published-evidence extractor — the case-arm membership test and the git-log grep pre-filter build one alternation from the roster, so the two roster reads cannot diverge
_stage_alt="$(IFS='|'; printf '%s' "${DRIFT_KIT_STAGES[*]}")"

emit=0
[[ "${1:-}" == "--emit" ]] && emit=1

# spec: drift-kit/SPEC.md §The published-evidence extractor — iteration+stages harvest; state file truncates at scope, history keeps every stamp
ITERS=()                 # iteration names, chronological (all, in-flight included — for range boundaries)
declare -A START_COMMIT  # iter -> first commit that added any of its stamps
declare -A CLOSE_COMMIT  # iter -> commit that added its 'close' stamp (unset if never closed)
declare -A STAGES        # iter -> space-joined stage names seen, in first-seen order

if git cat-file -e "HEAD:$STATE_FILE" 2>/dev/null; then
    while IFS= read -r line; do
        case "$line" in
            COMMIT\ *) cur_commit="${line#COMMIT }" ;;
            +*)
                stamp="${line#+}"
                read -r it st _ <<<"$stamp"
                case " ${DRIFT_KIT_STAGES[*]} " in
                    *" $st "*) ;;
                    *) continue ;;
                esac
                if [[ -z "${START_COMMIT[$it]:-}" ]]; then
                    START_COMMIT[$it]="$cur_commit"
                    ITERS+=("$it")
                    STAGES[$it]=""
                fi
                case " ${STAGES[$it]} " in
                    *" $st "*) ;;
                    *) STAGES[$it]="${STAGES[$it]:+${STAGES[$it]} }$st" ;;
                esac
                [[ "$st" == close ]] && CLOSE_COMMIT[$it]="$cur_commit"
                ;;
        esac
    done < <(git log --reverse --format='COMMIT %H' -p -U0 -- "$STATE_FILE" 2>/dev/null \
                 | grep -E "^COMMIT |^\+[a-z0-9-]+ ($_stage_alt) ")
fi

# spec: drift-kit/SPEC.md §The published-evidence extractor — every range-scoped column freezes at (close(N-1), close(N)]; no column reads HEAD, so the emission is a pure function of closed history
declare -A COMMIT_ITER FEAT DEBT   # commit -> owning closed iteration (range-attributed); per-iteration feat/debt subject split
CLOSED_ITERS=()
for it in "${ITERS[@]}"; do
    [[ -n "${CLOSE_COMMIT[$it]:-}" ]] && CLOSED_ITERS+=("$it")
done
prev_close=""
for it in "${CLOSED_ITERS[@]}"; do
    close_c="${CLOSE_COMMIT[$it]}"
    if [[ -n "$prev_close" ]]; then
        range="$prev_close..$close_c"    # (close(N-1), close(N)]
    else
        range="$close_c"                 # first row: close(0) is the empty boundary — every ancestor up to close(1)
    fi
    f=0; d=0
    while IFS=' ' read -r h subj; do
        COMMIT_ITER["$h"]="$it"
        case "$subj" in
            feat*)            f=$((f + 1)) ;;
            fix* | refactor*) d=$((d + 1)) ;;
        esac
    done < <(git log --format='%H %s' "$range" 2>/dev/null)
    FEAT[$it]="$f"; DEBT[$it]="$d"
    prev_close="$close_c"
done

# spec: drift-kit/SPEC.md §The published-evidence extractor — amendment-latency harvest, fixture/template paths excluded
declare -A ADD_DATE AMEND_COUNT AMEND_MAXLAG
while IFS= read -r line; do
    case "$line" in
        COMMIT\ *)
            rest="${line#COMMIT }"
            read -r cur_commit cur_date <<<"$rest"
            ;;
        A$'\t'*)
            path="${line#A$'\t'}"
            case "$path" in */gate-tests/*|*/templates/*) continue ;; esac
            ADD_DATE["$path"]="$cur_date"
            ;;
        D$'\t'*)
            path="${line#D$'\t'}"
            case "$path" in */gate-tests/*|*/templates/*) continue ;; esac
            add_date="${ADD_DATE[$path]:-}"
            [[ -n "$add_date" ]] || continue
            owner="${COMMIT_ITER[$cur_commit]:-}"
            [[ -n "$owner" ]] || continue
            a_epoch="$(date -d "$add_date" +%s 2>/dev/null)" || continue
            d_epoch="$(date -d "$cur_date" +%s 2>/dev/null)" || continue
            lag=$(((d_epoch - a_epoch) / 86400))
            AMEND_COUNT[$owner]=$(( ${AMEND_COUNT[$owner]:-0} + 1 ))
            [[ "$lag" -gt "${AMEND_MAXLAG[$owner]:-0}" ]] && AMEND_MAXLAG[$owner]="$lag"
            ;;
    esac
done < <(git log --reverse --format='COMMIT %H %ad' --date=short --diff-filter=AD \
             --name-status -- '*/SPEC-*.md' 'SPEC-*.md' 2>/dev/null)

# spec: drift-kit/SPEC.md §The published-evidence extractor — validate-attestation harvest (evidence manifest also truncates at scope)
declare -A VAL_SUITES VAL_FAIL VAL_PRESENT
if [[ -n "$EVIDENCE_FILE" ]] && git cat-file -e "HEAD:$EVIDENCE_FILE" 2>/dev/null; then
    while IFS= read -r stamp; do
        read -r it _suite rest <<<"$stamp"
        [[ -n "$it" ]] || continue
        VAL_PRESENT[$it]=1
        VAL_SUITES[$it]=$(( ${VAL_SUITES[$it]:-0} + 1 ))
        case "$rest" in
            *"fail=0 "*) ;;
            *"fail="*)
                fv="${rest##*fail=}"; fv="${fv%% *}"
                [[ "$fv" =~ ^[0-9]+$ ]] && VAL_FAIL[$it]=$(( ${VAL_FAIL[$it]:-0} + fv ))
                ;;
        esac
    done < <(
        for c in $(git log --format='%H' -- "$EVIDENCE_FILE" 2>/dev/null); do
            git show "$c:$EVIDENCE_FILE" 2>/dev/null
        done | grep -E '^[a-z0-9-]+ [a-z_]+ sha256=' | sort -u
    )
else
    EVIDENCE_FILE="${EVIDENCE_FILE:-(unset)}"
fi

# spec: drift-kit/SPEC.md §The published-evidence extractor — gate-roster growth (gates.list count at close commit)
declare -A GATE_COUNT
for it in "${!CLOSE_COMMIT[@]}"; do
    roster="$(git show "${CLOSE_COMMIT[$it]}:$DRIFT_KIT_GATES_FILE" 2>/dev/null)" \
        && GATE_COUNT[$it]="$(printf '%s\n' "$roster" | grep -Evc '^[[:space:]]*(#|$)')" \
        || GATE_COUNT[$it]="n/a"
done

# spec: drift-kit/SPEC.md §The published-evidence extractor — one slot per configured stage in roster order, present as its roster-unique abbreviation or absent as '·'
render_stages() {
    local seen=" $1 " out="" s
    for s in "${DRIFT_KIT_STAGES[@]}"; do
        case "$seen" in
            *" $s "*) out="$out${out:+ }${STAGE_ABBR[$s]}" ;;
            *)        out="$out${out:+ }·" ;;
        esac
    done
    printf '%s' "$out"
}

state_missing=0
git cat-file -e "HEAD:$STATE_FILE" 2>/dev/null || state_missing=1

if [[ "$emit" -eq 0 ]]; then
    echo "=== Governed trajectory (advisory — this repo's own committed history) ==="
    echo "One row per closed iteration; pure function of committed history (drift-kit/SPEC.md)."
    echo
fi

echo "| iteration | stages ($_stage_legend) | commits (feat/debt) | amendments (merged · max lag) | validate (suites) | gates.list |"
echo "| --- | --- | --- | --- | --- | --- |"

if [[ "$state_missing" -eq 1 ]]; then
    echo "| n/a (no $STATE_FILE) | · | · | · | · | · |"
    exit 0
fi

for it in "${ITERS[@]}"; do
    [[ -n "${CLOSE_COMMIT[$it]:-}" ]] || continue   # closed iterations only

    stages_cell="$(render_stages "${STAGES[$it]}")"
    commits_cell="${FEAT[$it]:-0}f/${DEBT[$it]:-0}d"

    ac="${AMEND_COUNT[$it]:-0}"
    if [[ "$ac" -eq 0 ]]; then
        amend_cell="0"
    else
        amend_cell="$ac · ≤${AMEND_MAXLAG[$it]:-0}d"
    fi

    if [[ -n "${VAL_PRESENT[$it]:-}" ]]; then
        vf="${VAL_FAIL[$it]:-0}"
        if [[ "$vf" -eq 0 ]]; then
            val_cell="${VAL_SUITES[$it]}s clean"
        else
            val_cell="${VAL_SUITES[$it]}s ${vf}✗"
        fi
    else
        val_cell="n/a (pre-evidence-kit)"
    fi

    echo "| $it | $stages_cell | $commits_cell | $amend_cell | $val_cell | ${GATE_COUNT[$it]:-n/a} |"
done
exit 0
