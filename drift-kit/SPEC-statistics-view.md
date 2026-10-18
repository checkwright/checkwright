# SPEC amendment: statistics-view

Pairs `iteration-statistics-view`. A consumer can read its workflow's per-iteration figures in four places today: the trajectory table (§The published-evidence extractor), the queue-flow rows (§The queue-flow arm), the stage-economics trend log (§The stage-economics meter) and the `/economics` narrative. All four are tables or prose, and a trend shows only to a session that recomputes it from them. This amendment adds one emit arm that charts those figures over the closed iterations, as a self-contained HTML document.

**The view stays local, and the home follows from its inputs.** Two of its figures come from the stage-economics log. That log lives under `DRIFT_KIT_METRIC_DIR`, which must be gitignored and never committed (§Layout and configuration). The meter already refuses a freshness gate over the log, because CI and a fresh clone hold none (§The stage-economics meter, *Refused*). So the view is written into the metric dir by default, beside the log it reads. It is not a committed projection, and the kit ships no freshness gate for it: a regenerated, uncommitted file has no committed copy to drift from. The entry scopes the unit as kit-side, single-repo and local, and a local view meets all three.

**Refused: charting only tracked inputs so that the view could be committed.** It would drop the spend and cache-read figures, the two that the stage-economics meter exists to make visible close-over-close. A consumer who wants a published page of the tracked figures already has the trajectory table, committed behind its own freshness gate.

**Refused: committing a projection of the log on the install-evidence pattern**, an aggregate page whose gate is inert off the observing machine. On a public tree that publishes the consumer's spend, which no direction asks for. The kit leaves that choice to the consumer: the view's path is a knob, and the consumer decides where it points.

## What changes

### (1) `--emit statistics` writes the statistics view {design-bearing} {user-facing: iteration-statistics-view's Deliverable, "which figures chart, the projection's form and home, and its freshness gate", left to this stage, within the entry's "kit-side, single-repo and local"}

A new section, `## The statistics view`, after §The queue-flow arm in `drift-kit/SPEC.md`. **Not yet applied.**

> ## The statistics view
>
> The `statistics` emit arm (gate-sdk/SPEC.md §The non-gate arm) charts the per-iteration figures the other arms report as tables, so a trend in workflow health is seen without recomputing it.
>
> - **Interface.** `bash gate-sdk/bin/run-gates.sh --emit statistics [<n>]`, where `<n>` is a positive integer, the number of trailing closed iterations charted. The default is every closed iteration. Anything else is a usage error, exit 2. The arm writes one document to `DRIFT_KIT_STATISTICS_FILE`, replacing the file whole and creating its dirname. It prints `statistics: wrote <path> — <k> closed iteration(s)`, then one line per figure, `  <figure>: charted` or `  <figure>: n/a (<reason>)`, then the caveat counts below where they are non-zero. Exit is 0 on every other path. The arm registers no gate.
> - **The iterations.** The x-axis is the closed iterations in the order the trajectory extractor emits them, one per `close`-stamped range (§The published-evidence extractor). The in-flight iteration is never charted, so a figure does not move between two closes for want of a finished row.
> - **The figures**, each one panel:
>   1. *spend*: the `cost` of the iteration's trend-log rows, summed, in three stacked bands: the stage rows, the supervision row, and the fan-out rows, told apart by `DRIFT_KIT_SUPERVISION_LABEL` and `DRIFT_KIT_FANOUT_SUFFIX`. A row whose `cost` is `n/a` adds nothing and marks its iteration's bar as incomplete.
>   2. *cache-read*: the `cr` tokens of the same rows, summed, as one bar per iteration. It is the meter's headline field, kept beside spend so the lever stays visible.
>   3. *commit shape*: the feature and debt commit counts, stacked (the extractor's commit-shape harvest).
>   4. *amendments*: the merged-amendment count as bars, and the longest lag as a line on its own axis, both from the extractor's amendment-latency harvest.
>   5. *validate*: one cell per iteration reading clean, non-clean or no attestation (the extractor's validate harvest).
>   6. *gate roster*: the `gates.list` member count at each close, as a line.
>   7. *queue flow*: the filed and drained counts per window (§The queue-flow arm), joined to the x-axis on the window's row label.
> - **The inputs.** Figures 3 to 7 are read from committed history alone, so they render in any clone. Figures 1 and 2 read `DRIFT_KIT_STAGE_ECONOMICS_LOG`, which only the measuring machine holds. One harvest feeds both renderers: the arm calls the trajectory extractor's harvest, the queue-flow arm's window computation and the meter's log-line parse in process. It never parses an arm's printed table, so the view and the tables cannot disagree about a figure.
> - **Degrade.** A figure whose input is missing renders its panel with the reason in place of the marks, and the stdout line says the same: no log, no committed state file, fewer than two iteration starts. The figures that have their input still render. The two limitation lines the trajectory emission opens with head the document too, since its figures carry the same two limits.
> - **Caveat counts.** On stdout only: the trend-log iterations naming no closed iteration (the in-flight one, a placeholder name), the iterations with incomplete pricing, and the queue-flow windows whose label names no closed iteration.
> - **The document** is one HTML file with its CSS inline and its charts inline SVG. It carries no script, and it references nothing outside itself (the self-contained-artifact rule, gate-sdk/SPEC.md §check-graph). Each mark carries an SVG `<title>` naming its iteration and value, so hovering reads the number without a script. Each panel's caption names its source. Colours follow `prefers-color-scheme` for light and dark.
> - **Determinism.** The document is a function of closed history, the log and `<n>`. It reads no now-relative field and carries no generation date, so two emissions over unchanged inputs are byte-identical.
> - **Privacy.** The document carries no account identifier and no session id, since neither input carries one. It does carry the consumer's spend, so its default home is the metric dir, and the never-committed rule there covers it. A consumer who points `DRIFT_KIT_STATISTICS_FILE` at a tracked path publishes that spend; the kit ships no gate for a committed copy.
> - **Named caller.** The `/economics` skill renders the view as its last tool step and names the path in its report (§The `/economics` skill). Its reader is the operator who opens the file. A session may run the arm ad hoc.

`native/src/emit/statistics.rs` implements the arm, with its `#[cfg(test)]` module exercising the operand grammar, the iteration trimming, the band split, the incomplete-pricing mark, each degrade and the byte-identity of two renders (gate-sdk/SPEC.md §The non-gate arm). The arm table in `native/src/emit/mod.rs` gains the `--emit-statistics` row as an `Arm::Emit`, since the arm's whole exit grammar is the 0/2 collapse.

### (2) One harvest, two renderers {design-bearing}

`native/src/emit/trajectory.rs` exposes its harvest: a public function returning the closed iterations in emission order with each one's stages, commit split, amendment count and lag, validate verdict and gate count. Its `emit` renders the markdown table from that function, byte-identical to today's output. `native/src/emit/queue_flow.rs` exposes its window rows the same way. The meter's trend-log line parse in `native/src/emit/stage_economics.rs` is the one the arm calls. Where the meter keeps that parse private, the build exposes it rather than writing a second parse. **Not yet applied.**

The committed trajectory projection is the check on this delta. `check-trajectory-fresh` byte-compares `docs/evidence-data.md` against a re-emission, so a refactor that moved one byte of the table reds it at the commit (docs/generated-projections.md, the trajectory projection).

### (3) The knob and the metric dir's members {mechanical} {user-facing: as delta 1}

Replacements in `drift-kit/SPEC.md` §Layout and configuration. **Not yet applied.**

- A new knob bullet, after `DRIFT_KIT_STAGE_ECONOMICS_LOG`:

  > - `DRIFT_KIT_STATISTICS_FILE` — the statistics view's document (§The statistics view); default `.metric/statistics.html`, derived as `${DRIFT_KIT_METRIC_DIR}/statistics.html`. The arm is its one writer, and the operator opening it its reader.

- The paragraph opening **A sibling kit's fact keeps drift-kit's own knob**, the sentence beginning "**A metric-dir log**", becomes:

  > **A metric-dir member** (`DRIFT_KIT_OVERHEAD_LOG`, `DRIFT_KIT_STAGE_ECONOMICS_LOG`, `DRIFT_KIT_MANUAL_OPS_LOG`, the two install records and `DRIFT_KIT_STATISTICS_FILE`) defaults under `DRIFT_KIT_METRIC_DIR`, so a set metric dir moves it.

- `DRIFT_KIT_METRIC_DIR`'s bullet, "its members are append-only trend logs that survive scratch wipes", becomes "its members are append-only trend logs that survive scratch wipes, and the view rendered from them".

The knob table in `native/src/knobs/drift_kit.rs` gains the row, a scalar whose default derives from `DRIFT_KIT_METRIC_DIR` as the log rows' defaults do. `--emit knob-roster` then prints it, and `check-knob-default-coupling` holds the table's rendered default to the bullet's.

### (4) `/economics` renders the view {mechanical} {user-facing: as delta 1}

Replacements. **Not yet applied.**

- `drift-kit/SPEC.md` §The `/economics` skill, first sentence:

  > `/economics` is the customer-facing post-iteration narrative. Run at close, it chains `--emit overhead-meter` → `--emit stage-economics` → `--emit manual-ops` into one report answering "what did this iteration cost, where, and was the model posture worth it", then renders the statistics view (§The statistics view) and names its path.

- `drift-kit/templates/economics.md`, after step 3, a fourth step:

  > 4. **Statistics view** — the same binary's `--emit statistics` (drift-kit/SPEC.md §The statistics view): the per-iteration figures charted into one local HTML file. Run it after step 2, so the spend panel holds this iteration's rows.

  and after the **Posture verdict** bullet:

  > - **The view.** Name the file the statistics view wrote, and any panel it printed as `n/a`, so the reader knows where the trend is and what it lacks.

### (5) The smoke drives the arm {design-bearing}

Replacement for `drift-kit/SPEC.md` §Testing, a new bullet after **The queue-flow arm**. **Not yet applied.**

> - **The statistics view** runs against the trajectory extractor's fake-history repo, with a purpose-built trend log under a throwaway `DRIFT_KIT_METRIC_DIR`. The log holds a stage row, a supervision row, a fan-out row, an `n/a`-cost row and a row for the in-flight iteration. The smoke asserts that the document is written at `DRIFT_KIT_STATISTICS_FILE` and carries all seven panels. It asserts that the closed iteration is charted and the in-flight one is not, and that the incomplete-pricing count reads 1. It asserts that the document holds no `<script`, no `src=`, no `url(` and no `href` leaving the document, and that two emissions are byte-identical. A log-absent run charts figures 3 to 7 and prints figures 1 and 2 as `n/a (no stage-economics log)`. `<n>` trims to the trailing iterations, and `0` and a non-integer exit 2.

`drift-kit/smoke/install.sh` carries the leg.

### (6) The README names the arm {mechanical} {user-facing: as delta 1}

`drift-kit/README.md`: the command block gains `"$gates" --emit statistics [<n>]  # the per-iteration figures charted into one local HTML view`, and the paragraphs after it gain one for the arm, pointing at §The statistics view and saying the file stays under the metric dir. **Not yet applied.**

### (7) This repo's close renders the view after the feed {mechanical}

`.claude/commands/close.md`, the **housekeeping** binding, the sentence that feeds the stage-economics log: after `run `--emit stage-economics``, add "then render the statistics view with `--emit statistics`, which writes it under the metric dir". The binding's `econ 0` confirmation is unchanged. **Not yet applied.**

The view is read where the cost is judged, and the feed is the last point at which every one of the iteration's stamps exists. Housekeeping runs after close's entry stamp is committed, so a render there already charts the closing iteration.

## Producers and consumers

- **The arm and its document (delta 1).**
  - Producer: `--emit statistics`, run by the `/economics` skill's fourth step (delta 4), and ad hoc. This repo's close binding does not run `/economics`; its housekeeping feeds the trend log with `--emit stage-economics` directly (`.claude/commands/close.md`, probe `grep -n economics .claude/commands/close.md`). So delta 7 has that binding render the view right after the feed, and a deployed configuration reaches the producer at every close.
  - Consumer: the operator opening the HTML file, at close or later. The stdout lines are read by the `/economics` narrative (delta 4).
  - Roster-holding readers of the new arm name: the arm table, which is the roster `--emit no_such_arm` and the bare binary print (gate-sdk/SPEC.md §The non-gate arm), so the row is the registration. `check-crate-arms`' test arm resolves each row's function to its file and reds a file with no test module, which delta 1's module satisfies. The README command block (delta 6) is prose and no gate rosters it; probe: `git grep -n "emit queue-flow" -- '*.md' ':!docs/posts'`, which names drift-kit/README.md, its mirror, drift-kit/SPEC.md and lifecycle-kit's scope template and SPEC. The last two cite the queue-flow arm by name for scope's ranking and keep no roster of arms.
  - Fields: every figure has its reader in the operator. The stdout per-figure lines and caveat counts are read by the narrative's view bullet (delta 4). The document carries no field beyond its panels.
- **The harvest exposure (delta 2).** Producer: the trajectory and queue-flow modules. Consumers: their own `emit` functions and the statistics arm, in process. The existing committed reader, `check-trajectory-fresh`, reads the trajectory output and is held byte-identical.
- **`DRIFT_KIT_STATISTICS_FILE` (delta 3).**
  - Producer: the knob table's default, so every deployed configuration sets it.
  - Consumer: the arm, at its write.
  - Roster-holding readers: the knob table and `--emit knob-roster`, which print it; `check-knob-default-coupling`, which couples the table's default to the SPEC bullet; `check-knob-citation`, which bars the value outside drift-kit's SPEC, so the README cites the knob by name only (delta 6). Probe: `git grep -n "DRIFT_KIT_STAGE_ECONOMICS_LOG" -- native/src '*.md' ':!docs/posts'`, the nearest sibling knob's readers.
- **The `/economics` step (delta 4).** Producer: the skill template. Consumer: a session running `/economics`. This repo's shim, `.claude/commands/economics.md`, names the template and binds only the posture, so it needs no edit.
- **Point 5:** delta 3 widens the metric dir's member set and narrows no corpus. No gate reads the metric dir (§The stage-economics meter).
- **Point 6:** no delta obliges the members of an enumerable corpus.

## Existing sections updated

The roster came from the reader probes above and `git grep -n "metric-dir log\|append-only trend logs\|chains .--emit overhead-meter" -- drift-kit` over the tracked tree.

- `drift-kit/SPEC.md`, the new §The statistics view (delta 1).
- `native/src/emit/statistics.rs`, a new file, and `native/src/emit/mod.rs`, the arm table (delta 1).
- `native/src/emit/trajectory.rs`, `native/src/emit/queue_flow.rs` and `native/src/emit/stage_economics.rs` (delta 2).
- `drift-kit/SPEC.md` §Layout and configuration and `native/src/knobs/drift_kit.rs` (delta 3).
- `drift-kit/SPEC.md` §The `/economics` skill and `drift-kit/templates/economics.md` (delta 4).
- `drift-kit/SPEC.md` §Testing and `drift-kit/smoke/install.sh` (delta 5).
- `drift-kit/README.md` (delta 6).
- `.claude/commands/close.md`, the housekeeping binding (delta 7).
- `.workflow/release-declarations.md`: a `## Behavior changes` bullet for the new arm and the `/economics` step, and a `## Knob changes` bullet for `DRIFT_KIT_STATISTICS_FILE` (deltas 1, 3 and 4).
- `docs/drift-kit/SPEC.md` and `docs/drift-kit/README.md`, the generated mirrors (all deltas).

## Retired spellings

- `A metric-dir log` — the lead-in of §Layout and configuration's default-placement sentence, which becomes `A metric-dir member` once the set holds a view as well as logs (delta 3).

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Run on this tree** — `--emit statistics` run over this repo's history and its own trend log, every panel `charted`, the file opened and read once; a panel that misleads at this tree's iteration count is fixed before the merge.
- [ ] **Fixture suites** — the drift-kit consumer smoke (`--run-consumer-smoke` with every kit vendored), the crate tests, `bash gate-sdk/bin/build-native.sh` and the battery run green, with `check-trajectory-fresh` clean against the refactored harvest.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, before the drain stage. Its `[roadmap:]` tag leaves with it, and `ROADMAP.md` is regenerated in that commit.
