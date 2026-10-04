# SPEC amendment: toplevel-refusal

Paired with the queue entry `toplevel-refusal-fail-open`. The crate's crosser folds every non-zero `git rev-parse --show-toplevel` into the absent answer, so a repository git refuses (a `safe.directory` ownership refusal, broken repository metadata) reads as *outside a work tree* at every caller of `walk::toplevel_opt` and `walk::toplevel_in_opt`, and as the wrong cause, `not a git repository`, at every caller of `walk::toplevel`, `walk::toplevel_in` and `fresh::toplevel`. Canon-kit's tracked-set finder alone tells the two apart today, through a `.git`-entry mark private to `native/src/spec.rs`. This amendment moves that mark into the crosser and makes the refusal its third answer.

## What changes

### (1) The crosser's third refusal: a marked anchor git refuses {design-bearing} {user-facing: the queue entry's deliverable, "a third refusal kept apart in the crosser … each caller's verdict re-read" — operator selection, direction 2026-10-04}

`native/src/walk.rs` takes over the repository mark from `native/src/spec.rs`, unchanged in what it reads: a non-empty `GIT_DIR` names one outright; otherwise a `.git` entry of any type (directory, gitdir file, symlink) at the anchor or above, the ascent stopping where `GIT_CEILING_DIRECTORIES` stops git's own. The function stays private to `walk.rs`, since its only reader becomes `toplevel_args`.

`toplevel_args` classifies a non-zero exit through it. The **anchor** is the working directory for the bare form and the `-C` directory, absolutized against `walk::cwd()` through `walk::abs_against`, for the `-C` form.

- **Marked anchor → the refused answer**, carried in the `Err` arm with its own sentence: git answers no work tree at `<anchor>` though `<mark>` marks a repository, a refused or broken repository is not read as outside one, and `git -C <anchor> status` prints git's reason. The sentence is the one `spec.rs` prints today, generalized from *the tracked set* to the crosser's answer, and it names no caller.
- **Unmarked anchor → the absent answer**, `Ok(None)`, unchanged.
- **An anchor that is not an existing directory** (a `-C` operand naming nothing; git exits 128 with `cannot change to`) stays the absent answer, unchanged, without consulting the mark. A lexical ascent from a missing directory would find an enclosing repository's mark and blame a refusal that is not there.

The signature stays `Result<Option<String>, String>`. The refusal rides `Err` beside the dead-`git` message because both mean *the tree cannot be answered*, so every caller that propagates `Err` fails closed with no edit, and a third variant would force a match arm on every caller to say the same thing. Telling a refusal from *outside a work tree* by git's stderr is refused: git 2.55 answers a repository with an invalid `HEAD` with exactly the message and exit status, 128, it gives a directory in no repository, and its text is localized.

`toplevel()` and `toplevel_in()` keep folding the absent answer into `not a git repository`; the refusal reaches their callers as its own sentence rather than that one.

**The sibling's fix lands in the moved function either way.** `spec-mark-symlink-ascent` makes the mark's ascent find what git's physical discovery finds. If it rides the same build batch, it lands after this delta, in `walk.rs`'s mark. If it rides a later one, it lands there the same way, with its test in walk.rs's tests rather than spec.rs's.

**Replacement text** for gate-sdk/SPEC.md §The crate's crosser, the `walk::toplevel()` bullet. *Not yet applied.*

> - **`walk::toplevel()`** is the crate's only `git rev-parse --show-toplevel` spawn, its answer passed through `normalize_abs`. Git-for-Windows answers in Windows spelling even to a Rust process, so this producer crosses for the same reason `cwd()` does. It has **three answers besides a toplevel, kept apart because callers report them differently**. A dead `git` is `proc::run`'s own spawn message, propagated. A non-zero exit at an anchor nothing marks as a repository is **outside a work tree**, the absent answer. A non-zero exit at a **marked** anchor is a **refused repository**, an error naming the anchor, the mark and `git -C <anchor> status` as the way to git's reason, since a refused repository read as no repository fails every caller open. The **mark** is a non-empty `GIT_DIR`, else a `.git` entry at the anchor or above, the ascent stopping where `GIT_CEILING_DIRECTORIES` stops git's own; a `-C` anchor that is no directory is never marked. `toplevel_opt()` returns these as `Result<Option<String>, String>`, both errors in the error arm, and `toplevel()` folds the absent answer into `not a git repository`. `fresh::toplevel()` suffixes its own sentence, `— the emitter anchor cannot be resolved`.
>
>   **Honest limit:** the mark ascends lexically and does not stop at a filesystem boundary where git's discovery does without `GIT_DISCOVERY_ACROSS_FILESYSTEM`, so a non-repository mounted inside a repository's tree reads as refused rather than absent. git's stderr cannot tell a broken repository from none, so the mark is the only discriminator the crosser has.

### (2) Canon-kit's tracked-set finder reads the crosser's refusal {mechanical}

`Tracked::at` in `native/src/spec.rs` drops its own mark check and propagates `walk::toplevel_in_opt(root)?`: `None` is the walk, `Err` exits 2. Its message now comes from the crosser (delta 1), so the finder's existing test `a_repository_git_cannot_answer_for_is_refused_rather_than_walked` asserts the crosser's sentence rather than `spec.rs`'s, and keeps its three cases (unmarked walks, marked refuses, a ceiling hides the mark).

**Replacement text** for canon-kit/SPEC.md §The shared spec adapters, the *Outside a work tree the walk stands* sub-bullet. *Not yet applied.*

> - **Outside a work tree the walk stands**, as in a bespoke test's `mktemp -d` sandbox, and a repository git refuses exits 2 rather than walking, since the walk would grade untracked files with no notice. Both are the crosser's answers at the scan root (gate-sdk/SPEC.md §The crate's crosser), which owns what marks a repository. A scan root inside an enclosing repository's ignored directory has an empty tracked set, so a sandbox sits outside every repository or initializes its own.

### (3) Each caller's verdict on the refusal {design-bearing} {user-facing: the queue entry's deliverable, "each caller's verdict re-read", and gate-sdk/SPEC.md's fail-closed contract — operator selection, direction 2026-10-04}

**The rule: a caller degrades on the absent answer only.** A caller that turns the crosser's error into a fallback, an empty result or its own sentence has to tell the two error answers apart from the absent one, and the non-`_opt` forms fold the absent answer into an error, so such a caller cannot. Every such site moves to the `_opt` form. It keeps its own fallback or sentence for `Ok(None)`, and it passes the crosser's `Err` through, as an exit 2, a refusal or an error naming the crosser's sentence. Two surfaces stay soft on every answer because they must never block: the statusline hook and the update notice.

The roster is the call-site survey this stage bought. The probe was `grep -rn "toplevel_opt\|toplevel_in_opt\|walk::toplevel()\|walk::toplevel_in(\|fresh::toplevel\|toplevel()\|toplevel_in(" native/src --include=*.rs`, run over native/src minus walk.rs's definitions at `c33b3efb6`, with the module-local aliases (`emit/trajectory.rs`, `gates/kit_registration.rs`, `history::stamp_lines`, `installer::repo_root`) followed to their callers. Each swallowing or rewriting row was re-read at its site. Line numbers are at that revision.

**Edited by this delta.** Each moves to the `_opt` form under the rule:

| Site | Surface | Today in a refused repository | After |
|---|---|---|---|
| `queue.rs:638` `transitions` | queue-history arm; the `demote` and `thaw` verbs | `.ok().flatten()` turns it into no transitions: the arm prints the slug alone, and `demote` reads no prior place | the refusal, exit 2 |
| `installer/mod.rs:217` `repo_root` | `init`, `diff` and `uninstall` (and `update`, which falls through to `init`) | refuses as *not inside a git work tree*, `init`'s help saying `Run 'git init' first` | `repo_root` returns `Result<Option<PathBuf>, String>`; each verb refuses at exit 2 with the crosser's sentence as the cause and a help line naming repair of git's access, never `git init`. `update`'s precondition falls through to `init` on either answer, as now |
| `gates/stage_evidence.rs:99` | `check-stage-evidence` | inert (passes), and a dead `git` is inert too | absent stays inert (a); any error exits 2 |
| `gates/kit_roots_dialect.rs:239` | `check-kit-roots-dialect` | anchors on the cwd | absent anchors on the cwd; the refusal exits 2 |
| `emit/upgrade_smoke.rs:240` `resolve_repo` | `--upgrade-smoke` | `unwrap_or(None)` refuses as `not a git repository: <unset>` | the crosser's error |
| `emit/pack_installer.rs:495`, `:505` `resolve_root` | `--emit pack-installer`, with and without `--root` | refuses as *not inside a git work tree* | absent keeps that sentence; the refusal passes the crosser's |
| `gates/install_disposition.rs:156`, `gates/kit_ref_liveness.rs:151`, `gates/fence_command_head.rs:43`, `gates/docs_cmd.rs:34`, `gates/fence_run.rs:167`, `gates/kit_enum.rs:44`, `gates/kit_registration.rs:11`, `gates/template_copy_parity.rs:225` | the gates of the same names | exit 2 under each site's own `not a git repository …` sentence, the crosser's text discarded | absent keeps the site's sentence; the refusal passes the crosser's |
| `emit/roadmap_lag.rs:107` `locate` | the roadmap-lag arm | degrades with `<file> is in no git work tree` on stderr | the same degrade, exit 0 with no row, naming the crosser's sentence as the reason |

**Closed by delta 1 alone, no edit.** These already propagate the error arm, so the refusal reaches them as an error. The `_opt` callers among them read `Ok(None)` today and now fail closed: `emit/close_surfaces.rs:23`, `emit/pub_index.rs:150`, `emit/ruling_staleness.rs:391` (its cwd fallback stays for the absent answer), `spec.rs:233` (delta 2), `emit/foreign_run.rs:686`, `emit/mod.rs:125` (the index arms' target resolver), `emit/git_hook.rs:143` and `:211`, `emit/git_hooks.rs:133`, `emit/queue_verbs.rs:137` (the verb's post-check, after its write), `emit/md_index.rs:131`, `emit/file_survey.rs:78` (the survey, gap and friction anchors, which today write at the cwd), `gates/memory_off.rs:18` (`check-memory-off`, which today derives no dir and reads clean), `gates/brevity.rs:130`, `gates/battery_roster.rs:88` and `gates/smoke_entry_guard.rs:18`. The non-`_opt` callers already err and keep the crosser's text: `fresh.rs:21` (its suffix stays true, since the anchor cannot be resolved), `emit/csmoke.rs:120`, `emit/installer_smoke/roster.rs:122`, `emit/entry_history.rs:45`, `queue.rs:670` and `gates/gate_tamper.rs:226`.

**Soft on every answer, no edit.**

- `hook/statusline.rs:232` and `emit/update_notice.rs:62` must never block.
- `installer/doctor.rs:278` reaches its diagnosis rather than refusing (installer/SPEC.md §doctor), so it keeps its cwd fallback.
- drift-kit's three readouts register no gate and degrade fail-visibly to `n/a (<reason>)` (drift-kit/SPEC.md §The published-evidence extractor): `emit/trajectory.rs:20`, `emit/queue_flow.rs:67` and `history.rs:100`, the stage-economics meter and its KPI. **Honest limit:** their reason names the symptom (no committed state file, nothing to read), not the refusal.

Test-only: `main.rs:305`, `gates/memory_off.rs:271`.

**Replacement text** for the canonical passages this delta moves. *Not yet applied.*

- lifecycle-kit/SPEC.md §check-stage-evidence, inertness (a):

  > - **(a)** git answers that the state file's directory lies in no work tree, or in a different one from the configured surfaces, such as a vendored tree under test or a sandbox. A repository git refuses, or a dead `git`, is no such answer and exits 2 (gate-sdk/SPEC.md §The crate's crosser);

- queue-kit/SPEC.md §The queue-history arm, the **Exit** bullet:

  > - **Exit** 0 with the report, and 2 on a usage error, a refused slug, or a repository git refuses (gate-sdk/SPEC.md §The crate's crosser). There is no 1, on entry-history's ground. The declared reads are the queue file and the four section knobs a place reads, `QUEUE_KIT_DONE_SECTION` among them.

- queue-kit/SPEC.md §The roadmap-lag arm, the **Degradations** bullet:

  > - **Degradations** are the retired set's (§The queue-edges arm). With no git work tree, a repository git refuses, or no `git`, no slice can be confirmed, so the arm prints no row and says why on stderr. A shallow clone under-claims.

- installer/SPEC.md §update, the refusal roster in its last paragraph: `(not a git work tree, a repository git refuses, an unknown schema, …)`, and the sentence before it reads *Outside a git work tree, or in a repository git refuses, it falls through to `init`'s refusal*.

### (4) The crosser's own tests {mechanical}

Crate tests in `native/src/walk.rs` over a sandbox outside every repository (`std::env::temp_dir()`, under the `knobenv` lock for the two environment variables):

- an unmarked directory: `toplevel_in_opt` answers `Ok(None)`;
- a `.git` file naming an absent gitdir (broken metadata, the stand-in for a `safe.directory` refusal, which a test cannot produce without changing a directory's owner): `toplevel_in_opt` errs, the sentence naming the anchor and the mark, and `toplevel_in` errs with the same sentence rather than `not a git repository`;
- the same tree under `GIT_CEILING_DIRECTORIES` set to the sandbox: `Ok(None)`;
- a `-C` anchor that does not exist inside the marked tree: `Ok(None)`;
- a non-empty `GIT_DIR` naming nothing, from an unmarked directory: the refusal.

### (5) The release declaration {mechanical}

The landing commit appends to `.workflow/release-declarations.md` (gate-sdk/SPEC.md §upgrade-smoke):

- **New and tightened gates** — one bullet led by `check-stage-evidence`, `check-memory-off` and `check-kit-roots-dialect`, the gates whose verdict in a refused repository becomes exit 2 where it was inert, clean or anchored on the cwd, and naming the message-only members of delta 3's first table. The remedy is to repair git's access (`git -C <dir> status` prints git's reason). The front end's own `cd` already refuses such a tree, so the change reaches a run through the binary door.
- **Behavior changes** — one bullet for the emit arms, verbs and installer verbs that now refuse rather than read a refused repository as none: the queue-history arm with `demote` and `thaw`, the index and pub-index arms, `--emit file-survey` with its gap and friction anchors, `--emit ruling-staleness`, and `checkwright init`, `diff` and `uninstall`, whose help no longer says `git init`.

## Producers and consumers

**The refused answer (deltas 1 and 3), a new state of an existing interface.**

- *Producer:* `walk::toplevel_args`, on a non-zero `git rev-parse --show-toplevel` at an existing anchor directory the mark finds. No configuration enables it; every deployed configuration reaches it, wherever git refuses a repository: a CI container whose user does not own the checkout is the common `safe.directory` case.
- *Consumers:* every caller of the four `walk::toplevel*` forms and of `fresh::toplevel`, through the error arm they already handle, per delta 3's roster. No new name is minted. The mark stays private to `walk.rs` and the crosser's public API is unchanged, so gate-sdk/SPEC.md §check-path-dialect's list of crosser calls that are not occurrences needs no entry.
- *Fields:* the sentence names the anchor and the mark, the person reading the refusal uses both to find the repository at fault, and `git -C <anchor> status` is the remedy. Nothing parses it. delta 4 asserts the anchor and the mark, and spec.rs's moved test asserts the mark phrase.

**The mark's move (deltas 1 and 2).** Its one reader becomes `toplevel_args`. `Tracked::at`, its only reader before, now reads the crosser's answer instead. The reading is not narrowed: the finder refused at a marked root before and still does, and the anchor-must-exist clause cannot reach it, since the finder walks the scan root before it reads the tracked set, and `walk_pruned` errs on a directory it cannot read.

**Obliging every caller (delta 3, point 6).** The corpus is the probe's roster, and each member's satisfying value is its row: an edit under the rule, closed by delta 1, soft on every answer with its ground, or test-only. No member is left without a value.

**No corpus narrows.** No reader's red condition moves toward *finding none*. Each verdict moved here goes from pass or skip to exit 2, which is the fail-closed direction.

## Existing sections updated

The roster's probe: `grep -rn "two refusals\|outside a work tree\|toplevel_opt\|not a git repository\|repository_mark\|marks a repository" --include=*.md .` over the tracked tree at `c33b3efb6`, plus a read of each surface delta 3's edited sites own.

- gate-sdk/SPEC.md §The crate's crosser, the `walk::toplevel()` bullet (delta 1).
- canon-kit/SPEC.md §The shared spec adapters, the *Outside a work tree the walk stands* sub-bullet (delta 2).
- lifecycle-kit/SPEC.md §check-stage-evidence, inertness (a) (delta 3).
- queue-kit/SPEC.md §The queue-history arm, Exit; §The roadmap-lag arm, Degradations (delta 3).
- installer/SPEC.md §update, the fall-through sentence and the refusal roster (delta 3).
- `.workflow/release-declarations.md` (delta 5).
- The on-site SPEC mirrors `docs/gate-sdk/SPEC.md`, `docs/canon-kit/SPEC.md`, `docs/lifecycle-kit/SPEC.md`, `docs/queue-kit/SPEC.md` and `docs/installer/SPEC.md`, regenerated with `--emit docs-mirror --write` (all deltas).

## Retired spellings

- None — no delta retires a name: the mark moves module under its own name, and the crosser's public forms and signatures are unchanged.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Battery and fixtures** — `bash gate-sdk/bin/build-native.sh`, the crate tests, the full battery, and the fixture suite of every kit whose gates delta 3 edits.
- [ ] **Entry moved** — `toplevel-refusal-fail-open` moved to Done in the merge commit, before the drain stage.
