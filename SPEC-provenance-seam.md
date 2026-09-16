# SPEC amendment: provenance-seam

**The last kit SPECs carrying this project's ruling provenance are swept, and
`check-provenance-seam` lands in canon-kit to keep every kit SPEC swept.** This is the
closing increment of a corpus deliverable. Earlier increments swept gate-sdk, then
context-kit and guard-kit, then lifecycle-kit, delegation-kit and queue-kit, and each
shipped without a gate. This one ships the gate, so the unit ends here. It does not
demote.

**The predicate is recovered, never re-authored.** The discriminator, the taxonomy and
the move-versus-delete test are deltas 1-3 of `git show d190c2f6^:gate-sdk/SPEC-seam-sweep.md`.
Four supplementary rules (a specimen is not a stamp; a live procedural status is swept
and never migrated; the cut lands between an attribution and numeric evidence that
abuts it; a passage's claim to be its own home is about the mechanism, not the stamp)
are deltas 1-4 of `git show aae9f868^:SPEC-seam-slice.md`. The merged rule's home is
gate-sdk/SPEC.md §The provenance seam. Nothing below restates any of them. Delta 2
adds the one rule this corpus needs on top.

**The census is filed on `.workflow/survey-record.md` and not carried here**, which
means both the scope block and this stage's retired-slug block. What a reader of this
file needs: the scope census **under-counts in two directions**. First, a slug scan
limited to live queue slugs cannot see a *retired* slug, and retired slugs are cited
in three kits the census had as clean or already landed: evidence-kit, lifecycle-kit
and context-kit. So evidence-kit does not leave the corpus. Second, a queue slug can
**equal a mechanism name** (a gate name, a kit directory), and that shapes the gate's
slug arm (delta 3).

## What changes

### (1) The sweep: every kit SPEC, with the recovered predicate applied as the build finds the file

Build applies the recovered predicate to **every kit SPEC**, not to the four kits
the entry still names {design-bearing}. The entry calls the remainder derived and
never frozen, and this stage's probe shows why: retired-slug pointers survive in
kits whose sweep has already landed. Build works through the files in the order below.
A site list is not a roster. It is a floor build re-derives with the named probes,
because a roster rots between spec and build.

- **The gated floor**: every site `check-provenance-seam` (delta 3) reds on. Build
  runs the gate's own arms before registering it. At this stage's rev, the probes
  below gave: two dated attribution stamps (canon-kit §check-measured-claim's
  born-native paragraph, and drift-kit's session-key divergence paragraph); one
  ruling-record pointer (the same canon-kit paragraph); and live queue-slug
  citations in canon-kit (the two-tier-detector debt, the criterion-4 membership
  ruling), drift-kit (the A/B benchmark rung, three sites), site-kit (the gawk-floor
  residue) and gate-sdk (the hosted-attestation rung, which is accretion).
- **The ungated remainder**, swept under the same predicate but not caught by the
  gate (delta 3 states why for each shape):
  - retired-slug pointers: the tail-port cut in canon-kit, site-kit and evidence-kit;
    the deprecation and upgrade-path rungs in lifecycle-kit; the doctrine lockstep
    unit in context-kit;
  - dated landing labels: drift-kit's port-cut and survey-cut labels and its
    released-hold date, and doctrine-kit's port date;
  - dated incidents: site-kit's table incident;
  - attributive pointers into the agent file: gate-sdk's commit-message section
    names its rule's owner that way.
- **Judged, not presumed**: site-kit's `faithful-artifact-verification class` may use
  a retired slug as an ordinary term. Build applies the recovered corollary: the
  test is whether the identifier resolves only in this tree, never the tense.
- **Kept**: dated measurements such as site-kit's two-battery-cycle measurement and
  the dated oracle runs in gate-sdk, and specimens such as lifecycle-kit's
  `verified <date>` idiom. The recovered exclusions protect both.

**Move versus delete, answered for the last kit that still owed it.** drift-kit's
session-key paragraph is the one site in this corpus that argues for its own home ("a
later reader asking why two meters ever keyed differently needs the answer"). Under
the slice's fourth rule, that is a claim about where the mechanism lives. The
mechanism and the refused alternative stay in that section, undated and impersonal.
The attribution and the date are deleted, and nothing moves to the ruling record.

**Probes** (the ones that bought the floor; build re-runs them rather than reading
the list above):
(a) every lead-line slug of 12+ characters in the current queue, matched against
each kit SPEC with `-` counted as part of a word;
(b) the same match over every slug that ever appeared on a queue lead line in
`git log -p` of the queue file;
(c) a sentence-scoped search for an ISO date plus an attribution marker from
delta 4's default set, fenced blocks skipped;
(d) a search for the ruling-record, local-brief and agent-file pointer shapes.
The kit SPEC corpus is `*/SPEC.md` less the non-kit `installer/`.

### (2) A date that bounds a consumer's own data names the release, never the landing

One passage is not covered by the recovered rules {design-bearing}. drift-kit's
overhead-meter section says the log series splits at a date. The date is load-bearing:
a reader of the trend log splits populations on it. But it is this project's landing
date. An adopter who upgraded across the change sees the split at *their* upgrade,
not at our landing, so the date is false for every consumer except this repo. It is
also provenance.

**The rule: when a kit behaviour change partitions data the kit writes into a
consumer's tree, the SPEC names the release that carried the change.** A release is a
public identifier that a vendoring consumer can resolve against its own pin. A landing
date is neither. Build reads the release off the tag containing the change. The
rule goes into gate-sdk/SPEC.md §The provenance seam as one sentence beside the
dated-measurement exclusion, because it marks the same boundary from the other side:
a date that must survive, respelled so that it resolves for everyone.
**Not yet applied.**

### (3) `check-provenance-seam`: a canon-kit gate over every kit root's SPEC, exemption-free

canon-kit ships a new gate, born native (a Rust module, a `.gate` descriptor and a
`good/`+`bad/` fixture pair) {design-bearing}.
**Invariant:** no kit SPEC carries a publisher-provenance marker of any shape below.
**Not yet applied.**

**Corpus.** The canonical spec (`CANON_KIT_SPEC_NAME`) at the root of every
`gate_kit_roots` directory. The set is derived, never listed. The corpus is scanned
**only when `CANON_KIT_SCAN_KIT_ROOTS` is `1`**, which is that knob's existing meaning:
the kit docs are the consumer's own first-party content. At the default `0` the gate
passes with a clean line saying kit roots are a dependency's. That default is correct
for an adopter who vendors a SPEC-bearing tree, such as a submodule. That adopter's
queue slugs and private file names would otherwise red someone else's document.
A consumer authoring its own kits sets the knob, and so does this repo. The gate is
kit-shipped rather than a consumer gate in `scripts/` for that reason: the provenance
seam is a rule for every kit publisher, and gate-sdk carries its home in every profile.

**Fenced blocks are skipped. Nothing else is: no per-site valve, no section carve-out,
no path exemption.** A fence holds grammar being shown, such as a knob-file example
naming a ruling record, and the recovered specimen rule already rules a shown instance
out of the class. An inline-code span *is* scanned, because a pointer written in
backticks is exactly as dead in a vendored copy.

**Arms** (each finding reports file, line, arm and matched span; a match that wraps a
line is reported at its first physical line through the shared manifest-prose driver):

- **A — dated attribution.** An ISO `YYYY-MM-DD` date and an authority marker
  (delta 4) in one sentence. A sentence is a paragraph span, rejoined across line
  wraps, that ends at `.`, `?`, `!` or `;` followed by whitespace. A date with no
  marker passes, and that is the dated-measurement exclusion made mechanical. A marker
  with no date passes, because operator, lead and ruling are this kit family's
  role vocabulary and an unattributed rule about them is mechanism.
- **B — agent-file pointer.** A name from `CANON_KIT_SEAM_AGENT_FILES`, followed
  directly by a section citation (`§`, with or without a joining space or comma)
  **or a possessive `'s`**. A bare mention passes, because a knob default naming the
  consumer's agent file is mechanism.
- **C — private surface.** Any mention of a path in
  `CANON_KIT_SEAM_PRIVATE_SURFACES`, matched as a whole path token (so a knob name
  that contains the stem does not match).
- **D — live queue slug.** Any lead-line slug in `CANON_KIT_QUEUE_FILE`, from every
  section, that is at least `CANON_KIT_SEAM_SLUG_MIN_LEN` characters long, matched with
  `[A-Za-z0-9_-]` counted as word characters on both sides. **A slug equal to a
  mechanism name the tree defines is not a finding**: a `gate_kit_roots` directory
  name, or a gate name that resolves through the registry or a kit's `checks/`. This
  is derived and structural, not a valve. Its ground is measured: across the queue's
  history, units named after the gate or kit they minted would otherwise red that
  gate's own SPEC section while the unit's entry stayed live.

**The possessive shape is named, and this is the ruling the entry left to this
slice.** A possessive pointer into the agent file points into the publisher's
always-loaded file exactly as a section citation does. The recovered corollary already
sweeps "a pointer a consumer cannot follow", so the difference is spelling, not class.
Leaving it out would carve an exemption by omission into a gate that ships
exemption-free. There are zero sites in the tree today, so naming it costs nothing.

**Honest limits, each stated in the merged section so a green run is not read as
a clean seam:**
- undated attribution passes arm A, since the role vocabulary rules a marker-only
  arm out;
- a dated landing or incident label with no attribution passes, since a date-only
  arm cannot tell it from a frozen measurement;
- a stamp dated in words rather than ISO passes;
- a retired slug or cut ordinal passes, because deriving retired slugs means reading
  queue history, which a fixture cannot pin and a shallow clone does not have;
- a slug below the length floor passes;
- an attributive agent-file pointer (*the agent file's ban*) passes, because telling
  it from a consumer-side mention (*the consumer's agent file carries …*) is judgement;
- kit templates, kit READMEs and `DOCTRINE.md` are outside the corpus.

The build sweep (delta 1) handles each ungated shape by hand. What stops those shapes
from building up again is the close-stage review, not this gate.

**A queue edit can now red a kit SPEC.** Filing a queue entry whose slug appears as
text in some kit SPEC reds the commit that files it. The finding names the SPEC line.
The remedy is to rename the new slug, which costs nothing while the slug is fresh.
The gate-inbox route never trips it, because the inbox is not the queue. So the
descriptor's graph-manifest couples list carries the queue file, as `knob:CANON_KIT_QUEUE_FILE`
in the §Layout and configuration class that knob-bounded walks use. Without it the
hook would not fire on the commit that introduces the finding.

**Fail-closed (exit 2):** a kit SPEC or the queue file that exists but cannot be
read; a malformed marker ERE (compiled through the crate's matcher before the first
line, as §check-manifest-temporal does); a non-positive or non-integer slug floor.
**An absent queue file** switches arm D off and says so in the clean line, and does
not fail the gate. An adopter authoring kits with no work queue is an ordinary
configuration, and the other three arms still judge it. `precommit` tier.

### (4) Four knobs, all static, in canon-kit's table

Four knobs are added to canon-kit's defaults table (`native/src/knobs/canon_kit.rs`)
and to the §Layout and configuration list, under the existing `_EXTRA` semantics
{design-bearing}. **Not yet applied.**

- `CANON_KIT_SEAM_AUTHORITY_MARKERS`: an ERE array matched against the case-folded
  sentence. It defaults to the bundled attribution set calibrated at this stage:
  `operator[- ](ruled|ruling|direction|directed|decided|ratified|approved|chose)`,
  `lead, `, `own-authority`, `ruled`, `ratified`, `consult`. Against the whole kit
  SPEC corpus, together with arm A's date, it finds exactly the two stamps delta 1
  names. A broader set that adds bare `operator`, `lead`, `ruling` or `direction` also
  reds a dated oracle run in gate-sdk and the specimen in lifecycle-kit, which is why
  the set is attribution-shaped rather than role-shaped. It is generic English plus
  the kit family's steering verbs, so it ships as a literal.
  `CANON_KIT_SEAM_AUTHORITY_MARKERS_EXTRA` defaults to empty and joins the
  `_EXTRA` roster. A base and extra that are both empty is malformed config, on the
  temporal-marker precedent.
- `CANON_KIT_SEAM_AGENT_FILES`: an array defaulting to `("CLAUDE.md")`. This is the
  harness's public file name, the same universal default the manifest set already
  derives. A consumer on another harness adds its own name.
- `CANON_KIT_SEAM_PRIVATE_SURFACES`: an array defaulting to **empty**. A publisher's
  ruling-record and local-brief names are that publisher's vocabulary, so no spelling
  ships as a literal. The empty default switches off arm C only and is stated in the
  clean line. **This repo sets its ruling record and its local-only brief.** The merged
  canon-kit section must say that **without spelling either name**. Spelling them would
  red the gate on its own SPEC, and it is the one place a knob section's usual
  "this repo sets `<value>`" idiom cannot be followed.
- `CANON_KIT_SEAM_SLUG_MIN_LEN`: a positive integer, default `12`. The floor keeps
  short slugs from matching ordinary hyphenated prose.

The gate reads `CANON_KIT_QUEUE_FILE`, `CANON_KIT_SPEC_NAME` and
`CANON_KIT_SCAN_KIT_ROOTS` as they already exist, and adds no corpus knob. The corpus
is derived from kit roots on purpose.

### (5) gate-sdk §The provenance seam names its enforcement

The seam section gains one clause saying that canon-kit's `check-provenance-seam`
holds the publisher-provenance class over kit SPECs, and where the gate's honest
limits are recorded. It also gains delta 2's release-boundary sentence {mechanical}.
A kit SPEC that cites the section keeps doing so, and no citation moves.
**Not yet applied.**

### (6) Registration, fixtures and the generated projections ride the landing commit

All landing work follows the gate-sdk landing checklist, with no deviation
{mechanical}. **Not yet applied.**
- **Descriptor and module**: `canon-kit/checks/check-provenance-seam.gate` and a
  `native/src/gates/` module registered in `native/src/gates/mod.rs`.
- **Fixture pair** under the gate's `gate-tests/`. `bad/` trips each arm once:
  - arm A: one dated stamp;
  - arm B: a section citation and a possessive;
  - arm C: a configured private name;
  - arm D: a live slug.
  `bad/` also holds a fenced copy of every one of them, which must *not* add a
  finding. `good/` holds a dated measurement, a specimen date, a bare agent-file knob
  default, a slug equal to a kit directory name, and a knob name containing a
  private-surface stem.
- **Config-driven paths** that a stock pair cannot spell go in a `.test.sh` beside
  the pair, as §check-manifest-count does:
  - `CANON_KIT_SCAN_KIT_ROOTS=0`;
  - an absent queue file;
  - `_EXTRA` union.
- **This repo**: `scripts/gates.list` registration, and `scripts/canon-config.knobs`
  sets `CANON_KIT_SEAM_PRIVATE_SURFACES`. The file already sets
  `CANON_KIT_SCAN_KIT_ROOTS` to `1`, and build checks that before relying on it.
- **Rosters that hold a name**: canon-kit's README gate list, `canon-kit/smoke/install.sh`,
  the regenerated pre-commit hook, and the docs projections, which are the canon-kit
  and gate-sdk mirrors, the enforcement map, the footprint and value rollups, and the
  check-graph page. Each freshness gate prints its own regen command.

### (7) The entry completes: this is the last increment, so the terminal move is Done

**The unit moves to Done at merge. It is not demoted** {mechanical}. canon-kit/SPEC.md
§Merging an amendment step 4's demotion branch exists for an entry whose corpus still
owes after the increment. After delta 1, no kit SPEC owes anything, and the gate that
the entry's end condition names ("lands WHOLE, gate green") is green on the tree. So
the entry's own end condition is met, and a demotion would leave a deferred entry
with no work.

## Producers and consumers

**New interface: `check-provenance-seam`'s findings.**
- *Producer:* the generated pre-commit hook and `run-gates.sh`, on a commit that
  touches a kit SPEC, the queue file or canon-kit's knob file (the descriptor's
  couples). This is reachable here because this repo sets `CANON_KIT_SCAN_KIT_ROOTS=1`.
- *Consumer:* the committing session, through the output contract. Each finding is
  read once at the scan transition, and no state persists.
- *Fields* (file, line, arm, matched span): the reader uses file and line to locate
  the site, arm to pick the remedy (delete the attribution, respell the pointer, or
  rename a fresh slug), and span to see which marker fired.
- *Roster-holding readers of the minted gate name* (causal-completeness point 2):
  `scripts/gates.list`, canon-kit's README list, `canon-kit/smoke/install.sh`, the
  generated hook, `docs/enforcement.md` and the check-graph page. The substrate-parity
  meta-gate holds the descriptor-to-module pairing.

**New knobs (delta 4).**
- *Producer:* canon-kit's defaults table and the consumer's knob file.
- *Consumer:* this gate alone, at resolve time.
- *Roster-holding readers:* `--emit knob-roster`, `check-kit-ref-liveness` (every
  `CANON_KIT_` knob in prose must resolve to kit source), `check-knob-citation`,
  `check-knob-default-coupling`, and `check-prose-enum`, where the `_EXTRA` roster
  sentence in §Layout and configuration is hand-listed. Each knob mention in the
  merged SPEC resolves only after the table row lands, so the table and the prose
  land in one commit.

**Satisfying value for every corpus member (point 6)**, because the gate obliges each
kit SPEC. The members are the `gate_kit_roots` directories, enumerated with
`ls */checks */smoke -d` over the tree. The kit-root set is canon-kit, context-kit,
delegation-kit, doctrine-kit, drift-kit, evidence-kit, gate-sdk, guard-kit,
lifecycle-kit, queue-kit and site-kit. Every member's satisfying value is **zero
findings**. After delta 1, the members with gated sites at this rev reach zero by
the sweep: canon-kit (arms A, C, D), drift-kit (A, D), site-kit (D) and gate-sdk (D).
The rest are at zero already, by probes (c) and (d) of delta 1 and by probe (a).
No member lacks a satisfying value, so no assertion narrows.

**Point 5: the sweep narrows a corpus** (the prose a reader finds in kit SPECs), so
each reader's red condition is named:
- **`check-md-refs`** reds on a reference that resolves to nothing. Deleting a
  ruling-record pointer only removes references, but delta 1's rewrites re-point
  surviving `§` citations, so build runs it rather than inspecting.
- **`check-manifest-count`** reds when it *finds* a bare cardinal. A rewrite that
  replaces a slug with what the slug stood for can introduce one ("the three
  benchmark legs"), so build runs it.
- **`check-manifest-temporal`** reds on a temporal marker that has no valve. Three
  drift-kit retirement sentences carry `manifest-temporal-exempt:` valves, each on
  the line where its dated cut label sits or the line above (two of the three wrap
  to the line above). Rewriting either line can separate a valve from its marker,
  so build runs it.
- **`check-measured-claim`** reds when a `measured:` marker disagrees with its oracle,
  and fails closed on an unknown key. The recovered abutment rule keeps the sweep off
  bound numbers, and build runs the gate as well.
- **`check-prose-enum`** has a coverage floor: it reds when a hand-listed enumeration
  omits a member. Delta 4 extends the `_EXTRA` roster sentence, and it must stay
  complete.
- **`check-unmarked-claim`** and **`check-tracking-claim`** read claim and tracking
  shapes. Removing a live procedural status (canon-kit's open criterion-4 ruling,
  site-kit's "that entry's taker" sentence) is exactly the shape they judge, so
  build runs them.
- **`check-provenance-seam` itself** reds on *finding* a marker, so it is monotone
  under the sweep. But it reds against the rest of the tree at the commit that
  registers it, so it lands registered only after, or together with, the last swept
  site.
- **The docs mirrors' freshness gates** red on any byte of drift and clear only by
  regeneration.

## Existing sections updated

- `canon-kit/SPEC.md` gains a new §check-provenance-seam next to the manifest-prose
  gates, and §Layout and configuration's knob list and `_EXTRA` roster sentence change
  (deltas 3, 4). The born-native paragraph under §check-measured-claim, the
  criterion-4 paragraph, the two-tier-detector limitation and the tail-port dispatch
  lines are swept (delta 1).
- `gate-sdk/SPEC.md` §The provenance seam changes (deltas 2, 5). The hosted-attestation
  rung pointer and the commit-message section's attributive pointer are swept (delta 1).
- `drift-kit/SPEC.md` is swept across the whole file, and the overhead-meter series
  split is respelled to its release (deltas 1, 2).
- `site-kit/SPEC.md`, `doctrine-kit/SPEC.md`, `evidence-kit/SPEC.md`,
  `lifecycle-kit/SPEC.md` and `context-kit/SPEC.md` are swept at the sites delta 1's
  probes find (delta 1).
- `native/src/knobs/canon_kit.rs` and a new `native/src/gates/` module with its
  `mod.rs` registration (deltas 3, 4).
- `canon-kit/checks/check-provenance-seam.gate` and its `gate-tests/` pair and
  `.test.sh` (deltas 3, 6).
- `canon-kit/README.md` and `canon-kit/smoke/install.sh` rosters (delta 6).
- `scripts/gates.list` and `scripts/canon-config.knobs` (deltas 4, 6).
- `TASK-QUEUE.md` `kit-spec-provenance-seam-sweep-remainder` moves to Done at merge
  (delta 7).
- <!-- update-target-exempt: generated projections, each rostered with its freshness gate and regen command in docs/site-architecture.md §Generated projections and their freshness gates; content derived from the sources the deltas edit --> The pre-commit hook, the `docs/` mirrors of every swept SPEC, `docs/enforcement.md`, `docs/footprint.md`, `docs/value.md` and `docs/check-graph.html`.

## Retired spellings

- None — no delta retires a spelling. The swept slugs stay live on their own queue
  entries or in history, and the swept stamps are attributions, not names.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness
      check holds for the gate, its findings and its four knobs. The narrowed prose
      corpus's non-monotone readers are run, not inspected.
- [ ] **Instruction surfaces: instruction only** — no template or shim changes. If
      build touches one, it carries no grounds (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — every engineering ground stays undated and
      impersonal in its section. No specimen, dated measurement or abutting number is
      swept. Nothing migrates to the ruling record unless the recovered test sends it
      there.
- [ ] **Gate green, registered, exemption-free** — `check-provenance-seam` passes on
      the whole tree with this repo's knobs, and its fixture pair and `.test.sh` pass.
- [ ] **Amendment deleted** — this file is removed on merge, and none remain at the
      repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — mirrors regenerated, `check-md-refs` green, and
      `check-amendment-retired-spelling` green on the negative form.
- [ ] **The entry completes** — it moves to Done, not demoted (delta 7).
- [ ] **Gaps filed** — cross-component gaps found during the work go to the gap inbox.
