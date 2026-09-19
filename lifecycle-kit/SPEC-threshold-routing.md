# SPEC amendment: threshold-routing

`threshold-recurrence-routing-residency` carries one clause verbatim because no
permanent surface does: *"a third threshold recurrence routes to the operator, not
to a third decline; two is where lead discretion ends."* An earlier scope already
ruled the clause general rather than specific to its entry. That ruling was a lead
routing call under an operator ruling that gave the home decision to scope. Scope
also named `templates/stages/scope.md` as the live candidate home. This amendment
lands it there, and the entry retires at the merge.

**What the clause says, read off its origin.** It was written when a threshold entry
was declined for the second time at scope on the lead's ruling. The count is
therefore of **declines of a threshold-proposed entry**, not of `recurrence:` dates.
The recurrence threshold decides when an entry is proposed. This clause decides who
may turn the proposal down: the lead twice, and after that only the operator.
`git show d45a773a -- TASK-QUEUE.md` carries the originating text beside "Declined a
second time 2026-08-16 at scope on the lead's ruling".

**Where it completes the contract.** scope.md's threshold paragraph ends "it puts
the unit in front of the authority this stage already escalates to", and stops
before saying when that authority changes. The clause is that missing sentence. It
needs a count, and the decline count is recorded nowhere today. So the declining
session records each decline on the entry, and scope's escalation says when a
threshold entry is already at two. Scope may write the queue, so this adds no
writer.

**Its reader is the lead**, which routes scope's unit-set escalation. So lead.md's
routing sentence gains the carve-out. It sits beside the operator-class carve-out
§The escalation protocol already makes for recorded rulings.

**Refused:** a `declined:` body declaration with a grammar, so that a scan could
count declines. Nothing reads such a declaration except the one scope session that
writes the escalation, and a declaration grammar is queue-kit's to mint for a reader
that needs it. A decline line in the entry's prose is read by the same session that
reads the entry's grounds.

**Probed at authoring, 2026-09-19.** `grep -rn -i "third threshold\|lead discretion"
lifecycle-kit .claude CLAUDE.md`, run before this file existed, returned nothing:
the clause lived only in the queue entry. `grep -n "authority this stage already" lifecycle-kit/templates/stages/scope.md`
returns line 104.

## What changes

### (1) scope's threshold paragraph gains the clause {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/stages/scope.md`, the paragraph's
closing sentences, "**The collision is decided, not resolved in the theme's
favour**: the rule does not promote, it puts the unit in front of the authority
this stage already escalates to.", become:

> **The collision is decided, not resolved in the theme's favour**: the rule does
> not promote, it puts the unit in front of the authority this stage already
> escalates to. A decline of such an entry is written on the entry, with its date
> and who ruled it. An entry already declined twice is escalated as
> operator-routed: a third threshold recurrence routes to the operator, not to a
> third decline, because two is where lead discretion ends.

### (2) The lead routes an operator-routed threshold entry {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/lead.md` §The escalation protocol,
the paragraph opening "**One class the lead never rules, under either posture.**"
gains a closing sentence:

> Nor does it decline a threshold-proposed entry that scope's escalation marks
> operator-routed. That entry has been declined twice, and it is relayed.

### (3) §templates/stages/ carries the ground {design-bearing}

**Not yet applied.** In `lifecycle-kit/SPEC.md` §templates/stages/, the paragraph
"The `scope` template's recurrence override **decides** the collision …" is
re-phrased to add the ground. Recurrence dates decide when an entry is proposed,
and declines decide who may refuse it. Lead discretion is bounded at two so that a
counted recurrence cannot be deferred forever by the party it was escalated to.
Declines are prose on the entry because the escalating scope session is their only
reader. Also record the one breach the carrier entry names: a lead ruled a third
decline on 2026-09-04, while the clause lived only in a queue entry.

**Shares this anchor with `SPEC-deferred-drain.md` delta 4**, which grounds the
same paragraph with the inflow-drain rule (found at align, 2026-09-19). "Re-phrase"
here does not touch the paragraph's own opening sentence — it appends this ground
after it, and after deferred-drain's bullet block if that has already landed; if
this lands first, deferred-drain's session appends after this addition in turn.
Both grounds stand side by side — neither restates or supersedes the other.

### (4) The generated mirror {mechanical}

`docs/lifecycle-kit/SPEC.md` is regenerated after delta 3.

## Producers and consumers

- **The decline record** (delta 1). Producer: the scope session that lands a
  decline of a threshold-proposed entry. It writes on the entry in the commit
  recording the unit set, and states who ruled, per §The stamp protocol's
  class-and-date rule. Consumer: a later scope session composing its escalation,
  which counts the entry's decline lines.
- **The operator-routed mark** (deltas 1-2). Producer: scope's unit-set escalation,
  on the message channel. Consumer: the lead at its routing transition. No gate,
  because the message channel is transport, the same limit scope.md's composition
  line already states.
- **Point 6.** The corpus is the deferred entries currently carrying two recorded
  declines of a threshold proposal. Probe: `grep -n -i "declined a second time"
  TASK-QUEUE.md`, which returned no entry at authoring, so the corpus is empty and
  no entry needs editing. Build re-runs the probe and records the result in the
  merge commit.

## Existing sections updated

Roster probe: `git grep -n "authority this stage already escalates to\|One class the
lead never rules\|recurrence override" -- lifecycle-kit`.

- `lifecycle-kit/templates/stages/scope.md` threshold paragraph (delta 1)
- `lifecycle-kit/templates/lead.md` §The escalation protocol (delta 2)
- `lifecycle-kit/SPEC.md` §templates/stages/ (delta 3)
- `docs/lifecycle-kit/SPEC.md` (delta 4)

## Retired spellings

- None — nothing is renamed or removed. The clause moves from a queue entry into a
  template.

## Definition of Done

- [ ] **Causal completeness** — every point holds for the decline record and the
      operator-routed mark.
- [ ] **Instruction surfaces: instruction only** — scope.md and lead.md carry the
      rule, and the SPEC carries its grounds.
- [ ] **Merged with no information lost** — delta 3 re-phrases its paragraph.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component, discharged at the iteration.
- [ ] **Done move** — the paired entry moves to Done in the merge commit, before the
      drain stage. Its `not-icebox-eligible:` declaration goes with the body, and the
      clause it protected now lives in scope.md.
- [ ] **Gaps filed** — any cross-component gap found during the work filed.
