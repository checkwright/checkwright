# SPEC amendment: rewrite-arm

Queue entry: `rewrite-arm`, in unit set `guard-friction-reach` (operator direction, 2026-09-15,
lead-relayed). The unit was split from `in-place-rewrite-steer-reach`, whose rule 8 widening
(`guard-kit/SPEC-perl-rewrite-steer.md`) steers to this arm, so **this unit builds first**.

A root-level amendment, because it spans four components:

- guard-kit owns the arm's contract, beside §scratch-run, the other steer target the ruleset prints.
- `native/` holds the module, a start-offset search on the ERE matcher, and a shared predicate
  factored out of the workflow-state hook.
- gate-sdk owns the non-gate arm roster and the ERE matcher's section.
- lifecycle-kit owns the state file the arm refuses to write.

**What the direction asks, and the shape that answers it.** Agents are to be steered off interpreters
and utilities whose side effects cannot be read, onto predictable tools checkwright builds in Rust. An
in-place `perl` or `sed` can do anything its program says: read, write elsewhere, execute. The arm
does one thing whose effect is fully determined by its command line: in the named files, replace each
match of one pattern with one fixed text. It writes nothing else, runs nothing, and prints every span
it changed.

**Two forms were weighed and refused.**

- **Capture-group replacement** would need a substitution engine inside the matcher.
  gate-sdk/SPEC.md §The POSIX ERE matcher sized that engine out of the owed work, and a backreference
  makes the output depend on the matched text, which is the unpredictability the direction names.
- **A pattern read from a file** would make the program invisible on the command line, the hazard
  rule 23 blocks for scripts.

Both fall to the Edit tool or to `!<command>`, and the steer says so.

## The seam

- **Kit mechanism:** the arm, its contract section, its tests, a start-offset search on the crate's ERE
  matcher, and a path predicate shared with lifecycle-kit's hook. Nothing in the arm names a project,
  a vocabulary or a path beyond the kit-owned state file.
- **Consumer config:** none added. The arm reads `GATE_SDK_WORKFLOW_DIR`, an existing gate-sdk knob,
  to resolve the state file.
- **Private rule content:** none in reach.

## What changes

### (1) guard-kit/SPEC.md §rewrite, the arm's contract {design-bearing}

A new `## rewrite` section after §scratch-run. **Not yet applied:**

> `--rewrite [--regex] [--expect <n>] [--] <find> <replace> <file>…` replaces text in the named files
> and does nothing else. It is reached through the front-end as
> `bash gate-sdk/bin/run-gates.sh --rewrite …`, and it is the steer target of rule 8's in-place arms.
> The `--emit-` prefix is load-bearing (gate-sdk/SPEC.md §The non-gate arm) and this member writes
> files, so it is spelled bare, as `--scratch-run` is.
>
> **What a match is.**
> - **Without `--regex`**, `<find>` is a literal matched over each file's whole content, so a match
>   may span lines. Three escapes are read in both `<find>` and `<replace>`: `\n` (newline), `\t`
>   (tab) and `\\` (backslash). Any other backslash is literal. A multi-line edit can therefore stay
>   on one command line.
> - **With `--regex`**, `<find>` is a POSIX extended regular expression (gate-sdk/SPEC.md §The POSIX
>   ERE matcher), applied line by line. A match never spans a newline, and `^` and `$` anchor at the
>   line's ends. Matches are leftmost-longest and non-overlapping, scanning on from each match's end.
>   An empty match advances one byte.
> - `<replace>` is always literal text after the three escapes. There are no backreferences and
>   nothing is evaluated.
>
> **What it reports.** For every replaced span the arm prints `<file>:<line>:`, the original lines the
> span touched, each prefixed `-`, and the resulting lines, each prefixed `+`. It then prints one
> summary line: `rewrite: <n> replacement(s) in <k> of <m> file(s)`. A file with no match is named on
> a line of its own and left untouched.
>
> **Exit status.**
> - **0** — at least one replacement was written.
> - **1** — nothing was written, for one of two reasons: no operand matched, or `--expect <n>` was
>   given and the total differs from `<n>`. `--expect` is the Edit tool's missing-or-ambiguous check
>   carried to a sweep.
> - **2** — a refusal. The usage (on stderr), an unparseable pattern, or an operand refused below.
>   Every refusal happens before the first write.
>
> An unrecognized `-`-prefixed argument before `--` is a refusal, and `--` ends option processing, so
> a `<find>` beginning with `-` stays reachable (gate-sdk/SPEC.md §The bin/-tool contract's shape
> half). Help lives at the refusal and in guard-kit/README.md, as for §compare-settings-allow.
>
> **An operand is refused, and nothing is written, when any of these holds:**
> - it is not an existing regular file, or it is a symlink;
> - its canonical path is not inside the canonical working directory, which the front-end sets to the
>   repository root (both sides resolved through `walk::canonicalize`, gate-sdk/SPEC.md §The crate's
>   crosser);
> - a component of its path is `.git`;
> - it resolves to the lifecycle state file (lifecycle-kit/SPEC.md §check-stage-evidence, the
>   `workflow-state-guard` paragraph), tested with that hook's own predicate;
> - its content, or the content the rewrite would produce, is not UTF-8 or carries a NUL byte;
> - it names the same canonical file as an earlier operand.
>
> **The write.** Every operand is read, checked and rewritten in memory before the first byte is
> written. Each changed file is then written to a temporary file in its own directory and renamed over
> the original, with the original's permission bits. An I/O error part-way through is exit 2, and the
> message names the files already replaced, since a rename already done is not undone.
>
> ### The rewrite arm's security posture
>
> **Its reach is narrower than what it replaces, and the grant it rides is not widened.** The arm is
> reached through the front-end grant a consumer already holds, so it needs no settings edit. That is
> the shape §scratch-run refuses for a second interpreter, and the difference is what the two widenings
> would add. Teaching the runner a second interpreter turns *run bash on a reviewed body* into *run
> anything*. But the front-end grant already reaches `--scratch-run`, and a bash body run there reaches
> `sed -i` and `perl -pi` on any path the process can write. So this arm adds **no capability** that
> grant lacks. What it adds is a **narrower form** of one capability the grant already has: bounded to
> the repository tree, refusing `.git` and the state file, executing nothing, and deterministic in its
> effect.
>
> **The compensating control is the printed span, and it relocates review rather than removing it.**
> A rewrite granted through the front-end is not decided before it runs. Its before-and-after lines
> land in the transcript as it runs, and the working tree's diff stands until commit. That is
> §scratch-run's echo posture, applied to an edit rather than a body.
>
> **A path guard on the harness's file tools does not see this arm.** A `PreToolUse(Write|Edit)` hook
> reads a file-tool call and never a Bash command. That is why the arm refuses the state file itself,
> through the predicate the kit's one shipped path guard uses: rule 8 now steers an in-place rewrite
> here first, where it used to steer only to the Edit tool, which that guard covers, and the steer
> must not open the file the guard exists for. **The honest limit:** a consumer's own `Write|Edit` path guard is not consulted,
> and a file it protects is rewritable here as it already is by `--scratch-run`. Such a consumer's
> protection is only as wide as its Bash rules.
>
> **What the grant now reads as.** A consumer who reads the front-end grant as "run the gates" is also
> granting silent, bounded rewrites of tracked files. It already granted unbounded ones through
> `--scratch-run`, so a consumer cannot separate the decision without declining the battery. That is
> the taken cost §scratch-run records, recorded again here for this member.
>
> **The matcher cannot be made to run away.** It is a Thompson-construction matcher with a bounded
> program (gate-sdk/SPEC.md §The POSIX ERE matcher), so a pattern costs at most linear memory and
> polynomial time per line, and a construct outside the grammar is a refusal.

### (2) The crate module and its arm-table row {design-bearing}

`native/src/emit/rewrite.rs`, with `pub fn run(args: &[String]) -> i32` and
`pub const KNOBS: &[&str] = &["GATE_SDK_WORKFLOW_DIR"]`. An `Arm::Run` row `"--rewrite"` goes in
`native/src/emit/mod.rs` beside `--scratch-run`. It is an `Arm::Run` because its contract is the 0/1/2
split, which an emitting arm collapses. It is a table member because it resolves a consumer knob.

The module implements delta 1 exactly:

- It parses the arguments.
- It validates operands in argument order, taking containment from `walk::canonicalize` on both sides
  with scratch_run.rs's `inside` comparison.
- It computes every new content before writing, using `Ere::find_from` for `--regex`.
- It writes through a temporary sibling and a rename, preserving the mode.

It spawns no program. The `.git`-component and symlink tests read the operand as given (via
`symlink_metadata`) before canonicalizing.

### (3) The ERE matcher gains a start-offset search {mechanical}

`native/src/ere.rs`: `pub fn find_from(&self, hay: &str, from: usize) -> Option<(usize, usize)>` is
the leftmost-longest match starting at or after `from`. It is `find`'s loop over
`start in from..=len`, and `^` still holds only at position 0 of `hay`. `find` becomes
`find_from(hay, 0)`. The in-crate tests gain three cases:

- `^a` against `aa` from offset 1 finds nothing;
- `a*` from offset 2 of `baa` returns the span (2, 3);
- an empty-match pattern from `len` returns (len, len).

gate-sdk/SPEC.md §The POSIX ERE matcher, **not yet applied**: after the paragraph ending `and no
substitution engine or capture-group replacement**`, add:

> A caller that substitutes composes the matcher's spans itself. `--rewrite` (guard-kit/SPEC.md
> §rewrite) replaces each leftmost-longest span with literal text through a start-offset search,
> `find_from`, which the matcher carries for that caller alone. The engine still owns no substitution
> and no capture group.

### (4) The workflow-state predicate is shared, and lifecycle-kit names the second reader {mechanical}

`native/src/hook/workflow_state.rs`: the comparison inside `run` becomes
`pub fn is_state_file(path: &str) -> Result<bool, String>`. It reads `GATE_SDK_WORKFLOW_DIR` and
compares `resolve(path)` with `resolve(<dir>/WORKFLOW-STATE.txt)`. The hook keeps its fail-open advise
on `Err`. The arm treats `Err` as a refusal (exit 2), because a writer that cannot tell whether it is
about to write the state file must not write.

lifecycle-kit/SPEC.md §check-stage-evidence, the residual paragraph beginning `The residual is stated
because`. **Not yet applied:** after its first sentence add:

> One Bash writer is closed by construction rather than by the hook: guard-kit's `--rewrite` arm
> refuses an operand resolving to the state file through this hook's own predicate
> (guard-kit/SPEC.md §rewrite), since rule 8 steers in-place rewrites to that arm as well as to the
> file tools this hook reads.

### (5) Tests {mechanical}

- **In-crate, in `rewrite.rs`:**
  - literal replacement across a line boundary with the `\n` escape;
  - `--regex` anchoring per line, and an empty match advancing;
  - `--expect` mismatch writing nothing;
  - no match exiting 1 and writing nothing;
  - a non-UTF-8 result refused;
  - a duplicate operand refused;
  - the printed `-`/`+` lines for a two-line span.
- **Front-end, `guard-kit/gate-tests/rewrite.test.sh`**, in a `mktemp -d` git sandbox holding a
  relocated workflow dir:
  - a two-file literal sweep rewrites both and exits 0;
  - an operand outside the sandbox, a symlink operand, a path through `.git`, and the sandbox's
    `WORKFLOW-STATE.txt` (reached through `GATE_SDK_WORKFLOW_DIR`) each exit 2, with every file
    byte-identical afterwards, including a matching valid operand listed before the refused one;
  - an unrecognized `-x` exits 2, and `-- -x y file` rewrites a literal `-x`.

### (6) Rosters and the install and use surfaces {mechanical}

- gate-sdk/SPEC.md §The non-gate arm: the `Arm::Run` members list gains
  `` `--rewrite` (guard-kit/SPEC.md §rewrite) `` after `--scratch-run`'s entry.
- guard-kit/SPEC.md §Layout and configuration: the tree gains
  `gate-tests/rewrite.test.sh  # the --rewrite arm's operand refusals and write, run by gate-sdk's runner`.
- guard-kit/README.md §Use gains
  `bash gate-sdk/bin/run-gates.sh --rewrite [--regex] [--expect <n>] [--] <find> <replace> <file>…  # replace text in tracked files, printing every changed span`.

## Producers and consumers

- **The rewritten files** — producer: `rewrite::run`, reached through the front-end by an agent the
  rule 8 steer sent there (`SPEC-perl-rewrite-steer.md`), or called directly. The enabling config is
  the front-end grant, present in this repo's committed settings and recommended by
  `templates/settings-allow.json`. Consumers: the working tree, and the commit that reviews its diff.
- **The printed spans and summary** — consumer: the session's transcript, the arm's compensating
  control, read by the agent to confirm the change and by a supervisor reviewing the run. Each field
  has that reader: `<file>:<line>` to locate, the `-`/`+` lines to review, and the summary counts to
  compare with `--expect`.
- **`--expect`** — reader: the arm's own exit-1 decision.
- **`find_from`** — callers: `find` and `rewrite::run`.
- **`is_state_file`** — callers: the hook and the arm.
- **The arm-table row** is a new member of the knob-roster derivation: its declared knob
  (`GATE_SDK_WORKFLOW_DIR`) is what `check-reads-couples`, `check-gate-substrate-parity` and the
  knob-file derivation read for a member. The build runs the battery to find which of them wants a
  row, never predicting it.
- **Point 5:** no corpus narrows. `find`'s behaviour is unchanged, and its readers keep their
  verdicts: the three ERE cohort gates, whose fixture pairs the battery runs.

Derivation of the rosters here and below:
- `git grep -n -e "--scratch-run"` over the tracked tree, excluding the queue and docs, for where an
  arm of this class is named: the non-gate arm roster, the guard-kit layout, the README, the gate test
  and the table row;
- `git grep -n "Ere::\|ere::"` over `native/src` for the matcher's callers;
- `git grep -n "workflow-state-guard"` over the SPECs;
- a read of guard-kit/SPEC.md §scratch-run for the grant posture this arm answers.

The build unit re-derives them.

## Existing sections updated

- `guard-kit/SPEC.md` — §rewrite and §The rewrite arm's security posture, new (delta 1); §Layout and
  configuration's tree (delta 6).
- `native/src/emit/rewrite.rs` — new; `native/src/emit/mod.rs` — the table row (delta 2).
- `native/src/ere.rs` — `find_from` and its tests (delta 3).
- `gate-sdk/SPEC.md` — §The POSIX ERE matcher (delta 3) and §The non-gate arm's `Arm::Run` list
  (delta 6).
- `native/src/hook/workflow_state.rs` — `is_state_file` (delta 4).
- `lifecycle-kit/SPEC.md` — §check-stage-evidence's residual paragraph (delta 4).
- `native/src/emit/rewrite.rs` tests and `guard-kit/gate-tests/rewrite.test.sh` — new (delta 5).
- `guard-kit/README.md` — §Use (delta 6).
- `docs/guard-kit/SPEC.md`, `docs/gate-sdk/SPEC.md` and `docs/lifecycle-kit/SPEC.md` — the on-site SPEC
  mirror, regenerated with `--emit docs-mirror --write` (deltas 1, 3, 4 and 6).
- `.workflow/release-declarations.md` — one guard-kit bullet: a new `--rewrite` arm replaces text in
  files inside the repository through the front-end. A consumer's front-end grant now reaches bounded
  rewrites of tracked files, and it already reached unbounded ones through `--scratch-run`. The bullet
  says so, and no action is required (delta 2).

## Retired spellings

- None — no delta retires a spelling. `find` survives as `find_from(hay, 0)`, and `run` in the
  workflow-state hook keeps its name.

## Definition of Done

- [ ] **Causal completeness** — the arm, its printed spans, `find_from` and `is_state_file` each have a
      named producer and named consumers, and the table row's roster readers are run, not predicted.
- [ ] **Instruction surfaces: instruction only** — the README usage line carries no grounds, and delta 1
      places them.
- [ ] **Security posture merged** — §The rewrite arm's security posture lands with the arm, not after
      it.
- [ ] **Merged with no information lost** — each SPEC addition integrated, not appended.
- [ ] **Amendment deleted** — this file removed on merge.
- [ ] **Roster re-derived** — rosters re-derived against the tree before the merge counts as complete,
      with any missed site landed and named in the commit.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any cross-component gap found while landing filed through the gap inbox.
