# SPEC amendment: liveness-record

**The narrow unit, stated first, because this entry's history is mostly about a different axis.**
The subject is the **shell-child class alone**: a session backgrounds a long-running shell
producer, the observer dies, and the child survives orphaned and still writing while the harness
fires a completion notification that means only *no live `Agent` children* and is silent about a
backgrounded shell child. The unit is **a liveness read on a backgrounded producer**.

**The 2026-08-15 narrowing stands unreversed and is not re-litigated here.** The reach axis —
whether the residency rule reaches the sessions whose definitions name it — was put to the
operator as a re-scope and **not taken**, and nothing below widens back toward it. Two neighbours
are likewise untouched and are named so the boundary is drawn on purpose rather than by omission:
`waiting-rule-fourth-firing-post-fix` holds the *enforcement design* question (given that prose
alone does not hold, what does), declined as not-yet-designable, and this amendment does not
answer it; `waiter-loop-condition-predicate-gap` holds the guard's standalone loop-condition
hole, and no delta below touches a guard rule.

**What already landed, and why it does not close this.** The waiting amendment merged this
iteration named the waiting primitive by its reactivity property, gave the awaited artifact a home
in repo-local `.tmp/`, split an `Agent` dispatch (awaited by its completion notification) from a
shell child (awaited on an artifact the session placed), refused the bracket trick, and landed two
guard rules. All of it holds. What it left is a hole with a precise shape, and the shape is the
whole design problem:

- The shell-child branch says a shell child is awaited **on an artifact the session placed**, and
  names no artifact, no producer for it, and no grammar.
- The liveness clause sanctions `kill -0` against a **recorded PID** expressly *where liveness
  genuinely is the condition — a producer **this session did not start***. The self-backgrounded
  child is carved **out** of the one clause that would cover it.
- Of every long-running shell producer in this tree, exactly one — `evidence-kit/bin/run-validate.sh`
  — publishes a liveness record at all. `run-gates.sh`, `run-consumer-smoke.sh`,
  `run-gate-tests.sh`, `build-native.sh` and `demo/run-demo.sh` publish none; the timing and
  stderr files `run-gates.sh` writes are measurement, not liveness.

So a session backgrounding its own producer today has no named artifact, is excluded from the
liveness clause, and in five of six cases has no record to read even if it were not.

**The distinction the design turns on: a completion marker is a message, a liveness record is a
fact.** The obvious repair is to have the child touch a file when it finishes and wait on that.
It is not enough, and the attested failure is exactly why: a completion marker answers *is it
done* **to a live observer**, and the failure case is the one where the observer is gone. A
liveness record answers *is it still running* **to whoever arrives next**, which is a question a
dead session's successor can still ask. The unit is a liveness read and not a completion marker
for that reason.

## What changes

### 1. The shell-child branch names its artifact: a launch-time liveness record

**The doctrine stops saying *an artifact the session placed* and starts saying which artifact,
written when, in what grammar** — design-bearing, because it is the naming the whole unit reduces
to **[design-bearing]**.

A session that backgrounds a shell child **records that child's PID at launch**, to a repo-local
`.tmp/` record in the main checkout — the location the waiting bullet already fixes for awaited
artifacts, cited rather than re-argued. The record is the wait target: the session's in-turn wait
is a condition loop over that record's liveness, and the loop ends when the PID stops answering.

**The record's producer is the launcher, and that is the substantive ruling.** The alternative —
each producer claims its own lock, `run-validate.sh`'s shape generalized to its five siblings — is
refused on three counts, recorded so it is not re-drafted. It reaches only producers this tree
owns, where most backgrounded work is an ad-hoc command composed at the call site. It puts the
record's production in the one process whose death is the event being detected, so a producer that
dies before claiming is invisible in exactly the window that matters. And it bills five scripts
for a property that belongs to the *launch*, not to the program: the same script backgrounded is
in this class and run in the foreground is not. The launcher, by contrast, knows the PID of
anything it started, is the party that needs to read it back, and is the only one present at the
moment the class is entered.

### 2. The liveness clause's carve-out closes, and it closes by widening rather than by exception

**`kill -0` against a recorded PID becomes sanctioned for a self-started backgrounded child too**
— design-bearing, because it revises a clause whose current wording excludes the attested case
**[design-bearing]**.

The clause reads *where liveness genuinely is the condition — a producer this session did not
start*. That exclusion was sound on its own premise: a session that started the producer was
assumed to have a better instrument than liveness, namely its own artifact. The attested failure
falsifies the assumption — the better instrument requires a live reader, and the failure is the
loss of the reader. So the clause widens to cover a producer the session started **and
backgrounded**, and the PID it matches is the one §1 has the launcher record.

**Nothing in the refused half moves.** Pattern-matched liveness stays refused, the bracket trick
stays refused as the sanctioned repair, and the reasons stay where they are. What changes is only
which producers a *recorded* PID is sanctioned against — the widening is on the subject, never on
the predicate.

**The predicate itself is not restated here, it is cited.** `kill -0` first and `ps -p` as the
cross-uid fallback, PID reuse accepted as a residual that fails toward *held*, and an age-based
TTL refused because a long run outlives any honest one — all four are already ruled in
evidence-kit/SPEC.md §The producer-liveness lock, and a second copy in delegation-kit would be two
readings of one fact. delegation-kit owns *when a session waits and on what*; evidence-kit owns
*how a PID's liveness is decided*.

### 3. The record's grammar is evidence-kit's, and that is what gives the second reader a consumer that already exists

**The launch record is written in the `pid=<n> …` grammar the producer-liveness lock already
carries** — design-bearing, because grammar reuse is what makes the second reader free
**[design-bearing]**.

There are two readers of a liveness record and they are different parties at different
transitions. The first is the **launching session**, in-turn, as its wait condition. The second is
**whoever arrives next** — the case the attested failure is actually about, where the launcher is
gone and something else must be able to tell that a producer is still mutating shared files.

Writing the record in evidence-kit's existing grammar makes the second reader cost nothing:
`check-producer-liveness` already takes a lock path as its argument and already reports exactly
*is the producer named here still running*, with a ruled exit contract — 1 on a live PID, 0 on a
dead one or an absent record, 2 on a record it cannot parse. Pointed at a launch record it works
**unchanged**. No new gate, no new tool, no new grammar, and a reader that was already specified,
already fixture-covered and already wired.

**An inverted grammar would have cost a second implementation of a decided thing**, which is the
only argument this delta needs; it is recorded because "a launch record is not a lock, so it
should have its own shape" is the reading a later session will reach for, and the answer is that
the two records carry the same field, answer the same question, and differ only in who wrote them.

### 4. No new affordance is added, and the refusal is the design

**A `bin/` tool to write the record and a second one to read it are both refused** — design-bearing,
because declining a tool is a call a later session will otherwise re-open **[design-bearing]**.

The write is a PID captured at launch; the read is `kill -0`. Each is a shell builtin away, both
are already spelled in the doctrine, and TRAJECTORY.md's objective 6 shrinks the script-interpreter
surface to the unavoidable — a script whose body is one builtin is the clearest case of avoidable
there is. The generic reading affordance that would genuinely earn its place already exists and is
`check-producer-liveness`, which §3 reaches without adding anything.

What the refusal costs is honest and is stated rather than hidden: an affordance would make the
record's grammar impossible to get wrong, and a hand-written one can be malformed. That failure is
bounded by the consumer — `check-producer-liveness` exits **2**, fail-closed, on a record it cannot
parse, so a malformed record reads as *could not be established*, never as *nothing running*.

### 5. The restatements move in lockstep, and a third carrier outside delegation-kit takes the same clause

**The waiting imperative's carriers move in lockstep** — mechanical **[mechanical]**.
delegation-kit/SPEC.md §Operative residency's restatement sanction names the *Background +
notification, never poll* rule explicitly, and its live carriers inside delegation-kit's own reach
are exactly `.claude/agents/stage-session.md` and `.claude/agents/audit-sweep.md`. Both carry the
shell-child branch and both take §1's naming and §2's widened clause. The restatement stays an
imperative with an adjacent citation: it does not absorb §1's rejected alternative or §3's grammar
argument, which are amendment-tier and stay behind the pointer.

**A third carrier sits outside delegation-kit's own restatement roster and states the same narrow
clause in its corrective help text.** `guard-kit/lib/guard.sh`'s `guard_rule_pgrep_self_match`
blocks the self-matching `pgrep`/`pkill -f` shape, and its block message names the sanctioned
repair in the pre-widening phrasing, verbatim — *"where liveness genuinely is the condition,
waiting on a producer this session did not start — `kill -0 <pid>` against a PID you recorded"* —
mirrored in prose at guard-kit/SPEC.md §The generic ruleset's rule 12. §2 widens exactly this
clause, so this message goes stale in the same commit that fixes the two agent definitions, or a
guard-blocked session reads advice that undersells what it may now do with its own backgrounded
child. This is distinct from `waiter-loop-condition-predicate-gap`, which holds the rule's *firing
predicate* (whether loop-condition position also triggers the block) and is untouched here; this is
the rule's existing *corrective text*, carrying the clause this amendment is the ruling for.

### 6. What this unit does not claim, carried into the section so a later reader does not close it by inference

**Three non-assertions, stated where the rule lands** — mechanical **[mechanical]**.

- **Nothing here is enforced by a gate, and the reason is structural rather than budgetary.** The
  subject is *did a session record a PID before backgrounding*, which leaves no tracked artifact
  at the moment it is skipped — a session that skips the whole rule writes nothing, and a gate
  cannot read an absence it was never told to expect. The artifact-side backstop is the one
  lifecycle-kit already documents: `check-producer-liveness` on the entry pre-flight covers the
  **concurrent** case, a producer still live when the next entry is stamped. §3 widens what that
  backstop can be pointed at; it does not make the rule itself checkable, and this amendment does
  not claim it does.
- **The enforcement-design question stays open.** *Given that prose alone does not hold, what
  does* is `waiting-rule-fourth-firing-post-fix`'s, declined as not-yet-designable, and no delta
  above answers it. What this unit removes is a different failure: a session that **wanted** to
  comply had no named artifact and was carved out of the only clause that fit. That is a gap in
  the rule, not in its enforcement.
- **The reach axis is not reopened.** The narrowing that reduced this entry to the shell-child
  class stands, and §5 propagates to the two carriers that already exist rather than proposing a
  reach mechanism.

## Producers and consumers

**This amendment introduces one artifact and one widened clause. It introduces no new knob, no new
tag, no new gate, no new tool, no new evidence surface, and no new file grammar.**

**The launch liveness record (§1, §3).** *Producer:* the session that backgrounds a shell child,
at the launch, writing to repo-local `.tmp/` in the main checkout. Its **enabling configuration is
the scratch directory that already exists** — `.tmp/` is gitignored, present in every consumer that
vendored the kit, and already the doctrine's named home for an awaited artifact, so the producer is
reachable on the ordinary path and not only in a described ideal. *Consumers, both named at named
transitions:* (i) the **launching session**, in-turn, at every iteration of its condition wait,
reading the record's PID through `kill -0`; and (ii) **`check-producer-liveness`**, at a stage
entry's pre-flight or at any point a later party asks whether an orphan is still writing, reading
the same record through the same predicate and reporting its ruled exit contract.

**The record's fields have named readers, and there are exactly two fields.** `pid=<n>` is read by
both consumers above, at both transitions, and is the whole liveness question. The trailing
identifying field is read by consumer (ii) **only**, at the moment it refuses, to name *which*
producer blocks — a refusal that cannot say what is running is a refusal its reader cannot act on.
No third field is introduced: a start timestamp was already weighed and dropped as reader-less
when this grammar was ruled, and nothing in this amendment gives it a reader.

**The widened liveness clause (§2).** *Producer:* the delegation-kit template, at every load of the
agent-execution skill, its restated copies in the two agent definitions at every dispatch of those
types, and its third restated copy in guard-kit's corrective help text (`guard-kit/lib/guard.sh`,
mirrored at guard-kit/SPEC.md §The generic ruleset rule 12) — which is the propagation half, and
why §5 is not optional. *Consumer:* the dispatched session choosing how to wait, at the moment it
has backgrounded a shell child and holds no result; for the third copy, any session the guard has
just blocked, reading the corrective inline. A human or agent reader, never a gate — §6 states that
limit rather than leaving it to be inferred from the absence of one.

**Existing integration prose describing the prior flow is updated, not left to drift** — see below.
The one flow that genuinely changes is the shell-child wait: it stops being *place some artifact
and loop on it* and becomes *record the PID at launch, loop on its liveness, and leave a record the
next arrival can read*.

**No corpus is narrowed by this amendment**, so causal-completeness point 5's red-condition
enumeration does not bind: every delta adds prose or widens a clause, and no reader's input set
shrinks. Stated because the point is easy to skip silently and its absence should be a finding
rather than an omission.

## Existing sections updated

- **delegation-kit/templates/agent-execution.md**, the *Background + notification, never poll*
  bullet — §1, §2 and §4. The shell-child branch gains the launch record as its named artifact; the
  liveness clause's *a producer this session did not start* carve-out widens to include a
  self-backgrounded child; the refused affordance is not mentioned here, staying amendment- and
  SPEC-tier. The `.tmp/` home is cited from the bullet's existing clause, never restated.
- **delegation-kit/SPEC.md §The delegation model** — §1, §2, §3, §4. The owning section for the
  waiting contract. It gains the launcher-as-producer ruling with its three refused grounds, the
  clause widening with the premise that failed, the grammar-reuse argument and its second reader,
  and the refused affordances with the fail-closed bound that makes the refusal safe. This is where
  the rejected alternatives live permanently; the template carries the imperative.
- **delegation-kit/SPEC.md §Operative residency** — §5. The restatement sanction already names this
  bullet; the section records that the propagate obligation fired here and which two carriers took
  it.
- **evidence-kit/SPEC.md §check-producer-liveness** — §3's other half. The gate is unchanged in
  every respect, and what is added is the sentence that makes its reuse legible: its argument is a
  record path, so any record in the lock's grammar is a legal subject, and a launch record is one.
  Stated in the gate's own section because a reader deciding what they may point it at reads there,
  not in delegation-kit.
- **evidence-kit/SPEC.md §The producer-liveness lock** — §2's citation target. The predicate, the
  PID-reuse residual and the refused TTL are unchanged; the section notes that its grammar now has
  a second writer class, so a later change to the record shape has both callers in view.
- **`.claude/agents/stage-session.md` and `.claude/agents/audit-sweep.md`** — §5.
- **guard-kit/SPEC.md §The generic ruleset (rule 12) and `guard-kit/lib/guard.sh`** — §5. The
  self-matching-`pgrep` corrective's parenthetical — *a producer this session did not start* —
  takes the same widening the two agent definitions take; the loop-condition firing predicate
  itself (`waiter-loop-condition-predicate-gap`'s subject) is untouched.
- **TASK-QUEUE.md** — `waiting-rule-carrier-reach` moves to `## Done`, dropping its `[spec:]` tag.
  Its deliverable under the narrowing is the shell-child class alone and this amendment discharges
  it whole rather than incrementing it, so it does not take the demotion branch. Its `recurrence:`
  declaration goes with the Done move, as every tag does.

## Definition of Done

- [ ] **Causal completeness** — the record has a named, reachable producer and two named consumers;
      both its fields have a named reader at a named transition.
- [ ] **The second reader is proved, not assumed** — `check-producer-liveness` run against a launch
      record written in the landed grammar, returning 1 against a live PID, 0 against a dead one and
      an absent file, and 2 against a malformed record. §3's entire argument is that this reader is
      free; if it does not read the record unchanged, the grammar is wrong, not the gate.
- [ ] **The carve-out is closed, not duplicated** — the widened clause reads as one rule about
      recorded PIDs, and no second sentence describes the self-started case separately.
- [ ] **The predicate is cited, never restated** — delegation-kit carries no copy of `kill -0`-then-
      `ps -p`, the PID-reuse residual, or the TTL refusal; `check-shim-restatement` is the oracle,
      and a shared n-gram against evidence-kit's section is the failure this bullet exists to catch.
- [ ] **The three non-assertions survive the merge** — nothing in the landed text claims a gate
      enforces the rule, answers the enforcement-design question, or reopens the reach axis.
- [ ] **The restatements propagated** — both agent definitions carry the named artifact and the
      widened clause, guard-kit's corrective help text and its SPEC mirror carry the widened
      clause, and none of the three absorbs amendment-tier reasoning.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); each merged spec reads as one coherent document a reader who never saw
      the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls delegation-kit/SPEC-*.md`). Discharged at the **iteration**, not at this commit, where
      sibling amendments are in flight.
- [ ] **Removals propagated** — grepped every surface for the retired *a producer this session did
      not start* phrasing and for any surviving claim that a shell child's awaited artifact is
      unspecified; nothing dangles.
- [ ] **Gaps filed** — any producer discovered to need a record it cannot get from its launcher,
      and any cross-component gap found during the work, filed as debt rather than absorbed.
