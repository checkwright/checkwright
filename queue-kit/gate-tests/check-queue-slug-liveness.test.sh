#!/usr/bin/env bash
# spec: queue-kit/SPEC.md §check-queue-slug-liveness — assertions B and C read the queue file's own history for the retired set, which a static case dir cannot carry: the pair runs inside whatever repo vendored it. Every retirement-dependent case builds its own history here instead.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # queue-kit/
sb="$(mktemp -d)"
trap 'rm -rf "$sb"' EXIT

git -C "$sb" init -q
git -C "$sb" config user.email t@example.invalid
git -C "$sb" config user.name t
mkdir -p "$sb/bin" "$sb/docs"
printf '#!/usr/bin/env bash\n' >"$sb/bin/tool-name.sh"
cat >"$sb/TASK-QUEUE.md" <<'EOF'
## New Features

- **old-task** — landed later.
- **tool-name** — landed as a tracked script.

## Technical Debt

## Deferred

- **ice-task** — evicted later.

## Icebox

## Done

## Lessons Learned
EOF
git -C "$sb" add -A
git -C "$sb" commit -qm seed
cat >"$sb/TASK-QUEUE.md" <<'EOF'
## New Features

## Technical Debt

## Deferred

## Icebox

- **ice-task** — evicted.

## Done

- old-task
- tool-name

## Lessons Learned
EOF
git -C "$sb" commit -qam land
queue="$(cat "$sb/TASK-QUEUE.md")"

printf 'QUEUE_KIT_ICEBOX_SECTION = Icebox\n' >"$sb/empty.knobs"
printf 'QUEUE_KIT_ICEBOX_SECTION = Icebox\nQUEUE_KIT_CITATION_SURFACE_GLOBS[] = docs/*.md\n' >"$sb/cite.knobs"

fails=0
run() { ( cd "$sb" && QUEUE_KIT_KNOB_FILE="$sb/$1" gate_run check-queue-slug-liveness "$DIR/checks" . 2>&1 ); }
expect() {  # $1=case $2=knob-file $3=want-exit $4=want-line
    local out rc
    out="$(run "$2")"; rc=$?
    if [[ "$rc" -ne "$3" ]]; then
        echo "  FAIL [$1]: want exit $3, got $rc -- $out"; fails=$((fails + 1))
    elif ! grep -qF "$4" <<<"$out"; then
        echo "  FAIL [$1]: exit $3 but output lacks '$4': $out"; fails=$((fails + 1))
    fi
}

printf '# page\n\nThe pending `old-task` work.\n' >"$sb/docs/page.md"
expect empty-knob-skips empty.knobs 0 "QUEUE-SLUG-LIVENESS: clean"
expect retired-citation-reds cite.knobs 1 "docs/page.md:3:old-task"

printf '# page\n\nThe `tool-name` script.\n' >"$sb/docs/page.md"
expect live-name-exempt cite.knobs 0 "QUEUE-SLUG-LIVENESS: clean"

printf '# page\n\nThe `old-task` unit landed. <!-- retired-citation-exempt: history -->\n' >"$sb/docs/page.md"
expect valve-admits cite.knobs 0 "QUEUE-SLUG-LIVENESS: clean"

printf '# page\n\nThe `old-task` unit landed. <!-- retired-citation-exempt: -->\n' >"$sb/docs/page.md"
expect reasonless-valve-reds cite.knobs 1 "docs/page.md:3:old-task"
rm "$sb/docs/page.md"

printf '%s\n\nNote: `ice-task` (deferred) and `old-task` (retired).\n' "$queue" >"$sb/TASK-QUEUE.md"
expect stale-status-reds empty.knobs 1 "ice-task says (deferred) but is Icebox"

printf '%s\n\nNote: `ice-task` (icebox), `old-task` (retired), `tool-name` (done) and `nothing` (done).\n' "$queue" >"$sb/TASK-QUEUE.md"
expect accurate-status-clean empty.knobs 0 "QUEUE-SLUG-LIVENESS: clean"

if [[ "$fails" -gt 0 ]]; then
    echo "check-queue-slug-liveness.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-queue-slug-liveness.test.sh: clean (retired set from the scratch repo's own history: the empty knob skips, a retired citation reds, a live name and a reasoned valve clear, a reasonless valve reds, a stale status reds and accurate ones clear, 7 cases)"
exit 0
