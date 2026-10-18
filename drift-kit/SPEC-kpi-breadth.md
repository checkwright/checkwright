# SPEC amendment: kpi-breadth

Pairs `statistics-kpi-breadth`. The statistics view (§The statistics view) charts seven figures. None of them is a drift-report KPI, and spend shows only by stage band. The direction on the entry is to add the useful KPIs, governance overhead first, and to split spend and cache-read by model as well as by stage band, which needs no new input since the stage-economics log carries a model column. This amendment adds five panels and stacks a sixth, so the view charts twelve figures.

**Which KPIs join, and why these.** A KPI joins only where a per-iteration value can be re-derived at render time, from committed history at each close or from a series a log already records. That is the rule the view's current figures follow. It also keeps the view a function of closed history and the logs (§The statistics view, *Determinism*).

- **Governance share joins**, the direction's first. The overhead meter's log (`DRIFT_KIT_OVERHEAD_LOG`) records one line per measured session, and the stamps map a session to its iteration. In this tree at align, 168 of the 290 iteration names carrying a close stamp have a measured session the stamps name: 271 log sessions, 102 of them named by no stamp, joined against the history ∪ live stamps over the state file's committed history, a stamp of four fields or five alike, as the history grammar admits both (`native/src/history.rs`, `history_stamp`).
- **Always-loaded joins.** It is the health triad's second member, read at every close beside `qnet` and the governance share (this repo's `.claude/commands/close.md`, the housekeeping binding). Its surface half can be re-derived at any commit: context-kit's meter already measures the surfaces' size at a past commit for its stale mark (context-kit/SPEC.md §The always-loaded meter). The hook body cannot be recovered at a past commit, which that section states as its own limit, so the panel charts the surface half and says so.
- **Deferred age joins.** The queue file at each close commit is committed history, and `kpi-deferred-age`'s read takes it unchanged. The age is counted at the close stamp's date, never today, so the figure stays deterministic.
- **Refused, for want of a series:**
  - *prompt friction* and *knowledge-friction counts* are gitignored per-iteration capture, drained at each close;
  - *gate battery time* is a scratch timings file each unfiltered run overwrites;
  - adding a log for each would buy a series no reader but this view asks for.
- **Refused, for a series that misleads:** the *recurrence count* sums the `[recurrence:]` arrays of the entries still live at a close. An entry leaving for Done takes its count with it, so the sum falls when work lands, not when findings stop recurring.

**The model split's form is a panel, not a band.** One stack cannot hold two partitions of the same bar. So spend and cache-read each get a by-model panel beside a by-band one. The cache-read panel, one bar per iteration today, becomes the by-band stack, so its total stays the stack's height and nothing is lost. A model's colour follows the model, never its rank in the charted window: models are ranked over every closed iteration the log names, so a smaller `<n>` repaints no band.

## What changes

### (1) The view charts twelve figures {design-bearing} {user-facing: statistics-kpi-breadth's direction, "add the useful KPIs, governance overhead first, and split spend and cache-read by model as well as by stage band", the figure set left to this stage by its Deliverable}

Replacements in `drift-kit/SPEC.md` §The statistics view. **Not yet applied.**

- **The figures** bullet becomes:

  > - **The figures**, each one panel, in this order:
  >   1. *spend*: the `cost` of the iteration's trend-log rows, summed, in three stacked bands: the stage rows, the supervision row, and the fan-out rows, told apart by `DRIFT_KIT_SUPERVISION_LABEL` and `DRIFT_KIT_FANOUT_SUFFIX`, the suffix first, so a supervision anchor's fan-out is fan-out spend. A row whose `cost` is `n/a` adds nothing and marks its iteration's bar as incomplete. An iteration no trend-log row names draws a baseline tick, never a zero bar, since the log's absence of a row is not a measured zero.
  >   2. *spend by model*: the same sums stacked by the row's `<model>` instead (the model bands, below).
  >   3. *cache-read*: the `cr` tokens of the same rows, in the same three bands. It is the meter's headline field, kept beside spend so the lever stays visible.
  >   4. *cache-read by model*: the same tokens in the model bands.
  >   5. *governance share*: the overhead meter's byte-proxy proportion per iteration (the governance share, below), the gate-output share and the rest of governance stacked on a 0–100% axis.
  >   6. *always-loaded*: the code points of `CONTEXT_KIT_SURFACES` at each close commit, as a line (the always-loaded line, below).
  >   7. *commit shape*: the feature and debt commit counts, stacked (the extractor's commit-shape harvest).
  >   8. *amendments*: the merged-amendment count as bars, and the longest lag as a line in its own strip under the bars, sharing their x-axis, so no plot carries two y-scales. The line is drawn only where an amendment merged. Both come from the extractor's amendment-latency harvest.
  >   9. *validate*: one cell per iteration reading clean, non-clean or no attestation (the extractor's validate harvest).
  >   10. *gate roster*: the `gates.list` member count at each close, as a line.
  >   11. *queue flow*: the filed and drained counts per window (§The queue-flow arm), filed above the axis and drained below, joined to the x-axis on the window's row label. The windows read are the trailing `<n>`, every window by default, and an iteration no window names draws a baseline tick.
  >   12. *deferred age*: the age in days of the oldest defer date in the deferred section of the queue file at each close commit, counted at the close stamp's date, as a line (`kpi-deferred-age`'s read, §Bundled KPIs).
  >
  >   The stdout figure names are `spend`, `spend by model`, `cache-read`, `cache-read by model`, `governance share`, `always-loaded`, `commit shape`, `amendments`, `validate`, `gate roster`, `queue flow` and `deferred age`.

- **The inputs** bullet becomes:

  > - **The inputs.** Commit shape, amendments, validate, gate roster, queue flow, always-loaded and deferred age read committed history alone, so they render in any clone. The four spend and cache-read figures read `DRIFT_KIT_STAGE_ECONOMICS_LOG`, and governance share reads `DRIFT_KIT_OVERHEAD_LOG`. Only the measuring machine holds either log. One harvest feeds both renderers: the arm calls the trajectory extractor's harvest, the queue-flow arm's window computation, the meter's row parse, `kpi-overhead`'s line read, `kpi-deferred-age`'s defer-date read and the always-loaded meter's past-commit surface read in process. It never parses an arm's printed table, so the view and the tables cannot disagree about a figure. A log line off its grammar parses to nothing and charts nothing.

- **Degrade**: the reasons sentence becomes "The reasons are no log of the figure's own (the stage-economics log, or the overhead log for governance share), no committed state file, no closed iteration, fewer than two iteration starts, context-kit absent (always-loaded, on `kpi-always-loaded`'s presence witness), no charted close commit holding a defer date (deferred age), or an input naming none of the charted iterations." An iteration a figure has no value for draws a baseline tick, as spend's does: no measured session, a close commit holding no queue file or no defer date.
- **Determinism**: "The document is a function of closed history, the log and `<n>`" reads "The document is a function of closed history, the two logs and `<n>`".
- **Privacy**: "The document carries no account identifier and no session id, since neither input carries one." reads "The document carries no account identifier and no session id: neither log carries an account, and the overhead log's session keys join a line to its iteration and are never printed, a mark naming its session count only."
- **Caveat counts**: "the charted iterations with incomplete pricing, and the queue-flow windows whose label names no closed iteration." reads "the charted iterations with incomplete pricing, the queue-flow windows whose label names no closed iteration, and the overhead-log sessions the stamps map to no closed iteration." The stdout line is `<k> overhead-log session(s) name no closed iteration`, printed where non-zero as the others are.
- **The home paragraph**, "The view stays local, and the home follows from its inputs": its opening sentence "Two of its figures come from the stage-economics log, which lives under the gitignored `DRIFT_KIT_METRIC_DIR`." reads "Five of its figures come from logs under the gitignored `DRIFT_KIT_METRIC_DIR`: the four spend and cache-read panels from the stage-economics log, and governance share from the overhead log." Its "that log" in the next sentence reads "the stage-economics log". "it drops the two figures the meter exists to make visible close-over-close" reads "it drops the spend and cache-read figures the meter exists to make visible close-over-close", and "Committing a projection of the log" reads "Committing a projection of the logs".

`native/src/emit/statistics.rs`: `FIGURES` and the panel table take the twelve in that order, each panel's caption naming its source, as the existing seven do. Its `#[cfg(test)]` module gains cases for:

- a model ranking that a smaller `<n>` leaves unchanged, and an eighth model folding into `other models`;
- the governance join, including a session stamped in two iterations, which lands in the later one;
- the deferred age, counted at the close stamp's date off a literal queue;
- each new degrade.

### (2) The model bands {design-bearing} {user-facing: as delta 1}

A new bullet in §The statistics view, after **The figures**. **Not yet applied.**

> - **The model bands.** A by-model panel stacks one band per `<model>` value of the trend log. Models are ranked by their summed priced `cost` over every closed iteration the log names, ties by summed `cr`, then by the id, so the ranking does not depend on `<n>` and a trimmed view repaints no band. The first seven ranked models take the categorical slots in that order, both by-model panels share one assignment, and every later model folds into one `other models` band in a neutral. A model whose rows across the closed iterations carry no tokens draws no band. The legend names each band, and a mark's title names its model and value.

`drift-kit/SPEC.md` §The stage-economics meter, the trend log's field readers: the statistics view's bullet reads "the statistics view reads the iteration, `<stage>`, `<model>`, `cr` and `cost` fields through the row parse, the inverse of the writer's format beside it (§The statistics view);". **Not yet applied.**

`native/src/emit/statistics.rs`:

- the per-iteration sum keeps `cr` by band as well as `cost`, and both by model;
- the model ranking is computed once over the closed iterations before `trailing` trims them;
- the document's CSS gains the seven categorical slots and a neutral `other` swatch in both colour schemes. Run them through a categorical-palette check before landing them: lightness band, chroma floor, adjacent-pair separation under colour-vision deficiency, and contrast against each mode's surface. The current `--s1`–`--s3` become the first three slots where they pass.

### (3) The governance share {design-bearing} {user-facing: as delta 1}

A new bullet in §The statistics view, after **The model bands**. **Not yet applied.**

> - **The governance share.** An overhead-log line belongs to the iteration its `<session8>` stamped, read from the state file's history ∪ live stamps. A session stamping in two iterations belongs to its last stamp's, the attribution rule of §The stage-economics meter. An iteration's share sums its lines: the gate-output share is Σ`gate` / Σ`total`, and the rest of governance is (Σ`gov` − Σ`gate`) / Σ`total`. Both are ratios of sums, so a long session weighs as its bytes do. `kpi-overhead` instead averages `pct` per session over a trailing window, so the two readings agree only where an iteration has one measured session. A line whose session no stamp names charts nowhere, and the caveat counts gain `<k> overhead-log session(s) name no closed iteration`. The panel inherits the meter's byte-proxy caveat, which its caption carries.

`drift-kit/SPEC.md` §The overhead meter, the field-readers paragraph: "`gov` has no *log* reader and is kept: it is derivable-but-stated (`total - task`, and the sum of the four categories), and the field-removal rule reaches a field nothing reads at all." reads "The statistics view reads `session8`, `total`, `gov` and `gate`, its governance share's inputs (§The statistics view)." **Not yet applied.**

`drift-kit/SPEC.md` §Layout and configuration, `DRIFT_KIT_OVERHEAD_LOG`: "The collator, `kpi-overhead` and the meter all read it there" reads "The collator, `kpi-overhead`, the statistics view and the meter all read it there". **Not yet applied.**

`native/src/emit/kpi/overhead.rs`: the field read `tagged` becomes `pub(crate)`, so the view reads a line's fields through the KPI's own parse. A line's `session8` is its second whitespace field. The view takes the stamps through `native/src/history.rs`' `stamp_lines`, the collector the stage-economics meter and `kpi-stage-economics-lag` share.

### (4) The always-loaded line and the deferred age read committed history at each close {design-bearing} {user-facing: as delta 1}

A new bullet in §The statistics view, after **The governance share**. **Not yet applied.**

> - **The always-loaded line** is the code points of the surfaces `CONTEXT_KIT_SURFACES` names, read from each closed iteration's close commit through the always-loaded meter's past-commit read (context-kit/SPEC.md §The always-loaded meter). The roster applied is today's, so a surface added or dropped since is read at every past close as the roster now names it, and a path absent at a commit counts nothing, as the meter counts it. The hook body is not recoverable at a past commit and is not charted. Each point is sampled at the close commit, so a close's own brevity pass shows at the next close. Its presence witness is `kpi-always-loaded`'s.
> - **The deferred age** reads `DRIFT_KIT_QUEUE_FILE` at each close commit, takes the defer dates of its `DRIFT_KIT_DEFERRED_SECTION` through `kpi-deferred-age`'s read, and counts whole days from the oldest to the close stamp's own date.

`native/src/emit/trajectory.rs`: each `Closed` carries its close commit and its close stamp's date. The harvest already holds the commit, as `e.close`, to range the iteration and to count its gates. It holds no date, since `added_stamp` returns the iteration and the stage alone, so the harvest also reads the stamp's fourth field, the last close stamp's for an iteration stamped close twice. `native/src/emit/always_loaded.rs`: the past-commit surface read `surface_at` becomes `pub(crate)` and reads through `native/src/history.rs`' `Blobs`, the batched blob reader the queue-flow arm uses, in place of one `git show` per surface. So the meter's stale mark and the view share one read, and a long history buys one `git cat-file` process rather than one spawn per surface per close. The view hands it each close commit with the `CONTEXT_KIT_SURFACES` roster. `native/src/emit/kpi/deferred_age.rs`'s `defer_dates` and `native/src/emit/kpi/mod.rs`' `section_lines` are read as they stand, over the queue blob the same reader returns. **Not yet applied.**

### (5) The arm's knob roster {mechanical}

`native/src/emit/statistics.rs` `KNOBS` gains `DRIFT_KIT_OVERHEAD_LOG`, `CONTEXT_KIT_SURFACES`, `GATE_SDK_KIT_DIRS` and `GATE_SDK_GATES_DIR`. The last two are the kit-root read the presence witness takes, as `--emit-drift-report` declares them. The `--emit-statistics` row's comment in `native/src/emit/mod.rs` reads "its roster is its file and the knobs of the harvests and reads it calls". The row's roster is read by the knob-file derivation, `check-reads-couples` and `check-gate-substrate-parity` (§The report skeleton names the three for the collator's row), so a knob the arm reads and the row omits is missing from the consumer's derived knob file. **Not yet applied.**

### (6) The smoke charts the twelve {design-bearing}

`drift-kit/SPEC.md` §Testing, the **statistics view** bullet. "carries all seven panels" reads "carries all twelve panels", and these sentences follow "the incomplete-pricing count reads 1":

> The trend log carries two models, so both by-model panels name each in their legend. A purpose-built overhead log holds a line keyed on alpha's close stamp's session and one keyed on no stamp, so governance share is charted and the overhead caveat count reads 1. The fake history commits no queue file, so deferred age prints `n/a (no charted close commit holds a defer date)`, and the arm's crate tests chart it off a literal queue. The always-loaded line is asserted as the fixture's kit roots resolve context-kit's witness.

Its log-absent sentence reads "A log-absent run charts the rest and prints the four spend and cache-read figures as `n/a (no stage-economics log)`, and an overhead-log-absent run prints governance share as `n/a (no overhead log)`." **Not yet applied.**

`drift-kit/smoke/install.sh`, the statistics leg:

- the panel count asserts 12;
- the trend-log heredoc gains a second-model row for alpha;
- an overhead log is written under the throwaway metric dir;
- the charted-figure loop takes the new names and the `n/a` loop the four log-bound ones;
- a governance-share assertion, a two-model legend assertion, the overhead caveat assertion and the deferred-age degrade are added;
- the always-loaded line is asserted as `charted` or as `n/a (context-kit absent)`, whichever the fixture's kit roots give, and the build pins that one.

### (7) The README and the release record name the twelve {mechanical} {user-facing: as delta 1}

**Not yet applied.**

- `drift-kit/README.md`, the `--emit statistics` paragraph. "spend and cache-read from the stage-economics log, commit shape, amendments, validate, gate roster and queue flow from committed history" reads "spend and cache-read by stage band and by model from the stage-economics log, governance share from the overhead log, and commit shape, amendments, validate, gate roster, queue flow, the always-loaded surface and the oldest deferred entry's age from committed history".
- `.workflow/release-declarations.md`, `## Behavior changes`, the unreleased `--emit statistics` bullet. Its parenthesis is rewritten the same way rather than contradicted by a second bullet.

## Producers and consumers

- **The five new panels and the stacked cache-read (deltas 1 to 4).**
  - Producer: `--emit statistics`, run by the `/economics` skill's statistics step, by this repo's close binding right after the stage-economics feed (`.claude/commands/close.md`, probe `grep -n "emit statistics" .claude/commands/close.md`), and ad hoc.
  - Consumers: the operator opening the file. The stdout figure lines and caveat counts are read by the `/economics` narrative's view bullet (`drift-kit/templates/economics.md`), which names any `n/a` panel and keys on no figure name, so it needs no edit.
  - Each field has a reader. The model-band, governance and always-loaded values are each read by the operator, through a mark and its title. The new caveat count is read by the narrative, through stdout.
- **The overhead log's new reader (delta 3).**
  - Producer: the overhead meter, fed by the close binding's housekeeping, unchanged.
  - The reader takes `session8`, `total`, `gov` and `gate`, and §The overhead meter's field rule now names it, which keeps `gov` read.
  - Roster-holding readers of the knob: the arm's `KNOBS` (delta 5) and the three readers of an arm row's roster. The knob's table row is unchanged. Probe: `git grep -n "DRIFT_KIT_OVERHEAD_LOG" -- native/src '*.md' ':!docs/posts'`.
- **The exposed reads (deltas 3 and 4).** `tagged`, `surface_at` and the `Closed` fields are crate-internal, read by their own modules and the view. `check-trajectory-fresh` byte-compares the trajectory table, so delta 4's harvest fields reach its output only if a render reads them, and the trajectory renderer does not.
- **Point 5:** no delta narrows a corpus. Two readers assert an exact panel count and both move with the panels in the same commit: the smoke's, which delta 6 moves from 7 to 12, and the crate's, in `native/src/emit/statistics.rs`'s tests. There the document's panel count is asserted at 7 twice, a no-state-file case counts 7 `n/a` lines, and the test holding the first count is named for seven panels. Each moves to 12, and the name follows.
- **Point 6:** no delta obliges the members of an enumerable corpus.

## Existing sections updated

The roster came from `grep -n "statistics\|seven\|gov. has no" drift-kit/SPEC.md drift-kit/README.md drift-kit/smoke/install.sh .workflow/release-declarations.md`, `grep -n "statistics" native/src/emit/mod.rs` and the reader probes above.

- `drift-kit/SPEC.md` §The statistics view: the figures, inputs, degrade, caveat counts, determinism and privacy bullets, the home paragraph after the bullets, and the new bullets (deltas 1, 2, 3 and 4).
- `drift-kit/SPEC.md` §The stage-economics meter, the trend log's field readers (delta 2).
- `drift-kit/SPEC.md` §The overhead meter, the field-readers paragraph, and §Layout and configuration, `DRIFT_KIT_OVERHEAD_LOG` (delta 3).
- `drift-kit/SPEC.md` §Testing, the statistics view bullet (delta 6).
- `native/src/emit/statistics.rs` (deltas 1, 2, 3, 4 and 5).
- `native/src/emit/kpi/overhead.rs` (delta 3).
- `native/src/emit/trajectory.rs`, `native/src/emit/always_loaded.rs` and `native/src/emit/kpi/deferred_age.rs` (delta 4).
- `native/src/emit/mod.rs`, the `--emit-statistics` row's comment (delta 5).
- `drift-kit/smoke/install.sh` (delta 6).
- `drift-kit/README.md` and `.workflow/release-declarations.md` (delta 7).
- `docs/drift-kit/SPEC.md` and `docs/drift-kit/README.md`, the generated mirrors (all deltas).

## Retired spellings

- None — no delta retires a name. The seven figure names stay and five join them. The sentence on `gov`'s missing reader is prose, rewritten.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Run on this tree** — `--emit statistics` run over this repo's history and both its logs, every panel `charted`, the file opened and read once in both colour schemes. A panel that misleads at this tree's iteration and model count is fixed before the merge.
- [ ] **Fixture suites** — the drift-kit consumer smoke (`--run-consumer-smoke` with every kit vendored), the crate tests, `bash gate-sdk/bin/build-native.sh` and the battery run green, with `check-trajectory-fresh` clean.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, before the drain stage.
