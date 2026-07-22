# CLAUDE.md — Checkwright

Checkwright packages a coding-agent-assisted delivery methodology as
installable kits. The private companion brief — seam boundary,
identity/namespace ownership, forward design memory behind the deferred-queue
rungs — is `BRIEF.local.md`, which is **local-only and untracked** (it carries
private context that must never be committed); consult it before roadmap/seam
work. The kit map lives in [README.md](README.md).

This repo is public: no local paths, private repo/project names, accounts, or
internal session/commit references in tracked files or commit messages.

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
and the touched kit's fixture suite:

```bash
bash gate-sdk/bin/run-gates.sh   # full battery
```

The per-kit fixture-runner battery — one `run-gate-tests.sh` line per kit, the
consumer-gate fixtures, the guard-kit decision table — lives in
[README.md](README.md) §This repo, governed; run the touched kit's suite before
committing.

The git index is shared with any concurrent session: check `git status` for a
foreign staged path before `git add`, or stage and commit in one motion.

A master push is verified against the remote oracle: watch the `gates`
workflow to green (`gh run watch`) before calling the push done.

Beyond the gate battery, the repo runs its own iteration lifecycle — one
iteration per hardening or roadmap unit, the queue and stage roster below. Each
stage session stamps `.workflow/WORKFLOW-STATE.txt` as its first step, and that
file's last stamp *is* the current stage — the cursor has one source, and stage
motion never writes the queue (`check-stage-evidence` / `check-stage-entry`
enforce the stamp protocol; `check-lifecycle-registration` holds the block
below in lockstep with the machine).

<!-- lifecycle-kit:begin -->
The repo runs lifecycle-kit's iteration state machine on `TASK-QUEUE.md` — one
stage session per stage, each invoking its skill:
`/scope` `/spec` `/align` `/build` `/validate` `/close`.
The state machine, its stamp protocol, and the per-stage contracts:
[lifecycle-kit/SPEC.md](lifecycle-kit/SPEC.md).
<!-- lifecycle-kit:end -->

The pre-commit hook is **generated** — never hand-edit
`scripts/git-hooks/pre-commit`; edit a gate's `# graph:` manifest and run
`bash gate-sdk/bin/gen-pre-commit.sh --write`, then regenerate the graph
artifact: `bash gate-sdk/checks/check-graph.sh --emit > docs/check-graph.html`
(`check-graph` asserts both artifacts are fresh). Per-clone opt-in:
`bash gate-sdk/bin/install-hooks.sh`.

The enforcement map is a generated projection likewise — after a class-registry
change (a gate's `tier=`, `kpis.list`, the settings hooks, a `# enforce:`
marker), regenerate it:
`bash gate-sdk/bin/enforcement-map.sh --emit > docs/enforcement.md`
(`check-enforcement-fresh` byte-compares it). The docs-site projections are a
separate roster — docs/site-architecture.md.

New gates copy `gate-sdk/templates/check-skeleton.sh` and ship with a
`good/`+`bad/` fixture pair; the four contracts (output, fail-closed,
fixture-pair, self-lint) are specified in [gate-sdk/SPEC.md](gate-sdk/SPEC.md)
and enforced by the meta-gates — a red gate is fixed, never bypassed with
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
- **Scope-gated intake** — a mid-session initiative is filed as a costed Deferred entry by default, never started; work enters only through scope.
<!-- doctrine-kit:end -->

## Conventions established in gate-sdk (keep every kit consistent)

One-line rule per convention; mechanism, knob rosters, and default values
live in the owning SPEC section — cited, never restated.

- **Registry, not array:** gates register by name in `gates.list` and resolve
  consumer-first with kit shadowing — resolution order and the kit-dirs knob:
  gate-sdk/SPEC.md §Layout and configuration.
- **Config via env:** every kit follows the same `<KIT>_<KNOB>` shape with
  this repo's layout as the defaults; each kit's SPEC owns its knob roster
  and default values.
- **Self-contained artifacts:** emitted HTML inlines its CSS; no kit output may
  reference an asset outside the kit — the one sanctioned exception and its
  honest limit: gate-sdk/SPEC.md §check-graph.
- **Kit-landing checklist:** README + SPEC.md, fixtures for every shipped
  gate, `smoke/`, and registration in this repo's `gates.list` where
  applicable — gate-sdk/SPEC.md §Consumer smoke owns the checklist.

## Agent execution (all stages)

Delegation is pre-authorized for read-heavy audits and mechanical rename/merge
sweeps (the session-context nudge) — this repo's judgment on what dispatch needs
no ask. **Full protocol: `/agent-execution`.** The safety rules, resume-journal
mechanics, validate-after-commit set, and gate-driven worklist load behind that
trigger — a delegated `Agent` dispatch, whose per-dispatch budget guard names
the skill on a blocking verdict — so they are not resident here.

## Housekeeping

- `.tmp/` is gitignored, purely disposable scratch (gate timings, resume
  journals), wiped at the scope boundary; `.metric/` is gitignored persistent
  measurement trends — **never committed**,
  account-bearing (drift-kit/SPEC.md §Layout and configuration); `.workflow/`
  holds two tiers — tracked checked projections beside gitignored local capture
  (gate-sdk/SPEC.md §The workflow directory). `BRIEF.local.md` (private brief),
  `OPS.local.md` (private ops runbook — DNS records + GitHub repo-settings
  desired state; consult it for domain or repo-settings work), and `ENV.local.md`
  (context-kit's probed machine profile plus hand-authored gotchas — seed with
  `bash context-kit/bin/env-probe.sh`, context-kit/SPEC.md §bin/env-probe) are
  gitignored, local-only.
- `reserve/` holds the crates.io/npm name-reservation placeholders — do not
  develop in it.
- `CONTRIBUTING.md` + the `.github/` issue/PR templates and `RELEASING.md` are
  governed repo-meta (tracked, core-files-pinned, in the spec manifest so their
  links/commands resolve like any doc); the fixture is the unit of contribution —
  edit the guide, not GitHub UI settings. The release runbook is the
  load-triggered tier, resident only at close's release step.
- `docs/` is the public GitHub-Pages site (served from `docs/` on master via its
  `CNAME`), repo-root-governed, no owning kit. Its chrome, page-authoring rules,
  generated projections + regen commands, and docs gate roster live in the
  load-triggered [docs/site-architecture.md](docs/site-architecture.md).
- `demo/run-demo.sh` is the runnable adoption walkthrough (vendor → clean pass →
  violation blocked → fix → green), the evidence-kit `demo` validate suite on the
  gate-sdk consumer-smoke mechanics (`DEMO_TMP_DIR` its only knob, writing nothing
  in-tree), so a bit-rotted walkthrough is a red validate.
- **Knowledge-friction capture (any session):** re-deriving a fact no doc owns
  (off an implementation, a gate's source, a commit, or a prior/sibling
  deliverable)? stamp it in the moment with
  `bash drift-kit/bin/kfric.sh "<fact>" "<surface>"` — deferred capture is
  no capture; close triages it (drift-kit/SPEC.md §The knowledge-friction loop).
- No per-user memory files: durable guidance goes in tracked manifests (this
  file, kit SPECs) or `BRIEF.local.md` (local-only private context). Harness
  auto-memory is disabled and enforced off (`check-settings-pins`,
  `check-memory-off`); doctrine: context-kit/SPEC.md §The memory-off doctrine.
