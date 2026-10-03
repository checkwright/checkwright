#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §Layout and configuration — an UNPAIRED template: no same-named file under the gates dir, so it was never vendored out and has no copy to be in parity with. Running a template in place is a legitimate adoption mode, so the gate skips it silently rather than failing closed.
set -uo pipefail
DELEGATION_KIT_POLL_SECS="${DELEGATION_KIT_POLL_SECS:-300}"
poller_emit() { printf '%s\n' "$DELEGATION_KIT_POLL_SECS"; }
poller_emit
