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

**The field's value space has two axes, and conflating them is what the present
grammar does.** A walk's file set is bounded by its root, by what it *selects*, and by
how that selection is *matched*. The field today carries only a source (a bare knob
name) and leaves the matching implicit, which is why it is read as a single basename
pattern. It becomes `<kind>:<source>`:

- **`<source>`** — where the selection comes from. `knob:<NAME>`, the value of a kit
  knob, whose ground is §check-reads-couples' own: carrying the name rather than the
  value keeps the value single-sourced and reuses the bridge's resolution path. Or
  `lit:<comma-list>`, a **kit-generic literal** the crate owns at the walk site —
  the form the corpus forced (below).
- **`<kind>`** — how the member's own walker matches that selection: `name:` for a
  literal basename, `glob:` for a path glob, `ext:` for an extension list. **The kind
  is mandatory**, and the reason is measured rather than argued: the reader stands in
  for a walk it cannot see, so the walker's discipline travels on the field or
  nowhere. A kind-blind reader matching every selection as one basename pattern
  produced roughly **nine hundred** findings that a kind-aware reader does not.

And two cases sit outside the field entirely:

- **The field omitted** — the walk is **unfiltered**, which §check-reads-couples
  already rules ("a bare root's enumeration is unfiltered"). The base form, correct
  for a walk that filters nothing.
- **No declared root at all** — the member performs no walk in the analyzed sense.
  `git ls-files` enumeration, single-file reads and a non-recursive directory listing
  (delta 7) are outside that class, so a member doing only those has no root to
  declare, and `?` has been absorbing the case while meaning something else. Already
  practiced: `check-root-tiering` carries the root-argument idiom and correctly
  declares an empty root set.

**The two untagged declarations in the tree are migrated rather than defaulted.**
`check-stage-entry`'s two filter knobs are literal basenames, so they become
`name:knob:…` and nothing about their verdict changes. A *defaulting* kind was
refused: it would hand an author who omits it the basename semantics silently, which
is precisely the nine-hundred-finding wrongness above, and a silent wrong default is
worse than a migration of two sites. No name is retired — every knob name survives
and what changes is the form it sits in (§Retired spellings).

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

- **Knob-bounded**, each naming the knob delta 2 wants — and **a member may need
  several declarations, because a helper may be bounded by several knobs across
  several branches.** `spec::manifest_files` (`spec.rs:199-229`) is the case that
  multiplies: `CANON_KIT_MANIFEST_FILES` globs when non-empty, **else** a kit-literal
  default branch (`canonical_specs` on `CANON_KIT_SPEC_NAME`, plus `find_named` for
  `README.md` and `CLAUDE.md`), **plus** `CANON_KIT_PROSE_SURFACE_GLOBS` additively and
  always. So each manifest-family member needs roughly four declarations under delta 2's
  twice-declared rule rather than one, and `CANON_KIT_PROSE_SURFACE_GLOBS` is a bounding
  knob this measurement omitted. The rule reached it; the figure did not — which is the
  third time a hand figure under-read a rule that was already correct.
  Eight reach `CANON_KIT_MANIFEST_FILES`: six directly through `spec::manifest_files*`
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
  **`CANON_KIT_SPEC_NAME`** through `spec::canonical_specs_sorted`, and
  `check-amendment-queue` on `CANON_KIT_AMENDMENT_GLOB` through `spec::amendments`.
  *(That attribution is corrected from `CANON_KIT_DUP_SURFACES`, which feeds
  single-file reads at `surface_duplication.rs:281-285` and is outside the walk class
  — the mechanical test decides, and the earlier figure was read off the knob array
  rather than off the walk.)*
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

### (5) `canonical_specs` prunes the generated docs mirror

The canonical-spec finder excludes the generated `docs/<kit>/` mirror, so three prose
gates stop grading output no author can fix at the file {design-bearing}.

**Why this is a prerequisite and not a bystander.** It is a latent defect independent
of this amendment — `canonical_specs` filters `templates/` and prunes kit roots but
never excludes `docs/`, so it returns the mirror pages along with the real specs. It
becomes a prerequisite because delta 1's refusal gives these members a declared root
for the first time, and their first live coverage run surfaces it. **The assertion is
correct and its first run lit up a pre-existing defect in a mechanism it asserts
over** — which is neither a coverage gap the declaration failed to name nor something
deferrable beside a banked unit, since a red gate is fixed and never bypassed.

**The reasoning is already this tree's, stated for a sibling knob.**
`scripts/canon-config.sh:97` rules the same exclusion for `CANON_KIT_PROSE_TELL_GLOBS`:
*"the single-level `docs/*.md` glob deliberately excludes the generated kit mirror
(`docs/<kit>/`) and the immutable dated posts (`docs/posts/`), since a prose gate
forcing edits to generated or immutable pages contradicts them"*. A prose gate grading
a generated page is unfixable at the file — the fix is to the source and the
regeneration — so the finding can only be absorbed or ignored. That ground is the
consumer's for its own knob; what this delta does is apply it inside the **kit
mechanism**, where the mirror is excluded because it is generated, not because of any
consumer's editorial scope. No consumer path enters a kit literal: the exclusion is of
the mirror the kit's own emitter writes.

**Measured behaviourally by toggling the prune**, not inferred:
`check-spec-dod-singleton` 22 → 11 spec files, `check-spec-derivable-section` 22 → 11,
`check-spec-embedded-source` 23 → 12 — exactly the eleven mirror pages gone from each.

**Every reader of the narrowed corpus, with its red condition** — this is a corpus
**narrowing**, so causal-completeness point 5 binds and the verdicts are enumerated
rather than cleared by inspection. Four members read `canonical_specs` or
`canonical_specs_sorted`:

- `check-spec-dod-singleton` (`spec_dod_singleton.rs:22`) — **reds on an exact count**,
  `"exactly-one" => n != 1`, and an exact count is one of the three shapes point 5 names
  as non-monotone. It is nevertheless safe here, and the reason is that the count is
  **per spec file**, not over the corpus: dropping a file drops its own check, so the
  violation set can only shrink. Stated explicitly because the shape is the warned one
  and a reader who stops at the shape would block.
- `check-spec-derivable-section` (`spec_derivable_section.rs:37`) — reds on finding a
  banned heading. Monotone.
- `check-spec-embedded-source` (`spec_embedded_source.rs:168`) — reds on an embedded
  source block. Monotone.
- `check-surface-duplication` (`surface_duplication.rs:298`) — reds on a foreign
  definition in a scanned surface. Monotone, and the narrowing is a correctness gain on
  its own axis: the mirror is a byte copy of a kit SPEC, so scanning both made every
  mirrored definition a duplicate by construction.

No reader holds a minimum, a coverage floor, or reds on finding none over this corpus,
which is the other half of point 5's test and the half that would have blocked.

### (6) `comment_surface`'s knob branch honours every arm its default branch does

The configured branch returns early past five narrowing arms, so setting the knob
silently changes which files four gates scan {design-bearing}.

`spec.rs:237-243` returns `glob_files(root, &globs)` directly when
`CANON_KIT_COMMENT_SURFACE` is set, while the default branch below it applies five
further narrowings. The asymmetry means the knob is not a corpus *selector* but a
corpus *replacement* with silently different semantics. The five arms:

1. the `with_templates` / `under_templates` filter (`spec.rs:246`);
2. `prune_kit_roots` (`spec.rs:251`);
3. the byte sort (`spec.rs:255`);
4. **`workflow_tier`** (`spec.rs:256`) — the serious one on its own terms.
   Configuring the knob today silently drops the entire `.workflow/` tracked tier from
   `check-spec-pointer`, whose own `spec:` comment at `spec.rs:260-262` says the
   section requires it;
5. **the walker's own prune set.** The default branch uses `find_files`, which prunes
   `GATE_PRUNE_DIRS`; the knob branch uses `glob_files`, which is bash-faithful and
   prunes nothing.

**Arm 5 is the cause, and the attribution is worth recording because it was wrong
once.** The directory-shape explosion this asymmetry produces was attributed to arm 2,
`prune_kit_roots`; build's measurement corrects it — arm 2 is a **no-op on this tree**
because `CANON_KIT_SCAN_KIT_ROOTS=1`, so it cannot have been the cause. Arm 2 is
still genuinely missing and still fixed here; it simply was not what the symptom came
from. Recorded because a later reader inheriting the first attribution would fix the
wrong arm and see no change.

**The corpus equivalence is verified by the gates' own reports, not reasoned.** With
all five arms honoured, `check-todo-task-liveness` reports **421 governed sources** and
`check-spec-pointer` **3867 directive pointers**, identical with the knob set and
unset. So this delta changes **no gate's corpus on this tree**, and nothing about it is
owed to the operator as a behaviour change.

**The consumer value ships as it is, and the fragility is stated rather than
absorbed.** The value that achieves equivalence is depth-enumerated per extension —
five depths × `sh|gate|rs`, fifteen globs — deliberately **not** `**`, because the
filter matcher supports `**` and the **couples** matcher does not, so a `**` value
would create a demand its own token could not cover. Three things follow:

- It is **consumer config** in `scripts/canon-config.sh`, so the choice is this tree's
  and no kit literal carries it.
- The fragility is real: a depth-bounded enumeration is a maintained copy of "any
  depth", and a source six levels deep escapes it **silently**. The corpus is depth
  2–5, so there is one level of headroom.
- **It is not a new fragility and that is why it ships.** This tree already reasons the
  same way for the same mechanism: `CANON_KIT_MANIFEST_FILES`' own note
  (`scripts/canon-config.sh:21`) records "single-level globs skip the `gate-tests/`
  fixtures the finder pruned". Depth-bounded globbing is an established, reasoned
  practice here, so treating this one value as exceptional would be arbitrary. What is
  owed is a disposition on the **class**, not on the value, and it is filed rather
  than flagged.

**The matcher asymmetry underneath it is routed, not settled here.** That the filter
matcher and the couples matcher disagree on `**` is one more face of
`couples-glob-semantics-unowned`'s open question — *one semantics with stated
exceptions, or a per-reader meaning declared per reader*. It belongs to that entry and
this amendment settles nothing on its behalf.

### (7) The analyzed class's `recursive` discriminator is explicit

`§check-reads-couples`' invariant already says *recursive* walk, and the text must say
what that excludes, because a reader took the boundary for a ruling {design-bearing}.

The invariant reads "every **statically resolvable recursive walk**", and the section's
ground for excluding `git ls-files` is "because it is **not a walk**" — a statement
about mechanism, not about intent. `walk::list_dir` is a single `fs::read_dir` over
immediate children, and its own `spec:` comment says so, so it is outside the analyzed
class by the discriminator already written. This delta makes that explicit rather than
changing it.

**The evidence that the text under-states its own discriminator is that a careful
reader read the exclusion as a boundary *move*.** That is the defect: a discriminator
carried only by one adjective in an invariant and one parenthetical about a different
mechanism is not stated, whatever it entails.

**The discriminator lands in both places or neither**, which is where the tension
actually sat. `list_dir` was in the walk-entry roster the refusal analyzer reads and
in the recorder at `walk.rs:443`, so a member whose only listing is non-recursive
redded the `&[]` that is now its correct declaration. The same test goes in both:
`list_dir` leaves the refusal's root-entry roster and leaves the recorder.

**Red conditions under this narrowing** (point 5). Narrowing the recorder's observed
set narrows assertion A's input, and assertion A reds on **observed roots not a subset
of declared** — monotone, so fewer observations can only remove violations. The
non-monotone risk is the inverse and it is named: two members move to `&[]`
(`check-deferred-board-tags`, `check-workflow-tiering`), and if either performed a
*recursive* walk as well, `&[]` would be a false declaration — which assertion A itself
catches, and does not, across 701 crate tests. Assertion B (no filesystem-walk API
outside `walk.rs`) is untouched: `list_dir` stays in `walk.rs` and stays sanctioned;
only its membership in the recursive class changes.

### (8) A walk's declaration carries its narrowing

A walk has three dimensions and the declaration has forms for two, so the resolver
demands coverage of files the walk provably does not read {design-bearing}.

**The blocker moved rather than shrank, and this is where it landed.** After delta 5
the walk no longer reads the mirror, and thirty-three findings still stand, because the
coverage demand is computed from the **declared filter over the declared root**, never
from the walk: `name:knob:CANON_KIT_SPEC_NAME` resolves to basename `SPEC.md` over root
`.` and selects every tracked `SPEC.md`, the mirror included. The finding text says so
verbatim. This is no longer a seam question and no longer a couples gap — it is the
resolver **over-approximating**.

**Both absorptions are closed, so only precision is live.** §check-reads-couples offers
exactly two: the covering sibling glob, which here means a `docs/` token in a canon-kit
descriptor and is what the seam forecloses; and `# reads-couples-exempt:`, which the
section deliberately withholds from a compiled member — *"a port ends this assertion by
answering it, never by opting out of it"*. Neither is available, and an assertion
shipping with a documented false-positive class and no disposition is the
flagged-and-skipped shape the gap-disposition rule refuses.

**The answer: a walk's root declaration carries its prune set, and the resolver honours
it exactly as it already honours the global one.** A walk is bounded by three things —
where it starts, where it refuses to descend, and which of the files it reaches it
selects. The declaration has a form for the first and, after delta 2, a precise one for
the third. It has none for the second, and the second is not a new dimension: the
resolver **already models it**, because §check-reads-couples rules that a reported root
"is filtered by the prune list exactly as a `gate_find` walk is", with the stated ground
that a substrate honouring less "would scan a different tree than the shell". An
assertion demanding coverage of files its own walk skips is that identical failure. What
is missing is only that the prune set is sourced from two global knobs and a member
cannot declare its own.

**This is genuinely distinct from the fifth filter case delta 2 refuses, and the
distinction is the dimension rather than the syntax.** Delta 2 refused a further
*filter* form — another way of saying which files are selected — on the ground that the
over-demand it was invoked for had an existing absorption. A prune says which subtrees
are never entered. Spelling a prune as a filter-with-exclusion would put two dimensions
in one field and leave the reader unable to tell a selection from a refusal, which is
the same conflation delta 2 just split apart on the kind axis. **The empirical check
settles it:** every narrowing in `canonical_specs` is a directory prune and none is a
file-level exclusion — `under_templates`, `prune_kit_roots`, and delta 5's mirror
exclusion. The form the corpus needs is a prune list, and the corpus says so.

**The seam holds because the resolver learns nothing about canon-kit.** gate-sdk's
resolver must not know that `canonical_specs` prunes `docs/`; the member declares it,
and the resolver honours whatever prune a member declares exactly as it honours
whatever filter a member declares. The kit boundary is not crossed, which is what
reshapes build's coupling objection rather than defeating it.

**The declaration is held to executed behaviour, and that is not optional.** A declared
prune *narrows* the demand, so a member declaring a prune it does not apply would hide
a real coverage gap — the precise failure this gate exists to catch, and the inverse of
the root declaration's risk. The prune therefore rides the same mechanism the roots
do: the sanctioned walk records the prune set it was invoked with, as it already records
the root at `walk.rs:443,496,661`, and assertion A's subset test extends to it. The
recorder is `#[cfg(test)]`-scoped and assertion A is a unit test, so this is the
existing *registry-data-held-to-executed-behaviour* shape and not new machinery. **An
unrecorded prune declaration is refused**: without the recording the form would be a
self-certified narrowing, which is the unbound self-declaration §check-reads-couples
exists to refuse.

**The honest limit, stated rather than discovered.** This form reaches a narrowing
expressible as a directory prune. A helper that narrowed by something else — a content
predicate, a per-file exclusion list — is not covered, and the right answer there is a
new question rather than a stretched prune. None exists in the corpus today; the limit
is recorded so its first instance is recognised as new rather than forced into this
form.

### (9) The filter field's resolution is specified, not left to its one reader

Three resolution facts the field's reader needs are unstated, and each produces a wrong
verdict rather than a refusal {design-bearing}.

The field today is read as a single `-name` pattern against the basename
(`reads_couples.rs:209-214`), which is correct for the two scalar knobs that were its
whole population and wrong for every array-valued path-glob knob delta 2 now puts
through it. Three facts become contract:

- **The value is tab-split and matched the way the member's own walker matches it.**
  The bridge joins a knob's members with a tab, so a fifteen-glob knob arrives as one
  string and matching it whole against a basename selects nothing. This is delta 2's
  *"the interpretation is the walker's, never the field's"* made operational, and the
  kind axis is what carries the discipline.
- **An indexed knob resolves to its elements; a keyed knob resolves to its values.**
  `check-evidence-baseline` is the live instance — a statically resolvable
  `Path::new(".")` root at `evidence_baseline.rs:159` whose filter is
  `EVIDENCE_KIT_SCENARIO_GLOBS`, read through `walk::knob_map`. This is mechanics of
  the existing form rather than a further case, and it does **not** reopen delta 3's
  recorded refusal: `CANON_KIT_EMBED_LANGS` packs `kind|fence-langs|file-globs` triples
  into each *element*, so its pattern is a bespoke projection of a value rather than the
  value, and that walk keeps `?`.
- **"Unresolvable", "no filter" and "resolved empty" are three verdicts, not two.**
  `reads_couples.rs:209` branches on `if !namepat.is_empty()`, so a declared filter
  knob a consumer left empty reads as **unfiltered** and the whole root is demanded —
  the widest possible demand produced by the narrowest possible configuration. An
  absent bridge variable is already a refusal by the bridge's contract; a resolved-empty
  filter must select **nothing**, because that is what the member's own walk does with
  it. Sharing a verdict with "no filter" inverts the demand.

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
- **The pruned mirror** (delta 5).
  - *Producer:* `spec::canonical_specs`, on every invocation — the shipped
    configuration, not a test-only path.
  - *Consumers:* the four members that call it or its sorted spelling, each at its
    per-file assertion loop: `check-spec-dod-singleton`,
    `check-spec-derivable-section`, `check-spec-embedded-source`,
    `check-surface-duplication`.
  - *Red conditions:* enumerated in the delta, because this is a narrowing. One is an
    exact count and is cleared on the ground that the count is per-file.
- **The symmetrical `comment_surface`** (delta 6).
  - *Producer:* `spec::comment_surface`'s configured branch, reached whenever
    `CANON_KIT_COMMENT_SURFACE` is non-empty — which no deployed configuration sets
    today, so the delta's own first act is to set it and prove the corpus unchanged.
  - *Consumers:* `check-comment-tier`, `check-spec-pointer`,
    `check-deprecation-task`, `check-todo-task-liveness`.
  - *Reader of the equivalence claim:* the two gates' own report lines — 421 governed
    sources and 3867 directive pointers — read at the knob-set and knob-unset runs.
    That pair is the delta's oracle and the reason nothing is owed to the operator.
- **The `recursive` discriminator** (delta 7).
  - *Producer:* the discriminator itself, applied in two places that must agree — the
    refusal analyzer's root-entry roster and the recorder in `walk.rs`.
  - *Consumer:* assertion A, at the fixture run, whose observed set narrows.
  - *Readers:* `check-deferred-board-tags` and `check-workflow-tiering`, whose `&[]`
    declarations become correct rather than refused.
- **The declared prune set** (delta 8).
  - *Producer:* the member's own registry root declaration, reported by the `--reads`
    arm alongside the root and filter it already reports.
  - *Consumer:* the coverage assertion's per-root loop, which subtracts the declared
    prune from the root's tracked enumeration before demanding coverage — exactly where
    it already subtracts the global prune set.
  - *Reader that keeps it honest:* assertion A, extended to prunes. The sanctioned walk
    records the prune set it was invoked with, as it records the root, and a declared
    prune may name nothing the walk did not apply. **Without that reader the form is
    refused**, because a self-certified narrowing is the unbound self-declaration the
    section exists to refuse.
  - *Field with no reader:* none. The prune is read at one transition by one assertion
    and held by one test.
- **The specified filter resolution** (delta 9).
  - *Producer:* the bridge, which already joins knob members with a tab and already
    distinguishes an indexed knob from a keyed one.
  - *Consumer:* the filter field's sole reader, the per-root coverage loop.
  - *Readers of the three verdicts:* that same loop. "Unresolvable" is the bridge's
    existing refusal; "no filter" is the omitted field; "resolved empty" selects
    nothing. Each carries a distinct demand, which is the point of separating them.
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
- `canon-kit/SPEC.md` §lib/spec.sh, the `spec_canonical_specs` contract (delta 5). The
  finder's corpus gains the generated-mirror exclusion and the ground for it — a prose
  gate grading generated output is unfixable at the file. Not yet applied.
- `canon-kit/SPEC.md` §lib/spec.sh, the `_spec_comment_surface` contract (delta 6). The
  configured branch must state that it honours every narrowing the default branch
  applies, and the five arms are named so a later reader can check rather than trust.
  Not yet applied.
- `canon-kit/SPEC.md` §check-spec-pointer (delta 6). Its own `spec:` comment already
  says the section requires the `.workflow/` tracked tier; the section gains the
  statement that the tier survives a configured `CANON_KIT_COMMENT_SURFACE`, which
  before this change it did not. Not yet applied.
- `gate-sdk/SPEC.md` §check-reads-couples, the tractable-class paragraph's **`recursive`
  adjective** (delta 7). It gains the mechanism test it already implies: a single-level
  directory listing is not a recursive walk, so it is outside the analyzed class beside
  `git ls-files` enumeration and single-file reads. Listed separately from delta 1's
  unit-of-account target because this one is about the class's *membership* rather than
  its granularity. Not yet applied.
- `gate-sdk/SPEC.md` §check-reads-couples, the declared-root field's grammar (deltas 2,
  8 and 9). It gains the `<kind>:<source>` filter grammar, the declared **prune** as the
  walk's third dimension, the tab-split and indexed-versus-keyed resolution, and the
  three-way unresolvable/no-filter/resolved-empty verdict. Not yet applied.
- `gate-sdk/SPEC.md` §check-reads-couples, *"A reported root is filtered by the prune
  list exactly as a `gate_find` walk is"* (delta 8). The sentence is the precedent delta
  8 extends, and it gains the per-member prune beside the two global knobs, with the
  same stated ground — a substrate honouring less would scan a different tree.
  Not yet applied.
- `gate-sdk/SPEC.md` §Meta-gate conservation for the binary substrate, assertion A
  (deltas 7 and 8). Its observed set narrows by the recursive discriminator and widens
  by the recorded prune set, and both are properties of what it holds honest rather
  than of its own rule. Not yet applied.
- `scripts/canon-config.sh` (delta 6) — the consumer's `CANON_KIT_COMMENT_SURFACE` value
  and the comment carrying its depth-enumeration ground and the stated fragility. A
  consumer surface, so the value is this tree's and the delta states only that it must
  carry its reasoning. Not yet applied.
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

- None — no delta retires a name, and three things read as though they might.
  **`?`** keeps its spelling, its meaning in the `--reads` grammar, and every walk whose
  root is genuinely unresolvable declares it correctly; what delta 1 narrows is its
  *availability for one shape*, which is a predicate rather than a spelling. Declaring
  it would be false in both directions — asserting a removal that did not happen, and
  putting a one-character token through a whole-tree re-grep whose every hit is an
  unrelated use. **The filter field's untagged form** (delta 2) is a *value grammar*,
  not a name: every knob name that sat in it survives, in the same field, under a
  mandatory kind, and the two migrated sites are `check-stage-entry`'s.
  **`list_dir`** (delta 7) keeps its spelling, its home in `walk.rs` and its sanction
  under assertion B; only its membership in the recursive class changes, and a
  membership is not a spelling either.

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
- [ ] **Each prerequisite is proved by its own oracle, not by the battery going
      green** — delta 5 by toggling the prune (22 → 11, 22 → 11, 23 → 12 spec files);
      delta 6 by the two report lines identical knob-set and knob-unset (421 governed
      sources, 3867 directive pointers); delta 7 by assertions A and B passing unchanged
      with two members at `&[]`. A battery that is green because a corpus silently
      shrank is the failure these three oracles exist to exclude.
- [ ] **Delta 8's prune declaration does not land without its recorder** — the prune is
      held to executed behaviour by assertion A or it is not admitted. A declared prune
      narrows the demand, so an unheld one hides exactly the coverage gap this gate
      exists to catch, and it is the one form in this amendment whose failure direction
      is silent.
- [ ] **The residual is read against its own measurement** — 1660 findings before the
      three prerequisites and 33 after, the 33 being delta 8's subject. A residual that
      is not one of those two numbers means a premise moved, and the response is to
      re-measure rather than to widen a couples.
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
