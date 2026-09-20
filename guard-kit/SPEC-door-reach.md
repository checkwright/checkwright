# SPEC amendment: door-reach

`check-door-binding`'s reach stops at the kit roots, so no gate sees a door on a
consumer's own surfaces — which is how `README.md`'s headline try-it command
stayed invisible to every oracle across three measurements. This amendment mints
the adopter/contributor discriminator the widening needs and adds the assertion
that reads it. Its sibling `gate-sdk/SPEC-front-end-residency.md` rules *which
spelling* an adopter-facing door may carry; this one rules *which surfaces a gate
reads as doors*.

## What changes

### (1) A third assertion, rather than a widened assertion A

`check-door-binding` gains **assertion C**: over a consumer-configured surface
set beyond the kit roots, every door reds unless its site declares itself
contributor-facing. Assertions A and B are untouched. **{design-bearing}**

**Why a third assertion and not a wider A.** A's corpus is *kit-shipped*
surfaces, whose audience the door sweep already settled — an adopter-facing
surface by construction, needing no per-site declaration, which is why A can
red on a bare door. A consumer's own surfaces are **mixed-audience by nature**,
so the same predicate cannot apply: measured, `README.md` carries both classes,
line 25 being the landing page's headline try-it command and lines 122–141 the
contributor battery register held in name-set parity with `EVIDENCE_KIT_SUITES`
(evidence-kit/SPEC.md §check-battery-roster). Folding two predicates into one
assertion would make its red line ambiguous about which rule was breached, and
would move every existing consumer's A verdict on upgrade. C is additive: with
its corpus knob unset, no consumer's verdict changes at all.

### (2) The discriminator is default-deny plus a site declaration, and two alternatives are refused

The token is `<!-- door-contributor: <reason> -->`, the reason mandatory on the
`comment-tier-exempt:` convention (canon-kit/SPEC.md §The shared spec adapters);
an empty reason is malformed and reds rather than exempting.
**{design-bearing}**

**There is no `door-adopter:` twin, and the omission is the design.** The
unmarked case is adopter-facing, because declaring the complement on every other
door would be a roster maintained against itself — context-kit/SPEC.md
§bin/env-probe's audience axis already rules exactly that for its own empty
value, and the rule is reused rather than restated. Default-deny is what closes
the failure the queue entry names: *a future adopter-facing page reintroduces the
repaired defect and nothing reds*. Under default-deny a new page's door reds the
day it lands, with no roster to remember to update.

**Two scopes, because a one-site-one-declaration grammar is unaffordable and an
unaffordable gate does not land.** Measured over this repo's own candidate
corpus, 57 door sites on 19 hand-authored files plus 144 on 25 generated mirrors
would each need a line.

- **Site scope** — the declaration on the door's own line or the line above, the
  shared exempt window this repo's other valves already ride.
- **Span scope** — a declaration standing alone on its line covers every door to
  the end of its enclosing unit, resolved two ways: the **fenced block** it
  immediately precedes, or the **whole file** when it stands before the file's
  first `#` heading. The fence resolution is what lets one line cover
  `README.md`'s eighteen-line battery register without putting HTML comments
  inside a `bash` fence; the file resolution is what lets one emitter edit cover
  a whole generated page.

**A door inside a generated region takes its declaration from the emitter, never
from the page**, because a declaration written into emitted output is erased by
the next regeneration. Measured, this repo's generated door carriers and their
owners are `docs/<kit>/{README,SPEC,DOCTRINE}.md` (the docs-mirror emitter),
`docs/enforcement.md`, `docs/footprint.md`, `docs/check-graph.html` and
`docs/value.md`'s `value-rollup` block — five emitters, not fifty pages. The
gate states the rule; which emitter writes which declaration is the build's
roster, re-derived from the freshness gates rostered in
docs/site-architecture.md §Generated projections and their freshness gates.

**Refused — a configured roster of adopter-facing sites.** It is the
trivially-true proxy gate-sdk/SPEC.md §When a gate earns its place bars: a page
added tomorrow is absent from the roster and reds nowhere, so the gate would
manufacture confidence at exactly the point where every previous measurement
missed.

**Refused — a gate-side path heuristic** (`README.md` adopter, `docs/`
contributor). Measured false on the one file that matters: both audiences live
inside `README.md`, so no file-level axis separates them, and a per-site
heuristic has no signal to read. This is also why the queue entry's "the `docs/`
sites are contributor-facing" reading is kept only as a *finding about today's
tree*, never as the predicate.

### (3) The corpus knob, default empty

`GUARD_KIT_DOOR_ROOTS` — array of repo-relative paths, each a tracked file or a
directory; default **empty**, in which case assertion C is inert and
`check-door-binding`'s verdict is what it is today. **{design-bearing}**

Each directory entry is walked with the prune set A already applies —
`walk::prune_dirs()` composed with `gate-tests/` and `smoke/` — so a consumer
adds a governed tree once and every page later added under it is swept with no
further edit, which is the derivation-first half of delta 2's default-deny.
**The kit ships no default entry**: which of a consumer's surfaces it governs is
the consumer's fact, never the kit's (gate-sdk/SPEC.md §The provenance seam), on
`GUARD_KIT_BREADTH_PROBES`' precedent in §Layout and configuration.

**Fail-closed:** a configured entry that resolves to neither a tracked file nor
a directory is exit 2, the misconfiguration answer, never a false clean — the
posture every sibling in that section takes.

**The clean line gains a third count**, the configured surfaces swept, on the
existing ground that a corpus which silently shrank to nothing must be visible on
green. So the *verdict* is unchanged for a consumer configuring nothing; the
clean line is one count longer.

### (4) The predicate and the arm exemption are A's, not a second copy

Assertion C reuses `is_door`, `token_span` and `names_fail_open_arm` unchanged.
**{mechanical}**

One implementation, because a second copy of the door/file discriminator is the
shape §check-door-binding refuses everywhere else: two spellings of one
predicate would let A and C disagree about whether a line is a door. The
fail-open arm exemption in particular is corpus-wide and keys on the arm alone —
the sibling amendment's delta 2 records the measurement that refuses narrowing
it to configuration files.

### (5) This repo's configuration and its declaration sweep

`scripts/guard-config.knobs` gains `GUARD_KIT_DOOR_ROOTS[] = README.md` and
`GUARD_KIT_DOOR_ROOTS[] = docs`, and the declarations land across the swept
corpus. **{mechanical}**

The roster is a probe, not a transcription — the corpus moves with every commit,
so a copy here would be stale on landing. Run
`git grep -nE "(bash|sh|-File) +[^ ]*run-gates\.(sh|ps1)|run-gates\.(sh|ps1)('|\`)? +-" -- README.md docs`
and subtract the fail-open arms; `git grep -l "^generated: true" -- docs` names
the mirror set whose declaration the emitter writes. **Measured at authoring:**
59 door-shaped sites on non-generated pages and 144 on the 25 mirrors. **Exactly
two of the 59 are adopter-facing and are deliberately left red**, being the
paired debt entry's deliverable: `README.md`:25 and `docs/index.md`:27. Every
other site is contributor-facing on its face — a regeneration banner, a battery
register, a worked example in an architecture page — and takes a declaration
whose reason says which.

### (6) The fixture pair grows an executable statement per new arm

`guard-kit/gate-tests/check-door-binding/{good,bad}/` gain a configured extra
root; `good/` carries a site-scoped declaration, a fence-scoped one and a
file-scoped one, `bad/` an undeclared door and a declaration with an empty
reason. **{mechanical}**

The empty-reason case is in `bad/` rather than left to review because it is the
one way the valve can be written and still fail, and the fixture pair is the
consumer's whole verification oracle once the source is withheld (gate-sdk/SPEC.md
§Consumer payload). The two fail-closed exits — a configured entry resolving to
neither a file nor a directory, and an unreadable configured surface — go to
`gate-tests/`, which a one-pair harness cannot spell; `good/expect.txt` matches on
`DOOR-BINDING: clean` and so is unaffected by the third count, which is recorded
so the count change is not mistaken for a fixture edit that was skipped.

## Producers and consumers

**The new names, and every one has a reader.** Three are minted: the assertion
`assertion C`, the knob `GUARD_KIT_DOOR_ROOTS`, and the comment directive
`door-contributor:`.

- **`GUARD_KIT_DOOR_ROOTS`.** Producer: the consumer's knob file, resolved
  in-process from guard-kit's static defaults table and the consumer's file
  (§Layout and configuration). Its enabling configuration is set by a **deployed
  configuration** and not only by a test — delta 5 sets it in
  `scripts/guard-config.knobs` in the same unit, which is what keeps the knob off
  the dead-everywhere-but-unit-tests branch of causal-completeness point 1.
  Consumers: `check-door-binding` assertion C; the knob roster emitter
  (`--emit knob-roster`), which prints every knob with its shape and rendered
  default and needs the defaults-table row; `check-knob-citation` and
  `check-knob-default-coupling` (canon-kit/SPEC.md §check-knob-citation,
  §check-knob-default-coupling), the **roster-holding readers** that red on a
  knob whose SPEC row or whose declared default disagrees with the crate — so
  the §Layout and configuration row and the crate's defaults row are one change,
  never two.
- **`door-contributor:`.** Producer: an author, or an emitter writing a
  generated page. Consumer: assertion C. **Roster-holding reader:**
  `check-comment-tier` (canon-kit/SPEC.md §check-comment-tier) reds at an
  obliged site on a comment directive its roster lacks, so the directive takes
  its roster row in the same unit. This is causal-completeness point 2's
  comment-directive clause, named rather than left to the build to discover.
- **`assertion C`.** Producer: the gate run — the generated pre-commit hook,
  `run-gates.sh`, CI, and `--run-gate-tests` through the fixture pair. Consumer:
  the committing session, through the output contract.

**Every field has a named reader.** The declaration carries exactly two fields,
its class token and its reason. The class token is read by C as the exempting
match; the reason is read by C into the **clean-line and red-line detail** so an
audit trail exists, on the same ground `[drain-exempt:]`'s reason is echoed
(lifecycle-kit/SPEC.md §check-stage-entry). No third field is reserved.

**Point 5 — each reader's red condition, and this amendment does narrow one
corpus.** Delta 4 reuses A's predicate, which *widens* nothing and prunes
nothing, so A's own verdict is monotone and unchanged. But **delta 3's prune is a
narrowing** — `gate-tests/` and `smoke/` are excluded from each configured
directory — and the point binds, so each reader's red condition is named rather
than cleared by inspection:

- **Assertion C** reds on *finding a door*, never on finding none, so a narrower
  corpus can only remove violations. Monotone; clears.
- **The clean line's third count** is a **report, not an assertion** — there is
  no floor and no exact-count arm on it, so a pruned corpus prints a smaller
  number and stays green. Stated because a count on a clean line is exactly the
  shape point 5 warns about, and here it is deliberately not a verdict.
- **No other gate reads the knob or the directive**, so no minimum, coverage
  floor or exact-count reader exists to flip. Probe:
  `git grep -n "DOOR_ROOTS\|door-contributor" -- native/src scripts` returns
  nothing before this unit lands.

**Point 6 — every member's satisfying value.** The corpus delta 5 obliges is
enumerable at authoring time, and the probe in that delta is the enumeration.
Each member's satisfying value is a `door-contributor:` declaration at the
narrowest scope that covers it, **except two members with none**:
`README.md`:25 and `docs/index.md`:27 are adopter-facing, so no declaration
satisfies them and the assertion is **not narrowed past them** — they are meant
to red until the paired debt entry lands the sibling amendment's delta 3. That is
the one case where a member's satisfying value is a *repair* rather than a
declaration, and naming it here is what stops a build batch from silencing them
with a declaration to make the battery green.

## Existing sections updated

- **guard-kit/SPEC.md §check-door-binding** — the opening invariant sentence
  ("Invariant, two assertions over the tracked tree") and the corpus paragraph
  are re-phrased to carry three assertions and the configured extra corpus; the
  door/file discriminator, the permission-pattern paragraph and the arm-exemption
  paragraph are **kept unchanged** and are explicitly re-used by C rather than
  restated for it. The red/clean paragraph gains C's finding shape and the third
  count. (deltas 1, 2, 3 and 4)
- **guard-kit/SPEC.md §Layout and configuration** — the knob roster gains
  `GUARD_KIT_DOOR_ROOTS` with its shape, its empty default and the seam ground
  for shipping no default entry, sited beside `GUARD_KIT_BREADTH_PROBES` whose
  precedent it takes. (delta 3)
- **canon-kit/SPEC.md §check-comment-tier** — the comment-directive roster gains
  the `door-contributor:` row, without which a declaration at an obliged site
  reds. (delta 2)
- **`scripts/guard-config.knobs`** — the two `GUARD_KIT_DOOR_ROOTS[]` entries,
  each under the `# spec:` header line the knob-file grammar obliges. (delta 5)
- **`guard-kit/README.md`** — the gate's one-line summary names two assertions;
  re-phrased to three. (delta 1)
- **`native/src/emit/docs_mirror.rs` and the four banner emitters named in delta
  2** — each writes the file-scoped declaration into its output. The roster is
  the probe in delta 5, re-derived at the merge rather than transcribed here.
  (deltas 2 and 5)
- **`guard-kit/gate-tests/check-door-binding/{good,bad}/`** and
  `guard-kit/gate-tests/` — the fixture pair and the two fail-closed exits.
  (delta 6)
- **`docs/guard-kit/SPEC.md` and `docs/canon-kit/SPEC.md`** — generated mirrors
  of the two edited SPECs, stale the moment any delta lands and regenerated by
  their emitter rather than edited. (all deltas)

## Retired spellings

- None — no delta retires a spelling. The invariant sentence's word *two*
  becomes *three*, which is a count in running prose and not a name; no token,
  knob, arm, path or directive is displaced, and the three names the amendment
  mints had no earlier spelling to retire.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation; points 5 and 6 are discharged in §Producers and consumers
      above, 5 by naming each reader's red condition against delta 3's prune and
      6 by enumeration including the two members with no satisfying declaration.
- [ ] **Instruction surfaces: instruction only** — no template, agent definition
      or shim is touched; the knob-file row carries its `# spec:` pointer and no
      grounds, the grounds living in the SPEC row delta 3 adds.
- [ ] **Merged with no information lost** — the discriminator, permission-pattern
      and arm-exemption paragraphs survive verbatim in substance; the invariant
      and corpus paragraphs are re-phrased, not appended to.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls guard-kit/SPEC-*.md`), and any surviving filename citation
      of `gate-sdk/SPEC-front-end-residency.md` is repointed at the canonical
      section it merged into, per canon-kit/SPEC.md §check-amendment-queue arm
      (e).
- [ ] **Removals propagated** — declared negative above, re-run by
      `check-amendment-retired-spelling`.
- [ ] **Gaps filed** — cross-component gaps filed to the gap inbox. One is
      already visible: `docs/posts/` carries a door-shaped line and is immutable
      release history, so whether a post takes a declaration or the corpus
      excludes the tree is a disposition the build costs and files rather than
      decides by whichever is easier.
