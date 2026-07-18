# SPEC amendment: stage-economics

Delta for the `stage-economics-report` queue task: a tracked drift-kit tool
`bin/stage-economics.sh` that prices lifecycle spend by stage × model ×
iteration, plus the `/economics` close-cadence skill that chains the three
drift-kit/delegation-kit reporting tools into one post-iteration narrative.
Merges into `drift-kit/SPEC.md` and is deleted on completion. It states only
the delta — the drift-kit tool conventions (advisory exit-0, `DRIFT_KIT_`
config-via-env, `DRIFT_KIT_METRIC_DIR` retention/privacy) are §The overhead
meter's and §Layout and configuration's, cited not restated.

## What changes

### The stage-economics meter (new §, sibling to §The overhead meter)

`bin/stage-economics.sh` answers the one question no built-in surface prices:
**real spend by lifecycle stage × model × iteration**. The overhead meter
measures governance-versus-task *bytes* and delegation-kit's usage-trend the
rate-window *percentage*; neither converts a stage's token draw into money. This
tool does, so the operator can see close-over-close whether the posture that
rides every stage on Opus (the current all-stages-Opus posture recorded in the
lead command's ruling-config binding, not a scope-only choice) earns its cost.

It is advisory by the same contract as the overhead meter (§The overhead meter):
exit is always 0, it never joins `gates.list`, and a missing input is a 0-exit
notice, not a failure. It writes only under `DRIFT_KIT_METRIC_DIR` (the
gitignored, account-bearing persistent home) and **emits no account
identifiers** — the trend log carries stage, model, iteration, token counts, and
priced cost, never the account the tokens billed to.

**The join.** Three inputs, joined on the session:

1. **Stamps** — the WORKFLOW-STATE data lines, one per stage-skill invocation,
   grammar `<iteration> <stage> <session8> <date>` (owned by
   lifecycle-kit/SPEC.md §The state machine; this tool is a read-only consumer of
   that contract, it changes nothing there). The stamp supplies the
   iteration↔stage↔session8 mapping. Read from `DRIFT_KIT_STATE_FILE` (new knob
   below), defaulting to the same state-file path the trajectory extractor
   already reads (§Layout).
2. **Transcripts** — under `DRIFT_KIT_SESSIONS_DIR` (the knob the overhead meter
   already resolves). Each stamp's `session8` selects the transcript whose id
   shares that 8-char prefix; the tool sums that session's assistant-turn usage
   into four token categories — `input`, `output`, `cache_read`,
   `cache_creation` — per model id seen on those turns. Because this repo runs
   one session per stage (lifecycle-kit/SPEC.md §The state machine,
   `LIFECYCLE_KIT_SESSION_BOUNDARY=stage`), a session maps to exactly one stage,
   so per-session usage *is* per-stage usage. Under an `iteration`-boundary
   consumer a session may span stages; the tool attributes such a session to the
   stage its stamp names and says so in the emitted caveat — the honest limit,
   parallel to the overhead meter's byte-proxy caveat.
3. **Price table** — a consumer-supplied data file mapping model id → per-token
   price for each of the four categories. This is **consumer config, never a kit
   literal** (the provenance seam, the `check-graph`/`graph-vocab` pattern): the
   per-token prices are public facts, but the *roster of models a consumer runs*
   is theirs, and a kit literal enumerating it would publish that roster. The kit
   ships `templates/price-table.tsv` with placeholder rows and the column schema;
   the consumer copies it and fills their roster. Resolved from
   `DRIFT_KIT_PRICE_TABLE` (new knob).

**Degradation.** A price table that is absent, or that has no row for a model the
transcripts name, degrades that model's cost cell to `n/a` and the tool emits the
token counts alone — the same degradation contract the trajectory extractor
applies to an unreadable surface (§Bundled KPIs / §Layout). Cost is additive over
priced cells only; an `n/a` cell never poisons the total silently — the total
carries an "incomplete pricing" caveat when any contributing cell degraded.

**The trend log.** One line is appended per `(iteration, stage, model)` triple to
`DRIFT_KIT_STAGE_ECONOMICS_LOG`, grammar:

```
<date> <iteration> <stage> <model> in=<tok> out=<tok> cr=<tok> cw=<tok> cost=<usd|n/a>
```

`cr` is cache-read and `cw` is cache-creation. `cr` is the headline field: the
motivating dig showed cache-read of accumulated context — not model choice — is
the dominant burn (build ~73% of session cost, climbing 37M→86M cr-tokens per
session), so the field exists to keep that lever visible close-over-close. The
dedup key read on append is the `<iteration> <stage> <model>` triple —
re-measuring a triple replaces its line rather than double-counting, exactly as
the overhead meter dedups on `session8`. Any per-model sub-breakdown beyond these
fields stays on stdout at measurement time; a log field with no reader is a field
removed.

### The `/economics` close-cadence skill (new)

`/economics` is the customer-facing post-iteration narrative: run at close, it
chains `bin/overhead-meter.sh` → `bin/stage-economics.sh` →
delegation-kit's usage-trend (delegation-kit/SPEC.md §The staleness contract owns
the usage surface; the skill is a read-only caller) into one report answering
"what did this iteration cost, where, and was the model posture worth it". It
ships as a drift-kit skill template `templates/economics.md`, materialized in this
repo as the consumer copy `.claude/commands/economics.md` — the same
template↔consumer-copy split the guard/hook skills use. It is not a lifecycle
stage (it moves no cursor, stamps nothing) and so is outside
`check-stage-skill-coverage`'s stage roster; it is a reporting ritual the close
skill may invoke, never a gate.

## Producers and consumers

**stage-economics trend log** (`DRIFT_KIT_STAGE_ECONOMICS_LOG`):
- **Producer** — `bin/stage-economics.sh`, invoked by the `/economics` skill at
  close and ad hoc by any session. Enabling config: the `DRIFT_KIT_` knobs all
  carry working defaults (this repo's layout), so the tool runs on a bare
  invocation; the one input the consumer must supply is the price table, and its
  absence degrades to token-only output rather than failing — so nothing is
  dead-on-arrival for a consumer who has not yet authored a roster.
- **Consumers** — (1) the `/economics` skill narrative, which reads `cost` and the
  four token fields to compose the close report; (2) the operator reading that
  report close-over-close for the cache-read lever; (3) the future
  `benchmark-ab-experiment` rung, whose measurement half consumes this log rather
  than rebuilding it (that queue entry is already repointed at this tool).
- **Field readers** — `iteration`,`stage`,`model` are the narrative's grouping
  keys and the dedup triple; `in`/`out`/`cr`/`cw` are read by the narrative
  (`cr` headline) and by benchmark's per-stage burn axis; `cost` is read by the
  narrative and the operator; `date` carries the reading-age caveat, as it does
  for the overhead log. No field lacks a reader.

**price table** (`DRIFT_KIT_PRICE_TABLE`):
- **Producer** — the consumer, by copying `templates/price-table.tsv` and filling
  their model roster (consumer config; the kit ships only the placeholder
  template and schema).
- **Consumer** — `bin/stage-economics.sh`'s pricing step, one lookup per
  `(model, category)`.

## Existing sections updated

drift-kit/SPEC.md changes at merge — all additive, each into its proper section:

- **New §The stage-economics meter**, placed as a sibling after §The overhead
  meter, carrying the join, degradation, and trend-log grammar above.
- **New §The `/economics` skill** (or a subsection of the meter §) naming the
  chain, the template↔consumer-copy split, and its non-stage / non-gate status.
- **§Layout and configuration** — the file tree gains
  `bin/stage-economics.sh`, `templates/price-table.tsv`, and
  `templates/economics.md`; the knob roster gains:
  - `DRIFT_KIT_STAGE_ECONOMICS_LOG` — default
    `$DRIFT_KIT_METRIC_DIR/stage-economics-log.txt` (gitignored; the meter
    `mkdir -p`s the dirname), the append trend log.
  - `DRIFT_KIT_PRICE_TABLE` — default `${GATE_SDK_GATES_DIR:-scripts}/price-table.tsv`
    (beside `graph-vocab.sh`, the consumer-config precedent); the consumer-owned
    model→price roster.
  - `DRIFT_KIT_STATE_FILE` — the WORKFLOW-STATE path the join reads; default
    `${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt` (the same default the
    trajectory extractor's surface list already computes — drift-kit re-derives
    with its own knob rather than importing a sibling kit's bin contract, the
    established `DRIFT_KIT_SESSIONS_DIR` precedent).
- **§The overhead meter** — a one-line sibling cross-reference to the new meter
  (they share the `DRIFT_KIT_METRIC_DIR` retention contract and the advisory
  exit-0 posture); no restatement.
- **§Testing** — a smoke driving the join over a synthetic fixture set: a small
  WORKFLOW-STATE stamp file, a synthetic transcript carrying usage records for
  the stamped `session8`, and a placeholder price table, asserting the emitted
  trend line's fields and the degradation caveat when a model has no price row.
  Follows the `smoke/overhead-fixture.jsonl` precedent.

No other component's SPEC changes: the tool is a read-only consumer of
lifecycle-kit's stamp contract and delegation-kit's usage surface, and adds no
name to either. The `/economics` consumer copy lands in `.claude/commands/`
(harness config, not a kit roster dir).

## Definition of Done

- [ ] **Causal completeness** — the trend log has a named, reachable producer
      (`bin/stage-economics.sh` via `/economics` at close, defaults ship it live
      bar the consumer-supplied price table whose absence degrades not fails) and
      named consumers (the `/economics` narrative, the operator, the future
      benchmark rung); every log field has a named reader at a named read.
- [ ] **Merged with no information lost** — the new meter §, the `/economics` §,
      and the Layout/knob/Testing additions integrated into `drift-kit/SPEC.md`
      in their proper sections (not appended); the merged spec reads coherently
      for a reader who never saw this amendment.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change retires
      (none expected; the change is purely additive); nothing dangles.
- [ ] **Gaps filed** — any cross-component causal gap discovered during build is
      resolved that session and its spec updated before resuming, not deferred;
      a discovered debt goes to the queue.
- [ ] **Provenance seam held** — the shipped price-table template carries only
      placeholder rows and the column schema; no model roster is a kit literal.
- [ ] **Privacy held** — the trend log and stdout carry no account identifier;
      `DRIFT_KIT_METRIC_DIR` stays gitignored.
