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
product constant sets. When a kit component needs such content, it becomes
optional consumer config (the `check-graph` / `scripts/graph-vocab.sh`
pattern), never a kit literal. This is a privacy boundary before it is a design
one: a kit literal carrying a private vocabulary publishes it.

## This repo is governed by its own kits

The gates in [`scripts/gates.list`](scripts/gates.list) run on this tree —
dogfooding is day-one, not optional. Before committing, run the full battery
(`bash gate-sdk/bin/run-gates.sh`) and the touched kit's fixture suite, whose
per-kit runner lives in [README.md](README.md) §This repo, governed.

The git index is shared with any concurrent session: check `git status` for a
foreign staged path before `git add`, or stage and commit in one motion.

A master push is verified against the remote oracle: watch the `gates` workflow
to green (`gh run watch`) before calling the push done — and **budget one to two
pushes per iteration**. Commits accumulate locally and ride a single watched push
at close; a release tag earns the second run (`publish`). Every push also costs a
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
Per-clone opt-in: `bash gate-sdk/bin/install-hooks.sh`. Every generated
projection, that hook included, is rostered with its trigger and regen command
in [docs/site-architecture.md](docs/site-architecture.md) §Generated
projections, which also carries the full fan-out a new gate stales; each
freshness gate prints its own command on red.

New gates here are **born native** — a Rust module plus a `.gate` descriptor;
shell needs a cause from the two live exception classes, stated in the gate's
own SPEC section (gate-sdk/SPEC.md §The port-candidate criteria) — no gate is
permanently shell (TRAJECTORY.md §The closed rulings, 2026-08-23). `port-blockers.sh`
has **two** oracles: its registry arms answer for the battery, `--tree` for the
project, and only `--tree`'s owed count is the completion predicate. Either
substrate ships with a `good/`+`bad/` fixture pair; the four contracts (output,
fail-closed, fixture-pair, self-lint) are specified in
[gate-sdk/SPEC.md](gate-sdk/SPEC.md) and enforced by the meta-gates — a red gate
is fixed, never bypassed with `--no-verify` except as a one-off with cause.

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
- **Scope-gated intake** — a mid-session initiative is filed as a costed Deferred entry by default, never started; work enters only through scope.
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

- `.tmp/` is gitignored, purely disposable scratch (gate timings, resume
  journals, and the `<key>.run` launch-liveness records a backgrounding session
  leaves), wiped at the scope boundary by `enter-stage.sh`'s boundary reset —
  mechanized, not by hand; the keep-list is in `scripts/lifecycle-config.sh`
  (lifecycle-kit/SPEC.md §bin/enter-stage.sh); `.metric/` is gitignored persistent
  measurement trends — **never committed**,
  account-bearing (drift-kit/SPEC.md §Layout and configuration); `.workflow/`
  holds two tiers — tracked checked projections beside gitignored local capture
  (gate-sdk/SPEC.md §The workflow directory). `BRIEF.local.md` (private brief),
  `OPS.local.md` (private ops runbook — DNS, GitHub repo-settings desired state,
  and the release account and push transport; consult it before any domain,
  repo-settings **or release** work), and `ENV.local.md`
  (context-kit's probed machine profile plus hand-authored gotchas — seed with
  `bash context-kit/bin/env-probe.sh`, context-kit/SPEC.md §bin/env-probe) are
  gitignored, local-only.
- `reserve/` holds the crates.io name-reservation placeholder — do not develop
  in it (the npm name is the `installer/` package below).
- `native/` is the Rust crate off the shell substrate — one multi-call binary:
  one subcommand per ported gate, **plus the non-gate arms** that ported emitters
  and tools register in. **The binary is live**, so the commit-time obligation in
  this tree is the battery — which runs the crate's lint and test arms through
  `check-crate-arms` — **plus** `bash gate-sdk/bin/build-native.sh`, and neither
  discharges the other. It is **not a kit** — no `checks/`, no `smoke/`, the
  predicate that makes a root directory one; `check-gate-binary-fresh` holds the
  binary's currency. Dispatch, descriptor format, port sequencing and the
  toolchain floor: gate-sdk/SPEC.md §Porting a gate to the binary substrate;
  what a non-gate port costs: gate-sdk/SPEC.md §The non-gate arm; the shipped
  install behavior: installer/README.md §The gate binary.
- The governed repo-meta pinned in `scripts/core-files.list` is tracked and gated
  like any doc; the fixture is the unit of contribution, so edit the guide, not
  GitHub UI settings.
- `ROADMAP.md` is a generated root projection of this queue's curated
  `[roadmap:]` tags — never hand-edit its marker block; it is rostered with its
  trigger and regen command like every other generated projection.
- [`TRAJECTORY.md`](TRAJECTORY.md) is the hand-authored ruling record — the
  objectives, the closed operator rulings, and the port sequence; a recorded
  ruling is closed, so escalate rather than reverse one, and retiring a spent
  ruling is not reversing it.
- `docs/` is the public GitHub-Pages site (served from `docs/` on master via its
  `CNAME`), repo-root-governed, no owning kit; its chrome, page-authoring rules,
  generated projections and docs gate roster live in the load-triggered
  [docs/site-architecture.md](docs/site-architecture.md).
- `demo/run-demo.sh` is the runnable adoption walkthrough (vendor → clean pass →
  violation blocked → fix → green), the evidence-kit `demo` validate suite on the
  gate-sdk consumer-smoke mechanics, so a bit-rotted walkthrough is a red validate.
- `installer/` is the published activation surface (bash inside, shipped over
  two transports from one payload — the Release tarball and the npm package),
  repo-root-governed, no owning kit; its layout is
  installer/README.md. Not a kit either, and must not become one — by the same
  predicate stated under `native/` above. Its payload is never
  committed: `scripts/pack-installer.sh` assembles it out of tree from this
  repo's kit roots and stamps the version from the tag, writing nothing in-tree.
- **Knowledge-friction capture (any session):** re-deriving a fact no doc owns
  (off an implementation, a gate's source, a commit, or a prior/sibling
  deliverable)? stamp it in the moment with
  `bash drift-kit/bin/kfric.sh "<fact>" "<surface>"` — deferred capture is
  no capture; close triages it (drift-kit/SPEC.md §The knowledge-friction loop).
- **Gap capture (any mid-iteration session):** a work-shaped finding — a gap,
  a task, a defect — routes to the committed gap inbox with
  `bash lifecycle-kit/bin/file-gap.sh "<gap>"`, never a mid-iteration queue
  edit contending on a stage session's surface; close drains it
  (lifecycle-kit/SPEC.md §The committed gap inbox). The sanctioned exception:
  an operator-directed filing may land in the queue directly, staged and
  committed in one motion under the shared-index rule above.
- **Survey capture (any stage session):** bought a survey — a census, a cohort
  sweep, a roster over a corpus — that a later stage will want? land it before
  you act on it with
  `bash lifecycle-kit/bin/file-survey.sh "<question>" "<corpus>" "<oracle>" "<finding>"`;
  and before buying one, read the record and run its witness
  (lifecycle-kit/SPEC.md §The survey record).
- No per-user memory files: durable guidance goes in tracked manifests (this
  file, kit SPECs) or `BRIEF.local.md` (local-only private context). Harness
  auto-memory is disabled and enforced off (`check-settings-pins`,
  `check-memory-off`); doctrine: context-kit/SPEC.md §The memory-off doctrine.
