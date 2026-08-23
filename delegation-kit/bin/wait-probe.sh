#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §bin/wait-probe — the instrument, its trial grammar, its closed cause list and its honest limit
# usage: wait-probe.sh <subcommand> [args]   ('-h' or '--help' prints the subcommand roster; exit 2 on misuse)
set -euo pipefail

SCRATCH="${GATE_SDK_TMP_DIR:-.tmp}"
WORK="$SCRATCH/wait-probe"
EVIDENCE="${GATE_SDK_WORKFLOW_DIR:-.workflow}/wait-primitive-evidence.txt"
SWEEP_MS=(10000 100000 200000)

usage() {
    cat <<'EOF'
usage: wait-probe.sh <subcommand> [args]

  produce <key> <duration_ms>   stand a producer up: sleep <duration_ms>, then write the marker.
                                Records its pid at launch in <scratch>/<key>.run.
  waiter <key> <form>           the wait body itself: until <marker>; do sleep 1; done.
                                Identical across every form; whoever arms it names the form.
  arm-local <key>               arm `waiter` as a detached shell child (the harness-uninvolved control).
  record <key> <form> <ms>      append one trial line for an armed-and-finished waiter.
  report                        classify the recorded trials and print the verdict.
  sweep                         the self-contained local run: produce + arm-local + record over the
                                declared duration sweep. This is the reproducer a second machine runs.

Scratch resolves through GATE_SDK_TMP_DIR; the evidence file through GATE_SDK_WORKFLOW_DIR.
EOF
}

now_ms() {
    local t
    t="$(date +%s%3N)"
    case "$t" in
        *[!0-9]*) t="$(date +%s)000" ;;
    esac
    printf '%s\n' "$t"
}

# spec: delegation-kit/SPEC.md §bin/wait-probe — the producer's own pid is recorded at launch and never logged in a trial line
cmd_produce() {
    local key="$1" ms="$2" pid
    mkdir -p "$WORK"
    rm -f "$WORK/$key.marker" "$WORK/$key.t0" "$SCRATCH/$key.run"
    now_ms > "$WORK/$key.t0"
    nohup bash -c 'sleep "$(( $2 / 1000 ))"; date +%s%3N > "$1"' _ "$WORK/$key.marker" "$ms" \
        >/dev/null 2>&1 &
    pid=$!
    printf 'pid=%s run=%s\n' "$pid" "$key" > "$SCRATCH/$key.run"
    printf 'wait-probe: producer %s pid=%s duration_ms=%s marker=%s\n' "$key" "$pid" "$ms" "$WORK/$key.marker"
}

# spec: delegation-kit/SPEC.md §bin/wait-probe — one wait body for every form, so no form's result rests on a differently-shaped trial
cmd_waiter() {
    local key="$1" form="$2" pred="${3:-marker}"
    local marker="$WORK/$key.marker" hb="$WORK/$key.$form.hb"
    local t0 ppid_rec alive
    WAITER_ST="$WORK/$key.$form.st"
    t0="$(cat "$WORK/$key.t0")"
    ppid_rec="$(sed -n 's/^pid=\([0-9]*\) .*/\1/p' "$SCRATCH/$key.run")"
    printf '%s\n' "$pred" > "$WORK/$key.$form.pred"
    : > "$hb"
    rm -f "$WAITER_ST"
    trap 'printf "%s\n" "$?" > "$WAITER_ST"' EXIT
    trap 'exit 143' TERM
    trap 'exit 130' INT
    trap 'exit 129' HUP
    if [ "$pred" = liveness ]; then
        until kill -0 "$ppid_rec" 2>/dev/null; do
            waiter_beat "$t0" "$ppid_rec" "$hb"
            sleep 1
        done
    else
        until [ -f "$marker" ]; do
            waiter_beat "$t0" "$ppid_rec" "$hb"
            sleep 1
        done
    fi
    waiter_beat "$t0" "$ppid_rec" "$hb"
    printf 'wait-probe: condition true for %s (%s/%s)\n' "$key" "$form" "$pred"
}

waiter_beat() {
    local t0="$1" ppid_rec="$2" hb="$3" alive=0
    kill -0 "$ppid_rec" 2>/dev/null && alive=1
    printf '%s %s\n' "$(( $(now_ms) - t0 ))" "$alive" > "$hb"
}

cmd_arm_local() {
    local key="$1" pred="${2:-marker}" pid
    nohup bash "$0" waiter "$key" local "$pred" > "$WORK/$key.local.out" 2>&1 &
    pid=$!
    printf 'pid=%s run=%s-local\n' "$pid" "$key" > "$SCRATCH/$key-local.run"
    printf 'wait-probe: local waiter for %s pid=%s\n' "$key" "$pid"
}

cmd_record() {
    local key="$1" form="$2" ms="$3"
    local t0 marker="$WORK/$key.marker" hb="$WORK/$key.$form.hb" st="$WORK/$key.$form.st"
    local marker_at="-" waiter_at="-" waiter_exit="killed" alive="-" class pred="marker"
    [ -s "$WORK/$key.$form.pred" ] && pred="$(cat "$WORK/$key.$form.pred")"
    t0="$(cat "$WORK/$key.t0")"
    [ -f "$marker" ] && marker_at="$(( $(cat "$marker") - t0 ))"
    if [ -s "$hb" ]; then
        waiter_at="$(cut -d' ' -f1 < "$hb")"
        alive="$(cut -d' ' -f2 < "$hb")"
    fi
    [ -s "$st" ] && waiter_exit="$(cat "$st")"
    class="$(classify "$marker_at" "$waiter_at" "$waiter_exit" "$alive")"
    mkdir -p "$(dirname "$EVIDENCE")"
    printf 'form=%s predicate=%s producer_ms=%s waiter_exit=%s marker_at_ms=%s waiter_at_ms=%s producer_alive_at_exit=%s class=%s\n' \
        "$form" "$pred" "$ms" "$waiter_exit" "$marker_at" "$waiter_at" "$alive" "$class" >> "$EVIDENCE"
    tail -n 1 "$EVIDENCE"
}

# spec: delegation-kit/SPEC.md §bin/wait-probe — the closed cause list, 'unexplained' included, and (ii) is cross-trial so it is named by `report` and never by a line
classify() {
    local marker_at="$1" waiter_at="$2" waiter_exit="$3" alive="$4"
    if [ "$marker_at" != "-" ] && [ "$waiter_at" != "-" ] && [ "$waiter_at" -ge "$marker_at" ]; then
        printf 'ok\n'; return
    fi
    if [ "$waiter_exit" = "0" ] && [ "$marker_at" = "-" ]; then
        printf 'predicate\n'; return          # (iii) clean zero, condition never expressible
    fi
    if [ "$alive" = "1" ] && [ "$waiter_exit" != "0" ]; then
        printf 'reaped\n'; return             # (i) died with the producer verifiably alive
    fi
    printf 'unexplained\n'                    # (iv)
}

# spec: delegation-kit/SPEC.md §bin/wait-probe — cause (ii)'s tell is a threshold across the sweep, so the report is the only reader that can see it
cmd_report() {
    [ -s "$EVIDENCE" ] || { echo "wait-probe: no trials recorded in $EVIDENCE" >&2; return 1; }
    printf '=== trials ===\n'
    cat "$EVIDENCE"
    printf '\n=== verdict ===\n'
    awk '
      { split($0, f, " "); for (i in f) { split(f[i], kv, "="); v[kv[1]] = kv[2] }
        fo = v["form"]; pr = v["predicate"]; ms = v["producer_ms"] + 0; cls = v["class"]
        cell = fo "/" pr
        n[cell]++; forms[fo]; preds[pr]
        if (cls == "ok") { okc[cell]++; if (ms > okmax[cell]) okmax[cell] = ms; okform[fo]++; okpred[pr]++ }
        else { bad[cell]++; if (badmin[cell] == 0 || ms < badmin[cell]) badmin[cell] = ms
               badform[fo]++; badpred[pr]++; badcls[cls]++ } }
      END {
        for (c in n) printf "%-18s %d trial(s), %d ok, %d early; longest clean wait=%dms\n",
              c ":", n[c], okc[c], bad[c], okmax[c]
        for (c in n) if (bad[c] && okc[c] && badmin[c] > okmax[c])
              printf "  ceiling tell HOLDS for %s: every early exit outlasts every clean wait (threshold between %dms and %dms) -> cause (ii)\n", c, okmax[c], badmin[c]
        for (p in badpred) { np++; lastp = p }
        for (fo in badform) nff++
        for (fo in okform) nof++
        if (np == 1 && nff > 1)
          printf "\nEvery early exit carries predicate=%s, across %d of the %d forms measured, and no form is early on any other predicate.\nThe early exits are predicate-shaped, not form-shaped: the wait form is exonerated and the condition is the cause (iii).\n", lastp, nff, nof
        else if (np == 0) printf "\nNo early exit recorded.\n"
        else printf "\nEarly exits span %d predicate(s) and %d form(s) -- read the class column; no single-cause reading is licensed.\n", np, nff }
    ' "$EVIDENCE"
}

cmd_sweep() {
    local ms key
    mkdir -p "$WORK"
    for ms in "${SWEEP_MS[@]}"; do
        key="sweep$ms"
        cmd_produce "$key" "$ms"
        cmd_arm_local "$key"
    done
    for ms in "${SWEEP_MS[@]}"; do
        key="sweep$ms"
        while kill -0 "$(sed -n 's/^pid=\([0-9]*\) .*/\1/p' "$SCRATCH/$key-local.run")" 2>/dev/null; do sleep 1; done
        cmd_record "$key" local "$ms"
        rm -f "$SCRATCH/$key.run" "$SCRATCH/$key-local.run"
    done
    cmd_report
}

case "${1:-}" in
    produce)   shift; [ $# -eq 2 ] || { usage >&2; exit 2; }; cmd_produce "$@" ;;
    waiter)    shift; [ $# -ge 2 ] && [ $# -le 3 ] || { usage >&2; exit 2; }; cmd_waiter "$@" ;;
    arm-local) shift; [ $# -ge 1 ] && [ $# -le 2 ] || { usage >&2; exit 2; }; cmd_arm_local "$@" ;;
    record)    shift; [ $# -eq 3 ] || { usage >&2; exit 2; }; cmd_record "$@" ;;
    report)    shift; [ $# -eq 0 ] || { usage >&2; exit 2; }; cmd_report ;;
    sweep)     shift; [ $# -eq 0 ] || { usage >&2; exit 2; }; cmd_sweep ;;
    -h|--help) usage ;;
    *)         usage >&2; exit 2 ;;
esac
