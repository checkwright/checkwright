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
mapfile -t argv < <(gate_command "$gate" "${dirs[@]+"${dirs[@]}"}")
# spec: gate-sdk/SPEC.md §Fail-closed contract — an argv the resolver would not build is exit 2,
# the dispatcher's own verdict for it, never a silently skipped pre-flight
[[ ${#argv[@]} -gt 0 ]] || {
    echo "gate-exec: $gate resolves in none of: ${dirs[*]}" >&2
    exit 2
}
exec "${argv[@]}" "$@"
