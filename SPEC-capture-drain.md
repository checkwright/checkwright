# SPEC amendment: capture-drain

A close-stage triage reads a capture log, acts on it, and then truncates it with `: >`. A line a concurrent session appends between the read and the truncate is erased unread. The window is the whole triage, minutes long, and a close runs while other sessions work.

This amendment replaces truncation with **rotation** for every capture log a close procedure reads before draining it. The log is renamed aside before the read, the renamed copy is triaged, and the copy is removed afterwards. Writers open their log by path for each line, so a line written after the rename lands in a fresh log that the next close reads. A new non-gate arm does the rename, so the drain is one allowlisted command on every host.

Offset truncation, the entry's other option, was weighed and refused: cutting a file's read prefix rewrites it in place, and a line appended during the rewrite is lost the same way. It narrows the window, where rotation closes it.

**The component span.** The drain rule is gate-sdk's, which owns the workflow directory's tiers and their reclaim path (gate-sdk/SPEC.md §The workflow directory), and the arm is gate-sdk's too. lifecycle-kit's close-surface derivation learns the drain companion. guard-kit's and drift-kit's triage templates, their SPEC rows and their adopter setup steps change. This repo's close binding and `.gitignore` change too. So the unit sits at the repo root.

**What was run at authoring** (at `3cd24370`), by a read-only sweep over the tracked tree (`git grep -n ': >'`, `truncat`, `reclaim`, and each log's name) and by reading the writers:

- **Four logs are read before they are drained.** The prompt-friction log (guard-kit/templates/close-triage.md step 5) and the wakeup log (step 3, "read …, act …, then delete it") are two. The knowledge-friction log (drift-kit/templates/close-knowledge.md step 3) is the third. The essay-harvest sink (`.claude/commands/close.md`, "merged into the essay, then cleared") is the fourth.
- **Two capture logs are drained with no procedure reading them first**: delegation-kit's `.workflow/subagent-stop-liveness.log` and `.workflow/wait-primitive-evidence.txt`. This amendment leaves their `reclaim=: >` rows alone.
- **Every writer opens by path for each line.** `Host::log_fallthrough` (`native/src/guard/host.rs`), `append_creating` (`native/src/emit/kfric.rs`) and `append_attempt` (`native/src/hook/wakeup.rs`) each call `OpenOptions::new().create(true).append(true)` per write, with no lock.
- **The readers.** `--emit scan-prompts` takes an optional log path operand (`usage: --emit scan-prompts [--count] [--] [<log>]`). `kpi-knowledge-friction` reads three states: absent means no capture loop, present-and-empty means nothing logged.
- **The close-surface derivation lists every gitignored file present in the workflow directory** (`native/src/emit/close_surfaces.rs`, source 2), so a renamed copy present at a commit would read `(undeclared)` and red `check-close-surfaces` assertion A.
- **No gate pins the templates' text.** `check-close-surfaces` checks only that a capture row names some `reclaim=`.
- `.claude/settings.json` allows `bash gate-sdk/bin/run-gates.sh *`, so an `--emit` arm needs no new grant. It also carries exact-string grants for the two `: >` clears this amendment retires.

**Inferred, cannot run before build:** on native Windows, renaming a log that another process holds open for append succeeds, because Rust's standard library opens files sharing delete access — the arm does not exist yet; its test case runs on the native-Windows leg the first time.

## What changes

### (1) The drain rule {design-bearing}

**Not yet applied.** In gate-sdk/SPEC.md §The workflow directory, the header paragraph's first two sentences become:

> **The header requirement follows tracking, and it must.** Local capture drains by emptying the file, whether by rotation or by truncation, and either way leaves no header. A header requirement on that tier would fight the tier's own reclaim mechanism.

A paragraph follows it:

> **A capture log a close procedure reads before draining drains by rotation, never by truncation.** Truncating after a read erases every line appended between the read and the truncate, and a close runs while other sessions write. `--emit capture-drain <log>` (below) renames the log aside to `<log>.drain` and leaves an empty live log in its place. The procedure reads and acts on the drain file, then removes it with `--emit capture-drain --done <log>`. A writer opens its log by path for each line, so a line written after the rename lands in the live log for the next close. A log no procedure reads before draining may still truncate. **Honest limit:** a writer that opened the log just before the rename and writes just after lands its line in the drain file. That line is lost only if the removal runs before the write, which leaves a window of one write rather than one triage.

### (2) `--emit capture-drain` {design-bearing}

**Not yet applied.** A new non-gate arm, `native/src/emit/capture_drain.rs`, with its `--emit-capture-drain` row in `native/src/emit/mod.rs`'s `ARMS`. It declares no knob: the log is an operand. The contract, as a gate-sdk/SPEC.md §The workflow directory subsection after the paragraph above:

> **`--emit capture-drain [--done] [--] <log>`.** Without `--done`, it moves every unread line of `<log>` into `<log>.drain`, creating the drain file if needed:
>
> - With no drain file present, it renames `<log>` to `<log>.drain`.
> - With a drain file present, left by a triage that never finished, it renames `<log>` to `<log>.drain.part`, appends that file's bytes to `<log>.drain`, and removes it.
> - A `<log>.drain.part` found at the start is appended to the drain file first. A crash inside the arm can therefore repeat lines and never drops them.
> - It then opens `<log>` for append, creating it, so the live log is present and empty as a truncate left it. An append-open never truncates, so a line a writer landed after the rename survives.
> - It prints the drain file's path when the drain holds a line, and prints nothing otherwise.
>
> With `--done`, it removes `<log>.drain`; an absent drain file is not an error. Every rename, append and removal failure exits 2 naming the path. A missing operand exits 2 with the usage line. The arm touches only the operand and its two companions.

`gate-sdk/gate-tests/capture-drain.test.sh` (or the crate's unit tests, whichever the arm's siblings use) pins the cases: first drain, a drain with a leftover present, recovery from a `.part`, a line appended after the rename surviving in the live log, `--done` on a present and an absent drain, and the refusals.

### (3) The derivation folds a drain into its row {design-bearing}

**Not yet applied.** In `native/src/emit/close_surfaces.rs` and lifecycle-kit/SPEC.md §The close-surface roster and §The close-surfaces emit arm.

- Source 2 skips a gitignored `<p>.drain` or `<p>.drain.part` whose `<p>` is a declared row. It is that row's drain and never reads as `(undeclared)`.
- A declared row's `<state>` reads `non-empty` when its file or either companion holds content. So a drain an unfinished triage left behind shows on the row the next close walks.

The derivation paragraph of §The close-surface roster gains one sentence after "…therefore close-inbound by definition.":

> A drain file beside a declared log (gate-sdk/SPEC.md §The workflow directory) is part of that log's row rather than a surface of its own.

§The close-surfaces emit arm's `<state>` paragraph gains: "A file row also reads `non-empty` when its drain companions hold content."

### (4) guard-kit's triage and rows {mechanical}

**Not yet applied.**

- `guard-kit/templates/close-triage.md` step 1 opens with the drain: run `--emit capture-drain .workflow/prompt-friction.log` on the gate binary, then `--emit scan-prompts` over the path it printed. Step 3 reads the wakeup log the same way: `--emit capture-drain .workflow/wakeup-attempts.log`, read the printed path, act, then `--done`. Step 5 becomes: **Clear the drained friction log** — `--emit capture-drain --done .workflow/prompt-friction.log`. The exact-string note about bare `: >` clears goes, because the front end's grant covers the arm.
- guard-kit/SPEC.md §The close-stage triage step: "review and delete the wakeup log if present" becomes "drain and review the wakeup log if present", and "clear the friction log" becomes "remove the drained friction log". The row becomes:

  `close-surface: .workflow/prompt-friction.log advisory reclaim=bash gate-sdk/bin/run-gates.sh --emit capture-drain .workflow/prompt-friction.log`
- guard-kit/SPEC.md §wakeup-guard: "reviewed and deleted in the same close-stage triage pass" becomes "drained and reviewed in the same close-stage triage pass", and its row's reclaim takes the same arm over `.workflow/wakeup-attempts.log`.
- `guard-kit/README.md` setup step 3 adds the drain companions to the lines to gitignore, and `guard-kit/smoke/install.sh`'s seeded `.gitignore` gains them.

### (5) drift-kit's triage and row {mechanical}

**Not yet applied.**

- `drift-kit/templates/close-knowledge.md` step 1 becomes: **Walk the log** — run `--emit capture-drain .workflow/knowledge-friction.log` and read the path it prints. Each line's grammar is unchanged. Step 3 becomes: **Remove the drained log** — `--emit capture-drain --done .workflow/knowledge-friction.log`.
- drift-kit/SPEC.md §The knowledge-friction loop, step 2's closing sentences: "Then clear the log — its named reclaim path." becomes "Then remove the drained log — its named reclaim path.", and the row's reclaim takes the arm over `.workflow/knowledge-friction.log`. "the close commit that clears the log" becomes "the close commit that drains the log". The KPI's three states are unchanged, because the arm leaves the live log present.
- `drift-kit/README.md`'s setup names the log to gitignore; its drain companions join that line.

### (6) This repo's binding {mechanical}

**Not yet applied.**

- `.gitignore` gains `.workflow/*.drain` and `.workflow/*.drain.part`.
- `.claude/commands/close.md`'s essay-harvest row takes the arm, and its gloss becomes "drained, merged into the essay, then removed".
- `.claude/settings.json` loses `Bash(: > .workflow/prompt-friction.log)` and `Bash(: > .workflow/knowledge-friction.log)`, which no procedure runs any more. A delegated session prepares that diff, and it is applied on the operator's behalf, as the repo's rule for a permission-settings edit requires.

## Producers and consumers

- **The drain file** — produced by the arm, and only by it. Its consumers are the triage procedure, reading the printed path (`--emit scan-prompts <path>` for the friction log, a direct read for the others), and the close-surface derivation, which folds it into its base row (delta 3). Its one remover is `--done`.
- **The `.drain.part` file** — produced and consumed by the arm alone, and present only across a crash inside it.
- **The printed path** — read by the triage step that ran the arm; an empty print means nothing to triage.
- **Roster-holding readers.** `check-workflow-tiering` needs the companions gitignored, which delta 6 does for this repo and delta 4's README step does for an adopter. `check-close-surfaces` assertion A would red a companion as undeclared until delta 3 folds it. Its assertion C reads the reclaim field's presence only, so the new commands satisfy it. The front end's `--emit` dispatch finds the arm through `ARMS`, and gate-sdk's help text and README arm list gain it.
- **The KPIs.** `kpi-prompt-friction` and `drift-report` read the live log, whose state after a drain is present and empty, as after a truncate. `kpi-knowledge-friction` reads its three states unchanged.
- **Narrowing check (point 5).** Delta 3 narrows source 2's closure: a drain companion of a declared row no longer yields an `(undeclared)` row. `check-close-surfaces` assertion A reds on an undeclared row, so the narrowing can only remove its findings, and no reader asserts a count of rows.

## Existing sections updated

- `gate-sdk/SPEC.md` §The workflow directory (deltas 1 and 2)
- `native/src/emit/capture_drain.rs`, new (delta 2)
- `native/src/emit/mod.rs`, the `ARMS` row (delta 2)
- `lifecycle-kit/SPEC.md` §The close-surface roster and §The close-surfaces emit arm (delta 3)
- `native/src/emit/close_surfaces.rs` (delta 3)
- `lifecycle-kit/gate-tests/check-close-surfaces.test.sh`, a case with a declared log and a present drain reading no `(undeclared)` row (delta 3)
- `guard-kit/templates/close-triage.md` (delta 4)
- `guard-kit/SPEC.md` §The close-stage triage step and §wakeup-guard (delta 4)
- `guard-kit/README.md` (delta 4)
- `guard-kit/smoke/install.sh` (delta 4)
- `drift-kit/templates/close-knowledge.md` (delta 5)
- `drift-kit/SPEC.md` §The knowledge-friction loop (delta 5)
- `drift-kit/README.md` (delta 5)
- `.gitignore` (delta 6)
- `.claude/commands/close.md` (delta 6)
- `.claude/settings.json` (delta 6)
- `docs/guard-kit/SPEC.md`, the generated mirror (all deltas)
- `docs/drift-kit/SPEC.md`, the generated mirror (all deltas)
- `docs/lifecycle-kit/SPEC.md`, `docs/gate-sdk/SPEC.md` and the README mirrors, generated (all deltas)

## Retired spellings

- `: > .workflow/prompt-friction.log` — the friction log's clear, now the arm (deltas 4 and 6).
- `: > .workflow/knowledge-friction.log` — the knowledge-friction log's clear, now the arm (deltas 5 and 6).
- `: > .workflow/wakeup-attempts.log` — the wakeup log's reclaim, now the arm (delta 4).
- `: > .workflow/essay-harvest.md` — the essay sink's reclaim, now the arm (delta 6).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the arm, the drain file and the derivation fold.
- [ ] **Instruction surfaces: instruction only** — the two templates and the close binding carry steps, not grounds.
- [ ] **Merged with no information lost** — each delta re-phrases the text it refines.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every spelling above is gone from the tracked tree, which `check-amendment-retired-spelling` checks.
- [ ] **The entry moves** — the paired entry moves to Done before the drain stage.
- [ ] **Gaps filed** — any cross-component gap found during the work filed through the gap inbox.
