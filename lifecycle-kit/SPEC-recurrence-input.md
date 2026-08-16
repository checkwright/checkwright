# SPEC amendment: recurrence-drain-input-widening

lifecycle-kit/SPEC.md §The committed gap inbox states the question this closes,
in its own words: "a session that observes a recurrence outside the capture
channel can stamp one directly, and whether that path should be sanctioned,
forbidden, or mechanized is **open**."

**The ruling is sanctioned — and the sharper half is that it is also
*obliged*.** The obligation attaches to the **judgment**, not to the channel the
observation arrived through. Permitting a direct stamp without obliging it would
answer the question in form and leave it open in substance, because the failure
the counter exists to end is a recurrence that is *seen and not recorded*, and a
merely-permitted write is one a session under pressure correctly declines.

## What changes

### 1. Any session that judges a recurrence stamps it; the drain stays the only mechanized producer

The section's existing rules are **unchanged and are what this ruling rests on**:
a date "records a session's judgment that a finding re-occurred", a slug in a
bullet "is an input to that judgment and never a verdict", and "no mechanism
produces the declaration". What the amendment adds is one sentence's worth of
reach — the judgment obliges the stamp wherever the judgment is made, and the
drain is described as the only mechanized *producer* rather than the only
sanctioned one. **Design-bearing.**

Stamping stays **idempotent per (slug, date)** and stays a manual queue edit; no
code changes, in this kit or any other.

**Why not forbidden.** Three independent refutations, none of them a preference:

- It would strand a whole class by construction. `close-generated-finding-route`
  records that close's drain runs **once**, early, while close's own later steps
  — audits, lesson disposition, staleness reads, release disposition —
  necessarily generate findings that postdate it. Forbidding the direct stamp
  gives those findings no legal channel at all, in the one stage that is
  downstream of every drain.
- It is already being obeyed, and the cost is recorded.
  `survey-edge-aggregation-residue` declines to stamp *precisely because this
  question is open*, and its own entry states what that costs. A rule whose
  disciplined observance produces a known-wrong count is the wrong rule.
- It would red a third of its own precedent. lifecycle-kit/SPEC.md already
  refuses a provenance gate on exactly this ground — "a gate that reds a third
  of the precedent it was derived from is measuring the rule, not the tree" —
  and that refusal was made *to keep this route open*. Forbidding the route by
  prose what the tree refused to forbid by gate would be the same error with a
  weaker instrument.

**Why not mechanized.** This one is foreclosed by the owning section rather than
weighed: "**no mechanism produces the declaration**" is a recorded ruling, and
mechanizing the direct-stamp path reverses it. The independent second ground is
that the only mechanism available is the capture-time matcher, and
`recurrence-resolver-literal-match-only` establishes it is unreliable **in the
under-counting direction** — a bullet describing a real recurrence without
spelling the slug resolves as new. Mechanizing on it would inherit that blind
spot and dress a known-lossy predicate as a count.

### 2. The auditability price, which is what the sanction actually costs

lifecycle-kit/SPEC.md rules that the same-commit property is "**load-bearing
rather than incidental**: it is the audit artifact" — the drain stamps in the
same commit that truncates the inbox, "so the judgment and the prose it was made
from sit in one diff and a reader auditing a date reads that commit and sees the
bullet the judge read". A direct stamp truncates no inbox, so it has no such
diff **by construction**, and this is the one real thing the sanction gives up.
**Design-bearing.**

The repair is to notice which half of that sentence is the invariant. The
invariant is **the judgment and its grounds land in one commit**; the inbox
truncation is one *way* of satisfying it, not the thing itself. So the direct
stamp carries the same obligation, discharged the only way it can be when no
bullet is being deleted: **the observation is written into the entry, beside the
declaration, in the stamping commit.** A reader auditing a direct date reads that
commit and finds the grounds there, exactly as with a drained one.

That evidence write is **the same mandated write** as the date it accompanies —
its grounds half, not a discretionary addition — so it claims the relief
queue-kit/SPEC.md §check-queue-entry-budget already defines and this amendment
does not restate. That is not an incidental note: the entry most likely to be at
the cap is the one that has recurred most, and this amendment's own entry is at
the cap and attests the collision twice.

**The honest limit, stated because it is real.** The rule does not make the count
re-derivable — lifecycle-kit/SPEC.md already concedes that trade for the drain's
own stamps, so the sanction adds no *new* class of unverifiability. What it does
add is **judges**: the population that may stamp widens from one stage to every
session, and some of those sessions judge their own tool use, which is a session
grading itself. No gate can hold this and none is proposed; the counterweight is
the grounds obligation above, which makes a thin judgment visibly thin.

### 3. Two entries are relieved in part, and neither is ruled here

`close-generated-finding-route` keeps its subject. What this amendment settles is
only its **recurrence half** — a close-generated recurrence now has a channel,
namely the stamp itself. A close-generated finding that is *not* a recurrence
still has no drainer inside its own iteration, which is that entry's actual
question and stays open. **Design-bearing.**

`survey-edge-aggregation-residue` may stamp under this ruling and its declining
clause is spent, but its aggregation defect is untouched and it stays deferred on
that. **Mechanical**, being a re-read of one clause against a landed rule.

### 4. The measurement the entry carries is re-taken

`recurrence-drain-input-widening` carries "three of the nine commits that ever
added a `recurrence:` date stamped one outside the drain", accurate when measured
on 2026-08-08 and stale now. Re-measured at this rev: **10 of 27 stamping commits
(13 of 43 stamp events)** were off-channel. **Mechanical.**

**The ratio is not the finding, and this matters for what gets merged.** It moved
from roughly a third to roughly 37% — so lifecycle-kit/SPEC.md's refused-gate
paragraph, which says "roughly one in three", **is re-verified and needs no
edit**. What the proportion hides is the finding: the original back-test observed
only close-session-generated stamps, whereas the live history additionally shows
`chore(build)`, `chore(align)` and `chore(scope)` sessions stamping directly —
**three producing stages the original ruling never contemplated as producers at
all.** That is qualitative, carries no number into governed prose, and is the
strongest single argument that the obligation belongs on the judgment rather
than on close.

**The oracle is part of the finding.** `git log -S'recurrence:'` **undercounts
and is the wrong tool**: the pickaxe fires only when the literal's occurrence
count changes, so an append that rewrites one line is invisible to it. The
correct oracle is `git log -G'^  recurrence:' -- TASK-QUEUE.md`, cross-checked
per commit against whether `.workflow/gap-inbox.md` changed in the same commit.
Both readings and the census are filed in `.workflow/survey-record.md` behind
their witness, so a later stage re-runs rather than re-derives.

## Producers and consumers

**No new state, event, interface or field is introduced.** The `recurrence:`
declaration's grammar (queue-kit/SPEC.md §The tag algebra), its idempotence, and
its two consumers are all unchanged; this amendment widens *who may write an
existing field* and attaches an obligation to that write.

- *Producer* — unchanged in mechanism, widened in population: the closing
  stage's drain remains the only **mechanized** producer, reachable at every
  close with no enabling config because the drain is mandatory and the boundary
  refusal forces it. Added: any session that judges a recurrence, in the commit
  it is already making. Enabling config: **none**, and none is possible — the
  act is a queue edit, so it is live in every vendored consumer the moment the
  rule lands.
- *Consumers* — both existing, both unchanged, and both **strictly better
  served**, which is the point of the widening. Scope's pre-emption rule reads
  the date count against `LIFECYCLE_KIT_RECURRENCE_THRESHOLD` and escalates a
  deferred entry that reaches it; `kpi-incident-recurrence` sums the dates as a
  **lag** metric whose own contract already says "a recurrence nobody files is
  uncounted, and so is one no session judged". Both read a count that was
  undercounting by every judgment made outside close.
- *Every field has a named reader.* The declaration's fields are unchanged and
  keep their readers. The grounds prose §2 obliges is not a new field: it is
  ordinary entry body, read by the same reader that reads the rest of the entry
  — a session or reader auditing the date, at the transition where it does so —
  and it is the entry's existing prose tier rather than a declaration.

**Red conditions.** This amendment narrows no corpus and adds no scan, so
point 5's non-monotonicity hazard does not arise. Two readers are nonetheless
worth naming because the widening changes how often they fire, not whether:

- `check-queue-entry-budget` assertion A — reds on a deferred entry exceeding
  the cap, monotone in the count. **More sessions may now trigger it**, on the
  entries least able to pay, which is exactly the case queue-kit's mandated-write
  relief already covers; §2 routes to it rather than re-ruling it. Its
  at-most-one `recurrence:` discount already applies to a directly-stamped line,
  since the discount matches the declaration's grammar and asks nothing about
  who wrote it — verified against the gate's own predicate, not assumed.
- `check-gap-inbox-neutrality` — reds on an inbox bullet that opens with a
  verdict shape. **Untouched, and it is corroborating evidence rather than a
  risk**: the kit already forbids the *capture* channel from carrying a verdict
  while the *declaration* carries the judgment. That asymmetry is this ruling in
  miniature — the bullet was never the authority, so removing the bullet from
  the path removes an input, not a warrant.

## Existing sections updated

- **lifecycle-kit/SPEC.md §The committed gap inbox** — owned by deltas 1 and 2,
  and the only section carrying substantive change. The sentence "whether that
  path should be sanctioned, forbidden, or mechanized is open" is replaced by
  the ruling and its two refusals; the drain keeps its description as the only
  *mechanized* producer and as the stage that must not skip the judgment. The
  "What the judgment rule costs, and what pays for it" paragraph gains the
  generalization in §2 — the same-commit property is the invariant, the inbox
  truncation one way of meeting it — and states the grounds obligation for a
  stamp that truncates nothing. The **refused provenance gate** paragraph is
  **unchanged**: its "roughly one in three" figure is re-verified at this rev,
  and its stated reason for the refusal ("it would foreclose the direct-stamp
  path above by gating exactly the route that open question may want opened")
  becomes retrospectively correct rather than speculative, which is worth one
  clause and no more.
- **lifecycle-kit/templates/stages/close.md, step 2** — owned by delta 1. Its
  recurrence sub-step already says "**you are the judge, and nothing upstream
  ruled**"; that framing is now the general rule rather than close's local one,
  so the step cites the widened contract instead of reading as the sole
  sanctioned occasion. Close's own obligation is unchanged and undiminished.
- **queue-kit/SPEC.md §check-queue-entry-budget, the mandated-write class** —
  owned by delta 2, and the reason this amendment is cross-component. The
  class's first instance reads "the gap-inbox drain's `recurrence:` date, which
  must land in the commit that truncates the inbox because that commit is the
  audit artifact", and its producer list names "the closing stage's drain". Both
  widen to the judging session, with the audit-artifact ground restated as
  same-commit rather than same-commit-as-a-truncation. The class's own test —
  "does a named contract oblige this write in this commit" — is **why delta 1
  had to oblige rather than permit**: a merely-permitted stamp would fall out of
  the class and lose the relief on the entries that need it most.
- **queue-kit/SPEC.md §The tag algebra** — owned by delta 1, for its sentence
  naming "the closing stage's gap-inbox drain" as the declaration's only
  mechanized producer. The word *mechanized* is already doing the right work
  there, so the edit is to make its companion explicit rather than to correct it.
- **TASK-QUEUE.md `recurrence-drain-input-widening`** — owned by delta 4, for
  the stale "three of the nine commits" figure. The re-measurement supersedes it
  and the census lives in the survey record.
- **docs mirror of every file above** — regenerated, not edited.

No wire contract, no knob, no gate and no script is touched by this amendment.

## The seam ruling

**Kit mechanism:** the whole of it. The rule is a generic authoring contract
about who may write a declaration the kit already specifies, and it holds in any
consumer that vendors lifecycle-kit and queue-kit. **Consumer config:** none, and
none is added — `LIFECYCLE_KIT_RECURRENCE_THRESHOLD` and
`LIFECYCLE_KIT_GAP_INBOX_FILE` are untouched, and the ruling deliberately mints
no knob: a consumer that wanted the forbidden reading would be configuring away
an auditability contract, which is the kind of thing a SPEC rules rather than a
knob offers. **Private rule content:** none is involved. The census in delta 4 is
over this repo's own public history, and the finding that reaches governed prose
is qualitative — three stage names, all of them public kit vocabulary — so no
count and no private vocabulary crosses into a kit literal.

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
