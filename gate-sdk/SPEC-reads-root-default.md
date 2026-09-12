# SPEC amendment: reads-root-default

`check-reads-couples` reports 63 walk roots it left undecided against 2 it
covered, and the largest decidable slice of that 63 is one uniform shape the
registry's own comment already names. This amendment ends `?` for that shape,
and gives the genuine remainder a reviewed cadence instead of a printed counter.

## What changes

### (1) A walk whose root is statically resolvable is not `?`

`?` stops being an available answer for **any walk whose root resolves statically**,
and the obligation is **per walk, not per member** {design-bearing}.

`?` means *a root that cannot be bounded statically*. The predicate is therefore the
**property** — does this walk's root resolve without running the gate — and nothing
narrower. §check-reads-couples already supplies the judgement, about the filter axis:
spelling `?` for a root that is *an argument with a default, where these are literal*
would be "the foreclosed opt-out moved into the registry". This delta applies it on
the root axis, to every statically resolvable root.

**Two shapes satisfy the property today, and they are instances rather than the
definition.** Stating them the other way round is the mistake this delta was drafted
with, and it is worth one paragraph because it cost three successive recounts:

- **A hardcoded literal root** — `spec::manifest_files(".")`. Trivially resolvable,
  and the easier case, which an idiom-shaped predicate misses entirely.
- **A positional argument with a literal default**, which the crate spells at least
  four ways: `…unwrap_or(".")`, `…unwrap_or_else(|| ".".to_string())`, a `match` on
  `args.first().filter(…)` with a `None => "."` arm, and each of those on
  `args.get(<N>)` as well as `args.first()`. The generated hook dispatches these with
  no trailing argv, so the literal is the root at every real invocation.

**The predicate is the property and a sweep over spellings is only ever a floor.**
Three separate hand-written sweeps during this amendment's authoring each missed a
spelling and each produced a different population; one of them found a member only by
accidentally matching an unrelated mode argument. Read the property; treat any
enumerated pattern as a lower bound that a new module can fall outside of without
anyone editing it.

**Per walk, not per member, and this is the rule rather than an aside.** A member may
perform several walks with different root shapes — one statically resolvable and one
not. Such a member declares the resolvable ones and keeps `?` for the rest; the
refusal reds a `?` that stands where a resolvable root could, never a member that
still has an unresolvable walk. `check-spec-embedded-source` is the worked case
(delta 3), and `check-spec-pointer` walks twice for unrelated reasons.

**The extension is a refusal, not a parser, and the substrate ruling is why.** A
`.gate` member "is answered by the substrate, not parsed", so the gate may not
reach into a crate module's source to resolve what the registry declined to
declare. What changes instead is that the substrate may no longer *answer* `?`
where it demonstrably knows better: a crate-side assertion reds a `?` declaration
standing where the walk's root resolves statically. The declaration then carries the
resolved root, and the existing coverage assertion runs unchanged.

**The refusal cannot be partial.** A resolvable walk exempted from it is the
descriptor-level opt-out §check-reads-couples removed and refused to reinstate, moved
one layer down. So the predicate is the property, and every walk satisfying it
declares — which is also why no member, cohort or count appears in the rule.

**Where the deferred entry's premise was corrected, and what survived.** The entry
describes the three shapes `resolve_root` handles and concludes that a root held in
a *variable* falls to skipped. That describes the gate's **shell-text** path, and
that path is dead on this corpus: no gate script remains anywhere in the tree, so
every one of the 63 skips arrives on the registry path instead. The entry's
*substance* is untouched — the decidable subset is still "an argument with a
literal default", which is what the registry's own comment says the `?` population
is — and only the substrate and the idiom's spelling move. Recorded here because a
later reader meeting the entry's shell spelling would extend dead code.

### (2) A declared root names what bounds its walk, and one new form says "a kit literal"

Every walk delta 1 moves off `?` names what bounds it — a knob's value, a kit-generic
literal, nothing because it filters nothing, or no root at all because it performs no
analyzed walk — because a wide root with no filter is an over-demand no couples set can
satisfy {design-bearing}.

A wide root with no filter demands that the member's `couples=` cover everything the
walk reaches, which for a recursive `.` is the whole tree — not a tighter assertion but
an unsatisfiable one, and absorbing it by widening couples to `*` would retire the
trigger the field exists to compute. So the root carries the optional second field
§check-reads-couples already defines — and that field gains one form, because **the
present grammar has no satisfying value for part of the corpus this amendment asserts
over.**

**The field's value space, and the one addition this delta makes.** A walk's file set
is bounded by its root and its filter, and the field is **optional**, so three of the
four cases below already exist — only `lit:` is new:

- **The field omitted** — the walk is **unfiltered**, which §check-reads-couples
  already rules ("a bare root's enumeration is unfiltered"). This is the base form and
  it is already correct for a walk that filters nothing; `check-deferred-board-tags`
  takes it, listing one directory level with no pattern at all.
- **`knob:<NAME>`** — the filter is a knob's value. Today's form, whose bare spelling
  keeps its present meaning unchanged; the tag is admitted beside it so a reader of
  either sibling field meets one vocabulary. Its ground is §check-reads-couples' own:
  carrying the name rather than the value keeps the value single-sourced and reuses
  the resolution path the bridge already owns.
- **`lit:<comma-list>`** — the filter is a **kit-generic literal** the crate owns at
  the walk site. **The only addition**, and the one the corpus forced.
- **No declared root at all** — the member performs no walk in the analyzed sense.
  `git ls-files` enumeration and single-file reads are already outside that class, so a
  member doing only those has no root to declare, and `?` has been absorbing the case
  while meaning something else. Already practiced: `check-root-tiering` carries the
  root-argument idiom and correctly declares an empty root set.

**An unfiltered bare root can over-demand, and the absorption route already exists.**
Declaring root `.` with no filter demands that couples cover every tracked path the
walk reaches — for a depth-one listing, every top-level tracked path. Where that is
right, couples widens to the covering sibling glob, which §check-reads-couples already
names as the correct absorption. Where the demand is genuinely wrong for the walk's
purpose, delta 1's per-walk `?` retention applies and delta 4's cadence row owns it.
Stated so build does not read an over-demand as a reason to mint a fifth case.

**Why the literal form does not breach the refusal it appears to breach.**
§check-reads-couples states that the filter "is carried as a knob name and **never**
as a literal pattern", and states its ground in the same breath: spelling the
pattern in would be "a second spelling of a knob's default, which
de-literalization forbids". That ground is narrow and decisive — **a kit-generic
literal that is nobody's knob has no default to second-spell.** `yml`/`yaml` and
`md` carry no adopter-specific information; minting a knob to hold them would be
the inverse of the error the provenance seam guards, manufacturing consumer config
out of legitimate kit mechanism and handing every adopter a knob whose only correct
value is the one the kit already knows. So this delta **narrows the refusal to its
stated ground** rather than overturning it, and the refusal keeps full force
wherever a knob does exist.

**The interpretation of a literal is the walker's, never the field's.** The crate's
walk entry points already take different filter kinds — extensions for one, globs
for another — so a `lit:` list is read by whichever walker the member's walk uses.
The field therefore mints no third matcher, on the same ground its sibling token
mints none.

**One walk may need two declarations, and that needs no new syntax.** A shared
enumeration helper may take a knob's value when the consumer configured it and a
kit literal when they did not, selecting at runtime — so which form applies is a
property of the member **and the consumer's configuration**, not of the member
alone, and no single static declaration is true of both consumers. Such a member
declares its root **twice**, once per branch, and the coverage demand is their
union. That is the same argument `SPEC-knob-token.md` delta 1 makes for the trigger
— a knob's value and the kit's default corpus are alternatives at runtime, so
couple both branches — and it reuses a mechanism that already ships rather than
minting a union spelling: `check-stage-entry` already declares one root twice under
two filter-knob names.

**The third form is not an opt-out, and an existing assertion is why.** A member
could try to escape coverage by declaring no root while still walking. Assertion A
of the declared-roots pair (§Meta-gate conservation for the binary substrate) runs
every member over its own fixture cases with root recording on and requires
**observed roots to be a subset of declared roots**, so a false no-walk claim reds
there. The outcome is backstopped before this amendment adds anything, which is why
it can be admitted as a form rather than fenced with a new check.

**Footing: this form is not generalised from one instance.** `check-stage-entry` is
the only member with a *declared* resolvable root, which understates the case badly —
**most of the `?` population** already holds the bounding knob in its own source and
declares `?` anyway, and a further cohort holds a knob for one of two runtime branches.
What is missing is permission to say so, not a mechanism — and `check-root-tiering`
already practises the empty-root form, so neither form is new to the registry.

### (3) Every resolvable walk declares, and the couples its declaration forces widen

Each walk delta 1 moves off `?` takes a declaration in delta 2's forms, and its
member's `couples=` widens to whatever the newly-asserted coverage demands
{design-bearing}.

**No count below is load-bearing, and that is a design decision rather than a
hedge.** Every obligation in this amendment is per-walk and keyed on a test — the
root's static resolvability (delta 1), the walk's filter argument (delta 2 and the
sibling's delta 4), the coverage assertion's own verdict (here). Nothing branches on
a cardinality, so an exact maintained population would be a copy of a derivable set,
which is what derivation-first refuses and what already took down the sibling's
roster. The figures below are a **dated measurement at this amendment's authoring**,
kept because build needs the scale and the lead tiers batches off it, and a later
reader must treat a drifted figure as stale measurement rather than as a broken
obligation.

**Two figures come from an oracle and are citable; one is a hand sweep and is a
floor.** `check-reads-couples`' own clean line reads **2 resolvable walk(s) covered,
63 undecidable skipped-and-counted, across 113 gates**, and a `--reads` sweep over
every `scripts/gates.list` member puts those 63 roots across **59 members** — both
oracle-derived. The **decidable subset is a floor of 29 members**, and it is a floor
precisely because arriving at it meant enumerating spellings: three successive hand
sweeps during authoring produced 26, then 27, then 29, each time because another
spelling of one idiom surfaced. Read delta 1's property; treat 29 as a lower bound.

**Classes by the one mechanical test that needs no judgement** — what the walk's
filter argument is, followed one hop through a shared enumeration helper:

- **Knob-bounded**, each naming the knob delta 2 wants. Eight reach
  `CANON_KIT_MANIFEST_FILES`: six directly through `spec::manifest_files*`
  (`check-manifest-count`, `check-manifest-temporal`, `check-knob-citation`,
  `check-prose-enum`, `check-spec-pointer`, `check-tracking-claim`) and two through
  `spec::governed_docs`, which calls it (`check-install-claim`,
  `check-payload-claim`). Two reach `CANON_KIT_AMENDMENT_GLOB` through
  `spec::amendments_strict` (`check-amendment-retired-spelling`,
  `check-amendment-update-target`). Two reach `CANON_KIT_SPEC_NAME` through
  `spec::canonical_specs_sorted` (`check-spec-dod-singleton`,
  `check-spec-derivable-section`). The remaining four bound their walks directly:
  `check-queue-slug-liveness` on `QUEUE_KIT_PROSE_SURFACE_GLOBS`,
  `check-measured-claim` and `check-unmarked-claim` on
  `CANON_KIT_MEASURED_SURFACE_GLOBS`, `check-prose-tells` on
  `CANON_KIT_PROSE_TELL_GLOBS`, `check-surface-duplication` on
  `CANON_KIT_DUP_SURFACES` (zero-hop), and `check-amendment-queue` on
  `CANON_KIT_AMENDMENT_GLOB` through `spec::amendments`.
- **Kit-literal-bounded**, the set with no knob to name and therefore the set that
  made delta 2's literal form necessary: the four `check-action-*` members on
  `["yml","yaml"]` and `check-graph` on `["md"]`.
- **Unfiltered**, taking delta 2's base form: `check-deferred-board-tags`, a
  depth-one `list_dir` with no pattern.
- **Runtime-selected, needing both** — the `spec::comment_surface` cohort
  (`check-comment-tier`, `check-deprecation-task`, `check-spec-pointer`,
  `check-todo-task-liveness`), whose helper globs by `CANON_KIT_COMMENT_SURFACE`
  when a consumer set it and falls back to the kit literal `["sh","gate","rs"]`
  when they did not. These take delta 2's twice-declared form.
- **Named-other, both named rather than left in a residue.**
  `check-workflow-tiering` has a knob-valued **root** (`GATE_SDK_WORKFLOW_DIR`) and
  no filter at all, which is a different axis from every case above.
  `check-spec-embedded-source` walks twice: its spec corpus is knob-bounded by
  `CANON_KIT_SPEC_NAME` and `CANON_KIT_AMENDMENT_GLOB`, while its source-candidate
  corpus is filtered by globs **projected out of** `CANON_KIT_EMBED_LANGS`, a keyed
  knob whose elements are `kind|fence-langs|file-globs` triples. Its knob's *value*
  is therefore not its pattern, so delta 2's `knob:` form would hand the reader the
  triples. **It keeps `?` for that one walk and declares the other two**, which is
  why delta 1's refusal is stated per-walk and not per-member; delta 4's cadence row
  owns the undeclared walk.

**The classes overlap and there is no leftover class.** `check-spec-pointer` is both
knob-bounded through `spec::manifest_files` and runtime-selected through
`spec::comment_surface`, because it walks twice — the worked example for the
twice-declared form, needing three declarations rather than two. Every member the
sweep found sits in a named class above. An earlier draft consigned six of them to a
"rest" it described as having no walk entry point and no enumeration helper; that
description was **false** — all six reach a knob one hop through a shared helper
whose name the draft had guessed rather than derived. Recorded because the false
framing is the more dangerous half: a build session reading it would stop looking.

What is not uniform, and is where the judgement sits, is the consequence: every walk
moved off `?` has its couples coverage asserted **for the first time**, and a member
whose couples under-covers its real scan pattern reds.

**The already-measured shortfall is the `CANON_KIT_MANIFEST_FILES` group**, whose
corpus reaches `TRAJECTORY.md`, `RELEASING.md`, `CONTRIBUTING.md`,
`CODE_OF_CONDUCT.md`, `SECURITY.md`, `ROADMAP.md`, `doctrine-kit/DOCTRINE.md` and the
`docs/` pages while its couples reach none of them. **Eight** members of that group
carry a statically resolvable root and are reached here. **Three more scan the same
corpus and are reached only through delta 1's hardcoded-literal shape** —
`check-docs-cmd`, `check-md-refs` and `check-spec-fence-balance` read
`spec::manifest_files(".")` with a hardcoded root, and take their `args` as a **file
list rather than a root**, so they walk only in the no-argument invocation. An
idiom-shaped predicate would have missed all three and left them
triggered-but-uncovered with no oracle reaching them; delta 1's property-keyed
predicate is what brings them in, and that is the concrete gain from stating the
property rather than the idiom.

**For every other walk the shortfall is unmeasured and cannot be measured before the
refusal is built**, so the first act of this delta is to build the refusal and *read*
the reds, before any couples is touched.

**This is where the sibling amendment becomes load-bearing rather than adjacent.**
The eleven canon-kit reds are satisfiable in exactly one way: their couples must
cover a corpus the consumer configured, and writing the consumer's file names into
a kit descriptor is what the provenance seam refuses. So the only seam-legal fix is
the `knob:` token `SPEC-knob-token.md` mints — the gate this delta
sharpens is what *forces* that token rather than merely permitting it. Neither unit
lands alone: this one without the token reds eleven members with no legal repair,
and the token without this one is a spelling nothing demands.

### (4) The undecidable remainder takes a named cadence, not a printed counter

The roots that stay `?` join the consumer's close-stage audit roster as one
reviewed class with event-keyed due-ness {design-bearing}.

After delta 3, the remainder is every walk whose root is genuinely not statically
resolvable — including the multi-hop case, a root resolved from a knob
whose literal default sits two files away in a kit shell library, decidable in
principle and outside the single-hop shape by design. The skip count is printed on
every clean line and reviewed by nobody on any schedule, which is precisely the
state doctrine-kit/DOCTRINE.md §Enforcement-first's false-positive carve-out
forbids: *"A stated manual duty carries a named cadence, or it is a duty no session
performs: the un-gateable class joins a tracked audit roster reviewed on a
lifecycle hook, with event-keyed due-ness — a named observable event … beats an
iteration counter no surface tracks."*

One row on `.workflow/audit-roster.txt`, in that file's existing one-line format,
read by close's Audit-roster review sub-step. Its `due:` events are **observable
and named**, not a counter: *a new gate joining the registry* (which is what grew
this population from the filing's 37 to today's 63), *a gate's walk root or scan
pattern changing*, and *a port cohort landing*. Its review act is to read the skip
count and the `?` roster, and to judge whether any member has become decidable —
the judgement nothing performs today.

**Two candidates were considered and set aside.** A drift-kit KPI plugin reporting
the count produces a trend, which is the bare counter the carve-out says is
insufficient on its own. Widening this delta's predicate to the multi-hop case
would close more of the remainder, but it requires the resolver to read a second
file in another kit to decide one root, which is a materially heavier extension
than the shape the entry names and would buy its coverage at the cost of a
false-positive surface the single-hop shape has none of.

**One coupling is named rather than depended on.** The deferred entry
`audit-roster-row-carry-unruled` holds an open question about **compaction** of
rows that have already accreted across many closes. It does not reach appending a
new row, which every close already does; this row will meet that question later,
like every other row, and does not wait on it.

## Producers and consumers

- **The refusal of `?` for a literal-default root** (delta 1).
  - *Producer:* a crate-side assertion over the gate modules, run in the
    commit-time battery like its sibling meta-gates — reached in the deployed
    configuration, not only in fixtures, because the registry it reads is the one
    the battery already runs.
  - *Consumer:* the gate author, at the commit that adds or edits a gate module,
    who must declare the root rather than `?`.
  - *Red condition, stated because this predicate is the one that could
    false-fire:* the assertion reds **only** on the conjunction of a `?`
    declaration and a first-argument-with-literal-default walk root in the same
    module. A root that is genuinely dynamic, a module with no walk, and a module
    that already declares its root are each silent.
- **The three-form root declaration** (delta 2).
  - *Producer:* each member's registry tuple, reported by the binary's `--reads`
    arm — the existing producer of the existing field. **No new field is minted**;
    one existing field gains a value kind, and the empty declaration is an existing
    state this delta only gives a meaning.
  - *Consumer:* `check-reads-couples`' per-root coverage assertion, at the per-root
    loop, which already resolves a named knob through the bridge it sources and
    filters the root's tracked enumeration by the resolved value.
  - *Reader of the new `lit:` value kind:* that same loop, at that same transition,
    which is the field's existing sole reader — so the value kind adds no reader. It
    passes the literal where it today passes a resolved knob value, and the
    interpretation is the member's own walker's (extensions for one entry point,
    globs for another), so no matcher is added either.
  - *Reader of the twice-declared root:* the same loop, which already iterates roots
    rather than members and so needs no change to take one root twice — the
    `check-stage-entry` precedent is what proves this rather than an argument.
  - *Reader of the empty declaration:* assertion A of the declared-roots pair, at
    the fixture run, which requires observed roots to be a subset of declared. It is
    the reader that makes the third form honest rather than an opt-out, and it
    exists already.
- **The widened couples** (delta 3).
  - *Producer:* the descriptor edits, whose trigger consequence is emitted by
    `gen-pre-commit.sh`.
  - *Consumer:* the generated `pre-commit` hook's staged-path test, and
    `check-graph`'s couples-to-hook parity assertion.
- **The audit-roster row** (delta 4).
  - *Producer:* this amendment's build, writing the row once.
  - *Consumer:* close's Audit-roster review sub-step, at every close, reading
    `due:` against `last:`.
  - *Reader of each field:* `due:` and `last:` are read by that sub-step and by
    `check-close-surfaces`' roster assertions; the row carries no field those two
    do not read.
- **Red conditions under a narrowing** (causal-completeness point 5). **No delta
  narrows a corpus** — delta 1 removes an available answer, deltas 2 and 3 move
  roots from unanalyzed to analyzed and widen couples, delta 4 adds a row. The
  analyzed set grows and the skipped set shrinks, so the clean line's two counters
  move in the safe direction and no reader's verdict is cleared by inspection on a
  narrowing argument. The one counter-shaped claim to watch is the opposite case,
  and it is named here so build does not assume it: the clean line's
  `{skipped}` figure is **not** monotone across this work, because delta 3's
  declarations move members out of it while new gates move members into it, so a
  later reading of that number is not a verdict on this amendment.

## Existing sections updated

- `gate-sdk/SPEC.md` §check-reads-couples, the paragraph refusing `?` for a root
  that is "an argument with a default, where these are literal" (delta 1). It
  widens from the filter axis to the root axis, and is restated as a **property**
  rather than an idiom — the population is most of the `?` set, which is what makes the
  widening worth stating rather than implying.
- `gate-sdk/SPEC.md` §check-reads-couples, the tractable-class paragraph naming the
  three resolvable shapes and the skipped-and-counted rule (deltas 1 and 4). It
  gains the literal-default root as a resolvable shape, and records that the
  remainder now has a named cadence rather than only an honesty label.
- `gate-sdk/SPEC.md` §check-reads-couples, *"A member resolving to a `.gate` is
  answered by the substrate, not parsed"* (delta 1). The paragraph keeps its rule
  and gains the clause this delta turns on: the substrate answers, and may not
  answer `?` for a root it can bound.
- `gate-sdk/SPEC.md` §check-reads-couples, the skipped-and-counted rule's **unit of
  account** (delta 1). The section must say that a declaration, a refusal and a skip
  are all **per walk, never per member** — a member with one resolvable and one
  unresolvable walk declares the first and keeps `?` for the second. Listed as its own
  target because the clarification is load-bearing and would otherwise survive only as
  an example inside delta 3, which a merge session integrating delta 1 need never read.
  Not yet applied.
- `gate-sdk/SPEC.md` §check-reads-couples, the optional filter field's **base form**
  (delta 2). The section already says a bare root's enumeration is unfiltered; it gains
  the statement that omitting the field is therefore a *positive* declaration of an
  unfiltered walk rather than an absence, which is what makes it answer
  `check-deferred-board-tags` without a further form. Not yet applied.
- `gate-sdk/SPEC.md` §check-reads-couples, the optional root filter-knob field and
  its live-instance paragraph naming `check-stage-entry` (delta 2). The field gains
  the `lit:` form and the `knob:` tag beside its bare spelling, its population stops
  being one member, and the paragraph states the general rule while keeping
  `check-stage-entry` as its first instance.
- `gate-sdk/SPEC.md` §check-reads-couples, *"The filter is carried as a knob name and
  never as a literal pattern, and that is the load-bearing detail"* (delta 2). This
  is the passage delta 2 narrows, and it is narrowed **to its own stated ground** —
  a second spelling of a knob's default — rather than weakened. The paragraph keeps
  that sentence for the case it was written about and gains the case it does not
  reach: a kit-generic literal that is nobody's knob. Not yet applied.
- `gate-sdk/SPEC.md` §check-reads-couples, the no-declared-root case (delta 2). The
  section records that an empty root set means *performs no analyzed walk* and that
  `?` had been absorbing it, citing assertion A as what keeps the empty declaration
  honest. Not yet applied.
- `gate-sdk/SPEC.md` §check-reads-couples, the shell-text tractable class and
  `resolve_root`'s three shapes (delta 1). The section records that this path is
  **dead on a corpus with no gate scripts** and is retained for a vendoring
  consumer that still ships them, so a later reader does not extend it believing it
  governs this tree. Not yet applied.
- `gate-sdk/SPEC.md` §Meta-gate conservation for the binary substrate (delta 1).
  The new assertion joins the roster of crate-side assertions holding registry data
  to executed behaviour, beside the two that already keep `--reads` honest.
- `lifecycle-kit/SPEC.md` §check-close-surfaces, or the close-surface roster
  section owning the Audit-roster review step's assertions (delta 4), only if the
  roster's row count or class set is asserted there. **Not yet applied, and
  conditional:** build checks whether that gate asserts over roster membership
  before editing it, and drops this target if it does not.
- `.claude/commands/close.md`, the Audit-roster review sub-step (delta 4). No text
  change is expected — the step already reads every row — and the target is listed
  so build confirms rather than assumes. Not yet applied.
- `scripts/git-hooks/pre-commit` and `docs/check-graph.html` (deltas 1 and 3) — the
  generated projections, stale on a new gate and on every couples edit. Regenerate
  with `bash gate-sdk/bin/gen-pre-commit.sh --write`, then
  `bash gate-sdk/bin/run-gates.sh --emit graph > docs/check-graph.html`.
- `docs/enforcement.md` and `docs/value.md`'s rollup block (delta 1) — a new
  assertion joins the class registry. Regenerate with
  `bash gate-sdk/bin/run-gates.sh --emit enforcement-map > docs/enforcement.md`,
  then `bash gate-sdk/bin/run-gates.sh --emit value-rollup --write`.
- `docs/gate-sdk/SPEC.md` (all deltas) — the generated on-site mirror; regenerate
  with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`.

## Retired spellings

- None — no delta retires a name, and `?` is the one that reads as though it might.
  It keeps its spelling, its meaning in the `--reads` grammar, and every walk whose
  root is genuinely unresolvable declares it correctly; what delta 1 narrows is its *availability
  for one shape*, which is a predicate rather than a spelling. Declaring it here
  would be false in both directions: it would assert a removal that did not happen,
  and it would put a one-character token through a whole-tree re-grep whose every
  hit is an unrelated use.

## Definition of Done

- [ ] **Causal completeness** — every new state, event and interface has a named,
      reachable producer and a named consumer; delta 2 mints no field and delta 4's
      row carries no field the review step does not read.
- [ ] **The reds are read before the couples are touched** — build lands the
      refusal first, records what each redded walk reds on, and only then
      widens a couples. A couples widened ahead of its red is a guess, and for the
      15 members outside the canon-kit family no measurement exists yet.
- [ ] **The population is re-derived from the property, never inherited as a count** —
      every figure in delta 3 is a dated authoring measurement and no obligation rests
      on one. Build re-derives from delta 1's predicate, and treats the 29-member
      decidable figure as a **floor**: three hand sweeps at authoring produced three
      different numbers, each missing a spelling of one idiom, so a fourth sweep keyed
      on spellings is expected to be wrong too.
- [ ] **The partition is run by the mechanical test, never by reading a module for
      intent** — the discriminator is what the walk's **filter argument** is, followed
      one hop through a shared enumeration helper. Reading a module's knob list for
      whether a knob "looks corpus-bounding" produced three wrong answers during
      authoring, every one of them scoring a satisfiable member unsatisfiable; a
      module may read six classification knobs and bound its walk through a seventh,
      or read none and bound its walk inside a helper.
- [ ] **A member may walk more than once** — the partition is keyed on walks, not on
      members. `check-spec-pointer` is the worked case: it is knob-bounded through
      `spec::manifest_files` *and* runtime-selected through `spec::comment_surface`,
      so it needs more declarations than a one-walk-per-member sweep would give it,
      and a sweep keyed on members drops its second walk silently.
- [ ] **The sibling lands with it** — `SPEC-knob-token.md`'s token is what
      makes the eleven canon-kit reds legally satisfiable. Neither amendment's
      merge leaves the tree green alone.
- [ ] **The cadence row is reviewable on its first close** — `due:` names
      observable events, `last:` is stamped with this iteration, and the row's
      review act is stated on the row rather than only here.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section, not appended; the shell-path paragraph's
      correction lands beside the rule rather than as a footnote.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`). Discharged at the iteration, since a
      sibling amendment is in flight for this component.
- [ ] **Queue transition at the merging build batch, not at close** — the drain
      stage is `validate`, whose entry refuses a non-empty active queue, so the
      batch that merges this amendment moves its entry in the same commit.
      **Judge the terminal move:** if the unresolvable-root remainder leaves the entry's
      corpus deliverable unfinished, the move is a demotion with `[cost:]` and
      `[surface:]` restored from the promoting diff, not a Done move.
      **A demotion states its cause on the entry, and the two causes are not
      interchangeable.** Firing on the remainder delta 4 gave a cadence to is an
      *honest completion* — that remainder was out of scope from the moment it got a
      cadence instead of a fix. Firing because a delta had no satisfying value for
      part of the corpus it asserts over is *an authoring defect absorbed as a cost
      overrun*, and a demotion that does not say so leaves the record showing a
      correctly-sized unit that merely ran long. Name which, in the demoting commit
      and on the entry.
- [ ] **Removals propagated** — `## Retired spellings` above holds, re-run by
      `check-amendment-retired-spelling`.
- [ ] **Gaps filed** — a cross-component gap found during the work is resolved
      that session, not deferred.
