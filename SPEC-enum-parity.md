# SPEC amendment: enum-parity

Queue entries: `readme-roster-enum-coverage` (the lead) and `prose-enum-common-word-tag-members`,
the enum half of `enum-and-citation-parity`. Both change `check-prose-enum`: the first gives it
a set to read and rules on what is not a set, and the second changes how it chains members into a
list.

A root-level amendment, because it spans several components:

- canon-kit owns §check-prose-enum and its gate.
- `native/` holds the bundled enum-set emitter.
- drift-kit's and delegation-kit's READMEs carry the landing fixes.

**What the survey found.** A read-only survey at spec, plus a probe of the live gate under an
overridden set command, found the following (the survey record's 2026-09-17 spec block carries the
witness):

- **The KPI roster is drifting right now.** drift-kit/README.md's bundled-KPI paragraph describes
  eleven of the twelve lead KPIs in words and omits `kpi-stage-economics-lag`. That is a fourth
  instance, and no close found it. The bundled set is a compiled table (`BUILTINS` in
  `native/src/emit/kpi/mod.rs`, 14 ids). Declaring it as a set reds **nothing** anywhere in the
  manifest set today, because the README names the KPIs by description rather than by id. So the
  set costs nothing to declare, and the README becomes gateable once it names ids.
- **The `## Use` invocation blocks are fenced**, and `walk_prose` skips fences. They are complete
  today, and the attested omission was a *mode* (an argv-tail flag) of an arm, which no registry
  holds.
- **The verdict set** the delegation-kit README restated now lives in a compiled `matches!` in
  `native/src/hook/stop_liveness.rs`. The README lists three verdicts and omits that `unresolved`
  refuses only once per continuing turn, which delegation-kit/SPEC.md §The turn-end liveness hook
  states.
- **Whitespace chaining.** `adjacent()` chains two members across a whitespace-only gap. No real
  hand list in the manifest set depends on that. The only site it reads as a list is ordinary prose
  ("a third cost surface" in a release post), which carries an exemption for exactly that.

## The seam

- **Kit mechanism:** the chaining rule, the new derived family and the not-a-set ruling. The
  family's members come from the binary's own compiled table, the same for every consumer, so no
  project vocabulary enters.
- **Consumer config:** none added. A consumer who configures its own `CANON_KIT_ENUM_SETS_CMD`
  keeps full control of its sets.
- **Private rule content:** none in reach.

## What changes

### (1) A list separator must carry a delimiter

`check-prose-enum` chains two members only when the gap between them holds a delimiter character
(comma, slash, bracket, parenthesis, colon, period, backtick or pipe) or the word `and` or `or`. A gap of
whitespace alone is prose, not a list {mechanical}.
**Not yet applied.**

- **Why this and not the two refused fixes.** Requiring the bracketed or backticked spelling for a
  common-word member would blind the gate to README.md's bare slash-joined tag algebra, which is
  where its attested drift lives. A per-member match mode is a third field in a two-field grammar
  the section refuses. This change touches neither: the member grammar and the set grammar stay as
  they are, and only the adjacency test narrows.
- **Why it is safe.** Every attested and live hand list is delimited: slash-joined, comma-joined,
  bracketed or backticked. A bare-word run of set members is the collision shape, and the survey
  found no real list relying on it.
- **Red condition, point 5.** The narrowing can only remove findings, and the gate reds only on a
  found list, never on finding none. The one site it clears is the release post, whose
  `prose-enum-exempt:` marker the build deletes in the same commit, so the gate verifies its own
  claim.
- The width caps stay as they are: 8 bytes for a punctuation gap, 16 for an `and`/`or` gap.

**Replacement text, canon-kit/SPEC.md §check-prose-enum** (**Not yet applied**). In the paragraph
opening "The scanned set and the paragraph walk", the sentence "Two or more members present count
as a *hand list* only when a run of them is chained by list separators (comma, slash, brackets,
whitespace, or `and`/`or`)" becomes:

> Two or more members present count as a *hand list* only when a run of them is chained by a
> separator carrying a delimiter (a comma, slash, bracket, parenthesis, colon, period, backtick or pipe) or
> the word `and`/`or`. Whitespace alone never chains, because two set members side by side in
> running prose are ordinary words ("a cost surface"), and a list always spells its separator.

### (2) The emitter declares the bundled KPI set

`--emit-enum-sets` adds one set, `kpi-builtin`: the ids in the binary's bundled KPI table,
referenced from that table and never re-listed {mechanical}.
**Not yet applied.**

- **Source.** `crate::emit::kpi::BUILTINS`' names, read in process on the `tag_lead_line::CLASSES`
  precedent, so a KPI added to the table enrols with no edit. The set name is not kit-rooted: the
  table is the binary's, not a kit root's layout.
- **Why this passes the family rule.** §check-prose-enum declares a derived set only when prose
  rosters the shape it reads. The bundled KPIs are rostered, by drift-kit/README.md and by the
  §Bundled KPIs section. They are not an incidental relation prose discusses.
- **Consumer-registered KPIs** stay outside. They live in a consumer's `kpis.list` and a consumer
  plugin directory. A consumer rostering its own KPIs declares its own set.

**Replacement text, canon-kit/SPEC.md §check-prose-enum** (**Not yet applied**). The paragraph opening
"The same emitter adds two roster families over the kit tree" gains a closing sentence:

> One set is not kit-rooted: `kpi-builtin`, the ids of the binary's bundled KPI table
> (drift-kit/SPEC.md §Bundled KPIs), referenced from that table the way the tag vocabulary is
> referenced from `check-tag-lead-line`'s.

### (3) What a README may enumerate, and what it must cite

§check-prose-enum records which enumerations are sets. A set held in a registry or a compiled
table is declared. A set that exists only as a conditional's arms, or as the flag forms an arm
parses, is a **contract**, and prose cites the owning section rather than restating it
{design-bearing}.
**Not yet applied.**

- **The ground.** A declared set has to be readable without executing anything: a tracked listing,
  a registry file, or a table the emitter references. A verdict disjunction or an argv parser is
  none of these. An extractor written against one script is a maintained oracle for one site, and
  it breaks on the next refactor. That refactor has already happened: the shell `[[ ]]` the entry
  describes is now a Rust `matches!`.
- **The remedy is tiering, not a gate.** A README restating a behavioural contract duplicates a
  SPEC-owned rule, and the duplicate is what went stale. The delegation-kit instance was also
  imprecise, since it omits the once-per-continuing-turn clause. The README states what the hook
  does and points at the section that owns when it refuses.
- **The honest limit.** A `## Use` block rostering an arm's flag forms stays unguarded. It is fenced,
  it is the README's one invocation surface, and no registry holds the forms. Close's staleness
  review remains its detector. The count half of this class stays `check-measured-claim`'s.

**Replacement text, canon-kit/SPEC.md §check-prose-enum** (**Not yet applied**). A paragraph before
"Calibration follows the count gate's procedure":

> **A set is what a registry holds, and a contract is cited, not enumerated.** A derived set reads
> a tracked listing, a registry file or a compiled table. A value set that exists only as a
> conditional's arms or an argument parser's modes is behaviour. An extractor for it would be a
> per-site oracle that a refactor breaks. Prose names the behaviour and cites its owning section, so
> nothing restates the members. An invocation block rostering an arm's modes is the residue, held by
> review.

### (4) The landing fixes

Two READMEs and one post change in the commit that lands deltas 1 to 3 {mechanical}.
**Not yet applied.**

- **drift-kit/README.md**, the bundled-KPI paragraph: reshaped to name the ids, lead and lag, so
  `kpi-builtin` holds it complete. `kpi-stage-economics-lag` joins the lead list. The KPI table's
  id order is not the grouping, so the paragraph keeps its lead/lag split, and the two clauses
  together name all 14.
- **delegation-kit/README.md**, install step 5: "when that reading is `red`, `corrupt` or
  `unresolved`" becomes "when that reading is a refusal verdict", keeping the pointer to SPEC
  §The turn-end liveness hook.
- **docs/posts/2026-07-19-checkwright-v0-9-0.md**: the `prose-enum-exempt:` marker on the "cost
  surface" line is deleted, since delta 1 makes it unnecessary.
- `docs/drift-kit/`, `docs/delegation-kit/` and `docs/canon-kit/` mirrors regenerate.

### (5) Tests and declarations

Each change carries an executable statement {mechanical}.
**Not yet applied.**

- `canon-kit/gate-tests/check-prose-enum.test.sh`: two members separated by one space do not chain,
  the same two separated by a backtick pair or a slash do, and a declared set's bracketed and bare
  forms still match.
- `native/src/emit/enum_sets.rs` tests: `kpi-builtin` equals `kpi::BUILTINS`' names.
- `.workflow/release-declarations.md` gains a **Tightened gates** bullet for `check-prose-enum`. The
  bundled emitter declares `kpi-builtin`, so a consumer on the bundled command whose prose
  hand-lists bundled KPI ids by delimited run reds on an incomplete one. The whitespace narrowing
  only removes findings.

## Producers and consumers

- **Delta 1's adjacency rule.** Producer: none new; `adjacent()` changes. Consumer: the paragraph
  judge in `check-prose-enum`. Red condition: unchanged (a delimited run of 2+ members, members
  omitted, no exempt context). The narrowing is monotone, so point 5 clears by inspection, and it
  is stated anyway.
- **Delta 2's set.** Producer: `--emit-enum-sets`, which this repo configures as
  `CANON_KIT_ENUM_SETS_CMD`, so the set is live outside tests. Consumers: `check-prose-enum` at
  match time (member) and in the report (set name). Roster-holding readers of the new set name: none
  hold a roster of set names. Probe: `git grep -n "queue-task-tag"` over the tracked tree, for where
  a sibling set name is rostered.
- **Delta 2's cross-module reference.** `enum_sets.rs` imports `crate::emit::kpi`. Its `// spec:`
  directive names §check-prose-enum, and the module's `KNOBS` gains no knob, because `BUILTINS` is a
  compile-time table.
- **Delta 3's ruling** has no producer or consumer. It is a recorded boundary read by the next
  session to weigh declaring a set.
- **Point 6, members enumerable at authoring.** Delta 2 obliges every manifest paragraph that
  delimited-lists 2+ KPI ids. The probe found zero today, and after delta 4 there is one, the
  drift-kit README paragraph, whose satisfying value is all 14 ids. Delta 1's cleared site is the
  post line, and its satisfying value is the deleted marker.

Roster derivations, re-derived by the build:

- `CANON_KIT_KNOB_FILE=<copy> bash gate-sdk/bin/run-gates.sh --only check-prose-enum`, with the copy's
  `CANON_KIT_ENUM_SETS_CMD` emitting the bundled sets plus `kpi-builtin`, before the emitter lands
  (delta 2's zero-red claim).
- `git grep -n "prose-enum-exempt"` over the tracked tree, for exemptions delta 1 makes redundant
  (delta 4).

## Existing sections updated

- `canon-kit/SPEC.md` — §check-prose-enum: the hand-list sentence (delta 1), the roster-families
  paragraph (delta 2), and the new contract paragraph (delta 3).
- `native/src/gates/prose_enum.rs` — `adjacent()` (delta 1).
- `native/src/emit/enum_sets.rs` — the `kpi-builtin` set and its test (deltas 2 and 5).
- `canon-kit/gate-tests/check-prose-enum.test.sh` (delta 5).
- `drift-kit/README.md`, `delegation-kit/README.md`, `docs/posts/2026-07-19-checkwright-v0-9-0.md`
  (delta 4).
- `.workflow/release-declarations.md` (delta 5).
- `docs/canon-kit/SPEC.md`, `docs/drift-kit/README.md`, `docs/delegation-kit/README.md` — on-site
  mirrors (all deltas).

## Retired spellings

- None — no delta retires a name. The README's verdict literals are rephrased, not a governed
  spelling.

## Definition of Done

- [ ] **Causal completeness** — the adjacency rule, the `kpi-builtin` set and the contract ruling each
      have a named producer and reader.
- [ ] **Instruction surfaces: instruction only** — the gate's `help:` line is unchanged or carries the
      fix only.
- [ ] **Merged with no information lost** — each §check-prose-enum passage re-phrased, not appended.
- [ ] **Entries moved before the drain stage** — both queue entries move to Done at a stage before
      `LIFECYCLE_KIT_DRAIN_STAGE`, with the merge that deletes this file.
- [ ] **Amendment deleted** — this file removed on merge, and none remain at the root for this unit.
- [ ] **Roster re-derived** — the probes above re-run before the merge counts.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — a cross-component gap found while landing filed through the gap inbox.
