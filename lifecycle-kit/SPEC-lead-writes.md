# SPEC amendment: lead-writes

The lead template bounds what the lead writes while a dispatched stage session is live only for tracked captures: a gap bullet or a survey block waits in the lead's journal. Every other write is unbounded. A lead once created a linked worktree for a read probe while a validate session was live, then removed that session's live reproduction worktrees after calling them obsolete without reading where they came from, and the operator instructed the removal on that description. This amendment widens the bound to every write the lead makes, and makes a worktree's provenance a read the lead owes before calling it stale.

One queue entry pairs it: [lead-writes-during-live-stage](../TASK-QUEUE.md#lead-writes-during-live-stage).

**The rulings.**

- **The bound is every write, gitignored scratch included.** A live stage keeps its evidence in scratch as much as in the tree: its resume journal and its producers' liveness records sit in the scratch directory. A lead's own scratch write can also stop the stage outright. A liveness record the lead leaves under a scratch directory blocks every session's `git` writes while its pid lives (guard-kit/SPEC.md §The generic ruleset, rule `git_mutation_under_producer`), the live stage's commits included.
- **Three writes stay the lead's.** Its resume journal, its session-role marker, and what the arms it runs write for it: the dispatch marker (`--dispatch`, `--dispatch-withdraw`), the journal's opening (`--open-lead-journal`) and a budget verdict's snapshot. A second batch may be dispatched while a first is live, so the dispatch arms cannot wait for a quiet tree.
- **A probe that must write runs after the stage, or outside the checkout.** A read probe needs no write. One that does waits for the completion notification, or runs in a separate clone.
- **Provenance before a stale call.** Before calling a linked worktree stale, obsolete or safe to remove, the lead reads its branch and lock reason, whether it is dirty, its commits the main checkout lacks, and which session's journal or dispatch names it. A worktree a live or unreported session may own is that session's to answer for, so the lead asks it. The attested failure was a description, and the read is what the description must carry.
- **No gate.** A stage session's liveness is a fact about a session that no read of the tree settles (§The state machine's honest limit on the dispatch precondition), and in one checkout the shell guard cannot tell a lead's write from the stage's. The rule is prose, like the dispatch precondition it sits beside.
- **The seam.** Kit mechanism: the template rule and its SPEC grounds, generic to any consumer running a lead. No consumer config and no private rule content is involved.

**Refused.**

- **A guard refusing a lead's writes.** The session-role marker names the lead's session, but nothing on disk says a stage session is live: the dispatch marker is discharged at the stage's entry, and a journal without `DONE` is also what a crashed session leaves. A guard keyed on either would refuse legitimate writes or miss live stages.
- **Scoping the bound to tracked writes plus worktrees.** It would leave the liveness-record hazard open, and every scratch write the lead needs is already one of the three it keeps.

## What changes

### (1) The write bar while a stage is live {mechanical}

**Not yet applied.** lifecycle-kit/templates/lead.md §Stamps are authoritative (the load-bearing invariant): the paragraph opening **And file and commit it yourself, between dispatches.** is replaced by:

> **While a dispatched stage session is live, write nothing into the checkout.** A stage session is live from its dispatch until its completion notification. While one is, write nothing into the checkout or any linked worktree: no file, no directory, no worktree added or removed, no `git` write, gitignored scratch included. Three writes stay yours: your resume journal, your session-role marker, and what the arms you run write for you (`--dispatch`, `--dispatch-withdraw`, `--open-lead-journal`, a budget verdict). A capture you make, a gap bullet or a survey block, goes to your journal. A probe that must write runs after the stage completes, or in a clone outside the checkout.
>
> **File and commit your captures between dispatches.** At the first moment no dispatched stage session is live, file each held capture to its channel (`--emit file-gap`, `--emit file-survey`) and commit it on its own. A capture commit is none of the lifecycle state the invariant above enumerates, so it is yours to make. Neither capture arm makes a `git` call: judging when the tree is free is the filing session's (lifecycle-kit/SPEC.md §The committed gap inbox).

### (2) A worktree's provenance before a stale call {mechanical}

**Not yet applied.** lifecycle-kit/templates/lead.md §A running session is asked, never instructed gains a paragraph after the one opening **A running session is never stopped on the lead's authority alone.**:

> **Read a worktree's provenance before you call it stale.** Before you describe a linked worktree as stale, obsolete or safe to remove, read who made it and what it holds: its branch and lock reason (`git worktree list --porcelain`), whether it is dirty (`git -C <path> status --porcelain`), its commits the main checkout lacks (`git log --oneline HEAD..<branch>`), and the journal or dispatch that names its path. A worktree a live or unreported session may own is that session's: ask it, as above. Your description names what you read.

### (3) The SPEC carries the grounds {design-bearing}

**Not yet applied.** lifecycle-kit/SPEC.md §templates/lead.md gains one paragraph. Its act depends on whether [lifecycle-template-brevity](../TASK-QUEUE.md#lifecycle-template-brevity) has already passed the section when this delta lands:

- **Brevity has not landed on the section:** append the paragraph after the section's second paragraph, the one opening *The template owns the orchestration protocol whole*, as its own paragraph and never inside it.
- **Brevity has landed on the section:** place the paragraph beside the rewritten section's statement of the stamps-authoritative invariant and its corollaries, as its own paragraph. Re-phrase it to the rewritten section's register if the brevity pass changed that register; the four facts below are the content, and none is dropped.

> **While a stage session is live the lead writes nothing into the checkout, scratch included, and it reads a worktree's provenance before calling it stale.** A live stage keeps evidence in scratch as well as in the tree: its resume journal and its producers' liveness records. A lead's scratch write can also stop the stage, since a liveness record under a scratch directory blocks every session's `git` writes while its pid lives (guard-kit/SPEC.md §The generic ruleset, rule `git_mutation_under_producer`). The lead keeps three writes: its journal, its session-role marker, and what the arms it runs write for it; the dispatch arms are among them because a second batch may be dispatched while a first is live. The provenance read is owed because the attested harm was a description: a lead called a live stage's worktrees obsolete without reading where they came from, and they were removed on that description. **Honest limit:** no gate holds either rule. Whether a stage session is live is a fact about a session that no read of the tree settles (§The state machine), and in one checkout the shell guard cannot tell a lead's write from the stage's.

## Producers and consumers

- **The write bar** (delta 1) is an obligation with no new name.
  - Producer: the lead template, read by every lead session at its first step.
  - Consumer: the lead session, which checks the three exempt writes before writing. The failure it prevents lands on the live stage session, whose commits, journal and worktrees the bar protects.
  - No gate reads it (the honest limit in delta 3).
- **The provenance read** (delta 2).
  - Producer: the same template.
  - Consumer: the lead, at the moment it would describe a worktree. The operator, or whoever acts on the description, reads what the lead read.
  - The reads it names are existing `git` commands; no field or name is minted.
- **Roster-holding readers:** none. The amendment mints no name, so no roster owes a row.

## Existing sections updated

Roster probe: `git grep -n` over the tracked tree for `And file and commit it yourself`, `the capture paragraph`, `A running session is never stopped` and `§templates/lead.md`.

- `lifecycle-kit/templates/lead.md` — §Stamps are authoritative (the load-bearing invariant)'s capture paragraph (delta 1); §A running session is asked, never instructed (delta 2). §Economics' *Write the lead journal at every stage completion* bullet points at the capture paragraph by description and stays true under delta 1.
- `lifecycle-kit/SPEC.md` — §templates/lead.md (delta 3).
- `docs/lifecycle-kit/SPEC.md` — the generated on-site mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (delta 3).
- `.workflow/release-declarations.md` — Behavior changes gains a bullet led by **lifecycle-kit/templates/lead.md**: the lead writes nothing into the checkout while a stage session is live, scratch included, and reads a worktree's provenance before calling it stale (deltas 1 and 2).

## Retired spellings

- None — every delta adds or re-phrases prose; no name, path or token is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Battery green** — the full battery, and lifecycle-kit's fixture suite, green on the landing commit. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`).
