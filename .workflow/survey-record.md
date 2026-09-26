# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-26 scope — Which deferred entries rank first for this iteration, which gate-sdk sections does spec-brevity-residue still owe, and what fills the set on the most-rowed surface?
- corpus: TASK-QUEUE.md gate-sdk/SPEC.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: 525e167a1e5fac17321f01167534647c9c9eac8e
- finding: Tier 1 (session/iteration class): spec-brevity-residue (session/high, gate-sdk) leads; one-motion-commit-race-remains-open (CLAUDE.md), gate-tests-suite-identity-in-evidence (evidence-kit), recurrence-declaration-grammar-ungated (queue-kit) and stage-cursor-unread-by-index-check (lifecycle-kit) share no surface with it. No deferred entry carries two recurrence dates; no entry carries observed-by. gate-sdk per-component sections not yet passed: upgrade-smoke through check-shellcheck (about 16.4k words) and check-graph through templates/gates-workflow.yml (about 37.8k), two slices: upgrade-smoke through check-enforcement-fresh (about 27.7k), then check-kit-enum to the end (about 26.5k). Slice names touched by no deferred or icebox deliverable beyond independent mentions. Every other gate-sdk row is design-pending, so joining one walks spec. lifecycle-kit carries the most board rows (7), all needing a ruling; survey-witness-composed-from-unvalidated-corpus and isolated-tracked-capture-lost share file_survey.rs. design-partner-preview: v0.26.0 is published and docs/install-evidence.md reads zero installs, so its re-promotion trigger has not fired.
- inferred: none

## 2026-09-26 spec — Which of the 37 crate-tests-windows failures are test portability and which are product defects, by class?
- corpus: native/src .github/workflows/gates.yml gate-sdk/SPEC.md
- oracle: gh run view 36254992168 --log-failed
- rev: ae257de7e3dd22a6e22b272b117ad9f22cfe201a
- finding: Five mechanisms over 37 tests, the same set on both Windows triples: 4 tests spawn bare bash and reach the System32 WSL launcher; 8 emit::rewrite tests seed a sandbox composed from the unstripped canonicalize answer (OS error 123); 2 gates::tests fail because the Windows job installs no ShellCheck and check-action-run-shell refuses at exit 2; 12 path-spelling tests, of which enter_stage's wipe report and recipe's queue_source (3 tests) print a Path::join backslash spelling, a product defect under the path-dialect contract, while proc (4, operate-only), knobs (4) and drift_report (1) feed or expect foreign-dialect test values; 11 hook::stop_liveness tests spawn shebang stubs Windows cannot start (OS error 193). Per-test roster and fix shapes: gate-sdk/SPEC-windows-crate-tests.md.
- inferred: none

## 2026-09-26 build — Which facts do other tracked surfaces cite into gate-sdk/SPEC.md's tooling sections (upgrade-smoke through check-shellcheck, check-graph through check-enforcement-fresh)?
- corpus: . ':(exclude)docs/gate-sdk/SPEC.md' ':(exclude)docs/check-graph.html'
- oracle: git grep -n for each section name spelled with its section sign, read against the section text
- rev: bf761b34c3cde8efae96846b837c04214603baec
- finding: Every citing site resolves to a stated sentence except five mis-citations outside the slice (canon-kit check-fence-command-head and check-fence-run, context-kit bin/env-probe, lifecycle-kit The committed gap inbox, gate-sdk port-candidate criterion 7), filed to the gap inbox. Facts other surfaces rest on inside the slice: check-shellcheck's probe-inside-corpus bound (site-kit, gate-sdk check-action-run-shell), check-graph's generator sizing and assertion B's deliberately incomplete predicate, check-reads-couples' removed descriptor opt-out, install-hooks' three-way dispatch and 0/1/2 contract, upgrade-smoke's producer clause, one-binary-per-ref and shared-clone shapes.
- inferred: none
