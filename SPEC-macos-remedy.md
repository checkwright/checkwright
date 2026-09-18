# SPEC amendment: macos-remedy

**The macOS adopter remedy is written in three places and none of them holds
another.**

- docs/install.md §Requirements states it in prose: install GNU bash together with
  coreutils, put them ahead of `/usr/bin` on `PATH`, and take `shellcheck` from
  Homebrew. It never prints the command.
- The two macOS install-smoke legs each carry a literal copy,
  `brew install bash coreutils gawk shellcheck` (`.github/workflows/gates.yml`, the
  `install-smoke-macos` and `install-smoke-macos-intel` legs). Each copy also carries
  a hand-written `PATH` ordering.
- The step header rules the two copies equal to the page "in both directions".
  Nothing checks that, and the copies have already drifted: they still install gawk,
  which the page stopped naming (the active debt entry `macos-adopter-legs-brew-gawk`).

**Measured at spec, 2026-09-18.** `grep -n 'brew install' .github/workflows/*.yml
scripts/ci-macos-floor.sh` returns three sites:

- the two adopter-claim legs;
- `scripts/ci-macos-floor.sh`, with a different set (`bash coreutils gawk`). That
  script is the `native-artifacts` build legs' runner floor, and the comment above
  its call in gates.yml rules it scaffolding, "the producer's own prerequisite".

`grep -n gawk docs/install.md` returns nothing.

**The ruling: derive, do not gate.** The entry offered two shapes.

- **Refused: extracting a shared script both legs call.** It merges the two copies,
  but it still holds nothing equal to the page, and a prior amendment refused a
  third `scripts/ci-*` body.
- **Refused: gating the brew set against §Requirements.** It needs a mapping from
  floor members to brew formulae, and no such surface exists. The pool triage
  recorded that the roster's floor members map to formulae "neither one-to-one nor
  derivably".
- **Chosen: a third shape that removes the duplication instead of policing it.**
  The page carries the remedy as a literal marked block of commands. Both legs
  extract that block and run it. The page and the legs are then equal in both
  directions by construction:
  - a formula the page drops leaves the legs on the same run;
  - a formula a leg would need but the page lacks makes the binding leg red, which
    is the adopter's broken path showing up as a red.

  This follows the precedent of the `native-artifacts-roster` step, which already
  derives the producer matrix from docs/install.md's `platforms` marker block.

**What this subsumes.** The block names no gawk, so running it drops gawk from both
legs. That is the debt entry `macos-adopter-legs-brew-gawk`'s whole deliverable for
the legs, and its BSD-awk push risk carries over unchanged.
`scripts/ci-macos-floor.sh` keeps its set, as that entry already rules.

## What changes

### (1) docs/install.md §Requirements carries the macOS remedy as a marked block

The macOS paragraph ("Install GNU bash together with coreutils, then put them ahead
of `/usr/bin` on `PATH`") is re-phrased to introduce the commands, and a marked
block follows it {mechanical}. **Not yet applied.**

````markdown
<!-- macos-remedy:begin -->

```sh
brew install bash coreutils shellcheck
export PATH="$(brew --prefix)/opt/coreutils/libexec/gnubin:$(brew --prefix)/bin:$PATH"
```

<!-- macos-remedy:end -->
````

- The re-phrased paragraph states what each line is for: GNU bash for the 4.3
  floor, coreutils for `sort` and `date`, `shellcheck` because `init` refuses
  without it, and the `export` because the gnubin directory is what makes the GNU
  names resolve unprefixed.
- The `shellcheck` bullet's "On macOS it comes from Homebrew" is re-phrased to point
  at this block.
- The block is the whole remedy. The page names no step for a Mac outside it.

### (2) The two macOS legs run the page's block instead of carrying a copy

In `install-smoke-macos` and `install-smoke-macos-intel`, the step
`install the GNU userland docs/install.md names, and PATH-order it` replaces its
body {design-bearing}. The new body extracts the block's fence body from
docs/install.md and runs it in the step's shell. It then persists the `PATH` entries
the block prepended, in the block's order, to `$GITHUB_PATH`. **Not yet applied.**

- **The reader** is POSIX awk. It takes the lines between the two marker lines,
  excluding the fence lines. The step refuses (exit 1) on an empty extraction and
  says so: a leg that performs the page's remedy has nothing to perform without the
  block.
- **The persistence.** The prepended entries are the part of `$PATH` in front of the
  pre-run value, split on `:`. The runner puts each `$GITHUB_PATH` line ahead of the
  lines written before it. The entries are therefore written last to first, so the
  block's first entry ends up first.
- **The ordering is not load-bearing today.** The two entries share no command name:
  coreutils installs only `g`-prefixed names into `$(brew --prefix)/bin`. Run
  35214069775's probe step on the current step, which writes the same two
  directories, resolved `sort` to the gnubin copy (`sort (GNU coreutils) 9.11`,
  `sort -V exited 0`) and `bash` to Homebrew's 5.3. The probe step stays after this
  step and is the witness on every run.
- **The shell.** The body keeps to bash 3.2, because it runs before the remedy has
  installed a newer bash.
- **Failure handling.** A failing `brew` keeps the step's never-fails posture
  (`set +e`, the probe reads the damage). The extraction refusal is the one exit
  that fails the step.
- **The step's header.** "Hold this package set and §Requirements' list equal, in
  both directions" is re-phrased to "this step runs §Requirements' macOS block
  verbatim, so the leg has no package set of its own". The `findutils` directive
  moves with it: it becomes "do not add a formula here; a leg that needs one the
  block lacks is the page's defect, fixed in the block". The Intel leg's header
  already cites the arm64 sibling's and keeps doing so. Its "SECOND copy of that
  claim" sentence becomes "a second run of the same block on a second host".

### (3) The block's grammar and readers get a row beside its neighbours

docs/site-architecture.md §Generated projections and their freshness gates gains a
row after the install-platforms parity contract {mechanical}. **Not yet applied.**
The row states:

- **Grammar.** One hand-authored marker block, `<!-- macos-remedy:begin -->` to
  `<!-- macos-remedy:end -->`, holding one `sh` fence whose lines are shell commands
  run verbatim.
- **Readers.** The two macOS install-smoke legs.
- **Why no gate.** The block is not a projection. The binding legs are its
  enforcement: a malformed block reds the push that carries it.
- **Honest limit.** That red arrives at push, not at commit. No local battery runs
  a Mac.

### (4) The comments that describe the old relation are re-phrased

These comments in `.github/workflows/gates.yml` are re-phrased {mechanical}. **Not
yet applied.**

- **The `native-artifacts` macOS floor step's header.** "That step's package set IS
  an adopter claim and is held equal to §Requirements' list in both directions"
  becomes "that step runs §Requirements' own macOS block". The rest of the header
  stands, including "the install-smoke leg's step is deliberately NOT folded in",
  because the two sets still answer to different surfaces.
- **The Windows leg's `choco` header.** "Read it the way the macOS legs' `brew` step
  is read: the package set is an adopter claim, held equal to what that section
  states" becomes a statement that this step is a copy of the `shellcheck` bullet's
  Chocolatey route, held by nothing, and the filed gap names it.

## Producers and consumers

- **The block (delta 1).**
  - Producer: the hand-authored docs/install.md.
  - Consumers: the two macOS legs' remedy step (delta 2), which run it through the
    extraction above; and an adopter reading the page.
  - Every line of the fence is read and run, and no line is ignored except the two
    fence lines. So every field has a reader.
- **The persisted `PATH` entries (delta 2).** Producer: the remedy step. Consumer:
  every later step on the leg, through the runner's `$GITHUB_PATH`. That covers the
  probe step and the suite, whose `bash`, `sort` and `date` resolve through them.
- **Roster-holding readers of the minted marker name.**
  - Every marker reader in the crate reads its own named marker: `git grep -n
    ':begin -->' -- native/src` lists `toolchain`, `platforms` and the other named
    blocks, and none enumerates unknown ones. So no gate reds on a new marker.
  - docs/site-architecture.md's marker rows are the one roster that names install.md
    blocks, and delta 3 adds the row.
  - `check-docs-cmd` assertion A reads fenced `.sh` invocations. The block has none.
  - `check-docs-render-fidelity` re-renders the page. Build runs it.
- **Point 5 (narrowing).** Delta 2 narrows each leg's installed set by gawk. Red
  conditions:
  - the leg's suite reds on a GNU-only awk construct run through bare `awk` (the
    debt entry's push risk, owned there);
  - the probe step never fails.
  - `scripts/ci-macos-floor.sh`'s legs are not narrowed.
- **Point 6 (members).** Delta 2 obliges each command in the block to succeed on
  both hosts.
  - `bash`, `coreutils` and `shellcheck` are the formulae the current step already
    installs green on both labels (run 35214069775; install-smoke-macos and
    install-smoke-macos-intel both `success`).
  - The `export` line names the two directories the current step already writes.
  - gawk is the one member dropped, and no formula is added.

## Existing sections updated

- `docs/install.md` §Requirements: the macOS paragraph, the block, and the
  `shellcheck` bullet's macOS clause (delta 1).
- `.github/workflows/gates.yml`: the two macOS legs' remedy step body and header
  (delta 2); the floor step's header and the Windows `choco` header (delta 4).
- `docs/site-architecture.md` §Generated projections and their freshness gates: the
  new row (delta 3).
- `TASK-QUEUE.md`: at merge, in the build session that lands delta 2 and before the
  drain stage is entered (deltas 1 and 2):
  - `macos-adopter-package-set-copied-per-leg` moves to Done;
  - if `macos-adopter-legs-brew-gawk` is still active, delta 2 discharges its
    deliverable and it moves to Done in the same commit, whose message names delta 2
    as what closed it.

Roster produced by `git grep -n "brew install\|gnubin\|adopter claim\|held equal"`
over the tracked tree. It is a floor that build re-derives.

## Retired spellings

- `brew install bash coreutils gawk shellcheck` — the per-leg copy (delta 2).

## Definition of Done

- [ ] **Causal completeness**: every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each delta.
- [ ] **Run the system**: on the watched push, both macOS legs' logs show the block
      extracted and run, their probe steps resolve `bash` to Homebrew's and `sort` to
      GNU, and both legs conclude `success`.
- [ ] **One home**: `git grep -n 'brew install' -- .github` returns nothing, and
      the tree's one other package list is `scripts/ci-macos-floor.sh`'s.
- [ ] **Merged with no information lost**: the refused shapes and their grounds
      survive in the docs/site-architecture.md row.
- [ ] **Queue moves placed before the drain stage.**
- [ ] **Amendment deleted**: this file is removed on merge.
- [ ] **Gaps filed**: the Windows `choco` copy (filed at spec), and any gap build
      finds.
