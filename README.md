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
| [lifecycle-kit/](lifecycle-kit/) | **landed** | The iteration stage state machine for stateless agent sessions: a stage header + evidence-stamp file, stage-skill templates (scope/align/build/validate/close — stages are config), and the gates that make skipping a stage fail the commit. |
| [queue-kit/](queue-kit/) | **landed** | A git-native, agent-readable task tracker: the TASK-QUEUE format, one slug namespace, the blocked-by/needs-spec/spec tag algebra, `queue-index.sh`, and the gates that hold the grammar an agent selects work by. |
| [spec-kit/](spec-kit/) | **landed** | Spec discipline for agent-authored components: one canonical spec per component, deltas as short-lived amendment files, a content-tiering star topology (one owner per fact; cite, never restate), and gates over the copy-shaped failure modes. |
| [guard-kit/](guard-kit/) | **landed** | Permission-friction tooling for agent sessions: a `PreToolUse` guard framework (block/steer/rewrite/auto-allow) with a harness-generic ruleset, a prompt-source scanner, tracked-vs-local allowlist curation, an optional wakeup-guard, and a close-stage friction-triage step. Registers no gates. |
| [delegation-kit/](delegation-kit/) | **landed** | Safe delegated-agent execution for budget-bounded sessions: the supervisor protocol (serialize on the shared git index, one commit per unit, resume journal, validate after every agent commit), a trustworthy budget verdict (`usage-verdict`), and `check-gate-tamper` — a commit-shape gate blocking the two attested gate-weakening shapes. |
| [context-kit/](context-kit/) | **landed** | Token-economics-aware context management: the index-first reading tools (md-index/md-section/pub-index), a session-start hook that assembles a compact brief, an always-loaded meter with its committed baseline, and `check-brevity` over the densest always-loaded section. |
| [drift-kit/](drift-kit/) | **landed** | Advisory drift reporting for stateless sessions: a `drift-report.sh` that collates pluggable KPIs from the other kits' governed surfaces under lead/lag honesty labels, a KPI plugin registry, a one-line trend summary the session hook injects, and the knowledge-friction loop. Registers no gates. |
| [evidence-kit/](evidence-kit/) | **landed** | A held-constant test baseline and a committed per-run evidence manifest for validate: a stage stamp proves a stage was invoked, this proves it produced its green result. The versioned manifest (`# contract: evidence-manifest v1`) is a wire contract an external verifier can consume; ships `run-validate.sh`/`diff-baseline.sh` and gates over baseline grammar/slug-liveness and manifest grammar/close-entry coupling. |

Every kit landed with its own fixtures and README, extracted in the order
listed. `guard-kit` entered as an unsequenced candidate, to be slotted only if
permission friction kept compounding — it did, and it landed ahead of the
planned sequence. The repo is a monorepo — a kit is split out only if it earns
independent adoption.

## This repo, governed

The gates registered in [`scripts/gates.list`](scripts/gates.list) run on this
tree: `bash gate-sdk/bin/run-gates.sh` for the full battery,
`bash gate-sdk/bin/install-hooks.sh` to opt this clone into the generated
pre-commit hook. The repo also runs lifecycle-kit's own iteration state
machine — [`TASK-QUEUE.md`](TASK-QUEUE.md) carries the stage header, one
iteration per kit through extraction, then per hardening or roadmap unit.

Contributing: the fixture is the unit of contribution — see
[`CONTRIBUTING.md`](CONTRIBUTING.md).

## License

[Apache-2.0](LICENSE).
