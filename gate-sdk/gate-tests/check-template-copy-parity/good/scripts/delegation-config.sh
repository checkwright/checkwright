# shellcheck shell=bash
# spec: delegation-kit/SPEC.md §Layout and configuration — consumer copy, customized: a different value and an extra knob. Out of scope by the *-config.sh suffix rule, so this divergence must NOT be reported.
DELEGATION_KIT_PAUSE_PCT="${DELEGATION_KIT_PAUSE_PCT:-75}"
DELEGATION_KIT_WEEKLY_PCT="${DELEGATION_KIT_WEEKLY_PCT:-60}"
