# SPEC amendment: lead-capture

lifecycle-kit/templates/lead.md has the lead commit a gap bullet "at the first moment the git index is free of stage-session work". It rules when the lead commits and says nothing about when the lead writes. A lead filing mid-batch leaves a tracked file dirty under a running batch, where two things go wrong:

- **A clean-tree precondition refuses its dirt.** `installer/consumer-smoke/run-smoke.sh:84-85` refuses a dirty worktree outright, and its build-leg check at `:149-150` would misattribute a lead write to the build.
- **A batch's `git add` can sweep the bullet into the batch's own commit.** §The committed gap inbox names this as the failure its commit rule exists to stop.

At gate-sdk-blind-spots the lead's uncommitted `.workflow/gap-inbox.md` made that smoke refuse, so it shipped unrun and a later batch discharged it.

**The ruling: a capture made while a dispatched stage session is live is held in the lead journal, and filed and committed at the first moment none is live.** The other option the entry offers is refused: exempting the gap inbox from clean-tree preconditions.

- **The survey record has the same shape.** The survey record is the lead's other tracked capture channel (`--emit file-survey`), and an exemption for the inbox alone would leave it with the same failure.
- **The index sweep stays open.** An exemption leaves the batch's `git add` free to sweep the bullet.
- **The installer has already refused this fix.** installer/SPEC.md §The packer refuses a gap-inbox-specific carve-out as the fix for its own clean-tree predicate, on the ground that ownership of the capture's commit is the lifecycle's question, not the installer's.

Holding needs no new surface. The lead journal is on disk, boundary-protected by kit invariant (§bin/enter-stage.sh), and already the lead's store for iteration-local state. lead.md's close disposition already moves journal findings into "a gap bullet, a survey block".

**The one rule this bends, and why it may bend.** lead.md makes a durable finding's committed channel its home "in the moment it is found", and the journal "never the only home". Under this ruling the journal is the only home for the span of one live batch. That is a bounded window on a protected file, and it replaces an unbounded one: today's bullet rides uncommitted until whichever session next stages. It is not the deferred-capture antipattern §The committed gap inbox refuses, since the finding is on disk from the moment it is found.

**Measured at authoring (2026-09-23)**, by the research dispatch:

- **The rule.** `lead.md:152` holds the commit rule, and `lead.md:166` holds "filed to its committed channel in the moment it is found".
- **The refusals.** `run-smoke.sh:84-85` and `:149-150` are whole-tree clean checks. The packer's own check (`native/src/emit/pack_installer.rs:145`) is footprint-scoped and untouched by a lead's `.workflow/` write.
- **The file states.** `git check-ignore -v` shows `.workflow/gap-inbox.md` and `.workflow/survey-record.md` tracked, and `.tmp/lead-journal.md` ignored.
- **The attested timeline.** `git log --format='%h %ad %s' --date=format:%H:%M 2efdfdd5^..e1bf748a` shows batch 1's commits at 10:22-10:46, the lead's gap commit 87a289d7 at 10:48, and batch 2's stamp at 10:50.
- **The refusal itself.** The smoke refusing on the lead's dirt is attested only by the filing bullet, `git show 9d16c63c`; no run log survives.

## What changes

### (1) The lead holds a mid-batch capture and files it between dispatches {mechanical}

**Not yet applied.** In `lifecycle-kit/templates/lead.md`, the paragraph opening "**And commit the bullet yourself**" becomes:

> **And file and commit it yourself, between dispatches.** While a dispatched stage session is live, a capture you make — a gap bullet, a survey block — goes to your own journal, never to its tracked channel: a lead write into the tree mid-batch is foreign dirt to that batch, which a clean-tree precondition refuses and a batch's `git add` can sweep into its own commit. At the first moment no dispatched stage session is live, file each held capture to its channel (`--emit file-gap`, `--emit file-survey`) and commit it on its own. The invariant above enumerates stamps, queue writes and evidence files; a capture commit is none of them, and reading the omission as a ban leaves a bullet riding uncommitted into the next iteration. Neither capture arm makes a `git` call: judging when the tree is free is the filing session's (lifecycle-kit/SPEC.md §The committed gap inbox).

In the same template's "**Write the lead journal at every stage completion.**" bullet, "is filed to its committed channel in the moment it is found; the journal may carry a copy for your own use and is never the only home" becomes:

> is filed to its committed channel in the moment it is found, or — found while a dispatched stage session is live — at the first moment none is (the capture paragraph above); outside that window the journal may carry a copy for your own use and is never the only home

### (2) The owning section states the timing and closes the survey record's open question {mechanical}

**Not yet applied.** In lifecycle-kit/SPEC.md §The committed gap inbox, "**The filing session commits its own bullet**, at the first moment the git index is free of stage-session work." becomes:

> **The filing session commits its own bullet**, at the first moment the git index is free of stage-session work — and a filing session supervising live stage sessions *writes* it then too, holding the capture in its own journal meanwhile, because a tracked write under a live session is dirt that session's clean-tree preconditions refuse and its `git add` can sweep.

At the paragraph's end, after "a literal reader takes the omission of commits for a prohibition.", add:

> Holding rather than exempting the inbox from clean-tree preconditions is the ruling: an exemption covers one capture channel of two, leaves the sweep open, and is the carve-out the installer's own predicate refused (installer/SPEC.md §The packer).

In lifecycle-kit/SPEC.md §The survey record, the paragraph "**Open, and shared with the gap inbox:** who commits an append filed by a session that is not the stage session. …" becomes:

> **Committed as the gap inbox's bullets are:** by the filing session, at the first moment no stage session's work holds the index, and written then too where the filer supervises live stage sessions (§The committed gap inbox).

## Producers and consumers

- **The held capture.**
  - Producer: the lead, while a dispatched stage session is live.
  - Store: the lead journal, `LIFECYCLE_KIT_LEAD_JOURNAL_FILE`, boundary-spared.
  - Consumer: the lead itself, at the next moment no session is live, through the existing capture arms. No arm, knob or field is added.
- **Readers of the gap inbox and the survey record.** The drain stage and the survey witness are unchanged. They read the committed file, which now receives a held bullet one batch later at most.
- **Point 5.** No corpus narrows.
- **Point 6.** Not obliged.

## Existing sections updated

Roster from `grep -n "first moment the git index\|in the moment it is found\|Open, and shared with the gap inbox" lifecycle-kit/templates/lead.md lifecycle-kit/SPEC.md installer/SPEC.md`, run 2026-09-23.

- `lifecycle-kit/templates/lead.md` (delta 1).
- `lifecycle-kit/SPEC.md` §The committed gap inbox and §The survey record (delta 2).
- `.workflow/surface-ceiling.txt` and `docs/footprint.md`, regenerated for the template's growth by their gates' printed commands (delta 1).
- `.workflow/release-declarations.md`, one Behavior changes bullet (delta 1): the lead template holds a capture made during a live batch in the lead journal and files it between dispatches.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/lifecycle-kit/SPEC.md`.

## Retired spellings

- None — no delta renames or deletes a spelling.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the held capture.
- [ ] **Instruction surfaces: instruction only.** The lead paragraph carries the act and one clause of reason, and the grounds sit in delta 2's SPEC text.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Entry moved.** `lead-capture-dirties-batch` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
