# CLAUDE.md — Checkwright

Checkwright packages a coding-agent-assisted delivery methodology as
installable kits, extracted from the governance meta-layer of a private
production platform (the first dogfooding consumer). The private companion
brief — seam boundary, identity/namespace ownership, forward design memory
behind the deferred-queue rungs — is `BRIEF.local.md`, which is **local-only
and untracked** (it carries private context that must never be committed);
consult it before roadmap/seam work. Kit status lives in [README.md](README.md).

This repo is public: no local paths, private repo/project names, accounts, or
internal session/commit references in tracked files or commit messages.

## The extraction seam (never cross it)

Copy-first, not carve-out: the source platform's repo stays untouched; generic
mechanism is copied here and de-hardcoded. **Rule content never leaves the
platform** — term lists, coupling vocabularies, glossary bodies, wire-contract
couplings, product constant sets. When a kit component needs such content, it
becomes optional consumer config (the `check-graph` / `scripts/graph-vocab.sh`
pattern), never a kit literal. Migrating the platform onto Checkwright is NOT
part of extraction; that is a later task in the platform's own queue.

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
bash guard-kit/bin/run-guard-tests.sh                                                # guard-kit decision table
```

The repo also runs lifecycle-kit's iteration state machine on itself — one
iteration per kit through extraction, and per hardening or roadmap unit
thereafter. `TASK-QUEUE.md` carries the
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

## Conventions established in gate-sdk (keep later kits consistent)

- **Registry, not array:** `gates.list` (one name per line, `#` comments)
  replaces the platform's `CHECKS=()`; names resolve against the consumer
  gates dir first, then each vendored kit's `checks/` (`gate_kit_roots`, env
  `GATE_SDK_KIT_DIRS`) — a consumer shadows a kit gate by dropping a
  same-named file in its gates dir, and a new kit's gates register by name
  alone.
- **Config via env, platform values as defaults:** `GATE_SDK_GATES_DIR`
  (`scripts`), `GATE_SDK_TESTS_DIR`, `GATE_SDK_HOOKS_DIR`
  (`scripts/git-hooks`), `GATE_SDK_WORKFLOW_DIR` (`.workflow`),
  `GATE_SDK_TMP_DIR` (`.tmp`), `GATE_SDK_QUEUE_FILE` (`TASK-QUEUE.md`),
  `GATE_SDK_PRUNE_DIRS`. Later kits follow the same `<KIT>_<KNOB>` shape and
  keep the platform's layout as the default.
- **Self-contained artifacts:** emitted HTML inlines its CSS (the platform's
  `diagram-assets/` did not leave); no kit output may reference a
  platform-only asset.
- **Each kit lands with its own README + SPEC.md, fixtures for every gate
  it ships, + a `smoke/` directory** (`smoke/install.sh`, plus
  `smoke/violation.sh` where a violation is craftable — gate-sdk/SPEC.md
  §Consumer smoke), and registers in this repo's `gates.list` where applicable.

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
  brief).
- `reserve/` holds the crates.io/npm name-reservation placeholders — do not
  develop in it.
- **Knowledge-friction capture (any session):** catch yourself re-deriving a
  fact no doc owns (off an implementation, a gate's source, a commit)? append
  `<date> <fact> ← <surface>` to `.workflow/knowledge-friction.log` (gitignored
  scratch) at that moment — deferred capture is no capture; close triages it
  into doc-owner edits (drift-kit/SPEC.md §The knowledge-friction loop).
- No per-user memory files for this repo: durable guidance goes in tracked
  manifests (this file, kit SPECs), or `BRIEF.local.md` for local-only private
  context.
