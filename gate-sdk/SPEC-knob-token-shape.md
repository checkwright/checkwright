# SPEC amendment: knob-token-shape

Two entries pair this amendment. A `couples=` `knob:` token can name a knob whose
shape the token grammar cannot represent, and nothing reds on it: the attested
instance, `knob:GATE_SDK_KIT_DIRS`, sat inert through a whole build stage. And a
consumer-shaped regression is invisible to every oracle a build stage runs,
because they all see this tree's configuration only. **This amendment makes the
unrepresentable shape a static `check-graph` finding. It adds a build-time unit
test that emits the hooks under every shipped install profile. It states the
upgrade-shaped half as having no build-time form.**

**Why the tightening is on the row's shape and not on an empty expansion.** The
first entry proposed making a zero-member expansion a manifest finding. Measured
2026-09-21, that reds a stock consumer. Ten indexed knobs named by `knob:` tokens
ship with an empty kit default by design: every canon-kit prose-surface and
manifest knob, `CONTEXT_KIT_RATCHET_PATHS`, `GUARD_KIT_DOOR_ROOTS` and
`QUEUE_KIT_*_SURFACE_GLOBS`. Two scalars do too, `GATE_SDK_PROJECTION_ROSTER` and
`LIFECYCLE_KIT_AUDIT_ROSTER_FILE`. The probe read each `Row::` default in
`native/src/knobs/*.rs` against every kit `templates/*.knobs`, and every template
sets none. guard-kit's template states the empty state as intended. The rule that
the token rides *beside* a descriptor's literal globs (§The `# graph:` manifest) is
what makes an empty expansion lawful. That empty expansion is also not what failed.
`GATE_SDK_KIT_DIRS` is a `Row::scalar` holding a whitespace-joined list. Unset, it
expands to nothing. Set to one kit, it expands to one pattern. Set to a real list,
the expansion refuses. The defect is the shape, and the shape is static, so the
finding can be too: it is config-independent and reds in this tree.

**Why the unit test, and why it covers half the class.** Emitting the pre-commit
hook takes about 12 ms, measured unset and under the `delegation` profile's
eight-kit `GATE_SDK_KIT_DIRS`. So a test emitting under every profile costs nothing,
rides `check-crate-arms`'s `cargo test` at commit time and mints no name. It
would have caught the second entry's regression 1 at commit time. It does not reach
regression 2. That regression's shape is a TO release's kits over a FROM tree, and
it needs two refs vendored into one scratch consumer, which is `upgrade-smoke`'s
whole job. The entry's own text places the cheap leg in `bin/build-native.sh`, but
that script builds and never tests. The commit-time `cargo test` is
`check-crate-arms`'s (§check-crate-arms).

## What changes

**Batching.** Delta 1 lands before or with delta 2, since the finding reads the marker.

### (1) A scalar row declares that it holds a word list {design-bearing}

**Not yet applied.** `Row` gains a `.words()` builder beside `.empty_takes_default()`,
declaring a `Shape::Scalar` row whose value is a whitespace-joined list. Two holds
keep the declaration true:

- `walk::knob_words`, the reader that splits such a value, refuses a row not
  declared `.words()` at exit 2, naming the row. A list-scalar read through the
  splitter cannot go undeclared.
- A crate unit test in `native/src/knobs/` asserts that every scalar row whose
  static default contains whitespace is declared `.words()`.

**The members, enumerated by probe.** The scalars read through `knob_words`
(`grep -rhoE 'knob_words\("[A-Z0-9_]+"' native/src`) are `GATE_SDK_EXEC_GLOBS`,
`GATE_SDK_EXEC_PRUNE`, `GATE_SDK_GRAPH_EXTERNAL_REFS`, `GATE_SDK_LINT_EXTRA_DIRS`,
`GATE_SDK_MSG_PATTERN_FILES`, `GATE_SDK_MSG_PATTERN_FILES_LOCAL`,
`GATE_SDK_NATIVE_PUBLISH_WORKFLOW`, `GATE_SDK_PORTABILITY_PATHS`,
`GATE_SDK_PORTABILITY_PATTERNS`, `GATE_SDK_PRUNE_DIRS` and
`GATE_SDK_PRUNE_EXTRA_DIRS`. Three more whitespace-list scalars are split by their
own readers: `GATE_SDK_KIT_DIRS` (`walk::roots_at`), `GATE_SDK_PAYLOAD_WITHHOLD`
(default `SPEC.md smoke`) and `CONTEXT_KIT_MEMORY_DIRS`
(`native/src/gates/memory_off.rs`). They were found by
`grep -rn -B3 split_whitespace native/src` filtered to a knob read within three
lines, plus a read of the two walk and gate readers that split a value fetched
earlier. Each of the fourteen members' satisfying value is the `.words()`
declaration. **Honest limit:** a word-list scalar with an empty default, split
outside `knob_words`, is held by review. `GATE_SDK_KIT_DIRS`, the attested
instance, is that shape, and it is declared by hand here.

### (2) `check-graph` reds a `knob:` token naming a word-list row {design-bearing}

**Not yet applied.** In the live-registry manifest loop, beside the existing
admissibility check (`native/src/gates/graph.rs`), a `knob:<NAME>` token in
`couples=` or `trigger=` whose row is declared `.words()` is a MANIFEST finding. The
finding says the row's members carry whitespace after expansion, so the token is
inert when unset and refused when set, and that the descriptor should name the
corpus by literal or by an indexed knob. The verdict reads the row table only, so it
is the same on every tree.

Today the finding reds nothing: no descriptor carries such a token
(`grep -rn 'knob:GATE_SDK_KIT_DIRS\|knob:GATE_SDK_PAYLOAD_WITHHOLD' --include=*.gate .`
returns nothing), since the one shipped instance was removed at `4c8411ae`. The
fixture pair gains a `bad/` descriptor carrying `knob:` on a `.words()` row, with its
`expect.txt` line.

**The runtime empty expansion stays lawful, and the registry's diagnostic stops
saying otherwise.** In `registry::expand_couples`, the resolution-error message's
clause "an empty expansion would be a lost trigger" becomes "an unresolvable knob
would be a lost trigger". The message fires on a resolution error only, and the
unit test pinning an `Ok` empty expansion stays.

### (3) The hooks are emitted under every shipped install profile at commit time {mechanical}

**Not yet applied.** It adds a `#[test]` in `native/src/emit/git_hooks.rs`'s test
module. For each profile in `installer/profiles.list`, and for the `full` profile
derived from the kit roots, it calls `pre_commit` with `GATE_SDK_KIT_DIRS` set to
that profile's kits and asserts `Ok`. The knob is set through a knob file in a
temporary gates dir, never the process environment, so parallel tests cannot race
on it. It runs inside `check-crate-arms`'s `cargo test`. No gate, arm or registration
changes.

### (4) §The `# graph:` manifest, §check-graph and §check-crate-arms state the rules {mechanical}

**Not yet applied.**

- §The `# graph:` manifest: in the paragraph **Which knob *shapes* survive that
  grammar is the corollary**, replace the sentence "A `knob:` on such a row is
  therefore never right, and the row's shape — not the knob's name — is what decides
  it." with: "Such a row declares `.words()`, and `check-graph` reds a `knob:` token
  naming one, so the rule is an oracle rather than a reader's duty." In the
  **Resolution order is fixed** paragraph, the closing clause about a set-empty knob
  gains: "which is a consumer's designed state for every kit-empty indexed knob, so
  it is never a finding."
- §check-graph: the manifest-loop paragraph **The live-registry manifest loop
  asserts the `knob:` token's admissibility** gains one sentence for delta 2's
  finding.
- §check-crate-arms: one sentence names delta 3's profile test and its limit. The
  test covers a consumer-*shaped* kit set over this tree's kits. A defect whose shape
  is one release's kits over another's tree has no build-time form, and
  `upgrade-smoke` at validate is its earliest oracle.

## Producers and consumers

- **`.words()` (delta 1).** Producer: the row declarations in
  `native/src/knobs/gate_sdk.rs` and `native/src/knobs/context_kit.rs`.
  Consumers: `walk::knob_words`' refusal, the static-default unit test, and delta 2's
  finding. The marker adds no knob, so `--emit knob-roster`, `check-knob-citation`
  and every `templates/*.knobs` are untouched.
- **The finding (delta 2).** Producer: `check-graph`, registered at `precommit`.
  Consumer: the battery and hook through check-graph's existing finding channel. No
  roster gains a name. **Release declaration:** `check-graph` is kit-shipped and gets
  stricter, so build adds a `check-graph` bullet to the Tightened-gates section of
  `.workflow/release-declarations.md`. The bullet's remedy is to drop the token and
  keep the descriptor's literal globs.
- **The profile test (delta 3).** Producer: `check-crate-arms`, triggered by crate
  source edits, as today. It reads `installer/profiles.list` beside the crate, so the
  test is inert in a tree without the installer. Such a tree has no crate to test
  either.
- **Point 5.** Nothing narrows. The tightening reds on a found token and nothing
  else.
- **Point 6.** Delta 1's members and their value are enumerated above. Delta 3's
  corpus is the profile roster, `starter`, `delegation` and `prose` plus the derived
  `full`, and each one's value is `pre_commit` returning `Ok`.

## Existing sections updated

Rosters from `grep -n "knob:" gate-sdk/SPEC.md`, the two `grep` probes above,
`grep -n "profiles.list" installer/SPEC.md`, and reading §check-crate-arms.

- gate-sdk/SPEC.md §The `# graph:` manifest, §check-graph and §check-crate-arms
  (delta 4).
- `native/src/knobs/mod.rs`, `native/src/knobs/gate_sdk.rs`,
  `native/src/knobs/context_kit.rs` and `native/src/walk.rs` (delta 1).
- `native/src/gates/graph.rs`, `native/src/registry.rs`,
  `gate-sdk/gate-tests/check-graph/` and `.workflow/release-declarations.md`
  (delta 2).
- `native/src/emit/git_hooks.rs` (delta 3).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`.

## Retired spellings

- None — no name is removed; one builder and one finding are added.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The
      causal-completeness check holds for the marker, the finding and the test.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** The shape corollary's grounds survive
      the rewrite.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entries moved.** `couples-knob-token-empty-expansion-passes-silently` and
      `consumer-shaped-regressions-invisible-to-build-oracles` move to Done in the
      merge commit, at a stage before the drain stage.
- [ ] **Fails closed.** `bad/` reds on a `knob:` token naming a `.words()` row, and
      `knob_words` on an undeclared row exits 2 under a unit test.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block
      above.
- [ ] **Gaps filed.** Any cross-component gap build discovers is resolved that
      session.
