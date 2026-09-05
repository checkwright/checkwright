# shellcheck shell=bash
# spec: guard-kit/SPEC.md §The guard framework — hook primitives + generic ruleset; no project rule content
# no-port: guard-kit/SPEC.md §The guard framework (`lib/guard.sh`) — permanently shell on two independent grounds, and that section states them. (1) The config bridge: this library is the sole resolver for the GUARD_KIT_* knobs, one of which a ported non-gate arm declares and the bridge resolves by sourcing this file, so a crate-side resolver would be the second producer criterion 6 refuses — gate-sdk/SPEC.md §The kit-library port disposition is the class ruling and gate-sdk/SPEC.md §lib/gate.sh the rule it rests on. (2) The extension point: guard-kit/SPEC.md §Consumer rules rules that a consumer's project block/steer/allow rules live in its copy of templates/bash-guard.sh, composed from these primitives, so this library is the API those rules are written against and porting it deletes the extension point — which is native-gate-port-remaining-corpus' ruling (1), a cut narrows the port and never an extension point. Structural, not a sizing judgment.

# spec: guard-kit/SPEC.md §Layout and configuration — a set-but-missing GUARD_KIT_CONFIG_FILE exits 2, surfacing as a hook block with this message on the first guarded command
_frik_cfg="${GUARD_KIT_CONFIG_FILE:-}"
if [[ -n "$_frik_cfg" ]]; then
    [[ -f "$_frik_cfg" ]] || {
        echo "guard-kit: GUARD_KIT_CONFIG_FILE not found: $_frik_cfg" >&2
        exit 2
    }
    # shellcheck source=/dev/null  # consumer config path is resolved at runtime
    source "$_frik_cfg"
else
    _frik_cfg="${GATE_SDK_GATES_DIR:-scripts}/guard-config.sh"
    if [[ -f "$_frik_cfg" ]]; then
        # shellcheck source=/dev/null  # consumer config path is resolved at runtime
        source "$_frik_cfg"
    fi
fi
unset _frik_cfg

: "${GUARD_KIT_LOG:=${GATE_SDK_WORKFLOW_DIR:-.workflow}/prompt-friction.log}"
: "${GUARD_KIT_WAKEUP_LOG:=${GATE_SDK_WORKFLOW_DIR:-.workflow}/wakeup-attempts.log}"
: "${GUARD_KIT_SETTINGS:=.claude/settings.json}"
: "${GUARD_KIT_SETTINGS_LOCAL:=.claude/settings.local.json}"
declare -p GUARD_KIT_BREADTH_PROBES >/dev/null 2>&1 || GUARD_KIT_BREADTH_PROBES=()
declare -p GUARD_KIT_BREADTH_DECLARED >/dev/null 2>&1 || declare -A GUARD_KIT_BREADTH_DECLARED=()
declare -p GUARD_KIT_RO_SCRIPTS >/dev/null 2>&1 || GUARD_KIT_RO_SCRIPTS=("check-*.sh")
declare -p GUARD_KIT_SCRATCH_DIRS >/dev/null 2>&1 || GUARD_KIT_SCRATCH_DIRS=(".tmp")
declare -p GUARD_KIT_RO_BINS >/dev/null 2>&1 || GUARD_KIT_RO_BINS=(
    grep egrep fgrep rg head tail cat wc sort uniq cut tr nl rev tac paste comm column diff jq find ls xargs
)
declare -p GUARD_KIT_APPEND_BINS >/dev/null 2>&1 || GUARD_KIT_APPEND_BINS=(cat printf echo)
declare -p GUARD_KIT_SCRIPT_INTERPRETERS >/dev/null 2>&1 || GUARD_KIT_SCRIPT_INTERPRETERS=(
    python python3 node deno ruby perl php zsh
)

# spec: guard-kit/SPEC.md §The guard framework — the payload cache: called directly (never in a substitution, which would kill the global with its subshell) so a rule needing a second field can have one
guard_read_input() {
    GUARD_INPUT="$(cat 2>/dev/null)" || return 1
    [[ -n "$GUARD_INPUT" ]] || return 1
    return 0
}

# spec: guard-kit/SPEC.md §The guard framework — one field of the cached payload by jq path; an unset or empty GUARD_INPUT and an absent path alike print nothing, which is what keeps a guard that never opted in working unchanged
guard_input_field() {
    local v
    [[ -n "${GUARD_INPUT:-}" ]] || return 0
    v="$(printf '%s' "$GUARD_INPUT" | jq -r "$1" 2>/dev/null)" || return 0
    [[ "$v" == "null" ]] && return 0
    printf '%s' "$v"
}

# spec: guard-kit/SPEC.md §The guard framework — GUARD_INPUT first, stdin otherwise: the fallback is what keeps every consumer copy that never opted in byte-identical
guard_read_command() {
    local input cmd
    if [[ -n "${GUARD_INPUT:-}" ]]; then
        input="$GUARD_INPUT"
    else
        input="$(cat 2>/dev/null)" || return 1
    fi
    cmd="$(printf '%s' "$input" | jq -r '.tool_input.command // empty' 2>/dev/null)" || return 1
    [[ -z "$cmd" ]] && return 1
    printf '%s' "$cmd"
}

# spec: guard-kit/SPEC.md §The guard framework — the path counterpart of guard_read_command; a call carrying no file_path returns non-zero so a matcher covering it falls through instead of blocking
guard_read_path() {
    local input path
    if [[ -n "${GUARD_INPUT:-}" ]]; then
        input="$GUARD_INPUT"
    else
        input="$(cat 2>/dev/null)" || return 1
    fi
    path="$(printf '%s' "$input" | jq -r '.tool_input.file_path // empty' 2>/dev/null)" || return 1
    [[ -z "$path" ]] && return 1
    printf '%s' "$path"
}

guard_block() {
    printf '%s\n' "${GUARD_NAME:-guard}: $1" >&2
    exit 2
}

guard_advise() {
    printf '%s' "$1" | jq -Rc '{hookSpecificOutput:{hookEventName:"PreToolUse",additionalContext:.}}'
    exit 0
}

guard_allow() {
    jq -nc --arg r "$1" \
        '{hookSpecificOutput:{hookEventName:"PreToolUse",permissionDecision:"allow",permissionDecisionReason:$r}}'
    exit 0
}

guard_rewrite() {
    jq -nc --arg c "$1" --arg r "$2" \
        '{hookSpecificOutput:{hookEventName:"PreToolUse",permissionDecision:"allow",permissionDecisionReason:$r,updatedInput:{command:$c}}}'
    exit 0
}

guard_log_fallthrough() {
    local fline
    fline="$(printf '%s' "$1" | tr '\n\t' '  ' | cut -c1-500)"
    printf '%s\n' "$fline" >>"$GUARD_KIT_LOG" 2>/dev/null || true
}

guard_allow_match() {
    local s="$1" glob="${2//:\*/\*}"
    # shellcheck disable=SC2053  # intentional glob match: $glob is a pattern, not a literal
    [[ "$s" == $glob ]]
}

# spec: guard-kit/SPEC.md §The guard framework — the one context-aware normalizer; a rule names the classes inert for it and every lexical view in the file comes from here
guard_skeleton() {
    local cmd="$1"
    shift
    local c want_sq=0 want_dq=0 want_hd=0 want_hdq=0
    for c in "$@"; do
        case "$c" in
            sq) want_sq=1 ;;
            dq) want_dq=1 ;;
            hd) want_hd=1 ;;
            hdq) want_hdq=1 ;;
        esac
    done

    # comment-tier-exempt: measured local fact — jumping between significant characters rather than stepping per character took a 355-char command from 2.3ms to 0.17ms, at fourteen call sites per guarded call
    local nl=$'\n'
    local live_class="[\"'\\\\<${nl}]*" dq_class="[\"\\\\]*"
    local n=${#cmd} i=0 out='' span='' state=none ch rest chunk line term body quoted
    local -a pending=() pending_q=()
    while ((i < n)); do
        rest="${cmd:i}"
        if [[ "$state" == sq ]]; then
            chunk="${rest%%\'*}"
            if [[ "$chunk" == "$rest" ]]; then
                span+="$rest"
                i=$n
                continue
            fi
            span+="$chunk'"
            ((i += ${#chunk} + 1))
            if ((want_sq)); then out+='SQ'; else out+="$span"; fi
            span=''
            state=none
            continue
        fi
        if [[ "$state" == dq ]]; then
            chunk="${rest%%$dq_class}"
            span+="$chunk"
            ((i += ${#chunk}))
            if ((i >= n)); then continue; fi
            if [[ "${cmd:i:1}" == '\' ]]; then
                span+="${cmd:i:2}"
                ((i += 2))
                continue
            fi
            span+='"'
            ((i++))
            if ((want_dq)); then out+='DQ'; else out+="$span"; fi
            span=''
            state=none
            continue
        fi
        chunk="${rest%%$live_class}"
        if [[ -n "$chunk" ]]; then
            out+="$chunk"
            ((i += ${#chunk}))
            ((i >= n)) && continue
        fi
        ch="${cmd:i:1}"
        case "$ch" in
            "'")
                state=sq
                span="'"
                ((i++))
                continue ;;
            '"')
                state=dq
                span='"'
                ((i++))
                continue ;;
            '\')
                out+="${cmd:i:2}"
                ((i += 2))
                continue ;;
            '<')
                if [[ "${cmd:i:3}" == '<<<' ]]; then
                    out+='<<<'
                    ((i += 3))
                    continue
                fi
                if [[ "${cmd:i}" =~ ^\<\<-?[[:space:]]*(\"[^\"]*\"|\'[^\']*\'|[A-Za-z_][A-Za-z0-9_]*) ]]; then
                    out+="${BASH_REMATCH[0]}"
                    ((i += ${#BASH_REMATCH[0]}))
                    term="${BASH_REMATCH[1]}"
                    case "$term" in
                        \"*\" | \'*\') pending_q+=(1) ;;
                        *) pending_q+=(0) ;;
                    esac
                    term="${term#[\"\']}"
                    term="${term%[\"\']}"
                    pending+=("$term")
                    continue
                fi
                ;;
            $'\n')
                out+=$'\n'
                ((i++))
                while ((${#pending[@]} > 0)); do
                    term="${pending[0]}"
                    quoted="${pending_q[0]}"
                    pending=("${pending[@]:1}")
                    pending_q=("${pending_q[@]:1}")
                    body=''
                    while ((i < n)); do
                        rest="${cmd:i}"
                        line="${rest%%$'\n'*}"
                        [[ "${line#"${line%%[![:space:]]*}"}" == "$term" ]] && break
                        body+="$line"$'\n'
                        ((i += ${#line} + 1))
                        ((i > n)) && i=$n
                    done
                    if [[ -n "$body" ]]; then
                        if ((want_hd)) || { ((want_hdq)) && ((quoted)); }; then
                            out+='HD'$'\n'
                        else
                            out+="$body"
                        fi
                    fi
                    if ((i < n)); then
                        rest="${cmd:i}"
                        line="${rest%%$'\n'*}"
                        out+="$line"
                        ((i += ${#line}))
                        if ((i < n)); then
                            out+=$'\n'
                            ((i++))
                        fi
                    fi
                done
                continue ;;
        esac
        out+="$ch"
        ((i++))
    done
    [[ -n "$span" ]] && out+="$span"
    printf '%s' "$out"
}

# spec: guard-kit/SPEC.md §The guard framework — one splitter for every shell consumer that reasons per compound segment (rules 8/12/14/15/17/18/19/20/22, the read-compound carve-out), fed a guard_skeleton view so the harness's per-segment boundary set never drifts; the compiled twin holds the other substrate
guard_split_compound() {
    sed -E 's/\|\||&&|;|\|/\n/g' <<<"$1"
}

# spec: guard-kit/SPEC.md §The generic ruleset — the committed Bash(...) allow inners, one per line; the fail-open read rules 18 and 19 share, so a missing jq or settings file emits nothing and every reader declines
_guard_allow_inners() {
    command -v jq >/dev/null 2>&1 || return 0
    [[ -f "$GUARD_KIT_SETTINGS" ]] || return 0
    local e inner
    while IFS= read -r e; do
        case "$e" in
            Bash\(*\)) inner="${e#Bash(}"; inner="${inner%)}" ;;
            *) continue ;;
        esac
        [[ -n "$inner" ]] && printf '%s\n' "$inner"
    done < <(jq -r '.permissions.allow[]?' "$GUARD_KIT_SETTINGS" 2>/dev/null)
}

# spec: guard-kit/SPEC.md §The generic ruleset — a segment with its redirects removed and trimmed: what rules 18 and 19 compare against a committed bare allow entry
_guard_segment_core() {
    local seg
    seg="$(sed -E 's/[[:space:]]*[0-9]*(>>?|<)[[:space:]]*(&?[0-9-]+|[^[:space:]]+)?//g' <<<"$1")"
    seg="${seg#"${seg%%[![:space:]]*}"}"
    seg="${seg%"${seg##*[![:space:]]}"}"
    printf '%s' "$seg"
}

# spec: guard-kit/SPEC.md §The generic ruleset — true when the segment exactly matches a committed *bare* allow entry (no glob): the reviewed-lead half of rule 18's predicate and rule 20's lead test
_guard_is_bare_allow() {
    local core bl
    core="$(_guard_segment_core "$1")"
    [[ -n "$core" ]] || return 1
    while IFS= read -r bl; do
        case "$bl" in *'*'*) continue ;; esac
        [[ "$core" == "$bl" ]] && return 0
    done < <(_guard_allow_inners)
    return 1
}

# spec: guard-kit/SPEC.md §The generic ruleset — the guard_rule_* run below; order is load-bearing
guard_rule_cd_compound() {
    local cmd
    cmd="$(guard_skeleton "$1" sq dq hd)"
    if grep -qE '(^|[;&|(])[[:space:]]*cd[[:space:]]' <<<"$cmd" && grep -qE '[;&|]' <<<"$cmd"; then
        guard_block "don't use 'cd' in a compound command (cwd drift, and the allowlist can't match the compound — the call costs an out-of-band permission decision). Pass absolute paths, or 'git -C <dir>' for git."
    fi
}

guard_rule_git_c_root() {
    local cmd
    cmd="$(guard_skeleton "$1" sq dq hd)"
    if grep -qF "git -C $PWD " <<<"$cmd"; then
        guard_block "drop 'git -C $PWD ' — cwd is the repo root, so the bare 'git <subcommand>' form is allowlisted and resolves on the match; the absolute '-C' spelling matches nothing and costs an out-of-band permission decision. Reserve 'git -C <dir>' for a different repo."
    fi
}

guard_rule_scratch_redirect() {
    local cmd
    cmd="$(guard_skeleton "$1" sq dq hd)"
    if grep -qE '(^|[[:space:]])([0-9]*|&)>>?[[:space:]]*[^[:space:]/|&]+\.(err|out|log)([[:space:]]|$)' <<<"$cmd"; then
        guard_block "don't redirect scratch to a bare repo-root filename (e.g. 2> op.err) — it pollutes cwd and risks a 'git add -A'. Send it to a gitignored scratch dir (e.g. ${GUARD_KIT_SCRATCH_DIRS[0]}/<name>.err)."
    fi
}

guard_rule_abs_script() {
    local raw="$1" cmd rest base g relcmd
    cmd="$(guard_skeleton "$raw" sq dq hd)"
    case "$cmd" in
        "bash $PWD/"*) rest="${cmd#bash "$PWD/"}" ;;
        "$PWD/"*)      rest="${cmd#"$PWD/"}" ;;
        *)             return 0 ;;
    esac
    rest="${rest%%[[:space:]]*}"            # first token = repo-relative script path
    case "$rest" in *.sh) ;; *) return 0 ;; esac   # only .sh scripts; rule 5 handles the rest
    base="${rest##*/}"
    relcmd="${raw//"$PWD/"/}"               # the rewrite carries the real command, not its skeleton
    for g in "${GUARD_KIT_RO_SCRIPTS[@]}"; do
        # shellcheck disable=SC2053  # intentional glob match: $g is a pattern, not a literal
        if [[ "$base" == $g || "$rest" == $g ]]; then
            guard_rewrite "$relcmd" "abs repo read-only script normalized to relative (${GUARD_NAME:-guard})"
        fi
    done
    guard_block "use the repo-relative form '$rest' (cwd is the repo root) — it's allowlisted and resolves on the match; the absolute spelling matches nothing and costs an out-of-band permission decision. If you truly need the absolute path, run it yourself with !<command>."
}

guard_rule_abs_prefix() {
    local cmd
    cmd="$(guard_skeleton "$1" sq dq hd)"
    [[ "$cmd" == git\ * ]] && return 0
    if grep -qF "$PWD/" <<<"$cmd"; then
        guard_block "drop the repo-root absolute prefix '$PWD/' — cwd is the repo root, so the repo-relative path is allowlisted and resolves on the match; the absolute spelling matches nothing and costs an out-of-band permission decision. If you truly need the absolute path, run it yourself with !<command>."
    fi
}

guard_rule_expansion() {
    local cmd="$1" sqexp expn
    # spec: guard-kit/SPEC.md §The generic ruleset — a double-quoted "$x" still expands, so 'dq'
    # stays live; a quoted-delimiter heredoc body cannot, which is what 'hdq' names
    sqexp="$(guard_skeleton "$cmd" sq hdq)"
    if grep -qE '\$\{|\$\(|<\(|\$[A-Za-z_]' <<<"$sqexp"; then
        guard_block "avoid shell variables/expansions (\$VAR, \${...}, \$(...), <(...)) — the harness's matcher refuses every expansion, so no allowlist entry can match the command and it costs an out-of-band permission decision. Inline the literal path, use a relative path, or 'git -C <dir>'. If you genuinely need the expansion, run it yourself with !<command>."
    fi
    expn="$(guard_skeleton "$cmd" sq dq hdq)"
    if grep -qE '(^|[;(]|&&|\|\|)[[:space:]]*[A-Za-z_][A-Za-z0-9_]*=[^[:space:];|&]*[[:space:]]*($|;)' <<<"$expn"; then
        guard_block "avoid shell variable assignments (NAME=value; ... \$NAME) — they defeat allowlist matching, so the call costs an out-of-band permission decision no allowlist entry can pre-empt. Inline the literal value/path at each use site, or 'git -C <dir>'. If you genuinely need it, run it yourself with !<command>."
    fi
}

guard_rule_brace_glyph() {
    local cmd="$1" sqstripped resid ph='{}' q="'{}'"
    # spec: guard-kit/SPEC.md §The generic ruleset — 'sq dq hd': '{' is a matcher glyph, not a
    # shell expansion, so rule 6's reason to keep double-quoted spans live does not carry here.
    sqstripped="$(guard_skeleton "$cmd" sq dq hd)"
    case "$sqstripped" in *'{'*) ;; *) return 0 ;; esac
    resid="${sqstripped//"$ph"/}"
    case "$resid" in
        *'{'* | *'}'*) ;;   # a non-placeholder brace remains: fall to the blocks
        *) guard_rewrite "${cmd//"$ph"/"$q"}" \
               "bare {} placeholder single-quoted so the harness matcher passes it (${GUARD_NAME:-guard})" ;;
    esac
    if grep -qF '@{' <<<"$sqstripped"; then
        guard_block "spell out the git-ref shorthand '@{...}' — the harness's matcher refuses the '{' glyph, so the call costs an out-of-band permission decision. Use 'origin/<branch>..HEAD' for '@{u}..', or the resolved ref/hash for a reflog form."
    fi
    if grep -qE '\{[^}]*(,|\.\.)[^}]*\}' <<<"$sqstripped"; then
        guard_block "write out the brace expansion '{a,b}'/'{a..b}' — the harness's matcher refuses the '{' glyph and no allowlist entry can match around it, so the call costs an out-of-band permission decision. Spell the members (e.g. 'mkdir -p a/b a/c') or use a loop for a long range."
    fi
    guard_block "quote the '{' if it's literal (an unquoted awk/sed program), or write it out if it expands — the harness's matcher refuses every bare '{' glyph before allowlist matching, so the call is decided out of band. A brace inside quotes of either kind, or in a heredoc body, is already inert and never reaches this block."
}

_guard_sed_segment() {
    local seg="$1" tok skip=0 have_script=0
    local -a toks
    read -ra toks <<<"$seg"
    for tok in "${toks[@]}"; do
        if [[ "$skip" == 1 ]]; then skip=0; have_script=1; continue; fi
        case "$tok" in
            sed) ;;
            -i | -i* | --in-place*)
                guard_block "don't rewrite a file with 'sed -i' — use the Edit tool: it replaces an exact string, fails loudly when the match is missing or ambiguous, and keeps the harness's view of the file current. If you genuinely need the in-place edit, run it yourself with !<command>." ;;
            -e | -f) skip=1 ;;
            --expression=* | --file=*) have_script=1 ;;
            --*) ;;
            -[!-]*)
                case "$tok" in
                    *i*) guard_block "don't rewrite a file with 'sed -i' — use the Edit tool: it replaces an exact string, fails loudly when the match is missing or ambiguous, and keeps the harness's view of the file current. If you genuinely need the in-place edit, run it yourself with !<command>." ;;
                esac ;;
            *)
                if [[ "$have_script" == 0 ]]; then
                    have_script=1
                else
                    guard_block "don't read a file through 'sed' — use the Read tool (offset/limit for a line range): it returns numbered lines and registers the file for a later Edit. For a markdown section, the consumer's section extractor beats a line range. If you genuinely need sed, pipe into it or run it yourself with !<command>."
                fi ;;
        esac
    done
}

guard_rule_sed_file() {
    local cmd="$1" s seg
    s="$(guard_skeleton "$cmd" sq dq hd)"
    while IFS= read -r seg; do
        seg="${seg#"${seg%%[![:space:]]*}"}"
        case "$seg" in
            sed | sed[[:space:]]*) _guard_sed_segment "$seg" ;;
        esac
    done < <(guard_split_compound "$s")
}

# spec: guard-kit/SPEC.md §The generic ruleset — a literal echo/printf banner segment: the natural separator of a batched read (no expansion survives here — rule 6 ran first, the caller bailed on substitution/backtick)
_guard_is_banner() {
    local seg="${1#"${1%%[![:space:]]*}"}"
    case "${seg%%[[:space:]]*}" in echo | printf) return 0 ;; *) return 1 ;; esac
}

# spec: guard-kit/SPEC.md §The generic ruleset — one segment is a lone single-file cat read: leads with cat, exactly one non-flag operand
_guard_is_cat_read() {
    local seg="${1#"${1%%[![:space:]]*}"}" rest tok operands=0
    [[ "${seg%%[[:space:]]*}" == cat ]] || return 1
    rest="${seg#cat}"
    local -a toks
    read -ra toks <<<"$rest"
    for tok in "${toks[@]}"; do
        case "$tok" in -*) ;; *) operands=$((operands + 1)) ;; esac
    done
    [[ "$operands" == 1 ]]
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 18's xargs discriminator: xargs runs a command rather than filtering text, so the segment is read-only only when the command it runs is itself on the roster
_guard_is_ro_xargs() {
    local seg="${1#"${1%%[![:space:]]*}"}" tok want_arg=0 cmdtok='' b i n
    local -a toks
    read -ra toks <<<"$seg"
    [[ "${toks[0]:-}" == xargs ]] || return 1
    n=${#toks[@]}
    for ((i = 1; i < n; i++)); do
        tok="${toks[i]}"
        if [[ "$want_arg" == 1 ]]; then want_arg=0; continue; fi
        case "$tok" in
            -0 | -t | -r | -x | -p | --null | --no-run-if-empty | --verbose | --exit | --interactive | --open-tty) ;;
            -I | -L | -n | -P | -s | -E | -d | -a) want_arg=1 ;;
            -[0ILnPsEdae]*) ;;
            --*=*) ;;
            -*) return 1 ;;
            *) cmdtok="$tok"; break ;;
        esac
    done
    [[ -z "$cmdtok" ]] && return 0
    [[ "$cmdtok" == xargs ]] && return 1
    case "$cmdtok" in echo | printf) return 0 ;; esac
    for b in "${GUARD_KIT_RO_BINS[@]}"; do
        [[ "$cmdtok" == "$b" ]] && return 0
    done
    return 1
}

# spec: guard-kit/SPEC.md §The generic ruleset — one segment is a bare find listing: leads with find, carries no action predicate
_guard_is_find_listing() {
    local seg="${1#"${1%%[![:space:]]*}"}"
    [[ "${seg%%[[:space:]]*}" == find ]] || return 1
    grep -qE '\-(execdir|exec|okdir|ok|delete|fls|fprintf|fprint0|fprint)\b' <<<"$seg" && return 1
    return 0
}

# spec: guard-kit/SPEC.md §The generic ruleset — a ';'-compound skeleton is a batched read when every segment is a bare read (the passed predicate) or a literal banner, and at least one is a read; the caller has already bailed on every non-';' separator, so guard_split_compound sees only ';' sequencing
_guard_is_read_batch() {
    local s="$1" pred="$2" seg reads=0
    while IFS= read -r seg; do
        seg="${seg#"${seg%%[![:space:]]*}"}"
        [[ -z "$seg" ]] && continue
        if "$pred" "$seg"; then
            reads=$((reads + 1))
        elif ! _guard_is_banner "$seg"; then
            return 1
        fi
    done < <(guard_split_compound "$s")
    [[ "$reads" -ge 1 ]]
}

guard_rule_find_glob() {
    local cmd="$1" s
    grep -qE '\$\(|<\(|>\(' <<<"$cmd" && return 0
    case "$cmd" in *'`'*) return 0 ;; esac
    s="$(guard_skeleton "$cmd" sq dq hd)"
    grep -qE '(&&|\|\||\||&|<|>)' <<<"$s" && return 0
    _guard_is_read_batch "$s" _guard_is_find_listing || return 0
    guard_block "don't list files with a bare 'find' — use the Glob tool: it returns matching paths (registered for a later Read) and needs no permission decision at all. This fires on a lone listing and on a ';'-sequence of them (a literal echo/printf banner between them is fine); a 'find' carrying an action predicate (-exec/-delete/…), piped into a consumer, or redirected is untouched. If you genuinely need find, run it yourself with !<command>."
}

guard_rule_cat_file() {
    local cmd="$1" s
    grep -qE '\$\(|<\(|>\(' <<<"$cmd" && return 0
    case "$cmd" in *'`'*) return 0 ;; esac
    s="$(guard_skeleton "$cmd" sq dq hd)"
    grep -qE '(&&|\|\||\||&|<|>)' <<<"$s" && return 0
    _guard_is_read_batch "$s" _guard_is_cat_read || return 0
    guard_block "don't read files with a bare 'cat' — use the Read tool: it returns numbered lines registered for a later Edit, and needs no permission decision at all. This fires on a lone 'cat <file>' and on a ';'-sequence of them (a literal echo/printf banner between reads is fine — batch them into one Read); a 'cat' feeding a pipe or heredoc, redirecting, or concatenating multiple files in one command is composition and untouched. If you genuinely need cat, run it yourself with !<command>."
}

guard_rule_git_grep() {
    local cmd="$1" s tok i n positionals=0 want_arg=0 pat_opt=0 working_tree=0
    grep -qE '\$\(|<\(|>\(' <<<"$cmd" && return 0
    case "$cmd" in *'`'*) return 0 ;; esac
    s="$(guard_skeleton "$cmd" sq dq hd)"
    grep -qE '(&&|\|\||;|\||&|<|>)' <<<"$s" && return 0
    local -a toks
    read -ra toks <<<"$s"
    [[ "${toks[0]:-}" == git && "${toks[1]:-}" == grep ]] || return 0
    n=${#toks[@]}
    for ((i = 2; i < n; i++)); do
        tok="${toks[i]}"
        if [[ "$want_arg" == 1 ]]; then want_arg=0; continue; fi
        case "$tok" in
            --) break ;;
            --cached | --staged | --no-index | --untracked) return 0 ;;
            -e | -f) pat_opt=1; want_arg=1 ;;
            -m | -A | -B | -C | --max-depth | --max-count | --threads | --context | --after-context | --before-context)
                want_arg=1 ;;
            -*) ;;
            *) positionals=$((positionals + 1)) ;;
        esac
    done
    if [[ "$pat_opt" == 1 ]]; then
        [[ "$positionals" == 0 ]] && working_tree=1
    else
        [[ "$positionals" == 1 ]] && working_tree=1
    fi
    [[ "$working_tree" == 1 ]] || return 0
    guard_block "don't search with 'git grep' over the working tree — use the Grep tool: it returns matching lines (files registered for a later Read) and needs no permission decision at all. A 'git grep' naming a revision, searching the index (--cached), or piped into a consumer is untouched — those reach beyond the working tree the Grep tool sees. If you genuinely need git grep, run it yourself with !<command>."
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 12's lead test: a leading shell keyword or negation does not change which binary the segment runs, so the loop-headed spelling the rule exists for is reached
_guard_command_word() {
    local seg="${1#"${1%%[![:space:]]*}"}" tok
    while [[ -n "$seg" ]]; do
        tok="${seg%%[[:space:]]*}"
        case "$tok" in
            '!' | until | while | if | then | else | elif | do) ;;
            *) break ;;
        esac
        seg="${seg#"$tok"}"
        seg="${seg#"${seg%%[![:space:]]*}"}"
    done
    printf '%s' "$seg"
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 12's pattern operand: the literal '-f' will scan argv for, or non-zero where the segment's options cannot be walked without guessing
_guard_pgrep_pattern() {
    local seg="$1" tok rest k pat='' have_f=0 skip=0
    local -a toks
    read -ra toks <<<"$seg"
    for tok in "${toks[@]:1}"; do
        if [[ "$skip" == 1 ]]; then skip=0; continue; fi
        case "$tok" in
            --full) have_f=1 ;;
            --exact | --inverse | --count | --newest | --oldest | --ignore-case | --list-name | --list-full | --lightweight) ;;
            --signal | --parent | --pgroup | --group | --session | --terminal | --euid | --uid | --delimiter) skip=1 ;;
            --*) return 1 ;;
            -[0-9]*) ;;
            -d | -P | -g | -G | -s | -t | -u | -U) skip=1 ;;
            -*)
                rest="${tok#-}"
                for ((k = 0; k < ${#rest}; k++)); do
                    case "${rest:k:1}" in
                        f) have_f=1 ;;
                        a | c | i | l | n | o | v | x | w) ;;
                        *) return 1 ;;
                    esac
                done ;;
            *) [[ -z "$pat" ]] || return 1; pat="$tok" ;;
        esac
    done
    [[ "$have_f" == 1 && -n "$pat" ]] || return 1
    case "$pat" in
        \'*\') pat="${pat#\'}"; pat="${pat%\'}" ;;
        \"*\") pat="${pat#\"}"; pat="${pat%\"}" ;;
    esac
    [[ -n "$pat" ]] || return 1
    case "$pat" in *\'* | *\"*) return 1 ;; esac
    printf '%s' "$pat"
}

guard_rule_pgrep_self_match() {
    local raw="$1" seg cmdseg pat occurrences
    grep -qE '\$\(|<\(|>\(|\$\{|\$[A-Za-z_]' <<<"$raw" && return 0
    case "$raw" in *'`'*) return 0 ;; esac
    while IFS= read -r seg; do
        cmdseg="$(_guard_command_word "$seg")"
        case "${cmdseg%%[[:space:]]*}" in pgrep | pkill) ;; *) continue ;; esac
        pat="$(_guard_pgrep_pattern "$cmdseg")" || continue
        occurrences="$(grep -oF -- "$pat" <<<"$raw" | wc -l)"
        [[ "$occurrences" -ge 2 ]] || continue
        guard_block "don't wait on process liveness with 'pgrep -f $pat' — '-f' matches full argv, and this command's own argv (the harness's wrapper included) carries that same literal, so the predicate is permanently true and the loop never exits. Nothing reds: the work finishes and the only symptom is the foreground cap absorbing an unbounded wait. Wait on the work's own artifact instead (an evidence file, a lock, an exit marker the work itself writes), or — where liveness genuinely is the condition — 'kill -0 <pid>' against a PID you recorded, whoever started that producer: a child you backgrounded yourself counts, and its PID is the one you recorded at launch, one line 'pid=<n> run=<key>' in a '<key>.run' file under your gitignored scratch dir. A pattern is a guess about a process table that includes the guesser; a PID is an identity. If you genuinely need pgrep, run it yourself with !<command>."
    done < <(guard_split_compound "$raw" | tr '&' '\n')
}

# spec: guard-kit/SPEC.md §The generic ruleset — the ruleset's one shell-keyword walk, shared by rules 13 and 15: emits '<depth> <cmdpos> <token>' per skeleton token and returns non-zero on an unbalanced do/done so both callers decline rather than guess
_guard_loop_span() {
    local s tok depth=0 cmdpos=1
    s="$(tr '\n' ';' <<<"$1" | sed -E 's/(\|\||&&|;|\||&|\(|\)|\{|\})/ \1 /g')"
    local -a toks
    read -ra toks <<<"$s"
    for tok in "${toks[@]}"; do
        case "$tok" in
            ';' | '|' | '||' | '&&' | '&' | '(' | ')' | '{' | '}' | \
                '!' | until | while | if | then | else | elif | for)
                printf '%s %s %s\n' "$depth" "$cmdpos" "$tok"
                cmdpos=1 ;;
            do)
                depth=$((depth + 1))
                printf '%s %s %s\n' "$depth" "$cmdpos" "$tok"
                cmdpos=1 ;;
            done)
                depth=$((depth - 1))
                [[ "$depth" -lt 0 ]] && return 1
                printf '%s %s %s\n' "$depth" "$cmdpos" "$tok"
                cmdpos=1 ;;
            *)
                printf '%s %s %s\n' "$depth" "$cmdpos" "$tok"
                cmdpos=0 ;;
        esac
    done
    [[ "$depth" -eq 0 ]]
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 13's loop-wrapper span: 'until <cond>; do sleep N; done' is the sanctioned wait, so only a sleep outside every do…done span fires, and an unresolvable span declines
guard_rule_bare_sleep() {
    local raw="$1" s span depth cmdpos tok bare=0
    grep -qE '\$\(|<\(|>\(|\$\{|\$[A-Za-z_]' <<<"$raw" && return 0
    case "$raw" in *'`'*) return 0 ;; esac
    s="$(guard_skeleton "$raw" sq dq hd)"
    span="$(_guard_loop_span "$s")" || return 0
    while read -r depth cmdpos tok; do
        [[ "$tok" == sleep && "$cmdpos" == 1 && "$depth" == 0 ]] && bare=1
    done <<<"$span"
    [[ "$bare" == 1 ]] || return 0
    guard_block "don't wait by sleeping in the foreground — a wait must end when its condition goes true, not when a duration expires, and a foreground sleep spends a full-price turn doing nothing. Background a command that *exits* on the condition ('run_in_background' wrapping 'until <cond>; do sleep N; done') and take its completion notification: it fires the moment the condition holds and then ends. A dispatched agent is awaited by its own completion notification and never by a path on disk. The harness's event-stream form stays armed to its deadline after its event fires when the command it was armed with is unbounded, so it is the second choice for a single completion. Mind the polarity: 'until' takes a done predicate ('until [ -f marker ]'), while a PID's liveness is a still-running one and takes 'while' ('while kill -0 <pid> 2>/dev/null; do sleep N; done') — inverted, the loop exits at once with the producer still running. Spell that PID as the literal number you read out of the .run record: a '\"\$var\"' expansion in the condition is refused by the expansion rule before this steer can be followed, and the literal form is the one the bounded-wait grant recognizes. A sleep inside a condition loop is untouched — that is the sanctioned form. If you genuinely need the settle, run it yourself with !<command>."
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 14's record reader: the one-line 'pid=<n> run=<key>' grammar is evidence-kit/SPEC.md §The producer-liveness lock's and is read rather than sourced, because a PreToolUse hook cannot depend on a sibling kit being vendored; a record that does not parse yields nothing, so the rule declines on one
_guard_live_run_records() {
    local d rec line pid
    for d in ${GUARD_KIT_SCRATCH_DIRS[@]+"${GUARD_KIT_SCRATCH_DIRS[@]}"}; do
        [[ -d "$d" ]] || continue
        for rec in "$d"/*.run; do
            [[ -f "$rec" ]] || continue
            IFS= read -r line <"$rec" || continue
            [[ "$line" =~ ^pid=([1-9][0-9]*)[[:space:]]run=([^[:space:]]+)$ ]] || continue
            pid="${BASH_REMATCH[1]}"
            { kill -0 "$pid" 2>/dev/null || ps -p "$pid" >/dev/null 2>&1; } \
                && printf "'%s' (pid %s, recorded in %s)\n" "${BASH_REMATCH[2]}" "$pid" "$rec"
        done
    done
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 14's subcommand walk: git's global options are consumed so 'git -C dir commit' is reached, and any option this list does not recognize returns non-zero so the segment declines rather than guessing which token is the subcommand
_guard_git_subcommand() {
    local seg="$1" tok first=1 expect_arg=0
    for tok in $seg; do
        if [[ "$first" == 1 ]]; then
            [[ "$tok" == git ]] || return 1
            first=0
            continue
        fi
        if [[ "$expect_arg" == 1 ]]; then expect_arg=0; continue; fi
        case "$tok" in
            -C | -c | --git-dir | --work-tree | --namespace | --exec-path | --config-env)
                expect_arg=1 ;;
            --git-dir=* | --work-tree=* | --namespace=* | --exec-path=* | --config-env=*) ;;
            -p | -P | --paginate | --no-pager | --bare | --no-replace-objects | \
                --literal-pathspecs | --no-literal-pathspecs | --glob-pathspecs | \
                --noglob-pathspecs | --icase-pathspecs | --no-optional-locks) ;;
            -*) return 1 ;;
            *) printf '%s' "$tok"; return 0 ;;
        esac
    done
    return 1
}

guard_rule_git_mutation_under_producer() {
    local raw="$1" s seg cmdseg sub
    case "$raw" in *git*) ;; *) return 0 ;; esac
    grep -qE '\$\(|<\(|>\(|\$\{|\$[A-Za-z_]' <<<"$raw" && return 0
    case "$raw" in *'`'*) return 0 ;; esac
    s="$(guard_skeleton "$raw" sq dq hd)"
    local -a writes=()
    while IFS= read -r seg; do
        cmdseg="$(_guard_command_word "$seg")"
        sub="$(_guard_git_subcommand "$cmdseg")" || continue
        case "$sub" in
            add | commit | rm | mv | restore | checkout | switch | reset | stash | \
                merge | rebase | cherry-pick | revert | apply | am | clean) ;;
            *) continue ;;
        esac
        writes+=("git $sub")
    done < <(guard_split_compound "$s")
    [[ "${#writes[@]}" -gt 0 ]] || return 0

    local -a runs=()
    mapfile -t runs < <(_guard_live_run_records)
    [[ "${#runs[@]}" -gt 0 ]] || return 0

    local list
    list="$(printf '%s; ' "${runs[@]}")"
    guard_block "don't run '${writes[0]}' while a producer you recorded is still running — ${list%; } names a live pid, and a tracked-tree mutation under a live producer is what the wait rule exists to prevent: the run is still writing, so a commit taken now dirties the worktree underneath it and its verdict has to be discarded and re-run. Two exits, both cheap: wait for that producer on its own artifact (loop on the recorded pid's liveness, 'until ! kill -0 <pid> 2>/dev/null; do sleep 5; done', backgrounded so its completion notifies you), or — if the producer has already exited — delete its .run file, which is not a workaround but the statement of fact becoming false and being retracted. Read-only git ('status', 'log', 'diff', 'show') is untouched. If you genuinely need this mutation now, run it yourself with !<command>."
}

# spec: guard-kit/SPEC.md §The generic ruleset — every redirect target in a skeleton, one per line: the corpus rules 15 and 18 both read
_guard_redirect_targets() {
    grep -oE '[0-9]*>>?[[:space:]]*[^[:space:]|;&]+' <<<"$1" \
        | sed -E 's/^[0-9]*>>?[[:space:]]*//'
}

# spec: guard-kit/SPEC.md §The generic ruleset — the read-only-segment test rules 15 and 18 share, xargs discriminator included because xargs runs a command rather than filtering text
_guard_is_ro_segment() {
    local seg="${1#"${1%%[![:space:]]*}"}" first b
    first="${seg%%[[:space:]]*}"
    [[ -n "$first" ]] || return 1
    [[ "$first" == xargs ]] && { _guard_is_ro_xargs "$seg" || return 1; }
    for b in "${GUARD_KIT_RO_BINS[@]}"; do
        [[ "$first" == "$b" ]] && return 0
    done
    return 1
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 15's shell arm: a statement-ending bare '&' in the skeleton, never the '&&' operator and never a redirect's fd-dup
_guard_shell_backgrounds() {
    grep -qE '(^|[^&>])&([[:space:]]|;|$)' <<<"$1"
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 15's record-writing test: at PreToolUse the child has not started and no record can exist yet, so the only observable is whether the launch is going to write one
_guard_writes_run_record() {
    local d tgt
    while read -r tgt; do
        case "$tgt" in *.run) ;; *) continue ;; esac
        for d in ${GUARD_KIT_SCRATCH_DIRS[@]+"${GUARD_KIT_SCRATCH_DIRS[@]}"}; do
            case "$tgt" in "$d"/*) return 0 ;; esac
        done
    done < <(_guard_redirect_targets "$1")
    return 1
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 15's exemption 3: a child that writes nothing has nothing for a later commit to corrupt, so it owes no record
_guard_is_ro_background() {
    local s="$1" tgt seg reads=0
    while read -r tgt; do
        case "$tgt" in
            /dev/null | '&'[0-9]*) ;;
            *) return 1 ;;
        esac
    done < <(_guard_redirect_targets "$s")
    while IFS= read -r seg; do
        seg="${seg#"${seg%%[![:space:]]*}"}"
        [[ -z "$seg" ]] && continue
        _guard_is_banner "$seg" && continue
        _guard_is_ro_segment "$seg" || return 1
        reads=$((reads + 1))
    done < <(guard_split_compound "$s")
    [[ "$reads" -ge 1 ]]
}

guard_rule_background_no_record() {
    local raw="$1" s span depth cmdpos tok
    grep -qE '\$\(|<\(|>\(|\$\{|\$[A-Za-z_]' <<<"$raw" && return 0
    case "$raw" in *'`'*) return 0 ;; esac
    s="$(guard_skeleton "$raw" sq dq hd)"
    [[ "$(guard_input_field '.tool_input.run_in_background')" == "true" ]] \
        || _guard_shell_backgrounds "$s" || return 0
    _guard_writes_run_record "$s" && return 0
    span="$(_guard_loop_span "$s")" || return 0
    while read -r depth cmdpos tok; do
        [[ "$depth" -ge 1 ]] && return 0
    done <<<"$span"
    _guard_is_ro_background "$s" && return 0
    guard_advise "this call backgrounds a child and writes no liveness record — write one at the launch, in the same command: a single line 'pid=<n> run=<key>' in a '<key>.run' file under your gitignored scratch dir (e.g. ${GUARD_KIT_SCRATCH_DIRS[0]}/<key>.run), naming the PID you just backgrounded. The record buys two things nothing else does: it is what gives the tracked-tree-mutation rule its reach, so a commit taken while this child is still writing is refused rather than silently taken; and it is what lets the next arrival tell whether the producer is still writing instead of guessing at a process table. Delete it once the producer has exited, and not before — a record naming a dead pid blocks nothing, and one deleted early buys the harm back. A backgrounded wait loop and a backgrounded read-only pipeline own no work a commit could corrupt and owe no record."
}

guard_rule_truncate_scratch() {
    local cmd
    cmd="$(guard_skeleton "$1" sq dq hd)"
    if [[ "$cmd" =~ ^[[:space:]]*:([[:space:]]+[0-9]*\>\>?[[:space:]]*[^[:space:]\&\|\;\<]+)+[[:space:]]*$ ]]; then
        local all_ignored=1 tgt
        while read -r tgt; do
            [[ -z "$tgt" ]] && continue
            git check-ignore --quiet -- "$tgt" || { all_ignored=0; break; }
        done < <(grep -oE '[0-9]*>>?[[:space:]]*[^[:space:]&|;<]+' <<<"$cmd" \
            | sed -E 's/^[0-9]*>>?[[:space:]]*//')
        if [[ "$all_ignored" == 1 ]]; then
            guard_allow "truncate gitignored scratch (${GUARD_NAME:-guard} auto-allow)"
        fi
    fi
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 17's redirect scan, operator and target together and fd-dups included: _guard_redirect_targets' target class excludes '&', so it drops an fd-dup target entirely and a rule that must exempt one cannot see it there
_guard_redirect_pairs() {
    grep -oE '[0-9]*>>?[[:space:]]*(&[0-9-]+|[^[:space:]|;&<>]+)' <<<"$1"
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 17's single-statement test: guard_skeleton leaves a heredoc's body placeholder and terminator on lines of their own and guard_split_compound emits per line, so one statement is one segment plus exactly the residue that segment's own openers produce, never one segment
_guard_only_heredoc_residue() {
    local -a segs=() terms=()
    local t seg i=1
    mapfile -t segs < <(guard_split_compound "$1")
    [[ "${#segs[@]}" -ge 1 ]] || return 1
    while IFS= read -r t; do
        t="${t#*<<}"
        t="${t#-}"
        t="${t#"${t%%[![:space:]]*}"}"
        t="${t#[\"\']}"
        t="${t%[\"\']}"
        terms+=("$t")
    done < <(grep -oE '<<-?[[:space:]]*("[^"]*"|'\''[^'\'']*'\''|[A-Za-z_][A-Za-z0-9_]*)' <<<"${segs[0]}")
    for t in ${terms[@]+"${terms[@]}"}; do
        seg="${segs[i]:-}"
        seg="${seg#"${seg%%[![:space:]]*}"}"
        if [[ "$seg" == HD ]]; then
            ((i++))
            seg="${segs[i]:-}"
            seg="${seg#"${seg%%[![:space:]]*}"}"
        fi
        [[ "$seg" == "$t" ]] || return 1
        ((i++))
    done
    [[ "$i" -eq "${#segs[@]}" ]]
}

guard_rule_append_scratch() {
    local raw="$1" s live
    # spec: guard-kit/SPEC.md §The generic ruleset — rule 17 clause (d): the substitution and backtick declines run on the 'hdq' view rather than the raw command, because a quoted-delimiter heredoc body cannot substitute (rule 6's own ground, one rule over) while every other region can — rule 6 blocks three of the four substitution spellings and exits 2 first, but not the output-process-substitution one, and a grant may not rest on a coverage claim that is only mostly true
    live="$(guard_skeleton "$raw" hdq)"
    grep -qE '\$\(|<\(|>\(' <<<"$live" && return 0
    case "$live" in *'`'*) return 0 ;; esac
    s="$(guard_skeleton "$raw" sq dq hd)"
    _guard_shell_backgrounds "$s" && return 0
    _guard_only_heredoc_residue "$s" || return 0

    local lead b on_roster=0
    lead="${s%%$'\n'*}"
    lead="${lead#"${lead%%[![:space:]]*}"}"
    lead="${lead%%[[:space:]]*}"
    for b in "${GUARD_KIT_APPEND_BINS[@]}"; do
        [[ "$lead" == "$b" ]] && { on_roster=1; break; }
    done
    [[ "$on_roster" == 1 ]] || return 0

    local pair tgt
    local -a targets=()
    while IFS= read -r pair; do
        [[ -z "$pair" ]] && continue
        pair="${pair#"${pair%%[!0-9]*}"}"
        if [[ "$pair" == '>>'* ]]; then tgt="${pair#>>}"; else tgt="${pair#>}"; fi
        tgt="${tgt#"${tgt%%[![:space:]]*}"}"
        case "$tgt" in /dev/null | '&'[0-9-]*) continue ;; esac
        case "$tgt" in *[\"\']*) return 0 ;; esac
        targets+=("$tgt")
    done < <(_guard_redirect_pairs "$s")
    [[ "${#targets[@]}" -ge 1 ]] || return 0

    for tgt in "${targets[@]}"; do
        git check-ignore --quiet -- "$tgt" || return 0
    done
    guard_allow "write to gitignored scratch (${GUARD_NAME:-guard} auto-allow)"
}

guard_rule_ro_pipeline() {
    local raw="$1"
    # spec: guard-kit/SPEC.md §The guard framework — the raw-command carve-out these two tests take
    grep -qE '\$\(|<\(|>\(' <<<"$raw" && return 0
    case "$raw" in *'`'*) return 0 ;; esac
    local s
    s="$(guard_skeleton "$raw" sq dq hd)"
    grep -q "['\"]" <<<"$s" && return 0
    grep -qE '(&&|\|\||;|&)' <<<"$s" && return 0
    local tgt
    while read -r tgt; do
        [[ -z "$tgt" ]] && continue
        case "$tgt" in
            /dev/null | '&'[0-9]*) ;;
            *) return 0 ;;
        esac
    done < <(_guard_redirect_targets "$s")
    if grep -qE '(^|[[:space:]])find([[:space:]]|$)' <<<"$s" \
        && grep -qE '\-(exec|execdir|ok|delete)\b' <<<"$s"; then
        return 0
    fi
    local -a segs
    mapfile -t segs < <(guard_split_compound "$s")
    local seg i reads=0
    for ((i = 0; i < ${#segs[@]}; i++)); do
        seg="${segs[i]}"
        seg="${seg#"${seg%%[![:space:]]*}"}"
        [[ -z "$seg" ]] && continue
        _guard_is_banner "$seg" && continue
        [[ "${seg%%[[:space:]]*}" == xargs ]] && { _guard_is_ro_xargs "$seg" || return 0; }
        if ! _guard_is_ro_segment "$seg"; then
            # spec: guard-kit/SPEC.md §The generic ruleset — rule 18's widened lead: a bare
            # committed allow entry qualifies, but only where something decorates it
            [[ "$i" == 0 && "${#segs[@]}" -gt 1 ]] || return 0
            _guard_is_bare_allow "$seg" || return 0
        fi
        reads=$((reads + 1))
    done
    [[ "$reads" -ge 1 ]] || return 0
    guard_allow "read-only search pipeline (${GUARD_NAME:-guard} auto-allow)"
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 19's inert-redirect test: a target that is not /dev/null and not an fd-dup is a write, and a grant may not bless one
_guard_wait_redirects_inert() {
    local pair tgt
    while IFS= read -r pair; do
        [[ -z "$pair" ]] && continue
        pair="${pair#"${pair%%[!0-9]*}"}"
        tgt="${pair#>}"
        tgt="${tgt#>}"
        tgt="${tgt#"${tgt%%[![:space:]]*}"}"
        case "$tgt" in /dev/null | '&'[0-9-]*) ;; *) return 1 ;; esac
    done < <(_guard_redirect_pairs "$1")
    return 0
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 19's clause (c): the loop condition runs once per iteration, unboundedly often, so it is held to rule 18's segment test plus the two condition forms that are read-only without being roster binaries — the shell tests, and 'kill -0', which asks a PID a question and sends no signal
_guard_is_wait_condition_segment() {
    local seg="$1" core cw first
    _guard_wait_redirects_inert "$seg" || return 1
    core="$(_guard_segment_core "$seg")"
    cw="$(_guard_command_word "$core")"
    first="${cw%%[[:space:]]*}"
    case "$first" in
        '[' | '[[' | test) return 0 ;;
        kill) grep -qE '(^|[[:space:]])-0([[:space:]]|$)' <<<"$cw" && return 0; return 1 ;;
    esac
    _guard_is_ro_segment "$cw"
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 19's clause (d): every statement after the loop meets rule 18's own test, so the grant covers the compound the measured class is written in and grants nothing rule 18 standing alone would not have granted
_guard_is_wait_tail_segment() {
    local seg="$1" core
    _guard_wait_redirects_inert "$seg" || return 1
    core="$(_guard_segment_core "$seg")"
    core="${core#"${core%%[![:space:]]*}"}"
    [[ -z "$core" ]] && return 0
    _guard_is_banner "$core" && return 0
    _guard_is_ro_segment "$core"
}

guard_rule_bounded_wait() {
    local raw="$1" s view cond body tail seg
    # spec: guard-kit/SPEC.md §The guard framework — the raw-command carve-out every auto-allow rule takes, adopted unchanged rather than reasoned about afresh
    grep -qE '\$\(|<\(|>\(' <<<"$raw" && return 0
    case "$raw" in *'`'*) return 0 ;; esac
    s="$(guard_skeleton "$raw" sq dq hd)"
    grep -q "['\"]" <<<"$s" && return 0
    # spec: guard-kit/SPEC.md §The generic ruleset — rule 19 clause (0): a compound that launches something beside its wait is a producer, not a waiter, and that is rule 15's subject
    _guard_shell_backgrounds "$s" && return 0

    # spec: guard-kit/SPEC.md §The generic ruleset — rule 19 clause (a), on the ruleset's one shell-keyword walk: the first statement is a while/until loop carrying exactly one balanced do…done span
    local span depth cmdpos tok dos=0 dones=0 firsttok=''
    span="$(_guard_loop_span "$s")" || return 0
    while read -r depth cmdpos tok; do
        [[ -z "$firsttok" ]] && firsttok="$tok"
        [[ "$tok" == "do" ]] && dos=$((dos + 1))
        [[ "$tok" == "done" ]] && dones=$((dones + 1))
    done <<<"$span"
    [[ "$firsttok" == until || "$firsttok" == while ]] || return 0
    [[ "$dos" == 1 && "$dones" == 1 ]] || return 0

    view="$(tr '\n' ';' <<<"$s")"
    [[ "$view" =~ ^[[:space:]]*(until|while)[[:space:]]+(.+)[[:space:]]+do[[:space:]]+(.+)[[:space:]]+done([[:space:];].*)?$ ]] || return 0
    cond="${BASH_REMATCH[2]}"
    body="${BASH_REMATCH[3]}"
    tail="${BASH_REMATCH[4]:-}"

    # spec: guard-kit/SPEC.md §The generic ruleset — rule 19 clause (b), the rule's safety core: a loop body running anything but sleep is unbounded work executing an unbounded number of times under a grant, and no clause elsewhere would bound it
    local sleeps=0
    local -a btoks
    while IFS= read -r seg; do
        seg="${seg#"${seg%%[![:space:]]*}"}"
        seg="${seg%"${seg##*[![:space:]]}"}"
        [[ -z "$seg" ]] && continue
        read -ra btoks <<<"$seg"
        [[ "${#btoks[@]}" -eq 2 && "${btoks[0]}" == sleep ]] || return 0
        case "${btoks[1]}" in *[!0-9.]*) return 0 ;; esac
        sleeps=$((sleeps + 1))
    done < <(guard_split_compound "$body")
    [[ "$sleeps" -ge 1 ]] || return 0

    local conds=0
    while IFS= read -r seg; do
        seg="${seg#"${seg%%[![:space:]]*}"}"
        [[ -z "$seg" ]] && continue
        _guard_is_wait_condition_segment "$seg" || return 0
        conds=$((conds + 1))
    done < <(guard_split_compound "$cond")
    [[ "$conds" -ge 1 ]] || return 0

    while IFS= read -r seg; do
        seg="${seg#"${seg%%[![:space:]]*}"}"
        [[ -z "$seg" ]] && continue
        _guard_is_wait_tail_segment "$seg" || return 0
    done < <(guard_split_compound "$tail")

    guard_allow "bounded in-turn wait (${GUARD_NAME:-guard} auto-allow)"
}

guard_rule_allowlist_chain() {
    local cmd="$1" inner
    local -a bare_leads=() pattern_inners=()
    while IFS= read -r inner; do
        pattern_inners+=("$inner")
        case "$inner" in *'*'*) ;; *) bare_leads+=("$inner") ;; esac
    done < <(_guard_allow_inners)
    [[ ${#bare_leads[@]} -gt 0 ]] || return 0

    local skel
    skel="$(guard_skeleton "$cmd" sq dq hd)"

    local -a segs
    mapfile -t segs < <(guard_split_compound "$skel")

    local lead="${segs[0]}"
    lead="${lead#"${lead%%[![:space:]]*}"}"; lead="${lead%"${lead##*[![:space:]]}"}"

    local lead_core
    lead_core="$(_guard_segment_core "$lead")"

    local bl matched_lead=0
    for bl in "${bare_leads[@]}"; do
        [[ "$lead_core" == "$bl" ]] && { matched_lead=1; break; }
    done
    [[ "$matched_lead" == 1 ]] || return 0

    local steer="run '$lead_core' bare — it's a statically allowlisted command, but the decoration (chaining or a redirect) leaves a segment nothing grants, so the whole call falls off the match path and costs an out-of-band permission decision. Run the allowlisted command on its own; issue the rest as separate calls."

    [[ "$lead" != "$lead_core" ]] && guard_block "$steer"

    [[ ${#segs[@]} -le 1 ]] && return 0

    local seg p i seg_matched
    for ((i = 1; i < ${#segs[@]}; i++)); do
        seg="${segs[i]}"
        seg="${seg#"${seg%%[![:space:]]*}"}"; seg="${seg%"${seg##*[![:space:]]}"}"
        [[ -z "$seg" ]] && continue
        seg_matched=0
        for p in "${pattern_inners[@]}"; do
            if guard_allow_match "$seg" "$p"; then seg_matched=1; break; fi
        done
        [[ "$seg_matched" == 1 ]] || guard_block "$steer"
    done
    return 0
}

guard_rule_git_rewrite() {
    local cmd="$1" s
    s="$(guard_skeleton "$cmd" sq dq hd)"
    if { grep -qE '(^|[[:space:]])git[[:space:]]+commit([[:space:]]|$)' <<<"$s" \
        && grep -qE '(^|[[:space:]])(-F|--file|--amend)\b' <<<"$s"; } \
        || { grep -qE '(^|[[:space:]])git[[:space:]]+reset([[:space:]]|$)' <<<"$s" \
        && grep -qE '(^|[[:space:]])--soft\b' <<<"$s"; }; then
        guard_advise "re-verify volatile git state before this history rewrite (DOCTRINE.md: Re-verify volatile state before a git history rewrite): confirm HEAD with 'git log --oneline -3' before an amend or squash; after a 'git reset --soft' re-stage and verify staged content with 'git show :<path>' before committing (the soft reset keeps the old index snapshot); write any 'git commit -F' message file fresh this turn — prefer '-m' for a short message, since a leftover file lands the wrong message with exit 0; and rewrite the message when amending so it states the combined change."
    fi
}

guard_rule_rm_tracked() {
    local raw="$1" s
    grep -qE '\$\(|<\(|>\(|\$\{|\$[A-Za-z_]' <<<"$raw" && return 0
    case "$raw" in *'`'*) return 0 ;; esac
    s="$(guard_skeleton "$raw" sq dq hd)"
    local seg lead arg
    while IFS= read -r seg; do
        seg="${seg#"${seg%%[![:space:]]*}"}"
        lead="${seg%%[[:space:]]*}"
        [[ "$lead" == "rm" ]] || continue
        for arg in ${seg#rm}; do
            case "$arg" in -* | '') continue ;; esac
            if git ls-files --error-unmatch -- "$arg" >/dev/null 2>&1; then
                guard_block "don't delete the git-tracked path '$arg' with a bare 'rm' — use 'git rm -q $arg': it removes the file and stages exactly that deletion in one motion, so no later 'git add -A' is needed to pick it up (which risks staging a concurrent session's foreign path). An 'rm' of an untracked or gitignored path is untouched. If you genuinely need rm, run it yourself with !<command>."
            fi
        done
    done < <(guard_split_compound "$s")
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 23's interpreter classification: arm (a) is the bash/sh pair the runner already serves, arm (b) the GUARD_KIT_SCRIPT_INTERPRETERS roster, and a word on neither is not a script interpreter at all
_guard_interpreter_arm() {
    local w="${1##*/}" i
    case "$w" in bash | sh) printf 'a'; return 0 ;; esac
    for i in ${GUARD_KIT_SCRIPT_INTERPRETERS[@]+"${GUARD_KIT_SCRIPT_INTERPRETERS[@]}"}; do
        [[ "$w" == "$i" ]] && { printf 'b'; return 0; }
    done
    return 1
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 23's scratch-source test on one token, the prefix idiom rule 15's record test already uses
_guard_is_scratch_path() {
    local p="$1" d
    for d in ${GUARD_KIT_SCRATCH_DIRS[@]+"${GUARD_KIT_SCRATCH_DIRS[@]}"}; do
        case "$p" in "$d"/* | "./$d"/*) return 0 ;; esac
    done
    return 1
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 23's cheap bail and its substitution arm's inner test: a scratch dir named anywhere in a string, which every arm of the rule requires
_guard_names_scratch() {
    local d
    for d in ${GUARD_KIT_SCRATCH_DIRS[@]+"${GUARD_KIT_SCRATCH_DIRS[@]}"}; do
        case "$1" in *"$d"/*) return 0 ;; esac
    done
    return 1
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 23's substitution arm: a command-substitution span naming a scratch path, in both spellings, since rule 6 reaches only the '$(…)' one and a guard that blocks one spelling teaches the spelling rather than the rule
_guard_substitution_scratch() {
    local span
    while IFS= read -r span; do
        _guard_names_scratch "$span" && return 0
    done < <(grep -oE '`[^`]*`|\$\([^)]*\)' <<<"$1")
    return 1
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 23's body-source resolution: an interpreter takes its program body from a -c/-e argument, from its first bare operand, or from stdin; emits 'inline', 'file <path>' or 'stdin', and returns non-zero on an option this walk cannot size
_guard_interpreter_body() {
    local seg="$1" arm="$2" tok rest k skip=0
    local -a toks
    read -ra toks <<<"$seg"
    for tok in ${toks[@]+"${toks[@]:1}"}; do
        if [[ "$skip" == 1 ]]; then skip=0; continue; fi
        case "$tok" in
            -c | --command | -m | --module) printf 'inline'; return 0 ;;
            -e | --eval) [[ "$arm" == a ]] || { printf 'inline'; return 0; } ;;
            - | /dev/stdin | /dev/fd/0) printf 'stdin'; return 0 ;;
            '<' | '>' | '>>' | '&>' | '&>>' | [0-9]'>' | [0-9]'>>' | [0-9]'<') skip=1 ;;
            '<'* | '>'* | [0-9]'>'* | [0-9]'<'*) ;;
            --) ;;
            -*)
                rest="${tok#-}"
                case "$rest" in -*) return 1 ;; esac
                for ((k = 0; k < ${#rest}; k++)); do
                    case "${rest:k:1}" in
                        B | E | I | O | i | l | n | s | t | u | v | x) ;;
                        *) return 1 ;;
                    esac
                done ;;
            *) printf 'file %s' "$tok"; return 0 ;;
        esac
    done
    printf 'stdin'
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 23's stdin source: a segment's '<' redirect target, never a '<<' heredoc opener, whose body rides in the command string and is the shape the rule deliberately does not fire on
_guard_stdin_redirect() {
    grep -oE '(^|[^<])<[[:space:]]*[^[:space:]<>|;&]+' <<<"$1" \
        | sed -E 's/^[^<]?<[[:space:]]*//'
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 23's two decisions: arm (a) steers to the runner, arm (b) states the bash-only rule, and both resolve the runner from GUARD_KIT_LIB so a consumer that vendors the kit elsewhere is told where its own copy is. The runner is a bridged arm of gate-sdk's front end, so what the steer composes is that front end's path, derived from the vendor root this kit's own location already names rather than hardcoded — a relocated tree still prints a path that resolves.
_guard_block_interpreter() {
    local arm="$1" word="$2" src="$3" runner="${GUARD_KIT_LIB:-guard-kit/lib/guard.sh}" root
    root="${runner%/lib/guard.sh}"
    if [[ "$root" == */* ]]; then root="${root%/*}/"; else root=""; fi
    runner="${root}gate-sdk/bin/run-gates.sh --scratch-run"
    if [[ "$arm" == a ]]; then
        guard_block "run a scratch script through the runner: 'bash $runner <script> [args…]' (guard-kit/SPEC.md §scratch-run). This call takes the program body for '$word' from '$src', which sits in a scratch dir any session can rewrite, so the body reviewed at the permission decision need not be the body that runs. The runner is allowlistable and echoes the body as it executes, which is the compensating control a direct run has none of. A body carried in the command string — a '-c' argument, a heredoc, a herestring — is untouched. If you genuinely need the direct form, run it yourself with !<command>."
    fi
    guard_block "scratch execution is bash-only (guard-kit/SPEC.md §scratch-run) and '$word' is not bash: this call takes its program body from '$src' under a scratch dir, where no compensating control reaches it. Write the body as a shell script and run it through 'bash $runner <script> [args…]', which echoes the body as it executes; a script whose shebang names a non-bash interpreter is refused there too. A body carried in the command string — a '-c' argument, a heredoc, a herestring — is untouched, because the approver and the friction log both see it verbatim. If you genuinely need the direct run, run it yourself with !<command>."
}

guard_rule_script_interpreter() {
    local raw="$1" s stmt seg word arm body src tok i j n
    local -a pipes=() ptoks=()
    _guard_names_scratch "$raw" || return 0
    # spec: guard-kit/SPEC.md §The generic ruleset — rule 23 declines on an expansion (rule 6 blocks those shapes already) but *not* on a backtick, which is the one body-source spelling rule 6 does not reach
    grep -qE '\$\{|<\(|>\(|\$[A-Za-z_]' <<<"$raw" && return 0
    s="$(guard_skeleton "$raw" sq dq hd)"
    # spec: guard-kit/SPEC.md §The generic ruleset — rule 23 splits statements then pipes rather than calling guard_split_compound: what it needs is dataflow (which segment's stdout is the interpreter's stdin), and the shared splitter erases the separator that tells a pipe from a ';'
    while IFS= read -r stmt; do
        mapfile -t pipes < <(tr '|' '\n' <<<"$stmt")
        n=${#pipes[@]}
        for ((i = 0; i < n; i++)); do
            seg="$(_guard_command_word "${pipes[i]}")"
            word="${seg%%[[:space:]]*}"
            [[ -n "$word" ]] || continue
            arm="$(_guard_interpreter_arm "$word")" || continue
            body="$(_guard_interpreter_body "$seg" "$arm")" || continue
            case "$body" in
                'file '*)
                    _guard_is_scratch_path "${body#file }" \
                        && _guard_block_interpreter "$arm" "$word" "${body#file }"
                    ;;
                stdin)
                    while read -r src; do
                        _guard_is_scratch_path "$src" \
                            && _guard_block_interpreter "$arm" "$word" "$src"
                    done < <(_guard_stdin_redirect "${pipes[i]}")
                    for ((j = 0; j < i; j++)); do
                        read -ra ptoks <<<"${pipes[j]}"
                        for tok in ${ptoks[@]+"${ptoks[@]}"}; do
                            _guard_is_scratch_path "$tok" \
                                && _guard_block_interpreter "$arm" "$word" "$tok"
                        done
                    done
                    ;;
                inline)
                    _guard_substitution_scratch "$raw" \
                        && _guard_block_interpreter "$arm" "$word" "a command substitution"
                    ;;
            esac
        done
    done < <(sed -E 's/\|\||&&|;/\n/g' <<<"$s")
}

guard_generic_rules() {
    local cmd="$1"
    guard_rule_cd_compound "$cmd"
    guard_rule_git_c_root "$cmd"
    guard_rule_scratch_redirect "$cmd"
    guard_rule_abs_script "$cmd"
    guard_rule_abs_prefix "$cmd"
    guard_rule_expansion "$cmd"
    guard_rule_brace_glyph "$cmd"
    guard_rule_sed_file "$cmd"
    guard_rule_find_glob "$cmd"
    guard_rule_cat_file "$cmd"
    guard_rule_git_grep "$cmd"
    guard_rule_pgrep_self_match "$cmd"
    guard_rule_bare_sleep "$cmd"
    guard_rule_git_mutation_under_producer "$cmd"
    guard_rule_background_no_record "$cmd"
    guard_rule_truncate_scratch "$cmd"
    guard_rule_append_scratch "$cmd"
    guard_rule_ro_pipeline "$cmd"
    guard_rule_bounded_wait "$cmd"
    guard_rule_allowlist_chain "$cmd"
    guard_rule_git_rewrite "$cmd"
    guard_rule_rm_tracked "$cmd"
    guard_rule_script_interpreter "$cmd"
}
