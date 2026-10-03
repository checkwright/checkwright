# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-10-03 scope — Which delegation-kit/SPEC.md sections carry the check-prose-bounds findings the brevity slice drains?
- corpus: delegation-kit/SPEC.md
- oracle: CANON_KIT_PROSE_BOUND_CEILING_FILE= bash gate-sdk/bin/run-gates.sh --only check-prose-bounds
- rev: 470763a3a16d899ed7fcd1c1443ac6d33d83636a
- finding: 129 findings: the turn-end liveness hook 96 (79 in its body, 13 in its attribution subsection, 3 in the probe-asymmetry subsection, 1 in background_tasks), the resume journal 19, bin/wait-probe 8, verify after every agent commit 4, out of scope 1, preamble 1. The liveness hook with its three subsections is the slice; the rest stays on spec-brevity-residue.
- inferred: none

## 2026-10-03 scope — Which deferred entries rank into this iteration's unit set, and does any carry aggregated inbound weight?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: 470763a3a16d899ed7fcd1c1443ac6d33d83636a
- finding: No entry aggregates more than 3 inbound edges (design-partner-preview 3; heterogeneous-agent-delegation, plugin-harness-reach, companion-toolkit-profile, benchmark-ab-experiment 2). No deferred entry reaches the recurrence threshold of 2 dates. Tier one: spec-brevity-residue (session/high, delegation-kit) leads; heterogeneous-agent-delegation (iteration/low, delegation-kit, roadmap now) joins on surface; audit-trigger-mirror-component (lifecycle-kit) and manual-operation-spend-channel (drift-kit) lead later sets. Delegation-kit board rows: heterogeneous-agent-delegation, foreign-vendor-critique, background-credential-swap-support, fan-width-unenforced. Supersession: foreign-vendor-critique reshaped by heterogeneous-agent-delegation's next slice (adapter-to-tier mapping); the rest independent.
- inferred: per-entry supersession was read by grep over names, not by an oracle
