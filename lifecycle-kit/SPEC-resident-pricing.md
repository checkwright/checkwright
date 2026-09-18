# SPEC amendment: resident-pricing

context-kit/SPEC.md §The consumer footprint gives each kit's resident ask by
citing the section that owns it. The cited sections are uneven. drift-kit's row
points at a section that states the line's cost and what earns it back: "costs
one always-loaded bullet in the consumer's instructions file; that line is the
loop's hook and must earn its recurring cost by the log actually filling".
lifecycle-kit's row cites three obligations: gap capture, the recurrence stamp
and survey capture. Their owning sections state neither the cost nor what earns
it back. A consumer costing adoption follows the citation and learns the ask
without its price. This amendment writes one cost-and-earn-back sentence into
each owning section, and has the roster row say they are there.

**What an earn-back sentence claims, and what it does not.** Each sentence
states the line's recurring cost and the observable that shows the line being
read. drift-kit's precedent does the same and no more. A sentence names no
threshold and no withdrawal trigger. The recurrence-stamp line landed under an
operator direction, and when that line would be withdrawn is a question neither
that direction nor its amendment reached. Pricing a line informs whoever
performs the brevity pass. Conditioning the line's survival would settle that
question by the back door, so these sentences do not.

**Why each earn-back is the line's marginal reader.** A resident line costs every
session. It earns that cost only through the sessions that would not otherwise
meet its obligation. For each of the three, that is a session no trigger loads
the owning section for, which is the roster's own admission test. Each
observable is therefore chosen as the act only such a session performs.

**Probed at authoring, 2026-09-18.** `grep -n -i "earn\|always-loaded\|recurring
cost" lifecycle-kit/SPEC.md` returns no cost or earn-back sentence in §The
committed gap inbox (lines 1011-1568) or §The survey record (from line 1570).
The only resident-tier passage, at lines 1476-1484, argues where the recurrence
obligation's reach is carried. It does not price it. The tree does not already
do what this amendment asks.

## What changes

### (1) §The committed gap inbox prices gap capture {design-bearing}

**Not yet applied.** This replaces the last sentence of the section's opening
paragraph ("…a *work-shaped* finding (a gap, a task, a defect) is backlog, not
knowledge friction, and routes here."):

> A *work-shaped* finding (a gap, a task, a defect) is backlog, not knowledge
> friction, and routes here. The route costs one always-loaded line in the
> consumer's instructions file, because the mid-iteration session that finds a
> gap loads no trigger for this section. It earns that cost by the inbox
> actually filling: the bullets a close drain dispositions are gaps that would
> otherwise have been a queue edit racing a stage session, or nothing at all.

### (2) The resident-tier paragraph prices the recurrence stamp {design-bearing}

**Not yet applied.** In §The committed gap inbox, the paragraph that opens
"**The reach half — stages that may write the queue but never load this rule —
is closed at the consumer's resident tier**" gains its price by a rewrite of its
sentence "This section stays the owner and the templates stay silent — with the
one exception that ruling's corollary preserves, a reader whose *discharge
differs*, which is the lead immediately below.":

> This section stays the owner and the templates stay silent, except where that
> ruling's corollary lets a reader whose *discharge differs* be told directly:
> the lead, immediately below. The resident line costs one always-loaded bullet.
> It earns that cost by direct stamps, meaning `recurrence:` dates that land in
> commits other than the close drain's. The drain loads this rule through its own
> template, so the line's only marginal reader is a session that is not the
> drain.

### (3) §The survey record prices survey capture {design-bearing}

**Not yet applied.** This replaces the opening paragraph's last sentence, "This
surface is where the expensive half of a survey is carried across the stage
boundary.":

> This surface is where the expensive half of a survey is carried across the
> stage boundary. It is reached through one always-loaded line in the consumer's
> instructions file, because both halves of the obligation bind every stage
> session: reading the record before buying a survey, and filing one a later
> stage will want. It earns that cost by citation. A later stage runs a recorded
> block's witness and cites the finding instead of re-buying the survey.

### (4) The footprint roster row says the prices are there {mechanical}

**Not yet applied.** In context-kit/SPEC.md §The consumer footprint, the
lifecycle-kit roster row ends "…and survey capture (§The survey record)." It
becomes "…and survey capture (§The survey record), each of which states its
line's cost and earn-back condition", matching the drift-kit row's clause.
Afterwards two rows price their asks by citation. The delegation-kit and
doctrine-kit rows are unchanged, because this unit's reach is lifecycle-kit's
three lines.

### (5) The generated mirrors {mechanical}

`docs/lifecycle-kit/SPEC.md` and `docs/context-kit/SPEC.md` are regenerated after
deltas 1-4 land. Each mirror's freshness gate prints its own regeneration
command.

## Producers and consumers

- **The three sentences** (deltas 1-3). This is prose contract only, with no new
  state, event, knob or gate. Producer: the kit author at this merge. Consumers
  are the two readers context-kit/SPEC.md §The consumer footprint already
  names. The first is a consumer evaluating adoption cost, who follows the
  roster citation into the section. The second is this repo's close-stage
  brevity pass, which reads the roster when judging a resident line. Each
  sentence's two parts are the cost, which the first reader reads, and the
  observable, which the second reader checks against the iteration's commits and
  inbox. Neither goes unread.
- **The roster clause** (delta 4). Its reader is the same adoption-cost reader.
  It is told the citation prices the ask, so it does not stop at the row.
- **Every member's satisfying value** (point 6, the obligation that each
  lifecycle-kit resident obligation be priced). The corpus is the lifecycle-kit
  roster row's three named obligations. Gap capture's value is delta 1's
  sentence, the recurrence stamp's is delta 2's, and survey capture's is delta
  3's. The registration block in the same row is generated and gate-held. The
  block-sized-ask paragraph of §The consumer footprint already prices it, so it
  owes no sentence.

## Existing sections updated

Roster probe: `git grep -n "gap capture\|survey capture\|recurrence stamp" -- '*.md'`
over the tracked tree, with `docs/` mirrors and `TASK-QUEUE.md` excluded. It hits
`CLAUDE.md` (this repo's own resident lines, which state the obligations and not
their prices, so they are not a target) and the context-kit roster row.

- `lifecycle-kit/SPEC.md` §The committed gap inbox, the opening paragraph and the
  resident-tier paragraph (deltas 1 and 2).
- `lifecycle-kit/SPEC.md` §The survey record, the opening paragraph (delta 3).
- `context-kit/SPEC.md` §The consumer footprint, the lifecycle-kit roster row
  (delta 4).
- `docs/lifecycle-kit/SPEC.md` and `docs/context-kit/SPEC.md` (delta 5).

## Retired spellings

- None — no delta retires a spelling. Each delta re-phrases a sentence and names
  nothing new.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template, agent definition or shim carries no grounds; a delta places them
      (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it; the merged spec
      reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`) once `SPEC-declined-target.md` has
      merged too.
- [ ] **Removals propagated** — `## Retired spellings` above is accurate, and
      `check-amendment-retired-spelling` is green.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
