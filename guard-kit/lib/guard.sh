# shellcheck shell=bash
# spec: guard-kit/SPEC.md §The guard framework — hook primitives + generic ruleset; no project rule content

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
declare -p GUARD_KIT_RO_SCRIPTS >/dev/null 2>&1 || GUARD_KIT_RO_SCRIPTS=("check-*.sh")
declare -p GUARD_KIT_SCRATCH_DIRS >/dev/null 2>&1 || GUARD_KIT_SCRATCH_DIRS=(".tmp")
declare -p GUARD_KIT_RO_BINS >/dev/null 2>&1 || GUARD_KIT_RO_BINS=(
    grep egrep fgrep rg head tail cat wc sort uniq cut tr nl rev tac paste comm column diff jq find ls xargs
)

guard_read_command() {
    local input cmd
    input="$(cat 2>/dev/null)" || return 1
    cmd="$(printf '%s' "$input" | jq -r '.tool_input.command // empty' 2>/dev/null)" || return 1
    [[ -z "$cmd" ]] && return 1
    printf '%s' "$cmd"
}

# spec: guard-kit/SPEC.md §The guard framework — the path counterpart of guard_read_command; a call carrying no file_path returns non-zero so a matcher covering it falls through instead of blocking
guard_read_path() {
    local input path
    input="$(cat 2>/dev/null)" || return 1
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

# spec: guard-kit/SPEC.md §The guard framework — one splitter for every consumer that reasons per compound segment (rules 8/12/14/16/17/19, the read-compound carve-out, scan-prompts), fed a guard_skeleton view so the harness's per-segment boundary set never drifts
guard_split_compound() {
    sed -E 's/\|\||&&|;|\|/\n/g' <<<"$1"
}

# spec: guard-kit/SPEC.md §The generic ruleset — the committed Bash(...) allow inners, one per line; the fail-open read rules 16 and 17 share, so a missing jq or settings file emits nothing and every reader declines
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

# spec: guard-kit/SPEC.md §The generic ruleset — a segment with its redirects removed and trimmed: what rules 16 and 17 compare against a committed bare allow entry
_guard_segment_core() {
    local seg
    seg="$(sed -E 's/[[:space:]]*[0-9]*(>>?|<)[[:space:]]*(&?[0-9-]+|[^[:space:]]+)?//g' <<<"$1")"
    seg="${seg#"${seg%%[![:space:]]*}"}"
    seg="${seg%"${seg##*[![:space:]]}"}"
    printf '%s' "$seg"
}

# spec: guard-kit/SPEC.md §The generic ruleset — true when the segment exactly matches a committed *bare* allow entry (no glob): the reviewed-lead half of rule 16's predicate and rule 17's lead test
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

# spec: guard-kit/SPEC.md §The generic ruleset — rule 16's xargs discriminator: xargs runs a command rather than filtering text, so the segment is read-only only when the command it runs is itself on the roster
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

# spec: guard-kit/SPEC.md §The generic ruleset — rule 13's loop-wrapper span: 'until <cond>; do sleep N; done' is the sanctioned wait, so only a sleep outside every do…done span fires, and an unresolvable span declines
guard_rule_bare_sleep() {
    local raw="$1" s tok depth=0 cmdpos=1 bare=0
    grep -qE '\$\(|<\(|>\(|\$\{|\$[A-Za-z_]' <<<"$raw" && return 0
    case "$raw" in *'`'*) return 0 ;; esac
    s="$(guard_skeleton "$raw" sq dq hd)"
    s="$(tr '\n' ';' <<<"$s" | sed -E 's/(\|\||&&|;|\||&|\(|\)|\{|\})/ \1 /g')"
    local -a toks
    read -ra toks <<<"$s"
    for tok in "${toks[@]}"; do
        case "$tok" in
            ';' | '|' | '||' | '&&' | '&' | '(' | ')' | '{' | '}') cmdpos=1 ;;
            '!' | until | while | if | then | else | elif | for) cmdpos=1 ;;
            do) depth=$((depth + 1)); cmdpos=1 ;;
            done)
                depth=$((depth - 1))
                [[ "$depth" -lt 0 ]] && return 0
                cmdpos=1 ;;
            sleep)
                [[ "$cmdpos" == 1 && "$depth" == 0 ]] && bare=1
                cmdpos=0 ;;
            *) cmdpos=0 ;;
        esac
    done
    [[ "$depth" == 0 && "$bare" == 1 ]] || return 0
    guard_block "don't wait by sleeping in the foreground — a wait must end when its condition goes true, not when a duration expires, and a foreground sleep spends a full-price turn doing nothing. Background a command that *exits* on the condition ('run_in_background' wrapping 'until <cond>; do sleep N; done') and take its completion notification: it fires the moment the condition holds and then ends. A dispatched agent is awaited by its own completion notification and never by a path on disk. The harness's event-stream form stays armed to its deadline even after its event fires, so it is the wrong tool for a single completion. A sleep inside a condition loop is untouched — that is the sanctioned form. If you genuinely need the settle, run it yourself with !<command>."
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
    done < <(grep -oE '[0-9]*>>?[[:space:]]*[^[:space:]|;&]+' <<<"$s" \
        | sed -E 's/^[0-9]*>>?[[:space:]]*//')
    if grep -qE '(^|[[:space:]])find([[:space:]]|$)' <<<"$s" \
        && grep -qE '\-(exec|execdir|ok|delete)\b' <<<"$s"; then
        return 0
    fi
    local -a segs
    mapfile -t segs < <(guard_split_compound "$s")
    local seg first b i matched reads=0
    for ((i = 0; i < ${#segs[@]}; i++)); do
        seg="${segs[i]}"
        seg="${seg#"${seg%%[![:space:]]*}"}"
        [[ -z "$seg" ]] && continue
        _guard_is_banner "$seg" && continue
        first="${seg%%[[:space:]]*}"
        [[ "$first" == xargs ]] && { _guard_is_ro_xargs "$seg" || return 0; }
        matched=0
        for b in "${GUARD_KIT_RO_BINS[@]}"; do
            [[ "$first" == "$b" ]] && { matched=1; break; }
        done
        if [[ "$matched" == 0 ]]; then
            # spec: guard-kit/SPEC.md §The generic ruleset — rule 16's widened lead: a bare
            # committed allow entry qualifies, but only where something decorates it
            [[ "$i" == 0 && "${#segs[@]}" -gt 1 ]] || return 0
            _guard_is_bare_allow "$seg" || return 0
        fi
        reads=$((reads + 1))
    done
    [[ "$reads" -ge 1 ]] || return 0
    guard_allow "read-only search pipeline (${GUARD_NAME:-guard} auto-allow)"
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
    guard_rule_truncate_scratch "$cmd"
    guard_rule_ro_pipeline "$cmd"
    guard_rule_allowlist_chain "$cmd"
    guard_rule_git_rewrite "$cmd"
    guard_rule_rm_tracked "$cmd"
}
