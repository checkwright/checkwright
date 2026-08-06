#!/usr/bin/env bash
# spec: evidence-kit/SPEC.md §bin/run-validate.sh — the codified validate spine: optional pre-hook, run each suite foreground, parse, diff the baseline slice per-scenario, batch the evidence lines and fold them into the manifest once the whole roster has run. Never edits the baseline, never retries, surfaces a non-zero suite verbatim.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/evidence.sh
source "$KIT/lib/evidence.sh"

if [[ ${#EVIDENCE_KIT_SUITES[@]} -eq 0 ]]; then
    echo "run-validate: no suites configured (EVIDENCE_KIT_SUITES) — nothing to run" >&2
    exit 2
fi

if ! key="$(ek_run_key)"; then
    echo "run-validate: no evidence-line key — name the iteration in $EVIDENCE_KIT_QUEUE_FILE or set EVIDENCE_KIT_RUN_ID" >&2
    exit 2
fi

manifest="$EVIDENCE_KIT_MANIFEST_FILE"
[[ -f "$manifest" ]] || {
    echo "run-validate: manifest not found: $manifest (seed it with a '# contract: $EVIDENCE_MANIFEST_CONTRACT' header)" >&2
    exit 2
}

mkdir -p "$EVIDENCE_KIT_TMP_DIR"
today="$(date +%F)"
overall=0

# spec: evidence-kit/SPEC.md §The producer-liveness lock — release only if the lock is still ours: an out-of-band removal lets a second producer claim the freed slot, and an unconditional rm would delete that live holder's record
lock_release() {
    local holder
    holder="$(ek_lock_read "$EVIDENCE_KIT_LOCK_FILE")" || return 0
    [[ "${holder%% *}" == "$$" ]] && rm -f "$EVIDENCE_KIT_LOCK_FILE"
    return 0
}

lock_refuse() {
    rm -f "$lock_tmp"
    echo "run-validate: $1" >&2
    exit 2
}

# spec: evidence-kit/SPEC.md §The producer-liveness lock — the claim sits after the guards and the scratch mkdir (a run that refuses to start must not claim) and before any evidence work; the record is built whole in a temp file and `ln`ed into place, so the claim is create-exclusive and the lock is never half-written
lock_tmp="$(mktemp "$EVIDENCE_KIT_TMP_DIR/run-validate-lock.XXXXXX")" || exit 2
printf 'pid=%s run=%s\n' "$$" "$key" >"$lock_tmp"
reclaimed=0
while ! ln "$lock_tmp" "$EVIDENCE_KIT_LOCK_FILE" 2>/dev/null; do
    holder="$(ek_lock_read "$EVIDENCE_KIT_LOCK_FILE")"; hs=$?
    if [[ "$hs" -eq 2 ]]; then
        lock_refuse "the lock $EVIDENCE_KIT_LOCK_FILE carries no readable 'pid=<n> run=<key>' record — refusing to start; delete it if no producer is running"
    fi
    if [[ "$hs" -eq 0 ]] && ek_pid_alive "${holder%% *}"; then
        lock_refuse "a producer is already running for run key '${holder#* }' (pid ${holder%% *}) — refusing to start; wait for that run to finish, or delete $EVIDENCE_KIT_LOCK_FILE once pid ${holder%% *} is gone"
    fi
    # spec: evidence-kit/SPEC.md §The producer-liveness lock — a dead or vanished holder is reclaimed exactly once; a second failed claim means the slot is not ours to take, so refusing beats looping
    [[ "$reclaimed" -eq 0 ]] \
        || lock_refuse "could not claim $EVIDENCE_KIT_LOCK_FILE after reclaiming a stale lock — refusing to start rather than retrying; another producer won the reclaim race, or that path is not writable"
    reclaimed=1
    rm -f "$EVIDENCE_KIT_LOCK_FILE"
done
rm -f "$lock_tmp"
trap lock_release EXIT

# spec: evidence-kit/SPEC.md §Evidence manifest — the run accumulates its rows here and touches the tracked manifest only after the last suite, so no suite runs against a tree the spine has already written to
batch="$(mktemp "$EVIDENCE_KIT_TMP_DIR/validate-evidence-batch.XXXXXX")" || exit 2

for suite in "${EVIDENCE_KIT_SUITES[@]}"; do
    cmd="$(ek_suite_cmd "$suite")"
    if [[ -z "$cmd" ]]; then
        echo "run-validate: suite '$suite' has no EVIDENCE_KIT_RUN_$suite command configured" >&2
        exit 2
    fi

    if [[ -n "$EVIDENCE_KIT_PRE_HOOK" ]]; then
        # shellcheck disable=SC2086  # a multi-word consumer pre-hook word-splits by design
        if ! $EVIDENCE_KIT_PRE_HOOK "$suite"; then
            echo "run-validate: pre-hook failed for suite '$suite' — aborting (no evidence appended)" >&2
            exit 2
        fi
    fi

    log="$EVIDENCE_KIT_TMP_DIR/validate-$suite.log"
    # shellcheck disable=SC2086  # the suite command word-splits by design
    $cmd >"$log" 2>&1
    status=$?
    [[ "$status" -ne 0 ]] && echo "run-validate: suite '$suite' exited $status (log: $log)" >&2

    parsed="$EVIDENCE_KIT_TMP_DIR/validate-$suite.parsed"
    ek_parse "$suite" "$log" "$status" >"$parsed"
    if [[ ! -s "$parsed" ]]; then
        echo "run-validate: parser '$(ek_parser_for "$suite")' produced no result for suite '$suite' (log: $log) — a run failure, not an empty diff" >&2
        exit 1
    fi

    npass=$(grep -c ' pass$' "$parsed" || true)
    nfail=$(grep -c ' fail$' "$parsed" || true)
    nignore=$(grep -c ' ignore$' "$parsed" || true)

    if ek_diff "$EVIDENCE_KIT_BASELINE_FILE" "$suite" "$parsed" "$EVIDENCE_KIT_SKIP_FILE" >/dev/null; then
        verdict=clean
    else
        verdict=new-failures
        overall=1
    fi

    hash="$(sha256sum "$log" | awk '{print $1}')"; hs=$?
    fail_closed "$hs" run-validate sha256sum
    printf '%s\n' "$key $suite sha256=$hash pass=$npass fail=$nfail ignore=$nignore verdict=$verdict $today" >>"$batch"

    echo "run-validate: $suite -> $verdict (pass=$npass fail=$nfail ignore=$nignore)"
done

# spec: evidence-kit/SPEC.md §Evidence manifest — one fold, after the last suite: this iteration's prior line for every suite the run covered is superseded and the batch re-appended in roster order, so a repeated run leaves the line order unchanged
tmpm="$EVIDENCE_KIT_TMP_DIR/validate-evidence.$$"
awk -v k="$key" -v bf="$batch" '
    FILENAME == bf { superseded[$2] = 1; next }
    !($1 == k && ($2 in superseded))
' "$batch" "$manifest" >"$tmpm"; as=$?
fail_closed "$as" run-validate awk
cat "$batch" >>"$tmpm"
mv "$tmpm" "$manifest"
rm -f "$batch"

exit "$overall"
