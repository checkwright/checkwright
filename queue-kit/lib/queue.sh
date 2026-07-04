# shellcheck shell=bash
# spec: queue-kit/SPEC.md §lib/queue.sh — sourced config loader + shared section/slug adapters, never gate structure
#
# Extracted from the governance meta-layer of a private production platform;
# product-specific section names, wrap budget, and protocol vocabulary are the
# defaults below, overridable per consumer — the platform's task bodies and
# rule content stayed behind. This file carries values and parse adapters only
# (gate-sdk's lib/gate.sh rule): the section-boundary regexes both sides of
# every gate must agree on, and the slug/done extractors the tools share.

# Source the consumer config first so its assignments win; the defaults below
# fill only what it left unset. A malformed config exits 2 — a broken grammar
# must not gate anything.
_qk_cfg="${QUEUE_KIT_CONFIG_FILE:-}"
if [[ -n "$_qk_cfg" ]]; then
    [[ -f "$_qk_cfg" ]] || {
        echo "queue-kit: QUEUE_KIT_CONFIG_FILE not found: $_qk_cfg" >&2
        exit 2
    }
    # shellcheck disable=SC1090  # consumer-supplied config, path is config
    source "$_qk_cfg"
else
    _qk_cfg="${GATE_SDK_GATES_DIR:-scripts}/queue-config.sh"
    if [[ -f "$_qk_cfg" ]]; then
        # shellcheck disable=SC1090  # consumer-supplied config, path is config
        source "$_qk_cfg"
    fi
fi
unset _qk_cfg

# --- defaults (the platform's queue) -----------------------------------------

# The governed queue file (repo-root-relative; every gate also takes it as $1).
[[ -v QUEUE_KIT_QUEUE_FILE ]] || QUEUE_KIT_QUEUE_FILE="${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}"

# The pickable queue sections, in selection order. Plain text — each name is
# spliced into a '^## (…)$' heading regex, so avoid regex metacharacters.
declare -p QUEUE_KIT_ACTIVE_SECTIONS &>/dev/null \
    || QUEUE_KIT_ACTIVE_SECTIONS=("New Features" "Technical Debt")

# The parked section (excluded from selection) and the completed section.
[[ -v QUEUE_KIT_DEFERRED_SECTION ]] || QUEUE_KIT_DEFERRED_SECTION="Deferred"
[[ -v QUEUE_KIT_DONE_SECTION ]]     || QUEUE_KIT_DONE_SECTION="Done"

# The check-queue-wrap gate floor (columns; the authoring target is ~80).
[[ -v QUEUE_KIT_WRAP_BUDGET ]] || QUEUE_KIT_WRAP_BUDGET=100

# Column-0 lead tokens exempt from the hygiene gate's no-prose axis (whole-line
# lead match, not a substring).
declare -p QUEUE_KIT_PROSE_LEADS &>/dev/null || QUEUE_KIT_PROSE_LEADS=("Protocol:")

# The forward-precondition trigger set for check-queue-prose-precondition —
# present-tense, forward-looking phrasing only (past-tense narration is stripped
# before matching, so it never trips this). Extended regex, matched lowercased.
[[ -v QUEUE_KIT_PRECONDITION_REGEX ]] || QUEUE_KIT_PRECONDITION_REGEX='revisit when|once [^.]*(lands|ships|is (done|ready|merged))|gated on|contingent on|waiting on|pending [a-z]|blocked on'

# --- shared section adapters (both sides of every boundary parse identically) -

# queue_alt <name>... — a '|'-joined regex alternation body.
queue_alt() { local IFS='|'; printf '%s' "$*"; }

# Ready-to-use section-boundary regexes, consumed by the sourcing gates.
# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
QUEUE_ACTIVE_RE="^## ($(queue_alt "${QUEUE_KIT_ACTIVE_SECTIONS[@]}"))[[:space:]]*$"
# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
QUEUE_DEFERRED_RE="^## ${QUEUE_KIT_DEFERRED_SECTION}[[:space:]]*$"
# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
QUEUE_DONE_RE="^## ${QUEUE_KIT_DONE_SECTION}[[:space:]]*$"
# The task sections (active + deferred) share one slug namespace.
# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
QUEUE_TASK_RE="^## ($(queue_alt "${QUEUE_KIT_ACTIVE_SECTIONS[@]}" "$QUEUE_KIT_DEFERRED_SECTION"))[[:space:]]*$"
# Any '##' heading is a section boundary (closes whatever section we were in).
# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
QUEUE_SECTION_RE="^## "

# --- shared slug extractors (the lenient collectors the tools share) ---------

# queue_live_slugs <file> — the bold kebab-case handle at every task-section
# bullet lead (active + deferred, sub-tasks included), one per line. The
# authoritative shape validation lives in check-task-names; this is the
# collector conservation and the index ride.
queue_live_slugs() {
    awk -v taskre="$QUEUE_TASK_RE" -v sectre="$QUEUE_SECTION_RE" '
        $0 ~ taskre { inq = 1; next }
        $0 ~ sectre { inq = 0 }
        inq && $0 ~ /^[[:space:]]*-[[:space:]]+\*\*[a-z0-9][a-z0-9-]*\*\*/ {
            match($0, /\*\*[a-z0-9][a-z0-9-]*\*\*/)
            print substr($0, RSTART + 2, RLENGTH - 4)
        }
    ' "$1"
}

# queue_done_slugs <file> — the bare slug on every done-section line.
queue_done_slugs() {
    awk -v donere="$QUEUE_DONE_RE" -v sectre="$QUEUE_SECTION_RE" '
        $0 ~ donere { ind = 1; next }
        $0 ~ sectre { ind = 0 }
        ind && $0 ~ /^[[:space:]]*-[[:space:]]+[a-z0-9][a-z0-9-]*[[:space:]]*$/ {
            line = $0
            sub(/^[[:space:]]*-[[:space:]]+/, "", line)
            sub(/[[:space:]]*$/, "", line)
            print line
        }
    ' "$1"
}

# --- config validation (a malformed machine must not gate anything) ----------

_qk_errs=()
[[ ${#QUEUE_KIT_ACTIVE_SECTIONS[@]} -gt 0 ]] || _qk_errs+=("QUEUE_KIT_ACTIVE_SECTIONS is empty")
[[ -n "$QUEUE_KIT_DEFERRED_SECTION" ]] || _qk_errs+=("QUEUE_KIT_DEFERRED_SECTION is empty")
[[ -n "$QUEUE_KIT_DONE_SECTION" ]] || _qk_errs+=("QUEUE_KIT_DONE_SECTION is empty")
[[ "$QUEUE_KIT_WRAP_BUDGET" =~ ^[0-9]+$ && "$QUEUE_KIT_WRAP_BUDGET" -gt 0 ]] \
    || _qk_errs+=("QUEUE_KIT_WRAP_BUDGET must be a positive integer (got '$QUEUE_KIT_WRAP_BUDGET')")
[[ -n "$QUEUE_KIT_PRECONDITION_REGEX" ]] || _qk_errs+=("QUEUE_KIT_PRECONDITION_REGEX is empty")
if [[ ${#_qk_errs[@]} -gt 0 ]]; then
    printf 'queue-kit: malformed queue config — the gates cannot run:\n' >&2
    printf '  %s\n' "${_qk_errs[@]}" >&2
    exit 2
fi
unset _qk_errs
