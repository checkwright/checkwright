# shellcheck shell=bash
# spec: queue-kit/SPEC.md §lib/queue.sh — sourced config loader + shared section/slug adapters, never gate structure

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
# spec: queue-kit/SPEC.md §lib/queue.sh — the local overlay: a gitignored <config>.local.sh beside the tracked config sources last, carrying private sink values a tracked config cannot
_qk_local="${_qk_cfg%.sh}.local.sh"
if [[ -f "$_qk_local" ]]; then
    # shellcheck disable=SC1090  # consumer-supplied overlay, path is config
    source "$_qk_local"
fi
unset _qk_cfg _qk_local

[[ -v QUEUE_KIT_QUEUE_FILE ]] || QUEUE_KIT_QUEUE_FILE="${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}"

declare -p QUEUE_KIT_ACTIVE_SECTIONS &>/dev/null \
    || QUEUE_KIT_ACTIVE_SECTIONS=("New Features" "Technical Debt")

[[ -v QUEUE_KIT_DEFERRED_SECTION ]] || QUEUE_KIT_DEFERRED_SECTION="Deferred"
[[ -v QUEUE_KIT_ICEBOX_SECTION ]]   || QUEUE_KIT_ICEBOX_SECTION=""
[[ -v QUEUE_KIT_DONE_SECTION ]]     || QUEUE_KIT_DONE_SECTION="Done"

[[ -v QUEUE_KIT_WRAP_BUDGET ]] || QUEUE_KIT_WRAP_BUDGET=100

[[ -v QUEUE_KIT_ENTRY_LINE_CAP ]] || QUEUE_KIT_ENTRY_LINE_CAP=50

[[ -v QUEUE_KIT_ICEBOX_AGE_DAYS ]] || QUEUE_KIT_ICEBOX_AGE_DAYS=30

declare -p QUEUE_KIT_REQUIRED_SECTIONS &>/dev/null \
    || QUEUE_KIT_REQUIRED_SECTIONS=("Iteration:" "New Features" "Technical Debt" "Deferred" "Done" "Lessons Learned")

# spec: queue-kit/SPEC.md §Layout and configuration — the icebox joins the required set by derivation, never by a second consumer list: a configured-but-absent section would let every icebox assertion pass open on a section that is not there
[[ -n "$QUEUE_KIT_ICEBOX_SECTION" ]] && QUEUE_KIT_REQUIRED_SECTIONS+=("$QUEUE_KIT_ICEBOX_SECTION")

declare -p QUEUE_KIT_PROSE_LEADS &>/dev/null || QUEUE_KIT_PROSE_LEADS=("Protocol:")

declare -p QUEUE_KIT_PROSE_SURFACE_GLOBS &>/dev/null || QUEUE_KIT_PROSE_SURFACE_GLOBS=()

[[ -v QUEUE_KIT_PRECONDITION_REGEX ]] || QUEUE_KIT_PRECONDITION_REGEX='revisit when|once [^.]*(lands|ships|is (done|ready|merged))|gated on|contingent on|waiting on|pending [a-z]|blocked on'

declare -p QUEUE_KIT_LESSON_TAGS &>/dev/null || QUEUE_KIT_LESSON_TAGS=()

declare -p QUEUE_KIT_LESSON_SINKS &>/dev/null || declare -A QUEUE_KIT_LESSON_SINKS=()

[[ -v QUEUE_KIT_ATTEND_CAP ]] || QUEUE_KIT_ATTEND_CAP=3

declare -p QUEUE_KIT_HORIZONS &>/dev/null || QUEUE_KIT_HORIZONS=()

declare -p QUEUE_KIT_TRACKS &>/dev/null || QUEUE_KIT_TRACKS=()

[[ -v QUEUE_KIT_ROADMAP_FILE ]] || QUEUE_KIT_ROADMAP_FILE=""

[[ -v QUEUE_KIT_ROADMAP_MARKER ]] || QUEUE_KIT_ROADMAP_MARKER="roadmap"

queue_alt() { local IFS='|'; printf '%s' "$*"; }

# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
QUEUE_ACTIVE_RE="^## ($(queue_alt "${QUEUE_KIT_ACTIVE_SECTIONS[@]}"))[[:space:]]*$"
# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
QUEUE_DEFERRED_RE="^## ${QUEUE_KIT_DEFERRED_SECTION}[[:space:]]*$"
# spec: queue-kit/SPEC.md §The icebox tier — an unset knob leaves a regex nothing can match, so every icebox reader degrades to "no icebox" rather than to "every section"
# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
QUEUE_ICEBOX_RE="${QUEUE_KIT_ICEBOX_SECTION:+^## ${QUEUE_KIT_ICEBOX_SECTION}[[:space:]]*$}"
# spec: queue-kit/SPEC.md §lib/queue.sh — the task-section set in configured order, the one composition the task regex and every per-section reader share
QUEUE_TASK_SECTIONS=("${QUEUE_KIT_ACTIVE_SECTIONS[@]}" "$QUEUE_KIT_DEFERRED_SECTION")
# spec: queue-kit/SPEC.md §The icebox tier — the icebox is a *live* task section: joining the shared task regex is what makes eviction a conserved move and carries slug uniqueness, blocker resolution, the living-prose contract and the lead-line guard onto the tier with no gate edit
[[ -n "$QUEUE_KIT_ICEBOX_SECTION" ]] && QUEUE_TASK_SECTIONS+=("$QUEUE_KIT_ICEBOX_SECTION")
# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
QUEUE_TASK_RE="^## ($(queue_alt "${QUEUE_TASK_SECTIONS[@]}"))[[:space:]]*$"
# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
QUEUE_SECTION_RE="^## "
# spec: queue-kit/SPEC.md §The tag algebra — the Lessons heading is fixed spelling (a required-sections default), read by queue-index + check-tag-lead-line; no knob
# shellcheck disable=SC2034  # consumed by sourcing tools, never within this lib
QUEUE_LESSONS_RE="^## Lessons Learned[[:space:]]*$"

# spec: queue-kit/SPEC.md §lib/queue.sh — the entry lead-line grammar as a shared adapter rather than an internal of the live-slug reader: bin/queue-edges.sh's history walk is its second reader, and the library's rule is that a grammar with more than one reader is named here
# spec: queue-kit/SPEC.md §lib/queue.sh — bracketed literals, never backslash escapes: these cross into awk through -v, where awk's string-escape pass would eat a backslash before the regex ever sees it
# shellcheck disable=SC2034  # consumed by sourcing tools, never within this lib
QUEUE_SLUG_BOLD_RE="[*][*][a-z0-9][a-z0-9-]*[*][*]"
# shellcheck disable=SC2034  # consumed by sourcing tools, never within this lib
QUEUE_LEAD_RE="^[[:space:]]*-[[:space:]]+$QUEUE_SLUG_BOLD_RE"

queue_live_slugs() {
    awk -v taskre="$QUEUE_TASK_RE" -v sectre="$QUEUE_SECTION_RE" \
        -v leadre="$QUEUE_LEAD_RE" -v slugre="$QUEUE_SLUG_BOLD_RE" '
        $0 ~ taskre { inq = 1; next }
        $0 ~ sectre { inq = 0 }
        inq && $0 ~ leadre {
            match($0, slugre)
            print substr($0, RSTART + 2, RLENGTH - 4)
        }
    ' "$1"
}

_qk_errs=()
[[ ${#QUEUE_KIT_ACTIVE_SECTIONS[@]} -gt 0 ]] || _qk_errs+=("QUEUE_KIT_ACTIVE_SECTIONS is empty")
[[ -n "$QUEUE_KIT_DEFERRED_SECTION" ]] || _qk_errs+=("QUEUE_KIT_DEFERRED_SECTION is empty")
[[ -n "$QUEUE_KIT_DONE_SECTION" ]] || _qk_errs+=("QUEUE_KIT_DONE_SECTION is empty")
[[ "$QUEUE_KIT_WRAP_BUDGET" =~ ^[0-9]+$ && "$QUEUE_KIT_WRAP_BUDGET" -gt 0 ]] \
    || _qk_errs+=("QUEUE_KIT_WRAP_BUDGET must be a positive integer (got '$QUEUE_KIT_WRAP_BUDGET')")
[[ "$QUEUE_KIT_ATTEND_CAP" =~ ^[0-9]+$ && "$QUEUE_KIT_ATTEND_CAP" -gt 0 ]] \
    || _qk_errs+=("QUEUE_KIT_ATTEND_CAP must be a positive integer (got '$QUEUE_KIT_ATTEND_CAP')")
[[ "$QUEUE_KIT_ENTRY_LINE_CAP" =~ ^[0-9]+$ && "$QUEUE_KIT_ENTRY_LINE_CAP" -gt 0 ]] \
    || _qk_errs+=("QUEUE_KIT_ENTRY_LINE_CAP must be a positive integer (got '$QUEUE_KIT_ENTRY_LINE_CAP')")
[[ "$QUEUE_KIT_ICEBOX_AGE_DAYS" =~ ^[0-9]+$ && "$QUEUE_KIT_ICEBOX_AGE_DAYS" -gt 0 ]] \
    || _qk_errs+=("QUEUE_KIT_ICEBOX_AGE_DAYS must be a positive integer (got '$QUEUE_KIT_ICEBOX_AGE_DAYS')")
[[ "$QUEUE_KIT_ICEBOX_SECTION" != "$QUEUE_KIT_DEFERRED_SECTION" ]] \
    || _qk_errs+=("QUEUE_KIT_ICEBOX_SECTION must not name the deferred section")
[[ -n "$QUEUE_KIT_PRECONDITION_REGEX" ]] || _qk_errs+=("QUEUE_KIT_PRECONDITION_REGEX is empty")
[[ ${#QUEUE_KIT_REQUIRED_SECTIONS[@]} -gt 0 ]] || _qk_errs+=("QUEUE_KIT_REQUIRED_SECTIONS is empty")
[[ -n "$QUEUE_KIT_ROADMAP_MARKER" ]] || _qk_errs+=("QUEUE_KIT_ROADMAP_MARKER is empty")
# spec: queue-kit/SPEC.md §Layout and configuration — the roadmap vocabulary is all-or-nothing: one array set while the other is empty would accept every value of the unconfigured field, so a half-configured pair is malformed config, not a lenient default
if [[ ${#QUEUE_KIT_HORIZONS[@]} -gt 0 && ${#QUEUE_KIT_TRACKS[@]} -eq 0 ]]; then
    _qk_errs+=("QUEUE_KIT_HORIZONS is set but QUEUE_KIT_TRACKS is empty — the roadmap vocabulary is configured as a pair")
fi
if [[ ${#QUEUE_KIT_TRACKS[@]} -gt 0 && ${#QUEUE_KIT_HORIZONS[@]} -eq 0 ]]; then
    _qk_errs+=("QUEUE_KIT_TRACKS is set but QUEUE_KIT_HORIZONS is empty — the roadmap vocabulary is configured as a pair")
fi
if [[ ${#_qk_errs[@]} -gt 0 ]]; then
    printf 'queue-kit: malformed queue config — the gates cannot run:\n' >&2
    printf '  %s\n' "${_qk_errs[@]}" >&2
    exit 2
fi
unset _qk_errs
