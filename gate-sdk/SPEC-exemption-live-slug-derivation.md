# SPEC amendment: exemption-live-slug-derivation

Queue entry: **`gate-exemption-live-slug-derivation`**.

`check-gate-exemption-tasks` decides whether an `# until: <slug>` exemption is
still backed by real work. It builds its live-slug set by reading **every
bold-emphasis token on every line** of the scanned span, so any bolded lowercase
word in queue prose joins the set and an exemption can resolve green against no
task at all. The gate's own clean line then asserts that "every element declares
until-with-live-task", which is the shape this iteration is named for: a green
that asserts less than it claims.

**Scope, fixed at scope and not re-opened here.** A former *section span* half
was struck 2026-08-02 on spec-over-precedent — §check-gate-exemption-tasks
specifies the positional span as deliberate and contractual, and names "nothing
enforces section order" as accepted residue. This amendment changes the **line
predicate only**; the span, its no-reset-on-unknown-heading behavior, and the
icebox coupling that rides on it are untouched and stay exactly as specified.

## The seam question, and the answer already sitting in the tree

The entry has always flagged this as a seam question: gate-sdk needs a slug parse
it can own **without** depending on queue-kit for the bullet format, which the
provenance seam forbids.

The answer is not new mechanism — it is the posture §check-gate-exemption-tasks
already takes for the *sibling* fact. That section already rules on the section
set in exactly these words: *"gate-sdk cannot depend on queue-kit for the section
set, so the coupling is carried by both SPECs rather than by code."* The bullet
lead-line format is the same kind of fact, and takes the same treatment: gate-sdk
carries the predicate as its own code, and the coupling is carried by SPEC prose
on both sides.

That is not a novel bet, and it is not even a new ruling — **queue-kit/SPEC.md
§The queue format already states this rule for a different kit**: "drift-kit
re-implements the definition rather than sourcing `lib/queue.sh`, because a kit
dependency the other way would close a cross-kit cycle; both implementations cite
this section." Re-implement, and cite from both ends. gate-sdk's case is the same
rule with a stronger reason — not a cycle but a layering inversion, since
gate-sdk is the substrate every other kit vendors.

Two kits already carry this exact predicate independently, and neither depends
on the other:

| Holder | Line | Predicate |
|---|---|---|
| queue-kit | `lib/queue.sh:95` (`queue_live_slugs`) | `/^[[:space:]]*-[[:space:]]+\*\*[a-z0-9][a-z0-9-]*\*\*/` |
| canon-kit | `lib/spec.sh:150` (`spec_queue_slugs`) | the same text, character for character |

gate-sdk becomes the third holder of the same text. What it must **not** do —
and the reason this needed a ruling rather than a build edit — is invent a
*fourth dialect*: a predicate that is nearly the queue format would fail
differently from its siblings on the same queue, and the divergence would be
invisible because all three run green on a well-formed file.

**Rejected: source queue-kit's `lib/queue.sh` from gate-sdk.** It is the obvious
DRY move and it is refused on the seam. gate-sdk is the *substrate* every kit
vendors; making it require queue-kit inverts the dependency direction and makes
a queue format a precondition for running any gate at all. A consumer that
vendors gate-sdk and no queue-kit must still get a working exemption gate.

**Rejected: a `GATE_SDK_LIVE_SLUG_RE` knob.** Config-via-env is the convention
for a *consumer's* posture — its section names, its surfaces, its vocabularies.
The bullet lead-line shape is not posture; it is the one format the gate's
`# until:` contract is written against, and a consumer free to redefine it can
redefine it into the fail-open this amendment is closing. Configuration adds
**no new knob**, matching the sentence §check-gate-exemption-tasks already ends
on for its scan roots.

## What changes

### 1. The live-slug derivation becomes a lead-line predicate {design-bearing}

`gate-sdk/checks/check-gate-exemption-tasks.sh`'s `IS_LIVE` walk stops scanning
every bold token on every line and emits **one slug per bullet lead line**: a
line matching `^[[:space:]]*-[[:space:]]+\*\*[a-z0-9][a-z0-9-]*\*\*`, from which
the first bold token is the slug. The `while (match(...))` multi-token loop goes
away with it — a lead line carries exactly one lead-in, so there is nothing to
iterate.

Design-bearing rather than mechanical: the delta is small in diff and the
judgment is in *which* predicate, which is the seam ruling above. Build adopts
the sibling text verbatim rather than re-deriving an equivalent one, and that
instruction is the substance of the delta.

**Measured at this HEAD** (the numbers build should reproduce before and after,
because a derivation change that cannot be counted is the vacuous-green shape
again):

| Set | Size |
|---|---|
| Today's token scan | **159** (the entry filed 160 at an earlier HEAD; the queue moved by one, which is the growth the entry predicted) |
| Lead-line predicate over the same span | **94** |
| Spurious members eliminated | **65** |
| Lead-line predicate vs. `queue_live_slugs` on this queue | **identical, 94 for 94** |

That last row is the correctness evidence and it is worth more than the other
three: an independently written predicate reproducing queue-kit's own answer
exactly, on a 94-entry queue, is what the SPEC-carried coupling is supposed to
buy. Build re-runs the comparison rather than trusting this table.

**The whitespace-tolerant form is adopted deliberately, and it is not a
tolerance — it is the format.** Anchoring at column 0 yields the same 94 on this
queue today, so local measurement cannot decide between the two forms. The owner
doc does: queue-kit/SPEC.md §The queue format specifies that "an *indented*
bullet with a bold lead-in is a sub-task (same grammar)", and that the slug
namespace spans "active + deferred + icebox + **sub-tasks**". A sub-task's slug
is therefore a real, live handle an `# until:` may legitimately name, and a
column-0 anchor would silently drop that whole class from the live set —
re-introducing a *fail-closed* version of the same defect at the other end. The
whitespace-tolerant form is what both siblings carry precisely because it is what
the format says.

### 2. The clean line reports the derived set's size {design-bearing}

`GATE-EXEMPTION-TASKS: clean (…)` today reports the exemption-array count alone.
It gains the live-slug count, so the line reads `N exemption array(s)` beside
`M live task slug(s)`.

This is the vacuous-pass tripwire §run-gates already names, applied to this
gate's *other* empty set. The array count already tells a reader when the gate
ranged over no arrays — that is how the fourth instance under
`vacuous-assertion-count-discipline` was found. Nothing tells a reader when the
slug set is empty or absurd, and both failures are silent in opposite
directions: an empty set turns every `# until:` red at once (loud, self-fixing),
while a 159-member set turns every `# until:` green (silent, and the defect this
amendment is closing). A number on the line makes the second one readable
without an audit, and it is the number whose *drift* would signal the predicate
breaking against a reformatted queue.

### 3. The fixture pair grows the case that pins the tightening {design-bearing}

The current pair cannot see this change: both cases' synthetic queues name their
slugs on bold lead lines and nowhere else, so old and new predicates agree on
them and the pair passes identically before and after. **A fix whose fixture pair
is green either way has no coverage**, which is precisely the class this
iteration is eliminating — so re-running the pair is not evidence and build must
not report it as such.

The `bad/` case gains what the defect actually looks like: its synthetic
`TASK-QUEUE.md` carries a bolded non-slug token in body prose (the real
population's shape — `**count**`, `**every**`, `**feature**`), and the fixture
gate's `# until:` points at *that token*. Under the token scan it resolves green;
under the lead-line predicate it is a violation. The case therefore fails before
the change and passes after, which is the property the present pair lacks.

**This is the delta with a cross-unit dependency, and it is the reason the batch
has an order.** The `bad/` case already fires two findings (a Done-only `# until:`
and an element with no disposition) and its `expect.txt` pins one line. Adding a
third finding and pinning it too needs a multi-line expect to mean a
conjunction — which is exactly what the sibling entry
`gate-fixture-expect-conjunction` changes, and which today would silently read as
alternatives. Build has two sound routes and must pick deliberately:

- **Land the sibling first** and pin all three findings as a conjunction. The
  stronger fixture, and the sibling is in this same iteration.
- **Land this unit first** and re-point `expect.txt` at the *new* finding's
  message alone, leaving the existing two lines asserted by exit code only.
  Sound, and weaker.

A third route — adding the case and leaving `expect.txt` as it is — is refused
outright. The pair would then pass on the old finding while the new case's
verdict rides on nothing, which is a fixture that reads as coverage and is not.

## Producers and consumers

**The live-slug set (existing state, changed derivation).**
Producer: the awk walk inside `check-gate-exemption-tasks.sh`, over
`$GATE_SDK_QUEUE_FILE` (default `TASK-QUEUE.md`) — the same producer as today,
reachable on every pre-commit run through the generated hook and on every
`run-gates.sh` battery. Its enabling configuration is unchanged and is already
emitted everywhere it must be: the queue-file knob and the scan-dir derivation
both keep their current defaults, so no configuration anywhere in the tree or in
a consumer needs to change for the new derivation to be live. That property is
what makes this delta safe to land without a migration note.
Consumer: the `until)` arm of the disposition case in the same script, which
tests `${IS_LIVE[$slug]:-}` — the set's one reader, unchanged. The set is
internal to one script and crosses no component boundary, which is why the whole
delta is a predicate swap and not an interface change.

**The live-slug count (new field on an existing line).**
Producer: the same walk, counted once after it completes. Reader, named because
a field without one is removed: the operator reading the gate's clean line at the
pre-commit transition, and — through `GATE_SDK_VERBOSE` — the same operator
reading a green battery for vacuous passes, which §run-gates already names as
that reading's purpose. It joins a line that already has a reader and an
established convention for carrying scanned counts, so it opens no channel.

**The bullet lead-line format (existing cross-kit interface, new holder).**
Producer: the operator or stage session writing a queue entry, under queue-kit's
lead-line rule — unchanged, and already gated by queue-kit's own lead-line gate.
Consumer: three independent parsers after this lands — `queue_live_slugs`,
`spec_queue_slugs`, and gate-sdk's walk. The coupling is carried by SPEC prose in
each kit, never by code, which is the same seam mechanism
§check-gate-exemption-tasks already uses for the section set. **The honest cost:
a queue-format change now requires three edits and no gate enforces the third**,
exactly as no gate enforces the section-set coupling today. That residue is
accepted on the same ground the existing one is — a cross-kit code dependency
would cost more than the divergence risk — and it is stated in the SPEC rather
than left for a reader to discover.

**Whole-component-set reader survey.** The gate itself is invoked by
`scripts/gates.list` (registry), the generated pre-commit hook, and
`run-gates.sh`; its fixture pair lives at
`gate-sdk/gate-tests/check-gate-exemption-tasks/`; its behavior is described in
gate-sdk/SPEC.md §check-gate-exemption-tasks, mirrored to `docs/gate-sdk/SPEC.md`
by the docs projection, and rostered in gate-sdk/README.md. Nothing outside
`check-gate-exemption-tasks.sh` reads its live-slug set — it is a local
associative array, never exported, never written to a file. Build re-runs this
survey against the tree before implementing, with **no `2>/dev/null` on any path
probe**: a silenced stderr on a mistyped path reads a live reader as absent,
which is the same false-negative shape this unit is about.

## Existing sections updated

- **gate-sdk/SPEC.md §check-gate-exemption-tasks** — the invariant paragraph's
  parenthetical defining a live task gains the derivation: the live set is the
  slugs on **bullet lead lines** within the scanned span, one per entry, not
  every bold token in the span. A new paragraph states the seam ruling — the
  lead-line format is carried by SPEC prose in each holding kit rather than by a
  code dependency, gate-sdk is the third holder of the same predicate text, and
  the three-edit residue is named. It sits beside the existing
  *live-section span is positional* paragraph, which already argues the identical
  seam for the section set and is otherwise unchanged (deltas 1, 2).
- **gate-sdk/SPEC.md §check-gate-exemption-tasks** — the clean-line contract
  gains the live-slug count and the vacuous-pass reading it serves (delta 2).
- **queue-kit/SPEC.md §The queue format** — the existing drift-kit
  re-implementation sentence generalizes: it already states the
  re-implement-and-cite-from-both-ends rule for one kit, and it becomes the
  statement of the rule with its holders named, gate-sdk among them. The entry
  grammar paragraph itself is unchanged — this amendment reads that format, it
  does not alter it.
- **canon-kit/SPEC.md §lib/spec.sh** — the `spec_queue_slugs` bullet notes it is
  one of three independent holders, for the same reason.
- **docs/site-architecture.md §Generated projections** — no new gate and no
  `# graph:` manifest change, so neither the pre-commit hook nor the graph
  artifact goes stale on this amendment's account. The docs mirror of
  gate-sdk/SPEC.md, queue-kit/SPEC.md and canon-kit/SPEC.md **does** restale, and
  is regenerated from its rostered command. Stated explicitly because a build
  session batching this with its siblings should regenerate once at the end, not
  per-unit.

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
