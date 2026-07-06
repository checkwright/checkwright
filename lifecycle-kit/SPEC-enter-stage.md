# SPEC amendment: enter-stage

## What changes

One new advisory tool, `lifecycle-kit/bin/enter-stage.sh <stage>`: the
deterministic half of a stage transition, mechanized. Today every stage
skill's first step is the same four-part ritual performed by hand — read
the iteration from the queue header, pull the id from `session-id.sh`,
append the stamp line, flip the `[stage:]` field — and each hand
performance is a chance to misformat what two gates then reject. The
script does exactly that mechanical part; **judgment stays in the skill**
(what the stage means, its exit condition, when to enter it at all), and
the stage gates stay the independent verifier — the same writer/asserter
split as `gen-pre-commit.sh` ↔ `check-graph`.

Behavior:

- **Ordinary stage** (`enter-stage.sh build`, …): read the iteration name
  from the `LIFECYCLE_QUEUE_FILE` header, take the id from
  `session-id.sh` (never an argument — the no-hand-picking rule rides
  into the tool), append `<iteration> <stage> <id> <date>` to
  `LIFECYCLE_STATE_FILE`, and flip the header's `[stage:]` field to
  `<stage>`. The stage argument must be one of `LIFECYCLE_STAGES`;
  anything else is a usage error (exit 2).
- **First stage** (`enter-stage.sh` with the `LIFECYCLE_FIRST_STAGE`
  value, default `scope`) performs the iteration-boundary reset instead:
  truncate the state file back to its header (git history keeps the prior
  iteration's stamps), stamp `— <stage> <id> <date>`, and set the header
  to the unnamed-iteration form `## Iteration: —  [stage: <stage>]`.
- **Pre-flight, not enforcement:** before mutating anything, run
  `check-stage-entry` for the target stage and refuse (exit 1, findings
  printed, no writes) when it is red. The gate reads the stage from the
  queue header and takes no target-stage argument, so the pre-flight
  evaluates the *entered* stage through the gate's existing positionals:
  a header-flipped temp copy of the queue file (under
  `${GATE_SDK_TMP_DIR:-.tmp}`) as `[queue-file]`, the real state file as
  `[state-file]` — the gate itself stays untouched. The refusal is advisory in the same
  sense the gate is at commit time — the operator who intends an override
  performs the ritual by hand, exactly as today; the tool takes no
  `--force` flag, so the easy path is always the compliant one.
- **Idempotent:** if the state file already ends with a stamp for the
  same `<iteration> <stage> <id>`, the script reports it and exits 0
  without appending — a crashed-and-resumed session can re-run its entry
  step safely.
- Stamp and flip land in the same invocation, honoring the
  flip+stamp-ride-together protocol; committing them remains the skill's
  business.

No new config: the script reads the existing `lib/stages.sh` knobs
(`LIFECYCLE_QUEUE_FILE`, `LIFECYCLE_STATE_FILE`, `LIFECYCLE_STAGES`,
`LIFECYCLE_FIRST_STAGE`). New name on a governed surface:
`bin/enter-stage.sh` (feature litmus satisfied).

## Producers and consumers

- **Producer:** each stage skill invokes the script as its first step —
  the shipped skill templates (`templates/skills/`) and this repo's
  consumer copies (`.claude/commands/*.md`) replace their hand-ritual
  paragraph with the invocation plus a one-line statement of what the
  script does. `session-id.sh` becomes an internal callee; skills stop
  invoking it directly.
- **Consumers:** `check-stage-evidence` and `check-stage-entry` read the
  stamp and header the script wrote — unchanged, and deliberately so:
  they must keep verifying the artifacts, not trusting the writer.
- **Fields:** the stamp line and header field are existing grammar
  (lifecycle-kit/SPEC.md §The state machine); the script introduces no
  new fields, so every field already has its named readers in the two
  gates.

## Existing sections updated

- `lifecycle-kit/SPEC.md` — the tool joins the kit inventory beside
  `session-id.sh`; the flip+stamp protocol section names the script as
  the standard performer and the hand ritual as the fallback/override
  path.
- `lifecycle-kit/templates/skills/*` and this repo's
  `.claude/commands/{scope,align,build,validate,close}.md` — first-step
  text replaced by the invocation (the scope skill keeps its
  naming-the-iteration and triage judgment; only the reset+stamp+flip
  mechanics move).
- `lifecycle-kit/README.md` — tool listed.

Testing: the script is advisory tooling, not a gate — no fixture pair is
owed. It gets exercised end-to-end in `smoke/install.sh` (enter the first
stage in the scratch consumer, assert the truncated state file, stamp
format, and header; enter a second stage, assert append+flip; re-run,
assert idempotent no-op; run with the queue header absent, assert the
pre-flight refusal writes nothing).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
