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

The kit carries the report skeleton, the KPI plugin contract, a bundled set
of kit-coupled generic KPIs, and the knowledge-friction loop; a consumer's
product- and toolchain-shaped KPIs stay in the consumer repo (§Out of scope).

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

A plugin is `kpi-<name>.sh`, resolved through the registry and invoked
directly (`"$path"`, not `bash "$path"`) — so it must carry the execute bit or
the report degrades that row to its fail-visible read. Two modes:

- **Full (no args)** — one or more rows on stdout, each
  `lead|lag<TAB><label><TAB><value>`: the section tag, a short human label,
  and a freeform value that carries its own caveat parenthetical (reading
  age, undercount note, pointer to the acting close-stage step). Multiple
  rows are legitimate for one measurement with two axes.
- **`--trend`** — at most one compact `<key> <value>` fragment, or nothing
  (a plugin may opt out of the trend line).

The report exports `DRIFT_KIT_KIT_ROOTS` — newline-separated kit roots,
`gate_kit_roots` when gate-sdk resolves, else the kit's parent — before
invoking plugins; a plugin needing sibling-kit surfaces reads it rather than
re-deriving the roster, and falls back to its own derivation when run
standalone without it. The driver's handoff, not a consumer knob:
`drift-report.sh` recomputes it every run.

Plugins never block and never write outside `$DRIFT_KIT_TMP_DIR` scratch;
a measurement needing state (a baseline, a log) reads a file some
*other* mechanism owns and stamps its reading-age caveat when the file is a
past measurement rather than live state (the gate-runtime pattern below).

## Bundled KPIs

The generic set — each coupled to a kit-governed surface, each degrading to
`n/a` when the consumer lacks that surface. Lead:

- **kpi-task-split** — the feature↔debt split of the queue's Done slugs,
  classified by their commit subjects (`feat` / `fix`+`refactor`); reads
  the queue file and git. gate-sdk's check-commit-subject is the parse
  guarantee this classification assumes — every subject carries a roster
  type, so a mistyped prefix is a blocked commit, not a miscounted row.
- **kpi-gate-backlog** — proposed-but-absent gates: `check-*`/`scan-*`
  names appearing anywhere in the queue with no file in any gate-resolution
  dir, over the live gate count. A name with a file on disk is built and
  drops out of the numerator.
- **kpi-amendment-age** — age in days of the oldest amendment on disk
  (`SPEC-*.md`, git add-date; fixture and template paths excluded, matching the
  published-evidence extractor's amendment-latency harvest); the pressure gauge
  behind canon-kit's short-lived-amendment rule.
- **kpi-deferred-age** — age of the oldest `Surfaced <date>` mark
  (queue-kit's ungated convention) in the queue's deferred section:
  premise-rot pressure on design-pending work.
- **kpi-prompt-friction** — distinct/total prompting calls via guard-kit's
  `scan-prompts.sh --count`; `n/a` when guard-kit or its log is absent.
- **kpi-always-loaded** — the standing per-session surface: level and
  since-baseline delta via context-kit's `always-loaded.sh` meter.
- **kpi-settings-local** — entry count of the untracked local permission
  overlay (`.claude/settings.local.json` allow/deny/ask, via `jq`); the
  notice signal for guard-kit's close-stage prune/promote step.
- **kpi-gate-runtime** — full-battery runtime from the runner's timings
  file (`<tmp-dir>/gate-timings.txt`): total, the slowest gates by runtime, and
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
   The affordance is `bin/kfric.sh "<fact>" "<surface>"`: it stamps that
   grammar (date from `date +%F`) into `DRIFT_KIT_KNOWLEDGE_LOG`, creating
   the log's parent dir if missing, and refuses with a usage message and
   exit 2 unless both arguments are present and non-empty. It exists so
   capture is prompt-free — the raw form is a shell redirect
   (`printf … >> <log>`) that no allowlist glob suppresses safely (a
   mid-pattern wildcard is the command-injection shape the bash guard
   catches, and a decorated write trips the guard's decoration rule
   regardless), whereas the helper takes the fact as an argument with no
   caller-side redirect, so its invocation is a safe end-wildcard prefix-glob
   allowlist entry and a permission prompt never turns capture into deferred
   capture. The raw append stays legal as the fallback — the grammar, not
   the writer, is the log's contract; both consumers below read lines, not
   provenance. The convention costs one always-loaded bullet in the
   consumer's instructions file; that line is the loop's hook and must earn
   its recurring cost by the log actually filling.
2. **Triage (close)** — `templates/close-knowledge.md`, spliced into the
   consumer's close skill (the close-triage/close-brevity pattern): walk
   the log; for each entry, the remediation is a **doc-owner edit** — give
   the fact a home under the consumer's tier contract (canon-kit's star
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

## The published-evidence extractor

`bin/trajectory.sh` publishes this repo's own governed trajectory — the
evidence behind the docs evidence page. The benefits claim is
*self-referential* by ruling: the extractor emits the governed arm's real
history and states plainly that no controlled ungoverned baseline exists; a
synthetic controlled A/B experiment is the separate deferred
`benchmark-ab-experiment` rung, not this mechanism.

The extractor is a pure function of *closed* history — byte-stable across any
commit that is not a close. Each closed iteration N owns the commit range
`(close(N-1), close(N)]` (`close(0)` is the empty boundary — the first row runs
from the root up to its close commit); no range-scoped column reads HEAD, so an
interstitial commit — filed or hotfixed after a close, before the next scope —
falls into the *next* iteration's range and surfaces only when that iteration
closes, leaving every published row byte-identical until a new close lands.
Totals conserve across rows: every commit up to the last close belongs to
exactly one range. The extractor reads no now-relative field (no age-from-today)
either, so re-emission over an unchanged closed history is byte-identical —
exactly what the consumer freshness gate below byte-compares. It emits one row
per **closed** iteration (one carrying a
`close` stamp): an in-flight iteration's counts are still moving, so including
it would stale the committed projection at every commit — the closed-only rule
keeps the projection stable between iteration boundaries. Per closed iteration
it harvests:

- **iteration + stages run** — the stamp lines from `WORKFLOW-STATE.txt`'s git
  history (the file truncates at each scope boundary; history keeps every
  stamp), rendered as fixed stage slots so a skipped stage reads as a gap.
- **validate attestations** — the evidence-manifest lines
  (`validate-evidence.txt` history): the per-iteration suite roll-up and any
  non-clean verdict. This is the primary satisfiable-drift surface — a
  commitment made an iteration earlier that silently broke a surface shows as
  a failing suite, not a consistent-looking pass.
- **amendment latency** — per amendment file, git add-date to delete-date
  (merge), the longest lag in the iteration: the commitment-to-merge gauge. An
  amendment is attributed to the iteration whose range contains its delete
  (merge) commit; its add-date may precede the range start, since latency gauges
  commitment-to-merge wherever the commitment was made. Fixture and template
  amendment paths are excluded from the harvest from day one
  (`kpi-amendment-age` applies the same `*/gate-tests/*`/`*/templates/*`
  exclusion).
- **commit shape** — the feature/debt split of the iteration's commit subjects
  (`kpi-task-split`'s classification, applied over the same
  `(close(N-1), close(N)]` range).
- **gate-roster growth** — the `gates.list` member count at the iteration's
  close commit; with the queue's proposed-gate mentions this bounds the
  named-but-unbuilt backlog.

Excluded, and stated as a limitation on the framing page: knowledge-friction
counts — their log is gitignored per-iteration scratch, not committed history,
so the extractor cannot harvest it and that KPI stays a session-local lower
bound.

Interface: `trajectory.sh --emit` writes the markdown table (one row per closed
iteration, stable columns) to stdout — the shape the committed projection
pins; bare invocation prepends a human-oriented header. The extractor degrades
per surface to an `n/a (<reason>)` cell and exits 0 — drift-kit's fail-visible
discipline, registering no gate. `DRIFT_KIT_TRAJECTORY_SURFACES` overrides the
harvested state-file paths (§Layout and configuration).

Consumer wiring (this repo, not kit mechanism): the emission is committed at
`docs/evidence-data.md`, and the consumer gate
`scripts/check-trajectory-fresh.sh` (registered in `gates.list`) re-emits and
byte-compares — the gen-pre-commit/check-graph freshness pattern — so a
hand-edited or stale number is red at commit. The gate carries its own
`# graph:` manifest coupling `docs/evidence-data.md` to the harvested state
files, and a `good/`+`bad/` fixture pair that exercises the byte-compare
hermetically: because the harvest reads real git history, the fixture supplies
a synthetic emission as a second argument rather than regenerating one.
`docs/evidence.md` — the framing page, owned by the docs site — carries the
narrative and cites the data file, hand-copying no numbers.

## Layout and configuration

```
drift-kit/
  bin/drift-report.sh
  bin/trajectory.sh              # the published-evidence extractor
  bin/kfric.sh                   # the knowledge-friction capture affordance
  kpis/kpi-*.sh                  # the bundled generic set
  templates/drift-config.sh
  templates/kpis.list            # example registry (consumer copies + prunes)
  templates/kpi-deprecated-surface.sh   # example toolchain-shaped KPI (§Out of scope)
  templates/close-knowledge.md
  smoke/install.sh
```

Registers no gates (advisory; the guard-kit precedent), so no `checks/`,
`gate-tests/`, or `smoke/violation.sh`.

Config follows the established kit pattern: copy `templates/drift-config.sh`
into the gates dir (or point `DRIFT_KIT_CONFIG_FILE` elsewhere) and override
any knob; defaults fill what the consumer left unset. Knobs (this repo's
layout as defaults):

- `DRIFT_KIT_KPIS_FILE` — the registry; default
  `${GATE_SDK_GATES_DIR:-scripts}/kpis.list`.
- `DRIFT_KIT_KPI_DIRS` — extra resolution roots searched before the
  vendored kits' `kpis/` dirs; default: the consumer gates dir.
- `DRIFT_KIT_QUEUE_FILE` — default `${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}`.
- `DRIFT_KIT_KNOWLEDGE_LOG` — default
  `${GATE_SDK_WORKFLOW_DIR:-.workflow}/knowledge-friction.log`.
- `DRIFT_KIT_TIMINGS_FILE` — default
  `${GATE_SDK_TMP_DIR:-.tmp}/gate-timings.txt`.
- `DRIFT_KIT_TMP_DIR` — plugin scratch root; default
  `${GATE_SDK_TMP_DIR:-.tmp}`.
- `DRIFT_KIT_DONE_SECTION` / `DRIFT_KIT_DEFERRED_SECTION` — queue section
  headings the task-split and deferred-age KPIs scan; defaults `Done` /
  `Deferred` (queue-kit's).
- `DRIFT_KIT_TRAJECTORY_SURFACES` — the state-file paths the trajectory
  extractor harvests, given as `<state-file> <evidence-file>`; default
  `${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt` and its
  `validate-evidence.txt` sibling. A surface it cannot read degrades that
  iteration's cell to `n/a`.
- `DRIFT_KIT_GATES_FILE` — the registry whose member count the trajectory
  extractor reads at each close commit (gate-roster growth); default
  `${GATE_SDK_GATES_DIR:-scripts}/gates.list`.

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
emits exactly one line. The trajectory extractor needs committed history the
throwaway consumer lacks, so `smoke/install.sh` proves it against a hermetic
fake-history repo — one closed, range-bounded iteration — and asserts the
table parses, that iteration's row is emitted, and the in-flight iteration's
is not. Gate-sdk's `check-shellcheck` lints all kit sources as usual.

## Out of scope

Toolchain-shaped KPIs are consumer content: orphan-crate and bare-`#[allow]`
scans (Rust-specific dead-surface detection) and a `TODO(spec-ambiguity)`
marker count (a marker convention is consumer vocabulary; generalize the
convention first if it ever ships). A deprecated-surface trend is the same
shape — it counts markers over the consumer's `CANON_KIT_DEPRECATION_MARKERS`
roster (canon-kit's `check-deprecation-task` vocabulary), so it ships as
`templates/kpi-deprecated-surface.sh`, an **example** the consumer registers in
its `kpis.list` rather than a bundled plugin under `kpis/`: the marker spelling
is a consumer literal, and the kit stays deprecation-neutral. Registered, it
trends the live-marker backlog between majors so it surfaces gradually instead
of at one release; it degrades to `n/a` when the roster is unset (the bundled
plugins' fail-visible discipline). The release-boundary disposition walk over
the same roster is lifecycle-kit's `release-sweep` skill template. So are product-workflow KPIs: gate
exemptions (a `scan-exceptions` disposition split) and backlog-aging finding
counts — both read consumer gates. A narration-marker by-eye count is
superseded by canon-kit's `check-manifest-temporal`, which gates the same axis
instead of trending it. The always-loaded baseline mechanics are context-kit's
surface; drift-kit ships only the `kpi-always-loaded` plugin that reads its
meter.
