# SPEC amendment: merge-stamps

§check-stamp-subject reds a merge commit that brings in stamp lines, because git's own `Merge …` subject has no scope to parse (its honest-limits paragraph). gate-sdk's `check-commit-subject` admits that subject as a git-generated carve-out, so a consumer merging a branch that carries stamps meets two gates with conflicting remedies.

**The ruling: a merge adds only the stamps no parent carries.** The entry offered two remedies. (a) Exempt git-generated subjects on `check-commit-subject`'s precedent. (b) Keep the red and name the merge-time remedy. Both are refused in favour of fixing what the gate counts as added. A stamp on a merged branch was introduced by its own commit on that branch, which the gate already checked there. At the merge it is inherited, not introduced, so reading it as an addition re-checks a line against a subject that never introduced it. Remedy (a) keys the exemption on the subject, so a hand-written `Merge …` subject on an ordinary entry commit would pass a stamp nobody checked. Remedy (b) makes every such merge carry a stage scope that describes no stage entry. Counting against every parent removes the false red and keeps the check on a stamp the merge itself introduces, such as one written while resolving a conflict.

**When the case arises.** The state file is `merge=iteration-scoped` (§Multi-operator semantics), but git runs a merge driver only when both sides changed the file. A branch that stamped merged into a home branch that did not takes the branch's version without the driver, so the merge adds those lines relative to the first parent. That is a topic branch cut inside one iteration, the shape a multi-operator consumer uses.

**Measured at authoring (2026-09-22), in a scratch clone with this repo's commit-msg hook.** A side branch added a build stamp under `chore(build): …`, and the home branch diverged. `git merge --no-ff --no-edit` refused: `check-stamp-subject` named the subject `Merge branch 'side'` and expected `(build)`. `git merge --no-ff -m "chore(build): merge side"` passed all three commit-msg gates. So remedy (b)'s workaround works today, and the entry's inferred marker is settled. A second probe ran a hook printing `git rev-parse -q --verify MERGE_HEAD`. `MERGE_HEAD` resolved on a clean `--no-ff` merge (one parent line), on a conflict resolved and finished with `git commit` (one line), and on an octopus merge (two lines). It was absent on an ordinary commit. So the parents are readable at the commit-msg tier on every path that creates a merge commit.

## What changes

### (1) §check-stamp-subject: the added stamps are read against every parent {design-bearing}

**Not yet applied.** In "What the gate reads", replace "the added stamps are the data lines of the staged state file absent from `HEAD`'s" with "the added stamps are the data lines of the staged state file absent from `HEAD`'s version and from the version of every commit `MERGE_HEAD` lists, when a merge is in progress". Add after the sentence ending "so writer and asserter read the same stamp.":

> A merge inherits the stamps its parents carry, so only a line no parent holds is an addition. The inherited stamps were introduced, and checked, at their own commits. A stamp the merge itself introduces, for example one written while resolving a conflict, still needs the stamped stage as the merge's subject scope. The exemption rides the parents and never the subject, so a hand-written `Merge …` subject on a commit that introduces a stamp still reds.

In "The honest limits", delete the sentence "A git-generated subject (`Merge …`) on a commit adding stamps reds: it has no scope to parse." and add: "A parent whose state-file version git cannot show counts as carrying no lines, which is `HEAD`'s absent-side rule applied to it."

### (2) The gate module reads the merge parents {mechanical}

**Not yet applied.** `native/src/gates/stamp_subject.rs`: in the live form, read `MERGE_HEAD` through git (`git rev-parse -q --verify MERGE_HEAD` for presence, and the git dir's `MERGE_HEAD` file for the parent list). Take each listed commit's `<sha>:<state file>` blob through the existing `blob()` helper, and pass the data lines of `HEAD`'s version together with those of every parent's as the prior set `stages::last_added_stamp` reads. With no merge in progress the prior set is `HEAD`'s alone, as today. A parent blob git cannot show contributes nothing. Unit tests: a line only a merge parent carries is not an addition; a line no parent carries is; with no parents the behaviour is unchanged.

**Batch condition, with `build-work-before-entry-stamp` (`SPEC-dispatch-entry.md`).** That entry adds `check-dispatch-entry`, which reads the same added-stamp set. If it has landed in an earlier batch or lands in this one, the parent-aware read becomes one shared function in the gate modules and both gates call it in this commit. If it lands later, this gate alone carries the read, and that entry's merge takes it from here.

### (3) The fixture form takes merge-parent blobs {mechanical}

**Not yet applied.** The fixture form becomes `<message-file> <staged-state> <head-state> [<merge-parent-state>…]`, so the arity check admits one argument or three and more. §check-stamp-subject's fixture paragraph says so, and `gate-tests/check-stamp-subject.test.sh` gains two cases: a stamp only a merge-parent blob carries, under a `Merge …` subject, is clean; a stamp in no blob, under a `Merge …` subject, reds. The usage line names the optional trailing blobs.

### (4) The pending release declaration states the narrowed red {mechanical}

**Not yet applied.** `check-stamp-subject` is unreleased, so its pending `Tightened gates` bullet in `.workflow/release-declarations.md` is corrected in place and no `Behavior changes` bullet is added. Replace "a merge commit that brings in stamp lines under git's own `Merge …` subject reds, having no scope to parse" with "a merge commit reds only on a stamp none of its parents carries; stamps a merged branch brought in were checked at their own commits".

## Producers and consumers

- **The parent-carried set** (deltas 1 to 3). Its producer is git, which writes `MERGE_HEAD` before it runs the commit-msg hook on each merge path (measured above). The hook runs the gate with no enabling config wherever the gate is registered. Its consumer is `stages::last_added_stamp`, called with the widened prior set by this gate and by `check-dispatch-entry` under the batch condition in delta 2. `--enter-stage`'s own call passes the pre-write state file and never a merge parent, so its printed subject is unchanged.
- **The fixture blobs** (delta 3). Read by the gate's argument-mode branch only, and exercised by the scenario test.
- **Point 5 (narrowing).** This narrows the set the gate counts as added. The gate reds on a non-empty set with a wrong scope, so the verdict is monotone in that set: a smaller set can only clear reds. It holds no count or floor. No other gate reads this set.
- **Point 6.** Not reached; no member obligation is added.

## Existing sections updated

Roster from `git grep -n -i "stamp-subject\|stamp_subject\|Merge …" -- ':!docs/' ':!TASK-QUEUE.md'`, run 2026-09-22.

- `lifecycle-kit/SPEC.md` §check-stamp-subject, "What the gate reads", the honest limits and the fixture paragraph (deltas 1 and 3).
- `native/src/gates/stamp_subject.rs`, `run()` and its unit tests (delta 2).
- `lifecycle-kit/gate-tests/check-stamp-subject.test.sh`, two merge cases (delta 3).
- `.workflow/release-declarations.md`, the pending `check-stamp-subject` bullet (delta 4).

## Retired spellings

- None — no delta retires a name; the removed honest-limit sentence is prose and names nothing.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness checklist holds for the parent-carried set and the fixture blobs.
- [ ] **Instruction surfaces: instruction only.** Not reached; no template or shim changes.
- [ ] **Merged with no information lost.** "What the gate reads" is re-phrased, not appended to, and the honest-limit sentence it falsifies is deleted.
- [ ] **Amendment deleted.** This file is removed on merge (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Entry moved.** `stamp-subject-merge-carve-out-unruled` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached; the block above declares none.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
