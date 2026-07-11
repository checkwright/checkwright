# SPEC amendment: overhead-meter

## What changes

- `bin/overhead-meter.sh [transcript.jsonl]` — the methodology's own cost,
  measured so efficiency claims cut both ways: what fraction of a session's
  volume is governance (gate output, hook payloads, stage ritual, governed-doc
  reads) versus task work. Bare invocation resolves the newest transcript
  under `DRIFT_KIT_SESSIONS_DIR` (default
  `${CLAUDE_CONFIG_DIR:-$HOME/.claude}/projects/<cwd-slug>` — the derivation
  lifecycle-kit's `bin/session-id.sh` already applies; drift-kit re-derives
  with its own knob rather than importing a sibling kit's bin contract).
  Advisory by construction: exit always 0, never joins `gates.list`.
- The measurement contract, honesty first: **byte-proxy at line
  granularity**. Each JSONL transcript line is classified whole by a fixed
  marker table in the script (gate verdict shapes, hook/system-reminder
  blocks, stage-skill template loads, Read results on governed markdown —
  kit-name markers, mechanism not vocabulary) and its byte length lands in
  that category; everything unmatched is task work. Bytes are not tokens
  and a line is not a message — the claim this buys is *proportion across
  sessions of the same shape*, and every emitting surface carries that
  caveat parenthetical. No transcript content is ever written out: the
  meter emits counts and percentages only, and its log lives under the
  gitignored `$DRIFT_KIT_TMP_DIR` (the gate-timings precedent), so the
  private transcript stays private.
- Persistence: one line appended per measured session to
  `DRIFT_KIT_OVERHEAD_LOG` (default `$DRIFT_KIT_TMP_DIR/overhead-log.txt`),
  grammar `<date> <session8> total=<bytes> gov=<bytes> gate=<bytes>
  pct=<n>`. Field readers are named below; the per-category breakdown
  beyond `gate=` stays on the meter's stdout at measurement time (a log
  field with no reader is a field removed).
- `kpis/kpi-overhead.sh` — bundled KPI over the log, registered in the
  consumer's `kpis.list` (lead: act before overhead compounds). Full mode
  emits two rows: governance share (`pct` over the last N log lines, with
  session count and reading-age caveat) and gate-output share (`gate`/
  `total` — the axis the economy levers below target). `--trend` emits
  `ovh <pct>%`. Degrades fail-visible to "no measurement yet — run
  bin/overhead-meter.sh" when the log is absent.
- The economy levers, sequenced *behind* the meter (the queue entry's
  tension, ruled): **commit-first** (the generated hook already runs and
  prints the coupled gates, so a separate pre-battery run duplicates that
  output) and **failures-only run-gates output** (clean lines carry no
  decision value at 55-gate scale) are evaluation targets, not this
  unit's deliverables. Neither lands, and the CLAUDE.md battery wording
  does not change, until the meter shows gate-output share material over
  at least three measured sessions — the wording changes *with* the
  measurement, not ahead of it. Their design (a gate-sdk output-mode
  knob) is a future scope's amendment against that data.
- Testing: a synthetic fixture transcript under `smoke/` drives the
  classifier (known category bytes in, known percentages out) — the
  advisory-tool testing shape §Testing already establishes; no gate, no
  gate fixtures.

## Producers and consumers

- Producer of the log: this repo's close-stage binding
  (`.claude/commands/close.md`) gains a ritual step invoking the meter on
  the closing session — consumer config, not a lifecycle-kit change — and
  any session may invoke it ad hoc. Enabling config ships by default: both
  knobs carry working defaults, and the sessions-dir default matches the
  harness layout this repo already reads for stage stamps.
- Consumers of each log field: `kpi-overhead` reads `pct` (row 1), `gate`
  and `total` (row 2), `date` (the reading-age caveat), and the line count
  (the session-count caveat); `session8` is the dedup key the meter itself
  reads on append (re-measuring a session replaces its line rather than
  double-counting it).
- Consumer of the KPI rows: `drift-report.sh` full report and the
  session-context trend line — the existing plugin contract; no report
  skeleton change.

## Existing sections updated

- drift-kit/SPEC.md §Bundled KPIs — `kpi-overhead` joins the roster with
  its degradation row.
- drift-kit/SPEC.md §Layout and configuration — `DRIFT_KIT_SESSIONS_DIR`
  and `DRIFT_KIT_OVERHEAD_LOG` join the knob roster; `bin/overhead-meter.sh`
  joins the layout.
- drift-kit/SPEC.md §Testing — the synthetic-transcript smoke joins the
  advisory-tool testing shape.

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
