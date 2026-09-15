# SPEC amendment: depth-bound-globs

Queue entry: `depth-enumerated-glob-bound-unoracled`, selected for `couples-field-semantics` as its
depth-bound unit (operator direction, 2026-09-15, lead-relayed). It merges after gate-sdk's
`SPEC-couples-semantics.md`. Delta 2 respells a value whose `knob:` couples token needs that
amendment's covering conversion.

## The question, and what this stage probed

The entry names a class: a depth-enumerated glob value is a maintained copy of "any depth", and a
file one level below its last rung leaves the scanned corpus silently. It cites two instances in
this repo's `scripts/canon-config.knobs` and offers two dispositions:

- widen the measured-claim corpus to reach the knob file;
- assert that no tracked governed source lies deeper than the enumerations reach.

**Probed at this stage.**

- **`CANON_KIT_COMMENT_SURFACE`'s headroom is zero, not one level.** Its value enumerates each of
  `sh|gate|rs|knobs` at one to five path segments. The deepest tracked `.sh`/`.gate`/`.rs`/`.knobs`
  outside `gate-tests/` sits at exactly five segments, for example
  `native/src/emit/kpi/task_split.rs` and `native/src/emit/pub_lang/rust.rs`. That is the last rung.
  The knob's own comment and the queue entry both say one level of headroom, so the next source file
  added one directory deeper under `native/src/emit/` escapes today.
- **Its cause was gate-sdk's.** The comment reasons that `**` was avoided because the couples matcher
  cannot express it while the filter matcher can. `SPEC-couples-semantics.md` removes that cause: a
  `knob:` member `**/*.rs` expands to the covering pattern `**.rs`. The configured branch already
  prunes like the default one (canon-kit/SPEC.md §The shared spec adapters, *The configured branch
  narrows exactly as the default branch does*). `walk::glob_files`' `**` skips hidden directories as
  its `*` does (`walk.rs`, `subdirs`), so a `**` value reaches the same tree the rungs did.
- **`CANON_KIT_MANIFEST_FILES`' single-level globs are a selection, not a copy.** The only tracked
  `README.md`/`SPEC.md` files three segments deep outside fixtures are the generated mirror under
  `docs/<kit>/` and `reserve/crates/README.md`, which the value lists by name. So `*/README.md` and
  `*/SPEC.md` stop at one level (two segments: `<kit>/README.md`) in order to exclude the
  three-segment mirror, which a `**` spelling would admit.
  Their bound is the consumer's editorial choice. A new nested `README.md` escaping it is the
  ordinary staleness of any enumerated selection, the same as a new root document the list does not
  name, and not a depth defect.

## The ruling

**Remove the copy rather than oracle it. The class member that is a copy of "any depth" is respelled
`**`, and the one that is a selection is ruled out of the class.** Enforcement-first ranks removing
the duplication above gating it, and once the respelling lands no depth-enumerated copy remains in
the tree to gate. Three oracles were weighed and refused, and none needs a gate:

- **Widening the measured-claim corpus to the knob file** would make one more surface measured
  forever, for a claim (the depth of a corpus) whose only instance this amendment removes.
- **Asserting no governed source lies deeper than the enumerations** has no subject once the
  respelling lands. On a selection it is circular, because what is governed is defined by the
  selection itself.
- **A ladder detector**, reading a glob array whose members differ only by leading `*/` rungs as a
  copy of `**`, false-positives on the one legitimate shape in the tree: `CANON_KIT_MANIFEST_FILES`'
  `README.md` beside `*/README.md` is a two-rung ladder, and it is a deliberate selection.

## The seam

Kit prose: the glob-array corpus knobs' authoring rule, in canon-kit's SPEC. Consumer config: this
repo's `CANON_KIT_COMMENT_SURFACE` value and its comment. No private rule content, and no knob added.

## What changes

### (1) The any-depth spelling is stated where the corpus knobs are {design-bearing}

canon-kit/SPEC.md §Layout and configuration. **Not yet applied.**

- **After** the paragraph beginning **A corpus knob below widens the declaring gates' *triggers***,
  add:

  > **A glob-array corpus knob spells "any depth" as `**`, never as one glob per depth.** Its value is
  > a `walk::glob_files` pattern, whose `**` spans any number of directories and skips hidden ones as
  > `*` does, and its `knob:` couples token expands a `**` member to a pattern covering every path it
  > selects (gate-sdk/SPEC.md §The `# graph:` manifest). A value that enumerates depths to mean every
  > depth is a maintained copy that drops a file one level below its last rung with no diagnostic. A
  > depth bound that excludes a subtree on purpose is a selection, and stays the consumer's to spell.

- **In the `CANON_KIT_COMMENT_SURFACE` bullet**, *default empty ⇒ derive: shell sources under the
  root* becomes *default empty ⇒ derive: `.sh`, `.gate` and `.rs` sources under the root*. The
  bullet's current text understates the derivation `spec::comment_surface`'s default branch performs
  (`find_files` over `sh`, `gate`, `rs`). The sentence is corrected in place here because this delta
  is already editing the bullet.

### (2) This repo's comment surface is respelled {mechanical}

`scripts/canon-config.knobs`. The twenty `CANON_KIT_COMMENT_SURFACE[]` lines become four:
`**/*.sh`, `**/*.gate`, `**/*.rs` and `**/*.knobs`. The line's `comment-tier-exempt:` reason keeps
two things:

- its first clause, that the surface is set so the corpus arrives through the knob the descriptors
  couple to;
- its knob-file clause, that the `.knobs` globs are the one widening past the default branch, and why.

It loses the depth-enumeration reasoning, the headroom claim and the MANIFEST_FILES comparison.
`CANON_KIT_MANIFEST_FILES`' comment is unchanged: its single-level reason is true and is the
selection this amendment rules.

**Build verifies the corpus is unchanged, not assumed.** Run `spec::comment_surface`'s corpus before
and after, for example through the scanned-file counts in `check-comment-tier`'s and
`check-spec-pointer`'s clean lines, and compare. On this tree it should be byte-identical, since
nothing lies below the last rung. A difference is reported in the commit and resolved at its file
before landing.

## Producers and consumers

- **The respelled value (delta 2).**
  - **Producer:** this repo's tracked knob file.
  - **Consumers**, each at an existing transition:
    - `spec::comment_surface`'s configured branch, feeding `check-comment-tier`,
      `check-spec-pointer`, `check-todo-task-liveness` and `check-deprecation-task`.
    - The `knob:CANON_KIT_COMMENT_SURFACE` couples token, feeding every trigger reader.
    - `check-reads-couples`, reading the `glob:knob:CANON_KIT_COMMENT_SURFACE` filter through
      `glob_walk`'s `**` arm.
- **The rule sentence (delta 1)** has no machine reader. It is read by a consumer authoring a corpus
  knob value, at authoring time.
- **Readers' red conditions (point 5).** Delta 2 is identity on this tree's corpus by the probe
  above, and a widening wherever a file lies below the old last rung. What reds:
  - The four comment gates red on a newly scanned file carrying a violation. None is expected, and
    the corpus comparison is the check.
  - `check-reads-couples` reds if the `**` filter selects a tracked read that no couple covers. The
    converted `**.<ext>` token covers every such path, and it requires `SPEC-couples-semantics.md`
    delta 4, which is why this amendment merges after it.
  - `check-graph` assertion D reds until the hook is regenerated, because the expansion of the
    `knob:` token changes.

## Existing sections updated

Roster probe: `git grep -n "CANON_KIT_COMMENT_SURFACE\|CANON_KIT_MANIFEST_FILES"` over
`canon-kit/SPEC.md` and `scripts/canon-config.knobs`, plus a depth histogram of tracked sources
outside `gate-tests/`, run at this stage.

- `canon-kit/SPEC.md` — §Layout and configuration's corpus-knob paragraph and the
  `CANON_KIT_COMMENT_SURFACE` bullet (delta 1).
- `scripts/canon-config.knobs` — the `CANON_KIT_COMMENT_SURFACE` value and its reason (delta 2).
- `scripts/git-hooks/pre-commit` — regenerated for the token's new expansion (delta 2).
- `docs/check-graph.html` — the coupling-graph artifact, whose nodes draw each knob member,
  regenerated (delta 2).
- `docs/canon-kit/SPEC.md` — the generated mirror, regenerated (delta 1).

## Retired spellings

- `*/*/*/*/*.rs` — the deepest rung of the enumerated comment surface, standing for the whole ladder
  (delta 2).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and
      a named consumer; every new field has a named reader at a named transition.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition
      or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); the merged spec reads as one coherent document a reader who never saw
      the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls canon-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings`
      above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked
      tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
