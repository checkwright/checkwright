# drift-kit — drift reporting with pluggable KPIs and lead/lag honesty labels

Gates block what is mechanically decidable per commit; drift is what
accumulates *between* commits — trends no single diff violates. A backlog
ages, an always-loaded surface swells, friction recurs, and every individual
session looks fine, because no stateless session ever sees the slope. The
kit is the trend surface: an advisory report that collates KPIs from the
other kits' governed surfaces, groups them under honest weight labels
(lead indicators act before drift compounds; lag indicators undercount by
construction), and emits a one-line trend summary the session-start hook
injects — so every session opens seeing the slope it cannot otherwise see.

Advisory by construction: the report never fails a session, never joins
`gates.list`, and reads *trend, not level* — a KPI's absolute value is
noise, its direction across sessions is the signal.

Extracted from the governance meta-layer of a private production platform.
The kit carries the report skeleton, the KPI plugin contract, a bundled set
of kit-coupled generic KPIs, and the knowledge-friction loop; the
platform's product- and toolchain-shaped KPIs stay behind (§What stayed on
the platform).

## The report skeleton

`bin/drift-report.sh` — the collator. It owns the frame; every measurement
lives in a plugin:

1. Read the KPI registry (`kpis.list` — one plugin name per line, `#`
   comments; the gates.list grammar) and resolve each name against the
   consumer KPI dir first, then each vendored kit's `kpis/` (the gate-sdk
   resolution pattern: a consumer shadows a bundled KPI by dropping a
   same-named file in its own dir).
2. Run each plugin, collect its rows, and group them into the two labeled
   sections — the honesty labels are the frame's contract:
   - header: `=== Drift KPIs (advisory — trend, not level) ===`, plus the
     iteration-start commit when derivable;
   - `--- Lead (weighted high — act before drift compounds) ---`;
   - `--- Lag (weighted low — undercounts by construction) ---`;
   - footer: `Read trend across sessions; lag KPIs lower-bound only.`
3. `--trend` emits a single compact line instead — the fragment each plugin
   volunteers, joined with `·` — consumed by context-kit's session-context
   hook (`CONTEXT_KIT_DRIFT_REPORT`, already wired in its template).

Degrade discipline: exit is always 0. A plugin that exits non-zero or
prints nothing yields a visible `<name>  n/a (plugin failed)` row in the
lead section — fail-visible, not fail-closed, because a silently vanishing
KPI is itself drift. A plugin whose *surface* is missing (no log yet, no
timings file) degrades to `n/a (<reason>)` in its own value, not by dying.

## The KPI plugin contract

A plugin is `kpi-<name>.sh`, resolved through the registry. Two modes:

- **Full (no args)** — one or more rows on stdout, each
  `lead|lag<TAB><label><TAB><value>`: the section tag, a short human label,
  and a freeform value that carries its own caveat parenthetical (reading
  age, undercount note, pointer to the acting close-stage step). Multiple
  rows are legitimate for one measurement with two axes.
- **`--trend`** — at most one compact `<key> <value>` fragment, or nothing
  (a plugin may opt out of the trend line).

Plugins never block and never write outside `$DRIFT_KIT_TMP_DIR`-style
scratch; a measurement needing state (a baseline, a log) reads a file some
*other* mechanism owns and stamps its reading-age caveat when the file is a
past measurement rather than live state (the gate-runtime pattern below).

## Bundled KPIs

The generic set — each coupled to a kit-governed surface, each degrading to
`n/a` when the consumer lacks that surface. Lead:

- **kpi-task-split** — the feature↔debt split of the queue's Done slugs,
  classified by their commit subjects (`feat` / `fix`+`refactor`); reads
  the queue file and git.
- **kpi-gate-backlog** — proposed-but-absent gates: `check-*`/`scan-*`
  names appearing anywhere in the queue with no file in any gate-resolution
  dir, over the live gate count. A name with a file on disk is built and
  drops out of the numerator.
- **kpi-amendment-age** — age in days of the oldest amendment on disk
  (`SPEC-*.md`, git add-date); the pressure gauge behind spec-kit's
  short-lived-amendment rule.
- **kpi-deferred-age** — age of the oldest `Surfaced <date>` mark in the
  queue's deferred section: premise-rot pressure on design-pending work.
- **kpi-prompt-friction** — distinct/total prompting calls via guard-kit's
  `scan-prompts.sh --count`; `n/a` when guard-kit or its log is absent.
- **kpi-always-loaded** — the standing per-session surface: level and
  since-baseline delta via context-kit's `always-loaded.sh` meter.
- **kpi-settings-local** — entry count of the untracked local permission
  overlay (`.claude/settings.local.json` allow/deny/ask, via `jq`); the
  notice signal for guard-kit's close-stage prune/promote step.
- **kpi-gate-runtime** — full-battery runtime from the runner's timings
  file (`<tmp-dir>/gate-timings.txt`): total, the three slowest gates, and
  the file's reading age — a *measurement*, not live state, so the age
  caveat rides the value.

Lag:

- **kpi-knowledge-friction** — re-derivations captured this iteration: the
  line count of the knowledge-friction log (§The knowledge-friction loop).
  Lag by construction: only what a session *noticed and logged* is counted,
  so the value lower-bounds the real rate.

The lag section is expected to be sparse — most lag measurements (review
finding rates, detection latency) are manual tallies, and the kit ships no
fake automation for them; a consumer with a structured source adds its own
plugin.

## The knowledge-friction loop

guard-kit's friction loop catches *permission* friction — its log is fed by
a hook that fires on every command. Knowledge friction has no hook: a
session re-derives a fact no doc owns (reads it off an implementation, a
gate's source, a commit message), gets the right answer, and moves on —
nothing prompts, nothing logs, and the next session re-pays the same
derivation. The loop mirrors guard-kit's, with capture moved to convention:

1. **Capture (any session)** — the moment a session catches itself
   re-deriving a fact from a non-owning surface, it appends one line to the
   knowledge-friction log (`.workflow/knowledge-friction.log` by default;
   gitignored per-iteration scratch, the prompt-friction.log pattern):
   `<date> <fact re-derived> ← <surface it was read from>`. One line,
   written at the moment of re-derivation — deferred capture is no capture.
   The convention costs one always-loaded bullet in the consumer's
   instructions file; that line is the loop's hook and must earn its
   recurring cost by the log actually filling.
2. **Triage (close)** — `templates/close-knowledge.md`, spliced into the
   consumer's close skill (the close-triage/close-brevity pattern): walk
   the log; for each entry, the remediation is a **doc-owner edit** — give
   the fact a home under the consumer's tier contract (spec-kit's star
   topology: one owner per fact), or a pointer from where the session
   looked to where the owner is. Never a standing session-start
   instruction: that converts one re-derivation into a permanent
   per-session tax, exactly what context-kit's brevity machinery rejects.
   Then clear the log — its named reclaim path.
3. **Aggregate (drift)** — `kpi-knowledge-friction` trends the per-iteration
   count; it falls as the tier contract's holes fill. Detection is the
   loop; elimination is a tiering edit.

The heavy alternative — periodic LLM-scan of session transcripts reduced to
each party's messages — is deliberately out of kit scope: it needs harness
transcript access no kit mechanism owns.

## Layout and configuration

```
drift-kit/
  bin/drift-report.sh
  kpis/kpi-*.sh                  # the bundled generic set
  templates/drift-config.sh
  templates/kpis.list            # example registry (consumer copies + prunes)
  templates/close-knowledge.md
  smoke/install.sh
```

Registers no gates (advisory; the guard-kit precedent), so no `checks/`,
`gate-tests/`, or `smoke/violation.sh`.

Config follows the established kit pattern: copy `templates/drift-config.sh`
into the gates dir (or point `DRIFT_KIT_CONFIG_FILE` elsewhere) and override
any knob; defaults fill what the consumer left unset. Knobs (platform
values as defaults):

- `DRIFT_KIT_KPIS_FILE` — the registry; default
  `${GATE_SDK_GATES_DIR:-scripts}/kpis.list`.
- `DRIFT_KIT_KPI_DIRS` — extra resolution roots searched before the
  vendored kits' `kpis/` dirs; default: the consumer gates dir.
- `DRIFT_KIT_QUEUE_FILE` — default `${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}`.
- `DRIFT_KIT_KNOWLEDGE_LOG` — default
  `${GATE_SDK_WORKFLOW_DIR:-.workflow}/knowledge-friction.log`.
- `DRIFT_KIT_TIMINGS_FILE` — default
  `${GATE_SDK_TMP_DIR:-.tmp}/gate-timings.txt`.
- `DRIFT_KIT_DONE_SECTION` / `DRIFT_KIT_DEFERRED_SECTION` — queue section
  headings the task-split and deferred-age KPIs scan; defaults `Done` /
  `Deferred` (queue-kit's).

Per-KPI couplings (which meter, which log, which scan flag) are the
plugins' own headers, not knobs — a consumer retargeting one edits its copy
in the consumer dir (registry shadowing), the guard/hook consumer-copy
precedent.

## Testing

The report and every bundled plugin are advisory plain text over live git
state — not fixture-stable, so no expected-output corpus (the gate
contracts do not fit; context-kit's reasoning). `smoke/install.sh` builds
the throwaway consumer, registers the bundled set, and asserts: exit 0 with
both section headers and one row per registered KPI; a registry naming a
missing plugin yields its visible `n/a` row without failing; `--trend`
emits exactly one line. Gate-sdk's `check-shellcheck` lints all kit
sources as usual.

## What stayed on the platform

The platform's toolchain-shaped KPIs: orphan-crate and bare-`#[allow]`
scans (Rust-specific dead-surface detection) and the `TODO(spec-ambiguity)`
marker count (its marker convention is platform vocabulary; extract the
convention first if it ever generalizes). Its product-workflow KPIs: gate
exemptions (`scan-exceptions` disposition split) and backlog-aging finding
counts — both read platform gates that were not extracted. The
narration-marker by-eye count: superseded here by spec-kit's
`check-manifest-temporal`, which gates the same axis instead of trending
it; the platform keeps its by-eye copy until it adopts the gate. The
always-loaded baseline mechanics moved to context-kit in kit 6; the
platform's report keeps its inline copy until migration.
