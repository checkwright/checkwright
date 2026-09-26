# Checkwright

[![gates](https://github.com/checkwright/checkwright/actions/workflows/gates.yml/badge.svg)](https://github.com/checkwright/checkwright/actions/workflows/gates.yml) [![release](https://img.shields.io/github/v/tag/checkwright/checkwright?label=release)](https://github.com/checkwright/checkwright/releases)

**Verification for coding-agent delivery.** Checkwright is the verification layer under agent orchestration: spec drift, skipped stages, and unsupported *done* claims become failing checks before a merge, instead of review findings after one. It ships as installable kits: gates plus an evidence-stamped iteration lifecycle designed for stateless agent sessions.

Who it is for, how it complements the workflow you already run, and a done claim it catches: <https://checkwright.dev>, the same pages served in-repo under [`docs/`](docs/index.md).

## Try it first

`demo` runs the whole arc in a scratch repository of its own and removes it, without touching yours; [docs/install.md](docs/install.md) §Install says what each act does. It installs the `full` profile, which needs bash 4.3 or later: on stock macOS run the Homebrew bash step there first, and on Windows the Git for Windows step.

```sh
curl -fsSL https://checkwright.dev/install.sh | sh -s -- demo                  # macOS and Linux
```

```powershell
& ([scriptblock]::Create((irm https://checkwright.dev/install.ps1))) demo      # Windows, in PowerShell
```

```sh
npx checkwright demo                                                           # with Node
```

## Install

From the root of a clean git repository, the same three routes without `demo` install the kits as one commit, the first two from the Release tarball:

```sh
curl -fsSL https://checkwright.dev/install.sh | sh                             # macOS and Linux
```

```powershell
irm https://checkwright.dev/install.ps1 | iex                                  # Windows, in PowerShell
```

```sh
npx checkwright init                                                           # with Node
```

Profiles, the other verbs and how to pass them arguments: [docs/install.md](docs/install.md) §Install.

Where the project is heading, and what moves an item: [`ROADMAP.md`](ROADMAP.md). What is already *ruled* — the operator's standing overrides of business as usual — is [`TRAJECTORY.md`](TRAJECTORY.md), hand-authored rather than generated.

## The premise

When coding agents do the writing, discipline does not hold: conventions live in prose no stateless session reliably re-reads, and drift is silent. The remedy is mechanization — every cheap, low-false-positive, mechanically-decidable consistency axis is enforced by a gate that blocks the commit, and the human (or agent) residue is held to the irreducibly semantic judgment alone. Checkwright packages that machinery, and this repository governs itself with its own kits, day one. A *wright* is a craftsman — shipwright, playwright; this is the craft of checks.

## Kits

The kits, in reading order with a line on each: the [Kit Reference](docs/kits.md). Every kit ships its own fixtures, README, and SPEC in its own top-level directory. The repo is a monorepo — a kit is split out only if it earns independent adoption.

## This repo, governed

The gates registered in [`scripts/gates.list`](scripts/gates.list) run on this tree. What a *commit* owes is the full battery plus a selection of the fixture suites below — that selection rule is [`CLAUDE.md`](CLAUDE.md)'s, stated there and not restated here.

The block below is a different thing: the **register of this repo's runnable verification suites**, the set the validate stage runs in full. It is held in name-set parity with the configured `EVIDENCE_KIT_SUITES` by `check-battery-roster`, so the register is complete by enforcement — a suite validate runs and this block omits is red, and so is a line whose command runs no configured suite.

<!-- battery-roster:begin -->
<!-- door-contributor: the contributor battery register — every line is a suite the validate stage runs in full, typed in a clone whose binary is built by construction -->
```bash
bash gate-sdk/bin/run-gates.sh                                                                  # full battery
bash gate-sdk/bin/run-gates.sh --run-gate-tests gate-sdk/gate-tests gate-sdk/checks             # gate-sdk fixtures
bash gate-sdk/bin/run-gates.sh --run-gate-tests lifecycle-kit/gate-tests lifecycle-kit/checks   # lifecycle-kit fixtures
bash gate-sdk/bin/run-gates.sh --run-gate-tests queue-kit/gate-tests queue-kit/checks           # queue-kit fixtures
bash gate-sdk/bin/run-gates.sh --run-gate-tests canon-kit/gate-tests canon-kit/checks           # canon-kit fixtures
bash gate-sdk/bin/run-gates.sh --run-gate-tests delegation-kit/gate-tests delegation-kit/checks # delegation-kit fixtures
bash gate-sdk/bin/run-gates.sh --run-gate-tests context-kit/gate-tests context-kit/checks       # context-kit fixtures
bash gate-sdk/bin/run-gates.sh --run-gate-tests evidence-kit/gate-tests evidence-kit/checks     # evidence-kit fixtures
bash gate-sdk/bin/run-gates.sh --run-gate-tests site-kit/gate-tests site-kit/checks             # site-kit fixtures
bash gate-sdk/bin/run-gates.sh --run-gate-tests doctrine-kit/gate-tests doctrine-kit/checks     # doctrine-kit fixtures
bash gate-sdk/bin/run-gates.sh --run-gate-tests scripts/gate-tests                              # consumer-gate fixtures
bash gate-sdk/bin/run-gates.sh --run-gate-tests guard-kit/gate-tests guard-kit/checks           # guard-kit fixtures + bespoke seam suites
bash gate-sdk/bin/run-gates.sh --run-guard-tests                                                # guard-kit decision table
bash gate-sdk/bin/run-gates.sh --run-index-tests                                                 # context-kit index tools
cargo test --release --manifest-path native/Cargo.toml                                          # native crate unit tests
bash gate-sdk/bin/run-gates.sh --agents-md-smoke                                                # the AGENTS.md projection, end to end
bash gate-sdk/bin/run-gates.sh --run-consumer-smoke                                             # every kit installs into a scratch consumer
bash gate-sdk/bin/run-gates.sh --upgrade-smoke                                                  # a vendored tree upgrades in place
bash installer/consumer-smoke/run-smoke.sh                                                      # the activation path, per profile
bash gate-sdk/bin/run-gates.sh --run-demo                                                        # the adoption walkthrough
bash gate-sdk/bin/run-gates.sh --projection-witness                                              # each projection's declared trigger, perturbed
```
<!-- battery-roster:end -->

The last line, `--run-demo`, is the adoption walkthrough, and it runs from a checkout because it copies the kits out of this tree: against a throwaway consumer repo, touching no tree but its own, it vendors the kits, passes the battery clean, introduces a defect and shows the gate that blocks it, then drops the defect and goes green again. A checkout tracks no binary, so `bash gate-sdk/bin/build-native.sh` builds one first. The arm is specified in [gate-sdk/SPEC.md](gate-sdk/SPEC.md) §Consumer smoke.

The gate binary's `--install-hooks` arm opts this clone into the generated pre-commit and commit-msg hooks. The repo also runs lifecycle-kit's own iteration state machine — [`TASK-QUEUE.md`](TASK-QUEUE.md) carries the iteration header, one iteration per hardening or roadmap unit.

Contributing: the fixture is the unit of contribution — see [`CONTRIBUTING.md`](CONTRIBUTING.md) and [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md). Reporting a vulnerability: [`SECURITY.md`](SECURITY.md), never a public issue.

## License

[Apache-2.0](LICENSE).
