# shellcheck shell=bash
# Consumer delegation config for delegation-kit (delegation-kit/SPEC.md §Layout
# and configuration). Copy into your gates dir as delegation-config.sh (or point
# DELEGATION_KIT_CONFIG_FILE at it). Every knob is optional: anything left unset
# keeps the kit default shown here. A malformed config exits 2 — a broken machine
# must not gate anything. Array knobs (globs, prefixes) live here, not in env.

# --- usage-verdict ---------------------------------------------------------------

# The usage snapshot usage-verdict reads (its positional $1 still overrides, for
# test injection). Default the single-operator harness config path.
#DELEGATION_KIT_USAGE_FILE="${CLAUDE_CONFIG_DIR:-$HOME/.claude}/usage.txt"

# The credentials file whose mtime dates the last auth event (positional $2
# overrides). Default the usage file's sibling.
#DELEGATION_KIT_CRED_FILE="${DELEGATION_KIT_USAGE_FILE%/*}/.credentials.json"

# PAUSE above this percentage of the live 5h window; STALE_AGE / LOGIN_WINDOW in
# seconds (the reading-staleness floor, and the post-login re-read window).
#DELEGATION_KIT_PAUSE_PCT=80
#DELEGATION_KIT_STALE_AGE=600
#DELEGATION_KIT_LOGIN_WINDOW=600

# --- check-gate-tamper --------------------------------------------------------

# Globs naming gate files (assertion A trigger). Default the platform's single
# gates-dir layout. A gate-sdk consumer with kit checks/ dirs widens it, e.g.:
#DELEGATION_KIT_GATE_FILES=(
#    "*/checks/*.sh"
#    "gate-sdk/lib/gate.sh"
#    "gate-sdk/bin/run-gate-tests.sh"
#)

# Path prefixes counted as meta-layer (assertion A). Root-level *.md is always
# meta, and — when gate.sh resolves — every vendored kit root is auto-unioned in
# (a kit's edits are meta by definition), so list only non-kit prefixes here.
# Default the platform's governance surfaces, e.g.:
#DELEGATION_KIT_META_PATHS=(
#    scripts/ .workflow/ .claude/
#)
