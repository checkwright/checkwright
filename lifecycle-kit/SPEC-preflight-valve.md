# SPEC amendment: preflight-valve

Closes `close-entry-baseline-bootstrap-deadlock`. The state machine has no
in-contract path for its own "validate ends on an accepted red" case: close's
entry pre-flight refuses until every suite carries a clean evidence line, the
sanctioned way to make a known red clean is a baseline `fail` row whose blocking
slug must resolve to a **live** queue task, and the stage chartered to file that
task is the one the pre-flight is refusing. Every escape taken to date was the
operator-directed filing exception — a carve-out, not a mechanism — and it was
taken three times.

**The fork is ruled, and the ruling is the amendment's premise rather than its
subject.** The operator ruled the **documented one-shot pre-flight valve** on
2026-08-25; the other two candidates are closed. A baseline slug filed against a
gap-inbox bullet weakens the liveness assertion that makes the mark
self-retiring, and a configured permanent marker contradicts a red that is not
permanent. This file builds on that ruling and does not re-open it.

**The ruling's stated cost is constitutive of the deliverable, not a caveat on
it.** A valve gets reached for whenever close is inconvenient. So the
documentation is part of what was ruled: a one-shot that does not say what it is
for, and that reaching for it twice is the failure, is not the thing that was
ruled. Delta 4 is therefore a delta rather than a note, and it is the one whose
omission would leave the unit unbuilt while every mechanism in it worked.

**Three shapes ruled out here, each because it fails a property the tree already
holds.**

- **A `--force` flag on `bin/enter-stage.sh`.** §bin/enter-stage.sh says of the
  pre-flight refusal that it is "advisory in the same sense the gate is at commit
  time (no `--force`, so the easy path is the compliant one)". A flag is reached
  by typing, leaves no artifact, and generalises over every refusal the tool
  takes. The valve differs on all three axes: it is armed in a **committed
  file**, it carries a **mandatory written reason**, and it valves **one arm**
  rather than the tool.
- **A gate that reds on valve use.** It re-wedges the machine the valve exists to
  un-wedge, and it would fire in exactly the situation where the tree is already
  known to be imperfect — the shape gate-sdk/SPEC.md §When a gate earns its place
  rules against, since a gate that cries wolf trains its readers to bypass it.
  The decidable half — grammar, state token, iteration and stage match — is
  fail-closed **inside the writer**, where it is cheap and exact, on the same
  reasoning §bin/enter-stage.sh already gives for putting the rename's
  columns-2-to-last witness in the writer rather than in a `PreToolUse` hook.
- **A sixth field on the stamp line.** `check-stage-evidence` asserts the stamp
  grammar and several readers parse it positionally; recording the admission
  there would change a grammar with many readers to carry a fact the ledger
  already owns beside it. Stated as a non-target because the witness's
  read-to-`NF` spelling makes the field look free, and it is not.

## What changes

### (1) A one-shot valve admits one entry past a refusing entry pre-flight

`bin/enter-stage.sh` gains an arm on the `LIFECYCLE_KIT_ENTRY_PREFLIGHT` loop:
when a matching pre-flight command refuses **and** a valve ledger carries an
`armed` line for the entering iteration and stage, the entry is **admitted**
instead of refused, and the consumed line's state token is rewritten to `used`.
**{design-bearing}**

New knob **`LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE`**, a path to the ledger,
**defaulting to the empty string** — no valve, and every pre-flight refusal is
final, which is today's behavior exactly. An unconfigured consumer sees no
change; the tree that filed the defect sets it.

**The ledger's grammar** is the one this kit's evidence files already use: a
`# contract:` pointer header, then one data line per arming —

```
<iteration> <stage> armed|used <reason...>
```

`<reason>` runs to end of line and is **mandatory**. It is the field the ruling
makes constitutive, so a line without one is not a weaker arming, it is not an
arming.

**The admission is loud.** The admitted entry prints the pre-flight command's own
findings — the text a refusal would have printed — prefixed to say the valve
admitted them, then the reason, then **how many `used` lines this iteration
already carries**. The entry proceeds and stamps as normal.

**Four narrowings, each a consequence of an existing ruling rather than a
carve-out.**

- **This arm only.** The valve does not reach the built-in `check-stage-entry`
  pre-flight (which asserts the state machine's own stamp-protocol invariants),
  the predecessor-journal assertion (which already has its own named escape), or
  any iteration-boundary refusal — Lessons, the gap inbox, the linked-worktree
  check, `LIFECYCLE_KIT_BOUNDARY_REQUIRE` — each of which guards against work
  leaking across an iteration boundary and each of which already states a
  recovery. `LIFECYCLE_KIT_ENTRY_PREFLIGHT` is the **consumer-wired** arm, and a
  consumer-wired precondition is the only one whose deadlock a consumer can
  reach at all.
- **One line per admission.** Only the **first** matching `armed` line is
  consumed, so arming twice does not admit twice on one entry, and the second
  line is still there for close to see.
- **Iteration *and* stage must match.** An arming aimed at another stage or left
  from another iteration never admits. Both halves are asserted even though
  boundary truncation should make the cross-iteration case unreachable, because
  the truncation is a knob a consumer may decline to set and an assertion that
  rests on another consumer's configuration is not an assertion.
- **The idempotent no-op consumes nothing.** A re-entry whose stamp is already
  the last line exits 0 before the pre-flight runs at all, so a crashed-and-
  resumed session cannot spend a second valve line on the same transition.

**Two fail-closed refusals (exit 2, nothing written):** a data line with fewer
than four whitespace-separated fields, and a state token that is neither `armed`
nor `used`. A ledger that cannot be parsed makes "is it armed?" unanswerable, and
**both** silent branches are wrong there — admitting hides a malformed arming and
refusing hides a valid one.

**A configured path that does not exist is *not armed*, not an error**, because
header-only is the ledger's resting state and requiring the file would oblige
every consumer setting the knob to create one. The failure direction is a
refusal, which is the safe one. So that a typo'd path cannot masquerade as a
never-armed valve, **the refusal message names the configured valve path** when
the knob is set — the mistake becomes visible at the one moment it matters.

**`--simulate` reports the would-be admission and writes nothing**: it names the
valve line that would be consumed and the reason it carries, leaves the ledger
untouched, and exits 0, since the real entry would proceed.

### (2) Arming is the accepting stage's move, and it is not a queue write

`templates/stages/validate.md` gains the arming step: a validate that ends on a
**deliberately accepted** red — a suite whose failure is understood and is not a
regression from this iteration's diff — arms the valve for `close` and commits
the ledger, rather than stopping for an operator round-trip.
**{design-bearing}**

**Why validate can take this move when it cannot file the blocking task.** The
entry's whole finding is that validate may not pre-empt close by writing the
queue — a mid-iteration queue edit is what the gap inbox exists to prevent.
Arming the valve is not a queue write: it is an evidence-adjacent record on a
surface validate already writes at this exact point in its ritual, beside the
evidence manifest and the baseline diff. The move the machine lacked was never a
queue edit; it was a **hand-off**, and this is the artifact that carries it.

**The reason field is that hand-off's payload**, so validate writes what close
needs: which suite, what the red is, and why it is accepted rather than fixed.
Close reads it as the input to the task it is about to file.

### (3) A used valve is a close-stage obligation, not a free pass

`templates/stages/close.md` gains a disposition step over the ledger: for every
`used` line in the closing iteration, close **files the blocking task and lands
the baseline row that names it**, this session. **{design-bearing}**

That is the deadlock's actual resolution rather than a courtesy. The valve buys
close its entry; what makes the valve one-shot in substance — and not merely in
mechanism — is that the entry it bought is spent making the next iteration's
pre-flight pass without one. A close that enters through the valve and files
nothing has moved the deadlock forward by one iteration at the cost of a
record.

**A residual `armed` line is dispositioned too**, in the same step and with the
same weight: it means a session expected a refusal that never came, or armed the
wrong stage, and either is a fact about this iteration worth one line rather than
a file the boundary quietly truncates.

**The horizon is the iteration**, because the ledger is a
`LIFECYCLE_KIT_BOUNDARY_TRUNCATE` member (delta 6) and the first stage of the
next iteration clears it. So "how many times did we reach for the valve" is a
question with a bounded, committed answer for exactly as long as anyone can act
on it, and the ledger never accretes into a log nobody reads.

### (4) The documentation carries the cost as a constraint

`SPEC.md` §bin/enter-stage.sh states, in the valve's own contract, **what the
valve is for** — the one deadlock above, named — and **that reaching for it twice
in one iteration is the failure rather than a supported mode**. The refusal's
`help:` line names the valve **and its single sanctioned cause**, so it does not
read as a generic bypass. **{design-bearing}**

This delta is the ruling's second half. The mechanism without it is a documented
`--force` with extra steps, and the ruling's own words for the cost — "a valve
gets reached for whenever close is inconvenient" — are the prediction the prose
has to answer.

**What answers it is not a prohibition but a count at the moment of the act.**
The admission report prints this iteration's prior `used` count (delta 1), so the
second reach announces itself to the session taking it, in its own transcript,
before it proceeds. That is the same shape the predecessor-journal escape already
takes in §bin/enter-stage.sh: an evadable assertion whose value is that the
deviation becomes **deliberate and written** instead of silent, at the one moment
someone is looking.

**The honest limit is stated with it, not inferred from it.** A session can arm
its own valve. The valve is bypassable in exactly the sense the journal escape
is, and claiming otherwise would be the stronger claim the evidence does not
carry. What it buys is that a bypass leaves a committed artifact with a written
reason, a named obligation on the next stage, and a count that makes the second
one visible — where the operator-directed filing exception left a chat message
and three interrupts.

### (5) The stamp-commit purity exemption is generalized to its own principle

`check-stage-evidence`'s stamp-commit purity assertion gains the valve ledger,
**at any stage's stamp rather than the first stage's alone**.
**{design-bearing}**

**This is a gap delta 1 creates, found by enumerating the reader's red condition
rather than its subject.** That assertion says: where the state file is among the
staged paths and introduces a stamp, the staged set must contain **only** the
state file — plus, **for the first stage's stamp only**, the queue, the lesson
evidence, the survey record, the `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` members and
the gap inbox. An admitting entry at `close` writes the ledger *and* the stamp, so
committing them together reds, and committing them apart contradicts every stage
template's "commit the stamp on its own". Without this delta, delta 1's mechanism
lands and the very entry it exists to admit cannot be committed.

**The widening is a generalization, not a carve-out, and that distinction is the
whole argument.** The existing exemption set is already derived rather than
minted, and what it derives is one predicate: *the paths `bin/enter-stage.sh`
itself writes at this entry*. It is scoped to the first stage only because, until
now, the first stage's entry was the only one that wrote anything besides the
stamp. The valve ledger is written by the same tool at the same instant, so it
belongs to the set by the set's own rule; what changes is that the rule is now
stated as the predicate it always was, and the stage restriction rides the
*membership* (only an admitting entry writes the ledger, and a non-admitting one
leaves it untouched and therefore unstaged) rather than a hard-coded stage name.

**Safe to reason about by inspection in both directions**, which the section
already states of itself: widening the exemption set can only remove violations,
narrowing it can only add them. So this delta cannot manufacture a red elsewhere,
and the assertion it weakens is weakened by exactly one path, written by exactly
one tool, at exactly one branch.

### (6) The paperwork the new knob and the new tracked member owe

The knob joins §Layout and configuration's roster with its default and the reason
it defaults off; `lib/stages.sh` resolves it beside the other knobs; this repo's
`scripts/lifecycle-config.sh` sets it and adds the ledger to
`LIFECYCLE_KIT_BOUNDARY_TRUNCATE`; the ledger is created with its pointer header
and committed; and the `.gitattributes` merge-driver block is **re-emitted from
the live config** rather than hand-edited, since the supersede set is derived
from that array. **{mechanical}**

Every step here is oracle-running against a fixed battery: emit, run
`bash gate-sdk/bin/run-gates.sh`, and resolve what reds. No judgment is exercised
about *what* the paperwork says — deltas 1 through 5 decided that; this delta is
the sweep that lands it everywhere the derivation reaches. It is called out as
its own delta rather than folded into delta 1 because its fan-out is wide and
mechanical, and a batch can take it on a cheaper tier than the five above.

## Producers and consumers

**New state — the valve ledger's `armed` / `used` lines (deltas 1, 2, 3).**

- *Producer, two of them, and the split is contract.* The **validate-stage
  session** writes every line, by editing the configured ledger and committing it
  (delta 2). `bin/enter-stage.sh` writes **no line**: it rewrites the state token
  of exactly one existing line, `armed` to `used`, at an admitting entry. So the
  ledger's line set is the arming session's alone and the tool can only ever
  narrow what is admissible — which is why a two-writer surface is safe here.
  **Enabling config actually set:** `scripts/lifecycle-config.sh` sets
  `LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE` in delta 6, so the arm is live in this
  tree at the moment it lands; the empty kit default leaves it dead in every
  consumer that has not opted in, which is the intended posture and not an
  unreached producer.
- *Consumer, three, at three transitions.* (a) `bin/enter-stage.sh` at **stage
  entry**, matching iteration and stage on the `LIFECYCLE_KIT_ENTRY_PREFLIGHT`
  refusal branch. (b) The **close-stage session** at its ledger disposition step
  (delta 3), reading the closing iteration's lines. (c) A **reviewer** reading
  the arming commit and the admitting commit, the ledger being tracked — which is
  the consumer the ruling's "documented" is about.
- *Named reader for every field, at a named transition:*
  - `<iteration>` — read by (a) at stage entry, as half the match. Also the field
    that makes a leftover arming inert if the boundary truncation is declined.
  - `<stage>` — read by (a) at stage entry, the other half of the match.
  - `<state>` — read by (a) at stage entry (only `armed` admits), **written** by
    (a) at admission, and read by (b) at close's disposition step (`used` obliges
    the task filing, residual `armed` obliges a stated disposition).
  - `<reason>` — read by (a), which prints it in the admission report; by (b), as
    the input to the task it files; and by (c) in the diff. Mandatory, and the
    fail-closed arm exists because a reason-less line has no content for any of
    its three readers.
  - **No date field, and its absence is a decision.** The ledger is truncated at
    the boundary, so every line belongs to one iteration by construction, and the
    state file already dates that iteration's stages. A date here would be a
    second, drift-capable copy of a fact another surface owns, and the tree's own
    rule is that a field with no reader is removed rather than kept for
    plausibility.

**New interface — the admission arm (delta 1).**

- *Producer:* the non-zero branch of the `LIFECYCLE_KIT_ENTRY_PREFLIGHT` loop in
  `bin/enter-stage.sh` — an **existing** producer on an **existing** enabling
  path: this repo wires that knob today, and the close-entry manifest gate is one
  of its entries, which is the very command whose refusal the valve admits past.
  Nothing new must be configured for the arm to be reachable.
- *Consumer:* the **entering session**, through the admission report on stdout and
  exit 0; and the **committing session** immediately after, since an admitted
  entry writes both the stamp and the ledger and the two commit together — which
  they may only do because delta 5 widened the purity exemption to admit it.
- *Every field of the report has a reader:* the relayed pre-flight findings (read
  by the entering session, to know what it entered on), the reason (same reader,
  and the transcript), and the prior-use count (read by the session at the moment
  of the second reach — delta 4's whole mechanism).

**Deltas 2, 3 and 4 introduce no state, event or interface** beyond the above;
each is prose on a surface that already exists, consumed by the stage session
that loads it — validate at its ritual, close at its disposition steps, and a
reader of §bin/enter-stage.sh.

**Existing integration prose that describes the prior flow, updated in this
amendment rather than left to drift:** §bin/enter-stage.sh's pre-flight paragraph
(which states the refusal contract as unconditional), its `--simulate` roster
(stated in full and therefore read as a guarantee — a new arm absent from it
makes the roster false), and §Layout and configuration's knob roster. All three
are in the update-target roster below.

**Red conditions of every reader this change reaches** — enumerated as
canon-kit/SPEC.md §The causal-completeness check point 5 requires, **what makes
each reader red** rather than what it is about, since only a monotone verdict is
clearable by inspection:

- `check-merge-attrs` — reds when the tracked `.gitattributes` block and the
  block `lifecycle_merge_attrs_block()` renders from live config differ. **Not
  monotone**: delta 6 adds a `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` member, which
  extends the derived supersede set, so the block must be **re-emitted** through
  `bin/install-lifecycle.sh` in the same commit. This is the reader most easily
  missed, because the knob edit and the file that goes stale are in different
  kits' surfaces.
- `check-knob-citation` — reds on a knob defined in code and cited nowhere in its
  owning SPEC's roster. **Not monotone** (its red condition is a zero citation
  count): `LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE` reds until §Layout and
  configuration carries it.
- `check-knob-default-coupling` — reds when a code default and the SPEC's stated
  default disagree, and, per its own limit, has **no singularity assertion** — two
  agreeing literals pass. **Not monotone**: the empty-string default must be
  stated in both places and spelled the same way, and it must not be
  open-coded a second time in the consumer config.
- `check-workflow-tiering` — reds on a `.workflow/` member that is neither
  tracked nor ignored. **Not monotone** (its red condition is finding an
  unclassified member): the new ledger reds until it is committed.
- `check-spec-pointer` — reds on a `# contract:`/`# spec:` comment whose path does
  not resolve. **Not monotone**: the ledger's header must be pointer-form and
  resolve, on the pattern `.workflow/release-disposition.txt` already sets.
- `check-close-surfaces` — reds on a declared close surface missing a required
  field, and reports an `(undeclared)` capture surface. **Not monotone**: a new
  per-iteration capture surface under `.workflow/` owes its declaration, and
  delta 3's obligation is what that declaration's `reclaim=` names.
- `check-stage-evidence` — three red conditions, and only two of them clear by
  inspection. Stamp **grammar** and **name-axis** disagreement: monotone and
  cleared — no delta touches the grammar, and delta 1's admitted entry writes the
  same stamp a clean entry writes. The **stamp-commit purity** assertion is the
  third, and it **reds**: its condition is a staged path outside the exemption
  set while the state file introduces a stamp, and an admitting entry at a
  non-first stage stages the ledger beside the stamp. **Not monotone**, and it is
  the reader that forced delta 5. Recorded at this length because the first two
  arms clear so readily that stopping at "no delta touches the stamp grammar"
  would have shipped delta 1 with its own commit unable to land.
- `check-stage-entry` — reds on its own three assertions. **Monotone and cleared
  by inspection**: the valve deliberately does not reach this pre-flight (delta
  1's first narrowing), so its verdict is unchanged on every path.
- `check-evidence-manifest` / `check-evidence-baseline` — red on their stated
  assertions. **Monotone and cleared by inspection**: the valve changes what
  `enter-stage` does with a refusal, never what either gate asserts. Recorded
  explicitly because the deadlock is theirs and a reader will expect them to move;
  they do not, and that is the design — the valve is a lifecycle mechanism, not a
  weakening of the evidence contract.
- `check-lifecycle-registration` — reds when the resident registration block and
  the live config's rendering disagree. **Monotone and cleared by inspection**:
  that block is rendered from the stage roster, which no delta touches.
- `check-shim-restatement` / `check-skill-binding` / `check-stage-skill-coverage`
  — read the stage templates and their skill shims. **Not monotone** for the
  restatement arm (its red condition is an n-gram overlap between shim and
  template): deltas 2 and 3 add template prose, so the shims must not acquire a
  copy of it.
- `check-surface-duplication` — reds when a canonical definition appears on a
  second surface. **Not monotone**: the valve's contract is owned by
  §bin/enter-stage.sh and the stage templates must **point** at it, not restate
  it. Deltas 2 and 3 are the ones to diff against delta 4's text.
- `lifecycle-kit/gate-tests/enter-stage.test.sh` — reds when an entry's exit code,
  written state or emitted text differs from the asserted one. **Not monotone**
  (exact exits, exact text): existing cases run with the knob unset and are
  unaffected by the empty default, but new cases are owed for armed-and-admitted,
  armed-for-another-stage, armed-for-another-iteration, already-`used`, both
  fail-closed malformed shapes, the configured-but-absent path, and
  `--simulate` leaving the ledger byte-identical.
- `lifecycle-kit/smoke/` — reds on its own assertions over a live install.
  **Not monotone**: the `--simulate` scenarios there are the roster's executable
  statement, so the new arm's would-admit branch is owed a scenario beside the
  existing would-pass / would-refuse / would-no-op ones.

## Existing sections updated

- `lifecycle-kit/SPEC.md` §bin/enter-stage.sh — the pre-flight paragraph states
  the refusal contract as unconditional and must gain the valve arm, its four
  narrowings, its two fail-closed exits, the absent-path reading and the
  path-naming refusal message, placed with the predecessor-journal assertion
  whose refusal-and-escape shape it copies (deltas 1 and 4).
- `lifecycle-kit/SPEC.md` §bin/enter-stage.sh — the `--simulate` roster, which is
  **stated in full** precisely so that it is not read as a guarantee it does not
  give; the would-admit branch joins it in the order a real entry runs it, and
  the mode's write-nothing contract is restated for the ledger (delta 1).
- `lifecycle-kit/SPEC.md` §bin/enter-stage.sh — the knob roster at the section's
  foot, which enumerates every `lib/stages.sh` knob this tool reads (delta 1).
- `lifecycle-kit/SPEC.md` §Layout and configuration — the knob roster gains
  `LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE`, its empty default, and the reason the
  default is off (deltas 1 and 6).
- `lifecycle-kit/SPEC.md` §lib/stages.sh — the resolution-and-validation roster
  gains the knob's resolution and the fail-closed arms delta 1 names (deltas 1
  and 5).
- `lifecycle-kit/SPEC.md` §templates/stages/ — the per-stage contract prose for
  `validate` and `close`, which is where the arming move and the disposition
  obligation are owed a home beside the lesson and gap-inbox dispositions
  (deltas 2 and 3).
- `lifecycle-kit/SPEC.md` §Multi-operator semantics — the iteration-scoped
  supersede set is defined there as "exactly the surfaces enter-stage.sh
  truncates at the iteration boundary"; the ledger joining that set is a fact
  this section describes and a `.gitattributes` re-emit it implies (delta 6).
- `lifecycle-kit/SPEC.md` §check-stage-evidence — the stamp-commit purity
  assertion's exemption set and the first-stage-only scoping on it, restated as
  the predicate it derives from (delta 5).
- `lifecycle-kit/SPEC.md` §The stamp protocol, and every stage template's
  "commit the stamp on its own" sentence — the one exception an admitting entry
  creates, stated where the rule is rather than only where the valve is, since a
  session reads the rule at its entry step and the valve's contract three
  sections away (delta 5).
- `lifecycle-kit/templates/stages/validate.md` — the arming step, placed with the
  baseline-diff clause that already names the accepted-red case, pointing at the
  SPEC for the contract rather than restating it (delta 2).
- `lifecycle-kit/templates/stages/close.md` — the ledger disposition step, placed
  with the lesson and gap-inbox dispositions it is a sibling of (delta 3).
- `scripts/lifecycle-config.sh` — this repo's consumer config sets the knob and
  adds the ledger to `LIFECYCLE_KIT_BOUNDARY_TRUNCATE`, each with the `# spec:`
  one-line binding the file's other knobs carry (delta 6).
- `.gitattributes` — the merge-driver block, **re-emitted from live config**
  through `bin/install-lifecycle.sh` rather than hand-edited, because the
  supersede set is derived (delta 6). Its **fixtures do not move**, probed rather
  than assumed: `gate-tests/check-merge-attrs/{good,bad}` pin a block rendered
  against the *kit defaults*, where `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` is empty, so
  a consumer-side member reaches the tracked root file and nothing else.
- `.workflow/preflight-valve.txt` — created with its `# contract:` pointer header
  and committed, which is also what clears `check-workflow-tiering` (delta 6).
- This repo's close-surface declaration for the new `.workflow/` capture member,
  including the `reclaim=` field the boundary truncation satisfies (deltas 3
  and 6).
- `lifecycle-kit/gate-tests/check-stage-evidence/` and its `.test.sh` — the purity
  assertion's fixture pair and behavioral cases, which pin the exemption set delta
  5 widens; a widening asserted only in prose is a widening with no oracle
  (delta 5).
- `lifecycle-kit/gate-tests/enter-stage.test.sh` — the hermetic cases the reader
  enumeration above enumerates (all deltas).
- `lifecycle-kit/smoke/` — the `--simulate` would-admit scenario beside the
  existing three (delta 1).

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
      retired; nothing dangles. The specific claim to chase is any prose stating
      the entry pre-flight's refusal as unconditional.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The cost is documented as a constraint** — the SPEC says what the valve
      is for and that reaching for it twice in one iteration is the failure; the
      refusal's `help:` line names the valve and its single sanctioned cause. A
      build that lands every mechanism and not this has not built delta 4.
- [ ] **Owner-and-pointer held** — §bin/enter-stage.sh owns the valve contract
      and both stage templates point at it, diffed rather than assumed.
- [ ] **The derived surfaces are re-emitted, not hand-edited** — the
      `.gitattributes` block comes from `bin/install-lifecycle.sh` against the
      live config, and `check-merge-attrs` is green because of that and not
      because the two were typed to match.
