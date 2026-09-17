# SPEC amendment: tier-discovery

Queue entry: `validate-tier-premise-mechanical-only`, a unit of `validate-red-holding`.

A root-level amendment, because it spans two components:

- lifecycle-kit owns the lead template's tiering rule (delta 1).
- The repo root owns this consumer's ruling config: the stage-session agent definition and the
  lead binding (deltas 2 and 3).

**What this delivers.** It delivers an operator ruling already recorded on the queue entry. The
cheaper tier for `validate` stays. What gets added is an **escalate-on-discovery** transition: a
session dispatched on a cheaper tier finds that its work needs a fix authored, and it moves to the
judgment tier by a named, cheap route. Before this, it had to improvise one. This amendment
re-judges neither the tier nor the ruling. The prose in `build-stage-tier-economics` that the entry
names is out of scope, and the entry says so.

**Why it is a kit rule and not a validate rule.** The premise that failed was "this dispatch's
work is mechanical". That premise is behind every downgraded dispatch the lead template sanctions,
and a stage-uniform `validate` tier is only one of them. An all-mechanical build batch can equally
turn out to need a repair designed. So the transition goes on the tiering rule, and the consumer's
roster carries the class a dispatched session reads.

**Why a re-dispatch and not a resume.** A lead's answer resumes the paused session in place, and
on the tier it was dispatched with. A resume cannot change the model, so moving tiers takes a
fresh same-stage session. That session reads the first one's resume journal, so the diagnosis is
not bought twice. The transition costs one lead turn and one session start. Grinding on the wrong
tier costs more, and nobody sees it happen.

## The seam

- **Kit mechanism:** the transition in lifecycle-kit/templates/lead.md. It names no model, no stage
  roster and no consumer.
- **Consumer config:** the escalation class in the agent definition the lead dispatches (the
  template's ruling-config slot), and the lead binding's validate bullet. No knob.
- **Private rule content:** the ruling's date, channel and instance stay in the queue entry and git
  history. The kit text states the rule undated.

## What changes

### (1) A downgraded dispatch carries an escalate-on-discovery transition

lifecycle-kit/templates/lead.md §Economics' **Tier each batch to its work class.** bullet is
rewritten to state the transition a cheaper-tier dispatch carries {design-bearing}.
**Not yet applied.**

- **Trigger.** A session dispatched on the cheaper tier finds a fix it would have to author: code,
  a contract, a fixture, or a diagnosis that crosses components. Running an oracle, editing a
  grammar-governed record, and filing a finding are not triggers.
- **The session's move.** It authors nothing toward the fix. It writes the diagnosis to its resume
  journal under a heading, commits whatever is complete and unrelated, and ends its turn on one
  escalation whose Question is a re-tier and whose Evidence names that heading.
- **The lead's move.** It rules alone, because assigning a batch's tier is already the lead's.
  It re-dispatches the rest of the work as a same-stage session on the judgment tier, pointing at
  that heading. It does not message the paused session, because a resume keeps the old tier.
- **Its relation to a recorded tier.** A consumer that bound a stage-uniform cheaper tier keeps
  it. One re-dispatch fits inside that binding and does not reverse it.
- **Honest limit.** Nothing detects a session that does not recognize its discovery. The
  transition makes the right move cheap and named, and a grinding session stays unseen.

**Replacement text, lifecycle-kit/templates/lead.md §Economics** (**Not yet applied**). After the
bullet's sentence "A batch carrying **any design-bearing** delta **stays on the judgment tier**.",
insert:

> A cheaper tier is priced on a premise the work can falsify, so every such dispatch carries an
> **escalate-on-discovery** transition. A session that finds it must author a fix, beyond running
> an oracle or editing a governed record, stops before authoring it. It journals the diagnosis and
> escalates a re-tier. You rule that alone. Re-dispatch the rest of the work as a same-stage
> session on the judgment tier, pointed at the journal. Do not resume the paused session, because
> a resume keeps its tier. Your ruling-config's escalation roster names the class, so a dispatched
> session can recognize it.

**Replacement text, lifecycle-kit/SPEC.md §templates/lead.md** (**Not yet applied**). The clause
"the per-batch work-class tiering (judgment being what the tier buys, downgrading a design-bearing
batch trades a large correctness risk for a small window saving, while a stage-uniform class is a
collapsed default and not a bound roster)" gains, before its closing parenthesis, "; a downgraded
dispatch carries an escalate-on-discovery transition to a same-stage re-dispatch on the judgment
tier, because a resume cannot change a session's model".

### (2) The stage-session roster names the discovery class

`.claude/agents/stage-session.md` §Ruling classes gains an **Escalate to the lead** bullet for
the class {mechanical}.
**Not yet applied.**

**Replacement text, `.claude/agents/stage-session.md`** (**Not yet applied**). A bullet appended to
the escalate list:

> - A **work-class discovery**: you were dispatched on a cheaper tier and find a fix you would
>   have to author. Journal the diagnosis, author none of the fix, and escalate a re-tier
>   (lifecycle-kit/templates/lead.md §Economics, *Tier each batch to its work class*).

### (3) The lead binding's validate bullet states its premise as a price

`.claude/commands/lead.md`'s `validate` bullet stops saying validate's batches *are* uniformly
mechanical. It says they are *priced* as mechanical, and points at the transition {mechanical}.
**Not yet applied.**

**Replacement text, `.claude/commands/lead.md`** (**Not yet applied**). The bullet's second
sentence (the em-dash after "default" continues it, rather than starting a third) becomes:

> validate is priced as *mechanical oracle-running* (run the battery, report), so the stage
> collapses to a single stage-uniform-mechanical default. That is the degenerate case of per-batch
> tiering, not a bound per-stage roster. A validate that finds a fix it must author takes the
> escalate-on-discovery transition (template §Economics, *Tier each batch*), and the tier stays.

## Producers and consumers

- **Re-tier escalation.** Produced by a stage session dispatched with a cheaper `model` override,
  which reachably exists in this repo: validate and align on every iteration, and all-mechanical
  build batches. Consumed by the lead at its answer transition, through the existing message
  channel. It carries no new field. The four headers are the escalation protocol's, and the
  Evidence header's journal heading is read by the re-dispatched session at its entry.
- **Journal heading.** Produced by the discovering session. Read by the re-dispatched session,
  which the lead points at it. The resume journal's path is per stage, so both sessions derive the
  same file (lifecycle-kit/templates/lead.md §Channel design).
- **Same-stage re-dispatch.** Uses the existing same-stage re-entry (lifecycle-kit/SPEC.md
  §The state machine). It stamps, and the cursor does not move. No gate changes.
- **Roster-holding readers of the minted class name.** `check-shim-restatement` reads the agent
  definition and the binding for copies of an owner's text. Delta 2's bullet and delta 3's
  sentence are pointers, and build runs the gate to confirm. No gate enumerates escalation classes
  (`grep -rn 'Ruling classes' native/src` finds none).
- **Points 5 and 6.** Neither applies: this narrows no corpus and obliges no enumerable member set.

## Existing sections updated

- `lifecycle-kit/templates/lead.md` §Economics — the tiering bullet (delta 1).
- `lifecycle-kit/SPEC.md` §templates/lead.md — the design summary's tiering clause (delta 1).
- `.claude/agents/stage-session.md` §Ruling classes — the escalate list (delta 2).
- `.claude/commands/lead.md` — the `validate` bullet (delta 3).
- `docs/lifecycle-kit/SPEC.md` — the generated mirror, regenerated with
  `--emit docs-mirror --write` (delta 1).

The roster comes from `grep -rn 'Tier each batch\|stage-uniform' --include=*.md` over the tracked
tree at `66682ce6`. Build re-derives it.

## Retired spellings

- None — no delta retires a spelling. Delta 3 rewrites one binding sentence whose words appear
  nowhere else.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check
      holds for the re-tier escalation and the roster class.
- [ ] **Instruction surfaces: instruction only** — the agent definition and binding point at the
      template, and the grounds stay here until merged into lifecycle-kit/SPEC.md.
- [ ] **Merged with no information lost** — delta 1 re-phrases the tiering bullet rather than
      appending a paragraph.
- [ ] **Amendment deleted** — this file is removed on merge.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` is green.
- [ ] **Gaps filed** — any gap found at build goes to the gap inbox.
