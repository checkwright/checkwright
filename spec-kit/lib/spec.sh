# shellcheck shell=bash
# spec: spec-kit/SPEC.md §lib/spec.sh — sourced config loader + shared section/spec adapters, never gate structure

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

[[ -v SPEC_KIT_SPEC_NAME ]]      || SPEC_KIT_SPEC_NAME="SPEC.md"
[[ -v SPEC_KIT_AMENDMENT_GLOB ]] || SPEC_KIT_AMENDMENT_GLOB="SPEC-*.md"

[[ -v SPEC_KIT_QUEUE_FILE ]] || SPEC_KIT_QUEUE_FILE="${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}"

declare -p SPEC_KIT_FEATURE_SECTIONS &>/dev/null || SPEC_KIT_FEATURE_SECTIONS=("New Features")
declare -p SPEC_KIT_ACTIVE_SECTIONS  &>/dev/null || SPEC_KIT_ACTIVE_SECTIONS=("New Features" "Technical Debt")

[[ -v SPEC_KIT_DEFERRED_SECTION ]] || SPEC_KIT_DEFERRED_SECTION="Deferred"

[[ -v SPEC_KIT_DOD_HEADING ]] || SPEC_KIT_DOD_HEADING="Definition of Done"
[[ -v SPEC_KIT_DOD_MODE ]]    || SPEC_KIT_DOD_MODE="exactly-one"

[[ -v SPEC_KIT_SCAN_KIT_ROOTS ]] || SPEC_KIT_SCAN_KIT_ROOTS=0

declare -p SPEC_KIT_BANNED_HEADINGS &>/dev/null \
    || SPEC_KIT_BANNED_HEADINGS=("Directory Structure" "Public API" "Cargo.toml Dependencies")
[[ -v SPEC_KIT_DERIVABLE_DENSITY ]]       || SPEC_KIT_DERIVABLE_DENSITY=60
[[ -v SPEC_KIT_DERIVABLE_POINTER_REGEX ]] || SPEC_KIT_DERIVABLE_POINTER_REGEX='pub-index|proto-index'

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
[[ -v SPEC_KIT_EMBED_WIRE_KIND ]] || SPEC_KIT_EMBED_WIRE_KIND="proto"

[[ -v SPEC_KIT_GLOSSARY_FILE ]] || SPEC_KIT_GLOSSARY_FILE="GLOSSARY.md"
declare -p SPEC_KIT_DUP_SURFACES &>/dev/null || SPEC_KIT_DUP_SURFACES=("VISION.md")

declare -p SPEC_KIT_MDREF_EXCLUDE &>/dev/null || SPEC_KIT_MDREF_EXCLUDE=()

declare -p SPEC_KIT_MANIFEST_FILES &>/dev/null || SPEC_KIT_MANIFEST_FILES=()
declare -p SPEC_KIT_TEMPORAL_MARKERS &>/dev/null || SPEC_KIT_TEMPORAL_MARKERS=(
    "previously"
    "formerly"
    "renamed from"
    "no longer"
    "used to be"
    "was (retired|removed|renamed|replaced)"
)
declare -p SPEC_KIT_TEMPORAL_EXEMPT_SECTIONS &>/dev/null || SPEC_KIT_TEMPORAL_EXEMPT_SECTIONS=()
declare -p SPEC_KIT_TEMPORAL_EXEMPT_PATHS &>/dev/null || SPEC_KIT_TEMPORAL_EXEMPT_PATHS=()

declare -p SPEC_KIT_COUNT_COLLECTIONS &>/dev/null || SPEC_KIT_COUNT_COLLECTIONS=(
    "gates"
    "meta-gates"
    "checks"
    "kits"
    "stages"
    "rules"
    "KPIs"
)
declare -p SPEC_KIT_COUNT_ALLOWED_PHRASES &>/dev/null || SPEC_KIT_COUNT_ALLOWED_PHRASES=()
[[ -v SPEC_KIT_COUNT_WEDGE_WORDS ]] || SPEC_KIT_COUNT_WEDGE_WORDS=2

declare -p SPEC_KIT_COMMENT_MACHINE &>/dev/null || SPEC_KIT_COMMENT_MACHINE=()
declare -p SPEC_KIT_COMMENT_REASON  &>/dev/null || SPEC_KIT_COMMENT_REASON=()
declare -p SPEC_KIT_COMMENT_SURFACE &>/dev/null || SPEC_KIT_COMMENT_SURFACE=()
declare -p SPEC_KIT_COMMENT_POSITIONAL &>/dev/null || SPEC_KIT_COMMENT_POSITIONAL=()
declare -p SPEC_KIT_COMMENT_WHITELIST &>/dev/null || SPEC_KIT_COMMENT_WHITELIST=()
[[ -v SPEC_KIT_COMMENT_RUN_CAP ]] || SPEC_KIT_COMMENT_RUN_CAP=3

spec_alt() { local IFS='|'; printf '%s' "$*"; }

# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
SPEC_FEATURE_RE="^## ($(spec_alt "${SPEC_KIT_FEATURE_SECTIONS[@]}"))[[:space:]]*$"
# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
SPEC_ACTIVE_RE="^## ($(spec_alt "${SPEC_KIT_ACTIVE_SECTIONS[@]}"))[[:space:]]*$"
# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
SPEC_DEFERRED_RE="^## ${SPEC_KIT_DEFERRED_SECTION}[[:space:]]*$"
# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
SPEC_SECTION_RE="^## "

# spec: spec-kit/SPEC.md §lib/spec.sh — finders skip templates/ stubs and vendored kit roots under the scan root (an ancestor kit root never prunes)
_spec_prune_kit_roots() {
    if [[ "$SPEC_KIT_SCAN_KIT_ROOTS" == "1" ]]; then cat; return 0; fi
    local root="${1:-.}" root_abs
    case "$root" in
        /*)  root_abs="$root" ;;
        .)   root_abs="$PWD" ;;
        ./*) root_abs="$PWD/${root#./}" ;;
        *)   root_abs="$PWD/$root" ;;
    esac
    root_abs="${root_abs%/}"
    local -a roots=()
    local r rabs f fabs keep
    while IFS= read -r r; do
        [[ -n "$r" ]] || continue
        [[ "$r" == /* ]] && rabs="$r" || rabs="$PWD/$r"
        rabs="${rabs%/}"
        [[ "$rabs" == "$root_abs/"* ]] || continue   # only a vendored subtree prunes
        roots+=("$rabs")
    done < <(gate_kit_roots)
    [[ ${#roots[@]} -eq 0 ]] && { cat; return 0; }
    while IFS= read -r f; do
        [[ -n "$f" ]] || continue
        [[ "$f" == /* ]] && fabs="$f" || fabs="$PWD/${f#./}"
        keep=1
        for r in "${roots[@]}"; do
            [[ "$fabs" == "$r/"* ]] && { keep=0; break; }
        done
        [[ "$keep" == "1" ]] && printf '%s\n' "$f"
    done
}

spec_canonical_specs() { gate_find "$1" -name "$SPEC_KIT_SPEC_NAME" -type f 2>/dev/null | grep -v '/templates/' | _spec_prune_kit_roots "$1" || true; }

spec_amendments() { gate_find "$1" -name "$SPEC_KIT_AMENDMENT_GLOB" -type f 2>/dev/null | grep -v '/templates/' | _spec_prune_kit_roots "$1" || true; }

# spec: spec-kit/SPEC.md §lib/spec.sh — the manifest set shared by the manifest-narration gate family: canonical specs (kit-root pruned per SPEC_KIT_SCAN_KIT_ROOTS) plus README.md at any depth and CLAUDE.md; explicit globs when SPEC_KIT_MANIFEST_FILES is set. Amendments are excluded by construction — a transition artifact describes change.
spec_manifest_files() {
    local root="${1:-.}" g f
    if [[ ${#SPEC_KIT_MANIFEST_FILES[@]} -gt 0 ]]; then
        shopt -s nullglob globstar
        for g in "${SPEC_KIT_MANIFEST_FILES[@]}"; do
            for f in "$root"/$g; do [[ -f "$f" ]] && printf '%s\n' "$f"; done
        done
        shopt -u nullglob globstar
    else
        spec_canonical_specs "$root"
        gate_find "$root" -name 'README.md' -type f 2>/dev/null | grep -v '/templates/' || true
        gate_find "$root" -name 'CLAUDE.md' -type f 2>/dev/null || true
    fi
}

# spec: spec-kit/SPEC.md §lib/spec.sh — the two governed comment surfaces: the
#   tier gate scans _with_templates, the pointer gate scans the pruned surface
#   (templates exempt as placeholders-by-design).
_spec_comment_surface() {  # $1=root  $2=1 keeps templates/ shell sources, else prunes them
    local root="${1:-.}" incl="${2:-0}" g f
    if [[ ${#SPEC_KIT_COMMENT_SURFACE[@]} -gt 0 ]]; then
        shopt -s nullglob globstar
        for g in "${SPEC_KIT_COMMENT_SURFACE[@]}"; do
            for f in "$root"/$g; do [[ -f "$f" ]] && printf '%s\n' "$f"; done
        done
        shopt -u nullglob globstar
    else
        if [[ "$incl" == "1" ]]; then
            gate_find "$root" -name '*.sh' -type f 2>/dev/null | _spec_prune_kit_roots "$root" | sort
        else
            gate_find "$root" -name '*.sh' -type f 2>/dev/null | grep -v '/templates/' | _spec_prune_kit_roots "$root" | sort
        fi
        shopt -s nullglob
        for f in "$root/${GATE_SDK_WORKFLOW_DIR:-.workflow}"/*.txt; do printf '%s\n' "$f"; done
        shopt -u nullglob
    fi
}

spec_comment_surface() { _spec_comment_surface "$1" 0; }

spec_comment_surface_with_templates() { _spec_comment_surface "$1" 1; }

spec_comment_whitelisted() {  # $1=root-relative path — true when it matches a consumer whitelist glob
    local rel="$1" g
    for g in "${SPEC_KIT_COMMENT_WHITELIST[@]}"; do
        # shellcheck disable=SC2053  # intentional glob match: $g is a pattern
        [[ "$rel" == $g ]] && return 0
    done
    return 1
}

# spec: spec-kit/SPEC.md §lib/spec.sh — the count grammar the restated-total gate family shares: one cardinal alternation, one consumer noun vocabulary, two match shapes
SPEC_COUNT_CARDINAL_RE='([0-9]+|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)'

spec_count_noun_alt() {
    local n out=""
    for n in "${SPEC_KIT_COUNT_COLLECTIONS[@]}"; do
        [[ -n "$n" ]] || continue
        out="${out:+$out|}${n,,}"
    done
    printf '%s' "$out"
}

# spec: spec-kit/SPEC.md §check-manifest-count — the wedge groups are optional, so bare adjacency is this shape's zero-wedge case rather than a second branch
spec_count_quantifier_re() {
    local i opt=""
    for ((i = 0; i < SPEC_KIT_COUNT_WEDGE_WORDS; i++)); do
        opt+='([[:space:]]+[[:alnum:]_-]+)?'
    done
    printf '%s%s[[:space:]]+(%s)' "$SPEC_COUNT_CARDINAL_RE" "$opt" "$(spec_count_noun_alt)"
}

spec_count_range_re() {
    printf '(%s)[[:space:]]+[0-9]+-[0-9]+' "$(spec_count_noun_alt)"
}

_sk_errs=()
[[ -n "$SPEC_KIT_SPEC_NAME" ]]      || _sk_errs+=("SPEC_KIT_SPEC_NAME is empty")
[[ -n "$SPEC_KIT_AMENDMENT_GLOB" ]] || _sk_errs+=("SPEC_KIT_AMENDMENT_GLOB is empty")
[[ ${#SPEC_KIT_FEATURE_SECTIONS[@]} -gt 0 ]] || _sk_errs+=("SPEC_KIT_FEATURE_SECTIONS is empty")
[[ ${#SPEC_KIT_ACTIVE_SECTIONS[@]} -gt 0 ]]  || _sk_errs+=("SPEC_KIT_ACTIVE_SECTIONS is empty")
[[ -n "$SPEC_KIT_DEFERRED_SECTION" ]] || _sk_errs+=("SPEC_KIT_DEFERRED_SECTION is empty")
[[ -n "$SPEC_KIT_DOD_HEADING" ]]      || _sk_errs+=("SPEC_KIT_DOD_HEADING is empty")
[[ "$SPEC_KIT_DOD_MODE" == "exactly-one" || "$SPEC_KIT_DOD_MODE" == "at-most-one" ]] \
    || _sk_errs+=("SPEC_KIT_DOD_MODE must be exactly-one|at-most-one (got '$SPEC_KIT_DOD_MODE')")
[[ "$SPEC_KIT_SCAN_KIT_ROOTS" == "0" || "$SPEC_KIT_SCAN_KIT_ROOTS" == "1" ]] \
    || _sk_errs+=("SPEC_KIT_SCAN_KIT_ROOTS must be 0|1 (got '$SPEC_KIT_SCAN_KIT_ROOTS')")
[[ "$SPEC_KIT_DERIVABLE_DENSITY" =~ ^[0-9]+$ && "$SPEC_KIT_DERIVABLE_DENSITY" -ge 0 && "$SPEC_KIT_DERIVABLE_DENSITY" -le 100 ]] \
    || _sk_errs+=("SPEC_KIT_DERIVABLE_DENSITY must be 0..100 (got '$SPEC_KIT_DERIVABLE_DENSITY')")
[[ "$SPEC_KIT_EMBED_THRESHOLD" =~ ^0?\.[0-9]+$|^1(\.0+)?$ ]] \
    || _sk_errs+=("SPEC_KIT_EMBED_THRESHOLD must be a 0..1 fraction (got '$SPEC_KIT_EMBED_THRESHOLD')")
[[ "$SPEC_KIT_EMBED_MINLINES" =~ ^[0-9]+$ && "$SPEC_KIT_EMBED_MINLINES" -gt 0 ]] \
    || _sk_errs+=("SPEC_KIT_EMBED_MINLINES must be a positive integer (got '$SPEC_KIT_EMBED_MINLINES')")
[[ -n "$SPEC_KIT_GLOSSARY_FILE" ]] || _sk_errs+=("SPEC_KIT_GLOSSARY_FILE is empty")
[[ ${#SPEC_KIT_TEMPORAL_MARKERS[@]} -gt 0 ]] || _sk_errs+=("SPEC_KIT_TEMPORAL_MARKERS is empty")
[[ ${#SPEC_KIT_COUNT_COLLECTIONS[@]} -gt 0 ]] || _sk_errs+=("SPEC_KIT_COUNT_COLLECTIONS is empty")
[[ "$SPEC_KIT_COUNT_WEDGE_WORDS" =~ ^[0-9]+$ && "$SPEC_KIT_COUNT_WEDGE_WORDS" -gt 0 ]] \
    || _sk_errs+=("SPEC_KIT_COUNT_WEDGE_WORDS must be a positive integer (got '$SPEC_KIT_COUNT_WEDGE_WORDS')")
[[ "$SPEC_KIT_COMMENT_RUN_CAP" =~ ^[0-9]+$ && "$SPEC_KIT_COMMENT_RUN_CAP" -gt 0 ]] \
    || _sk_errs+=("SPEC_KIT_COMMENT_RUN_CAP must be a positive integer (got '$SPEC_KIT_COMMENT_RUN_CAP')")
if [[ ${#_sk_errs[@]} -gt 0 ]]; then
    printf 'spec-kit: malformed spec config — the gates cannot run:\n' >&2
    printf '  %s\n' "${_sk_errs[@]}" >&2
    exit 2
fi
unset _sk_errs
