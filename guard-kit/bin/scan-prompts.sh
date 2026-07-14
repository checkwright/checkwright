#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §scan-prompts — rank recurring prompt sources from the friction log
set -uo pipefail

BIN="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../lib/guard.sh
source "$BIN/../lib/guard.sh"

LOG="$GUARD_KIT_LOG"
SETTINGS="$GUARD_KIT_SETTINGS"

COUNT=0
case "${1:-}" in
    --count) COUNT=1 ;;
    "") ;;
    *) LOG="$1" ;;
esac

if [[ ! -s "$LOG" ]]; then
    [[ "$COUNT" -eq 1 ]] && { echo "0/0"; exit 0; }
    echo "PROMPT-FRICTION: clean (no fall-through commands logged this iteration)"
    exit 0
fi

mapfile -t ALLOW < <(jq -r '.permissions.allow[]?
    | select(startswith("Bash("))
    | sub("^Bash\\(";"") | sub("\\)$";"")' "$SETTINGS" 2>/dev/null || true)

GIT_RO=" status log diff show blame branch tag remote ls-files ls-remote rev-parse describe shortlog cat-file for-each-ref worktree reflog "
DOCKER_RO=" ps images logs inspect version "

strip_prefix() {
    local c="$1"
    c="${c#sudo }"
    c="${c#timeout }"
    c="${c#[0-9]* }"
    printf '%s' "$c"
}

allowed() {
    local c t1 t2 rest p glob
    c="$(strip_prefix "$1")"
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
    return 1
}

pattern_of() {
    local c t1 t2 rest
    c="$(strip_prefix "$1")"
    t1="${c%%[[:space:]]*}"
    rest="${c#"$t1"}"; rest="${rest#"${rest%%[![:space:]]*}"}"
    t2="${rest%%[[:space:]]*}"
    case "$t1" in
        git | gh | cargo | docker | npm | bun | yarn | pnpm | bash | sh | kubectl | python | python3)
            if [[ -n "$t2" ]]; then printf '%s %s' "$t1" "$t2"; else printf '%s' "$t1"; fi ;;
        *) printf '%s' "$t1" ;;
    esac
}

declare -A counts
total=0
while IFS= read -r line; do
    [[ -z "$line" ]] && continue
    allowed "$line" && continue
    key="$(pattern_of "$line")"
    [[ -z "$key" ]] && continue
    counts["$key"]=$((${counts["$key"]:-0} + 1))
    total=$((total + 1))
done < "$LOG"

distinct=${#counts[@]}
logged="$(wc -l < "$LOG" 2>/dev/null | tr -d ' ')"

if [[ "$COUNT" -eq 1 ]]; then
    echo "$distinct/$total"
    exit 0
fi

if [[ "$distinct" -eq 0 ]]; then
    echo "PROMPT-FRICTION: clean ($logged fall-through(s) logged, all allowlisted / auto-allowed)"
    exit 0
fi

echo "=== Prompt friction (advisory — triage at close, not a gate) ==="
echo "$total prompting call(s) across $distinct pattern(s), from $logged logged fall-through(s)."
echo "log: $LOG"
echo
for key in "${!counts[@]}"; do
    printf '%s\t%s\n' "${counts[$key]}" "$key"
done | sort -rn | while IFS=$'\t' read -r n key; do
    printf '%5dx  %s\n' "$n" "$key"
done
echo
echo "Triage each by the criterion (guard-kit/SPEC.md §The triage criterion):"
echo "  (a) allowlist entry — safe & already in the form to reinforce,"
echo "  (b) guard rule — a better form exists (steer), or logic a glob can't express,"
echo "  (c) habit change — a true one-off."
echo "Then clear the log:  : > $LOG"
exit 0
