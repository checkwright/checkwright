# SPEC amendment: seam-sweep

**gate-sdk/SPEC.md is stripped of this project's ruling provenance and left stating its rules
undated**, under CLAUDE.md §The provenance seam (never cross it). This is the parent half of a
kit-boundary split: the other ten kit SPECs **and the gate that would enforce the rule** are
`kit-spec-provenance-seam-sweep-remainder`'s, so this amendment ships gate-sdk swept and
ungated, deliberately. A session meeting a stripped gate-sdk/SPEC.md and no gate should read
that as the split working rather than as an omission.

**The unit is operator-directed** (2026-09-05, consult) as this iteration's joined unit beside
the test-harness cut, and split at the gate-sdk boundary by the lead on its own authority
under the permission the directive itself grants. Neither is reopened here.

**The entry's sizing is corrected by this amendment's own census and the correction is
material.** The 157 figure came from `grep -icE 'ruled|operator ruling|TRAJECTORY\.md'`, which
matches ordinary engineering prose — *the rule the flip must not produce*, *reads as a rule* —
far more often than it matches a stamp, while missing every stamp spelled without the word
(*Ratified by the operator, 2026-08-21*). Censused this session across the whole file, the
defensible union of dated authority stamps and TRAJECTORY.md pointers is **≈87-92 sites**,
and the **lines a literal sweep touches is ≈300-400** — roughly twice the entry's figure, not
because the class grew but because the entry's predicate never measured it. The corrected
figures land on the entry in the promoting commit.

## What changes

### (1) The discriminator, which is the amendment's whole design content

Two owner documents appear to disagree about a refused alternative's grounds, and the sweep is
unexecutable until they are reconciled {design-bearing}.

- **CLAUDE.md §The provenance seam** bars "a dated operator stamp, a `TRAJECTORY.md` pointer,
  a refused alternative's grounds" from a kit SPEC, "which states the rule undated".
- **canon-kit/SPEC.md §Merging an amendment, step 2** rules the opposite way about the same
  words: at merge "design rationale relocates into the spec's prose (**its permanent home**)".

**The ruling: they reconcile on *attribution*, and that is the sweep's predicate.** What the
seam bars is a ground carried **as provenance** — attached to an authority, a date, a channel,
or an internal identifier that resolves only in this repo. What the merge rule keeps is the
**engineering** ground, stated undated and impersonally. The same refused alternative can be
written either way, and the seam decides the *voice*, never the *content*:

> *Refused: a per-gate header, because it would cost a retrospective declaration on every
> existing member.* — kept.
>
> *Refused 2026-08-23 by the operator on a consult's recommendation: a per-gate header …* —
> swept to the first form, the stamp moving to TRAJECTORY.md if the ruling is live.

This is why the sweep is not a deletion pass. Nothing that a consumer reading the kit cold can
*use* leaves the file; what leaves is every marker that only this project can resolve. The
entry's own cost line states the same test from the reader's side — "private ruling history as
mechanism, and pointers a consumer cannot follow".

**The corollary that bounds the unit, stated because it is the biggest sizing lever in it.**
Undated narrative that names **no** authority, date, or internal identifier is engineering
prose under the merge rule and **is not swept**. Undated narrative that names an internal
identifier a consumer cannot resolve — a cohort slug, a numbered budget batch, a queue slug,
an iteration name — **is** swept, on the same *pointer a consumer cannot follow* ground as a
TRAJECTORY.md reference. The test is the identifier, not the tense.

### (2) The taxonomy and its per-class disposition

The file's provenance partitions five ways, censused this session {design-bearing}. Each class
takes a disposition and the amendment states it once so the build applies a rule rather than a
judgment per site:

- **(a) Dated authority stamps** — ≈45-50 sites, clustering in §Porting a gate to the binary
  substrate and its cohort subsections, in §gen-pre-commit and in §check-gate-fail-closed.
  *Disposition:* the attribution clause is **deleted in place** and the rule left standing;
  the grounds move only under delta 3's test.
- **(a′) Authority-attributed but undated** — the subclass the entry's predicate could not
  see: *the operator ruled 6-8 members*, *the 14-member group is operator-ruled not a cohort*.
  *Disposition:* identical to (a). The date was never what made it provenance.
- **(b) TRAJECTORY.md pointers** — ≈42 sites, spread thin rather than clustered.
  *Disposition:* **deleted outright** wherever the surrounding sentence stands alone without
  them, which is the common case; where the pointer is the sentence's only support, the
  sentence is rewritten to state its own engineering ground.
- **(c) Dated landing and cohort labels** — ≈48 sites: *went in-crate on 2026-09-03*, *its
  2026-08-31 members*. *Disposition:* **deleted**; delta 4 rules the roster case, which is the
  one the entry singles out.
- **(d) Undated narrative naming this project's internal identifiers** — concentrated in
  §Meta-gate conservation's table (~19 `since <cohort-slug>` citations across its rows) and
  across the numbered budget batches and named cohorts. *Disposition:* **swept under delta 1's
  corollary** — the identifier is replaced by what it denoted (*since the settings cohort* →
  *since the port carried the first `.gate` descriptor*) or the clause is dropped where it
  denoted only a date. Undated narrative naming no identifier is kept.
- **(e) Refused-alternative grounds** — split by delta 1. The **attributed** ones are swept to
  the impersonal voice; the **unattributed** ones are already compliant and are **not
  touched**, which is most of the file's *refused* prose and is the class a blanket sweep
  would destroy.

**The one class deliberately excluded from all five: a dated *measurement*.** §The decisions
this substrate already closed says of its own figures that they are "a dated measurement, not
a live claim … a recount is a step toward re-deciding"; the same shape recurs at the cohort
selection-evidence lines, at the build-timing benchmark, and at the `cargo fmt` divergence
count. A frozen measurement is not a ruling stamp and TRAJECTORY.md's own text treats it as a
separate class. These stay, dates and all, and the merged section says so — because a sweep
that removes a date a later reader is instructed **not** to refresh converts a frozen
attestation into a live-looking claim, which is a defect the sweep would introduce rather than
remove.

### (3) Move versus delete is decided per site, by TRAJECTORY.md's own discharge test

A ground the kit SPEC is the sole home of does not automatically move {design-bearing}. The
test is already written, in TRAJECTORY.md's §The recording rule at its completion-time half:
"**A ruling whose subject is finished is deleted outright.** Not distilled to a line, and not
annotated as finished. Git history holds the obsolete text and the motivation behind it, and
the cost of going there to retrieve it is the accepted cost."

**So the sweep applies that test at each site, and the two answers are common in this file.**
Most of the dated cohort-selection stamps — *the operator ruled 2026-08-11 that the next
cohort is queue-kit*, and its successors — direct an action that has since completed. Under
TRAJECTORY's own rule those are **deleted**, not migrated, and migrating them would fill the
ruling record with finished rulings its own doctrine forbids keeping. A ground **moves** only
when it is still load-bearing for a live rule and has no other home; the census found six such
clusters, of which one is carved out by delta 5 and the rest are the amendment's migration
list.

**Stated as a rule and not as a roster** because a roster of sites is exactly the thing that
rots between spec and build: the build applies delta 1's discriminator and this test to the
file as it finds it, and the amendment's job is to make both decidable without a second
reading of the census.

### (4) §The non-gate arm's cohort labels are retired, and the roster becomes a plain enumeration

The entry names this the one question it must answer to sweep gate-sdk at all, and the answer
is that the labels go {design-bearing}.

**The ruling reaches them on the owner doc's own words, not on precedent.** CLAUDE.md names
the class first — "**and this project's provenance**" — and then three instances of it. A
landing-date cohort label is this project's provenance published as a reading aid, so the class
reaches it; the three-item list is instances, not a closed enumeration. Nothing about a
member's behaviour, contract or caller depends on when it landed.

**Three measured facts make it an easy call rather than a close one.** The labelling is not
systematic: nine foundational members and six bridged `Arm::Run` members carry no date at all,
so the roster is already half undated. One label reads *its 2026-09-04 one* while grouping
**two** members, so it is not even internally accurate. And the roster is cleanly enumerable
without the labels, grouped by owning kit — which is the grouping a reader actually needs,
since it is the grouping every other sentence in the section uses.

**The alternative the entry raised — keeping the labels behind a stated carve-out — is
refused.** A carve-out would have to say why a landing date is mechanism here and provenance
everywhere else, and no such reason exists; and it would oblige every future member to choose
a shape, which is the cost the entry predicted. The three sibling port cuts of this iteration
add their members **undated** for the same reason, so the roster arrives at this sweep already
in the shape the sweep leaves it.

### (5) One section is carved out of this sweep pending an operator ruling, and the carve-out is stated

**§The decisions this substrate already closed is not swept by this amendment**
{design-bearing}, and the reason is a conflict between two recorded decisions rather than a
sizing choice.

TRAJECTORY.md §The closed rulings does not merely permit that section's provenance; it
**positively assigns ownership** of two closed rulings to it — the Rust-versus-Go refusal and
the bash-portability-floor costing — with a stated reason: "recorded at gate-sdk/SPEC.md
§The decisions this substrate already closed, **which owns them because the component that
depends on them must be readable alone**". The gate-sdk section states the same arrangement
from the other side: "The project-wide register of closed rulings is TRAJECTORY.md, which
points here for these two rather than restating them."

**Sweeping that section would reverse a recorded ruling**, which is operator-class however
well-grounded the finding. Escalated at authoring and **ruled `lead, own-authority
2026-09-05`: the carve-out stands.** The ruling's ground is that the default is that a
recorded ruling stands — so declining to reverse is the *answer*, not a deferral of one — and
that nothing in the finding argues for spending the operator's reconsideration.

**What the sweep would have to change is not the section but TRAJECTORY.md's recording rule**
— the same rule the entry's own *how the class arrived* paragraph names as the class's cause.
That is filed as a costed Deferred entry rather than started, under scope-gated intake, and it
wants an operator at the point someone works it because it reaches the provenance seam itself.

**The carve-out is written into the merged section rather than left implicit**, so a later
reader meeting an otherwise-swept file does not read the survivor as a miss: the section keeps
a sentence saying that its two dated rulings are held here **by TRAJECTORY.md's own
assignment** and that the assignment, not this section, is what a future sweep must move
first.

**The consequence that lands on someone else if it is left here: the seam gate must encode
this exemption.** The gate left this window with `kit-spec-provenance-seam-sweep-remainder`,
so a session building it against a swept tree would find one section still carrying dated
authority stamps that the ruling record sanctions — and a gate that goes green in development
and reds on a sanctioned section is worse than no gate. The obligation is therefore recorded
**on the remainder entry, which owns the gate**, and not only here: a carve-out visible only
in the parent's amendment is invisible to the session that builds the enforcement.

### (6) The docs mirror and the payload are the reason this ships without its gate

gate-sdk/SPEC.md has one generated public mirror under `docs/`, and the mirror is a
rostered projection with its own freshness gate and regen command {mechanical}. The sweep
stales it on every delta, so the regen rides the same commit.

**What this unit delivers without the gate, said plainly.** The swept file stops publishing
this project's ruling history to every adopter's vendored copy and to the public site, which
is the whole of the cost line. What it does not deliver is a machine that keeps it swept —
that is `kit-spec-provenance-seam-sweep-remainder`'s, attached there as the last kit's
landing, and the interim exposure is one file's worth of re-accretion risk over one iteration.
**TRAJECTORY.md's directive paragraph does not discharge at this close**: it discharges on
"that unit lands and the gate is green", and the gate left with the remainder.

## Producers and consumers

**No new state, event or interface is introduced.** This unit edits prose on one governed
surface and regenerates one projection from it. The causal-completeness points are answered
about the surfaces whose content this changes.

**The swept text's producer and consumers.** *Producer* — the build session editing
gate-sdk/SPEC.md, applying delta 1's discriminator and delta 3's test. *Consumers* — (a) the
generated docs mirror, produced by its rostered emit arm and held byte-equal by its freshness
gate, whose transition is the pre-commit hook; (b) every adopter's vendored copy, produced by
`scripts/pack-installer.sh` at release and read by a human; (c) a session loading the section
at its own load trigger. All three read the file's text and nothing structured, so no field is
added and none has a reader to name.

**The migrated grounds' producer and consumer.** *Producer* — the same session, writing into
TRAJECTORY.md under its §The recording rule authoring half, which rules that what this file
owns is "what has no other durable home". *Consumer* — a later session reading the ruling
record before reopening a closed question; the transition is that read. Nothing machine-held
reads TRAJECTORY.md's prose, which is why delta 3's per-site test matters: a ground migrated
into a file whose own rule forbids keeping finished rulings has a producer and no consumer.

**This delta set narrows a corpus — the prose a reader can find in gate-sdk/SPEC.md — so
point 5 binds and each reader's RED condition is enumerated rather than its subject.**

- **`check-md-refs`** reds on a reference that resolves to nothing. Deleting a
  `(TRAJECTORY.md §…)` pointer can only remove references, so its verdict is monotone here and
  it is clearable by inspection — *except* where a delta rewrites a sentence and re-points a
  surviving reference, which the build checks by running it.
- **`check-manifest-count`** reds on **finding** a bare cardinal quantifying a governed
  collection. A sweep that replaces *its 2026-09-03 ones* with an enumeration can **introduce**
  a cardinal where none stood, so this reader is not monotone under the *rewrite* half of the
  sweep and must be run, not inspected. Delta 4's plain enumeration is written without a
  count for exactly this reason.
- **`check-measured-claim`** reds when a `measured:` marker's oracle value disagrees with the
  marker, and **fails closed on an unknown key**. The sweep must not delete a marker's bound
  claim out from under it; the frozen-measurement exclusion in delta 2 is what keeps that from
  happening by accident.
- **`check-surface-duplication`** reds on a canonical definition restated outside its
  glossary. Delta 5's carve-out sentence and delta 1's reconciliation both *cite* their owner
  docs rather than restating them, which is what keeps this reader green.
- **`check-prose-enum`** holds a **coverage floor** — it reds when two or more members of a
  declared set are hand-listed in one paragraph with a member omitted. Delta 4 converts a
  labelled grouping into a hand-listed enumeration, which is precisely the shape that gate
  measures, so the enumeration must be **complete** or carry no partial list at all. This is
  the one reader whose red condition the sweep could trip by doing its job correctly, and it
  is named here so build meets it deliberately rather than at the hook.
- **The docs mirror's freshness gate** reds on any byte of drift between the source and the
  projection; it is monotone in nothing and is cleared only by regenerating.

## Existing sections updated

- **gate-sdk/SPEC.md, whole-file** — every site in classes (a), (a′), (b), (c), (d) and
  (e-authority) (deltas 1, 2, 3), with §The non-gate arm's roster specifically (delta 4) and
  §The decisions this substrate already closed carved out and annotated (delta 5).
- **gate-sdk/SPEC.md §Porting a gate to the binary substrate** — the file's densest cluster of
  dated stamps, and the section whose *this paragraph exists because the same resolution was
  reached and lost twice* passage is the worked test of delta 1's corollary: it names no
  identifier, so it stays (deltas 1, 2).
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — its table's
  `since <cohort-slug>` citations, replaced by what each slug denoted (delta 2, class d).
- **TRAJECTORY.md** — the migrated grounds, under delta 3's test; and **not**
  §The closed rulings' assignment paragraph, which delta 5 escalates rather than edits.
- **TASK-QUEUE.md `kit-spec-provenance-seam-sweep`** — the corrected census and predicate (all
  deltas); and **`kit-spec-provenance-seam-sweep-remainder`**, which owns the gate, carrying
  two things this amendment cannot hold for it: the ~89-line sizing marked **unsized** because
  the *predicate itself* is what keeps failing, and the delta 5 carve-out the gate must encode
  (delta 5).
- **TASK-QUEUE.md, a new Deferred entry** for TRAJECTORY.md's recording rule — the class's
  stated cause, filed rather than started under scope-gated intake, operator-class when worked
  (delta 5).
- <!-- update-target-exempt: a generated projection with its own freshness gate and regen command, rostered in docs/site-architecture.md §Generated projections and their freshness gates; delta 6 names it and no delta owns its content, which is derived byte-for-byte from the source this amendment edits --> the `docs/` mirror of gate-sdk/SPEC.md, plus `docs/footprint.md` and `docs/value.md`'s rollup block.

## Definition of Done

- [ ] **Causal completeness** — the narrowed corpus's readers are enumerated by red condition
      rather than by subject, and the non-monotone ones are run rather than inspected.
- [ ] **Merged with no information lost** — every ground still load-bearing for a live rule
      either stands in the section, undated and impersonal, or has been written into
      TRAJECTORY.md; nothing was deleted that TRAJECTORY.md's own discharge test would have
      kept.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls gate-sdk/SPEC-*.md`), discharged at the iteration while the sibling gate-sdk
      amendments are in flight.
- [ ] **Removals propagated** — every surface pointing at a deleted passage re-checked;
      the docs mirror regenerated; `check-md-refs` green.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks,
      and the delta 5 carve-out's disposition recorded wherever the operator rules it.
