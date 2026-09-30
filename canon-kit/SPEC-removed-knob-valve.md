# SPEC amendment: removed-knob-valve

**A removed knob is history, and `check-docs-cmd` admits history on assertion C only.** A release note's Renamed-knobs removal bullet leads with the removed knob backticked (installer/SPEC.md §The upgrade contract, the `old → ∅` form). Assertion B reds every backticked kit-prefixed name no kit code carries, which a removed knob is by definition. The three history valves (§check-manifest-temporal) admit a retired *path* under C and never a knob, so the one valve that reaches B is the whole-doc `CANON_KIT_MDREF_EXCLUDE`. That exclude also drops the note from `check-md-refs`, `check-fence-command-head`, `check-fence-run`, `check-install-claim` and `check-payload-claim`.

**The fix mirrors C.** C reds on retirement and A on absence. B keeps redding on absence, and under a history valve it admits a name the repository has *retired*: one that tracked code carried at some revision this clone holds. A misspelled knob on a valved line was never carried and still reds, so a dated post keeps the knob resolution the path valve's own comment promises it.

**Measured at authoring.**

- With the note taken off the exclude, `check-docs-cmd` reds 14 findings, all assertion B, all in `docs/posts/2026-09-26-checkwright-v0-26-0.md`: 13 names on lines 359, 367, 373, 377 under Renamed knobs, and a repeat of one at 784 under Behavior changes. The other five exclude readers pass on the note. The oracle is `CANON_KIT_KNOB_FILE=<copy without the exclude line> bash gate-sdk/bin/run-gates.sh --only <gate>`.
- Every one of the 13 names has a commit in `git log -S<name> -- . ':(exclude)*.md' ':(exclude)**/gate-tests/**'`, and an invented name has none, at about a quarter second per name on this history.
- The note is path-valved: `scripts/canon-config.knobs` binds `CANON_KIT_TEMPORAL_EXEMPT_PATHS` to `docs/posts/*`.

**Refused: B reading the removal bullet's grammar.** The `old → ∅` form and the Renamed-knobs heading are installer's grammar, and canon-kit reads no installer vocabulary. The v0.26.0 note also names removed knobs in multi-knob bullets and under Behavior changes, which a lead-token reader would still red.

**Refused: listing each removed knob in its kit's `retired` table.** That table's names join B's defined set everywhere (`knobs::static_names`), so a live doc naming a removed knob would pass B on every line. The table also pairs each name with a replacement, which a removal has none of.

## What changes

### (1) Assertion B admits a retired knob on a history-valved line

`native/src/gates/docs_cmd.rs` {design-bearing}: a kit-prefixed token that `knob_ok` refuses, on a line the history valves exempt, is admitted when the name is retired. That means the valve marker, an exempt section or an exempt path, read through the same `TemporalValve` line classifier C reads.

A name is **retired** when some commit in the history this clone holds carries it in a tracked path outside markdown and outside the fixture trees. The pathspec is the one `defined_knobs` greps, widened from the kit roots to the whole tree, because a static kit's names live outside kit-root code.

The history read runs once per run, and only when a valved line holds a name the defined set lacks, over that candidate set alone. A retired name on an unvalved line reds as before. A never-carried name on a valved line reds as before.

The clean line counts admitted retired knobs apart from resolved ones. The help text drops "the doc joins `CANON_KIT_MDREF_EXCLUDE`" as B's remedy and names the history valves for a removed knob.

The unit test `a_valved_line_exempts_citations_and_not_knobs` becomes one holding that a valved line still yields its knob tokens. `canon-kit/gate-tests/check-docs-cmd.test.sh` gains three history-built cases:

- a retired knob on a path-valved line, clean;
- the same knob unvalved, red;
- an invented knob on a valved line, red.

### (2) §check-docs-cmd states B's history admission

canon-kit/SPEC.md §check-docs-cmd {mechanical}. **Not yet applied.**

The paragraph opening "**History is admitted where `check-manifest-temporal` admits it**" becomes:

> **History is admitted where `check-manifest-temporal` admits it, through its three valves and no fourth:** a `manifest-temporal-exempt: <reason>` marker on the line or the one above, a section named in `CANON_KIT_TEMPORAL_EXEMPT_SECTIONS`, or a file matching `CANON_KIT_TEMPORAL_EXEMPT_PATHS`. Naming a retired path or a retired knob is narration about the past, and that gate already rules where narration may stand; the compiled member reads the valves through that member's own line classifier, so neither gate can disagree with the other about which line is history. On a valved line C admits a retired path, and B admits a **retired knob**: a name some commit in the held history carried in a tracked path outside markdown and the fixture trees. A name never carried still reds there, and a valved line's fenced invocations stay scanned. In a flowing paragraph the marker rides the end of a line as inline HTML, since a line-leading `<!--` opens an HTML block that ends the paragraph; one marker covers its own line and the next.

In the following paragraph, "**A shallow clone under-reds and never invents:**" and its sentence become:

> **A shallow clone reads less history, and the direction depends on the assertion:** C reports fewer findings and no false ones, while B admits fewer retired knobs, so a removed knob on a valved line reds until the history is fetched. gate-sdk's workflow template fetches full depth for history-reading gates, and the clean line names the shallow case rather than passing silently.

The rest of that paragraph stands. Its fixture sentence gains "and B's admission" after "a retirement case".

### (3) §check-manifest-temporal names B's admission

canon-kit/SPEC.md §check-manifest-temporal {mechanical}. **Not yet applied.** The sentence "These three valves also admit a retired-path citation under §check-docs-cmd assertion C, so …" becomes "These three valves also admit a retired path under §check-docs-cmd assertion C and a retired knob under its assertion B, so …", with the rest of the sentence unchanged.

### (4) The v0.26.0 note leaves the exclude

`scripts/canon-config.knobs` {mechanical}: the `CANON_KIT_MDREF_EXCLUDE` line naming `docs/posts/2026-09-26-checkwright-v0-26-0.md` and its `comment-tier-exempt:` comment are deleted, leaving the knob unset. The path valve's comment keeps saying posts take link and fenced-command resolution, which now holds for every post.

This lands with delta 1 or after it. Alone it reds `check-docs-cmd` on the 14 findings above.

### (5) The release declaration

`.workflow/release-declarations.md` {mechanical}, under Behavior changes:

> - **canon-kit `check-docs-cmd`, assertion B** — on a line the history valves exempt (the `manifest-temporal-exempt:` marker, `CANON_KIT_TEMPORAL_EXEMPT_SECTIONS`, `CANON_KIT_TEMPORAL_EXEMPT_PATHS`), a kit-prefixed knob no kit code carries is admitted when your held history once carried it, so a release note or dated post naming a removed knob needs no whole-doc `CANON_KIT_MDREF_EXCLUDE`. A misspelled knob still reds there. Nothing to do; a doc you excluded only for a removed knob can leave the exclude.

## Producers and consumers

- **The retired-knob admission.**
  - Producer: `check-docs-cmd`'s rule, per run, on the generated pre-commit hook, `run-gates.sh` and CI.
  - Enabling config: any history valve. This repository sets `CANON_KIT_TEMPORAL_EXEMPT_PATHS` to `docs/posts/*` and `CANON_KIT_TEMPORAL_EXEMPT_SECTIONS` to `Out of scope`.
  - Consumer: the committing session through the output contract, which reads the clean line's admitted count and the red line's finding.
  - The admission adds no field and no knob.
- **Readers of the exclude.** Delta 4 empties `CANON_KIT_MDREF_EXCLUDE` in this repository. Each of its six readers (`git grep -n CANON_KIT_MDREF_EXCLUDE -- native/src`) either passes on the note or was probed above. Their red conditions are per-violation, never a count or a floor, so a wider corpus can only add findings, and the probe found none beyond B.
- **Readers of B's code.** `check-docs-restatement-parity` reuses B's matcher `kit_knob_runs` and `check-action-run-path` reuses A's `invoked_tokens` (`git grep -n docs_cmd -- native/src`). The admission lives in the rule and not in either helper, so neither changes.

## Existing sections updated

- `native/src/gates/docs_cmd.rs`: the admission, the clean-line count, the help text and the unit test (delta 1).
- `canon-kit/gate-tests/check-docs-cmd.test.sh`: the three cases (delta 1).
- `canon-kit/SPEC.md` §check-docs-cmd (delta 2) and §check-manifest-temporal (delta 3).
- `docs/canon-kit/SPEC.md`: the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (deltas 2 and 3).
- `scripts/canon-config.knobs`: the exclude and its comment (delta 4).
- `.workflow/release-declarations.md`: the Behavior changes bullet (delta 5).
- `.workflow/prose-bound-ceiling.txt`: the `canon-kit/SPEC.md` row, re-stamped to the count `check-prose-bounds` prints if deltas 2 and 3 move it (deltas 2 and 3).

The roster came from `git grep -n -e CANON_KIT_MDREF_EXCLUDE -e 'History is admitted' -e 'also admit a retired'` over the tracked tree, and from `grep -n 'fn \|help' native/src/gates/docs_cmd.rs`.

## Retired spellings

- None — the amendment adds an admission and renames nothing.

## Definition of Done

- [ ] **Causal completeness**: every point of canon-kit/SPEC.md §The causal-completeness check holds for the admission.
- [ ] **Instruction surfaces: instruction only**: the help text names the valves and carries no grounds.
- [ ] **Merged with no information lost**: §check-docs-cmd reads as one section, the history paragraph covering B and C alike.
- [ ] **Amendment deleted**: this file is removed on merge, and `ls canon-kit/SPEC-*.md` lists no file of this amendment.
- [ ] **Removals propagated**: nothing retired.
- [ ] **Gaps filed**: a cross-component gap found during the work is filed with `--emit file-gap`.
- [ ] **The entry moves**: `removed-knob-docs-cmd-valve` moves to Done in the landing commit, before the drain stage.
