# SPEC amendment: lib-parity

`queue-kit/lib/queue.sh` survives the queue-kit port as a live shell library, and
the port left one of its helpers implemented twice with nothing machine-holding
the pair. This amendment designs the thing that holds it, and rules the question
the entry was `[design-pending]` for: **what, exactly, is comparable between two
implementations that do not share a representation.**

It does not restate the queue format (§The queue format), the port procedure
(gate-sdk/SPEC.md §Porting a gate to the binary substrate), or criterion 6's
qualification (gate-sdk/SPEC.md §The port-candidate criteria).

**This is criterion 6's qualification firing in the direction opposite to the
`spec_comment_surface` cohort's**, and the two are authored in the same
iteration so the contrast is stated rather than inferred. There, the primitive's
caller set empties at the port, so the shell form is deleted and the duplication
becomes *absent* — the criterion's strongest form. Here it cannot: seven live
consumers still source this library. So the duplication is real, permanent, and
gets the criterion's other disposition — **machine-held**.

## What the tree actually holds, probed rather than inherited

The queue entry's body is three facts wrong about the state it describes, and
each correction changes the deliverable rather than merely tidying the record.

**One helper is dual-implemented, not two.** `queue_live_slugs`
(`queue-kit/lib/queue.sh:91`) has its counterpart at `native/src/queue.rs:195`
(`live_slugs`), called by `native/src/gates/queue_slug_liveness.rs` and
`native/src/gates/task_conservation.rs`. **`queue_roadmap_entries`
(`lib/queue.sh:116`) has no Rust counterpart anywhere in the crate** — a grep for
the name and for any roadmap-shaped module returns nothing — and §lib/queue.sh
already says so correctly: it is *not* in the split, because its only two
consumers are `bin/roadmap.sh` and `check-roadmap-fresh`, and that gate is held
on shell. The SPEC is right and the entry is wrong; the amendment corrects the
entry and leaves the SPEC's sentence alone.

**A third helper is split and the entry misses it, and it is the one that
matters most.** `queue_done_slugs` (`lib/queue.sh:102`) has a live counterpart at
`native/src/queue.rs:236` (`done_slugs`), called by `task_conservation.rs` — and
the **shell form has zero callers in the entire tree**. It is dead shell code
with a live compiled twin.

**Seven consumers source the library, not five.** The five `bin/` scripts the
entry names — `queue-index.sh:11`, `queue-counts.sh:11`, `queue-edges.sh:11`,
`roadmap.sh:14`, `lesson-sink.sh:11` — plus two still-shell gates the entry does
not: `check-roadmap-fresh.sh:16` and `check-queue-prose-precondition.sh:16`.
Both of those port eventually, and both would then need what this unit builds,
so the census matters to sequencing and not just to accuracy.

**And `QUEUE_DONE_RE` has no `bin/` reader.** The entry claims every one of the
eight derived globals is read directly by at least one `bin/` script's awk. Seven
are: `QUEUE_ACTIVE_RE`, `QUEUE_DEFERRED_RE`, `QUEUE_ICEBOX_RE` and
`QUEUE_LESSONS_RE` by `bin/queue-index.sh`; `QUEUE_TASK_SECTIONS` by
`bin/queue-counts.sh`; `QUEUE_TASK_RE` by `bin/queue-edges.sh`;
`QUEUE_SECTION_RE` by three of them. `QUEUE_DONE_RE` is read only inside
`queue_done_slugs`'s own awk — which nothing calls.

**No cross-implementation harness exists anywhere in the tree.** That part of the
entry is accurate and was verified rather than assumed:
`check-gate-substrate-parity` carries six assertions and every one is static —
declaration counts, roster agreement, disposition rows, annotation locus,
opacity-by-structure, target-roster ownership. Not one executes a rule and
compares output. `run-gate-tests.sh` runs *whichever single substrate a member
dispatches to*, never both. The deliverable does not exist and is not partly
built.

## What changes

**(1) The dead shell twin is deleted rather than held in parity.** [design-bearing]
`queue_done_slugs` has no caller, is named in no SPEC section, and its compiled
counterpart is live and tested. Buying a standing parity obligation for a
function nothing calls is gating a duplication that removal disposes of, and
enforcement-first ranks removing the duplication above gating it.

So the first thing this unit does is make its own subject smaller: delete the
shell function and `QUEUE_DONE_RE` with it, since the global's only reader is
that function's awk. What remains under the parity obligation is **one helper and
seven globals**, and that is the number the harness is built for.

The cost is stated rather than banked: this removes a shell function a consumer's
own tool could have called. It is undocumented kit surface — no SPEC section
names it — which is what separates it from the section-regex globals below, every
one of which is documented and load-bearing and none of which is touched.

**(2) Parity is asserted on behavior over a corpus, never on the derived
literals.** [design-bearing] This is the ruling the entry's `[design-pending]`
tag was held for, and it is not a detail of the harness — it decides whether the
harness is buildable at all.

The two sides do not share a representation. The shell derives eight ERE strings
and one array and exposes them as globals; the crate derives a `Sections` value
(`native/src/queue.rs:17-74`) from the same bridged knobs and exposes free
predicates over it — `is_section_line`, `heading_name`, `is_lessons_line`,
`is_bullet`. Comparing a regex string to a struct is a category error, and a
harness written to compare *values* would either compare nothing or force one
side to grow an accessor whose only reader is the test.

The comparable thing is **classification**: given the same queue file, do the two
sides agree on which lines are section boundaries, which are task bullets, which
is the lessons line, which slugs are live? So the harness feeds one canned corpus
to both and compares bytes of the classification each produces. That is what the
entry's own "golden-value" instinct was reaching for, corrected: there is no
golden *value* here, only a golden *corpus* and two witnesses held to each other.

Two consequences follow and are stated so the build does not re-derive them. The
corpus must exercise every branch each side has — a configured icebox and an
unconfigured one, since the icebox regexes degrade to the empty string rather
than to "every section"; a lessons line; a bare-slug done bullet; a bold lead-in
in each task section; and a bullet in a section that is none of these. And the
comparison is **A against B directly, with no committed expected file**: a
maintained golden is a third copy to drift, and the failure this unit exists to
catch is one side edited without the other, which a direct comparison catches
and a golden catches no better.

**(3) The binary grows a primitive-report arm, and it must be a flag arm rather
than a subcommand.** [design-bearing] The crate's `live_slugs` is reachable today
only *through* a gate, so nothing can ask it a question directly. The harness
needs to, and the shape is fixed by an existing assertion rather than by taste:
`check-gate-substrate-parity` assertion B reds a **subcommand no descriptor
dispatches to**, with the reference-only disposition as its one exception. A new
`queue-parity` subcommand would therefore either red the parity gate or consume a
disposition intended for implementations held ahead of a port — neither of which
is what this is.

It is a **binary-level flag arm**, the shape `--list` already has, and the
per-member `--reads`, `--knobs` and `--needs` arms establish as the crate's
answer whenever something needs to interrogate the binary rather than run a gate.
It prints the classification of delta (2) for a named queue file, one record per
line, and it dispatches no gate.

**The provenance seam holds and is checked rather than assumed.** The arm
discloses queue-kit's own format behavior, which queue-kit/SPEC.md §The queue
format already publishes in full; it carries no consumer vocabulary and no rule
content, and every knob it reads crosses the bridge as a resolved value exactly
as a gate's does. A consumer's section names arrive as data and are printed back
as data, so no spelling of a consumer's configuration becomes a kit literal.

**(4) The harness is a kit scenario runner, and it asserts exactly where a
dispatch exists.** [design-bearing] It lands as
`queue-kit/gate-tests/queue-lib-parity.test.sh`, beside the four runners the kit
already carries (`queue-counts.test.sh`, `queue-edges.test.sh`,
`queue-index.test.sh`, `roadmap.test.sh`), picked up by `run-gate-tests.sh`'s
`*.test.sh` sweep with no harness change. It sources `lib/queue.sh` the way
`queue-counts.test.sh` already does, and reaches the compiled side through the
resolved command rather than a path.

The one design question the runner carries is what it does in a tree with **no
binary** — a consumer on an uncovered platform, where the shell library is
vendored and the artifact is not. It must not fail there, and it must not pass
vacuously either. The predicate is one the tree already owns:
`check-gate-binary-fresh`'s **a declaration is not a dispatch**. The runner
asserts parity where the binary is genuinely dispatched to, and where it is not,
**skips and says so on its clean line** — the same reporting shape the port's
omitted-member roster uses, so a reader can tell *no binary here* from *parity
holds*. A silent skip is the vacuity this whole unit exists to end, arriving one
layer up.

**(5) §lib/queue.sh's split passage is corrected where it stands.** [mechanical]
The passage at that section's port-debt paragraphs is accurate about
`queue_live_slugs` and about `queue_roadmap_entries`, and it names the risk it
files as debt. What changes is its terminal sentence: the risk is no longer
filed-as-debt but machine-held, by the harness deltas (2)–(4) build, and the
section names the harness as the thing that holds it. `queue_done_slugs` and
`QUEUE_DONE_RE` come out of the library in delta (1), so nothing in the section
describes them.

No correction is appended beside the old sentence and nothing is marked
superseded: two readings of one fact is the defect whichever wears the label
*current*.

**(6) The fuller close stays out of scope, and the boundary is restated because
this is where it would erode.** [mechanical] Collapsing the five `bin/` scripts
onto the same Rust core through a thin subcommand was explicitly outside the
queue-kit cohort's scope and stays outside this unit's. It is the *removal* this
unit's parity harness is the *gate* for, and enforcement-first would ordinarily
rank it first — the reason it does not is that the collapse is a port of five
tools rather than a test, sized nowhere, and taking it here would silently widen
a unit the operator sized as a harness. The harness makes it safe to defer, which
is the honest form of the argument.

**(7) `queue-lib-dual-implementation-parity` moves to `## Done`.** [mechanical]
Its deliverable is the harness and the ruling, both complete at merge; it is not
a corpus entry and carries no roadmap tag, so the terminal move is a Done move
rather than a demotion.

## Producers and consumers

**The binary's primitive-report arm — producer.** Added to the crate's
argument dispatch beside `--list`, produced by the commit that lands the harness.
Its **one named consumer is `queue-kit/gate-tests/queue-lib-parity.test.sh`**,
which reads its stdout and compares it byte-for-byte against the shell side's.
An introspection arm with no consumer would be dead mechanism; this one has
exactly one, in the same commit, and if the runner is not landed the arm is not
landed either.

**Each record the arm prints has a named reader and a named transition.** The
records are the classification of delta (2) — a line's section verdict, its
bullet verdict, and the live-slug roster — and the reader is the runner's
comparison, at the single transition where it diffs the two captures. No record
is printed that the comparison does not consume: a field the runner ignores is a
field with no reader and comes out, which is the check run in the direction that
catches the omission rather than in the direction that confirms the design.

**The canned corpus — producer is the fixture, consumer is both
implementations.** It is a committed queue file under the runner's own fixture
directory, plus the consumer config that configures an icebox and a second that
does not, since the unconfigured-icebox branch is the one whose degradation
behavior §lib/queue.sh states and neither side's tests currently drive over a
shared input.

**The shell library — an existing producer losing one function and one global.**
`queue_done_slugs` and `QUEUE_DONE_RE` are removed by delta (1). Their reader
census is the amendment's own: the function has none and the global's only reader
is the function. That census was run tree-wide with stderr unsilenced, because a
path grep whose stderr is discarded reads a bad path as *no reader* and
manufactures exactly this conclusion.

**Seven consumers keep sourcing the library unchanged.** Five `bin/` scripts and
two still-shell gates, none of which reads either removed name, and none of which
this unit modifies. Stated as the enumeration rather than as a count, because the
count is what the entry got wrong.

**The narrowing check, run in the direction that is not monotone.** Delta (1)
removes a function and a global from a sourced library, which narrows nothing any
gate *scans* — no corpus loses a file, no glob tightens. The readers that could
still be affected are the ones whose red condition is a zero or a floor rather
than a finding, and they are enumerated rather than described:
`check-knob-citation` and `check-knob-default-coupling` red on a knob mentioned
in prose without a source, so removing a global's *definition* while a SPEC still
names it flips them red — which is why delta (5) removes the name from the
section in the same commit, and why the verdict is taken from a run rather than
from this sentence. `check-shellcheck` and `check-comment-tier` lose lines and
red only on findings, so both are monotone and clear by inspection.

## Existing sections updated

- **queue-kit/SPEC.md §lib/queue.sh** — owned by deltas (1) and (5). The library
  loses `queue_done_slugs` and `QUEUE_DONE_RE`; the port-debt passage's terminal
  sentence changes from *filed as debt* to *held by the harness*, naming it.
- **queue-kit/SPEC.md §The queue format** — owned by delta (2), one line: the
  parity subject between holders is classification behavior over a corpus, never
  the derived literals, because holders do not share a representation.
- **gate-sdk/SPEC.md §The port-candidate criteria** — owned by the framing above,
  criterion 6's qualification gaining its **machine-held** worked case beside the
  duplication-absent one the same iteration's cohort supplies. Two dispositions,
  one criterion, stated together so neither reads as the general rule.
- **gate-sdk/SPEC.md §check-gate-substrate-parity** — owned by delta (3), one
  line recording that an introspection arm is a **flag** arm because assertion B
  reds a subcommand no descriptor dispatches to. The assertion is unchanged; what
  is added is the design consequence a later reader would otherwise rediscover by
  reddening it.
- **gate-sdk/SPEC.md §run-gate-tests** — owned by delta (4), the scenario
  runner's dispatch-conditional assertion and its declared skip.
- **TASK-QUEUE.md `queue-lib-dual-implementation-parity`** — owned by delta (7),
  the Done move. The entry's three false claims were corrected **at promotion**,
  where they stood, rather than only here: a build session reads the entry beside
  this amendment, and a knowingly-false premise left live for the length of an
  iteration is the defect whichever surface is the more authoritative.

## Definition of Done

- [ ] **Causal completeness** — the report arm has its one named consumer in the
      same commit; every record it prints is consumed by the comparison; the
      removed function and global have a tree-wide reader census run with stderr
      unsilenced.
- [ ] **Merged with no information lost** — the split passage reads as one
      coherent statement of what is held and how, to someone who never saw this
      amendment.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls queue-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for `queue_done_slugs` and
      `QUEUE_DONE_RE`; nothing dangles.
      `bash gate-sdk/bin/run-gate-tests.sh queue-kit/gate-tests` green, and the
      runner's skip branch exercised at least once against a tree with no binary
      so the declared skip is proved rather than assumed.
- [ ] **Gaps filed** — anything found and not fixed routed to the gap inbox with
      its cost, never flagged and skipped.
