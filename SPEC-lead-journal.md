# SPEC amendment: lead-journal

**Nothing in this amendment is applied.** Every passage below is a proposal for the build stage to
land; where replacement wording is given it is marked **Not yet applied** at the passage itself.
`spec` authors, build lands, and a reader arriving here mid-iteration must not read a quoted
sentence as one already in the tree.

The entry this amendment discharges says its remaining subject is one question — *where the lead's
durable state lives*. **Both of that question's stated premises are false at HEAD**, and correcting
them is what turns an open-ended siting problem into three bounded deltas. The corrections are
probed, not reasoned, and each is stated with the surface that falsifies it.

## The two premises, falsified

**Premise 1 — "the template now names `.tmp/`, which the scope boundary sweeps."** False.
`scripts/lifecycle-config.sh:11` reads `LIFECYCLE_KIT_BOUNDARY_PRESERVE=(session-role
lead-journal.md)`, and the `# spec:` comment above it names the journal *"the third attested victim
of this wipe"* and records the fix as taken with its cost accepted. `lifecycle-kit/templates/lead.md`
§Closing an iteration already states the new truth: the session-role marker and the lead's own
resume journal *"are the scratch artifacts that survive the boundary reset."* The journal is not
swept. It has a home and the home works.

**Premise 2 — "a tracked home collides with the invariant that the lead writes no governed
state."** Also false as stated. That invariant (§Stamps are authoritative) enumerates *stamps, queue
writes and evidence files* — and the same template, a few paragraphs later, **directs** the lead to
write the tracked, committed `.workflow/gap-inbox.md` through `--emit file-gap` in order to discharge
a recurrence judgment it cannot stamp. So the lead already writes a tracked file with the
template's own sanction, and tracking is not what rules a tracked lead journal out. Something else
does, and delta 4 says what.

## What is actually left, which is not a siting question

Three things, none of them "where":

1. **The template contradicts itself about the journal's fate, in one file.** §Economics' journal
   bullet still ends *"It is a scratch artifact, swept with the rest at the iteration boundary"* —
   the direct negation of §Closing an iteration's sentence quoted above, some seven hundred words
   away. A reader who arrives at the obligation learns the journal dies; a reader who arrives at the
   boundary learns it survives. One of them acts on the wrong fact.
2. **Durability is consumer config, so the kit ships the obligation without the mechanism.**
   `LIFECYCLE_KIT_BOUNDARY_PRESERVE` **defaults empty**. Every adopter who takes `lead.md` — a kit
   template — inherits its "write everything a compact would lose to your journal" obligation and
   inherits **none** of the protection, because the protection lives in one consumer's config file.
   The kit is telling adopters to store state in a location its own boundary reset deletes.
3. **Preserving is not draining, and the config comment concedes it:** *"one stale journal carries
   across a boundary crossed with no lead live, until the next lead overwrites it."* A
   preserved-but-unread journal loses a finding exactly as a deleted one does. This is the entry's
   **second-order instance** — the lead recorded the defect itself in the disposable journal, and
   *"a lead cannot bootstrap a lesson out of a file swept before that lesson's next occasion."*
   Preservation alone does not answer it: an unswept file nobody reads is the same loss with a
   longer fuse.

## What changes

### (1) The template stops contradicting itself about the journal's fate

§Economics' journal bullet closes on a sentence the tree falsified {mechanical}.

**Not yet applied.** The replacement text is fixed by facts already stated elsewhere in the same
file and in delta 2, so this delta demands no design call — it is a pin against a stated truth:

> It is a scratch artifact with a **protected lifetime**: the iteration-boundary reset spares it by
> kit invariant (lifecycle-kit/SPEC.md §bin/enter-stage.sh), for the same reason §Closing an
> iteration relies on — a session live at the boundary must be able to file a judgment there for
> the entering session's intake. Protected is not permanent: delta 3's disposition step is what
> keeps the file from becoming an accumulator nobody reads.

The cross-reference in §Closing an iteration is left standing and gains no edit; it was already
right, and the repair belongs where the falsehood is.

### (2) Durability becomes kit mechanism, and the shape is the one this SPEC already argued for

The lead journal's survival moves from a consumer array member to a **kit invariant beside
`.gitkeep`**, addressed by a new scalar knob {design-bearing}.

`LIFECYCLE_KIT_LEAD_JOURNAL_FILE` — a **basename** within `GATE_SDK_TMP_DIR`; default
`lead-journal.md`. The boundary wipe spares it the way it spares `.gitkeep`: as a tier the consumer
keep-list layers on top of rather than replaces.

**Why a scalar knob and not a default array member, which is the whole design content of this
delta.** §bin/enter-stage.sh already rules out shipping the `.gitkeep` exemption as
`LIFECYCLE_KIT_BOUNDARY_PRESERVE`'s default, on the ground that *a defaulted bash array is replaced,
not merged, when a consumer assigns it, so protection would decrease as configuration increases*.
That argument applies to the lead journal unchanged and with an attested instance behind it — this
tree's array had to name `lead-journal.md` by hand, after the loss, and any adopter who assigns the
array for their own reasons silently drops it again. A **scalar** default has no such failure mode:
an assignment overrides one value and cannot drop a second one it never mentioned. So the same
reasoning that made `.gitkeep` an invariant makes this one, and the knob exists only so a consumer
who renames the file can say so.

**Why the kit may name this basename at all, held against the provenance seam.** The name is not
consumer vocabulary: the kit's own `lead.md` mints the artifact and is the only surface that writes
it. A default here is the kit naming its own output, which is the `LIFECYCLE_KIT_GAP_INBOX_FILE`
posture — as against `LIFECYCLE_KIT_RULING_RECORD`, which defaults **empty** precisely because a
ruling record is a *consumer's* artifact the kit must not presume. Stated so a later reader does not
"correct" this default to empty by false analogy with that one.

**Consumer-side consequence, in the same unit:** `scripts/lifecycle-config.sh:11` drops
`lead-journal.md` and keeps `session-role`, whose lifetime is context-kit's and whose protection is
genuinely this consumer's to declare. The `# spec:` comment above it loses the accepted-cost
sentence, which delta 2 pays off, and keeps the lifetime contrast it draws.

### (3) The disposition step, and the boundary learns to say the journal is undisposed

Preserving answers destruction; it does not answer the loss the entry's second-order instance
records. Two halves, and they are deliberately asymmetric {design-bearing}.

**The obligation half, in `lead.md` §Closing an iteration.** Before the lead reports at the
iteration's end, it **disposes of its journal**: every finding that must outlive the iteration goes
to the committed channel that owns it — a gap bullet, a survey block, a knowledge-friction line —
and a ruling goes to its governed surface; then the lead appends a terminal disposition mark as the
file's last line. This is the `DONE` marker's shape borrowed from the stage journal, with a
disposition semantic rather than a completion one.

**The detection half, in `--enter-stage`'s boundary reset.** When the lead journal exists and
carries no disposition mark, the boundary entry **prints its `## ` headings as an advisory and
proceeds**. This is the survey record's read-trigger, reused rather than reinvented: that arm
already prints a non-empty record's headings — *the questions the iteration's prior surveys
answered, never their findings* — and this is the same print over a different file.

**A refusal is declined, and the ground is the discriminator that separates this file from the gap
inbox.** The gap inbox refuses at a close-skipped boundary because its bullets are *drainable by the
entering session* — they are queue-shaped, and the entering scope may write the queue. A lead
journal's contents are another session's working prose, and the entering scope cannot discharge a
disposition it does not hold the context to judge. A refusal it could clear only by deleting the
file would train deletion of the very artifact the rule protects. So the boundary makes the state
**loud** and never blocking — which is exactly the failure mode on record, where three findings
survived only because the operator happened to ask.

**The honest limit.** The advisory can be read and ignored, and nothing forces the lead's own
disposition step. What it buys is that the undisposed state stops being silent, and delta 4 is what
keeps the disposition small enough to actually happen.

### (4) The routing rule: the journal is transport for a durable finding, never its store

The entry's *"what has no home"* list is partitioned by **lifetime**, and the partition is the
answer to the siting question the entry poses {design-bearing}.

- **Iteration-local working state** — the batch roster and its tiering rationale, findings carried
  between batches, budget verdicts, what the next dispatch would re-derive. The journal is its home
  and it dies with the iteration, correctly, because it is *about* the iteration. No change.
- **Anything that must outlive the iteration** — a gap, a survey, a re-derived fact, a ruling. It is
  filed to its committed channel **in the moment it is found**, never only journalled. The journal
  may carry a copy for the lead's own working use; it is never the only home.

Stated as the clause it parallels: §Stamps are authoritative already rules that *the message thread
is transport, never a store*, and that a ruling whose acting session is not imminent is filed to a
durable governed surface in the moment it is made. **This delta says the same of the lead's own
journal** — which is the sentence the second-order instance needed and no attestation reached,
because the journal reads like a store and the message thread does not.

That is also the real answer to premise 2: a tracked lead journal is refused not because tracking
collides with an invariant, but because it would be a **fourth capture surface with no drainer**,
duplicating three that already exist and already have one. The lead does not need a new committed
file; it needs to use the three it is already permitted to write.

**Dependency, named rather than absorbed.** This delta's routing rule presumes the lead **commits**
what it files. That clause is `gap-inbox-commit-ownership`'s deliverable, already promoted as debt
in this iteration and landing in the same template — its shape ruled 2026-09-06 by the operator
through an interactive prompt in the lead session, relayed by that lead. This amendment does not
re-author it and asserts nothing about it. It is a **producer/consumer edge**: that clause is the
producer, this delta the consumer, and a batch cut separating them dispatches this one against an
input that does not exist yet.

## Producers and consumers

The new interfaces are **one knob** and **one terminal mark**. No new file, no new tracked surface,
no new event, no new tag.

- **`LIFECYCLE_KIT_LEAD_JOURNAL_FILE` (delta 2).**
  - *Producer:* the consumer's lifecycle config, or nothing at all — the kit's own default is the
    live value in every consumer that does not set it, so the producer path is **reachable with no
    configuration**, which is the enabling-config limb this check exists for.
  - *Consumer:* `--enter-stage`'s boundary wipe, which reads it to compute the spared set alongside
    `.gitkeep` and `LIFECYCLE_KIT_BOUNDARY_PRESERVE`; and the boundary advisory of delta 3, which
    reads the same value to locate the file it inspects. Two readers, one value, no third.
  - *Field reader:* the knob is a scalar basename with no fields. A **path** knob was considered and
    refused: the journal's parent is `GATE_SDK_TMP_DIR` by the same pairing
    `LIFECYCLE_KIT_BOUNDARY_PRESERVE` already takes ("paired with the scratch dir it reads rather
    than adding a directory knob of its own"), so a second directory knob would have that argument
    as its only justification and no reader of its own.
- **The disposition mark (delta 3).**
  - *Producer:* the lead session, at the iteration's close, appending it as the file's last line
    (`lead.md` §Closing an iteration). Reachable with no tooling — it is a line of text in a scratch
    file, on the `DONE` marker's precedent, and deliberately needs no `--emit` arm.
  - *Consumer:* `--enter-stage`'s iteration-boundary entry, at the boundary transition, which reads
    for the mark's **presence** and prints the file's `## ` headings when it is absent.
  - *Field reader:* the mark carries no fields. A dated or authored mark was refused for the same
    reason as above — no reader would branch on either, and the stage journal's `DONE` sets the
    precedent that a presence marker stays a presence marker.
- **Existing integration prose describing the prior flow.** Three surfaces describe the journal's
  lifetime today and two of them disagree; all are in the update roster below, and the disagreement
  is delta 1's whole subject.
- **Readers surveyed across the whole component set.** A tree-wide sweep for the lead journal by
  every spelling it has (`lead-journal`, "lead's own resume journal") returned exactly six hits:
  two in `lifecycle-kit/templates/lead.md`, two in `scripts/lifecycle-config.sh`, and two historical
  passages in `TASK-QUEUE.md` that cite past events. `delegation-kit/SPEC.md` §Resume journal names
  no lead at all — it speaks of "a mutating agent" throughout — which is why this amendment adds
  nothing there and why the lead-specific facts stay in lifecycle-kit. No stderr was suppressed on
  any path in that sweep.
- **The tracked-surface question, settled here so build does not reopen it.** Nothing in this
  amendment adds a `.workflow/` member, so `.gitattributes` takes **no** edit and `check-merge-attrs`
  sees no new derived-set member. Delta 4 is the reason, and recording it here is what stops a build
  session from "completing" the design by adding the file.

## Existing sections updated

- `lifecycle-kit/templates/lead.md` §Economics — the journal bullet's closing sentence, which is
  false at HEAD (delta 1), and the routing rule the bullet's list of what to journal now needs
  (delta 4).
- `lifecycle-kit/templates/lead.md` §Closing an iteration — the disposition step joins the lead's
  closing acts; the section's existing sentence about the two surviving scratch artifacts is
  correct and stands (delta 3).
- `lifecycle-kit/SPEC.md` §bin/enter-stage.sh — the boundary wipe's spared set gains the lead
  journal as a kit invariant beside `.gitkeep`, carrying the replaced-not-merged argument that
  already sits there (delta 2); and the boundary entry gains the undisposed-journal advisory beside
  the survey record's read trigger, with the declined refusal and its discriminator (delta 3).
- `lifecycle-kit/SPEC.md` §Layout and configuration — the knob roster gains
  `LIFECYCLE_KIT_LEAD_JOURNAL_FILE` with its default and the tier-split note
  `LIFECYCLE_KIT_BOUNDARY_PRESERVE`'s own entry already models (delta 2).
- `lifecycle-kit/SPEC.md` §templates/lead.md — the section `scripts/lifecycle-config.sh:10` cites by
  name for the journal's lifetime; it must state the protected lifetime rather than the accepted
  cost (deltas 2 and 3).
- `scripts/lifecycle-config.sh` — the preserve array drops `lead-journal.md` and its `# spec:`
  comment drops the accepted-cost sentence, keeping the lifetime contrast (delta 2).
- `TASK-QUEUE.md`, the entry itself — its two falsified premises corrected and the body compressed
  onto this amendment (deltas 1 and 2). **Not yet applied** as prose; the promotion commit this
  stage lands carries it, and build owns nothing further here.
- `TASK-QUEUE.md`, `gap-inbox-commit-ownership` — <!-- update-target-exempt: promoted debt with no
  amendment owed; this amendment consumes its ruled clause and must not re-author or re-scope it -->
  named as delta 4's producer edge and deliberately untouched.

## Retired spellings

- None — no delta of this amendment retires a spelling. Delta 2 moves `lead-journal.md` from a
  consumer array member to a kit default, so the **string is unchanged** and every surface naming it
  keeps naming it; deltas 1, 3 and 4 replace prose whose subject keeps its name.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and
      a named consumer; every new field has a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); the merged spec reads as one coherent document a reader who never saw
      the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls SPEC-*.md lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings`
      above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked
      tree.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
- [ ] **The contradiction cannot recur silently** — after delta 1, no two passages in `lead.md`
      state opposite fates for the journal; the merge check is a read of both sections in one pass,
      not a grep.
