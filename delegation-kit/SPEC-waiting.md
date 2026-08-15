# SPEC amendment: waiting

**One causal chain, three queue entries, and the order is the design.** The
in-turn waiting rule names *the harness's waiting primitive* in the singular and
names nothing (`harness-wait-primitive-unnamed`); a session that has to pick one
unaided picks the process-liveness form, whose obvious spelling can never go
false (`waiter-predicate-self-match`); and a session that gives up on picking one
sleeps in the foreground instead (`poll-sleep-guard-steer`). The unnamed
primitive selects the shape the other two fail in, which is why naming it is the
head and not a wording fix.

**Operator-ruled 2026-08-15 at scope as one narrowed bundle**, with the other two
entries of the family deliberately not promoted:
`waiting-rule-carrier-reach` re-deferred on the corrected ground that its
founding reach gap is closed and reach is the wrong axis, and
`waiting-rule-fourth-firing-post-fix` declined against its own coupling clause as
not-yet-designable. Neither is reopened here and neither is re-litigated. What
this amendment owes them is one thing only: it must not quietly answer the
enforcement-**design** question the second entry holds, and it does not — it
enforces three *mechanically decidable* shapes and leaves *given that prose alone
does not hold, what does* exactly where that entry has it.

**What is deliberately left unsettled, stated first because it is the easiest
thing to accidentally close.** The harness's own tool documentation asserts that a
foreground `sleep` is blocked. This tree cannot corroborate that and cannot
refute its cause: probed at scope, the in-tree fact is that **no sleep rule exists
anywhere this repo controls** — not in `scripts/bash-guard.sh`, not in
`guard-kit/lib/`, not in `guard-kit/checks/`, not in `.claude/settings.json` — so
nothing in this repo enforces the never-poll rule's sleep half today. Why the
harness says otherwise is **unprobed as to cause**, is a claim about the harness
rather than about this tree, and is filed to the gap inbox. No delta below
asserts a resolution of it, and no delta cites an external block as coverage.
The design consequence is one-directional and safe: this amendment builds the
in-tree rule as though nothing outside the tree enforced it, which is correct
whichever way the open question resolves.

## What changes

### 1. The waiting primitive is named by the property it must have, and both harness forms are named against that property

**The doctrine bullet stops saying *the harness's waiting primitive* and starts
stating the reactivity requirement plus the two forms that do and do not meet it**
— design-bearing, because selecting a primitive is the delegation-kit contract
call the queue entry says it is **[design-bearing]**.

The requirement: **a wait must end when its condition goes true, not when a
duration expires.** Read against that, the harness offers two forms with opposite
reactivity, and the difference is exactly the wall-clock property the rule exists
to buy:

- **One completion → background a command that *exits* on the condition** (a
  `run_in_background` shell command wrapping `until <cond>; do sleep N; done`).
  It fires one notification at the moment the condition holds and then ends.
  **This is the sanctioned form for the in-turn condition wait**, and it is what
  the bullet will name.
- **A repeating signal → the harness's event-stream form**, armed with a command
  and a deadline. It emits one event per occurrence and **stays armed to its
  deadline even after the event fires**, so used for a single completion it
  converts a wait that should end in seconds into a wait that ends at the timeout.
  Named in the same breath, and named as the wrong tool for a completion,
  because a reader who is told only about the first form will reach for this one
  the next time the shape looks close.

**Naming both, rather than naming the sanctioned one alone.** A rule that names
one form teaches a spelling; a rule that names the property and sorts both forms
under it teaches the discriminator, and the discriminator is what survives a
harness that adds a third form.

### 2. The awaited artifact has a home, and the `Agent`/shell-child split is what makes the home reachable

**The bullet says in the same breath where the awaited artifact lives, and which
class of work has one at all** — design-bearing, because the second half is what
closes the attested failure rather than the first **[design-bearing]**.

The artifact goes in **repo-local `.tmp/` in the main checkout** — the identical
scoping the resume-journal rule already carries three bullets down (*never a
temporary worktree, which is deleted with it; never a system temp dir, which a
restart wipes*), widened from the journal to any artifact a session waits on. The
widening is stated as a widening so the two do not drift into two rules.

**And the split, which is the substantive half.** The two waitable things are not
the same kind of thing:

- **An `Agent` dispatch** is awaited by its **completion notification**. Its
  record is the harness's, not the session's, and the never-poll rule already
  governs it. A session must not go looking for that record on disk — which is
  exactly what the attested failure did.
- **A shell child** has no notification channel of its own, so it is awaited on an
  artifact — and that artifact is one *the session placed*, in `.tmp/`, precisely
  because the session controls where it goes.

**This is what the guard interaction was actually about, and it resolves without
touching the guard.** The attested sequence: a session reached for the correct
artifact wait, spelled it against the harness's own scratchpad path, and
`scripts/bash-guard.sh`'s `/tmp/claude-` block refused it and steered to `.tmp/`.
The guard was **right** and stays unchanged. What was missing is that the doctrine
never said where an awaited artifact belongs, so the refusal read as a refusal of
artifact-waiting itself and pushed the session onto the process-liveness form that
then broke. With the split above, a session waiting on an `Agent` never reaches
for a path at all, and a session waiting on a shell child has a path the guard
already sanctions. The failure had no third case.

### 3. Process liveness is ruled out as the sanctioned predicate, and the bracket trick is ruled out as the sanctioned repair

**The rule is *wait on the work's artifact*; where liveness genuinely is the
condition, match a recorded PID and never a pattern** — design-bearing, because it
rejects the repair a reader will propose **[design-bearing]**.

The defect, re-verified at scope and stronger than filed: `pgrep -f '<pattern>'`
matches the **waiter's own argv**, and the harness's wrapper argv matches too, so
`until ! pgrep -f '<script>'; do …; done` has a permanently-true condition and
never exits. It does not depend on how the waiter is spelled, and it reds nothing
— the work completes correctly and the only symptom is the foreground cap
absorbing an unbounded loop, which reads from outside as a fixed ten-minute wait.

**Three candidate repairs, and only one is sanctioned.**

- **The artifact wait** — sanctioned, and the default. It is the form delta 1
  names and delta 2 gives a home.
- **A recorded PID** (`kill -0 "$pid"` against a PID the session captured when it
  started the child) — sanctioned **where liveness is genuinely the condition**,
  which is the case of waiting on a producer the session did not start. A PID is
  an identity; a pattern is a guess about a process table that includes the guesser.
- **The bracket trick** (`pgrep -f '[r]un-smoke.sh'`) — **refused as the
  sanctioned form**, and refused in writing because it is the first thing a reader
  reaches for and it *works*. What disqualifies it is not correctness but its
  failure mode: it is a spelling that must be remembered, its omission is
  invisible at the call site, and its cost when omitted is silent and expensive.
  A rule whose correct form differs from its incorrect form by one character is a
  rule that will be got wrong, and the guard rule in delta 4 exists precisely
  because it will be.

### 4. A generic guard rule blocks a self-matching process-liveness predicate

**Mechanically decidable, because the pattern is a literal in the same argv the
matcher will scan** — design-bearing, because the predicate and its placement are
both new **[design-bearing]**.

*Fires when:* a segment leads with `pgrep` or `pkill`, carries `-f` (bare or
bundled in a short cluster), and its pattern operand is a literal that also occurs
elsewhere in the command's own text. *Blocks* with the corrective naming both
sanctioned forms from delta 3 — the artifact wait, or a recorded PID — and the
escape hatch every block message in this ruleset carries.

*Conservative by construction, in the ruleset's established directions:* an
expansion or a substitution in the pattern declines outright rather than guessing
(rule 6 already blocks those shapes); a `pgrep` without `-f` matches process
**names** and not argv, so it is untouched; a `pgrep -f` whose pattern occurs
nowhere else in the command is a genuine query and is untouched. Each biases
toward passing rather than toward a false block.

*Placed with the read-steer rules and ahead of both auto-allow rules*, on rule 8's
stated reasoning: `pgrep` is a plausible member of a widened `GUARD_KIT_RO_BINS`
— it is, after all, a read-only query — and a consumer who added it would
otherwise have the read-only-pipeline grant silently bless a waiter that can never
exit.

### 5. A generic guard rule blocks a bare foreground `sleep`, and the condition loop is what it must not block

**The whole rule is the discriminator; a blanket `sleep` block is wrong and is
refused** — design-bearing, because separating the two is genuinely new parsing in
this ruleset **[design-bearing]**.

The sanctioned wait *is* a condition loop, and `until <cond>; do sleep N; done` is
expressly legitimate — it is the form delta 1 names. A settle inside a smoke or a
probe is legitimate for the same reason. So the rule fires on exactly one shape:
a **bare foreground `sleep`** standing in for a wait, and never on a `sleep`
inside a loop body.

*The discriminator, and it is new territory:* no rule in this ruleset parses shell
keywords today. The rule takes the skeleton view, then reads the command for an
`until`/`while` … `do` … `done` wrapper and treats a `sleep` token inside that
wrapper as **inert** — structurally the carve-out `_guard_is_banner` already
performs for `echo`/`printf` segments in the batched-read rules, applied to a
keyword span instead of a segment. A `sleep` outside every such wrapper fires.
Where the wrapper cannot be resolved the rule **declines**, which is the same
direction every other conservative clause in this ruleset takes.

*Blocks* rather than advises, on rule 14's reasoning: no consumer allowlist is
presumed to grant a bare `sleep`, so the rule fires on a command that would be
decided out of band anyway and converts that decision into a durable steer at no
extra cost.

*The corrective names the property and both forms* — delta 1's discriminator, not
a single spelling. This is the queue entry's open question (*whether the steer
names the notification channel, the harness's monitor form, or both*) and the
answer is **both, sorted by the property**, for delta 1's own reason: naming one
form teaches a spelling and naming the discriminator teaches the rule.

*Placed with rule 4's sibling, ahead of both auto-allow rules,* for the same
reason: `sleep` is not on the default read-only roster, but the placement argument
is about what a consumer may add and the whole family of steer rules already sits
there.

### 6. Both rules land in guard-kit's generic ruleset, and the section's scope sentence widens by one clause to say why

**Neither rule names any project's toolchain, and only the generic lane has a
verification surface** — design-bearing, because it moves the seam by one clause
**[design-bearing]**.

§The generic ruleset scopes itself to *rules that encode **harness behavior**, not
any project's toolchain*, and §Consumer rules holds the other side: *guard-kit
ships no consumer rule and names none. What a project blocks or steers is its own
toolchain knowledge and stays in its copy.* Delta 5's rule is squarely harness
behavior. Delta 4's is not — a `-f` pattern match seeing the matcher's own argv is
a property of the **shell substrate every consumer of this kit runs on**, which is
neither harness behavior nor any project's toolchain and therefore falls in a gap
the sentence leaves. The sentence widens to *harness or shell-substrate behavior,
never any project's toolchain*, which admits delta 4 without admitting anything a
project owns.

**The decisive ground is the verification surface, not the taxonomy.** §Testing
states that every generic rule carries at least one firing and one non-firing case
in `guard-tests/cases.tsv` — the fixture-pair discipline, transplanted — and that
the decision table, never `scan-prompts.sh`, is the instrument for any change to
what the guard refuses, *because `guard_block` exits 2 before the fallthrough log
is written, so a blocked command never reaches the log.* A rule placed in a
consumer's copy gets neither: this repo's four existing project rules in
`scripts/bash-guard.sh` carry no test anywhere in the tree. Enforcement-first says
the fix and the check that catches it land in one unit, and the generic lane is
the only placement where they can. **That a consumer's own guard rules have no
verification lane at all is a real finding and a separate one** — it is filed as a
gap rather than solved here, since solving it would design a testing lane for
consumer copies inside a unit about waiting.

**The seam is read from the owner doc rather than from precedent, and the reading
is recorded once for both rules.** CLAUDE.md §The provenance seam is a *privacy*
boundary over **private rule content** — term lists, coupling vocabularies,
glossary bodies, product constant sets — on the ground that a kit literal carrying
a private vocabulary publishes it. A harness tool name is public, documented,
shared by every consumer of that harness, and publishes nothing; it is not private
rule content and the seam does not reach it. What a kit literal naming one
harness's forms *does* cost is portability, not privacy — and that cost is
recorded as an honest limit rather than pre-paid: delegation-kit's waiting bullet
already names `run_in_background` three times, so the kit is harness-shaped in its
examples today, and a second harness is what would force the names into a binding
slot. Building that slot now would be designing against no case. Filed as a
costed note in the section rather than a task, because the trigger is an event
outside this tree.

### 7. The renumbering is named with its cross-references, because nothing gates a stale rule number

**Two rules inserted mid-list shift every number above them, and the ruleset's
prose cites its own numbers throughout** — mechanical **[mechanical]**. §The
generic ruleset states that *order is load-bearing where noted*, so the two new
rules take the positions delta 4 and delta 5 argue for and the auto-allow and
later rules shift up by two. **No gate reads a rule number**, so the roster is
enumerated here rather than left to a grep at build time. The references that move:
inside `guard-kit/SPEC.md`, the mentions of rules 13 and 14 in rule 8's placement
clause, rule 9's producer clause, rule 13's own two carve-outs, rule 14's ordering
clause, rule 15's placement clause, rule 16's block-not-advise clause, the
`GUARD_KIT_RO_BINS` and `GUARD_KIT_SETTINGS` knob entries in §Layout and
configuration, and the *rules 8/13/14/16* and *rules 9, 10, 11, 13 and 16* rosters
in §The guard framework; inside `guard-kit/lib/guard.sh`, the five `# spec:` lines
naming rules 13 and 14 and the `guard_split_compound` header's *rules 8/13/14/16*.
Rule 15's own note — that DOCTRINE.md is cited by **name** and not by number
because *this ruleset carries its own rule 15* — is itself invalidated by the
shift and is the one that would be missed.

### 8. The restated copies in the always-loaded agent definitions move in lockstep

**The waiting bullet is one of the two rules delegation-kit sanctions restating as
a bare imperative** — mechanical **[mechanical]**. §Operative residency's
restatement sanction names *Background + notification, never poll* explicitly, the
template's own header says *when either rule changes here, propagate*, and the
copies live in this consumer's `.claude/agents/stage-session.md` and
`.claude/agents/audit-sweep.md`. Both carry the imperative half and both take
delta 1's naming and delta 2's `.tmp/` clause. The restatement stays an imperative
with an adjacent citation — it does not absorb delta 3's rejected-alternative
reasoning, which is amendment-tier and stays behind the pointer.

### 9. Nothing in this unit claims the sleep half was already enforced

**A one-line non-assertion, carried into the guard's own section so a later reader
does not close it by inference** — mechanical **[mechanical]**. The record: before
delta 5, no rule in this repo's control enforced the never-poll rule's sleep half,
and delta 5 is therefore the first. Whether any mechanism outside this tree does
is open, unprobed as to cause, and filed. The rule below it is built as though
none does — which is the only construction that is correct whichever way the open
question resolves, and the reason the question does not block this unit.

## Producers and consumers

**This amendment introduces two guard rules, one widened scope clause, one
widened artifact-location clause and one named discriminator. It introduces no
new state surface, no new knob, no new tag, no new file and no new gate.**

**Two generic guard rules.** *Producer:* `guard-kit/lib/guard.sh`'s
`guard_generic_rules`, invoked by every consumer's copy of
`templates/bash-guard.sh` at the position the template marks. Its enabling
configuration is the consumer's `PreToolUse` hook registration — live in this
repo at `.claude/settings.json`, matcher `Bash`, and live in every consumer that
installed the template, so the producer is reachable on the ordinary path and not
only in tests. *Consumers:* the harness, at every `Bash` tool call, reading exit 2
plus the stderr message as a block; and `bin/run-guard-tests.sh`, at every gate
run of the kit's own suite, reading each rule's firing and non-firing case out of
`guard-tests/cases.tsv`. **Both rules gain their pair in the same commit** —
§Testing's requirement, and the reason delta 6 places them here.

**Their two new fields have named readers.** The self-match rule's *pattern
literal recurrence* is read only by the rule itself, at the block decision — it
produces no record. The sleep rule's *loop-wrapper span* is likewise read only at
its own decision. Neither writes anything, which is the point: a guard rule that
recorded state would need a reader for that state, and neither has one. What both
*do* write is the block message, whose reader is the session that just made the
call — the one transition where a corrective can still be followed.

**One widened scope clause** (§The generic ruleset's opening sentence).
*Producer:* this amendment. *Consumer:* the next session deciding where a guard
rule belongs — a human or agent reader, never a gate. Stated because no gate can
decide *is this rule harness behavior or project toolchain*, and an unstated
placement rule is re-derived per rule.

**One widened artifact-location clause** (the waiting bullet). *Producer:* the
delegation-kit template, at every load of the agent-execution skill; and its
restated copies in the two agent definitions, at every dispatch of those types —
which is the reach half, and is why delta 8 is not optional. *Consumer:* the
dispatched session choosing how to wait, at the moment it has long-running work
and no result. The clause's truth is held by `scripts/bash-guard.sh`'s existing
`/tmp/claude-` block, which now steers **toward** a documented destination rather
than away from an undocumented one.

**Existing integration prose describing the prior flow is updated, not left to
drift** — see below. The one flow that genuinely changes is the dispatched
session's wait: it stops being *pick a primitive* and becomes *sort your case by
reactivity, then place the artifact where the guard already sanctions*.

## Existing sections updated

- **delegation-kit/templates/agent-execution.md**, the *Background +
  notification, never poll* bullet — deltas 1, 2 and 3. The
  `the harness's waiting primitive` phrase is replaced by the property plus both
  forms; the `.tmp/` home and the `Agent`/shell-child split land in the same
  breath; the sanctioned liveness predicate and the refused bracket trick land as
  the bullet's own rejected alternative. The resume-journal bullet's `.tmp/`
  scoping is cited as the widened rule's source, not restated.
- **delegation-kit/SPEC.md §The delegation model** — the waiting-primitive
  contract's owning section. It gains the reactivity discriminator as a named
  rule, the two-class split (an `Agent` awaited by notification, a shell child by
  an artifact), and the refusal of the pattern-matched liveness predicate with its
  grounds. This is where delta 3's rejected alternatives live permanently; the
  template carries the imperative.
- **delegation-kit/SPEC.md §Operative residency** — delta 8. Its restatement
  sanction already names this bullet; the section records that the propagate
  obligation fired here and which two carriers took it.
- **guard-kit/SPEC.md §The generic ruleset** — deltas 4, 5, 6 and 7: two new
  numbered rules with their firing conditions, conservative directions and
  placement grounds; the opening scope sentence widened by one clause; the
  renumbering of the auto-allow and later rules with every internal cross-reference
  moved.
- **guard-kit/SPEC.md §Consumer rules** — delta 6's other half. The placement
  contract is unchanged; what is added is the one sentence that makes it usable —
  that the generic lane carries the decision-table obligation and the consumer lane
  carries none, so placement is a verification decision and not only a seam one.
- **guard-kit/SPEC.md §The guard framework** — delta 7's rosters
  (`guard_split_compound`'s and the raw-read rosters) gain the two new rules and
  take the renumbering.
- **guard-kit/guard-tests/cases.tsv** — a firing and a non-firing case per new
  rule. The non-firing cases are the load-bearing ones: a `sleep` inside an
  `until … do … done` loop, and a `pgrep -f` whose pattern appears nowhere else.
- **`.claude/agents/stage-session.md` and `.claude/agents/audit-sweep.md`** —
  delta 8.
- **TASK-QUEUE.md** — all three entries move to `## Done` on merge, dropping
  their `[spec:]` tags; each entry's deliverable is discharged whole by this
  amendment rather than incremented, so none of the three takes the demotion
  branch.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named reader
      at a named transition.
- [ ] **The open question stays open** — nothing in the landed text asserts that
      any mechanism outside this tree blocks a foreground `sleep`, and no coverage
      claim rests on one.
- [ ] **The decision table is re-derived, not appended to** — §Testing's red
      condition is not monotone, so every existing case whose command carries a
      `sleep`, a `pgrep`, or a loop keyword has its expected column **re-derived**
      rather than assumed still correct, and `bash guard-kit/bin/run-guard-tests.sh`
      is green.
- [ ] **The non-firing cases are proved** — a `sleep` inside a condition loop and a
      non-self-matching `pgrep -f` both fall through, run rather than reasoned.
      A blanket `sleep` block is the failure mode this unit exists to avoid.
- [ ] **The renumbering is complete** — every cross-reference in delta 7's roster
      moved, including rule 15's own *this ruleset carries its own rule 15* note;
      no gate reads a rule number, so this is verified by reading the roster back.
- [ ] **The restatements propagated** — both agent definitions carry the updated
      imperative, and neither absorbs amendment-tier reasoning.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); each merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`). Discharged at the **iteration**,
      not at this commit, where sibling amendments are in flight.
- [ ] **Removals propagated** — grepped every surface for the retired
      `the harness's waiting primitive` phrasing; nothing dangles.
- [ ] **Gaps filed** — the absent verification lane for consumer-copy guard rules
      filed as debt rather than solved inside this unit.
