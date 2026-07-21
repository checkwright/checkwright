# SPEC amendment: price-table-age-kpi

<!-- Owning component: drift-kit (owns the KPI plugin contract, the bundled set,
     and the stage-economics meter). Single-component: nothing here changes
     lifecycle-kit's state machine, its stamp grammar, or any other kit's
     contract — the meter stays a read-only consumer of the stamp line.
     Pairs with the queue entry tagged [spec: SPEC-price-table-age-kpi.md].

     Two deltas, one envelope. Delta A is the named feature. Delta B is the
     governed name that stage-economics-attribution-honesty defect (c) needs if
     supervision is its own row — ruled here rather than in a second amendment
     (lead ruling at promotion, recorded in that entry's body). Both are
     drift-kit economics-honesty contract additions; neither can land without
     the other's surfaces being touched (§The stage-economics meter carries
     both). -->

## What changes

### Delta A — `kpi-price-table-age`, an advisory staleness signal

A new bundled lead KPI reports **how old the price table's own dating is**, so a
stale table surfaces as a trend signal instead of silently mispricing every
economics read.

- **The plugin** — `kpis/kpi-price-table-age.sh`, the `kpi-deferred-age`
  age-KPI idiom exactly: full mode emits one lead row, `--trend` emits one
  compact fragment or nothing, exit is always 0.
  - Full: `lead<TAB>price table age<TAB>priced <N>d ago (as-of <date>)`.
  - `--trend`: `price <N>d`, and nothing at all when the value is `n/a`.
- **The surface it reads** — the price table already resolved by
  `DRIFT_KIT_PRICE_TABLE` (§Layout and configuration). No new knob: the plugin
  reads the same consumer-config path the meter prices through, so the table has
  one owner and one address.
- **The dating contract** — the first line of that file matching
  `^#[[:space:]]*priced-as-of:[[:space:]]*(\d{4}-\d{2}-\d{2})` supplies the
  date; trailing prose on the same line is ignored. This promotes a comment
  convention to a **governed header name** (`priced-as-of:`) — the litmus that
  makes this unit a feature.
- **Degradations**, each a visible `n/a (<reason>)` in the plugin's own value,
  never a failure and never a block: `n/a (no price table)`,
  `n/a (no priced-as-of: header)`, `n/a (unparseable priced-as-of date)`.
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
  - the *header* — `# priced-as-of: <YYYY-MM-DD>` in the table itself.
    `scripts/price-table.tsv` already carries it (line 5 of its header block),
    so this repo reports a real value the day the plugin lands; but
    `templates/price-table.tsv` does **not**, so without the template edit the
    KPI is `n/a` in every consumer that ever copies it — the dead-producer shape
    the causal-completeness check exists to catch. The template gains the header
    line with a placeholder date and a one-line note on what it dates.
- **Consumers** — `drift-report.sh`'s lead section (full mode) and, through
  `--trend`, context-kit's session-context hook via `CONTEXT_KIT_DRIFT_REPORT`
  (already wired; a new registry member needs no context-kit change).
- **Every field has a named reader.** The plugin emits no new record type — it
  emits a row on the existing two-field KPI row contract. Its two data points:
  `<N>d` read by the operator as the trend level and by the session-context
  trend line via the `price <N>d` fragment; `(as-of <date>)` read by the
  operator as the caveat identifying *what* aged, exactly as `kpi-deferred-age`
  carries `(Surfaced <date>)`. No other field is added.

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
  **kpi-price-table-age**: age in days of the `priced-as-of:` header in the
  consumer price table (`DRIFT_KIT_PRICE_TABLE`), the staleness pressure gauge
  behind the meter's pricing honesty; degrades `n/a` when the consumer has no
  table or no header.
- **`drift-kit/SPEC.md` §The stage-economics meter** — input 3 (Price table)
  gains the `priced-as-of:` header contract and names `kpi-price-table-age` as
  its reader; **The trend log** gains the reserved `supervision` stage value,
  the derivation-from-path producer, the attribution invariant, the
  apportionment key, and the collision rule, and its field-readers sentence
  gains the supervision row's readers.
- **`drift-kit/SPEC.md` §Layout and configuration** — a knob entry for
  `DRIFT_KIT_SUPERVISION_LABEL` (default `supervision`), placed with the other
  stage-economics knobs. The layout block needs no new line: it already globs
  `kpis/kpi-*.sh`. `templates/price-table.tsv`'s comment there stays accurate.
- **`drift-kit/SPEC.md` §Testing** — the smoke's stage-economics fixture set
  gains (i) a `priced-as-of:` header on the fixture price table plus assertions
  on the KPI's parsed row, its `--trend` fragment, and its no-header
  degradation; (ii) a nested-tier fixture transcript (a synthetic
  `<lead>/subagents/<agent>.jsonl` beside a `<lead>.jsonl`) and assertions that
  exactly one supervision row is emitted for the iteration, that the label is
  the knob's value, and that a stamp naming the label suppresses the row with a
  notice.
- **`drift-kit/README.md`** — the prose enumeration of the bundled lead set
  ("…gate runtime, and session overhead") gains price-table age.
- **`drift-kit/templates/price-table.tsv`** — gains the
  `# priced-as-of: <YYYY-MM-DD>` header line (placeholder date) with a one-line
  note that `kpi-price-table-age` reads it.
- **`drift-kit/templates/kpis.list`** — registers `kpi-price-table-age` in the
  Lead block.
- **`drift-kit/templates/economics.md`** — the composed narrative gains the
  supervision line item beside the per-stage cost bullets, naming it as burn no
  stage stamp carries.
- **`scripts/kpis.list`** — this repo's registry, Lead block.
- **Generated projections, regenerated in the same unit:**
  `docs/drift-kit/SPEC.md` and `docs/drift-kit/README.md`
  (`bash scripts/gen-docs-mirror.sh --write`, `check-docs-mirror-fresh`
  byte-gates them) and `docs/enforcement.md`
  (`bash gate-sdk/bin/enforcement-map.sh --emit > docs/enforcement.md`,
  `check-enforcement-fresh` byte-compares it) — a `kpis.list` change is a
  class-registry change.

**Reviewed and deliberately unchanged** — `scripts/price-table.tsv` needs no
edit: it already carries a `priced-as-of:` line in the grammar above, and its
`KNOWN CLIFF` prose block stays prose (no kit mechanism parses it).
`lifecycle-kit/SPEC.md` §The state machine is untouched: no new stamp, no
cursor motion, no roster member. `bin/trajectory.sh` reads stamps, not this
log, and is unaffected.

## Definition of Done

- [ ] **Causal completeness** — both deltas have a named, reachable producer and
      a named consumer; the KPI's two enabling-config halves (registry entry and
      shipped-template header) are both emitted; the supervision row's reader is
      the `/economics` narrative, edited in the same unit; no field is added
      without a reader.
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
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
