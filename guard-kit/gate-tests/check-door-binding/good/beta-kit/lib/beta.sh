#!/usr/bin/env bash
# A shipped library reaches the binary through the accessor, never through a
# path of its own: gate_native_bin_spelled owns the spelling.
set -uo pipefail
door="$(gate_native_bin_spelled)"
"$door" --emit knob-values ALPHA_KIT_X
