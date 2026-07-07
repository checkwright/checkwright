# shellcheck shell=bash
# spec: guard-kit/SPEC.md §The guard framework — hook primitives + generic ruleset; no project rule content

_frik_cfg="${GUARD_KIT_CONFIG_FILE:-${GATE_SDK_GATES_DIR:-scripts}/guard-config.sh}"
if [[ -f "$_frik_cfg" ]]; then
    # shellcheck source=/dev/null  # consumer config path is resolved at runtime
    source "$_frik_cfg"
fi
unset _frik_cfg

: "${GUARD_KIT_LOG:=${GATE_SDK_WORKFLOW_DIR:-.workflow}/prompt-friction.log}"
: "${GUARD_KIT_WAKEUP_LOG:=${GATE_SDK_WORKFLOW_DIR:-.workflow}/wakeup-attempts.log}"
: "${GUARD_KIT_SETTINGS:=.claude/settings.json}"
: "${GUARD_KIT_SETTINGS_LOCAL:=.claude/settings.local.json}"
declare -p GUARD_KIT_RO_SCRIPTS >/dev/null 2>&1 || GUARD_KIT_RO_SCRIPTS=("check-*.sh")
declare -p GUARD_KIT_SCRATCH_DIRS >/dev/null 2>&1 || GUARD_KIT_SCRATCH_DIRS=(".tmp")
declare -p GUARD_KIT_RO_BINS >/dev/null 2>&1 || GUARD_KIT_RO_BINS=(
    grep egrep fgrep rg head tail cat wc sort uniq cut tr nl rev tac paste comm column diff jq find ls
)

guard_read_command() {
    local input cmd
    input="$(cat 2>/dev/null)" || return 1
    cmd="$(printf '%s' "$input" | jq -r '.tool_input.command // empty' 2>/dev/null)" || return 1
    [[ -z "$cmd" ]] && return 1
    printf '%s' "$cmd"
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

# spec: guard-kit/SPEC.md §The generic ruleset — rules 1-10 below; order is load-bearing
guard_rule_cd_compound() {
    local cmd="$1"
    if grep -qE '(^|[;&|(])[[:space:]]*cd[[:space:]]' <<<"$cmd" && grep -qE '[;&|]' <<<"$cmd"; then
        guard_block "don't use 'cd' in a compound command (permission prompts / cwd drift). Pass absolute paths, or 'git -C <dir>' for git."
    fi
}

guard_rule_git_c_root() {
    local cmd="$1"
    if grep -qF "git -C $PWD " <<<"$cmd"; then
        guard_block "drop 'git -C $PWD ' — cwd is the repo root, so the bare 'git <subcommand>' form is allowlisted and won't re-prompt. Reserve 'git -C <dir>' for a different repo."
    fi
}

guard_rule_scratch_redirect() {
    local cmd="$1"
    if grep -qE '(^|[[:space:]])([0-9]*|&)>>?[[:space:]]*[^[:space:]/|&]+\.(err|out|log)([[:space:]]|$)' <<<"$cmd"; then
        guard_block "don't redirect scratch to a bare repo-root filename (e.g. 2> op.err) — it pollutes cwd and risks a 'git add -A'. Send it to a gitignored scratch dir (e.g. ${GUARD_KIT_SCRATCH_DIRS[0]}/<name>.err)."
    fi
}

guard_rule_abs_script() {
    local cmd="$1" rest base g relcmd
    case "$cmd" in
        "bash $PWD/"*) rest="${cmd#bash "$PWD/"}" ;;
        "$PWD/"*)      rest="${cmd#"$PWD/"}" ;;
        *)             return 0 ;;
    esac
    rest="${rest%%[[:space:]]*}"            # first token = repo-relative script path
    case "$rest" in *.sh) ;; *) return 0 ;; esac   # only .sh scripts; rule 5 handles the rest
    base="${rest##*/}"
    relcmd="${cmd//"$PWD/"/}"               # strip every repo-root prefix
    for g in "${GUARD_KIT_RO_SCRIPTS[@]}"; do
        # shellcheck disable=SC2053  # intentional glob match: $g is a pattern, not a literal
        if [[ "$base" == $g || "$rest" == $g ]]; then
            guard_rewrite "$relcmd" "abs repo read-only script normalized to relative (${GUARD_NAME:-guard})"
        fi
    done
    guard_block "use the repo-relative form '$rest' (cwd is the repo root) — it's allowlisted and won't re-prompt; the absolute spelling isn't. If you truly need the absolute path, run it yourself with !<command>."
}

guard_rule_abs_prefix() {
    local cmd="$1"
    [[ "$cmd" == git\ * ]] && return 0
    if grep -qF "$PWD/" <<<"$cmd"; then
        guard_block "drop the repo-root absolute prefix '$PWD/' — cwd is the repo root, so the repo-relative path is allowlisted and won't re-prompt; the absolute spelling isn't. If you truly need the absolute path, run it yourself with !<command>."
    fi
}

guard_rule_expansion() {
    local cmd="$1" sqexp expn
    sqexp="$(sed -E "s/'[^']*'//g" <<<"$cmd")"
    if grep -qE '\$\{|\$\(|<\(|\$[A-Za-z_]' <<<"$sqexp"; then
        guard_block "avoid shell variables/expansions (\$VAR, \${...}, \$(...), <(...)) — the harness prompts on every expansion and no allowlist entry can suppress it. Inline the literal path, use a relative path, or 'git -C <dir>'. If you genuinely need the expansion, run it yourself with !<command>."
    fi
    expn="$(sed -E "s/'[^']*'//g; s/\"[^\"]*\"//g" <<<"$cmd")"
    if grep -qE '(^|[;(]|&&|\|\|)[[:space:]]*[A-Za-z_][A-Za-z0-9_]*=[^[:space:];|&]*[[:space:]]*($|;)' <<<"$expn"; then
        guard_block "avoid shell variable assignments (NAME=value; ... \$NAME) — they force a permission prompt that can't be allowlisted. Inline the literal value/path at each use site, or 'git -C <dir>'. If you genuinely need it, run it yourself with !<command>."
    fi
}

guard_rule_brace_glyph() {
    local cmd="$1" sqstripped resid ph='{}' q="'{}'"
    sqstripped="$(sed -E "s/'[^']*'//g" <<<"$cmd")"
    case "$sqstripped" in *'{'*) ;; *) return 0 ;; esac
    resid="${sqstripped//"$ph"/}"
    case "$resid" in
        *'{'* | *'}'*) ;;   # a non-placeholder brace remains: fall to the blocks
        *) guard_rewrite "${cmd//"$ph"/"$q"}" \
               "bare {} placeholder single-quoted so the harness matcher passes it (${GUARD_NAME:-guard})" ;;
    esac
    if grep -qF '@{' <<<"$sqstripped"; then
        guard_block "spell out the git-ref shorthand '@{...}' — the harness prompts on the '{' glyph. Use 'origin/<branch>..HEAD' for '@{u}..', or the resolved ref/hash for a reflog form."
    fi
    if grep -qE '\{[^}]*(,|\.\.)[^}]*\}' <<<"$sqstripped"; then
        guard_block "write out the brace expansion '{a,b}'/'{a..b}' — the harness prompts on the '{' glyph and no allowlist entry suppresses it. Spell the members (e.g. 'mkdir -p a/b a/c') or use a loop for a long range."
    fi
    guard_block "single-quote the '{' if it's literal (an awk/sed program in double quotes), or write it out if it expands — the harness prompts on every bare '{' glyph before allowlist matching."
}

guard_rule_truncate_scratch() {
    local cmd="$1"
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
    grep -qE '\$\(|<\(|>\(' <<<"$raw" && return 0
    case "$raw" in *'`'*) return 0 ;; esac
    local s
    s="$(sed -E "s/'[^']*'//g; s/\"[^\"]*\"//g" <<<"$raw")"
    grep -q "['\"]" <<<"$s" && return 0
    grep -qE '(&&|\|\||;|&)' <<<"$s" && return 0
    local tgt
    while read -r tgt; do
        [[ -z "$tgt" ]] && continue
        case "$tgt" in
            /dev/null | '&'[0-9]*) ;;
            *) return 0 ;;
        esac
    done < <(grep -oE '[0-9]*>>?[[:space:]]*[^[:space:]|]+' <<<"$s" \
        | sed -E 's/^[0-9]*>>?[[:space:]]*//')
    if grep -qE '(^|[[:space:]])find([[:space:]]|$)' <<<"$s" \
        && grep -qE '\-(exec|execdir|ok|delete)\b' <<<"$s"; then
        return 0
    fi
    local seg first b matched
    while IFS= read -r seg; do
        seg="${seg#"${seg%%[![:space:]]*}"}"
        [[ -z "$seg" ]] && continue
        first="${seg%%[[:space:]]*}"
        matched=0
        for b in "${GUARD_KIT_RO_BINS[@]}"; do
            [[ "$first" == "$b" ]] && { matched=1; break; }
        done
        [[ "$matched" == 1 ]] || return 0
    done < <(tr '|' '\n' <<<"$s")
    guard_allow "read-only search pipeline (${GUARD_NAME:-guard} auto-allow)"
}

guard_rule_allowlist_chain() {
    local cmd="$1"
    command -v jq >/dev/null 2>&1 || return 0
    [[ -f "$GUARD_KIT_SETTINGS" ]] || return 0

    local -a allow_entries
    mapfile -t allow_entries < <(jq -r '.permissions.allow[]?' "$GUARD_KIT_SETTINGS" 2>/dev/null) || return 0
    [[ ${#allow_entries[@]} -gt 0 ]] || return 0

    local e inner
    local -a bare_leads pattern_inners
    for e in "${allow_entries[@]}"; do
        case "$e" in Bash\(*\)) inner="${e#Bash(}"; inner="${inner%)}" ;; *) continue ;; esac
        [[ -z "$inner" ]] && continue
        pattern_inners+=("$inner")
        case "$inner" in *'*'*) ;; *) bare_leads+=("$inner") ;; esac
    done
    [[ ${#bare_leads[@]} -gt 0 ]] || return 0

    local skel
    skel="$(sed -E "s/'[^']*'//g; s/\"[^\"]*\"//g" <<<"$cmd")"

    local -a segs
    mapfile -t segs < <(sed -E 's/\|\||&&|;|\|/\n/g' <<<"$skel")

    local lead="${segs[0]}"
    lead="${lead#"${lead%%[![:space:]]*}"}"; lead="${lead%"${lead##*[![:space:]]}"}"

    local lead_core
    lead_core="$(sed -E 's/[[:space:]]*[0-9]*(>>?|<)[[:space:]]*(&?[0-9-]+|[^[:space:]]+)?//g' <<<"$lead")"
    lead_core="${lead_core#"${lead_core%%[![:space:]]*}"}"; lead_core="${lead_core%"${lead_core##*[![:space:]]}"}"

    local bl matched_lead=0
    for bl in "${bare_leads[@]}"; do
        [[ "$lead_core" == "$bl" ]] && { matched_lead=1; break; }
    done
    [[ "$matched_lead" == 1 ]] || return 0

    local steer="run '$lead_core' bare — it's a statically allowlisted command, but the decoration (chaining or a redirect) forces a permission prompt no allowlist entry suppresses. Run the allowlisted command on its own; issue the rest as separate calls."

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

guard_generic_rules() {
    local cmd="$1"
    guard_rule_cd_compound "$cmd"
    guard_rule_git_c_root "$cmd"
    guard_rule_scratch_redirect "$cmd"
    guard_rule_abs_script "$cmd"
    guard_rule_abs_prefix "$cmd"
    guard_rule_expansion "$cmd"
    guard_rule_brace_glyph "$cmd"
    guard_rule_truncate_scratch "$cmd"
    guard_rule_ro_pipeline "$cmd"
    guard_rule_allowlist_chain "$cmd"
}
