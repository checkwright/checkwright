# SPEC amendment: matching-windows

Three kit gates relate two things they matched across a distance that is a bare number in the module, with no knob. `check-knob-citation` binds a knob token to a following "default" marker within 100 code points (`MARKER_REACH`, `native/src/gates/knob_citation.rs`). `check-knob-default-coupling` looks for a knob's default statement within 400 code points after the knob's name in the owning SPEC, an inline literal no SPEC sentence states (`native/src/gates/knob_default_coupling.rs`). `check-assertion-strength` reads a guard's failure text within 8 lines after the call (`WINDOW`, `native/src/gates/assertion_strength.rs`). Each passes the discriminator in doctrine-kit's Policy-as-choice rule: another value keeps the gate's stated contract true, so an adopter whose prose or scripts sit at a different distance meets a red, or a miss, that they can only bypass.

**The ruling: each distance becomes a knob in its kit's table, and the default is the value it shipped with, so no adopter's verdict moves on upgrade.** Each knob admits `off` where the gate's other assertions survive without it. A structural value is offered where the grammar has one:

- `CANON_KIT_KNOB_CITATION_REACH`: a code-point count, default `100`; `sentence`; or `off`, which stops the default-marker shape and keeps the `=` shape.
- `CANON_KIT_DEFAULT_COUPLING_WINDOW`: a code-point count, default `400`; or `off`, which stops assertion 2 (SPEC agreement) and keeps assertion 1 (source self-agreement).
- `GATE_SDK_ASSERTION_STRENGTH_WINDOW`: a line count, default `8`. It has no `off`, because the window bounds the gate's only assertion, and turning that off is unregistering the gate, which the registry already offers.

This repo binds each knob to its shipped value in its own knob files.

**Why a structural value for one gate only.** `check-knob-citation` has a sentence to bind to: a stated default reads knob first, then value, and the sentence terminator is one the kit already defines, the `.`, `?`, `!` or `;` before whitespace that §check-measured-claim's inline marker binds by. For the coupling window, the paragraph was weighed and refused. The gate reads each SPEC as one blob so that a wrapped default statement still binds its knob, and a kit SPEC that holds its whole knob roster in one paragraph (gate-sdk/SPEC.md §Layout and configuration, one line of 14,977 code points on this tree) would get a bound much wider than the count. For assertion-strength, the guard's own body was weighed and refused. Its end is shell block grammar (`fi`, a closing brace), and the gate parses none of it: it reads guard shape line by line, and its truncation at the next declaring call already bounds the window structurally from the far side.

**Why this repo binds the shipped values.** `100` is the reach this tree was measured at when its markdown was unwrapped. `400` and `8` move no verdict here: `check-assertion-strength` reaches zero calls on this tree (its clean line reads `0 call(s) to a script with a declared exit contract`), and the coupling gate is green at `400`. `sentence` has not been measured on this tree, and switching to it would be a verdict change that this unit does not take.

**Measured at authoring (2026-09-22).** All three gates are clean on this tree at `88a8df68`: `check-knob-citation` over 93 manifest files, `check-knob-default-coupling` over 133 literal default sites, and `check-assertion-strength` over 119 scripts with zero declaring calls. The pre-commit hook already triggers each gate on its kit's knob file (`scripts/canon-config.knobs` for the two canon gates, `scripts/gate-sdk-config.knobs` for all three), because each already declares a knob of that kit. So the new declarations add no trigger, and the hook needs no regeneration.

## What changes

### (1) The knob `CANON_KIT_KNOB_CITATION_REACH` {design-bearing}

**Not yet applied.** canon-kit's table (`native/src/knobs/canon_kit.rs`) gains the scalar `CANON_KIT_KNOB_CITATION_REACH`, default `100`. Its validator refuses any value other than a positive integer, `sentence` or `off`, at exit 2 under the kit's malformed-config lead line.

`check-knob-citation` reads it once per run, and the `MARKER_REACH` constant is deleted:

- a positive integer `n`: a "default" marker binds the token it follows when at most `n` code points separate them. This is today's leg.
- `sentence`: the marker binds when no sentence terminator (`.`, `?`, `!` or `;` followed by whitespace) sits between the token and the marker. The terminator predicate is the one `spec::bound_sentence` applies, a private function today, so it is made crate-visible and shared rather than copied.
- `off`: the default-marker shape is not read. The `=` shape still fires, and the clean line says the default leg is off, so it never reads as coverage of that shape.

The `check-knob-citation` registry entry in `native/src/gates/mod.rs` declares the knob. The unit test `a_default_marker_binds_only_the_knob_it_follows_within_reach` keeps its three assertions at the integer reach and gains the `sentence` and `off` readings of the same lines.

Fixtures: the `good/`+`bad/` pair keeps its config, so it runs at the default. `canon-kit/gate-tests/check-knob-citation.test.sh` gains cases over one line where a knob token and a stated default sit 150 code points apart inside one sentence: clean at the default, red at `200`, red at `sentence`, clean at `sentence` once a sentence terminator sits between them, and clean at `off`. It also gains an `off` case where a `<KNOB>=<value>` statement still reds, and a case where an unknown value exits 2.

### (2) §check-knob-citation states the reach as the knob's {mechanical}

**Not yet applied.** In canon-kit/SPEC.md §check-knob-citation, the two sentences beginning "The marker is one of two shapes" and ending "a stated default reads knob first, then value." become:

> The marker is one of two shapes: `=` appended directly to the token (`<KIT>_<KNOB>=<value>`), or the word "default" following the token within the reach `CANON_KIT_KNOB_CITATION_REACH` sets and bound within a short window to a value literal (a backticked value, a quoted string, or a number). A marker before the token binds nothing, since a stated default reads knob first, then value. The reach defaults to `100` code points. It may instead be any positive integer, or `sentence`, which binds a marker only when no sentence terminator (`.`, `?`, `!` or `;` before whitespace) sits between it and the token, or `off`, which stops reading the default shape so that only the `=` shape fires, and the clean line says so. Any other value exits 2. The reach is the consumer's to set, because the right distance depends on the tree. On a wrapped tree the line already bounds the leg. On an unwrapped tree a paragraph is one line, and an unbounded leg would bind a knob to another knob's default further along it. The default is the reach the leg shipped with, so an upgrade moves no verdict. `sentence` is the structural bound, with the honest limit that an abbreviation's period ends a sentence early.

The paragraph beginning "Two calibrations hold the false-positive rate" is unchanged.

### (3) The knob `CANON_KIT_DEFAULT_COUPLING_WINDOW` {design-bearing}

**Not yet applied.** canon-kit's table gains the scalar `CANON_KIT_DEFAULT_COUPLING_WINDOW`, default `400`. Its validator refuses any value other than a positive integer or `off`, at exit 2.

`check-knob-default-coupling` reads it once per run:

- a positive integer `n`: `spec_verdict`'s window runs `n` code points past the end of the knob name, which replaces the inline `400`. This is today's window.
- `off`: assertion 2 is not run and no SPEC is opened. Assertion 1 still reds a knob whose source sites disagree. The clean line reports the SPEC comparison as not run, never as agreement.

`spec_verdict` takes the window as a parameter, so the unit tests' `found` helper passes it. A new unit test holds that a default stated just inside the window is `Found` and one just outside is not. The registry entry declares the knob beside the four it declares. Its `dynamic@…:<line>` label names the kit-root walk call, so it moves to that call's new line if the edit shifts it.

Fixtures: the pair keeps its config. `canon-kit/gate-tests/check-knob-default-coupling.test.sh` gains cases over a sandbox kit whose SPEC states a knob's default 450 code points after the knob's name: red at the default, naming the knob as stated nowhere, and clean at `600`. At `off`, a SPEC stating a different default is clean while two disagreeing source sites still red. A case where an unknown value exits 2 is added too.

### (4) §check-knob-default-coupling states the window {mechanical}

**Not yet applied.** In canon-kit/SPEC.md §check-knob-default-coupling, the sentence "Assertion 2 (SPEC agreement): the owning SPEC states that same literal as the knob's default, read through the default-statement grammar `check-knob-citation` shares (§The shared spec adapters)." becomes:

> Assertion 2 (SPEC agreement): the owning SPEC states that same literal as the knob's default, read through the default-statement grammar `check-knob-citation` shares (§The shared spec adapters), within `CANON_KIT_DEFAULT_COUPLING_WINDOW` code points after a mention of the knob's name. The window defaults to `400` and may be any positive integer, or `off`, which skips assertion 2 and keeps assertion 1. That suits a consumer whose SPECs state defaults in a shape this grammar does not read, and the clean line then reports the comparison as not run. Any other value exits 2. The window is the consumer's to set: a SPEC that states each default beside its knob can hold it tight, while one that describes a knob at length first needs it wider, and every value keeps the assertion a comparison between two sites. A paragraph bound is refused, because the SPEC is read as one blob so that a wrapped statement still binds, and a SPEC that keeps its knob roster in one paragraph would get a bound wider than the count.

### (5) The knob `GATE_SDK_ASSERTION_STRENGTH_WINDOW` {design-bearing}

**Not yet applied.** gate-sdk's table (`native/src/knobs/gate_sdk.rs`) gains the scalar `GATE_SDK_ASSERTION_STRENGTH_WINDOW`, default `8`. gate-sdk's table carries no validator, so the member validates the value when it reads it: anything other than a positive integer exits 2 with the knob named, the shape `check-provenance-seam` uses for its slug floor. `check-assertion-strength` reads it once per run and replaces `WINDOW` with it; the constant is deleted. The registry entry declares the knob beside `GATE_SDK_KIT_DIRS`, with the same `dynamic@` label rule as delta 3.

Fixtures: the pair keeps its config. A new `gate-sdk/gate-tests/check-assertion-strength.test.sh`, sourcing `gate-sdk/lib/test-hermetic.sh` as its siblings do, runs over a sandbox kit whose `bin/verdict.sh` declares an `# exit:` token and whose `smoke/` script guards a call with a bare `if`, with the token-naming message 10 lines below the call. The run is clean at the default, red at `12`, and exits 2 at `0` and at a non-integer.

### (6) gate-sdk/SPEC.md states the window {mechanical}

**Not yet applied.** Two edits.

In §Layout and configuration's environment-override roster, after the `GATE_SDK_ENFORCE_SCAN_DIR` entry, add:

> `GATE_SDK_ASSERTION_STRENGTH_WINDOW` (default `8`; the lines after a guarded call that §check-assertion-strength reads for failure text — see there),

In §check-assertion-strength, the **Detection** paragraph's "within a bounded window" becomes "within `GATE_SDK_ASSERTION_STRENGTH_WINDOW` lines after the call". The section's last sentence, "Configuration adds **no new knob**: the scan roots come from the existing kit-roots derivation, as `check-test-hermetic`'s do.", becomes:

> The scan roots come from the kit-roots derivation, as `check-test-hermetic`'s do. The window is the gate's one knob, because how far a guard's message sits below its call is a script's style and not the contract. It takes no `off`: the window bounds the gate's only assertion, so a consumer wanting none unregisters the gate. The guard's own body was weighed as a structural bound and refused, because the gate reads guard shape line by line and parses no shell block, and the next-call truncation already bounds the window from the far side.

### (7) This repo binds its three choices {mechanical}

**Not yet applied.** `scripts/canon-config.knobs` gains `CANON_KIT_KNOB_CITATION_REACH = 100` and `CANON_KIT_DEFAULT_COUPLING_WINDOW = 400`, and `scripts/gate-sdk-config.knobs` gains `GATE_SDK_ASSERTION_STRENGTH_WINDOW = 8`. Each line is led by a `# spec:` pointer to its gate's section. The preamble gives the grounds, which are this repo's and stay out of the kit SPEC.

The header of `scripts/gate-sdk-config.knobs` says "set only what this repo overrides beyond the kit defaults", which a binding equal to the default contradicts. It is reworded to "set what this repo overrides beyond the kit defaults, and the calibrations it binds". The census filed in the gap inbox (this repo's policy knobs inherited by default) stays open, since it asks the question for every other policy knob.

gate-sdk ships no knob-file template, and `canon-kit/templates/canon-config.knobs` is comment-only and lists no knob, so neither template changes.

### (8) The release declaration {mechanical}

**Not yet applied.** `.workflow/release-declarations.md` §Behavior changes gains one bullet per gate. Each names its new knob, says the default is the value the gate shipped with so no verdict moves, and names the alternatives (`sentence` and `off` for the citation reach, `off` for the coupling window, none for the assertion-strength window). No bullet states a knob beside its default literal: the composed note publishes under `docs/posts/`, a manifest glob this repo configures, where `check-knob-citation` reds a knob stated with its value outside its owning SPEC.

## Producers and consumers

- **The three values** (deltas 1, 3 and 5). Producer: each kit's table, overridden by a consumer's knob file; this repo sets all three (delta 7). Consumers: `check-knob-citation`, at the per-line marker binding; `check-knob-default-coupling`, at each knob's SPEC read; `check-assertion-strength`, at each declaring call's window. Each is read once per run. The canon validator reads its two at the kit's first resolution, and `check-assertion-strength` validates its own.
- **Point 2, roster readers of the minted names.** The registry declarations in `native/src/gates/mod.rs` (a knob a member reads undeclared is a finding under the crate's declaration tests). `check-knob-default-coupling` couples each new scalar row's default to its SPEC statement, which is why deltas 2, 4 and 6 state each default as a literal. `check-kit-ref-liveness` resolves each name to its table row. `--emit knob-roster` and `check-docs-cmd`'s static knob names derive from the tables and need no edit. The knob-file couple is derived (gate-sdk/SPEC.md §The `# graph:` manifest), and the pre-commit hook already carries it for all three gates.
- **The clean lines under `off`** (deltas 1 and 3). Consumer: the committing operator through the output contract. Each says what was not read, so an `off` run never prints the full-coverage sentence.
- **Point 5.** A smaller value narrows what a marker, a SPEC read or a guard window reaches, and only for the citation reach is that monotone: fewer bindings can only remove reds. A smaller coupling window can add reds, since a default that falls outside it reds as stated nowhere. A smaller assertion-strength window can add them too, because the window that holds a guard's message also holds the status comparison that excuses it, and a comparison cut off leaves the message unexcused. `sentence` can add or remove citation reds. `off` removes the reds of the leg it stops. None of this moves a verdict here, because this repo binds the shipped values.
- **Point 6.** Not reached. No delta obliges each member of a corpus.

## Existing sections updated

Roster from `git grep -n -E "MARKER_REACH|WINDOW|100 code points|bounded window|no new knob" -- canon-kit/SPEC.md gate-sdk/SPEC.md native/src/gates/knob_citation.rs native/src/gates/knob_default_coupling.rs native/src/gates/assertion_strength.rs`, and `git grep -n -E "check-knob-citation|check-knob-default-coupling|check-assertion-strength" -- ':!docs/' ':!TASK-QUEUE.md' ':!*/gate-tests/*'`, run 2026-09-22 and read for surfaces stating any of the three distances.

- `native/src/knobs/canon_kit.rs`, two table rows and their validator arms (deltas 1 and 3).
- `native/src/knobs/gate_sdk.rs`, one table row (delta 5).
- `native/src/gates/knob_citation.rs`, the reach and its unit test (delta 1).
- `native/src/gates/knob_default_coupling.rs`, the window parameter and its unit tests (delta 3).
- `native/src/gates/assertion_strength.rs`, the window read and its validation (delta 5).
- `native/src/gates/mod.rs`, three registry declarations (deltas 1, 3 and 5).
- `canon-kit/gate-tests/check-knob-citation.test.sh`, the reach cases (delta 1).
- `canon-kit/gate-tests/check-knob-default-coupling.test.sh`, the window cases (delta 3).
- `gate-sdk/gate-tests/check-assertion-strength.test.sh`, new (delta 5).
- `canon-kit/SPEC.md` §check-knob-citation (delta 2) and §check-knob-default-coupling (delta 4).
- `gate-sdk/SPEC.md` §Layout and configuration and §check-assertion-strength (delta 6).
- `scripts/canon-config.knobs` and `scripts/gate-sdk-config.knobs`, the bindings and the header rewording (delta 7).
- `.workflow/release-declarations.md` (delta 8).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/canon-kit/SPEC.md`.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`.

## Retired spellings

- `MARKER_REACH` — the constant deleted by delta 1.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the three knobs.
- [ ] **Instruction surfaces: instruction only.** Not reached; no template or shim carries grounds.
- [ ] **Merged with no information lost.** Each SPEC sentence named is re-phrased in place, not appended to.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md` at the root).
- [ ] **Entry moved.** `baked-matching-windows` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` is green on the declared spelling.
- [ ] **Gaps filed.** Any gap found during the work is filed to the gap inbox.
