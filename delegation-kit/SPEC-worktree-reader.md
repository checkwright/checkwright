# SPEC amendment: worktree-reader

Closes `isolated-child-liveness-hook-displaces-its-report`. Under
`isolation: worktree` the turn-end liveness hook does exactly what it was told,
and doing so overwrites the child's report on the only channel the dispatcher
reads.

**The chain, traced end to end rather than inferred.**
`scripts/subagent-stop-liveness.sh:34` finds `DELEGATION_KIT_LIVENESS_CMD`
readable — it is a tracked file, so the worktree has it — and runs
`scripts/producer-liveness-reader.sh`, which `exec`s
`scripts/gate-exec.sh check-producer-liveness`. That resolves the gate to a
`.gate` descriptor whose binary is `GATE_SDK_NATIVE_BIN`, a **relative** path
under `native/target/`, which is gitignored and therefore absent in a fresh
worktree. `gate_command` refuses, the reader exits 2, the hook's own `*.run` glob
over the worktree's `.tmp` is empty, and `:55` reads the pair as `unresolved` —
a refusal. The harness returns only the child's **last** assistant message, so the
child's correct report-and-return displaces its survey.

**The bind is structural, and that is what forces a fix rather than a
discipline.** `scripts/agent-dispatch-guard.sh` **refuses** a read-only type
dispatched *without* `isolation: worktree`, and worktree isolation is precisely
what makes the binary unresolvable. The guard's required remedy is what arms the
defect, so no dispatch shape in this repo avoids it while staying in contract —
seven for seven across three sessions, and the rate is corroboration rather than
the finding.

## What changes

### (1) The consumer's liveness reader resolves the gate binary through the main checkout

`scripts/producer-liveness-reader.sh` stops being a bare `exec`. Where the
configured `GATE_SDK_NATIVE_BIN` does not exist **and** the cwd is a linked
worktree, it sets `GATE_SDK_NATIVE_BIN` to the same relative path resolved
against the **main** checkout before exec'ing, and otherwise changes nothing.
The main checkout is derived vendor-neutrally: `git rev-parse --git-common-dir`
answers `.git` in the main worktree and the main repository's git directory from
a linked one, so its parent is the main checkout. **{design-bearing}**

**This takes the entry's first candidate, and the two others are refused with
cause.**

- *Emit the unavailability on a channel that does not become the child's final
  assistant message* — **not buildable by the hook**. The hook's only channels
  are its log (which lands in the worktree and dies with it) and stderr on a
  non-zero exit, which is exactly what the harness turns into the message that
  displaces the report. There is no third channel for it to choose.
- *Return silently when the target is a worktree it cannot resolve* — **refused
  for what it costs**. It converts a *reader is broken* signal into silence
  everywhere, including in a main checkout whose reader genuinely broke, which is
  the actionable state §The turn-end liveness hook minted `unresolved` to name.
  The entry itself calls it "cheap and pulls against fail-closed"; the ground for
  refusing it is that candidate 1 removes the *trigger* instead of the *signal*.

**Why this narrow resolution is safe where a general one would not be.**
`check-producer-liveness` reads `*.run` records and nothing else: its verdict
does not depend on the binary matching the worktree's source, so a main-checkout
binary answers the same question a locally-built one would. That is **not** true
of gates in general — `check-gate-binary-fresh` exists precisely to compare a
binary against the source in its own tree — which is why this is a resolution in
one consumer front end and not a change to `gate_native_bin`.

**It does not build anything, and the recorded refusal it might look like
reversing is untouched.** `templates/agent-execution.md`'s declaration was ruled
on the ground that an 11.69s crate build on every dispatch is a real cost and
that no hook can reach into a worktree that does not exist yet at
`PreToolUse(Agent)` time. Both remain true. This resolves an artifact that
already exists in a tree that already exists, at the moment the reader runs.

### (2) The kit states the obligation the consumer's reader now discharges

delegation-kit/SPEC.md §The turn-end liveness hook (template) gains a stated
requirement on `DELEGATION_KIT_LIVENESS_CMD`: **a consumer's reader must resolve
from a worktree-isolated dispatch**, and the named consequence of one that does
not is that the hook's `unresolved` refusal **displaces the child's report**,
because the harness returns only the last assistant message. **{design-bearing}**

The kit ships the hook and the verdict table; only a consumer knows its front
end, which is why the template ships no default `LIVENESS_CMD` at all. So the
kit cannot fix this for a consumer and can state the bar the consumer's adapter
must clear — the same shape §The turn-end liveness hook already uses for the
reader's exit-code contract.

**The cost is stated with the requirement, because it is the reason the bar is
worth stating rather than left to be discovered.** The worst attested tail put no
ceiling on it: a 2026-08-26 `align` dispatch wedged in a Stop-hook loop across
three resumptions, burned roughly 166k child tokens and returned nothing,
recovered only by re-dispatching fresh. The ordinary case is one wasted
round-trip paid while the dispatcher waits, and it is silent in the worse
direction — the returned text reads as a liveness complaint rather than as a
dropped report.

### (3) The isolated-dispatch declaration is narrowed to what stays true

`templates/agent-execution.md`'s clause — *inside a worktree-isolated dispatch,
binary-dispatched gates do not resolve, so an isolated agent may not read a gate
verdict as available and may not repair one by building the crate* — keeps both
of its prohibitions and gains its exception: **the turn-end liveness reader is
resolved for the agent by the consumer's adapter**, so an isolated agent no
longer meets the refusal that invited it to build the crate. **{design-bearing}**

Narrowed rather than deleted, and narrowed on its own terms. The declaration was
added because "a refusal whose message names no reachable remedy invites a
read-only agent to take a mutating one" — which is an attested behaviour, not a
hypothesis. Delta 1 removes the refusal for the one gate that fires at turn end;
every other binary-dispatched gate is still unresolvable inside a worktree, and
the *may-not-build* prohibition is unchanged for all of them, this one included.

### (4) The recovery stays written down, because the fix does not reach a child already dispatched

The entry's recovery — **resume the child by id and ask it to re-emit verbatim,
telling it explicitly not to re-run the work** — is recorded in
`templates/agent-execution.md` beside the declaration, with its two attested
limits: it is **not reliable** (an earlier resume returned "this session just
started" and that sweep had to be re-run), and the usage block is no signal
either, since a successful re-emit reported the *original* run's tool count and
tokens. **{design-bearing}**

Recorded rather than retired with the defect, because a parent meeting a
displaced report has no reason to believe the report still exists, and the same
displacement is reachable from any consumer whose adapter has not taken delta 1 —
including an adopter vendoring this kit today.

## Producers and consumers

**The worktree-resolved binary path (delta 1)** — the one new value.

- *Producer:* `scripts/producer-liveness-reader.sh`, computed at each invocation
  from `git rev-parse --git-common-dir` and the configured
  `GATE_SDK_NATIVE_BIN`, guarded on the configured path being absent so a main
  checkout takes no new behaviour at all. **Enabling config actually set:** none
  is added; both inputs already resolve in every tree that runs the hook
  (`GATE_SDK_NATIVE_BIN` has a kit default, `git` is the toolchain floor). It is
  live on the next isolated dispatch in this tree, not test-only.
- *Consumer:* `scripts/gate-exec.sh` in the same process, through the exported
  environment, at gate resolution — and beyond it `gate_command`, which is what
  reads the knob today. No new consumer is introduced and no existing one
  changes its read.
- *Named reader for every field:* one value is produced, `GATE_SDK_NATIVE_BIN`,
  whose readers are named above at the resolution transition. It is written at
  exactly one place and read at exactly one, so no field is populated at a
  transition where nothing reads it.

**The verdict the hook then records changes, and its reader is unchanged.** With
the reader resolving, an isolated child's firing is `verdict=green records=0
decision=allow` instead of `verdict=unresolved … decision=refuse`. The log line's
grammar, its field list and its order are untouched, so the space-delimited parse
the close-stage triage uses does not move. **The honest limit already recorded on
that field stands**: `DELEGATION_KIT_STOP_LOG` resolves against the writing
session's cwd, so an isolated child's lines still land in its own worktree and
are destroyed at reclamation — the close-stage triage still cannot see this
class. What changes is that the class it cannot see is now `allow` rather than
`refuse`, which is a smaller loss and not a closed one.

**Deltas 2, 3 and 4 introduce no state, event or interface.** They are prose
obligations on surfaces that already exist; their consumers are the reader of
those surfaces — a consumer authoring a liveness adapter at delta 2, a dispatched
agent loading `templates/agent-execution.md` at deltas 3 and 4, and the
dispatching parent at delta 4.

**Narrowing check (canon-kit/SPEC.md §The causal-completeness check, point 5).**
Delta 1 **narrows** the set of readings on which the hook refuses — every
`unresolved` firing under isolation becomes `green` — so each reader's **red
condition** is enumerated rather than its subject:

- `delegation-kit/gate-tests/subagent-stop-liveness.test.sh` — reds when a driven
  arm's **exit code or `decision` column** differs from the asserted one. **Not
  monotone**: it asserts an exact exit per arm. Cleared by inspection **only**
  because it drives the hook with a stubbed `LIVENESS_CMD` and never with this
  consumer's reader, so its `unresolved` arm is untouched. That arm must stay —
  the verdict is not retired, it is no longer reached by one caller.
- `scripts/gate-tests/subagent-stop-reader.test.sh` — reds when the configured
  reader's arm does not produce the asserted verdict. **Not monotone** (it holds
  a minimum: specific verdicts must appear). It **must move in this unit**: a new
  arm runs the reader from a linked worktree with no local `native/target` and
  asserts exit 0, which is the only executable statement of delta 1 and the
  regression oracle for it. Listed as an update target.
- `check-producer-liveness` — **untouched**: no delta changes the gate, its exit
  classes, or the `pid=<n> run=<key>` grammar.
- `check-gate-binary-fresh` — reds when the registered binary is older than the
  crate source. **Not monotone** (a staleness comparison). Cleared by inspection
  and the reason is delta 1's own bound: the resolution is scoped to one front
  end for one gate, so no other gate call and no freshness comparison ever sees a
  foreign binary.
- `scripts/agent-dispatch-guard.sh` and its test suite — reds on a read-only type
  dispatched without isolation. **Not monotone** (a refusal condition). Untouched
  and deliberately so: delta 1 makes the guard's required remedy safe rather than
  relaxing the guard, which is the difference between fixing this and routing
  around it.
- `check-scratch-citation`, `check-settings-paths` — red on a scratch path or a
  settings path asserted somewhere it does not hold. **Monotone under an
  addition** and cleared by inspection: no delta adds, moves or renames a scratch
  path; `RUN_DIR` stays the dispatched session's own `.tmp`, which is correct —
  a child must not be blocked by its parent's producer.

## Existing sections updated

- `delegation-kit/SPEC.md` §The turn-end liveness hook (template) — the
  `DELEGATION_KIT_LIVENESS_CMD` paragraph gains the worktree-resolvability
  requirement and the named displacement consequence; the `unresolved` row of the
  verdict table is re-read to confirm it still describes a reachable state after
  delta 1 (deltas 1 and 2).
- `delegation-kit/SPEC.md` §The turn-end liveness hook (template), the `decision`
  field bullet — its recorded honest limit (an isolated child's firings never
  reach the close-stage triage) is restated with what delta 1 changes about it:
  the class is now `allow`, so the unseen firings are no longer refusals
  (delta 1).
- `delegation-kit/templates/agent-execution.md` — the isolated-dispatch
  declaration is narrowed to its two surviving prohibitions plus the liveness
  reader's exception (delta 3), and the resume-and-re-emit recovery joins it with
  its two limits (delta 4).
- `scripts/producer-liveness-reader.sh` — the script itself, and its `# spec:`
  pointer, which must bind to the section that now states the obligation
  (delta 1).
- `scripts/gate-tests/subagent-stop-reader.test.sh` — the worktree arm, which is
  delta 1's only executable statement (delta 1).
<!-- update-target-exempt: owned by no delta — a consistency re-read of a divergence no delta edits, kept as a target because the merge must confirm it still reads true -->
- `guard-kit/SPEC.md` §The generic ruleset, rule 14 — no content change; re-read
  at merge to confirm its recorded divergence from the hook's dispositions still
  reads true once one of those dispositions stops firing under isolation.
<!-- update-target-exempt: owned by no delta — the sibling entry's own scope, which this unit deliberately does not take -->
- `TASK-QUEUE.md`'s `worktree-isolated-dispatch-cannot-reach-the-main-checkout`
  entry — no content change. **Its subject is wider than "writes":** the entry's
  own text names two things, "resolves neither a binary-dispatched gate nor the
  capture-tier log its own firings must land in" — a read/resolution side and a
  write side. Delta 1 bridges exactly one gate for one consumer (the liveness
  reader), which narrows the read side by that one instance and leaves both the
  rest of that class and the write side untouched, so the entry stays open on
  its own terms rather than on a writes-only reading of it. Re-read at merge so
  the boundary drawn here still holds once the read side has a bridge for one
  gate and the write side and the rest of the gate class do not.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Proved by a real isolated dispatch, not by the test alone** — one
      read-only `isolation: worktree` agent is dispatched at build and returns its
      own report as its last message. Seven for seven is the record to break, and
      a green unit test is not the same claim.
- [ ] **The refusal still fires where it should** — the reader run from a main
      checkout with a deliberately broken binary still exits 2, so delta 1
      removed a trigger and not the signal.
