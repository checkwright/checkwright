#!/usr/bin/env bash
# spec: evidence-kit/SPEC.md §check-evidence-manifest — the consumer-side front-end this repo's
# LIFECYCLE_KIT_ENTRY_PREFLIGHT reaches every gate through: an entry names a *gate*, and this
# resolves it to whichever substrate declares it. That section owns why a path cannot be named.
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/gate-sdk"
# shellcheck source=../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

[[ $# -ge 1 ]] || { echo "gate-exec: usage: gate-exec.sh <gate-name> [args...]" >&2; exit 2; }
gate="$1"; shift

mapfile -t dirs < <(gate_check_dirs)
# spec: gate-sdk/SPEC.md §lib/gate.sh — a command substitution, never a process substitution: the
# two failure signals gate_command emits are told apart only by its *status*, and an `exit 2` raised
# inside a process substitution reaches this caller as nothing but an empty argv
argv_out="$(gate_command "$gate" "${dirs[@]+"${dirs[@]}"}")"; status=$?
argv=()
[[ "$status" -eq 0 && -n "$argv_out" ]] && mapfile -t argv <<<"$argv_out"
# spec: gate-sdk/SPEC.md §Fail-closed contract — an argv the resolver would not build is exit 2,
# never a silently skipped pre-flight; status 1 is this front end's own refusal to name, and any
# other non-zero is already named on stderr and takes no second, contradicting sentence
if [[ "$status" -ne 0 ]]; then
    [[ "$status" -eq 1 ]] && echo "gate-exec: $gate resolves in none of: ${dirs[*]}" >&2
    exit 2
fi
[[ ${#argv[@]} -gt 0 ]] || { echo "gate-exec: $gate resolved to an empty argv" >&2; exit 2; }
exec "${argv[@]}" "$@"
