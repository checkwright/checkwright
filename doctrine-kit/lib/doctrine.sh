# shellcheck shell=bash
# spec: doctrine-kit/SPEC.md §lib/doctrine.sh — sourced config loader + the two knob defaults, never gate structure

_dk_cfg="${DOCTRINE_KIT_CONFIG_FILE:-${GATE_SDK_GATES_DIR:-scripts}/doctrine-config.sh}"
if [[ -f "$_dk_cfg" ]]; then
    # shellcheck disable=SC1090  # consumer-supplied config, path is config
    source "$_dk_cfg"
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
