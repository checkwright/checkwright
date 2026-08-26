# SPEC amendment: ruling-authority

Closes `relayed-ruling-provenance-unrecorded`. Under the split-posture lead
architecture an operator ruling reaches a stage session as a peer message and is
landed in a tracked governance surface as "operator-directed", and nothing in the
tracked record distinguishes a genuinely relayed ruling from one a compromised or
confused lead invented. The reviewing layer cannot see the lead's transcript, and
the artifact outlives every session that could attest it. The class most often
relayed this way is the highest-consequence one — credentials, pushes, releases,
runbook deviations.

**The entry's fourth shape constrains the other three and is taken as the
design's centre: the record must name WHO RULED, with "lead, own authority" a
first-class value rather than an absence.** All three earlier shapes record only
*how* an authorization arrived. The 2026-08-24 firing went the other way — a
lead's own ruling recorded as the operator's — and that direction is the more
expensive one, because "operator-ruled" marks a decision a later session may not
reverse alone. The filed shape is a false **ceiling** on verifiability;
over-attribution upward is a false **floor** on re-ruling, and a session with no
first-class way to say *I ruled this myself* is pushed toward the false floor.

**The first shape was practised on 2026-08-19 and its two limits are what this
amendment answers.** A lead relaying five rulings stated the authorization
channel unprompted and the recording session carried it onto each entry. It
**did not fit twice**: `close-entry-baseline-bootstrap-deadlock` and
`stage-stamp-ordering-unenforced` took their rulings at 0–1 lines of headroom and
carry no channel at all. Delta 2 is the direct answer to that displacement. The
other limit — the citation is still the relaying party's own word — is not
closable in tree, and delta 4 says so rather than letting the record imply
otherwise.

## What changes

### (1) The `ruled:` declaration, on the `recurrence:` pattern

queue-kit's queue format gains a second body-line declaration:

```
ruled: <slug> <authority> <YYYY-MM-DD> <channel>
```

One indented body line naming the entry's own slug, then the authority that
ruled, the date, and a **keyword** naming how the authorization reached the
recording session. Repeat per ruling; never rewritten. **{design-bearing}**

**Every field earns its place against the fourth shape's constraint.**

- `<slug>` — **self-naming for the same mechanical reason `recurrence:` is.**
  `check-queue-hygiene` rejects any exact-duplicate non-blank line across the
  whole file, unnormalized, and two entries taking rulings from the same
  authority on the same day through the same channel is not a corner case — it is
  the ordinary shape of a lead's relay batch. The slug makes the line unique by
  construction, and it keeps the declaration resolvable by one anchored grep with
  no entry-boundary parsing.
- `<authority>` — **required, with no default, which is the whole of the fourth
  shape.** A lead's own ruling is written `lead … own-authority`, not left
  unmarked. An absence today reads as *nobody claims this*, and the attested
  correction at gate-sdk/SPEC.md §check-gate-fail-closed — which names the lead
  and records that the commit which landed the ruling (`0153a5c9`, per `git log`;
  the section itself names no SHA) misattributes it and is deliberately not
  rewritten — is what an absence costs to repair.
- `<YYYY-MM-DD>` — the date the ruling was taken, matching `recurrence:`'s date
  form so one date grammar serves the file.
- `<channel>` — **a keyword, not a sentence.** `check-queue-wrap`'s budget is the
  line's ceiling, and free prose reaches it immediately; the full account of how a
  ruling arrived belongs in the entry body, exactly as `[gate-exempt:]`'s reason
  keyword does (queue-kit/SPEC.md §The tag algebra).

**A declaration and not a tag, on canon-kit's own further-tag test.** Its readers
scan a line of its own, and it marks no move across a pending/ready boundary, so
a tag would add a state name without adding a caught error class.
`check-tag-lead-line` does not govern it and it cannot collide with the bracket
scans — the same two properties `recurrence:` is admitted on.

**The value set is the consumer's and no kit enumerates it.** `operator` and
`lead` are *this* repo's governance roles; a kit literal spelling them would ship
this project's posture as everyone's, which is the provenance seam. The kit ships
the **slot** — position, requiredness, one line, self-naming — and the consumer's
own always-loaded surface names its authorities. **No knob is minted**, because
no kit mechanism reads the values: a knob whose only reader is
`check-knob-citation` is a knob that should not exist.

### (2) The entry-budget discount widens from one declaration to one of each

`check-queue-entry-budget` (A)'s **count** currently discounts an entry's extent
by *at most one* line matching the `recurrence:` grammar. It widens to *at most
one line of **each** declaration grammar the format defines* — today
`recurrence:` and `ruled:`, and any later one by construction rather than by a
further edit. **{design-bearing}**

**This is the assertion's own third side doing its job, not a relaxation of the
cap.** §check-queue-entry-budget states the invariant from three sides, and the
third is "bounded in **what it may displace**, so a bound on filing never becomes
a bound on the record." A provenance declaration is record, not filing. The two
attested displacements are exactly the cap spending a record to stay inside
itself, which is the failure that clause names.

**The per-recurrence-variant ground survives unchanged, and it survives for the
same reason.** §The tag algebra records that a one-line-per-event form is refused
because it grows an entry linearly against the cap, and that "the discount is one
line" is what keeps that true. It stays one line **per grammar**: a second
`ruled:` line on one entry is counted like any other, so an entry taking many
rulings still pays for them and the cap still binds. Multiple rulings therefore
append to one line's tail on the `recurrence:` model where they share an
authority and channel, and take a counted second line where they do not.

### (3) The obligation lands at the relay and at the landing, on both surfaces

Two surfaces gain one clause each, because the record needs both parties to act
and neither can discharge the other's half. **{design-bearing}**

- `lifecycle-kit/templates/lead.md` — a lead relaying a ruling **states the
  authority and the channel in the relay**, and states its own authority when the
  ruling is its own. This is the party that holds the fact; a recording session
  that was not told cannot invent it, and inventing it is the failure mode.
- `lifecycle-kit/SPEC.md` §The state machine, the stage-session contract — a
  session landing a ruling in the queue **writes the `ruled:` declaration in the
  same commit as the ruling's content**. Not afterwards: the relay message is
  transport, never a store, and a ruling landed without its declaration has
  already lost the only party who could attest it.

**Shape 2 — operator-class rulings landed by the operator directly — is refused
here and named as refusable only by an operator.** It is the one shape that would
make an invented ruling *impossible* rather than merely costly, because it
removes the relay. It is refused at this stage because it is a change to
**operator behaviour**, which no repo mechanism can require and no session may
rule; it is named rather than dropped so a later operator can take it knowingly
rather than rediscover it.

### (4) Presence is deliberately ungated, and the residual bound is declared

No gate asserts that an entry claiming a ruling carries a `ruled:` declaration.
The grammar's one machine reader is delta 2's discount, so a malformed line
simply is not discounted — a soft, self-correcting consequence rather than a red.
**{design-bearing}**

**Measured at this stage, and the measurement is the argument.** 33 live entries
already carry ruling vocabulary across 52 occurrences. A presence gate would
demand a declaration on every one of them, and for many the provenance is
**unrecoverable** — the founding instance's own recording session had no operator
message in its transcript at all. Such a gate would be satisfied by *invention*,
which is the precise failure this entry exists to prevent. A gate that can only
be made green by fabricating the fact it audits is worse than no gate, and
grandfathering by date would put a permanent unexplained boundary in the file.
This follows `recurrence:`, which has no presence gate either, for a
compatible reason: a declaration written under judgment is not one a scanner can
demand.

**The residual bound, stated because a claim without one is what this unit is
repairing.** A `ruled:` line is still the relaying party's own word about a
channel the tracked record cannot reach. It **raises the cost** of inventing a
ruling and does **not** make an invented one detectable. What it does close is
the direction the practised shape missed: an unattributed ruling stops being
indistinguishable from an operator ruling, and a lead's own ruling has a
first-class way to be recorded as its own. No governed surface may state this as
verification.

## Producers and consumers

**The `ruled:` declaration (delta 1)** — the one new interface.

- *Producer:* the **stage session landing the ruling**, in the same commit as the
  ruling's content, under delta 3's clause in lifecycle-kit/SPEC.md §The state
  machine. Its upstream is the **lead's relay**, under delta 3's clause in
  `templates/lead.md`. **Enabling config actually set:** none — the declaration
  is a body line in a file every session already edits, needs no knob, and is
  live the moment the grammar lands. Nothing about it is test-only, and nothing
  about it waits on a consumer setting anything.
- *Consumers,* two, at two transitions and both real:
  - **`check-queue-entry-budget`'s (A) count**, at every commit touching the
    queue, which reads the line to discount it (delta 2). This is the machine
    reader, and it is what keeps the grammar from being decoration.
  - **The auditing reader** — a security review, a later session, or an operator
    reading the tracked record — at the moment a recorded ruling is relied on or
    re-ruled. This is the reader the whole unit exists for, and it is the reader
    the founding instance had: the harness's own security review flagged the
    2026-08-18 ruling on exactly this surface, correctly, on what it could see.
- *Named reader for every field:* `<slug>` is read by `check-queue-hygiene`'s
  duplicate scan (as uniqueness, at commit) and by the anchored grep that makes
  the line resolvable; `<authority>` and `<date>` are read by the auditing reader
  at the transition above; `<channel>` likewise. No field is populated for a
  reader that does not exist — which is why the line carries a keyword channel
  and not a prose account, the prose having its reader in the entry body already.

**Deltas 2, 3 and 4 introduce no new state.** Delta 2 changes an existing gate's
count over an existing extent; deltas 3 and 4 are prose obligations on surfaces
that already exist, read by a lead loading `templates/lead.md`, a stage session
loading lifecycle-kit/SPEC.md, and a reader of the queue format.

**Narrowing check (canon-kit/SPEC.md §The causal-completeness check, point 5).**
Delta 2 **narrows** the set of lines `check-queue-entry-budget` counts, which is
a narrowing of a corpus in the point's exact sense. Each reader is enumerated by
its **red condition**:

- `check-queue-entry-budget` (A) — reds when an entry's **counted** lines exceed
  `QUEUE_KIT_ENTRY_LINE_CAP`. **Monotone in the counted set** and therefore
  clearable by inspection in this direction: discounting more lines can only
  lower a count, so no entry newly exceeds the cap. Named anyway because the
  gate's own **fixture pair** asserts an exact boundary case, which is not
  monotone — a `bad/` case sitting one line over the cap goes **green** if that
  line is a declaration, and the pair must be re-read for exactly that.
- `check-queue-entry-budget` (C) — reds when a top-level deferred entry carries
  no `Cost while deferred` lead-in. **Holds a minimum, so not monotone.** Cleared
  by inspection: no delta removes a cost field, and a `ruled:` line is not a cost
  field and cannot be mistaken for one by a line-local scan.
- `check-queue-hygiene` — reds on an exact-duplicate non-blank line anywhere in
  the file. **Not monotone**: it reds on *finding* a duplicate, and delta 1
  **adds** lines, so it is the gate a same-day relay batch would trip. Cleared by
  construction rather than by inspection — the self-naming slug field is in the
  grammar for this reason and for no other.
- `check-queue-wrap` — reds on a line exceeding the wrap budget. **Not monotone**
  under an addition. This is the gate the keyword-channel rule exists to satisfy,
  and it is the declaration's own ceiling, exactly as it is `recurrence:`'s.
- `queue-lib-parity.test.sh` — reds when the shell and compiled classifications
  of one corpus differ. **Not monotone** (byte comparison). It must be re-read:
  the discount lives in the compiled `queue_entry_budget.rs` while the section
  regexes are shared, so a grammar added on one side alone is exactly the failure
  this comparison exists to catch.
- `check-queue-slug-liveness` — reds on a bold-code token on a prose surface
  naming no live task. **Untouched and cleared by inspection**: `ruled:`'s slug
  field is a bare token on a declaration line inside the queue, not a bold-code
  membership claim on a prose surface.

## Existing sections updated

- `queue-kit/SPEC.md` §The tag algebra — the `ruled:` grammar, its four fields
  and the ground for each, placed immediately after the `recurrence:` paragraph
  whose pattern it takes; the self-naming-slug paragraph is widened to state the
  rule for **declarations** rather than for `recurrence:` alone (delta 1).
- `queue-kit/SPEC.md` §The tag algebra, the refused-variant paragraph — the
  "discount is one line" ground is restated as *one line per grammar*, so the
  per-event variant stays refused for the reason it was refused (delta 2).
- `queue-kit/SPEC.md` §check-queue-entry-budget, assertion (A) — the discount's
  definition widens; the sentence that extent and count "differ by that one
  discounted line and by nothing else" is rewritten, since it is now one per
  grammar and is the sentence a reader will check the arithmetic against
  (delta 2).
- `lifecycle-kit/SPEC.md` §The state machine — the stage-session contract's
  clause: a ruling and its declaration land in one commit, and the relay message
  is transport rather than a store (delta 3).
- `lifecycle-kit/templates/lead.md` — the relaying clause: state the authority
  and the channel, and state your own authority when the ruling is your own
  (delta 3).
- `lifecycle-kit/SPEC.md` §Layout and configuration — no knob is added, stated
  where a reader will look for one, with the reason (delta 1).
- `native/src/gates/queue_entry_budget.rs` — the discount's implementation, which
  is the grammar's one machine reader (delta 2).
- `queue-kit/gate-tests/check-queue-entry-budget/{good,bad}` — the pair's exact
  boundary case, which delta 2 moves (delta 2).
- `CLAUDE.md` — this consumer's authority vocabulary, since the kit enumerates
  none; the always-loaded tier is where a session already learns this repo's
  governance roles (delta 1).

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
- [ ] **No retrofit** — the 33 entries already carrying ruling vocabulary are
      **not** back-filled. Delta 4's whole ground is that a declaration written
      from a transcript nobody has is an invention, and a build that back-fills
      them has built the defect.
- [ ] **The discount is proved by a red** — the widened count is observed
      admitting an entry that the old count rejected, at build, rather than
      reasoned from the arithmetic here.
