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
dogfooding is day-one, not optional. Before committing:

```bash
bash gate-sdk/bin/run-gates.sh                                                      # full battery
bash gate-sdk/bin/run-gate-tests.sh gate-sdk/gate-tests gate-sdk/checks             # gate-sdk fixtures
bash gate-sdk/bin/run-gate-tests.sh lifecycle-kit/gate-tests lifecycle-kit/checks   # lifecycle-kit fixtures
bash gate-sdk/bin/run-gate-tests.sh queue-kit/gate-tests queue-kit/checks           # queue-kit fixtures
bash gate-sdk/bin/run-gate-tests.sh spec-kit/gate-tests spec-kit/checks             # spec-kit fixtures
bash gate-sdk/bin/run-gate-tests.sh delegation-kit/gate-tests delegation-kit/checks # delegation-kit fixtures
bash gate-sdk/bin/run-gate-tests.sh context-kit/gate-tests context-kit/checks       # context-kit fixtures
bash gate-sdk/bin/run-gate-tests.sh evidence-kit/gate-tests evidence-kit/checks     # evidence-kit fixtures
bash gate-sdk/bin/run-gate-tests.sh site-kit/gate-tests site-kit/checks             # site-kit fixtures
bash gate-sdk/bin/run-gate-tests.sh scripts/gate-tests                              # consumer-gate fixtures
bash guard-kit/bin/run-guard-tests.sh                                                # guard-kit decision table
```

The git index is shared with any concurrent session: check `git status` for a
foreign staged path before `git add`, or stage and commit in one motion.

The repo also runs lifecycle-kit's iteration state machine on itself — one
iteration per hardening or roadmap unit. `TASK-QUEUE.md` carries the
`## Iteration: <name>  [stage: <stage>]` header; each stage session invokes
the matching `.claude/commands/<stage>.md` skill, which stamps
`.workflow/WORKFLOW-STATE.txt` and flips the header as its first step
(`check-stage-evidence` / `check-stage-entry` enforce the flip+stamp
protocol — see lifecycle-kit/SPEC.md).

The pre-commit hook is **generated** — never hand-edit
`scripts/git-hooks/pre-commit`; edit a gate's `# graph:` manifest and run
`bash gate-sdk/bin/gen-pre-commit.sh --write`, then regenerate the graph
artifact: `bash gate-sdk/checks/check-graph.sh --emit > .workflow/CHECK-GRAPH.html`
(`check-graph` asserts both artifacts are fresh). Per-clone opt-in:
`bash gate-sdk/bin/install-hooks.sh`.

The enforcement map is a generated projection likewise — after a class-registry
change (a gate's `tier=`, `kpis.list`, the settings hooks, a `# enforce:`
marker), regenerate it:
`bash gate-sdk/bin/enforcement-map.sh --emit > docs/enforcement.md`
(`check-enforcement-fresh` byte-compares it).

New gates copy `gate-sdk/templates/check-skeleton.sh` and ship with a
`good/`+`bad/` fixture pair; the four contracts (output, fail-closed,
fixture-pair, self-lint) are specified in [gate-sdk/SPEC.md](gate-sdk/SPEC.md)
and enforced by the meta-gates — a red gate is fixed, never bypassed with
`--no-verify` except as a one-off with cause.

Comments are directives, else deleted — a passing `check-comment-tier` is the
floor, not licence to keep a comment. Blessing a restatement (relocating prose
behind a `spec:` or `comment-tier-exempt:` tag rather than deleting it) is
itself the defect; the doctrine and the one-line-binding rule for `spec:` live
in [spec-kit/SPEC.md](spec-kit/SPEC.md) §check-comment-tier.

## Single source of truth

- **Content-tiering:** every governed prose surface owns exactly one content
  tier and *points to* — never restates — a fact another surface owns; a
  parallel copy is the defect, and the fix is one shape: replace the slab
  with a pointer to the owner, then a gate forbids it growing back.
  Quantitative literals (knob defaults, shared constants, derivable rosters)
  are owned by code or the owning SPEC — prose cites the name, never the
  value.
- **Enforcement-first:** on any fix or redundancy finding, name the defect
  class *and* its enforcing mechanism and land both in the same unit — never
  offer the gate as a follow-up; a green instance fix is the stop signal to
  ask what check should have caught it. When the gate cannot land in this
  unit, the instance fix rides the gate's unit rather than landing bare.

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
  reference an asset outside the kit.
- **Kit-landing checklist:** README + SPEC.md, fixtures for every shipped
  gate, `smoke/`, and registration in this repo's `gates.list` where
  applicable — gate-sdk/SPEC.md §Consumer smoke owns the checklist.

## Agent execution (all stages)

Delegation is pre-authorized for read-heavy audits and mechanical rename/merge
sweeps (the session-context nudge). Resident safety rules for every delegated
`Agent`; **full protocol: `/agent-execution`** (resume-journal mechanics,
validate-after-commit set, gate-driven worklist).

- **Supervisor owns SECURITY/design rulings; agents surface, never guess.**
- **Background + notification, never poll** (`run_in_background`; don't read the
  output file).
- **Serialize on shared files** — the generated `scripts/git-hooks/pre-commit` +
  `.workflow/CHECK-GRAPH.html`, `TASK-QUEUE.md`/`.workflow/WORKFLOW-STATE.txt`,
  the `scripts/*-config.sh` + `gates.list`, an amendment under edit — **and the
  git index/HEAD are shared for every committing agent** (serialize *or*
  `isolation: worktree`); **≤2-wide otherwise**, read-only fan-outs only.
- **One commit per unit; split** if >4 components, OR mixed
  mechanical+architectural, OR >300 tool calls.
- **Resume journal in the harness session dir** — never `.tmp/` (the
  session-context hook sweeps it) and never a system temp dir (a restart wipes
  it); grant the path explicitly before dispatch. Agent writes findings inline +
  a `DONE` marker; supervisor deletes it post-commit. A background agent's
  sandbox may block the write, so for a **read-only fan-out the return value is
  the contract** — reserve the journal for file-mutating agents.
- **Validate after every agent commit** — a sub-agent's "passed" is not
  trustworthy: re-run `bash gate-sdk/bin/run-gates.sh` plus the touched kit's
  fixture runner, and **diff every gate change** (an agent blocked by a gate
  weakens it rather than fix the code; `check-gate-tamper` is only the
  mechanical floor).
- **Budget-check before *each* dispatch** (`bash delegation-kit/bin/usage-verdict.sh`
  — one verdict, exit 0 OK / 1 PAUSE / 2 STALE; never eyeball the raw pct;
  enforced per-dispatch by the Agent budget guard).
- **Never revert substantial completed delegated work on your own design
  judgment** — surface the tension and wait for explicit go-ahead.

## Housekeeping

- `.tmp/` is gitignored (measurements, e.g. gate timings); `.workflow/` is
  committed (checked projections); `BRIEF.local.md` is gitignored (private
  brief); `OPS.local.md` is gitignored (private ops runbook — DNS records
  and the GitHub repo-settings desired state; consult it for domain or
  repo-settings work).
- `reserve/` holds the crates.io/npm name-reservation placeholders — do not
  develop in it.
- `CONTRIBUTING.md` + the `.github/` issue/PR templates are governed repo-meta
  (tracked, core-files-pinned); `CONTRIBUTING.md` joins the spec manifest so its
  links/commands resolve like any doc. The fixture is the unit of contribution —
  edit the guide, not GitHub UI settings, to change what arrives.
- `docs/` is the public GitHub-Pages site (served from `docs/` on master via its
  `CNAME`), repo-root-governed with no owning kit. Living pages (everything
  outside `docs/posts/`) are fully governed prose; dated posts under
  `docs/posts/` are immutable published artifacts — temporal-exempt but still
  link/command-resolved (`scripts/spec-config.sh`). A page owns orientation and
  sequencing and cites downward: the anti-restatement doctrine applies to docs
  as to comments — never restate an invariant a SPEC or README owns.
  `check-docs-kit-parity` holds every kit's row in `docs/index.md`.
- `demo/run-demo.sh` is the runnable adoption walkthrough — it builds a scratch
  consumer on the gate-sdk consumer-smoke mechanics and narrates the arc
  (vendor → clean pass → violation blocked → fix → green) on stdout, exiting 0
  only when every act behaved (the violation act *was* blocked). It writes
  nothing in-tree and `DEMO_TMP_DIR` is its only knob. Registered as the
  evidence-kit `demo` validate suite, so a bit-rotted walkthrough is a red
  validate, not a stale docs page.
- **Knowledge-friction capture (any session):** catch yourself re-deriving a
  fact no doc owns (off an implementation, a gate's source, a commit)? append
  `<date> <fact> ← <surface>` to `.workflow/knowledge-friction.log` (gitignored
  scratch) at that moment — deferred capture is no capture; close triages it
  into doc-owner edits (drift-kit/SPEC.md §The knowledge-friction loop).
- No per-user memory files for this repo: durable guidance goes in tracked
  manifests (this file, kit SPECs), or `BRIEF.local.md` for local-only private
  context. Harness auto-memory is disabled and enforced off — the settings
  pins hold at commit (`check-settings-pins`), the memory dir and any local
  override are scanned on the operator's machine (`check-memory-off`); the
  doctrine is context-kit/SPEC.md §The memory-off doctrine.
