#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §The turn-end liveness hook (template) — this repo's reader for
# DELEGATION_KIT_LIVENESS_CMD, a path run with the scratch dir as its only argument; the gate is
# name-addressed, so the adapter is consumer-side (evidence-kit/SPEC.md §check-evidence-manifest)
set -uo pipefail
exec bash "$(dirname "${BASH_SOURCE[0]}")/gate-exec.sh" check-producer-liveness "$@"
