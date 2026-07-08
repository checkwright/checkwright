# SPEC amendment: trajectory

## What changes

`bin/trajectory.sh` — the published-evidence extractor behind the docs
evidence page. Route ruling (scope 2026-07-09): the benefits claim is
**self-referential** — publish this repo's own governed trajectory and
state plainly that no controlled ungoverned baseline exists; the synthetic
two-arm A/B experiment is a separate deferred rung
(`benchmark-ab-experiment`), not this mechanism.

The extractor is a pure function of committed git history — no now-relative
fields (no ages-in-days-from-today), so re-emission on an unchanged history
is byte-identical. It harvests, per iteration:

- **iteration + stages run** — the stamp lines from
  `WORKFLOW-STATE.txt`'s git history (the file truncates at each scope
  boundary; history keeps every stamp).
- **validate attestations** — the evidence-manifest lines
  (`validate-evidence.txt` history): per-suite verdict, pass/fail counts,
  manifest hash. This series is the primary satisfiable-drift surface:
  the whole battery re-verified at every validate means a commitment made
  N iterations earlier that silently broke surfaces as a red suite, not a
  consistent-looking pass.
- **amendment latency** — per amendment file, git add-date → delete-date
  (merge); the commitment-to-merge lag series.
- **commit shape** — feat/debt split of the iteration's commit subjects
  (the kpi-task-split classification, applied over history).
- **gate roster growth** — `gates.list` line count at each iteration's
  close commit; with the queue's `check-*` mentions this bounds the
  named-but-unbuilt commitment backlog.

Excluded, stated as a limitation on the page: knowledge-friction counts
(the log is gitignored scratch — not committed history; the KPI stays a
session-local lower bound and the page says so).

Interface: `trajectory.sh --emit` writes a markdown table (one row per
iteration, stable column set) to stdout; plain invocation adds a
human-oriented header. Config: `DRIFT_KIT_TRAJECTORY_SURFACES` overriding
the default state-file paths (`<workflow-dir>/WORKFLOW-STATE.txt`,
`<workflow-dir>/validate-evidence.txt`), platform layout as default.
Degrades per surface to an `n/a (<reason>)` column, exit 0 — drift-kit's
fail-visible discipline; drift-kit stays a no-gates advisory kit.

Consumer wiring (this repo, not kit mechanism): the emitted table is
committed at `docs/evidence-data.md`; a thin consumer gate
`scripts/check-trajectory-fresh.sh` (registered in `gates.list`)
re-emits and byte-compares — the gen-pre-commit/check-graph freshness
pattern — so hand-edited or stale numbers are red at commit.
`docs/evidence.md` (slot owned by SPEC-docs-site.md) carries the framing
and cites the data file; it hand-copies no numbers. Framing content the
page must carry, fixed at scope:

- the honesty statement: governed-arm evidence only, baseline absent by
  design, A/B deferred;
- the failure-mode → mechanism table: Layering Effect →
  `check-comment-tier` + spec-kit's anti-restatement doctrine; premature
  lock-in → the lifecycle state machine (`check-stage-entry`/`-evidence`);
  satisfiable drift → whole-battery re-run each commit + spec fences +
  validate-after-commit;
- primary citations only: seqBench (arXiv 2509.16866) for dependent-task
  series vocabulary; Drift-Bench (arXiv 2602.02455 — real title
  "Diagnosing Cooperative Breakdowns in LLM Agents under Input Faults via
  Multi-Turn Interaction"; a "Decomposing Reasoning Into Failure Types"
  expansion circulates but is confabulated — do not cite it) for the
  satisfiable-drift framing; Lost-in-Conversation / FlowBench as
  supporting prior art.

Known input defect, filed as debt alongside this amendment
(`kpi-amendment-age-fixture-noise`): the amendment-age KPI's
`git ls-files | grep 'SPEC-*.md'` glob counts gate-test fixture amendments
(today: `SPEC-example-gate.md`, 4d); trajectory.sh must exclude fixture
and template paths from its amendment-latency harvest from day one, and
the KPI converges to the same exclusion.

## Producers and consumers

- Producer: any session runs `trajectory.sh --emit`; the freshness gate
  runs it at every pre-commit and in `run-gates.sh`.
- Consumers: `docs/evidence-data.md` (the committed emission),
  `check-trajectory-fresh` (byte-compare), adopters reading the page.
- Inputs read: `git log`/`git show` over the two configured state files,
  `gates.list` history, the queue file's history, commit subjects. Writes
  nothing; the redirect is the consumer's.
- `DRIFT_KIT_TRAJECTORY_SURFACES` reader: `trajectory.sh` at startup.

## Existing sections updated

- drift-kit SPEC §Layout and configuration gains `bin/trajectory.sh` and
  the knob; §Testing gains its smoke coverage (advisory-tool pattern —
  run over this live repo, assert the table parses and rows ≥ 1).
- drift-kit SPEC §Bundled KPIs: kpi-amendment-age's contract gains the
  fixture/template path exclusion (the debt task lands it).
- drift-kit README: one line for the extractor.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
