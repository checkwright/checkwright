# CLAUDE.md — Checkwright

Checkwright packages a coding-agent-assisted delivery methodology as
installable kits, extracted from the governance meta-layer of a private
production platform (the first dogfooding consumer). The extraction brief —
kit order, seam ruling, de-hardcoding worklist — is `EXTRACTION.md`, which is
**local-only and untracked** (it carries private context that must never be
committed); read it before starting any kit work. Kit status lives in
[README.md](README.md).

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
```

The repo also runs lifecycle-kit's iteration state machine on itself — one
iteration per kit extraction. `TASK-QUEUE.md` carries the
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
- **Each kit lands with fixtures + its own README + SPEC.md**, and gets
  registered in this repo's `gates.list` where applicable (the kits govern
  this repo too).

## Housekeeping

- `.tmp/` is gitignored (measurements, e.g. gate timings); `.workflow/` is
  committed (checked projections); `EXTRACTION.md` is gitignored (private
  brief).
- `reserve/` holds the crates.io/npm name-reservation placeholders — do not
  develop in it.
- No per-user memory files for this repo: durable guidance goes in tracked
  manifests (this file, EXTRACTION.md, kit SPECs).
