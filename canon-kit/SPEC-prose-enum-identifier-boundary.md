# SPEC amendment: prose-enum-identifier-boundary

Queue entry: **`prose-enum-identifier-boundary-class`**.

`check-prose-enum` decides whether a set member is *present* in a paragraph by
requiring that neither abutting character is alphanumeric or a hyphen
(`canon-kit/checks/check-prose-enum.sh:54`, `_sk_present`). That class excludes
the underscore, so an underscore-separated identifier reads as present inside a
longer sibling: a paragraph naming only `guard_allow_match` silently satisfies
the member `guard_allow`.

The failure direction is what makes this a unit rather than a note. A member that
falsely reads as present is a member the gate stops asking about — so a paragraph
that omits it is reported **clean**, with nothing in the output to suggest a
judgment was skipped. It is a false clean live for every consumer that declares a
set of underscore-bearing identifiers, and it is the consumer-facing member of
the vacuous-green set: the green is conditional on a naming convention the kit
has never stated.

**Nothing in this tree is currently wrong**, and that inertness is the reason the
unit exists rather than a reason to skip it. This repo's declared sets are file
basenames and bracketed tags (`scripts/enum-sets.sh`), all hyphenated, so no
member can be a prefix of a sibling across an underscore. The tree cannot red on
this, so no local evidence will ever surface it — the carry grows with adoption,
not with time.

## What changes

### 1. The boundary class admits the underscore {design-bearing}

`_sk_present`'s boundary test widens from `[[:alnum:]-]` to `[[:alnum:]_-]` on
both the preceding and the following character. A member then matches only when
both abutting characters are outside the identifier class — so `guard_allow`
is present in "the `guard_allow` helper" and absent from "the
`guard_allow_match` pair", which is what a reader means by both sentences.

**This is not a new convention; it is the tree's existing one, arriving late.**
The widened class is already what the neighbours use for exactly this job:

| Site | Class | Job |
|---|---|---|
| `gate-sdk/checks/check-readme-roster.sh:62` | `[[:alnum:]_-]` | gate-name token |
| `canon-kit/lib/spec.sh:286` | `[[:alnum:]_-]` | identifier-shaped token |
| `canon-kit/checks/check-prose-enum.sh:54` | `[[:alnum:]-]` | **the outlier** |

**What stays unchanged, and why the widening is exactly one character.** The
class must keep excluding `.` and `/`, and this is load-bearing rather than
incidental: §check-prose-enum specifies that a basename member "matches prose
spelling the file kit-relative, repo-relative or bare — the boundary rule above
accepts the leading slash". Admitting `.` would break `queue.sh` matching inside
`canon-kit/lib/queue.sh`; admitting `/` would break the repo-relative spelling
outright. The underscore is the one character that is part of an identifier and
part of no path convention this gate reads, so it is the whole delta.

`canon-kit/lib/spec.sh:320-321` — the count adapter's own boundary — is
**deliberately not touched**. Its `ac ~ /[[:alnum:]-]/` guards a prose *noun*
against gluing to a following word ("gate" inside "gatekeepers"), not an
identifier against gluing to a sibling. English nouns do not compound across
underscores, so widening it would add nothing and would quietly imply the two
boundaries are one rule. They are two rules that happen to have shared a
spelling; build must not "fix" the sibling for consistency.

**Rejected: make the class a knob.** `CANON_KIT_ENUM_BOUNDARY_CLASS` or similar
is refused. Config-via-env carries a *consumer's* posture — its surfaces, its
vocabularies, its section names — and set declarations already travel that way
through `CANON_KIT_ENUM_SETS_CMD`, which is where the provenance seam actually
bites. The boundary class is matcher mechanism, on the same footing as the valve
and marker spellings §Layout and configuration already calls "mechanism, not
config". Worse, a configurable boundary is a configurable falseness: the one
value a consumer would ever set it to is the narrow one that produces this
amendment's defect, and shipping a knob whose only use is to re-open the bug is
not configuration.

### 2. The battery is re-run for what the widening exposes {mechanical}

Widening the class moves verdicts in **both** directions, and the amendment says
so rather than promising a one-way tightening:

- A member that falsely read as present becomes absent. If the paragraph still
  names two or more members, `missn` rises from zero and a previously clean
  paragraph **reports** — the intended catch.
- If dropping that false match takes the present count below two, the paragraph
  is no longer a hand list and is **skipped**, so a previously reported paragraph
  can go quiet. That one is correct too: the finding rested on a match that was
  never real.

Expected outcome on this tree: **no change at all**, on the hyphenation argument
above. Expected, not assumed — build runs the full battery and dispositions every
delta by §check-prose-enum's stated procedure (cite the set, complete the list,
or site-exempt with reason), and reports a null result as a result. A tree that
reds here would mean a member *does* carry an underscore, which contradicts the
premise this unit was filed on and is a finding worth surfacing on its own.

Mechanical: the battery is the oracle.

### 3. The prefix-sibling case joins the unit test {design-bearing}

Coverage goes to `canon-kit/gate-tests/check-prose-enum.test.sh`, not the fixture
pair, and the split follows what §check-prose-enum already specifies: the pair
covers hand-list *shapes* over the repo's own config, the unit test covers the
**config-driven paths** the pair cannot reach. A prefix-sibling case needs a
declared set containing underscore members, which is a config the pair has no way
to supply — so the unit test is the only place it can live.

The case declares a set holding both `guard_allow` and `guard_allow_match`, over
a paragraph that hand-lists `guard_allow_match` beside a second member and never
names `guard_allow`. It **must fail against the unwidened matcher** and pass
after: under `[[:alnum:]-]`, `guard_allow` reads as present inside its sibling,
the set is complete, and the gate is clean. Build writes the case first, watches
it go green against the *unpatched* gate — a green where the red belongs — and
only then applies delta 1.

That ordering is not ceremony. A test written after the fix would pass, and
nothing would ever have demonstrated that it *could* fail; shipping an assertion
that has never failed is the defect this iteration is named for, and it would be
self-refuting to close a false-clean unit with an unexercised test.

Build also adds the mirror case — the paragraph naming `guard_allow` itself,
asserted clean — so the widening is pinned as a boundary rather than as a blanket
non-match.

### 4. The refusal passage this falsifies is corrected, not re-opened {design-bearing}

canon-kit/SPEC.md §check-prose-enum records two derived-set families that were
measured against this tree and refused, "so neither is re-minted". The
`<kit>-lib-fn` family's refusal rests on two arguments, and **this amendment
destroys the second one**:

> …a lib's functions and its sourcers are a vocabulary prose *discusses*, not a
> set prose *rosters*, so the set-difference is real while the finding is false.
> The function family reported an omission at every passage explaining how two
> helpers interact, none of them a roster, and **it is unsound against the
> matcher besides — the word boundary above excludes `_`, so `guard_allow` reads
> as present inside `guard_allow_match`**…

After delta 1 that sentence is false. The passage is updated so the record does
not carry a stale reason — but the family **stays refused**, and the amendment is
explicit about that because a reader watching one of two arguments fall would
reasonably ask.

The refusal was never a two-of-two conjunction. The roster-versus-vocabulary
argument is dispositive on its own: it is the argument that produced the rule the
passage ends on ("declare a derived set when the tree shape it reads is one the
prose rosters"), and it is backed by a measurement — an omission reported at
*every* passage explaining how two helpers interact. The matcher-unsoundness
clause was the secondary observation, and its real significance was never that it
strengthened the refusal. **It was the sighting that produced this unit** — the
entry records that this defect was surfaced by
`spec-roster-enumeration-derivation` while measuring that very family, and was
out of scope there because that unit's premise forbade a canon-kit change.

So the corrected passage keeps the refusal on its primary ground, drops the
unsoundness clause, and cites the boundary widening as the reason it is gone.
That is a strictly better record than the one it replaces: today the passage
reads as though the family is refused partly because the matcher is broken, which
invites exactly the wrong inference — fix the matcher, re-mint the family. The
correction closes that door while the door is in view.

## Producers and consumers

**The boundary class (existing interface, widened).**
Producer: `_sk_present` inside `canon-kit/checks/check-prose-enum.sh` — a
function-local test, evaluated per candidate match. Its enabling configuration is
`CANON_KIT_ENUM_SETS_CMD`, which is set in the real configuration
(`scripts/enum-sets.sh` in this repo) and defaults empty-to-clean-skip elsewhere,
so the producer is reachable in the deployed posture and not only under test. No
configuration changes for the widening to be live, in this repo or any consumer.
Consumer: `sk_on_pflush` in the same file, the sole caller — it reads the
returned position into `ppos`/`plen` for the adjacency run and into the
present/missing partition for the verdict. Both readers are named and both are
inside this one gate; the class is not exported, sourced, or shared.

**The widened boundary as a consumer contract (new obligation).**
Producer: canon-kit/SPEC.md §check-prose-enum, as the statement a consumer
declaring an enum set reads. Consumer: every consumer's `CANON_KIT_ENUM_SETS_CMD`
emitter — the delta that makes this unit a feature rather than a fix. A consumer
whose set members are underscore-bearing gets **different verdicts** after this
lands, and the honest framing of that is not "a breaking change" but "the
verdicts it was already entitled to". No emitter changes; the grammar stays two
fields; nothing needs migrating.

**No new field, no new state.** Deltas 1 and 4 change an existing test and an
existing prose passage; delta 3 adds test cases. Nothing is emitted, stored, or
passed between components, which is why the causal surface here is one function
and its one caller rather than a cross-component flow.

**Whole-component-set reader survey.** `check-prose-enum` is registered in
`scripts/gates.list`, run by the generated pre-commit hook and `run-gates.sh`,
configured by `scripts/enum-sets.sh`, covered by
`canon-kit/gate-tests/check-prose-enum/{good,bad}/` plus
`canon-kit/gate-tests/check-prose-enum.test.sh`, rostered in canon-kit/README.md,
and specified at canon-kit/SPEC.md §check-prose-enum (mirrored to
`docs/canon-kit/SPEC.md`). `_sk_present` has no caller outside its own file. The
two sibling boundary classes are enumerated in delta 1, with the one that must
not change and the reason. Build re-runs this survey against the tree before
implementing, with **no `2>/dev/null` on any path probe** — a silenced stderr on
a mistyped path reads a live reader as absent, which is the same false-negative
shape this whole unit is about.

## Existing sections updated

- **canon-kit/SPEC.md §check-prose-enum** — the matcher sentence "neither an
  alphanumeric nor a hyphen abutting" becomes "neither an alphanumeric, a hyphen,
  nor an underscore abutting", with the reason stated: an underscore-separated
  identifier must not read as present inside a longer sibling. The sentence
  immediately after it — the one explaining that a stem never matches inside a
  longer tag — extends to the identifier case rather than being restated
  (delta 1).
- **canon-kit/SPEC.md §check-prose-enum** — the same paragraph gains the
  deliberate exclusions: `.` and `/` stay outside the class because the
  repo-relative and kit-relative basename spellings depend on them, which is the
  fact that makes the widening exactly one character wide (delta 1).
- **canon-kit/SPEC.md §check-prose-enum, the refused-families paragraph** — the
  `<kit>-lib-fn` matcher-unsoundness clause is struck, the refusal is restated on
  its roster-versus-vocabulary ground alone, and the passage records that the
  struck clause was the sighting this unit came from (delta 4).
- **canon-kit/SPEC.md §check-prose-enum, the calibration paragraph** — the
  unit-test coverage list gains the prefix-sibling case beside the existing
  config-driven cases it enumerates (delta 3).
- **canon-kit/SPEC.md §lib/spec.sh** — the count adapter's boundary rule gains
  one clause naming it as a *prose-noun* boundary, distinct from the enum
  matcher's identifier boundary, so the two are not re-unified by a later reader
  who notices they once shared a spelling (delta 1).
- **docs/site-architecture.md §Generated projections** — no new gate and no
  `# graph:` manifest change, so the pre-commit hook and the graph artifact are
  untouched. No new `gate-tests/*.test.sh` file either — the cases join an
  existing one — so the derived `canon-kit-gate-test` enum family is unchanged.
  The docs mirror of canon-kit/SPEC.md restales and is regenerated from its
  rostered command.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls canon-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
