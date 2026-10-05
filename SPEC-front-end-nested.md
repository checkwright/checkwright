# SPEC amendment: front-end-nested

Paired with the queue entry `front-end-nested-toplevel`. Inside a broken nested checkout — a `.git` directory with a garbage `HEAD`, an empty `.git` directory, a `.git` symlink naming nothing — git skips the nested repository and answers the enclosing toplevel at exit 0. The crate's crosser refuses that answer (gate-sdk/SPEC.md §The crate's crosser), but only from the directory the run started in, and the front ends change to the toplevel before the binary runs. Measured at spec time: an initialized repository with an empty `sub/.git`, asked from `sub/deep`, answers `git rev-parse --show-toplevel --show-prefix --git-dir` with the outer toplevel, `sub/deep/` and the outer `.git` at exit 0. The binary run directly there refuses with the crosser's nested sentence; `run-gates.sh` run there changes to the outer toplevel first and runs on.

The lookups that change directory, by `git grep -n show-toplevel -- ':!native' ':!*.md'` at `46df0b05e`, excluding test scripts and fixtures: `gate-sdk/bin/run-gates.sh`, `gate-sdk/bin/run-gates.ps1`, `context-kit/templates/session-context.sh` and this repository's copy `scripts/session-context.sh`. `plugin/hooks/hooks.json`'s prefix resolves a toplevel with `git -C` and changes no directory; the front end it then runs does. `scripts/ci-pack-extension.sh` is a CI packing script run at a checkout's root, and refuses on a failed lookup already.

The units span gate-sdk (the front ends, lib/gate.sh, the parity corpus), context-kit (the session-context template) and plugin (one sentence), so the file sits at the root.

## What changes

### (1) The front ends refuse an answer that skipped a nested repository {design-bearing} {user-facing: the queue entry's deliverable, "each front end and shim refuses where git's answered prefix crosses a `.git` entry beneath the toplevel" — operator selection, direction 2026-10-05}

A new lib/gate.sh accessor, `gate_skipped_mark <top> <prefix>`, prints the first `<top>/<run>/.git` entry of any type (`-e` or `-L`, so a dangling symlink counts) along the prefix's leading runs, nearest the starting directory first, and returns 0; it returns 1 when none is found, when `GIT_DIR` is non-empty, since git then discovers nothing, and for an entry that is the repository git selected, the exception the crate's crosser carries for `GIT_WORK_TREE`. `<top>` itself is never tested.

`run-gates.sh` asks `git rev-parse --show-toplevel --show-prefix` in its one lookup spawn, splitting the answer as the crate splits it (§The crate's crosser), and on a zero exit calls the accessor before its `cd`. A mark takes the stub's refusal branch with its own path-free line:

> run-gates: git skipped the repository a .git entry marks between here and the toplevel it answered, and answered the enclosing one; git --git-dir=<that .git> status prints its reason

The PowerShell twin re-holds the accessor as it re-holds the others, and prints the same line. Delta 2 settles the branch's status.

**Two sibling units in this iteration move the rule this accessor copies.** `crosser-nested-mark-env` lands the selected-repository exception in the crate, and `crosser-git-fidelity-edges` the crate's answer split and its relative-ceiling skip. Where either has landed before this batch, the accessor and the twin copy its rule as landed. Where it has not, this batch lands the front-end half of that rule here and the sibling lands the crate half, the two stated in §The crate's crosser in whichever batch lands second.

**Replacement text** for gate-sdk/SPEC.md §run-gates, the *front-end requires a checkout* paragraph's sentence on the mark. *Not yet applied.*

> Where the crosser's mark covers the working directory (§The crate's crosser), it names git's refusal instead, since the crate runs only after this `cd`. Where git answers a toplevel whose prefix crosses a `.git` entry, the nested mark the crosser refuses, it names that instead, through `gate_skipped_mark` (§lib/gate.sh). Each half holds both marks itself, and neither line carries a path, so the twin reproduces it byte for byte.

**Replacement text** for gate-sdk/SPEC.md §The crate's crosser, its honest-limit paragraph's last sentence, which is deleted: the front end now refuses the tree it starts in. *Not yet applied.*

§lib/gate.sh gains the accessor's row, and §run-gates' *re-holds the five accessors* sentence counts six.

### (2) A fail-open arm declines in a refused tree {design-bearing} {user-facing: the operator direction of 2026-10-05, lead-relayed (not a ruling) — `--hook` and `--statusline` decline at exit 0 with the fixed `systemMessage` on both the nested refusal and the failed-lookup branch; every other arm keeps exit 2}

The stub's refusal branch, the failed lookup and delta 1's skipped mark alike, exits `$ARM_UNAVAILABLE_STATUS` rather than `2`: the fail-open set declines at `0`, and `--hook` writes a fixed `systemMessage` envelope saying the tree's hook guards are off because git refuses or skips its repository, naming `git status` and `git --git-dir=<that .git> status`. Every other arm exits 2 as today. `FAIL_OPEN_ARMS` is read before the lookup rather than after it, in both halves.

The ground is the one §The harness-integration arm gives for the absent binary: a hook arm exiting 2 blocks the tool call, and a session whose working directory sits in a refused tree would have every guarded call blocked, the `cd` that would leave it included. Today the failed-lookup branch already exits 2 for a hook arm; the plugin's prefix declines only where its own `git -C` lookup fails, and the session's working directory can sit in a refused tree under a valid project directory.

**Replacement text** for gate-sdk/SPEC.md §The harness-integration arm, the *cannot run at all* bullet. *Not yet applied.*

> - *Cannot run at all* has two causes the front-end can see: an absent or non-executable binary, and a tree whose repository git refuses or skips (§run-gates), where the stub runs no binary. Inside a linked worktree, which carries no build output, …

The bullet's remainder stays as written.

### (3) The session-context hook runs where git answers this tree {mechanical}

context-kit/templates/session-context.sh's first line resolves the toplevel, sources `<top>/gate-sdk/lib/gate.sh` where present, and changes to the toplevel only when `gate_skipped_mark` finds no mark; otherwise it stays at the working directory, as it does where git answers none, and each step degrades from there. This repository's copy, `scripts/session-context.sh`, takes the same line, which `check-template-copy-parity` holds.

**Replacement text** for context-kit/SPEC.md §The session-context hook (template), appended to its first paragraph's last sentence. *Not yet applied.*

> Every step is guarded and degrades silently: the hook never fails a session. It runs at the git toplevel, or at the working directory where git answers none or skips a nested repository (gate-sdk/SPEC.md §lib/gate.sh, `gate_skipped_mark`).

### (4) The plugin's guards sentence {mechanical}

plugin/SPEC.md §The guards: *The shell-guard fires anywhere inside a vendored repository and stays silent outside one* gains *and declines, through the front end, where git refuses or skips the repository the session stands in*. `plugin/hooks/hooks.json` and its rendering rule are not edited. *Not yet applied.*

### (5) The front ends' tests {design-bearing}

`--run-front-end-parity`'s corpus (`native/src/emit/front_end_parity.rs`) gains a starting subdirectory per case, defaulting to the scratch root, and these cases:

- an empty `sub/.git` directory, run from `sub/deep` under `--emit knob-values`: exit 2 with delta 1's line;
- the same under `--hook escalation-guard`: exit 0 with the `systemMessage` envelope (delta 2);
- the existing *a repository git refuses* case's `--hook` twin: exit 0 with the envelope;
- a `sub/.git` holding a garbage `HEAD`: delta 1's line;
- the empty `sub/.git` with `GIT_DIR` naming the scratch root's `.git`: no nested refusal;
- `sub` a valid repository: no refusal, the control.

A `#[cfg(unix)]` case carries the dangling `.git` symlink. The parity arm already reds when the bash stub does not do what a case names, so each case witnesses the stub as well as the twin. `check-front-end-fail-open` holds the moved `FAIL_OPEN_ARMS` line as before; its fixtures carry their own stubs and need no edit.

## Producers and consumers

- **The skipped-mark refusal** (delta 1): produced by `run-gates.sh` and its twin on every invocation from inside a work tree, so every deployed configuration reaches it; consumed by the invoking shell or harness through the exit status and the stderr line.
- **The accessor** (delta 1): readers are `run-gates.sh` and the session-context template (delta 3); the twin's copy is held by the parity corpus (delta 5).
- **The decline** (delta 2): its readers are the harness, which reads a hook's exit 0 as allow and renders the `systemMessage`, and the parity corpus.
- **Roster-holding readers of the stubs' text**: `check-front-end-fail-open` (the `FAIL_OPEN_ARMS` line, moved not renamed), `check-path-dialect`'s shell arm, which reads every `--show-toplevel` binding in tracked shell and reds a bound root never crossed (its `bad/tree/bound-root.sh`), so the stub's lookup keeps a shape its `good/` fixtures admit, the toplevel crossed by the `cd` as today and the prefix, a relative answer, read beside it, `check-shellcheck` over both bash files, and `check-portability-floor`'s ASCII arm over the twin. `check-template-copy-parity` holds the session-context copy (delta 3).
- **The Windows witness**: the twin runs under Windows PowerShell 5.1 only on the CI Windows legs (the `crate-tests-windows` x64 entry runs the parity arm), so the iteration's mid push the queue entry names carries it.
- **Corpus narrowing:** none.

## Existing sections updated

- gate-sdk/SPEC.md §run-gates — the mark sentence and the accessor count (delta 1); §The crate's crosser — the honest-limit sentence deleted (delta 1); §lib/gate.sh — the accessor row (delta 1); §The harness-integration arm — the *cannot run at all* bullet (delta 2).
- `gate-sdk/lib/gate.sh`, `gate-sdk/bin/run-gates.sh`, `gate-sdk/bin/run-gates.ps1` (deltas 1 and 2).
- context-kit/SPEC.md §The session-context hook (template); `context-kit/templates/session-context.sh`, `scripts/session-context.sh` (delta 3).
- plugin/SPEC.md §The guards (delta 4).
- `native/src/emit/front_end_parity.rs` (delta 5).
- The on-site mirrors of gate-sdk's, context-kit's and plugin's SPECs (all deltas).

## Retired spellings

- None — no delta of this amendment retires a spelling; both refusal lines and the fail-open set keep their names.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the refusal, the accessor and the decline.
- [ ] **Instruction surfaces: instruction only** — the session-context template's line carries no grounds; context-kit/SPEC.md carries them.
- [ ] **Merged with no information lost** — each replacement passage re-phrases the sentence it refines.
- [ ] **Amendment deleted** — this file removed on merge; no root `SPEC-*.md` of this unit remains.
- [ ] **Removals propagated** — none declared.
- [ ] **Gaps filed** — any further front end found changing to a toplevel is fixed in the batch or filed.
- [ ] **Built and run** — `bash gate-sdk/bin/build-native.sh`, the crate tests, the full battery, `--run-front-end-parity` on this host's `pwsh` where present, and the fixture suites of gate-sdk and context-kit.
- [ ] **Entry moved** — `--queue done front-end-nested-toplevel` a stage before the drain stage, once the Windows legs of the push carrying the twin are green; until then the entry bridges with a `[spec:]` path ref to gate-sdk/SPEC.md.
