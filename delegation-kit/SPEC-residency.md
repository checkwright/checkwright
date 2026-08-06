# SPEC amendment: operative residency

Pairs with the queue entry `dispatched-session-waiting-rule-residency`.

A dispatched stage session ends its turn to await a completion notification,
orphaning the work it started, because the in-turn condition-waiting rule is not
**operative** at the tier the session actually loads.

## The corrected premise, and why it changes the fix

The original filing said the rule "is not resident". That is false, and the
correction is load-bearing rather than cosmetic: a fix aimed at the absence would
fix nothing. `.claude/agents/stage-session.md` has carried a **Your turn end is
your session end** bullet since before either of the first two incidents, and the
third fired through a dispatch prompt in which the lead *also* restated the rule.

What that bullet does is **point**. Read it as an instruction and the gap is
plain — every sentence is descriptive:

> You are a dispatched agent, so backgrounded work does not survive your turn the
> way a supervisor's does, and findings you hold live only in your context. Both
> rules and their reasoning live in delegation-kit/templates/agent-execution.md …

"Backgrounded work does not survive your turn" is a fact. "Findings live only in
your context" is a fact. There is no imperative anywhere in it — not *do not end
a turn on work still running*, not *wait in-turn on the work's own artifact*, not
*write findings down before acting on them*. The reader must derive all three,
and the derivations are easy, which is exactly why their failure is instructive:
under load a descriptive bullet reads as background about one's situation rather
than as a constraint on the next action. Three sessions made that reading.

The defect is therefore **pointer versus operative statement at the always-loaded
tier**, not absence.

*First-hand attestation, this session.* The `spec` session that authored this
amendment is itself a dispatched stage session that held backgrounded work. It
loaded the bullet above, and had to open `agent-execution.md` to obtain an
operative instruction. No violation followed, so this is not a fourth incident —
it is direct evidence that the surface behaves as diagnosed, taken from inside
the failing role rather than inferred about it.

## The doctrine ruling (why this entry stayed design-pending)

The entry left open which doctrine governs: Content-tiering forbids restating a
fact another surface owns, while Load-trigger residency argues the other way.
Ruled here: **read correctly, Load-trigger residency's predicate is
reader-relative, and the two rules do not conflict at all.**

Load-trigger residency says the always-loaded file earns a rule "only when no
stage, skill, or tool-call trigger exists to load it". The decisive question is
*exists for whom*. A trigger that exists but that the bound actor never fires is,
for that actor, no trigger. A dispatched stage session fires exactly one trigger
— its stage skill — and neither the stage skill nor its template loads
`agent-execution.md`; `/agent-execution` is a trigger the **supervising** role
fires. So for the dispatched-session role no trigger exists, and the
always-loaded surface earns the rule.

This is the rule's own logic rather than an exception carved out of it. Its
stated justification is that "a rule that a stage or a tool call would load
anyway costs nothing to defer and everything to keep resident." For this actor
nothing loads it anyway, so the cost-benefit does not balance — it inverts.

**delegation-kit's own rationale is where the error actually lives, and this is
the sentence the amendment corrects.** §One template, a resident pointer argues
today that the consumer's always-loaded file carries no digest of the protocol
because "a rule is resident only when it has no load trigger … and **every
protocol bullet triggers at `Agent` dispatch**." That premise is **role-blind**.
It is true of the supervising role, which dispatches and therefore fires the
trigger. It is false of the dispatched role, which is not dispatching at all — it
is executing a stage, and its stage skill loads no delegation doc. The section
reasons from the dispatcher's vantage and silently generalizes to every reader,
which is exactly how a rule that no one disputes produced three incidents. The
fix is therefore not an exception bolted onto that section but a correction of
its premise: the trigger test is applied **per bound role**.

Content-tiering is then satisfied by **bounding what is copied**. The owning
surface keeps the tier it owns: the reasoning, the failure analysis
(turn-versus-session lifetime, why a surviving orphan is the worse case), and the
mechanism (waiting in-turn on a condition, and why ending the turn to wait is the
one act that revokes the channel). What the always-loaded surface carries is the
**imperative alone**, with the owner cited adjacently. That is still a
restatement — which is why it needs a sanction rather than an argument — but its
drift surface is one instruction rather than a slab, and the place it is
*explained and changed* remains single.

Widest-true-tier placement settles the last question, which surface holds what:
the sanction is true for every consumer that dispatches sessions, so it lands at
the kit-shipped tier as a rule about what consumers must do, while the imperative
sentence itself lands in each consumer's own always-loaded agent definition,
which is consumer-tracked because the path and its surrounding roster are the
consumer's. The deliverable therefore has two halves at two tiers, which is
precisely why the litmus reads this as a feature rather than as prose tidying.

## The governed name

**Operative residency** — stated in `delegation-kit/SPEC.md`. A rule may be
restated as an imperative in a surface that does not own it when **all three**
conditions hold:

- **(a) Unreachable trigger.** The actor bound by the rule never fires a trigger
  that loads the rule's owning doc. This is a property of the *reader*, checkable
  by asking which triggers that reader actually fires.
- **(b) Imperative only.** What is restated is the instruction — not its
  reasoning, its failure analysis, or its mechanism, which stay with the owner.
- **(c) Adjacent citation.** The restatement names the owning surface beside
  itself, so the owner remains the single place the rule is explained and changed.

**The anti-licence clause, which is part of the rule rather than commentary.**
This is not general permission to restate. It is keyed on (a), and a rule the
bound actor's own skill or template already loads fails (a) and stays a pointer.
Without that clause the sanction would read as "duplicate when it feels
important", which is the failure mode Content-tiering exists to prevent and would
cost more than the gap it closes.

## Enforcement-first, and its honest limit

**No gate is owed, and the reason is not budget.** No check can read a session's
choice to end a turn: the act leaves no tracked artifact, the same reason
`validate-verb-collision-and-check-routing` records for its own prose fix.

The anti-restatement gates were checked against the sanctioned restatement rather
than assumed inert, because a sanction that required weakening a gate would be
the wrong sanction. `check-shim-restatement` scans binding shims under the skills
directory against a corpus of the agent file plus kit templates, so a consumer
agent definition is neither its scanned surface nor its corpus.
`check-surface-duplication` is narrower still and does not run here at all: it is
unregistered in this repo's `gates.list` and exits without a glossary file, which
this tree does not ship; even where it runs, its predicate is a glossary term's
bold lead-in definition on a canonical SPEC surface, which reaches no agent
definition. So the sanction ships with **no gate exemption**, and the honest
converse is recorded with it: nothing in the battery would catch a
*non*-compliant restatement either, so conditions (a)–(c) are review-enforced.

The nearest buildable oracle is the lock sentinel in
`evidence-kit/SPEC-liveness-lock.md`, which detects the **consequence** — a
producer still running at the next stage's entry — rather than the act. That
amendment and `lifecycle-kit/SPEC-dispatch-signal.md` are the other two members
of this incident class: this one makes the completion notification **truthful**
(a session that does not end its turn on live work emits a signal that does not
lie), the dispatch amendment makes the lead depend on that signal instead of on
artifact state, and the lock detects the residual case where it is wrong anyway.

## What changes

Each delta carries its `{mechanical | design-bearing}` work class.

1. **`delegation-kit/SPEC.md` gains §Operative residency** — {design-bearing}
   The rule, its three conditions, and the anti-licence clause. Design-bearing:
   it is a doctrine ruling that sanctions a bounded exception to Content-tiering,
   and the conditions are what keep it from generalizing into a licence.

2. **`delegation-kit/templates/agent-execution.md` notes the subject rules** —
   {mechanical}
   A short note that its two dispatched-agent rules — **Background +
   notification, never poll** and **Findings you will act on are durable before
   you act on them** — are operative-residency subjects, so a reader of the owner
   knows a bounded copy legitimately exists elsewhere and does not delete it as
   drift on sight.

3. **`.claude/agents/stage-session.md`'s bullet becomes operative** —
   {design-bearing}
   The descriptive sentences are replaced by the imperatives they imply, for both
   rules the current bullet points at, with the existing citation kept as
   condition (c) requires. Design-bearing: turning a pointer into an imperative
   without importing the owner's reasoning is exactly the calibration condition
   (b) governs, and getting it wrong in either direction (too thin to act on, or
   a slab) is the defect.

## Producers and consumers

This amendment introduces a **governed rule**, not runtime state, so the causal
check is answered about the rule itself.

- **Producer** — `delegation-kit/SPEC.md` §Operative residency, the surface that
  states the sanction. It is reachable on the ordinary path because
  delegation-kit's SPEC is the doc a consumer reads when wiring dispatch, and no
  configuration gates it: the rule is unconditional kit mechanism, not a knob.
- **Consumers**, three, each by a named mechanism:
  1. **A consumer authoring an always-loaded agent definition**, at the moment it
     decides pointer-versus-imperative for a rule its dispatched role is bound
     by. This repo is the first such consumer, and delta 3 is that consumption.
  2. **A reviewer or authoring stage** assessing whether an existing restatement
     is sanctioned, by testing conditions (a)–(c).
  3. **The dispatched session itself**, which is the ultimate reader — it
     receives the imperative at the tier it always loads, which is the whole
     point of the rule.
- **Every condition has a named reader.** (a) is read by the author and the
  reviewer, at authoring and review; (b) bounds what may be copied and is the
  property a duplication gate's threshold must tolerate — read at review; (c) is
  the citation the content-tiering gate family and any later reader follow back
  to the owner.

No new field, message, or artifact is introduced, so no field-reader roster is
owed beyond the conditions above.

## Existing sections updated

- `delegation-kit/SPEC.md` §The delegation model — the new section is placed
  against it rather than appended at the end, since the sanction is about how the
  model's rules reach a dispatched role. Owned by delta 1.
- `delegation-kit/SPEC.md` §One template, a resident pointer — this section
  currently states the pointer posture for the resident tier. It is the surface
  that would otherwise read as forbidding delta 3, so it is updated to name the
  sanctioned exception rather than left to contradict it. Owned by delta 1.
- `delegation-kit/templates/agent-execution.md` — the two bullets named in
  delta 2. Owned by delta 2.
- `.claude/agents/stage-session.md` §Standing dispatch policy — the **Your turn
  end is your session end** bullet. Owned by delta 3.
- `doctrine-kit/DOCTRINE.md` — **not** updated. The sanction is delegation-kit's,
  bounded to dispatched roles, and the doctrine rules it reconciles are unchanged
  by it. Recorded so build does not widen a kit ruling into a doctrine edit.

## Out of the envelope, stated so build does not drift into it

`poll-sleep-guard-steer` is the **opposite half** of the same never-poll rule — a
session refusing to wait correctly by sleeping in the foreground, which unlike a
turn-end *does* leave a tracked artifact and so can carry a guard. It is a
separate deferred entry, not promoted this iteration, and this amendment must not
claim its ground: nothing here adds or changes a `scripts/bash-guard.sh` steer.

## Definition of Done

- [ ] **Causal completeness** — the rule has a named, reachable producer and
      three named consumers; each of the three conditions has a named reader at a
      named transition.
- [ ] **Condition (b) held in the merge** — the restatement in delta 3 carries
      the imperative and not the owner's reasoning; a merge that grows it into a
      slab has failed the rule it is implementing.
- [ ] **Merged with no information lost** — the doctrine ruling lands in
      `delegation-kit/SPEC.md`'s prose as the rationale for the sanction, not as
      an appendix.
- [ ] **Amendment deleted** — this file removed on merge; `ls delegation-kit/SPEC-*.md`
      returns none.
- [ ] **Removals propagated** — grepped every spec for names this change retired;
      nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks.
