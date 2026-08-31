# SPEC amendment: generator-cause

Round 7 of the Windows leg, and it is **cause-only** — ruled 2026-08-31 by the
operator in consult (TRAJECTORY.md §The closed rulings; recorded on
`platform-support-ci-matrix`). Its single change is the instrument:
`check-graph`'s assertion-D generator spawn surfaces the generator's own output
on any non-zero exit, whichever branch swallowed it today. **No repair ships in
this round**, and the alternative refused is a third fix-and-observe round —
rounds 5 and 6 each shipped a repair reasoned on Linux, the one dialect that
cannot exhibit the fault, and round 6 came back with exactly the failure the
instrument was built to explain and an **empty cause**.

**What this amendment delivers is a widening of a contract that was deliberately
narrowed, and the narrowing is preserved rather than traded away.** That is the
design question and it is real: `native/src/proc.rs`'s `Completed` hands out
stdout only through an accessor that has already read the exit status, precisely
so "capture `stdout`, ignore `status`" has no spelling in a gate module. The
resolution below is that the fail-closed contract narrowed stdout **as data**,
and what round 7 needs is stdout **as diagnostic** — two different things that
the accessor's *return type* keeps apart.

## What changes

### (1) `Completed`'s failure accessor is widened to the child's whole account of itself

`native/src/proc.rs` replaces `Completed::stderr_on_failure` with a single
accessor returning the failed child's **exit code and both captured streams**,
composed into one diagnostic string {design-bearing}. Its shape is
`stderr_on_failure`'s exactly, and the shape is the safety argument:

- **It returns `None` when the status succeeded.** So it is reachable only on
  the arm where `stdout()` already answered `None`, which is §Fail-closed
  contract's own stated property of the first widening — "it adds a *cause* to a
  refusal and no path to a false clean."
- **It returns a composed `String`, never `&[u8]`.** This is what makes the
  widening safe rather than a hole in the narrowing it reopens. The defect the
  narrowing closed is a rule *branching on* a crashed child's empty capture as
  though it were the child's answer; a string whose only operation is printing
  cannot be parsed back into a verdict, and no caller can mistake it for the
  child's stdout because it is not the child's stdout — it is a report *about*
  the child, labelled with its exit code.
- **`Completed::stdout()` is untouched**, so the one path to the child's bytes
  still reads the status first, and `proc.rs`'s
  `no_gate_module_constructs_a_subprocess_itself` roster test still refuses a
  `Command` spelling anywhere under `native/src/gates/`. Neither guard is
  relaxed, and the widening lands in `proc.rs` because §Fail-closed contract
  rules that "widening `proc.rs` is how a member that needs more of the child's
  result gets it".

**Superseded, not joined.** `stderr_on_failure` has exactly one caller in the
crate — `native/src/gates/graph.rs`'s `generator_emit` — measured by grep over
`native/src/` rather than assumed, so it is replaced rather than left beside its
successor. A second accessor with no reader is a field with no named reader,
which this amendment's own Definition of Done removes.

### (2) The report is never empty, which is the whole of round 7

The accessor's return is `Some` on every non-zero exit and its payload is
non-empty by construction {design-bearing}. Today's two silencing mechanisms are
both closed, and each is a real branch rather than a defensive one:

- **The generator wrote to stdout.** `Completed::stdout()` withholds it on a
  non-zero exit by the fail-closed contract's own design, so a generator that
  diagnoses itself on stdout — which a `bash` script's own `echo` does by
  default — is structurally unreadable today. The report carries it.
- **The generator wrote nothing to stderr.** `stderr_on_failure` trims and folds
  empty into `None`, and `generator_emit`'s `unwrap_or_default()` then hands
  `because()` an empty string, which returns `""`. A silent child therefore
  yields a bare refusal — which is round 6's observed reading, and the second run
  in a row to buy nothing.

**The datum that always exists and is never printed today is the exit code.**
`Completed::code()` has existed since the wrapper did and `graph.rs` has never
called it. It is carried unconditionally, through `proc.rs`'s existing private
`exit_code` so a signal-killed child reports `128 + n` rather than nothing —
the spelling §run-gates already fixed for the battery's tail, reused rather than
minted. A child that exits non-zero and says nothing on either stream therefore
still produces a report naming the status it died with, and the round returns a
cause even in the case that has already cost two rounds.

### (3) `generator_emit` carries the report and `because()` loses its empty branch

`native/src/gates/graph.rs`'s `generator_emit` puts the report in its `Err`
payload {mechanical}. Because the payload is non-empty by construction, both the
`unwrap_or_default()` and `because()`'s `if cause.is_empty()` arm become
unreachable and are deleted rather than kept as defensive dead code. The finding
line's grammar is otherwise **unchanged** — the same `ARTIFACT: gen-pre-commit.sh
<arm> failed; fix the generator before trusting the hook` verdict, the same
one-line suffix, the same newline folding, so `scripts/parse-gates-log.sh` and a
CI log reader both see what they saw. What changes is that the suffix is now
always present.

Both call sites take it: `--emit` and `--emit-commit-msg`. The ruling's
"whichever branch took it" reaches the two swallowing mechanisms in delta (2)
and the two arms here, and neither reading is dropped in favour of the other.

### (4) The two governed sentences the instrument retires are rewritten in place

§check-graph's *A generator that could not run reports why it could not run*
paragraph ends today "A child that fails silently still yields a bare refusal, so
the suffix is composed rather than assumed" {design-bearing}. That sentence is
what round 7 retires: the suffix is still composed, but a silent child now yields
its exit code rather than nothing, and the paragraph says so — with the
round-6 evidence that forced it, on the same terms the paragraph already uses for
the two rounds the previous widening cost. §Fail-closed contract's
`Completed::stderr_on_failure` sentence names the accessor and calls it "the
first such widening"; it is rewritten to name the successor and to state the
return-type argument in delta (1), because that argument is the thing a later
session widening `proc.rs` a third time most needs and cannot re-derive.

### (5) The instrument is proved directly, because no fixture pair can crash a child

Two `#[cfg(test)]` unit tests land in `native/src/proc.rs` beside the three the
wrapper already carries {design-bearing}, on §Fail-closed contract's stated
ground that the wrapper "is proved the way the shell helper is, directly rather
than through a member's fixture pair": a child that exits non-zero writing
**only to stdout**, and one that exits non-zero writing to **neither** stream.
The first is the branch the narrowing made unreachable; the second is round 6's
own shape, and its assertion is that the report is non-empty and names the code.
`check-graph`'s `good/`+`bad/` pair is untouched — no static input crashes a
generator — and `gate-sdk/gate-tests/check-graph-tree.test.sh`, which drives
assertion D's both arms against a constructed mini-consumer, is the behavioural
oracle that already covers the arms themselves and needs no new case for a
change that alters no verdict.

## Producers and consumers

The amendment introduces **no new state, no new event, no new field, no new knob
and no new tag**. It widens one existing interface — a crate-internal accessor —
and changes the text of one existing finding line. The causal chain is therefore
short and is stated in full rather than by exception.

- **Producer** — `native/src/proc.rs`'s `run`, at the one spawn site the crate
  has, on the arm where the child's `ExitStatus` is not success. Its enabling
  config is nothing: the accessor is reached by ordinary control flow inside
  `check-graph`'s assertion D, which runs on every battery invocation and every
  pre-commit run in this tree, and on the Windows leg through the consumer smoke.
  There is no configuration a deployment must set for the instrument to fire, and
  that is deliberate — a round-7 reading gated on a knob would need the knob set
  on the runner that has already cost six rounds.
- **Consumer 1 — `generator_emit`'s `Err` arm** (delta 3), by direct call, which
  composes the report into the `ARTIFACT:` finding line assertion D pushes.
- **Consumer 2 — the `install-smoke-windows` job's log**, read by the session
  that records round 7's finding on `platform-support-ci-matrix`. This is the
  reader the whole round exists for, and it is a **session**, not a gate: nothing
  parses the suffix and nothing must. Naming it is what stops a later reader
  looking for the machine that consumes it.
- **Consumer 3 — every other caller of the battery**, unchanged. The suffix rides
  an existing finding line, so `scripts/parse-gates-log.sh`'s tail grammar,
  `run-gates.sh`'s summary and the generated hooks all see the shape they
  already parse. Verified by reading the finding-line format string rather than
  by assuming the parse is line-prefix-only.

**No reader in the tree reds on finding an empty cause, asserts an exact finding
count, or holds a coverage floor over `check-graph`'s output**, so this widening
is monotone for every consumer: each sees strictly more text on a path that was
already red, and none sees a new verdict. The claim was taken by scanning the
callers of `check-graph`'s output rather than inferred from the change's shape.

**Every field has a named reader**, and the one field the report adds that no
predecessor carried — the exit code — is read by consumer 2 at the transition
where a silent child would otherwise have produced a bare refusal. That is
precisely the transition round 6 exhibited, so the field's reader is not
hypothetical.

## Existing sections updated

- `gate-sdk/SPEC.md §check-graph` — the *A generator that could not run reports
  why it could not run* paragraph: its closing sentence about a bare refusal is
  replaced, and the round-6 evidence that forced the widening is recorded beside
  the round-5 evidence already there (deltas 2 and 4).
- `gate-sdk/SPEC.md §Fail-closed contract` — the `Completed::stderr_on_failure`
  sentence names the successor accessor, states the return-type argument that
  keeps the widening off the false-clean path, and stops calling it the *first*
  widening now that it is the only one (deltas 1 and 4).
- `TASK-QUEUE.md`, the `platform-support-ci-matrix` entry — promoted out of the
  design-pending set with `[design-pending]` swapped for this amendment's
  `[spec:]` ref (all deltas).
- The generated projections this change stales — the on-site SPEC mirror, and
  the gate binary, whose currency `check-gate-binary-fresh` holds and which
  `bash gate-sdk/bin/build-native.sh` refreshes. Both are rostered with their
  triggers and regen commands in `docs/site-architecture.md` §Generated
  projections, which this amendment points at rather than restating (all deltas).

<!-- update-target-exempt: the round's reading is recorded by the session that observes the close push, which is after every stage this amendment reaches; the entry already carries that obligation from scope -->
- `TASK-QUEUE.md`, `platform-support-ci-matrix`'s round-7 finding record —
  deliberately unwritten here.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather
      than at the commit, this iteration carrying a sibling amendment.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; `stderr_on_failure` survives in no prose and no code.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The instrument is observed, not merely built** — the two new unit tests
      assert a non-empty report on the stdout-only and silent-child branches,
      which is the pair of mechanisms round 6 could not distinguish.
- [ ] **No repair rides along** — the round ships the instrument and nothing
      else; a fix suggested by reading the source is filed, never landed here,
      because a repair in the same round makes the reading uninterpretable.
