# SPEC amendment: spec-prune

`native/src/spec.rs`' `CANON_SPEC_PRUNE` bakes `**/templates` and `docs/*` into canonical-spec discovery for every consumer of `spec::canonical_specs`. The filed concern: `docs/*` is this repo's generated-mirror location, a consumer layout path in kit mechanism. An adopter whose site lives elsewhere would double-count its mirror as canonical specs, and one whose real specs sit under `docs/` would lose them.

**The ruling: both halves are kit mechanism, and the ground is stated where the constant lives. No knob is minted.**

- **`**/templates`** is already grounded. §The shared spec adapters' finders skip a `templates/` skeleton as "a copyable stub, not governed content", and canon-kit's own amendment skeleton under `templates/` is the instance.
- **`docs/*` is where canon-kit's own generator writes.** `--emit docs-mirror` (§The reference-link grammar) writes every mirror page under `docs/<dir>/` (`native/src/emit/docs_mirror.rs`). So the prune is not a guess at a consumer's site root. It is the kit excluding its own output from its own discovery, and the two literals are one fact: where the kit's mirror lives. The defect is that the fact has two spellings, one in the generator and one in the prune, so this unit gives it one.

**The "one place" the SPEC already claims is false today, and making it true is the substance of this unit.** canon-kit/SPEC.md §The shared spec adapters says the two narrowings are "one prune list, read by the walk and by the member's registry declaration from one place". The comment on `CANON_SPEC_PRUNE` says the same. But the registry does not read the constant. `native/src/gates/mod.rs` transcribes `**/templates,docs/*` as a literal six times, and nothing ties those copies to the constant. A declared prune narrows `check-reads-couples`' coverage demand, so a copy that drifted from the walk would narrow that demand over a subtree the walk actually reads. That is the hidden-gap direction that gate warns about.

**Refused, with grounds.**

- **A knob carrying this repo's two values.** A declared prune in a registry row is a literal comma list, and `check-reads-couples` resolves no `knob:` source for the prune field. A knob-valued prune would therefore need a gate-sdk grammar change, bought for a relocation no adopter has asked for. And a knob over `**/templates` would make a kit convention consumer-removable.
- **A derivation from a site-root knob.** The entry named `SITE_KIT_SCAN_ROOT`, but that is `check-docs-cname-parity`'s `git ls-files` root (site-kit/SPEC.md), not the site root. `SITE_KIT_DOCS_DIR` is the site root, and it is another kit's knob; lifecycle-kit/SPEC.md declines such a read, on the ground that it would make one kit depend on another's configuration. canon-kit's own `CANON_KIT_LINK_ROOT` (default `docs`) is the link-convention gate's scan root, not the mirror's output. Tying the generator to it is a relocation feature, and it carries the same prune-grammar cost.

**The honest limit.** An adopter's own canonical spec at `docs/<dir>/SPEC.md` is invisible to the four finders that take the prune with no valve: `check-spec-embedded-source`, `check-spec-dod-singleton`, `check-spec-derivable-section` and `check-surface-duplication`. That path is the kit mirror's namespace. The limit is filed to the gap inbox with this amendment's commit, as a relocatable mirror root owed to both the generator and the prune together.

**Measured at authoring (2026-09-23).**

- `git grep -c '\*\*/templates,docs/\*' native/src/gates/mod.rs` prints 6.
- `CANON_SPEC_PRUNE` has one reader, `canonical_specs` (`native/src/spec.rs`).
- `docs_mirror.rs` writes under `format!("docs/{}", …)`.
- `checkwright-gates --reads check-surface-duplication` prints the prune as `**/templates,docs/*`.
- No canon-kit fixture holds a `docs/<x>/SPEC.md` or a `templates/SPEC.md`.

**Verified at align (2026-09-23), widening delta 1.** The "one place" defect has a fourth instance the authoring probe's own `git grep` would have caught but the disposition above did not enumerate: `native/src/gates/docs_mirror_fresh.rs:41` composes its own `format!("docs/{}", src)` to derive each mirror page's expected path, independently of `docs_mirror.rs`'s copy. Left untouched, delta 2's rewritten sentence — "it has one spelling, which the generator, the walk and every member's registry declaration read" — would be false on merge. Delta 1 below is widened to include this site.

## What changes

### (1) One constant for the mirror root, read by the generator, the prune and the registry {design-bearing}

**Not yet applied.**

- **The mirror root.** `native/src/spec.rs` gains the mirror root as a constant (`docs`). `docs_mirror.rs` composes its output path and its `--write` report from that constant, not from the literal, and `docs_mirror_fresh.rs`'s `rule()` composes its own per-page expected path (line 41) from the same constant rather than transcribing `"docs/{}"` a third time.
- **The prune.** `CANON_SPEC_PRUNE`'s second member is derived from the mirror root (`<root>/*`).
- **The registry.** The six registry prune declarations in `native/src/gates/mod.rs` name one `spec` constant holding the comma-joined prune rather than transcribing it.
- **The test.** A `spec.rs` unit test holds that joined constant equal to `CANON_SPEC_PRUNE`'s members, so the walk and the declaration cannot disagree.

The comment on `CANON_SPEC_PRUNE` then states the ground in one line: the templates convention and the kit mirror's own output root. `--reads` output is unchanged, so `check-reads-couples` and the recorder's declared-is-observed unit test see the same string.

### (2) §The shared spec adapters states the ground {mechanical}

**Not yet applied.** In canon-kit/SPEC.md §The shared spec adapters, the bullet opening "**The canonical-spec finder prunes the generated on-site mirror**" becomes:

> - **The canonical-spec finder prunes the generated on-site mirror**, as a directory prune beside the `templates/` one it already applies. A prose gate grading a generated page is unfixable at the file — the repair is to the source and the regeneration — so the finding can only be absorbed or ignored. The mirror is excluded because it is **generated**, and the generator is the kit's own: `--emit docs-mirror` writes under one mirror root, and the prune is that root's children. So the path is kit mechanism rather than a consumer's layout, and it has one spelling, which the generator, the walk, the docs-mirror freshness comparator and every member's registry declaration read. A declared prune narrows a coverage demand, and two spellings could disagree about it (gate-sdk/SPEC.md §check-reads-couples). **The limit that buys:** an adopter's own `SPEC.md` at `<mirror root>/<dir>/` sits in the mirror's namespace and is not discovered. Relocating the root would move the generator and the prune together, and it needs a `knob:` source for a registry prune that gate-sdk does not yet have.

## Producers and consumers

- **The mirror-root constant** (delta 1). Readers: `docs_mirror.rs` at `--emit docs-mirror`, `docs_mirror_fresh.rs` at every `check-docs-mirror-fresh` run, `canonical_specs` at every finder call, and the six registry rows through the joined constant, which `--reads` prints and `check-reads-couples` consumes. The unit test is its roster-holding reader.
- **Point 5.** No corpus changes. The prune's members are the same two strings, now spelled once.
- **Point 6.** Not reached.

## Existing sections updated

Roster from `git grep -n "CANON_SPEC_PRUNE\|\*\*/templates,docs/\*\|docs/{}" native/src` and `git grep -n "prunes the generated on-site mirror" -- '*.md' ':!docs'`, run 2026-09-23.

- `canon-kit/SPEC.md` §The shared spec adapters (delta 2).
- `native/src/spec.rs`, `native/src/emit/docs_mirror.rs`, `native/src/gates/docs_mirror_fresh.rs` and `native/src/gates/mod.rs` (delta 1).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/canon-kit/SPEC.md`.

## Retired spellings

- None — the prune's strings and `--reads` output are unchanged; only the registry's six transcriptions become a constant reference.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the constant.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** The adapters bullet is re-phrased, not appended to.
- [ ] **Amendment deleted.** This file is removed on merge (`ls canon-kit/SPEC-*.md`).
- [ ] **Entry moved.** `canon-spec-prune-bakes-docs` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Nothing retired.
- [ ] **Gaps filed.** The relocatable mirror root is filed to the gap inbox with this amendment's commit.
