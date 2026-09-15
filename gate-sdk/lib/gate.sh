# shellcheck shell=bash
# spec: gate-sdk/SPEC.md §lib/gate.sh — sourced library: values + adapters, never gate structure
# no-port: gate-sdk/SPEC.md §lib/gate.sh, its port disposition — two grounds. Its API is shell functions sourced into bash callers (CI steps, the installer smoke, the author template, the front-ends and the test harness), and a binary arm cannot be sourced into bash; and its pre-binary accessors answer before any binary exists, the bootstrap cause bin/build-native.sh declares on. Structural, not a sizing judgment.

# spec: gate-sdk/SPEC.md §Fail-closed contract — non-zero capture status means the check could not run; exit 2, never a false clean
fail_closed() {
    if [[ "$1" -ne 0 ]]; then
        printf '%s: %s exited %s — the check could not run; treating as ' \
            "$2" "$3" "$1" >&2
        printf 'failure (not clean)\n' >&2
        exit 2
    fi
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — a value read on first use is memoised for one sourcing, never for the process, so a subshell re-sourcing under another environment reads its own
unset _gate_prune_loaded _gate_couples_loaded

gate_sdk_root() {
    ( cd "${BASH_SOURCE[0]%/*}/.." && pwd )
}

gate_sdk_gates_dir() {
    printf '%s\n' "${GATE_SDK_GATES_DIR:-scripts}"
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — the executable suffix has one owner and no other surface spells `.exe`: given a target triple it answers for that triple, given nothing (or an empty triple, which *is* the host triple — the shape `--target`-less cargo builds for) it answers for the host
# shellcheck disable=SC2120  # the argument-passing callers are in other files (bin/build-native.sh, scripts/ci-build-artifact.sh), so an analyser reading this file sees only the argument-less call below and cannot see that the parameter is optional by contract rather than unused
gate_exe_suffix() {
    local triple="${1:-}"
    if [[ -n "$triple" ]]; then
        case "$triple" in
        *-windows-*) printf '.exe' ;;
        esac
        return 0
    fi
    case "$(uname -s 2>/dev/null)" in
    MINGW* | MSYS* | CYGWIN* | Windows_NT) printf '.exe' ;;
    esac
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — one knob file's scalar line for a name, into <outvar>; returns 1 when the file names none. The binary refuses a malformed line at its first gate-sdk read, so this read skips one
_gate_prebinary_file_value() {  # <file> <NAME> <outvar>
    local _gpf_file="$1" _gpf_name="$2" _gpf_line _gpf_head
    local -n _gpf_out="$3"
    [[ -f "$_gpf_file" ]] || return 1
    while IFS= read -r _gpf_line || [[ -n "$_gpf_line" ]]; do
        _gpf_line="${_gpf_line#"${_gpf_line%%[![:blank:]]*}"}"
        [[ -n "$_gpf_line" && "$_gpf_line" != \#* && "$_gpf_line" == *=* ]] || continue
        _gpf_head="${_gpf_line%%=*}"
        _gpf_head="${_gpf_head%"${_gpf_head##*[![:blank:]]}"}"
        [[ "$_gpf_head" == "$_gpf_name" ]] || continue
        _gpf_line="${_gpf_line#*=}"
        _gpf_line="${_gpf_line#"${_gpf_line%%[![:blank:]]*}"}"
        _gpf_out="${_gpf_line%"${_gpf_line##*[![:blank:]]}"}"
        return 0
    done < "$_gpf_file"
    return 1
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — a pre-binary knob over the static precedence: the environment, the local overlay, the tracked file, then the default its caller derives as the table does; an empty value takes the default, as each of these rows does
_gate_prebinary_knob() {  # <NAME> <default>
    local _gpk_name="$1" _gpk_dir="${GATE_SDK_GATES_DIR:-scripts}" _gpk_v=""
    if [[ -n "${!_gpk_name:-}" ]]; then
        printf '%s\n' "${!_gpk_name}"
        return 0
    fi
    if _gate_prebinary_file_value "$_gpk_dir/gate-sdk-config.local.knobs" "$_gpk_name" _gpk_v && [[ -n "$_gpk_v" ]]; then
        printf '%s\n' "$_gpk_v"
        return 0
    fi
    if _gate_prebinary_file_value "${GATE_SDK_KNOB_FILE:-$_gpk_dir/gate-sdk-config.knobs}" "$_gpk_name" _gpk_v && [[ -n "$_gpk_v" ]]; then
        printf '%s\n' "$_gpk_v"
        return 0
    fi
    printf '%s\n' "$2"
}

# spec: gate-sdk/SPEC.md §Layout and configuration — GATE_SDK_NATIVE_BIN, read before any binary can answer for it
gate_native_bin() {
    _gate_prebinary_knob GATE_SDK_NATIVE_BIN "native/target/release/checkwright-gates$(gate_exe_suffix)"
}

# spec: gate-sdk/SPEC.md §Layout and configuration — GATE_SDK_NATIVE_CRATE, its trailing `/` stripped where it resolves
gate_native_crate() {
    local c
    c="$(_gate_prebinary_knob GATE_SDK_NATIVE_CRATE native)"
    printf '%s\n' "${c%/}"
}

# spec: gate-sdk/SPEC.md §Layout and configuration — GATE_SDK_NATIVE_TARGETS_FILE, defaulted off GATE_SDK_NATIVE_CRATE so the crate's location keeps one owner
gate_native_targets_file() {
    _gate_prebinary_knob GATE_SDK_NATIVE_TARGETS_FILE "$(gate_native_crate)/targets.list"
}

# spec: gate-sdk/SPEC.md §Layout and configuration — GATE_SDK_NATIVE_RUNNERS_FILE, defaulted off GATE_SDK_NATIVE_CRATE beside the roster's own
gate_native_runners_file() {
    _gate_prebinary_knob GATE_SDK_NATIVE_RUNNERS_FILE "$(gate_native_crate)/runners.list"
}

# spec: gate-sdk/SPEC.md §The non-gate arm — the resolved value of each named static knob, read off the binary's `--emit-knob-values` arm: the crate is the value's one producer, so no bash here recomputes a default
gate_knob_values() {
    "$(gate_native_bin)" --emit-knob-values "$@"
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — the prune set, read once on first use: the replacing knob's words, then the appending knob's, split on whitespace and expanded by nothing. It also fills GATE_GREP_EXCLUDES, which a gate splicing that array loads through this function first
gate_prune_load() {
    [[ -n "${_gate_prune_loaded:-}" ]] && return 0
    local out n el
    out="$(gate_knob_values GATE_SDK_PRUNE_DIRS GATE_SDK_PRUNE_EXTRA_DIRS)" || {
        echo "gate-sdk: the prune set could not be read from the gate binary; treating as failure (not clean)" >&2
        exit 2
    }
    _gate_prune=()
    GATE_GREP_EXCLUDES=()
    while IFS=$'\t' read -r n _ el; do
        [[ -n "$n" ]] || continue
        read -ra _gate_prune_words <<<"$el"
        _gate_prune+=(${_gate_prune_words[@]+"${_gate_prune_words[@]}"})
    done <<<"$out"
    for el in ${_gate_prune[@]+"${_gate_prune[@]}"}; do GATE_GREP_EXCLUDES+=(--exclude-dir="$el"); done
    unset _gate_prune_words
    _gate_prune_loaded=1
}

gate_find() {
    gate_prune_load
    local prune=() d
    for d in ${_gate_prune[@]+"${_gate_prune[@]}"}; do prune+=(-name "$d" -o); done
    unset 'prune[${#prune[@]}-1]'
    find "$1" \( "${prune[@]}" \) -prune -o "${@:2}" -print
}

gate_path_pruned() {
    gate_prune_load
    local p="$1" d
    for d in ${_gate_prune[@]+"${_gate_prune[@]}"}; do
        [[ "$p" == "$d/"* || "$p" == "./$d/"* || "$p" == */"$d"/* ]] && return 0
    done
    return 1
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

# spec: gate-sdk/SPEC.md §lib/gate.sh — resolve a gate name to its *invocation argv*, one element per line: `<dir>/<name>.sh` for a shell gate, `<binary> <name>` for a `.gate`-dispatched one. An absent or non-executable binary when a member dispatches to it is a harness error — exit 2, never a skip and never a pass (§Fail-closed contract).
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
            printf '%s\n%s\n' "$bin" "$g"
            return 0
        fi
    done
    return 1
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

# spec: gate-sdk/SPEC.md §lib/gate.sh — the target-to-runner map's single reader: it prints the runner one target is built on and returns 1 emitting nothing for a target the map does not name, so a caller refuses that target rather than picking a host for it. An absent map returns 1 for every target, which is the same refusal reached one step earlier.
gate_native_runner() {
    local target="$1" f line
    f="$(gate_native_runners_file)"
    [[ -f "$f" ]] || return 1
    while IFS= read -r line; do
        [[ "${line%%[[:space:]]*}" == "$target" ]] || continue
        line="${line#"$target"}"
        line="${line#"${line%%[![:space:]]*}"}"
        [[ -n "$line" ]] || return 1
        printf '%s\n' "${line%%[[:space:]]*}"
        return 0
    done < <(gates_list_members "$f")
    return 1
}

# spec: gate-sdk/SPEC.md §Layout and configuration — the kit roots the gate binary derives from the GATE_SDK_ROOT locator and the GATE_SDK_KIT_DIRS override, one per line relative to the working directory
gate_kit_roots() {
    GATE_SDK_ROOT="${GATE_SDK_ROOT:-$(gate_sdk_root)}" "$(gate_native_bin)" --emit-kit-roots
}

gate_check_dirs() {
    gate_sdk_gates_dir
    local k roots
    roots="$(gate_kit_roots)" || return 2
    while IFS= read -r k; do
        [[ -n "$k" ]] && printf '%s/checks\n' "$k"
    done <<<"$roots"
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — every `knob:` token the descriptor corpus carries, resolved in one binary call once per process and held by name, and the kit roots read once beside them
_gate_couples_ensure() {
    [[ -n "${_gate_couples_loaded:-}" ]] && return 0
    local dirs d f line kv tok out n el roots
    local -a files=() names=() parts=()
    declare -gA _GATE_COUPLES_VALUES=()
    _gate_couples_roots=()
    roots="$(gate_kit_roots)" || return 2
    [[ -n "$roots" ]] && mapfile -t _gate_couples_roots <<<"$roots"
    dirs="$(gate_check_dirs)" || return 2
    while IFS= read -r d; do
        [[ -n "$d" ]] || continue
        for f in "$d"/*.gate "$d"/*.sh; do
            [[ -f "$f" ]] && files+=("$f")
        done
    done <<<"$dirs"
    if [[ ${#files[@]} -gt 0 ]]; then
        while IFS= read -r line; do
            for kv in ${line#\# graph: }; do
                case "$kv" in couples=*|trigger=*) ;; *) continue ;; esac
                IFS=',' read -ra parts <<<"${kv#*=}"
                for tok in ${parts[@]+"${parts[@]}"}; do
                    [[ "$tok" == knob:* ]] && names+=("${tok#knob:}")
                done
            done
        done < <(grep -h '^# graph: ' "${files[@]}" 2>/dev/null || true)
    fi
    if [[ ${#names[@]} -gt 0 ]]; then
        out="$(gate_knob_values "${names[@]}")" || {
            printf 'gate_expand_couples: the couples knob tokens could not be read from the gate binary — an empty expansion is a lost trigger; treating as failure (not clean)\n' >&2
            return 2
        }
        while IFS=$'\t' read -r n _ el; do
            [[ -n "$n" ]] || continue
            if [[ -v _GATE_COUPLES_VALUES["$n"] && -n "${_GATE_COUPLES_VALUES[$n]}" ]]; then
                _GATE_COUPLES_VALUES["$n"]+=$'\t'"$el"
            else
                _GATE_COUPLES_VALUES["$n"]="$el"
            fi
        done <<<"$out"
    fi
    _gate_couples_loaded=1
}

# spec: gate-sdk/SPEC.md §The `# graph:` manifest — expand a comma-joined couples/trigger field in two passes in a fixed order: knob:<NAME> to that knob's members, then kit:<glob> to <kit-root>/<glob> for every kit root, so a knob member spelled kit:<glob> composes; anything else passes through verbatim. One pass each bounds the expansion without a cycle detector. Assigns into the caller's <outvar> by nameref rather than printing, so a per-manifest-line loop calls this directly with no fork.
# spec: gate-sdk/SPEC.md §Fail-closed contract — every knob-token refusal returns non-zero having named the knob on stderr, never an expansion missing the token's members: a silently lost trigger is a gate the hook stops running.
gate_expand_couples_var() {
    local -n _gate_expand_couples_out="$1"
    local field="$2"
    local -a parts=() once=() out=() members=()
    IFS=',' read -ra parts <<<"$field"
    local tok r glob name m value
    for tok in "${parts[@]}"; do
        if [[ "$tok" == knob:* ]]; then
            _gate_couples_ensure || return 2
            name="${tok#knob:}"
            if [[ ! -v _GATE_COUPLES_VALUES["$name"] ]]; then
                printf 'gate_expand_couples: couples token knob:%s names a knob the gate binary could not resolve — ' "$name" >&2
                printf 'an empty expansion is a lost trigger; treating as failure (not clean)\n' >&2
                return 2
            fi
            value="${_GATE_COUPLES_VALUES[$name]}"
            members=()
            [[ -n "$value" ]] && IFS=$'\t' read -ra members <<<"$value"
            for m in ${members[@]+"${members[@]}"}; do
                case "$m" in
                    knob:*)
                        printf 'gate_expand_couples: knob:%s has the member %s, itself a knob token — ' "$name" "$m" >&2
                        printf 'expansion is one pass each; treating as failure (not clean)\n' >&2
                        return 2 ;;
                    *,*|*[[:space:]]*)
                        printf 'gate_expand_couples: knob:%s has the member %s, which carries a comma or whitespace — ' "$name" "$m" >&2
                        printf 'it is unrepresentable after expansion; treating as failure (not clean)\n' >&2
                        return 2 ;;
                esac
                once+=("$m")
            done
        else
            once+=("$tok")
        fi
    done
    for tok in ${once[@]+"${once[@]}"}; do
        if [[ "$tok" == kit:* ]]; then
            _gate_couples_ensure || return 2
            glob="${tok#kit:}"
            for r in ${_gate_couples_roots[@]+"${_gate_couples_roots[@]}"}; do out+=("${r%/}/$glob"); done
        else
            out+=("$tok")
        fi
    done
    local IFS=','
    _gate_expand_couples_out="${out[*]+"${out[*]}"}"
}

# spec: gate-sdk/SPEC.md §check-graph — the gate_expand_couples_var expansion, printed to stdout for a `$(...)` caller; gate_expand_couples_var is the in-process form a hot per-line loop should call instead.
gate_expand_couples() {
    local __gate_expand_couples_result
    gate_expand_couples_var __gate_expand_couples_result "$1" || return 2
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

# spec: gate-sdk/SPEC.md §check-commit-msg — resolve the banned-pattern file set shared by check-commit-msg and check-tree-terms: explicit positional args win; otherwise GATE_SDK_MSG_PATTERN_FILES (tracked, must exist — fail-closed) plus GATE_SDK_MSG_PATTERN_FILES_LOCAL (gitignored, skipped when absent). Both are whitespace lists split with no expansion. Emits one existing readable file path per line; returns 2 when a required tracked file is missing.
gate_msg_pattern_files() {
    if [[ $# -gt 0 ]]; then
        printf '%s\n' "$@"
        return 0
    fi
    local f dir="${GATE_SDK_GATES_DIR:-scripts}"
    local -a required=() local_files=()
    read -ra required <<<"$(_gate_prebinary_knob GATE_SDK_MSG_PATTERN_FILES "$dir/msg-patterns.list")"
    read -ra local_files <<<"$(_gate_prebinary_knob GATE_SDK_MSG_PATTERN_FILES_LOCAL "$dir/msg-patterns.local.list")"
    for f in ${required[@]+"${required[@]}"}; do
        [[ -f "$f" ]] || { echo "gate_msg_pattern_files: required tracked pattern file missing: $f" >&2; return 2; }
        [[ -r "$f" ]] || { echo "gate_msg_pattern_files: pattern file not readable: $f" >&2; return 2; }
        printf '%s\n' "$f"
    done
    for f in ${local_files[@]+"${local_files[@]}"}; do
        [[ -f "$f" && -r "$f" ]] && printf '%s\n' "$f"
    done
    return 0
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
