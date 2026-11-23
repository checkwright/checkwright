# SPEC amendment: statement-store

Pairs `statement-out-of-tree-store`. `--run-validate` writes its Statement into the tree whose commit it attests, and the verify and import arms can read one from any store through `EVIDENCE_KIT_STATEMENT_CMD`. Nothing writes to a store outside the tree, so an adopter keeping Statements out of its history hand-writes one. This amendment adds the write half as a command knob. It also ships a git-notes write and read pair as a documented value of the two command knobs, exercised by a fixture.

**What the tree does today**, read from `native/src/emit/run_validate.rs`, `native/src/emit/verify_gate_run.rs` and `native/src/knobs/evidence_kit.rs`:

- **The Statement has one destination.** `run` renders a Statement only where `EVIDENCE_KIT_STATEMENT_FILE` is non-empty (`attest` is `None` otherwise). It stages the body under the tmp dir before the fold and renames it onto that path after the manifest publishes.
- **The read half is already a command knob.** `read_input` spawns `EVIDENCE_KIT_STATEMENT_CMD`'s argv through `Program::consumer` and `proc::run_streamed`, and reads its stdout as the Statement. A non-zero exit is the arm's exit 2.
- **A command knob already appends operands.** gate-sdk/SPEC.md §The knob file states that a reader appending an operand appends it after the argv's last element. `EVIDENCE_KIT_STATEMENT_CMD` appends none.
- **git notes fits both halves**, measured on a scratch repository. `git notes --ref=gate-run add -f -F - <commit>` stores stdin as the note on `<commit>`, leaves `git status --porcelain --untracked-files=all --ignored` empty, and with `-f` overwrites an earlier note. `git notes --ref=gate-run show HEAD` prints the note and exits 0, and exits 1 on a commit with none. The write exits 128 when no committer identity resolves.

## What changes

### (1) A write command for the Statement {design-bearing} {user-facing: the statement-out-of-tree-store entry's deliverable on this iteration's set record (operator direction 2026-11-24, lead-relayed, not a ruling): a Statement write paired with the existing read command knob}

A new command knob, `EVIDENCE_KIT_STATEMENT_WRITE_CMD`, names a program that stores the run's Statement. Its default is empty, which stores nothing. **Not yet applied.**

- **What it receives.** The Statement's bytes on stdin, the same bytes `EVIDENCE_KIT_STATEMENT_FILE` would hold, its trailing newline included. The subject commit's full hash is appended as the last argument, per §The knob file's operand rule. The subject is the store's natural key, and the command could not otherwise learn it without parsing the Statement.
- **When a Statement is rendered.** Either knob being non-empty now renders one, so the write command works with no file configured. With both set the run publishes to both. The body is staged under the tmp dir either way, so the staging guards are unchanged.
- **When it runs.** After the manifest publishes and after the file's rename, where one is configured. That makes the store the last of the run's writes. It is the one write the claim cannot roll back, since it leaves the tree. The run prints `run-validate: Statement stored by EVIDENCE_KIT_STATEMENT_WRITE_CMD` on success.
- **A failed store is exit 2.** A spawn failure or a non-zero exit exits 2. The message names the command's exit and each destination already published, the manifest and the file where one is configured, so a reader knows which writes exist. The command's stderr passes through, and its stdout is discarded. The staged body is removed by the claim's destructor, as today.
- **A run that exits 1 still stores its Statement**, under the existing rule that a `new-failures` run attests what happened.
- **It is a command knob and not a scalar.** The argv reaches the program with no shell, so a consumer on native Windows names its own interpreter, and it takes no environment override (gate-sdk/SPEC.md §The knob file).

Replacement text for §bin/run-validate.sh's *The Statement* lead and its third bullet. **Not yet applied:**

> **The Statement.** Where `EVIDENCE_KIT_STATEMENT_FILE` names a path, or `EVIDENCE_KIT_STATEMENT_WRITE_CMD` names a command, the run renders one Statement (§The gate-run predicate) covering the suites it ran. It writes the Statement to the path, hands it to the command, or does both. With both empty, the default, it renders none. Writing at run time, rather than from an emitter reading the manifest later, is what lets `provenance` describe the binary, the kits and the tree that actually ran.
>
> - **Built before the fold, published after it, inside the claim.** The Statement is staged under the tmp dir before the manifest fold, so a staging failure aborts at the guards' exit 2 with nothing written. A Statement path whose directory does not exist is such a failure. The manifest publishes first and the Statement file second, each by rename. The write command runs last, with the staged bytes on stdin and the subject commit appended as its last argument. A failed rename, or a write command that fails to spawn or exits non-zero, exits 2 and names each destination already published, so a reader knows which writes exist. The staged file joins the claim's destructor, as the batch file does.

The knob's §Layout and configuration bullet:

> - `EVIDENCE_KIT_STATEMENT_WRITE_CMD` — the command `--run-validate` hands the run's Statement to on stdin, with the subject commit appended (§bin/run-validate.sh, *The Statement*). Default empty, which stores none. A command knob like `EVIDENCE_KIT_STATEMENT_CMD`, its read counterpart.

`EVIDENCE_KIT_STATEMENT_CMD`'s bullet drops *It is the kit's one **command knob*** and reads *It is a **command knob** (gate-sdk/SPEC.md §The knob file)*, the rest unchanged.

### (2) A git-notes pair, shipped as documented knob values {design-bearing} {user-facing: the statement-out-of-tree-store entry's deliverable on this iteration's set record (operator direction 2026-11-24, lead-relayed, not a ruling): a git-notes Statement write and read pair with a fixture, the read reachable through the existing command knob}

§bin/run-validate.sh gains a paragraph after *The Statement*'s bullets that documents the pair. The smoke and a gate test run it. No default changes. **Not yet applied:**

> **Storing Statements in git notes.** A consumer keeping Statements out of its tree binds the two command knobs to a notes ref, with the ref name its own choice, one argv element per indexed line. `EVIDENCE_KIT_STATEMENT_WRITE_CMD` holds `git`, `notes`, `--ref=gate-run`, `add`, `-f`, `-F` and `-`. `EVIDENCE_KIT_STATEMENT_CMD` holds `git`, `notes`, `--ref=gate-run`, `show` and `HEAD`.
>
> The write stores stdin as the note on the appended subject. `-f` replaces an earlier run's note, so the latest run at a commit is the one stored, as with the file. The note lives in the repository's object store, so it leaves the worktree clean and never enters the commit it attests. The read prints `HEAD`'s note, so `--verify-gate-run` with no operand judges the checked-out commit. A Statement of another commit is read by `git notes --ref=gate-run show <commit> | checkwright-gates --verify-gate-run -`.
>
> **Honest limits.** A notes ref is neither pushed nor fetched by default. A store meant to outlive the clone is pushed with `git push <remote> refs/notes/gate-run`, and a verifier's clone fetches it with `git fetch <remote> refs/notes/gate-run:refs/notes/gate-run`. The write makes a notes commit, so it needs a committer identity, and with none the store fails at exit 2 after the manifest publishes. A commit with no note makes the read exit 1, which the verify arm reports as no Statement read, at exit 2. Each run adds a commit to the notes ref, so the ref's history grows by one per run.

- **The merge may spell the pair as a knob-file block.** It is a calibration of the paragraph's layout. The smoke writes the same twelve lines into its scratch knob file.
- **A documented value, not an `--emit` arm.** Each half is one `git` invocation whose argv the command knob already carries, and git is the floor. An arm would re-implement `git notes` in the crate for no gain in what the knob reaches. The documented value is the shipped working member gate-sdk/SPEC.md §The knob file asks of a selecting knob. It is exercised by the smoke, so it cannot drift from the arm it feeds.
- **No default changes.** Both knobs stay empty by default, since a notes ref is a store an adopter opts into and never one a fresh install creates (doctrine-kit/DOCTRINE.md, Policy-as-choice).

## Producers and consumers

- **`EVIDENCE_KIT_STATEMENT_WRITE_CMD`** (new knob) — producer: the consumer's knob file, resolved by the evidence-kit table with an empty default. Consumer: `--run-validate` alone, which renders a Statement when it or `EVIDENCE_KIT_STATEMENT_FILE` is non-empty and spawns it after the fold. No deployed configuration in this repo sets it. Its enabling configuration is an adopter's knob file, which the smoke and the gate test stand in for.
- **The Statement on the write command's stdin** (new event) — producer: `--run-validate`, the staged body. Consumer: the configured command. The git-notes value stores it as a note, and any other value stores it where its author chooses.
- **The appended subject** (new operand) — producer: `--run-validate`, the subject the Statement names. Consumer: the configured command.
- **The store failure** (new exit-2 cause) — producer: `--run-validate`, when the command fails to spawn or exits non-zero. Consumer: the caller through exit 2 and the message naming the published destinations, the channel the failed rename already uses.
- **The git-notes pair** (new documented value) — producer: §bin/run-validate.sh's paragraph and the README. Consumers: an adopter's knob file, the smoke and the gate test.
- **Point 5 (narrowing).** Nothing narrows. A configuration with both knobs empty renders no Statement, as today, and one with only the file set behaves byte-identically. The new exit-2 cause arises only under the new knob.
- **Point 6 (every member).** The members are the readers of the Statement-rendering condition and of the *one command knob* phrase. The condition's one reader is `run`'s `attest` binding (probe: `git grep -n "statement.is_empty" -- native/src/emit/run_validate.rs`). The phrase's one site is §Layout and configuration's `EVIDENCE_KIT_STATEMENT_CMD` bullet (probe: `git grep -n "one \*\*command knob" -- evidence-kit gate-sdk`). Every network-spawner roster names the arms that spawn a Statement command (probe: `git grep -n "EVIDENCE_KIT_STATEMENT_CMD" -- gate-sdk/SPEC.md native/src/emit/mod.rs`), and each gains `--run-validate`.

## Existing sections updated

Roster from `git grep -n -e "EVIDENCE_KIT_STATEMENT_FILE" -e "EVIDENCE_KIT_STATEMENT_CMD" -e "one \*\*command knob" -e "NETWORK_ARMS"` over the tracked tree, the generated mirrors and the queue excluded.

- `evidence-kit/SPEC.md` §Layout and configuration, which gains the knob's bullet and the re-phrased `EVIDENCE_KIT_STATEMENT_CMD` bullet (delta 1). §bin/run-validate.sh's *The Statement* lead and third bullet, its declared knob roster, which gains the knob, and its spawned-program sentence, which gains whatever the write command names (delta 1), plus the git-notes paragraph (delta 2). §Producers and consumers' Statement bullet, which gains the store as a destination, and its command-knob bullet, which gains the write knob (delta 1).
- `docs/evidence-kit/SPEC.md` — the generated mirror, regenerated by `--emit docs-mirror --write` (all deltas).
- `gate-sdk/SPEC.md` §The non-gate arm's network-spawner paragraph gains `--run-validate` through `EVIDENCE_KIT_STATEMENT_WRITE_CMD`, which may store a Statement outside the tree. Its *Changeable by a consumer* bullet's run-validate sentence gains whatever the write command names (delta 1).
- `docs/gate-sdk/SPEC.md` — the generated mirror (delta 1).
- `native/src/knobs/evidence_kit.rs` — an indexed row, `Row::indexed("EVIDENCE_KIT_STATEMENT_WRITE_CMD", &[])` (delta 1).
- `native/src/emit/run_validate.rs` — `KNOBS` gains the knob. The config reads it with `walk::knob_array`. `attest` renders when either destination is set, and after the file's rename `run` spawns the command through `Program::consumer` and `proc::run_streamed`, the body as stdin and the subject appended. Unit cases cover the write-only render, the store's ordering after both renames, and a non-zero exit's message naming the published manifest (delta 1).
- `native/src/emit/mod.rs` — `NETWORK_ARMS` in the fence-safety test gains `--run-validate` (delta 1).
- `native/src/runner.rs` — the `--run-validate` help paragraph names the write knob beside the file (delta 1).
- `evidence-kit/gate-tests/statement-store.test.sh` — new. A scratch repository's run under the git-notes pair stores a note on the subject. `--verify-gate-run` with no operand reads it clean. The worktree reads clean after the store. A second run's note replaces the first. A write command exiting non-zero makes the run exit 2, with the manifest named as published (deltas 1 and 2).
- `evidence-kit/smoke/install.sh` — the Statement block gains a git-notes leg in the installed-adopter shape. With `EVIDENCE_KIT_STATEMENT_FILE` unset and the pair bound, a run stores the note and the verify arm reads it clean through the read knob (delta 2).
- `evidence-kit/README.md` — the Statement sentence names the write knob and points to the git-notes paragraph (deltas 1 and 2).
- `.workflow/release-declarations.md` — *Knob changes* gains `EVIDENCE_KIT_STATEMENT_WRITE_CMD`. *Behavior changes*' `--run-validate` bullet gains the store as a Statement destination (delta 1).

## Retired spellings

- None — the knob and the paragraph are additions. The *one command knob* wording is a re-phrase of a sentence whose claim stops being true, not a removed name.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the write knob, its stdin event, its appended subject and its exit-2 cause.
- [ ] **Instruction surfaces: instruction only** — no template, agent definition or shim is touched.
- [ ] **Merged with no information lost** — each replacement re-phrases the passage it refines; the merged specs read as one document.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls evidence-kit/SPEC-*.md`).
- [ ] **Removals propagated** — none declared; `check-amendment-retired-spelling` stays clean.
- [ ] **Entry moved** — `statement-out-of-tree-store` to Done with `--queue done` in the build batch's merge commit, before the drain stage. Its oracle is local, the evidence-kit fixture suite, the smoke and the battery, so no remote run gates the move.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
