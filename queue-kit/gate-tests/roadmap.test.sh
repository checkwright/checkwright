#!/usr/bin/env bash
# Behavioral test of check-roadmap-fresh assertion B and the bin/roadmap.sh emit
# grammar. The gate's good/bad pair proves assertion A (the byte-compare), which
# is the one axis a single pair can carry; B's rejection shapes and the emitter's
# projection rules are driven directly here. Config is isolated via
# QUEUE_KIT_CONFIG_FILE so the repo's queue-config.sh does not leak in.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # queue-kit/
GATE="$DIR/checks/check-roadmap-fresh.sh"
EMIT="$DIR/bin/roadmap.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

cat >"$SANDBOX/config.sh" <<'EOF'
QUEUE_KIT_HORIZONS=(soon someday)
QUEUE_KIT_TRACKS=(alpha beta)
QUEUE_KIT_ROADMAP_FILE=ROADMAP.md
EOF
export QUEUE_KIT_CONFIG_FILE="$SANDBOX/config.sh"

fails=0
want() {   # $1=label $2=output $3=substring-that-must-be-present
    grep -qF -- "$3" <<<"$2" || { echo "  FAIL [$1]: output lacks '$3':"; printf '    %s\n' "$2"; fails=$((fails + 1)); }
}
absent() { # $1=label $2=output $3=substring-that-must-be-absent
    grep -qF -- "$3" <<<"$2" && { echo "  FAIL [$1]: output should not contain '$3':"; printf '    %s\n' "$2"; fails=$((fails + 1)); } || true
}
code() {   # $1=label $2=actual $3=expected
    [[ "$2" == "$3" ]] || { echo "  FAIL [$1]: expected exit $3, got $2"; fails=$((fails + 1)); }
}

queue() {  # $1=the entry lead line(s) — writes a minimal well-formed queue
    { printf '## Iteration: demo\n\n## New Features\n\n'
      printf '%s\n' "$1"
      printf '\n## Technical Debt\n\n## Deferred\n\n## Done\n\n## Lessons Learned\n'
    } >"$SANDBOX/TASK-QUEUE.md"
}

# The projection page is kept fresh throughout, so every verdict below is
# assertion B's and never assertion A's.
refresh() {
    printf 'framing\n\n<!-- roadmap:begin -->\n<!-- roadmap:end -->\n' >"$SANDBOX/ROADMAP.md"
    ( cd "$SANDBOX" && bash "$EMIT" --write >/dev/null )
}

run_gate() { ( cd "$SANDBOX" && bash "$GATE" 2>&1 ); }

# Every field valid -> clean.
queue '- **ok-thing** [roadmap: soon/alpha] — a valid entry.'
refresh
out="$(run_gate)"; code "valid-exit" "$?" 0
want "valid" "$out" "ROADMAP-FRESH: clean"

# Unknown horizon -> named, with the offending value.
queue '- **bad-horizon** [roadmap: eventually/alpha] — an unconfigured horizon.'
refresh
out="$(run_gate)"; code "horizon-exit" "$?" 1
want "horizon-slug"  "$out" "bad-horizon"
want "horizon-value" "$out" "unknown horizon 'eventually'"

# Unknown track -> named, with the offending value.
queue '- **bad-track** [roadmap: soon/gamma] — an unconfigured track.'
refresh
out="$(run_gate)"; code "track-exit" "$?" 1
want "track-value" "$out" "unknown track 'gamma'"

# Missing slash -> a parse failure, not a membership failure.
queue '- **no-slash** [roadmap: soon] — one field where two are required.'
refresh
out="$(run_gate)"; code "slash-exit" "$?" 1
want "slash" "$out" "does not parse as <horizon>/<track>"

# A second [roadmap:] tag on one entry -> rejected.
queue '- **two-tags** [roadmap: soon/alpha] [roadmap: someday/beta] — two tags.'
refresh
out="$(run_gate)"; code "twotags-exit" "$?" 1
want "twotags" "$out" "carries 2 [roadmap:] tags"

# An untagged entry is simply not projected — the normal case, not a violation.
queue '- **plain-thing** — no tag at all.'
refresh
out="$(run_gate)"; code "untagged-exit" "$?" 0
want "untagged" "$out" "ROADMAP-FRESH: clean"

# Emit grammar: the done section is history, never direction.
{ printf '## Iteration: demo\n\n## New Features\n\n'
  printf -- '- **live-thing** [roadmap: soon/alpha] — a live entry. A second sentence.\n'
  printf '  A body line that is never projected.\n'
  printf '\n## Technical Debt\n\n## Deferred\n\n'
  printf -- '- **deferred-thing** [roadmap: someday/beta] — a deferred entry.\n'
  printf '\n## Done\n\n- shipped-thing\n\n## Lessons Learned\n'
} >"$SANDBOX/TASK-QUEUE.md"
out="$( cd "$SANDBOX" && bash "$EMIT" --emit )"
want   "emit-live"     "$out" '- **`live-thing`** *(alpha)* — a live entry.'
absent "emit-sentence" "$out" "A second sentence"
absent "emit-body"     "$out" "never projected"
want   "emit-deferred" "$out" '- **`deferred-thing`** *(beta)* — a deferred entry.'
absent "emit-done"     "$out" "shipped-thing"
want   "emit-heading"  "$out" "### soon"

# An empty horizon still gets its heading, carrying the placeholder.
queue '- **only-soon** [roadmap: soon/alpha] — the only entry.'
out="$( cd "$SANDBOX" && bash "$EMIT" --emit )"
want "empty-heading"     "$out" "### someday"
want "empty-placeholder" "$out" "_Nothing is queued under this horizon._"

# The written block ends on a blank line before the :end marker — without it the
# Pages parser leaves the list open and renders the marker inside the last <li>.
printf 'framing\n\n<!-- roadmap:begin -->\n<!-- roadmap:end -->\n' >"$SANDBOX/ROADMAP.md"
( cd "$SANDBOX" && bash "$EMIT" --write >/dev/null )
if ! grep -B1 -F -- '<!-- roadmap:end -->' "$SANDBOX/ROADMAP.md" | head -1 | grep -qx ''; then
    echo "  FAIL [write-trailing-blank]: no blank line before the :end marker:"
    printf '    %s\n' "$(cat "$SANDBOX/ROADMAP.md")"
    fails=$((fails + 1))
fi

# --write leaves every byte outside the markers untouched.
printf 'KEEP-ABOVE\n\n<!-- roadmap:begin -->\nstale\n<!-- roadmap:end -->\n\nKEEP-BELOW\n' >"$SANDBOX/ROADMAP.md"
( cd "$SANDBOX" && bash "$EMIT" --write >/dev/null )
page="$(cat "$SANDBOX/ROADMAP.md")"
want   "write-above" "$page" "KEEP-ABOVE"
want   "write-below" "$page" "KEEP-BELOW"
absent "write-stale" "$page" "stale"

# An empty QUEUE_KIT_ROADMAP_FILE is the clean skip for a consumer with no page.
cat >"$SANDBOX/noroadmap.sh" <<'EOF'
QUEUE_KIT_HORIZONS=(soon someday)
QUEUE_KIT_TRACKS=(alpha beta)
EOF
out="$( cd "$SANDBOX" && QUEUE_KIT_CONFIG_FILE="$SANDBOX/noroadmap.sh" bash "$GATE" 2>&1 )"
code "skip-exit" "$?" 0
want "skip" "$out" "this consumer publishes no roadmap"

# A half-configured vocabulary is malformed config, not a lenient default.
cat >"$SANDBOX/halfconfig.sh" <<'EOF'
QUEUE_KIT_HORIZONS=(soon someday)
EOF
out="$( cd "$SANDBOX" && QUEUE_KIT_CONFIG_FILE="$SANDBOX/halfconfig.sh" bash "$GATE" 2>&1 )"
code "half-exit" "$?" 2
want "half" "$out" "QUEUE_KIT_TRACKS is empty"

if [[ "$fails" -gt 0 ]]; then
    echo "roadmap.test.sh: $fails case(s) failed"
    exit 1
fi
echo "roadmap.test.sh: clean (assertion B: valid, unknown horizon/track, unparseable field, duplicate tag, untagged; emit grammar: done excluded, body excluded, empty-horizon placeholder; --write splice bounds + trailing blank; empty-page skip; half-configured vocabulary; 27 checks)"
exit 0
