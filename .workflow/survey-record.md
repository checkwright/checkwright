# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-17 scope — undirected scope ranking: which deferred rows lead and which join by surface
- corpus: TASK-QUEUE.md
- oracle: grep -nE '^- .*\[cost: (session|iteration)' TASK-QUEUE.md; run-gates.sh --emit queue-edges
- rev: 4d4465c6b9167525349af582a1de7e88a6ae3d9f
- finding: 0 session rows; 7 iteration/high: inline-interpreter-substrate-census (guard-kit), binding-intel-leg-failed-one-run-in-two (.github), boundary-wipe-preserve-basename-reach (lifecycle-kit), readme-roster-enum-coverage (canon-kit), citation-liveness-family-convergence (canon-kit; waits for a cut on its four points), dod-parks-a-queue-transition-at-a-stage-that-cannot-perform-it (lifecycle-kit), install-smoke-slow-leg-residue (installer); no recurrence-threshold rows; boundary-wipe premise re-verified in native/src/emit/enter_stage.rs wipe_walk
- inferred: no row is session-class; iteration/high rows are the shortlist; lifecycle-kit carries two high rows plus several low ones

## 2026-09-17 align — does SPEC-wipe-and-drain.md wire cleanly against the current tree: is it self-consistent (count claims, Existing-sections-updated roster completeness) and are its cross-component claims (check-stage-entry assertion B, check-close-surfaces' three assertions, check-skill-binding's unbound-slot red, wipe_walk's current reach, truncate_to_header, canon-kit merge step 4, the git-aware-spare refusal) true of the live tree
- corpus: lifecycle-kit/SPEC-wipe-and-drain.md, lifecycle-kit/SPEC.md, canon-kit/SPEC.md, native/src/emit/enter_stage.rs, native/src/stages.rs, lifecycle-kit/gate-tests/boundary-scratch-wipe.test.sh, lifecycle-kit/templates/stages/spec.md, lifecycle-kit/templates/stages/close.md, .claude/commands/close.md, TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh (full battery); git grep -n on: check-close-surfaces, truncate_to_header, wipe_walk, release-policy, check-skill-binding, git-aware, LIFECYCLE_KIT_STAGES/DRAIN_STAGE; Read on canon-kit's amendment-lifecycle merge steps and check-stage-entry assertion B prose
- rev: d67b841dd25bad564b6b845ef4250580bf31a9be
- finding: amendment self-consistent (4 design-bearing deltas match the 4 unit slugs and the intro's count; every Existing-sections-updated bullet names its owning delta; the one exempt bullet carries a valid update-target-exempt reason); every checked cross-component claim (assertion B's stage-ordering grounds, the three close-surfaces assertions, check-skill-binding's unbound-slot red, wipe_walk's any-depth reach, truncate_to_header's stop-at-## behavior, canon-kit merge step 4, the git-aware-spare refusal, the release-policy sibling-slot precedent) held true against the live tree; full battery green at 115/115; no finding to resolve
- inferred: no known drift class in this corpus was left unchecked by the fanout
