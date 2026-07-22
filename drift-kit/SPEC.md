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

Plugins read **exported env only** — a plain assignment in the consumer
config reaches the collator's shell but never a child plugin. The collator
closes that gap structurally: before invoking plugins, `drift-report.sh`
exports every scalar `DRIFT_KIT_*` variable (`compgen` over the namespace;
arrays skipped — bash cannot export them, and array knobs like
`DRIFT_KIT_KPI_DIRS` are consumed inside the collator itself), so a config
override reaches writer and reader alike with no fixed export list to drift
out of parity. It also exports `DRIFT_KIT_KIT_ROOTS` — newline-separated kit
roots, `gate_kit_roots` when gate-sdk resolves, else the kit's parent; a
plugin needing sibling-kit surfaces reads it rather than re-deriving the
roster, and falls back to its own derivation when run standalone without it.
The driver's handoff, not a consumer knob: `drift-report.sh` recomputes it
every run.

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
  A slug reaches its subject through `git log -1 --grep=<slug>`, so the
  classification rests on the landing commit's *message naming the slug*;
  a Done slug no commit message mentions counts as unclassified however
  correctly that commit is typed, and a later `chore` commit naming the slug
  wins the lookup over an earlier `fix` that did not.
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
- **kpi-overhead** — governance and gate-output share over the overhead
  meter's log (§The overhead meter): the governance share (`pct` averaged
  across the recent window of sessions, carrying the session-count and
  reading-age caveats) and the gate-output share (`gate`/`total` — the axis the
  deferred economy levers target). `--trend` emits `ovh <pct>%`. Degrades
  fail-visible to a "run bin/overhead-meter.sh" n/a row when the log is absent.
- **kpi-price-table-age** — two rows off the consumer price table
  (`DRIFT_KIT_PRICE_TABLE`, §The stage-economics meter input 3): the age in days
  of its `priced-as-of:` header, and the time to — or past — its optional
  `prices-valid-through:` header. **Both, because at the moment that matters they
  point opposite ways.** Age measures when someone last *typed* the numbers;
  expiry is when the numbers *stop being true*, and nothing makes the first
  predict the second. The failure is an inversion rather than a correlation gap:
  a table retyped the day before a known introductory-pricing row lapses reads
  `priced 1d ago` — freshest exactly when it is least trustworthy. An age-only
  KPI would read reassuringly at the one moment it was built to flag, which is
  why the expiry row exists. Full mode emits
  `priced <N>d ago (as-of <date>)` and either
  `expires in <N>d (through <date>)` or
  `EXPIRED <N>d ago — re-verify (through <date>)`; `--trend` emits `price <N>d`
  and nothing when the age is `n/a`. The expiry row volunteers no second
  fragment — the trend line takes at most one per plugin (§The KPI plugin
  contract) and staleness is the one already specified. Age counts from the
  wall clock (the `kpi-deferred-age` idiom); expiry counts whole calendar days
  against the through-date, which is a calendar claim: `expires in 0d` on the
  last valid day, `EXPIRED 1d ago` the next. Degrades per row and fail-visible:
  `n/a (no priced-as-of: header)`, `n/a (unparseable priced-as-of date)`, and
  for the expiry row `n/a (no prices-valid-through: header)` /
  `n/a (unparseable prices-valid-through date)`. With **no table at all** it
  emits the single `n/a (no price table)` age row and no expiry row — a table
  that is not there has no expiry to report, and one row per absent surface is
  the report's row-count shape. Advisory like every KPI: it never joins
  `gates.list`. A freshness *gate* is ruled out by construction — prices are a
  dated literal with no machine-readable feed, so checking them would mean
  fetching externally, which reds on causes no commit produced and breaks
  hermeticity (site-kit/SPEC.md §The monitor boundary). Reading a date in-tree
  needs no network. The mechanism is kit-generic; no model id, price, or roster
  enters the kit (§The stage-economics meter, the provenance seam).

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
gate's source, a commit message, or a prior/sibling deliverable it consults
to shape a new one), gets the right answer, and moves on —
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
   its recurring cost by the log actually filling. **Seam:** kfric is the
   narrow sensor for a *re-derived fact* only — a *work-shaped* mid-iteration
   finding (a gap, a task, a defect) is not knowledge friction and routes to
   the consumer's committed gap channel instead
   (lifecycle-kit/SPEC.md §The committed gap inbox); overloading this log as a
   backlog inbox dilutes the `kpi-knowledge-friction` signal it exists to carry.
   The prior/sibling-deliverable cue does not relax that seam: consulting a
   prior artifact to shape the next, the conclusion "this artifact's chrome
   *should be owned or generated* rather than copied" is work-shaped and routes
   to the gap inbox; what kfric captures is the narrower *fact re-derived* — the
   specific value or structure reconstructed from the prior artifact because no
   doc owns it.
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
  stamp), rendered as one slot per configured stage (`DRIFT_KIT_STAGES`, roster
  order), each labelled by its shortest roster-unique prefix, so a skipped or
  non-roster stage reads as a gap.
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

The freshness gate is blind at the enter-close commit by construction: the
extractor emits the closing iteration's row only once its `close` stamp is in
committed history, but during that commit's own pre-commit run the stamp is
not yet history, so the gate regenerates rowlessly and passes — the honest
limit of a pre-commit projection whose own close is an input. The consumer
contract closes the gap: the close ritual regenerates the projection in the
first commit *after* the close stamp lands — for a queue-clearing close, the
Done-clearing commit, where the regenerated file and the cleared queue ride
together. To make that commit fire the gate, the consumer freshness gate's
manifest names the queue file in `trigger=` (a trigger, not a coupled
surface — the projection's content derives from the state files, not the
queue, so `couples=` is unchanged and the couples⊆trigger parity holds); a
close that skipped the regeneration is then red at that commit. CI's full
battery stays the outer backstop (gate-sdk/SPEC.md §Enforcement tiers).

## The overhead meter

`bin/overhead-meter.sh [transcript.jsonl]` measures the methodology's own cost,
so efficiency claims cut both ways: what fraction of a session's volume is
governance (gate output, hook payloads, stage ritual, governed-doc reads)
versus task work. A bare invocation resolves the newest transcript under
`DRIFT_KIT_SESSIONS_DIR`; the tool is advisory by construction — exit is always
0 and it never joins `gates.list`, and a missing transcript is a 0-exit notice,
not a failure.

The measurement is a **byte-proxy at line granularity**, honesty first. Each
JSONL transcript line is classified whole by a fixed marker table in the script
— gate-verdict shapes to `gate`, hook/system-reminder blocks and stage-skill
loads and governed-doc reads to the rest of governance, everything unmatched to
task work — and its byte length lands in that category. The markers are
mechanism (kit names, gate-output shapes), never a private vocabulary, so the
table crosses no provenance seam. Bytes are not tokens and a line is not a
message: the only claim this buys is *proportion across sessions of the same
shape*, and every emitting surface carries that caveat parenthetical. No
transcript content is written out — the meter emits counts and percentages
only, and its log lives under the gitignored `DRIFT_KIT_METRIC_DIR` (the
persistent measurement home — an append-only trend must survive the scratch
wipes `DRIFT_KIT_TMP_DIR` invites), so the private transcript stays private.

One line is appended per measured session to `DRIFT_KIT_OVERHEAD_LOG`, grammar
`<date> <session8> total=<bytes> gov=<bytes> gate=<bytes> pct=<n>` where `pct`
is the governance share. `session8` is the dedup key the meter reads on append
— re-measuring a session replaces its line rather than double-counting it. The
per-category breakdown beyond `gate=` (hook, stage, governed-doc) stays on the
meter's stdout at measurement time; a log field with no reader is a field
removed. Field readers: `kpi-overhead` reads `pct`, `gate`, and `total`
(§Bundled KPIs), plus `date` for the reading-age caveat and the line count for
the session-count caveat.

The producer of the log is the consumer's close-stage binding — this repo's
`.claude/commands/close.md` invokes the meter on the closing session (consumer
config, not a lifecycle-kit change) — and any session may invoke it ad hoc.
Both knobs carry working defaults, so the enabling config ships on by default,
and the sessions-dir default matches the harness layout this repo already reads
for stage stamps.

The economy levers this meter exists to inform stay *behind* it: **commit-first**
(the generated hook already runs and prints the coupled gates, so a separate
pre-battery run duplicates that output) and **failures-only run-gates output**
(clean lines carry no decision value at battery scale) are evaluation targets,
not deliverables. Neither lands, and the consumer's always-loaded battery
wording does not change, until the meter shows gate-output share material over
several measured sessions — the wording changes *with* the measurement, not
ahead of it. Their design (a gate-sdk output-mode knob) is a future amendment
against that data.

Sibling meter: §The stage-economics meter prices the token draw this byte-proxy
does not — they share the `DRIFT_KIT_METRIC_DIR` retention contract and the
advisory exit-0 posture.

## The stage-economics meter

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
   grammar `<iteration> <stage> <session-id> <date>` (owned by
   lifecycle-kit/SPEC.md §The state machine; this tool is a read-only consumer of
   that contract, it changes nothing there). The `<session-id>` field is not a raw
   transcript id: it is lifecycle's `session-id.sh` *normalization* of one — a
   leading `agent-` stripped, then the first 8 chars (lifecycle-kit/SPEC.md
   §bin/session-id.sh) — so this repo's stamps carry an 8-char value, `session8`
   below. The stamp supplies the iteration↔stage↔session8 mapping. Read from
   `DRIFT_KIT_STATE_FILE` (§Layout and configuration), defaulting to the same
   state-file path the trajectory extractor already reads — and read as
   **history ∪ live**: that path's *committed history* (added lines in its diff
   across history, the technique §The published-evidence extractor already uses)
   unioned with its *current content*, so a `/scope` boundary truncation of the
   live file destroys no economics. Union rather than replacement because
   replacement blinds the meter to a stage that has stamped but not yet committed
   — precisely the in-flight stage whose economics are being read — and rather
   than fallback because the live file is almost always present and almost always
   truncated, so a fallback arm would never fire on the path that needs it. The
   union costs nothing: the `iteration/stage/session8` dedup already collapses a
   stamp seen in both arms. The reconstruction is **unbounded and carries no depth
   knob**: the trajectory extractor sets the in-kit precedent, and the effective
   bound is already self-enforcing from the transcript side — a stamp whose
   session has aged out of `DRIFT_KIT_SESSIONS_DIR` takes the unmatched path below
   and costs one skipped row. A depth knob would be a second bound with no reader.
   Because unbounded history would turn a per-stamp skip notice into unbounded
   output, unmatched stamps are **counted into one summary line** rather than
   listed; the skipped rows were never logged and still are not.
   **Diagnosing a stage absent from the trend log** — the union makes truncation
   a *non-cause*, and saying so here is the point: a stage whose stamps were
   truncated out of the live file still prices off history, so a missing
   `(iteration, stage)` row means that stamp resolved no transcript (aged out of
   `DRIFT_KIT_SESSIONS_DIR`, counted in the unmatched summary) or that the
   transcript carried no assistant-turn usage — both reported at measurement
   time — and never that the boundary truncation lost it. The standing
   misdiagnosis runs the other way: the union backfills a truncated stage only
   on the first run that reads far enough back, so a log sampled before that run
   shows the stage missing and reads as permanent loss. It is not; re-run the
   meter before concluding the history arm is lossy.
2. **Transcripts** — under `DRIFT_KIT_SESSIONS_DIR` (the knob the overhead meter
   already resolves). A stamp's `session8` selects a transcript by applying that
   **same normalization** to each candidate basename and matching — not by a raw
   filename prefix: this repo's stage sessions are subagent transcripts named
   `agent-<hex>.jsonl` whose stamp is `<hex>` truncated to 8 chars, so a raw
   prefix match against the `agent-` prefix would select nothing. Candidates are
   two-tiered, because a subagent transcript is not a sibling of its lead's: a
   lead session sits directly under the sessions dir, while the sessions it
   dispatches sit two levels deep under `<lead-session-id>/subagents/`. The scan
   globs both tiers, so a stage session dispatched by a live lead is found on the
   nested tier and a stage run without one on the flat tier. The tool sums
   the matched session's assistant-turn usage into four token categories —
   `input`, `output`, `cache_read`, `cache_creation` — per model id seen on those
   turns. A streaming transcript repeats a message id across lines (input/cache
   constant, output growing), so the sum keeps the last usage per message id
   before aggregating — summing raw lines would multi-count. Because this repo runs
   one session per stage under `LIFECYCLE_KIT_SESSION_BOUNDARY` (lifecycle-kit/SPEC.md
   §The state machine; its roster owns the setting), a session maps to exactly one
   stage, so per-session usage *is* per-stage usage. Under an iteration-boundary
   consumer a session may span stages, and then it bears **several stamps**. The
   join therefore keys on the **session, not the stamp**: a session's usage is
   summed once and attributed to the `(iteration, stage)` of its **last** stamp,
   and every stamp it yielded takes no row at all, named in a caveat listing the
   yielded pairs. Keying on the stamp is the over-count defect this rule exists to
   forbid — it makes two stamps two keys, resolves the same transcript twice, and
   bills one session's whole burn to both stages in full, so a per-stage figure
   read off it compares one number against a copy of itself. *Assignment, not a
   split*: apportioning a spanning session across its stages would need an
   allocation key — relative effort per stage — that nothing in the join measures,
   and a fabricated key folded into figures a tier decision is read off is worse
   than a stated one, the same rule that decides supervision below (the reserved
   `supervision` value, under The trend log). Last rather than first because the
   stamp is a stage's *first* step, so
   everything after the final stamp is that stage's; the honest limit is that the
   yielded stages under-report, which is why the caveat names them and makes the
   residue countable rather than invisible. Parsing the transcript
   needs `jq`; its absence degrades to a token-less notice rather than a failure.
3. **Price table** — a consumer-supplied data file mapping model id → per-token
   price for each of the four categories. This is **consumer config, never a kit
   literal** (the provenance seam, the `check-graph`/`graph-vocab` pattern): the
   per-token prices are public facts, but the *roster of models a consumer runs*
   is theirs, and a kit literal enumerating it would publish that roster. The kit
   ships `templates/price-table.tsv` with placeholder rows and the column schema;
   the consumer copies it and fills their roster. Resolved from
   `DRIFT_KIT_PRICE_TABLE` (§Layout and configuration).
   **Two dating headers, on the same consumer-owned header block.** Each is read
   as the first line of the file matching
   `^#[[:space:]]*<field>:[[:space:]]*<YYYY-MM-DD>`, trailing prose on the same
   line ignored: `priced-as-of:` (when the numbers were last transcribed) and
   `prices-valid-through:` (the last date every row is still true, **optional**).
   Their reader is `kpi-price-table-age` (§Bundled KPIs), not this meter — the
   KPI reads the table's header and never its rows, so an expired table still
   prices, loudly rather than silently, and the meter's arithmetic is untouched
   by either header. Where a row is time-boxed, `prices-valid-through:` **owns**
   that date and the file's own prose cites the header rather than restating it;
   a second copy of a date the day it becomes machine-read is the duplication
   the header exists to remove (de-literalization).

**Degradation.** An absent live state file is a notice and the run continues to
the history arm — committed history can carry stamps for a file absent from the
working tree — and the 0-exit "nothing to read" notice fires only when *both*
sources yield no stamps. A price table that is absent, or that has no row for a
model the transcripts name, degrades that model's cost cell to `n/a` and the tool emits the
token counts alone — the same degradation contract the trajectory extractor
applies to an unreadable surface (§Bundled KPIs / §Layout and configuration). Cost
is additive over priced cells only; an `n/a` cell never poisons the total
silently — the output carries an "incomplete pricing" caveat when any contributing
cell degraded.

**The under-count bound.** The unmatched counter reports the *stamp with no
transcript* and is structurally blind to its inverse — the **transcript with no
stamp**. A stage that continues in a new session (a credential swap mid-stage,
any resume) leaves that session unstamped, so it matches no stamp and is never
sought; its burn is invisible rather than wrong. The meter therefore counts the
transcripts under `DRIFT_KIT_SESSIONS_DIR` that no row claimed and reports the
count. It is an **upper bound, never an attribution**: a transcript carries no
iteration and no stage, so nothing in the join could place it, and most
unstamped transcripts are ordinary non-lifecycle sessions. Sizing the blind spot
is the whole of what drift-kit can do alone — attributing a continuation would
need the *stamp* side to record it, which is lifecycle-kit's contract and no
part of this meter's read-only consumption of it.

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
the overhead meter dedups on `session8`. That key is also what makes the
history ∪ live read safe with no added mechanism: a history arm re-derives rows
already logged, and re-derivation replaces a triple's line rather than
double-counting it. Any per-model sub-breakdown beyond these
fields stays on stdout at measurement time; a log field with no reader is a field
removed. Field readers: the `/economics` narrative reads `cost` and the four token
fields (`cr` headline); the operator reads `cost` close-over-close; the deferred
`benchmark-ab-experiment` rung's measurement half consumes this log rather than
rebuilding it; `date` carries the reading-age caveat. `date` is the
**measurement** date, not the stage's — and under the history read the two can be
far apart, because re-deriving an old iteration restamps its row to the day it
was re-measured. The reading-age caveat stays correct (it ages the reading, which
is what it says), but the field may not be read as "when this stage ran": the
stage's own date is in the stamp, and the trajectory extractor is the surface
that renders it.

**The reserved `supervision` value — the lead's burn is its own row.** Under a
split-lead posture the lead session dispatches, verifies, and runs batteries
while carrying **no stamp**, so it appears in no row and every per-stage total
understates the iteration's true cost by the whole supervision line item. The
meter emits it as a distinct row instead, labelled by
`DRIFT_KIT_SUPERVISION_LABEL` (§Layout and configuration). *A row rather than an
apportionment across stages*: spreading the lead's burn over stages would need
an allocation key with no basis in anything measured — the lead's burn is not
proportional to any stage's tokens — and would fold a fabricated number into
figures a tier decision is read off, where a distinct row is a visible line item
the operator can accept, discount, or ignore. The meter's contract is that a
degraded or absent measurement is visible, never silently folded (§The report
skeleton); the same rule decides this.

- **The column's widened meaning.** `<stage>` reads **stage *or* cost-bearing
  role**, not stage alone. A supervision row is a role that carries cost and no
  stamp — never a lifecycle stage that dropped out of the roster. The column's
  members were roster-closed until this value existed, and a reader who assumes
  they still are would misdiagnose the row as roster drift.
- **The producer — derivation from the transcript path, not from a stamp.** The
  two-tier scan above already resolves a dispatched stage session at
  `<sessions-dir>/<lead-session-id>/subagents/<agent>.jsonl` while a lead sits
  directly under the sessions dir. So for every stamp whose transcript resolved
  on the **nested** tier, that path's `<lead-session-id>` component names the
  supervisor and the stamp supplies the iteration. The meter collects
  `(lead-session, iteration)` → dispatch count, resolves each lead's own
  transcript on the flat tier, and sums its assistant-turn usage per model with
  the same last-usage-per-message-id reader — no second parser. **No lifecycle
  change**: the lead still stamps nothing and moves no cursor; the supervising
  session is *derived*, which is what keeps this a read-only consumption of
  lifecycle-kit/SPEC.md §The state machine.
- **The attribution invariant.** A transcript's usage is attributed to exactly
  one `(iteration, stage)` pair — the rule the session-keyed join above enforces
  for stage rows, and the reason a lead whose own transcript already carried a
  stage row yields no supervision row. In the ordinary case a lead supervises one
  iteration and its whole usage lands on that iteration's supervision row —
  exact, no key. A lead spanning two or more iterations **apportions in
  proportion to the number of stamped stage sessions it dispatched per
  iteration**, integer-split with the remainder to the iteration holding the most
  dispatches (ties broken by iteration name, so the split is deterministic and
  the parts re-sum to the whole). The run's caveat names that key: a disclosed
  key is honest and a silent one is not.
- **Collision rule.** If any stamp the run reads names a stage equal to the
  label, the meter emits a visible notice naming `DRIFT_KIT_SUPERVISION_LABEL`
  and emits **no** supervision rows that run. It is checkable from data the meter
  already reads — its own stamps — so this adds no roster dependency and no
  second bound to drift.
- **Degradation.** A run with no nested-tier match (a stage run without a live
  lead) emits no supervision row and no notice: zero supervision burn is the
  honest reading, not a missing measurement. A lead transcript aged out of the
  sessions dir takes the existing unmatched-summary path.
- **No new field.** The four token fields, `cost`, and `date` carry a supervision
  row exactly as they carry a stage row, and the dedup key, the
  replace-on-re-measure behavior, and every existing reader work unchanged —
  `supervision` is a value in an existing column, not a new field. The
  apportionment key and the collision notice are **stdout caveats at measurement
  time**, not log fields; a log field with no reader is a field removed, and
  neither has one. The row's own named reader is the `/economics` narrative's
  supervision line item (§The `/economics` skill).
- **Blast radius of the widened column.** The log has one parsing reader in
  production code — this meter's own dedup grep — and one asserting reader in the
  harness, `smoke/install.sh`, which counts log lines. No *gate* reads it (it
  lives under the gitignored `DRIFT_KIT_METRIC_DIR`), and the `/economics`
  narrative reads it as prose rather than parsing the stage column.
  **`bin/trajectory.sh` does not parse this column at all** — the reader worth
  naming, since it is the surface a hardcoded stage roster once broke; it reads
  stamps and its own `DRIFT_KIT_STAGES` roster, never this log. So a non-stage
  value in the column cannot silently fall out of a roster the way the
  trajectory's stamps did.

## The `/economics` skill

`/economics` is the customer-facing post-iteration narrative: run at close, it
chains `bin/overhead-meter.sh` → `bin/stage-economics.sh` into one report
answering "what did this iteration cost, where, and was the model posture worth
it". `stage-economics` is the sole cost-attribution surface — it prices
per-transcript, per-stage, per-model token draw (the token SSOT), while
`overhead-meter` contributes the governance share, not a cost figure. The
narrative deliberately excludes delegation-kit's usage-trend budget-%: that
rate-window footprint is account-wide — confounded by overlapping sessions and
by a second operator on the same account — so it is the wrong instrument for
per-iteration cost attribution, and carrying it beside the per-transcript token
SSOT put a confounded advisory number next to a clean one a reader could
over-trust as this iteration's cost. It ships as a drift-kit skill
template `templates/economics.md`, materialized in the consumer as the copy
`.claude/commands/economics.md` — the template↔consumer-copy split the guard/hook
skills use, its one bound slot the consumer's model posture. It is not a lifecycle
stage (it moves no cursor, stamps nothing) and so is outside
`check-stage-skill-coverage`'s stage roster; it is a reporting ritual the close
skill may invoke, never a gate.

## Layout and configuration

```
drift-kit/
  bin/drift-report.sh
  bin/trajectory.sh              # the published-evidence extractor
  bin/kfric.sh                   # the knowledge-friction capture affordance
  bin/overhead-meter.sh          # the governance-overhead byte-proxy meter
  bin/stage-economics.sh         # the stage × model × iteration spend pricer
  kpis/kpi-*.sh                  # the bundled generic set
  templates/drift-config.sh
  templates/kpis.list            # example registry (consumer copies + prunes)
  templates/kpi-deprecated-surface.sh   # example toolchain-shaped KPI (§Out of scope)
  templates/close-knowledge.md
  templates/price-table.tsv      # placeholder + schema for the stage-economics price table (consumer fills the roster)
  templates/economics.md         # the /economics close-cadence skill template
  smoke/install.sh
  smoke/overhead-fixture.jsonl   # synthetic transcript driving the classifier smoke
```

Registers no gates (advisory; the guard-kit precedent), so no `checks/`,
`gate-tests/`, or `smoke/violation.sh`.

Config follows the established kit pattern: copy `templates/drift-config.sh`
into the gates dir (or point `DRIFT_KIT_CONFIG_FILE` elsewhere) and override
any knob; defaults fill what the consumer left unset, and a set-but-missing
`DRIFT_KIT_CONFIG_FILE` exits 2 rather than silently running on defaults.
Knobs (this repo's layout as defaults):

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
  `${GATE_SDK_TMP_DIR:-.tmp}`. Members are regenerated on every run, so a
  scratch wipe is harmless.
- `DRIFT_KIT_METRIC_DIR` — the persistent measurement home, distinct from
  `DRIFT_KIT_TMP_DIR` by retention contract: metric-dir members are
  append-only trend logs that survive scratch wipes. Default `.metric`. The
  dir must be gitignored and never committed — trend samples carry account
  identifiers and per-session refs, so committing it publishes them (the
  retention/privacy contract is kit-generic; a consumer's provenance seam
  makes the gitignore load-bearing).
- `DRIFT_KIT_SESSIONS_DIR` — the agent transcript directory the overhead
  meter reads for a bare invocation; default
  `${CLAUDE_CONFIG_DIR:-$HOME/.claude}/projects/<cwd-slug>`, where `<cwd-slug>`
  is the working directory with every non-alphanumeric replaced by `-` (the
  derivation lifecycle-kit's stage stamps already apply; drift-kit re-derives
  with its own knob rather than importing a sibling kit's bin contract).
- `DRIFT_KIT_OVERHEAD_LOG` — the overhead meter's append log; default
  `$DRIFT_KIT_METRIC_DIR/overhead-log.txt` (gitignored, so the private
  transcript's derived counts never enter version control; the meter
  `mkdir -p`s the log's dirname). All three resolvers — the meter,
  `kpi-overhead`, and the collator's namespace export — compute this same
  default, and the smoke's writer/reader assertion holds them together
  (§Testing).
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
- `DRIFT_KIT_STAGES` — the ordered stage roster the trajectory extractor
  renders (one slot per stage, labelled by its shortest roster-unique prefix);
  default `(scope align build validate close)`, which reduces to the frozen
  single-letter header. A consumer running a wider roster derives this from its
  sole roster owner — this repo's `scripts/drift-config.sh` sources
  `scripts/lifecycle-config.sh` and copies `LIFECYCLE_KIT_STAGES`, the SSOT
  activation — rather than re-listing. Third instance of drift-kit re-deriving a
  cross-kit fact with its own knob rather than importing a sibling kit's bin
  contract (alongside `DRIFT_KIT_SESSIONS_DIR` and `DRIFT_KIT_STATE_FILE`).
- `DRIFT_KIT_STAGE_ECONOMICS_LOG` — the stage-economics append trend log; default
  `$DRIFT_KIT_METRIC_DIR/stage-economics-log.txt` (gitignored; the meter
  `mkdir -p`s the dirname).
- `DRIFT_KIT_PRICE_TABLE` — the consumer-owned model→price roster the
  stage-economics meter prices through; default
  `${GATE_SDK_GATES_DIR:-scripts}/price-table.tsv` (beside `graph-vocab.sh`, the
  consumer-config precedent). Absent, cost degrades to `n/a` and tokens still report.
- `DRIFT_KIT_SUPERVISION_LABEL` — the reserved value the stage-economics meter
  writes into the trend log's `<stage>` column for a lead's own burn
  (§The stage-economics meter, the reserved `supervision` value); default
  `supervision`. A consumer whose
  lifecycle roster already carries that word renames it here; a stamp naming the
  label collides and suppresses the rows for that run rather than blending two
  meanings into one column value.
- `DRIFT_KIT_STATE_FILE` — the WORKFLOW-STATE path whose *committed history and
  live content* the stage-economics join reads for stamps (§The stage-economics
  meter, history ∪ live); default `${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt` (the
  same default the trajectory extractor's surface list computes — drift-kit
  re-derives with its own knob rather than importing a sibling kit's bin contract,
  the established `DRIFT_KIT_SESSIONS_DIR` precedent).

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
is not. The overhead meter has a fixed classifier, so it *is* fixture-stable:
`smoke/overhead-fixture.jsonl` carries known category bytes, and
`smoke/install.sh` drives the meter over it and asserts the log-line grammar,
that the task line is excluded from governance, that `gate` is a proper subset
of `gov`, that `pct` is the rounded governance share, and that a re-measure
replaces the session's line rather than doubling it; kpi-overhead is exercised
over that log (its two lead rows and the `ovh` trend fragment) and in its
log-absent degradation. The writer/reader-divergence assertion runs meter and
KPI under one `DRIFT_KIT_METRIC_DIR` override with no explicit
`DRIFT_KIT_OVERHEAD_LOG` and asserts the reader finds the log the writer
wrote — the surviving divergence surface the namespace export cannot guard:
writer and reader computing *defaults* independently. The stage-economics meter
is likewise fixture-driven: `smoke/install.sh` drives the join over a synthetic
fixture set — a small WORKFLOW-STATE stamp file, a synthetic transcript carrying
usage records for the stamped `session8`, and a placeholder price table — and
asserts the emitted trend line's fields (the `<iteration> <stage> <model>`
grouping and the `in`/`out`/`cr`/`cw`/`cost` values), that a re-measure replaces
the triple's line rather than doubling it, and the `n/a` cost cell plus the
incomplete-pricing caveat when the priced model has no row (the price-table-absent
degradation). Its **history ∪ live** read is proved against the same hermetic
fake-history repo the trajectory extractor uses, which already carries the
truncation shape: that repo's live WORKFLOW-STATE was overwritten with the
in-flight iteration's stamp, so the closed iteration's stamps survive only in
committed history. One run asserts both arms — the history-only stamp prices
(replacement would lose it) and the live-only stamp prices (a history-only read
would lose the uncommitted tail). Two further stage-economics fixtures each get
**their own sessions dir, state file, and log**, because the flat fixture set's
log is asserted to hold exactly one line and a second row there would red that
assertion rather than the behavior under test. (i) A *two-stamp* fixture — one
session stamped into a further stage — asserts the log holds exactly one line, that
the row names the **last** stamp's stage, and that the yielded stamp is named in
the collapsed caveat; a transcript matching no stamp sits in the same dir and
asserts the unstamped-transcript bound reports it. (ii) A *nested-tier* fixture —
a synthetic `<lead>/subagents/<agent>.jsonl` beside a flat `<lead>.jsonl` —
asserts exactly one supervision row is emitted for the iteration carrying the
lead transcript's own usage, that the dispatched session keeps its own stage row,
that the row's label is `DRIFT_KIT_SUPERVISION_LABEL`'s value rather than a
literal, and that a stamp naming the label suppresses the row with the collision
notice. `kpi-price-table-age` is fixture-stable in the same way — it reads two
dates out of a file the fixture writes — so `smoke/install.sh` drives it over
purpose-built tables and asserts the age row, the `price <N>d` trend fragment,
each header's absence degrading its own row independently, and — **the
inversion the KPI exists for** — that a table whose `priced-as-of:` is *today*
and whose `prices-valid-through:` has passed reads a fresh age row and an
`EXPIRED` expiry row in the same breath. A fixture that pins only the age row
has not tested the feature: the age row is reassuring in exactly that case,
which is the defect. The no-table degradation is asserted to emit its single
`n/a (no price table)` row, and the report-wide one-row-per-registered-KPI
assertion covers that shape from the other side. Gate-sdk's `check-shellcheck`
lints all kit sources as usual.

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
