# Checkwright

[![gates](https://github.com/checkwright/checkwright/actions/workflows/gates.yml/badge.svg)](https://github.com/checkwright/checkwright/actions/workflows/gates.yml)
[![release](https://img.shields.io/github/v/tag/checkwright/checkwright?label=release)](https://github.com/checkwright/checkwright/releases)

**Verification for coding-agent delivery.** Checkwright is the verification
layer under agent orchestration: spec drift, skipped stages, and unsupported
*done* claims become failing checks before a merge, instead of review findings
after one. It ships as installable kits — harness-independent gates plus an
evidence-stamped iteration lifecycle designed for stateless agent sessions.

**For the maintainer of a repository coding agents write most of**, who has to
answer at merge time whether the work is actually done, and cannot answer it by
reading every diff.

**It complements the workflow you already run.** Keep your spec process, your
prompts, your harness. Add Checkwright where a claim has to be mechanically
proven rather than asserted: the instructions shape, the gates enforce. Why that
split is the whole design: [Where Checkwright sits](docs/positioning.md).

One command runs the entire arc against a throwaway consumer repo, installing
nothing and touching no tree but its own:

```bash
bash demo/run-demo.sh
```

It vendors the kits into a fresh git repo, passes the battery clean, introduces
a defect and shows the gate that blocks it, then drops the defect and goes green
again. The script is [`demo/run-demo.sh`](demo/run-demo.sh), and it runs on every
validate stage, so the walkthrough cannot rot.

## What that buys you

**Before.** A session finishes a task and marks it done; the evidence is the
session's own say-so. A page keeps citing a spec section that a rename moved out
from under it. Both commits go in green, and the next stateless session reads
both as ground truth.

**After.** Neither commit lands:

```text
===== check-md-refs =====
check-md-refs: dangling reference in the governed doc set
  docs/guide.md:71 -> SPEC.md §Retry budget — no such section
FAIL: check-md-refs
===== check-stage-evidence =====
check-stage-evidence: a task reached Done with no validate stamp this iteration
FAIL: check-stage-evidence
```

Nothing there is a review opinion: each finding is cheap, mechanically decidable,
and low-false-positive by construction, which is what lets it block a commit
rather than open a thread. The semantic residue — is this design right, does the
evidence earn the claim — stays with the human or the agent, undiluted.

Where the project is heading, and what moves an item: [`ROADMAP.md`](ROADMAP.md),
generated from the queue entries a maintainer marked for the page and
freshness-gated on every commit. What is already *ruled* — the objectives that
work serves, and the decisions closed against them — is
[`TRAJECTORY.md`](TRAJECTORY.md), hand-authored rather than generated. Docs live
at <https://checkwright.dev> — the same pages served in-repo under
[`docs/`](docs/index.md).

## Quick start

Vendors a kit profile into a clean git repo and commits it. The primary path is
the **release tarball** — download it and its `.sha256` off the
[releases](https://github.com/checkwright/checkwright/releases) page, verify,
extract, run `init` — which needs nothing beyond a GNU userland; `npx
checkwright init` is the same vendoring over npm, for a consumer who already has
Node. Both recipes, with profiles and requirements:
[docs/install.md](docs/install.md) §Quick start.

## The premise

When coding agents do the writing, discipline does not hold: conventions live in
prose no stateless session reliably re-reads, and drift is silent. The remedy is
mechanization — every cheap, low-false-positive, mechanically-decidable
consistency axis is enforced by a gate that blocks the commit, and the human (or
agent) residue is held to the irreducibly semantic judgment alone. Checkwright
packages that machinery, and this repository governs itself with its own kits,
day one. A *wright* is a craftsman — shipwright, playwright; this is the craft
of checks.

## Kits

| Kit | What it is |
|---|---|
| [gate-sdk/](gate-sdk/) | A self-testing lint framework for prose/spec/config surfaces: the gate contracts (output, fail-closed, fixture-pair, self-lint), the fixture runner, `# graph:` coupling manifests, and a generated pre-commit hook. |
| [lifecycle-kit/](lifecycle-kit/) | The iteration stage state machine for stateless agent sessions: an iteration header + evidence-stamp file (its last stamp the stage cursor), stage-skill templates (scope/align/build/validate/close by default, plus an optional trigger-gated authoring stage — stages are config), and the gates that make skipping a stage — or clearing a lesson without dispositioning it — fail the commit. |
| [queue-kit/](queue-kit/) | A git-native, agent-readable task tracker: the TASK-QUEUE format, one slug namespace, the tag algebra over tasks (blocked-by/design-pending/spec/drain-exempt/roadmap/precondition-ok) and over Lessons Learned (an in-iteration attention channel plus consumer-named harvest tags), the queue-reading arms the binary carries (queue-kit/SPEC.md rosters them), the `roadmap` one projecting the curated entries onto a generated public page, and the gates that hold the grammar an agent selects work by. |
| [canon-kit/](canon-kit/) | Spec discipline for agent-authored components: one canonical spec per component, deltas as short-lived amendment files, a content-tiering star topology (one owner per fact; cite, never restate), and gates over the copy-shaped failure modes. |
| [guard-kit/](guard-kit/) | Permission-friction tooling for agent sessions: a `PreToolUse` guard framework (block/steer/rewrite/auto-allow) with a harness-generic ruleset, a prompt-source scanner, tracked-vs-local allowlist curation, an optional wakeup-guard, and a close-stage friction-triage step. Registers no gates. |
| [delegation-kit/](delegation-kit/) | Safe delegated-agent execution for budget-bounded sessions: the supervisor protocol (serialize on the shared git index, one commit per unit, resume journal, verify after every agent commit), a trustworthy budget verdict (`usage-verdict`), a `SubagentStop` turn-end hook that refuses a turn ended over a live recorded producer, and `check-gate-tamper` — a commit-shape gate blocking the two attested gate-weakening shapes. |
| [context-kit/](context-kit/) | Token-economics-aware context management: the index-first reading arms (`--emit md-index`/`md-section`/`pub-index`), a session-start hook that assembles a compact brief, an always-loaded meter with its committed baseline, and `check-brevity` over the densest always-loaded section. |
| [drift-kit/](drift-kit/) | Advisory drift reporting for stateless sessions: a `--emit drift-report` arm that collates pluggable KPIs from the other kits' governed surfaces under lead/lag honesty labels, a KPI plugin registry, a one-line trend summary the session hook injects, and the knowledge-friction loop. Registers no gates. |
| [evidence-kit/](evidence-kit/) | A held-constant test baseline and a committed per-run evidence manifest for validate: a stage stamp proves a stage was invoked, this proves it produced its green result. The versioned manifest (`# contract: evidence-manifest v1`) is a wire contract an external verifier can consume; ships `run-validate.sh`/`diff-baseline.sh` and gates over baseline grammar/slug-liveness and manifest grammar/close-entry coupling. |
| [site-kit/](site-kit/) | Deployment-truth governance for a repo-served docs site: `check-docs-cname-parity` makes the CNAME file the single gated source of truth for the docs host (no tracked file cites a configured alias in a URL; the alias set is consumer config), `check-docs-render-fidelity` re-renders every tracked docs page through the Pages parser and reds on the observed leakage classes, and a `site-health.yml` template scheduled-probes the live deployment (HTTPS, redirects, cert expiry, and release-body note pointers) as a monitor, never a gate. |
| [doctrine-kit/](doctrine-kit/) | The experience-packaging rung: the cross-kit delivery doctrine the other kits enforce piecemeal, stated once in a customer-deliverable `DOCTRINE.md` — referenced by link into a consumer's always-loaded agent file (re-vendor to upgrade, never copy-installed), installed by `install-doctrine.sh`, and held present by `check-doctrine-registration`. Ships the rule statements only; each kit's SPEC owns its mechanism, so no private rule content crosses the seam. |

Every kit ships its own fixtures, README, and SPEC. The repo is a monorepo — a
kit is split out only if it earns independent adoption.

## This repo, governed

The gates registered in [`scripts/gates.list`](scripts/gates.list) run on this
tree. What a *commit* owes is the full battery plus the touched kit's fixture
suite — that selection rule is [`CLAUDE.md`](CLAUDE.md)'s, stated there and not
restated here.

The block below is a different thing: the **register of this repo's runnable
verification suites**, the set the validate stage runs in full. It is held in
name-set parity with the configured `EVIDENCE_KIT_SUITES` by
`check-battery-roster`, so the register is complete by enforcement — a suite
validate runs and this block omits is red, and so is a line whose command runs
no configured suite.

<!-- battery-roster:begin -->
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
bash gate-sdk/bin/run-gate-tests.sh guard-kit/gate-tests                            # guard-kit bin unit tests
bash guard-kit/bin/run-guard-tests.sh                                               # guard-kit decision table
bash delegation-kit/bin/run-usage-tests.sh                                          # delegation-kit usage accounting
bash delegation-kit/bin/run-trend-tests.sh                                          # delegation-kit trend reader
bash context-kit/bin/run-index-tests.sh                                             # context-kit index tools
cargo test --release --manifest-path native/Cargo.toml                              # native crate unit tests
bash context-kit/smoke/agents-md.sh                                                 # the AGENTS.md projection, end to end
bash gate-sdk/bin/run-consumer-smoke.sh                                             # every kit installs into a scratch consumer
bash gate-sdk/bin/upgrade-smoke.sh                                                  # a vendored tree upgrades in place
bash installer/consumer-smoke/run-smoke.sh                                          # the activation path, per profile
bash demo/run-demo.sh                                                               # the adoption walkthrough
```
<!-- battery-roster:end -->

`bash gate-sdk/bin/install-hooks.sh` opts this clone into the generated
pre-commit and commit-msg hooks. The repo also runs lifecycle-kit's own iteration state
machine — [`TASK-QUEUE.md`](TASK-QUEUE.md) carries the iteration header, one
iteration per hardening or roadmap unit.

Contributing: the fixture is the unit of contribution — see
[`CONTRIBUTING.md`](CONTRIBUTING.md) and [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).
Reporting a vulnerability: [`SECURITY.md`](SECURITY.md), never a public issue.

## License

[Apache-2.0](LICENSE).
