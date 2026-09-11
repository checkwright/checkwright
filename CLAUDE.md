# CLAUDE.md — Checkwright

Checkwright packages a coding-agent-assisted delivery methodology as
installable kits. The private companion brief — seam boundary,
identity/namespace ownership, forward design memory behind the deferred-queue
rungs — is `BRIEF.local.md`, which is **local-only and untracked** (it carries
private context that must never be committed); consult it before roadmap/seam
work. The kit map lives in [README.md](README.md).

This repo is public: no local paths, private repo/project names, accounts, or
internal session/commit references in tracked files or commit messages —
including the harness's default `Claude-Session:` commit trailer.

## The provenance seam (never cross it)

A kit ships generic mechanism only. **Private rule content never lands here** —
term lists, coupling vocabularies, glossary bodies, wire-contract couplings,
product constant sets — **and this project's provenance**: a dated operator
stamp, a `TRAJECTORY.md` pointer, a refused alternative's grounds belong in
`TRAJECTORY.md` and git history, never in a kit SPEC, which states the rule
undated. When a kit component needs such content, it becomes
optional consumer config (the `check-graph` / `scripts/graph-vocab.sh`
pattern), never a kit literal. This is a privacy boundary before it is a design
one: a kit literal carrying a private vocabulary publishes it.

## This repo is governed by its own kits

The gates in [`scripts/gates.list`](scripts/gates.list) run on this tree —
dogfooding is day-one, not optional, though *which* gates register is a tuning
decision here as for any consumer (`operator 2026-09-10`). Before committing, run
the full battery (`bash gate-sdk/bin/run-gates.sh`) and the touched kit's fixture
suite, whose per-kit runner is in [README.md](README.md) §This repo, governed.

The git index is shared with any concurrent session: check `git status` for a
foreign staged path before `git add`, or stage and commit in one motion.

Internal work lands as **direct commits to master and never rides a pull
request** — the PR channel is inbound-only, for external contributors. A master
push is verified against the remote oracle: watch the `gates` workflow
to green (`gh run watch`) before calling the push done — and **budget one to two
pushes per iteration**. A finished run is read for free with
`gh run view <id> --log`, so a new round is owed only when the facts wanted were
never printed. Commits accumulate locally and ride one watched push at close; a
release tag earns the second run (`publish`). Every push also costs a
`pages-build-deployment`, so drip-pushing re-buys one push's information at N
times the wall-clock. Never hand-dispatch `site-health` — it runs on a schedule.

One iteration per hardening or roadmap unit. The stage cursor has exactly one
source, the last stamp in `.workflow/WORKFLOW-STATE.txt`, and stage motion never
writes the queue (`check-stage-evidence` / `check-stage-entry` enforce the stamp
protocol; `check-lifecycle-registration` holds the block below in lockstep with
the machine).

<!-- lifecycle-kit:begin -->
The repo runs lifecycle-kit's iteration state machine on `TASK-QUEUE.md` — one
stage session per stage, each invoking its skill:
`/scope` `/spec` `/align` `/build` `/validate` `/close`.
The state machine, its stamp protocol, and the per-stage contracts:
[lifecycle-kit/SPEC.md](lifecycle-kit/SPEC.md).
<!-- lifecycle-kit:end -->

The pre-commit hook is **generated** — never hand-edit
`scripts/git-hooks/pre-commit`; edit a gate's `# graph:` manifest and regenerate.
Per-clone opt-in: `bash gate-sdk/bin/run-gates.sh --install-hooks`. Every generated
projection, that hook included, is rostered with its trigger, its regen command and
the wide fan-outs in [docs/site-architecture.md](docs/site-architecture.md)
§Generated projections and their freshness gates; each freshness gate prints its
own command on red.

New gates here are **born native** — a Rust module, a `.gate` descriptor and a
`good/`+`bad/` fixture pair; no gate is permanently shell, and a shell gate
needs a cause from the live exception classes, stated in its own SPEC section
(gate-sdk/SPEC.md §The port-candidate criteria). The four contracts (output,
fail-closed, fixture-pair, self-lint) are
[gate-sdk/SPEC.md](gate-sdk/SPEC.md)'s, enforced by the meta-gates, and the
port oracle (gate-sdk/SPEC.md §port-blockers) answers for the battery and, with
`--tree`, for the project. A red gate is fixed, never bypassed with
`--no-verify` except as a one-off with cause.

Comments are directives, else deleted — a passing `check-comment-tier` is the
floor, not licence to keep a comment. Blessing a restatement (relocating prose
behind a `spec:` or `comment-tier-exempt:` tag rather than deleting it) is
itself the defect; the doctrine and the one-line-binding rule for `spec:` live
in [canon-kit/SPEC.md](canon-kit/SPEC.md) §check-comment-tier.

<!-- doctrine-kit:begin -->
## Delivery doctrine

The cross-kit delivery rules live in [doctrine-kit/DOCTRINE.md](doctrine-kit/DOCTRINE.md) — re-vendor
to upgrade. The always-loaded maintenance rules, one line each; the doctrine adds
an engineering-craft section behind the link:

- **Content-tiering / SSOT** — one content tier per surface; point, never restate.
- **Enforcement-first** — the fix and the gate that catches it land in one unit; removing the duplication outranks gating it.
- **De-literalization** — prose cites names; code or the owning SPEC owns values.
- **Derivation-first** — derive the derivable (a roster, a count), never maintain it; a needed copy is generated and freshness-gated.
- **Always-loaded shape** — one line per rule here; the mechanism behind the pointer.
- **Load-trigger residency** — resident only when no stage, skill, or tool loads it.
- **Widest-true-tier placement** — the widest tier true for every reader of it.
- **Oracle-first** — run the gate, never emulate it; a red run is the feedback channel.
- **Spec-over-precedent** — the owner doc is ground truth; history answers what happened, never what is correct.
- **Gap disposition** — a gap you defer is costed and filed, never flagged-and-skipped.
- **Scope-gated intake** — a mid-session initiative is filed as a costed Deferred entry by default, never started; work enters only through scope — or through an operator-ruled hotfix of an impacting failure, minimal and test-and-doc-complete in one commit.
- **Probe-before-assertion** — a claim one cheap command would settle is probed before it is asserted; relaying an unverified premise is asserting it.
<!-- doctrine-kit:end -->

## Conventions established in gate-sdk (keep every kit consistent)

- **Registry, not array:** gates register by name in `gates.list`, resolving consumer-first with kit shadowing — resolution order and the kit-dirs knob: gate-sdk/SPEC.md §Layout and configuration.
- **Config via env:** every kit takes `<KIT>_<KNOB>` with this repo's layout as the defaults; each kit's SPEC owns its knob roster and values.
- **Self-contained artifacts:** emitted HTML inlines its CSS and no kit output references an asset outside the kit — the one sanctioned exception and its honest limit: gate-sdk/SPEC.md §check-graph.
- **Kit-landing checklist:** README + SPEC.md, fixtures for every shipped gate, `smoke/`, and `gates.list` registration where applicable — gate-sdk/SPEC.md §Consumer smoke owns it.

## Agent execution (all stages)

Delegation is pre-authorized for read-heavy audits and mechanical rename/merge
sweeps — no ask needed. **Full protocol: `/agent-execution`.** The safety rules,
resume-journal mechanics, verify-after-commit set, and gate-driven worklist
load behind that trigger, so they are not resident here.

## Housekeeping

- `.tmp/` is gitignored disposable scratch — gate timings, resume journals,
  `<key>.run` liveness records — that the scope boundary wipes (keep-list:
  `scripts/lifecycle-config.sh`); `.metric/` is gitignored persistent,
  account-bearing measurement, **never committed**; `.workflow/` holds tracked
  projections beside gitignored capture
  (gate-sdk/SPEC.md §The workflow directory).
- Local-only and gitignored: `BRIEF.local.md` (private brief), `ENV.local.md`
  (probed machine profile plus gotchas — context-kit/SPEC.md §bin/env-probe) and
  `OPS.local.md` (DNS, repo settings, the release account and push transport).
  Consult `OPS.local.md` before any domain, repo-settings, release or push work,
  and run its account step before **any GitHub write**: a write needing no
  permission succeeds silently under the wrong account, so the step is per-write,
  never per-session.
- `reserve/` holds the crates.io name-reservation placeholder — do not develop
  in it (the npm name is the `installer/` package below).
- `native/` is the gate binary's Rust crate — one multi-call binary, a
  subcommand per gate plus the non-gate arms. The commit-time obligation is the
  battery, which runs the crate's lint and test arms through `check-crate-arms`,
  **plus** `bash gate-sdk/bin/build-native.sh`; neither discharges the other. It
  is **not a kit** — no `checks/`, no `smoke/`, the predicate that makes a root
  directory one — and `check-gate-binary-fresh` holds the binary's currency.
  Dispatch, descriptors, port sequencing and the toolchain floor:
  gate-sdk/SPEC.md §Porting a gate to the binary substrate.
- The governed repo-meta pinned in `scripts/core-files.list` is tracked and gated
  like any doc; the fixture is the unit of contribution, so edit the guide, not
  GitHub UI settings.
- `ROADMAP.md` is a generated root projection of this queue's curated
  `[roadmap:]` tags — never hand-edit its marker block.
- [`TRAJECTORY.md`](TRAJECTORY.md) is the hand-authored **override ledger** —
  the objectives of the running pivot and the operator's rulings, each naming
  the instruction it overrides and its discharge, and leaving the file on
  discharge. A ruling is closed: escalate it to `/consult`, never reverse,
  annotate or re-verify it. **Only the operator rules, and only through
  `/consult`**; what an operator says in a lead or stage session is a
  direction, revisable at scope or spec, and lands on the work surface it
  concerns as `operator direction, <date>`. The terms (objective, ruling,
  direction, decision, grant): lifecycle-kit/SPEC.md §The steering vocabulary.
- **A permission-settings edit is the operator's, applied out of band** — never
  build work, never queue work, and never promoted by scope; a session prepares
  the diff and states its security grounds and stops there. Criterion, grounds
  and the refused hand-it-up alternative: guard-kit/SPEC.md
  §compare-settings-allow.
- `docs/` is the public GitHub-Pages site (served from `docs/` on master via its
  `CNAME`), repo-root-governed, no owning kit; its chrome, page-authoring rules,
  generated projections and docs gate roster live in the load-triggered
  [docs/site-architecture.md](docs/site-architecture.md).
- `installer/` is the published activation surface — a bash and a PowerShell
  bootstrap in front of the gate binary, shipped as a Release tarball and an npm
  package from one payload; repo-root-governed, and not a kit by the predicate
  under `native/` above. Its payload is never committed. Layout, boundary and
  packing: installer/README.md.
- **Knowledge-friction capture (any session):** re-deriving a fact no doc owns
  (off an implementation, a gate's source, a commit, or a prior/sibling
  deliverable)? stamp it in the moment with
  `bash gate-sdk/bin/run-gates.sh --emit kfric "<fact>" "<surface>"` — deferred capture is
  no capture; close triages it (drift-kit/SPEC.md §The knowledge-friction loop).
- **Gap capture (any mid-iteration session):** a work-shaped finding — a gap,
  a task, a defect — routes to the committed gap inbox with
  `bash gate-sdk/bin/run-gates.sh --emit file-gap "<gap>"`, never a mid-iteration queue
  edit contending on a stage session's surface; close drains it
  (lifecycle-kit/SPEC.md §The committed gap inbox). The sanctioned exception:
  an operator-directed filing may land in the queue directly, staged and
  committed in one motion under the shared-index rule above.
- **Survey capture (any stage session):** bought a survey — a census, a cohort
  sweep, a roster over a corpus — that a later stage will want? land it before
  you act on it with
  `bash gate-sdk/bin/run-gates.sh --emit file-survey "<question>" "<corpus>" "<oracle>" "<edges>" "<finding>"`;
  and before buying one, read the record and run its witness
  (lifecycle-kit/SPEC.md §The survey record).
- No per-user memory files: durable guidance goes in tracked manifests (this file,
  kit SPECs) or `BRIEF.local.md`. Harness auto-memory is enforced off; doctrine:
  context-kit/SPEC.md §The memory-off doctrine.
