# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.





















## 2026-08-17 scope — Which deferred entries reach LIFECYCLE_KIT_RECURRENCE_THRESHOLD, and which have already been ruled at that count?
- corpus: TASK-QUEUE.md
- oracle: grep -n '^  recurrence:' TASK-QUEUE.md
- rev: cc0c812dc5e70949cf7f5ea96400f13cf2bf45b1
- finding: Threshold is 2 (lifecycle-kit/lib/stages.sh:56; scripts/lifecycle-config.sh does not override). 13 declarations live; exactly FOUR reach it. recurrence-drain-input-widening (2026-08-09 2026-08-17) reached it FOR THE FIRST TIME this session and has never been escalated at threshold — its second date is the stamp this scope's own gap-inbox drain made, so the count and its grounds sit in one commit. The other three were each ruled at this same count by the lead on 2026-08-16 and carry their grounds in-entry, so re-escalating them without a new date is churn: stage-stamp-ordering-unenforced (2026-08-07 2026-08-16), waiting-rule-fourth-firing-post-fix (2026-08-06 2026-08-16), icebox-worklist-roadmap-blind (2026-08-15 2026-08-16). entry-headroom-unexposed, which newly reached the threshold at the 2026-08-16 close, has since LANDED: it is gone from the declaration set and queue-kit/SPEC.md §check-queue-entry-budget now carries a 'Clean-path headroom' block. Live re-verification of icebox-worklist-roadmap-blind at this rev: queue-index.sh --icebox-candidates offers 4 rows (rendered-site-link-monitor, plugin-marketplace, benchmark-ab-experiment, hosted-attestation-service), 3 of 4 roadmap-tagged — precision zero a sixth consecutive time. That observation is OUT OF CHANNEL and its stamp was deliberately declined, since laundering it in would pre-empt the very question recurrence-drain-input-widening holds open.

## 2026-08-17 scope — Which deferred entries converge on one surface strongly enough to compose an iteration, measured by summed inbound citations rather than read one at a time?
- corpus: TASK-QUEUE.md
- oracle: bash queue-kit/bin/queue-edges.sh
- rev: cc0c812dc5e70949cf7f5ea96400f13cf2bf45b1
- finding: The strongest convergence in the pool is the gap-inbox drain seam, and one of its members states the bundling argument itself. recurrence-resolver-literal-match-only (TASK-QUEUE.md 3060) says of the recurrence matcher: 'All three are the same bounded-substring predicate from three sides, and a scope taking one alone re-derives the other two.' The third face, gap-resolver-mention-overcount, has LANDED (verified: cited five times, live nowhere), so two faces remain — recurrence-drain-input-widening (3 inbound: recurrence-resolver-literal-match-only twice, survey-edge-aggregation-residue) and recurrence-resolver-literal-match-only itself. close-generated-finding-route (TASK-QUEUE.md 1155) sits one altitude up on the same surface: the inbox is drained once, at close, and close is the stage that generates findings by design. All three amend lifecycle-kit/SPEC.md §The committed gap inbox. The cluster is not theoretical at this rev — it was exercised TWICE at this very boundary: the 2026-08-17 bullet was filed after close ran, so no drainer was left in the machine and the entering scope session paid the disposition (close-generated-finding-route's exact defect), and the finding it carried was that a close-measured recurrence has no sanctioned route when no bullet produced it (recurrence-drain-input-widening's live half). By contrast the port track's competing candidate carries no such convergence: native-gate-port-remaining-corpus has 2 inbound, powershell-installer-surface 3, platform-support-ci-matrix 4, but those are prerequisite edges within one track rather than several siblings converging on one open question. Other multi-inbound entries that did NOT form a takeable cluster: prose-filename-citation-liveness (4) and qualified-pointer-section-ownership, whose citation-liveness family is four entries wide but whose hardest member is explicitly 'honest not-buildable is a permitted outcome'; benchmark-ab-experiment (4) and design-partner-preview, both gated behind the private launch-readiness rule; dispatch-cited-evidence-unverified (4), whose four citers are four distinct provenance defects rather than one predicate.

## 2026-08-17 scope — Which native-port increment does gate-sdk/SPEC.md §The first cohort, and the rule that selects the next select NEXT?
- corpus: scripts/gates.list */checks/ native/src/ TASK-QUEUE.md
- oracle: bash gate-sdk/bin/port-blockers.sh --group
- rev: cc0c812dc5e70949cf7f5ea96400f13cf2bf45b1
- finding: The SIZE arm governs, because NO blocker-retiring override is live: the associative-array bridge was the last blocker holding more than one member, and it is discharged (cohort-held-members-port-prerequisites, TASK-QUEUE.md:45-49; check-stage-entry and check-evidence-baseline both resolve to .gate descriptors now). Everything still held is single-member sequencing — check-tree-terms on its own criterion 4, and the external-program members (shellcheck/ruby/jq/cargo) each independently on criterion 7. check-roadmap-fresh's design hold is RETIRED AS SPENT (gate-sdk/SPEC.md:1939-1941) and it now appears as an ordinary singleton. check-gate-tamper's exemption-list Rust reader blocks nothing today, since no ported member carries an exemption list. So --group's group 1 selects: check-close-surfaces + check-queue-prose-precondition, both c2=pair c3=precommit c7=clean, neither tripping criterion 4. SIZING CORRECTION the tool does not print: the pair is not 64+64 lines. check-close-surfaces shells out to lifecycle-kit/bin/close-surfaces.sh (94 lines), which greps every coupled surface for close-surface: declarations and unions that with a live walk of .workflow/*'s capture tier, so the port must reimplement that derivation too — about 222 shell lines for the cohort, unevenly split, with check-queue-prose-precondition self-contained over a single-file corpus. SECOND FINDING, and it falsifies a deferral premise: NO unit exists that ports queue-kit/bin/queue-index.sh (182 lines). icebox-worklist-roadmap-blind was deferred 2026-08-16 on the ground that its fix should ride that tool's port to a non-gate arm, but that sentence cites roadmap.sh's queued port only as PRECEDENT FOR THE SHAPE. roadmap.sh sits in freshness-emitter-port-cohort, whose corpus is the six check-*-fresh emitters and their comparators; queue-index.sh is neither a fresh emitter nor a gate (its own header says tool, not a gate; no # graph: manifest), so it falls outside that cohort and is unreachable through the gate corpus entirely. The entry is therefore deferred waiting on a carrier that does not exist. freshness-emitter-port-cohort itself is design-ANSWERED and held only by sequence (TASK-QUEUE.md:123-125), with three members left: gen-docs-mirror.sh (127), trajectory.sh (242), roadmap.sh (76).
