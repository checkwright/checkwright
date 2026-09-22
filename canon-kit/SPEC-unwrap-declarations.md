# SPEC amendment: unwrap-declarations

`--emit md-unwrap` joins a line-start declaration onto the paragraph above it, because §check-md-unwrapped's block scanner models only CommonMark block starts. A declaration such as lifecycle-kit's `ruling:`, `discharge:` or `close-surface:` line is read by a reader that keys on the line start, so the join disarms that reader while changing no rendering, and neither the gate nor the arm's postcondition reds. The attested case is TRAJECTORY.md, whose `discharge:` and `ruling:` lines the unwrap folded into the ruling paragraph; two-space hard breaks restored them.

**The ruling: a consumer-configured declaration-lead set that the scanner treats as a block start.** Which line leads a tree declares is that tree's vocabulary, and the three attested leads belong to another kit, so none ships as a canon-kit literal (the provenance seam). The default is empty, which leaves the scanner exactly as it is today.

## What changes

### (1) The knob: `CANON_KIT_UNWRAP_DECLARATION_LEADS` {mechanical}

**Not yet applied.** In §Layout and configuration, extend the `CANON_KIT_UNWRAP_GLOBS` / `CANON_KIT_UNWRAP_EXCLUDE` bullet with the third member of the family, rewriting the bullet rather than adding one:

> - `CANON_KIT_UNWRAP_GLOBS` / `CANON_KIT_UNWRAP_EXCLUDE` — arrays of git pathspecs, default empty: the tracked markdown `check-md-unwrapped` holds to one line per paragraph, less the excluded paths. An empty include set is the clean skip, since which files a tree keeps unwrapped is its own editorial choice. `CANON_KIT_UNWRAP_DECLARATION_LEADS` — array of line leads, default empty: a line whose content starts with a member opens a block, so the gate never reds it and `--emit md-unwrap` never joins it. A member that is empty is malformed config. Which declarations a tree writes is its own vocabulary, so none ships as a kit literal (§check-md-unwrapped).

The knob is a vocabulary rather than a walk filter, so it takes no `knob:` couples token on `check-md-unwrapped.gate`, on the ground `CANON_KIT_FENCE_RUN_PROGRAMS` already states.

### (2) §check-md-unwrapped: a declared lead opens a block {design-bearing}

**Not yet applied.** In the **The block scanner.** paragraph, extend the sentence listing what opens a block, after "or a link-reference definition (`[label]:`)", with:

> , or a line whose content, after indentation and any block-quote markers, starts with a member of `CANON_KIT_UNWRAP_DECLARATION_LEADS`

Then add, after the sentence ending "the fixture pair pins what it does model":

> A declared lead is the one block start the scanner takes from configuration rather than from CommonMark. CommonMark joins such a line into the paragraph above it unless the line before ends in a hard break, so the gate stops asserting what a renderer would do there. It asserts what a line-start reader needs instead, and that reader is why the lead was declared.

In the **`--emit md-unwrap`** paragraph, rewrite "A join never crosses a blank line, a block start or an exempt interior, so it changes no rendering the subset models." as:

> The arm reads the same knob the gate does, so a join never crosses a blank line, a block start, a declared lead or an exempt interior. Only a declared lead's own line can render differently from its source, and a hard break on the line before it keeps even that one unchanged.

### (3) The scanner and the arm read the lead set {mechanical}

**Not yet applied.** In `native/src/gates/md_unwrapped.rs`, `scan` and `unwrap` take the resolved lead set, and a line whose trimmed content starts with a member opens a block beside `is_link_def`. `rule` resolves `CANON_KIT_UNWRAP_DECLARATION_LEADS` through the same knob reader as the two pathspec knobs and refuses an empty member at exit 2, naming it. `native/src/emit/md_unwrap.rs` resolves the knob the same way before calling `unwrap`. The knob gets a `Row::indexed` row with an empty default in `native/src/knobs/canon_kit.rs`, and joins the `check-md-unwrapped` knob list in `native/src/gates/mod.rs`.

Coverage: a fixture row in each case dir. The `good/` tree binds one lead in its `scripts/canon-config.knobs` and carries a paragraph followed by a line opening with that lead, which is clean. The `bad/` tree carries the same shape with a lead it does not bind, which reds as an ordinary soft break. Unit tests hold that `unwrap` never joins a declared lead's line, and that an empty member refuses.

### (4) This repo binds its three declaration leads {mechanical}

**Not yet applied.** In `scripts/canon-config.knobs`, beside the two `CANON_KIT_UNWRAP_*` lines:

```
CANON_KIT_UNWRAP_DECLARATION_LEADS[] = ruling:
CANON_KIT_UNWRAP_DECLARATION_LEADS[] = discharge:
CANON_KIT_UNWRAP_DECLARATION_LEADS[] = close-surface:
```

The two-space hard breaks on TRAJECTORY.md's `discharge:` and `ruling:` lines stay. They keep those lines rendering as separate lines, which the knob does not change.

## Producers and consumers

- **The lead set** (deltas 1 and 3). Produced by the consumer's `canon-config.knobs`; this repo sets it (delta 4), so the producer is live outside unit tests. Consumed by `check-md-unwrapped`'s `rule` and by `--emit md-unwrap`, which both pass it to the one shared scanner. Every member is read at the scanner's block-start test and nowhere else.
- **Roster-holding readers of the new name.** The knob table in `native/src/knobs/canon_kit.rs` and the gate's knob list in `native/src/gates/mod.rs` each hold a roster, and both are update targets (delta 3). `check-knob-citation` and `check-kit-ref-liveness` resolve the name through the knob table once that row exists.
- **The readers the change protects.** lifecycle-kit's `ruling:` / `discharge:` reader (lifecycle-kit/SPEC.md §The ruling-staleness probe) and the `close-surface:` reader (lifecycle-kit/SPEC.md §The close-surface roster, derived in `native/src/emit/close_surfaces.rs`). Neither changes. Each keeps reading line starts, which the arm now leaves where they were.
- **Point 5.** The gate's corpus does not narrow. Its verdict set shrinks only by the lines a consumer declares, and the gate has no finding-none, exact-count or floor reading.
- **Point 6.** Not reached; the delta obliges no enumerable corpus.
- **Release declaration.** None owed: the default is empty, so no adopter's verdict moves.

## Existing sections updated

Roster from `git grep -n -e "CANON_KIT_UNWRAP" -e "md_unwrapped::" -- canon-kit native scripts` and `git grep -n -E "^\s*(ruling|discharge|close-surface):" -- '*.md'`, run 2026-09-22.

- `canon-kit/SPEC.md` §Layout and configuration, the `CANON_KIT_UNWRAP_GLOBS` bullet (delta 1).
- `canon-kit/SPEC.md` §check-md-unwrapped, the block-scanner and `--emit md-unwrap` paragraphs (delta 2).
- `native/src/gates/md_unwrapped.rs`, `native/src/emit/md_unwrap.rs`, `native/src/knobs/canon_kit.rs` and `native/src/gates/mod.rs` (delta 3).
- `canon-kit/gate-tests/check-md-unwrapped/good/` and `bad/` (delta 3).
- `scripts/canon-config.knobs` (delta 4).
- `docs/canon-kit/SPEC.md`, the generated mirror (all deltas).

## Retired spellings

- None — no delta retires a name.

## Definition of Done

- [ ] **Causal completeness.** Every point of §The causal-completeness check holds for the lead set.
- [ ] **Instruction surfaces: instruction only.** No template or shim changes.
- [ ] **Merged with no information lost.** §check-md-unwrapped reads as one document and the knob bullet as one bullet.
- [ ] **Amendment deleted.** This file is removed on merge (`ls canon-kit/SPEC-*.md`).
- [ ] **Entry moved.** `md-unwrap-folds-declarations` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached; the block above declares none.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
