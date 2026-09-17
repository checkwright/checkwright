# SPEC amendment: wipe-and-drain

**Four lifecycle-kit contracts that each report a state they cannot see.** The
boundary wipe reports success while a nested `.gitkeep` keeps a whole scratch
tree alive. An amendment's Definition of Done can park an entry's queue move at a
stage whose entry refuses that entry. The close-surface roster prints an absent
file the same way it prints an empty one. And close's gap drain runs before the
step that reads the capture log holding the facts the drain needs. They share
one kit and one reader, the closing and boundary sessions, so they are designed
as one pass.

**What stays as it is.** The wipe stays boundary-only, unconditional and never an
abort. `.gitkeep` and the lead journal stay kit invariants that
`LIFECYCLE_KIT_BOUNDARY_PRESERVE` cannot unset. The roster's first four columns,
its sort and its three `check-close-surfaces` assertions are unchanged. Close's
step numbering is unchanged. No knob is added.

## What changes

### (1) The boundary wipe spares names at the scratch root only, and a spared directory whole

The wipe's spare test matches `.gitkeep`, `LIFECYCLE_KIT_LEAD_JOURNAL_FILE` and each
`LIFECYCLE_KIT_BOUNDARY_PRESERVE` entry against the scratch dir's immediate children
only, and a spared child that is a directory keeps its whole subtree {design-bearing}.
**Not yet applied.**

- **Anchored.** Below the scratch root nothing is spared. A nested `.gitkeep`, or a
  nested file whose basename happens to equal a keep-list entry, is deleted like
  any other member. That removes the defect's cause: today one nested `.gitkeep`
  keeps its parent, the parent's removal fails as non-empty, and every ancestor up
  to the root survives with it.
- **A spared directory is not descended.** Today the keep-list filters the delete
  and never the descent, so a preserved directory is emptied and kept. Under
  anchoring that rule would delete the contents of a directory a consumer named
  in order to keep it, so the descent stops at a spared child.
- **The knob's grammar stays basenames, and now says root children.** Each
  `LIFECYCLE_KIT_BOUNDARY_PRESERVE` entry and `LIFECYCLE_KIT_LEAD_JOURNAL_FILE`
  names one immediate child of the scratch dir. Every known value already does:
  this repo's `session-role`, context-kit's default session-role marker
  `${GATE_SDK_TMP_DIR:-.tmp}/session-role`, and the lead journal default
  `lead-journal.md`. An entry containing `/` spares nothing, as it spares nothing
  today (a basename never contains `/`), and the boundary report now names it:
  `  note: boundary-preserve entry '<entry>' is not a scratch-root name and spared nothing`.
  A note, never a refusal: a config typo must not block the iteration boundary.
- **Why root-relative basenames and not paths.** A nested path entry would add a
  grammar (anchoring, `..`, trailing slash) for a need no reader has. The attested
  keep-list members are all root children. A consumer that tracks nested
  scaffolding names the root directory holding it, which now keeps it whole.

**The `.gitkeep` invariant's honest limit moves with it.** The invariant exists so the
kit never deletes a file the consumer tracks. After anchoring, a consumer that
tracks a `.gitkeep` below the scratch root loses it at the boundary unless it names
that root child on the keep-list. That is the declared route, and the git-aware
"spare any tracked file" rule stays refused on its recorded grounds.

### (2) The wipe reports what it removed, and separately what it could not

The `boundary-wiped` note names only members whose removal succeeded, and a
second non-blocking note names each member whose removal failed {mechanical}.
**Not yet applied.**

Today `wipe_walk` pushes a member onto the reported set before removing it and
discards the removal's error. So a directory that survived is still reported as
wiped. After this change a failed removal goes to its own set, printed as
`  note: boundary-wipe could not remove from <scratch>: <paths>`. The entry still
exits as it would have. The note is the visibility the SPEC's "noise, never an
abort" sentence lacked. After delta 1 an expected failure no longer exists: a
non-empty directory is either spared whole or fully descended. So any line here
is a real residue, such as a permission error or a concurrent writer.

### (3) A Definition of Done places an entry's queue move before the drain stage

The `spec` stage template gains a clause: a DoD item that moves an active queue
entry names a stage the drain assertion lets that entry reach {design-bearing}.
**Not yet applied.**

**The ruling is derived, not new.** The entry asked whether a DoD may ever name a
stage other than the drain stage. §check-stage-entry assertion B answers it:

- a drain-stage entry refuses any untagged top-level entry in the active sections;
- every drain successor's entry refuses any active entry, `[drain-exempt:]` or not.

So an entry's Done move or demotion can be performed only by a stage that
**precedes** the drain stage in `LIFECYCLE_KIT_STAGES`, which is where canon-kit's
merge step 4 already puts it (at task completion). The one exception is a
`[drain-exempt:]` entry, which may also move at the drain stage itself. A DoD
therefore not only may name a non-drain stage but must, for an untagged entry.
Naming the drain stage is as unexecutable as naming a successor. The queue entry's
first candidate, "names the configured drain stage", was wrong in exactly this
way, and the attested failure named a successor (`close`, behind `validate`).

**Replacement text, `lifecycle-kit/templates/stages/spec.md`**, a new paragraph
after "Writing the amendment *is* promoting the deferred entry":

> **A Definition-of-Done item that moves the entry names a stage that can move it.**
> An entry's Done move or demotion lands before the drain stage
> (`LIFECYCLE_KIT_DRAIN_STAGE`), or at it only for a `[drain-exempt:]` entry — never
> at a stage after it (lifecycle-kit/SPEC.md §check-stage-entry, assertion B). Name
> that stage by its relation to the drain stage, never by a literal stage name.

**Why a clause and not an assertion.** An assertion over the amendment glob would
need a DoD item to spell its stage in a grammar. canon-kit's amendment template
has none: a DoD item is prose, and a stage word in it may name a stage for any
reason ("close files the gap"). So the assertion either guesses from prose, which
gives false positives on every such mention, or adds a stage field to canon-kit's
template, which is a second kit's grammar minted to catch one authoring slip. The
mechanical backstop already exists and is cheap. The lead's pre-completion
`--enter-stage --simulate <next stage>` read (§templates/lead.md) refuses exactly
this case. The attested cost came from that read running after the push, not from
the read being missing.

### (4) The close-surface roster prints each row's on-disk state

`--emit close-surfaces` gains a fifth tab-separated column, `<state>`, one of
`absent`, `empty` or `non-empty` for a file row, and `absent` or `-` for a
`<file>#<section>` row {design-bearing}.
**Not yet applied.**

- **`absent`**: the row's file (a locator's part before `#`) does not exist under the
  computed base.
- **`empty`**: the file exists and holds no non-blank line past its header run. The
  header run is the one §bin/enter-stage.sh's truncate keeps: leading blank and `#`
  comment lines, stopping at the first data line or a `## ` heading. So a surface
  truncated to its `# contract:` header reads `empty`, which is what a drained
  surface is. The predicate moves out of `native/src/emit/enter_stage.rs` into the
  shared stage module, and both callers use it: two copies of one header rule would
  drift.
- **`non-empty`**: anything else.
- **A section row reads `-` unless its file is absent.** Emptiness of a section is
  its owner's read (the Lessons row is forced by the boundary refusal, the Deferred
  row is never empty), and resolving an anchor-form heading would give this arm a
  heading resolver it does not otherwise need.

**Three named states, not a byte size.** drift-kit/SPEC.md §The knowledge-friction
loop already rules the same discrimination for one log. It uses three named states
(absent, present and empty, present and non-empty) because a size is a number whose
only consumer is this question, and a header-only file has a non-zero size. The
column is appended rather than inserted, so the first four positions every reader
indexes stay put.

**The gate does not red on `absent`.** A capture-tier surface is gitignored and
legitimately absent in CI and in a fresh clone, so an absence is a closing
session's finding and never a tree violation. `check-close-surfaces` reads the
fifth field as nothing: its row split widens from four fields to five, so the owner
field stops carrying the state.

**Replacement text, `lifecycle-kit/templates/stages/close.md` step 4**, re-phrasing
the sentence "An `(undeclared)` row is a capture surface nobody declared — file the
missing declaration rather than reading past it.":

> An `(undeclared)` row is a capture surface nobody declared: file the missing
> declaration. A row's state is read too: `empty` is a clean read to state, and
> `absent` is a finding. Either nothing ever wrote the surface or its writer is gone,
> so confirm the writer or file the gap.

### (5) Close's gap drain reads the consumer's named capture surfaces before its first bullet

Step 2 of the `close` template opens with a read of the capture surfaces a new
named slot, `drain-inputs`, lists, before any bullet is dispositioned
{design-bearing}.
**Not yet applied.**

**The kit mechanism.** The read happens first, and its record goes in the drain's
commit message. **The consumer content** is which surfaces the read covers. Every
capture surface is not the answer: this repo's prompt-friction and subagent-liveness
logs are raw machine captures of tens of kilobytes that bear on no bullet's claim.
And lifecycle-kit cannot name drift-kit's log, since the kit depends on no other
kit. The reader is a session, not a gate, so the value is a template slot and not
a knob (§Layout and configuration states the same ground for the general-availability
criterion).

**Replacement text, `lifecycle-kit/templates/stages/close.md` step 2**, inserted as
the step's second sentence, after the disposition-set lead-in's first sentence:

> **Read the drain's inputs first.** Before the first bullet, run
> `bash gate-sdk/bin/run-gates.sh --emit close-surfaces` and read each surface the
> slot below names whose row is not `empty` or `absent`. A fact there bearing on a
> bullet's claim is that bullet's re-verification input. The surface's own
> disposition and reclaim stay at step 4. Name the surfaces read in the drain's
> commit message.
> *<drain-inputs: the capture surfaces holding facts a gap bullet's re-verification
> may need (a knowledge-friction capture, say) — never a raw machine log.>*

**This repo's binding, `.claude/commands/close.md`**, a new entry:

> **drain-inputs** — `.workflow/knowledge-friction.log`, read whole; its triage stays
> the roster row's (drift-kit/templates/close-knowledge.md).

**Why a first read and not a reorder.** The entry's other candidate moves the kfric
walk ahead of the drain. That reorders one kit's template step against another
kit's triage template. It also renumbers steps that the consumer binding cites by
number (the brevity pass is "step 11"), and it moves a roster sweep that includes
the drain's own `forced=` row ahead of that drain. The entry's objection to a
precondition, that a hurried reader skips it, is met two ways. The read is a
command the step runs, and its record goes in a commit message a reviewer reads.
Delta 4's state column makes the read cheap: an `empty` or `absent` surface costs
no read at all.

**The honest limit.** Nothing gates that the read ran. The commit-message line
makes a skip visible, not impossible. That is the posture this template already
takes for the drain's re-verification record.

### (6) All four entries complete

At merge, `boundary-wipe-preserve-basename-reach`,
`dod-parks-a-queue-transition-at-a-stage-that-cannot-perform-it`,
`close-surface-row-absent-reads-as-empty` and `gap-drain-precedes-its-own-kfric-read`
move to Done. None is a corpus increment, so none demotes {mechanical}.

## Producers and consumers

**The anchored spare set (delta 1).**
- *Producer:* `--enter-stage <first stage>`'s boundary block, reached on every
  iteration boundary through the scope template's stamp step. That is live in this
  repo.
- *Consumers:*
  - the scratch dir itself;
  - the session-role marker's reader, context-kit's session-context hook. The
    marker's default is a root child, so it is unaffected;
  - the lead journal's readers, `--open-lead-journal` and the undisposed advisory.
    These read `<scratch>/<file>`, a root child, and are unaffected;
  - delegation-kit's journal-retention paragraph and evidence-kit's lock-reclaim
    layer. Both rely on the wipe *removing* scratch, which anchoring only widens.
- *Probe:* `git grep -n BOUNDARY_PRESERVE` and `git grep -n -i wipe` over the
  tracked tree, excluding `docs/`. No reader relies on a spare below the root.
- *Red conditions (point 5: delta 1 narrows the spared corpus):*
  - `lifecycle-kit/gate-tests/boundary-scratch-wipe.test.sh` `keep-nested` reds when
    `mixed-sub/keep-me` is deleted, which is now correct behavior. The assertion
    inverts to require its deletion.
  - The unit test
    `the_wipe_keeps_the_invariant_and_every_preserved_basename_at_any_depth` asserts
    the same survival and inverts the same way.
  - Both gain a nested `.gitkeep` case (deleted, and its ancestors with it) and a
    spared root directory case (kept whole).
  - The both-substrates comparison paragraph in §Testing names "a nested preserved
    basename" as a compared case. That describes a comparison already bought, so it
    is re-phrased to past tense, not re-run.

**The non-root preserve note and the removal-failure note (deltas 1 and 2).**
- *Producer:* the same boundary block, after the wipe.
- *Consumer:* the entering first-stage session reading the entry's stdout. No gate
  parses either note, and each field (the entry, the scratch path, the paths) is read
  by that session to act on.

**The DoD clause (delta 3).**
- *Producer:* the spec template, loaded by every authoring-stage session.
- *Consumers:* the authoring session writing a DoD, and the lead reviewing it.
  `check-shim-restatement` reads the template corpus, and the clause names the
  knob and cites the section rather than restating assertion B's grounds.

**The state column (delta 4).**
- *Producer:* `native/src/emit/close_surfaces.rs` `derive`, via `--emit close-surfaces`.
- *Consumers:*
  - close's step 4 (delta 4's replacement text) and step 2's first read (delta 5);
  - `check-close-surfaces`, in process. It ignores the field, and its split widens to
    five, since a four-way split would fold the state into `owner` and corrupt the
    owner in every error line.
- *Probe for other row parsers:* `git grep -n 'close_surfaces::'` over `native/src`,
  plus `git grep -n 'close-surfaces'` over the tracked tree. There are no other
  parsers.
- *Every field has a reader:* `absent` and `empty` are read by step 4 and step 2, and
  `-` tells a section row's reader to take the owner's read.
- *Satisfying value per current row (point 6, the roster is enumerable):* at this
  stage's run, `.workflow/wait-primitive-evidence.txt` and
  `.workflow/wakeup-attempts.log` are `absent`; `.workflow/knowledge-friction.log`
  (0 bytes) is `empty`; `.workflow/gap-inbox.md` reads by its content at the time
  (header plus one bullet today, so `non-empty`); `.workflow/preflight-valve.txt`
  (header only) is `empty`; the two `TASK-QUEUE.md#` rows are `-`; every other file
  row is `non-empty`.

**The drain's first read and the `drain-inputs` slot (delta 5).**
- *Producer:* the close template, and this repo's binding supplies the slot.
- *Consumers:*
  - the closing session;
  - `check-skill-binding`, which holds shim-to-template slot parity. It reds on the
    close shim until the binding entry lands, so the template edit and the binding
    ride one commit;
  - the drain commit message's reviewer.
- *Roster-holding readers of the minted slot name (point 2):* `check-skill-binding`
  (the shim) is the only one. Probe: `git grep -n 'release-policy'`, the sibling
  slot, over the tracked tree, which finds the template, its one shim, the SPEC and
  the generated mirror.

## Existing sections updated

- `lifecycle-kit/SPEC.md` §bin/enter-stage.sh, the wipe paragraph ("deleting every
  member whose basename is neither `.gitkeep`, nor … at any depth the wipe reaches"
  and "A delete that fails because a preserved basename sits inside an
  otherwise-doomed subdirectory is noise, never an abort") re-phrased to root
  anchoring, whole-directory sparing and the two notes (deltas 1 and 2).
- `lifecycle-kit/SPEC.md` §Layout and configuration, the
  `LIFECYCLE_KIT_BOUNDARY_PRESERVE` and `LIFECYCLE_KIT_LEAD_JOURNAL_FILE` bullets say
  scratch-root names (delta 1).
- `lifecycle-kit/SPEC.md` §Testing, the both-substrates comparison's "nested
  preserved basename" case read as history (delta 1).
- `native/src/emit/enter_stage.rs`: `wipe`/`wipe_walk`, the report block, the
  `// spec:` comments on both, and the unit test (deltas 1 and 2); the header
  predicate moves to the shared stage module (delta 4).
- `lifecycle-kit/gate-tests/boundary-scratch-wipe.test.sh`: the seed, the
  `keep-nested` assertion, new nested-`.gitkeep`, spared-directory and non-root-entry
  cases, and the header and clean-line comments (deltas 1 and 2).
- `lifecycle-kit/templates/stages/spec.md`: the DoD clause (delta 3).
- `lifecycle-kit/SPEC.md` §templates/stages/, the `spec.md` paragraph gains the
  DoD clause's ground: assertion B and the refused DoD assertion (delta 3).
- `lifecycle-kit/SPEC.md` §The close-surfaces emit arm: the row grammar gains
  `<state>` and its three-plus-one values (delta 4).
- `lifecycle-kit/SPEC.md` §check-close-surfaces: one clause saying the gate ignores
  the state and never reds on `absent` (delta 4).
- `native/src/emit/close_surfaces.rs` (the column, with a unit test per state) and
  `native/src/gates/close_surfaces.rs` (the five-way split) (delta 4).
- `native/src/stages.rs`: the shared header-run predicate (delta 4).
- `lifecycle-kit/templates/stages/close.md` step 4 (delta 4) and step 2 with the
  `drain-inputs` slot (delta 5).
- `lifecycle-kit/SPEC.md` §templates/stages/: a sentence on the close template's
  drain-inputs read and why it is a first read and not a reorder (delta 5).
- `.claude/commands/close.md`: the `drain-inputs` binding (delta 5).
- `TASK-QUEUE.md`: the four entries move to Done at merge (delta 6).
- <!-- update-target-exempt: generated projections, each rostered with its freshness gate and regen command in docs/site-architecture.md §Generated projections and their freshness gates --> The `docs/` mirror of `lifecycle-kit/SPEC.md`, `docs/enforcement.md`, `docs/footprint.md`, and the generated pre-commit hook.

## Retired spellings

- `the_wipe_keeps_the_invariant_and_every_preserved_basename_at_any_depth` — the
  unit test's name asserts the reach delta 1 removes; it is renamed to the anchored
  behavior (delta 1).
- `keep-nested` — the boundary harness's assertion label for the same reach (delta 1).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for the anchored spare set, both notes, the DoD
      clause, the state column and the drain's first read.
- [ ] **Instruction surfaces: instruction only** — the spec, close and binding
      replacement text carries no grounds; deltas 3, 4 and 5 place them, and they
      merge into §templates/stages/ and §The close-surfaces emit arm.
- [ ] **Merged with no information lost** — the wipe paragraph and the knob bullets
      are re-phrased, not appended to; the refused alternatives (path entries, the
      DoD assertion, a byte size, the reorder) survive in the merged prose.
- [ ] **Attested reproduction** — `mkdir -p .tmp/x/y && touch .tmp/x/y/.gitkeep`, then a
      sandboxed boundary entry, leaves no `x/`.
- [ ] **Queue moves placed before the drain stage** — the four Done moves land in
      the session that merges this file, before the drain stage is entered.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — `check-amendment-retired-spelling` is green on both
      declared spellings.
- [ ] **Gaps filed** — cross-component gaps found during the work go to the gap
      inbox.
