# SPEC amendment: run-gates name selector

Pairs with `TASK-QUEUE.md` entry **single-gate-run-config-bridge**.

## The fork this amendment resolves

The entry filed the unit `[design-pending]` on a real three-way fork and refused
to pre-design it: *"an `--only` flag, a separate `bin` tool, or a documented
one-liner is a real fork, and the runner's argument grammar is the constraint
that decides it."* One arm has since half-landed on its own — the documented
one-liner is owned at §Layout and configuration, which is what discharged the
entry's unowned-fact half — and what survives is ergonomics: a session still
authors a scratch directory and a one-line `gates.list` to ask one gate for its
verdict.

**Ruled: the flag.** `run-gates.sh --only <name> [<name>...]`.

### The constraint decides it, exactly as the entry predicted

The runner's argument grammar already has two option arms in the same position
and one positional behind them: `--emit <arm> [args...]`, `--for <path>...`, and
`[gates-dir]`. `--only <name>...` is the third of a shape that exists twice,
parsed in the same place, consuming the rest of argv on the same terms. It
inherits, at zero design cost, everything the entry says a hand-built invocation
loses: `gate_command`'s `GATE_SDK_KNOB_*` bridge, the consumer-first resolve-dir
order, the stdout/stderr dispatch split, the timing file, the omission
accounting, and the quiet-green/loud-red output contract.

**The separate `bin` tool is refused.** It would re-implement every one of those
against the same registry, and it would grow the `bin/` roster that
**kit-bin-entry-point-unrostered** prices as a per-guess discovery cost. That
entry's own evidence is worth reading in this direction: a session reached for a
nonexistent `gate-sdk/bin/gate.sh`, which is evidence about *discoverability* —
where a session looks — not about where the code belongs. A flag on the tool the
session already invokes answers both, and delta 3 answers the discoverability
half directly.

**The documented one-liner is the fallback, not the answer**, and it is what
makes this flag thin rather than new: the scratch-registry route proves that
selection composes out of knobs already shipped, so `--only` adds a spelling and
no mechanism. The entry's third firing is the measurement that the route is not
sufficient — the session found it, used it, and still authored a file to ask one
question.

**The bare positional name selector is not reopened.**
**kit-bin-entry-point-unrostered** ruled `run-gates.sh <gate-name>` *"a habit
rather than a hole"* on the ground that the positional argument is a gates-dir.
That disposition stands and this amendment does not touch it: one positional
keeps one meaning. Delta 3 serves the habit without minting a second.

### Why a flag rather than an env knob

The kit convention is **config via env**, and §run-gates states it in this
tool's own voice for `GATE_SDK_VERBOSE`: *"Env over flag by the kit convention:
one mechanism serves the interactive run, the generated hooks, and any CI
wrapper without an argv contract change."* Read as a general rule it would
demand `GATE_SDK_ONLY`, so the distinction is ruled here rather than left for a
later reader to re-litigate.

The convention governs **configuration** — a value every caller of the battery
should carry uniformly. `--only` is **selection**, and selection is the one
thing that must not be ambient. An environment variable naming a gate would be
inherited by the generated hook, by `run-consumer-smoke.sh`, and by any CI
wrapper that ran under it, silently narrowing a battery to one member while the
summary line still reported a pass — the green-with-nothing-behind-it failure
that **consumer-smoke-subset-accounting-verdict** and its siblings already
price. A selector expressed in argv dies with the process that typed it. The
runner already draws this line: `GATE_SDK_VERBOSE` (how the battery reports) is
env, `--for` (which members run) is argv, and `--only` joins the second set.

### The friction's root is a contract violation, and it rides this unit

The entry's attested failure is not that `--only` was missing. It is that typing
it produced `no registry at --only/gates.list` — a message about a missing file
in answer to a rejected argument, three times across three sessions, each of
which then reached for the binary directly. §The bin/-tool contract already
rules the behavior that would have stopped every one of them: `-h`/`--help` as
the first argument prints usage on stdout at exit 0, and *"a positional argument
beginning with `-` that is not a recognized option is a **refusal** — usage on
stderr, exit 2."* Probed at authoring time: `bash gate-sdk/bin/run-gates.sh
--help` exits 2 with `run-gates: no registry at --help/gates.list`. The tool
does not honour its own family's contract, and that is why a wrong guess cost
three steps instead of one.

Delta 2 is therefore **debt** — behavior converging on a name the spec already
carries — and it lands in this unit because enforcement-first ranks landing the
fix with the feature above filing it beside one. Without it, `--only` ships
into a tool where the *next* wrong guess is still unrecoverable.

### The provenance seam, and the config surface

**Kit mechanism:** the flag, its grammar, its refusals, and the help behavior
§The bin/-tool contract already requires. Every one of them is a statement about
argument handling, true for any consumer's registry.

**Consumer content, untouched:** the gate names themselves. `--only` takes them
from argv and validates membership against *the consumer's own registry* — it
carries no name list, no default selection and no notion of which gates matter.
A kit literal naming this project's gates is exactly what the seam forbids, and
the flag is the shape that cannot hold one.

**No new knob**, which is the argv-versus-env ruling above read from the other
side: the selection is per-invocation and belongs in argv, and the registry
location it selects within is already `GATE_SDK_GATES_DIR`'s. Delta 1 adds one
argument arm and no `GATE_SDK_*` value, so config-via-env has nothing to take.

## What changes

### (1) `run-gates.sh --only <name> [<name>...]` — the name-keyed selector

The runner gains a third option arm: it resolves the registry exactly as a bare
run, then runs only the named members. **{design-bearing}**

**Grammar.** `--only` is recognized as the first argument, on the same terms as
`--emit` and `--for`, and consumes every remaining argument as a gate name. As
with `--for`, the `[gates-dir]` positional is unavailable in this form — a
consumer pointing at a non-default registry composes it through
`GATE_SDK_GATES_DIR` instead, which is the knob the positional shadows anyway.
An empty name list is a refusal: `run-gates: --only needs at least one gate
name`, exit 2, matching `--for`'s existing message shape.

**Selection is set-shaped and registry-ordered.** The named set is intersected
with the registry and run in **registry order**, not argv order, so two names
produce the same transcript whichever way they were typed and the output reads
like a narrower bare run. Duplicates collapse silently — the argument is a set.

**An unregistered name is a refusal, exit 2**, naming both the name and the
registry path. This is the one place `--only` deliberately diverges from `--for`,
and the divergence is the point: §run-gates rules that when no gate couples to a
given path the selector *prints a note and exits 0*, because **an ungoverned
path is a fact, not a failure**. A name is not a fact about the tree — it is a
claim about the registry, and a wrong one is a typo or a stale memory. Exiting 0
there would print `All 0 gates passed.`, which is the vacuous green the summary
line exists to make impossible.

**A `mode=staged` member receives no positional arguments**, which is the
bare-run behavior rather than the hook's. `--for` hands such a member its
matching paths because it *has* paths; `--only` names gates and has none, so the
member runs over its full corpus. Stated because a reader who knows `--for`'s
staged branch will otherwise expect a pathspec.

**Everything downstream is untouched**, and that is the whole value of putting
this on the runner: the config bridge, resolve-dir order, dispatch capture,
per-gate timing, `GATE_SDK_VERBOSE`, the declared-omission line and the output
contract all behave exactly as in a bare run. The summary line's `N` is the
**selected** count, as it already is under `--for`; the roster-collapse tripwire
that reading serves is a property of a bare run, and neither selector claims it.

### (2) The runner honours §The bin/-tool contract

`run-gates.sh` gains the help and refusal behavior its family's contract already
requires, which is the fix for the friction that produced this entry.
**{design-bearing}**

- `-h` or `--help` as the first argument prints the tool's usage on **stdout**
  and exits **0**. Usage on a successful help request is output, not a
  diagnostic.
- A first argument beginning with `-` that is not `--emit`, `--for`, `--only`,
  `-h`, `--help` or `--` is a refusal: usage on **stderr**, exit 2, naming the
  unrecognized option. This is what turns `--onlyy check-graph` and every future
  near-miss into one readable step.
- `--` ends option processing, so a gates-dir legitimately spelled with a
  leading dash is still reachable.

The contract binds here because the positional is free text — a path — and an
arity check is not a shape check. The delta is **debt** rather than a feature:
it mints no name, it converges behavior on §The bin/-tool contract's existing
one, and it is recorded as debt so a later reader does not look for a design
ruling behind it.

The refusal also composes with delta 1 rather than shadowing it: `--only --for`
refuses at the *name* rather than silently treating `--for` as a gate name,
because a name beginning with `-` is an unrecognized option wherever it appears
in the `--only` list.

### (3) A gates-dir that is really a gate name steers to `--only`

When the positional does not hold a `gates.list` and the argument is the name of
a member of the **default** registry, the refusal says so and names `--only`.
**{design-bearing}**

This is the habit **kit-bin-entry-point-unrostered** ruled a habit — and a
habit that costs a step every time is worth one branch. The steer resolves the
default registry (`gate_sdk_gates_dir`), which is available precisely because
the failing argument was never a registry path; membership there turns
`run-gates: no registry at check-queue-entry-budget/gates.list` into a message
that names the flag that does what the caller meant. Where the default registry
is itself unresolvable, the plain message stands unchanged — the steer is an
addition to a refusal, never a new failure path.

It is deliberately **not** a fallback: the positional keeps one meaning, and a
caller who typed a name gets a refusal plus a remedy rather than a run they did
not ask for. That is the same shape every gate's `help:` line takes, applied to
a `bin/` tool's argument error.

### (4) §Layout and configuration's targeted-run paragraph is re-pointed, not deleted

The knob-composition fact stays; the ergonomic claim moves to `--only`.
**{design-bearing}**

That paragraph currently opens *"A **targeted run** needs no mechanism beyond
these two knobs composed"*, and delta 1 falsifies the sentence while confirming
the fact underneath it. The fact — that pointing the positional at a scratch
registry and setting `GATE_SDK_VERBOSE` yields one gate's verdict with the
`GATE_SDK_KNOB_*` bridge intact — is what makes `--only` a spelling rather than
a mechanism, and deleting it would drop the reason the flag is thin. It is
retained as the composition it describes and re-pointed: `--only` is the
ergonomic form, and the scratch-registry route remains the answer for a caller
composing a registry that is not a subset of the default one.

Retaining it is also the honest record of the entry's own history: that
paragraph landed as the discharge of this entry's unowned-fact half, and a
merge that deleted it would make the discharge unreadable.

### (5) Behavioural coverage in `smoke/`

`gate-sdk/smoke/install.sh` gains legs for the new surface, on the precedent
§The bin/-tool contract names. **{mechanical}**

The contract rules that no gate reads it and that each `bin/` member's coverage
is behavioural, citing lifecycle-kit's `enter-stage.sh --simulate` arm. The
runner's existing legs in that file are the model — they already drive quiet
green, verbose green and the emitter arm against a vendored scratch consumer.
Four legs are added: `--only <one registered member>` runs exactly that member
and exits 0; `--only <unregistered name>` exits 2 naming the registry;
`--help` prints usage on stdout at exit 0; and an unrecognized `-`-leading first
argument exits 2 with usage on stderr. The unregistered-name and help legs are
the two that would silently regress, since both are refusals nothing else
exercises.

### (6) The documented surfaces catch up

Every surface that spells the runner's grammar gains the arm. **{mechanical}**

`gate-sdk/bin/run-gates.sh`'s own `# usage:` header — which documents
`[gates-dir]` and `--for` and, today, **not** `--emit` — becomes complete;
gate-sdk/README.md's runner bullet and command block gain the targeted form;
§run-gates gains delta 1's contract beside `--for`'s. The `--emit` omission is
folded in here rather than filed: it is one line of the same header, and leaving
it would ship a usage text that is wrong about the tool it prints usage for.

## Producers and consumers

**New interface: the `--only` selector (delta 1).** No new state, no new
message, no new field, no new knob — one argument arm on a shipped tool.

- **Producer** — `gate-sdk/bin/run-gates.sh`'s argument parser, in the arm
  position `--emit` and `--for` already occupy. Its enabling configuration is
  **none**: the tool is executable, rostered in gate-sdk/README.md and invoked
  by name, so the arm is reachable the moment it lands. There is no deployed
  configuration that must set anything, which is the failure mode the
  causal-completeness check's first point names and the reason this line is
  short rather than absent.
- **Consumer** — the invoking session, through the process exit code and the
  gate's own output on the runner's existing quiet-green/loud-red contract. The
  named downstream consumers are the ones §run-gates already lists for `--for`:
  a mid-edit session, and a delegated agent's gate-driven worklist
  (delegation-kit's agent-execution template), both of which want a verdict
  rather than a battery.
- **The selection's own reader** — the runner's `RUN_LIST` loop, at the
  transition between registry resolution and dispatch. `--only` populates the
  same array `--for` populates, with `RUN_ARGSTR` left empty for every member,
  which is precisely how a bare run already populates it. No new datum crosses
  that transition, so there is no new field to give a reader.

**New behavior on an existing interface (deltas 2 and 3).**

- **Producer** — the same parser, before registry resolution. `--help`'s
  producer is the first-argument test; the steer's producer is the
  registry-resolution refusal, which gains a membership probe against
  `gate_sdk_gates_dir`.
- **Consumer** — the invoking session, on stdout (help, exit 0) or stderr
  (refusal, exit 2), and `gate-sdk/smoke/install.sh`'s new legs (delta 5), which
  are the machine consumer that keeps both from regressing.

**Existing integration prose, and the flow it described.** §run-gates describes
the runner's grammar as *bare or `--for`*; §Layout and configuration describes
the targeted run as a knob composition; gate-sdk/README.md and the tool's own
usage header enumerate the arms. All four describe a prior flow this change
alters, and all four are updated in this amendment rather than left to drift
(deltas 4 and 6).

**No corpus narrows.** Point 5 of the causal-completeness check binds on a
delta that prunes a corpus, tightens a glob or drops a file, and this amendment
does none: `--only` narrows a *run*, never a scanned corpus, and the narrowing
is per-invocation and caller-chosen. The one verdict that changes for an
existing input is `run-gates.sh --help`, which moves from exit 2 to exit 0 —
enumerated here rather than assumed harmless. Its callers were enumerated by
grepping the tree rather than recalled: `gate-sdk/smoke/install.sh` (bare,
positional and `--emit graph`), `gate-sdk/bin/run-consumer-smoke.sh` (bare),
`installer/consumer-smoke/run-smoke.sh` (bare, three sites) and
`installer/lib/init.sh` (`--emit graph`, plus one printed suggestion line). None
passes `--help` or a `-`-leading first argument, and none reads exit 2 from this
tool as a success condition — every one of them matches on the green summary
phrase or on exit 0. Read, not inferred from their shape.

## Existing sections updated

- gate-sdk/SPEC.md §run-gates — gains `--only`'s contract beside `--for`'s: the
  grammar, registry-ordered set selection, the unregistered-name refusal and why
  it diverges from `--for`'s exit-0 note, the `mode=staged` no-args reading, and
  the summary line's `N` under a selector. It also gains the help/refusal
  behavior and the argv-versus-env ruling, the latter beside the
  `GATE_SDK_VERBOSE` paragraph that states the convention this ruling bounds
  (deltas 1, 2 and 3).
- gate-sdk/SPEC.md §Layout and configuration, the `gates.list` bullet — the
  targeted-run paragraph is re-pointed at `--only` and keeps the
  knob-composition fact (delta 4).
- gate-sdk/SPEC.md §The bin/-tool contract — unchanged in content; it is the
  contract delta 2 converges on, cited from §run-gates rather than restated —
  a pointer, because restating it there is the duplication content-tiering
  forbids.
- `gate-sdk/README.md` — the `bin/run-gates.sh` bullet and the command block
  gain the targeted form (delta 6).
- `gate-sdk/bin/run-gates.sh`'s `# usage:` header — gains `--only`, `--help` and
  the missing `--emit` line (delta 6).
- `gate-sdk/smoke/install.sh` — gains the four behavioural legs (delta 5).
- `TASK-QUEUE.md` — **single-gate-run-config-bridge** promotes with this
  amendment's ref (all deltas).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
