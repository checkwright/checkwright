#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §Layout and configuration — the vendored copy of the knob-less template, carrying the same declared surface.
set -uo pipefail
note_emit() { printf '%s\n' "$1"; }
guard_log "session note"
note_emit "$*"
