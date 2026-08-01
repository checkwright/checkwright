# SPEC amendment: agent-execution-backgrounding-role-scope

<!-- The file basename drops the `agent-execution-` prefix the queue slug
     carries: the prefix is already implied by the amendment's directory and its
     subject, and the full slug would push the queue entry's lead line past
     check-queue-wrap's budget with the [spec:] tag check-tag-lead-line pins
     there. The pairing is by the explicit [spec:] ref, not by name equality. -->

The agent-execution protocol states its rules for the **dispatching** role and
leaves the dispatched role unstated. Two rules have now failed that way, and
both failed the same silent way: nothing reddened, the session reported in good
faith, and the loss looked like something else.

- **Backgrounding.** The **Background + notification, never poll** rule says to
  always dispatch in the background and wait for the notification. Load-bearing
  for a supervisor. A stage session read the same protocol, backgrounded its
  whole validate suite, ended its turn to report progress, and its child died
  with it — an entry stamp, no results, the suite re-run from scratch. Missing
  evidence is indistinguishable from pending evidence, which is why nothing
  caught it.
- **Durability of findings.** The resume-journal mechanics are written for a
  dispatched *mutating* agent — the agent writes, the supervisor deletes. A
  read-only child journals nothing, so its findings exist only as a return value
  in its parent's context. A close stage dispatched two audit sweeps and then
  hit the session wall; both had already returned, which was luck. Had the wall
  landed mid-sweep, the audit work would have been lost with no record it ran.

One defect, one role-scoping pass. This amendment does the pass **in the bullet
bodies only** — every bold lead-in is preserved verbatim, because a lead-in is a
rule's stable name and `check-rule-citation` resolves the SPEC's citations
forward into it. Folding a role marker into a lead-in would rename the rule and
drag the citations into the same commit for no semantic gain.

**Sequencing.** The debt unit resolving the resume journal's contradictory
lifetimes lands **first** in this iteration's delegation lane and edits two of
the same surfaces. Nothing here restates the journal's lifetime or its deletion
timing; where these deltas touch the journal they cite it. Delta 3 is stated as
a semantic delta against whatever text that unit leaves, deliberately, rather
than as a verbatim before/after that would go stale between the two commits.

## What changes

### Delta 1 — the backgrounding rule gains the fact that decides it {design-bearing}

Appended to the body of **Background + notification, never poll**; the lead-in
is untouched.

The rule reads as universal because the sentence that would scope it is missing,
and the missing sentence is not "supervisors only" — it is *what backgrounding
actually survives*:

> Backgrounding survives a **turn**, not a **session**. A supervisor's turn ends
> and its session lives on, so a backgrounded dispatch wakes it by notification
> — that is the sanctioned pattern above. A **dispatched agent's** turn end *is*
> its session end: it returns and terminates, taking every background child with
> it. So an agent awaits its own long-running work in the **foreground**, however
> long, and never ends a turn to report progress on work still running.

**Why this phrasing and not a role-neutral one.** A neutral "never end your turn
with detached work" was drafted and rejected: it forbids the supervisor pattern
the same bullet prescribes two sentences earlier, because a supervisor ending a
turn on a background dispatch is exactly correct. The two roles genuinely differ,
and they differ on one fact — whether turn end and session end coincide — so
naming that fact scopes both arms at once without asserting either as a bare
prohibition. A reader who knows *why* transfers the rule to a case the bullet
does not enumerate.

**Provenance of the fact, stated honestly.** That backgrounded work survives a
turn is documented harness behavior. That a *subagent's* return terminates its
background children is **attested here and inferred from the harness docs, not
stated by them.** The rule is therefore anchored on the invariant rather than on
the mechanism: an agent's contract with its caller is its **return value**, so
work whose result is not in the return did not happen as far as the caller is
concerned — true whether or not a given harness version happens to reap the
child. The mechanism is cited as the attested reason, so a harness that changed
it would cost this bullet a sentence, not its rule.

### Delta 2 — a new rule: findings are durable before they are acted on {design-bearing}

A new protocol bullet, whose lead-in is its stable name:

> **Findings you will act on are durable before you act on them.** A child's
> return value lives only in your context, and your context dies with your
> session. Before you *act* on a dispatched agent's findings — edit against
> them, rule from them, plan the next wave on them — land them somewhere that
> survives you: a commit, or your own journal. The write is the parent's,
> discharged on receipt, not a chore delegated back to the child.

This is deliberately role-neutral and *is* achievable here, unlike delta 1: it
binds whoever holds findings they will act on, at any depth. The gap it closes
is a fan-out parent's, but a lead consuming a stage session's report is the same
shape and the same rule.

The cost is bounded by design: it obliges a write on **receipt**, not a running
narration, so a parent pays one write per returned child rather than a second
journal discipline.

### Delta 3 — the read-only-fan-out caveat is narrowed, not overturned {design-bearing}

The protocol currently rules the opposite for exactly delta 2's case: for a
read-only fan-out "the return value *is* the contract — don't rely on a
journal", because a background agent's sandbox may block the journal write. That
caveat is three weeks older than the incident above and rests on its own
observation, so it is a live ruling, not debris — and it is **kept**.

The resolution is that the two were never about the same party:

- The caveat's evidence is about **the child's** write failing silently. Its
  true content is *do not make recoverability depend on a backgrounded child's
  write*, and that content survives intact.
- Delta 2's evidence is about **the parent** holding the only copy. Its
  requirement is discharged by the parent, which has already demonstrated it can
  write — it is the session that granted the path.

So the duty moves to the **receiving end** rather than being cancelled. The
caveat gains that clause: the return value is the contract, and a contract has
two ends — the child owes no journal, and the parent owes the durable landing of
what it received before acting on it. This is strictly better than overturning:
overturning would put the write back where the evidence says writes fail, and
would re-manufacture the false `.tmp/` defect signal this iteration is already
paying for elsewhere.

Stated as a semantic delta because the debt unit landing first owns the
surrounding text: wherever the read-only-fan-out caveat ends up worded, it gains
the receiving-end clause and loses nothing.

### Delta 4 — the SPEC records both failure surfaces {design-bearing}

§The delegation model keeps "only the rationale that earns spec residency (a
failure surface, a calibration history, a bound that is correctness rather than
preference)". Both incidents are failure surfaces and both earn it — a
lost-suite and a nearly-lost audit sweep are the calibration history for why
these two rules read the way they do.

Two paragraphs, each citing its rule by name in the established grammar (`the
template's **<name>** rule`) rather than restating it:

- The backgrounding paragraph records the attested loss and delta 1's deciding
  fact, and marks the subagent-reaping half as attested rather than documented,
  so a later reader does not upgrade it to a harness guarantee.
- The durability paragraph records the near-loss, and records delta 3's
  narrowing explicitly — that the read-only-fan-out caveat was re-examined
  against new evidence and kept with its duty relocated, not overturned. Without
  that sentence the next reader finds two rules that look like each other's
  contradiction and re-litigates the whole thing.

Delta 2's new lead-in must match its citation verbatim; `check-rule-citation`
resolves it forward and is the oracle for this delta.

### Delta 5 — the consumer's agent definition gains one pointer {mechanical}

`.claude/agents/stage-session.md` §Standing dispatch policy is the surface every
dispatched stage session loads unconditionally — including the validate session
that hit the backgrounding incident, which is not guaranteed to have loaded the
protocol at all. One bullet there, **citing** delta 1 and delta 2 rather than
transcribing them, per the agent-definition's own content-tiering rule ("it
cites delegation-kit's resume-journal mechanics, say, never transcribes them").

It carries no journal-lifetime claim, so it does not contend with the debt
unit's edit to the neighbouring bullet.

**Honest limit, and why no more is attempted.** The rules reach a session that
loads the protocol or the agent definition; a session that backgrounds work
having loaded neither carries only the resident pointer. That is exactly the
limit §One template, a resident pointer already states for protocol literacy,
and it is cited rather than restated. Promoting either rule into always-loaded
residency was considered and rejected: it fights load-trigger residency, and the
one surface that would need it — a session with no dispatch trigger at all — is
a wider unit than this one.

### Delta 6 — landing {mechanical}

The docs-site mirror of delegation-kit/SPEC.md and the other generated
projections, regenerated; each freshness gate names its own regen command on
red. `check-rule-citation` and the delegation-kit fixture runner are the gates
this unit must leave green.

## Producers and consumers

Both deltas add **rules**, not machinery, so causal completeness here is about
who reads them and at which transition. Nothing new is emitted, so no field is
introduced that could lack a reader.

- **The backgrounding arm (delta 1)** — *Producer:* the protocol template, read
  at the dispatch/long-running-work decision. *Consumers:* (a) a dispatched
  stage session, at the moment it chooses foreground versus background for its
  own long-running oracle — the transition the validate incident failed at; (b)
  the supervisor, unchanged, at its dispatch transition. Its enabling load is
  real rather than nominal: the template is loaded by `/agent-execution`, whose
  block message the budget guard prints, **and** the arm is cited from the
  consumer agent definition (delta 5), which a dispatched stage session loads
  unconditionally. That second path is what makes the rule reachable by the role
  that broke it.
- **The durability rule (delta 2)** — *Producer:* the protocol template.
  *Consumer:* any session holding a returned child's findings, at the transition
  where it first acts on them. The durable artifact it obliges — a commit or a
  journal entry — has an existing named reader: a cold-read resumer, whose
  contract §Resume journal already owns.
- **The narrowed caveat (delta 3)** — *Producer:* the protocol template and
  §Resume journal. *Consumers:* the dispatching parent (now told it owes the
  landing) and the read-only child (still told it owes no journal). The two
  readings are disjoint, which is the point of the narrowing.
- **The SPEC paragraphs (delta 4)** — *Producer:* §The delegation model.
  *Consumers:* a maintainer reading for rationale, and `check-rule-citation`,
  which reads the citations mechanically and reds if a cited name does not
  resolve to a template lead-in.

**Seam.** Pure mechanism. The rules are about dispatch lifetime and evidence
durability — universal to any coding-agent harness with a parent/child dispatch
model — and carry no consumer vocabulary, no project rule content, and no new
knob. The one consumer-side artifact is the pointer bullet in this repo's own
agent definition, which is consumer config by construction and names nothing the
kit could not have named generically.

## Existing sections updated

- **`delegation-kit/templates/agent-execution.md`** — the **Background +
  notification, never poll** body (delta 1); the new **Findings you will act on
  are durable before you act on them** bullet (delta 2); the read-only-fan-out
  caveat inside the resume-journal bullet (delta 3, after the debt unit lands).
- **delegation-kit/SPEC.md §The delegation model** — the two rationale
  paragraphs and their rule citations (delta 4).
- **delegation-kit/SPEC.md §Resume journal — agent writes, supervisor deletes**
  — the caveat's receiving-end clause (delta 3), in the same section the debt
  unit edits for lifetime; the two edits are disjoint sentences and the debt
  unit's lands first.
- **`.claude/agents/stage-session.md` §Standing dispatch policy** — the pointer
  bullet (delta 5).
- **The docs-site mirror of delegation-kit/SPEC.md** and the other generated
  projections (delta 6).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
