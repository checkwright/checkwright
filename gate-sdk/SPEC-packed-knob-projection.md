# SPEC amendment: packed-knob-projection

Queue entry: `packed-knob-projection-filter-form`, selected for `couples-field-semantics` as its
projection unit (operator direction, 2026-09-15, lead-relayed), to be reconciled against the
packed-knob refusal. It merges after `SPEC-couples-semantics.md`, whose `knob:` covering conversion
the projection token rides.

## The question

`check-spec-embedded-source`'s source-candidate walk (`spec_embedded_source.rs:147`, a
`find_with_prune` over `.` filtered by basename) selects by the file globs packed into the third field
of `CANON_KIT_EMBED_LANGS`' `kind|fence-langs|file-globs` elements. The filter field reaches a knob's
value, and a keyed knob's values. It does not reach a projection out of a packed element, so the
member declares `?` under the `projection` ground class, the one admitted exception to the refusal of
`?` on a statically resolvable root (gate-sdk/SPEC.md §check-reads-couples).
§gen-pre-commit rules such a knob **not a pattern set**, and so it takes no couples token.

**Probed at this stage.**

- **`CANON_KIT_EMBED_LANGS` is the only packed knob in the static tables.** A grep of
  `native/src/knobs/*.rs` for multi-`|` literal defaults finds its nine rows. The other hits are
  regex alternations and derived-path formats.
- **The member's descriptor transcribes the knob's default.** Its couples carry
  `*.rs,*.toml,*.sql,*.rego,*.ts,*.tsx,*.yaml,*.yml,*.proto,*.sh,Dockerfile` beside its spec tokens.
  That is a second spelling of a knob default, which de-literalization forbids, and it is wrong for
  any consumer who adds a language: that consumer's new source files are scanned and never trigger
  the gate.
- **Two parsers would exist the moment a form is bolted on.** The member's `langs()` splits each
  element with `splitn(3, '|')` and each field on `,`. An index-addressed filter form such as
  `knob:CANON_KIT_EMBED_LANGS#3` would be a second parser in the reader, and a transcription of the
  element's field order into every declaration.

## The ruling

**The knob's owning row declares its element packing as named fields, and a declared field is a
pattern set.** The refusal stands as written for the knob itself: a packed element is not a pattern,
so a bare `knob:CANON_KIT_EMBED_LANGS` token or filter stays refused. What becomes addressable is a
**named projection**, `CANON_KIT_EMBED_LANGS.file-globs`: the flattened members of one declared field
across every element, read through the one parser the member itself uses.

Two forms were weighed and refused, and neither needs a gate:

- **An index-addressed projection (`NAME#3`)** puts the packing layout in every token. Reordering the
  element's fields would then rot each token silently.
- **A derived companion knob (`CANON_KIT_EMBED_FILE_GLOBS`)** is settable independently of the packed
  knob it derives from. A consumer who set one and not the other would hand the walk and the coverage
  reader two corpora: a second source rather than a projection.

## The seam

Kit mechanism only: a packing declaration on a knob-table row, one parser, a token suffix, and the
member's declaration and descriptor. The consumer's `CANON_KIT_EMBED_LANGS` value stays consumer
config and gains nothing to write. A consumer's added language reaches the trigger and the coverage
demand with no descriptor edit, which is the point. No private rule content.

## What changes

### (1) A row declares its element packing, and one parser reads it {design-bearing}

`native/src/knobs/`. An indexed row may declare the packing of its elements:
- its **field separator**;
- an ordered list of **field names**;
- for each field, whether it is a **list** and its **list separator**.

`CANON_KIT_EMBED_LANGS`' row declares `kind`, `fence-langs` (a `,`-list) and `file-globs` (a
`,`-list), separated by `|`, the names the queue entry already gives the triple's fields.

`knobs` gains two readers:
- `unpack(NAME)`, which returns each element's fields by name;
- `project(NAME, FIELD)`, which flattens one field's members across elements, in element order.

Each refuses (a `Result` error, so exit 2 at every caller) a name whose row declares no packing and
a field its packing does not name. An element with fewer fields reads its missing fields as empty,
exactly as `langs()`' `splitn` does today, so no consumer value changes verdict. `spec_embedded_source::langs()`
reads its three fields through `unpack` rather than `splitn`, so the walk and every reader of the
projection share one parser. The names `unpack` / `project` and the packing type's shape are build's
calibration.

### (2) The projection token, in both fields {design-bearing}

A knob reference in a `--reads` filter source or a `knob:` couples token may be `<NAME>.<field>`. The
first `.` splits it, which is unambiguous because static knob names carry no `.`. A reference
carrying one resolves through `knobs::project`, and a bare reference resolves as today. Every reader
that turns a knob reference into a name strips the suffix first:

- **`registry::expand_couples`** expands `knob:<NAME>.<field>` to `project`'s members, then applies
  `SPEC-couples-semantics.md` delta 4's covering conversion. So `*.rs` becomes `**.rs` and
  `Dockerfile` becomes `*Dockerfile`.
- **`check-reads-couples`' `resolve_filter`** resolves a projected filter through `project`, and fails closed on its refusals as on an unresolvable knob.
- **`check-graph`'s `knob:` admissibility loop** (`graph.rs`, *carries couples token 'knob:…', but …
  declares no such knob*) checks the bare name against the member's declared set, and that the row
  declares the field.
- **`gates::filter_knob`**, the `EVERY_FILTER_KNOB` union, `registry::couples_knob_names` and
  `registry::knob_files`' couples-knob sentinel reach carry and resolve the bare name.

`check-graph`'s `valid_glob_token` needs no widening: after `knob:` is stripped, `.` and the field
name's characters are already in its class. §The `# graph:` manifest's rule, that a third prefix is
widened only in the delta landing its resolver, is not engaged, because no prefix is added.

gate-sdk/SPEC.md §The `# graph:` manifest, the `knob:` rule. **Not yet applied**, a paragraph after
**Resolution order is fixed**:

> **A packed knob is addressed by a declared field, never whole.** Where a knob's row declares its
> element packing, `knob:<NAME>.<field>` names one field's members across its elements, read through
> the same parser the member's walk uses; a bare token on a packed knob is refused, since an element
> is not a pattern. The field is the row's to declare, so a reordered packing moves no token.

### (3) The member declares its root, and `projection` retires {design-bearing}

`native/src/gates/mod.rs`, `check-spec-embedded-source`'s entry. The
`("?", "", "", "projection@src/gates/spec_embedded_source.rs:147")` declaration becomes
`(".", "name:knob:CANON_KIT_EMBED_LANGS.file-globs", "", "")`. The kind is `name:`, because the walk
matches each glob against the basename with `walk::pattern_match`. There is no prune field: the walk
applies the global prune set, which a registry root already takes. Its comment, *projection out of
`CANON_KIT_EMBED_LANGS`' packed elements…*, becomes one binding to the projection form.
`GROUND_CLASSES` drops `"projection"` and the refusal message drops it. `SPEC-fallback-roots.md`
drops `"fallback"` from the same constant, so whichever lands second edits a one-member list.

gate-sdk/SPEC.md. **Not yet applied:**

- **§check-reads-couples, the class list.** The `projection` bullet and the paragraph
  **`projection` is the one admitted exception** are deleted.
- **§check-reads-couples, the refused alternatives.** In **Two alternatives are refused**, *The
  `projection` ground class above is the one admission against this refusal, and it reaches no further
  than a filter the field cannot express.* is deleted. The refusal of `?` on a statically resolvable
  root now has no admitted exception.
- **§check-reads-couples, the resolution facts.** In **Three resolution facts are contract**, *and it
  does not reach a knob whose elements *pack* several fields, whose pattern is a bespoke projection of
  a value rather than the value* becomes *and a knob whose elements pack several fields is reached
  through a declared field (§The `# graph:` manifest)*.
- **§gen-pre-commit.** In **An emitted trigger set is knob-derived wherever a walk is**, *a knob
  whose elements **pack several fields** is not a pattern set, so a walk filtered by a projection out
  of one (`CANON_KIT_EMBED_LANGS`' `kind|fence-langs|file-globs` triples) is outside the form and takes
  no token* becomes *a knob whose elements **pack several fields** is not a pattern set and takes no
  bare token, while a walk filtered by one of its declared fields carries `knob:<NAME>.<field>`,
  which is a pattern set*.
- **§check-reads-couples, the census.** The ground partition's live classes and the figures are
  re-measured at landing (shared with `SPEC-fallback-roots.md` delta 2).

### (4) The descriptor stops transcribing the knob's default {mechanical}

`canon-kit/checks/check-spec-embedded-source.gate`. The literal run
`*.rs,*.toml,*.sql,*.rego,*.ts,*.tsx,*.yaml,*.yml,*.proto,*.sh,Dockerfile` is replaced by
`knob:CANON_KIT_EMBED_LANGS.file-globs`. Its spec tokens stay. The generated hook and the
coupling-graph artifact are regenerated.

canon-kit/SPEC.md §Layout and configuration, the `CANON_KIT_EMBED_LANGS` bullet. **Not yet applied:**
*one `kind|fence-alias,…|file-glob,…` entry per language family* becomes *one
`kind|fence-langs|file-globs` entry per language family, the two list fields `,`-separated — field
names the row declares, so a descriptor couples the candidate globs as
`knob:CANON_KIT_EMBED_LANGS.file-globs` (gate-sdk/SPEC.md §The `# graph:` manifest)*.

## Producers and consumers

- **The packing declaration (delta 1).**
  - **Producer:** the static table row, compiled in.
  - **Consumers:** `knobs::unpack` / `project`. Their callers are `spec_embedded_source::langs()` at
    rule start, `registry::expand_couples` at every trigger read, `check-reads-couples`'
    `resolve_filter` at the per-root coverage loop, and `check-graph`'s admissibility loop.
  - **Enabling configuration:** none beyond the row, which every tree carries, including one with no
    knob file.
- **Every field has a reader.**
  - The **field separator** and each **list separator** are read by `unpack`.
  - The **field names** are read by `project`'s field lookup and by `langs()`'s by-name access.
  - A declared field no reader names would be removed, and `fence-langs` is read by `langs()`'s alias
    map.
- **The projection token (delta 2).**
  - **Producer:** a descriptor author or a registry declaration.
  - **Consumers:** the four readers delta 2 lists, each at its existing transition.
- **Readers' red conditions (point 5).** Delta 4 replaces literals with a knob token. For this
  tree's default value that is the same pattern set, converted to covering patterns, so the trigger
  set widens only by the conversion. The readers that turn red:
  - `check-graph` assertion D and E, until the hook and artifact are regenerated.
  - `check-reads-couples`, on this tree, only if a tracked file the projection selects has no
    covering token. The converted `**.<ext>` tokens contain every such basename, so none is expected,
    and build reads the verdict.
  - Unit test A, if a fixture case's observed root is not `.` or a case positional. The member's
    root is its first positional, which a relocating case already supplies.
  - The locator assertion, whose site count drops by one. A seeded-locator test seeding a
    `projection` ground is re-seeded with a live class.
  - The literal `rs` … `Dockerfile` run in the descriptor is the default's copy. A consumer whose
    value adds a language now triggers on its files, which widens the trigger set and turns nothing
    red.

## Existing sections updated

Roster probe: `git grep -n "projection@\|packed\|pack several\|EMBED_LANGS"` over `gate-sdk/SPEC.md`,
`canon-kit/SPEC.md`, `native/src` and the member's descriptor, run at this stage.

- `native/src/knobs/canon_kit.rs` — the row's packing declaration (delta 1).
- `native/src/knobs/mod.rs` — the packing type, `unpack` and `project` (delta 1).
- `native/src/gates/spec_embedded_source.rs` — `langs()` through `unpack` (delta 1).
- `native/src/registry.rs` — projection expansion, name stripping for the sentinel and knob files
  (delta 2).
- `native/src/gates/reads_couples.rs` — projected filter resolution (delta 2).
- `native/src/gates/graph.rs` — admissibility over the bare name and declared field (delta 2).
- `native/src/gates/mod.rs` — `filter_knob`, the filter-knob union, the member's declaration and
  comment, `GROUND_CLASSES`, the refusal message and any seeded `projection` ground (deltas 2 and 3).
- `gate-sdk/SPEC.md` — §The `# graph:` manifest's `knob:` rule (delta 2), §check-reads-couples and
  §gen-pre-commit (delta 3).
- `canon-kit/checks/check-spec-embedded-source.gate` — the knob token (delta 4).
- `canon-kit/SPEC.md` — §Layout and configuration's `CANON_KIT_EMBED_LANGS` bullet (delta 4).
- `scripts/git-hooks/pre-commit` — regenerated (delta 4).
- `docs/check-graph.html` — regenerated (delta 4).
- `docs/gate-sdk/SPEC.md`, `docs/canon-kit/SPEC.md` — generated mirrors, regenerated (all deltas).

## Retired spellings

- `projection@src/gates/spec_embedded_source.rs` — the member's `?` ground (delta 3).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and
      a named consumer; every new field has a named reader at a named transition.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition
      or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); the merged spec reads as one coherent document a reader who never saw
      the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings`
      above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked
      tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
