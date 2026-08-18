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
# spec: gate-sdk/SPEC.md §The workflow directory — the same resolution for the workflow directory, and for the same reason: the governed-comment corpus takes its tracked tier, so a compiled member reading that corpus declares this knob and the bridge can only carry a value some kit library defines
[[ -v GATE_SDK_WORKFLOW_DIR ]] || GATE_SDK_WORKFLOW_DIR=".workflow"
# spec: gate-sdk/SPEC.md §Layout and configuration — and the same resolution for the queue file, on the third occurrence of the same cause: a compiled member valving the queue out of its corpus declares this knob, and an environment-only override no kit library defines is the bridge's third refusal whatever prefix its name carries. Every inline reader keeps its spelling and its value; what changes is that the name now resolves to something declare -p can find.
[[ -v GATE_SDK_QUEUE_FILE ]] || GATE_SDK_QUEUE_FILE="TASK-QUEUE.md"
# spec: gate-sdk/SPEC.md §Layout and configuration — the same resolution again, for the two knobs the enforcement-map emitter reads once it is a compiled arm: a value no kit library defines is the bridge's third refusal, and these two were previously defaulted inside the emitter script itself, which is the duplication the bridge exists to remove. Resolved *after* the config seam above, so the config file still cannot name its own directory — what changes is that the resolved value is now something declare -p can find, not where it comes from.
# spec: gate-sdk/SPEC.md §enforcement-map — a guarded default erases set-ness, and the enforcement map tells its two adoption modes apart *by* set-ness (adopted-but-broken refuses where not-adopted degrades). These two are safe only because neither default can be absent: `scripts` is the gates dir the registry was already read from, and `.` is the cwd, so the refusal arm each knob's preflight guards is unreachable for the defaulted value. **This is not a precedent for a knob whose default may legitimately not exist** — `DRIFT_KIT_KPIS_FILE`'s is such a knob and takes a mode-preserving resolution in its own kit's library instead (drift-kit/SPEC.md §lib/drift.sh).
[[ -v GATE_SDK_GATES_DIR ]] || GATE_SDK_GATES_DIR="scripts"
[[ -v GATE_SDK_ENFORCE_SCAN_DIR ]] || GATE_SDK_ENFORCE_SCAN_DIR="."
# spec: gate-sdk/SPEC.md §check-hook-exec-bit — the same resolution once more, for the hooks
# directory, off GATE_SDK_GATES_DIR's own resolved default above rather than the not-yet-defined
# gate_sdk_gates_dir, so the two stay one value by construction rather than by two readers agreeing
[[ -v GATE_SDK_HOOKS_DIR ]] || GATE_SDK_HOOKS_DIR="$GATE_SDK_GATES_DIR/git-hooks"
# spec: gate-sdk/SPEC.md §check-root-tiering — the same resolution for that member's two remaining knobs, on the cause the four above already state: a knob no kit library defines is the bridge's third refusal, so the defaults could not stay inline in a check that dispatches to the binary. The allowlist default rides GATE_SDK_GATES_DIR's own resolved value above rather than the not-yet-defined gate_sdk_gates_dir, so the two stay one value by construction; an absent allowlist is the gate's own built-in-fallback branch, not a refusal, which is why defaulting a path that need not exist is safe here.
[[ -v GATE_SDK_ROOT_ALLOWLIST ]] || GATE_SDK_ROOT_ALLOWLIST="$GATE_SDK_GATES_DIR/root-allowlist.list"
[[ -v GATE_SDK_AGENT_FILE ]] || GATE_SDK_AGENT_FILE="CLAUDE.md"
# spec: gate-sdk/SPEC.md §check-commit-msg — the banned-pattern file set as arrays, so the config bridge can carry the value gate_msg_pattern_files already resolves. Distinct names on §lib/gate.sh's rule (a whitespace scalar feeding an array), and filled by the unquoted expansion the resolver itself used, so word-splitting and pathname expansion keep the semantics they had.
# shellcheck disable=SC2034  # read across the dispatch seam by the compiled member and by gate_msg_pattern_files below
GATE_MSG_PATTERN_FILES=()
for _gmp in ${GATE_SDK_MSG_PATTERN_FILES:-$GATE_SDK_GATES_DIR/msg-patterns.list}; do
    GATE_MSG_PATTERN_FILES+=("$_gmp")
done
# shellcheck disable=SC2034  # read across the dispatch seam by the compiled member and by gate_msg_pattern_files below
GATE_MSG_PATTERN_FILES_LOCAL=()
for _gmp in ${GATE_SDK_MSG_PATTERN_FILES_LOCAL:-$GATE_SDK_GATES_DIR/msg-patterns.local.list}; do
    GATE_MSG_PATTERN_FILES_LOCAL+=("$_gmp")
done
unset _gmp
# spec: gate-sdk/SPEC.md §check-core-files — the same resolution for that member's manifest path, on the cause the knobs above state: a default the bridge's `declare -p` cannot find is its undeclared-knob refusal. Rides GATE_SDK_GATES_DIR's resolved value; an absent manifest is the gate's own optional-config branch, not a refusal.
[[ -v GATE_SDK_CORE_FILES_FILE ]] || GATE_SDK_CORE_FILES_FILE="$GATE_SDK_GATES_DIR/core-files.list"
# spec: gate-sdk/SPEC.md §lib/gate.sh — the four knobs the fifth batch's members declare, resolved here for the cause the roster above states: a default written inline at a use site or inside a helper's body is invisible to the bridge's `declare -p`, which is its undeclared-knob refusal on the member's first post-port run. Each keeps the `:-` semantics its use site had (an empty value takes the default), and the identity manifest and tests dir ride GATE_SDK_GATES_DIR's resolved value above so the pair stays one value by construction. An absent identity manifest is the gate's own optional-config branch and an absent tests dir the coverage gate's own no-pair branch, not a refusal.
[[ -n "${GATE_SDK_IDENTITY_FILE:-}" ]] || GATE_SDK_IDENTITY_FILE="$GATE_SDK_GATES_DIR/identity.conf"
# spec: gate-sdk/SPEC.md §check-identity — the two actual-source knobs: each names a file standing in for one thing the *clone itself* says, and each is empty by default so the gate falls through to the live git read, which is the production path. Defined rather than left unset so the bridge can find them; the family's third member is the rider's own.
[[ -v GATE_SDK_GIT_EMAIL_FILE ]] || GATE_SDK_GIT_EMAIL_FILE=""
[[ -v GATE_SDK_GIT_REMOTES_FILE ]] || GATE_SDK_GIT_REMOTES_FILE=""
# spec: gate-sdk/SPEC.md §check-identity — the family's third member, the account kind's hosts file. Empty by default and *derived in the member* rather than here, on check-memory-off's cause: this knob's derivation reads $HOME, and a HOME-less derivation would yield a path under `/` that is absent, which this kind's graded absence posture reads as clean — the one false clean it exists to refuse. Empty means "derive it", never "no file".
[[ -v GATE_SDK_GH_HOSTS_FILE ]] || GATE_SDK_GH_HOSTS_FILE=""
# spec: gate-sdk/SPEC.md §check-identity — the host whose block the account kind reads, config-via-env on the CLI's own host-variable shape rather than a third manifest field
[[ -n "${GATE_SDK_GH_HOST:-}" ]] || GATE_SDK_GH_HOST="github.com"
[[ -n "${GATE_SDK_TESTS_DIR:-}" ]] || GATE_SDK_TESTS_DIR="$GATE_SDK_GATES_DIR/gate-tests"
[[ -n "${GATE_SDK_NATIVE_BIN:-}" ]] || GATE_SDK_NATIVE_BIN="native/target/release/checkwright-gates"
# spec: gate-sdk/SPEC.md §lib/gate.sh — the crate root is normalized where it is resolved rather than at each read, so the value the bridge carries is the canonical one gate_native_crate already printed
[[ -n "${GATE_SDK_NATIVE_CRATE:-}" ]] || GATE_SDK_NATIVE_CRATE="native"
GATE_SDK_NATIVE_CRATE="${GATE_SDK_NATIVE_CRATE%/}"
# spec: gate-sdk/SPEC.md §check-exec-bit — check-exec-bit's two whitespace-scalar overrides, resolved to arrays here so the config bridge can carry them. Distinct names on §lib/gate.sh's own rule: a scalar feeding an array is the one case a resolved global earns a spelling of its own, which is why GATE_PRUNE_DIRS above has one and the scalar-in/scalar-out knobs beside it do not. The glob default rides GATE_SDK_GATES_DIR's resolved value rather than the not-yet-defined gate_sdk_gates_dir, so the two stay one value by construction.
# shellcheck disable=SC2034  # consumed by the compiled member across the bridge, never within this lib
if [[ -n "${GATE_SDK_EXEC_GLOBS:-}" ]]; then
    read -r -a GATE_EXEC_GLOBS <<<"$GATE_SDK_EXEC_GLOBS"
else
    GATE_EXEC_GLOBS=('*/checks/*.sh' '*/kpis/*.sh' '*/bin/*.sh'
        "$GATE_SDK_GATES_DIR/check-*.sh" "$GATE_SDK_GATES_DIR/kpi-*.sh")
fi
# shellcheck disable=SC2034  # consumed by the compiled member across the bridge, never within this lib
if [[ -n "${GATE_SDK_EXEC_PRUNE:-}" ]]; then
    read -r -a GATE_EXEC_PRUNE <<<"$GATE_SDK_EXEC_PRUNE"
else
    GATE_EXEC_PRUNE=(gate-tests fixtures templates smoke)
fi

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

# spec: gate-sdk/SPEC.md §lib/gate.sh — resolve one declared knob to its tab-joined value by sourcing the owning kit's lib/*.sh in a subshell, so a kit library's globals cannot leak into the dispatcher or across members. Emits the joined value on stdout; returns non-zero having named the knob on stderr for each refusal (undeclared knob, tab or newline in an element, `=` in a key of a keyed knob).
# spec: gate-sdk/SPEC.md §lib/gate.sh — which arm a knob takes is *derived* from its own `declare -p` output rather than declared by the member: the same call that confirms the knob is defined carries the `declare -A` marker, so a keyed knob answers the shape question itself and no roster spelling is maintained beside it.
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
        local _gkv_decl
        if ! _gkv_decl="$(declare -p "$knob" 2>/dev/null)"; then
            printf 'gate_command: %s declares knob %s, but %s/lib defines no such knob — ' "$gate" "$knob" "$kit" >&2
            printf 'the config bridge could not resolve it; treating as failure (not clean)\n' >&2
            exit 2
        fi
        if [[ "$_gkv_decl" =~ ^declare[[:space:]]+-[a-zA-Z]*A ]]; then
            _gate_knob_pairs "$knob"
            exit $?
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

# spec: gate-sdk/SPEC.md §lib/gate.sh — the keyed arm's serialization: one `<key>=<value>` per tab-separated element, sorted by key. The sort is `LC_ALL=C` because the resolved argv is baked verbatim into the tracked pre-commit hook, so the emitted order must not depend on the invoking locale any more than it may depend on bash's hash seed. The split is on the *first* `=`, the rule `env` itself applies one level out, so only the key is constrained and a value carries `=` freely.
_gate_knob_pairs() {
    local -n _gkp_map="$1"
    local knob="$1" _gkp_k _gkp_v
    local -a _gkp_keys=() _gkp_pairs=()
    # comment-tier-exempt: the emptiness guard is a count test rather than the `${!map[@]+…}`
    # alternate-value form the indexed arms use — with a `!` prefix bash reads that as
    # indirect expansion with a default and hands back the values
    if [[ ${#_gkp_map[@]} -gt 0 ]]; then
        for _gkp_k in "${!_gkp_map[@]}"; do
            _gkp_v="${_gkp_map[$_gkp_k]}"
            case "$_gkp_k$_gkp_v" in
                *$'\n'*)
                    printf 'gate_command: knob %s has key %s whose pair contains a newline — ' "$knob" "$_gkp_k" >&2
                    printf 'the argv protocol is one element per line; treating as failure (not clean)\n' >&2
                    return 2 ;;
                *$'\t'*)
                    printf 'gate_command: knob %s has key %s whose pair contains a tab — ' "$knob" "$_gkp_k" >&2
                    printf 'tab separates the serialized elements; treating as failure (not clean)\n' >&2
                    return 2 ;;
            esac
            case "$_gkp_k" in
                *=*)
                    printf 'gate_command: knob %s has key %s containing an "=" — ' "$knob" "$_gkp_k" >&2
                    printf 'the pair splits on its first "=", so such a key is unsplittable; treating as failure (not clean)\n' >&2
                    return 2 ;;
            esac
            _gkp_keys+=("$_gkp_k")
        done
    fi
    if [[ ${#_gkp_keys[@]} -gt 0 ]]; then
        while IFS= read -r _gkp_k; do
            _gkp_pairs+=("$_gkp_k=${_gkp_map[$_gkp_k]}")
        done < <(printf '%s\n' "${_gkp_keys[@]}" | LC_ALL=C sort)
    fi
    local IFS=$'\t'
    printf '%s' "${_gkp_pairs[*]+"${_gkp_pairs[*]}"}"
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — the prefix form of _gate_knob_value: a declared name ending in `*` resolves the whole family defined under it, one `GATE_SDK_KNOB_<NAME>=<tab-joined>` element per match, sorted so the emitted environment is deterministic. Resolution happens at the same instant the scalar arm's does — after the owning kit's lib/*.sh has been sourced, which is what puts a consumer config's loop-declared variables in scope. A prefix matching nothing is a refusal naming it, never a resolved-empty set: the member asked for a family and the family is absent. The element-shape refusals are the scalar arm's, applied per match.
_gate_knob_prefix_values() {
    local prefix="$1" gate="$2" kit
    kit="$(_gate_knob_owning_kit "$prefix")"
    (
        shopt -s nullglob
        export GATE_SDK_RESOLVING_KNOB="$prefix"
        local _gkp_f
        for _gkp_f in "$kit"/lib/*.sh; do
            # shellcheck disable=SC1090  # the owning kit's library, resolved by prefix
            source "$_gkp_f"
        done
        local _gkp_n _gkp_e _gkp_hits=0
        local -a _gkp_names=()
        while IFS= read -r _gkp_n; do
            [[ "$_gkp_n" == "$prefix"* ]] && _gkp_names+=("$_gkp_n")
        done < <(compgen -v | sort)
        for _gkp_n in ${_gkp_names[@]+"${_gkp_names[@]}"}; do
            local -n _gkp_val="$_gkp_n"
            for _gkp_e in "${_gkp_val[@]+"${_gkp_val[@]}"}"; do
                case "$_gkp_e" in
                    *$'\n'*)
                        printf 'gate_command: knob %s has an element containing a newline: %s — ' "$_gkp_n" "$_gkp_e" >&2
                        printf 'the argv protocol is one element per line; treating as failure (not clean)\n' >&2
                        exit 2 ;;
                    *$'\t'*)
                        printf 'gate_command: knob %s has an element containing a tab: %s — ' "$_gkp_n" "$_gkp_e" >&2
                        printf 'tab separates the serialized elements; treating as failure (not clean)\n' >&2
                        exit 2 ;;
                esac
            done
            ( IFS=$'\t'; printf 'GATE_SDK_KNOB_%s=%s\n' "$_gkp_n" "${_gkp_val[*]+"${_gkp_val[*]}"}" )
            unset -n _gkp_val
            _gkp_hits=$(( _gkp_hits + 1 ))
        done
        # spec: gate-sdk/SPEC.md §lib/gate.sh — a prefix matching nothing resolves to an empty family and passes: the bridge holds no roster, so it has no expectation to fail closed on. The reader that named the roster is what refuses on a member it expected and did not get.
        : "$_gkp_hits"
    )
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — the bridged environment for one binary arm, whether that arm is a `.gate`-dispatched gate or a non-gate arm a front-end invokes (§The non-gate arm): asks the binary what the arm reads, resolves each name through _gate_knob_value, and emits one `GATE_SDK_KNOB_<NAME>=<tab-joined>` element per line. Returns non-zero having named the refusal on stderr — the status is the caller's to propagate, which is why this prints to stdout for a `$(…)` capture rather than writing into a process substitution that would swallow it.
gate_knob_env() {
    local g="$1" bin knob_names knob_status knob value
    bin="$(gate_native_bin)"
    knob_names="$("$bin" --knobs "$g" 2>&1)"; knob_status=$?
    if [[ "$knob_status" -ne 0 ]]; then
        printf 'gate_command: %s --knobs %s exited %s — the config bridge could not ' "$bin" "$g" "$knob_status" >&2
        printf 'report what %s reads; treating as failure (not clean)\n%s\n' "$g" "$knob_names" >&2
        return 2
    fi
    while IFS= read -r knob; do
        [[ -n "$knob" ]] || continue
        gate_knob_env_one "$knob" "$g" || return 2
    done <<<"$knob_names"
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — one declared name resolved to its bridged element(s): the trailing `*` selects the prefix family, anything else the scalar arm. Split out so the dispatcher and any harness resolving a member's declared knobs share one implementation of which arm a name takes, rather than each re-deriving it from the spelling.
gate_knob_env_one() {
    local knob="$1" g="$2" value
    if [[ "$knob" == *'*' ]]; then
        _gate_knob_prefix_values "${knob%\*}" "$g" || return 2
        return 0
    fi
    value="$(_gate_knob_value "$knob" "$g")" || return 2
    printf 'GATE_SDK_KNOB_%s=%s\n' "$knob" "$value"
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
            local env_out
            env_out="$(gate_knob_env "$g")" || exit 2
            local -a env_elems=()
            [[ -n "$env_out" ]] && mapfile -t env_elems <<<"$env_out"
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

# spec: gate-sdk/SPEC.md §Layout and configuration — the accessor for GATE_SDK_NATIVE_BIN, whose default is resolved at the top of this library so the bridge can find it; a knob default gains readers without gaining spellings
gate_native_bin() {
    printf '%s\n' "$GATE_SDK_NATIVE_BIN"
}

# spec: gate-sdk/SPEC.md §Layout and configuration — the accessor for GATE_SDK_NATIVE_CRATE, resolved and trailing-slash-stripped at the top of this library, so its three shell readers share a spelling rather than each carrying one
gate_native_crate() {
    printf '%s\n' "$GATE_SDK_NATIVE_CRATE"
}

# spec: gate-sdk/SPEC.md §check-gate-substrate-parity — the authoring-tree test: this tree carries the crate's *tracked source*, which is what makes it the tree that declared the kits it carries rather than a tree that vendored them. Source, so build output under the crate root cannot read as authorship. One holder for a predicate its readers scope themselves by; its honest limit is that it is tree-shaped, so a consumer authoring its own kit beside vendored ones reads as non-authoring.
gate_authoring_tree() {
    local crate
    crate="$(gate_native_crate)"
    [[ -d "$crate" ]] || return 1
    [[ -n "$(git -C "$crate" ls-files 2>/dev/null)" ]]
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

# spec: gate-sdk/SPEC.md §lib/gate.sh — the memo is scoped to one *sourcing* of this library rather than to the process: _gate_knob_value resolves a knob in a subshell that inherits the dispatcher's shell variables, so a memo carried across the re-source would hand a bridged member the dispatcher's kit-root set instead of the one the consumer config in scope resolves
unset _gate_kit_roots_rel_cache_set
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
    local f
    for f in "${GATE_MSG_PATTERN_FILES[@]}"; do
        [[ -f "$f" ]] || { echo "gate_msg_pattern_files: required tracked pattern file missing: $f" >&2; return 2; }
        [[ -r "$f" ]] || { echo "gate_msg_pattern_files: pattern file not readable: $f" >&2; return 2; }
        printf '%s\n' "$f"
    done
    for f in "${GATE_MSG_PATTERN_FILES_LOCAL[@]}"; do
        [[ -f "$f" && -r "$f" ]] && printf '%s\n' "$f"
    done
    return 0
}

# spec: gate-sdk/SPEC.md §check-commit-subject — the single home of the commit-type roster (check-commit-subject's type alternation; the trajectory arm and kpi-task-split classify over the same tokens). The default resolves onto the knob's own name so `declare -p` can find it, the shape §lib/gate.sh's document knobs already take: the compiled member declares GATE_SDK_COMMIT_TYPES, and a value no kit library defines is the config bridge's third refusal. Nothing moves but where the default is written — this stays the one place the roster is computed, and gate_commit_types stays its accessor, emitting the space-separated roster on one line.
[[ -n "${GATE_SDK_COMMIT_TYPES:-}" ]] || GATE_SDK_COMMIT_TYPES="feat fix refactor perf docs test build ci chore style"
gate_commit_types() {
    printf '%s\n' "$GATE_SDK_COMMIT_TYPES"
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
