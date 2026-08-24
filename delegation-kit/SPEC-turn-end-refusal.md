# SPEC amendment: turn-end-refusal

Turns delegation-kit's `SubagentStop` observer into a refuser. Pairs with
`turn-end-chokepoint-and-wait-primitive`. The authorization is
TRAJECTORY.md §The closed rulings, 2026-08-24, unconditional — and that ruling
names its own discharge event ("spent when the enforcing hook ships"), so the
merge that retires this amendment retires the ruling with it.

**What the ruling binds, restated only because getting it wrong is the failure
mode.** The capped variant — refuse only on `verdict=red live=yes` carrying at
least one record, advisory otherwise — was offered as an explicit option and
refused. Every narrowing below is therefore argued on its own stated grounds,
and none is an inherited safety margin. Where a narrowing here happens to
coincide with the refused option's boundary, the coincidence is called out and
the independent ground is given, so a later reader cannot read the refused
option back in through the resemblance.

## What changes

### (1) The hook refuses, by exit 2, and the refusal needs no emitter

`templates/subagent-stop-liveness.sh` stops exiting 0 unconditionally. It exits
**2** when the liveness reading is `red` or `corrupt` (delta 2 owns that pair),
writing its message to **stderr**, and exits 0 on every other path. It emits no
hook JSON on either path. {design-bearing}

**The emitter cost this variant was priced with came back as zero, and that is
a repair to the current section rather than a new claim.** §The turn-end
liveness probe (template) argues that a hook which had to *speak* at
`SubagentStop` would need a primitive guard-kit does not have, since all three
of guard-kit's emitters hardcode `hookEventName:"PreToolUse"` — and concludes
that "the blocking variant's emitter cost is stacked on its own authorization".
The harness's published hook contract, fetched at authoring rather than
recalled, settles it the other way for the exit-code route: `SubagentStop` is
listed as an event a hook **can** block, exit 2 is the blocking route, and the
hook's **stderr is shown to Claude and is itself the blocking reason when the
hook emits no JSON decision**. So the refusal speaks through stderr, needs no
emitter, mints no guard-kit primitive, and leaves this template's standing
property — it sources no kit lib, so delegation-kit acquires no dependency on
guard-kit being vendored — intact.

**The same contract closes the advisory question before it is asked.** At exit
0 a hook's stderr goes to the debug log only and Claude never sees it. There is
therefore no advisory tier at this event: the choice is deliver or do not
deliver, and exit 2 is the only delivery. A "log it more loudly" alternative
does not exist to be weighed.

**Honest limit, inherited deliberately.** That roster is the harness's
published contract and nothing in this tree. It sits on the same footing the
current section already declares for the `PreToolUse` payload roster: a future
harness revision reshaping the event is drift no gate here can self-detect, and
only re-reading the contract catches it. Recorded so the next reader does not
mistake a fetched fact for a measured one.

### (2) The refusal predicate: `red` **or** `corrupt`, and nothing else

The hook maps `check-producer-liveness`'s exit class exactly as it does today,
and refuses on two of the four arms. {design-bearing}

| reading | today | after | why |
| --- | --- | --- | --- |
| `green` | log | log, exit 0 | no live producer; there is nothing to refuse |
| `red` | log | log, **exit 2** | a live producer under a launch record — the whole subject |
| `corrupt` | log | log, **exit 2** | see below; this is the arm with a real argument |
| `unavailable` | log | log, exit 0 | the hook holds no reading at all |
| `error` (new, delta 3) | — | log, exit 0 | the hook holds no reading at all |

**`corrupt` refuses, and it diverges from guard-kit rule 14 on purpose.** Rule
14 — tracked-tree mutation under a live producer — states that "a record that
does not parse **declines** rather than blocks, because a guard is not the place
a corruption verdict is taken and `check-producer-liveness` already exits 2 on
one". That reasoning does not transfer, and the reason it does not is structural
rather than a matter of appetite. Rule 14 reads the records **one at a time**, so
a malformed record declines *for itself* while a sibling record naming a live PID
still blocks. This hook reads the whole set through **one exit code**, and
`check-producer-liveness`'s own resolution is *exit 2 wins over red wins over
green* (evidence-kit/SPEC.md §check-producer-liveness). Allowing on `corrupt`
would therefore mean that **one malformed record anywhere under the scratch dir
suppresses every refusal in the tree** — a bypass rule 14 does not have and
cannot have, minted by copying rule 14's disposition into a reader that lost the
per-record view. The divergence is recorded here, in both directions, so neither
surface reads as the other's drift.

**The corrupt arm is cheap to be wrong about and expensive to skip.** A corrupt
record cannot be carried far: `check-producer-liveness` is a battery member, so
the next commit reds on it. The false-refusal window is one malformed record's
lifetime; the bypass window, if the arm were dropped, is however long a session
cares to leave one in place.

**`records=0` is not a clause, and the resemblance to the refused option is
disclosed.** The refused capped variant read "`verdict=red live=yes` carrying at
least one record". The record count is not a condition here and is not being
imported as one: `check-producer-liveness` cannot return red over an empty set,
so `red` already implies at least one record by the reader's own contract. That
half of the refused option was vacuous, and this amendment neither adopts nor
needs it.

**`unavailable` and `error` allow, on the degradation posture and not on
leniency.** Both mean the hook obtained no reading — the knob names no reader,
or a configured reader failed. Refusing there would refuse every turn end in a
tree that has not configured a reader, in a kit that ships this hook opt-in and
inert. It is guard-kit/SPEC.md §The guard framework's fail-open-but-loud posture
for a deny-guard whose rule turns on an external reader, the same posture
§The delegation model's dispatch guard already takes, and delta 3 is what
supplies the "loud".

**`jq`-absence does not disable enforcement, and this is the one place this hook
is strictly better off than its `PreToolUse` sibling.** The decision reads the
liveness reader over the run directory; it reads **no payload field at all**. The
payload feeds only the log's `event`, `session` and `keys` columns. So an absent
`jq` degrades the log line and leaves the refusal exact, and the advisory-envelope
problem the dispatch guard had to solve by hand does not arise here.

**The bounded call stays, and its meaning inverts.** The reader is still invoked
under `timeout` where one is available. Today the comment on that call reads "a
reader that hung would refuse the turn end by accident, which is the blocking
variant this one is not". After this amendment the hook *is* the blocking
variant, and the bound is what keeps a hung **reader** from being mistaken for a
live **producer**: a timeout is an `error` (delta 3) and allows, so the refusal
is only ever the reader's own verdict.

### (3) The log line gains `decision`, and `verdict` gains `error`

The grammar in §The turn-end liveness probe (template) takes one new field and
one new value. {design-bearing}

```
<UTC ISO-8601>  event=<…>  session=<…>  live=<yes|no>  verdict=<green|red|corrupt|error|unavailable>  records=<n>  decision=<refuse|allow>  keys=<…>
```

- **`decision`** — `refuse` exactly when the hook exits 2. Its named reader is
  the **close-stage triage** at the close-surface drain, at the transition where
  the log is read and cleared, and it is read for a question no existing field
  answers: whether a firing was **acted on**. `verdict` alone cannot answer it
  across the landing commit, because the same `verdict=red` line means *observed*
  before this amendment and *refused* after it, in one append-only file that
  spans both. `decision` is also what makes a refusal countable — the forcing
  function's own effectiveness is otherwise unmeasurable, which is the defect
  §The probe is asymmetric spent a whole iteration recording about `live=no`.
- **`verdict=error`** — a **configured** reader that ran and did not answer: an
  unmapped exit code, or the `timeout` bound firing. Its named reader is the same
  triage, at the same transition, distinguishing *this tree never configured
  enforcement* (`unavailable`) from *this tree's enforcement is broken*
  (`error`). Only the second is actionable, and today they are one token. The
  value earns its place only under enforcement: before it, both meant "no
  reading" and the distinction cost more than it bought.
- **`live`** stays two-valued and stays paired with `verdict`. A `corrupt`
  refusal carries `live=no decision=refuse`, which is exactly why `decision`
  cannot be derived from `live` and has to be its own column.

`decision` is placed before `keys` deliberately: `keys` is the one free-ish
field and sits last so the space-delimited parse never has to step over it.

### (4) The refusal message, and where the valve is

The stderr text names the finding and both lawful exits, as guard-kit requires
of every block message. {design-bearing}

It states: a launch record under the scratch dir names a live producer (or a
record does not parse); the turn may not end on it; and the two ways forward are
the two rule 14 already names — **wait for the producer on its own artifact, or
delete the record once the producer has exited** — plus, for the `corrupt` arm,
run the reader to see which record is malformed. It names the reader command so
the session can see the record set for itself. It carries no session identity,
because the hook has none to carry (delta 5).

**There is no knob, and unwiring the hook is the valve.** §The delegation model
rules exactly this for D1's unconditional block: "the valve is **unwiring the
hook**, never a knob: a per-dispatch override is exactly the honour system these
rules exist to end, and a knob would restore it under a better name." The same
reasoning binds here and for the same reason, so no new knob is introduced.

**The ordinary escape is not the operator's, and that matters under the
permission wall.** Unwiring means editing the consumer's settings, which is
operator-class work no stage session may do (TRAJECTORY.md §The closed rulings,
2026-08-22). If unwiring were the *only* escape, this hook would be a mechanism
whose every recovery path needed the operator. It is not: the record set is the
session's own artifact, `check-producer-liveness` names the blocking record, and
deleting a record whose producer has exited is — in rule 14's own words — "not a
workaround, it is the statement of fact becoming false and being retracted". The
operator-class valve is the last resort, not the first.

**Loop protection reads the producer, never `stop_hook_active`.** The live
payload carries a `stop_hook_active` key — this tree's own `keys` reading found
it — and the published hook contract does not mention the field at all. Keying
the refusal off it would couple a hook to an **uncontracted** harness artifact,
which is the precise ground §The delegation model gives for refusing the
transcript-meta depth route while a contracted boolean was available. Here the
contracted alternative is better than a boolean: the refusal's own trigger is a
real-world condition that ends when the producer ends, so the loop is bounded by
the thing the rule is about. A producer that never ends is bounded by the
message's second exit.

### (5) The shared scratch dir is the subject, stated rather than discovered

The hook's reading is over `${GATE_SDK_TMP_DIR:-.tmp}`, which concurrent
sessions in one checkout share. Under enforcement that stops being cosmetic: a
record written by one session can refuse another's turn end. This is stated as
the mechanism's subject, not filed as a defect. {design-bearing}

**The project already binds every session to that shared set, and this changes
no policy.** guard-kit rule 14 blocks index-writing `git` in **any** session
while any `*.run` record under a scratch dir names a live PID — it is not
narrowed to the record's writer, and it never was. Extending the same binding
from "you may not commit under a live producer" to "you may not end your turn
under one" widens the *act set*, not the subject. A design that narrowed the
turn-end rule to the writer would leave the two rules disagreeing about whose
producers bind whom, on one record set, with no surface owning the difference.

**Attribution was weighed and is not available.** The payload's `session_id` is
shared by a dispatched agent and its dispatcher — the current section records
that the field settled this against its own original reader — and the `pid=<n>
run=<key>` grammar carries no writer identity. Adding one is a grammar change
across evidence-kit and guard-kit for a narrowing the paragraph above argues
against wanting.

**The residue, costed.** Two firings this hook cannot reach are unchanged and
are not closed here: an **unrecorded** launch (guard-kit rule 15's advisory
residue), and the harness's own `background_tasks` view, which §What
`background_tasks` carries established enumerates what was *launched* rather
than what is *running* and so "cannot substitute" for the record set. Both stay
exactly as recorded.

### (6) `Stop` stays unregistered, and the per-step firing is the cost this buys

Neither the template nor this repo registers `Stop`. That is unchanged; the
main-session turn end still has no attested firing, and registering it would
widen the subject past the evidence. {design-bearing}

**`SubagentStop` is not the session-end event, and enforcement inherits that.**
The current section records the measurement: it fired seventeen times inside one
dispatched session that had ended no turn at all, spaced by assistant steps. So
a refusal fires at intermediate steps too, and the amendment does not pretend
otherwise.

**Why that is accepted rather than narrowed.** There is no contracted
discriminator for a real turn end in the payload, and both uncontracted
candidates are refused on the precedent delta 4 cites. What is left is a
judgment about the cost, and it comes out in favour: at an intermediate step a
refusal reaches the agent with the obligation it is about to breach *before* it
breaches it, which is earlier and cheaper than at the end, and it is the only
channel that reaches the agent at all (delta 1). The frequency is bounded by the
producer's own life, and a session that is correctly waiting in-turn is making
few assistant steps by construction.

**What the build must observe rather than assume.** The contract says exit 2
"prevents the subagent from stopping"; it does not say what that means at a
firing where the subagent was not stopping. Both readings are survivable — the
reason is injected and the session continues, or nothing happens and the line is
still logged — and the design does not branch on the answer. The build wires the
hook, reads its own `decision=refuse` lines against the transcript, and records
which reading holds in the merged section. It is not licensed to change the
predicate on what it finds; a finding that argues for a different predicate is an
escalation, not a build decision.

### (7) The section is renamed, and the mirrors follow

`## The turn-end liveness probe (template)` becomes `## The turn-end liveness
hook (template)`. Every in-tree pointer moves with it; the `docs/` mirrors are
generated and are regenerated, not edited. {mechanical}

One word, because a section titled *probe* owning a mechanism that refuses is
the stale-name defect this tree files entries about, and because a one-word
change keeps every pointer's diff trivially reviewable. `check-spec-pointer` is
the oracle: a missed pointer reds. The in-tree pointer set at authoring is
delegation-kit/SPEC.md (four self-references plus the heading),
delegation-kit/README.md, guard-kit/SPEC.md's rule 14, TRAJECTORY.md's ruling,
`delegation-kit/smoke/install.sh`, and the four `# spec:` lines across the
template and its consumer copy — enumerated as a starting point for the sweep,
never as a substitute for running the oracle.

### (8) Behavioral coverage, on both sides of the copy seam

`gate-tests/subagent-stop-liveness.test.sh` keeps driving a **stub** reader per
exit class and gains the exit-code assertion for each: 2 on `red` and `corrupt`,
0 on `green`, `unavailable` and `error`, plus the `decision=` column's value on
each arm and a non-empty stderr on the two refusing arms. A stub is what lets it
hold every arm hermetically, which is why it stays a stub. {design-bearing}

`scripts/gate-tests/subagent-stop-reader.test.sh` — the consumer-side test that
fires this repo's own hook copy against its own configured reader — must be
updated in the same unit, because it already constructs a run dir with a record
naming an always-alive PID and asserts `red`. That firing now exits 2, so a test
left unchanged goes red on a correct implementation. It gains the exit code and
the `decision=refuse` column as assertions rather than merely tolerating them.
{mechanical}

The gap between the two lanes is the one the current section already names — a
dead default lived for a whole iteration under a green battery because only the
consumer configures the reader — and this amendment adds enforcement to exactly
that seam, so both lanes move together or the seam re-opens.

## Producers and consumers

**New state — the refusal.**
- *Producer:* `templates/subagent-stop-liveness.sh`, and its vendored copy
  `scripts/subagent-stop-liveness.sh`, at a `SubagentStop` firing whose liveness
  reading is `red` or `corrupt`; the enabling configuration is the
  `SubagentStop` hook registration in the consumer's settings **plus**
  `DELEGATION_KIT_LIVENESS_CMD` naming a readable reader. Both are already
  present and deployed in this repo — the registration in `.claude/settings.json`
  and the reader at `scripts/producer-liveness-reader.sh` — so no
  permission-class edit is reachable from this unit and the 2026-08-22 wall is
  not met. Recorded as a **cost** finding: the authorization rests on the class,
  not on this.
- *Consumer:* the harness, by contract, at the `SubagentStop` transition — it
  converts exit 2 into a refusal and shows the hook's stderr to the dispatched
  agent as the blocking reason. The **second** consumer is the dispatched agent
  itself, which receives that stderr as its only delivery of the obligation.

**New field — `decision`.**
- *Producer:* the same hook, one value per firing, on every path including the
  allowing ones.
- *Named reader:* the close-stage triage of `.workflow/subagent-stop-liveness.log`
  at the close-surface drain transition (lifecycle-kit/SPEC.md §The close-surface
  roster; the log's roster line is unchanged and stays `advisory`), reading it to
  separate an acted-on firing from an observed one across the landing commit.

**New value — `verdict=error`.**
- *Producer:* the same hook, on a configured reader that ran and returned an
  unmapped exit code, the `timeout` bound included.
- *Named reader:* the same close-stage triage at the same transition, reading it
  to tell an unconfigured tree from a broken one.

**No field is added that this list does not name a reader for**, and the two
values the ruling might have invited and that are **not** added are recorded so
the omission does not read as an oversight: no session-attribution field (delta 5
— the payload has no attributing value to carry) and no refusal counter (delta 4
— the bound is the producer's life, not a count).

**Existing integration prose describing the prior flow** is enumerated in the
next section; every one of them currently asserts the logging-only boundary in
its own words, so none may be left to drift.

## Existing sections updated

- delegation-kit/SPEC.md §The turn-end liveness probe (template) — the whole
  section: the logging-only boundary, the "exits 0 unconditionally" contract, the
  emitter-cost paragraph, the "it never wedges a turn" paragraph, the grammar
  block and its field list, the bounded-call reasoning, and the heading itself
  (deltas 1, 2, 3, 4, 5, 6, 7 and 8).
- delegation-kit/SPEC.md §The probe is asymmetric, and no reading may treat it
  otherwise — its closing sentences say a blocking hook "is a second
  authorization this result does not grant" and weigh the per-step firing as a
  cost that authorization should be measured against. Both are now spent
  (deltas 1 and 6).
- delegation-kit/SPEC.md §Operative residency — the paragraph ruling that the
  turn-end "buys enforcement only through a blocking hook nobody has authorized",
  and the paragraph recording the search that found an oracle which "*observes*
  rather than one that refuses" (deltas 1 and 6).
- delegation-kit/SPEC.md §What `background_tasks` carries — its closing finding
  that a blocking variant "cannot substitute" the harness view for the record
  set now describes a shipped mechanism rather than a hypothetical one
  (delta 5).
- delegation-kit/README.md — the wiring step's parenthetical calling the hook a
  non-enforcement mechanism, and the section pointer in it (deltas 1 and 7).
- guard-kit/SPEC.md §The generic ruleset, rule 14 — its sentence "observing there
  is not refusing there — so this rule stays the enforcement and is unchanged".
  Rule 14 does stay unchanged and stays the `PreToolUse` enforcement; what
  changes is that the turn-end axis now refuses too, and the corrupt-arm
  divergence in delta 2 is stated from this side as well as from
  delegation-kit's, so neither reads as the other's drift (deltas 2, 5 and 7).
- TRAJECTORY.md §The closed rulings, the 2026-08-24 authorization — it names its
  own discharge event and this unit is that event, so the merge retires it as a
  spent ruling. Retiring a spent ruling is not reversing one (deltas 1 and 7).
- delegation-kit/smoke/install.sh — its crafted-payload assertion requires the
  probe to "exit 0 … whatever the reader's verdict", which is now false for two
  verdicts; the smoke's own scratch dir is empty, so the assertion narrows to the
  arm it actually exercises rather than being deleted (deltas 1 and 7).
- `templates/subagent-stop-liveness.sh` and `scripts/subagent-stop-liveness.sh`
  — the four `# spec:` header and inline comments, each of which currently states
  the logging-only contract or the not-the-blocking-variant reasoning as its
  directive (deltas 1, 2, 4 and 7).
- `gate-tests/subagent-stop-liveness.test.sh` and
  `scripts/gate-tests/subagent-stop-reader.test.sh` (delta 8).

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
      retired; nothing dangles. Specifically: no surface still asserts the
      logging-only boundary or the unclaimed authorization, and
      `check-spec-pointer` is green over the renamed section.
- [ ] **Ruling retired** — TRAJECTORY.md's 2026-08-24 authorization is spent by
      its own terms and is retired with the merge, not left standing.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
