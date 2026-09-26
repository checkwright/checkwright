# SPEC amendment: isolated-capture

`--emit file-gap` and `--emit file-survey` write tracked records, the gap inbox and the survey record, at a path anchored on the running checkout's own top level (`file_survey::anchored`). From an isolated child that top level is the child's linked worktree. The bullet or block lands in the worktree's copy of the record, the read-only child never commits it, and the harness reaps the worktree with the line in it. gate-sdk/SPEC.md §The workflow directory moves *gitignored* capture to the main checkout and leaves a tracked write to "the child's own commit", which a read-only child never makes. The arm exits 0, so the child reports the finding as filed, and its caller believes it was.

**The ruling: the two tracked capture arms refuse in a linked worktree, and the refusal tells the filer to hand the finding back.** An isolated child reports the finding to its dispatcher. The dispatcher runs in the main checkout and files it there, which is where both surfaces already place the producer: the survey record names the parent as its producer, and the gap inbox has the filing session commit its own bullet.

**Refused: routing the write to the main checkout**, as `walk::capture_path` does for gitignored capture. That makes the arm write the main checkout's tracked tree from an isolated child. Isolation exists to prevent exactly that write: the harness refuses the child's file-tool writes into the main checkout, and guard-kit rule `worktree_confinement` refuses its shell commands naming a main-checkout path outside the scratch dir (delegation-kit/SPEC.md §The delegation model). Gitignored capture is scratch-tier, which is why routing is right for it and wrong here. A routed tracked line would also be dirt in the tree a live stage session holds, which its clean-tree preconditions refuse and its `git add` can sweep (lifecycle-kit/SPEC.md §The committed gap inbox), and no session would own its commit.

**The discriminator is the linked worktree, not the isolated child, and the difference is this ruling's honest limit.** An arm sees no agent type. The resolver `walk::main_checkout_root` answers "is this a linked worktree of a non-bare repository", which is the test guard-kit's `worktree_confinement` already applies to decide a session is confined. A session that works in a linked worktree and commits there is refused too, for example an operator whose iteration's home branch is checked out as a worktree of the same clone (lifecycle-kit/SPEC.md §Multi-operator semantics). The raw append stays that session's legal fallback, because the record's grammar is the contract, not the writer. A worktree of a bare repository has no main checkout by the resolver's test, so it files as before.

**Measured at authoring (2026-09-26):**

- **Both arms anchor on their own top level.** `grep -n "anchored" native/src/emit/file_gap.rs native/src/emit/file_survey.rs` shows the inbox resolved at `file_gap.rs:104`, the record at `file_survey.rs:108`, and `anchored` resolving against `walk::toplevel_opt` (`file_survey.rs:42-60`). No tracked-record writer calls `anchored_capture`.
- **The only other `anchored` caller is a reader.** `grep -rn "file_survey::anchored" native/src` finds `native/src/emit/install_evidence.rs`, which reads `DRIFT_KIT_INSTALL_RECORD` to render a projection and writes no record.
- **A harness isolation worktree is a linked worktree of the main clone.** `git worktree list` during this session showed an isolated child's worktree under `.claude/worktrees/agent-<id>` beside the main checkout, which is the shape `main_checkout_root` answers `Some` for.

## What changes

### (1) The capture arms refuse in a linked worktree {design-bearing}

**Not yet applied.** `native/src/emit/file_gap.rs` and `native/src/emit/file_survey.rs` each refuse at exit 2, writing nothing, when `walk::main_checkout_root()` is `Some`. The check runs after the shape and arity refusals, so a usage error still reports first, and before any stamp, resolution or write. One shared function in `file_survey.rs` holds the check and its message, the way `positionals` is already shared. The message names the record and the main checkout, and says:

> this checkout is a linked worktree of `<main>`, so a `<record>` line written here reaches the backlog only through a commit this worktree makes and merges back. An isolated child hands the finding back to its dispatcher, who files it from the main checkout. A session that commits from this worktree appends the line by hand in the record's grammar.

A unit test in each module runs the arm from a linked worktree of a sandbox repository and asserts exit 2, the message, and an unchanged record in both the worktree and the main checkout. A second case runs it from the sandbox's main checkout and asserts the append.

### (2) The owning sections state the refusal {mechanical}

**Not yet applied.** In lifecycle-kit/SPEC.md §The committed gap inbox, the affordance paragraph's opening, "`run-gates.sh --emit file-gap [--] "<gap prose>"` (the `--emit-kfric` pattern: repo-root anchor, config-via-env, exit 2 on an empty argument) appends one dated bullet," becomes:

> `run-gates.sh --emit file-gap [--] "<gap prose>"` (the `--emit-kfric` pattern: repo-root anchor, config-via-env, exit 2 on an empty argument, and exit 2 in a linked worktree) appends one dated bullet,

After the paragraph opening "**The filing session commits its own bullet**", add:

> **An isolated child files nothing; it hands the finding back.** The arm refuses in a linked worktree, telling the filer to report the finding to its dispatcher, who files it from the main checkout. Routing the write to the main checkout, as gitignored capture is routed (gate-sdk/SPEC.md §The workflow directory), is refused: it is the main-checkout tracked write that isolation exists to prevent (delegation-kit/SPEC.md §The delegation model), and no session would own its commit. The arm tests for a linked worktree and cannot see an isolated child, so a session that works and commits in a linked worktree is refused too and appends by hand. A worktree of a bare repository is not a linked worktree by that test.

In the same section's **Producers and consumers** paragraph, "Producer: any mid-iteration session (lead or stage) via `--emit file-gap`" becomes:

> Producer: any mid-iteration session (lead or stage) in the main checkout, via `--emit file-gap` (an isolated child hands its finding back instead)

In lifecycle-kit/SPEC.md §The survey record, the affordance paragraph's "It keeps the repo-root anchor — a relative record path names the same file from any subdirectory, falling back to the working directory outside a repository —" becomes:

> It keeps the repo-root anchor — a relative record path names the same file from any subdirectory, falling back to the working directory outside a repository — refuses at exit 2 in a linked worktree on §The committed gap inbox's ground, since the record's producer is the parent,

### (3) The workflow directory's tracked-write sentence names the refusal {mechanical}

**Not yet applied.** In gate-sdk/SPEC.md §The workflow directory, "A tracked write from a worktree is the child's own commit and is outside this rule." becomes:

> A tracked write from a worktree is outside this rule. It is the child's own commit, and the two tracked capture arms refuse there instead of routing (lifecycle-kit/SPEC.md §The committed gap inbox).

## Producers and consumers

- **The refusal** (delta 1). Producer: either arm, run from a linked worktree of a non-bare repository. Nothing enables it: `main_checkout_root` answers from git alone. Consumer: the filing session, on stderr at exit 2, which hands the finding back or appends by hand. Its dispatcher then files the handed-back finding through the arm from the main checkout, where it already owns the commit (§The committed gap inbox, §The survey record).
- **Roster readers.** No gate reads the arms' output. `check-survey-record` and `check-gap-inbox-neutrality` read the records, and a refusal writes nothing to either. The on-site mirrors of lifecycle-kit/SPEC.md and gate-sdk/SPEC.md re-render.
- **Point 5.** No corpus narrows: the arms' write set shrinks, and no reader reds on an absent line or counts lines.
- **Point 6.** Not obliged.

## Existing sections updated

Roster from `grep -rn "anchored" native/src --include=*.rs`, `grep -n "tracked write from a worktree" gate-sdk/SPEC.md` and `grep -n "Producer: any mid-iteration session\|It keeps the repo-root anchor" lifecycle-kit/SPEC.md`, run 2026-09-26.

- `native/src/emit/file_gap.rs` and `native/src/emit/file_survey.rs` (delta 1).
- lifecycle-kit/SPEC.md §The committed gap inbox and §The survey record (delta 2).
- gate-sdk/SPEC.md §The workflow directory (delta 3).
- The on-site mirrors of `lifecycle-kit/SPEC.md` and `gate-sdk/SPEC.md`, regenerated by `check-docs-mirror-fresh`'s printed command (deltas 2 and 3).
- `.workflow/release-declarations.md`: one Behavior changes bullet naming `--emit file-gap` and `--emit file-survey` (delta 1). Both refuse in a linked worktree and steer an isolated child to hand the finding back.

## Retired spellings

- None — no delta renames or deletes a spelling.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness checklist holds for the refusal.
- [ ] **Instruction surfaces: instruction only.** No template changes. The refusal message carries the act, and the grounds sit in delta 2's SPEC text.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines. The one added paragraph has no passage to rewrite.
- [ ] **Amendment deleted.** This file is removed on merge (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Entry moved.** `isolated-tracked-capture-lost` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
