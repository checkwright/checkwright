# SPEC amendment: migration-lint-parity

Pairs `migration-guard-lint-parity`. check-migration-guard gains the two refusals an adopter's own migrations lint makes that the gate does not. **DUP** is two migrations the branch adds with one number. **CLASH** is a number the branch adds that another unmerged local branch also adds. CLASH reads a ref set, `GATE_SDK_MIGRATION_REFS`, and counts each ref's additions since that ref's merge base with the base ref. ORDER's highest widens to the base ref's own commit, so a number another branch already merged into the base reads as ORDER. A file that commit already holds by name is not the branch's own and gets no finding. Each added file carries at most one of the three findings.

**What the tree does today**, each bullet with the probe that settled it:

- `judge` reports EDITED and ORDER alone, so two additions sharing a number above the merge base's highest pass (`grep -n 'fn judge' -A16 native/src/gates/migration_guard.rs`).
- ORDER's highest is the merge base's. `since_base::resolve` returns the merge base and keeps no other commit (`grep -n 'pub struct Base' -A3 native/src/since_base.rs`). In a sandbox, a branch adding `0002_mine.sql` passes clean against `main` after another branch adding `0002_theirs.sql` merged into `main`. Merging the first branch then lands both files without a conflict.
- The gate reads no ref but its base. `for-each-ref`, `symbolic-ref` and `--is-ancestor` appear elsewhere in the crate, never in `migration_guard.rs` (`git grep -n 'for-each-ref\|symbolic-ref\|is-ancestor' -- native/src`).
- At git 2.56, `git for-each-ref --no-merged <commit> 'refs/heads/**'` lists every unmerged branch, including an orphan and a nested name such as `feat/x`. `'refs/heads/*'` misses the nested one. `git merge-base` exits 1 for an orphan and 128 for a name that is no object, and `git symbolic-ref -q HEAD` names the checked-out branch (sandbox probe). *Inferred:* `**` matches the same way at the git floor, 2.15 (docs/requirements.md). No 2.15 binary was run.
- The class tokens have no roster outside the gate, its pair and its sandbox test (`git grep -n 'MIGRATION-ORDER' -- ':!docs/llms-full.txt'`).

## What changes

### (1) DUP and CLASH, and ORDER against the base ref's own commit {design-bearing} {user-facing: two new refusals in a shipped gate, on by default over local branches, and ORDER also reading the base ref's commit (adopter direction 2026-11-26, lead-relayed; the ORDER widening a lead decision of 2026-11-26 under the operator's delegation, adopter-confirmed)}

**Not yet applied** — gate-sdk/SPEC.md §check-migration-guard, the **Order** bullet becomes, and two bullets follow it:

> - **Order.** A file added since the merge base is numbered above the highest file at the merge base or at the base ref's own commit: `<path>: [MIGRATION-ORDER] <added> <= <highest>`. A number the base took after the branch left it counts, so a branch merged into the base after this one forked reads here, before a merge lands the number twice.
> - **No duplicate.** No two added files share a number. Every added file but the first by name that shares one reds `<path>: [MIGRATION-DUP] <n> also <first>`.
> - **No clash.** No added file shares a number with a file another unmerged ref adds, under another name: `<path>: [MIGRATION-CLASH] <n> <ref> <their-name>`, one record per ref, naming the ref's first such file by name. A ref's additions are its files under the directory that its merge base with the base ref lacks. Two refs adding one name are no clash, since git refuses that merge as an add/add conflict, so the names are compared and the contents never.
>
> An added file carries at most one of these three, the first that applies in that order, so each case has one finding: a clash with a ref the base has already merged reads as **Order**. A number prints as its file spells it.

**Not yet applied** — a paragraph after the **The declaration** bullet:

> **The ref set** is `GATE_SDK_MIGRATION_REFS`, ref globs as `git for-each-ref` matches them, less the branch's own ref, the one `HEAD` names. A detached `HEAD` excludes none. A ref the base ref's commit already contains is skipped, and so is one sharing no merge base with it, an orphan or a shallow cut. The value `none` turns the check off and the clean line says so, and a glob opening with `-` refuses at exit 2. Any other git failure is exit 3. The gate never fetches, so a ref reads as the local clone last saw it. Under `--base-dir` there is no base ref, so the check is off there too and the clean line says so. Another ref's file whose name is not a migration name is skipped, since that tree is not the one this commit makes.

**Not yet applied** — the `help:` sentence "The `help:` lines name the new file an edit belongs in, and the declaration." becomes "The `help:` lines name the new file an edit belongs in, a number above every other branch's for a duplicate or a clash, and the declaration."

**Not yet applied** — the **Honest limit** gains, after its `UPDATE` clause: "and a ref holding an older copy of this branch's work, a remote-tracking copy under a widened ref set say, clashes with its renumbering."

**Not yet applied** — gate-sdk/SPEC.md §Layout and configuration, a bullet after `GATE_SDK_MIGRATION_DESTRUCTIVE`'s:

> - `GATE_SDK_MIGRATION_REFS` (default `refs/heads/**`): space-separated ref globs whose unmerged refs that member reads for a clash, or `none`; empty takes the default.

`native/src/since_base.rs`: `Base` gains the base ref's own commit, which `resolve` already holds before it takes the merge base. `native/src/gates/migration_guard.rs`:

- `judge` takes the tip's files beside the merge base's, and the added set's precedence.
- A ref reader runs `for-each-ref --no-merged <tip> --format=%(refname) <globs…>`, then per ref `merge-base <ref> <tip>` and `diff --name-only -z --no-renames --diff-filter=A <mb> <ref> -- <dir>`, reading names through `number`.
- The knob row joins `native/src/knobs/gate_sdk.rs`, with a value-shape check refusing a word opening with `-` and `none` beside another word. Its name joins the member's knob list in `native/src/gates/mod.rs`.

The clean line adds `, <k> other ref(s) read` after the merge base, or `; clash check off: <reason>`.

**Both acts, by whether `content-gate-calibrations` lands in this batch.** Its delta 1 reads a number at the directory's one width. If it lands first, the tip's names join its shared-width check beside the merge base's and the working tree's, and another ref's names stay outside it. If this delta lands first, `number` stays four-digit until that delta widens it, and a ref's name it refuses is skipped either way. **And by whether `migration-guard-byte-identity` lands in this batch:** that debt rewrites the **Append only** bullet and the `--base-dir` paragraph, which this delta does not touch. Whichever lands second keeps the other's text as it finds it.

- **Why each ref since its merge base with the base ref.** That is what the ref will bring when it merges: a ref forked before this branch adds what it added since, never what the base had then. Counting since the ref's merge base with this branch would charge a ref with the base's own history whenever this branch is the one behind.
- **Why names and not contents.** A same-named pair cannot merge silently, git stops it, and a lint flagging it would double-report what the merge already refuses.
- **Why ORDER reads the base ref's commit.** Skipping a merged ref is what keeps one finding per case, and the merged ref's number is then in the base, which only the base ref's commit shows. Read at the merge base alone, that case would get no finding at all, as the sandbox probe shows.
- **Why one finding per file.** A renumbered file clears every class at once. The precedence gives the finding the cheapest-to-read cause, the base before a sibling branch.

### (2) A file the base ref's commit already holds is not the branch's own {design-bearing} {user-facing: a file the base already landed under its name, by cherry-pick say, leaves the added set and gets no finding (adopter direction 2026-11-26, lead-relayed)}

**Not yet applied** — gate-sdk/SPEC.md §check-migration-guard, a sentence closing the paragraph delta 1 adds after the three bullets:

> A file added since the merge base that the base ref's own commit already holds under its name is not this branch's addition, a file the base took by cherry-pick say, and none of **Order**, **No duplicate**, **No clash** or **Not destructive** reads it. The names are compared and the contents never, as for **No clash**: a same-named file with other content cannot merge silently.

`judge` drops those names from the added set before any class reads it, off the tip's file list delta 1 already passes it. Under `--base-dir` there is no base ref, so nothing is dropped.

- **Why the skip.** With ORDER reading the base ref's commit, a branch whose file the base already holds would red its own landed file as at or below the base's highest. The base holding the name means the merge brings nothing under it.
- **Why the skip reaches the destructive check.** The file's statements are the base's, judged where they landed. A copy with other content stops at the merge as an add/add conflict, so dropping it never lets a destructive statement land unread.

### (3) The pair, the sandbox cases and the declarations {mechanical}

- **The pair.** `bad/` gains two additions above the base's highest sharing one number, for the DUP record. `good/` gains a second addition numbered above the first. Both clean lines gain the clash check's `--base-dir` note, and `expect.txt` follows. The pair cannot reach CLASH or the tip, so `check-migration-guard.test.sh` drives them.
- **Sandbox cases**, a bullet in the section's list:
  - a clash with an unmerged sibling, and none for one adding the same name with other content;
  - a sibling merged into the base after the branch forked, read as ORDER and not as CLASH;
  - a detached `HEAD`, an orphan ref skipped, `none` and a narrowing glob, and a glob opening with `-` at 2;
  - the precedence: an addition both a duplicate and a clash prints DUP alone;
  - a branch whose addition the base took by cherry-pick, same name, passes clean, beside the merged-after-fork case.
- **Unit tests** in `migration_guard.rs`: `judge` over a tip, a duplicate, the precedence, and a tip already holding an added name, destructive text in it included.
- `.workflow/release-declarations.md`: *New and tightened gates* gains DUP, CLASH on by default and ORDER's widening, each able to red a tree that passes today with another local branch or a moved base. *Behavior changes* gains the skip: a destructive statement in a file the base already holds by name no longer reds on the branch. *Knob changes* gains `GATE_SDK_MIGRATION_REFS`.

## Producers and consumers

- **The DUP record.** Producer: `judge`, over the added set. Consumer: the person reading the gate's output; `run-gates` places it as a path record.
- **The CLASH record.** Producer: the ref reader, from `for-each-ref`, `merge-base` and `diff`. Consumer: the same reader. The `<ref>` and `<their-name>` fields tell the reader which branch to coordinate with and which file it holds.
- **The base ref's commit.** Producer: `since_base::resolve`. Consumers: ORDER's highest, the already-landed skip, the ref set's `--no-merged` filter and each ref's merge base. No other since-base reader reads it.
- **The already-landed skip.** Producer: `judge`, from the tip's names. Consumers: every class reading the added set, the destructive check included; the clean line's count of added files reads the set after it.
- **`GATE_SDK_MIGRATION_REFS`.** Producer: the knob table and the consumer's knob file. Consumers: the ref reader and the knob validator.
- **Point 2, roster-holding readers.** §Layout and configuration's knob roster, held by `check-knob-default-coupling` and `check-knob-citation`; the table rows; the member's knob list in `native/src/gates/mod.rs`. The class tokens have none.
- **Point 5, narrowing.** The ref set narrows by glob, `none` and the merged-ref skip. The only reader of its size is the clean line's count, which reds nothing.

## Existing sections updated

Roster probe: `git grep -n 'MIGRATION-ORDER\|GATE_SDK_MIGRATION_DESTRUCTIVE' -- ':!docs/llms-full.txt' ':!TASK-QUEUE.md'` and `git grep -n 'pub struct Base' -- native/src`, over the tracked tree.

- `gate-sdk/SPEC.md` — §check-migration-guard's **Order** bullet, the two new bullets, the ref-set paragraph, the `help:` sentence and the honest limit, and §Layout and configuration's knob bullet (delta 1), the already-landed sentence (delta 2), and the sandbox list (delta 3).
- `native/src/since_base.rs` — `Base`'s base-ref commit (delta 1).
- `native/src/gates/migration_guard.rs` — `judge`, the ref reader, the clean line and the help lines (delta 1), the skip (delta 2), and the unit tests (delta 3).
- `native/src/knobs/gate_sdk.rs` — the knob row and its check (delta 1).
- `native/src/gates/mod.rs` — the member's knob list (delta 1).
- `gate-sdk/gate-tests/check-migration-guard/good/` and `bad/` — the DUP case and the clean lines (delta 3).
- `gate-sdk/gate-tests/check-migration-guard.test.sh` — the sandbox cases (delta 3).
- `.workflow/release-declarations.md` — the tightening, the skip and the knob (delta 3).
- `docs/gate-sdk/SPEC.md` — generated mirror (all deltas).
- `docs/llms-full.txt` and `docs/requirements.md`'s install-size block — generated, each regenerated by the command its freshness gate prints on red (all deltas).

## Retired spellings

- None — no delta of this amendment retires a spelling; the **Order** record keeps its text.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for DUP, CLASH, the base ref's commit, the skip and the knob.
- [ ] **Instruction surfaces: instruction only** — the knob bullet states the value and the refusals, the grounds staying in the section.
- [ ] **Merged with no information lost** — each passage is re-phrased in place, not appended to.
- [ ] **Amendment deleted** — this file removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` holds.
- [ ] **Run once** — the merged-after-fork sandbox, ORDER and no CLASH, an unmerged sibling clash, and a cherry-picked addition clean, each by hand with the built binary.
- [ ] **Release declared** — `.workflow/release-declarations.md` carries the tightening, the skip and the knob.
- [ ] **Entry moved** — `migration-guard-lint-parity` moves to Done before the drain stage, in the commit that lands the merge.
- [ ] **Gaps filed** — any cross-component gap discovered during the work filed as a debt task.
