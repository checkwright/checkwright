# SPEC amendment: deferred-drain

`scope-unit-set-ignores-deferred-inflow`: scope composes a unit set with no view of
how fast the Deferred section refills, so an iteration can close about as many
entries as its own close drain files, and the section never trends to empty. It was
measured at filing over the week to 2026-09-19: twenty iterations closed and
Deferred fell from 148 to 105, about two net per iteration. The operator's
direction, relayed by the lead: depletion is the ranking's purpose, since the
adopter preview waits on a near-empty Deferred section (TRAJECTORY.md, the
enhancement admission filter's grounds). The entry is admitted in full as
drain-serving machinery, and candidate (a)'s tool arm is allowed. That direction is
a reading of the filter, not a reversal of it.

The amendment spans drift-kit and lifecycle-kit, so it sits at the repo root.

**Ruling: candidates (a) and (b); (c) and (d) refused.**

- **(a) A net-drain figure on the Composition line, derived by a tool arm.** Taken.
  The figure compares the units the set takes out of the design-pending pool with
  the trailing mean of entries filed per iteration. A set at or below that mean
  argues why on the same line.
  **The arm is drift-kit's, not lifecycle-kit's**, because drift-kit already owns
  the definition. `kpi-queue-net-delta` defines *filed*: a slug now deferred that
  was in neither design-pending section at the baseline. It also defines *drained*:
  a slug that has left the pool entirely. It computes both, but only for the
  in-flight iteration. The arm runs that same computation over consecutive
  iteration-start commits, so there is one definition with two readers. A
  lifecycle-kit arm would be a third implementation of the pool parse, and
  lifecycle-kit depends on no other kit.
  **Slug sets, not filing dates.** The entry proposed reading the queue's filing
  dates. At HEAD those dates see only the entries still deferred. An entry filed and
  promoted inside the window is invisible to them, so a date count undercounts
  inflow by exactly the drain it is meant to be compared with. Diffing the pool's
  slug sets at each window's two ends counts every entry that entered.
- **(b) The third ranking tier fills beyond the lead unit's surface.** Taken, and
  gated on (a)'s figure. Where the same-surface fill leaves the set draining no more
  than the trailing inflow, the fill continues one whole surface at a time. The
  cost basis is the lead's own: dispatches are batched by surface
  (`lifecycle-kit/templates/lead.md` §Economics), so a second surface costs one
  more batch, where a second iteration costs another stage walk. Without (b), the
  figure would only report a drain rate that scope has no lever to change.
- **(c) A spec-led batch iteration per surface over design-pending entries.**
  Refused as new mechanism. This iteration is that shape, opened under an ordinary
  scope by operator direction. With (a) and (b), scope can compose it unaided.
- **(d) Measure the close drain's fix-inline versus promote mix.** Refused: no
  reader would act on it. The figure (a) already makes a bad mix visible, as
  inflow.

**Probed at authoring, 2026-09-19.** `grep -n -i "inflow\|net drain\|refill"
lifecycle-kit/templates/stages/scope.md lifecycle-kit/SPEC.md drift-kit/SPEC.md`
returns nothing. `grep -n "fn pool\|fn run" native/src/emit/kpi/queue_net_delta.rs`
returns the pool parse (line 50) and the in-flight run (line 91), which is the
parse delta 1 reuses. `grep -n "DRIFT_KIT_STATE_FILE" drift-kit/SPEC.md` returns the
state-file knob whose history the windows are read from.

## What changes

### (1) drift-kit's `queue-flow` emit arm {design-bearing}

**Not yet applied.** A new drift-kit/SPEC.md section, §The queue-flow arm, placed
after §The published-evidence extractor, owns:

- **Interface.** `bash gate-sdk/bin/run-gates.sh --emit queue-flow [<n>]`, where
  `<n>` is a positive integer, default `5`, the number of trailing windows.
- **Windows.** The iteration-start commits are the `<head>` of each boundary
  stamp, the first data line after a truncation, read from the committed history
  of `DRIFT_KIT_STATE_FILE`. The stage-economics join already reads that history.
  A window runs from one iteration-start commit to the next. Windows are keyed on
  those heads and never on an iteration's name, on §The report skeleton's ground
  that a name search lands on the sentinel or a renamed stamp. The row label is the
  name the iteration's last committed stamp carries. The in-flight iteration is
  never a window, because its end has not happened.
- **Rows.** One per window, oldest first: `<iteration> filed <f> drained <d>`, where
  *filed* and *drained* are `kpi-queue-net-delta`'s definitions applied to the
  queue file (`DRIFT_KIT_QUEUE_FILE`) at the window's two commits, with the same
  pool sections (`DRIFT_KIT_DEFERRED_SECTION`, `DRIFT_KIT_ICEBOX_SECTION`). The
  final row is `mean-filed <x>`, the mean of the rows' *filed* to one decimal.
- **Degrade.** Fewer than two iteration-start commits in reach, no state file, or a
  head this clone cannot resolve prints `n/a (<reason>)` and exits 0. This is
  drift-kit's fail-visible discipline, and the arm registers no gate.
- **Shared code.** The arm reuses the KPI's pool parse, so the two readers cannot
  disagree about what an entry is.

### (2) The Composition line carries the drain figure {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/stages/scope.md`, the paragraph
opening "**Record the test's verdict in the unit-set escalation**" changes its
second sentence to:

> The escalation's Recommendation carries one line in one of two forms:
> `Composition: bundled — <the deferred entries the set joins and the surface they
> share>; drain <k> against inflow <m>`, or `Composition: stands alone — <why this
> unit justifies an iteration's fixed cost>; drain <k> against inflow <m>`. Here
> `<k>` is the entries the set takes out of the design-pending pool and `<m>` is the
> `mean-filed` figure of `bash gate-sdk/bin/run-gates.sh --emit queue-flow`, never
> hand-counted. Where `<k>` is not above `<m>`, the line adds why.

### (3) The third tier fills past the lead unit's surface {mechanical}

**Not yet applied.** In the same template's paragraph opening "**Rank the pool by
what deferral costs and what landing buys**", the sentence "Third, the window fills
with rows whose `[surface:]` value matches the lead unit's, under the composition
test below." becomes:

> Third, the window fills with rows whose `[surface:]` matches the lead unit's.
> Where the set so far drains no more than the trailing inflow (the composition
> line's figure), the fill continues with the surface carrying the most board rows,
> one whole surface at a time. A further surface costs the lead one batch, not an
> iteration (lifecycle-kit/templates/lead.md §Economics).

### (4) §templates/stages/ records the ground {design-bearing}

**Not yet applied.** In `lifecycle-kit/SPEC.md` §templates/stages/, the paragraph
"The `scope` template's recurrence override **decides** the collision …" is
followed by a re-phrased ground for deltas 2-3:

- A unit set is weighed against the pool's refill rate, because a set draining no
  more than inflow leaves the pool unchanged however well it ranks.
- The figure is derived by drift-kit's arm from slug sets at iteration-start
  commits, not from filing dates, which miss entries filed and drained inside the
  window.
- The figure travels on the message channel. Like the rest of the Composition line
  it is read by the party ruling on the set and by no gate.
- The widened fill is gated on the figure, and its cost basis is the lead's
  surface batching.

### (5) The generated mirrors and the arm's roster readers {mechanical}

`docs/drift-kit/SPEC.md` and `docs/lifecycle-kit/SPEC.md` are regenerated after
deltas 1 and 4. The arm's listing in `drift-kit/README.md` and anything else that
rosters drift-kit's emit arms is found by
`git grep -l -- "--emit trajectory"`, the sibling arm.

## Producers and consumers

- **The `queue-flow` output** (delta 1). Producer: the arm, run by hand. Its inputs
  are the state file's committed history and the queue file at those commits, both
  present in every consumer running lifecycle-kit and drift-kit together.
  Consumer: the scope session composing its unit-set escalation (delta 2), which
  reads the `mean-filed` row. The per-window rows are read by the same session when
  it argues a set at or below inflow, which is why they are printed rather than
  only the mean.
- **The drain figure** (delta 2). Producer: scope. Consumers: the party ruling on
  the set, meaning the lead or the operator through the lead, and the widened fill
  of delta 3. The lead's presence check in `templates/lead.md` §Opening an
  iteration reads the Composition line's presence, not its content, and is
  unchanged.
- **A cross-kit citation, not a dependency.** scope.md already cites queue-kit's
  arms, and citing a drift-kit arm is the same shape. A consumer without drift-kit
  gets no figure. Delta 2's line then states the arm is absent, the degrade the arm
  itself prints when it cannot measure.
- **Point 6.** The corpus is the Composition line's two forms, and delta 2 gives
  both the figure.

## Existing sections updated

Roster probe: `git grep -n "Composition:" -- lifecycle-kit ':!*/SPEC-*.md'` and
`git grep -n "window fills" -- lifecycle-kit`.

- `drift-kit/SPEC.md` §The queue-flow arm, new, plus a pointer from the
  `kpi-queue-net-delta` bullet in §Bundled KPIs saying its definition has a second
  reader (delta 1)
- `native/src/emit/` (the arm) and `native/src/emit/kpi/queue_net_delta.rs` (the
  shared parse) (delta 1)
- `lifecycle-kit/templates/stages/scope.md` composition-verdict paragraph (delta 2)
- `lifecycle-kit/templates/stages/scope.md` ranking paragraph (delta 3)
- `lifecycle-kit/SPEC.md` §templates/stages/ (delta 4)
- `docs/drift-kit/SPEC.md`, `docs/lifecycle-kit/SPEC.md`, `drift-kit/README.md`
  (delta 5)

## Retired spellings

- None — the Composition line's forms gain a clause and keep their spelling.

## Definition of Done

- [ ] **Causal completeness** — every point holds for the arm's output, the drain
      figure and the widened fill.
- [ ] **`bash gate-sdk/bin/build-native.sh` plus the battery**, both green, and
      drift-kit's fixture suite covering the arm's rows and its degrade.
- [ ] **Instruction surfaces: instruction only** — scope.md carries the line and the
      fill, and the SPECs carry the grounds.
- [ ] **Merged with no information lost** — delta 4 re-phrases its paragraph.
- [ ] **Amendment deleted** — this file removed on merge.
- [ ] **Done move** — the paired entry moves to Done in the merge commit, before the
      drain stage.
- [ ] **Release declaration** — the new arm and the changed scope template are
      declared in the unit that lands them.
- [ ] **Gaps filed** — any cross-component gap found during the work filed.
