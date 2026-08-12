# shellcheck shell=bash
# spec: gate-sdk/SPEC.md §lib/gate.sh — sourced library: values + adapters, never gate structure

# spec: gate-sdk/SPEC.md §Layout and configuration — auto-source the consumer config seam so a layout knob's override persists past the shell that set it; GATE_SDK_CONFIG_FILE wins, else <gates-dir>/gate-sdk-config.sh (GATE_SDK_GATES_DIR stays env-or-default — a config file cannot name its own directory)
_gate_sdk_config="${GATE_SDK_CONFIG_FILE:-}"
if [[ -n "$_gate_sdk_config" ]]; then
    [[ -f "$_gate_sdk_config" ]] || {
        echo "gate-sdk: GATE_SDK_CONFIG_FILE not found: $_gate_sdk_config" >&2
        exit 2
    }
    # shellcheck disable=SC1090  # consumer-supplied config, path is a knob
    source "$_gate_sdk_config"
else
    _gate_sdk_config="${GATE_SDK_GATES_DIR:-scripts}/gate-sdk-config.sh"
    if [[ -f "$_gate_sdk_config" ]]; then
        # shellcheck disable=SC1090  # consumer-supplied config, path is a knob
        source "$_gate_sdk_config"
    fi
fi
unset _gate_sdk_config

# spec: gate-sdk/SPEC.md §Fail-closed contract — non-zero capture status means the check could not run; exit 2, never a false clean
fail_closed() {
    if [[ "$1" -ne 0 ]]; then
        printf '%s: %s exited %s — the check could not run; treating as ' \
            "$2" "$3" "$1" >&2
        printf 'failure (not clean)\n' >&2
        exit 2
    fi
}

if [[ -n "${GATE_SDK_PRUNE_DIRS:-}" ]]; then
    read -r -a GATE_PRUNE_DIRS <<<"$GATE_SDK_PRUNE_DIRS"
else
    GATE_PRUNE_DIRS=(target .git node_modules .tmp gate-tests worktrees)
fi
# spec: gate-sdk/SPEC.md §lib/gate.sh — GATE_SDK_PRUNE_EXTRA_DIRS appends to the resolved set whichever branch produced it, so a consumer adds one directory without copying the default
for _gpx in ${GATE_SDK_PRUNE_EXTRA_DIRS:-}; do GATE_PRUNE_DIRS+=("$_gpx"); done
unset _gpx

# spec: gate-sdk/SPEC.md §lib/gate.sh — check-kit-registration's two document knobs, resolved here rather than inline in the check so the config bridge can carry them: a knob the owning kit's library does not define is the bridge's third refusal, so a compiled member declaring either would fail-close on every invocation. Resolved under their own consumer names (the queue-kit shape) rather than renamed the way GATE_PRUNE_DIRS is: that rename exists because a whitespace scalar feeds an array and one name would mean two grammars, and these are scalars in and scalars out.
[[ -v GATE_SDK_REGISTRY_DOC ]] || GATE_SDK_REGISTRY_DOC="README.md"
[[ -v GATE_SDK_RUNNER_DOC ]] || GATE_SDK_RUNNER_DOC="README.md"

gate_find() {
    local prune=() d
    for d in "${GATE_PRUNE_DIRS[@]}"; do prune+=(-name "$d" -o); done
    unset 'prune[${#prune[@]}-1]'
    find "$1" \( "${prune[@]}" \) -prune -o "${@:2}" -print
}

# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
GATE_GREP_EXCLUDES=()
for _gpd in "${GATE_PRUNE_DIRS[@]}"; do GATE_GREP_EXCLUDES+=(--exclude-dir="$_gpd"); done
unset _gpd

# spec: gate-sdk/SPEC.md §lib/gate.sh — the programs the payload is entitled to assume present, so a command-position word in this set is not a criterion-7 requirement; git is on it because §The port-candidate criteria already rules it the one sanctioned exception, "because it is the floor"
declare -p GATE_SDK_PROGRAM_FLOOR &>/dev/null \
    || GATE_SDK_PROGRAM_FLOOR=(awk basename bash cat cd chmod cmp comm cp cut date diff dirname
        env find git grep head ln ls mkdir mktemp mv printf pwd realpath rm sed sh sort tail tee
        touch tr uniq wc xargs)

gate_path_pruned() {
    local p="$1" d
    for d in "${GATE_PRUNE_DIRS[@]}"; do
        [[ "$p" == "$d/"* || "$p" == "./$d/"* || "$p" == */"$d"/* ]] && return 0
    done
    return 1
}

gate_sdk_root() {
    ( cd "${BASH_SOURCE[0]%/*}/.." && pwd )
}

gate_sdk_gates_dir() {
    printf '%s\n' "${GATE_SDK_GATES_DIR:-scripts}"
}

gates_list_members() {
    grep -Ev '^[[:space:]]*(#|$)' "$1" || true
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — resolve a gate name to its *declaration* path: the file whose text carries the `# graph:` manifest and the `# spec:`/`# assertion` directives. Dirs are tried consumer-first and `.sh` beats `.gate` within a dir, so a consumer shadowing a ported gate with its own shell script still wins. A dir carrying both spellings for one name is ambiguous dispatch, caught by check-gate-substrate-parity assertion A rather than silently ordered here.
gate_resolve() {
    local g="$1" d
    shift
    for d in "$@"; do
        if [[ -f "$d/$g.sh" ]]; then
            printf '%s\n' "$d/$g.sh"
            return 0
        fi
        if [[ -f "$d/$g.gate" ]]; then
            printf '%s\n' "$d/$g.gate"
            return 0
        fi
    done
    return 1
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — the owning kit of a bridged knob, derived from the knob's own `<KIT>_` prefix rather than from a maintained knob→kit roster: each gate_kit_roots member's basename, hyphens to underscores and upper-cased, is tried as a prefix. A knob matching no other kit's prefix is gate-sdk's own — the one kit every `.gate` dispatch already runs inside — never a parse error and never a third kit guessed at.
# spec: gate-sdk/SPEC.md §lib/gate.sh — the configured set is consulted first, then the shipped one: GATE_SDK_KIT_DIRS narrows which kits a battery *scans*, and reading it as the set of kits that *exist* would leave a narrowed run unable to attribute another kit's knob and fail-close on every member that declares one
# spec: gate-sdk/SPEC.md §lib/gate.sh — the candidates are read to EOF *before* the match loop, never streamed through a `while read` the first prefix hit returns out of: this runs under a stdout capture, so a producer left writing into a closed pipe reports the write error on stderr wherever SIGPIPE is ignored, which §run-gates' capture is what makes dispatch-fatal
_gate_knob_owning_kit() {
    local knob="$1" kit base prefix
    local -a kits=()
    mapfile -t kits < <(gate_kit_roots; [[ -n "${GATE_SDK_KIT_DIRS:-}" ]] && _gate_kit_roots_derived)
    for kit in ${kits[@]+"${kits[@]}"}; do
        kit="${kit%/}"
        [[ -n "$kit" ]] || continue
        base="${kit##*/}"
        prefix="${base^^}"; prefix="${prefix//-/_}_"
        [[ "$knob" == "$prefix"* ]] && { printf '%s\n' "$kit"; return 0; }
    done
    gate_sdk_root
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — resolve one declared knob to its tab-joined value by sourcing the owning kit's lib/*.sh in a subshell, so a kit library's globals cannot leak into the dispatcher or across members. Emits the joined value on stdout; returns non-zero having named the knob on stderr for each of the three refusals (undeclared knob, tab in an element, newline in an element).
_gate_knob_value() {
    local knob="$1" gate="$2" kit
    kit="$(_gate_knob_owning_kit "$knob")"
    (
        shopt -s nullglob
        # spec: gate-sdk/SPEC.md §lib/gate.sh — the knob under resolution, published to the
        # kit library being sourced, so a library whose knob costs a subprocess resolves
        # that one rather than all of them on every source
        export GATE_SDK_RESOLVING_KNOB="$knob"
        local _gkv_f
        for _gkv_f in "$kit"/lib/*.sh; do
            # shellcheck disable=SC1090  # the owning kit's library, resolved by prefix
            source "$_gkv_f"
        done
        if ! declare -p "$knob" >/dev/null 2>&1; then
            printf 'gate_command: %s declares knob %s, but %s/lib defines no such knob — ' "$gate" "$knob" "$kit" >&2
            printf 'the config bridge could not resolve it; treating as failure (not clean)\n' >&2
            exit 2
        fi
        local -n _gkv_val="$knob"
        local _gkv_e
        for _gkv_e in "${_gkv_val[@]+"${_gkv_val[@]}"}"; do
            case "$_gkv_e" in
                *$'\n'*)
                    printf 'gate_command: knob %s has an element containing a newline: %s — ' "$knob" "$_gkv_e" >&2
                    printf 'the argv protocol is one element per line; treating as failure (not clean)\n' >&2
                    exit 2 ;;
                *$'\t'*)
                    printf 'gate_command: knob %s has an element containing a tab: %s — ' "$knob" "$_gkv_e" >&2
                    printf 'tab separates the serialized elements; treating as failure (not clean)\n' >&2
                    exit 2 ;;
            esac
        done
        local IFS=$'\t'
        printf '%s' "${_gkv_val[*]+"${_gkv_val[*]}"}"
    )
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — resolve a gate name to its *invocation argv*, the execution counterpart of gate_resolve's declaration path: one element `<dir>/<name>.sh` for a shell gate, two elements `<binary> <name>` for a `.gate`-dispatched one — prefixed, when that member declares knobs, by `env` and one `GATE_SDK_KNOB_<NAME>=<tab-joined>` element per knob. Emits one argv element per line, so a caller looking for the dispatch executable takes the first element that is neither `env` nor an assignment. An absent or non-executable binary when a member dispatches to it is a harness error — exit 2, never a skip and never a pass (§Fail-closed contract): a skip would let the battery silently stop running a gate whenever a build is missing. A binary that cannot report its knobs, and each of the three knob-resolution refusals, exit 2 by the same contract.
gate_command() {
    local g="$1" d bin
    shift
    for d in "$@"; do
        if [[ -f "$d/$g.sh" ]]; then
            printf '%s\n' "$d/$g.sh"
            return 0
        fi
        if [[ -f "$d/$g.gate" ]]; then
            bin="$(gate_native_bin)"
            if [[ ! -x "$bin" ]]; then
                printf 'gate_command: %s dispatches to the native binary, but %s is ' "$g" "$bin" >&2
                printf 'absent or not executable — the gate could not run; treating as ' >&2
                printf 'failure (not clean). Build it: bash gate-sdk/bin/build-native.sh\n' >&2
                exit 2
            fi
            local knob_names knob_status knob value
            knob_names="$("$bin" --knobs "$g" 2>&1)"; knob_status=$?
            if [[ "$knob_status" -ne 0 ]]; then
                printf 'gate_command: %s --knobs %s exited %s — the config bridge could not ' "$bin" "$g" "$knob_status" >&2
                printf 'report what %s reads; treating as failure (not clean)\n%s\n' "$g" "$knob_names" >&2
                exit 2
            fi
            local -a env_elems=()
            while IFS= read -r knob; do
                [[ -n "$knob" ]] || continue
                value="$(_gate_knob_value "$knob" "$g")" || exit 2
                env_elems+=("GATE_SDK_KNOB_$knob=$value")
            done <<<"$knob_names"
            if [[ ${#env_elems[@]} -gt 0 ]]; then
                printf 'env\n'
                printf '%s\n' "${env_elems[@]}"
            fi
            printf '%s\n%s\n' "$bin" "$g"
            return 0
        fi
    done
    return 1
}

# spec: gate-sdk/SPEC.md §Layout and configuration — the one home of GATE_SDK_NATIVE_BIN's default, so a knob default gains readers without gaining spellings
gate_native_bin() {
    printf '%s\n' "${GATE_SDK_NATIVE_BIN:-native/target/release/checkwright-gates}"
}

# spec: gate-sdk/SPEC.md §Layout and configuration — the one home of GATE_SDK_NATIVE_CRATE's default, trailing slash stripped, so its three shell readers share a spelling rather than each carrying one
gate_native_crate() {
    local crate="${GATE_SDK_NATIVE_CRATE:-native}"
    printf '%s\n' "${crate%/}"
}

# spec: gate-sdk/SPEC.md §check-gate-output — the implementation module a .gate-dispatched member's rule lives in, derived from the gate name by the crate's own convention (drop the `check-` prefix, `-`→`_`) rather than held in a second registry that could drift from it
gate_native_module() {
    local g="${1#check-}"
    printf '%s/src/gates/%s.rs\n' "$(gate_native_crate)" "${g//-/_}"
}

# spec: gate-sdk/SPEC.md §Layout and configuration — GATE_SDK_NATIVE_TARGETS_FILE, defaulted off GATE_SDK_NATIVE_CRATE so the crate's location keeps one owner
gate_native_targets_file() {
    local crate
    crate="$(gate_native_crate)"
    printf '%s\n' "${GATE_SDK_NATIVE_TARGETS_FILE:-$crate/targets.list}"
}

# spec: gate-sdk/SPEC.md §check-gate-binary-fresh — the tree side of the source stamp: the same three git invocations native/build.rs bakes into the binary, so the comparison stays one algorithm rather than two implementations of one. Returns 1 emitting nothing when git cannot answer, so a caller fails closed rather than comparing against an empty string.
gate_native_source_stamp() {
    local crate listing hashed stamp i
    local -a paths=() hashes=()
    crate="$(gate_native_crate)"
    listing="$(git -C "$crate" ls-files 2>/dev/null)" || return 1
    [[ -n "$listing" ]] || return 1
    mapfile -t paths <<<"$listing"
    hashed="$(git -C "$crate" hash-object -- "${paths[@]}" 2>/dev/null)" || return 1
    mapfile -t hashes <<<"$hashed"
    [[ ${#hashes[@]} -eq ${#paths[@]} ]] || return 1
    local manifest=""
    for ((i = 0; i < ${#paths[@]}; i++)); do
        manifest+="${hashes[i]} ${paths[i]}"$'\n'
    done
    stamp="$(printf '%s' "$manifest" | git -C "$crate" hash-object --stdin 2>/dev/null)" || return 1
    [[ -n "$stamp" ]] || return 1
    printf '%s\n' "$stamp"
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — the target roster's single reader; an absent roster returns 1 and emits nothing, so a caller tells "no roster declared" from "a roster declaring nothing" rather than reading both as no targets
gate_native_targets() {
    local f
    f="$(gate_native_targets_file)"
    [[ -f "$f" ]] || return 1
    gates_list_members "$f"
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — the shipped kit set, by the checks/-or-smoke/ predicate alone: what a tree *contains*, before GATE_SDK_KIT_DIRS narrows what a battery scans
_gate_kit_roots_derived() {
    local d kit sdk parent
    sdk="$(gate_sdk_root)"
    printf '%s\n' "$sdk"
    parent="${sdk%/*}"
    for d in "$parent"/*/; do
        kit="${d%/}"
        [[ "$kit" == "$sdk" ]] && continue
        [[ -d "$kit/checks" || -d "$kit/smoke" ]] || continue
        printf '%s\n' "$kit"
    done
    return 0
}

gate_kit_roots() {
    local d
    if [[ -n "${GATE_SDK_KIT_DIRS:-}" ]]; then
        for d in $GATE_SDK_KIT_DIRS; do printf '%s\n' "$d"; done
        return 0
    fi
    _gate_kit_roots_derived
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — the resolved kit-root set as a bridgeable variable, the shape GATE_PRUNE_DIRS already has: GATE_SDK_KIT_DIRS is an override a consumer sets in the environment, so `declare -p` cannot find it and the config bridge cannot carry it. Resolving it here gives the value one computation serving both substrates, which is criterion 6's discharge-by-construction rather than a Rust twin of the predicate.
# spec: gate-sdk/SPEC.md §lib/gate.sh — spelled relative to the *current directory*, because a bridged value is baked verbatim into the generated pre-commit hook: an absolute root would commit one machine's checkout path to a tracked file. Resolving each root against the invoking directory keeps the path-prefix comparison exact on the binary side while nothing environment-specific crosses.
GATE_KIT_ROOTS_HERE=()
while IFS= read -r _gkr; do
    [[ -n "$_gkr" ]] || continue
    if [[ "$_gkr" == "$PWD"/* ]]; then
        GATE_KIT_ROOTS_HERE+=("${_gkr#"$PWD"/}")
    elif [[ "$_gkr" == /* ]]; then
        GATE_KIT_ROOTS_HERE+=("$(realpath --relative-to="$PWD" "$_gkr" 2>/dev/null || printf '%s' "$_gkr")")
    else
        GATE_KIT_ROOTS_HERE+=("$_gkr")
    fi
done < <(gate_kit_roots)
unset _gkr

gate_check_dirs() {
    gate_sdk_gates_dir
    local k
    while IFS= read -r k; do
        printf '%s/checks\n' "$k"
    done < <(gate_kit_roots)
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — the fixture-suite derivation shared by CI and evidence-kit's validate config: every dir with a gate-tests/ tree (the kit roots plus the gates dir), one tab-separated '<suite> <tests-dir> <checks-dir-or-empty>' row per suite in kit-roots-then-gates-dir order. suite = the dir basename with '-'→'_' (a valid var suffix + scenario name); checks-dir is the sibling checks/ when present, else empty so run-gate-tests falls back to consumer-first resolution. A new kit's gate-tests/ enrols with no hand-list to drift.
gate_fixture_suites() {
    local anchor base suite
    anchor="${GATE_SDK_ROOT:-$(gate_sdk_root)}"; anchor="${anchor%/*}"
    { gate_kit_roots_rel; gate_sdk_gates_dir; } | while IFS= read -r base; do
        base="${base%/}"
        [[ -d "$anchor/$base/gate-tests" ]] || continue
        suite="${base##*/}"; suite="${suite//-/_}"
        if [[ -d "$anchor/$base/checks" ]]; then
            printf '%s\t%s\t%s\n' "$suite" "$base/gate-tests" "$base/checks"
        else
            printf '%s\t%s\t\n' "$suite" "$base/gate-tests"
        fi
    done
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — the gate_kit_roots_rel cache-fill, split out so an in-process caller (gate_expand_couples) can prime the shared _gate_kit_roots_rel_cache array and read it directly, with no stdout round-trip / process-substitution fork of its own. The kit-root set cannot change mid-process, so a cold-cache fill (the realpath-per-root fork cost) happens at most once per gate invocation.
_gate_kit_roots_rel_ensure_cache() {
    [[ -n "${_gate_kit_roots_rel_cache_set:-}" ]] && return 0
    local anchor root
    anchor="${GATE_SDK_ROOT:-$(gate_sdk_root)}"; anchor="${anchor%/*}"
    _gate_kit_roots_rel_cache=()
    while IFS= read -r root; do
        # spec: gate-sdk/SPEC.md §lib/gate.sh — a root already under the anchor is the
        # common case and its relative form is the string remainder, so the realpath fork
        # is paid only for the exotic root that is not
        if [[ "$root" == "$anchor"/* ]]; then
            root="${root#"$anchor"/}"
        elif [[ "$root" == /* ]]; then
            root="$(realpath --relative-to="$anchor" "$root" 2>/dev/null || printf '%s' "$root")"
        fi
        _gate_kit_roots_rel_cache+=("$root")
    done < <(gate_kit_roots)
    _gate_kit_roots_rel_cache_set=1
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — gate_kit_roots as repo-root-relative dirs (the anchor the couples globs share); absolute roots resolve against the kits' parent, relative roots (a GATE_SDK_KIT_DIRS override) pass through
gate_kit_roots_rel() {
    _gate_kit_roots_rel_ensure_cache
    printf '%s\n' "${_gate_kit_roots_rel_cache[@]}"
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — the anchored spelling as a bridgeable variable, GATE_KIT_ROOTS' counterpart: a binary-side member needing repo-relative roots (a pathspec, a knob-prefix owner) reads this rather than re-deriving the anchor rule, which an override makes underivable from the absolute set alone
_gate_kit_roots_rel_ensure_cache
# shellcheck disable=SC2034  # read across the dispatch seam, never in this shell: the config bridge resolves it by name for a member that declares it
GATE_KIT_ROOTS_REL=("${_gate_kit_roots_rel_cache[@]}")

# spec: gate-sdk/SPEC.md §check-graph — expand each kit:<glob> token in a comma-joined couples/trigger field to <kit-root>/<glob> for every gate_kit_roots_rel member; non-kit tokens pass through verbatim. Assigns into the caller's <outvar> by nameref rather than printing, so a per-manifest-line loop calls this directly (no `$(...)` fork) and the whole loop shares one _gate_kit_roots_rel_ensure_cache fill instead of paying it — and forking — once per line. gate_expand_couples below is the same expansion for a caller that still wants the stdout form.
gate_expand_couples_var() {
    local -n _gate_expand_couples_out="$1"
    local field="$2"
    local -a parts=() out=()
    _gate_kit_roots_rel_ensure_cache
    IFS=',' read -ra parts <<<"$field"
    local tok r glob
    for tok in "${parts[@]}"; do
        if [[ "$tok" == kit:* ]]; then
            glob="${tok#kit:}"
            for r in "${_gate_kit_roots_rel_cache[@]}"; do out+=("${r%/}/$glob"); done
        else
            out+=("$tok")
        fi
    done
    local IFS=','
    _gate_expand_couples_out="${out[*]}"
}

# spec: gate-sdk/SPEC.md §check-graph — the gate_expand_couples_var expansion, printed to stdout for a `$(...)` caller; gate_expand_couples_var is the in-process form a hot per-line loop should call instead.
gate_expand_couples() {
    local __gate_expand_couples_result
    gate_expand_couples_var __gate_expand_couples_result "$1"
    printf '%s\n' "$__gate_expand_couples_result"
}

# spec: gate-sdk/SPEC.md §The `# graph:` manifest — read one field from a resolved gate's `# graph:` line; the shared field reader gen-pre-commit and run-gates --for selection draw the manifest through (the couples-token expansion is gate_expand_couples_var, the reader check-graph also shares). Emits the value, empty when the field is absent; never fails on a missing field.
gate_manifest_field() {
    local src="$1" key="$2" man kv
    man="$(grep -m1 '^# graph: ' "$src" 2>/dev/null || true)"
    for kv in ${man#\# graph: }; do
        [[ "$kv" == "$key="* ]] && { printf '%s' "${kv#"$key"=}"; return 0; }
    done
    return 0
}

# spec: gate-sdk/SPEC.md §run-gates — the path/glob matcher shared by run-gates --for selection and the emitted pre-commit hook: true when a path in the caller's staged_all array matches one of the given globs (bash glob, `*` spans '/'). gen-pre-commit emits this body verbatim into the hook's staged_matches; check-graph's freshness assertion holds the two in sync.
# shellcheck disable=SC2154  # staged_all is the caller's array: the hook's staged set, the selector's --for paths
gate_staged_matches() {
    local f pat
    for f in "${staged_all[@]}"; do
        for pat in "$@"; do
            # shellcheck disable=SC2053
            [[ "$f" == $pat ]] && return 0
        done
    done
    return 1
}

# spec: gate-sdk/SPEC.md §check-commit-msg — resolve the banned-pattern file set shared by check-commit-msg and check-tree-terms: explicit positional args win; otherwise GATE_SDK_MSG_PATTERN_FILES (tracked, must exist — fail-closed) plus GATE_SDK_MSG_PATTERN_FILES_LOCAL (gitignored, skipped when absent). Emits one existing readable file path per line; returns 2 when a required tracked file is missing.
gate_msg_pattern_files() {
    if [[ $# -gt 0 ]]; then
        printf '%s\n' "$@"
        return 0
    fi
    local f gd
    gd="$(gate_sdk_gates_dir)"
    for f in ${GATE_SDK_MSG_PATTERN_FILES:-$gd/msg-patterns.list}; do
        [[ -f "$f" ]] || { echo "gate_msg_pattern_files: required tracked pattern file missing: $f" >&2; return 2; }
        [[ -r "$f" ]] || { echo "gate_msg_pattern_files: pattern file not readable: $f" >&2; return 2; }
        printf '%s\n' "$f"
    done
    for f in ${GATE_SDK_MSG_PATTERN_FILES_LOCAL:-$gd/msg-patterns.local.list}; do
        [[ -f "$f" && -r "$f" ]] && printf '%s\n' "$f"
    done
    return 0
}

# spec: gate-sdk/SPEC.md §check-commit-subject — the single home of the commit-type roster (check-commit-subject's type alternation; trajectory.sh and kpi-task-split classify over the same tokens). Emits the space-separated roster on one line: GATE_SDK_COMMIT_TYPES when set, else the shipped default.
gate_commit_types() {
    printf '%s\n' "${GATE_SDK_COMMIT_TYPES:-feat fix refactor perf docs test build ci chore style}"
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — the self-repo blob-link prefix `<identity>/blob/<ref>/`, shared by check-md-refs' resolver and the reference-link producers (the enforcement map) so an emitted link and the pass that validates it derive one identity. Identity comes from `git remote get-url origin`; the git@ and https remote forms normalize to one https identity, so no kit ships a repo name (the provenance seam holds). Empty output ⇒ no origin or an unrecognized remote form, and the caller skips the self-repo pass. The ref is the caller's policy arg, never a literal here.
gate_self_repo_prefix() {
    local ref="$1" origin id rest
    origin="$(git remote get-url origin 2>/dev/null)" || return 0
    [[ -n "$origin" ]] || return 0
    id="${origin%.git}"; id="${id%/}"
    case "$id" in
        git@*:*)  rest="${id#git@}"; id="https://${rest/:/\/}" ;;
        https://*|http://*) ;;
        *) return 0 ;;
    esac
    printf '%s/blob/%s/\n' "$id" "$ref"
}
