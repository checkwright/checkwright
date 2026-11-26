# shellcheck shell=bash
# A library, linted from its first line; its two sites are within its baseline row.
first() { ls | head -1; }
quiet() { ls | read -r _; }
