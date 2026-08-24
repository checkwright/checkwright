# SPEC amendment: stage-journal

Closes `stage-journal-contract-unoracled`. A granted resume journal that is never
written is indistinguishable from a session that had nothing to say, so the
durable narration channel fails silently.

**The fork the entry names is ruled the first way: the stage journal becomes a
stage contract with an oracle.** The second option — leaving it a dispatch-time
request — is what failed, and the entry's own framing of the cost decides it: the
loss landed on precisely the stage the journal exists for, the one interrupted
mid-work.

**The enabling move is not the assertion, it is the derivation.** A path granted
ad hoc in a prompt is unreachable by any oracle, because nothing on disk records
what was granted. Delta 1 is therefore the load-bearing one and delta 2 is only
possible after it.

## What changes

### (1) The stage journal's path is derived from the stage, not granted per dispatch

The journal path becomes a **computable function of the stage name** rather than
a string a dispatcher invents, so any reader can name the file a given stage owes
without having seen the dispatch. **{design-bearing}**

New knob **`LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN`**, whose value carries a
`<stage>` placeholder and whose default is
`${GATE_SDK_TMP_DIR:-.tmp}/<stage>-journal.md` — the shape this repo's sessions
have converged on unaided, which is the argument for making it the default rather
than inventing one. It is derivation-first applied to a fact the tree was already
maintaining by hand in every dispatch prompt.

**Two things the derivation changes, and the second is the point.** The
dispatcher stops choosing a name, so two sessions cannot disagree about where one
stage's journal lives; and a **gate or a stage entry can compute the path**,
which is what the entry says is missing.

**The dispatcher still grants the path, and the grant is now a restatement of
the derivation rather than its source.** delegation-kit's contract requires the
supervisor to grant the path **absolute** in the prompt, and that clause stands
unchanged — an agent must be told, because an agent cannot read a knob it has no
reason to look for. What changes is that the grant is now checkable against the
derivation instead of being the only record of it.

### (2) A stage entry asserts its predecessor's journal exists and is non-empty

`bin/enter-stage.sh` refuses entry to a stage whose in-iteration predecessor
left no journal at the derived path, under a new
`LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE` knob defaulting to `0`.
**{design-bearing}**

The refusal takes the same contract as the boundary-precondition family it joins:
**exit 1, the expected path printed, nothing written**, and `--simulate` relays
the would-be refusal rather than taking it.

**The three narrowings, each argued rather than inherited.**

- **Existence and non-emptiness, never the `DONE` marker.** §Resume journal
  rules that on the ordinary completion path the supervisor consumed the agent's
  return and "that return plus its post-commit verification *is* the recovery
  contract, so the marker is redundant there" — the marker carries its signal only
  in a **cold read**, where no return was ever consumed. A stage entry is not a
  cold read: the entering session exists because the previous stage returned. So
  asserting the marker would mint an obligation the owning section already rules
  redundant at exactly this transition, and the entry's phrase "exists and
  terminates" is honoured on the half that is decidable here.
- **The in-iteration predecessor only.** The boundary scratch wipe deletes the
  scratch dir at the first stage's entry, and `LIFECYCLE_KIT_BOUNDARY_PRESERVE`
  deliberately does not keep journals — §Resume journal names that wipe as the
  journal's intended reclaim path. So the first stage of an iteration has no
  predecessor journal by construction and is never asserted against. This is a
  consequence of an existing ruling, not a carve-out.
- **Stages only.** A supervising session's own journal is not a stage journal and
  is not asserted. It has no stamp, so the cursor cannot name it, and inventing a
  second roster to reach it would be a surface this unit did not scope.

**Default `0`.** An unconfigured consumer sees no change, which is what keeps the
delta additive for every vendoring tree.

**RULED 2026-08-24 by the iteration lead, and recorded here because the acting
session is build and build is not imminent: this repo's knob stays `0` for this
iteration, and the switch is thrown at the NEXT SCOPE ENTRY.** Build lands the
kit mechanism and leaves `scripts/lifecycle-config.sh` unset; it does not set
`1`. The ruling is lead-class rather than operator-class because it is
sequencing, and it is written to this surface rather than left in a message
because a message thread is transport and never a store — build reads the ruling
here and needs to ask no one.

The ground, and it is the reason the obvious "make it live where it was filed"
is wrong: `REQUIRE=1` asserts against the **predecessor** stage, so throwing the
switch mid-iteration lands it underneath validate and close — both dispatched
before the rule existed — and puts the first enforced entry *inside* a stage
rather than at a boundary. At a boundary a refusal costs a stage re-entry; inside
an iteration, against a session that has already ended and cannot be asked to fix
anything, it costs a wedge. Delta 3's escape exists precisely so that wedge is
recoverable, and a rule whose first live firing needs its own escape has been
scheduled badly rather than designed badly.

### (3) The refusal's escape is the honest record, and the honest limit is stated with it

The refusal's help text names one way forward — **write the missing journal
yourself, stating that the predecessor left none** — and the section records
plainly that this makes the assertion **evadable**. **{design-bearing}**

Both halves matter and neither is decoration.

An oracle over an artifact the asserted-against session can itself create is
bypassable by definition; a session can `touch` the file. Claiming otherwise
would be the stronger claim the evidence does not carry, and this project's
posture is to state a limit rather than let a reader infer a guarantee. **What
the mechanism actually buys is that the absence becomes deliberate and written
instead of silent and unnoticed** — which is precisely the defect: the entry's
finding is not that a session refused to journal, it is that *nobody knew* one
had not. A session that writes "the previous stage left no journal" has produced
the recovery record the channel exists for, and has done so at the one moment
someone is looking.

This also averts the failure shape the deadlock class is named for: a refusal
with no reachable escape would wedge the lifecycle behind a session that has
already ended and cannot be asked to fix anything.

### (4) The sandbox caveat is narrowed to what is established

delegation-kit's §Resume journal caveat — that a background agent may be unable
to write the granted path and falls back silently — is narrowed from a property
of isolation to an **unexplained** single observation. **{design-bearing}**

**Probed at this spec stage.** A worktree-isolated agent, dispatched with an
absolute path in the main checkout, **wrote it successfully** while running from
inside its own worktree. So isolation does not block the write, and the caveat's
own prescribed remedy — grant the path absolutely in the main checkout — works
under the strongest isolation this tree dispatches.

That matters for delta 2 rather than being trivia: if the write were incapable
under isolation, an entry-time assertion would be refusing sessions for something
they could not do, and delta 3's escape would be the normal path rather than the
exception. Capability being established, a missing journal is a session that did
**not** write rather than one that **could not** — which is what makes the
assertion fair to assert.

The caveat is narrowed, not deleted: one observation of capability does not
falsify an observation of failure, and the failure that filed this entry
remains unexplained. The narrowed text says what was seen on each side and stops
generalising from the failing one.

## Producers and consumers

**The derived path (delta 1)** — a computed value, not stored state.

- *Producer:* `lifecycle-kit/lib/stages.sh` resolves and validates the pattern
  knob; `bin/enter-stage.sh` expands `<stage>` against the stage it is entering
  and against the predecessor it asserts. **Enabling config actually set:** the
  pattern has a **non-empty default**, so the derivation is live in every
  consumer the moment it lands, and this repo's existing journals already sit at
  the default's paths — the derivation names files that exist rather than
  proposing a migration.
- *Consumer:* three, at three transitions. (a) `bin/enter-stage.sh` at stage
  entry, to compute the predecessor's path for delta 2. (b) The **dispatching
  supervisor**, at dispatch time, to grant the path it must still spell out in
  the prompt. (c) The **dispatched stage session**, at its own entry, to know
  where to write without being told twice.
- *Named reader for every field:* the pattern's one field is the `<stage>`
  placeholder, read by the expansion in (a) at stage entry. No other field is
  carried.

**The assertion (delta 2).**

- *Producer:* `bin/enter-stage.sh`, in the pre-flight run before anything is
  written, alongside the boundary-precondition family it copies its refusal
  contract from. Its inputs are the **stage cursor** (the state file's last
  stamp, already read there) and the derived path — both existing producers with
  existing enabling paths.
- *Consumer:* the **entering session**, through the refusal on stderr and the
  non-zero exit; and the **committing session** downstream, since a refused
  entry writes no stamp and therefore cannot be committed.
- *Named reader for every field:* the refusal carries the expected path (read by
  the session to know what to create) and the predecessor stage name (read to
  know whose journal is missing). No third field is carried.

**The knobs (deltas 1 and 2).**

- *Producer:* `lifecycle-kit/lib/stages.sh`'s resolution-and-validation block,
  which refuses fail-closed on a pattern carrying no `<stage>` placeholder — a
  pattern that ignores the stage would name one file for every stage and assert
  the wrong session's journal — and on a `REQUIRE` value that is not `0` or `1`,
  the same arm shape `LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK` already takes.
- *Consumer:* `bin/enter-stage.sh` at stage entry.

**Deltas 3 and 4 introduce no state, event or interface**; both are prose on
surfaces that already exist, consumed by their readers — the refused session at
delta 3, a dispatching supervisor at delta 4.

**Narrowing check (canon-kit/SPEC.md §The causal-completeness check, point 5).**
Delta 4 narrows an asserted claim's scope and delta 2 adds a refusal, so each
reader's **red condition** is enumerated rather than its subject:

- `lifecycle-kit/gate-tests/enter-stage.test.sh` — reds when an entry's exit
  code, written state or emitted text differs from the asserted one. **Not
  monotone** (exact exits and exact text), and it is the reader this amendment
  must move: every existing case entering a non-first stage now runs with
  `REQUIRE=0` by default and is unaffected, but new cases are owed for
  `REQUIRE=1` with the journal present, absent, and present-but-empty.
- `check-stage-evidence` — reds on stamp grammar or name-axis disagreement.
  **Monotone and cleared by inspection**: a refused entry writes no stamp, so
  the file this gate reads is untouched on the refusing path and unchanged on
  the passing one.
- `check-stage-entry` — reds on its three assertions. **Monotone and cleared by
  inspection**: no delta touches a stamp, a queue section or the amendment
  corpus.
- `check-knob-citation` — reds on a knob defined and never cited in its owning
  SPEC's roster. **Not monotone**: its red condition is a zero citation count,
  so both new knobs red it until §Layout and configuration carries them.
- `check-knob-default-coupling` — reds when a code default and the SPEC's stated
  default disagree. **Not monotone**: both defaults must be stated, and the
  pattern's default is the one to watch, since it embeds `GATE_SDK_TMP_DIR`'s
  own default and must not restate it as a literal.
- `check-workflow-tiering` — reds on a `.workflow/` member that is neither
  tracked nor ignored. **Monotone and cleared by inspection**: journals live
  under the scratch dir, not `.workflow/`, and no delta moves either.
- `check-surface-duplication` — reds when a canonical definition appears on a
  second surface. **Not monotone**: delta 1 puts a path convention in
  lifecycle-kit while delegation-kit's §Resume journal owns the journal
  contract, so the two must be written as owner-and-pointer and diffed, not
  assumed disjoint.

## Existing sections updated

- `lifecycle-kit/SPEC.md` §bin/enter-stage.sh — the new pre-flight assertion,
  its refusal contract, its two narrowings and the `--simulate` relay, placed
  with the boundary-precondition family whose contract it copies (deltas 2
  and 3).
- `lifecycle-kit/SPEC.md` §Layout and configuration — the knob roster gains
  `LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN` and
  `LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE` with their defaults and the reason the
  second defaults off (deltas 1 and 2).
- `lifecycle-kit/SPEC.md` §lib/stages.sh — the validator roster gains both
  fail-closed arms (deltas 1 and 2).
- `lifecycle-kit/SPEC.md` §The state machine — the stage contract gains the
  journal as an obligation of a stage rather than of a dispatch, which is the
  ruling this amendment takes (deltas 1 and 2).
- `delegation-kit/SPEC.md` §Resume journal — agent writes, scratch reset sweeps
  — the grant clause is restated as a restatement of a derivation lifecycle-kit
  owns, and the sandbox caveat is narrowed to the observation it rests on
  (deltas 1 and 4).
- `delegation-kit/templates/agent-execution.md` — the **Resume journal** bullet
  is the surface an agent loads; it points at the derived path instead of
  implying the name is the dispatcher's invention (delta 1).
- `lifecycle-kit/templates/lead.md` §Channel design — the supervisor reads the
  journal as its pull channel and mints the path it grants; the mint becomes a
  derivation it reads rather than a name it chooses (delta 1).
- `lifecycle-kit/templates/stages/` — every stage template's dispatch-facing
  text that names a journal path, so no template keeps a hand-spelled name the
  derivation now owns (delta 1).
<!-- update-target-exempt: a no-restatement re-read owned by no delta — the always-loaded tier must not acquire the convention, and claiming this bullet for a delta would assert exactly the write the content-tiering rule forbids -->
- `CLAUDE.md` §Housekeeping — the `.tmp/` bullet already names resume journals
  among the scratch dir's contents; re-read at merge to confirm it stays a
  pointer and gains no second copy of the path convention.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`), discharged at the iteration
      rather than at this commit, a sibling amendment being in flight for it.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. The specific strings to chase are hand-spelled
      journal paths in stage templates and in the lead template.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks.
- [ ] **Owner-and-pointer held** — lifecycle-kit owns the path derivation,
      delegation-kit owns the journal contract, and neither restates the other,
      per the `check-surface-duplication` reading above.
- [ ] **This iteration's own journals validate the default** — the derived paths
      must name the files this iteration actually wrote, checked rather than
      assumed.
