# Checkwright

A coding-agent-assisted delivery methodology as installable kits: machine-gated
documentation/spec consistency, an evidence-stamped iteration lifecycle
designed for stateless agent sessions, and token-economics-aware context
management. A *wright* is a craftsman — shipwright, playwright; this is the
craft of checks.

The premise: when coding agents do the writing, discipline does not hold —
conventions live in prose no stateless session reliably re-reads, and drift is
silent. The remedy is mechanization: every cheap, low-false-positive,
mechanically-decidable consistency axis is enforced by a gate that blocks the
commit, and the human (or agent) residue is held to the irreducibly semantic
judgment alone. Checkwright packages that machinery, extracted from a working
platform's governance meta-layer — and this repository governs itself with its
own kits, day one.

## Kits

| Kit | Status | What it is |
|---|---|---|
| [gate-sdk/](gate-sdk/) | **landed** | A self-testing lint framework for prose/spec/config surfaces: the gate contracts (output, fail-closed, fixture-pair, self-lint), the fixture runner, `# graph:` coupling manifests, and a generated pre-commit hook. |
| [lifecycle-kit/](lifecycle-kit/) | **landed** | The iteration stage state machine for stateless agent sessions: a stage header + evidence-stamp file, five stage-skill templates (scope/align/build/validate/close — stages are config), and the two gates that make skipping a stage fail the commit. |
| [queue-kit/](queue-kit/) | **landed** | A git-native, agent-readable task tracker: the TASK-QUEUE format, one slug namespace, the blocked-by/needs-spec/spec tag algebra, `queue-index.sh`, and six gates holding the grammar an agent selects work by. |
| [spec-kit/](spec-kit/) | **landed** | Spec discipline for agent-authored components: one canonical spec per component, deltas as short-lived amendment files, a content-tiering star topology (one owner per fact; cite, never restate), and five gates over the copy-shaped failure modes. |
| [friction-kit/](friction-kit/) | **in extraction** | Permission-friction tooling: a bash command-guard, tracked-vs-local settings curation, and a close-stage friction-triage step. |
| delegation-kit/ | planned | Agent-execution protocol templates, usage gating, resume-journal mechanics. |
| context-kit/ | planned | Markdown/pub indexes, session-context hooks, always-loaded-baseline metering. |
| drift-kit/ | planned | Drift reporting with pluggable KPIs and lead/lag honesty labels. |

The **planned** kits are extracted in the order listed; each lands with its own
fixtures and README. `friction-kit` entered as an unsequenced candidate, to be
slotted only if permission friction kept compounding — it did, and the active
iteration is extracting it ahead of the planned three. The repo is a monorepo
— a kit is split out only if it earns independent adoption.

## This repo, governed

The gates registered in [`scripts/gates.list`](scripts/gates.list) run on this
tree: `bash gate-sdk/bin/run-gates.sh` for the full battery,
`bash gate-sdk/bin/install-hooks.sh` to opt this clone into the generated
pre-commit hook. The repo also runs lifecycle-kit's own iteration state
machine — [`TASK-QUEUE.md`](TASK-QUEUE.md) carries the stage header, one
iteration per kit extraction.

## License

[Apache-2.0](LICENSE).
