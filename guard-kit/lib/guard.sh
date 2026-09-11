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
declare -p GUARD_KIT_RO_FORMS >/dev/null 2>&1 || declare -A GUARD_KIT_RO_FORMS=()
declare -p GUARD_KIT_APPEND_BINS >/dev/null 2>&1 || GUARD_KIT_APPEND_BINS=(cat printf echo)
declare -p GUARD_KIT_SEARCH_TOOLS >/dev/null 2>&1 || GUARD_KIT_SEARCH_TOOLS=(Glob Grep)
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

# spec: guard-kit/SPEC.md §The guard framework — one splitter for every shell consumer that reasons per compound segment (rules 2/4/7/8/12/14/15/17/18/19/20/22/24, the read-compound carve-out), fed a guard_skeleton view so the harness's per-segment boundary set never drifts; the compiled twin holds the other substrate
guard_split_compound() {
    sed -E 's/\|\||&&|;|\|/\n/g' <<<"$1"
}

# spec: guard-kit/SPEC.md §The guard framework — the harness view's word step: moves the head word of the caller's cur into its w and drops the blanks after it; non-zero when cur is empty
_guard_hv_pop() {
    [[ -n "$cur" ]] || return 1
    w="${cur%%[[:space:]]*}"
    cur="${cur:${#w}}"
    cur="${cur#"${cur%%[![:space:]]*}"}"
}

# spec: guard-kit/SPEC.md §The guard framework — the harness view: a segment as the permission matcher reads it, its documented leading wrappers stripped from the head repeatedly; a classification view and never a grant view, held equal to its compiled twin by --guard-lib-parity
_guard_harness_view() {
    local rest="${1#"${1%%[![:space:]]*}"}" cur w head
    local dur_re='^([0-9]+\.?[0-9]*|\.[0-9]+)[smhd]?$' int_re='^[+-]?[0-9]+$'
    local nice_re='^(-n[+-]?|--adjustment=[+-]?|-)[0-9]+$' asg_re='^[A-Za-z_][A-Za-z0-9_]*=[^"'\'']*$'
    while [[ -n "$rest" ]]; do
        cur="$rest"
        _guard_hv_pop
        head="$w"
        case "$head" in
            time)
                [[ "${cur%%[[:space:]]*}" == -p ]] && _guard_hv_pop ;;
            timeout)
                while :; do
                    case "${cur%%[[:space:]]*}" in
                        --preserve-status | --foreground | -v | --verbose) _guard_hv_pop ;;
                        -s | -k | --signal | --kill-after) _guard_hv_pop; _guard_hv_pop || break 2 ;;
                        -s?* | -k?* | --signal=?* | --kill-after=?*) _guard_hv_pop ;;
                        *) break ;;
                    esac
                done
                _guard_hv_pop && [[ "$w" =~ $dur_re ]] || break ;;
            nice)
                while :; do
                    w="${cur%%[[:space:]]*}"
                    if [[ "$w" == -n ]]; then
                        _guard_hv_pop
                        _guard_hv_pop && [[ "$w" =~ $int_re ]] || break 2
                    elif [[ "$w" =~ $nice_re ]]; then
                        _guard_hv_pop
                    else
                        break
                    fi
                done ;;
            stdbuf)
                while :; do
                    case "${cur%%[[:space:]]*}" in
                        -i | -o | -e) _guard_hv_pop; _guard_hv_pop || break 2 ;;
                        -[ioe]?* | --input=?* | --output=?* | --error=?*) _guard_hv_pop ;;
                        *) break ;;
                    esac
                done ;;
            nohup | builtin | noglob | command | xargs) ;;
            *)
                [[ "$head" =~ $asg_re ]] || break ;;
        esac
        [[ -n "$cur" && "$cur" != -* ]] || break
        rest="$cur"
    done
    printf '%s' "$rest"
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
    local cmd seg cmdseg w globals
    cmd="$(guard_skeleton "$1" sq dq hd)"
    if grep -qF "git -C $PWD " <<<"$cmd"; then
        guard_block "drop 'git -C $PWD ' — cwd is the repo root, so the bare 'git <subcommand>' form is allowlisted and resolves on the match; the absolute '-C' spelling matches nothing and costs an out-of-band permission decision. Reserve 'git -C <dir>' for a different repo."
    fi
    case "$cmd" in *git*-c* | */time* | */nice* | */nohup* | */stdbuf*) ;; *) return 0 ;; esac
    while IFS= read -r seg; do
        cmdseg="$(_guard_command_word "$seg")"
        w="${cmdseg%%[[:space:]]*}"
        if [[ "$w" == git ]] && globals="$(_guard_git_subcommand "$cmdseg" globals)" \
            && grep -qx -- -c <<<"$globals"; then
            guard_block "drop the 'git -c <key>=<value>' override and run the bare 'git <subcommand>' — a pager or color override has no effect without a terminal, and the bare form is allowlisted and resolves on the match. A '-c' form is never granted: a '-c' key can name a program the subcommand runs (core.pager, core.fsmonitor, core.sshCommand, alias.*), which is also why the harness's matcher does not see through it. If you genuinely need the config override, run it yourself with !<command>."
        fi
        case "$w" in */*) ;; *) continue ;; esac
        case "${w##*/}" in
            time | timeout | nice | nohup | stdbuf)
                guard_block "use the bare wrapper name '${w##*/}' rather than '$w' — the harness's matcher strips a bare '${w##*/}' before it matches, so the wrapped command resolves on its own allowlist entry, while an absolute spelling is matched as itself and costs an out-of-band permission decision. A format option only the binary takes ('/usr/bin/time -f') has no stripped spelling; if you genuinely need it, run it yourself with !<command>." ;;
        esac
    done < <(guard_split_compound "$cmd")
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
            _guard_rewrite_granted "$relcmd" \
                && guard_rewrite "$relcmd" "abs repo read-only script normalized to relative (${GUARD_NAME:-guard})"
            break
        fi
    done
    guard_block "use the repo-relative form '$rest' (cwd is the repo root) — an allowlist entry is written against the relative spelling, so it is the one that can resolve on the match; the absolute spelling matches nothing and costs an out-of-band permission decision. If you truly need the absolute path, run it yourself with !<command>."
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
        *)
            _guard_rewrite_granted "${cmd//"$ph"/"$q"}" \
                && guard_rewrite "${cmd//"$ph"/"$q"}" \
                    "bare {} placeholder single-quoted so the harness matcher passes it (${GUARD_NAME:-guard})"
            guard_block "quote each bare {} placeholder yourself, spelling it '{}' — the harness's matcher refuses a bare '{' glyph, and this guard rewrites the placeholder silently only when the rewritten command is allowlisted and stays inside its grant's path slots, which this one does not. The quoted spelling passes the same literal {} to the command, and the call then reaches the harness's own permission decision. If you genuinely need the bare form, run it yourself with !<command>." ;;
    esac
    if grep -qF '@{' <<<"$sqstripped"; then
        guard_block "spell out the git-ref shorthand '@{...}' — the harness's matcher refuses the '{' glyph, so the call costs an out-of-band permission decision. Use 'origin/<branch>..HEAD' for '@{u}..', or the resolved ref/hash for a reflog form."
    fi
    if grep -qE '\{[^}]*(,|\.\.)[^}]*\}' <<<"$sqstripped"; then
        guard_block "write out the brace expansion '{a,b}'/'{a..b}' — the harness's matcher refuses the '{' glyph and no allowlist entry can match around it, so the call costs an out-of-band permission decision. Spell the members (e.g. 'mkdir -p a/b a/c') or use a loop for a long range."
    fi
    guard_block "quote the '{' if it's literal (an unquoted awk/sed program), or write it out if it expands — the harness's matcher refuses every bare '{' glyph before allowlist matching, so the call is decided out of band. A brace inside quotes of either kind, or in a heredoc body, is already inert and never reaches this block."
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 8's one program-then-operands walk: a per-tool option table separates a sed or awk segment's program word from its file operands, filling the caller's prog (empty when an option supplied the program), inplace and prog_operands; non-zero on an awk option the table does not carry, so the caller declines rather than guess
_guard_program_operands() {
    local tool="$1" tok skip='' ends=0 have_prog=0 amp='&'
    local -a toks
    read -ra toks <<<"$2"
    prog='' inplace=0 prog_operands=()
    for tok in ${toks[@]+"${toks[@]:1}"}; do
        tok="${tok//$'\x01'/ }"
        tok="${tok//$'\x02'/$'\t'}"
        tok="${tok//$'\x03'/;}"
        tok="${tok//$'\x04'/|}"
        tok="${tok//$'\x05'/"$amp"}"
        if [[ -n "$skip" ]]; then
            [[ "$skip" == prog ]] && have_prog=1
            skip=''
            continue
        fi
        if [[ "$ends" == 0 ]]; then
            case "$tool:$tok" in
                sed:-i | sed:-i* | sed:--in-place*) inplace=1; continue ;;
                sed:-e | sed:-f) skip=prog; continue ;;
                sed:--expression=* | sed:--file=*) have_prog=1; continue ;;
                sed:--*) continue ;;
                sed:-[!-]*) [[ "$tok" == *i* ]] && inplace=1; continue ;;
                awk:--) ends=1; continue ;;
                awk:-F | awk:-v) skip=arg; continue ;;
                awk:-f) skip=prog; continue ;;
                awk:-F?* | awk:-v?*) continue ;;
                awk:-f?*) have_prog=1; continue ;;
                awk:-*) return 1 ;;
            esac
        fi
        if [[ "$have_prog" == 0 ]]; then
            prog="$tok"
            have_prog=1
        else
            prog_operands+=("$tok")
        fi
    done
    return 0
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 8's dequoted view: the raw command walked in lockstep with its 'sq dq hd' skeleton, every region decision taken from the skeleton, quote characters removed and each quoted span's blanks and statement separators held as sentinels so a split or a word split cuts exactly where it cuts the skeleton; non-zero where the two cannot be aligned (a heredoc body, an unterminated span, a newline inside quotes)
_guard_dequoted_view() {
    local raw="$1" s="$2" out='' i=0 j=0 k rest chunk span c
    local lit="[\"'\\\\]*" dqlit="[\"\\\\]*"
    while ((j < ${#raw})); do
        rest="${raw:j}"
        chunk="${rest%%$lit}"
        [[ "${s:i:${#chunk}}" == "$chunk" ]] || return 1
        out+="$chunk"
        ((i += ${#chunk}, j += ${#chunk}))
        ((j < ${#raw})) || break
        c="${raw:j:1}"
        if [[ "$c" == '\' ]]; then
            [[ "${s:i:2}" == "${raw:j:2}" ]] || return 1
            out+="${raw:j:2}"
            ((i += 2, j += 2))
            continue
        fi
        if [[ "$c" == "'" ]]; then
            [[ "${s:i:2}" == SQ ]] || return 1
            rest="${raw:j+1}"
            span="${rest%%\'*}"
            [[ "$span" == "$rest" ]] && return 1
            ((j += ${#span} + 2))
        else
            [[ "${s:i:2}" == DQ ]] || return 1
            k=$((j + 1))
            span=''
            while :; do
                rest="${raw:k}"
                chunk="${rest%%$dqlit}"
                [[ "$chunk" == "$rest" ]] && return 1
                span+="$chunk"
                ((k += ${#chunk}))
                [[ "${raw:k:1}" == '\' ]] || break
                span+="${raw:k:2}"
                ((k += 2))
            done
            ((j = k + 1))
        fi
        ((i += 2))
        [[ "$span" == *$'\n'* ]] && return 1
        span="${span// /$'\x01'}"
        span="${span//$'\t'/$'\x02'}"
        span="${span//;/$'\x03'}"
        span="${span//|/$'\x04'}"
        span="${span//&/$'\x05'}"
        out+="$span"
    done
    ((i == ${#s})) || return 1
    printf '%s' "$out"
}

# spec: guard-kit/SPEC.md §The generic ruleset — the runner path rules 8 and 23 print: gate-sdk's front end, derived from the vendor root GUARD_KIT_LIB already names rather than hardcoded, so a relocated tree still prints a path that resolves
_guard_front_end() {
    local lib="${GUARD_KIT_LIB:-guard-kit/lib/guard.sh}" root
    root="${lib%/lib/guard.sh}"
    if [[ "$root" == */* ]]; then root="${root%/*}/"; else root=""; fi
    printf '%sgate-sdk/bin/run-gates.sh' "$root"
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 8's awk arm: a pipeline-head awk segment with exactly one file operand whose program is a line range (NR comparisons only) or a markdown heading range, with no action or the print-all one; the steer is chosen by the program's shape
_guard_awk_read() {
    local raw="$1" s="$2" v stmt seg prog inplace p
    local -a pipes=() prog_operands=()
    local cmp='NR(==|>=|<=|>|<)[0-9]+'
    local conj="${cmp}(&&${cmp})*"
    local nr_re="^${conj}(,${conj})?"'(\{print(\$0)?\})?$'
    local hd_re='^[[:space:]]*/([^/\\]|\\.)+/[[:space:]]*,[[:space:]]*/([^/\\]|\\.)+/[[:space:]]*(\{[[:space:]]*print([[:space:]]+\$0)?[[:space:]]*\})?[[:space:]]*$'
    case "$s" in *awk*) ;; *) return 0 ;; esac
    v="$(_guard_dequoted_view "$raw" "$s")" || return 0
    while IFS= read -r stmt; do
        mapfile -t pipes < <(tr '|' '\n' <<<"$stmt")
        seg="${pipes[0]:-}"
        seg="${seg#"${seg%%[![:space:]]*}"}"
        case "$seg" in awk[[:space:]]*) ;; *) continue ;; esac
        _guard_program_operands awk "$seg" || continue
        [[ -n "$prog" && "${#prog_operands[@]}" == 1 ]] || continue
        case "${prog_operands[0]}" in - | /dev/stdin) continue ;; esac
        p="${prog//[[:space:]]/}"
        if [[ "$p" =~ $nr_re ]]; then
            guard_block "don't read a line range through 'awk' — use the Read tool with offset/limit: it returns numbered lines and registers the file for a later Edit. An awk program carrying an action, read from a program file, given a second operand, or fed by a pipe is a transform or a filter and untouched. If you genuinely need awk, run it yourself with !<command>."
        fi
        if [[ "$prog" =~ $hd_re && "${prog_operands[0]}" == *.md ]]; then
            guard_block "don't read a markdown section through an 'awk' range — use the section extractor: 'bash $(_guard_front_end) --emit md-section ${prog_operands[0]} \"<heading>\"' prints exactly the section under that heading, bounded by the next heading at its level. A range over a non-markdown file, a program carrying an action, or an awk fed by a pipe is untouched. If you genuinely need awk, run it yourself with !<command>."
        fi
    done < <(sed -E 's/\|\||&&|;/\n/g' <<<"$v")
}

guard_rule_sed_file() {
    local cmd="$1" s seg prog inplace
    local -a prog_operands=()
    s="$(guard_skeleton "$cmd" sq dq hd)"
    while IFS= read -r seg; do
        seg="${seg#"${seg%%[![:space:]]*}"}"
        case "$seg" in sed | sed[[:space:]]*) ;; *) continue ;; esac
        _guard_program_operands sed "$seg"
        if [[ "$inplace" == 1 ]]; then
            guard_block "don't rewrite a file with 'sed -i' — use the Edit tool: it replaces an exact string, fails loudly when the match is missing or ambiguous, and keeps the harness's view of the file current. If you genuinely need the in-place edit, run it yourself with !<command>."
        fi
        if [[ "${#prog_operands[@]}" -ge 1 ]]; then
            guard_block "don't read a file through 'sed' — use the Read tool (offset/limit for a line range): it returns numbered lines and registers the file for a later Edit. For a markdown section, the consumer's section extractor beats a line range. If you genuinely need sed, pipe into it or run it yourself with !<command>."
        fi
    done < <(guard_split_compound "$s")
    _guard_awk_read "$cmd" "$s"
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

# spec: guard-kit/SPEC.md §The generic ruleset — rule 18's roster membership test on one command word
_guard_on_ro_roster() {
    local b
    for b in "${GUARD_KIT_RO_BINS[@]}"; do
        [[ "$1" == "$b" ]] && return 0
    done
    return 1
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 18's xargs option walk: prints the index of the word naming the command xargs runs, nothing for a bare xargs, and returns non-zero on an option the walk does not recognize
_guard_xargs_command_index() {
    local tok want_arg=0 i n
    local -a toks
    read -ra toks <<<"$1"
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
            *) printf '%s' "$i"; return 0 ;;
        esac
    done
    return 0
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 18's xargs discriminator: xargs runs a command rather than filtering text, so the segment is read-only only when the command it runs is itself on the roster
_guard_is_ro_xargs() {
    local seg="${1#"${1%%[![:space:]]*}"}" idx cmdtok
    local -a toks
    idx="$(_guard_xargs_command_index "$seg")" || return 1
    [[ -z "$idx" ]] && return 0
    read -ra toks <<<"$seg"
    cmdtok="${toks[idx]}"
    [[ "$cmdtok" == xargs ]] && return 1
    case "$cmdtok" in echo | printf) return 0 ;; esac
    _guard_on_ro_roster "$cmdtok"
}

# spec: guard-kit/SPEC.md §The generic ruleset — one segment is a bare find listing: leads with find and carries none of find's declared write and execute forms, so the action roster has one literal
_guard_is_find_listing() {
    local seg="${1#"${1%%[![:space:]]*}"}"
    [[ "${seg%%[[:space:]]*}" == find ]] || return 1
    _guard_ro_invocation_clear find "${seg#find}" "$(_guard_segment_core "${seg#find}")"
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

# spec: guard-kit/SPEC.md §The generic ruleset — rules 9 and 11 fire only toward a dedicated search tool GUARD_KIT_SEARCH_TOOLS declares the harness build carries
_guard_has_search_tool() {
    local t
    for t in ${GUARD_KIT_SEARCH_TOOLS[@]+"${GUARD_KIT_SEARCH_TOOLS[@]}"}; do
        [[ "$t" == "$1" ]] && return 0
    done
    return 1
}

guard_rule_find_glob() {
    local cmd="$1" s
    _guard_has_search_tool Glob || return 0
    grep -qE '\$\(|<\(|>\(' <<<"$cmd" && return 0
    case "$cmd" in *'`'*) return 0 ;; esac
    s="$(guard_skeleton "$cmd" sq dq hd)"
    grep -qE '(&&|\|\||\||&|<|>)' <<<"$s" && return 0
    _guard_is_read_batch "$s" _guard_is_find_listing || return 0
    guard_block "don't list files with a bare 'find' — use the Glob tool: it returns matching paths (registered for a later Read) and needs no permission decision at all. If your toolset carries no Glob tool, keep 'find' and pipe the listing into a read-only consumer ('find <dir> -type f | sort'), which the read-only pipeline grant allows. This fires on a lone listing and on a ';'-sequence of them (a literal echo/printf banner between them is fine); a 'find' carrying an action predicate (-exec/-delete/…), piped into a consumer, or redirected is untouched. If you genuinely need find, run it yourself with !<command>."
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
    _guard_has_search_tool Grep || return 0
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
    guard_block "don't search with 'git grep' over the working tree — use the Grep tool: it returns matching lines (files registered for a later Read) and needs no permission decision at all. If your toolset carries no Grep tool, bare 'grep -rn <pattern> <path>' searches the same working tree. A 'git grep' naming a revision, searching the index (--cached), or piped into a consumer is untouched — those reach beyond the working tree the Grep tool sees. If you genuinely need git grep, run it yourself with !<command>."
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
    local seg="$1" mode="${2:-}" tok first=1 expect_arg=0 found=0
    local -a globals=()
    for tok in $seg; do
        if [[ "$found" == 1 ]]; then
            printf '%s\n' "$tok"
            continue
        fi
        if [[ "$first" == 1 ]]; then
            [[ "$tok" == git ]] || return 1
            first=0
            continue
        fi
        if [[ "$expect_arg" == 1 ]]; then expect_arg=0; continue; fi
        case "$tok" in
            -C | -c | --git-dir | --work-tree | --namespace | --exec-path | --config-env)
                globals+=("$tok")
                expect_arg=1 ;;
            --git-dir=* | --work-tree=* | --namespace=* | --exec-path=* | --config-env=*) globals+=("$tok") ;;
            -p | -P | --paginate | --no-pager | --bare | --no-replace-objects | \
                --literal-pathspecs | --no-literal-pathspecs | --glob-pathspecs | \
                --noglob-pathspecs | --icase-pathspecs | --no-optional-locks) globals+=("$tok") ;;
            -*) return 1 ;;
            *)
                # spec: guard-kit/SPEC.md §The generic ruleset — rule 2's reading of this same walk: 'globals' emits the global option words consumed before the subcommand, one per line, instead of the subcommand
                if [[ "$mode" == globals ]]; then
                    printf '%s\n' ${globals[@]+"${globals[@]}"}
                # spec: guard-kit/SPEC.md §The generic ruleset — rule 22's force arm reads this same walk: 'args' emits the subcommand and then every word after it, one per line
                elif [[ "$mode" == args ]]; then
                    printf '%s\n' "$tok"
                    found=1
                    continue
                else
                    printf '%s' "$tok"
                fi
                return 0 ;;
        esac
    done
    [[ "$found" == 1 ]]
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

# spec: guard-kit/SPEC.md §The generic ruleset — the read-only-segment test rules 15, 18 and 19 share, xargs discriminator included because xargs runs a command rather than filtering text; the roster-membership half only, the declared forms being _guard_ro_forms_clear's
_guard_is_ro_segment() {
    local seg="${1#"${1%%[![:space:]]*}"}" first
    first="${seg%%[[:space:]]*}"
    [[ -n "$first" ]] || return 1
    [[ "$first" == xargs ]] && { _guard_is_ro_xargs "$seg" || return 1; }
    _guard_on_ro_roster "$first"
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 18's kit declaration table for the default roster, in the declaration grammar; a binary absent here is undeclared
_guard_ro_forms_kit() {
    case "$1" in
        sort) printf '%s' '-o --output --compress-program' ;;
        uniq) printf '%s' 'pos:2' ;;
        find) printf '%s' '-delete -exec -execdir -ok -okdir -fprint -fprint0 -fprintf -fls' ;;
        rg) printf '%s' '--pre' ;;
        grep | egrep | fgrep | head | tail | cat | wc | cut | tr | nl | rev | tac | paste | comm | column | diff | jq | ls | xargs)
            printf 'none' ;;
        *) return 1 ;;
    esac
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 18's declared-forms test on one invocation: the consumer's GUARD_KIT_RO_FORMS entry else the kit table, option tokens against the dequoted words and pos:N against the skeleton core's words; non-zero on a matched form, an undeclared member, or any pos:N under xargs (the fourth argument), since xargs appends operands the segment does not show
_guard_ro_invocation_clear() {
    local bin="$1" forms tok w need pos
    local -a decl=() words=() skel=()
    if [[ -n "${GUARD_KIT_RO_FORMS[$bin]+x}" ]]; then
        forms="${GUARD_KIT_RO_FORMS[$bin]}"
    else
        forms="$(_guard_ro_forms_kit "$bin")" || return 1
    fi
    read -ra decl <<<"$forms"
    [[ "${#decl[@]}" -ge 1 ]] || return 1
    read -ra words <<<"${2//\\/}"
    read -ra skel <<<"${3:-}"
    for tok in "${decl[@]}"; do
        case "$tok" in
            none) ;;
            pos:*)
                need="${tok#pos:}"
                [[ "$need" =~ ^[1-9][0-9]*$ && "${4:-}" != xargs ]] || return 1
                pos=0
                for w in ${skel[@]+"${skel[@]}"}; do
                    case "$w" in -?*) ;; *) pos=$((pos + 1)) ;; esac
                done
                [[ "$pos" -lt "$need" ]] || return 1 ;;
            --?*)
                for w in ${words[@]+"${words[@]}"}; do
                    w="${w%%=*}"
                    [[ "$w" == --?* && "${tok#--}" == "${w#--}"* ]] && return 1
                done ;;
            -[[:alnum:]])
                for w in ${words[@]+"${words[@]}"}; do
                    [[ "$w" == -[!-]* && "$w" == *"${tok#-}"* ]] && return 1
                done ;;
            -?*)
                for w in ${words[@]+"${words[@]}"}; do
                    [[ "$w" == "$tok" ]] && return 1
                done ;;
            *) return 1 ;;
        esac
    done
    return 0
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 18's one declared-forms reader, called by rules 15 (exemption 3), 18 and 19 (clauses c and d) once their segment tests pass; reads the dequoted view aligned segment-for-segment with the skeleton, and withholds where the two cannot be aligned
_guard_ro_forms_clear() {
    local raw="$1" s="$2" v i cw dcw bin core idx xcmd
    local -a segs=() dsegs=() dtoks=()
    v="$(_guard_dequoted_view "$raw" "$s")" || return 1
    mapfile -t segs < <(guard_split_compound "$s")
    mapfile -t dsegs < <(guard_split_compound "$v")
    [[ "${#segs[@]}" == "${#dsegs[@]}" ]] || return 1
    for ((i = 0; i < ${#segs[@]}; i++)); do
        cw="$(_guard_command_word "${segs[i]}")"
        bin="${cw%%[[:space:]]*}"
        if [[ -z "$bin" ]] || ! _guard_on_ro_roster "$bin"; then continue; fi
        dcw="$(_guard_command_word "${dsegs[i]}")"
        core="$(_guard_segment_core "$cw")"
        _guard_ro_invocation_clear "$bin" "${dcw#"$bin"}" "${core#"$bin"}" || return 1
        [[ "$bin" == xargs ]] || continue
        idx="$(_guard_xargs_command_index "$dcw")" || return 1
        [[ -n "$idx" ]] || continue
        read -ra dtoks <<<"$dcw"
        xcmd="${dtoks[idx]//\\/}"
        case "$xcmd" in echo | printf) continue ;; esac
        _guard_on_ro_roster "$xcmd" || continue
        _guard_ro_invocation_clear "$xcmd" "${dtoks[*]:idx+1}" "" xargs || return 1
    done
    return 0
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
    local raw="$1" s="$2" tgt seg reads=0
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
    [[ "$reads" -ge 1 ]] && _guard_ro_forms_clear "$raw" "$s"
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
    _guard_is_ro_background "$raw" "$s" && return 0
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
    _guard_ro_forms_clear "$raw" "$s" || return 0
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
    _guard_ro_forms_clear "$raw" "$s" || return 0

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
    local seg lead arg v cmdseg w i=0
    local -a dsegs=() words=()
    v="$(_guard_dequoted_view "$raw" "$s")" && mapfile -t dsegs < <(guard_split_compound "$v")
    while IFS= read -r seg; do
        seg="${seg#"${seg%%[![:space:]]*}"}"
        cmdseg="$(_guard_command_word "${dsegs[i]:-$seg}")"
        i=$((i + 1))
        if [[ "${cmdseg%%[[:space:]]*}" == git ]]; then
            mapfile -t words < <(_guard_git_subcommand "$cmdseg" args)
            [[ "${words[0]:-}" == rm ]] || continue
            for w in "${words[@]:1}"; do
                w="${w//\\/}"
                case "$w" in
                    --) break ;;
                    --f | --fo | --for | --forc | --force | -[!-]*f* | -f*) ;;
                    *) continue ;;
                esac
                guard_block "don't force a 'git rm' with '$w' — the force flag is the one spelling of 'git rm' that destroys uncommitted work, silently when a committed grant matches it. Three exits: drop the flag, since 'git rm' refuses a file with local modifications and says so; use 'git rm --cached <path>' to untrack the file and keep it; or, where the loss is intended, run it yourself with !<command>."
            done
            continue
        fi
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

# spec: guard-kit/SPEC.md §The generic ruleset — rule 23's two decisions: arm (a) steers to the runner, arm (b) states the bash-only rule, and both name the runner through _guard_front_end so a consumer that vendors the kit elsewhere is told where its own copy is
_guard_block_interpreter() {
    local arm="$1" word="$2" src="$3" runner
    runner="$(_guard_front_end) --scratch-run"
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

# spec: guard-kit/SPEC.md §The generic ruleset — rule 24's regex literal for one run of a committed pattern: every metacharacter bracketed or escaped, so no pattern text reaches the regex engine as syntax
_guard_ere_literal() {
    local s="$1" out='' c k
    for ((k = 0; k < ${#s}; k++)); do
        c="${s:k:1}"
        case "$c" in
            '\') out+='[\]' ;;
            '^') out+='\^' ;;
            ']') out+='[]]' ;;
            '[' | '.' | '(' | ')' | '{' | '}' | '$' | '|' | '?' | '+' | '*') out+="[$c]" ;;
            *) out+="$c" ;;
        esac
    done
    printf '%s' "$out"
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 24's cleanness test on one word: the full word carries no '..' component, and the text opening the path does not begin with '/' or '~'; prints what the word reaches past
_guard_unclean_word() {
    case "/$1/" in */../*) printf "'%s', which carries a '..' component" "$1"; return 0 ;; esac
    case "$2" in
        /*) printf "'%s', an absolute path" "$1"; return 0 ;;
        '~'*) printf "'%s', a home-relative path" "$1"; return 0 ;;
    esac
    return 1
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 24 on one path-slot capture of the sentinel view: the first word is tested in the full shell word carrying it, and every later non-option word must re-match the slot's own token and be clean itself; prints the reach
_guard_slot_capture() {
    local sv="$1" off="$2" len="$3" token="$4" start="$5" right="$6"
    local cap firstw pre post word lead rest ext w
    local -a later=()
    cap="${sv:off:len}"
    firstw="${cap%%[[:space:]]*}"
    pre="${sv:0:off}"
    pre="${pre##*[[:space:]]}"
    post="${sv:off}"
    post="${post%%[[:space:]]*}"
    word="$pre$post"
    word="${word//$'\x01'/ }"
    word="${word//$'\x02'/$'\t'}"
    lead="$firstw"
    [[ -z "$lead" && "$start" == 1 ]] && lead="$right"
    _guard_unclean_word "$word" "$lead" && return 0
    rest="${cap:${#firstw}}"
    if [[ "$rest" == *[![:space:]]* && "$rest" != *[[:space:]] ]]; then
        ext="${sv:off+len}"
        rest+="${ext%%[[:space:]]*}"
    fi
    read -ra later <<<"$rest"
    for w in ${later[@]+"${later[@]}"}; do
        case "$w" in -*) continue ;; esac
        w="${w//$'\x01'/ }"
        w="${w//$'\x02'/$'\t'}"
        if ! guard_allow_match "$w" "$token"; then
            printf "'%s', a second operand outside the slot" "$w"
            return 0
        fi
        _guard_unclean_word "$w" "$w" && return 0
    done
    return 1
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 24 on one segment, reading the caller's inners: the skeleton's words say which words are redirects, the dequoted words aligned one-for-one carry the content, the harness view strips the wrappers, and each matching committed pattern carrying a path slot is parsed leftmost-greedy; prints the reach and returns 0, 1 when clean, 2 when the segment cannot be decided
_guard_slot_segment() {
    local k n sv rv inner pat j c lit left right re g off len reach nstars
    local -a sw=() dw=() kept=() caps=() slot=() token=() start=() rlit=() litlen=()
    read -ra sw <<<"$1"
    read -ra dw <<<"$2"
    [[ "${#sw[@]}" == "${#dw[@]}" ]] || return 2
    n=${#sw[@]}
    for ((k = 0; k < n; k++)); do
        case "${sw[k]}" in
            '<<<' | '>' | '>>' | '<' | '&>' | '&>>' | '>&' | [0-9]'>' | [0-9]'>>' | [0-9]'<' | [0-9]'>&')
                k=$((k + 1))
                continue ;;
            *'<<'?*) [[ "${sw[k]}" == '<<<'?* ]] && continue; return 2 ;;
            '>'?* | '<'?* | '&>'?* | [0-9]'>'?* | [0-9]'<'?*) continue ;;
        esac
        kept+=("${dw[k]}")
    done
    [[ "${#kept[@]}" -ge 1 ]] || return 1
    sv="$(_guard_harness_view "${kept[*]}")"
    sv="${sv//\\/}"
    rv="${sv//$'\x01'/ }"
    rv="${rv//$'\x02'/$'\t'}"
    for inner in "${inners[@]}"; do
        pat="${inner//:\*/\*}"
        case "$pat" in *'*'*) ;; *) continue ;; esac
        case "$pat" in *'?'* | *'['*) continue ;; esac
        guard_allow_match "$rv" "$pat" || continue
        re='^'
        lit=''
        slot=() token=() start=() rlit=() litlen=()
        for ((j = 0; j < ${#pat}; j++)); do
            c="${pat:j:1}"
            if [[ "$c" != '*' ]]; then
                lit+="$c"
                continue
            fi
            re+="$(_guard_ere_literal "$lit")(.*)"
            litlen+=("${#lit}")
            lit=''
            left="${pat:0:j}"
            left="${left##*[[:space:]]}"
            right="${pat:j+1}"
            right="${right%%[[:space:]]*}"
            token+=("$left*$right")
            if [[ "$left*$right" == */* ]]; then slot+=(1); else slot+=(0); fi
            if [[ -z "$left" ]]; then start+=(1); else start+=(0); fi
            rlit+=("${right%%\**}")
        done
        re+="$(_guard_ere_literal "$lit")\$"
        [[ "$rv" =~ $re ]] || continue
        caps=("${BASH_REMATCH[@]:1}")
        nstars=${#slot[@]}
        off=0
        for ((g = 0; g < nstars; g++)); do
            off=$((off + litlen[g]))
            len=${#caps[g]}
            if [[ "${slot[g]}" == 1 ]] \
                && reach="$(_guard_slot_capture "$sv" "$off" "$len" "${token[g]}" "${start[g]}" "${rlit[g]}")"; then
                printf "'Bash(%s)' matches this command only through a path slot that reaches %s" "$inner" "$reach"
                return 0
            fi
            off=$((off + len))
        done
    done
    return 1
}

# spec: guard-kit/SPEC.md §The generic ruleset — rule 24's test, a block for rule 24 and a predicate for rules 4 and 7: prints the reach and returns 0, returns 1 when every matching slot is clean, 2 when the command cannot be decided; 'predicate' reads a segment carrying a quoted statement separator whole rather than skipping it
_guard_slot_reach() {
    local raw="$1" mode="${2:-}" live s v i dseg rc undecided=0
    local -a inners=() segs=() dsegs=()
    live="$(guard_skeleton "$raw" sq hdq)"
    case "$live" in *'$'* | *'<('* | *'>('*) return 2 ;; esac
    case "$raw" in *'`'*) return 2 ;; esac
    mapfile -t inners < <(_guard_allow_inners)
    [[ "${#inners[@]}" -ge 1 ]] || return 1
    s="$(guard_skeleton "$raw" sq dq hd)"
    v="$(_guard_dequoted_view "$raw" "$s")" || return 2
    mapfile -t segs < <(guard_split_compound "$s")
    mapfile -t dsegs < <(guard_split_compound "$v")
    [[ "${#segs[@]}" == "${#dsegs[@]}" ]] || return 2
    for ((i = 0; i < ${#segs[@]}; i++)); do
        dseg="${dsegs[i]}"
        if [[ "$dseg" == *[$'\x03\x04\x05']* ]]; then
            if [[ "$mode" != predicate ]]; then
                undecided=1
                continue
            fi
            dseg="${dseg//$'\x03'/;}"
            dseg="${dseg//$'\x04'/|}"
            dseg="${dseg//$'\x05'/&}"
        fi
        rc=0
        _guard_slot_segment "${segs[i]}" "$dseg" || rc=$?
        [[ "$rc" == 0 ]] && return 0
        [[ "$rc" == 2 ]] && undecided=1
    done
    [[ "$undecided" == 1 ]] && return 2
    return 1
}

# spec: guard-kit/SPEC.md §The generic ruleset — rules 4 and 7's grant test on a rewritten command: every segment of its dequoted view matches a committed allow pattern, and rule 24's test finds no reach and can decide; a failed settings read, an unalignable view or an undecidable bound fails, so a rewrite never turns a missing read into an allow
_guard_rewrite_granted() {
    local cmd="$1" s v seg inner hit rc=0
    local -a inners=()
    mapfile -t inners < <(_guard_allow_inners)
    [[ "${#inners[@]}" -ge 1 ]] || return 1
    s="$(guard_skeleton "$cmd" sq dq hd)"
    v="$(_guard_dequoted_view "$cmd" "$s")" || return 1
    while IFS= read -r seg; do
        seg="${seg//$'\x01'/ }"
        seg="${seg//$'\x02'/$'\t'}"
        seg="${seg//$'\x03'/;}"
        seg="${seg//$'\x04'/|}"
        seg="${seg//$'\x05'/&}"
        seg="${seg#"${seg%%[![:space:]]*}"}"
        seg="${seg%"${seg##*[![:space:]]}"}"
        [[ -z "$seg" ]] && continue
        hit=0
        for inner in "${inners[@]}"; do
            if guard_allow_match "$seg" "$inner"; then
                hit=1
                break
            fi
        done
        [[ "$hit" == 1 ]] || return 1
    done < <(guard_split_compound "$v")
    _guard_slot_reach "$cmd" predicate >/dev/null || rc=$?
    [[ "$rc" == 1 ]]
}

guard_rule_grant_path_slot() {
    local reach
    case "$1" in */*) ;; *) return 0 ;; esac
    reach="$(_guard_slot_reach "$1")" || return 0
    guard_block "don't reach past a committed grant's path slot — $reach. A Bash rule's '*' spans '/', '..' and whole words, so a grant written for one directory reaches paths its author never named. Spell the path inside the pattern's reach, or, if you genuinely need this command, run it yourself with !<command>."
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
    guard_rule_grant_path_slot "$cmd"
}
