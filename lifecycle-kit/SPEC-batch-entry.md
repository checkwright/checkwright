# SPEC amendment: batch-entry

Two entries, one mechanism. `stage-journal-path-unsourced-mid-stage`: a batch
session dispatched into an already-entered stage writes a journal under a
discriminated filename, and the next stage's entry refuses on the canonical one.
`batch-split-stamp-ownership`: a batch session skips its own stage entry, so the
evidence file under-reports the stage's sessions. Both are a batch session that
takes something from its dispatch prompt that only its own `--enter-stage` run
should give it: the journal path in the first case, leave not to stamp in the
second.

**The fork the stamp entry asks for is already decided.** It asked whether to stamp
per batch or narrow the contract to per stage. §The state machine rules per batch
("each leaves its own stamp, so per-batch provenance rides the existing stamp
grammar"). `templates/lead.md` §Stamps are authoritative and
`templates/stages/build.md` ("Every session still stamps") carry the same rule.
Spec-over-precedent settles it. The owed half is what makes that instruction bind.

**The journal path has two sources today, and the dispatch one wins.** The stage
template's last step names "the journal the `--enter-stage` arm named at the stamp".
§Channel design has the lead "still spell it out in the dispatch prompt", and the
standing agent definition tells the dispatched session "The dispatch names the
journal path". Where the two disagree the dispatched session follows its prompt.
The prompt restatement is the channel both measured instances came through: seven
batch sessions across two iterations wrote `-batchN-` journals named by their
dispatches.

**Ruling: one source.** The entry tool's report is the only source of a stage
journal's path. The lead names none, and a dispatched session writes to the path its
own `--enter-stage` printed. This also binds the stamp. A batch session that does
not enter has no journal path, so skipping the entry now costs the session its
journal at its first write, and the session sees the cost immediately. Before this,
the skip was invisible to the session and to every later reader.

**Refused, on stated grounds:**

- **A read-only `--emit stage-journal-path` arm.** Its reader would be a stage
  session that does not enter, and under this ruling there is no such session. An
  arm serving one would make not entering cheap again, which is the failure being
  closed.
- **Moving the path grant into the agent contract, as a grant.** The contract loses
  its "dispatch names the journal path" line. The path is not re-stated anywhere
  else, because a restated derivation is a second source.
- **A detector for a batch that did not stamp.** No tracked artifact names a
  session except the stamp itself. Commits carry no session id, because this repo
  strips the harness trailer and a kit cannot assume one. Journals are scratch, and
  a session that never entered can write any heading. A gate would need a session
  roster the tree does not hold. The limit is written into §The state machine so it
  does not read as an omission.
- **Narrowing the stamp contract to per stage.** The drift KPIs read per-session
  stamp rows. `LIFECYCLE_KIT_SESSION_BOUNDARY`'s `stage` posture reads the session
  id. The practice has held since 2026-08-29: `git log --format=%s --
  .workflow/WORKFLOW-STATE.txt` shows one build stamp commit per batch across the
  batched iterations since then.

**Probed at authoring, 2026-09-19.** `grep -n "dispatch names the journal path"
.claude/agents/stage-session.md` returns line 68, and `grep -n "still spells it
out" lifecycle-kit/templates/lead.md` returns line 339, so the second source exists
as described. `grep -n "The supervisor still spells the" lifecycle-kit/SPEC.md`
returns line 204. The tree does not already hold this
amendment's rule.

## What changes

### (1) §Channel design stops granting the path {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/lead.md` §Channel design, the
paragraph opening "**The lead does not name that path — it derives it.**" becomes:

> **The lead does not name that path.** A stage journal's path is a function of the
> stage (lifecycle-kit/SPEC.md §The state machine), and the entering session's own
> `--enter-stage` report is its one source, so a dispatch prompt carries no journal
> path. A lead splitting one stage across sessions puts the batch discriminator in a
> **heading inside** that stage's journal, never in a filename, and reads **one**
> pull channel per stage.

### (2) The stage-session agent contract takes the path from its entry {mechanical}

**Not yet applied.** In `.claude/agents/stage-session.md`:

- §Standing dispatch policy's lead paragraph: "the stage skill to invoke, the
  batch's task slugs, and batch-specific pointers such as the journal path" becomes
  "the stage skill to invoke, the batch's task slugs, and batch-specific pointers".
- The **Resume journal.** bullet's last sentence, "The dispatch names the journal
  path.", becomes: "Your journal is the path your own `--enter-stage` printed.
  Enter even when the cursor already names your stage, since that entry is your
  stamp and your journal's source. A dispatch that names another path, or tells you
  not to enter, is declined."

### (3) §The state machine's derivation paragraph drops the prompt grant {design-bearing}

**Not yet applied.** In `lifecycle-kit/SPEC.md` §The state machine, the paragraph
opening "**The derivation is the enabling move, and it is why this was unoracled
before.**" is re-phrased to say:

- A path invented per dispatch leaves no record, which is why the path is derived.
- The derivation's one source a session reads is its own entry report
  (§bin/enter-stage.sh). The retired sentence says the supervisor restates the path
  in the prompt. Replace it with the ground: a restated path is a second source a
  dispatched session prefers, and measured batches wrote journals under
  prompt-named paths.
- Entering is therefore how any session of a stage learns its journal, which is what
  binds the per-session stamp (the same-stage re-entry paragraph's rule).
- **Honest limit:** no oracle detects a session that did not enter, because no
  tracked artifact names a session except the stamp itself. The binding is that a
  non-entering session has no journal path to write to.

The "Measured against this repo's own journals" paragraph's closing clause
("the discriminator belongs in a heading inside the stage's journal, never in its
filename") stands unchanged.

### (4) §templates/lead.md's channel clause follows {design-bearing}

**Not yet applied.** In `lifecycle-kit/SPEC.md` §templates/lead.md, the parenthesis
"(… the journal path being derived per *stage*, a per-batch filename empties that
derivation and the refusal lands cold on the **next** same-stage session)" is
re-phrased to add that the lead names no path in a dispatch, because the entry
report is the path's one source.

### (5) build's every-session-stamps paragraph names what the entry gives {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/stages/build.md`, the paragraph
opening "**Every session still stamps**" gains, after its first sentence:

> The entry is also where this session's journal path comes from; there is no
> other source.

### (6) The generated mirror {mechanical}

`docs/lifecycle-kit/SPEC.md` is regenerated after deltas 3 and 4 land. Its
freshness gate prints the command.

## Producers and consumers

- **The journal path** (deltas 1-3). Producer: `--enter-stage`'s report, which is
  already shipped and is reached on every entry where
  `LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE` is set (this repo sets it). Consumers: the
  entering stage session, at its last step and every write before it. The
  successor's entry assertion reads the same derived path. No new field, knob or
  arm.
- **The decline obligation** (delta 2). Producer: a dispatch prompt naming a path
  or suppressing the entry. Consumer: the dispatched session, which reads it against
  its agent contract. A lead-less stage run has no dispatch and needs no decline,
  because it reads its path from the entry report through the template's last step.
- **Point 6, every member's value.** The corpus is the surfaces that name where a
  stage journal's path comes from. Probe: `git grep -n -i "journal path\|names the
  journal\|spells it out\|spell.*path" -- lifecycle-kit .claude/agents CLAUDE.md
  delegation-kit/templates`. Values: lead.md §Channel design → delta 1.
  stage-session.md (two lines) → delta 2. SPEC §The state machine → delta 3. SPEC
  §templates/lead.md → delta 4. build.md → delta 5. The stage templates' last steps
  already name the entry-tool path and are unchanged.

## Existing sections updated

Roster probe: the point-6 grep above, plus `grep -n "Every session still stamps"
lifecycle-kit/templates/stages/build.md`.

- `lifecycle-kit/templates/lead.md` §Channel design (delta 1)
- `.claude/agents/stage-session.md` §Standing dispatch policy (delta 2)
- `lifecycle-kit/SPEC.md` §The state machine (delta 3)
- `lifecycle-kit/SPEC.md` §templates/lead.md (delta 4)
- `lifecycle-kit/templates/stages/build.md` (delta 5)
- `docs/lifecycle-kit/SPEC.md` (delta 6)

## Retired spellings

- None — the retired sentences are prose, and no name, knob or path is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for the path source and the decline
      obligation.
- [ ] **Instruction surfaces: instruction only** — the lead.md, build.md and
      agent-definition text carries no grounds; the grounds sit in deltas 3-4.
- [ ] **Merged with no information lost** — deltas 3-4 re-phrase the paragraphs
      they refine.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`), discharged at the iteration.
- [ ] **Done moves** — both paired entries move to Done in the merge commit, before
      the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`).
- [ ] **Gaps filed** — any cross-component gap found during the work filed.
