# SPEC amendment: port-declaration-shape

The malformed-shape validation for port-disposition declarations reaches only
registered gates, where the field is rarest, and never the tree-wide population
where it lives. This amendment widens it. It is the **enforcement-first pair**
of the `port-declaration-cohort-and-windows-leg` cuts rather than a cut of its
own: those three license every new tree-wide declaration this iteration lands,
each of them outside `gates.list` and so outside the only validation that
exists, and enforcement-first says the widening lands **with** the declarations
it must check rather than after them.

**What the failure actually is, stated precisely, because the entry's word for
it — *silently* — is easy to over-read.** The crate's disposition reader folds
every malformed shape into `Owed`: an empty cause, a slug-less hold, a doubled
field and a file carrying both are one rule, "a file that has not made a
reviewable declaration has not made one". That fold is correct for §port-blockers,
whose job is to count. Its consequence is that a malformed declaration is
**indistinguishable from no declaration**, and `--tree` is an `--emit` report
that no battery member runs. So a malformed declaration lands in a commit, reads
as made to every human reader, passes the whole battery green, and keeps being
counted as owed against the completion predicate. The silence is at commit time,
not in the oracle.

**Probed rather than assumed, before this was written**: over the whole tracked
non-test shell corpus at this iteration's head, **no** file carries a malformed
header declaration and **no** owed file carries a disposition token outside its
header block. The widening is green on landing, and its entire value is
prospective — which is the enforcement-first case, not an argument against it.

## What changes

### (1) Assertion G's corpus widens to the tracked shell tree

`§check-gate-substrate-parity` assertion G's shape clauses apply to any tracked
non-test shell file, not only to a registered member's declaration path
{design-bearing}. The corpus becomes the **union** of the declaration set
assertion A already derives and the tracked-shell-tree corpus §port-blockers
derives, de-duplicated against **both** halves of the declaration walk — the
in-scope set and the out-of-scope set — which is what makes the widening
monotone: it can add findings and never remove one, so no existing verdict flips
by inspection failure. The union is a union and never a replacement, for the
reason its sibling states: a `.gate` descriptor is no `*.sh`, so a corpus that
replaced the declaration set would silently drop every descriptor-borne field
and lose clause 1 entirely.

**Which clauses widen, and which do not.** Clauses 2 through 5 — a non-empty
cause, at most one `# no-port:`, a non-empty slug, and never both fields — read
a header field and nothing else, so they apply to a plain script unchanged.
Clause 1, *a `.gate` descriptor carries neither field*, stays on the declaration
set: a plain script is not a descriptor, so the clause has nothing to range over
there.

**A disposition is read from the header block alone over the tree corpus, and
the declaration corpus keeps its whole-file scan.** The tree corpus contains
scripts that *write* shell — smoke recipes, installers, template authors — and a
line-anywhere scan cannot tell a declaration from a heredoc literal. The
restriction is the field's own name rather than a new rule, both being **header**
fields. Leaving the declaration corpus alone is what keeps the widening monotone:
narrowing a scan that already shipped would retire findings, which a widening may
not do. This asymmetry is deliberate and reads like an oversight, which is why it
is stated; its sibling adopted it after a live run reported against a heredoc
literal in a smoke recipe.

**The tree half is scoped to the tree that authored the declaration**, on the
same ground and with the same predicate the gate already holds for assertion F's
publishing test: a vendored kit's malformed cause is the kit author's, and
asserting it in an adopter tree demands of an adopter something only the kit
author can satisfy. Where there is no tracked set the tree half is empty and the
declaration half still asserts, which returns exactly the pre-widening assertion
— monotone again, and the opposite disposition from §port-blockers' `--tree` arm,
whose *whole* subject is the tracked tree and which therefore refuses. Here the
tree corpus is an **addition** to a corpus the gate can still read.

**The clean line counts the two corpora separately**, so the counted-zero tell
survives the widening: assertion G's corpus was empty in this tree and its
verdict was green-with-a-counted-zero, and folding the tree count into the
declaration count would make an empty declaration half indistinguishable from a
populated one.

### (2) One derivation, two readers — the disposition reader gains a shape verdict

The crate must not grow a second implementation of the well-formedness rule
{design-bearing}. The rule the disposition reader already applies is made
explicit: it yields the shape fault it found — empty cause, empty slug, doubled
field, both fields — and the existing three-valued disposition becomes a
**projection** of that richer verdict, mapping every fault to `Owed`. So
§port-blockers' classifier and this assertion read one function, and the two can
never disagree about whether a declaration is well formed. This is criterion 6's
*unless* clause satisfied in its strongest form — the duplication is not
machine-held, it is **absent** — which is the same discharge the config bridge
takes and is preferred to a parity lane for exactly the reason criterion 6 gives:
a lane expires at the next edit to either side.

The disposition reader and the header-block read sit on the crate's universal
layer, and this delta makes assertion G a **third** named reader of them. That
sentence is an update target, not a side effect: §port-blockers states the pair
has "two named readers and no third".

### (3) The gate's manifest gains the corpus it now walks

`check-gate-substrate-parity`'s `# graph:` `couples=` field gains `scripts/*.sh`
and `kit:*.sh` {mechanical} — the exact tokens `check-gate-exemption-tasks`
already carries for the same corpus, so the spelling is precedented rather than
invented. Without it the most likely red arrives a tier late: a script gaining a
malformed declaration touches no file the current manifest names. The existing
tokens are kept rather than folded, on the ground assertion H's own coupling
paragraph already states for the `gate-sdk/SPEC.md` literal beside `kit:SPEC.md`.

### (4) The tree arm's coverage lands in the bespoke test, not the fixture pair

A fixture case directory is not a repository, so the tracked-shell-tree corpus
degrades to empty inside one and the pair cannot reach the new arm at all
{design-bearing}. `check-gate-substrate-parity.test.sh` stands up a throwaway
repository carrying tracked shell files that exercise each widened clause — a
bare `# no-port:`, a slug-less `# port-until:`, a doubled field, a file carrying
both, a well-formed declaration that stays clean, a disposition token below the
header block that is correctly *not* read, and the empty-corpus case outside any
repository. This is the shape its sibling already uses for its own tree arm, and
recording *why* the pair cannot carry it is what keeps a later session from
reading the pair's silence as a coverage hole to fill there.

### (5) The standing sentence that forbids this widening is corrected

`§The `# graph:` manifest` states that "assertions G and H do not widen with the
fields", and grounds it on a plain script having "no registry membership and no
`# spec:` pointer for H to open" {design-bearing}. That ground is **true of H**,
whose whole mechanism is opening the section a declaration's `# spec:` field
names, and true of **clause 1 of G**, which is about descriptors. It is false of
G's clauses 2 through 5, which need neither a registry membership nor a pointer.
The sentence is corrected to say what its own reason supports: **H** does not
widen, and G widens in the four clauses that read a header field while clause 1
stays on the declaration set. The same section's *Its readers* paragraph and its
plain-script paragraph — "the reader of a plain script's declaration is `--tree`
and, for a slug, §check-gate-exemption-tasks" — gain the third reader.

**This corrects an over-general sentence; it does not reverse its reason.** The
split the two fields already draw is unchanged and is what puts the widening
here rather than on the sibling: *a field's spelling-domain is the dispatch
seam's own partition, while a claim about the queue belongs with the gate that
already reads the queue.* Shape is spelling-domain. The sibling refusal that sent
slug **liveness** the other way turned on a second holder of one claim about one
queue, and this widening adds no claim and no coupling kind — assertion G already
reads shell declarations for exactly this property; what changes is which files
it reads.

**What it does not assert, deliberately, and the direction that makes that
safe.** Presence is still not asserted for either field: no clause demands that a
permanently-shell file declare, because permanence is a ruling in prose and a
gate deriving which files hold one would have to parse argument text. An
undeclared file is counted owed, which is the status-quo over-count, so the
mechanism still fails toward today's state rather than toward an under-count.
And slug liveness stays where it is: this gate gains no queue coupling.

## Producers and consumers

The new state is a **shape verdict** on the crate's universal layer, and a
**wider corpus** for one existing assertion. No new field, tag, knob, file
format, or output row shape.

- **Producer of the verdict** — the disposition reader on the crate's universal
  layer, computed from a file's header block, at every call. Its enabling path is
  unchanged: both callers already invoke it, and neither must be configured.
- **Consumer 1 — §port-blockers' three arms**, through the projection. Their
  behaviour is byte-identical to today's by construction, because the projection
  maps every fault to `Owed`, which is what the reader already returned. This is
  the property the port's own parity discipline would demand and it is
  discharged by construction rather than by a lane.
- **Consumer 2 — §check-gate-substrate-parity assertion G**, which reads the
  fault rather than the projection and emits one finding per malformed
  declaration, in the gate's existing per-finding output shape. It is the only
  new reader, and it is the reason the verdict is worth having at all: a field
  with no reader is removed, and this one is minted with its reader in the same
  amendment.
- **Consumer of the widened corpus** — the same assertion, at the same
  transition, over more files. Its input is `§port-blockers`' own corpus rule,
  an existing producer with an existing enabling path, so nothing new must be
  configured for the gate to see a live corpus.
- **Consumer of the manifest change** — `check-graph`, which holds the manifest
  against the generated hooks, and the generated hooks themselves, which bake the
  gate's resolved invocation. Both are named in the update targets.

**Every field has a named reader at a named transition.** The shape verdict's
fault kinds are read by assertion G at its own run, one finding each; the
projection is read by §port-blockers at classification. No third value is minted
— there is no fourth disposition and no fifth fault — and nothing is added for a
reader that does not exist.

**This delta widens a corpus rather than narrowing one**, so §The
causal-completeness check point 5's enumeration does not bind in the direction it
guards. It is stated anyway because the *pair* it ships with adds declarations,
and the two together must not be read as one narrowing: no reader over this
corpus reds on *finding none*, asserts an exact count, or holds a coverage floor,
and assertion G's own counted-zero is a print rather than an assertion. The one
reader that could have been narrowed — the declaration corpus's whole-file scan —
is deliberately left alone by delta (1).

## Existing sections updated

- `gate-sdk/SPEC.md §check-gate-substrate-parity`, assertion G — the corpus, the
  clause-by-clause split, the header-block restriction, the authoring-tree scope,
  the degradation direction and the two-corpus clean line; and its
  *corpus narrows to empty in this tree* paragraph, whose subject no longer holds
  (deltas 1 and 2).
- `gate-sdk/SPEC.md §check-gate-substrate-parity`, the manifest paragraph — the
  new `couples=` tokens and why the existing ones are kept (delta 3).
- `gate-sdk/SPEC.md §The `# graph:` manifest` — the "assertions G and H do not
  widen with the fields" sentence, its *Its readers* paragraph, and its
  plain-script-reader paragraph (delta 5).
- `gate-sdk/SPEC.md §port-blockers` — its statement that the disposition triple
  and the header-block read have "two named readers and no third", which this
  amendment makes three (delta 2).
- `gate-sdk/SPEC.md §check-gate-exemption-tasks` — its sentence that "this gate
  now walks a corpus assertion G does not", which stops being true; the ruling
  that sent slug liveness there survives on its stated load-bearing half, the
  second-holder-of-one-queue-claim accounting, and that is what the update says
  (delta 5).
- `gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate` — the
  assertion adds no member to the conservation table, the corpus being one this
  gate now walks itself; stated so the absence of a row is read as a ruling
  rather than an omission (delta 1).
- The generated `pre-commit`/`commit-msg` hooks and `docs/check-graph.html` —
  one set with one trigger, a gate's `# graph:` manifest (delta 3).
- The on-site SPEC mirror (all deltas).

<!-- update-target-exempt: the three cut amendments state this dependency from their own side and are not edited by it; each names this widening as the consumer that first reads its declarations -->
- The three sibling cut amendments — deliberately unwritten.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather
      than at the commit, this iteration carrying sibling amendments.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The projection is proved byte-identical, not asserted** — every
      `--emit port-blockers` arm captured before and after, same cwd and same
      argv, diffed with exit codes included, on the discipline every previous arm
      change in this section was held to.
- [ ] **The widened arm is red on each fault and green on the tree** — each of
      the four clauses driven to a finding in the bespoke test, and the live tree
      green, which is the probe this amendment already ran once and the landing
      commit re-runs.
- [ ] **It lands with the declarations it checks** — in the same iteration as the
      three cuts, which is the whole of its enforcement-first claim.
