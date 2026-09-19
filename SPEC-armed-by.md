# SPEC amendment: armed-by

A vendored consumer gets `check-portability-floor` registered at `init`, because
it is `zero-config`. It asserts nothing until `GATE_SDK_PORTABILITY_PATHS` names
their install path. The degradation itself is ruled and stays: a roster with no
corpus scans nothing, and the kit cannot know an adopter's install path
(gate-sdk/SPEC.md §check-portability-floor). What is missing is the on-ramp.
Neither `init`, `doctor` nor the docs tell the adopter that setting the knob is
what arms the gate.

Three answers were on the table:

- Seed the roster with an empty corpus.
- Have `doctor` report the disarmed state.
- Add a paragraph under docs/install.md §Requirements.

Only the second reaches an adopter who never opens a SPEC, so this amendment
takes it. Doctor must not carry a gate or knob name of its own (the installer's
recipe is held free of literal gate names by `check-install-disposition`
assertion C, and doctor sits in the same module family). So the gate declares
its arming knob, and doctor reads the declaration.

It is a root-level amendment because it spans gate-sdk (the directive, the gate's
header, `check-install-disposition`), the installer's `doctor`, and canon-kit's
built-in directive roster.

The tree does not already do this. `grep -rn "armed-by" --include=*.rs --include=*.gate --include=*.md .`
finds nothing outside this file, and `doctor::diagnose`'s installed block reports
identity, registry, artifact and omissions only.

## What changes

**Batching.** All four deltas land in one commit. The header line in delta 2 reds
`check-comment-tier` until delta 3's roster entry exists, and it reds
`check-install-disposition` until that gate knows the directive.

### (1) The arming declaration {design-bearing}

**Not yet applied.** A gate's header may carry one `# armed-by: <KNOB>` line,
beside `# install:`, in a `.sh` declaration or a `.gate` descriptor on the same
terms. It declares that the gate **asserts nothing while that knob resolves
empty**. The gate still runs and still prints its clean line. `<KNOB>` is one
static knob of the declaring kit's own table.

The declaration is gate-sdk/SPEC.md §The install disposition's sibling, and it
is recorded there. `zero-config` answers whether a gate can register on the tree
`init` makes. `armed-by` answers whether a registered gate is doing anything yet.
The two are independent, so the directive does not widen the disposition
vocabulary.

**Held by `check-install-disposition` assertion D.** Every `checks/` member
carries at most one `armed-by:` line. Its value is a static knob that
`knobs::owner` places in the kit whose root the gate sits under. The assertion is
the verifier that keeps the line from being the self-declaration §The `# graph:`
manifest refuses: a misspelled or foreign knob is a red, never a silent no-op.
The fixture pair gains the arm:

- `good/` has a member declaring a real knob of its kit.
- `bad/` has one naming a knob no kit owns.

The member that declares it in this unit is `check-portability-floor`, with
`# armed-by: GATE_SDK_PORTABILITY_PATHS`.

**Honest limit.** Other members may degrade the same way. No census of them is
claimed here, and one takes the declaration when its own disarmed state is found.
That census is filed to the gap inbox at this stage.

### (2) `check-portability-floor` declares its arming knob {mechanical}

**Not yet applied.** `gate-sdk/checks/check-portability-floor.gate` gains
`# armed-by: GATE_SDK_PORTABILITY_PATHS` in its header block. The
honest-limit paragraph of gate-sdk/SPEC.md §check-portability-floor
(*an adopter who deletes the roster silently disables the gate*) is rewritten.
The detail line still bounds the degradation, and `doctor` now names it (delta
4), so an adopter meets the cue without reading a clean line closely.

### (3) The directive joins canon-kit's built-in roster {mechanical}

**Not yet applied.** `armed-by:` joins `SHELL_COLON` in
`native/src/gates/comment_tier.rs`, and joins canon-kit/SPEC.md
§check-comment-tier's machine-directive list. It is read by `doctor` and by
`check-install-disposition`, and minted by gate-sdk/SPEC.md §The install
disposition. The roster entry is built-in rather than a consumer extra for the
reason `portability-declared:` is: the directive is kit mechanism.

### (4) doctor names a registered gate that is disarmed {design-bearing}

**Not yet applied.** In `doctor::diagnose`'s installed block, after the omitted
block, doctor goes through each member (`registry::members`) of the recorded
registry. For each one it resolves the declaration the battery would run
(`registry::resolve` over `registry::resolve_dirs` for that tree's gates
directory and kit roots) and reads its `armed-by:` line. It then resolves that
knob the way the battery resolves it for the tree, from the repository root. A
knob that resolves empty renders one line:

`  disarmed     check-portability-floor asserts nothing until GATE_SDK_PORTABILITY_PATHS is set`

The line reports without setting the exit status and leaves the verdict line
unchanged. It describes configuration the adopter has not written yet, which is
neither a machine below contract nor an install that failed to verify. So it
belongs with the omitted block's class, not the artifact finding's.

A member whose declaration does not resolve, or whose knob refuses to resolve,
renders nothing here: the battery reports both at its own run, and doctor is not
a second runner.

Unit cases:

- A registry naming a declaring member with the knob unset renders the line.
- The same member with the knob set renders nothing.
- A member with no `armed-by:` line renders nothing.
- The verdict is `0` in all three.

## Producers and consumers

- **The `armed-by:` line (delta 1).** Producer: a kit author writing the header.
  In this unit that is gate-sdk's `check-portability-floor` (delta 2), which
  ships to every adopter because it is `zero-config`. Consumers:
  - `doctor`'s installed block (delta 4). It reads the knob name, resolves it and
    renders on empty.
  - `check-install-disposition` assertion D. It reds on a second line, or on a
    value `knobs::owner` does not place in the declaring kit.
  - `check-comment-tier`. It reds on the line until delta 3 blesses it.
- **Roster-holding readers of a `.gate` header** (point 2), enumerated by
  `grep -rln "strip_prefix(\"# install:\")\|manifest_class\|SHELL_COLON" native/src`:
  - the recipe's `install_disposition`. It reads `# install:` alone and ignores
    other lines.
  - `check-gate-substrate-parity` assertion D. It holds only `graph:` as
    manifest-class.
  - `check-comment-tier`, covered by delta 3.
- **The disarmed line (delta 4).** Its reader is the adopter, and a CI step
  reading doctor's exit status is unaffected. Its field has one reader.
- **Point 6.** The corpus of declaring members is enumerable today:
  `grep -rln "armed-by:" */checks` yields exactly `check-portability-floor` once
  delta 2 lands. Its satisfying value is `GATE_SDK_PORTABILITY_PATHS`, a gate-sdk
  static knob (gate-sdk/SPEC.md §Layout and configuration).

## Existing sections updated

Rosters were produced by `grep -rn "install:" native/src/gates/comment_tier.rs canon-kit/SPEC.md`
for the directive roster, and by reading gate-sdk/SPEC.md §The install
disposition, §check-install-disposition and §check-portability-floor, and
installer/SPEC.md §doctor.

- `gate-sdk/SPEC.md` §The install disposition gains the arming-declaration
  paragraph (delta 1).
- `gate-sdk/SPEC.md` §check-install-disposition: "Three assertions" becomes four,
  with assertion D (delta 1).
- `native/src/gates/install_disposition.rs`, and its `good/` and `bad/` cases
  under `gate-sdk/gate-tests/check-install-disposition/` (delta 1).
- `gate-sdk/checks/check-portability-floor.gate` and gate-sdk/SPEC.md
  §check-portability-floor's honest-limit paragraph (delta 2).
- `native/src/gates/comment_tier.rs` and canon-kit/SPEC.md §check-comment-tier
  (delta 3).
- `native/src/installer/doctor.rs` and installer/SPEC.md §doctor, whose
  paragraph on what reports without setting the exit status gains the disarmed
  line (delta 4).
- `.workflow/release-declarations.md` §Behavior changes: one bullet for doctor's
  disarmed line and one for assertion D, appended by the landing session
  (deltas 1 and 4).
<!-- update-target-exempt: generated mirrors of the kit SPECs, regenerated by their freshness gate's printed command, never hand-edited -->
- `docs/gate-sdk/SPEC.md`, `docs/canon-kit/SPEC.md` and `docs/installer/SPEC.md`.

## Retired spellings

- None — no delta retires a name; the change adds a directive and a report line.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template carries no grounds; a delta places them.
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved** — `portability-floor-adopter-on-ramp-unstated` moves to
      Done in the merge commit, a stage before the drain stage.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` runs the
      block above against the tracked tree.
- [ ] **Gaps filed** — the census of other members that degrade on an unset
      knob is filed to the gap inbox at spec; any cross-component gap build
      discovers is resolved that session.
