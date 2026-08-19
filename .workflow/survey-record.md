# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.



























## 2026-08-19 scope — what remains of the native gate port, and what is takeable at this cut
- corpus: gate-sdk/checks native/src scripts/gates.list */checks
- oracle: bash gate-sdk/bin/port-blockers.sh --group
- rev: 84f35ec9f48b6641394f3c3a5c7ffca2416a72ae
- finding: Trailer at this rev: 104 scanned, 7 groups all singletons, 0 undecidable, 89 ported, 3 permanently shell, 5 temporarily held, 12 still owed, 7 takeable. The takeable seven are check-graph, check-knob-default-coupling, check-reads-couples, check-prose-tells, check-gate-exemption-tasks, check-spec-embedded-source, check-template-copy-parity. check-graph is RULED out of every budget batch (gate-sdk/SPEC.md fifth budget batch: non-gate HTML arm, designed before ported, wants its own iteration), so the composable set is SIX and exhausts the takeable tier. Shell declaration line counts, hand-derived because --group prints no count column (port-budget-sizing-input-absent): check-knob-default-coupling 240, check-reads-couples 219, check-prose-tells 208, check-gate-exemption-tasks 157, check-spec-embedded-source 134, check-template-copy-parity 127; total 1085, band 127-240. The FIFTH cut took seven at band 93-140, so the band has moved UP and a six-member cut here is heavier than a seven-member cut there. Width is ruled per cut and never inherited. check-reads-couples prints c7=? which resolves BENIGN: the unresolvable command-position expansion is the gate binary itself, not an external program. Every one of the six is c2=pair and c3=precommit. Groups are all singletons, so no member shares a derivation with any other: this is a budget cut, never a cohort.

## 2026-08-19 scope — which deferred entries have reached the recurrence threshold and so must ride this scope's escalation
- corpus: TASK-QUEUE.md
- oracle: grep -n 'recurrence:' TASK-QUEUE.md
- rev: 84f35ec9f48b6641394f3c3a5c7ffca2416a72ae
- finding: LIFECYCLE_KIT_RECURRENCE_THRESHOLD is 2 (lifecycle-kit/lib/stages.sh). AT OR OVER at this rev, counting dates on each declaration: stage-stamp-ordering-unenforced 3 (08-07, 08-16, 08-18); scratch-execution-control-is-bash-only 3 (08-16, 08-18, 08-19); close-entry-baseline-bootstrap-deadlock 2 (08-12, 08-18); session-mechanic-grants-uncommitted 2 (08-18, 08-19); delegation-provenance-floor 2 (08-18, 08-19) — the second date stamped AT THIS DRAIN from the carried fabricated-child-result bullet, so it is new to the threshold set and no prior scope has ruled on it. BELOW threshold at 1: amendment-owner-position-citation, enter-stage-arg-position-silent-drop, dead-queue-citation-report, kfric-empty-log-ambiguity, ruling-record-condition-staleness-probe, audit-class-corpus-attestation, lead-state-durable-home, gap-capture-argv-prompt-friction, turn-end-chokepoint-and-wait-primitive, in-crate-module-coupling-derivation, agent-worktree-reclamation-unenforced (new this drain), port-budget-sizing-input-absent (new this drain). PRIOR RULINGS that bind the reading, not to be re-litigated: stage-stamp-ordering-unenforced was declined FIVE times, the last three on the operator's ruling; close-entry-baseline-bootstrap-deadlock and scratch-execution-control-is-bash-only each reached the authority 2026-08-19 and were deferred with cause. The threshold rule asks only that the collision REACH the authority, and it is discharged by reaching it.

## 2026-08-19 scope — which deferred entries the rest of the pool converges on, by inbound citation count
- corpus: TASK-QUEUE.md
- oracle: bash queue-kit/bin/queue-edges.sh
- rev: 84f35ec9f48b6641394f3c3a5c7ffca2416a72ae
- finding: Aggregated at this rev. FIVE inbound: prose-filename-citation-liveness and stage-stamp-ordering-unenforced. FOUR: turn-end-chokepoint-and-wait-primitive. THREE: native-gate-port-remaining-corpus, delegation-provenance-floor, close-entry-baseline-bootstrap-deadlock, dispatch-cited-evidence-unverified, platform-support-ci-matrix, powershell-installer-surface, session-model-identity-verification, subagent-stop-liveness-hook-wiring, benchmark-ab-experiment. The reading that matters for ranking: prose-filename-citation-liveness is the hub of a CITATION-LIVENESS FAMILY whose siblings say so explicitly — unqualified-section-citation-liveness cites it twice and calls it the same family, link-wrapped-section-citation-liveness calls it a predicate question rather than three fixes, stale-identifier-after-retirement calls it the decisive neighbour that owns the same dangling class, and qualified-pointer-section-ownership names it beside spec-pointer-self-section-citation. Read one at a time each looks small; the aggregate is one predicate question with at least five filed instances, which is the promotion dividend the ranking rule says lives in the total. Two threshold entries carry near-zero inbound weight by contrast: session-mechanic-grants-uncommitted has 0 and scratch-execution-control-is-bash-only has 1, so their case is recurrence and friction cost, never convergence.

## 2026-08-19 build — what remains of the native gate port, and what is takeable at this cut — re-run after the sixth budget batch landed
- corpus: gate-sdk/checks native/src scripts/gates.list */checks
- oracle: bash gate-sdk/bin/port-blockers.sh --group
- rev: 85376b0cca67cc3c1ba360bc2b7d8ec591c3c9b3
- finding: SUPERSEDES the 2026-08-19 scope block of the same question, whose witness now fails on both halves: the corpus moved (the sixth budget batch ported six members) and the oracle's verdict changed. Trailer at this rev: 104 scanned, 1 group formed, 0 undecidable, 95 already ported and excluded, 3 permanently shell, 5 temporarily held, 6 still owed, 1 takeable. The one takeable member is check-graph, which is RULED out of every budget batch (gate-sdk/SPEC.md fifth budget batch), so the composable set is EMPTY and the takeable tier is exhausted until a '# port-until:' hold releases — the work cohort-held-members-port-prerequisites owns. The scope block's parenthetical reason for hand-deriving line counts is RETIRED as of this commit: --group now prints lines= on every still-shell row, keyed and unkeyed alike, so the next sizing session reads the count off the run. What the column does NOT settle is unchanged and is stated at gate-sdk/SPEC.md §The first cohort, and the rule that selects the next: it is a floor on a port's size and never a ranking of it, blind to cost concentrated in interfaces, in fresh derivation, and behind a spawned tool.
