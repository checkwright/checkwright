# SPEC amendment: check-amendment-update-target

Pairs with `TASK-QUEUE.md` entry **amendment-update-target-coverage**.

## The fork this amendment resolves

The queue entry filed the unit `[design-pending]` on one question, and it is a
real one: **a gate cannot match a grammar the template never specifies.** The
`## Existing sections updated` heading tells an author "each entry names the
delta that owns it", but no surface pins what a delta *is* or how it is named, so
"names the delta" is unmechanizable as written. The entry named two arms —
canon-kit **fixes** the numbering (redding a non-conforming amendment), or the
assertion **weakens** to "cites some bold token defined above", a proxy the entry
itself judged thin enough to fail gate-sdk/SPEC.md §When a gate earns its place.

**Ruled: fix the grammar.** Three grounds, the first measured.

- **The convention is not one convention, so there is nothing to weaken *to*.**
  A sweep of every amendment in this repository's history finds at least three
  live delta grammars: bold-token deltas (`**A1. …{design-bearing}**` through
  `**D2.`), numbered-heading deltas (`### (1) …` through `### (7) …`), and roman
  sub-headings (`### (i) …`) used for non-delta argument sections in the same
  file. A proxy matching "some bold token defined above" would match the bold
  lead-in of *any* paragraph in the second grammar and fire on nothing — the
  trivially-true proxy §When a gate earns its place bars by name. The proxy arm
  is not merely thin; against this corpus it is empty.
- **canon-kit owns the artifact.** The amendment file is canon-kit's own
  template's output. A kit is entitled to specify the form of the artifact it
  ships a skeleton for, and pinning a form here publishes no consumer vocabulary
  — the provenance seam is not in play, because a delta number is structure, not
  rule content.
- **The legacy cost is zero, measured.** No amendment is currently on disk;
  `ls */SPEC-*.md` outside `templates/` is empty but for this iteration's two.
  Both are authored in the pinned grammar, so the gate ships with its own corpus
  already conformant and no migration is owed.

## What changes

### (1) The delta-ID grammar becomes contract

canon-kit pins the form of a delta and the form of a citation to one, in the
SPEC and in the shipped template together. **{design-bearing}**

**A delta** is a `###` heading under `## What changes` of the form
`### (<N>) <title>`, where `<N>` is a positive decimal integer. Delta numbers
begin at 1 and are **sequential and unique** within the amendment — a gap or a
repeat means a delta was split or dropped without its citations moving, which is
the drift this grammar exists to make visible.

**A citation** is the token `delta <N>` or `deltas <N>` (case-insensitive),
optionally followed by further integers separated by commas and/or the word
`and`; a trailing possessive (`delta 3's`) is the same citation. The literal
`all deltas` is a citation to **every** delta the amendment defines — the form a
whole-amendment update target already uses in practice (the generated-mirror
bullet, stale the moment any delta lands), and it is admitted rather than forced
into an enumeration that would drift as deltas are added.

**Why `### (<N>)` and not the bold-token form.** A heading gives a delta an
anchor a reader can link to and a scanner can find with one pass, it survives
reordering under a diff more legibly than a bold lead-in inside a paragraph, and
it is the grammar the two most recent multi-delta amendments already chose. The
bold-token form additionally carried the work-class tag inside the token, which
couples two independent conventions in one string; separating them lets either
change without the other.

**Explicitly not pinned: the work-class tag.** `{mechanical}` /
`{design-bearing}` is the authoring stage's rule, not canon-kit's — its owner is
the roster's `spec` stage template and its one reader is the iteration lead at
batch-cut. This amendment neither specifies it nor asserts it, and that
non-target is stated so a later reader does not read the silence as an omission.

### (2) `check-amendment-update-target` — every update target cites a defined delta

A new canon-kit gate, born native, asserting the pinned grammar and the
citation rule it makes checkable. **{design-bearing}**

**Invariant:** in every amendment on disk, each entry under
`## Existing sections updated` cites at least one delta, and every cited delta is
defined under `## What changes` in the same amendment.

**Three arms and a valve.**

- **A — the grammar.** Red when a `###` heading under `## What changes` does not
  match `(<N>)`, or when the delta numbers are not 1..n unique and sequential.
  This is the arm that makes B and C possible, and it is where "canon-kit fixes
  the numbering" is discharged.
- **B — the uncited target.** Red when a top-level `-` bullet under
  `## Existing sections updated` carries no citation. This is the attested
  failure: an update target no delta claims **reaches build as an orphan a batch
  adopts on its own authority**, which is the template's own words for it.
- **C — the dangling citation.** Red when a citation names an `<N>` no
  `### (<N>)` heading defines. Without C, arms A and B pass on an amendment whose
  targets cite deltas that were renumbered out from under them.
- **Valve** — `<!-- update-target-exempt: <reason> -->` on the bullet's first
  line or the one above, riding the shared exempt-window (§lib/spec.sh), reason
  mandatory.

**Fail-closed (exit 2):** an amendment carrying `## Existing sections updated`
but no `## What changes`, or a file the finder returns and the reader cannot
read. Note the asymmetry with `check-amendment-queue`, whose finder is
**best-effort by design** (an unwalkable scan root yields no amendments rather
than a refusal, because an empty amendment set cannot hide a violation there —
every `[spec:]` ref then dangles and the run reds). That reasoning does **not**
transfer: an empty set here hides every violation silently, because this gate has
no second surface to contradict it. So this gate refuses on an unwalkable root
where its sibling does not, and the divergence is deliberate rather than an
inconsistency to reconcile.

**No new knob, and the reason is not laziness.** The corpus is `spec_amendments`
(§lib/spec.sh) — the same finder `check-amendment-queue` uses, already applying
the `templates/` exclusion that keeps a shipped `SPEC-amendment.md` skeleton from
being read as a live amendment. The two heading names are **kit constants**, not
config: they are canon-kit's own template's headings, and a consumer editing them
has edited canon-kit's artifact rather than configured it. The contrast with
§check-spec-dod-singleton's configurable DoD heading is real and the line is
where the surface is authored — that heading appears in **consumer-authored
canonical specs**, these appear in **copies of a kit-shipped skeleton**. A knob
is available later behind an attested consumer rename; adding one now would be a
knob whose only reader is §check-knob-citation.

**Deliberately not asserted: roster completeness.** The gate cannot check that
the update-target roster names every surface the change obliges a write to —
that is a claim about the world, not about the file, and no scanner reaches it.
The gate asserts the decidable half (every listed target is owned) and the align
stage keeps the other half. Two stronger arms were weighed and refused: requiring
every delta to be cited by some target (false — a delta adding a wholly new
section legitimately touches no existing one), and requiring every path or `§`
reference appearing in a delta body to appear in the roster (a delta body names
many surfaces for context, so this would be high-false-positive, and a gate that
cries wolf trains its readers to bypass it).

**What this gate would and would not have caught**, stated because the entry's
firing is its `bad/` fixture's provenance. The
`ruled-grant-surface-and-launch-chokepoint` iteration dropped a tightened-gates
declaration across all three build batches; a fourth batch repaired it at
validate. Arm B catches the shape where the target was *listed and unowned*; it
does **not** catch a target that was never listed at all. The entry's own
deliverable line is the narrower one — "a canon-kit gate asserting every entry
under that heading cites a delta the same amendment defines" — and that is what
ships. The residue stays with align, whose corpus is the amendment and whose tier
the entry already weighed and kept.

**Criterion 4** (§The port-candidate criteria) **clears**: the corpus is
`spec_amendments`, which reaches no gate declaration path.

### (3) Registration, descriptor, fixtures and the generated fan-out

The kit-landing obligations, each already specified and none re-decided here.
**{mechanical}**

A `check-amendment-update-target.gate` descriptor with its `# graph:` manifest
and one-line `# spec:` pointer; a row in `scripts/gates.list`; a `good/`+`bad/`
fixture pair under `canon-kit/gate-tests/check-amendment-update-target/` — the
`bad/` carrying an uncited target **and** a dangling citation **and** a
non-sequential delta number, so each arm has an executable statement — plus a
sibling `check-amendment-update-target.test.sh` for the fail-closed cases a
one-pair harness cannot spell; the roster block in `canon-kit/README.md`; and
regeneration of every projection a new gate stales, the generated pre-commit hook
included. The four gate-sdk contracts are gate-sdk/SPEC.md §The gate model's.

## Producers and consumers

**The delta grammar (new contract).** Producer: an authoring-stage session
writing an amendment, working from `canon-kit/templates/SPEC-amendment.md`, whose
comment block is where the grammar is stated at the point of use. Consumers:
`check-amendment-update-target` arms A and C at scan time, and the human reader
of `## Existing sections updated`, who follows a citation to the delta that owns
it. The grammar introduces no field and no runtime state — it is a form
constraint on a transition artifact, read once per gate run.

**`check-amendment-update-target` (new gate).** Producer of nothing but a
verdict. Consumer: the committing session through the output contract, on the
generated pre-commit hook, `run-gates.sh` and CI; `run-gate-tests.sh` consumes
its fixture pair. Its input is `spec_amendments`' output — an existing producer
with an existing enabling path, so nothing new must be configured for the gate to
see a live corpus.

**No new field, no new message, no new knob**, so causal-completeness points 1, 2
and 4 are discharged by the two paragraphs above and point 3 by the section list
below. **Point 5 does not bind**: this amendment narrows no corpus. It adds a
reader to `spec_amendments`' existing output and adds no glob, prune or
exclusion anywhere. Stated rather than skipped, because the point-5 trap is that
a narrowing is easy to introduce inadvertently and the first argument reached for
— "a narrower corpus can only remove violations" — is false.

## Existing sections updated

Each names the delta that owns it.

- **canon-kit/SPEC.md §The amendment lifecycle** — the section that currently
  states the bidirectional rule and the causal-completeness contract gains the
  **delta-ID grammar** as contract: what a delta is, that numbering is
  1..n unique and sequential, what a citation looks like including `all deltas`,
  and the ruling that the work-class tag is deliberately outside this grammar
  (delta 1). This is the section that already governs the amendment as an
  artifact, so the form belongs here rather than in a new one.
- **canon-kit/templates/SPEC-amendment.md** — the `## What changes` comment block
  gains the delta-heading form, and the `## Existing sections updated` comment
  block's "Each entry names the delta that owns it" gains the citation grammar
  and the valve. The template is where an author meets the rule, so a rule stated
  only in the SPEC would be a rule most authors never read (delta 1). The
  template is **excluded from the gate's own corpus** by `spec_amendments`'
  `templates/` prune, so the skeleton's illustrative headings cannot red the gate
  that governs its copies.
- **canon-kit/SPEC.md §check-amendment-update-target** — a new section beside
  §check-amendment-queue: the invariant, the three arms, the valve, the
  fail-closed contract **and its stated divergence from its sibling's
  best-effort finder**, the no-knob ruling with the DoD-heading contrast, the
  two refused stronger arms with their grounds, the roster-completeness
  non-assertion, and the criterion-4 verdict (deltas 2, 3).
- **canon-kit/SPEC.md §check-amendment-queue** — its **coverage-limit paragraph**
  is the prior-flow prose this change touches: it states which amendment
  assertions exist and where the guard is silent, and a reader of it must now
  learn that a second gate holds a different axis of the same artifact. It gains
  a pointer, not a restatement (delta 2).
- **canon-kit/SPEC.md §lib/spec.sh** — the finder roster gains
  `check-amendment-update-target` as a second consumer of `spec_amendments`, and
  the note that the two consumers **differ in fail-closed posture** on purpose,
  with the reason, so a later reader does not read the difference as drift to
  reconcile (delta 2).
- **canon-kit/README.md** — the gate roster block gains
  `check-amendment-update-target`; `check-readme-roster` reds a shipped check
  absent from it (delta 3).
- **scripts/gates.list** — the registration row (delta 3).
- **`docs/canon-kit/SPEC.md`** and **`docs/canon-kit/README.md`** — generated
  projections of the two surfaces above, stale the moment either is edited; the
  regen commands and their freshness gates are rostered at
  docs/site-architecture.md §Generated projections and the merge runs them in the
  same commit (all deltas).
- **The generated pre-commit hook** — `scripts/git-hooks/pre-commit` is generated
  and never hand-edited; a new `precommit`-tier gate with a `# graph:` manifest
  restales it (delta 3).
- **`native/`** — a new module plus its registration in the binary's subcommand
  dispatch, and `bash gate-sdk/bin/build-native.sh` as its own commit-time step
  which `cargo test` does not discharge (delta 2).
- **`lifecycle-kit/templates/stages/align.md`** — no change is owed, and it is
  recorded so the merge does not go looking. Align's manual verification of the
  update-target roster is the **completeness** half this gate deliberately does
  not take; its duty is unchanged and stating it as reduced would be false
  (delta 2).

## What the build owes beyond the deltas

- **This amendment and its sibling are both authored in the pinned grammar**, so
  the gate's first live corpus is the pair merging alongside it. A build session
  that renumbers a delta in either file must move its citations with it — which
  is arm C doing its job on its own author.
- **The `bad/` fixture is the attested miss, not an invention.** Its uncited-target
  case is the `ruled-grant-surface-and-launch-chokepoint` shape; its dangling-citation
  and non-sequential cases are arms C and A, which have no attestation and are
  built proactively on the cheap-and-low-FP ground §When a gate earns its place
  states.
- **Nothing is owed to the gap inbox from here.** The one adjacent question —
  whether the roster's completeness could be reached mechanically — is *ruled*
  in delta 2 with the two refused arms and their grounds, rather than deferred.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls canon-kit/SPEC-*.md`) — discharged at the iteration rather
      than at the commit, since a sibling amendment is in flight for this same
      component.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
