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
| lifecycle-kit/ | planned | The iteration stage state machine: stage skills (scope/align/build/validate/close), exit conditions as config, evidence stamps. |
| queue-kit/ | planned | A git-native, agent-readable task tracker: the TASK-QUEUE format, slug namespace, blocked-by/needs-spec tag algebra, hygiene gates. |
| spec-kit/ | planned | Spec amendment lifecycle, causal-completeness checklist, content-tiering star topology (one owner per fact; cite, never restate). |
| delegation-kit/ | planned | Agent-execution protocol templates, usage gating, resume-journal mechanics. |
| context-kit/ | planned | Markdown/pub indexes, session-context hooks, always-loaded-baseline metering. |
| drift-kit/ | planned | Drift reporting with pluggable KPIs and lead/lag honesty labels. |

Kits are extracted in this order; each lands with its own fixtures and README.
The repo is a monorepo — a kit is split out only if it earns independent
adoption.

## This repo, governed

The gates registered in [`scripts/gates.list`](scripts/gates.list) run on this
tree: `bash gate-sdk/bin/run-gates.sh` for the full battery,
`bash gate-sdk/bin/install-hooks.sh` to opt this clone into the generated
pre-commit hook.

## License

[Apache-2.0](LICENSE).
