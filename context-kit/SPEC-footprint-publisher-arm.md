# SPEC amendment: footprint-publisher-arm

**The footprint arm is withheld from the adopter's tool list as the publisher's own tool, and no knob makes it an adopter tool.** The queue entry asks for one of two dispositions. This amendment picks withholding, the posture §check-footprint-fresh already takes for the gate over the same page.

**Measured at authoring.** The binary at `native/target/release/checkwright-gates` ran `--emit-footprint` in an adopter-shaped scratch tree. That tree held a `CLAUDE.md` with a `context-kit` marker block, a vendored `context-kit/templates/` with no `SPEC.md`, and an adopter directory carrying its own `SPEC.md`. The arm printed this site's front matter and title. It gave the adopter's directory a row and the vendored kit none, with both totals an em dash, and exited 0.

**Why not the knob disposition.** Making the arm an adopter tool takes a roster knob and a page-head knob. It would buy an adopter two things they already hold. One is the load-triggered figures the published page prints, since a kit's templates ship byte for byte. The other is an always-loaded share the meter already counts in the adopter's live total (§The always-loaded meter).

**Read at authoring.** The arm's callers, from `git grep -n 'footprint::\|--emit footprint\|--emit-footprint' -- native/src` with each hit read:

- `native/src/gates/footprint_fresh.rs` calls `emit()` in process. It is the publisher's gate, declared in `scripts/`.
- `native/src/emit/value_rollup.rs` calls `measure()` in process. Its gate `check-value-rollup-fresh` is declared in `scripts/` too.
- `native/src/emit/agents_md_smoke.rs` spawns `--emit footprint` in a source-clone scratch consumer and asserts a non-zero total. That scratch is built from the kit sources, so it carries what the roster derives from.
- `native/src/emit/mod.rs` holds the arm-table row, and `native/src/gates/mod.rs` the two gates' registry rows.
- `native/src/gates/door_binding.rs` carries the spelling as a test literal only.

No caller changes, and the arm stays in the binary for the two in-process callers. That a kit's templates ship byte for byte was read off the packer: `native/src/emit/pack_installer.rs` rewrites a packed kit's `README.md` and no other file.

## What changes

### (1) context-kit/SPEC.md states the arm is the publisher's {mechanical}

**Not yet applied.**

§bin/footprint: the opening paragraph, "The footprint emitter publishes the kits' measured context footprint — the adoption-cost evidence a consumer weighs before vendoring, the concrete form of the token-economics positioning. Where the meter reads one consumer's live always-loaded total, this reads the tracked kit surfaces and attributes the cost per kit, split by when it lands.", becomes these two paragraphs:

> The footprint emitter publishes the kits' measured context footprint, attributed per kit and split by when the cost lands: the adoption-cost evidence a consumer weighs before vendoring, the concrete form of the token-economics positioning. Where the meter reads one consumer's live always-loaded total, this reads the tracked kit surfaces.
>
> **It is the publisher's own arm, and no adopter-facing surface lists it.** It prints the publisher's page, front matter and regeneration command included, over a roster derived from the `SPEC.md` files the payload withholds (gate-sdk/SPEC.md §Consumer payload). In an adopter's tree it gives no vendored kit a row, and gives one to any directory of theirs carrying a `SPEC.md`. The arm stays in the binary because §check-footprint-fresh and the value rollup call its library function in process. An adopter reads its own context cost off the meter. Knobs for the page head and the roster are refused: they would buy an adopter load-triggered figures the published page already prints, a kit's templates shipping byte for byte, and an always-loaded share the meter already counts.

§check-footprint-fresh: the opening paragraph, "**It is the publisher's own gate and does not ship.** Its subject is the publisher's footprint page, and its kit roster derives from `*/SPEC.md` files the payload withholds (gate-sdk/SPEC.md §Consumer payload), so an adopter's tree holds nothing it could measure. The descriptor and its fixture pair sit in the publisher's gates dir; the rule stays in the crate.", becomes:

> **It is the publisher's own gate and does not ship**, on §bin/footprint's ground: its subject is that arm's page. The descriptor and its fixture pair sit in the publisher's gates dir; the rule stays in the crate.

**Both acts, by the sibling's landing.** [context-kit-tail-brevity](../TASK-QUEUE.md#context-kit-tail-brevity) passes both sections and is ordered after this unit. Where it has not landed, replace the quoted paragraphs as written. Where it has already rewritten either section, the act is positional: the publisher paragraph follows the section's first paragraph in §bin/footprint, and §check-footprint-fresh's first paragraph takes the replacement above.

`check-surface-ratchet` holds `context-kit/SPEC.md` at its committed ceiling, and the delta grows the file past it. Re-stamp with `--emit always-loaded --ceiling` and commit `.workflow/surface-ceiling.txt` with the growth (§The surface ratchet).

### (2) context-kit/README.md drops the arm from the adopter's tools {mechanical} {user-facing: the queue entry's Deliverable, which names withholding the arm from the README as one of two dispositions and leaves the choice to this stage}

**Not yet applied.**

- The opening paragraph's list loses "a per-kit token-footprint projection, ", so it reads "… one gate over its governed always-loaded sections, a close-stage brevity pass that reacts to the meter's delta, …".
- The `## Use` fence loses its `"$gates" --emit footprint` line.

### (3) The site mirrors follow {mechanical} {user-facing: as delta 2, on the published copy of the README}

**Not yet applied.**

`docs/context-kit/SPEC.md` and `docs/context-kit/README.md` are regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`, in the commit landing deltas 1 and 2.

## Producers and consumers

- **No new state, event, interface, knob or name.** The arm's behaviour, its output and its callers are unchanged, so no adopter's script or battery moves.
- **Readers whose verdict moves, and their red conditions.**
  - `check-docs-mirror-fresh` reds on a byte diff between a source and its mirror. Delta 3 regenerates both mirrors in the commit that edits their sources.
  - `check-surface-ratchet` reds on `context-kit/SPEC.md` above its ceiling row. Delta 1 names the re-stamp.
  - `check-prose-bounds` counts findings in `context-kit/SPEC.md` against its committed ceiling row, and `check-provenance-seam` reads the file for publisher provenance.
  - Measured at authoring with delta 1 applied to the working tree and then reverted: `check-prose-bounds` and `check-provenance-seam` passed, and `check-surface-ratchet` went red at 124941cp over its 124315cp row.
  - No reader reds on finding none, and none asserts a count over the README's arm list. The `## Use` fence keeps its other lines, and `git grep -n '## Use' -- native/src` finds no reader of the section.
- **No release declaration is owed.** `check-release-change-declared` reads a removed kit `bin/` tool and a changed `init`-claimed template. A README edit is neither, and the arm itself is not removed.
- **Definition of Done move.** The entry moves to Done in the build commit that lands deltas 1 to 3 and deletes this file, which is before the drain stage. No remote oracle is owed.

## Existing sections updated

Roster produced by `git grep -n -i 'footprint' -- context-kit docs/context-kit README.md docs/kits.md`, with each hit read.

- `context-kit/SPEC.md` — §bin/footprint, the opening paragraph; §check-footprint-fresh, the opening paragraph (delta 1).
- `context-kit/README.md` — the opening paragraph and the `## Use` fence (delta 2).
- `docs/context-kit/SPEC.md`, `docs/context-kit/README.md` — the regenerated mirrors (all deltas).

Every other hit was read and left, each true of the publisher's arm: the remaining `context-kit/SPEC.md` mentions (§The consumer footprint's pointer to the published page, the `CONTEXT_KIT_SURFACES` and `CONTEXT_KIT_FOOTPRINT_FILE` rows, §Testing's emitter and smoke passages, §bin/footprint's later paragraphs) and `docs/context-kit/index.md`'s "consumer footprint", which names §The consumer footprint.

## Retired spellings

- None — the arm keeps its spelling and its row; only its adopter-facing listing goes.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls context-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
