# SPEC amendment: fanout-attribution

Queue entry: **`stage-fanout-burn-unbilled`**. Owning component: **drift-kit**
(`bin/stage-economics.sh` and drift-kit/SPEC.md §The stage-economics meter). The
basename drops the slug entirely — the slug names the *defect*, the amendment
names the *mechanism*, and the ref resolves as a bare basename tree-wide
(canon-kit/SPEC.md §check-amendment-queue).

This amendment closes the meter's last unbilled tier: a stage session's fan-out
subtree. It changes **one component's** contracts — drift-kit's. Its reads of
lifecycle-kit/SPEC.md §The state machine stay exactly what they already are
(read-only consumption of the stamp grammar; no stamp is added, moved, or
required), so no cross-component contract moves.

## What this amendment inherits and may not re-litigate

TRAJECTORY.md's closed ruling of 2026-08-06, in force: the iteration takes token
waste as its subject, and **`bin/stage-economics.sh` is extended to price the
fan-out subtree, so the reduction is measured rather than asserted**. That
ruling fixes *that* the tier is priced and orders it first. It does not fix
*how*, which is what this amendment rules.

Two rulings already in the SPEC bound the shape and are not reopened:

- **A degraded or absent measurement is visible, never silently folded**
  (§The report skeleton; the rule that decides the reserved `supervision` value).
- **A row rather than an apportionment**, wherever folding would need an
  allocation key with no basis in anything measured (§The stage-economics meter,
  the reserved `supervision` value).

## The attribution key — ruled, on a probe rather than on the hypothesis

The queue entry filed this unit `[design-pending]` on one open question: a
grandchild transcript's path names only the root session, never the stage
session between, so the parent edge has to come from somewhere other than the
path. The entry named a hypothesised "sibling spawn-record file" and instructed:
*probe for it before designing against it*. Scope re-verified by grepping the
repo for `spawn-record` and its spellings, found one hit — the entry's own prose
— and concluded the artifact was a hypothesis about the harness rather than an
observed artifact.

**That conclusion was drawn from the wrong corpus and is superseded here.** The
grep searched the *repo* for the entry's own coinage. The artifact does not live
in the repo and is not called that: it lives under `DRIFT_KIT_SESSIONS_DIR`
beside the transcripts, and it is named `<transcript-basename>.meta.json`. The
probe the entry asked for was run at this stage, against the sessions dir the
meter already reads, and the artifact is there.

**What the probe found** (this repo's sessions dir, 2026-08-06 — the population
is the whole dir, not a sample):

- The tree has exactly two transcript tiers, the two the meter already globs:
  root sessions flat, and every dispatched agent at
  `<root-session>/subagents/agent-<id>.jsonl`. There is no third tier — a
  grandchild sits **flat in the same `subagents/` directory** as a child. The
  entry's premise is confirmed: the path cannot carry the parent edge.
- Every nested transcript has a sibling `agent-<id>.meta.json`. Coverage was
  total (519 of 519); zero transcripts lacked one.
- The observed fields are `agentType`, `description`, `toolUseId`, `spawnDepth`,
  and — on deeper records — `parentAgentId` and `isFork`.
- `spawnDepth` distributes 1 / 2 / 3 across 347 / 143 / 29 records, so
  grandchildren and great-grandchildren are a real population rather than a
  worry.
- **Every** record with `spawnDepth` ≥ 2 carries `parentAgentId` (172 of 172),
  and **no** record with `spawnDepth` 1 carries it (0 of 347).
- **Every** `parentAgentId` resolves to a sibling `agent-<id>.jsonl` in the same
  directory (172 of 172; zero unresolved).

So the spawn forest is total and closed, and the parent edge is exact:

> **The parent of a `subagents/` transcript is the agent named by its meta
> record's `parentAgentId`, resolved in the same directory; a record with no
> `parentAgentId` is a direct child of the root session that names the
> directory.**

**Ruled: the meter reads that file.** The entry posed the ruling as *whether the
meter may read a harness-private artifact under no contract at all*. Three
grounds settle it yes:

1. **It is the same coupling class the meter already runs on, not a new one.**
   The meter already depends on two uncontracted harness shapes: the transcript
   JSONL usage schema (`.message.usage.cache_read_input_tokens` and its three
   siblings, read by `usage_by_model`) and the `<root>/subagents/` directory
   layout (read by `find_transcript` and by the supervision derivation). The
   meta sibling is one file over from both. Declining it while depending on
   those two would not be caution; it would be an inconsistency that costs the
   measurement and buys no isolation.
2. **The failure mode of the shape changing is bounded to a visible
   no-op.** If the harness renames the file, drops the field, or restructures
   the tree, the parent walk resolves no anchor, **zero** fan-out rows are
   emitted, one counted notice fires, and the affected transcripts fall back
   into the unstamped upper-bound counter — which is precisely where they sit
   today. A harness change can cost the meter this feature and a notice; it can
   never make the meter emit a wrong number. That is the difference between
   coupling to a shape for a *number* and coupling to it for an *enrichment*,
   and it is why the ruling can be yes here and would be no for the pricing
   arithmetic.
3. **The alternative is worse on the one axis that matters.** §The
   dispatcher-minted key, ruled out, below.

### The dispatcher-minted key, ruled out

The queue entry records a fallback as **design input, explicitly not a ruling**:
take the attribution key from the dispatcher instead — a dispatching session
already mints a path and names it in its child's prompt (this repo's
resume-journal idiom), so carrying the stage identity in that minted path would
make a fan-out attributable by a tracked artifact under this project's own
contract. It is refused, and the reasons are worth recording because the idea is
a good one for a different problem:

- **It is a convention, so it fails silently.** A dispatcher that forgets the
  convention produces a fan-out that attributes to nothing, and the meter cannot
  tell that case from a genuinely unattributable transcript. The failure mode is
  a **silent under-count in the exact tier the unit exists to stop
  under-counting** — strictly worse than ground 2 above, where the failure is a
  visible absence.
- **It does not remove the harness coupling, it relocates it.** Reading a minted
  path out of a child's prompt means parsing the child's transcript for its
  first user message — the same uncontracted JSONL shape, read for a *string
  the meter must then trust*, rather than for a field.
- **It cannot reach a fork.** A fork inherits its parent's context rather than
  receiving an authored prompt, so there is no minted path in it to read.
  Forks are 37 of the observed agent records and the entry's own motivating
  measurement was *five spawn-depth-3 forks* — the convention is blind to the
  population that produced the spike.
- **It taxes every dispatch forever** to buy what a file already on disk answers
  for free.

**Cross-unit consequence, stated because another unit's entry rests on it.** The
queue entry observes that a dispatcher-minted convention *"is also the
convention `cross-stage-census-duplication` needs, which is why the two may be
one design."* With the dispatcher-minted key ruled out here, **that shared
premise is gone**: this unit needs no dispatch convention at all, and
`cross-stage-census-duplication` must be designed on its own merits rather than
as the second half of one design. Nothing in this amendment forbids that unit
its own convention; it simply no longer inherits one.

## What changes

### 1. The fan-out row — a suffixed value in the existing stage-or-role column

**{design-bearing}**

A fan-out subtree bills to its own row, whose `<stage>` column carries the
anchor's stage-or-role value with `DRIFT_KIT_FANOUT_SUFFIX` appended — by
default `build+fanout`, `close+fanout`, `supervision+fanout`.

*A suffixed value rather than a bare reserved label.* A bare `fanout` label — the
literal shape `supervision` takes — would answer "how much did fan-out cost this
iteration" and lose *which stage's* fan-out it was, which is the question the
motivating measurement asked (one close's subtree, against seven closes'
subtrees). The suffix keeps the `(iteration, stage)` join intact and sorts the
subtree row adjacent to its own stage row.

*A separate row rather than folding the subtree into the stage row.* The queue
entry offers both — *"a fan-out row per (iteration, stage), or a subtree total
folded into the stage row with the split reported"* — and the fold is refused on
two grounds. First, separability **is** the deliverable: the operator's question
is how much the fan-out tier costs, and a folded row answers it only through a
stdout caveat that the trend log does not carry, so the close-over-close series
— the log's whole purpose — would be blind to the split it was extended to show.
Second, a fold silently redefines a value already logged: a `close` row written
before this change and one written after would mean different things under one
name, with no field distinguishing them, which is the drift the dedup key exists
to prevent rather than to hide.

*No new log field.* The four token fields, `cost`, and `date` carry a fan-out row
exactly as they carry a supervision row; the dedup key stays the
`<iteration> <stage> <model>` triple and a re-measure replaces a fan-out row's
line like any other. The split, the transcript count, and every degradation stay
**stdout caveats at measurement time** — a log field with no reader is a field
removed, and none of these has one.

### 2. The anchor set — what a fan-out row may attribute to

**{design-bearing}**

An **anchor** is a transcript that already carries a row of its own:

- every stamped stage session whose transcript resolved, anchoring its subtree
  to that stamp's `(iteration, stage)`;
- every lead that emitted a supervision row, anchoring its own direct fan-out to
  `(iteration, DRIFT_KIT_SUPERVISION_LABEL)` — a lead's audit-sweeps are as
  unbilled today as a stage's are, and leaving them out would fix half the tier
  the entry measured.

A lead that already carried a stage row yields no supervision row (the existing
invariant), and its fan-out anchors to that **stage** row instead — the anchor
lookup finds the stage anchor and needs no special case.

### 3. The parent walk — nearest anchor wins

**{design-bearing}**

For each `subagents/` transcript that is not itself an anchor and holds no row,
walk parents by the rule ruled above until an anchor is reached; the **nearest**
anchor takes the transcript's usage. Intermediate non-anchor agents are
transparent, which is what puts a depth-3 fork under the stage session that
ultimately caused it rather than under the sweep that happened to spawn it.

- A walk that reaches the root session without meeting an anchor emits **no
  row**; the transcript stays in the unstamped upper-bound counter, unchanged.
  That is the correct reading — an ordinary non-lifecycle session's fan-out
  belongs to no iteration and no stage.
- The walk is **bounded by the number of transcripts in the directory**. The
  observed forest is closed, but the file is uncontracted, so a cycle or a
  dangling `parentAgentId` must cost a counted unresolved notice rather than a
  loop. Fail-visible, like every other degradation on this tool.

### 4. The attribution invariant, restated to cover the new rows

**{design-bearing}**

The invariant becomes: **a transcript's usage is attributed to exactly one row
key** — either an `(iteration, stage-or-role)` pair or that pair's fan-out
value. An anchor never takes a fan-out row for itself, and a fan-out transcript
resolves exactly one anchor, so the existing `ATTRIBUTED` guard extends to the
new rows without a second mechanism.

Where a lead spans several iterations, its fan-out apportions **by the same
dispatch-count key, with the same integer split**, that already apportions its
supervision row. Reusing the key rather than minting a second one is the point:
two keys over one lead would let the supervision row and its fan-out row
disagree about which iteration the lead belonged to.

### 5. The collision rule for an overridden suffix

**{mechanical}**

The default suffix is collision-proof **by construction**: the stamp reader's
own stage-field alphabet is lowercase alphanumerics and hyphens, so a `+` can
never appear in a stamped stage name and `build+fanout` can never collide with
one. A consumer overriding `DRIFT_KIT_FANOUT_SUFFIX` can break that, so the
check mirrors the supervision label's exactly: if any stamp the run reads names
a stage ending in the suffix, emit a visible notice naming the knob and emit
**no** fan-out rows that run. It is checkable from the stamps the meter already
reads, so it adds no roster dependency and no second bound to drift.

**A suppressed anchor suppresses its fan-out.** When the supervision collision
rule fires, the lead-anchored fan-out rows are suppressed with the supervision
rows — a fan-out row for a role that emitted no row of its own would be an
orphan the reader cannot place.

### 6. `DRIFT_KIT_FANOUT_SUFFIX` — the one new knob

**{mechanical}**

One knob, following the established `<KIT>_<KNOB>` shape, defaulting to
`+fanout` and rostered in §Layout and configuration.

**Deliberately not knobs**, each ruled out for a stated reason:

- *A meta-filename or field-name knob.* A consumer on a different harness has a
  different artifact, not a differently-named one; the knob would be set by
  nobody and would imply a portability the mechanism does not have.
- *An opt-out for reading the meta layer.* Defaulting on makes it a knob nobody
  sets; defaulting off ships the feature as dead code. The degradation contract
  already gives a consumer the only thing an opt-out would buy — a run that
  emits no fan-out rows and says so.

### 7. Degradation — the contract that bounds the coupling

**{design-bearing}**

Every arm degrades to a visible absence, never to a wrong figure, and never to a
non-zero exit (the tool stays advisory):

| what is missing | behavior |
| --- | --- |
| the meta layer entirely | no fan-out rows; one notice; every existing row unchanged — the meter's behavior as shipped today |
| one transcript's meta record | that transcript takes no fan-out row and stays in the unstamped counter; counted in the unresolved notice |
| `parentAgentId` naming an agent with no transcript | same — counted, never guessed |
| a cycle or an over-long chain | same — the walk is bounded and the transcript counted |
| `jq` absent | already fatal to the whole join upstream; the fan-out pass never runs |
| the price table absent or missing a model row | the fan-out row's `cost` degrades to `n/a` and raises the existing incomplete-pricing caveat, exactly as a stage row's does |

## Producers and consumers

**New interface: the fan-out row.**

- *Producer* — a new pass in `bin/stage-economics.sh`, running after the
  supervision pass (it anchors on both stage rows and supervision rows, so it
  must see both) and before the unstamped-bound count (whose population it
  reduces). Its enabling config is the knob's default, so it is live on every
  run with no consumer action; the deployed configuration that must set it is
  none.
- *Consumer* — `DRIFT_KIT_STAGE_ECONOMICS_LOG`, via the unchanged `log_line`
  writer and its unchanged dedup key.
- *Readers, each at a named transition:*
  - the **`/economics` narrative** (`templates/economics.md`), at the report's
    cost-by-stage step, reading the fan-out row as a named line item beside its
    stage's — the new bullet in §Existing sections updated below;
  - the **operator**, reading the trend log close-over-close, which is the
    reading the motivating measurement was taken for;
  - the deferred **`benchmark-ab-experiment`** rung's measurement half, which
    consumes this log rather than rebuilding it and inherits the new rows with
    no change.

**No new field**, so the "every field has a named reader" obligation is
discharged by construction: the fan-out row populates exactly the four token
fields, `cost`, and `date`, each of which already has its named reader recorded
in §The trend log. The suffixed `<stage>` value is a new *value* in an existing
column whose reader is the same reader that column already has — the widened
meaning the `supervision` value already established.

**Nothing is produced on the lifecycle side.** No stamp is added, no cursor
moves, no stage-skill template changes. The fan-out edge is *derived*, exactly
as the supervision edge is, which is what keeps this a read-only consumption of
lifecycle-kit/SPEC.md §The state machine.

## Existing sections updated

Each named with the delta that owns it.

1. **§The stage-economics meter — The under-count bound** (delta 3). This is the
   load-bearing prose correction, and it must land or the SPEC contradicts the
   code. The section currently reads that the unstamped counter *"is an upper
   bound, never an attribution: a transcript carries no iteration and no stage,
   so nothing in the join could place it."* After this change that sentence is
   false for the fan-out population — a fan-out transcript **can** be placed,
   through its anchor. The bound survives, tightened: it now counts transcripts
   that resolved to no anchor, and the "most unstamped transcripts are ordinary
   non-lifecycle sessions" reading becomes more true rather than less. The
   sentence about a *continuation* needing the stamp side to record it stays
   exactly as it is — an unstamped continuation of a stage is still unreachable,
   and this change does not touch it.
2. **§The stage-economics meter — The trend log** (deltas 1, 4). The `<stage>`
   column's widened meaning already reads *"stage **or** cost-bearing role"*;
   it widens once more to admit a role's fan-out value, and the attribution
   invariant's statement is replaced with the one-row-key form from delta 4.
3. **§The stage-economics meter — the reserved `supervision` value** (deltas 2,
   5). Its *Blast radius of the widened column* bullet is re-verified rather
   than re-derived — the same three readers, the same conclusion — and its
   collision rule gains the suppressed-anchor clause.
4. **§Layout and configuration** (delta 6). `DRIFT_KIT_FANOUT_SUFFIX` joins the
   knob roster with its default.
5. **`templates/economics.md`** (delta 1). A new **Fan-out** bullet beside the
   existing **Supervision** bullet: report a stage's subtree as a named line
   item beside that stage's own figure, never folded into it, and say plainly
   that a per-stage figure excludes its subtree. Where a run's notice named the
   apportionment key or an unresolved count, repeat it so the reader can
   discount the row — the treatment the Supervision bullet already prescribes.
6. **§Testing** (delta 8 below). The new fixture's description joins the
   two-stamp and nested-tier fixtures already rostered there.

### 8. The fixture

**{mechanical}**

A third stage-economics fixture in `smoke/install.sh`, with **its own sessions
dir, state file, and log** for the reason already recorded for the other two —
the flat fixture set's log is asserted to hold exactly one line. It builds a
synthetic three-level tree: a flat `<lead>.jsonl`; under
`<lead>/subagents/`, a stamped stage transcript with a `spawnDepth` 1 meta
record, a child with `spawnDepth` 2 naming it as `parentAgentId`, and a
grandchild with `spawnDepth` 3 naming the child. It asserts:

- exactly one fan-out row for the stamped stage, its tokens the **sum** of the
  child and grandchild (so a walk that stopped at depth 2 reds);
- the stage row itself is unchanged and carries only the stage session's own
  usage (so a fold reds);
- the row's suffix comes from `DRIFT_KIT_FANOUT_SUFFIX` rather than a literal
  (the assertion the supervision fixture already makes for its label);
- the unstamped-transcript bound no longer counts the two attributed
  transcripts;
- **the degradation** — deleting the grandchild's meta record moves it back to
  the unstamped counter and raises the unresolved notice, while every other row
  is byte-identical. This is the assertion that makes ground 2 of the ruling
  above testable rather than asserted, and it is the one that must not be
  dropped for brevity.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition. *(No new field; the new row's producer,
      consumer, and three readers are named above.)*
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone. The
      six update targets in §Existing sections updated are the checklist.
- [ ] **The under-count-bound correction landed** — the superseded sentence is
      rewritten, not merely supplemented. A merged SPEC that still says nothing
      in the join could place an unstamped transcript is a failed merge.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
