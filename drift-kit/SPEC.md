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

`--emit drift-report` — the collator, a **bridged arm** of the gate binary
(gate-sdk/SPEC.md §The non-gate arm), reached as
`bash gate-sdk/bin/run-gates.sh --emit drift-report [--trend]`. It owns the
frame; every measurement lives in a member:

1. Read the KPI registry (`kpis.list` — one member name per line, `#`
   comments; the gates.list grammar) and resolve each name through the three
   tiers of §The extensibility contract.
2. Derive the iteration-start commit **before** the plugin export loop, not at
   print time, and assign it to `DRIFT_KIT_ITERATION_START` so the loop carries
   it to every plugin. Both the derivation and its invocation sit above the
   loop — relocating the call alone would leave it undefined — and its inputs
   are already set by that point. The report header keeps reading the same
   value. **Priced, because the position is not free:** deriving it above the
   loop runs it in `--trend` mode as well as in the full report, so one `grep`
   and one single-file `git log -S` run on every session start through the
   context hook. Accepted — a pickaxe over a file of a few dozen lines — but it
   is a per-session cost rather than a per-report one.
3. Run each plugin, collect its rows, and group them into the two labeled
   sections — the honesty labels are the frame's contract:
   - header: `=== Drift KPIs (advisory — trend, not level) ===`, plus the
     iteration-start commit when derivable;
   - `--- Lead (weighted high — act before drift compounds) ---`;
   - `--- Lag (weighted low — undercounts by construction) ---`;
   - footer: `Read trend across sessions; lag KPIs lower-bound only.`
4. `--trend` emits a single compact line instead — the fragment each plugin
   volunteers, joined with `·` — consumed by context-kit's session-context
   hook (`CONTEXT_KIT_DRIFT_REPORT`, already wired in its template).

Degrade discipline: exit is always 0. A member that fails or produces nothing
yields a visible `<name>  n/a (plugin failed)` row in the lead section —
fail-visible, not fail-closed, because a silently vanishing KPI is itself drift.
A member whose *surface* is missing (no log yet, no timings file) degrades to
`n/a (<reason>)` in its own value, not by dying.

**Why a bridged arm rather than a top-level flag.** The front-end composes
`--emit-<name>` from its `--emit <name>` operand and resolves each member's
declared knob roster; a hardcoded top-level flag receives no consumer override at
all. For a kit whose entire surface is consumer-overridable knobs that would be a
silent functional regression rather than a porting detail.

**The collator's own knobs resolve in `lib/drift.sh`** (§lib/drift.sh), not in the
arm: the config bridge resolves a declared name by sourcing the owning kit's
library alone, so a default left beside the reader would resolve to nothing and a
default restated on the crate side would be a second producer.

## The extensibility contract

**A consumer adds its own KPI by dropping a plugin in its own KPI dir. That is
drift-kit's promise to adopters, and no substrate move narrows it.** It is stated
here as a contract rather than left inferable from the README and from
`DRIFT_KIT_KPI_DIRS`' purpose, because inference is what the next session would
otherwise have to repeat.

A registry name resolves through **three tiers, consumer-first**, and the first
tier that answers wins:

1. each entry of `DRIFT_KIT_KPI_DIRS` — the adopter's own dirs, default the
   consumer gates dir — for a file named `<name>.sh`;
2. each vendored kit's `kpis/<name>.sh`;
3. the **built-in members the binary carries** (§Bundled KPIs).

So a consumer file named `kpi-task-split.sh` **shadows** the built-in of that
name. Tier 3 is where the bundled set lives after the port; tiers 1 and 2 are
unchanged, which is the whole point — the port moved the kit's own members and
touched nothing about how a consumer's are found.

A tier-1 or tier-2 hit is **executed directly** (`"$path"`, never
`bash "$path"`), so the execute bit still governs and a non-executable plugin
degrades to the fail-visible row rather than running anyway. A consumer plugin
reads the exported environment of §The KPI plugin contract; a built-in reads the
same resolved values in process. **Same values, same transition** — a consumer
plugin is not a second-class reader of the contract this extension point
promises.

`DRIFT_KIT_KPI_DIRS` exists for nothing but this, and a knob whose only purpose
is an extension point *is* that extension point's specification. Underneath that,
a consumer-first plugin registry is the `check-graph`/`graph-vocab.sh`
consumer-config pattern in another dress: narrowing it would narrow the
provenance seam's own mechanism.

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
config reaches the collator but never a child plugin. The collator closes that
gap structurally: before invoking a plugin it exports every scalar `DRIFT_KIT_*`
value the config bridge resolved, so a config override reaches writer and reader
alike **with no fixed export list to drift out of parity**.

**The derivation is the contract, and the arm keeps it derived.** The shell
collator spelled it `compgen -v DRIFT_KIT_`; the arm declares the **prefix
family** `DRIFT_KIT_*` in its knob roster, which the config bridge resolves by
running that same `compgen` inside the owning kit's already-sourced subshell
(gate-sdk/SPEC.md §lib/gate.sh). A knob a consumer's own `drift-config.sh`
declares and nothing in this repo names therefore crosses the bridge and reaches
the plugin. A transcribed `DRIFT_KIT_*` roster would lose exactly that knob and
would land on the derivation-first rule; none is introduced anywhere.

Arrays are skipped, as bash skipped them: the wire format joins an array's
elements with a tab, and the two array knobs (`DRIFT_KIT_KPI_DIRS`,
`DRIFT_KIT_STAGES`) are consumed inside the collator itself. **Honest limit:** a
*consumer-declared* one-element array is indistinguishable from a scalar on the
wire and crosses as one, where bash would have dropped it. That is a difference in
a consumer's favour, and it is the only place the exported set is not exactly the
shell's.

It also exports `DRIFT_KIT_KIT_ROOTS` — newline-separated kit roots, the resolved
kit-root set; a plugin needing sibling-kit surfaces reads it rather than
re-deriving the roster. The driver's handoff, not a consumer knob: the arm
recomputes it every run.

`DRIFT_KIT_ITERATION_START` is the same *class* of handoff as
`DRIFT_KIT_KIT_ROOTS` — computed by the driver every run, reaching every plugin
as exported environment and every built-in as the same resolved value, never a
consumer knob. What it does not share is the
parity hazard: it introduces no fixed export list that could drift out of step
with the knob set it travels beside. A plugin reading it gets the empty string
when no baseline is derivable and degrades to `n/a` on its own rows rather than
dying.

Plugins never block and never write outside `$DRIFT_KIT_TMP_DIR` scratch;
a measurement needing state (a baseline, a log) reads a file some
*other* mechanism owns and stamps its reading-age caveat when the file is a
past measurement rather than live state (the gate-runtime pattern below).

## Bundled KPIs

The generic set — each coupled to a kit-governed surface, each degrading to
`n/a` when the consumer lacks that surface. They are **built-in members the
binary carries**, resolution tier 3 of §The extensibility contract, and their
measurement semantics are exactly what they were on the shell substrate: the
parity oracle for the move is byte-identity of the emitted report across the two
substrates, captured before the originals were deleted and diffed after, in both
the full and `--trend` modes.

Two members read a date and are held to the operator's civil zone rather than to
UTC: they resolve a day through `date -d`, the choice queue-kit's queue-index
cutoff already made and for the same reason. `kpi-settings-local` has one
degrade fewer than its shell original: the compiled member parses the overlay
itself, so it carries no external-program dependency an absent `jq` could take.

Lead:

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
  **What `total` counts is entries that left the live pool, not deliverables.**
  The done section is the live pool's exit and not a delivery claim
  (queue-kit/SPEC.md §The queue format), so a slug that shipped nothing reaches
  it legitimately — mooted by a landed unit or a closed ruling, or ruled
  wontfix. Such an exit has no `feat`/`fix`/`refactor` commit naming its slug to
  be classified by, its landing commit being a `chore` or a `docs`, so it falls
  to **`unclassified` by construction** and the feat/debt **split is not
  polluted by it**. Only the "of N done" denominator moves.
  **`unclassified` is a mixed bucket and is not a count of non-shipping exits.**
  A task that genuinely delivered but whose landing commit was typed `chore` or
  `docs` lands there too, and so — by the `git log -1` caveat above — does one
  whose `feat` landing commit was followed by a later `chore` naming the same
  slug. It is the bucket a non-shipping exit **cannot escape**, which is the
  honest claim; measuring them would need a reader this KPI does not have.
- **kpi-gate-backlog** — proposed-but-absent gates: `check-*`/`scan-*`
  names appearing anywhere in the queue with no file in any gate-resolution
  dir, over the live gate count. A name with a file on disk is built and
  drops out of the numerator.
- **kpi-amendment-age** — age in days of the oldest amendment on disk
  (`SPEC-*.md`, git add-date; fixture and template paths excluded, matching the
  published-evidence extractor's amendment-latency harvest); the pressure gauge
  behind canon-kit's short-lived-amendment rule.
- **kpi-deferred-age** — age of the oldest **defer date** in the queue's
  deferred section: premise-rot pressure on design-pending work. The defer
  date is queue-kit/SPEC.md §The queue format's definition — the `Surfaced`
  mark where an entry carries one, else the `Filed` provenance date — which
  this plugin re-implements rather than sourcing, because a dependency on
  queue-kit's lib would close a cross-kit cycle. Accepted residual: one owner
  doc, two implementations, both carrying a `spec:` line citing the owner.
  Both marks count, because a pool whose entries carry provenance lines rather
  than `Surfaced` marks would otherwise leave the input covering a fraction of
  it. The KPI de-duplicates the dates it finds and reports the oldest, so it
  trends dates rather than counting entries. Its unknown-heading reset drops an
  icebox placed after the deferred section out of the input **by
  construction**, which is wanted: an evicted entry's age is not the thing this
  KPI trends. gate-sdk's `check-gate-exemption-tasks` carries the
  same-shaped scan with the opposite behavior, deliberately (gate-sdk/SPEC.md
  §check-gate-exemption-tasks) — recorded so a later reader does not "fix" one
  into agreement with the other.
- **kpi-queue-net-delta** — the design-pending pool at the iteration-start
  commit against the worktree, as **two rows, because one number would be
  gameable**: *entry net delta* (`filed − drained`, where filed is a slug now
  deferred that was in neither section at the baseline and drained is a slug
  that has left the pool entirely) and *carry weight* (the two sections' line
  count now against the baseline). An icebox move counts as **neither** — it is
  compression, not intake and not closure. The two axes move independently:
  intake pressure moves the first, compression the second, so a session that
  mass-evicted to flatter the delta row moves the weight row instead and the
  gaming is visible rather than hidden — the argument `kpi-price-table-age`
  makes for carrying two rows that point different ways. `--trend` volunteers
  one fragment, `qnet <±N>`, per the one-per-plugin rule; the weight row
  volunteers none, because intake is the axis a filing session can act on
  inside the session. With no baseline (a standalone run, a fresh clone) both
  rows degrade to `n/a (no iteration baseline)`.
- **kpi-prompt-friction** — distinct/total prompting calls via guard-kit's
  `scan-prompts.sh --count`; `n/a` when guard-kit or its log is absent. Its
  numerator is a **key count**, so a change to how that tool keys a row steps
  this trend without any behavior moving, and the `^[0-9]+/[0-9]+$` contract
  stays satisfied throughout — nothing reds, which is what makes such a step
  silent. A trend reader meeting one looks for the definitional cause before the
  behavioral one; guard-kit/SPEC.md §scan-prompts owns the key and records each
  step with its pre-change reading.
- **kpi-always-loaded** — the standing per-session surface: level and
  since-baseline delta via context-kit's `always-loaded.sh` meter.
- **kpi-settings-local** — entry count of the untracked local permission
  overlay (`.claude/settings.local.json` allow/deny/ask, via `jq`); the
  notice signal for guard-kit's close-stage prune/promote step.
- **kpi-gate-runtime** — full-battery runtime from the runner's timings
  file (`<tmp-dir>/gate-timings.txt`): total, the slowest gates by runtime, and
  the file's reading age — a *measurement*, not live state, so the age
  caveat rides the value. Its `TOTAL` is the **sum** of the per-member times and
  under the runner's worker pool it does not approximate wall-clock
  (gate-sdk/SPEC.md §run-gates): a run that got shorter can show a larger
  `TOTAL`, because contention lengthens each member while the battery finishes
  sooner. Read it as dispatch cost, and the slowest-gate rows as the critical
  path a pool cannot shorten.
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
  so the value lower-bounds the real rate. **The degenerate case is the one to
  read carefully, because the lag label alone does not warn a reader off it:** a
  lower bound of **zero** bounds nothing, so an empty log is not evidence of zero
  friction, and the emitter says so in a line of its own rather than reporting a
  count of nothing in the sentence that reads as a measurement of zero
  (§The knowledge-friction loop owns the three-state contract).
- **kpi-incident-recurrence** — re-filings of the same finding, summed over the
  queue's `recurrence:` declarations (queue-kit/SPEC.md §The tag algebra owns the
  grammar), plus the highest-count slug. Like `kpi-deferred-age` it re-implements
  the read rather than sourcing queue-kit's lib, under the same accepted residual
  — one owner doc, two implementations, both carrying a `spec:` line citing the
  owner. **The lag label is a measurement claim, not a priority one.** A
  recurrence nobody files is uncounted, exactly `kpi-knowledge-friction`'s
  structure, **degenerate case included** — and so is one no session **judged**,
  since each date records a
  judgment rather than a derivation: the same lag structure, one step later in
  the chain. So lag is the honest fidelity tier even though the metric is highly
  actionable. **It is a judged count, so it is only comparable across a fixed
  judging rule** — changing what counts as a recurrence breaks comparison across
  the change, and a series spanning such a change is two series, not one.
  Actionability rides lifecycle-kit's pre-emption rule and its
  `LIFECYCLE_KIT_RECURRENCE_THRESHOLD`, never this report's weighting — which is
  why the counter and the rule are two pieces and not one. A report-only signal
  would have reproduced the defect the pair exists to fix: a faithful record no
  ranking reads. `--trend` emits `recur <N>`; with no declaration anywhere it
  degrades to `n/a (no recurrence declaration in the queue)`.

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
   The affordance is `bin/kfric.sh [--] "<fact>" "<surface>"`: it stamps that
   grammar (date from `date +%F`) into `DRIFT_KIT_KNOWLEDGE_LOG`, creating
   the log's parent dir if missing, and refuses with a usage message and
   exit 2 unless both arguments are present and non-empty. Both are free text,
   so it also validates their **shape** — see
   gate-sdk/SPEC.md §The bin/-tool contract —
   scanning every positional rather than the first, two slots making arity
   safe in neither. It exists so
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
   **This is a rule about standing instructions, not a rule about triage** — it
   binds the **capture** side identically, which is why a per-stage capture
   prompt is refused below rather than reconsidered each time the log reads
   empty.
   Then clear the log — its named reclaim path. Nothing refuses a close that
   skips the walk, so the log declares itself advisory on the close-surface
   roster (lifecycle-kit/SPEC.md §The close-surface roster) with that clear as
   its reclaim command:

   close-surface: .workflow/knowledge-friction.log advisory reclaim=: > .workflow/knowledge-friction.log
3. **Aggregate (drift)** — `kpi-knowledge-friction` reports the per-iteration
   **capture** count, and reading it as a friction count is the error to avoid.
   It moves with two independent things — how much friction occurred, and how
   much of it a session stamped — so a **fall is attributable to neither**. In
   particular a **zero reading is not evidence of zero friction**: it is what an
   iteration produces when nobody captured, it is equally what one produces when
   nobody re-derived, and the log cannot tell a reader which. The error runs in
   the expensive direction, which is why it is stated here rather than left to a
   careful reader: the KPI reads **best** exactly where it is **least**
   trustworthy, so an iteration whose capture discipline collapsed is
   indistinguishable from one whose tier contract is complete — and the first is
   the one that needs acting on. The emitter carries the non-inference at the
   point of reading rather than leaving it to a reader who has read this section:
   three log states, three lines — **absent** (`n/a`: this tree runs no capture
   loop), **present and empty** (a count of nothing, said as such), and
   **present and non-empty** (a lower bound). A log holding only blank lines is
   the empty state, since the count is of non-blank lines. `--trend`'s grammar
   does **not** move for the empty state and emits `kfric 0` as before: a trend
   consumer plots a series, and changing a series' grammar for one of its values
   makes the history unreadable across the change (§Bundled KPIs applies the same
   reasoning to `kpi-incident-recurrence`). The limit belongs on the human-read
   line, where a reader can act on it. Detection is the
   loop; elimination is a tiering edit.

**Three alternatives were weighed and refused, and they are recorded so the next
session meeting an empty log does not re-open a settled call as if it were an
oversight.**

- *A corroborating signal* — refused for **weakness**, not for cost. A prompt log
  records that a session read history; it does not record *what fact* was
  re-derived, so it can raise a suspicion and can never resolve one. A signal
  that cannot identify an instance cannot correct a count.
- *A per-stage capture prompt* — refused on the ground step 2 already states for
  the remediation side: a standing session-start instruction converts one
  re-derivation into a permanent per-session tax. That reasoning binds the
  **capture** side identically.
- *A capture floor* — an independent signal that capture happened, which is the
  one this loop is most likely to be mistaken for having. **None cheap exists,
  and none is supplied here.** The honest consequence is that the KPI stops
  asserting what it cannot support; it does not start supporting it. The one
  metric measuring the tier contract's completeness still reads best exactly when
  nobody is capturing, and now says so.

The heavy alternative — periodic LLM-scan of session transcripts reduced to
each party's messages — is deliberately out of kit scope: it needs harness
transcript access no kit mechanism owns.

## The published-evidence extractor

The `trajectory` emit arm (gate-sdk/SPEC.md §The non-gate arm) publishes this
repo's own governed trajectory — the
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
closes, leaving every published row byte-identical until a new close lands. That
sentence is range arithmetic: it is descriptive of a commit that happens, and
*when* one may legitimately be made is lifecycle-kit/SPEC.md §Deviation
transitions' interstitial-mitigation rule, which leaves this accounting unchanged.
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

Interface: `bash gate-sdk/bin/run-gates.sh --emit trajectory` writes the markdown table (one row per closed
iteration, stable columns) to stdout — the shape the committed projection
pins; bare invocation prepends a human-oriented header. The extractor degrades
per surface to an `n/a (<reason>)` cell and exits 0 — drift-kit's fail-visible
discipline, registering no gate. `DRIFT_KIT_TRAJECTORY_SURFACES` overrides the
harvested state-file paths (§Layout and configuration).

Consumer wiring (this repo, not kit mechanism): the emission is committed at
`docs/evidence-data.md`, and the consumer gate
`check-trajectory-fresh` (registered in `gates.list`, declared by
`scripts/check-trajectory-fresh.gate` and dispatching to the compiled binary
since gate-sdk/SPEC.md §The consumer remainder cohort) re-emits and
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
   grammar `<iteration> <stage> <session-id> <date> <head>` (owned by
   lifecycle-kit/SPEC.md §The state machine; this tool is a read-only consumer of
   that contract, it changes nothing there — its parse names a catch-all past the
   date, which is why the fifth field arrived here as a restatement to update and
   not as a reader to fix). The `<session-id>` field is not a raw
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
count.

The bound is **tighter than the stamp side alone**, because a dispatched
transcript *can* now be placed — through its anchor, by the fan-out row below.
What remains in the count is the transcript that resolved **no anchor**, and for
that residue it is still an **upper bound, never an attribution**: an unanchored
transcript carries no iteration and no stage *and descends from no session that
does*, so nothing in the join could place it. The reading that most of the
residue is ordinary non-lifecycle sessions becomes **more** true rather than
less, since the fan-out pass drains the lifecycle subtrees out of it. The one
case the fan-out row cannot reach is the unstamped **continuation**: it is a
resumption rather than a dispatch, so it has no parent edge to walk. Sizing that
blind spot is the whole of what drift-kit can do alone — attributing a
continuation would need the *stamp* side to record it, which is lifecycle-kit's
contract and no part of this meter's read-only consumption of it.

**The trend log.** One line is appended per `(iteration, stage, model)` triple to
`DRIFT_KIT_STAGE_ECONOMICS_LOG`, grammar:

```
<date> <iteration> <stage> <model> in=<tok> out=<tok> cr=<tok> cw=<tok> cost=<usd|n/a>
```

`<stage>` is the **stage-or-role** column — a lifecycle stage, a cost-bearing
role, or either of those with the fan-out suffix appended (the reserved
`supervision` value and the fan-out row, below). `cr` is cache-read and `cw` is
cache-creation. `cr` is the headline field: the
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
  they still are would misdiagnose the row as roster drift. The column widens
  once more for the fan-out row below, which admits *either* member with a
  suffix appended.
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
  **one row key** — either an `(iteration, stage-or-role)` pair or that pair's
  fan-out value. That is the rule the session-keyed join above enforces for stage
  rows, the reason a lead whose own transcript already carried a stage row yields
  no supervision row, and (below) the reason a dispatched transcript resolves
  exactly one anchor. One guard serves all three: a transcript already claimed by
  a row is never a candidate for another. In the ordinary case a lead supervises one
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
  second bound to drift. **A suppressed anchor suppresses its fan-out**: with no
  supervision row emitted the lead is no anchor, so its direct fan-out finds none
  and emits nothing. A fan-out row for a role that emitted no row of its own
  would be an orphan the reader cannot place, and the suppression is structural
  rather than a second rule — the anchor registers only where the row did.
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
  **The trajectory arm does not parse this column at all** — the reader worth
  naming, since it is the surface a hardcoded stage roster once broke; it reads
  stamps and its own `DRIFT_KIT_STAGES` roster, never this log. So a non-stage
  value in the column cannot silently fall out of a roster the way the
  trajectory's stamps did. The same three readers were re-verified when the
  column widened again for the fan-out value, and the conclusion is unchanged.

**The fan-out row — a stage's dispatched subtree is its own line item.** A stage
session that dispatches agents pays for their whole subtree, yet every dispatched
transcript carries no stamp of its own, so its burn lands in no row and each
per-stage figure understates that stage by its entire fan-out. The meter bills the
subtree to a row whose `<stage>` value is the **anchor's stage-or-role with
`DRIFT_KIT_FANOUT_SUFFIX` appended** (§Layout and configuration) — `build+fanout`,
`close+fanout`, `supervision+fanout`.

- **A suffixed value rather than a bare reserved label.** A bare `fanout` label —
  the shape `supervision` takes — would answer "what did fan-out cost this
  iteration" and lose *which stage's* fan-out it was, the question the motivating
  measurement asked. The suffix keeps the `(iteration, stage)` join intact and
  sorts the subtree row adjacent to its own stage row.
- **A separate row rather than a fold into the stage row.** Separability *is* the
  deliverable: a folded total answers the operator's question only through a
  stdout caveat the trend log does not carry, so the close-over-close series — the
  log's whole purpose — would be blind to the split it was extended to show. A
  fold would also silently redefine a value already logged, a `close` row written
  before this change and one written after meaning different things under one
  name, which is the drift the dedup key exists to prevent rather than to hide.
- **What it is not: a dispatch-type breakdown.** The row carries **no
  dispatch-type dimension** — a fork's spend and a typed dispatch's spend fold
  into one anchor total. It is an aggregate proxy for "what this stage's
  delegation cost", never an isolation of fork cost, and a reader who splits a
  posture decision on fork-versus-dispatch cannot get that split from this row.
- **The anchor set.** An anchor is a transcript that already holds a row of its
  own: every **stamped stage session whose transcript resolved**, anchoring its
  subtree to that stamp's `(iteration, stage)`; and every **lead that emitted a
  supervision row**, anchoring its own direct fan-out to
  `(iteration, DRIFT_KIT_SUPERVISION_LABEL)` — a lead's audit sweeps are as
  unbilled as a stage's, and omitting them would fix half the tier. The two
  conditions differ deliberately: a stage anchors on its stamp **resolving**,
  because a stamped stage whose transcript carries no usage is still a real,
  placeable `(iteration, stage)` for its subtree, while a supervision role that
  emitted no row is not a role at all. A lead that already carried a stage row
  yields no supervision row (the invariant above) and its fan-out anchors to that
  **stage** row instead — the anchor lookup finds it and needs no special case.
- **The producer — the parent walk, nearest anchor wins.** A dispatched
  transcript's *path* names only the root session: a grandchild sits flat in the
  same `<root>/subagents/` directory as a child, so the path cannot carry the
  parent edge. The harness writes it one file over, as a `.meta.json` sibling of
  each nested transcript, whose `parentAgentId` names the parent agent — a **bare**
  id, resolved against `agent-<id>.jsonl` in the same directory (the bare spelling
  `<id>.jsonl` is accepted as a fallback, so neither spelling is guessed). A record
  with no `parentAgentId` is a direct child of the root session that names the
  directory. For each `subagents/` transcript that is neither an anchor nor
  already holding a row, the meter walks parents by that rule until an anchor is
  reached and gives the **nearest** anchor the transcript's usage; intermediate
  non-anchor agents are transparent, which is what puts a deep fork under the
  stage session that ultimately caused it rather than under the sweep that
  happened to spawn it. A walk reaching the root session without meeting an anchor
  emits **no** row and the transcript stays in the under-count bound — the correct
  reading, since an ordinary non-lifecycle session's fan-out belongs to no
  iteration and no stage. The walk is **bounded by the transcript population and
  by a visited set**, so a cycle or a dangling `parentAgentId` costs a counted
  notice rather than a loop. The parent forest is read with one pass per
  `subagents/` directory rather than one per hop; the usage itself is the same
  last-usage-per-message-id reader the stage rows use, so the pass costs one
  transcript parse per attributed subtree member and no second parser.
- **Reading a harness-private artifact — why the coupling is admissible.** The
  meta sibling is under no contract, and the meter reads it anyway on one ground:
  it is the **same coupling class the meter already runs on**, one file over from
  the transcript JSONL usage schema and the `<root>/subagents/` layout it already
  depends on, and the failure mode of the shape changing is bounded to a *visible
  no-op* rather than a wrong figure. That is the difference between coupling to a
  shape for an **enrichment** and coupling to it for the pricing arithmetic; the
  ruling is yes here and would be no there. The rejected alternative — taking the
  key from a dispatcher-minted path instead — fails on the axis that decides it: a
  convention a dispatcher can forget produces a **silent** under-count in the exact
  tier this row exists to stop under-counting, it relocates the harness coupling
  rather than removing it (parsing a child's first user message for a string the
  meter must then trust), and it cannot reach a **fork** at all, since a fork
  inherits context rather than receiving an authored prompt.
- **Apportionment.** Where a lead spans several iterations, its fan-out
  apportions by the **same dispatch-count key, with the same integer split**, that
  apportions its supervision row. Reusing the key rather than minting a second one
  is the point: two keys over one lead would let the supervision row and its
  fan-out row disagree about which iteration the lead belonged to.
- **Several anchors, one row — the fold, not a race.** One `(iteration, stage)`
  can hold **several** anchors: a stage run as several sessions in one iteration
  (a batch split) stamps once per session, and each session anchors its own
  subtree. Their subtrees **sum into the single fan-out row** for that pair. This
  follows from the invariant above rather than adding to it — the attribution unit
  is the *row key*, not the anchor — and the alternative is not a second row but a
  **lost** one: two appends under one `<iteration> <stage> <model>` triple make
  the dedup key replace the first with the second, stranding its transcripts
  attributed to a row the replacement erased. Apportionment happens **first and the
  fold second**, since the dispatch-count split is a property of the anchor while
  the row is not. The stdout caveat names the contributing anchor count where it
  exceeds one, so a folded row is never read as one session's.
- **Collision rule.** The default suffix is collision-proof by construction — the
  stamp reader's stage-field alphabet is lowercase alphanumerics and hyphens, so a
  `+` can never appear in a stamped stage name. A consumer overriding the knob can
  break that, so the check mirrors the supervision label's: if any stamp the run
  reads names a stage **ending in** the suffix, the meter emits a visible notice
  naming `DRIFT_KIT_FANOUT_SUFFIX` and emits **no** fan-out rows that run. It is
  checkable from the stamps the meter already reads, so it adds no roster
  dependency and no second bound to drift.
- **Degradation — the contract that bounds the coupling.** Every arm degrades to a
  visible absence, never to a wrong figure, and never to a non-zero exit:

  | what is missing | behavior |
  | --- | --- |
  | the meta layer entirely | no fan-out rows; one notice; every existing row unchanged — the meter's behavior before this row existed |
  | one transcript's meta record | that transcript takes no fan-out row and stays in the under-count bound; counted in the unresolved notice |
  | `parentAgentId` naming an agent with no transcript | same — counted, never guessed |
  | a cycle or an over-long chain | same — the walk is bounded and the transcript counted |
  | `jq` absent | already fatal to the whole join upstream; the fan-out pass never runs |
  | the price table absent or missing a model row | the row's `cost` degrades to `n/a` and raises the existing incomplete-pricing caveat, exactly as a stage row's does |

- **No new field.** The four token fields, `cost`, and `date` carry a fan-out row
  exactly as they carry a supervision row; the dedup key stays the
  `<iteration> <stage> <model>` triple and a re-measure replaces the row's line
  like any other. The split, the transcript count, and every degradation stay
  **stdout caveats at measurement time** — a log field with no reader is a field
  removed, and none of these has one. The row's own named readers are the
  `/economics` narrative's fan-out line item (§The `/economics` skill), the
  operator reading the trend log close-over-close, and the deferred
  `benchmark-ab-experiment` rung's measurement half, which consumes this log
  rather than rebuilding it and inherits the row with no change.
  **No dispatch-type dimension either, and that is the answer to a question the
  row invites.** A fork is distinguishable at the meta layer — its record carries
  `isFork` beside an `agentType` naming the fork type, and carries no `model` —
  but the row folds a stage's whole subtree into one line, so "what did forking
  cost" is not a query this log answers. It is re-derived by walking
  `parentAgentId` and pricing the fork transcripts directly, which is the honest
  cost of leaving the dimension out until a reader for it exists.
- **No lifecycle change.** No stamp is added, no cursor moves, no stage-skill
  template changes. The fan-out edge is *derived*, exactly as the supervision edge
  is, which is what keeps this a read-only consumption of lifecycle-kit/SPEC.md
  §The state machine.

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

## lib/drift.sh

The kit's sourced knob resolution — values, never tool structure. It exists for
one reason: `gate-sdk/SPEC.md §lib/gate.sh`'s config bridge resolves a compiled
member's declared knob by sourcing **the owning kit's** `lib/*.sh`, and the owner
is derived from the knob's own `DRIFT_KIT_` prefix. So a knob this kit owns can
only be resolved from here; there is no other place that would work, and a knob
no library defines is the bridge's undeclared-knob refusal (gate-sdk/SPEC.md §lib/gate.sh).

**The trajectory arm's four knobs live here for exactly that reason, and their
move is what the port paid.** `DRIFT_KIT_CONFIG_FILE`,
`DRIFT_KIT_TRAJECTORY_SURFACES`, `DRIFT_KIT_GATES_FILE` and `DRIFT_KIT_STAGES`
were resolved inside `bin/trajectory.sh` while the extractor was a shell tool
that could source its own config. A compiled arm cannot: it reads the bridge, the
bridge sources this library and nothing else, so a default left in the tool would
have made every one of them the bridge's undeclared-knob refusal. **Sourcing the consumer
config is part of that**, and it happens here first — a knob a consumer config
sets would otherwise resolve to the platform default and silently ignore the
override, which is the failure mode that looks like success
(gate-sdk/SPEC.md §The non-gate arm). The config path itself takes
`DRIFT_KIT_KPIS_FILE`'s two adoption modes below, for the same reason: an
explicitly-set path that does not exist is adopted-but-broken.

`DRIFT_KIT_TRAJECTORY_SURFACES` keeps its **scalar** two-field shape
(`"<state-file> <evidence-file>"`) across the substrate move rather than becoming
an array, so a consumer that set it need not learn a second spelling; the reader
splits its two whitespace-separated fields exactly as the shell form's `read -r`
did.

**`DRIFT_KIT_KPIS_FILE` resolves to the KPI registry path, and the two adoption
modes are preserved here rather than at the reader.** They are the same two the
enforcement map states for every registry it reads — *adopted-but-broken refuses
where not-adopted degrades*:

- **Explicitly set to a path that does not exist** is adopted-but-broken: exit 2
  naming the knob. Under the config bridge that refusal fires inside the
  resolution subshell and refuses the whole invocation, so a consumer who
  misconfigured the registry gets a failure rather than a quietly shorter page.
- **Unset, with the default path absent**, is not-adopted: the knob resolves to
  the **empty string**, which a reader takes as *no registry, drop the section*.

**A guarded default would have collapsed both into the refusing mode** — the
idiom every other knob here could have used is wrong for this one, because it
erases the set-ness the two modes are told apart by, turning a consumer that
never adopted KPIs into a hard failure. Emptiness carries the not-adopted signal
instead, which is what lets **a reader reached through the bridge carry no
default**: a reader that had to recognise the default path would be a second home
for it.

**The default had a second home, and the port closed it rather than the filing
doing so.** The shell collator resolved the same knob inline with its own copy and
was not a bridge reader; once the collator became a bridged arm it reads what this
library resolves, so this library is the knob's only home. The report's other
knobs moved here for the same reason and by the same forcing: a default left
beside a compiled reader resolves to nothing, because the bridge sources this
library and nothing else.

## Layout and configuration

```
drift-kit/
  lib/drift.sh                   # sourced knob resolution; the config bridge sources it
  bin/kfric.sh                   # the knowledge-friction capture affordance
  bin/overhead-meter.sh          # the governance-overhead byte-proxy meter
  bin/stage-economics.sh         # the stage × model × iteration spend pricer
  templates/drift-config.sh
  templates/kpis.list            # the shipped registry: every bundled KPI (consumer copies + prunes)
  templates/kpi-deprecated-surface.sh   # example toolchain-shaped KPI (§Out of scope)
  templates/close-knowledge.md
  templates/price-table.tsv      # placeholder + schema for the stage-economics price table (consumer fills the roster)
  templates/economics.md         # the /economics close-cadence skill template
  smoke/install.sh
  smoke/overhead-fixture.jsonl   # synthetic transcript driving the classifier smoke
```

Registers no gates (advisory; the guard-kit precedent), so no `checks/`,
`gate-tests/`, or `smoke/violation.sh`.

`templates/kpis.list` names every bundled member, never a starter subset: it is
the kit's claim about what it bundles, which the roster above, this SPEC, and the
smoke's per-member row assertion are all stated over. Pruning is the *consumer's*
act on its own copy, and the template's header says so.
`gate-sdk/SPEC.md §check-template-registry-parity` holds the two in parity both
ways. The kit ships **no `kpis/` directory** now that the bundled set is
natively dispatched, so what puts this template in that gate's population is the
binary's own native-dispatch declaration rather than a sibling directory; the
gate's population predicate takes either. `templates/kpi-deprecated-surface.sh`,
an example a consumer adapts rather than a bundled member, stays out of it.

Config follows the established kit pattern: copy `templates/drift-config.sh`
into the gates dir (or point `DRIFT_KIT_CONFIG_FILE` elsewhere) and override
any knob; defaults fill what the consumer left unset, and a set-but-missing
`DRIFT_KIT_CONFIG_FILE` exits 2 rather than silently running on defaults.
Knobs (this repo's layout as defaults):

- `DRIFT_KIT_KPIS_FILE` — the registry; default
  `${GATE_SDK_GATES_DIR:-scripts}/kpis.list`.
- `DRIFT_KIT_KPI_DIRS` — extra resolution roots searched before the
  vendored kits' `kpis/` dirs and before the binary's built-in members
  (§The extensibility contract, tier 1); default: the consumer gates dir.
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
  `mkdir -p`s the log's dirname). Two resolvers compute this default — the
  meter, which is a standalone tool, and `lib/drift.sh`, through which the
  collator and `kpi-overhead` both now read it — and the smoke's writer/reader
  assertion holds them together (§Testing).
- `DRIFT_KIT_DONE_SECTION` / `DRIFT_KIT_DEFERRED_SECTION` — queue section
  headings the task-split and deferred-age KPIs scan; defaults `Done` /
  `Deferred` (queue-kit's).
- `DRIFT_KIT_ICEBOX_SECTION` — the design-pending pool's second section,
  queue-kit's optional icebox tier; default **empty**, meaning the pool is the
  deferred section alone. Read by `kpi-queue-net-delta` so an eviction reads as
  compression rather than as closure. The same independent-knob shape
  queue-kit and canon-kit carry: a consumer enabling the tier sets each, and
  one left unset degrades that kit to "no icebox".
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
  stage-economics meter prices through and `kpi-price-table-age` ages; default
  `${GATE_SDK_GATES_DIR:-scripts}/price-table.tsv` (beside `graph-vocab.sh`, the
  consumer-config precedent). Absent, cost degrades to `n/a` and tokens still report.
  Two sites compute this default, not three: `lib/drift.sh` for every bridge
  reader, and the standalone meter. The KPI's own restatement went with the port —
  a compiled member reads what the bridge resolved, so the substrate move
  *removed* a duplicate here rather than converting one into a cross-substrate
  pair, which is the outcome that was priced as a hazard.
- `DRIFT_KIT_SUPERVISION_LABEL` — the reserved value the stage-economics meter
  writes into the trend log's `<stage>` column for a lead's own burn
  (§The stage-economics meter, the reserved `supervision` value); default
  `supervision`. A consumer whose
  lifecycle roster already carries that word renames it here; a stamp naming the
  label collides and suppresses the rows for that run rather than blending two
  meanings into one column value.
- `DRIFT_KIT_FANOUT_SUFFIX` — the suffix the stage-economics meter appends to an
  anchor's stage-or-role value to name its dispatched subtree's row
  (§The stage-economics meter, the fan-out row); default `+fanout`. The default is
  collision-proof by construction (`+` is outside the stamp's stage alphabet); an
  override that a stamped stage name ends in collides and suppresses the fan-out
  rows for that run. Deliberately the *only* new knob: a meta-filename or
  field-name knob would imply a portability the mechanism does not have (a
  consumer on a different harness has a different artifact, not a differently
  named one), and an opt-out for reading the meta layer would either default on
  and be set by nobody or default off and ship the feature as dead code — the
  degradation contract already gives a consumer the only thing an opt-out buys.
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
both section headers and **at least** one row per registered KPI; a registry
naming a missing plugin yields its visible `n/a` row without failing; `--trend`
emits exactly one line; and, per registered name, that **the member still emits a
row when it is the registry's only entry** — a `solo` run, one name written to a
throwaway registry and collated through the real resolution and rendering path,
because the kit ships no bypass that would invoke a member on its own.

A floor, not equality, because a member owns one *or more* rows —
`kpi-queue-net-delta` (§Bundled KPIs) emits two by design — so equality would red
the moment the bundled set contains a multi-row member. The solo probe
beside it is what carries the bite the floor cannot: the report substitutes its
own `n/a (plugin failed)` row for a member that fails or emits nothing,
so the row count holds even where every member is silent, and a multi-row member
would in any case offset a sibling contributing none. Contribution is the
property, not health — an `n/a` row is a row — and a name resolving to no member
is skipped, that row being the report's own and asserted separately. The probe
asserts it probed something, so a resolution change cannot make it vacuous.
Residual limit, stated because it is real: neither assertion pins *which* member
emitted a given row, so a member emitting the wrong number of rows is not caught.
The trajectory extractor needs committed history the
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
notice. (iii) A *fan-out* fixture — a synthetic three-level tree: a flat
`<lead>.jsonl`, and under `<lead>/subagents/` a stamped stage transcript with a
`spawnDepth` 1 meta record, a child at `spawnDepth` 2 naming it as
`parentAgentId`, and a grandchild at `spawnDepth` 3 naming the child; plus a
**second session stamped into the same `(iteration, stage)`** carrying no
assistant usage, with a child of its own — the batch-split shape, which is also
the one that proves a stage anchors on its stamp *resolving* rather than on
emitting a row. It asserts exactly one fan-out row for the stamped stage whose
tokens are the **sum over the whole subtree** (so both a walk that stopped at
depth 2 and two anchors racing under the dedup key instead of folding red), that
the contributing anchor count is named where it exceeds one, that the stage
row still carries only the stage session's own usage (so a fold reds), that the
suffix comes from `DRIFT_KIT_FANOUT_SUFFIX` rather than a literal, that a stamp
whose stage ends in the suffix raises the collision notice and suppresses the row,
and that the under-count bound excludes every transcript the pass attributed.
Its two **degradation** assertions are what make the coupling's bounded failure
testable rather than asserted, and neither may be dropped for brevity: deleting
the grandchild's meta record moves it back into the under-count bound and raises
the counted unresolved notice while every row the pass does not own stays
byte-identical, and deleting the whole meta layer emits zero fan-out rows plus the
single notice with — again — every other row byte-identical.
`kpi-price-table-age` is fixture-stable in the same way — it reads two
dates out of a file the fixture writes — so `smoke/install.sh` drives it over
purpose-built tables and asserts the age row, the `price <N>d` trend fragment,
each header's absence degrading its own row independently, and — **the
inversion the KPI exists for** — that a table whose `priced-as-of:` is *today*
and whose `prices-valid-through:` has passed reads a fresh age row and an
`EXPIRED` expiry row in the same breath. A fixture that pins only the age row
has not tested the feature: the age row is reassuring in exactly that case,
which is the defect. The no-table degradation is asserted to emit its single
`n/a (no price table)` row, and the report-wide one-row-per-registered-KPI
assertion covers that shape from the other side. `kpi-incident-recurrence` is
fixture-stable on the same grounds — it reads declarations out of a queue file the
fixture writes — so `smoke/install.sh` drives it over a purpose-built queue and
asserts the single lag row, that the count **sums dates across declarations**
rather than counting declarations, the highest-count slug, the `recur <N>` trend
fragment, and both degradations (a queue with no declaration, and no queue file at
all) including the trend's silence under the first. Gate-sdk's `check-shellcheck`
lints all kit sources as usual.

## Out of scope

Toolchain-shaped KPIs are consumer content: orphan-crate and bare-`#[allow]`
scans (Rust-specific dead-surface detection) and a `TODO(spec-ambiguity)`
marker count (a marker convention is consumer vocabulary; generalize the
convention first if it ever ships). A deprecated-surface trend is the same
shape — it counts markers over the consumer's `CANON_KIT_DEPRECATION_MARKERS`
roster (canon-kit's `check-deprecation-task` vocabulary), so it ships as
`templates/kpi-deprecated-surface.sh`, an **example** the consumer registers in
its `kpis.list` rather than a bundled member: the marker spelling
is a consumer literal, and the kit stays deprecation-neutral. Registered, it
trends the live-marker backlog between majors so it surfaces gradually instead
of at one release; it degrades to `n/a` when the roster is unset (the bundled
plugins' fail-visible discipline). The release-boundary disposition walk over
the same roster is lifecycle-kit's `release-sweep` skill template. So are product-workflow KPIs: gate
exemptions (a `scan-exceptions` disposition split) and backlog-aging finding
counts — both read consumer gates. A narration-marker by-eye count is
superseded by canon-kit's `check-manifest-temporal`, which gates the same axis
instead of trending it. The always-loaded baseline mechanics are context-kit's
surface; drift-kit ships only the `kpi-always-loaded` built-in member that reads
its meter.
