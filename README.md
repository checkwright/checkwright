# Checkwright

[![gates](https://github.com/checkwright/checkwright/actions/workflows/gates.yml/badge.svg)](https://github.com/checkwright/checkwright/actions/workflows/gates.yml)
[![release](https://img.shields.io/github/v/tag/checkwright/checkwright?label=release)](https://github.com/checkwright/checkwright/releases)

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
judgment alone. Checkwright packages that machinery — and this repository
governs itself with its own kits, day one.

Docs live at <https://checkwright.dev> — the same pages served in-repo
under [`docs/`](docs/index.md).

## Kits

| Kit | What it is |
|---|---|
| [gate-sdk/](gate-sdk/) | A self-testing lint framework for prose/spec/config surfaces: the gate contracts (output, fail-closed, fixture-pair, self-lint), the fixture runner, `# graph:` coupling manifests, and a generated pre-commit hook. |
| [lifecycle-kit/](lifecycle-kit/) | The iteration stage state machine for stateless agent sessions: a stage header + evidence-stamp file, stage-skill templates (scope/align/build/validate/close — stages are config), and the gates that make skipping a stage — or clearing a lesson without dispositioning it — fail the commit. |
| [queue-kit/](queue-kit/) | A git-native, agent-readable task tracker: the TASK-QUEUE format, one slug namespace, the tag algebra over tasks (blocked-by/needs-spec/spec/precondition-ok) and over Lessons Learned (an in-iteration attention channel plus consumer-named harvest tags), `queue-index.sh`, and the gates that hold the grammar an agent selects work by. |
| [canon-kit/](canon-kit/) | Spec discipline for agent-authored components: one canonical spec per component, deltas as short-lived amendment files, a content-tiering star topology (one owner per fact; cite, never restate), and gates over the copy-shaped failure modes. |
| [guard-kit/](guard-kit/) | Permission-friction tooling for agent sessions: a `PreToolUse` guard framework (block/steer/rewrite/auto-allow) with a harness-generic ruleset, a prompt-source scanner, tracked-vs-local allowlist curation, an optional wakeup-guard, and a close-stage friction-triage step. Registers no gates. |
| [delegation-kit/](delegation-kit/) | Safe delegated-agent execution for budget-bounded sessions: the supervisor protocol (serialize on the shared git index, one commit per unit, resume journal, validate after every agent commit), a trustworthy budget verdict (`usage-verdict`), and `check-gate-tamper` — a commit-shape gate blocking the two attested gate-weakening shapes. |
| [context-kit/](context-kit/) | Token-economics-aware context management: the index-first reading tools (md-index/md-section/pub-index), a session-start hook that assembles a compact brief, an always-loaded meter with its committed baseline, and `check-brevity` over the densest always-loaded section. |
| [drift-kit/](drift-kit/) | Advisory drift reporting for stateless sessions: a `drift-report.sh` that collates pluggable KPIs from the other kits' governed surfaces under lead/lag honesty labels, a KPI plugin registry, a one-line trend summary the session hook injects, and the knowledge-friction loop. Registers no gates. |
| [evidence-kit/](evidence-kit/) | A held-constant test baseline and a committed per-run evidence manifest for validate: a stage stamp proves a stage was invoked, this proves it produced its green result. The versioned manifest (`# contract: evidence-manifest v1`) is a wire contract an external verifier can consume; ships `run-validate.sh`/`diff-baseline.sh` and gates over baseline grammar/slug-liveness and manifest grammar/close-entry coupling. |
| [site-kit/](site-kit/) | Deployment-truth governance for a repo-served docs site: `check-docs-cname-parity` makes the CNAME file the single gated source of truth for the docs host (no tracked file cites a configured alias in a URL; the alias set is consumer config), `check-docs-render-fidelity` re-renders every tracked docs page through the pinned Pages parser and reds on the observed leakage classes, and a `site-health.yml` template scheduled-probes the live deployment (HTTPS, redirects, cert expiry) as a monitor, never a gate. |
| [doctrine-kit/](doctrine-kit/) | The experience-packaging rung: the cross-kit delivery doctrine the other kits enforce piecemeal, stated once in a customer-deliverable `DOCTRINE.md` — referenced by link into a consumer's always-loaded agent file (re-vendor to upgrade, never copy-installed), installed by `install-doctrine.sh`, and held present by `check-doctrine-registration`. Ships the rule statements only; each kit's SPEC owns its mechanism, so no private rule content crosses the seam. |

Every kit ships its own fixtures, README, and SPEC. The repo is a monorepo — a
kit is split out only if it earns independent adoption.

## This repo, governed

The gates registered in [`scripts/gates.list`](scripts/gates.list) run on this
tree. Before committing, run the full battery, each kit's fixture suite, and the
guard-kit decision table:

```bash
bash gate-sdk/bin/run-gates.sh                                                      # full battery
bash gate-sdk/bin/run-gate-tests.sh gate-sdk/gate-tests gate-sdk/checks             # gate-sdk fixtures
bash gate-sdk/bin/run-gate-tests.sh lifecycle-kit/gate-tests lifecycle-kit/checks   # lifecycle-kit fixtures
bash gate-sdk/bin/run-gate-tests.sh queue-kit/gate-tests queue-kit/checks           # queue-kit fixtures
bash gate-sdk/bin/run-gate-tests.sh canon-kit/gate-tests canon-kit/checks           # canon-kit fixtures
bash gate-sdk/bin/run-gate-tests.sh delegation-kit/gate-tests delegation-kit/checks # delegation-kit fixtures
bash gate-sdk/bin/run-gate-tests.sh context-kit/gate-tests context-kit/checks       # context-kit fixtures
bash gate-sdk/bin/run-gate-tests.sh evidence-kit/gate-tests evidence-kit/checks     # evidence-kit fixtures
bash gate-sdk/bin/run-gate-tests.sh site-kit/gate-tests site-kit/checks             # site-kit fixtures
bash gate-sdk/bin/run-gate-tests.sh doctrine-kit/gate-tests doctrine-kit/checks     # doctrine-kit fixtures
bash gate-sdk/bin/run-gate-tests.sh scripts/gate-tests                              # consumer-gate fixtures
bash guard-kit/bin/run-guard-tests.sh                                               # guard-kit decision table
```

`bash gate-sdk/bin/install-hooks.sh` opts this clone into the generated
pre-commit hook. The repo also runs lifecycle-kit's own iteration state
machine — [`TASK-QUEUE.md`](TASK-QUEUE.md) carries the stage header, one
iteration per hardening or roadmap unit.

Contributing: the fixture is the unit of contribution — see
[`CONTRIBUTING.md`](CONTRIBUTING.md).

## License

[Apache-2.0](LICENSE).
