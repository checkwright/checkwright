#!/usr/bin/env bash
# graph: couples=.tmp/run-validate.lock dir=one valve=none tier=align-only
# install: never
# spec: evidence-kit/SPEC.md §check-producer-liveness — a stage entry is refused while the evidence producer's lock names a live PID
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/evidence.sh
source "$KIT/lib/evidence.sh"

LOCK="${1:-$EVIDENCE_KIT_LOCK_FILE}"

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
