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

## 2026-10-03 build — Which surfaces cite into delegation-kit/SPEC.md's turn-end liveness hook section, and which of its facts do they rely on?
- corpus: .
- oracle: git grep -n -E 'turn-end liveness hook|Attribution was weighed|What .background_tasks. carries|The probe is asymmetric'
- rev: 3753fc14fab3fd149232ac9210f11c9e49d27b40
- finding: Read outside delegation-kit/SPEC.md and its docs mirror. External cites land on the section headings only; no italic paragraph-lead cite exists outside the section. Facts relied on: Stop unregistered (lifecycle-kit), own-axis refusal, view a supplement and corrupt divergence (guard-kit), exit 2 with stderr reason (README), emptied knob reads the compiled gate (smoke), sources no kit lib, advisory log reason, typed bounded error and wall-clock bound (proc.rs), and the stop_liveness.rs spec: tags (field order, open record, glob after reader, exit-2 split, refuse-once, task-view ownership, helper test, keys sorted, executability predicate, spawned default, three-way message, stub written in a child).
- inferred: none
