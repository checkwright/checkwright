# SPEC amendment: nested-repo-refusal

Paired with the queue entry `nested-broken-repo-toplevel`. git's discovery skips a nested `.git` it cannot read and answers `rev-parse --show-toplevel` with the enclosing repository's toplevel at exit 0, so the crate's crosser hands every caller the outer repository for a tree that is not its own. Measured on git 2.55 under an initialized outer repository, three inner shapes answer the outer toplevel at exit 0: a `.git` directory with a garbage `HEAD`, an empty `.git` directory, and a `.git` symlink naming nothing. Three more already exit 128 and reach the crosser's existing refused answer: a garbage gitfile, a gitfile naming a missing directory, and an empty `.git` file. A valid inner repository answers its own toplevel. In the skipped shapes `git -C <inner> ls-files` lists nothing at exit 0, so canon-kit's tracked-set finder keeps no file of the inner tree. gate-sdk/SPEC.md owns the crate's contracts (`native/` carries no SPEC of its own), so the amendment sits here.

## What changes

### (1) The crosser refuses an answer that skipped a nested repository {design-bearing} {user-facing: the queue entry's deliverable, "the crosser detects a `.git` entry between the probed directory and the toplevel git answered and refuses with that cause" — operator selection, direction 2026-10-04}

`toplevel_args` in `native/src/walk.rs` spawns `git rev-parse --show-toplevel --show-prefix` in place of `--show-toplevel` alone. On a zero exit the first line is the toplevel, crossed through `normalize_abs` as today, and the second is git's own prefix from that toplevel to the anchor: relative, `/`-separated, empty at the toplevel itself. Measured on git 2.55, the pair exits exactly as `--show-toplevel` alone does at every probed anchor (a toplevel, a skipped nested shape, a valid nested repository, inside `.git` under `core.worktree`, a bare repository). A non-zero exit still goes to `refused_or_absent`, unchanged.

A private `nested_mark(top, prefix)` then walks the prefix's leading runs, nearest the anchor first, and answers the first `<top>/<run>/.git` entry of any type, read through `symlink_metadata` so a dangling symlink counts. `top` itself is never tested. A hit is the **refused answer**, in the `Err` arm with its own sentence:

> git answers the toplevel `<top>` at `<top>/<prefix>`, though `<mark>` marks a repository beneath it — git skips a repository it cannot read and answers the enclosing one, which is not read as this tree's (`git --git-dir=<mark> status` prints git's reason)

The remedy names `--git-dir`, because `git -C <inner> status` prints the outer repository's status and no reason; `git --git-dir=<mark> status` answers `not a git repository: '<mark>'` for all three skipped shapes.

- **The prefix is git's, not the crate's.** It is the physical path git discovered through: an anchor spelled through a symlink answers the target's prefix. So the check needs no anchor of its own and is independent of how `refused_or_absent` derives one, the `crosser-anchor-lexical-ascent` sibling's subject. Whether that sibling lands in the same build batch or another, this delta's act is the same; the two edit neighbouring functions of `walk.rs`, so whichever lands second rebases onto the first.
- **A non-empty `GIT_DIR` skips the check.** git does no discovery under it, so a `.git` beneath the toplevel was never skipped. With `GIT_WORK_TREE` or `core.worktree` alone, discovery still runs and the check stands. An anchor outside such a work tree answers an empty prefix and is never tested.
- **The signature stays `Result<Option<String>, String>`.** The new refusal rides the error arm beside the existing one, so every caller that already obeys the caller rule (*a caller degrades on the absent answer only*) fails closed with no edit, and the soft surfaces stay soft.

**Replacement text** for gate-sdk/SPEC.md §The crate's crosser: the `walk::toplevel()` bullet's third sub-bullet, and its honest-limit paragraph. *Not yet applied.*

> - A **refused repository** is a non-zero exit at a **marked** anchor, or a zero exit whose `--show-prefix` answer crosses a `.git` entry of any type beneath the answered toplevel, where git skipped a repository it cannot read and answered the enclosing one. Either is an error naming the mark and the way to git's reason: `git -C <anchor> status` for the first, `git --git-dir=<mark> status` for the second, since `-C` there prints the enclosing repository's status. A refused repository read as no repository, or as the enclosing one, would fail every caller open.

> **Honest limit:** the mark's ascent does not stop at a filesystem boundary where git's discovery does without `GIT_DISCOVERY_ACROSS_FILESYSTEM`, so a non-repository mounted inside a repository's tree reads as refused rather than absent. git's stderr cannot tell a broken repository from none, and its zero exit skips one silently, so the mark is the crosser's only discriminator on both. A front end that changes to git's toplevel before the crate runs hands it an empty prefix, so the nested refusal never sees the tree the run started in.

The anchor-and-mark paragraph that follows the sub-bullets stays as written: it defines the non-zero exit's mark, which this delta leaves unchanged. The `spec:` comment above `refused_or_absent` stays true; `nested_mark` carries its own, citing §The crate's crosser.

### (2) Canon-kit's tracked-set finder refuses a nested scan root {mechanical}

`Tracked::at` in `native/src/spec.rs` is not edited: it propagates `walk::toplevel_in_opt(root)?`, so delta 1's refusal exits 2 there instead of listing an empty tracked set. Its witness is a case added to `a_repository_git_cannot_answer_for_is_refused_rather_than_walked`: inside an initialized outer repository, a scan root under a `.git` directory with a garbage `HEAD` errs with delta 1's sentence (`marks a repository beneath it`), where it returned no file before.

**Replacement text** for canon-kit/SPEC.md §The shared spec adapters, the *Outside a work tree the walk stands* sub-bullet's first sentence. *Not yet applied.*

> - **Outside a work tree the walk stands**, as in a bespoke test's `mktemp -d` sandbox, and a repository the crosser refuses exits 2 rather than walking, since the walk would grade untracked files with no notice, and an enclosing repository's empty tracked set would drop them.

The sub-bullet's remaining sentences stay as written.

### (3) The crosser's own tests {mechanical}

A crate test in `native/src/walk.rs`, `a_repository_git_skips_beneath_the_toplevel_is_refused`. Its sandbox is under `std::env::temp_dir()`, outside every repository, under the `knobenv` lock with `GIT_DIR` and `GIT_CEILING_DIRECTORIES` removed and restored as the existing refusal tests do. It initializes an outer repository and, beneath it, `inner/sub`:

- `inner/.git` a directory holding a garbage `HEAD`: `toplevel_in_opt` from `inner/sub` errs, naming the mark and the outer toplevel, and `toplevel_in` errs with the same sentence rather than `not a git repository`;
- `inner/.git` an empty directory: the same refusal;
- `inner` re-initialized as a valid repository: `Ok(Some(<inner>))`, the control the deliverable names;
- the broken shape restored, anchor at the outer toplevel: `Ok(Some(<outer>))`, since the mark is not on the prefix;
- the broken shape, `GIT_DIR` set to the outer repository's `.git`: no nested refusal.

A `#[cfg(unix)]` companion, `a_dangling_git_symlink_beneath_the_toplevel_is_refused`, covers the third skipped shape, a `.git` symlink naming nothing; Windows symlink creation needs a privilege a test cannot assume.

## Producers and consumers

- **Producer:** `toplevel_args`, reached by every `walk::toplevel*` form, on a zero exit whose prefix crosses a `.git` entry. No config enables it: it runs wherever the crate asks the crosser, which every deployed configuration does.
- **Consumers:** every caller of `walk::toplevel`, `toplevel_in`, `toplevel_opt` and `toplevel_in_opt`, receiving the refusal through the error arm they already carry. Probe: `grep -rn "toplevel_in_opt\|toplevel_opt()\|toplevel_in(\|toplevel()" native/src --include=*.rs`, minus `walk.rs`, at `c1a9fe546`: 46 lines across 40 files. Their verdicts on the error arm were settled under the caller rule when the crosser's first refusal landed, and this delta adds no answer class, so none is re-read; the soft surfaces (the statusline hook, the update notice, installer doctor, drift-kit's readouts) stay soft on it.
- **The sibling `repo-probe-refusal-blind`.** Its `--git-dir` and `--is-inside-work-tree` probes answer the enclosing repository at exit 0 in the same shapes (measured: `--git-dir` answers the outer `.git`, `--is-inside-work-tree` answers `true`). A probe it routes through a `walk::toplevel*` form receives this refusal with no further act; a probe it gives its own classification does not, and that unit's amendment or build names which.
- **Roster-holding readers of the spawn:** `check-path-dialect`'s Rust arm matches the `--show-toplevel` token and counts every hit in `walk.rs` as the crosser's (`scan_rust` in `native/src/gates/path_dialect.rs`). `--show-prefix` is a relative answer, absent from its `GIT_FLAGS`, and the gate holds no count floor, so its verdict does not move. Its `good/` and `bad/` fixtures carry their own `walk.rs` and read nothing of this one.
- **Fields:** the prefix's one reader is `nested_mark`; the toplevel line's readers are unchanged.
- **Corpus narrowing:** none. The tracked set at a nested scan root goes from empty to a refusal; no reader asserts a count over it.
- **Fixtures and sandboxes that could flip:** `git grep -nE 'mkdir[^|;&]*\.git\b|\.git/HEAD|join\("\.git"\)|\.git"\)'` over the tracked tree at `c1a9fe546` finds no test or smoke building an empty or garbage-`HEAD` `.git` beneath a repository; `find . -name .git` outside `.tmp/` finds none below this tree's own root.

## Existing sections updated

- gate-sdk/SPEC.md §The crate's crosser — the refused-repository sub-bullet and the honest-limit paragraph (delta 1).
- `native/src/walk.rs` — `toplevel_args`, the new `nested_mark` (delta 1); the two tests (delta 3).
- canon-kit/SPEC.md §The shared spec adapters — the *Outside a work tree the walk stands* sub-bullet (delta 2).
- `native/src/spec.rs` — the finder's refusal test (delta 2).

## Retired spellings

- None — no delta of this amendment retires a spelling; the spawn stays the crate's only `--show-toplevel` one.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the new refusal.
- [ ] **Instruction surfaces: instruction only** — no template, agent definition or shim is touched.
- [ ] **Merged with no information lost** — the two replacement passages re-phrase the sub-bullets they refine; the merged §The crate's crosser reads as one document.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — none declared.
- [ ] **Gaps filed** — the front ends' change to git's toplevel before the crate runs, which keeps the nested refusal from the tree a front-end run starts in, filed at spec to the gap inbox.
- [ ] **Built and run** — `bash gate-sdk/bin/build-native.sh`, the crate tests, the full battery, and the fixture suites of gate-sdk and canon-kit, whose gates `walk.rs` and `spec.rs` implement.
- [ ] **Entry moved** — `--queue done nested-broken-repo-toplevel` in the build batch that lands this, a stage before the drain stage; no remote oracle gates it, since the skipped shapes the tests build are host-independent and the symlink case is Unix-only by design.
