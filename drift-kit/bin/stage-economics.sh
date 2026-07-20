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

printf 'stage-economics: %s\n' "$today"
[[ "$PRICE_TABLE_PRESENT" -eq 1 ]] || printf '  no price table (%s) — token-only, cost=n/a (degraded, not failed)\n' "$DRIFT_KIT_PRICE_TABLE"

seen_triples=""
rows=0
incomplete=0
stamps=0
unmatched=0
while read -r iter stage session8 _date _rest; do
    [[ -z "$iter" || "$iter" == \#* || "$iter" == "---" ]] && continue
    [[ -n "$stage" && -n "$session8" ]] || continue
    case " $seen_triples " in *" $iter/$stage/$session8 "*) continue ;; esac   # a re-stamp of one session is one stage
    seen_triples="$seen_triples $iter/$stage/$session8"
    stamps=$((stamps + 1))

    transcript="$(find_transcript "$session8")"
    # spec: drift-kit/SPEC.md §The stage-economics meter — unbounded history makes a
    # per-stamp skip notice unbounded output, so unmatched stamps are counted, not listed.
    if [[ -z "$transcript" ]]; then
        unmatched=$((unmatched + 1))
        continue
    fi

    usage="$(usage_by_model "$transcript")"; st=$?
    if [[ "$st" -eq 3 ]]; then
        echo "  jq not found — cannot parse transcript usage (degraded, not failed)" >&2
        break
    fi
    [[ -n "$usage" ]] || { printf '  %s %s %s: no assistant-turn usage found (skipped)\n' "$iter" "$stage" "$session8"; continue; }

    while IFS=$'\t' read -r model in out cr cw; do
        [[ -n "$model" ]] || continue
        cost="$(price_cell "$model" "$in" "$out" "$cr" "$cw")"
        [[ "$cost" == "n/a" ]] && incomplete=1
        printf '  %s %s %s [%s]: in=%s out=%s cr=%s cw=%s cost=%s\n' \
            "$iter" "$stage" "$session8" "$model" "$in" "$out" "$cr" "$cw" "$cost"
        log_line "$iter" "$stage" "$model" \
            "$today $iter $stage $model in=$in out=$out cr=$cr cw=$cw cost=$cost"
        rows=$((rows + 1))
    done <<< "$usage"
done < <(collect_stamps)

if [[ "$stamps" -eq 0 ]]; then
    echo "stage-economics: no stamps in either source (committed history or $DRIFT_KIT_STATE_FILE) — nothing to read" >&2
    echo "  help: set DRIFT_KIT_STATE_FILE to the WORKFLOW-STATE path carrying the stage stamps." >&2
    exit 0
fi

[[ "$unmatched" -eq 0 ]] || \
    printf '  %d stamp(s) had no matching transcript (skipped — session transcripts age out of the sessions dir)\n' "$unmatched"

if [[ "$incomplete" -eq 1 ]]; then
    printf '  total pricing incomplete — one or more model cost cells degraded to n/a (unpriced model or absent table)\n'
fi
printf '  (cr=cache-read is the headline burn lever; per-session usage = per-stage usage under the stage session boundary — a span-multiple session under an iteration-boundary consumer is attributed to its stamp'\''s stage)\n'
printf '  logged: %s (%d row(s))\n' "$DRIFT_KIT_STAGE_ECONOMICS_LOG" "$rows"
exit 0
