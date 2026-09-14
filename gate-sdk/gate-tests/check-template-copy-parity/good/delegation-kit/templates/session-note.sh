#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §Layout and configuration — a PAIRED template carrying no knob-with-default idiom: its knob class is empty, and the pair is compared like any other.
set -uo pipefail
note_emit() { printf '%s\n' "$1"; }
guard_log "session note"
note_emit "$*"
