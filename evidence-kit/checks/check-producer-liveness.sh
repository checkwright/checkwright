#!/usr/bin/env bash
# graph: couples=.tmp/run-validate.lock,.tmp/*.run dir=one valve=none tier=align-only
# install: never
# spec: evidence-kit/SPEC.md §check-producer-liveness — a stage entry is refused while a producer named by the record, or by any '*.run' record in the directory, is still alive
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/evidence.sh
source "$KIT/lib/evidence.sh"

LOCK="${1:-$EVIDENCE_KIT_LOCK_FILE}"

# spec: evidence-kit/SPEC.md §check-producer-liveness — set mode: a directory argument quantifies the per-record verdict over its '*.run' launch records; exit 2 wins over red wins over green, so one corrupt record is never averaged away by clean ones
if [[ -d "$LOCK" ]]; then
    shopt -s nullglob
    records=("$LOCK"/*.run)
    shopt -u nullglob

    corrupt=()
    blocking=()
    for rec in ${records[@]+"${records[@]}"}; do
        holder="$(ek_lock_read "$rec")"; hs=$?
        if [[ "$hs" -eq 2 ]]; then
            corrupt+=("$rec")
            continue
        fi
        [[ "$hs" -eq 0 ]] || continue
        pid="${holder%% *}"
        runkey="${holder#* }"
        ek_pid_alive "$pid" \
            && blocking+=("check-producer-liveness: $rec: the producer for run key '$runkey' is still running (pid $pid)")
    done

    if [[ "${#corrupt[@]}" -gt 0 ]]; then
        printf "check-producer-liveness: %s carries no readable 'pid=<n> run=<key>' record\n" "${corrupt[@]}" >&2
        exit 2
    fi

    if [[ "${#blocking[@]}" -gt 0 ]]; then
        printf '%s\n' "${blocking[@]}"
        echo "  help: wait for each run named above on its own artifact — it is still writing, so anything read now can change underneath you; where its pid is gone the record is a statement of fact that has become false, and deleting that .run file retracts it"
        exit 1
    fi

    if [[ "${#records[@]}" -eq 0 ]]; then
        echo "PRODUCER-LIVENESS: clean (no '*.run' record under $LOCK — nothing in flight)"
    else
        echo "PRODUCER-LIVENESS: clean (${#records[@]} '*.run' record(s) under $LOCK, none naming a live pid — no producer in flight)"
    fi
    exit 0
fi

holder="$(ek_lock_read "$LOCK")"; hs=$?

if [[ "$hs" -eq 2 ]]; then
    echo "check-producer-liveness: $LOCK carries no readable 'pid=<n> run=<key>' record" >&2   # exit 2: the lock is published whole or not at all, so an unparseable one is corruption, never a free reading
    exit 2
fi

if [[ "$hs" -ne 0 ]]; then
    echo "PRODUCER-LIVENESS: clean (no producer lock at $LOCK — nothing in flight)"
    exit 0
fi

pid="${holder%% *}"
runkey="${holder#* }"

if ek_pid_alive "$pid"; then
    echo "check-producer-liveness: $LOCK: the evidence producer for run key '$runkey' is still running (pid $pid)"
    echo "  help: wait for that run-validate to finish — it is still writing the evidence manifest, so anything read now can change underneath you; if pid $pid is gone, the lock is stale and deleting $LOCK clears it"
    exit 1
fi

echo "PRODUCER-LIVENESS: clean (lock at $LOCK names dead pid $pid for run key '$runkey' — no producer in flight)"
exit 0
