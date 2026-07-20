# shellcheck shell=bash
# spec: delegation-kit/SPEC.md §Layout and configuration — a config template is a starting point the consumer customizes; divergence from the copy IS the contract, so the *-config.sh suffix is out of scope by name
DELEGATION_KIT_PAUSE_PCT="${DELEGATION_KIT_PAUSE_PCT:-90}"
