# shellcheck shell=bash
# spec: canon-kit/SPEC.md §lib/spec.sh — sourced config loader + shared section/spec adapters, never gate structure

_sk_cfg="${CANON_KIT_CONFIG_FILE:-}"
if [[ -n "$_sk_cfg" ]]; then
    [[ -f "$_sk_cfg" ]] || {
        echo "canon-kit: CANON_KIT_CONFIG_FILE not found: $_sk_cfg" >&2
        exit 2
    }
    # shellcheck disable=SC1090  # consumer-supplied config, path is config
    source "$_sk_cfg"
else
    _sk_cfg="${GATE_SDK_GATES_DIR:-scripts}/canon-config.sh"
    if [[ -f "$_sk_cfg" ]]; then
        # shellcheck disable=SC1090  # consumer-supplied config, path is config
        source "$_sk_cfg"
    fi
fi
unset _sk_cfg

[[ -v CANON_KIT_SPEC_NAME ]]      || CANON_KIT_SPEC_NAME="SPEC.md"
[[ -v CANON_KIT_AMENDMENT_GLOB ]] || CANON_KIT_AMENDMENT_GLOB="SPEC-*.md"

[[ -v CANON_KIT_QUEUE_FILE ]] || CANON_KIT_QUEUE_FILE="${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}"

declare -p CANON_KIT_DEPRECATION_MARKERS &>/dev/null || CANON_KIT_DEPRECATION_MARKERS=()

declare -p CANON_KIT_FEATURE_SECTIONS &>/dev/null || CANON_KIT_FEATURE_SECTIONS=("New Features")
declare -p CANON_KIT_ACTIVE_SECTIONS  &>/dev/null || CANON_KIT_ACTIVE_SECTIONS=("New Features" "Technical Debt")

[[ -v CANON_KIT_DEFERRED_SECTION ]] || CANON_KIT_DEFERRED_SECTION="Deferred"
[[ -v CANON_KIT_ICEBOX_SECTION ]]   || CANON_KIT_ICEBOX_SECTION=""

[[ -v CANON_KIT_DOD_HEADING ]] || CANON_KIT_DOD_HEADING="Definition of Done"
[[ -v CANON_KIT_DOD_MODE ]]    || CANON_KIT_DOD_MODE="exactly-one"

[[ -v CANON_KIT_SCAN_KIT_ROOTS ]] || CANON_KIT_SCAN_KIT_ROOTS=0

declare -p CANON_KIT_BANNED_HEADINGS &>/dev/null \
    || CANON_KIT_BANNED_HEADINGS=("Directory Structure" "Public API" "Cargo.toml Dependencies")
[[ -v CANON_KIT_DERIVABLE_DENSITY ]]       || CANON_KIT_DERIVABLE_DENSITY=60
[[ -v CANON_KIT_DERIVABLE_POINTER_REGEX ]] || CANON_KIT_DERIVABLE_POINTER_REGEX='pub-index|proto-index'

[[ -v CANON_KIT_EMBED_THRESHOLD ]] || CANON_KIT_EMBED_THRESHOLD="0.70"
[[ -v CANON_KIT_EMBED_MINLINES ]]  || CANON_KIT_EMBED_MINLINES=8
declare -p CANON_KIT_EMBED_LANGS &>/dev/null || CANON_KIT_EMBED_LANGS=(
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
declare -p CANON_KIT_EMBED_ILLUSTRATIVE &>/dev/null || CANON_KIT_EMBED_ILLUSTRATIVE=("json")
[[ -v CANON_KIT_EMBED_WIRE_KIND ]] || CANON_KIT_EMBED_WIRE_KIND="proto"

[[ -v CANON_KIT_GLOSSARY_FILE ]] || CANON_KIT_GLOSSARY_FILE="GLOSSARY.md"
declare -p CANON_KIT_DUP_SURFACES &>/dev/null || CANON_KIT_DUP_SURFACES=("VISION.md")

declare -p CANON_KIT_MDREF_EXCLUDE &>/dev/null || CANON_KIT_MDREF_EXCLUDE=()

[[ -v CANON_KIT_LINK_ROOT ]] || CANON_KIT_LINK_ROOT="docs"

[[ -v CANON_KIT_DOCS_BLOB_REF ]] || CANON_KIT_DOCS_BLOB_REF="master"

declare -p CANON_KIT_MANIFEST_FILES &>/dev/null || CANON_KIT_MANIFEST_FILES=()
declare -p CANON_KIT_PROSE_SURFACE_GLOBS &>/dev/null || CANON_KIT_PROSE_SURFACE_GLOBS=()
declare -p CANON_KIT_TEMPORAL_MARKERS &>/dev/null || CANON_KIT_TEMPORAL_MARKERS=(
    "previously"
    "formerly"
    "renamed from"
    "no longer"
    "used to be"
    "was (retired|removed|renamed|replaced)"
)
declare -p CANON_KIT_TEMPORAL_EXEMPT_SECTIONS &>/dev/null || CANON_KIT_TEMPORAL_EXEMPT_SECTIONS=()
declare -p CANON_KIT_TEMPORAL_EXEMPT_PATHS &>/dev/null || CANON_KIT_TEMPORAL_EXEMPT_PATHS=()

declare -p CANON_KIT_COUNT_COLLECTIONS &>/dev/null || CANON_KIT_COUNT_COLLECTIONS=(
    "gates"
    "meta-gates"
    "checks"
    "kits"
    "stages"
    "rules"
    "KPIs"
)
declare -p CANON_KIT_COUNT_ALLOWED_PHRASES &>/dev/null || CANON_KIT_COUNT_ALLOWED_PHRASES=()
[[ -v CANON_KIT_COUNT_WEDGE_WORDS ]] || CANON_KIT_COUNT_WEDGE_WORDS=2

[[ -v CANON_KIT_ENUM_SETS_CMD ]] || CANON_KIT_ENUM_SETS_CMD=""

[[ -v CANON_KIT_INSTALL_TRANSPORTS_CMD ]] || CANON_KIT_INSTALL_TRANSPORTS_CMD=""
[[ -v CANON_KIT_INSTALL_SECTION_RE ]]     || CANON_KIT_INSTALL_SECTION_RE=""
declare -p CANON_KIT_INSTALL_CLAIM_EXCLUDE &>/dev/null || CANON_KIT_INSTALL_CLAIM_EXCLUDE=()

[[ -v CANON_KIT_PAYLOAD_CLAIMS_CMD ]] || CANON_KIT_PAYLOAD_CLAIMS_CMD=""
declare -p CANON_KIT_PAYLOAD_CLAIM_EXCLUDE &>/dev/null || CANON_KIT_PAYLOAD_CLAIM_EXCLUDE=()

[[ -v CANON_KIT_MEASURED_CLAIMS_CMD ]] || CANON_KIT_MEASURED_CLAIMS_CMD=""
declare -p CANON_KIT_MEASURED_SURFACE_GLOBS &>/dev/null || CANON_KIT_MEASURED_SURFACE_GLOBS=()

[[ -v CANON_KIT_CLAIM_CLASSES_CMD ]] || CANON_KIT_CLAIM_CLASSES_CMD=""

declare -p CANON_KIT_COMMENT_MACHINE &>/dev/null || CANON_KIT_COMMENT_MACHINE=()
declare -p CANON_KIT_COMMENT_REASON  &>/dev/null || CANON_KIT_COMMENT_REASON=()
declare -p CANON_KIT_COMMENT_SURFACE &>/dev/null || CANON_KIT_COMMENT_SURFACE=()
declare -p CANON_KIT_COMMENT_POSITIONAL &>/dev/null || CANON_KIT_COMMENT_POSITIONAL=()
declare -p CANON_KIT_COMMENT_WHITELIST &>/dev/null || CANON_KIT_COMMENT_WHITELIST=()
[[ -v CANON_KIT_COMMENT_RUN_CAP ]] || CANON_KIT_COMMENT_RUN_CAP=3

declare -p CANON_KIT_PROSE_TELL_GLOBS &>/dev/null || CANON_KIT_PROSE_TELL_GLOBS=()
[[ -v CANON_KIT_PROSE_TELL_EMDASH_MAX ]]       || CANON_KIT_PROSE_TELL_EMDASH_MAX=2
[[ -v CANON_KIT_PROSE_TELL_CONTRAST_MAX ]]     || CANON_KIT_PROSE_TELL_CONTRAST_MAX=1
[[ -v CANON_KIT_PROSE_TELL_RHYTHM_MIN_SENTENCES ]] || CANON_KIT_PROSE_TELL_RHYTHM_MIN_SENTENCES=4
[[ -v CANON_KIT_PROSE_TELL_RHYTHM_CV_MIN ]]    || CANON_KIT_PROSE_TELL_RHYTHM_CV_MIN="0.25"
[[ -v CANON_KIT_PROSE_TELL_TRICOLON_MAX ]]     || CANON_KIT_PROSE_TELL_TRICOLON_MAX=2
declare -p CANON_KIT_PROSE_TELL_PHRASES &>/dev/null || CANON_KIT_PROSE_TELL_PHRASES=(
    "It's worth noting"
    "It is worth noting"
    "It's important to note"
    "That said"
    "Needless to say"
    "It goes without saying"
)
declare -p CANON_KIT_PROSE_TELL_ABBR_ALLOW &>/dev/null || CANON_KIT_PROSE_TELL_ABBR_ALLOW=(
    "API" "CLI" "URL" "HTML" "CSS" "JSON" "YAML" "CI" "SDK" "SSO" "DNS" "HTTPS"
)

# spec: canon-kit/SPEC.md §Layout and configuration — the _EXTRA arrays union onto the base sets after the base defaults resolve, so a consumer adding a token need not restate the bundled default; replacing a base array stays the narrowing valve
declare -p CANON_KIT_PROSE_TELL_PHRASES_EXTRA &>/dev/null || CANON_KIT_PROSE_TELL_PHRASES_EXTRA=()
declare -p CANON_KIT_PROSE_TELL_ABBR_ALLOW_EXTRA &>/dev/null || CANON_KIT_PROSE_TELL_ABBR_ALLOW_EXTRA=()
declare -p CANON_KIT_TEMPORAL_MARKERS_EXTRA &>/dev/null || CANON_KIT_TEMPORAL_MARKERS_EXTRA=()
CANON_KIT_PROSE_TELL_PHRASES+=(${CANON_KIT_PROSE_TELL_PHRASES_EXTRA[@]+"${CANON_KIT_PROSE_TELL_PHRASES_EXTRA[@]}"})
CANON_KIT_PROSE_TELL_ABBR_ALLOW+=(${CANON_KIT_PROSE_TELL_ABBR_ALLOW_EXTRA[@]+"${CANON_KIT_PROSE_TELL_ABBR_ALLOW_EXTRA[@]}"})
CANON_KIT_TEMPORAL_MARKERS+=(${CANON_KIT_TEMPORAL_MARKERS_EXTRA[@]+"${CANON_KIT_TEMPORAL_MARKERS_EXTRA[@]}"})

spec_alt() { local IFS='|'; printf '%s' "$*"; }

# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
SPEC_FEATURE_RE="^## ($(spec_alt "${CANON_KIT_FEATURE_SECTIONS[@]}"))[[:space:]]*$"
# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
SPEC_ACTIVE_RE="^## ($(spec_alt "${CANON_KIT_ACTIVE_SECTIONS[@]}"))[[:space:]]*$"
# spec: canon-kit/SPEC.md §The amendment lifecycle — the design-pending regex is the section *set*, deferred plus a configured icebox, so the tag's section-wide rule and the queue-resolution walk reach both with no per-gate edit; the icebox term is omitted when the knob is empty
_sk_dp=("$CANON_KIT_DEFERRED_SECTION")
[[ -n "$CANON_KIT_ICEBOX_SECTION" ]] && _sk_dp+=("$CANON_KIT_ICEBOX_SECTION")
# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
SPEC_DEFERRED_RE="^## ($(spec_alt "${_sk_dp[@]}"))[[:space:]]*$"
unset _sk_dp
# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
SPEC_SECTION_RE="^## "

# spec: canon-kit/SPEC.md §lib/spec.sh — lexical normalisation of an absolute path, the shell
#   twin of the crate's `walk::normalize_abs`: an empty or `.` component drops, `..` pops, and
#   no symlink resolves. Every path the prune below compares crosses it, or none do.
_spec_norm_abs() {   # sets _SPEC_NORM
    local p="${1%/}/" seg out=""
    while [[ -n "$p" ]]; do
        seg="${p%%/*}"; p="${p#*/}"
        case "$seg" in
            ''|.) ;;
            ..)   out="${out%/*}" ;;
            *)    out="$out/$seg" ;;
        esac
    done
    _SPEC_NORM="${out:-/}"
}

# spec: canon-kit/SPEC.md §lib/spec.sh — finders skip templates/ stubs and vendored kit roots under the scan root (an ancestor kit root never prunes)
_spec_prune_kit_roots() {
    if [[ "$CANON_KIT_SCAN_KIT_ROOTS" == "1" ]]; then cat; return 0; fi
    local root="${1:-.}" root_abs
    case "$root" in
        /*)  root_abs="$root" ;;
        .)   root_abs="$PWD" ;;
        ./*) root_abs="$PWD/${root#./}" ;;
        *)   root_abs="$PWD/$root" ;;
    esac
    _spec_norm_abs "$root_abs"; root_abs="$_SPEC_NORM"
    local -a roots=()
    local r rabs f fabs keep
    while IFS= read -r r; do
        [[ -n "$r" ]] || continue
        [[ "$r" == /* ]] && rabs="$r" || rabs="$PWD/$r"
        _spec_norm_abs "$rabs"; rabs="$_SPEC_NORM"
        [[ "$rabs" == "$root_abs/"* ]] || continue   # only a vendored subtree prunes
        roots+=("$rabs")
    done < <(gate_kit_roots)
    [[ ${#roots[@]} -eq 0 ]] && { cat; return 0; }
    while IFS= read -r f; do
        [[ -n "$f" ]] || continue
        [[ "$f" == /* ]] && fabs="$f" || fabs="$PWD/${f#./}"
        _spec_norm_abs "$fabs"; fabs="$_SPEC_NORM"
        keep=1
        for r in "${roots[@]}"; do
            [[ "$fabs" == "$r/"* ]] && { keep=0; break; }
        done
        [[ "$keep" == "1" ]] && printf '%s\n' "$f"
    done
}

spec_canonical_specs() { gate_find "$1" -name "$CANON_KIT_SPEC_NAME" -type f 2>/dev/null | grep -v '/templates/' | _spec_prune_kit_roots "$1" || true; }

spec_amendments() { gate_find "$1" -name "$CANON_KIT_AMENDMENT_GLOB" -type f 2>/dev/null | grep -v '/templates/' | _spec_prune_kit_roots "$1" || true; }

# spec: canon-kit/SPEC.md §check-spec-pointer — a prose-surface candidate joins the
#   manifest set iff slot-free: no `*<name: …>*` binding slot (the grammar
#   lifecycle-kit/SPEC.md §templates/stages/ owns), no `CONSUMER BINDING` header.
_spec_slot_free() { ! grep -qE '\*<[a-z][a-z0-9-]*:|^CONSUMER BINDING' -- "$1"; }

# spec: canon-kit/SPEC.md §lib/spec.sh — the manifest set shared by the manifest-narration gate family: canonical specs and README.md at any depth (both kit-root pruned per CANON_KIT_SCAN_KIT_ROOTS — a vendored kit's own README is its documentation, not the consumer's governed content) plus CLAUDE.md; explicit globs when CANON_KIT_MANIFEST_FILES is set. Amendments are excluded by construction — a transition artifact describes change. Slot-free CANON_KIT_PROSE_SURFACE_GLOBS candidates join the set (canon-kit/SPEC.md §check-spec-pointer).
spec_manifest_files() {
    local root="${1:-.}" g f
    if [[ ${#CANON_KIT_MANIFEST_FILES[@]} -gt 0 ]]; then
        shopt -s nullglob globstar
        for g in "${CANON_KIT_MANIFEST_FILES[@]}"; do
            for f in "$root"/$g; do [[ -f "$f" ]] && printf '%s\n' "$f"; done
        done
        shopt -u nullglob globstar
    else
        spec_canonical_specs "$root"
        gate_find "$root" -name 'README.md' -type f 2>/dev/null | grep -v '/templates/' | _spec_prune_kit_roots "$root" || true
        gate_find "$root" -name 'CLAUDE.md' -type f 2>/dev/null || true
    fi
    if [[ ${#CANON_KIT_PROSE_SURFACE_GLOBS[@]} -gt 0 ]]; then
        shopt -s nullglob globstar
        for g in "${CANON_KIT_PROSE_SURFACE_GLOBS[@]}"; do
            for f in "$root"/$g; do
                [[ -f "$f" ]] && _spec_slot_free "$f" && printf '%s\n' "$f"
            done
        done
        shopt -u nullglob globstar
    fi
}

# spec: canon-kit/SPEC.md §lib/spec.sh — the count grammar the restated-total gate family shares: one cardinal alternation, one consumer noun vocabulary, two match shapes
SPEC_COUNT_CARDINAL_RE='([0-9]+|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)'

spec_count_noun_alt() {
    local n out=""
    for n in "${CANON_KIT_COUNT_COLLECTIONS[@]}"; do
        [[ -n "$n" ]] || continue
        out="${out:+$out|}${n,,}"
    done
    printf '%s' "$out"
}

# spec: canon-kit/SPEC.md §check-manifest-count — the wedge groups are optional, so bare adjacency is this shape's zero-wedge case rather than a second branch
spec_count_quantifier_re() {
    local i opt=""
    for ((i = 0; i < CANON_KIT_COUNT_WEDGE_WORDS; i++)); do
        opt+='([[:space:]]+[[:alnum:]_-]+)?'
    done
    printf '%s%s[[:space:]]+(%s)' "$SPEC_COUNT_CARDINAL_RE" "$opt" "$(spec_count_noun_alt)"
}

spec_count_range_re() {
    printf '(%s)[[:space:]]+[0-9]+-[0-9]+' "$(spec_count_noun_alt)"
}

# spec: canon-kit/SPEC.md §lib/spec.sh — the awk half of the count adapter: the matcher both restated-total gates prepend, so the boundary rule and the mechanical exemptions have one home. Callers pass SK_QRE, SK_RRE and SK_PHRASES; sk_count_hit returns the offending span, or "".
spec_count_awk_lib() {
    cat <<'AWK'
BEGIN { SK_NP = split(SK_PHRASES, SK_PHRASE, "\n") }
function _sk_phrase_exempt(low, ms, me,   i, p, lp, start, idx, pp) {
    for (i = 1; i <= SK_NP; i++) {
        p = SK_PHRASE[i]; if (p == "") continue
        lp = length(p); start = 1
        while (1) {
            idx = index(substr(low, start), p)
            if (idx == 0) break
            pp = start + idx - 1
            if (ms >= pp && me <= pp + lp - 1) return 1
            start = pp + 1
        }
    }
    return 0
}
function _sk_span(low, scan, re, quantifier,   rest, off, ms, ml, me, bc, ac, ok, m, prefix, suffix) {
    rest = low; off = 0
    while (match(rest, re) > 0) {
        ms = off + RSTART; ml = RLENGTH; me = ms + ml - 1
        bc = (ms > 1) ? substr(low, ms - 1, 1) : " "
        ac = (me < length(low)) ? substr(low, me + 1, 1) : " "
        ok = 1
        if (bc ~ /[[:alnum:]]/) ok = 0          # match glued to a preceding word or number
        if (ac ~ /[[:alnum:]-]/) ok = 0         # noun glued to a following word (e.g. gatekeepers)
        if (ok) {
            m = substr(low, ms, ml)
            prefix = substr(low, 1, ms - 1)
            suffix = substr(low, me + 1)
            if (_sk_phrase_exempt(low, ms, me)) ok = 0
            else if (quantifier) {
                if (prefix ~ /(≥|≤|>|<|at least|at most|up to|more than|fewer than)[[:space:]]*$/) ok = 0
                else if (prefix ~ /all but[[:space:]]*$/) ok = 0
                else if (prefix ~ /(^|[^[:alnum:]])of[[:space:]]+(the[[:space:]]+)?$/) ok = 0
                else if (m ~ /(^|[[:space:]])of([[:space:]]|$)/) ok = 0
                else if (suffix ~ /^[[:space:]]+per([[:space:]]|$)/) ok = 0
            }
        }
        if (ok) return substr(scan, ms, ml)
        off = ms; rest = substr(low, ms + 1)
    }
    return ""
}
function sk_count_hit(text,   scan, low, s) {
    scan = text
    gsub(/`[^`]*`/, "", scan)   # a cardinal in inline code is a meta-reference, not a restated total
    low = tolower(scan)
    s = _sk_span(low, scan, SK_QRE, 1)
    if (s != "") return s
    return _sk_span(low, scan, SK_RRE, 0)
}
# the paragraph-join window over the walk driver's accumulator (sk_para_add,
# sk_pline/_sk_join): sk_count_hit sees one physical line, so a total whose
# cardinal and noun straddle a prose wrap ("two comment /\ngates") slips both
# gates. sk_para_wrapped reads back the first total whose span crosses a line
# boundary, at the span's first physical line (SK_WRAP_FNR/SK_WRAP_SPAN). A
# same-line span returns 0 here — the per-line scan owns it, so no double report.
function sk_para_wrapped(   k, hit, compK, startK) {
    SK_WRAP_FNR = 0; SK_WRAP_SPAN = ""
    if (sk_pn < 2) return 0
    compK = 0                                       # line where the first span completes
    for (k = 1; k <= sk_pn; k++) { hit = sk_count_hit(_sk_join(1, k)); if (hit != "") { compK = k; break } }
    if (compK == 0) return 0
    startK = 1                                      # largest start whose suffix-join still hits: the span's first line
    for (k = 2; k <= compK; k++) { if (sk_count_hit(_sk_join(k, compK)) != "") startK = k; else break }
    if (startK == compK) return 0                   # span sits on one physical line, per-line scan owns it
    SK_WRAP_FNR = sk_pfnr[startK]; SK_WRAP_SPAN = hit
    return 1
}
AWK
}

spec_count_phraselist() {
    [[ ${#CANON_KIT_COUNT_ALLOWED_PHRASES[@]} -eq 0 ]] && return 0
    printf '%s\n' "${CANON_KIT_COUNT_ALLOWED_PHRASES[@]}" | tr '[:upper:]' '[:lower:]'
}

# spec: canon-kit/SPEC.md §lib/spec.sh — the paragraph accumulator every manifest-prose gate shares: sk_para_add feeds physical lines, _sk_join reads back a logical window. Both walk drivers (the shared one and check-comment-tier's caller-owned comment walk) fill it; sk_para_wrapped and the enum paragraph read it.
spec_para_accum_awk() {
    cat <<'AWK'
function sk_para_reset() { sk_pn = 0 }
function sk_para_add(fnr, text) { sk_pn++; sk_pfnr[sk_pn] = fnr; sk_pline[sk_pn] = text }
function _sk_join(lo, hi,   k, s) {
    s = ""
    for (k = lo; k <= hi; k++) s = s (k > lo ? " " : "") sk_pline[k]
    return s
}
AWK
}

# spec: canon-kit/SPEC.md §lib/spec.sh — the manifest-prose walk driver both restated-manifest gates prepend: fence tracking, the blank-line paragraph reset, and the per-site exempt window (the line or the one above; the marker regex arrives in SK_EXEMPT). It calls the caller's sk_on_line(file,fnr,raw) per prose line and sk_on_pflush() at each paragraph boundary, over the shared accumulator (spec_para_accum_awk).
spec_manifest_walk_awk() {
    cat <<'AWK'
function _sk_pflush() { sk_on_pflush(); sk_para_reset() }
FNR == 1 { _sk_pflush(); sk_fence = 0; sk_prev = "" }
{
    sk_curfile = FILENAME
    sk_raw = $0
    if (sk_raw ~ /^[[:space:]]*```/) { _sk_pflush(); sk_fence = !sk_fence; sk_prev = sk_raw; next }
    if (sk_fence) { _sk_pflush(); sk_prev = sk_raw; next }
    if (sk_raw ~ SK_EXEMPT || sk_prev ~ SK_EXEMPT) { _sk_pflush(); sk_prev = sk_raw; next }
    if (sk_raw ~ /^[[:space:]]*$/) { _sk_pflush(); sk_prev = sk_raw; next }
    sk_on_line(sk_curfile, FNR, sk_raw)
    sk_para_add(FNR, sk_raw)
    sk_prev = sk_raw
}
END { _sk_pflush() }
AWK
}

# spec: canon-kit/SPEC.md §lib/spec.sh — the default-statement grammar the knob gates share, one owner so neither gate re-implements it. sk_literal_at(after) returns the value literal opening the window (a backticked non-knob string, a quoted string, or a number), or "". sk_default_literal(line, win) returns the literal the word "default" binds within a forward window of `win` chars, or "". The two bool wrappers (sk_after_has_literal / sk_default_bound at check-knob-citation's 24-char prose window) preserve that gate's leg. The caller supplies sk_is_knobname(token) — check-knob-citation resolves it against its prefix roster, check-knob-default-coupling against its own — so the "a bare knob name after default is a name citation, not a value" rule has one home.
spec_default_grammar_awk() {
    cat <<'AWK'
function sk_literal_at(after,   content) {
    if (match(after, /`[^`]+`/) > 0) {
        content = substr(after, RSTART + 1, RLENGTH - 2)
        gsub(/^[ \t]+|[ \t]+$/, "", content)
        if (!sk_is_knobname(content)) return content
    }
    if (match(after, /"[^"]*"/) > 0 && RLENGTH > 2) return substr(after, RSTART + 1, RLENGTH - 2)
    if (match(after, /'[^']*'/) > 0 && RLENGTH > 2) return substr(after, RSTART + 1, RLENGTH - 2)
    if (match(after, /(^|[^A-Za-z0-9_])[0-9]+([^A-Za-z0-9_]|$)/) > 0) {
        content = substr(after, RSTART, RLENGTH); gsub(/[^0-9]/, "", content); return content
    }
    return ""
}
function sk_after_has_literal(after) { return (sk_literal_at(after) != "") }
function sk_default_literal(line, win,   low, off, ms, me, lit) {
    low = tolower(line); off = 0
    while (match(substr(low, off + 1), /(^|[^a-z0-9_])default/) > 0) {
        ms = off + RSTART; me = ms + RLENGTH - 1
        lit = sk_literal_at(substr(line, me + 1, win))
        if (lit != "") return lit
        off = ms + 1
    }
    return ""
}
function sk_default_bound(line) { return (sk_default_literal(line, 24) != "") }
AWK
}

# spec: canon-kit/SPEC.md §check-prose-enum — run the consumer's declared sets command and echo its validated <set-name><TAB><member> lines; a command that fails or a line that does not parse (no tab, empty field, extra tab) returns 2 (fail-closed). Empty CANON_KIT_ENUM_SETS_CMD is the caller's clean-skip signal, handled before this is called.
spec_enum_sets() {
    local out st line name member
    out="$(bash -c "$CANON_KIT_ENUM_SETS_CMD")"; st=$?
    [[ $st -eq 0 ]] || { echo "spec_enum_sets: CANON_KIT_ENUM_SETS_CMD exited $st" >&2; return 2; }
    while IFS= read -r line; do
        [[ -n "$line" ]] || continue
        [[ "$line" == *$'\t'* ]] || { echo "spec_enum_sets: line has no tab: '$line'" >&2; return 2; }
        name="${line%%$'\t'*}"; member="${line#*$'\t'}"
        [[ -n "$name" && -n "$member" ]] || { echo "spec_enum_sets: empty set name or member: '$line'" >&2; return 2; }
        [[ "$member" == *$'\t'* ]] && { echo "spec_enum_sets: extra tab in line: '$line'" >&2; return 2; }
        printf '%s\t%s\n' "$name" "$member"
    done <<< "$out"
    return 0
}

# spec: canon-kit/SPEC.md §lib/spec.sh — run a consumer's claim-vocabulary command and echo its validated <id><TAB><ERE> lines; a command that fails, a line that does not parse, an id that is not slug-shaped, or a repeated id returns 2 (fail-closed), the same contract spec_enum_sets carries. $2 labels the vocabulary in every message, so a fail-closed exit says which one failed. An empty command is the caller's clean-skip signal, handled before this is called.
spec_claim_vocabulary() {  # $1=command  $2=label naming the vocabulary in messages
    local cmd="$1" label="$2" out st line id ere seen=""
    out="$(bash -c "$cmd")"; st=$?
    [[ $st -eq 0 ]] || { echo "spec_claim_vocabulary: $label exited $st" >&2; return 2; }
    while IFS= read -r line; do
        [[ -n "$line" ]] || continue
        [[ "$line" == *$'\t'* ]] || { echo "spec_claim_vocabulary: $label: line has no tab: '$line'" >&2; return 2; }
        id="${line%%$'\t'*}"; ere="${line#*$'\t'}"
        [[ -n "$id" && -n "$ere" ]] || { echo "spec_claim_vocabulary: $label: empty id or pattern: '$line'" >&2; return 2; }
        [[ "$ere" == *$'\t'* ]] && { echo "spec_claim_vocabulary: $label: extra tab in line: '$line'" >&2; return 2; }
        [[ "$id" =~ ^[a-z0-9][a-z0-9-]*$ ]] || { echo "spec_claim_vocabulary: $label: id is not slug-shaped: '$id'" >&2; return 2; }
        [[ " $seen " == *" $id "* ]] && { echo "spec_claim_vocabulary: $label: duplicate id: '$id'" >&2; return 2; }
        seen="$seen $id"
        printf '%s\t%s\n' "$id" "$ere"
    done <<< "$out"
    return 0
}

# spec: canon-kit/SPEC.md §check-install-claim — the install-transport vocabulary, one caller of the general loader above; the fail-closed contract is that function's and is not restated here.
spec_install_transports() {
    spec_claim_vocabulary "$CANON_KIT_INSTALL_TRANSPORTS_CMD" CANON_KIT_INSTALL_TRANSPORTS_CMD
}

# spec: canon-kit/SPEC.md §check-measured-claim — the measured-claim oracle's roster, the third caller of the general loader above; a key is slug-shaped and a value is one non-empty tab-free field, which is that function's contract rather than a second one here.
spec_measured_claims() {
    spec_claim_vocabulary "$CANON_KIT_MEASURED_CLAIMS_CMD" CANON_KIT_MEASURED_CLAIMS_CMD
}

_sk_errs=()
[[ -n "$CANON_KIT_SPEC_NAME" ]]      || _sk_errs+=("CANON_KIT_SPEC_NAME is empty")
[[ -n "$CANON_KIT_AMENDMENT_GLOB" ]] || _sk_errs+=("CANON_KIT_AMENDMENT_GLOB is empty")
[[ ${#CANON_KIT_FEATURE_SECTIONS[@]} -gt 0 ]] || _sk_errs+=("CANON_KIT_FEATURE_SECTIONS is empty")
[[ ${#CANON_KIT_ACTIVE_SECTIONS[@]} -gt 0 ]]  || _sk_errs+=("CANON_KIT_ACTIVE_SECTIONS is empty")
[[ -n "$CANON_KIT_DEFERRED_SECTION" ]] || _sk_errs+=("CANON_KIT_DEFERRED_SECTION is empty")
[[ "$CANON_KIT_ICEBOX_SECTION" != "$CANON_KIT_DEFERRED_SECTION" ]] \
    || _sk_errs+=("CANON_KIT_ICEBOX_SECTION must not name the deferred section")
[[ -n "$CANON_KIT_DOD_HEADING" ]]      || _sk_errs+=("CANON_KIT_DOD_HEADING is empty")
[[ "$CANON_KIT_DOD_MODE" == "exactly-one" || "$CANON_KIT_DOD_MODE" == "at-most-one" ]] \
    || _sk_errs+=("CANON_KIT_DOD_MODE must be exactly-one|at-most-one (got '$CANON_KIT_DOD_MODE')")
[[ "$CANON_KIT_SCAN_KIT_ROOTS" == "0" || "$CANON_KIT_SCAN_KIT_ROOTS" == "1" ]] \
    || _sk_errs+=("CANON_KIT_SCAN_KIT_ROOTS must be 0|1 (got '$CANON_KIT_SCAN_KIT_ROOTS')")
[[ "$CANON_KIT_DERIVABLE_DENSITY" =~ ^[0-9]+$ && "$CANON_KIT_DERIVABLE_DENSITY" -ge 0 && "$CANON_KIT_DERIVABLE_DENSITY" -le 100 ]] \
    || _sk_errs+=("CANON_KIT_DERIVABLE_DENSITY must be 0..100 (got '$CANON_KIT_DERIVABLE_DENSITY')")
[[ "$CANON_KIT_EMBED_THRESHOLD" =~ ^0?\.[0-9]+$|^1(\.0+)?$ ]] \
    || _sk_errs+=("CANON_KIT_EMBED_THRESHOLD must be a 0..1 fraction (got '$CANON_KIT_EMBED_THRESHOLD')")
[[ "$CANON_KIT_EMBED_MINLINES" =~ ^[0-9]+$ && "$CANON_KIT_EMBED_MINLINES" -gt 0 ]] \
    || _sk_errs+=("CANON_KIT_EMBED_MINLINES must be a positive integer (got '$CANON_KIT_EMBED_MINLINES')")
[[ -n "$CANON_KIT_GLOSSARY_FILE" ]] || _sk_errs+=("CANON_KIT_GLOSSARY_FILE is empty")
[[ -n "$CANON_KIT_DOCS_BLOB_REF" ]] || _sk_errs+=("CANON_KIT_DOCS_BLOB_REF is empty")
[[ ${#CANON_KIT_TEMPORAL_MARKERS[@]} -gt 0 ]] || _sk_errs+=("CANON_KIT_TEMPORAL_MARKERS is empty")
[[ ${#CANON_KIT_COUNT_COLLECTIONS[@]} -gt 0 ]] || _sk_errs+=("CANON_KIT_COUNT_COLLECTIONS is empty")
[[ "$CANON_KIT_COUNT_WEDGE_WORDS" =~ ^[0-9]+$ && "$CANON_KIT_COUNT_WEDGE_WORDS" -gt 0 ]] \
    || _sk_errs+=("CANON_KIT_COUNT_WEDGE_WORDS must be a positive integer (got '$CANON_KIT_COUNT_WEDGE_WORDS')")
[[ "$CANON_KIT_COMMENT_RUN_CAP" =~ ^[0-9]+$ && "$CANON_KIT_COMMENT_RUN_CAP" -gt 0 ]] \
    || _sk_errs+=("CANON_KIT_COMMENT_RUN_CAP must be a positive integer (got '$CANON_KIT_COMMENT_RUN_CAP')")
if [[ ${#_sk_errs[@]} -gt 0 ]]; then
    printf 'canon-kit: malformed spec config — the gates cannot run:\n' >&2
    printf '  %s\n' "${_sk_errs[@]}" >&2
    exit 2
fi
unset _sk_errs

# spec: canon-kit/SPEC.md §lib/spec.sh — the emitter-backed vocabularies as bridgeable
# variables, so a compiled member receives the consumer command's *output* and never spawns
# an interpreter to read consumer config
# spec: gate-sdk/SPEC.md §lib/gate.sh — index-aligned arrays because the wire format's own
# separator is the tab, and gated on GATE_SDK_RESOLVING_KNOB because resolution runs a
# subprocess and this library is sourced once per declared knob per gate
declare -p CANON_KIT_ENUM_SET_NAMES &>/dev/null   || CANON_KIT_ENUM_SET_NAMES=()
declare -p CANON_KIT_ENUM_SET_MEMBERS &>/dev/null || CANON_KIT_ENUM_SET_MEMBERS=()
if [[ "${GATE_SDK_RESOLVING_KNOB:-}" == CANON_KIT_ENUM_SET_NAMES \
   || "${GATE_SDK_RESOLVING_KNOB:-}" == CANON_KIT_ENUM_SET_MEMBERS ]] \
   && [[ -n "$CANON_KIT_ENUM_SETS_CMD" ]]; then
    _sk_sets="$(spec_enum_sets)" || exit 2
    while IFS=$'\t' read -r _sk_n _sk_m; do
        [[ -n "$_sk_n" ]] || continue
        CANON_KIT_ENUM_SET_NAMES+=("$_sk_n")
        CANON_KIT_ENUM_SET_MEMBERS+=("$_sk_m")
    done <<<"$_sk_sets"
    unset _sk_sets _sk_n _sk_m
fi

declare -p CANON_KIT_MEASURED_KEYS &>/dev/null   || CANON_KIT_MEASURED_KEYS=()
declare -p CANON_KIT_MEASURED_VALUES &>/dev/null || CANON_KIT_MEASURED_VALUES=()
if [[ "${GATE_SDK_RESOLVING_KNOB:-}" == CANON_KIT_MEASURED_KEYS \
   || "${GATE_SDK_RESOLVING_KNOB:-}" == CANON_KIT_MEASURED_VALUES ]] \
   && [[ -n "$CANON_KIT_MEASURED_CLAIMS_CMD" ]]; then
    _sk_meas="$(spec_measured_claims)" || exit 2
    while IFS=$'\t' read -r _sk_k _sk_v; do
        [[ -n "$_sk_k" ]] || continue
        CANON_KIT_MEASURED_KEYS+=("$_sk_k")
        CANON_KIT_MEASURED_VALUES+=("$_sk_v")
    done <<<"$_sk_meas"
    unset _sk_meas _sk_k _sk_v
fi

declare -p CANON_KIT_INSTALL_TRANSPORT_IDS &>/dev/null      || CANON_KIT_INSTALL_TRANSPORT_IDS=()
declare -p CANON_KIT_INSTALL_TRANSPORT_PATTERNS &>/dev/null || CANON_KIT_INSTALL_TRANSPORT_PATTERNS=()
if [[ "${GATE_SDK_RESOLVING_KNOB:-}" == CANON_KIT_INSTALL_TRANSPORT_IDS \
   || "${GATE_SDK_RESOLVING_KNOB:-}" == CANON_KIT_INSTALL_TRANSPORT_PATTERNS ]] \
   && [[ -n "$CANON_KIT_INSTALL_TRANSPORTS_CMD" ]]; then
    _sk_tr="$(spec_install_transports)" || exit 2
    while IFS=$'\t' read -r _sk_i _sk_p; do
        [[ -n "$_sk_i" ]] || continue
        CANON_KIT_INSTALL_TRANSPORT_IDS+=("$_sk_i")
        CANON_KIT_INSTALL_TRANSPORT_PATTERNS+=("$_sk_p")
    done <<<"$_sk_tr"
    unset _sk_tr _sk_i _sk_p
fi

declare -p CANON_KIT_PAYLOAD_CLAIM_IDS &>/dev/null      || CANON_KIT_PAYLOAD_CLAIM_IDS=()
declare -p CANON_KIT_PAYLOAD_CLAIM_PATTERNS &>/dev/null || CANON_KIT_PAYLOAD_CLAIM_PATTERNS=()
if [[ "${GATE_SDK_RESOLVING_KNOB:-}" == CANON_KIT_PAYLOAD_CLAIM_IDS \
   || "${GATE_SDK_RESOLVING_KNOB:-}" == CANON_KIT_PAYLOAD_CLAIM_PATTERNS ]] \
   && [[ -n "$CANON_KIT_PAYLOAD_CLAIMS_CMD" ]]; then
    _sk_pc="$(spec_claim_vocabulary "$CANON_KIT_PAYLOAD_CLAIMS_CMD" CANON_KIT_PAYLOAD_CLAIMS_CMD)" || exit 2
    while IFS=$'\t' read -r _sk_c _sk_r; do
        [[ -n "$_sk_c" ]] || continue
        CANON_KIT_PAYLOAD_CLAIM_IDS+=("$_sk_c")
        CANON_KIT_PAYLOAD_CLAIM_PATTERNS+=("$_sk_r")
    done <<<"$_sk_pc"
    unset _sk_pc _sk_c _sk_r
fi

declare -p CANON_KIT_CLAIM_CLASS_IDS &>/dev/null      || CANON_KIT_CLAIM_CLASS_IDS=()
declare -p CANON_KIT_CLAIM_CLASS_PATTERNS &>/dev/null || CANON_KIT_CLAIM_CLASS_PATTERNS=()
if [[ "${GATE_SDK_RESOLVING_KNOB:-}" == CANON_KIT_CLAIM_CLASS_IDS \
   || "${GATE_SDK_RESOLVING_KNOB:-}" == CANON_KIT_CLAIM_CLASS_PATTERNS ]] \
   && [[ -n "$CANON_KIT_CLAIM_CLASSES_CMD" ]]; then
    _sk_cc="$(spec_claim_vocabulary "$CANON_KIT_CLAIM_CLASSES_CMD" CANON_KIT_CLAIM_CLASSES_CMD)" || exit 2
    while IFS=$'\t' read -r _sk_ci _sk_cp; do
        [[ -n "$_sk_ci" ]] || continue
        CANON_KIT_CLAIM_CLASS_IDS+=("$_sk_ci")
        CANON_KIT_CLAIM_CLASS_PATTERNS+=("$_sk_cp")
    done <<<"$_sk_cc"
    unset _sk_cc _sk_ci _sk_cp
fi
