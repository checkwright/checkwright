# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-25 scope — Which deferred units lead an undirected iteration, and what composes with the lead
- corpus: TASK-QUEUE.md gate-sdk/SPEC.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: 1ade9a3c587d3598fa309a6436a26369351e2b04
- finding: Tier one on the board: spec-brevity-residue (session/high, gate-sdk) leads; one-motion-commit-race-remains-open (session/low, CLAUDE.md), enforcement-first-load-trigger (iteration/high, lifecycle-kit) and four iteration/low rows lead later sets. No deferred entry reaches the recurrence threshold of two dates. queue-edges shows no live inbound edge on the lead; its one retired edge is its own parent. gate-sdk/SPEC.md holds about 204k words: Per-component contracts about 107k, Porting about 46k, the framework sections the rest, so one iteration cannot pass the whole SPEC. gate-sdk-surface debt that composes without the authoring stage: kit-readme-validity-pass, fixture-runner-checks-dir-fails-open, bin-argv-shape-residual-member, surplus-arg-drop-in-six-emit-arms, emit-arm-usage-unreachable, non-gate-arm-roster-hand-maintained, upgrade-smoke-refuses-inside-a-worktree, consumer-smoke-accounting-spelling-unpinned, template-registry-population-predicate; premises probed and holding. non-gate-arm-testing-floor-unstated, release-asset-claim-class-owner, ci-one-line-action, config-variant-battery-harness, foreign-toolchain-docker-legs and crate-tests-unrun-on-windows are feature-class or push-needing and stay out.
- inferred: The per-section gate-sdk slice size is judged against the prior slice's churn, not measured as session cost; the eight debt entries' sizes are read off their bodies, not built.

## 2026-09-25 build — Which citations into gate-sdk/SPEC.md's framework sections rely on text the brevity pass removed?
- corpus: . :!docs
- oracle: git grep -n -e '§The non-gate arm' -e '§The harness-integration arm' -e '§Fail-closed contract' -e '§Layout and configuration' -e '§The path-dialect contract' -e '§The bin/-tool contract' -- . ':!docs' — then each quoted, italic-anchored or fact-attributing site read against the framework slice's current text (every heading from §The provenance seam to §The extensibility model)
- rev: 29e4552ffc075f6a169de9824c5cdebdbb42870a
- finding: three stale sites, all in the non-gate arm's citers, fixed in 29e4552f: gate-sdk §upgrade-smoke and guard-kit §Testing quoted reworded sentences; the roster queue entry moved to Done. No other quote, italic anchor or fact claim went stale.
- inferred: plain pointers were sampled rather than each re-read
