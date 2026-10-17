# SPEC amendment: companion-prove

Pairs `companion-prove-not-held`. `prove` stages one claim, `demo`'s: a task moved to Done and validate and close stamped with no evidence, written over the queue and the stage-state file (installer/SPEC.md §prove). A companion `prose` or `complement` install keeps no stage-state file, since lifecycle-kit is the kit those tiers leave to the toolkit (companion/SPEC.md §The tiers), so `prove` answers not held there by construction and its `help:` line recommends `full`, the tier the companion exists to avoid. Yet those installs catch a done claim of their own kind: a ticked task naming a path the tree does not hold, by the task-list gate the recipes arm (companion/SPEC.md §The tested claim). This amendment gives `prove` that claim.

**The direction that settles the form.** The entry's Deliverable left the choice between a companion claim and a withheld or reworded `then:` line to this stage. The set record (operator direction 2026-10-17, lead-relayed, not a ruling) names "the companion claim" and orders the confinement unit ahead of it, so the claim is the form chosen.

**The refused alternative: withholding the `then:` line where no claim can be staged.** It leaves a companion tester with no proof step at all, and `init` would have to predict at install time what `prove` decides at run time from `HEAD`. A rewording keeps the step and still answers not held. The claim keeps one step, one verdict grammar and one `then:` line for every install.

**Toolkit-blind.** The claim reads canon-kit's task-list knob and no toolkit name, so a consumer outside the companion that names task lists gets the same proof.

## What changes

### (1) `prove` stages the task claim where the tree keeps no stage state {design-bearing} {user-facing: companion-prove-not-held's Deliverable, which left the form to this stage, and the set record's operator direction of 2026-10-17 naming the companion claim, lead-relayed}

Replacement for `installer/SPEC.md` §prove, the paragraph after the heading's question. **Not yet applied.**

> It makes a done claim with no evidence behind it in a clone of your `HEAD`, runs your own battery over it, and reports what the install configures, enforces and leaves inactive. It writes nothing to your worktree, index, refs or object store.

Replacement for `installer/SPEC.md` §prove, run step 4. **Not yet applied.**

> 4. **The claim.** It is one of two, chosen by what the clone keeps, committed with `--no-verify` before the battery runs again:
>    - **The stage claim**, where the clone holds both files step 3 resolved: `demo`'s claim (§demo, act 3) written over them, caught by a reddened block naming the claim's iteration. A queue lacking the iteration header or the `## Done` section the claim is staged in has no claim to catch, and the verdict states which.
>    - **The task claim**, where the clone lacks either file and a task list is there: a ticked task naming `demo`'s task and a path the clone does not hold, under a top-level directory named for the verb, appended as a new list item at the end of the first task list in path order whose path resolves inside the clone, as step 3 resolves a knob's. A task list is a markdown file `CANON_KIT_TASK_LIST_GLOBS` matches, found through the pruning walk with every knob read from the clone, as a battery at its root reads them (canon-kit/SPEC.md §check-task-path-claim). The claim is caught by a reddened block naming that path. A clone already holding the path is the env verdict.
>
>    The claim is **caught** when the battery exits non-zero and such a block is found, each reddened block quoted back through `demo`'s excerpt rule. A tree keeping neither has no claim to catch.

Replacement for `installer/SPEC.md` §prove, the verdict's exit-1 bullet. **Not yet applied.**

> - **1** — `PROVE: not held — <cause>`: `the claim was not caught`; `this tree keeps no <file> and no task list to claim a task done in`, `<file>` the first missing of the queue and the state file; or `<file> keeps no <part> to claim a task done in`, `<part>` the missing heading, the iteration header or the `## Done` section. The no-file cause is followed, on stderr so the verdict stays the last line of stdout, by a `help:` line naming the profile `demo` installs, which keeps a task queue and stage state, and, where the lock's `kits` carries the kit owning `CANON_KIT_TASK_LIST_GLOBS`, that knob, which names a task list; `init --help` lists the profiles and the `--with-kit` selection.

Replacement for `installer/SPEC.md` §prove, the honest-limits paragraph's sentence on a red for another reason, rewritten to carry both claims. **Not yet applied.**

> A gate red for another reason whose block prints the queue header, or the task claim's path, would read as a catch. A tree narrowed from a profile that kept both lifecycle files keeps them on disk (§Selecting kits and gates), so it stages the stage claim, which a battery without the stage gates does not catch. A task list whose last fence is unclosed can hide the appended task from the gate, and the claim then reads as not caught.

`demo::caught` takes the string a block must name, `demo` passing its iteration and `prove` the claim's own, so the excerpt rule stays one function. The kit owning the knob is read off the knob table, so `prove.rs` spells no kit name.

### (2) The task claim's path takes the confined resolution {mechanical}

The task list the claim is appended to is a path the walk found in the clone, and a symlink there can point outside it, the escape `prove-claim-path-confinement` closes for the two lifecycle knobs. Delta 1's text already confines it ("whose path resolves inside the clone, as step 3 resolves a knob's"): a list resolving outside the clone is no task list of the clone's, and the walk takes the next. This delta owns the implementation, which depends on that sibling:

- **Where `prove-claim-path-confinement` has landed**, in an earlier batch or earlier in this one, the list's path calls the resolver it adds, with no second implementation.
- **Where it has not**, this delta does not land ahead of it: the set record orders confinement first, so the batch carrying deltas 1 and 2 follows the confinement commit, never precedes it.

A crate test plants a task list that is a symlink out of a scratch clone and asserts the claim is never written through it.

### (3) The companion arm proves each `prose` and `complement` install {mechanical}

Replacement for `installer/SPEC.md` §The consumer smoke, the companion arm's step 3. **Not yet applied.**

> 3. asserts the battery green, reading its summary line, then asserts a bare `init` re-run leaves the tree object unchanged, since it re-applies the recorded payload recipes. Then `checkwright prove`, run with stdin closed and `DEMO_TMP_DIR` at a directory the harness owns, must exit 0 with a last line matching the held grammar (§prove), leave the consumer's tree object and `git status --porcelain` unchanged and that directory empty;

The complement legs run steps 2 to 4, so each toolkit's `complement` line is proved as its install line is. The full and lifecycle legs run no step 3 and gain nothing. The step reuses the prove arm's unchanged-consumer and held-grammar readers in `native/src/emit/installer_smoke/profiles.rs` rather than copying them. The run's closing summary string in that file names the new assertion beside the companion arm's.

### (4) The companion's tested claim names the proof {mechanical}

Replacement for `companion/SPEC.md` §The tested claim, the shared roster's last bullet. **Not yet applied.**

> - a ticked task naming a path that does not exist, by `check-task-path-claim`, the claim `checkwright prove` makes on these tiers and holds on each ([installer/SPEC.md §prove](../installer/SPEC.md#prove)).

## Producers and consumers

- **The task claim (deltas 1 and 2).**
  - Producer: `prove`, on a clone keeping neither lifecycle file pair and at least one task list. The enabling config is the recipe line each toolkit sets, `CANON_KIT_TASK_LIST_GLOBS[]` in `canon-config.knobs`, from `grep -n TASK_LIST_GLOBS companion/*/recipe/*.knobs`: `specs/*/tasks.md` for Spec Kit, `openspec/changes/*/tasks.md` for OpenSpec. Each fixture holds one match, from `ls companion/fixtures/speckit/layout/specs/*/tasks.md companion/fixtures/openspec/layout/openspec/changes/*/tasks.md`.
  - Consumer: the clone's battery, through `check-task-path-claim` reading the list on its next run, which a companion tier registers (`# install: zero-config`, armed by the knob); then `prove`'s caught test, reading the excerpt for the path.
  - Selection premise, from `native/src/installer/recipe.rs`'s `seed` arms: the state file is seeded by lifecycle-kit's arm alone, and both `complement` lines and the `prose` profile leave lifecycle-kit out (`companion/exclusions.list`; installer/SPEC.md §Profiles), so a companion `prose` or `complement` clone never holds both files and always takes the task claim. A `full` clone holds both and keeps the stage claim.
  - Fields: the path, read by the caught test; `demo`'s task name, read by the adopter in the quoted block.
- **The not-held cause and the help line (delta 1).** Producer: `prove`. Consumers: the adopter or evaluating agent; the prove arm, which asserts the last line opens `PROVE: not held — this tree keeps no` on the lattice minimum (`grep -n "keeps no" native/src/emit/installer_smoke/profiles.rs`), a prefix the new cause keeps. The `help:` line's knob clause has its reader in a tree carrying canon-kit with no task list configured, the plain `prose` profile.
- **The companion arm's prove step (delta 3).** Producer: the consumer smoke, in each toolkit's install and `complement` consumers. Consumer: the smoke's verdict.
- **Point 5:** no delta narrows a corpus. The prove arm's minimum-profile assertion reads a prefix the new cause keeps.
- **Point 6:** delta 3 obliges each toolkit's `prose` and `complement` consumer. The members, from `ls -d companion/*/recipe` and `cut -d' ' -f1 companion/exclusions.list | sort -u`, are speckit and openspec, two tiers each; each satisfies with the task claim on its fixture's one task list.

## Existing sections updated

The roster below came from `grep -n "claim\|keeps no\|help:" installer/SPEC.md`, `grep -rn "caught\|keeps no" native/src/installer native/src/emit/installer_smoke` and `grep -n "check-task-path-claim\|prove" companion/SPEC.md` over the tracked tree.

- `installer/SPEC.md` §prove, `native/src/installer/prove.rs` and `native/src/installer/demo.rs` (deltas 1 and 2).
- `installer/SPEC.md` §The consumer smoke, `native/src/emit/installer_smoke/companion.rs` and `native/src/emit/installer_smoke/profiles.rs` (delta 3).
- `companion/SPEC.md` §The tested claim (delta 4).
- `docs/installer/SPEC.md` and `docs/companion/SPEC.md`, the generated mirrors (all deltas).

## Retired spellings

- None — the not-held cause gains a clause and keeps the prefix the prove arm reads, and no name is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; no root-level amendment of this unit remains (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Fixture suites** — the installer consumer smoke, its prove and companion arms included, and the installer crate cases run green.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, before the drain stage.
