# SPEC amendment: stage-economics-feed

**The stage-economics log gets a trustworthy date column, a feeding point, and a
KPI that shows when feeding lapses.** These are two units designed together. Each
fix breaks without the other. Feeding the log at every close is the act that
re-dates it today: running the meter rewrites every row it can still re-derive
with the day of the run. And a date fix that nobody feeds leaves a correct column
with no new rows in it.

**What the log is for, and the reader the defect harms.** The consumer's lead
binding judges a tier on the `cr` trend of the bare `align` rows. A trend is a
series ordered by date. Under today's semantics a re-run puts most of the history
on one day. At this stage's probe, the log held 1,695 rows, and 868 of them
carried the date of the last run. So the one hand-read reader that parses the
log reads a series whose order is an artifact of the run.

**What stays as it is.** The dedup key stays the `<iteration> <stage> <model>`
triple. The meter still re-derives history on every run with no watermark, and
the log grammar is unchanged: no field is added and none is renamed. Only the
*value* of `date` changes meaning, from the day the row was measured to the day
the stage ran. The meter stays advisory: exit 0, never in `gates.list`.

## What changes

### (1) A row's `date` is its stage's stamp date, re-derived for every logged line on every run

The trend log's `<date>` field changes from the day of the run to the latest stamp
date of the stage the row prices, so a re-run no longer re-dates history {design-bearing}.
**Not yet applied.**

**The derivation reads only stamps**, never transcripts. The meter already
collects every stamp from history ∪ live, and each stamp carries a
`<date>` field (lifecycle-kit/SPEC.md §The state machine owns the grammar). From
those stamps the run builds two maps: the latest date per
`(iteration, stage)`, and the latest date per iteration. Dates are ISO
`YYYY-MM-DD`, so the latest date is the greatest string. A row's date is:

- **a stage row**: the latest date among the stamps naming its `(iteration, stage)`;
- **a fan-out row**: the same lookup, on its stage with `DRIFT_KIT_FANOUT_SUFFIX`
  removed. The anchor's stage is the date of its subtree;
- **a supervision row**: the latest date among *all* stamps of its iteration. A lead
  carries no stamp of its own, and it supervises the whole iteration through close.

**Latest, not first.** A split stage stamps once per session, and the row is their
sum. The latest session is the one that makes the sum final. The latest date is also
what an ordered series wants: the day the stage's figures stopped moving.

**The derivation applies to every line the run writes back, not just the lines it
re-measures.** The meter already reads the whole log into memory and writes it back
whole. Before that write, every retained line whose `(iteration, stage)` derives a
date gets that date. So a row whose transcript has aged out is re-dated too. It can
no longer be re-measured, but its stamps are still in committed history. At this
stage's probe, every one of the 1,695 logged rows derived a date by this rule, and
1,672 of them would move. So the first run after the merge corrects the whole
existing log, and no migration step is needed.

**Degradation, visible and counted.** Two cases degrade:

- A live-file stamp whose fourth field is not an ISO date contributes no date. The
  history arm's grammar already refuses such a stamp.
- A row, logged or new, may find no stamp for its key. It keeps the date it has, and a
  new row takes the run's date.

Each case adds to one stdout count, `<n> row(s) dated by measurement — no dated
stamp names their (iteration, stage)`. The line is printed only when the count is
non-zero. The alternative, dropping such a row, is refused. It would destroy spend
the meter measured correctly because a date was missing.

**The measurement date stays on stdout, and only there.** The run header already
prints `stage-economics: <today>`, and the `/economics` narrative reads stdout. The
§The trend log sentence claiming `date` carries a reading-age caveat is replaced.
That claim had no reader: a row whose transcript still exists is re-measured on
every run, and a row whose transcript has aged out is final. Neither is helped by a
reading age.

### (2) A re-measured row is replaced where it stands, so a re-run over unchanged inputs is byte-identical

The log's replace-on-re-measure keeps each line in place, and only a key new to the
log appends {mechanical}.
**Not yet applied.**

Today the writer removes a key's lines and appends the new line. So every run
reorders the log even when no figure changed. After this change:

- a key already in the log has its **first** line rewritten in place;
- any later duplicate of that key is dropped (legacy lines from the pre-fold
  collision);
- a key not in the log appends.

Together with delta 1, a second run over unchanged stamps, transcripts and price
table writes a byte-identical log. That makes "the run re-dated nothing" something
a fixture can check (delta 5), instead of something a reader has to infer. The
touch rule is unchanged: a run that emitted no row neither creates nor rewrites the
log.

### (3) The feeding point: a consumer whose decisions read the log runs the meter at its terminal stage

drift-kit/SPEC.md §The stage-economics meter gains a short **Feeding** paragraph,
and this repo's close binding gains the run {design-bearing}.
**Not yet applied** (either half).

**Kit contract (drift-kit/SPEC.md).** The meter is on-demand by construction. A
consumer that reads the log (a tier decision, a close-over-close cost comparison)
binds a feeding point. The terminal stage's housekeeping is the natural one. By the
time that stage runs, every stamp of the iteration exists, and its own transcript
and the lead's are both still on disk. Because of delta 1, re-running the meter
changes no date, so it can run at every close. `kpi-stage-economics-lag` (delta 4)
makes a skipped feed visible. The kit names no stage and no binding: where the feed
runs is consumer config.

**Why a stage step, and not a freshness gate or on-demand-only.** The entry listed
three candidates. A freshness **gate** is ruled out by the meter's advisory
contract. The log lives under the gitignored, account-bearing metric dir, which is
absent in CI. There a gate could only pass vacuously, or red on a cause no commit
produced. §Bundled KPIs already refuses a freshness gate for the price table on the
same grounds, so the signal becomes a KPI instead (delta 4). **On-demand-only** would
cut the log's one parsing reader, the lead binding's tier revert signal, and leave
that tier with no way to be re-judged. The stage step has already worked in this
repo: the close binding's housekeeping runs `--emit overhead-meter` every close, and
the overhead log is current to this stage's date. The stage-economics log, which no
binding runs, fell three closes behind.

**Consumer binding (`.claude/commands/close.md` §housekeeping).** Replacement for
the opening sentence pair ("First meter this closing session with … feeding
`kpi-overhead`."). It re-phrases that sentence pair; it does not append to it:

> First meter this closing session with `--emit overhead-meter <session-id>`,
> passing this session's own `close` stamp id, whose invocation and byte-proxy
> contract drift-kit/SPEC.md §The overhead meter owns: it logs the governance-vs-task
> proportion, the per-session producer feeding `kpi-overhead`. Then feed the
> stage-economics log with `--emit stage-economics` (drift-kit/SPEC.md §The
> stage-economics meter, Feeding), the tier decisions' revert signal, and confirm
> `--emit drift-report --trend` reads `econ 0`.

### (4) `kpi-stage-economics-lag`: closes since the newest priced close

drift-kit ships a new built-in Lead KPI, born native, that counts the closed
iterations stamped after the newest closed iteration the log prices {design-bearing}.
**Not yet applied.**

**Definition.**

- **Closed iteration**: an iteration name with a stamp whose stage is the **last
  member of `DRIFT_KIT_STAGES`**.
- **Order**: closed iterations are ordered by the first appearance of that terminal
  stamp in the history ∪ live stamp read, committed history in commit order first,
  then the live file.
- **Priced**: an iteration is priced when any log line's iteration field (field 2)
  names it.
- **Lag**: the number of closed iterations after the last priced one in that order.

The count is keyed on iterations, never on dates. Several closes can stamp on one
day, so a date comparison could not tell them apart. It also means the KPI is
correct whether or not delta 1 has landed.

**Shared read, one producer.** The meter's history ∪ live stamp collector moves out
of `native/src/emit/stage_economics.rs` into a shared in-crate function, and both
the meter and this KPI call it. Two copies of the union would be a second producer
of one coupling. That is the shape the `native/src/history.rs` paragraph of
§The stage-economics meter already refuses for the git read.

**Rows and trend.** The KPI emits exactly one Lead row:

- `<N> close(s) unpriced since <iteration> (run --emit stage-economics)` when N > 0;
- `0 — newest close priced (<iteration>)` when N = 0.

`--trend` emits `econ <N>`.

**Degradations.** Each is fail-visible `n/a` with no trend fragment:

- `n/a (no stage-economics log)`: the log file is absent;
- `n/a (empty stage roster)`: `DRIFT_KIT_STAGES` is empty;
- `n/a (no closed iteration)`: no terminal stamp exists in either source;
- `n/a (no closed iteration priced)`: the log exists but names none of them.

The last case is not reported as a count. With nothing priced, the count would be
every close in history, which measures when the log was started rather than lapse.

**Lead, not lag.** The value is exact rather than a lower bound, and it is the
signal to act on before the gap widens.

**Cost on the session-start path.** `--trend` runs through the context hook on
every session start. At this stage's probe, the state file's `git log -p` took 34 ms
and the whole trend line 46 ms, so the added read is priced and accepted. The
§The report skeleton paragraph that prices the iteration-start read names this
second read beside it.

**Knobs read** (all existing, none new): `DRIFT_KIT_STAGE_ECONOMICS_LOG`,
`DRIFT_KIT_STATE_FILE`, `DRIFT_KIT_STAGES`.

### (5) Smoke coverage for both behaviours

`drift-kit/smoke/install.sh` gains assertions for the stamp date, the in-place
byte-identical re-run, the heal, the undated fallback, and the KPI {mechanical}.
**Not yet applied.**

- **Stamp date.** The flat fixture's grammar assertion tightens from `^[0-9-]+ ` to
  the fixture stamp's own date, `^2025-01-01 smoke build …`.
- **Byte-identical re-run.** The existing re-measure assertion additionally compares
  the log before and after the second run, byte for byte.
- **Heal.** A pre-seeded log line for the fixture key, carrying a different date and
  preceded by an unrelated line, comes back with the stamp's date and in its
  original position.
- **Undated fallback.** A live stamp with no date field yields the
  `dated by measurement` count line.
- **The KPI.** A `solo` run over a fake stamp history with two closed iterations,
  one priced and one not, reads `1 close(s) unpriced` with trend `econ 1`. Each of
  the four degradations is covered.

§Testing's prose names the new assertions.

### (6) Both entries complete

At merge, `stage-economics-log-redates-rows` and
`stage-economics-meter-has-no-feeding-obligation` both move to Done. Neither is a
corpus increment, so neither demotes {mechanical}.

## Producers and consumers

**The row date (delta 1).**
- *Producer:* `--emit stage-economics`, from the stamp read it already makes. It is
  reachable in this repo through the close binding (delta 3) and through any ad-hoc
  or `/economics` run.
- *Consumers:*
  - the lead binding's `cr` trend judgment (`.claude/commands/lead.md`), which
    orders bare `align` rows by date. Its contract is unbroken, and its series is
    now in stage order;
  - the operator reading `cost` close-over-close;
  - the A/B benchmark rung, which would consume the log.

  No gate reads the log.
- *Field reader:* `date` is read by those same three. The stdout `dated by
  measurement` count is read by the `/economics` narrative and the operator at
  measurement time.
- *Red conditions of the log's asserting readers:* `drift-kit/smoke/install.sh`
  asserts exact line counts (`-eq 1`) and an anchored grammar regex. Delta 2 keeps
  the counts. Delta 5 tightens the regex, and build runs the smoke rather than
  inspecting it.
- *Satisfying value per logged line (point 6):* a stamp date. The probe joined
  `git log --reverse -p -U0` of `.workflow/WORKFLOW-STATE.txt` plus the live file
  against `.metric/stage-economics-log.txt`, and every logged row derived a date.
  None falls to the counted fallback today.

**The feeding obligation (delta 3).**
- *Producer:* the close stage session, under this repo's close binding. That
  binding is the enabling config, and it is live.
- *Consumer:* the log itself, then everything above. The obligation's own check is
  the `econ 0` read in the same step.

**`kpi-stage-economics-lag` (delta 4).**
- *Producer:* the drift-report collator, via the `BUILTINS` table, when
  `kpis.list` registers the name. This repo registers it in `scripts/kpis.list`.
- *Consumers:*
  - the session-start trend line, which context-kit's hook injects;
  - the full report;
  - the close binding's `econ 0` confirmation (delta 3).
- *Roster-holding readers of the minted name (point 2):*
  - `native/src/emit/kpi/mod.rs` `BUILTINS`;
  - `drift-kit/templates/kpis.list`. `check-template-registry-parity` reds on a
    dispatched member the template does not register;
  - that gate's `good/drift-kit/templates/kpis.list` fixture, which a crate unit
    test pins to the dispatched roster;
  - `scripts/kpis.list`;
  - `drift-kit/smoke/install.sh`, whose floor asserts one row per registered name;
  - the generated `docs/enforcement.md`.
- *Fields:*
  - the row value is read by a person at the report;
  - the `econ <N>` fragment is read by the session-start reader and by the close
    binding's confirmation;
  - the iteration name in the row tells the reader where to resume feeding.

**The shared stamp read (delta 4).**
- *Producer:* one in-crate function.
- *Consumers:* the meter and the KPI. The report skeleton's iteration-start read is
  lifecycle-kit's shared read, unchanged, and is not this function.

No delta narrows a corpus, so point 5 binds only where a reader was named above.

## Existing sections updated

- `drift-kit/SPEC.md` §The stage-economics meter, §The trend log: the dedup-key
  paragraph's "replaces its line" becomes the in-place replace, and the
  `date`-semantics passage ("`date` carries the reading-age caveat … the trajectory
  extractor is the surface that renders it") is re-phrased to stamp-date semantics
  (deltas 1 and 2).
- `drift-kit/SPEC.md` §The stage-economics meter: the supervision and fan-out
  "No new field" bullets state their row's date rule (delta 1). A new Feeding
  paragraph follows §The trend log (delta 3).
- `drift-kit/SPEC.md` §Bundled KPIs gains the Lead member. §The report skeleton's
  session-start cost paragraph names the second read. §Layout and configuration's
  entries for `DRIFT_KIT_STATE_FILE`, `DRIFT_KIT_STAGES` and
  `DRIFT_KIT_STAGE_ECONOMICS_LOG` name the KPI as a reader (delta 4).
- `drift-kit/SPEC.md` §Testing names the new assertions (delta 5).
- `native/src/emit/stage_economics.rs`: date maps, the retained-line re-date, the
  in-place write and the count caveat (deltas 1 and 2). The stamp collector moves
  to the shared function (delta 4).
- `native/src/emit/kpi/` gains a new module, with `mod.rs` `BUILTINS` and the `Ctx`
  fields in `native/src/emit/drift_report.rs` (delta 4).
- `drift-kit/templates/kpis.list`, `scripts/kpis.list` and
  `gate-sdk/gate-tests/check-template-registry-parity/good/drift-kit/templates/kpis.list`
  (delta 4).
- `drift-kit/README.md`: the stage-economics paragraph under §Use names the feeding
  point and the lag KPI in one sentence (deltas 3 and 4).
- `.claude/commands/close.md` §housekeeping: the replacement text in delta 3.
- `drift-kit/smoke/install.sh` (delta 5).
- `TASK-QUEUE.md`: both entries move to Done at merge (delta 6).
- <!-- update-target-exempt: generated projections, each rostered with its freshness gate and regen command in docs/site-architecture.md §Generated projections and their freshness gates --> The `docs/` mirrors of `drift-kit/SPEC.md` and `drift-kit/README.md`, `docs/enforcement.md`, `docs/footprint.md`, `docs/value.md`, and the generated pre-commit hook.

## Retired spellings

- None — no name is retired. The `date` field keeps its name and its position, and
  only its value's meaning changes.

## Definition of Done

- [ ] **Causal completeness**: every point of canon-kit/SPEC.md §The
      causal-completeness check holds for the row date, the feeding obligation, the
      KPI and the shared read.
- [ ] **Instruction surfaces: instruction only**: the close-binding replacement text
      carries no grounds. The grounds live in delta 3 and merge into drift-kit's
      Feeding paragraph.
- [ ] **Merged with no information lost**: §The trend log's date passage is
      re-phrased, not appended to. The refused alternatives (a feeding gate,
      on-demand-only, dropping undated rows) survive in the merged sections'
      prose.
- [ ] **Log healed**: one run of the meter on this repo leaves no row dated after
      its stage's latest stamp. A second run leaves the log byte-identical.
- [ ] **KPI live**: `--emit drift-report --trend` reads `econ 0` after the close
      binding's feed.
- [ ] **Amendment deleted**: this file is removed on merge, and none remain for
      the component (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated**: `check-amendment-retired-spelling` is green on the
      negative form.
- [ ] **Gaps filed**: cross-component gaps found during the work go to the gap
      inbox.
