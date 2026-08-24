#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §scan-prompts — rank recurring prompt sources from the friction log
set -uo pipefail

BIN="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../lib/guard.sh
source "$BIN/../lib/guard.sh"

LOG="$GUARD_KIT_LOG"
SETTINGS="$GUARD_KIT_SETTINGS"
SETTINGS_LOCAL="$GUARD_KIT_SETTINGS_LOCAL"

COUNT=0
while [[ "$#" -gt 0 ]]; do
    case "$1" in
        --count) COUNT=1 ;;
        "") ;;
        *) LOG="$1" ;;
    esac
    shift
done

if [[ ! -s "$LOG" ]]; then
    [[ "$COUNT" -eq 1 ]] && { echo "0/0"; exit 0; }
    echo "PROMPT-FRICTION: clean (no fall-through commands logged this iteration)"
    exit 0
fi

read_allow() {
    jq -r '.permissions.allow[]?
        | select(startswith("Bash("))
        | sub("^Bash\\(";"") | sub("\\)$";"")' "$1" 2>/dev/null || true
}
mapfile -t ALLOW       < <(read_allow "$SETTINGS")
mapfile -t ALLOW_LOCAL < <(read_allow "$SETTINGS_LOCAL")

GIT_RO=" status log diff show blame branch tag remote ls-files ls-remote rev-parse describe shortlog cat-file for-each-ref worktree reflog "
DOCKER_RO=" ps images logs inspect version "

strip_prefix() {
    local c="$1"
    c="${c#sudo }"
    c="${c#timeout }"
    c="${c#[0-9]* }"
    printf '%s' "$c"
}

# spec: guard-kit/SPEC.md §scan-prompts — one segment granted by the committed allowlist, a harness read-only git/docker built-in, or (set=local) the uncommitted overlay
seg_allowed() {
    local seg="$1" set="$2" c t1 t2 rest p glob
    c="$(strip_prefix "$seg")"
    c="${c#"${c%%[![:space:]]*}"}"; c="${c%"${c##*[![:space:]]}"}"
    [[ -z "$c" ]] && return 0
    t1="${c%%[[:space:]]*}"
    rest="${c#"$t1"}"; rest="${rest#"${rest%%[![:space:]]*}"}"
    t2="${rest%%[[:space:]]*}"
    [[ "$t1" == "git"    && "$GIT_RO"    == *" $t2 "* ]] && return 0
    [[ "$t1" == "docker" && "$DOCKER_RO" == *" $t2 "* ]] && return 0
    for p in "${ALLOW[@]}"; do
        [[ -z "$p" ]] && continue
        glob="${p//:\*/\*}"
        # shellcheck disable=SC2053  # intentional glob match: $glob is a pattern, not a literal
        [[ "$c" == $glob ]] && return 0
    done
    if [[ "$set" == local ]]; then
        for p in "${ALLOW_LOCAL[@]}"; do
            [[ -z "$p" ]] && continue
            glob="${p//:\*/\*}"
            # shellcheck disable=SC2053  # intentional glob match: $glob is a pattern, not a literal
            [[ "$c" == $glob ]] && return 0
        done
    fi
    return 1
}

# spec: guard-kit/SPEC.md §scan-prompts — granted only if EVERY segment is; a whole-string glob spanning a compound the harness would split and refuse must not read as allowed
allowed() {
    local set="$1" cmd="$2" skel seg
    skel="$(sed -E "s/'[^']*'/SQ/g; s/\"[^\"]*\"/DQ/g" <<<"$cmd")"
    while IFS= read -r seg; do
        [[ -z "${seg// }" ]] && continue
        seg_allowed "$seg" "$set" || return 1
    done < <(guard_split_compound "$skel")
    return 0
}

# spec: guard-kit/SPEC.md §scan-prompts — the key's write-shape suffix: the segment's own write-redirect operator normalized to > or >>, the descriptor dropped and an fd-dup excluded on rule 17's own test, since an fd-dup is not a redirect to a file
redirect_op_of() {
    local pair op tgt
    while IFS= read -r pair; do
        [[ -z "$pair" ]] && continue
        pair="${pair#"${pair%%[!0-9]*}"}"
        if [[ "$pair" == '>>'* ]]; then op='>>'; tgt="${pair#>>}"; else op='>'; tgt="${pair#>}"; fi
        tgt="${tgt#"${tgt%%[![:space:]]*}"}"
        case "$tgt" in '&'* | '') continue ;; esac
        printf '%s' "$op"
        return 0
    done < <(_guard_redirect_pairs "$1")
    return 0
}

# spec: guard-kit/SPEC.md §scan-prompts — the ranking key: leading binary, plus subcommand for the common multi-command binaries, plus the write-shape suffix; word and suffix both come from the FIRST segment, so a key can never attribute a write to a command that performs none
pattern_of() {
    local c t1 t2 rest key op skel
    skel="$(guard_skeleton "$1" sq dq hd)"
    c="$(strip_prefix "$(guard_split_compound "$skel" | head -1)")"
    c="${c#"${c%%[![:space:]]*}"}"
    t1="${c%%[[:space:]]*}"
    rest="${c#"$t1"}"; rest="${rest#"${rest%%[![:space:]]*}"}"
    t2="${rest%%[[:space:]]*}"
    # spec: guard-kit/SPEC.md §scan-prompts — a write redirect in subcommand position is re-homed into the suffix below, never doubled into both tokens; a read redirect is not the suffix's subject and is left exactly where it keys today
    case "$t2" in '>'* | '&>'* | [0-9]'>'*) t2='' ;; esac
    case "$t1" in
        git | gh | cargo | docker | npm | bun | yarn | pnpm | bash | sh | kubectl | python | python3)
            if [[ -n "$t2" ]]; then key="$t1 $t2"; else key="$t1"; fi ;;
        *) key="$t1" ;;
    esac
    op="$(redirect_op_of "$c")"
    [[ -n "$op" ]] && key="$key $op"
    printf '%s' "$key"
}

declare -A counts local_counts
total=0
local_total=0
distinct=0
local_distinct=0
# comment-tier-exempt: ${#assoc[@]} on a still-empty associative array trips set -u on some bash, so distinct/local_distinct are counted incrementally on first sight of a key
while IFS= read -r line; do
    [[ -z "$line" ]] && continue
    key="$(pattern_of "$line")"
    [[ -z "$key" ]] && continue
    if allowed committed "$line"; then
        continue                                   # committed allowlist covers it: reinforced, no worklist entry
    elif allowed local "$line"; then
        [[ -z "${local_counts[$key]:-}" ]] && local_distinct=$((local_distinct + 1))
        local_counts["$key"]=$((${local_counts["$key"]:-0} + 1))   # granted only by the uncommitted overlay: did not prompt, but promote-or-prune
        local_total=$((local_total + 1))
    else
        [[ -z "${counts[$key]:-}" ]] && distinct=$((distinct + 1))
        counts["$key"]=$((${counts["$key"]:-0} + 1))               # nothing grants it: a true prompt
        total=$((total + 1))
    fi
done < "$LOG"

logged="$(wc -l < "$LOG" 2>/dev/null | tr -d ' ')"

if [[ "$COUNT" -eq 1 ]]; then
    echo "$distinct/$total"
    exit 0
fi

rank_section() {
    local -n src="$1"
    local key
    for key in "${!src[@]}"; do
        printf '%s\t%s\n' "${src[$key]}" "$key"
    done | sort -rn | while IFS=$'\t' read -r n key; do
        printf '%5dx  %s\n' "$n" "$key"
    done
}

overlay_section() {
    [[ "$local_distinct" -eq 0 ]] && return 0
    echo
    echo "--- Overlay-covered (advisory — did NOT prompt; granted only by the uncommitted"
    echo "    local overlay $SETTINGS_LOCAL). Promote the recurring-safe patterns to the"
    echo "    committed allowlist or prune the one-offs (guard-kit/SPEC.md §The triage criterion): ---"
    echo "$local_total call(s) across $local_distinct pattern(s)."
    rank_section local_counts
}

if [[ "$distinct" -eq 0 ]]; then
    echo "PROMPT-FRICTION: clean ($logged fall-through(s) logged, all allowlisted / auto-allowed)"
    overlay_section
    exit 0
fi

echo "=== Prompt friction (advisory — triage at close, not a gate) ==="
echo "$total prompting call(s) across $distinct pattern(s), from $logged logged fall-through(s)."
echo "log: $LOG"
echo
rank_section counts
echo
echo "Triage each by the criterion (guard-kit/SPEC.md §The triage criterion):"
echo "  (a) allowlist entry — safe & already in the form to reinforce,"
echo "  (b) guard rule — a better form exists (steer), or logic a glob can't express,"
echo "  (c) habit change — a true one-off."
overlay_section
echo
echo "Then clear the log:  : > $LOG"
exit 0
