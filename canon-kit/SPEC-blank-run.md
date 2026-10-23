# SPEC amendment: blank-run

Pairs `manifest-count-blank-run-bound`. `check-manifest-count` arm B reads a lead-in's list only past at most one blank line, and the SPEC states that bound. The reader is `lead_ins` in `native/src/gates/manifest_count.rs`, which skips one blank line with an `if`. So a lead-in, two blank lines, then a list is silently unread, though Markdown reads any blank run as one block separator. This amendment widens the bound to a blank run in the SPEC and the reader.

**What the tree holds today**, probed at spec:

- `lead_ins` steps past a single blank line (`if j < n && spec::is_blank(lines[j]) { j += 1; }`), so a second blank leaves `j` on a blank line, which `is_list_item` rejects.
- No tracked markdown file carries a line ending in `:`, then two or more blank lines, then a list item. An `awk` pass over `git ls-files '*.md'` found none, so the widened arm reads no lead-in here that the current one skips, and this tree gains no red.
- The SPEC sentence stating the bound has one mirror, `docs/canon-kit/SPEC.md`, which `git grep -n "at most one blank"` finds beside the source and the queue entry.

## What changes

### (1) The SPEC states the bound as a blank run {mechanical} {user-facing: manifest-count-blank-run-bound's Deliverable, "the bound widened to a blank run in the SPEC and the reader, with a bad-fixture case"}

Replacement for `canon-kit/SPEC.md` §check-manifest-count, the opening of the paragraph beginning **Arm B reads a lead-in**. **Not yet applied.**

> **Arm B reads a lead-in: a paragraph whose text ends in `:` and whose next block, past any run of blank lines, is a bullet or numbered list**, since Markdown reads a blank run as one separator.

The rest of that paragraph is unchanged.

### (2) The reader skips the run {mechanical} {user-facing: as delta 1}

All **Not yet applied.**

- **`native/src/gates/manifest_count.rs`**: in `lead_ins`, the single-blank `if` becomes a `while` over blank lines.
- **A unit case** in its test module: a lead-in, two blank lines, then a two-item list under the cardinal `Three` reds as `(line, "Three", 2)`, and the same list under `Two` reads clean. Both are counted in `read`.
- **The bad fixture**: `canon-kit/gate-tests/check-manifest-count/bad/SPEC.md` gains a closing paragraph. Its lead-in opens with the cardinal `Two` and ends in `:`, two blank lines follow, then a three-item list. Its `expect.txt` gains `lead-in count: 'Two' over 3 item(s)`.

### (3) The release declaration and the mirror {mechanical}

**Not yet applied.** `.workflow/release-declarations.md`, under **New and tightened gates**, gains:

> - `check-manifest-count` — arm B reads a lead-in's list past any run of blank lines, where it stopped after one. A lead-in whose cardinal disagrees with a list two or more blank lines below it now reds. Fix the count, or tag the site `manifest-count-exempt: <reason>`.

The docs mirror regenerates: `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`.

## Producers and consumers

- **The widened lead-in read** (delta 2). Its producer is any governed manifest the shared manifest-set finder scans. Its consumer is arm B's comparison, reached through `check-manifest-count` at `precommit`.
- **A widening, not a narrowing**, so point 5's binding case is absent. Each reader of the new reds is named all the same:
  - the gate itself reds a disagreeing lead-in it now reaches. This tree carries none (the probe above).
  - `run-gate-tests` reds the bad fixture unless the gate finds both findings `expect.txt` names (delta 2).
  - `check-docs-mirror-fresh` reds until the mirror regenerates (delta 3).
  - `check-release-declaration-parity` holds the next composed note to delta 3's bullet.
  - `check-amendment-queue` reds a `[spec:]` ref resolving to no file. The paired entry carries this file's name.
- **Consumers vendoring canon-kit** meet the wider read at their next update, which delta 3's bullet tells them.

## Existing sections updated

- `canon-kit/SPEC.md` §check-manifest-count, arm B's opening sentence (delta 1).
- `native/src/gates/manifest_count.rs` and `canon-kit/gate-tests/check-manifest-count/bad/` (delta 2).
- `.workflow/release-declarations.md` and `docs/canon-kit/SPEC.md` through the mirror (delta 3).

Roster probe: `git grep -n "at most one blank"` names `canon-kit/SPEC.md`, its docs mirror and the queue entry. `git grep -n "lead_ins\|list_items" -- native/src` names `manifest_count.rs` alone, beside an unrelated test name in `surface_duplication.rs`.

## Retired spellings

- `at most one blank line` — the bound's old statement (delta 1). <!-- retired-spelling-exempt: the paired queue entry quotes it as the defect, and the entry leaves the queue at Done -->

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls canon-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
