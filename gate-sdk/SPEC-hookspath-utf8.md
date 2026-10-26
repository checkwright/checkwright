# SPEC amendment: hookspath-utf8

Pairs `hookspath-non-utf8-restore`. A bare `--install-hooks` run reads the prior hooks directory off `core.hooksPath` and writes it into the chain record, beside the local value the restore line sets back (§install-hooks). Both reads decode git's output lossily, so a value that is not UTF-8 is recorded as another value. This amendment refuses such a value at the opt-in, beside the line-break and empty-value refusals, as the crate already refuses a listed name that is not UTF-8 (§The crate's crosser).

**What the tree does today**, probed at spec on Linux with git 2.55 and the built binary, in scratch repositories whose prior hooks directory holds a live `pre-commit` that prints a line:

- **A value that is not UTF-8 silences the prior hook.** `prior` and `local_hooks_path` (`native/src/emit/install_hooks.rs`) decode with `String::from_utf8_lossy`. With `core.hooksPath` naming a directory `h\377k`, the value becomes `h<U+FFFD>k`, which names no directory. So the run reads no live hook there and prints `core.hooksPath was …/h<U+FFFD>k, which holds no hook git runs`. The chain record's `dir=` and `restore=` both carry U+FFFD, and the receipt advises `git config core.hooksPath '…/h<U+FFFD>k'`. A commit before the opt-in runs the prior `pre-commit`; a commit after it does not.
- **A UTF-8 value naming a link to that directory silences it too.** Its restore line is right, the record's `restore=` being the value as set. But the prior directory resolves physically (`walk::physical_against`), and `walk::canonicalize` spells the resolved path through `display()`, so `dir=` carries U+FFFD and the hook is not reached.
- **A git directory whose path is not UTF-8 is outside this amendment.** A work tree whose own path is not UTF-8 is already refused before `prior` is reached (`not inside a git repository`, exit 2). A UTF-8 linked worktree whose common git directory is not UTF-8 is not refused: the run exits 0, places the hooks under a new directory spelled with U+FFFD, points `core.hooksPath` there, and the next commit fails. Its cause is the lossy common-directory read that `hook_launcher::hooks_dir` takes, not `prior`, so it is filed as a gap of its own, with `prior`'s `git rev-parse --git-path hooks` read left as it is.

## What changes

### (1) A prior hooks directory that is not UTF-8 is refused at the opt-in {design-bearing} {user-facing: lead decisions 2026-10-25 on the intent oracle's ruling, lead-relayed (not operator directions): a strict decode refused at exit 2 with no valve, the entry's deliverable, and the physically resolved directory refused too; operator direction 2026-10-26, lead-relayed: the default-directory read left out, its case filed as a gap}

A bare `--install-hooks` run refuses at exit 2, placing nothing, where a value it would chain or record is not UTF-8:

- `prior` decodes `git config --path --get core.hooksPath`'s output strictly. A value that is not UTF-8 refuses.
- `local_hooks_path` decodes the local-scope value strictly. Where a worktree or command scope overrides the local value, `prior` reads that override, so a local value that is not UTF-8 reaches this read alone.
- The prior directory a set value names, resolved physically, refuses where the resolved path has no UTF-8 spelling, as through a link to such a directory. The check reads the path's own bytes (an `OsStr` with no `str` form), at the resolved directory or, where it does not exist, its deepest existing ancestor. It never searches for U+FFFD, which a real name can hold.

The resolution stays the crosser's (§The crate's crosser), since `std::fs::canonicalize` is `walk.rs`'s alone and `walk::canonicalize` hands its answer on through `display()`, which has already replaced the bytes. So `walk.rs` gains a strict form of `physical_against`. It answers what `physical_against` answers where the resolved prefix has a UTF-8 spelling, and otherwise the resolved deepest existing ancestor as a `PathBuf`, its bytes untouched. The one `std::fs::canonicalize` call moves beneath a private resolution that keeps the `PathBuf`, which `walk::canonicalize` converts as it does today and the strict form tests with `to_str()` before any conversion. `prior`'s set-value arm takes the strict form. Its default arm keeps `physical_against`, as the default-directory read is left out. The appended rest is the strictly decoded value's own, so it is UTF-8 already. `physical_against`'s other callers, in `hook_launcher`, keep its lossy answer.

The refusal names the value with each byte that is not UTF-8 escaped as `\xNN`, and says the chain record and the restore line carry UTF-8 alone. Where the value is UTF-8 and the resolved directory is not, it names that directory too, escaped the same way: its bytes on POSIX, and on Windows each unpaired surrogate of its UTF-16 as `\u{XXXX}`. The crate's `rust-version` predates `OsStr::as_encoded_bytes`, so this needs a `cfg` split. The escaped spelling only goes into the message and is never read back as a path, so on Windows it keeps any extended-length prefix the resolution answered. Its `help:` is the remedy: rename the directory, or set `core.hooksPath` to a directory whose path is UTF-8, or unset it at the scope that set it (`git config --show-origin --get core.hooksPath` names the scope), then re-run. The help names no link, which the physical check refuses too. A re-run, its prior naming the hooks directory, refuses nothing here. `--refresh` meets no prior. **Not yet applied.**

**Refused rather than carried:** a byte-preserving record and restore line. The restore line is pasted through a terminal, which hands the shell UTF-8, and PowerShell's argument is UTF-16, so no spelling carries the byte on native Windows. The record's `dir=` is read back as a string by the launcher. Every path the crate's crosser hands on for use is a string, and the strict form's `PathBuf` reaches only a message. Carrying the bytes would hold one value on a path that drops it everywhere after.

### (2) The strict decode over bytes, and the one line feed {mechanical}

`prior` reads git's output as bytes. The act depends on whether `hookspath-line-break-trim` lands in the same build batch. Each **Not yet applied.**

- **With it in the batch:** `prior` strips the one line feed git appends from the bytes, then decodes the rest strictly. A value keeping a line break goes on to the line-break refusal.
- **Without it:** `prior` decodes the whole output strictly, then trims as it does today. That entry's later landing moves the strip ahead of the decode.

`local_hooks_path` strips its one line feed from the bytes before its strict decode in either case.

### (3) The SPEC sections {design-bearing}

Each **Not yet applied.**

- gate-sdk/SPEC.md §install-hooks, *A value the record or its restore line cannot carry is refused*: a sub-bullet for a value, or a prior directory resolved physically, that is not UTF-8, the record and the restore line carrying UTF-8 alone, a byte-preserving record refused on the ground delta 1 states (deltas 1 and 2).
- gate-sdk/SPEC.md §install-hooks, *Honest limits*: a clone opted in over such a directory before the refusal keeps a record naming the replaced spelling, so its prior hooks stay unreached and its restore line names another value. Setting `core.hooksPath` back by hand and re-running meets the refusal. With the key unset, a git directory whose path is not UTF-8 is not refused here (delta 1).
- gate-sdk/SPEC.md §install-hooks, the prior hooks directory's unit cases: a value that is not UTF-8, written into the scratch repository's config file as bytes so the case runs on every host, refused with nothing placed; a local value that is not UTF-8 under a UTF-8 worktree-scope value, refused; on Linux, a UTF-8 value naming a link to a directory whose name is not UTF-8, refused (delta 1).
- gate-sdk/SPEC.md §The crate's crosser: the `physical_against` bullet names its strict form, which answers the resolved deepest existing ancestor's bytes where the path has no UTF-8 spelling, taken by a caller that must refuse a path the record cannot carry rather than carry a replaced spelling. The `canonicalize` bullet becomes: the crate's one `std::fs::canonicalize` call sits beneath `walk::canonicalize()` and that strict form, both in `walk.rs`, and `walk::canonicalize()` still hands its answer back unconverted (delta 1).
- The release declaration surface gains a `behavior` bullet: `--install-hooks` refuses a `core.hooksPath`, or the prior hooks directory it resolves to, that is not UTF-8, where it chained that directory as one holding no hook (deltas 1 and 2).

## Producers and consumers

- **The refusal** (delta 1). Producer: `prior` and `local_hooks_path`, called by `plan` at a bare run before `wire`, so nothing is placed. Its trigger is adopter config: a `core.hooksPath` at any scope that is not UTF-8, or that resolves to a directory whose path is not. No knob enables it. Consumers: the adopter, through stderr under the `install-hooks:` prefix at exit 2, a code §install-hooks' exit grammar already gives a value the chain record cannot carry; the unit tests.
- **The strict physical resolution** (delta 1). Producer: `walk.rs`, beside `physical_against` and sharing its one `std::fs::canonicalize` call with `walk::canonicalize`, so `check-path-dialect`'s producer forms stay green. Consumer: `prior`'s set-value arm alone. `physical_against`'s callers in `native/src/emit/hook_launcher.rs` and `walk::canonicalize`'s callers keep their answers unchanged.
- **`judged_in`** (`native/src/installer/mod.rs`), the other caller of `prior`. It reads any refusal of `prior` as no established hook, so a failed `init` or `uninstall` commit in such a clone prints its plain line rather than naming a directory spelled with U+FFFD. installer/SPEC.md §init already says a read that fails names no hook, so its text does not change.
- **The chain record's readers** (`read_chain` in `native/src/emit/hook_launcher.rs`, `hooks_line` in `native/src/installer/uninstall.rs`) meet no new grammar: a record written after this change carries UTF-8 alone.
- **`gate-tests/native-git-hooks.test.sh`** sets no value that is not UTF-8, so none of its cases newly refuses.

The callers were found by `grep -rn "install_hooks::prior\|prior(&here\|local_hooks_path" native/src`, the decodes by `grep -n "from_utf8_lossy" native/src/emit/install_hooks.rs`, the record's readers by `grep -rn "read_chain" native/src`, and the resolution's callers by `grep -rn "walk::canonicalize\|physical_against" native/src`.

## Existing sections updated

- `native/src/emit/install_hooks.rs`, `prior`, `local_hooks_path`, the refusal beside `EMPTY_REFUSAL`, and their unit tests (deltas 1 and 2).
- `native/src/walk.rs`, the strict form of `physical_against`, the private resolution beneath it and `canonicalize`, and a unit test of the strict form's two answers on Linux (delta 1).
- `gate-sdk/SPEC.md` §install-hooks (deltas 1 and 3).
- `gate-sdk/SPEC.md` §The crate's crosser, the `physical_against` and `canonicalize` bullets (deltas 1 and 3).
- `.workflow/release-declarations.md`, a `behavior` bullet (delta 3).
- `docs/gate-sdk/SPEC.md`, the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).

## Retired spellings

- None — no delta retires a name; the refusal is new and the record's grammar is unchanged.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The local oracle** — `bash gate-sdk/bin/build-native.sh`, the battery, the crate's tests and gate-sdk's fixture suite green. No remote run observes this entry, so its Done move lands before the drain stage, in the build batch that lands it.
