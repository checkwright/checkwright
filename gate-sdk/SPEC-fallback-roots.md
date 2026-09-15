# SPEC amendment: fallback-roots

Queue entry: `couples-dynamic-root-resolution`, the lead unit of `couples-field-semantics` (operator
direction, 2026-09-15, lead-relayed). **It merges after `SPEC-couples-semantics.md`**, whose one
semantics it reads. The entry's blocker tag did not survive promotion: its lead line cannot carry the
tag beside the spec tag inside the queue's wrap budget (filed to the gap inbox), so this line is the
order's holder.

## The question, and what this stage probed

The entry withdrew the 26 kit-literal fallback roots of `spec::manifest_files` and
`spec::comment_surface`. The ground was that a kit cannot demand an adopter's `couples=` enumerate
the depth of an unseen tree while the field's semantics is open. It asserted, unprobed, that settling
the semantics is the only thing that can move them. **Probed at this stage, over this tree:**

- **The population is as recorded.** `--emit reads-census` prints 26 `fallback` root-lines across 14
  members:
  - 10 manifest members × 2 (`spec.rs:204`, `:210`, via `const MANIFEST_ROOTS`)
  - 3 comment-surface members × 1 (`:233`, via `const COMMENT_SURFACE_ROOTS`)
  - `check-spec-pointer` × 3 (via `const SPEC_POINTER_ROOTS`)
- **The walks' read sets, measured against each member's literal couples** (knob tokens empty, as on a
  tree taking the fallback), came to 0 uncovered manifest reads and 2 uncovered comment-surface reads
  under the settled matcher. Under the segment-wise one the figures were 24 of 26 and 395 of 403.
  So the settled semantics moves every one of the 26. The 2 residual reads are the genuine
  under-couple delta 3 fixes, and `SPEC-couples-semantics.md` §The ruling records the probe.
- **A walk the withdrawal never counted.** `manifest_files`' fallback branch walks **three** times:
  `canonical_specs` (`spec.rs:202`, a `find_named_pruning` over `CANON_KIT_SPEC_NAME` with prune
  `**/templates,docs/*`), then the README and CLAUDE finds. The consts carry grounds for the last two
  only. The `canonical_specs` walk is declared nowhere and hides behind the `.` root the configured
  branch declares, because unit test A compares roots by path. It is not a `?` today, so no census
  counted it. Delta 1 declares it.

## The seam

Kit mechanism: registry declarations, the ground-class set, one crate unit test, kit descriptor
couples and gate-sdk prose. The consumer side is this repo's cadence roster row, whose sub-class
retires. No consumer knob and no private rule content.

## What changes

### (1) Every fallback walk declares its root under its selector's guard {design-bearing}

`native/src/gates/mod.rs`. Each `?` with a `fallback` ground is replaced by a declared root whose
filter carries the `else:<SELECTOR>:` guard. The guard is already grammar
(`gates::filter_guard`, §check-reads-couples) and has had no live declaration until now. The
walker's discipline travels on the kind:

- **`MANIFEST_ROOTS`**, beside its two configured-branch roots:
  - `(".", "else:CANON_KIT_MANIFEST_FILES:name:knob:CANON_KIT_SPEC_NAME", "**/templates,docs/*", "")`
    for `canonical_specs`, whose walk records that prune, so assertion A's declared-prune-subset
    half holds it.
  - `(".", "else:CANON_KIT_MANIFEST_FILES:name:lit:README.md", "", "")`.
  - `(".", "else:CANON_KIT_MANIFEST_FILES:name:lit:CLAUDE.md", "", "")`.
- **`COMMENT_SURFACE_ROOTS`**: `(".", "else:CANON_KIT_COMMENT_SURFACE:ext:lit:sh,gate,rs", "", "")`.
- **`SPEC_POINTER_ROOTS`**: all four of the above.

Two narrowings the fallback applies are **not** declared, and both directions are safe. One is
`README.md`'s `/templates/` filter and the other is `prune_kit_roots` under
`CANON_KIT_SCAN_KIT_ROOTS`. Neither is a recorded directory prune, and leaving either undeclared
over-demands coverage, the direction §check-reads-couples' calibrations already take. Declaring one
the recorder never observed would be the unrecorded prune that section refuses.

The consts' comment, *the kit-literal fallback keeps `?`…*, is replaced by one binding: each branch
declares, the fallback under its selector's guard.

### (2) The `fallback` ground class retires {design-bearing}

`native/src/gates/mod.rs`: `GROUND_CLASSES` drops `"fallback"`, and the `?`-ground refusal message
drops it from its class list. `SPEC-packed-knob-projection.md` drops `"projection"` from the same
constant. `GROUND_CLASSES` today holds three members — `"fallback"`, `"dynamic"` and `"projection"`
— and `"dynamic"` is untouched by either amendment, so whichever of these two lands second finds a
two-member list (`"dynamic"` plus the other's already-dropped member) and removes the one member
that is not `"dynamic"`, leaving `GROUND_CLASSES` a one-member list only once both have landed.

gate-sdk/SPEC.md §check-reads-couples. **Not yet applied:**

- **The refusal paragraph.** In **The substrate answers, and may not answer `?` for a root it can
  bound**, the heading clause *— as far as the `couples=` semantics are settled, and no further* is
  deleted. So is *and whose coverage the author can then express* in its first sentence, because every
  statically resolvable root's coverage is now expressible (§Reading a `couples=` field's reach).
- **The two boundary paragraphs.** **The second half is the boundary, and it is a *withdrawal*
  rather than a narrowing** and **Read that boundary as a deliverable rather than a shortfall** are
  deleted and replaced by:

  > **A runtime-selected corpus helper declares every branch, and the kit-literal fallback is no
  > exception.** Each fallback walk declares its root under its selector's guard, so a tree that
  > configures the selector neither analyzes nor counts it, and a tree that leaves the selector empty
  > has its coverage asserted like any other walk's. The kit's literal couples must therefore cover
  > the fallback's selection on a tree the kit has never seen — which the field's one semantics makes
  > expressible, a `*<name>` or `*.<ext>` token containing every path a `name:` or `ext:` filter can
  > select at any depth — and the crate holds those literals to this tree before they ship
  > (below).
- **The class list.** The `fallback` bullet is deleted and the lead-in *Three classes are live* is
  corrected to the live count. The **figures** paragraph (*115 registry members, **32** …*) is
  re-measured by its own oracles at landing and restated. Its denominators move, and the paragraph
  already says the oracle wins.
- **The census paragraph**'s *three hand sweeps in one stage landed on 26* keeps its history.
  *The skip above has a population* becomes *The `?` set has a population*, because the skip it
  pointed at is the retired class's.

### (3) The kit literals the settled coverage exposes {mechanical}

The descriptors of the 14 members sharing the three consts. Roster probe: the const names in
`native/src/gates/mod.rs`'s registry entries. These are the canon-kit `.gate` files for
`check-manifest-count`, `check-prose-enum`, `check-manifest-temporal`, `check-install-claim`,
`check-payload-claim`, `check-docs-cmd`, `check-md-refs`, `check-tracking-claim`,
`check-knob-citation`, `check-spec-fence-balance`, `check-spec-pointer`, `check-comment-tier`,
`check-todo-task-liveness` and `check-deprecation-task`.

- **The four comment-surface readers** (`check-comment-tier`, `check-todo-task-liveness`,
  `check-deprecation-task`, `check-spec-pointer`) replace `scripts/*.sh,kit:*.sh` with `*.sh`. This
  is the measured miss: `installer/bin/checkwright.sh` and `installer/consumer-smoke/run-smoke.sh`
  are read by the fallback's `.sh` walk and reach none of their triggers. On this tree the gates take
  the configured branch, whose `knob:` token covers them, so the miss is an adopter's.
- **The eleven manifest readers** replace the literal `CLAUDE.md` with `*CLAUDE.md`. The fallback
  finds `CLAUDE.md` at any depth, and the exact token fires only on the root copy. No nested copy
  exists on this tree outside fixtures, so this is the tree-independent half of the same correction.
  `*README.md` and `*SPEC*.md` are already written that way, and `knob:CANON_KIT_SPEC_NAME` covers
  `canonical_specs` once `SPEC-couples-semantics.md` delta 4 converts it.

The generated hook and the coupling-graph artifact are regenerated.

### (4) The crate holds guarded kit literals to this tree {design-bearing}

`native/src/gates/reads_couples.rs` gains a unit test. For every registry root whose filter carries
an `else:` guard, it resolves that selector **empty** and runs `cover_root` over this repository's
tracked tree against the member's expanded couples. It reds naming the member, the root and each
uncovered path. It also reds if it analyzes no guarded root, so it cannot pass over nothing.

**Why a test and not the gate.** In this tree every guard's selector is configured, so
`check-reads-couples` never analyzes a fallback root here, correctly, because this tree does not
take that walk. Without the test a kit literal ships unasserted. The `installer/` miss is the
attested instance, and an adopter would meet it first.

**Its honest limit.** It runs over this tree's shapes. That a `*<name>` or `*.<ext>` token covers an
adopter's tree follows from the field's semantics, not from the test.

The test is stated in §check-reads-couples, beside the guard paragraph (**A declaration, a refusal
and a skip are each per walk**). **Not yet applied:**

> Because a configured tree never analyzes a guarded branch, a kit literal covering one would ship
> unasserted, so a crate unit test runs every guarded root's coverage with its selector resolved
> empty over the kit's own tracked tree, and reds on an uncovered read or on analyzing none.

### (5) The cadence row's second sub-class retires {mechanical}

`.workflow/audit-roster.txt`, row `unresolvable-walk-root`. The **SUB-CLASS B** passage is deleted:
the kit-literal fallback branches, 26 roots across 14 members, and the review question of whether
`couples-glob-semantics-unowned` has settled. The row then describes one population, read per walk.
The row's text is edited. Its `last:` stamp is not: that stamp is the close-stage review's, and
`audit-roster-last-stamp-author-unconstrained` is the attested case of a stage writing it early.

## Producers and consumers

- **The guarded declarations (delta 1).** The registry tuple produces them. `--reads` reports them.
  - `check-reads-couples`' `resolve_filter` reads the guard at run time and skips the root in a tree
    whose selector is set.
  - Unit test A holds each observed root and, for `canonical_specs`, the declared prune against the
    recorder on any fixture case taking the branch.
  - `--emit reads-census` stops printing 26 root-lines, and 14 members' `?` counts drop.
  - The locator assertion enumerates fewer sites.
  - Delta 4's test consumes every guarded root.
- **The narrowed ground-class set (delta 2).** Its reader is the locator assertion's class check
  and the refusal message. A seeded-locator test that seeds a `fallback` ground would red on a
  class that no longer exists, so build re-seeds it with a live class.
- **Readers' red conditions (point 5).** Deltas 1 and 2 shrink the `?` population, which is a
  narrowing of the skipped set:
  - `check-reads-couples`' clean line reports fewer skips, which has no red.
  - The census has no red.
  - Unit test A reds if a fixture case taking a fallback branch observes a root the new declarations
    do not carry. That can only mean a walk this amendment missed, which is the finding it exists
    for.
  - Delta 4's test reds on any uncovered guarded read. Delta 3's widenings exist so that it starts
    green.
  - `check-graph` assertions D and E red until the hook and artifact are regenerated after delta 3.
  - `check-gate-substrate-parity` assertion C: `*.sh` reaches gate declaration paths that `kit:*.sh`
    already reached for the same four members, so no member newly becomes substrate-sensitive. Build
    reads the verdict rather than trusting that.

## Existing sections updated

Roster probe: `git grep -n "fallback"` over `gate-sdk/SPEC.md`, `native/src` and `.workflow/`, and
`--emit reads-census` at HEAD, run at this stage.

- `native/src/gates/mod.rs` — the three consts and their comment, `GROUND_CLASSES`, the refusal
  message and any seeded `fallback` ground (deltas 1 and 2).
- `native/src/gates/reads_couples.rs` — the guarded-coverage unit test (delta 4).
- `gate-sdk/SPEC.md` — §check-reads-couples' refusal, boundary, class-list, figures, census and guard
  paragraphs (deltas 2 and 4).
- `canon-kit/checks/check-comment-tier.gate`, `canon-kit/checks/check-todo-task-liveness.gate`,
  `canon-kit/checks/check-deprecation-task.gate`, `canon-kit/checks/check-spec-pointer.gate` — `*.sh`
  (delta 3).
- `canon-kit/checks/check-manifest-count.gate` and the other ten manifest readers' descriptors,
  wherever `git ls-files '*/<name>.gate'` resolves each — `*CLAUDE.md` (delta 3).
- `scripts/git-hooks/pre-commit` — regenerated (delta 3).
- `docs/check-graph.html` — regenerated (delta 3).
- `.workflow/audit-roster.txt` — the `unresolvable-walk-root` row's sub-class B (delta 5).
- `docs/gate-sdk/SPEC.md` — the generated mirror, regenerated (all deltas).

## Retired spellings

- `fallback@src/spec.rs` — the ground spelling of every withdrawn root (delta 1).

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
