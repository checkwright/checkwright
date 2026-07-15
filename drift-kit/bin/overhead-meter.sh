#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §The overhead meter — the byte-proxy governance/task classifier over a session transcript
# usage: overhead-meter.sh [transcript.jsonl]   (bare: newest transcript under DRIFT_KIT_SESSIONS_DIR)
#   advisory by construction: exit is always 0, never joins gates.list; writes counts only, never transcript content
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

: "${DRIFT_KIT_TMP_DIR:=${GATE_SDK_TMP_DIR:-.tmp}}"
: "${DRIFT_KIT_METRIC_DIR:=.metric}"
: "${DRIFT_KIT_OVERHEAD_LOG:=$DRIFT_KIT_METRIC_DIR/overhead-log.txt}"

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

transcript="${1:-}"
if [[ -z "$transcript" ]]; then
    dir="$(sessions_dir)"
    if [[ -d "$dir" ]]; then
        newest=""
        shopt -s nullglob
        for f in "$dir"/*.jsonl; do
            [[ -z "$newest" || "$f" -nt "$newest" ]] && newest="$f"
        done
        shopt -u nullglob
        transcript="$newest"
    fi
fi

if [[ -z "$transcript" || ! -f "$transcript" ]]; then
    echo "overhead-meter: no transcript to measure${transcript:+: $transcript}" >&2
    echo "  help: pass a transcript path, or set DRIFT_KIT_SESSIONS_DIR to the agent transcript dir." >&2
    exit 0
fi

# spec: drift-kit/SPEC.md §The overhead meter — the fixed marker table: kit-name and
# gate-output shapes only (mechanism, never a private vocabulary; the seam holds).
# LC_ALL=C makes length() count bytes, honouring the byte-proxy contract.
read -r total gate hook stage govdoc < <(
    LC_ALL=C awk '
        { b = length($0); total += b }
        /PASS: check-|FAIL: check-|===== check|: clean \(|run-gate/                      { gate   += b; next }
        /<system-reminder>|PreToolUse|PostToolUse|SessionStart|bash-guard|hook error/    { hook   += b; next }
        /lifecycle-kit\/templates\/skills|enter-stage|WORKFLOW-STATE|Execute the template at/ { stage += b; next }
        /SPEC\.md|SPEC-|CLAUDE\.md|DOCTRINE\.md|BRIEF\.local/                            { govdoc += b; next }
        END { printf "%d %d %d %d %d\n", total+0, gate+0, hook+0, stage+0, govdoc+0 }
    ' "$transcript"
)

gov=$(( gate + hook + stage + govdoc ))
task=$(( total - gov ))
if (( total > 0 )); then
    pct=$(( (gov * 100 + total / 2) / total ))
    tpct=$(( (task * 100 + total / 2) / total ))
else
    pct=0; tpct=0
fi

base="${transcript##*/}"
base="${base%.jsonl}"
session8="${base:0:8}"
today="$(date +%F)"

mkdir -p "$(dirname "$DRIFT_KIT_OVERHEAD_LOG")" 2>/dev/null || true
line="$today $session8 total=$total gov=$gov gate=$gate pct=$pct"
if [[ -f "$DRIFT_KIT_OVERHEAD_LOG" ]]; then
    tmp="$(mktemp "${TMPDIR:-/tmp}/overhead-log.XXXXXX")"
    grep -Fv " $session8 total=" "$DRIFT_KIT_OVERHEAD_LOG" > "$tmp" 2>/dev/null || true
    mv "$tmp" "$DRIFT_KIT_OVERHEAD_LOG"
fi
printf '%s\n' "$line" >> "$DRIFT_KIT_OVERHEAD_LOG"

printf 'overhead-meter: %s %s\n' "$today" "$session8"
printf '  total=%d bytes\n' "$total"
printf '  governance=%d (%d%%)  [gate=%d hook=%d stage=%d govdoc=%d]\n' "$gov" "$pct" "$gate" "$hook" "$stage" "$govdoc"
printf '  task=%d (%d%%)\n' "$task" "$tpct"
printf '  (byte-proxy at line granularity — a proportion across same-shape sessions, not tokens; drift-kit/SPEC.md §The overhead meter)\n'
printf '  logged: %s\n' "$DRIFT_KIT_OVERHEAD_LOG"
exit 0
