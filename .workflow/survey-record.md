# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-26 scope — Which deferred entries rank first for this iteration, and does ci-one-line-action rank on its own ground?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: 929b9b69eb0c86fbe293619989375187a208de42
- finding: Tier 1 (session/iteration class): spec-brevity-residue (session/high) leads; one-motion-commit-race-remains-open (session/low, CLAUDE.md), gate-tests-suite-identity-in-evidence, recurrence-declaration-grammar-ungated and stage-cursor-unread-by-index-check (iteration/low) share no surface with it, so they lead later sets. No deferred entry carries two recurrence dates, so nothing pre-empts. No tier-1 candidate has inbound edges. ci-one-line-action on its own ground: a tier-2 roadmap entry (next/adoption) on the lead's gate-sdk surface with a decided deliverable, but a feature by the new-names litmus (an Action, a SARIF arm, init seeding), so joining it walks spec, spans gate-sdk and installer so it triggers align, and its Action runs only on a remote run. Remaining gate-sdk per-component contracts come to about 80k words; the meta-gate sections (check-gate-output through check-gate-exemption-tasks) are about 25.5k words and are touched by no deferred or icebox deliverable. check-kit-registration is reshaped by registry-assertion-duplicated, and templates/gates-workflow.yml by ci-one-line-action.
- inferred: none

## 2026-09-26 spec — Which verbs do the front-door routes advertise, and where?
- corpus: README.md docs/index.md docs/install.md installer/README.md
- oracle: grep -n -o -E '(sh -s --|install\.ps1\)\)\)|npx checkwright|`checkwright) +[^ `]+' README.md docs/index.md docs/install.md installer/README.md
- rev: 2c6490910fd56ea8ee34c6323c2fe8002a954adc
- finding: demo is route-advertised at 10 sites: README.md 14, 18, 22; docs/index.md 19, 23, 27; docs/install.md 18 (twice), 44; installer/README.md 21. Every other route token is a released verb (init, doctor, diff, update, uninstall), a flag (--profile, --help), or the <verb> placeholder at docs/install.md 142.
- inferred: none

## 2026-09-26 build — Which facts do other tracked surfaces cite into gate-sdk/SPEC.md's meta-gate sections (check-gate-output through check-gate-exemption-tasks, less check-crate-arms)?
- corpus: . ':(exclude)docs/gate-sdk/SPEC.md' ':(exclude)docs/check-graph.html'
- oracle: git grep -n for each section name spelled with its section sign, read against the section text
- rev: b294fd5f7ff1326a55e2f02f05d9daa16da1e269
- finding: Every citing site resolves to a stated sentence except implementation comments in native/src/gates citing literal regex or awk spellings the prose states only as policy, and native/src/hook/stop_liveness.rs citing check-test-hermetic for a scratch-root naming fact the section never held. Outside the slice, gate-sdk/SPEC.md cites check-install-disposition for live-tree parity proved (criterion 4), check-gate-assertions for its self-auditing port, and run-gate-tests cited a version-skew residual assertion B no longer carries.
- inferred: none
