#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §The stage-economics meter — prices lifecycle spend by stage × model × iteration via WORKFLOW-STATE stamps ⋈ transcripts ⋈ a consumer price table
# usage: stage-economics.sh   (reads DRIFT_KIT_STATE_FILE stamps, matches transcripts under DRIFT_KIT_SESSIONS_DIR, prices via DRIFT_KIT_PRICE_TABLE)
#   advisory by construction (§The overhead meter): exit is always 0, never joins gates.list; a missing input is a 0-exit notice, writes counts/cost only, never transcript content, and emits no account identifiers
set -uo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT" 2>/dev/null || exit 0

# spec: drift-kit/SPEC.md §Layout and configuration
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

: "${DRIFT_KIT_METRIC_DIR:=.metric}"
: "${DRIFT_KIT_STAGE_ECONOMICS_LOG:=$DRIFT_KIT_METRIC_DIR/stage-economics-log.txt}"
: "${DRIFT_KIT_PRICE_TABLE:=${GATE_SDK_GATES_DIR:-scripts}/price-table.tsv}"
: "${DRIFT_KIT_STATE_FILE:=${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt}"
: "${DRIFT_KIT_SUPERVISION_LABEL:=supervision}"

sessions_dir() {
    if [[ -n "${DRIFT_KIT_SESSIONS_DIR:-}" ]]; then
        printf '%s\n' "$DRIFT_KIT_SESSIONS_DIR"
        return 0
    fi
    local home slug
    home="${CLAUDE_CONFIG_DIR:-$HOME/.claude}"
    slug="$(pwd | sed 's/[^a-zA-Z0-9]/-/g')"
    printf '%s/projects/%s\n' "$home" "$slug"
}

normalize8() {                 # lifecycle's session-id.sh normalization: strip a leading agent-, take the first 8 chars
    local id="${1#agent-}"
    printf '%s' "${id:0:8}"
}

# spec: drift-kit/SPEC.md §The stage-economics meter — a stamp's session8 selects a transcript by
# applying that same normalization to each candidate basename (never a raw agent- prefix match).
find_transcript() {            # $1=session8 -> newest matching transcript path, empty if none
    local want="$1" dir newest="" f base
    dir="$(sessions_dir)"
    [[ -d "$dir" ]] || return 0
    shopt -s nullglob
    for f in "$dir"/*.jsonl "$dir"/*/subagents/*.jsonl; do
        base="${f##*/}"; base="${base%.jsonl}"
        [[ "$(normalize8 "$base")" == "$want" ]] || continue
        [[ -z "$newest" || "$f" -nt "$newest" ]] && newest="$f"
    done
    shopt -u nullglob
    printf '%s' "$newest"
}

# spec: drift-kit/SPEC.md §The stage-economics meter — sum the matched session's assistant-turn usage into four
# token categories per model id. Streaming records repeat a message id across lines (input/cache constant, output
# growing), so keep the last usage per message id, then aggregate by model — summing raw lines would multi-count.
usage_by_model() {             # $1=transcript -> lines: model<TAB>in<TAB>out<TAB>cr<TAB>cw
    command -v jq >/dev/null 2>&1 || return 3
    jq -rc 'select(.type=="assistant" and (.message.usage != null))
            | [ (.message.id // "?"), (.message.model // "?"),
                (.message.usage.input_tokens // 0),
                (.message.usage.output_tokens // 0),
                (.message.usage.cache_read_input_tokens // 0),
                (.message.usage.cache_creation_input_tokens // 0) ] | @tsv' "$1" \
    | awk -F'\t' '
        { model[$1]=$2; in_[$1]=$3; out[$1]=$4; cr[$1]=$5; cw[$1]=$6 }
        END {
            for (i in model) { m=model[i]; IN[m]+=in_[i]; OUT[m]+=out[i]; CR[m]+=cr[i]; CW[m]+=cw[i]; seen[m]=1 }
            for (m in seen) printf "%s\t%d\t%d\t%d\t%d\n", m, IN[m], OUT[m], CR[m], CW[m]
        }'
}

# spec: drift-kit/SPEC.md §The stage-economics meter — the price table is consumer config (the provenance seam);
# absent or missing a model's row degrades that cell to n/a rather than failing.
declare -A PRICE_IN PRICE_OUT PRICE_CR PRICE_CW
PRICE_TABLE_PRESENT=0
if [[ -f "$DRIFT_KIT_PRICE_TABLE" ]]; then
    PRICE_TABLE_PRESENT=1
    while IFS=$'\t' read -r m pin pout pcr pcw _; do
        [[ -z "$m" || "$m" == \#* || "$m" == "model" ]] && continue
        PRICE_IN[$m]="$pin"; PRICE_OUT[$m]="$pout"; PRICE_CR[$m]="$pcr"; PRICE_CW[$m]="$pcw"
    done < "$DRIFT_KIT_PRICE_TABLE"
fi

price_cell() {                 # $1=model $2=in $3=out $4=cr $5=cw -> USD cost, or "n/a"
    local m="$1"
    [[ -n "${PRICE_IN[$m]:-}" ]] || { printf 'n/a'; return 0; }
    awk -v i="$2" -v o="$3" -v cr="$4" -v cw="$5" \
        -v pi="${PRICE_IN[$m]}" -v po="${PRICE_OUT[$m]}" -v pcr="${PRICE_CR[$m]}" -v pcw="${PRICE_CW[$m]}" \
        'BEGIN { printf "%.4f", i*pi + o*po + cr*pcr + cw*pcw }'
}

# spec: drift-kit/SPEC.md §The stage-economics meter — the apportionment key: integer split in
# proportion to the given counts, remainder to the first, so the parts re-sum to the whole exactly.
split_tokens() {               # $1=total $2..=counts (caller's order) -> one integer per line
    local total="$1"; shift
    local n sum=0 acc=0 part
    local -a parts=()
    for n in "$@"; do sum=$((sum + n)); done
    [[ "$sum" -gt 0 ]] || { printf '%s\n' "$total"; return 0; }
    for n in "$@"; do
        part=$((total * n / sum))
        parts+=("$part")
        acc=$((acc + part))
    done
    parts[0]=$(( parts[0] + total - acc ))
    printf '%s\n' "${parts[@]}"
}

# spec: drift-kit/SPEC.md §The stage-economics meter — history ∪ live, the same reader
# bin/trajectory.sh already ships: the boundary truncation of the live file destroys no
# economics, and the live arm keeps a stamped-but-uncommitted stage visible.
collect_stamps() {
    git log --reverse --format='%H' -p -U0 -- "$DRIFT_KIT_STATE_FILE" 2>/dev/null \
        | sed -n -E 's/^\+([a-z0-9][a-z0-9-]* [a-z][a-z0-9-]* [A-Za-z0-9]+ [0-9]{4}-[0-9]{2}-[0-9]{2}.*)$/\1/p'
    [[ -f "$DRIFT_KIT_STATE_FILE" ]] && cat "$DRIFT_KIT_STATE_FILE"
    return 0
}

[[ -f "$DRIFT_KIT_STATE_FILE" ]] || \
    echo "stage-economics: no live state file ($DRIFT_KIT_STATE_FILE) — reading committed history alone" >&2

today="$(date +%F)"
mkdir -p "$(dirname "$DRIFT_KIT_STAGE_ECONOMICS_LOG")" 2>/dev/null || true

log_line() {                   # dedup on the <iteration> <stage> <model> triple, then append
    local iter="$1" stage="$2" model="$3" line="$4" tmp
    if [[ -f "$DRIFT_KIT_STAGE_ECONOMICS_LOG" ]]; then
        tmp="$(mktemp "${TMPDIR:-/tmp}/stage-economics-log.XXXXXX")"
        grep -Fv " $iter $stage $model in=" "$DRIFT_KIT_STAGE_ECONOMICS_LOG" > "$tmp" 2>/dev/null || true
        mv "$tmp" "$DRIFT_KIT_STAGE_ECONOMICS_LOG"
    fi
    printf '%s\n' "$line" >> "$DRIFT_KIT_STAGE_ECONOMICS_LOG"
}

rows=0
incomplete=0
emit_row() {                   # $1=iteration $2=stage-or-role $3=who $4=model $5..$8=in out cr cw
    local iter="$1" stage="$2" who="$3" model="$4" in="$5" out="$6" cr="$7" cw="$8" cost
    cost="$(price_cell "$model" "$in" "$out" "$cr" "$cw")"
    [[ "$cost" == "n/a" ]] && incomplete=1
    printf '  %s %s %s [%s]: in=%s out=%s cr=%s cw=%s cost=%s\n' \
        "$iter" "$stage" "$who" "$model" "$in" "$out" "$cr" "$cw" "$cost"
    log_line "$iter" "$stage" "$model" \
        "$today $iter $stage $model in=$in out=$out cr=$cr cw=$cw cost=$cost"
    rows=$((rows + 1))
}

printf 'stage-economics: %s\n' "$today"
[[ "$PRICE_TABLE_PRESENT" -eq 1 ]] || printf '  no price table (%s) — token-only, cost=n/a (degraded, not failed)\n' "$DRIFT_KIT_PRICE_TABLE"

# spec: drift-kit/SPEC.md §The stage-economics meter — the attribution invariant: one transcript,
# one (iteration, stage). The stamp pass keys on the session, not the stamp, so a session bearing
# two stamps resolves to its last one and the yielded stamps take no row instead of a duplicate.
declare -A STAMP_SEEN=() SESS_CHOICE=() SESS_YIELDED=() ATTRIBUTED=() DISPATCH=() LEAD_SEEN=()
SESS_ORDER=()
LEAD_ORDER=()
label_collision=0
stamps=0
while read -r iter stage session8 _date _rest; do
    [[ -z "$iter" || "$iter" == \#* || "$iter" == "---" ]] && continue
    [[ -n "$stage" && -n "$session8" ]] || continue
    [[ -n "${STAMP_SEEN["$iter/$stage/$session8"]:-}" ]] && continue
    STAMP_SEEN["$iter/$stage/$session8"]=1
    stamps=$((stamps + 1))
    [[ "$stage" == "$DRIFT_KIT_SUPERVISION_LABEL" ]] && label_collision=1
    if [[ -n "${SESS_CHOICE[$session8]:-}" ]]; then
        SESS_YIELDED[$session8]="${SESS_YIELDED[$session8]:-}${SESS_CHOICE[$session8]}; "
    else
        SESS_ORDER+=("$session8")
    fi
    SESS_CHOICE[$session8]="$iter $stage"
done < <(collect_stamps)

if [[ "$stamps" -eq 0 ]]; then
    echo "stage-economics: no stamps in either source (committed history or $DRIFT_KIT_STATE_FILE) — nothing to read" >&2
    echo "  help: set DRIFT_KIT_STATE_FILE to the WORKFLOW-STATE path carrying the stage stamps." >&2
    exit 0
fi

degraded=0
unmatched=0
for session8 in ${SESS_ORDER[@]+"${SESS_ORDER[@]}"}; do
    read -r iter stage <<< "${SESS_CHOICE[$session8]}"
    transcript="$(find_transcript "$session8")"
    # spec: drift-kit/SPEC.md §The stage-economics meter — unbounded history makes a
    # per-stamp skip notice unbounded output, so unmatched stamps are counted, not listed.
    if [[ -z "$transcript" ]]; then
        unmatched=$((unmatched + 1))
        continue
    fi
    ATTRIBUTED[$transcript]=1
    # spec: drift-kit/SPEC.md §The stage-economics meter — a nested-tier transcript names its supervising lead in
    # its own path, which is what makes the supervision row derivable with no stamp and no lifecycle change.
    if [[ "$transcript" == */subagents/*.jsonl ]]; then
        lead="${transcript%/subagents/*}"; lead="${lead##*/}"
        DISPATCH["$lead $iter"]=$(( ${DISPATCH["$lead $iter"]:-0} + 1 ))
        if [[ -z "${LEAD_SEEN[$lead]:-}" ]]; then
            LEAD_SEEN[$lead]=1
            LEAD_ORDER+=("$lead")
        fi
    fi

    usage="$(usage_by_model "$transcript")"; st=$?
    if [[ "$st" -eq 3 ]]; then
        echo "  jq not found — cannot parse transcript usage (degraded, not failed)" >&2
        degraded=1
        break
    fi
    [[ -n "$usage" ]] || { printf '  %s %s %s: no assistant-turn usage found (skipped)\n' "$iter" "$stage" "$session8"; continue; }

    while IFS=$'\t' read -r model in out cr cw; do
        [[ -n "$model" ]] || continue
        emit_row "$iter" "$stage" "$session8" "$model" "$in" "$out" "$cr" "$cw"
    done <<< "$usage"
done

for session8 in ${SESS_ORDER[@]+"${SESS_ORDER[@]}"}; do
    [[ -n "${SESS_YIELDED[$session8]:-}" ]] || continue
    printf '  %s: one session, several stamps — attributed to "%s"; yielded (no row): %s\n' \
        "$session8" "${SESS_CHOICE[$session8]}" "${SESS_YIELDED[$session8]%; }"
done

# spec: drift-kit/SPEC.md §The stage-economics meter — supervision is its own row, never an apportionment across
# stages: the lead's burn carries no stamp, and folding it into stage rows would need an allocation key
# grounded in nothing measured. A lead spanning iterations splits by dispatch count, the one key that is.
if [[ "$degraded" -eq 0 && ${#LEAD_ORDER[@]} -gt 0 ]]; then
    if [[ "$label_collision" -eq 1 ]]; then
        printf '  a stamp names the stage "%s", colliding with DRIFT_KIT_SUPERVISION_LABEL — no supervision row emitted this run\n' \
            "$DRIFT_KIT_SUPERVISION_LABEL"
    else
        for lead in "${LEAD_ORDER[@]}"; do
            lead_path="$(sessions_dir)/$lead.jsonl"
            [[ -n "${ATTRIBUTED[$lead_path]:-}" ]] && continue   # already a stage row: the invariant holds, no second claim
            if [[ ! -f "$lead_path" ]]; then
                unmatched=$((unmatched + 1))
                continue
            fi
            split=()
            while IFS= read -r dline; do split+=("$dline"); done < <(
                for dkey in "${!DISPATCH[@]}"; do
                    [[ "$dkey" == "$lead "* ]] || continue
                    printf '%s\t%s\n' "${DISPATCH[$dkey]}" "${dkey#"$lead" }"
                done | sort -k1,1nr -k2,2)
            counts=(); iters=()
            for dline in "${split[@]}"; do
                counts+=("${dline%%$'\t'*}")
                iters+=("${dline#*$'\t'}")
            done
            usage="$(usage_by_model "$lead_path")"; st=$?
            if [[ "$st" -eq 3 ]]; then degraded=1; break; fi
            [[ -n "$usage" ]] || continue
            [[ "${#iters[@]}" -le 1 ]] || \
                printf '  %s supervised %d iterations — apportioned by dispatch count (%s), remainder to %s\n' \
                    "$(normalize8 "$lead")" "${#iters[@]}" "${counts[*]}" "${iters[0]}"
            while IFS=$'\t' read -r model in out cr cw; do
                [[ -n "$model" ]] || continue
                p_in=(); p_out=(); p_cr=(); p_cw=()
                while IFS= read -r v; do p_in+=("$v"); done < <(split_tokens "$in" "${counts[@]}")
                while IFS= read -r v; do p_out+=("$v"); done < <(split_tokens "$out" "${counts[@]}")
                while IFS= read -r v; do p_cr+=("$v"); done < <(split_tokens "$cr" "${counts[@]}")
                while IFS= read -r v; do p_cw+=("$v"); done < <(split_tokens "$cw" "${counts[@]}")
                for i in "${!iters[@]}"; do
                    emit_row "${iters[$i]}" "$DRIFT_KIT_SUPERVISION_LABEL" "$(normalize8 "$lead")" \
                        "$model" "${p_in[$i]}" "${p_out[$i]}" "${p_cr[$i]}" "${p_cw[$i]}"
                done
            done <<< "$usage"
            ATTRIBUTED[$lead_path]=1
        done
    fi
fi

# spec: drift-kit/SPEC.md §The stage-economics meter — the under-count bound: the unmatched counter reports stamps
# with no transcript and is structurally blind to the inverse, so the inverse is counted too. A bound,
# never an attribution — a transcript carries no iteration and no stage, so nothing could place it.
unstamped=0
if [[ "$degraded" -eq 0 ]]; then
    _se_dir="$(sessions_dir)"
    if [[ -d "$_se_dir" ]]; then
        shopt -s nullglob
        for f in "$_se_dir"/*.jsonl "$_se_dir"/*/subagents/*.jsonl; do
            [[ -n "${ATTRIBUTED[$f]:-}" ]] && continue
            unstamped=$((unstamped + 1))
        done
        shopt -u nullglob
    fi
fi

[[ "$unmatched" -eq 0 ]] || \
    printf '  %d stamp(s) had no matching transcript (skipped — session transcripts age out of the sessions dir)\n' "$unmatched"
[[ "$unstamped" -eq 0 ]] || \
    printf '  %d transcript(s) in the sessions dir match no stamp and bill to no row — an upper bound on the unstamped-continuation under-count, not an attribution\n' "$unstamped"

if [[ "$incomplete" -eq 1 ]]; then
    printf '  total pricing incomplete — one or more model cost cells degraded to n/a (unpriced model or absent table)\n'
fi
printf '  (cr=cache-read is the headline burn lever; one transcript bills to exactly one (iteration, stage) — a session bearing several stamps is attributed to its last and the yielded stamps are named above, never billed twice)\n'
printf '  logged: %s (%d row(s))\n' "$DRIFT_KIT_STAGE_ECONOMICS_LOG" "$rows"
exit 0
