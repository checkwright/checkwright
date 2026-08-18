#!/usr/bin/env bash
# Behavioral test of check-roadmap-fresh assertion B and the roadmap arm's emit
# grammar. The gate's good/bad pair proves assertion A (the byte-compare), which
# is the one axis a single pair can carry; B's rejection shapes and the emitter's
# projection rules are driven directly here. Config is isolated via
# QUEUE_KIT_CONFIG_FILE so the repo's queue-config.sh does not leak in.
#
# Both halves are compiled now, so both are driven through their declared entry
# points: gate_run resolves the gate's descriptor and its config bridge, and the
# battery runner's --emit front-end resolves the arm's. The sandbox is a git repo
# because that front-end anchors at the toplevel before it dispatches.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # queue-kit/
CHECKS="$DIR/checks"
FRONTEND="$DIR/../gate-sdk/bin/run-gates.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT
git -C "$SANDBOX" init -q

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

queue() {  # $1=the entry lead line, $2=optional body line(s) — writes a minimal well-formed queue
    { printf '## Iteration: demo\n\n## New Features\n\n'
      printf '%s\n' "$1"
      [[ $# -ge 2 ]] && printf '%s\n' "$2"
      printf '\n## Technical Debt\n\n## Deferred\n\n## Done\n\n## Lessons Learned\n'
    } >"$SANDBOX/TASK-QUEUE.md"
}
decl='  roadmap-summary: A sentence an author marked for the public page.'

emit() {   # $@ = the arm's own argv tail
    ( cd "$SANDBOX" && bash "$FRONTEND" --emit roadmap "$@" 2>&1 )
}

# The projection page is kept fresh throughout, so every verdict below is
# assertion B's and never assertion A's.
refresh() {
    printf 'framing\n\n<!-- roadmap:begin -->\n<!-- roadmap:end -->\n' >"$SANDBOX/ROADMAP.md"
    emit --write >/dev/null
}

run_gate() { ( cd "$SANDBOX" && gate_run check-roadmap-fresh "$CHECKS" 2>&1 ); }

# Every field valid, one declaration -> clean.
queue '- **ok-thing** [roadmap: soon/alpha] — a valid entry.' "$decl"
refresh
out="$(run_gate)"; code "valid-exit" "$?" 0
want "valid" "$out" "ROADMAP-FRESH: clean"

# Unknown horizon -> named, with the offending value.
queue '- **bad-horizon** [roadmap: eventually/alpha] — an unconfigured horizon.' "$decl"
refresh
out="$(run_gate)"; code "horizon-exit" "$?" 1
want "horizon-slug"  "$out" "bad-horizon"
want "horizon-value" "$out" "unknown horizon 'eventually'"

# Unknown track -> named, with the offending value.
queue '- **bad-track** [roadmap: soon/gamma] — an unconfigured track.' "$decl"
refresh
out="$(run_gate)"; code "track-exit" "$?" 1
want "track-value" "$out" "unknown track 'gamma'"

# Missing slash -> a parse failure, not a membership failure.
queue '- **no-slash** [roadmap: soon] — one field where two are required.' "$decl"
refresh
out="$(run_gate)"; code "slash-exit" "$?" 1
want "slash" "$out" "does not parse as <horizon>/<track>"

# A second [roadmap:] tag on one entry -> rejected.
queue '- **two-tags** [roadmap: soon/alpha] [roadmap: someday/beta] — two tags.' "$decl"
refresh
out="$(run_gate)"; code "twotags-exit" "$?" 1
want "twotags" "$out" "carries 2 [roadmap:] tags"

# An untagged, unmarked entry is simply not projected — the normal case.
queue '- **plain-thing** — no tag at all.'
refresh
out="$(run_gate)"; code "untagged-exit" "$?" 0
want "untagged" "$out" "ROADMAP-FRESH: clean"

# Assertion C — a tagged entry with no declaration reds rather than projecting a
# bullet with no prose.
queue '- **no-decl** [roadmap: soon/alpha] — tagged, but nothing marked.'
refresh
out="$(run_gate)"; code "nodecl-exit" "$?" 1
want "nodecl" "$out" "carries 0 roadmap-summary: declaration(s)"

# Assertion C — two declarations are ambiguous, so they red too.
queue '- **two-decl** [roadmap: soon/alpha] — tagged twice over.' "$decl
$decl"
refresh
out="$(run_gate)"; code "twodecl-exit" "$?" 1
want "twodecl" "$out" "carries 2 roadmap-summary: declaration(s)"

# Assertion C, the other direction — a declaration with no tag is a dead marking,
# which is what a dropped or reflowed tag looks like from the page's side.
queue '- **dead-marking** — the tag fell off this lead line.' "$decl"
refresh
out="$(run_gate)"; code "dead-exit" "$?" 1
want "dead-slug" "$out" "dead-marking"
want "dead"      "$out" "no [roadmap:] tag"

# The declaration count and the tag fields are independent, so one run reports
# both rather than costing the author a second round trip.
queue '- **both-wrong** [roadmap: eventually/alpha] — bad horizon and no mark.'
refresh
out="$(run_gate)"; code "both-exit" "$?" 1
want "both-decl"    "$out" "carries 0 roadmap-summary: declaration(s)"
want "both-horizon" "$out" "unknown horizon 'eventually'"

# Emit grammar. The whitelist is the load-bearing property: only the declaration
# is projected, so the lead-line prose, the surrounding body, and the cost block
# all stay off the page. The done section is history, never direction.
{ printf '## Iteration: demo\n\n## New Features\n\n'
  printf -- '- **live-thing** [roadmap: soon/alpha] — LEADPROSE, an internal one-liner.\n'
  printf '  BODYPROSE that no reader should ever see.\n'
  printf '  roadmap-summary: The public sentence, and only this.\n'
  printf '  **Cost while deferred:** COSTPROSE, the accounting a public page must not inherit.\n'
  printf '\n## Technical Debt\n\n## Deferred\n\n'
  printf -- '- **deferred-thing** [roadmap: someday/beta] — internal framing.\n'
  printf '  roadmap-summary: A deferred rung, said publicly.\n'
  printf '\n## Done\n\n- shipped-thing\n\n## Lessons Learned\n'
} >"$SANDBOX/TASK-QUEUE.md"
out="$(emit)"
want   "emit-declared"  "$out" '- **`live-thing`** *(alpha)* — The public sentence, and only this.'
absent "emit-leadprose" "$out" "LEADPROSE"
absent "emit-bodyprose" "$out" "BODYPROSE"
absent "emit-costprose" "$out" "COSTPROSE"
want   "emit-deferred"  "$out" '- **`deferred-thing`** *(beta)* — A deferred rung, said publicly.'
absent "emit-done"      "$out" "shipped-thing"
want   "emit-heading"   "$out" "### soon"

# The whitelist holds even with the gate bypassed: a tagged entry whose
# declaration is missing contributes no bullet at all rather than a naked one.
queue '- **unmarked** [roadmap: soon/alpha] — LEADPROSE only, nothing marked.' '  BODYPROSE here too.'
out="$(emit)"
absent "bypass-slug"      "$out" "unmarked"
absent "bypass-leadprose" "$out" "LEADPROSE"
absent "bypass-bodyprose" "$out" "BODYPROSE"

# An empty horizon still gets its heading, carrying the placeholder.
queue '- **only-soon** [roadmap: soon/alpha] — the only entry.' "$decl"
out="$(emit)"
want "empty-heading"     "$out" "### someday"
want "empty-placeholder" "$out" "_Nothing is queued under this horizon._"

# The written block ends on a blank line before the :end marker — without it the
# Pages parser leaves the list open and renders the marker inside the last <li>.
printf 'framing\n\n<!-- roadmap:begin -->\n<!-- roadmap:end -->\n' >"$SANDBOX/ROADMAP.md"
emit --write >/dev/null
if ! grep -B1 -F -- '<!-- roadmap:end -->' "$SANDBOX/ROADMAP.md" | head -1 | grep -qx ''; then
    echo "  FAIL [write-trailing-blank]: no blank line before the :end marker:"
    printf '    %s\n' "$(cat "$SANDBOX/ROADMAP.md")"
    fails=$((fails + 1))
fi

# --write leaves every byte outside the markers untouched.
printf 'KEEP-ABOVE\n\n<!-- roadmap:begin -->\nstale\n<!-- roadmap:end -->\n\nKEEP-BELOW\n' >"$SANDBOX/ROADMAP.md"
emit --write >/dev/null
page="$(cat "$SANDBOX/ROADMAP.md")"
want   "write-above" "$page" "KEEP-ABOVE"
want   "write-below" "$page" "KEEP-BELOW"
absent "write-stale" "$page" "stale"

# An empty QUEUE_KIT_ROADMAP_FILE is the clean skip for a consumer with no page.
cat >"$SANDBOX/noroadmap.sh" <<'EOF'
QUEUE_KIT_HORIZONS=(soon someday)
QUEUE_KIT_TRACKS=(alpha beta)
EOF
out="$( cd "$SANDBOX" && QUEUE_KIT_CONFIG_FILE="$SANDBOX/noroadmap.sh" gate_run check-roadmap-fresh "$CHECKS" 2>&1 )"
code "skip-exit" "$?" 0
want "skip" "$out" "this consumer publishes no roadmap"

# A half-configured vocabulary is malformed config, not a lenient default.
cat >"$SANDBOX/halfconfig.sh" <<'EOF'
QUEUE_KIT_HORIZONS=(soon someday)
EOF
out="$( cd "$SANDBOX" && QUEUE_KIT_CONFIG_FILE="$SANDBOX/halfconfig.sh" gate_run check-roadmap-fresh "$CHECKS" 2>&1 )"
code "half-exit" "$?" 2
want "half" "$out" "QUEUE_KIT_TRACKS is empty"

if [[ "$fails" -gt 0 ]]; then
    echo "roadmap.test.sh: $fails case(s) failed"
    exit 1
fi
echo "roadmap.test.sh: clean (assertion B: valid, unknown horizon/track, unparseable field, duplicate tag, untagged; assertion C: missing/duplicate declaration, dead marking, independent findings reported together; emit whitelist: lead/body/cost prose excluded, done excluded, unmarked entry contributes nothing, empty-horizon placeholder; --write splice bounds + trailing blank; empty-page skip; half-configured vocabulary; 41 checks)"
exit 0
