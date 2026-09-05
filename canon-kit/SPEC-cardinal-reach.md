# SPEC amendment: cardinal-reach

**Whether a written count is enforceable stops depending on how it is spelled, and a count in
a work queue gains a way to be bound to an oracle without the queue inheriting a manifest's
ban.** Three separable calls, each answered here on a measurement taken this session rather
than on the entry's stated premises, two of which did not survive probing.

The unit joins this iteration under the operator's 2026-09-05 ruling, relayed through the lead
session, against a scope recommendation to hold. That composition is closed and is not
reopened here.

**Two of the entry's inputs are corrected by this amendment's own probes**, and both
corrections change what the deliverable is:

- **The cardinal grammar exists in *two* places, only one of which executes.** The entry names
  `SPEC_COUNT_CARDINAL_RE` in `canon-kit/lib/spec.sh:241` as the implementation. It is not:
  every canon-kit gate is ported, `canon-kit/checks/` holds `.gate` descriptors and no `.sh`
  file, and the executing matcher is `native/src/spec.rs:712-714`. Probed tree-wide,
  `SPEC_COUNT_CARDINAL_RE`, `spec_count_quantifier_re`, `spec_count_awk_lib` and
  `spec_count_noun_alt` have **no consumer outside the file that defines them** — so the shell
  copy is dead code inside a live, permanently-shell library, with nothing holding the two
  ceilings equal.
- **`CANON_KIT_MEASURED_SURFACE_GLOBS` is *derived* from `CANON_KIT_MANIFEST_FILES`** — it is
  that array plus `.claude/commands/*.md` — so widening the manifest corpus cascades onto
  `check-measured-claim` and `check-unmarked-claim` as well as onto `check-manifest-count` and
  `check-prose-enum`. The entry treats limb (b) as reaching two gates; it reaches four. That
  derivation is also what makes delta 4's narrower answer available at all.

## What changes

### (1) The dead shell copy of the cardinal grammar is deleted

`canon-kit/lib/spec.sh`'s `SPEC_COUNT_CARDINAL_RE` and the three helpers that read it are
removed {mechanical}. The library keeps its `# no-port:` declaration and every function that
still has a caller; only the count-grammar family goes.

**The ground is the doctrine's, not this unit's:** removing a duplication outranks gating it,
and a value computed in two places with no machine holding them equal is the shape
§lib/gate.sh's *exactly one place a value is computed* rule exists against. It is also the
precondition for delta 2 — extending a ceiling that lives in two copies means editing both and
hoping, which is the position this entry was filed to end.

**Nothing documented is deleted.** Probed: `canon-kit/SPEC.md` names neither the variable nor
its helpers, so the family is not part of the library's stated API and no shipped sentence
goes stale. A consumer's own shell gate sourcing the library loses three symbols the SPEC
never offered it.

### (2) The word branch is extended to the single-token cardinals, and the boundary gains its reason

This is the amendment's central design ruling {design-bearing}.

**What is actually wrong, stated precisely.** The digit branch is `[0-9]+`, unbounded; the
word branch is a fixed eleven-word list stopping at `twelve`. So `13 gates` is reachable and
`thirteen gates` is not, and `canon-kit/SPEC.md §check-manifest-count` states the ceiling
without justifying it. Probed: `git log -S twelve -- canon-kit/` reaches the originating
commit, whose message records the grammar and no reason; the SPEC explains why **`one`** is
excluded — "singleton and cardinality-rule idioms … are invariants, not totals" — and says
nothing about the upper bound. **No reason is recorded anywhere.** The entry's own guess at
one is marked unattested and this probe confirms it.

**The ruling: the word branch covers exactly the cardinals English spells as a single token —
`two`…`twenty` and the tens `thirty` through `ninety` — and the boundary is stated with its
ground.** The ground is the matcher's own shape rather than a style preference: the grammar is
**token-based**, matching a word between word boundaries, and every cardinal above ninety-nine
or between the tens is a hyphenated or multi-word construction that no single-token pattern
can reach. So the boundary is where the notation changes, not where a list happened to stop.
That is a rule a reader can apply to a spelling the table does not contain, which is what
`twelve` never was.

**The residue is named rather than left implicit.** A hyphenated compound —
`twenty-one gates` — stays unreachable while `21 gates` does not, so the discontinuity is
narrowed rather than closed. Measured this session, the tracked tree carries **35** hyphenated
compound cardinals across the manifest corpus and the queue combined. A compound grammar is
**refused**: it would have to distinguish a quantifying compound from the far commoner
partitive and ordinal prose the same tokens appear in, and the false-positive surface of that
distinction is worse than the hole it closes. Recorded as a stated limit with its own ground,
which is the state this delta is buying for the whole grammar.

**The extension is measured at build and not cleared by inspection**, and the amendment says
so because the number is large enough to matter: `check-manifest-count`'s red condition is
**finding** a bare cardinal, so widening the matcher can only **add** violations, and the
manifest corpus carries **125** occurrences of the newly-admitted words today. Most will be
ordinary prose that no governed-collection noun sits near, but any that reds is a **real
claim that was unreachable until now**, never a false positive to exempt. If the found set
exceeds what this cut can honestly repair, the residue is filed as a costed entry under the
gap-disposition rule rather than blanket-exempted.

**Extending the table also widens `check-measured-claim`'s Arm C**, which normalizes a
marker's spelled cardinal to digits so that a marker's `12` and a sentence's `twelve` are one
cardinal. Under the extension a marker can bind a claim spelled `thirteen`, which it could
not before — a strict gain, since Arm C's red condition is a marker whose cardinal is *absent
from* its bound claim and an unreachable spelling read as absent is the false red this
extension removes.

### (3) The remaining ceiling has one holder, and the SPEC states the rule rather than the list

`canon-kit/SPEC.md §check-manifest-count` currently transcribes the word list into prose
{mechanical}. After delta 1 there is one implementation, and the section is rewritten to state
the **rule** — digits unbounded; spelled cardinals to the single-token boundary, with `one`
excluded on its existing stated ground — and to cite the implementation for the list itself,
under the de-literalization rule that prose cites names while code owns values. A transcribed
list is a second copy that drifts the next time the boundary moves.

### (4) `TASK-QUEUE.md` joins the measured-surface corpus and **not** the manifest corpus

The entry's limb (b) asks whether the queue should join `CANON_KIT_MANIFEST_FILES`. The answer
is **no**, and the narrower widening that buys the value is available because the two knobs are
related by derivation rather than by identity {design-bearing}.

**Why the manifest corpus is refused.** `check-manifest-count`'s ban is a **manifest** tier
rule: a manifest states rules and points at owners, so a bare cardinal in one is a parallel
copy of a collection's size. A queue entry is a different tier — a dated work record whose
idiom **is** attestation, and whose entries are line-capped per entry. Importing the ban would
oblige a marker or a rewording on a surface that turns over every iteration, at a budget cost
per entry, for counts that frequently have **no tree oracle at all** to bind to — the entry's
own 2026-09-02 recurrence is the worked case, a maintained total that counted CI rounds and
therefore had no `measured:` key available. Widening a corpus onto a surface where the remedy
the gate points at does not exist converts a real defect into an unfixable red.

**What is widened instead.** `CANON_KIT_MEASURED_SURFACE_GLOBS` gains `TASK-QUEUE.md` as an
explicit element beside the derived ones. The effect is exactly the half the entry's cost line
asks for and no more: a queue author who **wants** a count watched writes a `measured:` marker
and `check-measured-claim` re-runs its oracle at every commit, so a queue count stops being
"unreachable by any oracle" the moment its author binds it. No bare cardinal is banned, no
entry is obliged to change, and no line-cap budget is spent except by an author choosing to
spend it.

**The cascade onto `check-unmarked-claim` is measured, not assumed.** That gate reds when a
paragraph matches a **consumer-declared** claim class with no marker. This repo declares one
live class, `gate-substrates`; probed against `TASK-QUEUE.md` it returns **zero** matches, so
the widening arms the mechanism without arming a single violation today. It stays
consumer-configurable, which is the property that makes the widening safe rather than lucky:
a class this repo later declares will reach the queue deliberately.

**`check-measured-claim`'s Arm B fails closed on an unknown key**, which is the one red this
widening can produce from nothing: a queue marker naming an oracle key that does not exist
exits 2 rather than passing. That is the correct behaviour and is stated here so that a build
session meeting it reads it as the mechanism working.

### (5) Limb (c), a digits convention for the queue, is declined with its ground recorded

The entry offers (c) as the mechanical limb that can land alone. It is **not landed**
{design-bearing}, and the ground is recorded on the entry so it is not re-proposed as an easy
win.

**Its motivation is consumed by deltas 2 and 4.** (c)'s value was that notation decides reach;
after delta 2 notation no longer decides reach in the manifest corpus, and after delta 4 the
queue takes no cardinal ban at all, so a spelling choice in the queue decides nothing. What is
left is the compaction argument, which the entry itself measures at roughly nine lines
file-wide and calls the weak half.

**Its home would not have been canon-kit either.** Probed: `queue-kit/SPEC.md` states no
numeral or notation convention today, while `scripts/gates.list` already registers six
`check-queue-*` members treating the queue as a capped, wrap-gated, budget-gated data surface.
A queue notation rule is queue-kit's by ownership, so landing it in canon-kit would have put a
queue convention in the manifest kit — recorded because the entry's framing invites exactly
that.

### (6) The gate couplings are widened with the corpus, or the widening does not fire

A corpus knob is only half a producer's enabling configuration {mechanical}.
`check-measured-claim.gate` and `check-unmarked-claim.gate` already couple `*.md`, which
reaches `TASK-QUEUE.md`, so delta 4 needs no coupling edit — verified rather than assumed.

**`check-manifest-count.gate` and `check-prose-enum.gate` couple
`*SPEC*.md,*README.md,CLAUDE.md`, which does not**, and this delta records that as the trap
limb (b) would have sprung: a session widening `CANON_KIT_MANIFEST_FILES` without widening
those two `# graph:` manifests would ship a knob whose gate never triggers on the file it was
widened for, catching the violation only in a full-battery run. Delta 4 declines that widening,
so no coupling edit is owed — but the fact is written into the merged section because it is
the first thing the next session proposing limb (b) needs and the last thing it would think to
check. The generated hook is regenerated in the same commit as any coupling edit, per the
generated-projection roster.

## Producers and consumers

**No new state, event or interface is introduced.** This unit changes one matcher, one corpus
knob, one SPEC section and deletes dead code. The causal-completeness points are answered
about the two values whose producer or reach changes.

**The cardinal grammar's producer set narrows from two to one.**
*Producer before* — `native/src/spec.rs:712-714` (executing) and `canon-kit/lib/spec.sh:241`
(dead). *Producer after* — `native/src/spec.rs` alone.
*Consumers, each with the transition at which it reads* — `check-manifest-count`, when it
scans a manifest file at pre-commit; `check-prose-enum`, sharing the same finder;
`check-measured-claim` Arm C, when it verifies a marker's cardinal appears in its bound claim.
Every one of them reads the compiled matcher; none reads the shell copy, which is what makes
delta 1 a deletion rather than a migration.

**`CANON_KIT_MEASURED_SURFACE_GLOBS`' extent widens by one file.**
*Producer* — `scripts/canon-config.sh`, this repo's consumer config, which the bridge sources
to resolve the knob; the kit ships the knob and no value, so no adopter inherits this repo's
choice. *Consumers* — `check-measured-claim` and `check-unmarked-claim`, each at the pre-commit
transition, reached through their existing `*.md` couplings (delta 6).

**Every field has a named reader.** The only datum this unit adds is a `measured:` marker's
availability inside a queue entry. Its reader is `check-measured-claim`, at the transition
above; the marker's key resolves through the existing measured-claims oracle, and a key with
no oracle is Arm B's fail-closed exit 2 rather than a silent pass.

**This delta set *widens* two corpora and *narrows* one producer set, so point 5 binds on the
narrowing and the widening's readers are enumerated for the opposite reason — a widening adds
violations rather than removing them, and none of these readers is safe to clear by
inspection.**

- **`check-manifest-count`** reds on **finding** a bare cardinal adjacent to a
  governed-collection noun. Delta 2 widens what counts as a cardinal over an unchanged corpus:
  monotone increasing, so it must be **run**, and delta 2 states the 125-occurrence surface it
  is run against.
- **`check-prose-enum`** holds a **coverage floor** — two or more hand-listed members of a
  declared set in one paragraph with a member omitted. Delta 3 rewrites a section that
  currently transcribes a word list; replacing a list with a rule removes members from a
  paragraph rather than adding them, which is the direction that can trip a coverage floor.
  Run, not inspected.
- **`check-measured-claim`** reds on Arm A (oracle value ≠ marker value), Arm B (unknown key,
  exit 2) and Arm C (marker cardinal absent from its claim). Delta 4 widens its corpus and
  delta 2 widens Arm C's grammar; the first can add violations and the second can only remove
  false ones. Both run.
- **`check-unmarked-claim`** reds when a declared claim class matches a paragraph carrying no
  marker. Its corpus widens under delta 4; measured at zero matches today, and re-run rather
  than trusted.
- **`check-knob-citation` and `check-knob-default-coupling`** red on a knob mentioned without
  its owner and on a default spelled twice. Delta 1 removes three symbols and delta 4 adds an
  explicit array element, so both are exercised and neither is assumed.

## Existing sections updated

- **canon-kit/SPEC.md §check-manifest-count** — the cardinal grammar restated as a rule with
  its single-token boundary and that boundary's ground, the hyphenated residue named with its
  refused remedy, and the word list cited rather than transcribed (deltas 2, 3).
- **canon-kit/SPEC.md §check-measured-claim** — Arm C's cardinal grammar follows §check-manifest-count's
  and gains the widened reach; the queue's new membership in the measured surface and Arm B's
  fail-closed behaviour on a keyless marker (deltas 2, 4).
- **canon-kit/SPEC.md §check-unmarked-claim** — its corpus is the shared derivation, so the
  widening reaches it; the measured zero-match reading is recorded as a reading, not a promise
  (delta 4).
- **canon-kit/SPEC.md §Layout and configuration** — `CANON_KIT_MEASURED_SURFACE_GLOBS`'
  relationship to `CANON_KIT_MANIFEST_FILES` stated where a consumer widening either will
  read it, and the coupling obligation delta 6 records (deltas 4, 6).
- **canon-kit/SPEC.md §lib/spec.sh** — the library's function inventory after delta 1.
- **`canon-kit/lib/spec.sh`** — the deleted count-grammar family (delta 1).
- **`scripts/canon-config.sh`** — the explicit `TASK-QUEUE.md` element (delta 4).
- **TASK-QUEUE.md `cardinal-notation-splits-gate-reach`** — promoted with this ref, and limb
  (c)'s refusal with its ground recorded on the entry so it is not re-proposed (delta 5).
- **queue-kit/SPEC.md** — checked and **not** edited: it states no notation convention and
  delta 5 declines to give it one; recorded so the next reader does not take the silence for an
  omission (delta 5).
- <!-- update-target-exempt: generated projections with their own freshness gates and regen commands, rostered in docs/site-architecture.md §Generated projections and their freshness gates; the hook regenerates only if a coupling edit lands, which delta 6 makes conditional --> the generated `scripts/git-hooks/pre-commit`, `docs/footprint.md` and `docs/value.md`'s rollup block.

## Definition of Done

- [ ] **Causal completeness** — the narrowed producer set's consumers are named with their
      transitions; both widened corpora's readers are enumerated by red condition and **run**
      rather than inspected.
- [ ] **Merged with no information lost** — the ceiling's ground is stated where the ceiling
      is, the residue is named with its refused remedy, and limb (c)'s refusal is recorded on
      the entry rather than in this file.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls canon-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec, config and gate source for the three
      deleted shell symbols; nothing dangles.
- [ ] **Gaps filed** — any violation set delta 2's extension exposes that this cut cannot
      honestly repair is filed as a costed entry, never blanket-exempted.
