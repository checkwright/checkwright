# shellcheck shell=bash
# A library sets no pipefail and is linted from its first line all the same.
first() { ls | head -1; }
