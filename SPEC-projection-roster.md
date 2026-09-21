# SPEC amendment: projection-roster

The generated-projections roster (docs/site-architecture.md §Generated
projections and their freshness gates) gives a later author the fan-out a new
projection or gate costs. Nothing asserts that the roster gains a row when a
projection lands. This amendment makes the set of projections machine-readable
at its source, the freshness gate's own descriptor. It then holds the roster to
that set in both directions with one generic gate. The declaration it mints is
also the input SPEC-projection-witness.md's witness reads, which is why the two
entries were filed to be read together.

**The survey, measured at this spec (2026-09-21).**

- **The roster is prose.** `grep -n "^- \*\*" docs/site-architecture.md` gives 13
  bold-led bullets. Seven describe a generated projection, and one of those rows
  covers two gates (enforcement and footprint). Two are parity contracts: the
  install-toolchain and install-platforms blocks, whose hand-authored content a
  gate compares against code, with no emitter behind them
  (`grep -n "toolchain\|platforms" native/src/emit/mod.rs` returns nothing).
  Three are advisory fan-outs (KPI roster, new tag-class member, new gate). One,
  the remedy blocks, is ungated by design. No bullet carries a key.
- **The freshness gates' descriptors already self-couple their output.** Each
  gate's `couples=` names the tracked path it byte-compares. That is still not a
  declaration, because an ordinary couple looks the same, so nothing can tell a
  projection's output apart from any other coupled surface. The members, read off
  their descriptors, are:
  - `check-docs-mirror-fresh`
  - `check-value-rollup-fresh`
  - `check-enforcement-fresh`
  - `check-footprint-fresh`
  - `check-trajectory-fresh`
  - `check-install-evidence-fresh`
  - `check-roadmap-fresh`
  - `check-graph`
- **Today both directions hold.** Every member above has a bullet that names it,
  and every projection bullet names a member. So the gate this amendment adds is
  green on landing, and it stays green only until the next projection forgets its
  row. The entry's "stands at 12" was the count at its filing. The bullet count
  moved to 13 with a row added later.
- **gate-sdk already names this family.** gate-sdk/SPEC.md §The first cohort, and the rule that selects the next,
  carries *The generated-projection freshness family*, a port-status
  classification listing six of the eight members. It is derived per member in
  prose, not declared.

It is a root-level amendment because it spans gate-sdk (the directive's grammar
and the generic gate), canon-kit (`check-comment-tier`'s directive roster), five
other kits' descriptors and this repo's `docs/` roster.

## What changes

**Batching.** Deltas 1 to 3 land together. The directive with no reader is the
self-declaration that §The `# graph:` manifest refuses, and the gate with no
directive has no population. Delta 4 is the consumer config that arms the gate.

### (1) The `# projection:` header directive {design-bearing}

**Not yet applied.** A gate's header may carry one
`# projection: <globs>` line, beside `# install:` and `# armed-by:`, in a `.sh`
declaration or a `.gate` descriptor on the same terms. It declares that the gate
**byte-compares the tracked paths `<globs>` names against a live emitter**, so
those paths are a generated projection and this gate is its freshness gate.
`<globs>` is comma-separated, in `couples=`' literal-glob syntax, and it carries
no `kit:` or `knob:` token, because an output path is the consumer's tree and not
a kit-relative read. It is read through a constant beside `registry::ARMED_BY`.

**It must be covered by the gate's own `couples=`**, under the one matcher
§Reading a `couples=` field's reach names. A projection whose output the gate
does not couple is a projection whose staleness the hook never triggers on. The
new gate's assertion A (delta 2) is the verifier. Without one, the line would be
the self-declaration §The `# graph:` manifest refuses.

**Point 6, each member's satisfying value.** The corpus is the eight gates the
survey enumerated. Each one's value is the path set its descriptor already
self-couples:

- `check-docs-mirror-fresh`: `docs/*/SPEC.md,docs/*/README.md,docs/doctrine-kit/DOCTRINE.md`
- `check-value-rollup-fresh`: `docs/value.md`
- `check-enforcement-fresh`: `docs/enforcement.md`
- `check-footprint-fresh`: `docs/footprint.md`
- `check-trajectory-fresh`: `docs/evidence-data.md`
- `check-install-evidence-fresh`: `docs/install-evidence.md`
- `check-roadmap-fresh`: `ROADMAP.md`
- `check-graph`: `docs/check-graph.html,scripts/git-hooks/pre-commit`

Build copies each value off the descriptor's own coupled output, and where a
descriptor couples a narrower spelling, that spelling wins.

**Two findings on `check-graph`'s descriptor, which build settles when it
writes that line.** Its `couples=` names `scripts/CHECK-GRAPH.html`, which
`git ls-files` does not track. It omits `scripts/git-hooks/commit-msg`, which is
tracked and generated by the same `--emit git-hooks --write`. Build finds which
gate byte-compares the `commit-msg` hook (`grep -rn "commit-msg"
native/src/gates/graph.rs`). If `check-graph` does, the hook joins both
`couples=` and the `# projection:` line, and assertion A is the check that keeps
the two in step. The untracked couple is dropped, with a debt bullet filed if
its removal is not trivially safe. The parity contracts
take no line, because no emitter produces them.

### (2) `check-projection-roster` holds a consumer's roster to the declared set {design-bearing}

**Not yet applied.** This is a new gate-sdk gate on the native substrate, with a
`good/`+`bad/` fixture pair and `# armed-by: GATE_SDK_PROJECTION_ROSTER`. Its
corpus is every registered gate's header, and the roster section the knobs name.

- **A.** Every `# projection:` glob is covered by its own gate's expanded
  `couples=`.
- **B.** Every registered gate carrying `# projection:` has **exactly one** row in
  the roster section carrying the key `<!-- projection: <gate> -->`. A row is a
  top-level bullet, and the key sits on the bullet's first line.
- **C.** Every key in the section names a registered gate that carries
  `# projection:`. A row whose gate was renamed, deregistered or stopped
  generating reds here.

Rows without a key are not constrained: advisory fan-outs, parity contracts and
ungated notes may sit in the same section.

**Point 5, the red condition.** A reds naming the gate and the uncovered glob.
B reds naming the gate and the count of keyed rows found, whether 0 or 2 or more.
C reds naming the key and its line. The clean line counts declaring gates and
keyed rows, so an armed roster over zero declarations prints `0` and not a
silent pass. Exit 2 applies when the roster file is absent, the section is not
found, or a header fails to parse.

**Knobs**, both gate-sdk static:

- `GATE_SDK_PROJECTION_ROSTER`: the roster file, default empty. Empty disarms
  the gate, which still prints its clean line.
- `GATE_SDK_PROJECTION_ROSTER_SECTION`: the heading of the section holding the
  rows, default empty, meaning the whole file.

The file path is config, never a kit literal, as `gates-must-not-bind-to-document-paths`
rules. So the kit ships the mechanism and this repo supplies its roster
location.

### (3) The directive's rosters learn the name {mechanical}

**Not yet applied.**

- canon-kit's `check-comment-tier` built-in directive roster (`SHELL_COLON`,
  `native/src/gates/comment_tier.rs`) gains `projection:`. Without it, the new
  header line reds as an undirected comment on every descriptor that carries it.
- gate-sdk/SPEC.md gains the directive paragraph beside the arming-declaration paragraph of §The install disposition.
- the family paragraph of §The first cohort, and the rule that selects the next, is re-phrased to say the family is now
  declared by `# projection:`, not derived in prose.

Mechanical: the wording is fixed by delta 1.

### (4) This repo declares its eight members and keys its roster {mechanical}

**Not yet applied.**

- The eight descriptors gain their `# projection:` line with delta 1's values.
- `scripts/gate-sdk-config.knobs` sets `GATE_SDK_PROJECTION_ROSTER =
  docs/site-architecture.md` and the section knob to `Generated projections and
  their freshness gates`.
- `scripts/gates.list` registers `check-projection-roster`.
- Each of the seven projection bullets gains its key or keys. The enforcement and
  footprint bullet carries both, which means B's "exactly one row per gate" holds
  while a row can serve two gates.
- The new-gate fan-out bullet (docs/site-architecture.md, "the other wide
  trigger") names `# projection:` as the step that makes a new projection's row
  mandatory.

Mechanical: every value is enumerated above.

## Producers and consumers

- **`# projection:` (delta 1).** Producer: a gate author, in the descriptor.
  Consumers:
  - `check-projection-roster`, assertions A to C;
  - SPEC-projection-witness.md's witness, which reads the same declaration to
    choose its members;
  - `check-comment-tier`, a roster-holding reader, which gains the name
    (delta 3).

  No hook or graph reader reads it, since it is not a `# graph:` field. So
  `check-graph`'s parity is untouched.
- **`GATE_SDK_PROJECTION_ROSTER` and `_SECTION` (delta 2).** Producer: gate-sdk's
  static knob table defaults, and this repo's knob file (delta 4). Consumer: the
  gate. Roster-holding readers:
  - `check-knob-citation`, satisfied by the SPEC row delta 2 writes;
  - `check-install-disposition` assertion D, which verifies `# armed-by:` names
    a declared static knob, satisfied by the knob's table row;
  - doctor's disarmed-member line, which gains the gate on a fresh install with
    no roster set. That is the intended adopter signal.

  Each knob names a path, so the gate's `couples=` carries
  `knob:GATE_SDK_PROJECTION_ROSTER` beside its descriptor-glob literal.
- **`check-projection-roster` (delta 2).** Producer: its `.gate` descriptor
  with `# install: zero-config`. It is armed by the knob, so it is clean
  everywhere until a consumer points it at a roster. Consumers: the battery, the
  hook (`tier=precommit`, since a missing row is restorable in the commit that
  adds the projection), and gate-sdk/README.md's gate roster block.
- **The row key (delta 4).** Producer: the roster's author. Consumer: assertions
  B and C. The key has one field, the gate name, and B and C both read it.

## Existing sections updated

Rosters produced by `grep -n "^- \*\*" docs/site-architecture.md`, by reading the
eight descriptors' `couples=` lines, by
`grep -rn "armed-by:" native/src --include=*.rs` (which finds the directive
constant at `native/src/registry.rs:143` and the comment-tier roster at
`native/src/gates/comment_tier.rs:17`), and by reading gate-sdk/SPEC.md
§The `# graph:` manifest, the arming-declaration paragraph of §The install disposition, and §The first cohort, and the rule that selects the next.

- gate-sdk/SPEC.md: §The install disposition, beside the arming declaration (the new directive),
  a new §check-projection-roster, §Layout and configuration's knob roster, and
  the family paragraph of §The first cohort, and the rule that selects the next, (deltas 1 to 3).
- `native/src/registry.rs` (the directive constant), a new gate module and its
  `REGISTRY` row, `native/src/knobs/gate_sdk.rs`, `gate-sdk/checks/check-projection-roster.gate`,
  its fixture pair, and gate-sdk/README.md's gate roster (delta 2).
- `native/src/gates/comment_tier.rs` and canon-kit/SPEC.md §check-comment-tier's
  built-in roster sentence (delta 3).
- The eight descriptors, `scripts/gate-sdk-config.knobs`, `scripts/gates.list`,
  and docs/site-architecture.md §Generated projections (delta 4).
<!-- update-target-exempt: generated mirrors and projections, regenerated by their freshness gates' printed commands, never hand-edited -->
- `docs/gate-sdk/SPEC.md`, `docs/gate-sdk/README.md`, `docs/canon-kit/SPEC.md`,
  `docs/enforcement.md`, `docs/check-graph.html`, `scripts/git-hooks/pre-commit`.

## Retired spellings

- None — no name is removed. The family's prose derivation is re-phrased, and no
  identifier, knob or file is renamed.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only.** The roster's new-gate bullet
      names the step and not its grounds.
- [ ] **Merged with no information lost.** Each addition re-phrases the text it
      refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`), and
      SPEC-projection-witness.md's filename citation is repointed.
- [ ] **Entry moved.** `generated-projections-roster-ungated` moves to Done in
      the merge commit, at a stage before the drain stage.
- [ ] **Fails closed.** The `bad/` fixture reds on each of these:
      - an uncovered output glob;
      - a declaring gate with no row;
      - a declaring gate with two rows;
      - a key naming a non-declaring gate.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block
      above.
- [ ] **Gaps filed.** Any cross-component gap build discovers is resolved that
      session.
