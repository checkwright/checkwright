#!/usr/bin/env bash
# spec: queue-kit/SPEC.md §The roadmap-lag arm — the seeded history a crate unit test cannot reach,
# since a slice is read off the child's last live revision: a split child landing under a later
# horizon and under an untagged parent each print a row, one under a first-horizon parent prints
# none, a live pair citing each other prints none, and a queue file outside a work tree prints no
# row and says why on stderr.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

SANDBOX="$(mktemp -d)"
OUTSIDE="$(mktemp -d)"
trap 'rm -rf "$SANDBOX" "$OUTSIDE"' EXIT

fails=0
checks=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

git -C "$SANDBOX" init -q
git -C "$SANDBOX" config user.email seed@example.invalid
git -C "$SANDBOX" config user.name Seed

cat >"$SANDBOX/config.knobs" <<'EOF'
QUEUE_KIT_HORIZONS[] = now
QUEUE_KIT_HORIZONS[] = next
QUEUE_KIT_HORIZONS[] = later
QUEUE_KIT_TRACKS[] = core
EOF
export QUEUE_KIT_KNOB_FILE="$SANDBOX/config.knobs"

cite() {  # $1 = link or retired, $2 = the cited slug
    if [[ "$1" == link ]]; then printf '[%s](#%s)' "$2" "$2"; else printf '`%s`' "$2"; fi
}

queue() {  # $1 = the live children block, $2 = the parents' citation form, $3 = the done lines
    cat >"$SANDBOX/TASK-QUEUE.md" <<EOF
# TASK-QUEUE.md

## Iteration: seeded

## New Features

### p-later

[roadmap: later/core]

the later parent, split into $(cite "$2" c-later).

### p-untagged

the untagged parent, split into $(cite "$2" c-untagged).

### p-now

[roadmap: now/core]

the first-horizon parent, split into $(cite "$2" c-now).

### a-live

cites [b-live](#b-live).

### b-live

cites [a-live](#a-live).
$1
## Technical Debt

## Deferred

## Done
${3:-}

## Lessons Learned
EOF
}

commit() { git -C "$SANDBOX" add TASK-QUEUE.md && git -C "$SANDBOX" commit -q -m "$1"; }

CHILDREN='
### c-later

split from [p-later](#p-later).

### c-untagged

split from [p-untagged](#p-untagged).

### c-now

split from [p-now](#p-now).
'
DONE='- c-later
- c-untagged
- c-now'

queue "$CHILDREN" link;    commit "split the three children off their parents"
queue '' retired "$DONE";  commit "land the three children"

arm() { ( cd "$SANDBOX" && gate_arm_run --emit-roadmap-lag "$@" 2>"$SANDBOX/stderr" ); }

out="$(arm)"; rc=$?
checks=$((checks + 1))
[[ "$rc" -eq 0 ]] || note exit "the arm exited $rc rather than 0: $out"

checks=$((checks + 1))
grep -qxF "$(printf 'p-later\tlater\tc-later')" <<<"$out" \
    || note later-row "a slice under a later-horizon parent printed no row: $out"
checks=$((checks + 1))
grep -qxF "$(printf 'p-untagged\t-\tc-untagged')" <<<"$out" \
    || note untagged-row "a slice under an untagged parent printed no '-' row: $out"
checks=$((checks + 1))
grep -qF 'p-now' <<<"$out" && note first-horizon "a first-horizon parent printed a row: $out"
checks=$((checks + 1))
grep -qE '^(a|b)-live' <<<"$out" && note live-pair "two live entries citing each other printed a row: $out"
checks=$((checks + 1))
[[ "$(grep -c . <<<"$out")" -eq 2 ]] || note row-count "the report is not exactly the two lagging rows: $out"

checks=$((checks + 1))
cp "$SANDBOX/TASK-QUEUE.md" "$OUTSIDE/TASK-QUEUE.md"
out="$(arm "$OUTSIDE/TASK-QUEUE.md")"; rc=$?
[[ "$rc" -eq 0 && -z "$out" ]] || note outside "a queue outside a work tree printed rows or exited $rc: $out"
grep -qF 'no slice can be confirmed' "$SANDBOX/stderr" \
    || note outside-stderr "a queue outside a work tree said nothing on stderr: $(cat "$SANDBOX/stderr")"

checks=$((checks + 1))
: >"$SANDBOX/config.knobs"
out="$(arm)"; rc=$?
[[ "$rc" -eq 2 ]] || note no-horizon "no configured horizon exited $rc rather than 2: $out"

if [[ "$fails" -gt 0 ]]; then
    echo "roadmap-lag.test.sh: $fails case(s) failed"
    exit 1
fi
echo "roadmap-lag.test.sh: clean (a split slice under a later-horizon or untagged parent prints a row, one under a first-horizon parent and a live citing pair print none, a queue outside a work tree prints no row and says why on stderr, and no configured horizon is exit 2; $checks checks)"
exit 0
