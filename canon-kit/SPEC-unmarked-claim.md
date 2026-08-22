# SPEC amendment: check-unmarked-claim

Pairs with `TASK-QUEUE.md` entry **substrate-claim-staleness**.

## What this amendment is and is not

It is two things joined by one mechanism: a **new canon-kit gate**
(`check-unmarked-claim`) that makes a declared class of prose claim carry an
oracle, and the **thirteen-site prose repair** across eleven files that the gate
then holds. The repair is not a separate unit riding along — it is the gate's
`bad/` corpus in the live tree, and landing either half alone leaves the other
without its reason.

It is **not** a rewrite of this project's positioning. The load-bearing half of
every touched sentence survives unchanged: *no gate reads a harness surface, so
the battery runs under any harness, any CI, or none.* Only the clause asserting
what the enforcement core is *written in* is false, and only that clause moves.

It is **not** the whole of §check-measured-claim's stated known limit
("a claim nobody marks is uncaught here"). It closes that limit **inside a
declared class** and nowhere else, which is the only closure a mechanical scanner
can honestly offer.

### The fork this amendment resolves

The queue entry filed the unit `[design-pending]` on a real fork: **(i)** widen
the shipped `measured:` marker discipline onto the rewritten sentences — debt,
minting no name, reaching only a claim that carries a number; or **(ii)** a new
canon-kit sibling over a forbidden-literal roster — feature, reaching a
re-authored claim but structurally missing three of the census's thirteen sites.
The entry judged them complementary rather than exclusive and left the call here.

**Ruled: they are one mechanism, not two units.** Arm (ii)'s roster is the
**trigger** and arm (i)'s marker is one of the trigger's three **remedies**. The
gate does not ban a literal; it declares a **claim class** and asserts that a
sentence falling in it is not unoracled. Three consequences make this strictly
better than either arm alone, and each is checkable:

- It reaches the **inverse** claim. `CONTRIBUTING.md:57-58` overstates in the
  opposite direction ("Gates in this tree dispatch to a compiled subcommand",
  unqualified, against eight members that are still shell). No stale-shell
  literal roster can ever fire on that sentence; a *claim class* covering both
  directions fires on it, because the class is "asserts the enforcement core's
  substrate", not "says the word bash".
- It reaches a **re-authored** claim. A ban is discharged forever the moment the
  phrase is deleted. A class assertion fires again on the next author who states
  the substrate in new words that fall in the class.
- It survives the end of the port. When the shell residue reaches zero the
  oracle's value moves, every marked sentence reds, and the sentences that must
  change are enumerated by the gate rather than remembered.

Arm (i) survives inside arm (ii) rather than being dropped: four of the thirteen
rewritten sites keep a substrate claim *and* carry a marker, which is arm (i)
executed. What is refused is arm (i) **alone**, on the entry's own ground — it
reaches only a claim carrying a number, and the definitional sites carry none.

## What changes

### (1) `check-unmarked-claim` — a declared claim class must carry an oracle

A new canon-kit gate, born native — CLAUDE.md §This repo is governed by its own
kits, with the substrate rule at gate-sdk/SPEC.md §The port-candidate criteria,
no exception class applying here, so a shell form is not authored.
**{design-bearing}**

**Invariant:** on the scanned prose surface, a paragraph matching a
consumer-declared **claim-class** pattern carries a `measured:` marker. The class
this closes is the axis §check-measured-claim names as its own known limit: the
marker is voluntary, so the claims that most need an oracle are exactly the ones
an author never thought to mark. This gate makes marking **pressured inside a
declared class**, which is the same narrowing §check-manifest-count already
performs for the bare-cardinal shape — stated there as what narrows the limit,
and built here.

**Two arms and a valve.**

- **A — an unmarked claim.** Red (exit 1) when a paragraph matches a declared
  class ERE and no `measured:` marker binds it. The finding names the class id,
  the file, and the **physical line where the match starts**, and its `help:`
  line states the three remedies below.
- **B — the vocabulary fails to load.** Exit 2 on a command error, an unparsable
  line, a non-slug id, or a repeated id — inherited unchanged from
  `spec_claim_vocabulary` (§lib/spec.sh), which already names the failing
  vocabulary by its `<label>` in every message.
- **Valve** — `<!-- unmarked-claim-exempt: <reason> -->` on the flagged line or
  the one above, riding the shared exempt-window (§lib/spec.sh). The reason is
  mandatory and a reasonless valve is red, the `comment-tier-exempt:` convention
  §check-prose-tells states.

**The three remedies, and the gate is indifferent between them.** Rewrite the
sentence out of the class (the doctrine's preference — a claim not made cannot go
stale); attach a `measured:` marker binding it to an oracle key; or land a
reasoned valve. Naming all three on the finding is what keeps the gate from
reading as a ban, which it deliberately is not.

**Calibration — matching is whitespace-normalized across lines, and this is
load-bearing rather than a nicety.** The scope census established it by
measurement: a first single-line grep resolved eight of the eleven files and
**failed on three** — `docs/index.md`, `CONTRIBUTING.md` and `installer/README.md`
— because each claim spans a newline. A line-keyed predicate is therefore blind
by construction on a quarter of its own corpus. The ERE is matched against the
paragraph with runs of whitespace collapsed to one space; the finding is reported
at the physical line the match starts on. This is not a new mechanism: it is the
boundary §check-manifest-count already crosses for a total whose cardinal and
noun straddle a prose wrap, reported at the cardinal's physical line, and the
shared count adapter (§lib/spec.sh) is where that normalization already lives.
Matching is **case-insensitive**; a claim is prose, and its first word is
capitalized or not by where the sentence starts.

**A paragraph is the unit, and it is §check-measured-claim's paragraph**, so the
two gates agree on what a marker binds: the block below the marker, ending at a
blank line, a fence, a second marker, or end of file. Fenced blocks are skipped
for the reason that section gives — a fence is grammar being shown, not a claim
being made.

**Coverage-only, and the reason is a constraint rather than a preference.** The
gate asserts that a matched claim carries *a* marker; it does not assert *which*
key. `spec_claim_vocabulary` is a two-field loader that **rejects a line carrying
an extra tab**, so a class cannot declare a required key beside its ERE without a
third field the loader refuses. Adding one would fork the loader every claim gate
shares — the cost §lib/spec.sh explicitly took the shared-loader shape to avoid.
The key is checked by `check-measured-claim` on the very next arm (arm B fails
closed on a key nobody emits, arm A on a value the oracle no longer reports), so
the composition covers it and neither gate duplicates the other.

**Criterion 4** (§The port-candidate criteria) **clears**: the corpus is a pure
glob expansion of the configured surface set, reaching no gate declaration path
in this tree — the same verdict and the same reasoning as §check-prose-tells,
and flippable by the same consumer config.

### (2) `CANON_KIT_CLAIM_CLASSES_CMD` — the one new knob

A consumer command emitting one `<class-id>`⇥`<ERE>` line per declared claim
class, and the whole seam ruling rides on it. **{design-bearing}**

It is loaded through
`spec_claim_vocabulary` (§lib/spec.sh) and carries that loader's fail-closed
contract; default empty ⇒ clean skip, the inactive-by-default posture every
`*_CMD` sibling takes. Its parsed output bridges to the compiled member as
`CANON_KIT_CLAIM_CLASS_IDS` / `CANON_KIT_CLAIM_CLASS_PATTERNS`, index-aligned,
default empty arrays — the identical shape the transport and payload pairs
already use, so the pre-commit generator serializes it with no new case.

**The seam, and it is the family's own rather than a new call.** The scan
machinery, the normalization, the marker lookup and the verdict are **kit
mechanism**. Every class id and every pattern is **consumer config**, because a
kit literal spelling this project's positioning vocabulary would publish it —
the identical ground §check-payload-claim states for its own disclosure classes
and §check-measured-claim for its keys (CLAUDE.md §The provenance seam).

That seam does a second job worth naming, because it is what keeps the gate from
redding its own kit: canon-kit/SPEC.md is itself on the scanned surface, so a kit
SPEC that enumerated this project's substrate phrasings would match its own class
and need a valve to describe itself. Holding the roster in consumer config means
the kit's prose never spells a member.

**No new surface knob.** The gate reads `CANON_KIT_MEASURED_SURFACE_GLOBS` — the
same surface `check-measured-claim` scans, and for that section's stated reason
(the manifest set omits binding shims by silent omission and the prose surface
excludes them on a copy-shape ownership this rule is not covered by). Verified
against the corpus rather than assumed: **all eleven census files already fall
inside this repo's setting of that knob** (`README.md`, `CONTRIBUTING.md`,
`SECURITY.md` directly; `docs/index.md`, `docs/install.md`, `docs/methodology.md`,
`docs/positioning.md` via `docs/*.md`; `gate-sdk/SPEC.md`, `lifecycle-kit/SPEC.md`
via `*/SPEC.md`; `gate-sdk/README.md`, `installer/README.md` via `*/README.md`).
A second surface knob would therefore be a knob with no consumer, and
§check-knob-citation would be the only thing that ever read it. The cost of
sharing is stated rather than hidden: a consumer wanting claim classes over a
*different* surface than its measured claims cannot express that, and the split
is available later behind an attested need.

### (3) `scripts/claim-classes.sh` — this repo's declared class

A new consumer emitter beside `scripts/measured-claims.sh` and
`scripts/payload-claims.sh`, wired in `scripts/canon-config.sh` as
`CANON_KIT_CLAIM_CLASSES_CMD="bash scripts/claim-classes.sh"`.
**{design-bearing}**

It emits **one** class this iteration:

- `gate-substrates` — the pattern covering claims about what the enforcement core
  is *implemented in*, **both directions**: the stale-shell phrasings the census
  found (bare-bash, "small shell script", "written in awk", "a vendored kit is
  bash", "the one language the rest of the tree is written in") **and** the
  inverse compiled phrasing (`CONTRIBUTING.md`'s unqualified "dispatch to a
  compiled subcommand"). One class rather than two, because one oracle settles
  both and a sentence overstating in either direction is the same defect.

**Two roster rulings the census bought, recorded so the build does not re-derive
them.**

- **`every gate` is excluded as a roster member**, measured rather than judged:
  the census found dozens of non-substrate uses of that phrase, so rostering it
  would make the gate cry wolf and train its readers to bypass it — the failure
  §When a gate earns its place names as a defect in the gate. **The site it would
  have caught is not lost, and the repair is general:** roster the *predicate*
  the noisy subject attaches to, not the subject. `gate-sdk/SPEC.md:210`'s claim
  is "all source the library", and *that* phrase is specific to the substrate
  question, so the class reaches the site with none of the noise. Excluding a
  subject and rostering its predicate is the disambiguation move for any noisy
  member, and it is what keeps this exclusion from costing coverage.
- **A bare implementation-tool token is excluded**, and this one *does* cost
  coverage: `lifecycle-kit/SPEC.md:1912` ("an errored `awk` capture") states its
  substrate in a single tool name with no predicate to roster instead. Repaired
  by hand in delta 5 and uncovered afterwards. This is the amendment's honest
  bound; it is stated below rather than discovered at build.

### (4) `scripts/measured-claims.sh` gains `gate-substrates`

A second emitted key beside `ported-gate-members`, computed in the walk that key
already runs. **{mechanical}**

For each registered gate resolved through the
registry, record whether its declaration path is a `.gate` descriptor or a shell
script, and emit the live substrate set joined by `+` — `native+shell` at this
cut, `native` alone once the residue reaches zero.

**Why an extent key rather than a second count.** §check-measured-claim's arm C
fires only when `<value>` is a bare cardinal, so an extent value is arm-C-free —
which is exactly what the definitional and qualitative sites need, since "A gate
is a small program" carries no number to check a cardinal against. The section
already specifies extent claims as covered by arms A and B alone with a
set-valued oracle, and states that such an oracle joins its members with
something other than tab; `+` satisfies that. This is the key that makes the
touched sentences **self-correcting at the end of the port**: the day the shell
residue reaches zero the emitter returns `native`, every sentence marked
`gate-substrates=native+shell` reds, and the gate enumerates the sentences to
rewrite instead of a later reader finding them.

### (5) The thirteen-site repair across eleven files

Every row is a judgment about which half of a sentence is load-bearing, and none
of it is a search-and-replace. **{design-bearing}**

The verdicts, quotes and
witness are `.workflow/survey-record.md`'s 2026-08-22 block and are cited rather
than restated; what follows is the **disposition** each site takes, which the
census did not rule.

The measurement every rewrite is sized on, re-run at this stage and unmoved:
`bash gate-sdk/bin/port-blockers.sh --group` reports **104 scanned, 96 ported, 3
permanently shell, 5 held, 0 takeable**. That is 96 and **eight**, of which only
**three** are permanent — not 98 and six. A rewrite sized on six ships a second
false number, which is why the count appears here once and every marked sentence
takes it from the oracle rather than from this line.

| # | site | disposition |
|---|---|---|
| 1 | `README.md:9` | rewrite out of the class — the substrate clause is dropped, the harness-independence half kept. No marker. |
| 2 | `docs/index.md:62-63` | rewrite out of the class. The stated *cause* is also wrong: harness independence follows from no gate reading a harness surface, never from the implementation language. No marker. |
| 3 | `docs/methodology.md:45-47` | keep the substrate claim, corrected, **marked** `gate-substrates`. This page's job is to say what the enforcing layer is. |
| 4 | `docs/positioning.md:48-51` | tier one keeps its substrate sentence, corrected, **marked** `gate-substrates`. The tiering claim itself is untouched. |
| 5 | `docs/install.md:83` | rewrite the **note half only**. The bullet's backticked head and parenthetical are held to `context-kit/lib/toolfloor.sh`'s `PROBE_SET` by `check-install-toolchain`; the prose after the em-dash is free, so no toolfloor change is owed. |
| 6 | `docs/install.md:92` | as row 5 — note half only; `awk` stays on the floor because the shell residue and the generated hooks still invoke it. |
| 7 | `gate-sdk/SPEC.md:5` | **the kit's definition of its central noun.** A gate is defined substrate-free; the definition must not need a marker, because a definition that carries a measurement is a definition with an expiry date. |
| 8 | `gate-sdk/README.md:10` | the same definition, and it must land **word-for-word compatible** with row 7 — `check-surface-duplication` governs a canonical definition having one home. |
| 9 | `gate-sdk/SPEC.md:210` | rewrite: the library-sourcing parenthetical is true of a shell gate and false of a `.gate` member. Narrowed, not deleted. |
| 10 | `lifecycle-kit/SPEC.md:1912` | rewrite: the fail-closed clause names the failure, not the tool that had it. The gate is ported (`native/src/gates/lifecycle_registration.rs`), so the tool named is not the one that runs. |
| 11 | `SECURITY.md:42,44` | one finding, two lines. Rewrite so the threat statement covers a binary as well as a script, **marked** `gate-substrates`. The compiled-implementation trust story is **already correct** at `SECURITY.md:49-57` (digest-verified, citing gate-sdk/SPEC.md §Consumer payload) and is not touched — the threat model needs no widening, only the substrate wording. |
| 12 | `installer/README.md:26-27` | the installer *is* bash and that stays true. What drops is the claim about the rest of the tree; the reviewability argument survives on the installer's own terms. |
| 13 | `CONTRIBUTING.md:57-58` | **the inverse claim.** Qualified and **marked** `gate-substrates`. The build obligation it states is unchanged and correct; only its unqualified premise is false. |

**`docs/install.md:193-198` survives untouched and is the model.** It already
states 96 correctly under a live `measured:` marker bound to `ported-gate-members`
— the one site the census found right, and the shape rows 3, 4, 11 and 13 adopt.

**Three of the eleven are kit surfaces whose `docs/` mirror is generated** —
`gate-sdk/SPEC.md`, `gate-sdk/README.md` and `lifecycle-kit/SPEC.md`, mirrored at
`docs/gate-sdk/` and `docs/lifecycle-kit/`. Regenerate; never hand-edit the
mirror. **`installer/README.md` is the fourth kit-shaped surface and is
deliberately not one of them**: `installer/` is not a kit, so there is no
`docs/installer/` and its edit is a plain file edit — checked at this stage
rather than assumed from its shape, because assuming the mirror exists would
send a build batch looking for a regen that has no command. The regen commands
and the freshness gates are rostered at docs/site-architecture.md §Generated
projections, which is also where the fan-out a new gate stales is enumerated.

### (6) Registration, descriptor, fixtures and the generated fan-out

The kit-landing obligations, each already specified and none re-decided here.
**{mechanical}**

A `check-unmarked-claim.gate` descriptor carrying its
`# graph:` manifest (`couples=`, `dir=`, `valve=`, `tier=precommit`), its
`# install:` line and its one-line `# spec:` pointer; a row in
`scripts/gates.list`; a `good/`+`bad/` fixture pair under
`canon-kit/gate-tests/check-unmarked-claim/` whose `bad/` carries a
**line-wrapped** unmarked claim, so the census's measured blindness is executable
rather than asserted; a sibling `check-unmarked-claim.test.sh` for the
fail-closed vocabulary arms a one-pair harness cannot spell; the gate roster
block in `canon-kit/README.md`; and the regeneration of every projection a new
gate stales, the generated pre-commit hook included. The four gate-sdk contracts
(output, fail-closed, fixture-pair, self-lint) are gate-sdk/SPEC.md §The gate
model's and are satisfied, not restated.

## The honest limit — one site of thirteen, and it is a roster property

**First, the count, because two true numbers are in circulation and a build
session will meet both.** The census reports "twelve findings across eleven
files" while its own sub-counts read `FALSE (9)` and `OVERSTATED (4)`, which sum
to thirteen. Both are right and they count different things: **thirteen sites,
twelve findings, eleven files.** The reconciling item is the gate-sdk definitional
pair — `gate-sdk/SPEC.md:5` and `gate-sdk/README.md:10` are **one** finding (a
single canonical definition) living in **two** files, so the headline counts it
once and the `FALSE (9)` sub-count counts it twice. Delta 5's table is keyed on
**sites**, which is why it has thirteen rows; the envelope the operator ruled is
the same set either way, and nothing here widens or narrows it.

Against the roster delta 3 declares, **twelve of the thirteen sites fall inside
the class and one does not**: `lifecycle-kit/SPEC.md:1912`'s bare `awk` token,
excluded on the measured noise ground in delta 3. The census's own reading of the
originally-proposed roster was **three** structurally missed — a bare token, a
paraphrase, and the inverse claim. Two of those three are recovered here, and not
by tuning strings:

- the **paraphrase** (`SECURITY.md:42`, "A vendored kit is bash") is recovered
  because a class ERE is written over the claim, not over one canonical phrasing;
- the **inverse claim** (`CONTRIBUTING.md:57`) is recovered because the class
  covers both directions, which no stale-shell roster can.

The remaining miss is **not** closable by adding the token: doing so is what
would make the gate noisy, and a noisy gate is bypassed. It is stated here so a
later reader knows the bound was measured and priced rather than overlooked.

Two further bounds, both deliberate:

- The gate cannot tell a **true** substrate claim from a false one. It asserts
  that the claim is *bound to an oracle*; `check-measured-claim` then asserts the
  oracle agrees. Neither reaches a sentence whose prose is true today and whose
  reasoning is wrong — row 2's corrected *cause* is a human judgment and stays
  one.
- A consumer that declares no class gets a clean skip. This gate ships inactive
  and only a consumer's own vocabulary turns it on, which is the seam working as
  designed and not a gap.

## Producers and consumers

**`CANON_KIT_CLAIM_CLASSES_CMD` (new knob).** Producer: the consumer's config
file — in this repo `scripts/canon-config.sh`, which is sourced by
`lib/gate.sh`'s config seam on every gate load, so the enabling config is
actually emitted rather than test-only. Consumer: `check-unmarked-claim`, through
`spec_claim_vocabulary`, at gate startup. Bridged readers:
`CANON_KIT_CLAIM_CLASS_IDS` / `CANON_KIT_CLAIM_CLASS_PATTERNS`, read by the
compiled member and written only by the kit library while one of the two is the
knob under resolution — the transport/payload pattern, index-aligned, with the
generated pre-commit hook serializing them into the gate's `env` prefix exactly
as it already does for `CANON_KIT_PAYLOAD_CLAIM_IDS` /
`CANON_KIT_PAYLOAD_CLAIM_PATTERNS`.

**`scripts/claim-classes.sh` (new emitter).** Producer: invoked by the knob above.
Consumer: `spec_claim_vocabulary`, which validates slug-shaped ids, rejects a
repeated id, rejects a line carrying an extra tab, and exits 2 on a command
error. Every field it emits has a named reader: `<class-id>` is read by the
finding renderer (it names the class in the output) and by the loader's duplicate
check; `<ERE>` is read by the paragraph matcher at the single scan transition.
No field is added that nothing reads.

**`gate-substrates` (new oracle key).** Producer: `scripts/measured-claims.sh`,
already wired as `CANON_KIT_MEASURED_CLAIMS_CMD` and already emitting on every
battery run — no new enabling config, and the key is live the moment the emitter
ships. Consumers: `check-measured-claim` arm A (compares the marker's value to
the emitted value) and arm B (the key is now in the roster, so a marker naming it
no longer fails closed). Read at the single scan transition, per marker; no
persistent state. **The four sites that will carry it** are rows 3, 4, 11 and 13
of delta 5 — named here because a key with no marker naming it is a field with no
reader, and would be removed.

**`check-unmarked-claim` (new gate).** Producer of nothing but a verdict. Its
consumer is the committing session through the output contract, on three
triggers: the generated pre-commit hook (tier `precommit`), `run-gates.sh` for
the full battery, and CI. `run-gate-tests.sh` consumes its fixture pair.

**Existing integration prose that describes the prior flow.** Two paragraphs
describe a state this amendment changes and are updated in the sections listed
below rather than left to drift: §check-measured-claim's **known-limit paragraph**
("a claim nobody marks is uncaught here. What narrows it is §check-manifest-count's
discharge…") now has a second narrowing and must name it; and §lib/spec.sh's
**claim-vocabulary bullet**, which enumerates `spec_claim_vocabulary`'s callers
by name, gains a fourth.

**No corpus is narrowed by this amendment**, so point 5 of the
causal-completeness check (each reader's red condition under a narrowing) does
not bind. Stated rather than skipped: delta 5 edits eleven files and adds none
to and removes none from any gate's scanned set, and delta 2 adds a reader to an
existing surface knob without changing the glob array. The one place a set
*grows* is the measured-claims roster (delta 4), and a key added to a roster can
only clear arm-B failures, never create one.

## Existing sections updated

Each names the delta that owns it.

- **canon-kit/SPEC.md §check-unmarked-claim** — a new section, landing in the
  gate-section sequence beside its family (`check-measured-claim`,
  `check-install-claim`, `check-payload-claim`): the invariant, the two arms, the
  valve, the three remedies, the whitespace-normalization calibration and its
  §check-manifest-count precedent, the paragraph unit, the coverage-only bound
  and the loader constraint that forces it, the criterion-4 verdict, and the
  born-native substrate note (deltas 1, 2). It carries **no roster literal** —
  the seam in delta 2 is what keeps the kit's own prose out of its own class.
- **canon-kit/SPEC.md §check-measured-claim** — the known-limit paragraph is the
  prior-flow prose this change falsifies: it currently names
  §check-manifest-count's discharge as the *only* thing narrowing the
  nobody-marks limit. It gains the second narrowing and the boundary between them
  — the cardinal ban narrows by *shape*, the claim class narrows by *subject*
  (delta 1). The extent-claim paragraph gains `gate-substrates` as this repo's
  first set-valued inhabitant, which it currently describes hypothetically
  (delta 4).
- **canon-kit/SPEC.md §lib/spec.sh** — the claim-vocabulary bullet enumerates
  `spec_claim_vocabulary`'s callers (`spec_install_transports`,
  `check-payload-claim`, `spec_measured_claims`) and gains
  `check-unmarked-claim` as the fourth; the bullet's own argument — that taking
  the command as an argument means a second claim axis costs a caller and not a
  second copy — is now attested rather than anticipated, and says so (delta 2).
  The shared count adapter's boundary paragraph gains the note that its
  wrap-straddling normalization is now read by a second gate (delta 1).
- **canon-kit/SPEC.md §Layout and configuration** — the knob roster gains
  `CANON_KIT_CLAIM_CLASSES_CMD` and its bridged
  `CANON_KIT_CLAIM_CLASS_IDS` / `CANON_KIT_CLAIM_CLASS_PATTERNS` pair, in the
  shape and position the transport and payload entries already use, with this
  repo's setting named (delta 2). The `CANON_KIT_MEASURED_SURFACE_GLOBS` entry
  gains its second reader and the recorded cost of sharing (delta 2).
- **canon-kit/SPEC.md §Content tiering — the star topology** — the
  quantitative-literals bullet names `check-measured-claim` as the mechanism
  turning a transcribed number into a freshness-gated copy, and now must say what
  makes marking happen at all rather than leaving it to authorial memory
  (delta 1).
- **canon-kit/README.md** — the gate roster block inside its
  `<!-- gate-roster:begin -->` markers gains `check-unmarked-claim`;
  `check-readme-roster` reds a shipped check absent from it (delta 6).
- **scripts/gates.list**, **scripts/canon-config.sh** — the registration row and
  the knob wiring (deltas 2, 3, 6).
- **scripts/measured-claims.sh** — its `# spec:` header line describes the file
  as emitting the facts a governed sentence may state as a number; with an extent
  key it emits one that is not a number, so the line is corrected rather than
  left half-true (delta 4).
- **gate-sdk/SPEC.md §The gate model** and **§Layout and configuration** — no
  change is owed to either, and that is recorded so the merge does not go looking:
  the new gate adds no contract to the model and no gate-sdk knob. Its
  `# graph:` manifest and its descriptor format are consumed by existing
  machinery unchanged (delta 6).
- **The eleven prose surfaces** — `README.md`, `docs/index.md`,
  `docs/methodology.md`, `docs/positioning.md`, `docs/install.md`,
  `gate-sdk/SPEC.md`, `gate-sdk/README.md`, `lifecycle-kit/SPEC.md`,
  `SECURITY.md`, `installer/README.md`, `CONTRIBUTING.md`, each per its row in
  delta 5's table (delta 5). `gate-sdk/SPEC.md` appears both here and above
  because it is edited by two different deltas at two different sections.
- **The `docs/` kit mirror** — `docs/gate-sdk/SPEC.md`, `docs/gate-sdk/README.md`,
  `docs/lifecycle-kit/SPEC.md`, `docs/canon-kit/SPEC.md` and
  `docs/canon-kit/README.md` are generated projections that go stale the moment
  their source is edited. The regen commands and their freshness gates are
  rostered at docs/site-architecture.md §Generated projections; the merge runs
  them in the same commit (all deltas).
- **The generated pre-commit hook** — `scripts/git-hooks/pre-commit` is generated
  and never hand-edited; a new `precommit`-tier gate with a `# graph:` manifest
  restales it, and `bash gate-sdk/bin/gen-pre-commit.sh --write` regenerates
  (delta 6).
- **`native/`** — a new module plus its registration in the binary's subcommand
  dispatch, and `bash gate-sdk/bin/build-native.sh` as its own commit-time step
  which `cargo test` does not discharge (delta 1).

## What the build owes beyond the deltas

- **The oracle is re-read, never carried.** Every marked sentence takes its value
  from `bash gate-sdk/bin/port-blockers.sh --group` at the moment of the edit. If
  the number has moved since this amendment was written, the amendment is wrong
  and the oracle is right — that asymmetry is the entire point of the unit.
- **Row 7 and row 8 land together or not at all.** Two surfaces carry one
  canonical definition; splitting them across batches ships a tree where the
  kit's SPEC and its README define the same noun differently, and
  `check-surface-duplication` is the gate that would find it.
- **The census is not re-bought.** `.workflow/survey-record.md`'s 2026-08-22
  block is the roster of record; its witness (a clean corpus diff since the
  recorded rev plus an unchanged oracle verdict) was re-run at this stage and
  holds. A build session wanting the quotes reads that block.
- **No gap is filed by this amendment.** The one adjacent question surfaced
  while authoring — whether the definitional sites should carry a marker at all —
  is *ruled* in delta 5 rows 7 and 8 rather than deferred, so nothing is owed to
  the gap inbox from here.

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
