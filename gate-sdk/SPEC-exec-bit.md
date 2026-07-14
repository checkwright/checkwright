# SPEC amendment: check-exec-bit

## What changes

A new gate-sdk meta-gate, `checks/check-exec-bit.sh`, closing the
silent-degradation class the `check-plugin-exec-bit` queue task attests:
`drift-kit/kpis/kpi-overhead.sh` shipped `100644` and its KPI degraded to
`n/a (plugin failed)` in every drift report until a human noticed. The class
is wider than KPI plugins — gate-sdk's runner (`run-gates.sh`), drift-kit's
collator (`drift-report.sh`), and lifecycle-kit's entry preflight all invoke
kit scripts **by path**, and a shebang'd `bin/` tool is by-convention
path-invocable — so the gate generalizes past the filing's "KPIs only"
option. Live catches at authoring time: `drift-kit/bin/overhead-meter.sh`,
`guard-kit/bin/compare-settings-allow.sh`, `guard-kit/bin/run-guard-tests.sh`,
and `guard-kit/bin/scan-prompts.sh` are all tracked `100644` today; their
`chmod +x` lands in the same unit as the gate (enforcement-first).

**Invariant:** every tracked `*.sh` path matching an exec-glob carries mode
`100755` in the git index. The index, not the worktree, is asserted — the
mode that ships in a clone is the index mode, and a `Write`-tool-authored
script acquires `100644` there regardless of worktree state. Sibling, not
overlap: `check-hook-exec-bit` already asserts this same index-mode invariant
over the hooks dir — a disjoint target class (hook files are not `*.sh`, and
no default glob reaches the hooks dir), so both gates stand; the new SPEC
section cross-cites it.

**Knobs** (join gate-sdk/SPEC.md §Layout and configuration's roster,
`<KIT>_<KNOB>` shape):

- `GATE_SDK_EXEC_GLOBS` — array of path globs whose tracked `*.sh` members
  must be `100755`; default
  `('*/checks/*.sh' '*/kpis/*.sh' '*/bin/*.sh')` plus the computed
  consumer-dir entries `${GATE_SDK_GATES_DIR}/check-*.sh` and
  `${GATE_SDK_GATES_DIR}/kpi-*.sh` (consumer gates and KPI plugins resolve
  from the gates dir first, so they are by-path targets too).
- `GATE_SDK_EXEC_PRUNE` — path segments whose subtrees are exempt; default
  `(gate-tests fixtures templates smoke)`. Fixture trees deliberately carry
  glob-matching paths (e.g.
  `gate-sdk/gate-tests/check-readme-roster/*/alpha-kit/checks/*.sh`), and
  `templates/` members are copied content, sourced or bash-prefixed at their
  destination, never invoked in place. `lib/` needs no prune: sourced
  libraries match no default glob.

**Argument mode** (fixture capability): `check-exec-bit.sh [ls-files-dump]`
— with an argument, lint a canned `git ls-files -s` dump instead of running
`git ls-files -s` from the repo root, so a fixture is hermetic against the
host repo's index. Follows check-merge-attrs' precedent: an `expect.txt`
good/bad pair on the argument path plus a bespoke
`gate-tests/check-exec-bit.test.sh` that builds a temp git repo, adds a
`100644` KPI (red) and re-adds it `100755` (green), exercising the live
`git ls-files` path.

The gate copies `templates/check-skeleton.sh` and satisfies the four gate
contracts (gate-sdk/SPEC.md); tier `precommit`; registered in the consumer's
`gates.list`.

## Producers and consumers

- **Producer:** the gate run itself — the pre-commit hook and
  `run-gates.sh`, via the `gates.list` registration this unit adds (the
  enabling config, emitted where it must be: this repo's
  `scripts/gates.list`).
- **Consumer:** the committing session — a red run names each offending path
  and the `chmod +x` fix, per the output contract.
- **Knob readers:** both knobs are read only by `check-exec-bit.sh`; no
  other component consumes them.

## Existing sections updated

- gate-sdk/SPEC.md §Layout and configuration — the two knobs join the
  roster.
- gate-sdk/SPEC.md gains a §check-exec-bit per-gate section (invariant,
  calibration, honest limit — the honest limit: the gate reads the index, so
  a mode broken only in an uncommitted worktree file is invisible until
  staged).
- Regeneration ride-alongs on landing: pre-commit hook
  (`gen-pre-commit.sh --write`), `docs/check-graph.html`, enforcement map +
  value rollup (`tier=` registration changes both), docs mirror (SPEC
  edits).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
