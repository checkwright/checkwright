# SPEC amendment: hold-cause

Queue entry: `validate-hold-rule-admits-iteration-caused-red`, a unit of `validate-red-holding`.

A root-level amendment, because it spans three components:

- evidence-kit owns the baseline grammar and `check-evidence-baseline` (deltas 1 and 2).
- lifecycle-kit owns the validate template and its valve paragraph (delta 3).
- The repo root owns this consumer's validate binding and its seeded baseline (deltas 4 and 5).

**The shape spec picked, and why.** The entry left two shapes open: a template sentence, or an
enforced rule that a pass-to-fail flip names a cause outside the iteration's commits. This takes
the enforced one, because the template sentence is what the instance already had. The validate
template said a regression must not be held, and the session held one anyway. It read a filed
entry that matched the red as the red's cause, and nothing on the committed surface showed that
judgment. A lead reading the diff was the only thing that caught it.

A gate cannot decide cause. The claim a gate can check is narrower: the red **reproduces** at a
named commit that is not one of the iteration's own. The session gets that commit by running the
suite at the iteration-start commit or earlier. So a hold on a row that passed when the iteration
opened now needs a recorded reproduction, and the gate checks that the commit is outside the
iteration. It cannot check that the red really reproduces there. A false commit is a deliberate
false claim in one token of a diff the lead already reads. That turns a quiet slip into a visible
act, which is as far as the guarantee goes.

**What the survey found.** No survey was bought. The probes, all at `66682ce6`:

- `evidence::diff` (`native/src/evidence.rs`) reads fields one to three of a baseline row and
  ignores the rest. `check-evidence-baseline` reds a fifth field as "too many fields".
- `stages::iteration_start` already returns the iteration-start commit and verifies it resolves.
  It takes the state-file path from its caller, so an evidence-kit reader needs no lifecycle-kit
  dependency. `EVIDENCE_KIT_STATE_FILE` is already a declared knob.
- This repo's baseline carries no `fail` or `ignore` row, and it is unchanged since the
  iteration-start commit `af66137c`. So the new assertion lands clean here.
- CI's `gates` job checks out with `fetch-depth: 0`, so the iteration-start commit resolves there.

## The seam

- **Kit mechanism:** the token, the flip assertion and its early-outs, and the template's triage
  rule. None of them names a project, a suite or a vocabulary.
- **Consumer config:** no new knob. The assertion reads the existing `EVIDENCE_KIT_STATE_FILE`, and
  a consumer running no lifecycle has no iteration-start commit, so the assertion is disarmed.
- **Private rule content:** none. The instance that motivated this stays in git history and the
  queue entry. No kit SPEC cites it.

## What changes

### (1) A baseline row may record where its red reproduces

The baseline row grammar gains an optional fifth token, `reproduces-at=<rev>`, allowed only on a
`fail` or `ignore` row {design-bearing}.
**Not yet applied.**

- `<rev>` is 7 to 40 lowercase hex characters, the shape `--enter-stage` writes a head in.
- The token claims that the row's red reproduces at `<rev>`. Delta 2 says when a row needs it.
  On any other `fail` or `ignore` row it is optional, and it is still checked when present.
- A `pass` row that carries it is red. A promotion rewrites the row, so the token goes with it.
- A fifth field of any other shape stays a grammar error. So does a sixth field.
- The token is optional so that no existing row needs a migration, and a baseline with no flips
  never mentions it.

**Replacement text, evidence-kit/SPEC.md §Baseline manifest** (**Not yet applied**). The first
paragraph's grammar sentences become:

> Below it, one line per known scenario,
> `<suite> <scenario> <status> [<slug> [reproduces-at=<rev>]]`. A blocking `<slug>` is required
> exactly when status is `fail` or `ignore` and forbidden when `pass`; each slug resolves to a live
> queue task (the queue-file knob) or a configured permanent marker. `reproduces-at=<rev>` records
> a commit at which the row's red reproduces. It is allowed only after a slug, and
> §check-evidence-baseline says when a row owes it.

The header's em-dash tail carries the same widened grammar.

### (2) `check-evidence-baseline` reds a flip with no reproduction outside the iteration

`check-evidence-baseline` gains a **flip assertion**. A row that is `fail` or `ignore` now, and
whose scenario was `pass` in the baseline at the iteration-start commit, must carry
`reproduces-at=<rev>`. `<rev>` must be an ancestor of that commit or the commit itself
{design-bearing}.
**Not yet applied.**

- **Arming.** The iteration-start commit comes from `stages::iteration_start`, called with the
  state-file path (a new third positional, defaulting to `EVIDENCE_KIT_STATE_FILE`, the same
  `$1 $2 $3` shape as `check-evidence-manifest`). When there is no iteration-start commit, the
  assertion is off at a declared early-out. That covers every case lifecycle-kit/SPEC.md
  §The state machine lists, a clone that cannot resolve the commit included. The clean line says
  which way it went.
- **The prior baseline.** The gate reads the baseline file at the iteration-start commit. If the
  path does not exist at that commit, every row counts as new, so there are no flips. Any other
  git failure is fail-closed, exit 2.
- **Corpus and matching.** A flip is a `(suite, scenario)` pair that is `pass` in the prior
  baseline and `fail` or `ignore` now. A scenario missing from the prior baseline is not a flip.
  That is a non-target: a new scenario that is red from the start can be a new test for an old
  defect, and it needs its own argued change.
- **Red conditions.** (a) A flip row with no token: "`<suite> <scenario>` passed at the iteration
  start `<start>` and is now `<status>`; a hold needs `reproduces-at=<rev>` at or before `<start>`,
  or the red is a regression to fix". (b) Any token whose `<rev>` does not resolve to a commit.
  (c) Any token whose `<rev>` is not an ancestor of the start commit, or the commit itself
  (`git merge-base --is-ancestor`): "names a commit inside the iteration". (b) and (c) apply to
  every token, not only flip rows. A token that was valid in an earlier iteration stays valid,
  because each iteration starts after the one before.
- **What is not checked.** Whether the red really reproduces at `<rev>`. The SPEC text below says
  so.
- **Help text.** The findings help widens to name the token and the flip rule.
- **Tests.** The good/bad pair's `args` gain a third path that does not exist, so both fixtures hit
  the early-out and their verdicts do not change. `gate-tests/check-evidence-baseline.test.sh`
  gains a scratch-repository case for each red condition and each early-out, including a
  prior-baseline path that is missing at the start commit.
- **Graph manifest.** `checks/check-evidence-baseline.gate`'s `couples=` adds the state file,
  `native/src/stages.rs` and `knob:EVIDENCE_KIT_STATE_FILE`. The pre-commit hook is regenerated.

**Replacement text, evidence-kit/SPEC.md §check-evidence-baseline** (**Not yet applied**). The
invariant paragraph's assertion list gains a final clause, and a paragraph follows it:

> … and **flip causation**: a row held red that passed when the iteration opened carries
> `reproduces-at=<rev>` naming a commit at or before the iteration-start commit. Argument mode
> `$1 $2 $3` (baseline, queue, state) …
>
> **A filed task that matches a red is not what caused it.** A regression this iteration
> introduced can match a red someone filed earlier, and a hold keyed on the match commits the
> regression as expected. So a hold on a row that passed at the iteration start (the first
> state-file stamp's head, read through `stages::iteration_start`) must say where the red
> reproduces, and that commit must be outside the iteration. The gate checks the commit's position
> and **not** the reproduction, so a false commit still passes. What the rule buys is that holding
> a regression takes a claim the diff shows, rather than a match nobody wrote down. The assertion
> is off wherever there is no iteration-start commit, and the clean line says so. A shallow clone
> is one such case, so a consumer whose CI clones shallow gets the check only from the local
> hook.

### (3) The validate template treats a match as a possible cause, not a cause

The validate template's triage paragraph, completion rule and valve payload are rewritten, so a red
on an item that passed at the iteration start is a suspected regression until it reproduces
outside the iteration {design-bearing}.
**Not yet applied.**

**Replacement text, lifecycle-kit/templates/stages/validate.md** (**Not yet applied**). The
paragraph under **Triage a red against the queue before excavating it.** becomes:

> On any failure, first grep the queue's deferred/lessons sections — a pre-existing red is usually
> already a filed task with the diagnosis written. A filed entry that matches a red is a possible
> cause, not a cause. A red on an item your baseline held passing when the iteration opened is a
> suspected regression. Reproduce it at the iteration-start commit before you hold it. If it
> reproduces, hold it against the filed task and record the reproducing commit where your baseline
> grammar keeps it. If it does not reproduce, this iteration caused it: fix it. Only a red on an
> item already held red at the iteration start is noted and passed over on the match alone.

The completion sentence becomes:

> Do not declare validate complete until the baseline diff is clean: no item that passed at the
> iteration start is held red without a recorded reproduction outside the iteration, every
> held-constant red still carries a live blocking slug, and any recovered item has been promoted to
> pass.

In the valve paragraph, "which suite, what the red is, and why it is accepted rather than fixed"
becomes "which suite, what the red is, why it is accepted rather than fixed, and the commit it
reproduces at".

**Replacement text, lifecycle-kit/SPEC.md §templates/stages/** (**Not yet applied**). In the valve
hand-off paragraph, "which suite, what the red is, and why it is accepted rather than fixed" gains
", and the commit it reproduces at, which the row close lands must record", and the definition "a
suite whose failure is understood and is not a regression from this iteration's diff" becomes "a
suite whose failure is understood and reproduces at the iteration-start commit".

### (4) This consumer's validate binding names the token

`.claude/commands/validate.md`'s `suites` binding names `reproduces-at=` as where a hold records
its reproduction {mechanical}.
**Not yet applied.**

**Replacement text, `.claude/commands/validate.md`** (**Not yet applied**). The parenthetical "(a
new held-constant red with its blocking slug, a recovered row promoted to pass)" becomes "(a new
held-constant red with its blocking slug, and its `reproduces-at=` commit where the row passed at
the iteration start; a recovered row promoted to pass)".

### (5) The grammar spelling is swept to the widened form

Every copy of the row grammar spelling moves to
`<suite> <scenario> <status> [<slug> [reproduces-at=<rev>]]` {mechanical}.
**Not yet applied.**

The sites are the header of `.workflow/validate-baseline.txt`, `evidence-kit/README.md`'s seeding
step, `evidence-kit/smoke/install.sh`'s seed header, `native/src/installer/recipe.rs`'s seed
header, and the three messages in `native/src/gates/evidence_baseline.rs`. Delta 2 rewrites the
last help message anyway. A consumer's existing header keeps the old tail. Nothing gates the tail,
so no migration is owed.

## Producers and consumers

- **`reproduces-at=<rev>` token.** Produced by a human commit to the baseline: a validate session
  after reproducing a red at or before the iteration-start commit, or the closing stage landing the
  row a used valve line owes (delta 3's payload carries the commit to it). Consumed by
  `check-evidence-baseline`'s flip assertion at pre-commit and in CI's battery. It has one field,
  `<rev>`, read at the same transition by red conditions (b) and (c). `evidence::diff` and
  `--run-validate` read fields one to three only, so they ignore the token. That was probed, not
  assumed.
- **Flip assertion.** Its producer is the gate's ordinary run (`precommit`, registered in
  `scripts/gates.list`). Its enabling input is the state file, which every lifecycle consumer
  writes at its first stamp. The consumer of its findings is the committing session and the CI
  run. The clean line names the early-out it took, so a disarmed green does not look like a judged
  one.
- **The third positional.** Read by the gate only. The fixture pair's `args` set it, and the
  behavioral test covers both the positional and the knob path.
- **Readers of the widened grammar**, found with
  `grep -rn '<suite> <scenario> <status>'` over the tracked tree: the delta 5 sites, the two
  evidence-kit/SPEC.md sections deltas 1 and 2 rewrite, and the generated docs mirror. None of
  them parses the tail.
- **Readers of the rewritten triage.** The validate stage session, through the skill. The lead
  reads the evidence commit's baseline diff, and the token is now in that diff.
- **Red condition, point 5.** This amendment narrows no corpus. **Point 6:** the members the flip
  assertion obliges today are the flip rows between `af66137c` and `HEAD`. Probed with
  `git diff af66137c HEAD -- .workflow/validate-baseline.txt` (empty) and a grep for `fail`/`ignore`
  rows (none), so there are zero members and no value is owed. Build re-runs both probes at landing,
  since its own commits move `HEAD`.
- **The queued debt this iteration shares a surface with.** `baseline-move-stales-evidence-line`
  documents the promote, baseline, fresh-evidence recipe. For a row that passed at the iteration
  start, that recipe's baseline step now owes the token. The recipe batch therefore runs after
  deltas 1 and 2 land, or it names the token itself.

## Existing sections updated

- `evidence-kit/SPEC.md` §Baseline manifest — the grammar and the header tail (delta 1).
- `evidence-kit/SPEC.md` §check-evidence-baseline — the flip assertion, the argument mode and its
  honest limit (delta 2).
- `evidence-kit/SPEC.md` §Producers and consumers — the **Baseline line** bullet gains the token's
  producer and the flip assertion as its consumer (deltas 1 and 2).
- `evidence-kit/README.md` — the gate summary names flip causation, and the seed header takes the
  widened grammar (deltas 2 and 5).
- `native/src/gates/evidence_baseline.rs` — the assertion, the grammar branch and the help text
  (deltas 1, 2 and 5).
- `evidence-kit/checks/check-evidence-baseline.gate` — `couples=` and the `spec:` gloss (delta 2).
- `scripts/git-hooks/pre-commit` — regenerated from the widened manifest (delta 2).
- `evidence-kit/gate-tests/check-evidence-baseline/good/args` and `bad/args` — the third
  positional (delta 2).
- `evidence-kit/gate-tests/check-evidence-baseline.test.sh` — the flip cases (delta 2).
- `lifecycle-kit/templates/stages/validate.md` — triage, completion and valve payload (delta 3).
- `lifecycle-kit/SPEC.md` §templates/stages/ — the valve hand-off paragraph (delta 3).
- `.claude/commands/validate.md` — the `suites` binding (delta 4).
- `.workflow/validate-baseline.txt` — this repo's baseline header (delta 5).
- `evidence-kit/smoke/install.sh` — the smoke's seed header (delta 5).
- `native/src/installer/recipe.rs` — the installer's seed header (delta 5).
- `docs/evidence-kit/SPEC.md` — the generated mirror, regenerated with
  `--emit docs-mirror --write` (all deltas).
- `docs/evidence-kit/README.md` — the generated mirror (all deltas).
- `docs/lifecycle-kit/SPEC.md` — the generated mirror (all deltas).

The roster comes from `grep -rn '<suite> <scenario> <status>'`, `grep -rn 'Triage a red'` and
`grep -rn 'why it is accepted rather than fixed'` over the tracked tree at `66682ce6`. Build
re-derives it.

## Retired spellings

- `<suite> <scenario> <status> [<slug>]` — the row grammar before the token, widened in every
  copy (deltas 1 and 5).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check
      holds for the token, the flip assertion and the third positional.
- [ ] **Instruction surfaces: instruction only** — the validate template and binding carry the
      rule, and its grounds live in evidence-kit/SPEC.md §check-evidence-baseline.
- [ ] **Merged with no information lost** — deltas 1 and 2 rewrite their sections rather than
      appending.
- [ ] **Amendment deleted** — this file is removed on merge, and `ls SPEC-*.md` shows no
      hold-cause amendment.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` is green over the tracked tree.
- [ ] **Gaps filed** — any gap found at build goes to the gap inbox.
