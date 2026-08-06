# SPEC amendment: the payload-disclosure claim and its owner

Pairs with the queue entry `payload-disclosure-claim-owner`.

Two governed install surfaces disagree about what a consumer receives.
`installer/README.md` §init claims that what governs your tree afterwards is
**committed source you can read**; gate-sdk/SPEC.md §Consumer payload rules that
a compiled gate's implementation source **does not ship**. Today the first is
true of every tree that exists, which is why the contradiction has cost nothing
yet. This iteration ends that: `native-gate-binary-port` lands the first ported
cohort, and at that commit the §init sentence becomes a false statement rather
than an incomplete one.

## The tense question is settled by this iteration, not by judgment

The entry stayed `[design-pending]` on one open call — the claim's **tense**. It
had to say something true both before and after the first port without reading
as a warning about a thing that had not happened. That framing was correct when
it was filed and is now void: the port is in this iteration's unit set, so the
withheld-source claim becomes true of the tree in the same iteration this
correction lands. **The corrected claim is written in the present, describing the
tree as it will be at the end of this iteration**, and no surface hedges toward a
future.

One sequencing consequence, stated so a batch does not create the window it is
meant to close: if the disclosure correction lands in a build batch *before* the
cohort's descriptors, the prose is briefly ahead of the tree. The correction is
written so that this is harmless — it says what a gate discloses *by rule*, not
how many gates are compiled today — and the Definition of Done binds the two
batches to one iteration rather than to one commit.

## The finding the entry does not carry: the count itself is the defect

The entry names `installer/README.md:55` and records that build batch 1 already
corrected "the sibling claim on `docs/install.md`". Re-verified against the tree
at authoring, the corpus was measured at **four** claims across two documents.
Re-verified again at align, against delta 3's own `all-source` boundary rather
than against the authoring pass's list, it is larger than that and spans a
**third** document:

- `installer/README.md` §What this package is — "What governs your tree
  afterwards is committed, auditable source you read before you run it." —
  missed at authoring: a different section of the same file than §init, so a
  file-level check would have missed it too.
- `installer/README.md` §init — "what governs your tree afterwards is committed
  source you can read."
- `docs/install.md`'s intro, the GitHub-Release-transport bullet — "a
  downloaded, checksummed, extracted tarball you read before running it is the
  most auditable form of the same one-shot vendoring." — missed at authoring.
- `docs/install.md`'s intro, the one-shot-vendoring paragraph — "what governs
  your tree is still committed, auditable source you read before you run it."
- `docs/install.md` §Quick start — "what governs your tree is meant to be
  source you read before you run it," restating the intro's claim inline
  rather than citing it — missed at authoring.
- `docs/install.md` §What a gate discloses — "Today every gate is a shell script
  and you read all of it. **The ruled direction is** that a gate whose
  implementation moves to a compiled binary…" — batch 1's own correction,
  written in the future tense the cohort retires.
- `SECURITY.md` §Threat boundary — "Vendoring is a copy you read before you
  run it," naming the reviewable artifact as vendoring's whole point — missed
  at authoring, in a document neither pass had scoped as an install surface,
  and the sharper instance of the defect for being a security claim.

Plus one site that needs a qualifier rather than a correction: `docs/install.md`'s
H1 preamble ("the kit directories live in your repository as committed source")
names the vendoring model, stays true of it, and is not part of the overclaiming
class — delta 4 still touches it, for precision beside the claims that are wrong.

**The count is not the fact worth landing — its own instability is.** Measured
twice in one iteration, it grew from four to seven against a deliberately
widened search for the same phrasing family, in a document neither pass had
scoped as relevant. A third pass would plausibly find an eighth. This is the
defect class the entry names — *a disclosure claim restated across governed
surfaces with no owner and no oracle* — demonstrating itself against the two
sessions that tried to enumerate it by hand rather than merely being asserted
by them. Fixing the list above by hand and stopping repeats the failure the
entry already names, one level up: the eighth restatement is written by the
next author of a governed doc, and a hand count is exactly the thing with no
mechanism to notice it. What closes the class is `check-payload-claim`
assertion B (delta 2/3), run against the corrected tree — the list above is
retained only as delta 4's build worklist, the sites known at align time, and
is superseded by the gate's own verdict rather than trusted as complete.

## What changes

Each delta carries its `{mechanical | design-bearing}` work class.

### 1. `spec_claim_vocabulary` — the loader generalized {mechanical}

`canon-kit/lib/spec.sh` already carries the exact primitive this needs:
`spec_install_transports` runs a consumer command and validates its
`<id>`⇥`<ERE>` lines, fail-closed (exit 2) on a command failure, an unparseable
line, an empty field, an extra tab, a non-slug id, or a duplicate id.

Generalize it to **`spec_claim_vocabulary <command>`** — the same body, with the
command string taken as an argument instead of read from one named knob — and
re-implement `spec_install_transports` as a one-line caller passing
`$CANON_KIT_INSTALL_TRANSPORTS_CMD`. The documented name keeps its contract and
its `# spec:` pointer, so no caller and no consumer surface moves; what dies is
the second copy the new gate would otherwise mint. Its error messages carry the
caller's name so a fail-closed exit still says which vocabulary failed.

Mechanical: the assertion set is unchanged and the fixture pair that covers it
already exists; executing this is a parameterization and a re-run.

### 2. `check-payload-claim` — a new canon-kit gate {design-bearing}

**Invariant:** exactly one governed doc declares what a gate on the vendored
payload discloses, and no scanned governed doc asserts a different disclosure
class.

- **Declaration.** A full-line `<!-- payload-discloses: <claim-id> -->` HTML
  comment, the same tier-beside-the-prose shape `check-install-claim` uses and
  for the same reason: the reader-facing form of this claim already exists as
  prose and must stay prose. It lands in **gate-sdk/SPEC.md §Consumer payload**,
  which the entry names as the owner and which is the section that rules the
  fact.
- **Vocabulary as consumer config.** `CANON_KIT_PAYLOAD_CLAIMS_CMD`, a command
  emitting one `<claim-id>`⇥`<ERE>` line per disclosure class, default empty ⇒
  clean skip. A spelling of what a payload discloses is one project's
  distribution model, so no literal form of it ships as a kit fact — the same
  provenance-seam ruling `CANON_KIT_INSTALL_TRANSPORTS_CMD` already carries, and
  the reason this gate cannot be written with its patterns baked in.
- **Scanned set.** `check-md-refs`' governed doc set (the manifest set minus
  `CANON_KIT_MDREF_EXCLUDE`) minus `CANON_KIT_PAYLOAD_CLAIM_EXCLUDE`, an array of
  globs defaulting to empty — the identical composition `check-install-claim`
  uses, so a consumer configures one scanned-set idiom rather than two.
- **(A) Singleton owner.** Exactly one declaration across the scanned set. Zero
  is the defect the gate exists for — nothing owns the claim, so an unbounded
  number of surfaces drift with nothing watching, as the align audit's own
  recount just demonstrated — and two owners is that defect wearing a different
  shape. A `<claim-id>` outside the configured vocabulary is **fail-closed (exit
  2)**, not a violation: with no resolvable declared class the gate holds nothing
  to compare against and must not run rather than pass.
- **(B) No contradicting assertion.** Across each scanned document, any line
  matching the ERE of a claim id **other than the declared one** is a violation.
  The declaration line itself is skipped — a claim is not evidence for itself.

**Why this is a sibling gate and not a second assertion pair inside
`check-install-claim`.** The entry proposed extending that gate, and the
extension is refused on two structural differences rather than on taste. Its
assertion B is **positional** — the *earliest* transport match inside a section
whose heading matches `CANON_KIT_INSTALL_SECTION_RE` must be the primary, because
naming a secondary transport is correct prose and must stay green. A disclosure
claim has no such correct-but-secondary form: the non-declared class is wrong
wherever it appears, so the rule is membership over the whole document and not
position inside a selected section. And the two scopes do not coincide — §What a
gate discloses is not an install section under this repo's section regex, while
§init is. Folding a whole-document membership rule into a section-scoped
positional gate would make the gate's name false and give one gate two
unrelated calibrations to reason about on a red.

**Why one claim axis rather than a claim registry.** The generic shape behind
both gates — a declared owner, an id vocabulary, a scanned corpus — invites a
multi-axis registry keyed by claim name. It is refused for now, with the reason
recorded so it is not re-derived: exactly one axis exists, the two gates already
share the only piece worth sharing (delta 1's loader), and a registry would mint
an axis-name knob and an indirection to hold one member. The refusal is cheap to
reverse — a third axis is the trigger, and both gates would collapse into the
registry with their vocabularies unchanged.

**Calibration and the honest limit.** The gate is only as sharp as the emitted
EREs, and a loosely-written pattern wins matches by accident — the same limit
`check-install-claim` states and the same consumer-owned drift, since the kit
contract asks for the emit grammar and never for the patterns. Fenced content is
scanned, because a quoted recipe is exactly where a disclosure claim shows up in
passing. `precommit` tier.

### 3. This repo's disclosure vocabulary {design-bearing}

`scripts/payload-claims.sh`, wired through `CANON_KIT_PAYLOAD_CLAIMS_CMD` in
`scripts/canon-config.sh`, emitting two ids:

- `predicate-withheld` — the declared class. Its ERE recognizes the ruled
  phrasing: a gate's implementation source not shipping, the predicate withheld,
  the binary arriving digest-verified.
- `all-source` — the class the surfaces above assert and which the port
  falsifies. Its ERE recognizes the readable-everything phrasings actually found
  in the tree, narrowly enough that a sentence about the *kit directories*
  vendoring as committed source — which stays true — is not matched.

Design-bearing because the pattern boundary **is** the gate's false-positive
contract: `all-source` must catch "committed source you can read," "source you
read before you run it" (the corpus's most common phrasing, not "you read all of
it," which is `docs/install.md`'s own outlier), and "you read all of it," while
staying silent on "the kit directories live in your repository as committed
source" once that sentence is qualified. Writing those EREs is the judgment this
delta carries, and the `bad/` fixture is the near-miss. **The align audit
measured the snapshot above against a candidate ERE built from this boundary's
own description** (not the final pattern delta 3 will author, which does not
exist yet) — so the count correction above is evidence for this boundary's
shape, not a substitute for writing the real EREs at build time.

`CANON_KIT_PAYLOAD_CLAIM_EXCLUDE` is set to `("docs/posts/*")` — a published
dated post is an immutable artifact and states what was true when it was
published, the same valve `CANON_KIT_INSTALL_CLAIM_EXCLUDE` already carries for
the same corpus and the same reason.

### 4. The prose corrections {design-bearing}

Each surface in the snapshot above is corrected to the declared class, in the
present tense. Named as "the prose corrections" rather than by a count: the
worklist below is delta 4's scope as align could measure it, and it is
`check-payload-claim` (delta 2/3), not this list, that asserts nothing was
missed.

- **`installer/README.md` §What this package is and §init** — both sentences
  keep their shape and stop overclaiming: what governs your tree afterwards is
  committed and auditable — every gate's declaration, its `# spec:` pointer,
  and its `good/`+`bad/` fixture pair — and a gate whose implementation is
  compiled arrives as a digest-verified binary rather than as source. Both cite
  gate-sdk/SPEC.md §Consumer payload, which is where the rule and its bound
  live; neither restates the bound.
- **`docs/install.md`'s intro** — the GitHub-Release-transport bullet and the
  one-shot-vendoring paragraph both keep their subject, which is the vendoring
  model rather than the gate corpus, and both stop asserting that everything
  governing the tree is readable source. The H1 preamble gains the qualifier and
  a link to §What a gate discloses on the same page; it was never part of the
  overclaiming class, only imprecise beside it.
- **`docs/install.md` §Quick start** — the curl-pipe-shell rationale currently
  restates the intro's claim inline rather than citing it, so correcting the
  intro alone would leave this copy false. It is rewritten to point at the
  intro's corrected sentence instead of duplicating it, so a later edit to the
  intro cannot leave this copy stale the way it left this copy uncorrected once
  already.
- **`docs/install.md` §What a gate discloses** — retensed. "Today every gate is a
  shell script and you read all of it" and "the ruled direction is" both go; the
  section states the rule as it holds, names that most gates are shell today and
  that this is a fact about the corpus rather than about the contract, and keeps
  its narrow reason paragraph unchanged.
- **`SECURITY.md` §Threat boundary** — "Vendoring is a copy you read before you
  run it" is corrected to name what is committed and auditable (the same four
  things §Consumer payload ships), not the whole vendored tree. The section's
  actual point — that reviewing the diff at adoption and each upgrade is the
  trust step — survives unchanged for what still ships as source.

Design-bearing: each correction has to be true, has to survive the port count
changing, and must not drift into the confidentiality claim gate-sdk/SPEC.md
§Consumer payload forbids any governed surface from making. The bound is exact —
*raised cost of analysis relative to execution*, never confidentiality — and no
correction here may soften it.

### 5. The new-gate fan-out {mechanical}

A new gate stales six projections, and the roster with no other owner is
docs/site-architecture.md §Generated projections: the on-site SPEC mirror,
`docs/enforcement.md`, `docs/footprint.md`, `docs/value.md`'s rollup block,
`docs/check-graph.html`, and — because this is a `precommit`-tier gate — the
generated hooks. The footprint and the rollup regenerate **after** `git add`,
never before, since an unstaged new file is invisible to the emitter. Beside
those, gate-sdk's kit-landing checklist: the SPEC section, the `good/`+`bad/`
pair, canon-kit's README gate-roster block, and registration in
`scripts/gates.list`.

## Producers and consumers

**`<!-- payload-discloses: <claim-id> -->` (new interface).**
*Producer:* the maintainer editing gate-sdk/SPEC.md §Consumer payload — the
section that already rules the fact, so the declaration sits beside the prose it
is the machine tier of. Reachable and non-test-only: it lands in this delta, in
the live tracked spec.
*Consumer:* `check-payload-claim` assertion A reads it to resolve the declared
class, and assertion B reads that class per line. No other reader; the marker is
an HTML comment, so no rendered surface displays it.
*Verified, because it is the one way this delta could fail silently:* the
declaration must appear **exactly once** in the scanned set, and this repo
mirrors every kit SPEC into `docs/<kit>/SPEC.md`. `CANON_KIT_MANIFEST_FILES` uses
the single-level glob `*/SPEC.md`, which matches `gate-sdk/SPEC.md` and does not
match `docs/gate-sdk/SPEC.md`; the mirror is reachable only through
`docs/*/index.md`, which is a different file. So the mirror copy of the marker is
outside the scanned set and assertion A sees one declaration, not two. Checked
against `scripts/canon-config.sh` rather than assumed — a two-owner red at build
would otherwise be this delta's first symptom.

**`CANON_KIT_PAYLOAD_CLAIMS_CMD` / `CANON_KIT_PAYLOAD_CLAIM_EXCLUDE` (new
config).**
*Producer:* `scripts/canon-config.sh` (delta 3) — this repo's live consumer
config, not a fixture. A consumer that sets neither gets a clean skip, which is
the correct behavior for a tree with no compiled gate.
*Consumer:* `canon-kit/lib/spec.sh` defaults and validates both at source time,
the same place every other `CANON_KIT_*` knob is defaulted;
`check-payload-claim` reads them through that library.
*Every field has a named reader:* an emitted line carries exactly two fields.
The `<claim-id>` is read at assertion A's membership check, at the
declared-versus-other comparison in assertion B, and in the violation report —
which names the offending id, because "wrong disclosure class" alone would leave
the reader grepping. The `<ERE>` is read at assertion B's per-line match and
nowhere else.

**`spec_claim_vocabulary` (generalized interface).**
*Producer:* `canon-kit/lib/spec.sh`, sourced by every canon-kit gate.
*Consumers:* `spec_install_transports` (which becomes its only pre-existing
caller, preserving `check-install-claim`'s contract untouched) and
`check-payload-claim`. No third caller is introduced.

## Existing sections updated

- **canon-kit/SPEC.md §Layout and configuration** — owned by delta 2/3. Adds
  `CANON_KIT_PAYLOAD_CLAIMS_CMD` and `CANON_KIT_PAYLOAD_CLAIM_EXCLUDE` to the
  knob roster with their defaults and this repo's values, beside the
  install-claim knobs and citing the same provenance-seam reason.
- **canon-kit/SPEC.md §lib/spec.sh** — owned by delta 1. Documents
  `spec_claim_vocabulary` and restates `spec_install_transports` as its caller;
  the fail-closed contract moves to the general function and is cited, not
  duplicated.
- **canon-kit/SPEC.md §check-install-claim** — owned by delta 2. One paragraph
  drawing the boundary against the new gate: this one is positional and
  install-section-scoped, its sibling is membership over the whole governed doc
  set, and the two share only the vocabulary loader. Without it the next reader
  meets two claim gates and no statement of which question each answers.
- **canon-kit/SPEC.md — new §check-payload-claim** — owned by delta 2. The gate's
  own contract section, which its `# spec:` pointer binds to.
- **canon-kit/README.md** — owned by delta 2. The `<!-- gate-roster:begin -->`
  block, which `check-readme-roster` holds in both directions.
- **gate-sdk/SPEC.md §Consumer payload** — owned by delta 2. Carries the
  declaration marker, and one sentence naming it as the machine tier of the
  withholding rule this section already states, pointing at
  canon-kit/SPEC.md §check-payload-claim for what binds the two. The ruling
  itself is unchanged; nothing here reopens it.
- **`installer/README.md` (§What this package is, §init), `docs/install.md`
  (its intro, §Quick start, §What a gate discloses), and `SECURITY.md` §Threat
  boundary** — owned by delta 4, the snapshot enumerated above.
  `check-payload-claim` assertion B is the completeness oracle once built, not
  this list.
- **docs/site-architecture.md §Generated projections** — owned by delta 5, only
  in the sense that this gate's landing runs the fan-out that section rosters.
  No new row: `check-payload-claim` emits no projection.

## Cross-component notice

This amendment changes the contracts of **canon-kit** (a new gate, two knobs, a
generalized library function), **gate-sdk** (§Consumer payload gains the
declaration it is the owner of), and **installer** (§What this package is and
§init's claims), and edits the consumer docs surface and this repo's
root-governed `SECURITY.md`. That is a cross-component amendment on
`check-stage-entry` assertion C's own test, so the audit stage is owed before
build entry — see the recommendation the `spec` session records with it, and
see the align stage's own re-measurement above, which is what found the
`SECURITY.md` instance and widened this notice.

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
- [ ] **No governed-doc line matches `all-source` outside its declared form —
      asserted by running the gate, never by a tally.** `check-payload-claim`
      assertion B over the corrected tree is the completeness check; a hand
      count is explicitly refused here because align's own re-measurement grew
      from four to seven in one iteration against the same tree, which is the
      failure mode a checked-box tally cannot see itself fail. The `bad/`
      fixture holds one corrected-to-original sentence verbatim, so a
      reintroduction reds rather than a count going stale.
- [ ] **The confidentiality bound is not softened** — no corrected surface states
      the payload rule as secrecy; the claim stays raised cost of analysis
      relative to execution (gate-sdk/SPEC.md §Consumer payload).
- [ ] **Ships in the same iteration as the cohort** — the correction and
      `native-gate-binary-port`'s descriptors land in one iteration, so no
      release carries a tree the corrected prose is false about.
