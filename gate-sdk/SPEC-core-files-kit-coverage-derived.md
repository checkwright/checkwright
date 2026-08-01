# SPEC amendment: core-files-kit-coverage-derived

`scripts/core-files.list` carries a block headed *"One SPEC.md per kit (each
kit's canonical contract)"* listing **nine** kit SPECs. `gate_kit_roots` returns
**eleven** roots. `site-kit/SPEC.md` and `doctrine-kit/SPEC.md` are absent, as is
`doctrine-kit/DOCTRINE.md`, which `CANON_KIT_MANIFEST_FILES` does carry.

The heading states a per-kit invariant that the block below it does not hold,
and `check-core-files` asserts only that *listed* paths exist and are tracked —
never that a derivable set is covered — so the block degrades silently as kits
land and no battery goes red. This is the maintain-a-derivable antipattern:
`gate_kit_roots × SPEC.md` is computed, and the manifest keeps a hand copy of it.

**Backfilling the two names is refused as the fix.** It restores the invariant
for exactly as long as it takes kit twelve to land, and re-arms the identical
drift. That is the defect, not the remedy.

## The diagnosis the entry did not have

The entry frames the deliverable as *a derived coverage assertion* and asks two
open questions: what slice the derived set owns, and whether the assertion
belongs inside `check-core-files` or in a sibling gate that would mint a new
name. The survey behind this amendment found that **both questions are
dissolved by a token gate-sdk already ships**, and that a coverage assertion is
the wrong shape.

`gate_expand_couples_var` (`gate-sdk/lib/gate.sh`) expands a **`kit:<glob>`**
token to `<kit-root>/<glob>` for every `gate_kit_roots_rel` member. It exists
for `# graph:` `couples=` fields, and `check-kit-enum`'s help text already names
it as the correct fix for precisely this class of defect — *"the fix its help
text names is the token, not a longer list"*, gate-sdk/SPEC.md §check-kit-enum,
whose entire subject is a hand list that should have been a derivation.

So the manifest does not need a *check that its hand list covers the derived
set*. It needs to stop holding a hand list. Once the nine lines become
`kit:SPEC.md`, the coverage property is true by construction and there is
nothing left to assert — the gate's existing single invariant, applied to the
expanded set, is the whole guarantee.

That is the stronger move under the enforcement-first doctrine's own ordering:
**removing the duplication outranks gating it.** A coverage assertion would have
gated a duplication this amendment deletes instead.

## What changes

### Delta 1 — the core-files manifest grammar admits the `kit:` token *{design-bearing}*

`gate-sdk/SPEC.md` §check-core-files gains the token to its manifest grammar. A
manifest line is a repo-relative path **or** a `kit:<path>` token, which the
gate expands via the existing `gate_expand_couples_var` to one path per
`gate_kit_roots` member before running its existence-and-tracked test. Comments
and blanks are unaffected.

**This is the token gaining a second reader, not a new token.** The spelling,
the expansion, and the root set are gate-sdk's already; nothing about them is
re-specified here. What is new is the sentence naming `check-core-files` as a
consumer of the expansion.

**One limit, ruled here so build does not discover it.** The expansion is
unconditional — it emits `<root>/<glob>` per root with no existence test. In a
`couples=` field the result is matched as a glob against tracked files, so a
wildcard is meaningful there. In this manifest the expansion must name a path
the gate *requires to exist*, and "every kit has at least one file matching
`checks/*.sh`" is a different invariant from "this path exists and is tracked".
A `kit:` token carrying a wildcard is therefore **refused fail-closed (exit 2)**
by this reader, with a message naming the limit. One expansion, two readers, one
stated restriction — not divergent semantics.

**Backward compatibility is total.** A manifest with no `kit:` line behaves
exactly as today, so no consumer's manifest changes meaning. The absent-manifest
and empty-manifest clean paths are untouched.

### Delta 2 — the gate re-fires on the derivation's inputs, not only on the manifest *{design-bearing}*

`check-core-files`'s manifest today is `couples=scripts/core-files.list`, which
is complete while the listed set is hand-written: nothing but a manifest edit can
change what the gate must check. Once a line derives from `gate_kit_roots`, a
**new kit landing** changes the checked set without touching the manifest.

The gate's `# graph:` therefore gains `kit:SPEC.md` to its `couples=` field, so
staging any kit's canonical spec — including a twelfth kit's, on the commit that
creates it — re-fires the gate. The token is used in its native reader here, so
this adds no vocabulary; and `check-kit-enum` does not engage on it (it engages
only on literal multi-root hand lists, and `kit:` tokens are the fix it
recommends, not a case it polices).

This delta is what makes Delta 1 a guarantee rather than a convenience: without
it, a new kit's missing SPEC would be caught only by the whole-tree battery, one
tier out from where the perturbation happened.

### Delta 3 — the manifest's kit block becomes the derivation *{mechanical}*

`scripts/core-files.list`'s nine-line kit-SPEC block is replaced by the single
line `kit:SPEC.md`, and its heading comment is rewritten so it states the
derivation rather than a claim a reader must verify by counting. Purely
mechanical once Delta 1 lands: the ruling is fixed and the edit executes it.

### Delta 4 — the boundary, stated as a rule and demonstrated by the remainder *{design-bearing}*

The entry's first open question: which slice the derived rule owns, so it does
not over-reach into a roster that is legitimately hand-held. The manifest holds
38 entries; nine are the kit-SPEC block and 29 are not.

**The rule: the manifest derives a surface that is uniform across kit roots, and
hand-lists everything else.** Uniformity is not a taste judgment here — it is
decidable by running the expansion, which is what makes this a rule rather than a
convention:

- **`kit:SPEC.md`** expands to eleven paths, all of which exist. Derivable;
  Delta 3 takes it.
- **`doctrine-kit/DOCTRINE.md`** stays a hand line, and this is *proved* rather
  than preferred: `kit:DOCTRINE.md` would expand to eleven paths of which ten do
  not exist, so the token cannot express it. It is a single-kit deliverable with
  no per-kit counterpart. It is **added** to the manifest as a hand line — it is
  in `CANON_KIT_MANIFEST_FILES` and absent here, which is a real pin gap
  independent of the derivation — carrying a comment stating why it is hand-held,
  so the next reader does not try to derive it.
- **The other 29** — the queue and state files, the repo-root chrome, the
  governed repo-meta and GitHub template instances, the workflows, the installer
  entry points, the gate registry and checked projections, the validate baseline
  and evidence manifest — reach no `gate_kit_roots`-shaped derivation. They stay
  an honest hand list, and the manifest says so.

**Kit READMEs are ruled out of the manifest.** `kit:README.md` would expand
cleanly, so the token *could* express it — this is the case the rule above does
not settle on its own, and it is refused on the manifest's purpose instead. The
gate's own help text names its class: *the silent-deletion class downstream gates
catch only incidentally.* A kit README's deletion is caught **directly**:
`check-readme-roster` reds a kit root that has `checks/` but no marker block in
its README, and all eleven roots have `checks/`. Pinning it would add eleven
lines and no guarantee. `gate-sdk/SPEC.md` §When a gate earns its place's dual
rule — minimize the standing set subject to the guarantee being preserved —
applies to manifest lines as it does to gates.

## Producers and consumers

This amendment adds a **grammar element** to an existing optional consumer
config, and a coupling token to an existing gate. It adds no gate, no knob, no
state file, and no message.

**The `kit:` token in a core-files manifest** (new interface, Delta 1).
*Producer:* the consumer, hand-writing the line into `core-files.list` — the
same author and the same file that produces every other manifest line today. Its
*enabling configuration* is the manifest itself, which every clone of this repo
carries and which `check-core-files` already reads on every pre-commit run, so
the producer is not a path only fixtures reach.
*Consumer:* `check-core-files.sh`, at the point it materializes `paths[]` from
`gates_list_members`, expanding each `kit:` line through
`gate_expand_couples_var` before the existence-and-tracked loop.
*Expansion input:* `gate_kit_roots_rel`, whose own producer is the kit-root scan
(a sibling directory holding `checks/` or `smoke/`) or the `GATE_SDK_KIT_DIRS`
override — an existing, specified, already-consumed derivation. This unit does
not change what a kit root is.

**Scope of the new reader, stated because it bounds the blast radius.** The
expansion is applied **inside `check-core-files.sh` only**, never inside
`gates_list_members`. That parser is shared with `gates.list`; teaching it the
token would silently change how the gate registry itself parses, which is a
consequence no delta here claims and no reader wants.

**The `kit:SPEC.md` coupling token** (changed manifest, Delta 2).
*Producer:* the gate's own `# graph:` line.
*Consumers:* `check-graph`, which validates and expands `couples=` fields, and
`gen-pre-commit`, which projects the expanded trigger set into the generated
hook. Both consume `kit:` tokens today; neither gains a capability here.

**Every field has a named reader.** The unit adds one field — the `kit:` line in
the manifest — read by `check-core-files` at the transition above. No field is
added that no code path reads.

**Enforcement.** The guarantee is delivered by the existing gate's existing
invariant applied to a larger, derived set; no new gate is earned, and none is
minted. Under `gate-sdk/SPEC.md` §When a gate earns its place the question does
not arise: there is no remaining consistency property to check once the
duplication is gone.

**Exposure, established rather than assumed — the entry asked for this
explicitly.** The entry costed the derivation-first defect as confirmed and the
safety hole as unverified, and asked whoever picked it up to establish the real
exposure. Traced statically across the roster: a silent `rm site-kit/SPEC.md`
today reds **only** `check-docs-mirror-fresh`'s orphan branch, because the
committed `docs/site-kit/SPEC.md` mirror page loses its source. It is *not*
caught by `check-core-files` (unlisted), by `check-kit-registration` or
`check-docs-kit-parity` (kit-root membership keys on `checks/`/`smoke/`, not on
`SPEC.md`), by `check-gate-assertions` (an absent SPEC drops out of its array),
or by canon-kit's manifest finders (they enumerate what exists rather than
asserting what must). So the hole is **narrow, not absent** — and the coverage
that closes it is **incidental**: it is a side effect of the docs mirror, and it
would vanish for any kit that ever left the mirror. That is the honest costing.
The derivation-first defect remains this unit's primary justification, exactly as
the entry stated; the pin converts an incidental catch into a direct one.

## Existing sections updated

- **`gate-sdk/SPEC.md` §check-core-files** — the manifest grammar gains the
  `kit:<path>` token and its wildcard restriction (Delta 1); the `# graph:`
  coupling sentence is updated for the added token and the reason it is needed
  (Delta 2); the boundary rule and the kit-README exclusion are recorded (Delta
  4). This is the amendment's whole surface within the kit.
- **`scripts/core-files.list`** — the kit block becomes `kit:SPEC.md`,
  `doctrine-kit/DOCTRINE.md` is added as a hand line, and both comments are
  rewritten (Deltas 3–4). Consumer config, not kit content.
- **`gate-sdk/checks/check-core-files.sh`** — the expansion step and the
  wildcard refusal (Deltas 1–2), plus the `# graph:` line.
- **`gate-sdk/SPEC.md` §lib/gate.sh and §check-graph** — **not edited.** They
  own the token and its expansion; this unit adds a reader and must cite them
  rather than restate them. Listed so build does not helpfully re-document the
  token in two places.
- **`gate-sdk/gate-tests/check-core-files/`** — the fixture pair gains coverage
  for an expanding `kit:` line and for the wildcard refusal (Delta 1, whose
  grammar and fail-closed restriction are what the pair exercises), per the
  fixture-pair discipline.

## The seam

Ruled explicitly, per CLAUDE.md §The provenance seam.

**The kit ships the grammar; the consumer ships the roster.** gate-sdk gains the
ability to *express* a per-kit derivation in a core-files manifest. It gains no
knowledge of which surfaces this or any project pins — `SPEC.md`,
`DOCTRINE.md`, and the other 29 entries are all consumer content in
`scripts/core-files.list`, which §check-core-files already specifies as optional
consumer config in the `graph-vocab.sh` pattern. **No kit literal names a
Checkwright surface**, and a consumer whose kits carry no `SPEC.md` writes no
`kit:` line and sees no behavior change.

**No knob is introduced, deliberately.** The obvious alternative to a manifest
token is a `GATE_SDK_CORE_FILES_DERIVED`-shaped knob listing per-kit basenames
to pin. It is refused on two counts: it would put in env config a fact the
manifest is already the home for, giving the pinned set two sources, and it
would be a second spelling of a derivation gate-sdk already expresses one way.
The existing `GATE_SDK_KIT_DIRS` and `GATE_SDK_CORE_FILES_FILE` knobs are
sufficient and unchanged.

**Nothing moves up from the consumer either.** No rule content leaves
`scripts/core-files.list`.

## Definition of Done

- [ ] **Causal completeness** — the `kit:` manifest line has a named producer
      (the consumer's manifest, read every pre-commit run) and a named consumer
      (`check-core-files.sh` at path materialization); the coupling token's
      consumers (`check-graph`, `gen-pre-commit`) are existing readers; no field
      is added without a reader.
- [ ] **Merged with no information lost** — the grammar addition lands inside
      §check-core-files's existing manifest paragraph rather than as an appended
      note, and the boundary rule reads as part of that section's calibration.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      gate-sdk (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — the nine hand lines are gone, the false per-kit
      heading is gone, and no surface still describes the kit block as a
      maintained list.
- [ ] **Projections regenerated** — the `# graph:` change stales the generated
      pre-commit hook and the graph artifact; the SPEC edit stales the docs
      mirror, the footprint, and the value rollup. Each gate names its own regen
      command on red (docs/site-architecture.md §Generated projections).
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed via
      `bash lifecycle-kit/bin/file-gap.sh`. (The kit-README question is settled
      in Delta 4, not filed.)
