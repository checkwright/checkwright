# shellcheck shell=bash
# spec: spec-kit/SPEC.md §lib/spec.sh — sourced config loader + shared section/spec adapters, never gate structure
#
# Extracted from the governance meta-layer of a private production platform; the
# product's surface names, banned headings, and scanned languages are the
# defaults below, overridable per consumer — the rule content (glossary bodies,
# term lists, tier contract) stayed behind. This file carries values and parse
# adapters only (gate-sdk's lib/gate.sh rule): the section-boundary regexes both
# sides of the queue-facing gate must agree on, and the spec/amendment finders
# and language map the spec-scanning gates share. Never gate structure.

# Source the consumer config first so its assignments win; the defaults below
# fill only what it left unset. A malformed config exits 2 — a broken grammar
# must not gate anything.
_sk_cfg="${SPEC_KIT_CONFIG_FILE:-}"
if [[ -n "$_sk_cfg" ]]; then
    [[ -f "$_sk_cfg" ]] || {
        echo "spec-kit: SPEC_KIT_CONFIG_FILE not found: $_sk_cfg" >&2
        exit 2
    }
    # shellcheck disable=SC1090  # consumer-supplied config, path is config
    source "$_sk_cfg"
else
    _sk_cfg="${GATE_SDK_GATES_DIR:-scripts}/spec-config.sh"
    if [[ -f "$_sk_cfg" ]]; then
        # shellcheck disable=SC1090  # consumer-supplied config, path is config
        source "$_sk_cfg"
    fi
fi
unset _sk_cfg

# --- defaults (the platform's spec discipline) -------------------------------

# The canonical spec filename and the amendment-file glob.
[[ -v SPEC_KIT_SPEC_NAME ]]      || SPEC_KIT_SPEC_NAME="SPEC.md"
[[ -v SPEC_KIT_AMENDMENT_GLOB ]] || SPEC_KIT_AMENDMENT_GLOB="SPEC-*.md"

# The governed queue file (repo-root-relative; the queue-facing gate also takes
# it as $1). Shares queue-kit's default; the two knobs are independent.
[[ -v SPEC_KIT_QUEUE_FILE ]] || SPEC_KIT_QUEUE_FILE="${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}"

# Active-queue sections whose entries require a [spec:] ref (feature sections),
# and the broader active set where a [needs-spec] tag is a violation. Plain
# text — each name is spliced into a '^## (…)$' heading regex, so avoid regex
# metacharacters. Cross-kit note: queue-kit's QUEUE_KIT_ACTIVE_SECTIONS carries
# the same default; a consumer renaming its sections sets both (independent).
declare -p SPEC_KIT_FEATURE_SECTIONS &>/dev/null || SPEC_KIT_FEATURE_SECTIONS=("New Features")
declare -p SPEC_KIT_ACTIVE_SECTIONS  &>/dev/null || SPEC_KIT_ACTIVE_SECTIONS=("New Features" "Technical Debt")

# The parked section: every entry there requires [needs-spec].
[[ -v SPEC_KIT_DEFERRED_SECTION ]] || SPEC_KIT_DEFERRED_SECTION="Deferred"

# The Definition-of-Done heading and how many a canonical spec may carry.
# 'exactly-one' (platform default) flags a spec with none; 'at-most-one' allows
# zero (a reference-spec corpus like this repo's kits carries no DoD).
[[ -v SPEC_KIT_DOD_HEADING ]] || SPEC_KIT_DOD_HEADING="Definition of Done"
[[ -v SPEC_KIT_DOD_MODE ]]    || SPEC_KIT_DOD_MODE="exactly-one"

# The code-derivable heading set and the fenced-density budget (percent) above
# which such a section is a code dump. The pointer regex names the one-line
# index reference that exempts a shed section (consumer index tooling).
declare -p SPEC_KIT_BANNED_HEADINGS &>/dev/null \
    || SPEC_KIT_BANNED_HEADINGS=("Directory Structure" "Public API" "Cargo.toml Dependencies")
[[ -v SPEC_KIT_DERIVABLE_DENSITY ]]       || SPEC_KIT_DERIVABLE_DENSITY=60
[[ -v SPEC_KIT_DERIVABLE_POINTER_REGEX ]] || SPEC_KIT_DERIVABLE_POINTER_REGEX='pub-index|proto-index'

# check-spec-embedded-source calibration. SPEC_KIT_EMBED_LANGS is the scanned
# fence-language → source mapping: one 'kind|fence-alias,…|file-glob,…' entry
# per language family. A fence whose language is not a listed alias (or is in
# SPEC_KIT_EMBED_ILLUSTRATIVE) is treated as illustrative and skipped.
[[ -v SPEC_KIT_EMBED_THRESHOLD ]] || SPEC_KIT_EMBED_THRESHOLD="0.70"
[[ -v SPEC_KIT_EMBED_MINLINES ]]  || SPEC_KIT_EMBED_MINLINES=8
declare -p SPEC_KIT_EMBED_LANGS &>/dev/null || SPEC_KIT_EMBED_LANGS=(
    "rs|rust,rs|*.rs"
    "toml|toml|*.toml"
    "sql|sql|*.sql"
    "sh|bash,sh|*.sh"
    "yaml|yaml,yml|*.yaml,*.yml"
    "ts|typescript,ts,tsx|*.ts,*.tsx"
    "rego|rego|*.rego"
    "proto|proto,protobuf|*.proto"
    "dockerfile|dockerfile|Dockerfile"
)
declare -p SPEC_KIT_EMBED_ILLUSTRATIVE &>/dev/null || SPEC_KIT_EMBED_ILLUSTRATIVE=("json")
# The fence kind an amendment may embed as a not-yet-merged wire-contract delta
# (its own valve — the design home for a contract that does not exist yet).
[[ -v SPEC_KIT_EMBED_WIRE_KIND ]] || SPEC_KIT_EMBED_WIRE_KIND="proto"

# check-surface-duplication topology. The glossary owns the canonical
# definitions; the listed non-glossary surfaces are scanned for foreign
# bold-lead-in definitions (every component SPEC.md is added automatically).
[[ -v SPEC_KIT_GLOSSARY_FILE ]] || SPEC_KIT_GLOSSARY_FILE="GLOSSARY.md"
declare -p SPEC_KIT_DUP_SURFACES &>/dev/null || SPEC_KIT_DUP_SURFACES=("VISION.md")

# --- shared section adapters (both sides of every boundary parse identically) -

# spec_alt <name>... — a '|'-joined regex alternation body.
spec_alt() { local IFS='|'; printf '%s' "$*"; }

# Ready-to-use section-boundary regexes, consumed by the sourcing gate.
# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
SPEC_FEATURE_RE="^## ($(spec_alt "${SPEC_KIT_FEATURE_SECTIONS[@]}"))[[:space:]]*$"
# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
SPEC_ACTIVE_RE="^## ($(spec_alt "${SPEC_KIT_ACTIVE_SECTIONS[@]}"))[[:space:]]*$"
# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
SPEC_DEFERRED_RE="^## ${SPEC_KIT_DEFERRED_SECTION}[[:space:]]*$"
# Any '##' heading is a section boundary (closes whatever section we were in).
# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
SPEC_SECTION_RE="^## "

# --- shared spec/amendment finders (the collectors the gates share) ----------

# spec_canonical_specs <root> — every canonical spec under root (gate_find
# prunes the tree-walk exclusion set, so kit fixtures never leak into a
# whole-tree run). Requires gate-sdk's lib/gate.sh already sourced. A skeleton
# under a templates/ directory is a copyable stub, not governed content (same
# rationale as the gate-tests prune) — the finders skip it so a shipped
# SPEC-amendment.md template never reads as a live orphan amendment.
spec_canonical_specs() { gate_find "$1" -name "$SPEC_KIT_SPEC_NAME" -type f 2>/dev/null | grep -v '/templates/' || true; }

# spec_amendments <root> — every amendment file under root (templates skipped).
spec_amendments() { gate_find "$1" -name "$SPEC_KIT_AMENDMENT_GLOB" -type f 2>/dev/null | grep -v '/templates/' || true; }

# --- config validation (a malformed machine must not gate anything) ----------

_sk_errs=()
[[ -n "$SPEC_KIT_SPEC_NAME" ]]      || _sk_errs+=("SPEC_KIT_SPEC_NAME is empty")
[[ -n "$SPEC_KIT_AMENDMENT_GLOB" ]] || _sk_errs+=("SPEC_KIT_AMENDMENT_GLOB is empty")
[[ ${#SPEC_KIT_FEATURE_SECTIONS[@]} -gt 0 ]] || _sk_errs+=("SPEC_KIT_FEATURE_SECTIONS is empty")
[[ ${#SPEC_KIT_ACTIVE_SECTIONS[@]} -gt 0 ]]  || _sk_errs+=("SPEC_KIT_ACTIVE_SECTIONS is empty")
[[ -n "$SPEC_KIT_DEFERRED_SECTION" ]] || _sk_errs+=("SPEC_KIT_DEFERRED_SECTION is empty")
[[ -n "$SPEC_KIT_DOD_HEADING" ]]      || _sk_errs+=("SPEC_KIT_DOD_HEADING is empty")
[[ "$SPEC_KIT_DOD_MODE" == "exactly-one" || "$SPEC_KIT_DOD_MODE" == "at-most-one" ]] \
    || _sk_errs+=("SPEC_KIT_DOD_MODE must be exactly-one|at-most-one (got '$SPEC_KIT_DOD_MODE')")
[[ "$SPEC_KIT_DERIVABLE_DENSITY" =~ ^[0-9]+$ && "$SPEC_KIT_DERIVABLE_DENSITY" -ge 0 && "$SPEC_KIT_DERIVABLE_DENSITY" -le 100 ]] \
    || _sk_errs+=("SPEC_KIT_DERIVABLE_DENSITY must be 0..100 (got '$SPEC_KIT_DERIVABLE_DENSITY')")
[[ "$SPEC_KIT_EMBED_THRESHOLD" =~ ^0?\.[0-9]+$|^1(\.0+)?$ ]] \
    || _sk_errs+=("SPEC_KIT_EMBED_THRESHOLD must be a 0..1 fraction (got '$SPEC_KIT_EMBED_THRESHOLD')")
[[ "$SPEC_KIT_EMBED_MINLINES" =~ ^[0-9]+$ && "$SPEC_KIT_EMBED_MINLINES" -gt 0 ]] \
    || _sk_errs+=("SPEC_KIT_EMBED_MINLINES must be a positive integer (got '$SPEC_KIT_EMBED_MINLINES')")
[[ -n "$SPEC_KIT_GLOSSARY_FILE" ]] || _sk_errs+=("SPEC_KIT_GLOSSARY_FILE is empty")
if [[ ${#_sk_errs[@]} -gt 0 ]]; then
    printf 'spec-kit: malformed spec config — the gates cannot run:\n' >&2
    printf '  %s\n' "${_sk_errs[@]}" >&2
    exit 2
fi
unset _sk_errs
