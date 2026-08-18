# shellcheck shell=bash
# comment-tier-exempt: synthetic alias set for the bad-case fixture — the gate sources this via its config-file arg
# shellcheck disable=SC2034  # sourced by the gate under test
SITE_KIT_ALIASES=(
    alt.example
    legacy.example
)
