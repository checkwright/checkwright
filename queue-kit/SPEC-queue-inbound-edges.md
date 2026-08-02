# SPEC amendment: queue-inbound-edges

Queue entry: **`queue-inbound-edge-projection`**. The derivable half of
**`scope-survey-counter-evidence`**.

The defect this closes was diagnosed precisely by the entry that filed it, and the
diagnosis is what constrains the design: **the failure was aggregation, not
retrieval.** A scope survey read the sibling entries — it even edited one in the
same session — and still ranked a unit wrongly, because nothing anywhere sums an
entry's *inbound* edges into that entry's own cost/benefit. Three entries each say,
in their own bodies, that promoting `native-gate-binary-port` closes or converges
with them. Reading any one of them tells you nothing. Reading all three together is
a promotion dividend, and no surface performs that sum.

So the deliverable is a sum, not a rule. A prose instruction to "follow the
cross-references" would have changed nothing: they were followed.

## What changes

### 1. The edge grammar — declared, not invented {design-bearing}

The queue does not today declare that one entry may cite another, which is why
this half was `[design-pending]`. It gains that declaration in
queue-kit/SPEC.md §The tag algebra, and the whole ruling is that **the grammar
already in use becomes the governed one**:

> Inside a task section, a **single-backtick token** matching the slug grammar
> (`[a-z0-9][a-z0-9-]*`) appearing in an entry's body — anywhere but its own lead
> line — is a **citation** of the named entry.

Four rules make it parseable, each covering a shape the live corpus actually
contains:

- **Resolution is against the live slug set** (`queue_live_slugs`: active,
  deferred, and the configured icebox — the existing source of truth).
- **An unresolved token is not an error.** It is simply not an edge. This is the
  load-bearing rule: entries legitimately cite *landed* work by name — a defect
  class, a shipped contract, a closed ruling — and that citation is valuable prose
  which no gate may punish. It is also why this unit installs no liveness gate
  over the queue (delta 5).
- **Self-citation is not an edge** — an entry naming its own slug in its body is
  narration, not a relation.
- **`[blocked-by: <slug>]` is an edge too**, and the one already-structured class.
  It is included so the inbound set is complete rather than merely prose-derived.

**No relational vocabulary is declared, and that refusal is the design.** The live
corpus phrases relations at least a dozen ways — `Relation to`, `converges with`,
`converges on`, `subsumes`, `closes`, `Sibling to`, `Couples to`, `Companion to`,
`carved out of`, `split from`, `prerequisite`, `hard dependency on` — and, checked
against the tree rather than transcribed, resolves to 58 body-backtick-to-live-slug
edges today, not a round number; "the corpus as it stands" is a moving count by
construction and this figure is a snapshot, not a maintained one. Enumerating
them in a kit would be brittle against prose that keeps inventing phrasings, and —
the decisive objection — **a kit literal spelling one project's relational verbs
would ship that project's vocabulary as everyone's**. That is the provenance seam,
and it is the same reason `[roadmap:]`'s horizons and tracks are consumer-configured
arrays rather than kit literals. Declaring no vocabulary means there is no
vocabulary to leak and no consumer config to invent.

**A new declaration line was weighed and refused.** The obvious alternative is a
`relates: <kind> <slug>` line on the `roadmap-summary:` pattern. It is precise and
it carries a kind. It is also wrong here on three counts: the 58 edges already
written would each need hand-authoring, which is the maintained-roster anti-pattern
the derivation-first rule forbids; those lines would land inside entries measured
against `check-queue-entry-budget`'s raw-line cap, and sub-tasks do not relieve a
parent's budget, so precision would be paid for in evictions; and a hand-declared
edge can be forgotten in exactly the moment it matters, whereas a citation written
in prose is written because the author was already thinking about the relation.
**The derived grammar costs zero lines in every entry and works on the corpus as it
stands today.**

### 2. `bin/queue-edges.sh` — the aggregator {design-bearing}

A new tool beside `bin/queue-index.sh`, shaped like it: reads the queue, writes
**stdout only**, mutates nothing.

```
# usage: queue-edges.sh [--inbound <slug>] [queue-file]
#   default: every live slug with at least one inbound edge, its citing entries
#   --inbound <slug>: the inbound set for one slug
```

It is a **separate tool, not a fourth mode of `queue-index.sh`**, on that
section's own contract: `queue-index.sh` is a compact *task-selection* surface
walking bullet lead lines, and this walks entry bodies for a different purpose.
Folding them would give one tool two jobs and two output grammars.

**Inbound only.** An outbound view was considered and refused for want of a
reader: an entry's outbound edges are its own body, which the session asking the
question is already reading. Inbound is the direction that is invisible without a
scan of the whole file — which is the entire finding.

It reuses `lib/queue.sh`'s section-boundary regexes and `queue_live_slugs`; the
body-citation scan is new, since every existing primitive extracts a slug only from
a bullet's own lead line.

### 3. The output carries the citing line, and therefore the kind {design-bearing}

Per inbound edge the tool emits the citing entry's slug **and the citing line
verbatim**. A caller reading `native-gate-binary-port`'s inbound set sees, among
others, the line that says the port *subsumes* the citing entry.

This is what buys a kind without declaring one. The relation's nature is already
written, in the citing entry's own words, by the author who understood it; quoting
it is strictly better than a one-token classification chosen from a fixed set, and
it is free. It also keeps the tool honest about precision: a citation that is a
passing mention rather than a relation is *visibly* a passing mention once its line
is on screen, so the reader discards it in the time it takes to read one line.
Recall is what this unit needs — the missed dividend was three entries nobody
summed — and a low-precision, high-recall list whose noise self-identifies is the
right trade.

### 4. The reader: the scope stage's premise re-verification {design-bearing}

The consumer is a **scope session**, at the step where it re-verifies a queued
entry's premises before building on them
(`lifecycle-kit/templates/stages/scope.md`, the dated-hypothesis paragraph). That
paragraph today covers only the entry's *own* claims — which is precisely the
confirmation channel the parent entry diagnosed. It gains the aggregation half:
before ranking a candidate unit, run the tool for that slug and read what the rest
of the queue says about it.

Naming it there rather than in a consumer's ritual binding is deliberate: the
omission it fixes is a property of how surveys go wrong, not of this repo's
configuration. The template cites queue-kit's section for the mechanism and never
restates it — the same cross-kit citation shape that paragraph's neighbours already
use for canon-kit and `check-stage-entry`.

### 5. Two refusals, recorded so they are not helpfully undone {design-bearing}

**No tracked projection, and no freshness gate.** The entry's own phrasing asked
for "generated and freshness-gated"; this narrows it, with cause. A tracked
artifact needs a reader who cannot run the tool, and there is none — the one
consumer is a session with a shell. `ROADMAP.md` is tracked because it is a
*public* page for readers outside the repo; this has no such audience. Against
that, a committed copy of the highest-churn file's derived edges would restale on
essentially every queue edit, buying a per-commit regeneration tax and a new row on
the generated-projections roster for zero benefit. The sibling the entry itself
names — `bin/queue-index.sh` — writes nothing and is gated by nothing, for the same
reason. **Derivation-first is satisfied by deriving on demand; the "generate and
freshness-gate" clause of that rule governs a copy that is *needed*, and this one
is not.**

**Not a gate.** There is nothing to red: an entry with no inbound edges is normal,
and an entry with many is not a defect. The adjacent temptation — redding on a
backticked token that resolves to no live slug — is explicitly refused, because the
corpus shows those are overwhelmingly legitimate citations of landed work, and
`check-queue-slug-liveness` already owns the liveness invariant for the surfaces
where a dead reference *is* a false claim (external prose, `dir=one`, the queue as
ground truth). Extending it inward would red on good prose.

## Producers and consumers

**The citation (existing prose, newly governed).**
Producer: an author writing an entry body — already producing them, 58 times in
the live corpus (a snapshot count, not a maintained one — see §The tag algebra),
with no configuration enabling or disabling it. This is the
delta's whole economy: the producer needs no change and no migration, so the
grammar is satisfied by the tree on the day it lands.
Consumer: `bin/queue-edges.sh`'s body scan — the file's only reader of a
body-position slug token. No other component parses entry bodies for slugs today,
which bounds this change to one new scanner in one new tool.

**The inbound-edge record (new, transient).**
Producer: `queue-edges.sh`, once per invocation, per resolved citation. Every field
has a named reader at a named transition, which is why it has exactly three: the
**citing slug** and the **citing line** are read by the scope session at the
premise-re-verification transition (delta 4); the **target slug** is read by the
same session as the grouping key it asked for. A kind field was considered and
removed for want of a reader — the citing line already carries the kind (delta 3).
The record is printed and never stored, so it opens no channel, needs no freshness
gate, and cannot go stale.

**The `[blocked-by:]` inclusion (existing state, new consumer).**
Producer: the tag, on an entry's lead line, as today. Existing consumers —
`queue-index.sh`'s blocked/ready split and `check-task-names`' resolution — are
untouched; this adds a second reader of the same declaration rather than a second
declaration.

**Whole-component-set reader survey.** The surfaces that parse `TASK-QUEUE.md` are
`queue-kit/lib/queue.sh` and the tools over it (`queue-index.sh`, `roadmap.sh`,
`lesson-sink.sh`), `queue-kit/checks/` (hygiene, sections, entry-budget, wrap,
tag-lead-line, task-names, task-conservation, prose-precondition, slug-liveness,
roadmap-fresh), `canon-kit/checks/check-amendment-queue`, lifecycle-kit's
stage-entry and stage-evidence gates, and `ROADMAP.md` as the one generated
projection. **This amendment adds no syntax and removes none**, so every one of
those parsers is unaffected by construction — the grammar declares a meaning for
text they already skip. Build re-runs this survey at its own HEAD before
implementing, with stderr unsilenced on every path probe.

## Existing sections updated

- **queue-kit/SPEC.md §The tag algebra** — the citation grammar (delta 1) lands
  beside the bold-code cross-reference grammar already there, stating the contrast
  explicitly: the bold-code form is a *membership claim* on an external prose
  surface, audited by `check-queue-slug-liveness`; the single-backtick in-body form
  is a *citation*, aggregated by `bin/queue-edges.sh` and audited by nothing. Both
  the unresolved-token rule and the no-vocabulary refusal are stated here, since a
  later reader's first instinct will be to add both.
- **queue-kit/SPEC.md §bin/queue-edges.sh** — new per-component contract section:
  interface and the inbound-only ruling (delta 2), the output shape (delta 3), and
  the two refusals (delta 5) with their causes.
- **queue-kit/SPEC.md §bin/queue-index.sh** — one line drawing the boundary to the
  new sibling, so the "why two tools" question is answered where a reader looking
  at either will meet it (delta 2). It cites the new section rather than restating
  its contract.
- **queue-kit/SPEC.md §lib/queue.sh** — the body-citation scanner joins the
  primitive roster if it lands in the shared library rather than in the tool; the
  section states which, so a third reader knows where to find it (delta 2).
- **queue-kit/README.md** — the `bin/` command list gains its `queue-edges.sh`
  lines in the same shape as the `queue-index.sh` and `roadmap.sh` entries above
  them (delta 2).
- **lifecycle-kit/templates/stages/scope.md** — the dated-hypothesis paragraph
  gains the aggregation half and cites queue-kit for the mechanism (delta 4). This
  is the delta that makes the amendment cross-component.
- **lifecycle-kit/SPEC.md §templates/stages/** — if that section enumerates what
  the scope template obliges, the aggregation step joins it (delta 4); otherwise it
  is untouched and the template edit stands alone.
- **`scope-survey-counter-evidence`** — the queue entry keeps its un-gateable half
  and states that the derivable half shipped, so the remainder does not read as the
  whole. Its `.workflow/audit-roster.txt` class is untouched by this unit.
- **docs/site-architecture.md §Generated projections** — **no row is added**, and
  that is the point of delta 5: this unit ships a tool, not a projection. Recorded
  here so a later session does not read the omission as an oversight.

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
