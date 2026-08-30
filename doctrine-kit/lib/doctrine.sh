# shellcheck shell=bash
# spec: doctrine-kit/SPEC.md §lib/doctrine.sh — sourced config loader + the knob defaults, never gate structure
# no-port: gate-sdk/SPEC.md §The kit-library port disposition — the class ruling of 2026-08-30, reached by ground rather than by scope. This library is the config bridge's sole resolver for the DOCTRINE_KIT_* knobs: gate-sdk/SPEC.md §lib/gate.sh rules exactly one place a knob's value is computed, and the bridge computes it by sourcing this file, so a crate-side resolver would be the second producer criterion 6 refuses. Being among the smallest members changes nothing, since the ground is what the file resolves and not how much of it there is. Structural, not a sizing judgment.

_dk_cfg="${DOCTRINE_KIT_CONFIG_FILE:-}"
if [[ -n "$_dk_cfg" ]]; then
    [[ -f "$_dk_cfg" ]] || {
        echo "doctrine-kit: DOCTRINE_KIT_CONFIG_FILE not found: $_dk_cfg" >&2
        exit 2
    }
    # shellcheck disable=SC1090  # consumer-supplied config, path is config
    source "$_dk_cfg"
else
    _dk_cfg="${GATE_SDK_GATES_DIR:-scripts}/doctrine-config.sh"
    if [[ -f "$_dk_cfg" ]]; then
        # shellcheck disable=SC1090  # consumer-supplied config, path is config
        source "$_dk_cfg"
    fi
fi
# spec: doctrine-kit/SPEC.md §lib/doctrine.sh — the local overlay: a gitignored <config>.local.sh beside the tracked config sources last
_dk_local="${_dk_cfg%.sh}.local.sh"
if [[ -f "$_dk_local" ]]; then
    # shellcheck disable=SC1090  # consumer-supplied overlay, path is config
    source "$_dk_local"
fi
unset _dk_cfg _dk_local

: "${DOCTRINE_KIT_AGENT_FILE:=CLAUDE.md}"
: "${DOCTRINE_KIT_DOCTRINE_FILE:=doctrine-kit/DOCTRINE.md}"
: "${DOCTRINE_KIT_DIGEST_SECTION:=## Delivery doctrine}"
