# SPEC amendment: incident-recurrence-promotion-signal

A recurring incident's fix stays deferred on merit prose no ranking reads, so
recurrence never forces a promotion decision. Recurrence is captured faithfully —
in entry prose — and aggregated nowhere: the drift report counts defer-age, kfric
and gate-backlog, and `bin/queue-edges.sh` measures citation density rather than
incidence.

Three pieces, spanning three kits, on the split the queue entry already named:
**queue-kit the grammar, drift-kit the counter, lifecycle-kit the scope rule.**

## What changes

### 1. The `recurrence:` declaration {design-bearing}

A queue entry may carry one **declaration** line in its body, on the
`roadmap-summary:` / `close-surface:` pattern — a full line of its own, fixed
spelling, not a bracketed lead-line tag:

```
recurrence: <slug> <YYYY-MM-DD> [<YYYY-MM-DD>…]
```

`<slug>` is the entry's own slug. Each date records one **re-filing** of the same
finding; the initial filing is not a recurrence and is not listed. The count is
the number of dates. Dates are appended in order and never rewritten.

*Why it carries its own slug, redundantly.* Two reasons, and both are mechanism
rather than decoration. `check-queue-hygiene` rejects any exact-duplicate
non-blank line across the **whole file**, unnormalized — so a slug-free
`recurrence: 2026-08-04` on two entries stamped the same day would be a red gate,
and same-day recurrence on two entries is exactly the case this exists to
support. Naming the slug makes the line unique by construction. It also makes the
declaration resolvable by a single anchored grep with no entry-boundary parsing,
which is what lets the two consumers below read it without either of them
depending on the other's kit. Self-citation is already sanctioned prose
(queue-kit/SPEC.md §The tag algebra: an entry naming its own slug in its own body
is narration, not an edge), so this costs no new rule.

*Why one line with appended dates, not one line per recurrence.* One line per
recurrence is simpler to write and byte-unique on the (slug, date) pair, and it
is refused: it grows an entry linearly against `check-queue-entry-budget`'s
50-line cap, which means a **mechanical** writer could push a deferred entry over
a gate cap and red some later, unrelated session's commit — and that cap's remedy
is authorization-gated (queue-kit/SPEC.md §check-queue-entry-budget: a session
blocked by the cap does not self-serve the split). The single-line form costs one
entry line forever regardless of count. Its own ceiling is
`check-queue-wrap`'s 100 columns, reached after roughly four dates on a long
slug — and reaching it is the *correct* complaint: a slug recorded as recurring
four times that scope never pre-empted is the governance failure this unit exists
to prevent, surfaced loudly rather than absorbed.

*Why a declaration and not a tag.* queue-kit/SPEC.md §The tag algebra already
draws this line: a tag is bracketed and lead-line-scoped because its readers scan
lead lines alone; this is read off a line of its own. It also fails canon-kit's
further-tag test — it marks no move across a pending/ready boundary — and
`check-tag-lead-line` plus `check-queue-wrap` would squeeze a growing date list
on a lead line the way they already squeeze `[drain-exempt:]`'s reason.

*Why a hand-maintained roster is not what this is.* §The tag algebra refuses a
`relates:` declaration partly because every existing citation would need
hand-authoring. This line is **machine-written** by close's drain (delta 3) and
hand-read; no corpus needs migrating, and an entry with no declaration is simply
an entry that has not recurred.

### 2. `file-gap.sh` resolves the slug at capture — and writes no queue {design-bearing}

`bin/file-gap.sh` resolves its prose against the live slug set and, on a match,
records the match in the bullet it already writes:

```
- <YYYY-MM-DD> — recurrence of `<slug>`: <gap prose>
```

The slug in single backticks is queue-kit's existing in-body **citation** form,
so this borrows a spelling rather than inventing one. It also extends the tool's
existing point-of-capture stderr warning: the filer is told the finding is
already filed under that slug, which is worth knowing at the moment of filing.

**The tool does not touch `TASK-QUEUE.md`, and that is the load-bearing
constraint on this whole piece.** §The committed gap inbox exists *because* "a
gap surfaced mid-stage has no committed place to land except the queue file that
stage session is already contending on". A `file-gap.sh` that stamped a
recurrence declaration onto a queue entry would do precisely the thing the inbox
was built to prevent — contend on a live stage session's surface mid-iteration.
The queue write therefore belongs to close (delta 3), which writes the queue
anyway.

**Slug resolution is lifecycle-kit's own, not queue-kit's `queue_live_slugs`.**
The reach would be a new cross-kit code dependency, and drift-kit's
`kpi-deferred-age.sh` already records the standing ruling for exactly this shape:
a second implementation is accepted residual because the kit "cannot source
queue-kit's lib without a cross-kit cycle". lifecycle-kit already parses the queue
with its own awk (`check-stage-entry` assertion B) and takes the same residual
here. The set matched is **live** entries only — active, deferred, and a
configured icebox — so a slug already in the done section does not match: a
finding that recurs after its fix landed is a new defect, not a recurrence, and
files as one.

### 3. Close's drain stamps the declaration {design-bearing}

The close skill's drain step already dispositions every inbox bullet and writes
the queue in that commit. It gains one rule: a bullet naming a live slug stamps a
recurrence date onto that entry's declaration — creating the line if absent,
appending the date if present — **in addition to** its ordinary disposition,
never instead of it.

The drain **re-resolves the prose itself** rather than trusting delta 2's marker.
That is what makes the channel whole: §The committed gap inbox states that a raw
append into the inbox "stays a legal fallback, the grammar being the surface's
contract, not the writer", so a bullet filed without the tool carries no marker
and must still be counted. The marker is a capture-time convenience; the drain is
the channel.

Stamping is **idempotent per (slug, date)**: two filings of the same slug on the
same day record one date. The resolution is the day because the inbox bullet's
own grammar dates to the day — a finer resolution would be inventing precision
the capture channel does not have.

### 4. The pre-emption rule in scope {design-bearing}

`templates/stages/scope.md` gains a clause on its existing paragraph "**A standing
directive is a theme, not a unit list.**" — the paragraph that already states the
theme/unit boundary and names the escalation destination. The clause:

> A **deferred** entry whose recurrence count has reached
> `LIFECYCLE_KIT_RECURRENCE_THRESHOLD` enters the proposed unit set **regardless
> of theme**, and rides the same escalation the rest of the proposal does. The
> directive still bounds the survey; what it may no longer do is silently outrank
> a counted recurrence.

The rule reads the declaration directly (an anchored grep over the deferred
section), so it needs no tool and no queue-kit dependency. It is scoped to
deferred entries because promotion is the decision it forces — an entry already
active is being built.

**The collision is decided, not resolved in the theme's favour.** The rule does
not promote; it puts the unit in front of the ruling authority scope already
escalates to. That is deliberate: an automatic promotion would be a second
intake path around scope-gated intake, and the failure this unit measured was
never that the operator ruled wrongly — it was that the collision never reached
anyone.

**No `bin/queue-index.sh --recurrence` mode.** It was the obvious home and is
refused: drift-kit could not call it without the cycle delta 2 names, leaving
scope as its only reader, and queue-kit/SPEC.md §bin/queue-index.sh already
resists a fourth mode on the grounds that folding jobs together gives one tool
two output grammars. A fixed-spelling, self-slug-bearing, single-line declaration
is a one-grep read for both consumers, which is the property that makes the extra
name unnecessary.

### 5. `LIFECYCLE_KIT_RECURRENCE_THRESHOLD` {mechanical}

New knob, `<KIT>_<KNOB>` shape, registered in lifecycle-kit §Layout and
configuration. **Default `2`** — two recorded re-filings, i.e. a third incidence
of the same finding. Calibrated against the pool that motivated the unit: both
measured families (the validate-producer race and the fork failure mode) reached
that count before anyone acted. It is a stated policy with a stated purpose
rather than a derived one, and a knob for that reason — the same posture
`QUEUE_KIT_ENTRY_LINE_CAP` takes.

Its single reader is delta 4's rule. The threshold is lifecycle-kit's because the
*behavior* it governs is scope's; queue-kit owns the grammar and reads no
threshold, drift-kit reports the count and applies no verdict.

### 6. `kpi-incident-recurrence.sh` {mechanical}

A drift-kit KPI plugin on the existing contract (`kpi-<name>.sh`, execute bit,
`lead|lag<TAB><label><TAB><value>` rows, `--trend` fragment, config via exported
`DRIFT_KIT_*` env). It sums the declarations across the queue and reports the
count and the highest-count slug. Registered by name in the consumer's
`kpis.list`.

**Tiered `lag`, and the label is a measurement claim rather than a priority one.**
The honesty labels grade fidelity: lag is "undercounts by construction". A
recurrence nobody files is uncounted, which is exactly `kpi-knowledge-friction`'s
structure, so lag is the honest tier even though the metric is highly actionable.
Actionability rides delta 4's threshold, not the report's weighting — which is
precisely why the counter and the pre-emption rule are two pieces and not one.
A report-only signal would have reproduced the defect being fixed: a faithful
record no ranking reads.

The plugin re-implements the declaration scan rather than calling queue-kit,
under the same accepted-residual ruling `kpi-deferred-age.sh` records.

### 7. The interstitial lane, governed {design-bearing}

`lifecycle-kit/SPEC.md` §Deviation transitions gains a shape beside abandon,
split, reopen and close-merge: **the interstitial mitigation.** Between a close
and the next scope, a repo-local mitigation for a recurring incident may land
directly, while its queue entry stays open for the kit-shaped form.

The cap: **it adds no governed name.** That is canon-kit's feature/debt litmus
read the other way (canon-kit/SPEC.md §The amendment lifecycle — a task adding
any name to a governed surface is a feature and needs an amendment), cited rather
than restated. So an interstitial landing is admissible exactly when it is
debt-shaped; anything feature-shaped waits for scope, and the entry stays open
either way because the mitigation is repo-local and the entry's deliverable is
not.

This is the section's natural home on its own terms. §Deviation transitions is
where "the gate-legal shapes for leaving that walk are specified, not
improvised", and its constitutive constraint holds here: **no new tooling, state,
stamp grammar, or tag is introduced** by this delta — the cap is an existing
litmus and the landing is an ordinary commit. It also widens the shape already
adjacent to it: **Reopen after close** today routes a post-close defect to "a
debt entry and the follow-up iteration proceeds normally", which is the right
answer for a first occurrence and the wrong one for the fourth.

The accounting is already correct and needs no change — drift-kit/SPEC.md §The
published-evidence extractor states that an interstitial commit "falls into the
*next* iteration's range and surfaces only when that iteration closes". That
passage is descriptive of range arithmetic; it acknowledges such commits happen
and says nothing about when one may be made. It gains a pointer to the rule that
now says so, which is the whole of "the lane half-exists, the missing part is the
signal".

**Why not a parallel hotfix track**, recorded so it is not re-proposed: it
violates scope-gated intake, it contends on stage surfaces, and most incident
fixes are feature-shaped — so a "hotfix" of them is an unreviewed iteration.

## Producers and consumers

**The `recurrence:` declaration** (new state)

- **Producer** — the close skill's drain step (delta 3), in the commit that
  dispositions the inbox. Reachable at every close with no enabling config: the
  drain is already mandatory and the inbox's boundary refusal already forces it.
  There is no second producer, and no mid-iteration path writes it.
- **Consumers** — two, each by anchored grep over `TASK-QUEUE.md`, neither
  depending on the other's kit: (a) scope's pre-emption rule (delta 4), read once
  per iteration at the survey; (b) `kpi-incident-recurrence.sh` (delta 6), read on
  every drift report and `--trend` invocation.
- **Fields, each with a named reader.** `<slug>` — read by both consumers to name
  the entry the count belongs to, and structurally by `check-queue-hygiene`, whose
  duplicate rule the field is what satisfies. `<date>` list — its *cardinality* is
  read by both consumers; the *individual* dates are read by close's drain for the
  per-(slug, date) idempotence check, and by a human reading how tightly the
  recurrences cluster. No field is unread.

**The inbox bullet's recurrence marker** (new state)

- **Producer** — `bin/file-gap.sh` (delta 2), on a live-slug match. Live wherever
  the kit is vendored; the knob default makes the channel universal, as §The
  committed gap inbox already states for the bullet itself.
- **Consumer** — close's drain step. Its absence is not a hole: the drain
  re-resolves the prose independently (delta 3), so a raw-append bullet is counted
  identically. The marker's second reader is the filer, at capture, via stderr.

**`LIFECYCLE_KIT_RECURRENCE_THRESHOLD`** (new interface)

- **Producer** — the consumer's `lifecycle-config.sh`, or the kit default where
  unset. This repo leaves it at the default.
- **Consumer** — delta 4's rule, and nothing else.

**No new gate.** Every piece here is advisory tooling or a stated authoring rule,
and the one structural risk the grammar introduces — a duplicated declaration
line — is already caught by `check-queue-hygiene`, which is why delta 1's slug
field is mechanism rather than style.

**The provenance seam is held.** No term list, vocabulary or product constant
enters a kit: the declaration's spelling is fixed mechanism, the threshold is a
number behind a knob, and the incident *content* stays in the consumer's queue
where it already lives.

## Existing sections updated

Each names the delta that owns it:

- **queue-kit/SPEC.md §The tag algebra** — the declaration joins the surface
  beside `roadmap-summary:`, with the tag-versus-declaration distinction that
  section already draws (delta 1).
- **lifecycle-kit/SPEC.md §The committed gap inbox** — the bullet grammar gains
  the optional marker, the affordance paragraph gains slug resolution, and the
  closing "Producers and consumers" paragraph's claim that "each bullet's two
  fields have named readers" is restated for three (deltas 2-3).
- **lifecycle-kit/SPEC.md §Layout and configuration** — the new knob and its
  default (delta 5).
- **lifecycle-kit/SPEC.md §Deviation transitions** — the interstitial mitigation
  shape (delta 7).
- **lifecycle-kit/templates/stages/close.md** — the drain step's stamping rule
  (delta 3).
- **lifecycle-kit/templates/stages/scope.md** — the pre-emption clause on the
  standing-directive paragraph (delta 4).
- **drift-kit/SPEC.md §Bundled KPIs** — the plugin's entry and its lag-tier
  grounds (delta 6).
- **drift-kit/SPEC.md §The published-evidence extractor** — the interstitial-commit
  passage gains a pointer to the rule that governs the landing (delta 7).
- **`scripts/kpis.list`** — registration (delta 6).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
