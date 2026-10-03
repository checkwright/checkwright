# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-10-04 scope — Which delegation-kit/SPEC.md sections carry the check-prose-bounds findings, and how large is the unpassed remainder?
- corpus: delegation-kit/SPEC.md
- oracle: CANON_KIT_PROSE_BOUND_CEILING_FILE= bash gate-sdk/bin/run-gates.sh --only check-prose-bounds
- rev: 91c091cde6659b2837d176e2440bfe653482142a
- finding: 33 findings: the resume journal 19, bin/wait-probe 8, verify after every agent commit 4, out of scope 1, preamble 1, unchanged since delegation-transport-pass. The unpassed sections are the preamble, Resume journal, Verify after every agent commit, Trend reporter, bin/wait-probe, The foreign-vendor run with Resuming a session, and Out of scope: about 8.5k of the file's 36.6k words. That remainder is one slice and finishes delegation-kit on spec-brevity-residue.
- inferred: word counts by line range were read with awk over the section line spans, not an oracle

## 2026-10-04 scope — Which deferred entries rank into this iteration's unit set, and does any carry aggregated inbound weight?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: 91c091cde6659b2837d176e2440bfe653482142a
- finding: No entry aggregates more than 3 inbound edges (design-partner-preview 3). No deferred entry reaches the recurrence threshold of 2 dates. Tier one: spec-brevity-residue (session/high, delegation-kit, roadmap now) leads; heterogeneous-agent-delegation (iteration/low, delegation-kit, roadmap now) joins only with a feature stage; audit-trigger-mirror-component (lifecycle-kit) and manual-operation-spend-channel (drift-kit) lead later sets. Board surfaces by row count: installer 6, gate-sdk 6, then delegation-kit, lifecycle-kit and drift-kit 4. Supersession: releases-page-table reshaped by release-note-section-set-derivation; front-door-rehearsal-rule's post-publish job reshaped by tarball-build-attestation.
- inferred: per-entry supersession was read by grep over deliverable names, not by an oracle
