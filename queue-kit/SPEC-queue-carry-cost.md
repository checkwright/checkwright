# SPEC amendment: deferred-queue-carry-cost

The deferred pool has an intake asymmetry the doctrine itself creates: gap
disposition and scope-gated intake make filing mandatory and cheap, while the
only exit is building the entry. This adds the missing exit and the
counter-pressure that keeps it working — an **icebox** tier for entries whose own
cost field says they are dormant, a per-entry size budget so the compression
sticks, and a net-delta KPI that makes intake-versus-drain visible.

Measured on the live queue at authoring (baseline `c003a56`; re-verified at
align, and every figure below reconciles at that baseline): Deferred is
**59 entries / 2,241 lines**. Of those, **27** open `Cost while deferred` with
low/zero/bounded/cosmetic and hold **903 lines** between them; **11** exceed 50
lines (3 of which are this iteration's own promotions, leaving **8** to
compress); **8** carry no cost field at all, 5 of them `[roadmap:]`-tagged
public-direction entries. **14** dated `Surfaced` marks exist file-wide against
46 `Filed <date>` lines — they sit on **13** deferred entries and collapse to 9
distinct dates — but only **2** deferred entries carry neither mark.

Two conventions the numbers depend on, stated because a re-measurement that
picks the other convention will not reproduce them. **Extents are raw** — an
entry's lead line through the line before the next bullet at the same or
shallower indent, trailing blank included, which is the same rule delta 3
assertion A states; trimmed, the 903 reads 874 and the 11 reads 10, and
`spec-internal-identifier-prefix-drift` sits exactly on the boundary at 51 raw /
50 trimmed. The median is **34** (not 36) and the 75th percentile 43, which is
what delta 3's cap calibration is read against.

## Seam ruling

**Kit mechanism:** the icebox tier and its one-line entry grammar, the entry-size
budget gate, the defer-date definition, the `--icebox-candidates` projection, and
the net-delta KPI. All of it is generic queue arithmetic.

**Consumer config:** whether an icebox exists at all (`QUEUE_KIT_ICEBOX_SECTION`,
default **empty** — a consumer with a twelve-entry backlog has no carry problem,
and a kit-shaped empty section on its queue would be the same posture leak
`QUEUE_KIT_HORIZONS` and `QUEUE_KIT_PROSE_SURFACE_GLOBS` already ship empty to
avoid), the cap value, and the triage age filter.

**Private rule content:** none. The cost-class opener set the candidate filter
matches on (`low`/`zero`/`bounded`/`cosmetic`) is generic English about cost, not
a domain vocabulary, and it steers a *worklist* rather than a verdict — see
delta 5.

## What changes

### 1. The icebox tier — a third live task section — *design-bearing*

`QUEUE_KIT_ICEBOX_SECTION` (default empty; this repo sets `Icebox`) names a `##`
section that joins **`QUEUE_TASK_RE` and `queue_live_slugs`** — it is a *live*
task section, placed **after the deferred section and before the done section**.
The read order is pickable → parked → dormant → history, and the placement is a
contract rather than a preference: a cross-kit reader depends on it (delta 11).

Joining the shared adapters rather than standing outside them is the whole design
of the tier, and it settles the placement question the queue entry left open:

- **Eviction is a conserved move.** `check-task-conservation` diffs
  `queue_live_slugs` at HEAD against the worktree, so a Deferred → Icebox move
  keeps the slug live and the gate stays silent — no sanctioned disappearance
  needs inventing, and none exists. Every exit from the design-pending pool is
  conserved: Deferred → Icebox (live → live), Icebox → Deferred on real
  recurrence (live → live), and Deferred|Icebox → Done for a ruled wontfix
  (delta 9), whose ruling lands as a one-line boundary note in the owning SPEC.
- **One namespace, one parse.** `check-task-names` keeps slug uniqueness and
  `[blocked-by:]` resolution across the whole pool; an iceboxed slug is still a
  legal blocker target, because it is unbuilt.
- **The living-prose contract keeps reaching it.**
  `check-queue-slug-liveness` resolves a `` **`<slug>`** `` token against the live
  set, so a page citing an iceboxed task stays green — correct, because the task
  is dormant, not landed.
- **Tags stay lead-line-governed.** `check-tag-lead-line` scans `QUEUE_TASK_RE`,
  so the icebox is covered with no gate edit.

Four gates therefore widen with **no change to any gate file** — the widening is
entirely in `lib/queue.sh`'s shared regex, which is what the one-adapter rule
was built to buy.

`check-queue-prose-precondition` stays scoped to the active sections and does not
reach the icebox: forward-looking phrasing is normal vocabulary in a parked
entry, the same exemption the deferred section already carries.

### 2. Icebox entry grammar, eligibility, and the tag state — *design-bearing*

An icebox entry is **its lead line and nothing else**:

```
- **<slug>** [<design-pending tag>] — <one sentence: what it is and why it is dormant>
```

- **It carries the same design-pending tag every deferred entry carries.** No
  `[icebox]` tag is minted: section membership *is* the state, and a tag
  restating its own section is the two-sources defect. What generalizes is
  canon-kit's **section set**, not its tag set (delta 7) — the tag keeps its
  section-wide meaning and every guard clause verbatim, which is precisely the
  state set the sibling token-rename unit governs.
- **No `###` subsections, no sub-tasks.** Grouping is presentation on a surface
  whose entire purpose is minimum residency, and a sub-task is a continuation
  line assertion B rejects.
- **A `[roadmap:]`-tagged entry is not icebox-eligible**, and that rule needs no
  new code. `queue_roadmap_entries` walks `QUEUE_TASK_RE`, so the icebox enters
  its walk with the tier; `check-roadmap-fresh` assertion C then reds any
  `[roadmap:]` entry carrying no `roadmap-summary:` declaration — and a one-line
  entry has nowhere to put one. **The enforcement is conditional, and the SPEC
  says so:** that gate exits clean at its top when `QUEUE_KIT_ROADMAP_FILE` is
  empty, *before* assertion C runs, so the guarantee holds for a consumer that
  publishes a roadmap page (this repo does) and degrades to convention for one
  that does not. The eligibility rule binds either way; only its mechanical
  backstop is knob-conditional. Keeping the icebox *inside* the roadmap walk
  rather than excluding it is the deliberate call: excluded, a `[roadmap:]` tag
  that drifted into the icebox would silently drop a public commitment; included,
  it reds. Nine deferred entries are `[roadmap:]`-tagged today and all nine stay
  in Deferred.
- **The narrative is recoverable, not discarded.** The commit that iceboxes an
  entry necessarily *contains the removed body in its own diff*, so recovery is
  `git log -p -S'<slug>' -- TASK-QUEUE.md` — a pickaxe over the queue file that
  depends on no commit-message convention. This is the recovery path the SPEC
  names; nothing is copied into the queue to point at it.

**Eligibility (judged at close, delta 8):** the entry's cost field opens in the
low class; it carries no `[roadmap:]` tag; and it has **no live promotion
trigger** — an entry whose promotion waits on a named expected event (a launch, a
first external adopter) stays in Deferred, because it has a trigger, just not a
date.

**Nothing files directly into the icebox.** The only producer is the eviction
step; a newly filed finding has never been triaged and so cannot be pre-judged
dormant. This keeps the tier from becoming a dumping ground and keeps delta 6's
net-delta honest.

### 3. `check-queue-entry-budget` — *design-bearing*

A new queue-kit gate, `precommit` tier, manifest
`# graph: couples=TASK-QUEUE.md dir=one valve=none tier=precommit`, taking the
queue file as `$1` for its fixtures. One invariant stated from two sides — **a
deferred entry is a costed filing: bounded above so it is not an inlined
amendment, bounded below so it is not a flag-and-skip** — in three assertions:

- **(A) Size.** No deferred entry exceeds `QUEUE_KIT_ENTRY_LINE_CAP` lines,
  counted as the lead line through the line before the next bullet at the same or
  shallower indent (an entry's own lines; a sub-task is its own entry).
- **(B) Icebox shape.** Every icebox entry is exactly one line — a continuation
  line under an icebox bullet is a violation. Skips clean when
  `QUEUE_KIT_ICEBOX_SECTION` is empty, matching **`check-queue-slug-liveness`'s
  empty-globs behavior** — the origin of that pattern, which
  `check-roadmap-fresh` is itself a user of rather than the precedent for.
- **(C) Cost present.** Every deferred entry carries a `Cost while deferred` bold
  lead-in. Deliberately **not** applied to the icebox: a one-line entry cannot
  carry the field, and it does not need to — *membership in the tier is itself
  the cost declaration* (low, non-rotting, no live trigger). The requirement
  binds exactly where the section does not already imply the answer.

**`QUEUE_KIT_ENTRY_LINE_CAP` = 50.** The cap's job is to keep compression from
regrowing, not to force the initial cut — (a) and (d) do that — so it is
calibrated at the tail rather than the body: 50 sits above the 75th percentile of
today's distribution (median 36) and binds 8 entries after this iteration's three
promotions leave the pool. There is no natural break in the distribution to read
a value off, so the number is a stated policy with a stated purpose rather than a
derived one, and it is a knob for exactly that reason.

**The active sections are uncapped**, and the reason is structural: an active
entry's residency is one iteration by the drain rule, so it has no carry to cap.
The carry problem is the deferred pool's alone.

### 4. One defer-date definition, two readers — *design-bearing*

The **defer date** of an entry is its `Surfaced <date>` mark when present, else
the date on its `Filed <date>` provenance line. Owner section: queue-kit/SPEC.md
§The queue format.

This is a correction with evidence behind it. `kpi-deferred-age` reads `Surfaced`
alone, and practice has moved to `Filed <date> by <stage>`: 14 dated marks
against 46 filed lines, resting on **13 of 59** deferred entries. Under the
fallback the input covers **57 of 59**. Stated precisely, because the KPI does
not count entries — it de-duplicates the dates it finds and reports the oldest,
so today's input is 9 distinct dates drawn from 13 entries, and the widening
takes it to 57 entries' worth. Widening, not replacing — the existing marks
keep their meaning (`Surfaced` records when the *premise* was observed and is the
better premise-rot datum; `Filed` is the honest available fallback), so no
migration of existing entries is owed.

`kpi-deferred-age` takes the widened definition. **Accepted residual:** the
definition has one documented owner and two implementations, because drift-kit
cannot depend on queue-kit's `lib/queue.sh` without a cross-kit cycle — the same
reason `kpi-deferred-age` already re-implements the section scan inline. Both
implementations carry a `spec:` line citing the owner section.

**The two re-implemented scans in the tree disagree on a new section, and the
divergence is deliberate rather than overlooked.** `kpi-deferred-age` resets on
any unrecognized `## ` heading, so an icebox placed after Deferred drops out of
its input automatically — correct, since an iceboxed entry's age is no longer
the thing the KPI is trending. gate-sdk's `check-gate-exemption-tasks` has no
such reset and sweeps the icebox in (delta 11) — also correct, for the opposite
reason. Same shape, opposite behavior, both wanted; recorded so a later reader
does not "fix" one into agreement with the other.

### 5. `bin/queue-index.sh`: the icebox tally and `--icebox-candidates` — *design-bearing*

- **Tally line, both deferred renderings.** The icebox contributes exactly one
  line (`Icebox: N entries`) to the index — never an entry listing. The index is
  embedded in the always-loaded session-context surface, so listing the tier
  would re-import the tokens the tier exists to remove, while a bare count keeps
  it visible. **Precisely: the two `END`-block branches of `index` mode** — the
  `--collapse-deferred` branch and the full one. `--collapse-deferred` is a flag
  within index mode, not a mode; the tool's actual modes are `index` and
  `extent`, and the tally must **not** reach `extent`, whose contract is two
  integers and nothing else. The section dispatch needs a new icebox arm: an
  `## Icebox` heading currently falls through to the catch-all and is invisible.
- **`--icebox-candidates`** prints the eviction worklist: one line per deferred
  entry whose defer date is older than `QUEUE_KIT_ICEBOX_AGE_DAYS` (default 30),
  with the entry's line count and its cost-field opener. An entry with **no**
  defer date is listed as `(undated)` rather than filtered out — the
  `(undeclared)` row precedent from the close-surface roster: an absent input
  appears rather than vanishing.

The age value is deliberately **non-load-bearing**: this is a worklist filter in
a projection tool, not a threshold in a gate, so miscalibration costs a longer or
shorter review list and never a wrong disposition. That reframing is what
dissolves "an eviction age threshold needs a policy owner" — the *judgment* is
close's, and the filter only bounds how much close must look at. Matching a
cost-class opener on prose would be an unacceptable heuristic in a gate; in an
advisory worklist it is exactly the right ceiling.

No queue-mutating tool is added — `--extent <slug>` already yields the line range
an eviction deletes.

### 6. `kpi-queue-net-delta` and the iteration-start handoff — *design-bearing*

A new drift-kit lead KPI, registered in `scripts/kpis.list`. It compares the
design-pending pool at the iteration-start commit against the worktree and emits
**two rows, because one number would be gameable**:

- **Entry net delta** — `filed − drained`, where *filed* is a slug now in
  Deferred that was in neither section at the baseline, and *drained* is a slug
  that has left the pool entirely (promoted or Done). An icebox move counts as
  **neither**: it is compression, not intake and not closure.
- **Carry weight** — the line count of the two sections now against the baseline.

They are the two axes the unit is actually about, and they move independently:
intake pressure moves the first, compression moves the second. A session that
mass-iceboxed to make the number look good would move the weight row and leave
the delta row untouched — the gaming is visible rather than hidden, the same
argument `kpi-price-table-age` makes for carrying two rows that point different
ways. `--trend` emits one fragment, `qnet <+N>`, per the one-per-plugin rule; the
weight row volunteers none, because intake is the axis a filing session can act
on inside the session.

**Enabling handoff:** `drift-report.sh` already derives the iteration-start
commit for its header, but only at print time — after the plugin loop. **Both
the `iteration_start` function definition and its invocation** move above the
`DRIFT_KIT_*` export loop (a two-part move — relocating the call alone leaves it
undefined), assigning `DRIFT_KIT_ITERATION_START`, so the existing `compgen`
export carries it to plugins with no new export list to drift out of parity. Its
inputs are already set by that point. It is a driver handoff recomputed every
run, not a consumer knob — the same *class* as `DRIFT_KIT_KIT_ROOTS`, though not
its mechanism: `KIT_ROOTS` is set after the loop and carries its own explicit
`export`, so it is a precedent for the handoff shape and not for riding compgen.
The header keeps reading the same value. With no baseline (a standalone run, a
fresh clone) the plugin degrades to `n/a (no iteration baseline)` per row.

**Priced, because the move is not free:** hoisting the derivation above the loop
makes it run in `--trend` mode too, which today exits before ever computing it.
That puts one `grep` and one `git log -S` over `.workflow/WORKFLOW-STATE.txt` on
**every session start** via the context hook. Accepted — it is a single-file
pickaxe over a file of a few dozen lines — but it is a per-session cost, not a
per-report one, and a build session should not discover that after landing it.

### 7. canon-kit: the design-pending section set — *design-bearing*

`CANON_KIT_ICEBOX_SECTION` (scalar, default empty), the independent-knob shape
`CANON_KIT_DEFERRED_SECTION` and `QUEUE_KIT_DEFERRED_SECTION` already use (either
kit runs without the other; a consumer enabling an icebox sets both — three kits
now, with drift-kit's in delta 6). `lib/spec.sh` builds the design-pending regex
from deferred **plus** icebox, omitting the icebox term when the knob is empty.

`check-amendment-queue` assertion (b) then reads "every entry in a design-pending
section carries **the design-pending tag**, and one already carrying `[spec:]`
must be promoted" over both sections. Assertions (a) and (c) are untouched.

The tag is written abstractly here on purpose: the sibling `needs-spec-tag-rename`
unit renames that token to `[design-pending]` in this same iteration, and this
amendment's body is itself live grammar destined to merge into a canonical spec.
Whichever unit lands second must not merge the other's stale spelling.

### 8. Close's eviction step and its roster declaration — *design-bearing*

The eviction triage is a **judgment step in close**, and it declares itself so a
skip is visible rather than silent:

```
close-surface: TASK-QUEUE.md#Deferred advisory
```

`advisory` is the honest mode: no structural forcing function refuses on an
untriaged backlog, and inventing one would red an unrelated commit over a
curation opinion. The declaration lands in queue-kit/SPEC.md §The icebox tier —
the section that owns the surface — and `bin/close-surfaces.sh` picks it up from
the kit-root sweep with no registry edit. This is what makes eviction *symmetric*
to promotion: promotion is forced by scope's exit condition, eviction gets roster
visibility and an auditable skip.

The procedure itself lands in **this repo's** `/close` housekeeping binding
(`.claude/commands/close.md`), beside the audit-roster and trajectory steps: run
`--icebox-candidates`, and for each row either evict (rewrite the lead line as a
self-contained sentence, delete the body, move it under `## Icebox`), rule
wontfix (delta 9), or keep it in Deferred with the trigger that keeps it there.
No lifecycle-kit template change: the kit template's step 3 already routes to the
binding's per-surface procedures.

### 9. The one-time sweep — *design-bearing*

Enforcement-first binds this to delta 3 in one unit; the sweep is what makes the
gate green rather than a bypass, and it is design-bearing throughout — every part
of it authors prose or rules on a task:

- **Compress the 8 over-cap entries** to ≤50 lines. The excess narrative is not
  discarded: the compression commit's own diff carries it, recoverable by the
  pickaxe in delta 2. No detail-file convention is invented.
- **Add a cost line to the 8 uncosted entries** (5 of them the `[roadmap:]`
  public-direction cohort). The gap-disposition rule already required this; the
  gate converges behavior on an existing doctrine rule rather than minting a
  policy.
- **Seed the icebox** from `--icebox-candidates`: at most 27 by the cost-class
  filter, less the one `[roadmap:]` overlap (`preview-release-cadence`), less
  whatever the triage judgment keeps. Each eviction authors a one-sentence lead
  line.
- **Rule the wontfix cases** whose own text already admits the outcome: the
  ruling lands as a one-line boundary note in the owning SPEC and the slug moves
  to `## Done`. That is a completed task — the decision was made and recorded in
  an owner doc — so Done is the honest home and conservation holds.

  **The move is a rewrite to a bare slug line, not a relocation of the entry**,
  and the distinction is load-bearing. `queue_done_slugs` matches only
  `- <slug>` — no bold, no tag, no trailing prose. An entry carried into Done
  with its `- **<slug>** [<the design-pending tag>] — …` shape intact matches
  neither that adapter nor `queue_live_slugs`, so the slug lands in neither set
  and `check-task-conservation` reds it as a lost task. Conservation holds for
  this path *because* the entry is reduced to its slug; the design-pending tag
  is dropped by that reduction rather than swapped, which is also why the
  sibling unit's token migration must not visit these entries (its delta 6).

Projected: ~2,241 → ~1,040 lines in the design-pending pool, with 27 of 59
entries reduced to one line each.

### 10. Configuration, registration, and fixtures — *mechanical*

- `lib/queue.sh`: the three new knobs with their defaults and validation, the
  `QUEUE_TASK_RE` widening, and — **derivation, not a second maintained list** —
  appending `QUEUE_KIT_ICEBOX_SECTION` to the effective
  `QUEUE_KIT_REQUIRED_SECTIONS` when it is non-empty. This closes the
  half-configured hole by construction: an icebox configured but absent from the
  file would otherwise let every icebox assertion pass open on a section that is
  not there, which is the exact fail-open class `check-queue-sections` exists to
  stop.
- `scripts/queue-config.sh` sets `QUEUE_KIT_ICEBOX_SECTION=Icebox`;
  `scripts/canon-config.sh` and this repo's drift config set their counterparts —
  the enabling config is live in a real configuration, not test-only. The cap and
  age knobs keep their kit defaults and stay absent from the consumer config (a
  consumer line restating a kit default is the de-literalization defect).
- `scripts/gates.list` gains `check-queue-entry-budget`; `scripts/kpis.list` gains
  `kpi-queue-net-delta`.
- Fixture pair `queue-kit/gate-tests/check-queue-entry-budget/{good,bad}/` with
  `TASK-QUEUE.md` + `expect.txt`, each supplying `scripts/queue-config.sh` to set
  the icebox knob — the runner cds into the case dir, the arrangement
  `check-roadmap-fresh/good/` already uses. `bad/` must trip all three assertions
  so each carries its own rejection substring.
- `templates/TASK-QUEUE.md` is **unchanged**: the icebox default is empty, so the
  starter template ships no icebox section and stays battery-clean.
- Regenerate the three projections after registration: the pre-commit hook
  (`gen-pre-commit.sh --write`), the graph artifact, and the enforcement map;
  plus the docs mirror.

### 11. Two cross-component readers the survey turned up — *design-bearing*

A whole-tree reader sweep (not a hand-picked subset, no silenced stderr) found
two consumers of the design-pending pool outside queue-kit and canon-kit. Both
are resolved by specification rather than by code, and the reasoning is recorded
because each looks like a non-event until it is not:

- **`check-gate-exemption-tasks` (gate-sdk) depends on the icebox's *position*.**
  It resolves an `# until: <slug>` exemption against a live-slug set built by
  hardcoding the section headings: it opens on New Features / Technical Debt /
  Deferred and closes on Done / Lessons Learned, with **no reset on an unknown
  heading**. A section placed between Deferred and Done is therefore swept in by
  default — which is why an exemption backed by an iceboxed task keeps resolving.
  That is an accident, and the amendment refuses to leave it as one: the icebox's
  placement **between the deferred and done sections is a stated contract**, and
  gate-sdk/SPEC.md §check-gate-exemption-tasks names this gate as the reader that
  depends on it. **Accepted residual, and it is quiet — no gate enforces section
  order:** nothing in the tree checks where a section sits.
  `check-queue-sections` counts occurrences per required section with no
  positional state, and no other gate carries one. An icebox placed after the
  done section drops those slugs from that set, and the *only* way that surfaces
  is an exemption naming an iceboxed slug reddening with "no live task" — which
  requires such an exemption to exist. Otherwise the misplacement is silent. The
  contract is therefore carried by the SPEC and by review, not by a gate, and
  saying so is the honest form of the residual. The underlying fragility is
  pre-existing, is not this unit's to fix (gate-sdk cannot depend on queue-kit
  for the section set), and is filed to the gap inbox rather than absorbed here,
  per scope-gated intake — the filed bullet understates it, and align has filed
  the correction: the reader scans **every bold token on every line** in the
  span, not bullet lead lines, so its live set is 109 tokens against 61 real
  slugs and `# until: scope` resolves green against no task at all.
- **context-kit's session-context hook is a consumer of the tally line.** Step 1
  of the hook runs `queue-index.sh`, collapsing the deferred section except on
  the close and scope stages. The icebox tally (delta 5) rides **both** branches,
  so the hook needs no change and no new stage routing: an icebox entry is one
  line, so there is no "full listing" of it to withhold, and close's eviction
  work reads `--icebox-candidates` rather than the index. The integration prose
  in context-kit/SPEC.md is updated to say so, because a reader of that step
  would otherwise have to infer which branch the new tier lands on.

### 12. Release-note declaration — *mechanical*

Added at align: the unit lands a registered gate on a vendored consumer's tree
and so owes a note bullet, which the amendment did not declare.

`check-queue-entry-budget` declares under **Tightened gates**, whose grammar is
"one bullet per gate that landed **new** or got stricter" and whose lead tokens
are the release's mechanical allowed-red set (docs/install.md §The upgrade
contract). It is an allowed-red on a real consumer, not a formality: assertions
A and C bind the deferred section unconditionally, so any vendored queue with an
over-cap or uncosted entry reds on upgrade. Only assertion B is knob-gated.

Nothing else here earns a bullet. The three new knobs are **added, not renamed**,
and `kpi-queue-net-delta` and `kpi-deferred-age`'s widened input are advisory
KPIs that never join `gates.list` — the two near-misses docs/install.md:414
names as trailing-clause material rather than bullets.

This unit rides **one minor with its two siblings and one shared release note**;
none of the three owns the note alone, and each declares its own bullets into it.
The floor is minor on this unit's Tightened-gates bullet alone.

**The shared note's Renamed-knobs section reads "None." for this release**, and
the clause that qualifies it routes rather than denies (:415 forbids a clause
that only restates the heading's negation). Both sibling renames declare under
Behavior changes — a queue tag and a template path are consumer content and
copied-out-template residue, not own-config knobs — so the clause names this
unit's added-not-renamed knobs *and* points at Behavior changes for the two
renames. Recorded here because this unit is the only one of the three with
nothing in that section, so a build session assembling the note from three
amendments would otherwise read three silences as an empty body with no clause.

## Producers and consumers

- **The icebox section.** Producer: close's eviction step (delta 8) — the only
  writer, enabled by `QUEUE_KIT_ICEBOX_SECTION` in `scripts/queue-config.sh`.
  Consumers: `queue_live_slugs` (→ `check-task-conservation`,
  `check-queue-slug-liveness`), `QUEUE_TASK_RE` (→ `check-task-names`,
  `check-tag-lead-line`, `queue_roadmap_entries`), the derived required-sections
  set (→ `check-queue-sections`), `check-queue-entry-budget` assertion B,
  `queue-index.sh`'s tally line, `kpi-queue-net-delta`, canon-kit's
  `check-amendment-queue` assertion (b), and — by position rather than by name —
  gate-sdk's `check-gate-exemption-tasks` (delta 11).
- **The icebox tally line.** Producer: `bin/queue-index.sh`, both modes.
  Consumer: context-kit's session-context hook (step 1) → every session's brief.
- **`QUEUE_KIT_ENTRY_LINE_CAP`.** Producer: `lib/queue.sh` default. Consumer:
  `check-queue-entry-budget` assertion A, its only reader.
- **`QUEUE_KIT_ICEBOX_AGE_DAYS`.** Producer: `lib/queue.sh` default. Consumer:
  `queue-index.sh --icebox-candidates`, its only reader.
- **The defer date.** Producer: the filing session's `Surfaced`/`Filed`
  provenance line — already emitted on 57 of 59 entries, so the consumers are
  live on real data, not on a convention that has to be adopted first. Consumers:
  `kpi-deferred-age` and `--icebox-candidates`.
- **`DRIFT_KIT_ITERATION_START`.** Producer: `drift-report.sh`, recomputed every
  run before the export loop. Consumers: `kpi-queue-net-delta` and the report
  header (its existing reader, unchanged).
- **`kpi-queue-net-delta`'s rows.** Producer: the plugin. Consumers: the
  collator's lead section; the `qnet` fragment → context-kit's session-context
  hook → every session's trend line.
- **`check-queue-entry-budget`.** Producer: the pre-commit hook via its `# graph:`
  manifest and `scripts/gates.list`. Consumer: the committing session.
- **The `close-surface:` declaration.** Producer: queue-kit/SPEC.md §The icebox
  tier. Consumer: `bin/close-surfaces.sh` → close's step-3 roster.

Every field on every new line is read: the icebox lead line's slug by the four
widened gates, its tag by `check-amendment-queue` and `check-tag-lead-line`, its
prose by the reading session. `--icebox-candidates` prints line count and cost
opener, both read by close's eviction judgment. No field is added without one.

## Existing sections updated

queue-kit/SPEC.md — §The queue format (the icebox bullet, delta 1; the
four-field roster gains the `Cost while deferred` requirement and its gate,
delta 3; the defer-date definition, delta 4), §The tag algebra (the
design-pending tag now spans two sections, delta 7; `[roadmap:]`'s
"unconstrained by section" clause names the icebox exclusion and the
knob-conditional limit on assertion C's backstop, delta 2), §Layout and
configuration (three knobs, the derived required-sections rule, the three-kit
cross-kit note, delta 10), §lib/queue.sh (`QUEUE_TASK_RE`, the derivation,
delta 10), §bin/queue-index.sh (tally line on both index-mode branches,
`--icebox-candidates`, delta 5), §bin/roadmap.sh (reads three live task
sections, delta 1), §check-queue-sections (delta 10), §check-task-names,
§check-task-conservation (including the bare-slug Done grammar the wontfix path
depends on, delta 9), §check-tag-lead-line, §check-queue-slug-liveness,
§check-queue-prose-precondition (one sentence each on icebox reach or deliberate
non-reach, delta 1), plus new §The icebox tier (deltas 1, 2, 8) and
§check-queue-entry-budget (delta 3).

canon-kit/SPEC.md — §The amendment lifecycle (design-pending spans the icebox),
§check-amendment-queue (assertion b), §Layout and configuration (the knob) —
all three delta 7.

drift-kit/SPEC.md — §The report skeleton (the baseline function and call moved
above the export loop, and the per-`--trend` cost, delta 6), §The KPI plugin
contract (`DRIFT_KIT_ITERATION_START` as a driver handoff, delta 6), §Bundled
KPIs (`kpi-deferred-age`'s widened input and its unknown-heading reset, delta 4;
`kpi-queue-net-delta`, delta 6).

gate-sdk/SPEC.md — §check-gate-exemption-tasks (the live-section span it reads
and the icebox's stated position within it, delta 11).

context-kit/SPEC.md — the session-context hook's step 1 (the icebox is a tally on
both branches, delta 11).

queue-kit/README.md — the section roster gains the optional icebox tier
(delta 1).

`.claude/commands/close.md` — the housekeeping binding's eviction step (delta 8).

docs/posts/ — the release note's Tightened-gates bullet (delta 12).

**Deliberately not updated: `CLAUDE.md`.** Load-trigger residency — close's
binding loads the eviction procedure and queue-kit/SPEC.md owns the tier, so a
standing per-session line would be resident for every session that never touches
it. A build session should not add one.

## Ruled out

- **A separate icebox file.** Every section-scoped gate reads one queue file, and
  `check-task-conservation`'s HEAD-versus-worktree diff is single-file by
  construction, so a Deferred → file move would read as a lost task and the slug
  namespace would need two parses. The compression, not the file boundary, is
  what buys the tokens: a one-line entry costs ~1/33 of the average deferred
  entry whether or not it moves house, and `--collapse-deferred` already proves
  the read-cost lever is a projection rather than a file.
- **An `[icebox]` tag.** Section membership is the state; a tag restating its own
  section is the two-sources defect the tag algebra already avoids.
- **A `draft:` pre-amendment state.** Decisive on triage evidence rather than on
  principle: the icebox compresses only entries whose cost field says the
  derivation was cheap, so a `draft:` file would serve a population the icebox
  never touches — the expensive-derivation entries stay in Deferred with full
  bodies. Structurally it is also an amendment with the age pressure removed,
  which `kpi-amendment-age` exists to apply; it would anchor an authoring session
  on a rotted file that reads as authority (spec-over-precedent: a draft is
  neither owner doc nor history); and it would need a new lifecycle state in
  `check-amendment-queue`, since a deferred entry carrying `spec:` must today be
  promoted. The commit-diff recovery path in delta 2 delivers the reusable
  thinking at zero new governed names.
- **A gate on the wontfix path.** Asserting that a slug moving to Done names a
  SPEC boundary note needs commit-diff correlation across two files and would
  fire on every ordinary build. The close procedure and the clearing-is-not-
  processing rule are the honest ceiling.
- **Requiring the defer-date mark.** Its reader degrades visibly — an undated
  entry appears as `(undated)` on the worklist, and close reads the whole section
  at its backlog-aging review regardless. The cost field has no such fallback: an
  uncosted entry violates the gap-disposition rule whether or not anything reads
  it. That asymmetry is why assertion C requires one field and not two.
- **A queue-mutating `bin/icebox.sh`.** queue-kit ships projections, not
  mutators, and `--extent` already yields the range an eviction deletes.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls queue-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Sweep landed with the gate** — `check-queue-entry-budget` is green on the
      live queue at the same commit that registers it; no `--no-verify`.
