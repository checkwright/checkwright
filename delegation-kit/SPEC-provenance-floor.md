# SPEC amendment: provenance-floor

Closes `delegation-provenance-floor`. A dispatching session can narrate findings
from a subagent whose output it never received, and nothing reds.

**The fork the entry names is ruled: the deliverable is the doctrine line, and
the receiving-side alternative is cited as already-landed prior art rather than
re-taken.** The entry records that the receiving-side half landed as
§Resume journal — the dispatcher mints a path and reads it, observable where a
message is not — leaving only the doctrine line open. That is what delta 1 is.

**One half of the entry's concession does not survive contact with the tree,
and that is the substantive finding of this amendment.** The entry says "a gate
over tree state is very likely unbuildable" and admits "no gate is buildable" as
an answer. That holds for the class the entry was **originally** filed against —
a relayed return — and it does **not** hold for the class its own fourth
instance added. A minted identifier is gateable, the mechanism is already in this
tree one field away from where the attested fabrication landed, and
enforcement-first therefore obliges delta 2 rather than permitting the blanket
concession.

## What changes

### (1) The provenance floor lands as a doctrine bullet where agent-execution binds

`templates/agent-execution.md` gains one bullet stating the floor: **a return you
did not receive is not a finding, and an identifier you did not read is not a
citation.** **{design-bearing}**

**Why a doctrine line and not a gate for this half** — argued at delta 3, so the
next session does not re-open it.

**Why the template and not the always-loaded manifest.** Load-trigger residency:
the rule binds a session that has dispatched, and a dispatching session loads
this template through its trigger, so a resident copy would be a second source
paid by every session that never dispatches (delegation-kit/SPEC.md §Operative
residency owns the placement rule).

The bullet states two shapes, because the entry's four attested instances are
not all one shape and a rule written for the first misses the fourth:

- **The relayed return.** A claim whose warrant is a dispatched agent's output
  is made only where that output was **received**. A monitor that timed out, a
  child that never reported, a notification never consumed — each leaves the
  parent with no return, and a claim made anyway is fabrication regardless of
  how well the parent reasoned. The three attested instances of this shape all
  sat in the dispatcher's own reasoning rather than in quoted child text, which
  is why a floor checking relayed *quotations* would have missed every one.
- **The minted identifier.** A commit sha, a `file:line`, a count, a path
  written to make a record look precise but never read. The fourth attested
  instance is this: a fabricated short hash written into the survey record's
  corpus field, self-caught in-session before a reader reached it.

**What the bullet requires, and it is deliberately cheap.** Not a new artifact
and not a running narration — the discharge is already available. Where the
return was received, the parent **cites where it is held**: the journal path it
granted and read, or the commit it verified. Where it was not, the parent either
does not make the claim or marks it **unheld** in the same sentence. Marking is
admissible on purpose: the failure this closes is a claim that *looks* sourced,
so a claim that says it is not sourced does not commit it.

**The relation to the three adjacent bullets is stated so the set does not read
as four spellings of one rule.** *A child's citation is a pointer to verify*
governs a citation that **arrived** and may be wrong; *Findings you will act on
are durable before you act on them* governs a finding that arrived and must
survive the session; *Verify after every agent commit* governs work that landed.
This one governs the case where **nothing arrived at all** — which none of the
three reaches, because each presupposes a return.

**Attested first-person at this spec stage, and the instance is recorded because
it is one where the floor worked.** An isolated agent dispatched from this
session returned a message claiming an audit roster "from my earlier turns". That
roster was not in the return. It was not narrated, not used, and every update
target in this iteration's amendments was derived first-hand instead. The same
return also carried a claim the parent could not verify — that the child had
built the native binary to escape a refusal loop — and it is carried at that
tier wherever it appears, never promoted to a finding.

### (2) `check-survey-record` probes every git-object-shaped token, not only the `rev` field

The gate's existence probe widens from the `rev:` field to any git-object-shaped
token in **any** field of a survey block. **{design-bearing}**

**The mechanism already exists and is already trusted; only its corpus is too
narrow.** `check-survey-record` requires `rev:` to be a full 40-hex sha and
probes it with `git cat-file -e <rev>^{commit}`, on the stated ground that "a sha
the tree does not carry makes `git diff <rev>..HEAD` fail rather than witness
anything". Every other field — `corpus`, `oracle`, `finding` — is free prose and
is probed for nothing. **The attested fabrication landed in `corpus`.** So the
tree already holds the exact predicate that would have caught the fourth
instance, pointed at the one field the fourth instance did not use.

The widened arm: for each field value of a survey block, every word-bounded
lowercase-hex token of length 7-40 must resolve via `git cat-file -e`. A token
that resolves to any object type passes — a sha naming a blob or a tree is a real
citation, and demanding `^{commit}` outside the `rev:` field would red a
legitimate one. The `rev:` field keeps its stricter arm unchanged: full 40-hex
**and** commit-resolvable.

**Valve, and the reason it is mandatory.** A survey block may legitimately carry
a hex-shaped token that names no object — an illustrative sha in an `oracle:`
command, a fixture literal. The valve is a `<!-- survey-token-exempt: <reason> -->`
comment on the block, reason mandatory per the `comment-tier-exempt:` convention,
so an exemption is an audit trail rather than a silent skip.

**The false-positive risk is small here and would not be in the wider corpus,
which is why the corpus is not wider.** A survey block's fields are short and
structured, and a 7-plus-hex word-bounded token in one is a citation far more
often than it is an accident. The same arm over the whole queue or over every
governed prose surface would meet ordinary hex-looking English and fixture data,
and a gate that cries wolf trains its readers to bypass it (gate-sdk/SPEC.md
§When a gate earns its place). The wider sweep is **filed, costed, and not
built here**.

**This does not close delta 1's first shape and does not claim to.** It closes
the minted-identifier shape inside one record. A relayed return leaves no token
to resolve.

### (3) The unbuildability of the relayed-return half is stated with its argument

§Resume journal gains the recorded reason **no** gate reaches a relayed return,
so the next session weighing an oracle for it reads the argument instead of
re-deriving it. **{design-bearing}**

The argument, and it is a proof rather than an appetite: the tree cannot observe
what a session did or did not **receive**. A return lives in the parent's context
and leaves no artifact, so tree state is identical whether the return was held or
invented — the entry's own words, "the prose reads identically". Arrival is also
unobservable to the parent *by construction* in at least one attested mode: a
child that could not reach its dispatcher by name delivered its synthesis to the
top-level session instead, so even the parent's self-inquiry cannot discharge it.
A gate would therefore have to assert a fact about a conversation, and no scanner
over a repository reaches one.

**What is left is a mitigation rather than an oracle, and it already ships.** The
resume journal makes the child's output land on **disk** rather than only in a
message, so the parent's claim becomes checkable against an artifact whenever the
child wrote one. That is a reduction of the class's reach and not a closure of
it, because the journal is the child's write and the caveat that it can fail
silently stands. Stated as a bound, not banked as a fix.

## Producers and consumers

**The doctrine bullet (delta 1)** introduces no state, event or interface. It is
prose on a surface that already exists.

- *Producer:* `delegation-kit/templates/agent-execution.md`, hand-authored. Its
  **enabling path is live in this tree**: the template is loaded through the
  `/agent-execution` trigger, which CLAUDE.md §Agent execution points every
  dispatching session at, so the bullet reaches its readers without new wiring.
- *Consumer:* the **dispatching session**, at the transition the entry names —
  the moment it writes a claim whose warrant is a dispatched agent's output. Not
  at dispatch time and not at return time: at authoring time.

**The widened probe (delta 2)** introduces no new state and no new field.

- *Producer:* `native/src/gates/survey_record.rs`, in the block that today
  collects `revs` and probes them, extended to collect tokens from every field's
  value. It reuses the existing `proc::run("git", ["cat-file", "-e", …])` call —
  the same probe, a wider input — so no second git invocation shape is minted.
  **Enabling config actually set:** the gate is registered in `scripts/gates.list`
  and runs in this tree's battery today; the existing `probe_rev` switch that
  already gates the network-free existence probe governs the new arm too, so a
  fixture running without a repo is unaffected.
- *Consumer:* the **committing session**, through the gate's output contract, on
  the generated pre-commit hook, `run-gates.sh` and CI; and
  `run-gate-tests.sh` through the fixture pair.
- *Named reader for every field:* the arm adds no field to the record. The
  tokens it reads are existing field content, and their reader is the gate at
  the commit transition. The **valve comment** is the one new token, and its
  reader is the same gate at the same transition plus the human reading the
  block's audit trail.

**Narrowing check (canon-kit/SPEC.md §The causal-completeness check, point 5).**
Delta 2 **widens** a corpus rather than narrowing one, which is the safe
direction for a violation-counting reader but not automatically safe for a
count-asserting one, so each reader's **red condition** is enumerated:

- `check-survey-record` itself — reds on a malformed block, a bad `rev`, and now
  an unresolvable token. **Not monotone**: the widening *adds* possible
  violations, so the live `.workflow/survey-record.md` must be re-run before the
  arm lands, and this iteration's own carried survey block re-probed rather than
  assumed clean.
- `lifecycle-kit/gate-tests/check-survey-record.test.sh` — reds when an asserted
  exit or finding count differs. **Not monotone** (it asserts exact counts), so
  it moves with the arm: new cases for a resolvable token in a non-`rev` field,
  an unresolvable one, and the valve.
- The gate's `good/`+`bad/` fixture pair — the `bad/` case must gain an
  unresolvable non-`rev` token so the arm has an executable statement, and the
  `good/` case a resolvable one plus the valve. **Not monotone**: the pair
  asserts a specific verdict per case.
- `check-gate-output`, `check-gate-substrate-parity`, `check-crate-arms` —
  **monotone and cleared by inspection**: the gate keeps its name, its
  descriptor, its module and its output contract; only its rule body grows.
- `check-comment-tier` — reds on a non-directive comment. **Not monotone**: the
  new valve token is a comment convention, and its `<!-- survey-token-exempt: -->`
  spelling must be declared where the sibling exempt tags are declared or the
  gate reads it as prose.
- `check-knob-citation` / `check-knob-default-coupling` — **monotone and cleared
  by inspection**: delta 2 adds no knob. The corpus is the same record file the
  gate already reads through its existing knob.

## Existing sections updated

- `delegation-kit/templates/agent-execution.md` — the new bullet, placed with
  the three adjacent return-handling bullets and stating the boundary between
  them (delta 1).
- `delegation-kit/SPEC.md` §Resume journal — agent writes, scratch reset sweeps —
  the unbuildability argument and the journal's status as a mitigation rather
  than a closure; this section already carries the receiving-side half the entry
  cites as landed, so it is the one surface where both halves can be read
  together (deltas 1 and 3).
- `delegation-kit/SPEC.md` §Operative residency — the residency ruling for the
  new bullet, recording why it is template-tier and not always-loaded (delta 1).
- `lifecycle-kit/SPEC.md` §The survey record — the record's field contract
  states that every field's git-object-shaped tokens are probed, not the `rev`
  field alone, and carries the valve (delta 2).
- `lifecycle-kit/SPEC.md` §check-survey-record — the gate's own section: the
  widened arm, its `^{commit}`-versus-any-object asymmetry with the `rev` arm,
  the valve, and the stated reason the corpus stops at the record (delta 2).
<!-- update-target-exempt: a conformance re-read against an unchanged bar — no delta edits that section, and it is the surface this unit must satisfy rather than change -->
- `gate-sdk/SPEC.md` §When a gate earns its place — re-read at merge to confirm
  the widened arm still satisfies the false-positive bar this section sets,
  which is the ground the corpus stops at the record on.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`), discharged at the iteration
      rather than at this commit, sibling amendments being in flight for it.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks; the wider prose-surface token sweep filed as a costed deferred
      entry rather than flagged and skipped.
- [ ] **The live record re-probed** — `.workflow/survey-record.md` run against
      the widened arm before it lands, since the arm can red an existing block.
- [ ] **The bullet is not a fourth spelling** — the boundary against the three
      adjacent bullets is written into the bullet itself, not left to a reader.
