# SPEC amendment: personal-prove

Pairs `personal-install-prove`. `prove` clones `HEAD`, and a personal install commits nothing, so today the verb refuses over one. Under the operator's direction of 2026-10-24 (lead-relayed, not a ruling), it copies the install into its clone instead and proves the working tree's install over `HEAD`'s tracked content. The direction's terms: refuse a destination reached through a link and a roster path `HEAD` now tracks; on a personal install the not-held help names the task-list knob, not `full`; the held line reads `PROVE: held — caught at <head>; personal install, no time to proof`.

**What the tree does today**, read at spec:

- **The refusal.** `native/src/installer/prove.rs`, `prove`, refuses `HEAD carries no checkwright.lock` at exit 2. Where `personal::is_personal` holds, its help is `personal_help`'s line.
- **What a personal proof would claim.** Every personal install is a kit set committing no process record (installer/SPEC.md §The personal install), so its clone keeps no queue and no state file. The claim is therefore the task claim. `CANON_KIT_TASK_LIST_GLOBS` defaults empty (`native/src/knobs/canon_kit.rs`), so a proof is held only where an adopter carrying canon-kit has named a task list. Otherwise it is `not held` at exit 1, with `not_held_help`'s line naming `--profile <derived>`, the profile `--personal` refuses.
- **Time to proof** runs from the oldest commit adding `checkwright.lock`, and a personal install has none. The manifest carries no time field (§The manifest).

## What changes

### (1) The personal proof's preconditions and the copy {design-bearing} {user-facing: operator direction 2026-10-24, lead-relayed, "copy the roster into prove's clone; refuse a destination reached through a link, and a roster path HEAD now tracks"}

In `prove.rs`, `prove`, where `HEAD` carries no `checkwright.lock` and `personal::is_personal(&root)` holds, the run is a **personal proof**. **Not yet applied.** Its manifest is the working tree's `checkwright.lock`, held to the three tests `HEAD`'s lock meets today: present, a known schema, a non-empty `version`. A failure refuses at exit 2 with today's causes, spelled of "this worktree's `checkwright.lock`". The help is `this worktree holds no personal install; run init --personal here, or prove from the worktree that holds one.`

After the detached checkout, and before the seam binary is resolved in the clone, the run copies the install in:

- **The set** is each `files` path of that manifest naming a regular file in the working tree, plus `checkwright.lock`. A recorded path off the tree is skipped, as the working tree lacks it too.
- **A path `HEAD` tracks refuses at exit 2**, asked of the clone's index in one batched `git ls-files -z --literal-pathspecs`. The cause is `HEAD tracks <n> path(s) this clone's personal install wrote`, listing them in the packer's bounded form (§The packer). The help is `re-run init --personal, which names the tracked paths an exclude cannot hide.`
- **A destination with a link or junction at any component below the clone's root refuses at exit 2**, read with `symlink_metadata` before the first copy. The cause is `<path> reaches through a link in HEAD's tree at <component>; prove writes nothing outside its clone`.
- **Each file is copied** with its bytes and, on a host with an executable bit, the bit, since the gate binary is on the set.
- **The clone takes the install's exclude block**, through `personal::write(&clone, <the set's files paths>)`. Gates reading the clone through git's ignore rules then see what they see in the working tree.

**The link test depends on `init-copy-follows-links`**, the sibling debt unit adding the same refusal to `init`'s `copy_in`. Where that unit lands first, this delta calls its helper. Where this delta lands first, it puts the helper in `native/src/installer/mod.rs` beside `seam_binary`, and that unit's `copy_in` calls it. One predicate either way.

The gate-binary precondition, `HEAD carries no gate binary at the path GATE_SDK_NATIVE_BIN names`, reads the copied seam and binary and keeps its wording.

### (2) The report, the verdict and the help on a personal proof {design-bearing} {user-facing: as delta 1, and the direction's held line and its "not-held help names the task-list knob, not full"}

Each **Not yet applied.**

- **The held line** is `PROVE: held — caught at <head>; personal install, no time to proof`. `Outcome::Held` carries the minutes as an option, none on a personal proof, and the time query is not run.
- **The not-held help**, on a personal proof, drops `not_held_help`'s profile clause. Where the manifest's `kits` carries `CANON_KIT_TASK_LIST_GLOBS`' owning kit, it reads `name a task list in CANON_KIT_TASK_LIST_GLOBS; init --help lists the --with-kit selection.` Otherwise it reads `add <owner> with init --personal --with-kit <owner>, then name a task list in CANON_KIT_TASK_LIST_GLOBS; init --help lists the --with-kit selection.` `<owner>` is the root `knobs::static_owner` returns for the knob, so no kit literal is spelled.
- **The note**, where the working tree differs from `HEAD`, reads `note: your worktree differs from HEAD; this run read HEAD and this clone's personal install alone.`
- `personal_help` and its line are deleted. Their one caller was the lockless refusal this delta replaces.
- **The usage text** (`USAGE` in `prove.rs`, the verb's `--help`) says the verb "clones HEAD into a scratch repository". Its sentence gains ", with a personal install copied in" after "scratch repository".

### (3) The SPEC sections {design-bearing}

Each **Not yet applied.**

- installer/SPEC.md §prove, **Preconditions**, the lock bullet becomes: "a `checkwright.lock` at `HEAD` carrying a `version` and a schema this build knows, else a refusal naming `init` and its commit. On a personal install (§The personal install) it is the working tree's lock, held to the same tests, and the run is a personal proof. A worktree holding no such lock is refused with a help naming `init --personal` there." The binary bullet ends ", or in the install a personal proof copies in".
- §prove, step 2: "Where your worktree differs from `HEAD`, a `note:` line says the run read `HEAD` alone." gains "On a personal proof the note says it read `HEAD` and this clone's personal install alone."
- §prove, **Honest limits**, "a gate reading a file git ignores reads that file's absence in the clone" gains ", except the files a personal proof copies in".
- §prove, **The run**, step 1 gains, after "checks `HEAD`'s commit out detached, …": "A personal proof then copies the install in: each recorded file the working tree holds and the lock, with their bytes and executable bit, and the install's exclude block written into the clone's own. A path `HEAD` tracks, or a destination reaching through a link in `HEAD`'s tree, refuses at exit 2, so nothing is written outside the clone or over `HEAD`'s content."
- §prove, step 5: "On a catch, the whole minutes …" gains "A personal proof has no install commit and reports none."
- §prove, the verdict list's **0** bullet gains "; on a personal proof `PROVE: held — caught at <head>; personal install, no time to proof`". The **1** bullet's help sentence gains "On a personal proof the help drops the profile, which `--personal` refuses, and names the knob, with the kit owning it where the lock lacks that kit."
- §prove, **Honest limits**, "It proves `HEAD`, never uncommitted work" becomes "It proves `HEAD`, never uncommitted work, with a personal install as the working tree holds it".
- §The personal install, "`prove` reads `HEAD`, so it refuses here, its help naming the personal install and the battery as the check of the working tree (§prove)." becomes "`prove` proves `HEAD` with the install copied into its clone (§prove)." **It runs at the inner two tiers only** keeps its sentence, since `prove` enforces nothing.
- §The personal install's honest limit "and its `prove` refuses as it would on any lockless `HEAD`" stays, being about a release built before `--personal`.

### (4) The personal arm's `prove` step {design-bearing}

`native/src/emit/installer_smoke/personal.rs`, step 4, and installer/SPEC.md §The consumer smoke's personal arm, step 4. **Not yet applied.** The step's sentence becomes:

> 4. **`prove`** exits 1, its last stdout line `PROVE: not held — this tree keeps no <queue-file> and no task list to claim a task done in`, since the minimum keeps no task list. Its `help:` line names `CANON_KIT_TASK_LIST_GLOBS` and no `--profile`, and its `configured` block names the minimum profile. `HEAD`, the porcelain and the exclude file are unchanged.

`<queue-file>` is the queue path the run resolves (§prove, step 3), the cause's first missing file.

### (5) The crate tests {mechanical}

In `prove.rs`'s `tests`, on the module's scratch `Tree`. **Not yet applied.**

- **The copy's two refusals:** a roster path the clone's index tracks, and a destination below a committed directory symlink, each refused with nothing written in the clone or outside it.
- **The held line**, both forms, held to the verdict grammar.
- **The personal not-held help**, with and without the owning kit in `kits`, naming the knob and no `--profile`.

### (6) The front door {mechanical} {user-facing: as delta 2}

Each **Not yet applied.**

- `docs/install.md` §On a project that does not adopt it: "`prove` reads `HEAD`, so it has nothing to read here." becomes "`prove` proves `HEAD` with this install copied into its scratch clone."
- `docs/install.md` §Prove it on your tree: "with the minutes since the install commit" becomes "with the minutes since the install commit, none for a personal install".

The two edits add 6 words to `docs/install.md`, which counts 1494 under `check-docs-page-length`'s own rule against its bound of 1500. With `hookspath-clobber`'s 20-word sentence in the same section the page comes to 1520, so a cut of at least 20 words is owed there, from prose restating an owner doc (the cut is that amendment's delta 5). The gate's verdict on the page with every such delta in place is the oracle.

## Producers and consumers

- **The personal proof** (delta 1). Producer: `prove`, when `HEAD`'s lock is absent and the personal marker present. Consumers: the copy, and delta 2's three readers of the proof's kind. Its red conditions are a missing or unreadable working-tree lock, a tracked roster path and a link destination, each exit 2.
- **The copied install** (delta 1). Producer: the copy. Consumers:
  - the seam binary resolution and the gates-directory read, which then find the install;
  - the report's `registry` row, reading the copied `gates.list`;
  - both battery runs;
  - the task claim's walk for a task list, reading the copied knob files.

  The clone's exclude block is read by git's ignore rules in the clone, as the working tree's is.
- **The held line's personal form** (delta 2). Producer: `Outcome::Held` with no minutes. Consumers: the adopter, delta 5's grammar test, and any reader matching the verdict's `PROVE: held — caught at <head>` prefix. That prefix is unchanged, so `prove`'s other oracles, the prove and companion arms and the PowerShell leg, all over tracked installs, do not move.
- **The personal not-held help** (delta 2). Consumers: the adopter and delta 4's step.
- **`personal_help`'s removal** (delta 2). Its one caller and the personal arm's step 4 are its readers, and delta 4 moves the step. Probe: `git grep -n -F "install is personal and commits nothing"` names `prove.rs` alone, and `git grep -n "this clone's install is personal"` adds the smoke's matcher.
- **Sibling units.** The copy reads the manifest's `files`, never the exclude block's lines, so `personal-collapse-hides-files` changes no act here. `personal-worktree-shared-block`'s keyed blocks change which block `personal::write` writes in the clone, the clone's main worktree's, and not whether. The scope order lands this unit after both.

**Inferred, cannot run before build:** that the copy and the link refusal behave on native Windows as on Linux, a junction reading as a link through `symlink_metadata` — no Windows host is on this machine. The witness is delta 4's step on the Windows install-smoke legs. Those run at `init-copy-follows-links`' conditional mid-iteration push where that push happens, and at the closing push otherwise.

## Existing sections updated

- `native/src/installer/prove.rs` (`prove`, `Outcome::Held`, `not_held_help`, `report`, `USAGE`, `personal_help` deleted, and the tests) (deltas 1, 2 and 5).
- `native/src/installer/mod.rs`, the link helper, where this delta lands before `init-copy-follows-links` (delta 1).
- installer/SPEC.md §prove and §The personal install (delta 3).
- `native/src/emit/installer_smoke/personal.rs` and installer/SPEC.md §The consumer smoke, the personal arm's step 4 (delta 4).
- `docs/install.md` §On a project that does not adopt it and §Prove it on your tree (delta 6).
- `docs/llms-full.txt`, regenerated with `bash gate-sdk/bin/run-gates.sh --emit llms --write` (delta 6).
- `docs/installer/SPEC.md`, the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (deltas 3 and 4).

The verb's surfaces were found by `grep -n "fn \|carries no checkwright.lock\|personal\|time to proof\|differs from" native/src/installer/prove.rs`. The doc surfaces by `git grep -n "nothing to read here\|minutes since the install commit" -- docs installer`.

## Retired spellings

- `this clone's install is personal and commits nothing` — `personal_help`'s line, deleted (delta 2).

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The local oracle** — `bash gate-sdk/bin/build-native.sh`, the battery, the crate's tests and the `installer_smoke` suite green.
- [ ] **The Done move** lands before the drain stage, on the local oracle. Delta 4's step on the Windows legs is read at whichever push runs them first. Where that is the mid-iteration push, the move waits for its runs watched green; where it is the closing push, a red there is filed at close.
