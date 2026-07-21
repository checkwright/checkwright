# SPEC amendment: price-table-age-kpi

<!-- Owning component: drift-kit (owns the KPI plugin contract, the bundled set,
     and the stage-economics meter). One contract owner, audited at align:
     nothing here changes lifecycle-kit's state machine, its stamp grammar, or
     any other kit's contract — the meter stays a read-only consumer of the
     stamp line. Other kits are reached as *gate constraints* and *projection
     obligations*, not as contracts this amendment may change; both rosters are
     in §Cross-component reach (align-verified).
     Pairs with the queue entry tagged [spec: SPEC-price-table-age-kpi.md].

     Two deltas, one envelope. Delta A is the named feature. Delta B is the
     governed name that stage-economics-attribution-honesty defect (c) needs if
     supervision is its own row — ruled here rather than in a second amendment
     (lead ruling at promotion, recorded in that entry's body). Both are
     drift-kit economics-honesty contract additions; neither can land without
     the other's surfaces being touched (§The stage-economics meter carries
     both). -->

## What changes

### Delta A — `kpi-price-table-age`, an advisory staleness *and expiry* signal

A new bundled lead KPI reports **how old the price table's own dating is** and
**when its prices stop being true**, so a stale or expired table surfaces as a
trend signal instead of silently mispricing every economics read.

**Why both, and not age alone** (the rationale that merges into §Bundled KPIs):
age and expiry measure different facts, and at the moment that matters most they
point opposite ways. Age measures when someone last *typed* the numbers; expiry
is when the numbers *stop being true*, and nothing makes the first predict the
second. The failure is an **inversion**, not a correlation gap: a table
re-transcribed on 2026-08-30 reads "1d — fresh" on the morning of 2026-09-01,
the day a known introductory-pricing row goes wrong — the age signal is
**quietest exactly when the table is least trustworthy**. That is why the expiry
header exists; an age-only KPI would read reassuringly at the one moment it was
built to flag.

- **The plugin** — `kpis/kpi-price-table-age.sh`, the `kpi-deferred-age`
  age-KPI idiom: full mode emits lead rows, `--trend` emits one compact fragment
  or nothing, exit is always 0. One plugin, one registered name — the expiry
  read is a second row from the same plugin, never a second KPI.
  - Full, always: `lead<TAB>price table age<TAB>priced <N>d ago (as-of <date>)`.
  - Full, second row: `lead<TAB>price table expiry<TAB>expires in <N>d
    (through <date>)`, or `EXPIRED <N>d ago — re-verify (through <date>)` once
    the date has passed.
  - `--trend`: `price <N>d`, and nothing at all when the value is `n/a`. The
    expiry row contributes no second fragment — the trend line takes at most one
    per plugin (§The KPI plugin contract), and staleness is the fragment already
    specified.
- **The surface it reads** — the price table already resolved by
  `DRIFT_KIT_PRICE_TABLE` (§Layout and configuration). No new knob: the plugin
  reads the same consumer-config path the meter prices through, so the table has
  one owner and one address. **The default expression travels with it**: that
  knob's default (`${GATE_SDK_GATES_DIR:-scripts}/price-table.tsv`) is stated
  only inside `bin/stage-economics.sh`, and plugins read *exported env only*
  (§The KPI plugin contract) — a consumer who sets nothing exports nothing, so
  the plugin restates the identical default, the `kpi-overhead` /
  `DRIFT_KIT_METRIC_DIR` precedent. The smoke's writer/reader-divergence
  assertion is the shape that keeps the two copies honest.
- **The dating contract** — two fields on the same consumer-owned header block,
  each read as the first line of that file matching
  `^#[[:space:]]*<field>:[[:space:]]*(\d{4}-\d{2}-\d{2})`, trailing prose on the
  same line ignored:
  - `priced-as-of:` — **required for the age row**; promotes an existing comment
    convention to a governed header name.
  - `prices-valid-through:` — **optional**; when absent the expiry row is
    `n/a (no prices-valid-through: header)` and nothing else changes.

  Both are governed header names — the litmus that makes this unit a feature.
- **Bounds on this delta** (the intent ruling's, recorded in the queue entry so
  the envelope cannot creep): the KPI stays advisory and never gates; the expiry
  header stays optional with an `n/a` lead row when absent; and **no third
  header field is authorized** — a further field is a new unit through scope.
- **Degradations**, each a visible `n/a (<reason>)` in the plugin's own value,
  never a failure and never a block: `n/a (no price table)`,
  `n/a (no priced-as-of: header)`, `n/a (unparseable priced-as-of date)`, and
  for the expiry row `n/a (no prices-valid-through: header)` /
  `n/a (unparseable prices-valid-through date)`.
- **The SSOT consequence, landing with this delta.** Once the header owns the
  expiry date, the date has exactly one home. `scripts/price-table.tsv`'s
  `KNOWN CLIFF` prose block therefore **keeps the post-cliff per-token numbers**
  — transcription help with no other owner — and **drops the duplicated date**,
  citing the header instead. Without that edit, mechanizing the cliff creates a
  second source for it in the very commit that fixes the unread-fact problem
  (de-literalization: prose cites names, the read surface owns the value). The
  shipped template follows the same shape.
- **Not a gate, by construction.** Prices are a dated literal with no
  machine-readable feed, so a freshness *check* would have to fetch externally —
  which reds on causes no commit produced and breaks hermeticity, the shape
  `site-kit/SPEC.md §The monitor boundary` already rules out. An age KPI needs no
  network: it reads a date in-tree and reports. It never joins `gates.list`.
- **Seam.** The mechanism is kit-generic — read a dated header out of a
  consumer-owned file and report its age. No model id, no price, and no roster
  enters the kit; the table stays consumer config (§The stage-economics meter,
  the provenance seam).

### Delta B — `supervision`, a reserved stage value in the economics trend log

Under the split-lead posture the lead session's dispatch, verification, and
battery burn carries **no stamp**, appears in no row, and so every per-stage
total understates the iteration's true cost by the whole supervision line item.
This amendment rules that outcome: **supervision is its own row**, not an
apportionment across stages.

Why a row rather than an apportionment: apportioning supervision across stages
would require an allocation key with no basis in anything measured — the lead's
burn is not proportional to any stage's tokens — and would fold a fabricated
number into figures a tier decision is read off. A distinct row is a *visible*
line item the operator can accept, discount, or ignore. The meter's contract is
that a degraded or absent measurement is visible, never silently folded
(§The report skeleton, degrade discipline); the same rule decides this.

- **The name** — `supervision`, a reserved value of the trend log's `<stage>`
  column, configurable via `DRIFT_KIT_SUPERVISION_LABEL` (default
  `supervision`) so a consumer whose lifecycle roster already carries that word
  can rename it. Per the config-via-env convention; the knob's roster home is
  §Layout and configuration.
- **What the column now means.** The `<stage>` column reads **stage *or*
  cost-bearing role**, not stage alone: a `supervision` row is a role that
  carries cost and no stamp, never a lifecycle stage that dropped out of the
  roster. Naming the widened meaning is the point — the column's members were
  roster-closed until this delta, and a reader who assumes they still are would
  misdiagnose the row as roster drift.
- **Collision rule** — if any stamp the run reads names a stage equal to the
  label, the meter emits a visible notice naming `DRIFT_KIT_SUPERVISION_LABEL`
  and emits **no** supervision rows that run. Checkable from data the meter
  already reads (its own stamps), so this adds no roster dependency and no
  second bound to drift.
- **The attribution invariant** — a transcript's usage is attributed to exactly
  one `(iteration, stage)` pair. In the ordinary case a lead supervises one
  iteration and its whole usage lands on that iteration's supervision row —
  exact, no key. A lead spanning ≥2 iterations apportions across them **in
  proportion to the number of stamped stage sessions it dispatched per
  iteration**, integer-split with the remainder to the iteration holding the
  most dispatches (deterministic); the run's caveat names that key, because a
  disclosed key is honest and a silent one is not.
- **No lifecycle change.** The lead still stamps nothing and moves no cursor:
  the supervising session is *derived*, not stamped (see Producers below).
  lifecycle-kit/SPEC.md §The state machine is untouched, which is what keeps
  this amendment single-component.
- **Ordering against the debt units.** Delta B rests on the attribution
  invariant above, which `stage-economics-attribution-honesty` (a) — one
  session, two stamps, counted in full twice — currently violates. (a) lands
  before or with Delta B, never after; a supervision row added on top of a
  double-counting join reports a fourth wrong number. (b), the unstamped
  continuation session, is orthogonal to this name and stays debt.

## Producers and consumers

### Delta A

- **Producer** — `bin/drift-report.sh` invokes the plugin through the registry
  (§The KPI plugin contract: resolved by name, invoked directly, so the file
  ships with the execute bit or the report degrades that row).
  **Enabling config actually emitted, both halves:**
  - the *registry* entry — `kpi-price-table-age` in the Lead block of
    `scripts/kpis.list` (this repo) and of `templates/kpis.list` (the shipped
    example registry a consumer copies). A plugin no registry names is dead.
  - the *headers* in the table itself. `scripts/price-table.tsv` already carries
    `# priced-as-of:` (line 5 of its header block), so the age row reports a real
    value the day the plugin lands; it does **not** carry
    `# prices-valid-through:`, so this repo's own expiry row is dead until the
    header lands — and the expiry row is the reason the delta widened, making
    this edit load-bearing, not cosmetic. `templates/price-table.tsv` carries
    **neither**, so without the template edit both rows are `n/a` in every
    consumer that ever copies it — the dead-producer shape the
    causal-completeness check exists to catch. Both files gain both headers
    (this repo's dated from its existing `KNOWN CLIFF` block, the template's with
    placeholder dates and a one-line note on what each dates).
- **Consumers** — `drift-report.sh`'s lead section (full mode) and, through
  `--trend`, context-kit's session-context hook via `CONTEXT_KIT_DRIFT_REPORT`
  (already wired; a new registry member needs no context-kit change).
- **Every field has a named reader.** The plugin emits no new record type — it
  emits rows on the existing KPI row contract. Its data points: the age row's
  `<N>d`, read by the operator as the trend level and by the session-context
  trend line via the `price <N>d` fragment, and its `(as-of <date>)` caveat,
  read by the operator as what identifies the thing that aged (the
  `kpi-deferred-age` `(Surfaced <date>)` shape); the expiry row's `<N>d` and
  `(through <date>)`, read by the operator as the re-verification trigger — the
  one reader the age row cannot serve, since at the cliff it reads fresh. No
  other field is added, and the expiry row volunteers no trend fragment because
  no reader takes a second one.

### Delta B

- **Producer** — `bin/stage-economics.sh`, inside the join it already runs. The
  supervising session is **derived from the transcript path, not from a stamp**:
  §The stage-economics meter already specifies the two-tier scan, where a
  dispatched stage session resolves at
  `<sessions-dir>/<lead-session-id>/subagents/<agent>.jsonl` while a lead sits
  directly under the sessions dir. So for every stamp whose transcript resolved
  on the **nested** tier, that path's `<lead-session-id>` component names the
  supervisor, and the stamp itself supplies the iteration. The meter collects
  `(lead-session, iteration) → dispatch count`, resolves each lead's own
  transcript on the flat tier, sums its assistant-turn usage per model with the
  same last-usage-per-message-id reader (no second parser), and emits one row per
  `(iteration, <label>, model)`.
  **Enabling config**: none — the derivation runs on paths the meter already
  walks, under this repo's live split-lead posture, so it is reachable today and
  not test-only. The knob has a default; a consumer sets nothing.
- **Consumers** — the trend log `DRIFT_KIT_STAGE_ECONOMICS_LOG` (same grammar,
  same `<iteration> <stage> <model>` dedup key — `supervision` is a value in an
  existing column, not a new field, so the key, the replace-on-re-measure
  behavior, and every existing reader work unchanged); the `/economics`
  narrative (`templates/economics.md`, materialized as
  `.claude/commands/economics.md`), which gains the supervision line item —
  **this is the row's named reader, and without that edit the row has none**;
  the operator reading close-over-close; the deferred `benchmark-ab-experiment`
  rung's measurement half, which consumes this log and inherits the row.
- **Reader survey of the widened column, run whole-tree so the next reader need
  not re-run it** (re-run and corrected at align — the earlier survey undercounted
  by one). The log has **one parsing reader in production code**,
  `bin/stage-economics.sh` itself (its own dedup grep), and **one asserting
  reader in the harness**, `drift-kit/smoke/install.sh`, which counts the log's
  lines. The path appears nowhere else but those two, the two SPEC copies of its
  knob entry, and queue prose — no *gate* reads it (it lives under the gitignored
  `DRIFT_KIT_METRIC_DIR`), and the `/economics` narrative reads it as prose, not
  by parsing the stage column. **`bin/trajectory.sh` does not parse this column
  at all** — the reader worth checking, since `trajectory-stage-roster-hardcode`
  was an entire iteration about a hardcoded stage roster; it reads stamps and its
  own `DRIFT_KIT_STAGES` roster, never this log. So a non-stage value in the
  column has **bounded blast radius** and cannot silently fall out of a roster
  the way the trajectory's stamps did.
- **What the smoke reader costs this delta.** `smoke/install.sh` asserts the
  stage-economics log holds **exactly one line** for one `(iteration, stage,
  model)` triple, both on first measure and after a re-measure. Those assertions
  survive only because every transcript in the existing fixture set sits on the
  **flat** tier, so no supervision row is derivable from it. The nested-tier
  fixture therefore gets **its own sessions dir, state file, and log**; adding it
  to the existing fixture set would emit a second row into the log those two
  assertions count and red the smoke.
- **No new field.** Delta B deliberately adds no log field: the four token
  fields, `cost`, and `date` carry the supervision row exactly as they carry a
  stage row, and their readers are already named in §The stage-economics meter.
  The apportionment key and the collision notice are **stdout caveats at
  measurement time**, not log fields — a log field with no reader is a field
  removed, and no reader was found for either.
- **Degradation** — a run with no nested-tier match (a stage run without a live
  lead) emits no supervision row and no notice: zero supervision burn is the
  honest reading, not a missing measurement. A lead transcript aged out of the
  sessions dir takes the existing unmatched-summary path.

## Existing sections updated

Each lands at merge in its proper canonical section; the docs mirror is a
generated projection of the first three.

- **`drift-kit/SPEC.md` §Bundled KPIs** — the lead list gains
  **kpi-price-table-age**: two rows off the consumer price table
  (`DRIFT_KIT_PRICE_TABLE`) — age in days of its `priced-as-of:` header, and
  time to (or past) its optional `prices-valid-through:` header — the staleness
  and expiry pressure gauges behind the meter's pricing honesty, carrying the
  one-line reason both exist (age inverts at an expiry cliff, reading freshest
  when the table is least true). Degrades `n/a` per row when the consumer has no
  table or lacks that header.
- **`drift-kit/SPEC.md` §The stage-economics meter** — input 3 (Price table)
  gains the two-field header contract (`priced-as-of:` required for the age row,
  `prices-valid-through:` optional) and names `kpi-price-table-age` as its
  reader; **The trend log** gains the reserved `supervision` stage value, the
  column's widened meaning (stage *or* cost-bearing role),
  the derivation-from-path producer, the attribution invariant, the
  apportionment key, the collision rule, and the one-reader survey bounding the
  widened column's blast radius; its field-readers sentence gains the
  supervision row's readers.
- **`drift-kit/SPEC.md` §Layout and configuration** — a knob entry for
  `DRIFT_KIT_SUPERVISION_LABEL` (default `supervision`), placed with the other
  stage-economics knobs. The layout block needs no new line: it already globs
  `kpis/kpi-*.sh`. `templates/price-table.tsv`'s comment there stays accurate.
- **`drift-kit/SPEC.md` §Testing** — the smoke's stage-economics fixture set
  gains (i) both headers on the fixture price table plus assertions on the age
  row, the `--trend` fragment, the no-header degradation of each row
  independently, and — the inversion this delta exists for — an expiry row
  reading EXPIRED on a fixture whose `priced-as-of:` is recent and whose
  `prices-valid-through:` has passed; (ii) a nested-tier fixture transcript (a synthetic
  `<lead>/subagents/<agent>.jsonl` beside a `<lead>.jsonl`) **in its own sessions
  dir, state file, and log** — the existing stage-economics fixture set is
  flat-tier and its log is asserted to hold exactly one line, so the supervision
  fixture must not share it — and assertions that
  exactly one supervision row is emitted for the iteration, that the label is
  the knob's value, and that a stamp naming the label suppresses the row with a
  notice.
- **`drift-kit/README.md`** — the prose enumeration of the bundled lead set
  ("…gate runtime, and session overhead") gains price-table age and expiry.
- **`drift-kit/templates/price-table.tsv`** — gains both header lines
  (`# priced-as-of:` and `# prices-valid-through:`, placeholder dates), each with
  a one-line note on what it dates and that `kpi-price-table-age` reads it. Any
  placeholder note about time-boxed pricing cites the header rather than
  restating a date, the shape the consumer file below must take.
- **`scripts/price-table.tsv`** — gains a `# prices-valid-through: 2026-08-31`
  header, and its `KNOWN CLIFF` prose block **drops the duplicated date while
  keeping the post-cliff per-token numbers**, citing the header for the date
  instead. Not optional and not cosmetic: without it the cliff date is
  maintained in two places the day it is mechanized, and the block's numbers have
  no other owner, so the block is edited, never deleted. **Two date literals, not
  one**: the block spells the same fact twice — `KNOWN CLIFF — 2026-09-01` in its
  heading and "in effect through 2026-08-31" in its first sentence. Both go; the
  header owns the date and the prose says "past the `prices-valid-through:`
  date". Dropping only the one the amendment named would leave the duplication
  the delta exists to remove.
- **`drift-kit/templates/kpis.list`** — registers `kpi-price-table-age` in the
  Lead block.
- **`drift-kit/templates/economics.md`** — the composed narrative gains the
  supervision line item beside the per-stage cost bullets, naming it as burn no
  stage stamp carries.
- **`scripts/kpis.list`** — this repo's registry, Lead block.
- **Generated projections, regenerated in the same unit** — four, not two; the
  last two were found at align by following `kpis.list`'s readers rather than the
  amendment's own list, and each is byte-gated, so a miss is a red pre-commit:
  - `docs/drift-kit/SPEC.md` and `docs/drift-kit/README.md`
    (`bash scripts/gen-docs-mirror.sh --write`, `check-docs-mirror-fresh`).
  - `docs/enforcement.md`
    (`bash gate-sdk/bin/enforcement-map.sh --emit > docs/enforcement.md`,
    `check-enforcement-fresh`) — a `kpis.list` change is a class-registry change.
  - `docs/footprint.md` (`check-footprint-fresh`, context-kit's) — drift-kit's
    load-triggered token cost moves because `drift-kit/SPEC.md` grows.
  - `docs/value.md`'s value-rollup block (`bash scripts/gen-value-rollup.sh`,
    `check-value-rollup-fresh`) — it joins the enforcement map's per-kit class
    counts to the footprint's token cost, so this unit moves it on **both** axes:
    drift-kit's Advisory-KPI count goes 10 → 11 and its token column grows.

**Reviewed and deliberately unchanged** — `lifecycle-kit/SPEC.md` §The state
machine: no new stamp, no cursor motion, no roster member. `bin/trajectory.sh`
reads stamps and its own `DRIFT_KIT_STAGES` roster, never this log's stage
column (surveyed above), so the widened column reaches it not at all.
`bin/stage-economics.sh`'s pricing arithmetic is untouched by Delta A — the KPI
reads the table's header, never its rows, so an expired table still prices,
loudly rather than silently.

## Cross-component reach (align-verified)

`check-stage-entry` assertion C fired on this amendment, naming four components
(`drift-kit lifecycle-kit site-kit docs/drift-kit`). The align audit resolved
each; the detector was **both over- and under-inclusive**, so this section is the
audited answer and supersedes any reference count.

- **drift-kit — the one contract owner.** Every contract this amendment changes
  is drift-kit's. No other kit's contract changes.
- **`docs/drift-kit` — not a component.** It holds a `SPEC.md`, which is what the
  detector's roster keys on, but it is a generated mirror of `drift-kit/`
  (`scripts/gen-docs-mirror.sh`), owning nothing. It is a regeneration
  obligation, listed above.
- **lifecycle-kit — the state-machine mention is a true negative, verified
  empirically.** Delta B's producer derives the supervisor from the transcript
  path, and the live layout was checked rather than assumed: this repo's sessions
  dir carries `<lead-uuid>/subagents/agent-<id>.jsonl` beside a flat
  `<lead-uuid>.jsonl`, so both tiers the derivation needs exist and the lead's own
  transcript is resolvable from the nested path's own directory component. No
  stamp, no cursor motion, no roster member — §The state machine stands.
  **But lifecycle-kit does hold one live gate over this unit**, which the
  reference count did not surface: `check-skill-binding` resolves whatever
  template a `.claude/commands/*.md` shim names and asserts the shim binds that
  template's exact slot set. `.claude/commands/economics.md` names
  `drift-kit/templates/economics.md` (today: one slot, `posture`; one binding).
  Delta B's edit to that template is prose within the existing slot — **it adds
  no `*<slot: …>*` and removes none**, or the shim gains the matching binding in
  the same commit.
- **site-kit — the cited section is a citation, and verified accurate.**
  §The monitor boundary does rule out exactly the network-fetching freshness
  gate Delta A declines to build. **The live reach is a different gate**:
  `check-docs-render-fidelity` covers every tracked docs page, `docs/drift-kit/SPEC.md`
  included, and this delta's prose is unusually dense in angle-bracket
  placeholders (`<N>`, `<date>`, `<TAB>`, `<agent>.jsonl`) — precisely the
  raw-non-HTML-element-tag symptom that gate reds on. Rendered through the pinned
  kramdown/GFM parser at align, every such token **escapes clean so long as it
  stays inside a code span**; that is the drafting rule for the merged sections,
  and it is the existing SPEC idiom already.
- **Reached but *not* flagged by the detector** — the repo-root and gate-sdk /
  context-kit projections above. The signal keys on `<component>/<contract-file>`
  references in the amendment's prose, so a surface this unit changes only
  *derivatively* (a rollup that counts `kpis.list` entries) is invisible to it.
  Following the registry's readers, not the prose's citations, is what found
  them.

## Definition of Done

- [ ] **Causal completeness** — both deltas have a named, reachable producer and
      a named consumer; the KPI's enabling config is emitted on every half
      (registry entry, and *both* headers in *both* the shipped template and this
      repo's table — the expiry row is dead wherever its header is missing); the
      supervision row's reader is the `/economics` narrative, edited in the same
      unit; no field is added without a reader.
- [ ] **One owner for the cliff date** — `scripts/price-table.tsv`'s prose block
      cites the `prices-valid-through:` header and restates no date; the
      post-cliff numbers survive the edit.
- [ ] **Envelope bounds honored** — the KPI never joins `gates.list`, the expiry
      header stays optional, and no third header field lands on this authority.
- [ ] **Merged with no information lost** — every addition integrated into its
      proper drift-kit/SPEC.md section (not appended); the merged spec reads as
      one coherent document for a reader who never saw this amendment.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      drift-kit (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — nothing is retired by this change; confirm by
      grep that no surface claims the price table is undated or that the
      economics log's stage column is roster-closed.
- [ ] **Ordering honored** — `stage-economics-attribution-honesty` (a) lands
      before or with Delta B.
- [ ] **All four projections regenerated in the unit** — docs mirror,
      `docs/enforcement.md`, `docs/footprint.md`, and `docs/value.md`'s rollup
      block; each is byte-gated, so the full battery is the oracle, not this list.
- [ ] **Cross-component constraints honored** — `check-skill-binding` stays green
      (the `economics.md` edit adds no slot, or the shim binds it), and every
      angle-bracket placeholder in the merged SPEC prose sits inside a code span
      so `check-docs-render-fidelity` stays green on the mirror.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
