# SPEC amendment: port-batch

The **fourth budget batch** of the native port — a *wide* cut, 6–8 members,
against the recent cadence of two to three a cut. It delivers one increment of
`native-gate-port-remaining-corpus`, whose deliverable is the whole corpus, so
its terminal move is a demotion rather than a Done move (canon-kit/SPEC.md
§Merging an amendment, the entry-outlives-the-amendment branch).

## The envelope

Stated first because it is what a build session needs before it cuts a batch,
and because it bounds what may be dropped without re-opening design.

**Asserted.** The batch is composed by the **budget arm**, at a width of 6–8
members drawn from the cheap clean band; each member takes its own descriptor,
its own registry entry, its own parity run and its own shell deletion, in its
own commit. The batch's price is measured, not reasoned, and its judgment is
ruled in delta 5.

**Not asserted, and deliberately so.** No **joint proof** binds the members.
They share no corpus derivation — the partition is 27 singletons — so no
member's parity run lends another proof, and **build may drop any member
without invalidating the rest**. Dropping one costs the batch one member and
nothing else. The two exceptions to that freedom are delta 3's ordering
constraint and delta 6's exclusion, both of which are about *sequence*, never
about which members are admissible.

**Out of envelope — escalate rather than absorb.** Widening the batch past
eight; admitting a member carrying a criterion-7 blocker or a design fork;
changing what a `.gate` descriptor may carry; and any ruling about
`check-tree-terms`' criterion-4 hold, which is
`cohort-held-members-port-prerequisites`' work and not this batch's.

**Where the width comes from, so the cut is a choice rather than a scramble.**
The band is ten members and it is **not uniform**: five clear criterion 4 and
five bind it, and a binder costs its fixture pre-work (delta 3) on top of the
port. So the marginal member gets more expensive as the batch widens — six is
the five clear members plus one binder, eight takes three. The width is the
operator's ruling; *which* members fill it is build's, inside this envelope, and
the price gradient is stated so that choice is priced rather than guessed.

## What changes

### 1. gate-sdk/SPEC.md gains `### The fourth budget batch`

Recording **the cut's findings and no member roster**, on the arm's own
record-only-findings rule: membership is derivable from the tree and the count
from `scripts/measured-claims.sh`'s `ported-gate-members`. A roster written here
would be a second copy that drifts the moment a member is dropped.
*design-bearing.*

### 2. The composition ground, and what is new about it

The **size arm is permanently exhausted**, not merely unattractive at this cut.
`bash gate-sdk/bin/port-blockers.sh --group` partitions the 27 owed members into
**27 groups of exactly one**, 0 undecidable — no group has a second member to
amortize a walk across — and since the 2026-08-14 born-native default no new
gate can arrive to form one. The budget arm's stated precondition (a `--group`
run reporting no takeable group) is met by that run. *design-bearing.*

**Width 6–8 is inside the arm's already-proven envelope rather than a novelty**,
and recording that is what keeps the widening from reading as a relaxation: §The
first budget batch took **six** members, and the drop-any-member property was
established at that width. What is genuinely new is the **selection ground**.
The first budget batch chose from the three smallest-*key* groups — a shared-
derivation proxy that no longer exists here. This cut composes by **declaration
size plus criterion cleanliness**: the cheap band is the members at or under
~103 shell lines with `c2=pair`, a generated-hook tier, `c7=clean` and no design
fork. *design-bearing.*

**The honest limit of that ground, stated so a later cut does not over-read it.**
Cheapness is **not** a shared derivation, so the batch amortizes nothing across
its members: per-member cost is unchanged from taking them one at a time. The
whole economy is **session overhead** — one cut, one criteria audit, one
assertion-C re-run, one criterion-5 residual measurement, one amendment — spread
over eight members instead of two. That is a real saving and it is the only one;
a later selector must not cite this batch as evidence that width is cheap in
itself. *design-bearing.*

### 3. Criterion 4 binds on half the band, and the ordering is the discharge

Surveyed at spec against the ten-member band, by reading each walk rather than
each name. **Five bind**: `check-gate-fail-closed` (its corpus *is*
`check-*.sh`), `check-gate-fixture-coverage`, `check-kit-enum` and
`check-gate-output` (each greps a resolved declaration path as text, which
§Meta-gate conservation's table already records for all three), and
**`check-docs-cname-parity`**. Five are clear:
`check-commit-msg`, `check-core-files`, `check-docs-link-convention`,
`check-battery-roster`, and `check-exec-bit` — which reads the index **mode**
column and never a byte, the *paths-as-a-set* carve-out applied to modes.
Build re-runs the verdict at the cut rather than reading it off this list; the
corpus is derived from the tree. *design-bearing.*

**`check-docs-cname-parity` is the finding, and it is the kind assertion C cannot
produce.** Its `couples=` is the single literal `docs/CNAME`, so the derived
substrate-sensitive set does not select it — and its *walk* nonetheless reaches
every tracked file, because `SITE_KIT_SCAN_ROOT` defaults to `.` and the rule
greps each file's bytes for URL hosts. Every kit's `checks/*.sh` and `*.gate` is
inside the corpus it scans as content, which is criterion 4's predicate verbatim,
reached through the **walk** rather than through the trigger field. §The
port-candidate criteria states that a conservation row and a criterion-4 hold are
independent facts; this is the first worked instance in the direction where the
**couple clears and the walk binds**, and it earns a sentence in that section
beside the two instances already recorded for the opposite direction.
*design-bearing.*

**`check-gate-fail-closed` is *not* excluded, and the reading that would exclude
it is named so it is not re-made.** §Meta-gate conservation's table dispositions
it *Retired with cause* — but that table's subject is what each meta-gate's
assertion is worth **about a `.gate`-dispatched member**, never whether the
meta-gate itself may port. Its corpus is full in every **vendoring consumer**,
where shell is not an exception but the only substrate, so the rule keeps its
whole reason to exist. What is true, and is the property to carry rather than the
verdict: its `check-*.sh` glob is **not substrate-adaptive**, so a ported sibling
leaves its corpus with no red — which that same row already rules correct, a
ported member having no shell to lint. *design-bearing.*

For each member the verdict binds on, the discharge is the one §The
port-candidate criteria already generalizes: **widen the fixture pair first,
then port**, the pair being the only corpus a port cannot invalidate; and the
**live-tree arm is demoted from proof to smoke**, its verdict recorded as *no
disagreement found on the pre-descriptor tree*, never as parity proved.
*design-bearing.*

**The pre-work was surveyed at spec, and it is real on every binder — and on
three members criterion 4 clears.** Each pair below is missing an arm of its own
derivation, so a port answering only the criterion's sentence would ship the hole
the sentence points at. That the gaps do not track the criterion-4 verdict is the
finding: a clear verdict says the *parity oracle* is not self-referential, never
that the pair reaches every arm. *design-bearing.*

- **`check-exec-bit`** — the sharpest, and criterion 4 clears it: both cases'
  index dumps carry **zero `.gate` paths**, so the descriptor-must-be-
  non-executable rule the conservation table records as *Retained, extended* is
  exercised by no case at all. A batch that lands eight descriptors under a rule
  nothing tests is the vacuity this pre-work exists to close, and it is owed
  whether or not this member is taken.
- **`check-gate-fixture-coverage`** — no case is a `.gate` descriptor carrying
  `# no-fixture:`, which is the live shape in the tree today.
- **`check-kit-enum`** — no `.gate` case; and its multi-kit hand-list branch fires
  on no live group, so the pair is the branch's only oracle in either substrate.
- **`check-gate-output`** — no case exercises a `.gate` member with no crate
  present, the out-of-reach branch.
- **`check-docs-cname-parity`** — both cases pass an explicit scan root, so the
  **default whole-tree arm — the very arm that creates the criterion-4 exposure —
  has no case**. Widening it is what makes the port's oracle reach the derivation.
- **`check-commit-msg`** and **`check-core-files`** — clear on criterion 4, and
  each leaves cheap arms untested (the default pattern-file resolution and both
  clean-skips; the `kit:` expansion, its wildcard refusal, and the
  untracked-but-present branch). Cheap to widen, and a port is when it is cheapest.

**One hazard was probed and is *false*, recorded because the mechanical reading
produces it and a build session will reach for it.** Porting the last shell gate
in a kit does **not** un-recognise that kit as a kit root: the derivation's
predicate is the **existence of the `checks/` or `smoke/` directory**, not a
`*.sh` glob, and four kits already carry zero `.sh` under `checks/` while
resolving normally. Nothing in the batch needs to defend against it.
*design-bearing.*

**The ordering constraint the width creates, which no prior batch faced at this
size.** Every port moves a declaration path from `<name>.sh` to `<name>.gate`,
and assertion A forbids the two spellings coexisting in one resolve dir, so a
member's shell form vanishes the instant its own port lands — while the live
corpus of every criterion-4-binding sibling moves with it. So: **every binding
member's live-tree comparison is captured against one shared pre-descriptor
snapshot, before the first descriptor of the batch lands** — one capture for the
batch, not one per member interleaved with the landings. An incremental order
silently compares each later member against a corpus its predecessors already
changed, and the resulting verdict reads like the earlier ones without being the
same claim. The fixture-pair runs carry no such constraint — `gate-tests` is
pruned from every live-tree walk — which is a second reason the pair is the
proof and the live tree is smoke. *design-bearing.*

### 4. Assertion C is re-run fresh after the last descriptor lands

On the rule §The first budget batch established: a port moves a declaration
path, which can move *other* members into or out of the derived
substrate-sensitive set, so the reading is never inherited from an earlier cut.
Any row it yields lands in §Meta-gate conservation for the binary substrate.
*mechanical.*

**The band spans four component directories** — gate-sdk holds seven of the ten,
with canon-kit, evidence-kit and site-kit one each — so unlike the three prior
batches this cut reaches gates outside gate-sdk's own dir, and a zero-row result
is one to check rather than to assume. Which dirs the *batch* spans is build's
cut, not this amendment's; assertion C is re-run either way. **This is
gate-sdk's meta-gate conservation assertion C, not lifecycle-kit's
`check-stage-entry` assertion C** — the two share a letter and nothing else, and
the second is the one this amendment's own cross-component body trips at the next
stage's entry. *mechanical.*

### 5. Criterion 5 is priced per member and paid per batch

The aggregate price is the **binary-less residual**, measured by
`installer_smoke`'s binary-less leg against the **post-batch** registry, from a
**clean checkout of the batch's own commit**, reached **by path** — the smoke
resolves its own tree from its script path and passes it to the packer as
`--root`, so cwd does not select it. *mechanical.*

**The growth is the count of the batch's `zero-config` members, not of its
members**, on the install disposition: a `never` or `on-surface` member is not
seeded into a freshly initialised consumer's registry, so no artifact-free
`init` can lose it. Probed at spec over the ten-member band: four are
`zero-config` and six are `on-surface`, so a batch drawn from this band grows
the residual **by at most four**, whatever its width. The number is measured at
build; this is the prediction it is checked against, and a divergence is a
finding. *design-bearing.*

**The judgment the amendment owes is ruled: accept and declare.** An adopter on
an uncovered platform loses each ported member and receives that omission
declared in its own `gates.list` rather than as a broken battery. The two rivals
are refused on the comment cohort's own grounds — restoring the class shell-side
reinstates the duplication the port deletes, which enforcement-first ranks below
removal, and a binary-gated declaration is what the omit path already is. The
honest limit rides with the ruling: this is a real subtraction for an uncovered
host, it lands because the 2026-08-09 directive ports the whole corpus, and it
shrinks as targets are published rather than being repaired here.
*design-bearing.*

### 6. Two members are excluded by name, and the ground is sequencing

- **`check-gate-exemption-tasks`** — the sibling rider `SPEC-port-until.md`
  widens this gate's invariant this same iteration. Porting a gate in the
  iteration that changes its contract makes the parity oracle chase a moving
  target and doubles the work. It sits at 106 shell lines, just outside the
  band, so the exclusion costs the batch nothing; it is stated by name so a
  build session sizing the band does not reach for it. *design-bearing.*
- **`check-tree-terms`** — carries a criterion-4 hold that is design work rather
  than a port, owned by `cohort-held-members-port-prerequisites`. Cheap by line
  count and **not** in the band. *design-bearing.*

`check-shellcheck` and `check-action-run-shell` (`c7=shellcheck`),
`check-docs-render-fidelity` (`c7=ruby`) and `check-gate-assertions` (`c7=paste`,
`c3=align-only`) are outside the band on the criterion columns and need no
separate exclusion — each is a **held** member the sibling rider gives a
declarable spelling. *mechanical.*

**`check-reads-couples` and `check-gate-binary-fresh` are outside the band on
*size*, not on their `c7=?`, and the distinction is worth a sentence because the
column invites the other reading.** Their `?` is `port-blockers.sh` failing to
resolve a command-position variable naming the crate's own binary — a scanner
limit, not a blocker, which the rider rules explicitly. At 219 and 107 shell
lines they are simply above the band; a later cut may take either without
retiring anything. *design-bearing.*

### 7. Per member, the port procedure is unchanged

`.gate` descriptor carrying only `# graph:`, `# spec:`, `# install:` and any
`# no-fixture:`; a Rust module registered in `gates::REGISTRY`; the
`gates.list` entry unchanged; the descriptor's `couples=` naming **every crate
module the implementation reaches, transitively**, including a module shared by
both sides of a compare; the shell original **deleted**; the cross-substrate
parity run taken while both implementations exist. *mechanical.*

**Two freshness obligations ride every commit** and neither discharges the
other: the full battery, which runs the crate's lint and test arms through
`check-crate-arms`, **and** `bash gate-sdk/bin/build-native.sh`. *mechanical.*

### 8. The seam, ruled per member rather than assumed

**Every member of this band reads consumer content, and none of it may become a
crate literal.** The band is unusually exposed here compared with the three prior
batches, and the exposure is the reason to rule it up front: `check-commit-msg`
resolves a **message-pattern file**, `check-core-files` a **governed-path
manifest**, `check-battery-roster` a **suite roster**, `check-docs-link-convention`
a **docs root**, `check-docs-cname-parity` a **host and a scan root**, and
`check-exec-bit` an **exec-glob set and a prune set**. Each of those values is one
project's vocabulary; a Rust `const` holding any of them publishes that project's
configuration as everyone's mechanism (CLAUDE.md §The provenance seam).
*design-bearing.*

The discharge is the one the config bridge already provides, and it is the
strongest form of criterion 6 rather than a concession: the bridge carries the
**resolved** value of a kit knob across the dispatch seam, so the value is
computed in exactly one place — the kit's shell library — and the binary holds no
default to drift from. A ported member therefore takes its inputs as bridged
knobs; the crate ships the walker and the rule, never the vocabulary. Two
consequences a build session should check per member rather than assume: a knob
the member reads must be in the emitter table (a hardcoded top-level flag is
bridged by nothing and would silently resolve platform defaults), and any
*fallback* the shell form carries must be re-examined — a fallback set is exactly
where a project-shaped literal hides in plain sight, which §The third budget
batch had to check for `check-root-tiering` and found generic. *design-bearing.*

### 9. The generated projections the batch stales

Every ported member changes `scripts/gates.list`'s resolution and the generated
pre-commit hook. The fan-out and each regen command are rostered at
docs/site-architecture.md §Generated projections; each freshness gate prints its
own command on red. Regenerate rather than hand-edit. *mechanical.*

## Producers and consumers

The batch introduces **no new state, event or interface**. Each member's port is
a substrate move behind an unchanged contract: the gate name, its `gates.list`
registration, its `# graph:` manifest grammar, its verdict and its clean-line
format are all invariant across the seam, which is the property the parity run
proves. That absence is the causal-completeness answer rather than a gap in it,
and it is stated because a batch of eight ports invites the assumption that
eight new things were introduced.

What each port *does* move is a **declaration path**, and its readers are named
and enumerated:

| reader | what it reads | red condition |
| --- | --- | --- |
| `gate_resolve` (§lib/gate.sh) | `<name>.gate` before `<name>.sh` in one resolve dir | both present in one dir |
| `gen-pre-commit` / `check-graph` | the manifest off whichever spelling resolved | manifest absent or malformed |
| `check-gate-substrate-parity` A/B | declaration set vs the roster `--list` prints | a descriptor with no subcommand, or the reverse |
| `check-gate-binary-fresh` | the descriptor set makes the binary load-bearing | binary older than the crate source stamp |
| `check-gate-fixture-coverage` | the fixture dir resolved for each member | a member with no pair and no `# no-fixture:` |
| `check-exec-bit` | file modes under `checks/` | a descriptor that is executable |
| `check-kit-enum` | which roots are kit roots, by `checks/` content | a kit root the registry does not know, or the reverse |
| `check-gate-exemption-tasks` | `check-*.sh` declarations under the check dirs | an exemption element with no live disposition |

**Three of those readers are themselves in the band, and that is the point of the
table** — `check-gate-fixture-coverage`, `check-exec-bit` and `check-kit-enum`.
A member the batch ports while it is reading a corpus the batch is changing is
exactly the shape delta 3's ordering constraint exists for; a fourth,
`check-gate-exemption-tasks`, is excluded by delta 6 because its collision is
with another *unit* rather than with a sibling.

**The narrowing question, answered rather than skipped** (canon-kit/SPEC.md §The
causal-completeness check, point 5). A port **narrows one corpus**: every
`<kit>/checks/*.sh` walk loses one file. So each reader above is enumerated by
its **red condition**, not by its subject, and the non-monotone ones are the
work:

- **`check-exec-bit`** is the sharp one, and the port is a **swap** rather than a
  removal: the narrowing deletes a file whose required mode is *executable* and
  adds one whose required mode is *non-executable*. Its red condition is a wrong
  mode, so the narrowing **adds** a violation class rather than removing one —
  point 5's own attested shape. Its pair carries no `.gate` case, so the arm that
  would catch a batch of eight wrongly-moded descriptors is currently
  unexercised; delta 3's pre-work is what closes it, and it is owed whether or
  not this member is in the batch.
- **`check-kit-enum`** reds on a *set inequality* between kit roots and registry
  members, which is non-monotone in both directions and therefore not clearable
  by inspection — so it was **probed rather than argued**. The kit-root predicate
  is the existence of a `checks/` or `smoke/` **directory**, not a `*.sh` glob, so
  porting a kit's last shell gate cannot remove a root. The verdict is clear, and
  it is recorded as a probe result because the plausible reasoning gives the
  opposite answer.
- **`check-gate-fixture-coverage`** reds on a member with no resolvable pair.
  A port keeps the pair and re-points it, so the verdict is monotone here — but
  only if the pair survives the deletion of the shell original, which is the
  failure §The third budget batch already paid for once.
- **`check-gate-fail-closed`** reds on a naked capture in a `check-*.sh`. Its glob
  is not substrate-adaptive, so each port removes one file from its corpus with no
  red — a *silent* narrowing, and the one place a reader should confirm the
  removal is intended rather than assume it. It is: a ported member has no shell
  to lint, which §Meta-gate conservation already rules.
- The remaining readers red on *presence of a malformed thing*, monotone under a
  narrowing, and are clearable by inspection.

**No probe silences stderr.** A `2>/dev/null` on a path grep reads a bad path as
"no reader" and manufactures exactly the false negative this section exists to
close.

## Existing sections updated

- **gate-sdk/SPEC.md §Porting a gate to the binary substrate** — gains §The
  fourth budget batch as a sibling of the three prior batch sections. Owned by
  delta 1.
- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** —
  the size arm is now **permanently** exhausted rather than exhausted at a cut,
  because born-native forecloses a new group forming. That is a change to the
  *rule*, not to a cohort, so it lands in the rule's own section. Owned by
  delta 2.
- **gate-sdk/SPEC.md §The port-candidate criteria, criterion 4** — the
  couple-clears-walk-binds instance. That criterion currently records two worked
  instances, both of the opposite kind (a member the derived substrate-sensitive
  set selects and criterion 4 clears); `check-docs-cname-parity` is the first in
  the direction that matters more, because that direction produces a *missed*
  hold rather than an over-selected row. Owned by delta 3.
- **gate-sdk/SPEC.md §The port-candidate criteria, criterion 5** — the batch's
  measured residual and its accept-and-declare judgment join the worked
  instances there. Owned by delta 5.
- **site-kit/SPEC.md §check-docs-cname-parity** — the gate's own section records
  that its default scan root makes the whole tracked tree its content corpus, and
  what that costs its port. site-kit already carries the sibling precedent: its
  renderer knob's section records that the knob's *value* is a port blocker,
  because the dependency is spelled nowhere in the gate's source. Owned by
  delta 3.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — any
  row assertion C's fresh run yields. Owned by delta 4.
- **The per-gate SPEC section of every ported member** — each gate's own section
  records what its port cost and any finding it produced, on the rule that a
  design ruling for a gate lives beside that gate's rule. Owned by delta 7.
- **`TASK-QUEUE.md`'s `native-gate-port-remaining-corpus` entry** — the
  ported/owed split is a dated oracle read, never a count the entry holds; the
  entry records this batch as the fourth and demotes. Owned by delta 1.

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
