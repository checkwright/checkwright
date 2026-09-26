# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-26 scope — Which deferred entries rank into this iteration's unit set, and what do their inbound queue edges add?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: d7fc17a1a195b7057a6b809e3e33f32e59038404
- finding: First tier: spec-brevity-residue (session/high, gate-sdk) leads; one-motion-commit-race-remains-open (session/low, CLAUDE.md) and the iteration/low rows (heterogeneous-agent-delegation, gate-tests-suite-identity-in-evidence, recurrence-declaration-grammar-ungated, stage-cursor-unread-by-index-check) share no surface with it. No [recurrence:] array reaches threshold 2. Second tier: demo-catches-a-done-claim (roadmap now/adoption), which front-door-demo-unreachable is cited as owing; design-partner-preview carries a consult recommendation to promote. Every other gate-sdk-surface row is design-pending. Inbound edges: crate-tests-windows-flip blocks on crate-tests-windows-failures; consult-inbox-drain-trigger on consult-inbox; gate-tamper-exemption-reader-substrate on gate-authoring-sdk-surface. The retired block holds only provenance citations, with one live-named row (check-spec-pointer).
- inferred: none

## 2026-09-26 scope — How many words does each gate-sdk/SPEC.md Per-component contracts subsection left on spec-brevity-residue carry, and which cut makes a slice near the prior slice's size?
- corpus: gate-sdk/SPEC.md
- oracle: awk '/^###? /{if(h)print h, w; h=$0; w=0} {w+=NF} END{print h, w}' gate-sdk/SPEC.md
- rev: d7fc17a1a195b7057a6b809e3e33f32e59038404
- finding: gate-sdk/SPEC.md holds 170375 words. The runner and library cut (lib/gate.sh 5814, lib/inject.sh 839, lib/declaration.sh 1805, lib/test-hermetic.sh 928, run-gates 7979, run-gate-tests 3198, run-consumer-smoke 86, upgrade-smoke 5368, with-foreign-shells 853) totals 26870. The meta-gate cut (check-shellcheck through check-gate-exemption-tasks, less check-crate-arms) totals 26819. Either is near the native-contracts slice's 25.7k.
- inferred: none

## 2026-09-26 align — Does appending */SPEC.md to CONTEXT_KIT_RATCHET_PATHS (SPEC-spec-ratchet delta 1) pull in docs/*/SPEC.md mirrors or gate-tests/ fixture SPEC.md files?
- corpus: scripts/context-config.knobs
- oracle: git ls-files -- '*/templates/*.md' '.claude/agents/*.md' '.claude/commands/*.md' ':(exclude)*/gate-tests/*' 'docs/*.md' ':(exclude)docs/*/SPEC.md' ':(exclude)docs/*/README.md' ':(exclude)docs/doctrine-kit/DOCTRINE.md' ':(exclude)docs/evidence-data.md' ':(exclude)docs/enforcement.md' ':(exclude)docs/footprint.md' ':(exclude)docs/install-evidence.md' '*/SPEC.md'
- rev: a1161ba127b3adc31157cb216268ff507ac06abc
- finding: No. The array's existing :(exclude)*/gate-tests/* and :(exclude)docs/*/SPEC.md entries apply globally across the whole pathspec set passed to one 'git ls-files --' invocation, regardless of position, so appending */SPEC.md at the end (as delta 1 directs) yields exactly the 12 canonical kit SPECs the amendment's own measurement names, plus the pre-existing governed surfaces — no gate-tests fixture or docs mirror SPEC.md leaks in.
- inferred: none
