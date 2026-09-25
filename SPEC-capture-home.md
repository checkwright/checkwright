# SPEC amendment: capture-home

An isolated child runs in a linked worktree, and every capture write it makes resolves inside that worktree. The guard's fall-through line, the turn-end liveness record, a knowledge-friction stamp: each lands under the worktree's own `.workflow/` or `.metric/`, and the harness deletes it with the worktree. Close triages the main checkout's capture, so it never reads an isolated session's friction or liveness. The KPIs that count those logs undercount exactly the sessions the delegation protocol says to isolate.

This amendment gives capture one home per clone. A capture writer running in a linked worktree resolves a relative capture path against the **main checkout** instead of its own worktree. Only writes move. An isolated child's reads still see its own tree, as delegation-kit/SPEC.md §The delegation model already rules for untracked surfaces.

**The component span.** The rule belongs to gate-sdk/SPEC.md §The workflow directory, which owns the capture tier. The resolver is a crate helper in `native/src/walk.rs`. The writers belong to guard-kit, delegation-kit, drift-kit and the front end's lesson sink, and delegation-kit's §The delegation model states the isolation-side consequence. So the unit sits at the repo root.

**What was run at authoring** (at `3cd24370`):

- **Reproduced.** This stage's own isolated audit child, running in `.claude/worktrees/agent-…`, had written `.workflow/prompt-friction.log` (2,236 bytes) and `.workflow/subagent-stop-liveness.log` (4,956 bytes) inside its worktree. A sibling child's worktree had already been reaped at that point, and its capture was gone with it.
- **The writers**, by `grep -rln "append(true)" native/src`, by the writers of each gitignored capture knob, and by a read-only sweep over the crate:
  - bare knob paths, which resolve against the process's working directory: `Host::log_fallthrough` in `native/src/guard/host.rs` (`GUARD_KIT_LOG`), `append_attempt` in `native/src/hook/wakeup.rs` (`GUARD_KIT_WAKEUP_LOG`), `append_record` in `native/src/hook/stop_liveness.rs` (`DELEGATION_KIT_STOP_LOG`), `cmd_record` in `native/src/emit/wait_probe.rs` (the wait-primitive evidence under `GATE_SDK_WORKFLOW_DIR`), `fallback` in `native/src/emit/lesson_sink.rs` (`<tag>-harvest.md` under `GATE_SDK_WORKFLOW_DIR`), the history append in `native/src/hook/verdict.rs` (`DELEGATION_KIT_USAGE_HISTORY`), `native/src/emit/overhead_meter.rs` (`DRIFT_KIT_OVERHEAD_LOG`) and `native/src/emit/stage_economics.rs` (`DRIFT_KIT_STAGE_ECONOMICS_LOG`);
  - paths anchored by `file_survey::anchored`, which resolves against `git rev-parse --show-toplevel`, and that is the worktree's own top: `native/src/emit/kfric.rs` (`DRIFT_KIT_KNOWLEDGE_LOG`), `native/src/emit/install_evidence.rs` and `native/src/emit/file_install.rs` (`DRIFT_KIT_INSTALL_RECORD`).
- **The same predicate is written three times, with no shared helper.** "A linked worktree whose common dir is `<main>/.git`" appears in `_gate_main_checkout_bin` (`gate-sdk/lib/gate.sh`), in the private `main_checkout_root` (`native/src/gates/crate_arms.rs`, one `git rev-parse --git-dir --git-common-dir` spawn), and in `Host::roots` (`native/src/guard/host.rs`, two spawns). `native/src/walk.rs` carries only `--show-toplevel` readers.
- **The tests that build a linked worktree** are `gate-sdk/gate-tests/run-gates-linked-worktree.test.sh` and `guard-kit/gate-tests/worktree-confinement.test.sh`. The second sets `GUARD_KIT_LOG` to an absolute sandbox path, so the default's resolution is untested today.

## What changes

### (1) The rule {design-bearing}

**Not yet applied.** gate-sdk/SPEC.md §The workflow directory gains a paragraph after **Two tiers, partitioned by tracking**:

> **Capture has one home per clone: the main checkout.** A writer of a gitignored capture path resolves a relative path against the main checkout's root when it runs in a linked worktree whose common dir is `<main>/.git`, and against its own root otherwise. An absolute path is used as written. A linked worktree is disposable, and the harness reaps an isolated child's worktree when the child ends. A capture line written there is lost before any close reads it, and the sessions it drops are the isolated ones. The rule covers every capture location a kit knob names, the workflow directory's capture tier and a metric directory alike. It moves writes only: a reader keeps its own root, so an isolated child still reads committed state (delegation-kit/SPEC.md §The delegation model). A tracked write from a worktree is the child's own commit and is outside this rule. The resolver is `walk::capture_path`. A consumer's own capture writer outside the crate meets the rule by the same test, `git rev-parse --git-dir --git-common-dir`.
>
> **Honest limits.** The main checkout must be writable from the child; where a sandbox confines the child's writes to its worktree, a best-effort writer drops the line, as it drops any write it cannot open. A worktree of a bare repository has no main checkout and keeps its own capture.

### (2) The resolver {design-bearing}

**Not yet applied.** `native/src/walk.rs` gains two helpers, with unit tests:

- `main_checkout_root()` — the main checkout's root when the process runs in a linked worktree whose common dir's leaf is `.git`, else none. It is `crate_arms.rs`'s single-spawn form, moved: `git rev-parse --git-dir --git-common-dir`, both canonicalized, and linked when they differ. A spawn that fails reads as none, so a capture write falls back to today's resolution rather than failing.
- `capture_path(value)` — an absolute `value` unchanged; a relative one joined onto `main_checkout_root()` when that is some, else resolved as the caller resolves it today.

`native/src/gates/crate_arms.rs` drops its private copy for `walk::main_checkout_root`. `Host::roots` in `native/src/guard/host.rs` takes its `main` field from the helper, keeping its own walk-up for `cwd` and `own`. The shell copy in `gate-sdk/lib/gate.sh` stays, because the shell front end runs before any binary exists.

### (3) The writers adopt it {mechanical}

**Not yet applied.** Each writer from the roster above resolves its target through `walk::capture_path` at the write: the guard's fall-through log, the wakeup log, the turn-end liveness log, the wait-primitive evidence, the lesson sink's fallback file, the usage history, the overhead log, the stage-economics log, the knowledge-friction log and the install record. `kfric.rs`, `install_evidence.rs` and `file_install.rs` move off `file_survey::anchored` for their capture paths. `anchored` stays for its tracked callers (`file_gap.rs`, `file_survey.rs`, `cite_survey.rs`), which this rule does not reach. The arms' printed paths keep the spelling the caller configured.

### (4) The delegation model states the write half {mechanical}

**Not yet applied.** delegation-kit/SPEC.md §The delegation model, after the paragraph that opens **The untracked half reaches no configuration at all, and its disposition is refusal.**, gains:

> **The write half is closed by construction.** A capture line an isolated child writes, such as a guard fall-through, a liveness record or a knowledge-friction stamp, lands in the main checkout's capture tier (gate-sdk/SPEC.md §The workflow directory), so close's triage and the KPIs that count it read the isolated sessions too. Reads are not redirected, and the ruling above stands.

`delegation-kit/templates/agent-execution.md` is unchanged: the child does nothing differently.

### (5) The fixtures {mechanical}

**Not yet applied.**

- `gate-sdk/gate-tests/run-gates-linked-worktree.test.sh` gains two cases run from the worktree. `--emit kfric` lands its line in the main checkout's `.workflow/knowledge-friction.log` and writes nothing under the worktree's. `--lesson-sink` with no sink configured stages into the main checkout's harvest file.
- `guard-kit/gate-tests/worktree-confinement.test.sh` gains a case with `GUARD_KIT_LOG` at its relative default. A fall-through call from the worktree lands in the main checkout's log.
- A case in each run from the main checkout keeps today's path, so the fallback is pinned too.

## Producers and consumers

- **`walk::main_checkout_root` and `walk::capture_path`** — produced by the git spawn at a capture write. Their consumers are the writers in delta 3, `crate_arms.rs` and `Host::roots` (delta 2). Their one field, the root, is read at the write and never stored.
- **The relocated lines** — produced by the writers. Their consumers are unchanged and all run in the main checkout: close's triage templates, `--emit scan-prompts`, `--emit drift-report` with its KPIs, and `--emit close-surfaces`, whose derivation already lists the main checkout's capture tier.
- **Roster-holding readers.** No new name enters a roster a gate holds. The helpers are crate-internal, and the knob tables are untouched.
- **Narrowing check (point 5).** No corpus narrows. The worktree's own capture files stop growing, and no gate reads a worktree's capture.

## Existing sections updated

- gate-sdk/SPEC.md §The workflow directory (delta 1)
- `native/src/walk.rs`, `native/src/gates/crate_arms.rs`, `native/src/guard/host.rs` (delta 2)
- `native/src/guard/host.rs`, `native/src/hook/wakeup.rs`, `native/src/hook/stop_liveness.rs`, `native/src/hook/verdict.rs`, `native/src/emit/wait_probe.rs`, `native/src/emit/lesson_sink.rs`, `native/src/emit/overhead_meter.rs`, `native/src/emit/stage_economics.rs`, `native/src/emit/kfric.rs`, `native/src/emit/install_evidence.rs`, `native/src/emit/file_install.rs` (delta 3)
- delegation-kit/SPEC.md §The delegation model (delta 4)
- `gate-sdk/gate-tests/run-gates-linked-worktree.test.sh`, `guard-kit/gate-tests/worktree-confinement.test.sh` (delta 5)
- the generated `docs/` SPEC mirrors (deltas 1 and 4)

## Retired spellings

- None — the one removal is a private function, `crate_arms.rs`'s `main_checkout_root`, which moves to `walk` under the same name (delta 2).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the helpers and each relocated write.
- [ ] **Instruction surfaces: instruction only** — no template changes.
- [ ] **Merged with no information lost** — each delta re-phrases the text it refines.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` holds, and `check-amendment-retired-spelling` runs it.
- [ ] **The entry moves** — the paired entry moves to Done before the drain stage.
- [ ] **Gaps filed** — any cross-component gap found during the work filed through the gap inbox.
