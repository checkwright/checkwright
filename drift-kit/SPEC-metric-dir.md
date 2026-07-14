# SPEC amendment: metric-dir

## What changes

`.tmp/` conflates disposable scratch (gate timings, validate logs, resume
journals) with append-only cross-session measurement trends, so a scratch
wipe destroys the trends. This amendment gives the trends a dedicated home
and fixes the writer/reader config-divergence defect found tracing it.

**New knob** (joins drift-kit/SPEC.md §Layout and configuration's roster):

- `DRIFT_KIT_METRIC_DIR` — the persistent measurement home, distinct from
  `DRIFT_KIT_TMP_DIR` by retention contract: metric-dir members are
  append-only trend logs that survive scratch wipes; tmp-dir members are
  regenerated on every run. Default `.metric`. The dir must be gitignored
  and never committed — trend samples carry account identifiers
  (usage-history's `account=` field) and per-session refs, so committing it
  publishes them (this repo's provenance seam makes the gitignore
  load-bearing, but the retention/privacy contract is kit-generic).
- `DRIFT_KIT_OVERHEAD_LOG` — default moves from
  `$DRIFT_KIT_TMP_DIR/overhead-log.txt` to
  `$DRIFT_KIT_METRIC_DIR/overhead-log.txt`, in all three resolvers
  (`bin/overhead-meter.sh`, `kpis/kpi-overhead.sh`, and the collator's
  export). `overhead-meter.sh` already `mkdir -p`s the log's dirname.

**The divergence fix — export the namespace, not a list.** The attested
defect: `drift-report.sh` sources the consumer config (so
`DRIFT_KIT_OVERHEAD_LOG` is *set* in its shell) but exports a fixed
`DRIFT_KIT_*` list that omits it, so a plain-assignment consumer override
reaches the writer (`overhead-meter.sh` sources the config directly) but
never the reader (`kpi-overhead.sh` reads only inherited env) — silently,
the KPI being advisory. Rather than gate the list into parity,
`drift-report.sh` exports **every scalar `DRIFT_KIT_*` variable** (compgen
over the namespace, arrays skipped — bash cannot export them, and array
knobs like `DRIFT_KIT_KPI_DIRS` are consumed inside the collator itself),
removing the fixed-list/namespace duplication structurally
(enforcement-first: removing the duplication outranks gating it). The KPI
plugin contract is unchanged: plugins read exported env only.

**The class oracle:** a drift-kit `smoke/` assertion runs the writer and the
reader under one config override and asserts both resolve the same log path
— the writer/reader-divergence scanner the queue task files within this
unit. It guards the surviving divergence surface the namespace export
cannot: writer and reader computing *defaults* independently.

**Consumer-side worklist (this repo), landing with the kit change:**

- `scripts/delegation-config.sh`: `DELEGATION_KIT_USAGE_HISTORY` →
  `.metric/usage-history.log` (verified: writer `usage-verdict.sh
  append_sample` and reader `usage-trend.sh` both resolve the knob through
  `lib/delegation.sh`'s config loader, and the writer `mkdir -p`s — a value
  change moves both coherently; no delegation-kit code change).
- Migrate the two live logs by `mv` (gitignored — plain rename preserves the
  data): `.tmp/overhead-log.txt` and `.tmp/usage-history.log` → `.metric/`.
- `.gitignore` gains `.metric/`.
- CLAUDE.md §Housekeeping: `.metric/` is gitignored persistent
  account-bearing measurements, never committed; `.tmp/` becomes purely
  disposable, wiped at the scope boundary.
- The scope skill's binding gains the deferred hygiene line this split
  unblocks: wipe `.tmp/`'s files at the iteration boundary (safe once
  nothing persistent lives there; a stale resume journal from a closed
  iteration is dead by definition).
- `scripts/bash-guard.sh`'s `git clean -x/-X` guard message names `.metric/`
  (the trend logs are now the irreplaceable loss, ahead of resume journals);
  the guard's `spec:` tag cites CLAUDE.md §Housekeeping and its text rides
  the same edit.

`DRIFT_KIT_TIMINGS_FILE` (gate timings) **stays** in `DRIFT_KIT_TMP_DIR` —
regenerated every `run-gates`, a wipe is harmless.

## Producers and consumers

- **`DRIFT_KIT_METRIC_DIR`** — producer: consumer config
  (`scripts/drift-config.sh` or exported env; the kit default `.metric`
  covers the unconfigured consumer). Consumers: `overhead-meter.sh` and
  `kpi-overhead.sh` (via the `DRIFT_KIT_OVERHEAD_LOG` default),
  `drift-report.sh` (namespace export). No other reader; delegation-kit
  reaches `.metric/` only through this repo's explicit
  `DELEGATION_KIT_USAGE_HISTORY` value, not through the knob.
- **The namespace export** — producer: `drift-report.sh` at startup;
  consumers: every registered KPI plugin (the existing exported-env
  mechanism, now fed the whole scalar namespace).
- **The smoke assertion** — producer: drift-kit's smoke runner (this repo's
  validate battery); consumer: the validate session reading a red suite.

## Existing sections updated

- drift-kit/SPEC.md §Layout and configuration — `DRIFT_KIT_METRIC_DIR`
  joins the roster; `DRIFT_KIT_OVERHEAD_LOG`'s stated default changes.
- drift-kit/SPEC.md §The KPI plugin contract — the export prose (today only
  the `DRIFT_KIT_KIT_ROOTS` handoff sentence; the plugins-read-exported-env
  rule is implicit) gains the scalar-namespace export statement.
  §The report skeleton carries no export sentence and needs none.
- drift-kit/SPEC.md §The overhead meter — log-home prose follows the new
  default.
- drift-kit/README.md — config example mentions the metric dir.
- delegation-kit/SPEC.md §Layout and configuration — the
  `DELEGATION_KIT_USAGE_HISTORY` bullet's "This repo sets
  `.tmp/usage-history.log`" example follows the move to `.metric/`.
- CLAUDE.md §Housekeeping (consumer prose, above).
- Docs-mirror regeneration rides the SPEC edits.

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
      retired (the old `.tmp` log paths); nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
