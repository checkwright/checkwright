# SPEC amendment: init-foreign-hooks

Pairs `init-commit-runs-adopter-hooks`. `init` makes one commit, and that commit runs every commit hook the adopter's clone already runs, over every vendored file. An operator-seat install stopped there. The adopter's own amendment-pairing gate read canon-kit's shipped amendment skeleton as an unpaired amendment, so the commit failed. The run ended with the vendored files staged and said nothing of the setup still owed. This amendment settles three things: what `init` does about a hook it does not own at its commit, whether the skeleton keeps a name the amendment glob matches, and what the install docs say.

**What the tree does today.** Probed at spec on git 2.55 in scratch repositories, and read off `native/src/installer/`:

- **`init` commits through every hook the clone runs.** `git_commit` (`native/src/installer/mod.rs`) runs `git commit -q -m <message>` with no `--no-verify`. A live `pre-commit` in the clone's default hooks directory that exits 1 fails that call at exit 1, and its stderr is carried in git's account.
- **A failed commit ends the run before the follow-up block.** `commit_or_refuse` returns its refusal through `?` before `follow_up` runs (`native/src/installer/init.rs`). So the `next:` block and its `then:` line are never printed. If the adopter then commits by hand, a re-run takes the no-op exit, which prints no block either. From then on, the commands that finish the setup are reachable only from the docs.
- **The refusal line does not name its cause.** It reads `the commit failed — the files are staged; commit them yourself to finish the install`, printed after git's account.
- **The skeleton is the one payload file that matches the default amendment glob.** The payload withholds each kit's `SPEC.md` (the default of `GATE_SDK_PAYLOAD_WITHHOLD`, gate-sdk/SPEC.md §Layout and configuration), but canon-kit/templates/SPEC-amendment.md ships. This tree's readers of the glob each exclude it:
  - canon-kit's finders prune `templates/`;
  - assertion C of `check-stage-entry` excludes `templates/` paths;
  - a unit test in `native/src/emit/kpi/amendment_age.rs` asserts the skeleton is no amendment.

  A reader of the same convention in an adopter's tree carries no such exclusion.

## What changes

### (1) A failed commit names the hook that judged it {design-bearing} {user-facing: oracle decision 2026-10-25, lead-relayed (not a ruling), option (c): commit through the adopter's hooks and, on failure, name them with the remedy; never `--no-verify`, never an up-front refusal; attribution: lead decision 2026-10-25 (not operator direction) — a read that errs or establishes nothing leaves the line as it is today, and the line never names a party the run cannot establish judged the commit, a gate's refusal never attributed to the adopter's hooks}

`commit_or_refuse` (`native/src/installer/mod.rs`) is the one commit path for three commits: `init`'s vendoring commit, its no-op residue commit and `uninstall`'s removal commit. The change lands there, so it reaches all three. **Not yet applied.**

- **After git's account, a failed commit names the hook that refused it, where git's own trace establishes one.** A live hook is no evidence: on git 2.55 a missing identity stops the commit before any hook runs, and a refused signature stops it after every hook passed, so naming the directory's live hooks would blame hooks that judged nothing. `git_commit` therefore spawns the commit with `GIT_TRACE2_EVENT` naming a fresh scratch file, set on that spawn alone (`proc::run_merged_in` carries an environment addition), and removes the file once read. A hook git ran is a `child_start` event whose `child_class` is `hook`, under the commit's own session id: a hook's nested git writes under a child id `<sid>/…`, which is not read. It refused when the `child_exit` matching its `child_id` carries a non-zero `code`. The hooks read are `pre-commit`, `prepare-commit-msg` and `commit-msg`, whose non-zero exit stops the commit at once; another hook's non-zero exit may be one git ignores, so it names nothing. The trace is read with the crate's `serde_json`.
- **The directory.** It is read the way gate-sdk/SPEC.md §install-hooks reads the prior hooks directory: `core.hooksPath`'s effective value, resolved physically, or else git's default. The reader is `install_hooks::prior` (`native/src/emit/install_hooks.rs`), made callable from the installer rather than copied.
- **The line stays as it is today, git's account standing, wherever the reads establish no adopter's hook.** That is the case when:
  - the trace records no refusing hook, because git stopped outside its hooks, or a git without trace2 wrote no file;
  - the trace or the directory read fails;
  - the directory is the gate hooks directory. On an opted-in clone a served hook's launcher runs the adopter's chained hook and then the gates in the one process git traces as one hook, so the trace cannot tell them apart. Where the chained hook refused, the launcher's own line in git's account already names it (gate-sdk/SPEC.md §git-hook). No chain record is read, so `uninstall`, which removes the gate hooks directory and its record before its commit while `core.hooksPath` still names it, needs no case of its own: git runs no hook from the removed directory, the trace records none, and the line stays as it is today.
- **The line and its help.** With a refusing hook established, the refusal reads `<verb>: the commit failed, and your <hook> hook in <dir> judged it — the files are staged`. Its `help:` says that hook judges every vendored file. It names the kit directories, from the kit set the run vendored, and tells the adopter to let that hook pass them, since they are a vendored dependency, then commit the staged files. It also offers `--no-commit` as the way to make the commit yourself.
- **`init` never commits past a hook.** No `--no-verify` is passed, and no flag adds one. A hook that judges the whole tree meets the vendored files again at the adopter's next commit, so a bypass would only move the refusal to a commit `init` cannot report on. A `commit-msg` hook refusing `init`'s subject is the same case, and `--no-commit` is how an adopter commits under their own message convention.

### (2) A failed vendoring commit still hands over the setup {design-bearing} {user-facing: oracle decision 2026-10-25, lead-relayed (not a ruling), option (c1)}

When `init`'s vendoring commit fails, the run prints the follow-up block and its `then:` line after the refusal, exactly as a successful run prints them, and still exits 1. The block's grammar is unchanged, so the consumer smoke's follow-up arm reads it as it does today. The no-op residue commit and `uninstall` print no block. **Not yet applied.**

### (3) The amendment skeleton leaves the amendment glob {mechanical} {user-facing: oracle decision 2026-10-25, lead-relayed (not a ruling), rename to `canon-kit/templates/amendment.md`, routed through the upgrade contract's release declaration}

canon-kit/templates/SPEC-amendment.md is renamed `canon-kit/templates/amendment.md`, a name no default amendment glob matches, and every citation of the old path moves with it. The `templates/` prunes stay, since they spare an adopter's own skeletons as well as this one. The rename and this amendment's deletion land in one commit. A path citation of the old name in this file resolves only while the old file exists (canon-kit/SPEC.md §check-amendment-queue, arm (e)). **Not yet applied.**

- canon-kit/README.md, step 2: "copy `templates/amendment.md` when a designed-but-unimplemented change needs a home". The rest of the step is unchanged.
- canon-kit/SPEC.md §templates/: "`amendment.md` — the amendment skeleton, named outside the amendment glob so a copy is an amendment and the skeleton never is: delta sections plus the Definition-of-Done checklist (…)". In §check-amendment-update-target, three sites cite the old path: the section's intro ("shown to the author in `templates/amendment.md`"), the *No new knob* paragraph, which names the skeleton by role ("a shipped amendment skeleton"), and the *roster completeness* paragraph's parenthetical, which cites `templates/amendment.md`.
- lifecycle-kit/SPEC.md §check-stage-entry, *C's signal*: "A shipped amendment skeleton is a copyable stub", named by role. The `templates/` exclusion is unchanged.
- lifecycle-kit/templates/stages/align.md cites `canon-kit/templates/amendment.md`.
- The skeleton's own comment keeps its naming instruction for the copy (`SPEC-<feature>.md`).
- The assertion in `native/src/emit/kpi/amendment_age.rs` names `canon-kit/templates/amendment.md`. It keeps a second assertion over a `templates/` path the glob matches, since an adopter's own skeleton still meets the prune, under a basename other than the retired one.
- Kept as written, as deliberate mentions:
  - the two fixtures in `lifecycle-kit/gate-tests/check-stage-entry.test.sh`, which plant an adopter-shaped skeleton under `templates/` to hold the exclusion;
  - the test in `native/src/emit/docs_mirror.rs`, whose subject is an amendment basename under a kit rather than the skeleton;
  - the dated release posts, which are history.
- **The upgrade route.** `init` never deletes a file a release stopped shipping (installer/SPEC.md §What init seeds). So an upgraded tree keeps the old skeleton beside the new one, and the old name still meets an adopter's amendment glob. §The upgrade contract's phase B tells an adopter about a moved surface only through a red gate or the release note, and no gate reds here. So the release declaration surface (`.workflow/release-declarations.md`) gains a `behavior` bullet in the note's grammar, landing in the same commit as the rename. Its lead token is the bolded new path. It says the skeleton was renamed out of the amendment glob, and that an upgraded tree keeps canon-kit/templates/SPEC-amendment.md until the adopter deletes it, which is safe whenever it is unedited. The release tag at close carries it into the note.

### (4) The install docs say whose hooks judge the commit {mechanical} {user-facing: as deltas 1 and 2}

Each **Not yet applied.**

- `docs/install.md`, the paragraph above *Pick your route*: `init`'s one commit runs your own commit hooks, so a hook that judges every file must let the kit directories pass.
- `docs/install.md`, the WSL bullet's second sentence becomes "The pre-commit hook the follow-up's `--install-hooks` places runs the Linux gate binary, so commit to that repository from WSL too: Git for Windows cannot start it." The current wording reads as though the install places the hook, and `init` places none (installer/SPEC.md §init).
- **The page budget.** The page stands at 1499 words against `check-docs-page-length`'s bound of 1500. Every word added is cut from prose on the same page that restates an owner doc, and that gate's verdict is the oracle.
- `installer/README.md`, the paragraph opening "`init` vendors the selected profile's kit directories": the one commit runs the clone's own hooks, and a failed commit leaves the files staged with the setup commands printed.
- The release declaration surface gains a `behavior` bullet for deltas 1 and 2: a failed install commit names the hook that judged it and prints the follow-up block.

### (5) The SPEC sections {design-bearing}

Each **Not yet applied.**

- installer/SPEC.md §init, *A failed commit relays git's own account* is rewritten:

  > **A failed commit relays git's own account, names the hook that judged it, and still hands over the setup.** The files stay staged and `init` exits 1, first printing what git and any hook it ran wrote. Where git's trace of the commit records a `pre-commit`, `prepare-commit-msg` or `commit-msg` hook exiting non-zero, from a directory read as gate-sdk/SPEC.md §install-hooks reads a prior hooks directory, the refusal names that hook and directory. Its help line names the kit directories that hook must let pass, and `--no-commit`. A live hook alone names nothing, since git can stop before its hooks run or after they pass. Neither does a hook in the gate hooks directory, where the trace cannot tell a chained hook from the gates; there a served hook's launcher writes its own line into git's account when the chained hook refused. It never passes `--no-verify`: a hook judging the whole tree meets the vendored files at the next commit anyway, and a bypass would move its refusal to where `init` cannot report it. Where the reads establish no refusing hook, or fail, git's account carries the cause, a missing identity among them. The follow-up block and its `then:` line print after the refusal, so the setup does not wait on a re-run.

- installer/SPEC.md §init, *The commit passes the hooks it vendors*, gains its counterpart clause: the commit passes the clone's own hooks too, the adopter's included.
- installer/SPEC.md §uninstall, *A failed commit leaves the removal staged and relays git's account, as `init`'s does* covers delta 1's line, under the same reads. It prints no follow-up block.

## Producers and consumers

- **The hooks line** (delta 1).
  - Producer: `commit_or_refuse`, on a failed commit, reading the commit's trace and the hooks directory after the failure.
  - Consumers: the adopter, through stderr, and the unit test below.
  - Fields: the directory and the hook name, which the adopter reads to find the hook, and the kit directories, which the adopter reads to scope the hook's exclusion. Each field has a reader.
- **The commit's trace** (delta 1).
  - Producer: git, writing `GIT_TRACE2_EVENT`'s file for the one spawned commit.
  - Consumer: `commit_or_refuse`, which reads it once and removes it on every path. A missing or unreadable file is no evidence, never a refusal of its own.
- **The follow-up block on failure** (delta 2).
  - Producer: `init`'s failure path for the vendoring commit.
  - Consumers: the adopter. The consumer smoke's follow-up arm reads only a successful run's output, so it is unchanged, as is the stated grammar in installer/SPEC.md §init.
- **The renamed skeleton** (delta 3).
  - Producer: the payload, at every install.
  - Consumers:
    - the adopter copying it, through canon-kit's README;
    - canon-kit's finders and lifecycle-kit's assertion C, which prune `templates/` and so read neither name;
    - `check-amendment-update-target` and `check-amendment-retired-spelling`, whose corpus is that same finder's.
  - No reader reds on a zero count of skeletons.
  - The surface ratchet holds one row per governed surface, keyed by path (context-kit/SPEC.md §The surface ratchet), so the row moves with the file.
  - The release declaration's `behavior` bullet is read by the note's composition at close and by `check-release-bump`'s section count.
- **The witness.** A crate unit test runs `init` over a scratch repository whose default hooks directory holds a live `pre-commit` that exits 1. It asserts that `init` exits 1, the vendored files are staged, stderr names the directory and `pre-commit`, and stdout carries the `next:` block. Two further cases hold the attribution: a `pre-commit` that exits 0 under a repository-local `commit.gpgSign` whose `gpg.program` is `false`, where the commit fails after its hooks pass, and a `core.hooksPath` naming the gate hooks directory over a `pre-commit` that exits 1. Each asserts the refusal line as it is today, naming no hook. The test sets its environment on the spawned `git` alone, never process-wide, for the reason gate-sdk/SPEC.md §install-hooks gives about `GIT_CONFIG_GLOBAL`. It runs on unix only, because the planted hook is a sh script; the Windows legs already commit through hooks of their own.

## Existing sections updated

- `native/src/installer/mod.rs`: `commit_or_refuse` and `git_commit` (delta 1).
- `native/src/emit/install_hooks.rs`: `prior` and its `Prior`, made callable from the installer (delta 1).
- `native/src/installer/init.rs`: the failure path and its unit tests (deltas 1 and 2).
- `installer/SPEC.md` §init and §uninstall (deltas 1, 2 and 5).
- `docs/install.md` and `installer/README.md` (delta 4).
- `.workflow/release-declarations.md`: two `behavior` bullets (deltas 3 and 4).
- `canon-kit/templates/SPEC-amendment.md`: renamed (delta 3).
- `canon-kit/README.md`: step 2 (delta 3).
- `canon-kit/SPEC.md`: §templates/ and §check-amendment-update-target (delta 3).
- `lifecycle-kit/SPEC.md`: §check-stage-entry (delta 3).
- `lifecycle-kit/templates/stages/align.md`: the template citation (delta 3).
- `native/src/emit/kpi/amendment_age.rs`: its unit test (delta 3).
- `lifecycle-kit/gate-tests/check-stage-entry.test.sh`: kept as written (delta 3).
- `native/src/emit/docs_mirror.rs`: its unit test, kept as written (delta 3).
- `docs/posts/2026-07-25-checkwright-v0-15-0.md`: dated history, kept as written (delta 3).
- `docs/posts/2026-09-26-checkwright-v0-26-0.md`: dated history, kept as written (delta 3).
- `TASK-QUEUE.md`: the paired entry's body, which leaves with its Done move (delta 3).
- `.workflow/surface-ceiling.txt`: the skeleton's row, moved to its new path (delta 3).
- `docs/installer/SPEC.md`: the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`, as is every mirror below (all deltas).
- `docs/installer/README.md`: the mirror (delta 4).
- `docs/canon-kit/SPEC.md`: the mirror (delta 3).
- `docs/canon-kit/README.md`: the mirror (delta 3).
- `docs/lifecycle-kit/SPEC.md`: the mirror (delta 3).
- `docs/llms-full.txt`: regenerated with `bash gate-sdk/bin/run-gates.sh --emit llms --write` (delta 4).

**How these were found.** The skeleton's readers by `git grep -n "SPEC-amendment"` over the tracked tree; the commit path's callers by `grep -n "commit_or_refuse\|COMMIT_FAILED" native/src/installer/*.rs`; the doc surfaces by `git grep -n -i "one commit\|hook" docs/install.md installer/README.md`.

## Retired spellings

- `SPEC-amendment` — the skeleton's basename, retired by delta 3; its deliberate survivors are named in the roster above.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The local oracle** — `bash gate-sdk/bin/build-native.sh`, the battery, the crate's tests, and the fixture suites of canon-kit and lifecycle-kit are green.
- [ ] **The release** — the entry's push need is the release tag at close, since the fix reaches an operator-seat install only on a published binary. The Done move lands in the merge commit, before the drain stage. The tag is a matter of close's release policy, not an observation the move waits on.
