# SPEC amendment: stamp-subject

`stage-commit-subject-scope-unowned`: no surface governs the subject scope of a
stage session's commits, so each session copies one from history. A recovery keyed on
a stage name (`git log --grep`, a pickaxe) then silently misses the commits that
stage filed under another scope.

**Measured at authoring, 2026-09-19.** The commit subjects that touched
`.workflow/WORKFLOW-STATE.txt` since 2026-09-01 (`git log --since=2026-09-01
--format=%s -- .workflow/WORKFLOW-STATE.txt`, scope token tallied) carry eight
scopes, and 38 of 571 carry `chore(workflow)` or `chore(lifecycle)` instead of
the stage. Over all history the same tally gives `chore(lifecycle)` 216 and
`chore(workflow)` 94. The drift is live, not historical.

**Ruling: candidate (a) plus the gate (b), scoped to the stamp commit.** The entry's
three candidates:

- **(a) State the scope as a per-stage fact.** Taken, but a stated fact is what
  already failed to exist, and prose alone is the control this repo has measured
  failing elsewhere. It lands with its writer and its check.
- **(b) A gate binding a stage session's commits to its stage.** Taken, narrowed to
  the one commit the gate can identify without a session roster: **a commit that
  adds a stamp line**. A stage session's other commits are unit work, whose scope is
  the component they touch (`feat(lifecycle-kit)`). They carry no stamp, so nothing
  tells the gate which stage made them, and binding them to the stage name would
  lose the component. The recovery the entry prices keys on the stamp commit, "the
  stage entered at X", which is what the tabulation measures.
- **(c) Rule it cosmetic.** Refused. The cost is a silent miss in a recovery, not a
  style difference.

**The writer is the entry tool.** `--enter-stage` already prints the `next: commit`
line a session follows. That line names the subject, `chore(<stage>): stamp the
<stage> stage entry`, so the compliant spelling is the one the session is handed.
The gate is the asserter, the same writer/asserter split §The stamp protocol draws
between `--enter-stage` and the stage gates.

**What the gate reads.** A commit-msg gate has the message file (`$1`) and the index.
The stamps a commit adds are the data lines of the staged state file absent from
`HEAD`'s. That one read covers the ordinary entry, the boundary reset (which
truncates, then stamps), the valve-admitted entry and `--rename` (whose rewritten
lines are all the first stage's). The gate takes the **last** added line's stage,
since that is the cursor the commit leaves. A waiver line (`LIFECYCLE_KIT_WAIVER_TOKEN`)
is not a stamp and is skipped.

**Probed at authoring, 2026-09-19.** `grep -rn "chore(<stage>)\|subject scope"
lifecycle-kit canon-kit queue-kit` returns nothing. `grep -n "next: commit"
native/src/emit/enter_stage.rs` returns lines 444, 1171, 1183 and 1192, none naming
a subject.

## What changes

### (1) §The stamp protocol states the subject scope {design-bearing}

**Not yet applied.** In `lifecycle-kit/SPEC.md` §The stamp protocol, the sentence
"Committing the stamp remains the skill's business, on its own" is re-phrased to
say that the commit adding a stamp carries the stamped stage as its subject scope
(`<type>(<stage>): …`). `--enter-stage` prints that subject and
`check-stamp-subject` holds it. The ground: a recovery keyed on the stage name finds
the entry, and a stage session's other commits keep their component scope because
nothing identifies their stage.

### (2) `--enter-stage` prints the subject in its `next:` line {design-bearing}

**Not yet applied.** Each `next: commit …` line in `native/src/emit/enter_stage.rs`
that follows a stamp write names the subject: `chore(<stage>): stamp the <stage>
stage entry` for an entry, the same with the boundary wording for the reset, and
`chore(<first stage>): name the iteration <name>` for `--rename`. Nothing changes
under `--simulate`, which writes nothing and prints no `next:` line. §bin/enter-stage.sh
states the printed subject in the sentence that owns the tool's report.

### (3) The `check-stamp-subject` gate {design-bearing}

Born native: a Rust module under `native/src/gates/`, a
`lifecycle-kit/checks/check-stamp-subject.gate` descriptor at `tier=commit-msg`, a
`good/`+`bad/` fixture pair, and a `scripts/gates.list` registration. A new
`lifecycle-kit/SPEC.md` §check-stamp-subject section owns:

- **Invariant.** When the staged state file (`LIFECYCLE_KIT_STATE_FILE`) adds data
  lines whose stage field is a `LIFECYCLE_KIT_STAGES` member, the subject line of
  the message file parses with a `<scope>` equal to the last such line's stage.
  Parsing is `check-commit-subject`'s grammar (gate-sdk/SPEC.md §check-commit-subject).
- **Skips, clean.** A commit adding no stamp line. A no-argument run (the whole-tree
  battery), on `check-commit-subject`'s precedent, since the message is not a
  tracked surface. A tree with no state file.
- **Fail-closed.** A message-file argument naming a missing file exits 2.
- **Red message.** It names the expected subject scope, the stamp line it read the
  stage from, and the tool's printed subject as the fix.
- **Fixtures.** The fixture supplies the staged and `HEAD` state-file blobs as
  files, so the pair runs hermetically without a git index. This follows
  `check-trajectory-fresh`'s synthetic second argument (drift-kit/SPEC.md §The
  published-evidence extractor). The `bad/` case is an align stamp committed under
  `chore(workflow)`. The `good/` case covers an entry, a boundary reset and a
  waiver line.

### (4) Rosters and the generated hooks {mechanical}

The gate's registration fans out to the generated projections that roster gates:
`scripts/git-hooks/commit-msg`, `docs/check-graph.html`, `docs/enforcement.md`,
the kit README's gate list and the gate-timing baseline. Each freshness gate prints
its regeneration command, and build re-derives the roster by
`git grep -l check-stage-skill-coverage`, the precedent member.

### (5) The generated mirrors {mechanical}

`docs/lifecycle-kit/SPEC.md` and `docs/lifecycle-kit/README.md` are regenerated after
deltas 1-4.

## Producers and consumers

- **The printed subject** (delta 2). Producer: `--enter-stage` on every stamp write.
  It is reached on the default path, needing no knob. Consumer: the entering session
  at its stamp commit.
- **The gate** (delta 3). Producer of its input: the staged state file, written by
  `--enter-stage`, and the message file from the commit-msg hook. Consumer: the
  generated `commit-msg` hook, installed per clone by `--install-hooks`. Roster
  readers: `check-graph`, the generated hook, the enforcement roster page and the
  gate-timing baseline (delta 4). `check-lifecycle-registration` reads the
  CLAUDE.md block, which names no gate, so it is unaffected.
- **Every field read.** The stage field of each added line is read, and only the
  last one decides. The message's scope token is read. Nothing else is.
- **Point 6.** The corpus is the stamp-writing paths of `--enter-stage`: entry,
  boundary reset, valve-admitted entry and `--rename`. Values: the entry prints
  `chore(<stage>): stamp the <stage> stage entry`. The reset prints the same with
  `scope` and the boundary wording. The valve entry prints the entry subject.
  `--rename` prints `chore(<first stage>): name the iteration <name>`.

## Existing sections updated

Roster probe: `grep -n "next: commit" native/src/emit/enter_stage.rs` and
`git grep -l check-stage-skill-coverage`.

- `lifecycle-kit/SPEC.md` §The stamp protocol (delta 1)
- `lifecycle-kit/SPEC.md` §bin/enter-stage.sh and `native/src/emit/enter_stage.rs`
  (delta 2)
- `lifecycle-kit/SPEC.md` §check-stamp-subject, new (delta 3)
- `lifecycle-kit/README.md`, `scripts/gates.list`, `scripts/git-hooks/commit-msg`,
  `docs/check-graph.html`, `docs/enforcement.md` and
  `.workflow/gate-timing-baseline.txt` (delta 4)
- `docs/lifecycle-kit/SPEC.md` and `docs/lifecycle-kit/README.md` (delta 5)

## Retired spellings

- None — no name is retired. The gate and the printed subject are additions.

## Definition of Done

- [ ] **Causal completeness** — every point holds for the printed subject and the
      gate.
- [ ] **Four gate contracts** — output, fail-closed, fixture pair and self-lint
      (gate-sdk/SPEC.md), with the meta-gates green.
- [ ] **`bash gate-sdk/bin/build-native.sh` plus the battery**, both green.
- [ ] **Merged with no information lost** — deltas 1-2 re-phrase their paragraphs.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component, discharged at the iteration.
- [ ] **Done move** — the paired entry moves to Done in the merge commit, before the
      drain stage.
- [ ] **Release declaration** — a new gate is declared in the unit that lands it.
- [ ] **Gaps filed** — any cross-component gap found during the work filed.
