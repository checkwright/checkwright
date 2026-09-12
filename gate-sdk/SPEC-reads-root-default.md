# SPEC amendment: reads-root-default

`check-reads-couples` reports 63 walk roots it left undecided against 2 it
covered, and the largest decidable slice of that 63 is one uniform shape the
registry's own comment already names. This amendment ends `?` for that shape,
and gives the genuine remainder a reviewed cadence instead of a printed counter.

## What changes

### (1) A root that is a positional argument with a literal default is not `?`

A declared walk root whose value is the member's own first argument with a literal
default is **statically known**, so `?` stops being an available answer for it
{design-bearing}.

`?` means *a root this member cannot bound statically*. For 26 of the 59 members
reporting one, the root is bounded: the module reads
`args.first().map(String::as_str).unwrap_or(".")` — the crate's spelling of the
shell `${1:-default}` idiom — and the generated hook dispatches every one of them
with no trailing argv, so the literal is the root at every real invocation.
§check-reads-couples already supplies the judgement this delta applies, about the
filter axis: spelling `?` for a root that is *an argument with a default, where
these are literal* would be "the foreclosed opt-out moved into the registry". This
delta says the same thing on the root axis, where the population is 26 members
rather than one.

**The extension is a refusal, not a parser, and the substrate ruling is why.** A
`.gate` member "is answered by the substrate, not parsed", so the gate may not
reach into a crate module's source to resolve what the registry declined to
declare. What changes instead is that the substrate may no longer *answer* `?`
where it demonstrably knows better: a crate-side assertion reds a `?` root
declaration for a module whose walk root is a positional argument with a literal
default. The member then declares the literal, and the existing coverage assertion
runs unchanged.

**The refusal cannot be partial.** A decidable member exempted from it is the
descriptor-level opt-out §check-reads-couples removed and refused to reinstate,
moved one layer down. So the predicate is the shape, and every member carrying the
shape declares.

**Where the deferred entry's premise was corrected, and what survived.** The entry
describes the three shapes `resolve_root` handles and concludes that a root held in
a *variable* falls to skipped. That describes the gate's **shell-text** path, and
that path is dead on this corpus: no gate script remains anywhere in the tree, so
every one of the 63 skips arrives on the registry path instead. The entry's
*substance* is untouched — the decidable subset is still "an argument with a
literal default", which is what the registry's own comment says the `?` population
is — and only the substrate and the idiom's spelling move. Recorded here because a
later reader meeting the entry's shell spelling would extend dead code.

### (2) A declared root names what bounds its walk, in one of three forms

Every member this delta moves off `?` declares what bounds its walk — a knob's
value, a kit-generic literal, or nothing because it performs no walk — because a
bare root is an over-demand no couples set can satisfy {design-bearing}.

A bare `.` root demands that the member's `couples=` cover every tracked file in
the tree. That is not a tighter assertion, it is an unsatisfiable one, and
absorbing it by widening couples to `*` would retire the trigger the field exists
to compute. So the root carries the optional second field §check-reads-couples
already defines — and that field gains one form, because **the present grammar has
no satisfying value for part of the corpus this amendment asserts over.**

**The three forms, and why there are exactly three.** A walk's file set is bounded
by its root and its filter, and a filter has exactly two possible sources:

- **`knob:<NAME>`** — the filter is a knob's value. This is today's form, whose
  bare spelling keeps its present meaning unchanged; the tag is admitted beside it
  so a reader of either sibling field meets one vocabulary. Its ground is
  §check-reads-couples' own: carrying the name rather than the value keeps the
  value single-sourced and reuses the resolution path the bridge already owns.
- **`lit:<comma-list>`** — the filter is a **kit-generic literal** the crate owns
  at the walk site. New, and the form the corpus forced.
- **no declared root at all** — the member performs no walk in the analyzed sense.
  `git ls-files` enumeration and single-file reads are already outside the analyzed
  class, so a member doing only those has no root to declare, and `?` has been
  absorbing that case while meaning something else.

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
the only member with a *declared* resolvable root, which understates the case —
ten further members already hold the knob half in their own source and declare `?`
anyway, and four more hold both halves. The form fits fifteen members today; what
is missing is permission to say so.

### (3) The decidable cohort declares, and the couples its declaration forces widen

The 26 members take their declarations in delta 2's forms, and each member's
`couples=` widens to whatever its newly-asserted coverage demands {design-bearing}.

**The partition is measured, and it is what delta 2's third form exists for.** By
the one mechanical test that needs no judgement — what the walk's filter argument
is, followed one hop through a shared enumeration helper:

- **10 knob-bounded**, each naming the knob delta 2 wants: six through
  `spec::manifest_files*` on `CANON_KIT_MANIFEST_FILES` (`check-manifest-count`,
  `check-manifest-temporal`, `check-knob-citation`, `check-prose-enum`,
  `check-spec-pointer`, `check-tracking-claim`), plus `check-queue-slug-liveness`
  on `QUEUE_KIT_PROSE_SURFACE_GLOBS`, `check-measured-claim` and
  `check-unmarked-claim` on `CANON_KIT_MEASURED_SURFACE_GLOBS`, and
  `check-prose-tells` on `CANON_KIT_PROSE_TELL_GLOBS`.
- **5 kit-literal-bounded**, which is the set with no knob to name and therefore the
  set that made delta 2's literal form necessary: the four `check-action-*` members
  on `["yml","yaml"]` and `check-graph` on `["md"]`.
- **4 runtime-selected, needing both** — the `spec::comment_surface` cohort
  (`check-comment-tier`, `check-deprecation-task`, `check-spec-pointer`,
  `check-todo-task-liveness`), whose helper globs by `CANON_KIT_COMMENT_SURFACE`
  when a consumer set it and falls back to the kit literal `["sh","gate","rs"]`
  when they did not. These take delta 2's twice-declared form.
- **The remainder is neither**, and build partitions it rather than assuming: one
  member filters **after** the walk through a closure over a knob-derived set
  (`check-spec-embedded-source`, over `CANON_KIT_EMBED_LANGS`), one has a
  knob-valued **root** and no filter at all (`check-workflow-tiering`, on
  `GATE_SDK_WORKFLOW_DIR`), and the rest locate no walk entry point and no
  enumeration helper — which most likely puts them in delta 2's third form, since
  `git ls-files` enumeration is already outside the analyzed class.

**The counts overlap by one**: `check-spec-pointer` is both knob-bounded through
`spec::manifest_files` and runtime-selected through `spec::comment_surface`, because
it walks twice. It is the worked example for the twice-declared form and likely
needs three declarations, not two. Named because a sweep keyed on one walk per
member would silently drop its second.

What is not uniform, and is where the judgement sits, is the consequence: each of
the 26 has its couples coverage asserted **for the first time**, and a member whose
couples under-covers its real scan pattern reds. For the eleven canon-kit members
the shortfall is already measured — their corpus knob reaches `TRAJECTORY.md`,
`RELEASING.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`,
`ROADMAP.md`, `doctrine-kit/DOCTRINE.md` and the `docs/` pages, and their couples
reach none of them. **For the rest the shortfall is unmeasured and cannot be
measured before the refusal is built**, so the first act of this delta is to build
the refusal and *read* the reds, before any couples is touched.

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

After delta 3, the remainder is 33 members whose roots are genuinely not bounded by
this delta's predicate — including the multi-hop case, a root resolved from a knob
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
  widens from the filter axis to the root axis, and the 26-member population is
  what makes the widening worth stating rather than implying.
- `gate-sdk/SPEC.md` §check-reads-couples, the tractable-class paragraph naming the
  three resolvable shapes and the skipped-and-counted rule (deltas 1 and 4). It
  gains the literal-default root as a resolvable shape, and records that the
  remainder now has a named cadence rather than only an honesty label.
- `gate-sdk/SPEC.md` §check-reads-couples, *"A member resolving to a `.gate` is
  answered by the substrate, not parsed"* (delta 1). The paragraph keeps its rule
  and gains the clause this delta turns on: the substrate answers, and may not
  answer `?` for a root it can bound.
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
  It keeps its spelling, its meaning in the `--reads` grammar, and a 33-member
  population that declares it correctly; what delta 1 narrows is its *availability
  for one shape*, which is a predicate rather than a spelling. Declaring it here
  would be false in both directions: it would assert a removal that did not happen,
  and it would put a one-character token through a whole-tree re-grep whose every
  hit is an unrelated use.

## Definition of Done

- [ ] **Causal completeness** — every new state, event and interface has a named,
      reachable producer and a named consumer; delta 2 mints no field and delta 4's
      row carries no field the review step does not read.
- [ ] **The reds are read before the couples are touched** — build lands the
      refusal first, records what each of the 26 members reds on, and only then
      widens a couples. A couples widened ahead of its red is a guess, and for the
      15 members outside the canon-kit family no measurement exists yet.
- [ ] **The decidable population is re-measured, not inherited** — the 26-member
      and 63-root figures are this amendment's authoring measurement at
      `af126eb7`; build re-runs the census before sweeping, because a gate added in
      between moves both.
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
      **Judge the terminal move:** if the 33-member remainder leaves the entry's
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
