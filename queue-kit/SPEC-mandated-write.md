# SPEC amendment: mandated-write

The per-entry line cap silently chooses what a governed mechanism may record,
and it bites hardest where the record matters most. This amendment names the
class of write the cap has no self-served answer for, gives that class two
reliefs — one mechanized, one an authoring contract — and corrects the prose
elsewhere in this SPEC that the mechanized half falsifies.

**The two candidate reliefs that ship are not the two the filing entry ranked
first, and the reason is evidence rather than taste.** The entry offered three
candidates and judged the cheapest — exempting the `recurrence:` line — weakest,
on the ground that it *"exempts the one line that grows without bound as dates
accumulate"*. That ground is **falsified by this SPEC's own §The tag algebra**,
which already rules the declaration width-bounded by `check-queue-wrap` and
calls reaching that bound *"the **correct** complaint"*. The line cannot grow
without bound. So the cheap candidate ships, scoped so the refusal it was
holding up stays intact (delta 3). And a scope-session note recorded in
`.workflow/survey-record.md` establishes the other half: at the 2026-08-16
firing, neither blocked write was a `recurrence:` stamp — both were argued prose
recording a ruling — so the exemption alone answers only one of the two shapes.
The second shape is delta 4's.

## What changes

### 1. §check-queue-entry-budget names the class it has no answer for

A **mandated write** is a write on a deferred entry that another governed
contract obliges the writing session to make **in the commit it is already
making**, and whose obligation is citable to that contract by name —
**design-bearing**.

Two instances exist today, and both are already in the tree rather than invented
here:

- the gap-inbox drain's `recurrence:` date, which must land in the commit that
  truncates the inbox because that commit is the audit artifact
  (lifecycle-kit/SPEC.md §The committed gap inbox);
- a ruling recorded onto the entry it rules, under this section's own
  recording-in-the-moment paragraph — *"That restatement is the moment the
  content is in hand and the cheapest it will ever be, so it is filed then"*.

**What is not a mandated write, because a class that excludes nothing is not a
class.** The session's own evidence for a claim; a further ground for a claim
already made; a cross-reference it would like to add; a correction it could
equally make in a later commit. All of those keep the reliefs they have. The
test is not *how important is this* — every session believes its write matters —
but *does a named contract oblige this write in this commit*.

### 2. The invariant gains its third side

The section's invariant reads *"bounded above so it is not an inlined amendment,
bounded below so it is not a flag-and-skip"*. It gains a third clause: **and
bounded in what it may displace, so a bound on filing never becomes a bound on
the record** — **design-bearing**.

This is the sentence the whole amendment hangs from. The cap's two existing
sides are about the entry's *size*; the defect is about what the cap *spends* to
stay inside it, and under pressure it spends argued content to seat generated
content, which is backwards.

### 3. Assertion A stops counting one `recurrence:` declaration line per entry

Assertion A's extent is unchanged except in one respect: **at most one line per
entry matching the `recurrence:` declaration grammar is discounted from the
count** — **design-bearing**.

Three grounds, and the third is the one that keeps the scope right:

- The line is **fixed-shape and width-bounded**. §The tag algebra rules its
  ceiling to be `check-queue-wrap`'s budget and rules reaching it the correct
  complaint, so the discount cannot let an entry grow without bound. This retires
  the objection the filing entry raised against this candidate.
- It is exactly the **generated-shaped content** that argued content was being
  spent to seat. Making the cap blind to it removes the trade rather than
  arbitrating it.
- **One line, not a grammar-wide exemption.** The declaration's own rule is one
  line per entry with dates appended, so a discount of one line is the whole of
  what that form can claim. This is deliberate: §The tag algebra refuses the
  one-line-per-recurrence variant partly *because* it grows an entry linearly
  against this cap, and a grammar-wide exemption would retire that refusal's
  ground as a side effect. Scoped to one line, the refusal keeps it — the second
  and later lines of such a variant still count.

**The fixture pair widens with the rule, and the widening is the part that can
go wrong.** The `good/` side gains a case standing one line over the cap whose
sole excess is a `recurrence:` declaration, which must now pass. The `bad/` side
gains a case standing one line over the cap with **no** declaration, and a case
with a declaration and one further excess line, both of which must still red — a
discount that let either of those green would have moved the cap rather than the
count.

### 4. Relocation becomes self-served for a mandated write; the split does not

The section names three reliefs and gates the third: *"a parent session (the
iteration lead), or the operator in the absence of one, grants permission to
split and issues the recipe with the ruling."* The gate stays, and what changes
is that the third relief is revealed to be **two different acts** wearing one
name — **design-bearing**.

- **Relocating grounds** into an entry that **already exists and already owns
  the ground's subject** becomes **self-served, when and only when what the cap
  blocks is a mandated write** as defined in delta 1. The relocating session
  cites the contract that mandated the write in the commit that makes the
  relocation.
- **Splitting the unit** — minting a *new* entry to hold what would not fit —
  stays authorization-gated, unchanged.

**Why the line falls there, and it is the filing entry's own worst case that
draws it.** That entry records a third displacement shape as *"the worst of the
three"*: an entry that could not hold its own design half, so a session filed
that half as a separate entry — *"A commit message is at least out of the
ranking surface honestly; a new entry is **in** it, competing with its own
parent for the scope attention that ranks both."* A blanket self-served relief
would make that shape **routine**. The distinction answers it exactly: grounds
are not rankable work and moving them into an entry that already owns their
subject mints no ranking peer, while a new entry is a new unit competing for
selection — which is a scope-class judgment, so it keeps the authorization the
gate already withholds it behind.

**Why the authorization is safe to drop for the narrow act.** The authorization
exists to stop a session inventing room for its own discretionary content — the
self-issued exemption the delegation doctrine names as the standard failure
mode. A mandated write is by construction not discretionary: its obligation
belongs to another surface and is citable, so the trigger is checkable by a
reader even though no gate reads it. And the act is **net-negative on the parent
by construction** — a relocation that does not reduce the parent's extent is not
a relocation, which assertion A already enforces from the other side.

**The honest limit, stated because it is real.** Where no existing entry owns
the ground's subject, the self-served relief is **unavailable** and the session
is back to compressing by answering, or to asking. That is the correct outcome
rather than a hole: needing to mint an entry *is* the signal that a unit, not a
ground, is what would not fit.

### 5. The relocation link rides the citation grammar this SPEC already has

The relocating entry names its target with a **single-backticked slug in its
body** — already a citation under §The tag algebra, already aggregated by
`bin/queue-edges.sh`, already audited by nothing on purpose —
**design-bearing**.

**No `relocated:` declaration is added, and the refusal is inherited rather than
re-argued.** §The tag algebra records a `relates: <kind> <slug>` declaration
weighed and refused on three grounds, and the second bites hardest here: such
lines *"would land inside entries measured against `check-queue-entry-budget`'s
raw-line cap … so precision would be paid for in evictions"*. A declaration
whose whole purpose is to relieve the cap, costing a line against the cap, on
the entry least able to pay it, is the defect wearing the fix's clothes. The
third ground inverts and is worth recording as the one place this case differs:
a hand-declared edge *"can be forgotten in exactly the moment it matters"*,
where a relocation's citation is written in the moment the author is performing
the relocation — but that is an argument for the citation grammar being
sufficient, not for a declaration being better.

### 6. Its own filing entry is what this is measured against, and the entry says so

The filing entry carries an **Owed** line — a truncation sub-case in which two
entries each stand at exactly 50 so neither can name the other. Delta 3's
discount is what pays for that cross-reference, and the sub-case is discharged by
naming it rather than left standing as owed prose after the mechanism that
answers it lands — **mechanical**.

### 7. The seam verdict, stated because the class could easily have crossed it

Everything here ships as **kit mechanism** and nothing becomes consumer config
— **design-bearing**, because the natural way to write delta 1 would have
crossed the seam.

The mandated-write class is generic: it is defined by *a named contract obliges
this write in this commit*, and it names no contract of this project's beyond
the two kit-owned instances it cites — lifecycle-kit's gap-inbox drain and this
section's own recording rule. A consumer's own mandated writes satisfy the
definition without queue-kit ever hearing about them, which is the test.

**A roster of mandated writes was the obvious shape and is refused.** A kit
literal enumerating which contracts may mandate a write would ship one
project's governance vocabulary as everyone's mechanism, and it would be a
maintained roster besides — the two failures §The tag algebra already refuses a
relational verb set on. The definition selects; the roster would have listed.

**No knob is added**, and that is a verdict rather than an omission: the
discount is one line by the declaration's own form, and the relief's trigger is
a property of the write rather than a consumer preference. Neither has a value
to configure.

## Producers and consumers

This amendment introduces **one named class, one change to an existing
assertion's extent, and one narrowing of an existing authorization**. It adds no
tag, no declaration, no field, no knob, no file and no script — stated so a
later reader does not go looking for a mechanism that was deliberately not
built.

### The mandated-write class

- **Producer, named and reachable:** the sessions the two instance contracts
  already bind — the closing stage's gap-inbox drain, whose `recurrence:` write
  lands in the commit that truncates the inbox, and any session recording a
  ruling onto the entry it rules. Both run today on every iteration; neither
  needs configuration turned on, and the class names no new trigger.
- **Consumer, named, with the mechanism:** the same session, reading
  §check-queue-entry-budget when assertion A blocks it — the gate's failure text
  already cites that section rather than inlining the recipe, so the class
  arrives through a path that already carries the reader. Second consumer:
  lifecycle-kit's drain contract, which cites the class by name for the write it
  mandates.
- **Every new field has a named reader** — there is no new field. The class is a
  term in an authoring contract, and the section's own established voice for
  that limit is *"The gate cannot hold this … So the rule is a stated authoring
  contract."*

### Assertion A's changed extent

- **Producer:** `check-queue-entry-budget`, in its compiled form
  (`native/src/gates/queue_entry_budget.rs` — this member ported with the
  queue-kit cohort, so the change lands in Rust and rides
  `bash gate-sdk/bin/build-native.sh` as well as the battery).
- **Consumers, named across the whole component set, because the count is read
  in more than one place:** the gate itself at commit time;
  `bin/queue-index.sh --extent`, whose yield §check-queue-entry-budget defines
  assertion A's extent to equal — **so the tool and the gate must move together
  or the section's own equality sentence goes false**, and that equality is the
  reason this reader is named rather than assumed; and the eviction path, which
  deletes the range that extent yields.
- **Red conditions, because this delta narrows what the gate counts.** A
  narrowing of a *count* is exactly causal-completeness point 5's warning shape:
  `check-queue-entry-budget` assertion A reds on an entry **exceeding** the cap,
  which is monotone in the count and therefore clearable by inspection — a
  smaller count reds strictly less. Assertions B and C are untouched: B counts
  icebox continuation lines, C asserts a `Cost while deferred` lead-in is
  **present**, and C's red condition is a *zero* — the non-monotone shape — but
  the discount removes no line from the scan, only from the count, so C cannot
  flip. `check-queue-wrap` is unaffected: it measures columns, not lines, and
  §The tag algebra's ceiling argument for the declaration is exactly its
  business.

### The narrowed authorization

- **Producer:** the session performing a relocation, at the moment assertion A
  blocks a mandated write.
- **Consumer:** the reader of the resulting commit and of the two entries — and
  `bin/queue-edges.sh`, which picks the relocation's backticked slug up as an
  edge with no change to its own rule, which is delta 5's whole point.
- **Every new field has a named reader** — again none; the link is an existing
  citation with an existing aggregator.

## Existing sections updated

- **queue-kit/SPEC.md §check-queue-entry-budget** — deltas 1, 2, 3, 4, 6. The
  invariant sentence gains its third clause; assertion A's extent sentence gains
  the discount; the *Compression is lossless, and the split is
  authorization-gated* paragraph gains the relocate/split distinction, and its
  sentence *"A session blocked by the cap does not self-serve the split"* stays
  exactly true while stopping being the whole rule. The *Why the cap is not
  widened for exceptional content* paragraph is **untouched and still binding**:
  nothing here raises the number or makes the cap conditional, and saying so
  where that paragraph sits is what stops a later reader mistaking the discount
  for the widening it refuses.
- **queue-kit/SPEC.md §The tag algebra** — two corrections owed by delta 3, and
  they are corrections rather than additions because leaving them would put two
  readings of one fact on one page:
  - the `recurrence:` paragraph's sentence *"The single-line form costs one entry
    line forever regardless of count"* becomes false at merge — the single-line
    form now costs **no** counted line, which strengthens rather than weakens the
    case for it;
  - the same paragraph's refusal of the one-line-per-recurrence variant keeps its
    cap ground **because** the discount is one line, and the paragraph says so,
    or a later reader re-deriving the refusal will find its ground apparently
    retired;
  - the sentence in the same paragraph reading *"the cap's remedy is
    authorization-gated (§check-queue-entry-budget: a session blocked by it does
    not self-serve the split)"* gains the relocate/split distinction, since the
    parenthetical is a pointer to the rule delta 4 changes.
- **lifecycle-kit/SPEC.md §The committed gap inbox** — the drain's `recurrence:`
  write is one of the class's two instances, and that section is where a draining
  session reads its obligation. It cites the class by name and cites this
  section for the relief; it does not restate either.
- **queue-kit/gate-tests/check-queue-entry-budget/{good,bad}/** — delta 3's
  fixture widening, one good case and two bad ones.
- **No knob table change**, and that is a verdict: the discount is a property of
  the rule with one right answer, not a consumer-varying value, so
  `QUEUE_KIT_ENTRY_LINE_CAP` is the section's only knob and stays so.

## Definition of Done

- [ ] **Causal completeness** — the class's producer and consumers are named in
      §check-queue-entry-budget itself; assertion A's second reader
      (`bin/queue-index.sh --extent`) moves with the gate or the section's
      extent-equality sentence goes false; the count narrowing's monotonicity is
      discharged per assertion rather than asserted wholesale.
- [ ] **Merged with no information lost** — deltas land inside
      §check-queue-entry-budget and §The tag algebra in their own voice, not
      appended; the two §The tag algebra sentences are **corrected where they
      stand**, never left beside a newer sentence that contradicts them.
- [ ] **Amendment deleted** — this file removed on merge;
      `ls queue-kit/SPEC-*.md` checked.
- [ ] **Removals propagated** — grepped for the retired reading of the
      recurrence line's cap cost and for any surface restating the
      authorization-gated remedy without the relocate/split distinction; nothing
      dangles.
- [ ] **Gaps filed** — cross-component gaps filed through
      `bash lifecycle-kit/bin/file-gap.sh`; a build-time causal gap resolved that
      session rather than deferred.
