#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §scan-prompts — the headline count filters the friction log against BOTH the committed allowlist and the local overlay, matching per compound segment as the harness does; an overlay-only grant did not prompt (kept off the headline, surfaced in the advisory promote-or-prune section), and a whole-string glob spanning a compound the harness would split and refuse does not read as allowed
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT" || exit 2
SCAN="guard-kit/bin/scan-prompts.sh"
[[ -x "$SCAN" ]] || { echo "scan-prompts.test: scanner not found: $SCAN"; exit 2; }
command -v jq >/dev/null 2>&1 || { echo "scan-prompts.test: jq not found on PATH"; exit 2; }

sb="$(mktemp -d)"
trap 'rm -rf "$sb"' EXIT
mkdir -p "$sb/.claude"
# committed: a bare grant and a glob whose whole-string reach spans a compound
printf '%s\n' '{ "permissions": { "allow": ["Bash(git status:*)", "Bash(ls)"] } }' \
    > "$sb/.claude/settings.json"
# local overlay: grants a command the committed set does not
printf '%s\n' '{ "permissions": { "allow": ["Bash(npm test)"] } }' \
    > "$sb/.claude/settings.local.json"

LOG="$sb/friction.log"
{
    echo 'npm test'                 # granted ONLY by the local overlay: did not prompt
    echo 'git status && rm -rf x'   # committed glob matches the whole string, but the harness splits and refuses 'rm'
    echo 'git status && ls'         # every segment committed: silently granted, off every list
    echo 'make build'               # nothing grants it: a true prompt
} > "$LOG"

run() { GUARD_KIT_SETTINGS="$sb/.claude/settings.json" \
        GUARD_KIT_SETTINGS_LOCAL="$sb/.claude/settings.local.json" \
        GUARD_KIT_LOG="$LOG" bash "$SCAN" "$@"; }

fails=0
assert_has()    { grep -qF -- "$2" <<<"$3" || { echo "FAIL [$1]: expected present: $2"; fails=$((fails + 1)); }; }
assert_absent() { grep -qF -- "$2" <<<"$3" && { echo "FAIL [$1]: expected absent: $2"; fails=$((fails + 1)); }; return 0; }

full="$(run "$LOG")"

# The overlay-only grant did not prompt: it is off the headline and in the advisory section.
assert_has  overlay-off-headline 'Overlay-covered' "$full"
overlay_body="${full#*Overlay-covered}"
assert_has  overlay-listed 'npm test' "$overlay_body"

# The compound the harness would split and refuse is a true prompt, despite the
# whole-string glob 'git status*' spanning it — this is the compound-splitting fix.
head_body="${full%%Overlay-covered*}"
assert_has  compound-prompts 'git status' "$head_body"
assert_has  compound-prompts-make 'make' "$head_body"

# A compound whose every segment is committed is silently granted — on no list.
assert_absent all-committed-omitted 'git status && ls' "$full"

# --count is the true prompt count: the two genuine prompts (the split compound
# and 'make'), never the overlay-only grant.
count="$(run --count)"
[[ "$count" == "2/2" ]] || { echo "FAIL [count]: expected 2/2, got '$count'"; fails=$((fails + 1)); }

# Baseline: with the overlay-only command as the sole log line, the headline is
# clean (it did not prompt) yet the advisory worklist still surfaces it.
soleLOG="$sb/sole.log"; printf 'npm test\n' > "$soleLOG"
sole="$(GUARD_KIT_SETTINGS="$sb/.claude/settings.json" \
        GUARD_KIT_SETTINGS_LOCAL="$sb/.claude/settings.local.json" \
        bash "$SCAN" "$soleLOG")"
assert_has  sole-clean 'clean' "$sole"
assert_has  sole-worklist 'npm test' "$sole"

# The write-shape axis, on a log of its own so the three-way-split fixture above
# keeps its own count. A row is asserted anchored at end-of-line: 'cat >' is a
# prefix of 'cat >>', so an unanchored match would pass on the wrong row.
assert_row() { grep -qE -- "  $2\$" <<<"$3" || { echo "FAIL [$1]: expected row: $2"; fails=$((fails + 1)); }; }

shapeLOG="$sb/shape.log"
{
    echo 'cat > .tmp/a.md <<EOF'        # create redirect: keys as 'cat >'
    echo 'cat >> .tmp/b.md <<EOF'       # append redirect: keys as 'cat >>'
    echo 'make build 2>&1'              # fd-dup is not a redirect to a file: no suffix
    echo 'mkdir -p .tmp && cat > .tmp/c.md'  # the ONLY redirect is in a later segment
} > "$shapeLOG"
shape="$(GUARD_KIT_SETTINGS="$sb/.claude/settings.json" \
         GUARD_KIT_SETTINGS_LOCAL="$sb/.claude/settings.local.json" \
         bash "$SCAN" "$shapeLOG")"
assert_row shape-create 'cat >'  "$shape"
assert_row shape-append 'cat >>' "$shape"
assert_row shape-fd-dup 'make'   "$shape"
# Delta 2's guard against mis-attribution: the word and the suffix come from the
# same segment, so a compound whose write lives downstream keys as 'mkdir', never
# 'mkdir >' — a write attributed to a command that performs none.
assert_row     shape-first-segment 'mkdir' "$shape"
assert_absent  shape-no-misattribution 'mkdir >' "$shape"

# --count over the same log: four keys across four calls. The occurrence total is
# what the axis never moves — splitting a key raises the numerator alone, which is
# the definitional step the KPI row records.
shape_count="$(GUARD_KIT_SETTINGS="$sb/.claude/settings.json" \
               GUARD_KIT_SETTINGS_LOCAL="$sb/.claude/settings.local.json" \
               GUARD_KIT_LOG="$shapeLOG" bash "$SCAN" --count)"
[[ "$shape_count" == "4/4" ]] || { echo "FAIL [shape-count]: expected 4/4, got '$shape_count'"; fails=$((fails + 1)); }

[[ "$fails" -eq 0 ]] || { echo "scan-prompts.test: $fails assertion(s) failed"; exit 1; }
echo "scan-prompts.test: clean (overlay-only grants stay off the headline and in the promote-or-prune section; a split-and-refused compound counts as a true prompt; the write-shape suffix splits create from append, skips an fd-dup, and never attributes a downstream write to the leading word)"
exit 0
