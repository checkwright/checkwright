# shellcheck shell=bash
# spec: lifecycle-kit/SPEC.md §Layout and configuration — this repo's lifecycle-kit consumer config: the boundary-truncate and entry-preflight knobs wire evidence-kit's manifest across the seam, the boundary-require knob makes close's release disposition a boundary precondition, the session-boundary posture is set below; every other knob keeps the platform default
# no-port: CLAUDE.md §The provenance seam (never cross it) — operator-ruled 2026-08-24 for this repo's scripts/ config-and-vocabulary class, on the vocabulary half of scripts/measured-claims.sh's cause alone (scripts/ riding no installer payload is the half the ruling deliberately does NOT declare on). LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE is this harness's own lock vocabulary, ruled consumer-side against an empty kit default in its comment below, so porting this file would spell one harness's layout in every adopter's binary.
# shellcheck disable=SC2034  # consumed by lifecycle-kit/lib/stages.sh after sourcing
LIFECYCLE_KIT_BOUNDARY_TRUNCATE=(.workflow/validate-evidence.txt .workflow/release-disposition.txt)
LIFECYCLE_KIT_BOUNDARY_REQUIRE=(.workflow/release-disposition.txt)
# spec: context-kit/SPEC.md §The session-context hook — the one scratch member this repo carries across the boundary: the marker the hook reads to suppress the delegation nudge for a lead session, whose lifetime is that live session's, not the iteration's
LIFECYCLE_KIT_BOUNDARY_PRESERVE=(session-role)
# spec: evidence-kit/SPEC.md §check-producer-liveness — the liveness gate is wired at the entry and nowhere else, now in set mode at *every* stage against the scratch directory: the subject is any recorded producer and any stage can leave one (the ninth firing's `gh run watch` had no lock at all), where the two lock-pointed entries were chosen when the subject was one producer with one lock. The cost is one invocation per entry against a usually-empty directory. It is deliberately absent from gates.list — this repo's `gates` suite is the battery run-validate itself invokes, so a registered liveness gate would red every validate run against its own lock.
# spec: evidence-kit/SPEC.md §check-producer-liveness — the two lock-pointed entries are *kept beside* the set entries rather than replaced by them, and this is a widening with no narrowing inside it: set mode globs `*.run`, and `EVIDENCE_KIT_LOCK_FILE` deliberately keeps the `.lock` suffix (a lock's absence means free; a launch record's means nothing was recorded), so the directory pass cannot see run-validate's own lock and dropping these two would have traded the ninth firing's coverage for the eighth's.
# spec: evidence-kit/SPEC.md §check-evidence-manifest — every entry names a *gate* and reaches it through scripts/gate-exec.sh, never a declaration path: enter-stage.sh execs the configured argv with no interpreter word, so a ported member's `.gate` descriptor is a non-executable data file and the entry is refused. One member is already compiled and the rest are portable, so the front-end is wired for the whole roster rather than for the one that forced it.
LIFECYCLE_KIT_ENTRY_PREFLIGHT=(
    'close=scripts/gate-exec.sh check-evidence-manifest .workflow/validate-evidence.txt'
    'scope=scripts/gate-exec.sh check-producer-liveness .tmp'
    'spec=scripts/gate-exec.sh check-producer-liveness .tmp'
    'align=scripts/gate-exec.sh check-producer-liveness .tmp'
    'build=scripts/gate-exec.sh check-producer-liveness .tmp'
    'validate=scripts/gate-exec.sh check-producer-liveness .tmp'
    'close=scripts/gate-exec.sh check-producer-liveness .tmp'
    'validate=scripts/gate-exec.sh check-producer-liveness .tmp/run-validate.lock'
    'close=scripts/gate-exec.sh check-producer-liveness .tmp/run-validate.lock'
)
# spec: lifecycle-kit/SPEC.md §Layout and configuration — this repo's session-boundary posture: 'iteration' sanctions the lead's inline fallback; cost accepted that the dogfood evidence stops demonstrating the strict posture
LIFECYCLE_KIT_SESSION_BOUNDARY=iteration
# spec: lifecycle-kit/SPEC.md §The close-surface roster — this repo's declaration surfaces beyond the kit SPECs: the always-loaded agent file, the doctrine deliverable, and the stage-skill bindings, which own the consumer-side capture surfaces no kit may name
LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS=("*/SPEC.md" "CLAUDE.md" "doctrine-kit/DOCTRINE.md" ".claude/commands/*.md")
# spec: lifecycle-kit/SPEC.md §check-scratch-citation — this repo's widening beyond the kit default (the queue alone): the governed spec set, the other permanent surface where a carried finding wants to land. The array replaces rather than extends, so the queue is restated here.
LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS=("TASK-QUEUE.md" "*/SPEC.md")
# spec: lifecycle-kit/SPEC.md §check-stage-entry — this repo splits amendment authoring into a dedicated trigger-gated `spec` stage (scope bounds, spec authors, align verifies); the roster and its `[spec]=scope` predecessor edge are set together — a roster member absent from the predecessor map fails config-load validation. Only `scope` (LIFECYCLE_KIT_FIRST_STAGE) resets the evidence file; `spec` appends. `spec` is omitted as any stage's mandatory predecessor (same calibration as the trigger-gated audit stage), so align/build/validate/close edges are unchanged.
LIFECYCLE_KIT_STAGES=(scope spec align build validate close)
# comment-tier-exempt: the seam. This harness's worktree lock reason is its own vocabulary, so the kit ships an empty default and the pattern lands here beside the other consumer-vocabulary knobs — a kit literal spelling it would publish one harness's layout. The one capture group is the holder's pid; the start field is matched and not captured, argued in lifecycle-kit/SPEC.md §bin/enter-stage.sh.
LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE='^claude agent [^ ]+ \(pid ([0-9]+) start [0-9]+\)$'
declare -A LIFECYCLE_KIT_PREDECESSOR=([spec]=scope [align]=scope [build]=scope [validate]=build [close]=validate)
