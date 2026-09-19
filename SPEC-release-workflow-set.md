# SPEC amendment: release-workflow-set

`check-gate-substrate-parity` assertion F audits one workflow, and the tree has two
that build and hash a release-shaped artifact. The knob naming the audited
workflow, `GATE_SDK_NATIVE_PUBLISH_WORKFLOW`, is a scalar
(`native/src/knobs/gate_sdk.rs:122`, read at
`native/src/gates/gate_substrate_parity.rs:774`). Its default is
`.github/workflows/publish.yml`, and `.github/workflows/gates.yml` is never
opened. That second workflow carries the `native-artifacts` job, which builds and
hashes each roster target through `scripts/ci-build-artifact.sh` (`gates.yml:1155`)
and hands the upload to the install-smoke legs. It also carries a roster-derived
matrix (`gates.yml:1081-1082`), and a local rebuild hashed inside
`install-smoke-powershell` (`gates.yml:592`).

The filing's line citations for `gates.yml` (:494, :737, :819-822) have drifted and
name no digest today. The sites above were read at this amendment's authoring.

**Pointing F at `gates.yml` today would not be green, and the cause is F's own
detector.** Run against a copy of `gates.yml`, the gate reds four jobs:
`install-smoke-windows`, `install-smoke-macos`, `install-smoke-macos-intel` and
`install-smoke-linux-arm64`. Each finding says the job computes a digest as a
consumer. None does. Each job lists `sha256sum` as a word in a toolchain-presence
loop (`gates.yml:275`, `:1303`, `:1610` and `:1797`, `for t in … sha256sum …`),
and
`computes_digest` (`gate_substrate_parity.rs:340-354`) is a substring test on the
line. So the widening needs the detector to read an invocation rather than a
mention, or it lands red.

This amendment does two things:

- The knob becomes a set of workflows.
- F's detector counts `sha256sum` only in command position.

It is a gate-sdk amendment: the knob, the gate, its fixtures and its SPEC section
are gate-sdk's, and this repo's knob file is consumer config.

**The sibling debt unit shares assertion F and is not this amendment's.**
`substrate-parity-digest-assertion-stops-at-the-workflow-text` makes F follow a
`run:` line's called script, and counts a `sha256sum`-else-`shasum` branch pair as
one producer. That is enforcement of the existing §Consumer payload rule and needs
no amendment. The two units edit `computes_digest` and its caller, so they land in
one batch or this one first. Delta 2's command-position reader is what the script
follower should scan with.

The tree does not already do this. The knob's row is `Row::scalar`, the gate reads
it through `walk::knob_scalar`, and `computes_digest` has no command-position
test.

## What changes

**Batching.** Deltas 1 to 3 land in one commit. This repo's knob file (delta 3)
names two workflows, which the scalar reader of delta 1's prior shape would take
as one path that exists nowhere. And `gates.yml` reds F until delta 2 lands.

### (1) The knob names a set of workflows {mechanical}

**Not yet applied.** `GATE_SDK_NATIVE_PUBLISH_WORKFLOW` becomes an indexed knob,
spelled space-separated in a knob file like every other indexed gate-sdk knob
(`GATE_SDK_KIT_DIRS` is the precedent, gate-sdk/SPEC.md §Layout and
configuration). Its default stays `.github/workflows/publish.yml`, now a
one-element list.

Every assertion F check runs per member, and every finding names the member it
read. Each absent member is reported rather than red, on the existing ground: a
consumer whose release rides elsewhere names the workflow it does ride. The clean
line reports the members read and the members absent, so a set whose every member
is absent stays visible.

**The name is kept, and the cost is stated.** A plural rename would be a retired
spelling, and an adopter knob file still carrying the old name would be refused
at its first read. That turns a harmless upgrade into a red battery. A scalar
value already in a knob file reads as a one-element list, so keeping the name
makes the widening invisible to every adopter who set it. What the name now
undersells is that `gates.yml` publishes nothing. It builds the artifacts the
install-smoke legs install, which is the publish path's shape one step earlier,
and that shape is what the assertion's subject is.

### (2) A digest is computed by an invocation, not a mention {design-bearing}

**Not yet applied.** A job *computes* a digest when a `run:` step's shell body
invokes `sha256sum` in **command position** without `-c`. The body is the inline
value after `run:` or the indented block under `run: |` or `run: >`. The command
position is the crate's own shell-word reader's
(`bashscan::command_positions`, which §port-blockers already reads shell text
with). A word that names the tool as an argument,
a loop word, a string or a comment is not an invocation.

The count, the per-job rule and both findings are unchanged. Only the detector
narrows, from any line mentioning the tool to a step invoking it.

The loop words are not command words, and that is read off the reader rather than
assumed: `for` is one of `bashscan`'s operand keywords (`native/src/bashscan.rs:32`),
which clear command position (`:162-164`), and nothing resumes it before the loop's
`do`.

**Point 5, the narrowed corpus's readers.** The detector narrows what counts, so
each reader's red condition was checked:

- Both findings red on a count above a bound. Fewer counted invocations can only
  remove findings, and neither reads *found none*.
- The clean line reports the job count, not the digest count, so no count asserts
  an exact value.
- The fixture pair's three F plants all invoke `sha256sum` as a command
  (`gate-sdk/gate-tests/check-gate-substrate-parity/bad/publish.yml`), so each
  stays red.

The fixture pair gains the case the change exists for. `good/` carries a job that
lists `sha256sum` in a presence loop beside a download and no upload, and stays
clean. The bad plants are unchanged.

### (3) This repo audits both workflows {mechanical}

**Not yet applied.** `scripts/gate-sdk-config.knobs` sets
`GATE_SDK_NATIVE_PUBLISH_WORKFLOW = .github/workflows/publish.yml .github/workflows/gates.yml`.
That is consumer config: which of a tree's workflows build its release shape is
the tree's own fact, so the kit default does not name `gates.yml`.

**Point 6, each member's satisfying value, enumerated from the two workflows.**
With delta 2 in place:

- `publish.yml` stays clean, as the default run is today.
- In `gates.yml`, the matrix at `:1081-1082` is an expression, so the matrix check
  passes. `native-artifacts` computes no digest in its own text, because the hash
  is inside the called script (the debt unit's reach).
  `install-smoke-powershell` computes one digest (`:592`) and downloads no run
  artifact, so it passes. The four presence-loop jobs compute none.

**Inferred, cannot run before build:** that `gates.yml` reads clean under deltas 1 to 3 — the command-position detector is delta 2's and does not exist yet; build runs the gate with the knob set before the commit.

**Honest limit.** The matrix check asks only that a matrix value is an
expression. `gates.yml`'s expression is derived from docs/install.md's platform
block (`native-artifacts-roster`, `gates.yml:999-1058`), not from
`GATE_SDK_NATIVE_TARGETS_FILE`. The relation between that page and the target
roster is §Consumer payload's to hold, so "roster-derived" reads true of
`gates.yml` only through one hop, and the check does not follow it.

## Producers and consumers

- **The knob's new shape (delta 1).** Producer: gate-sdk's static table row, and
  a consumer's `gate-sdk-config.knobs` line. Consumers:
  - `check-gate-substrate-parity` assertion F, the only reader
    (`git grep -n GATE_SDK_NATIVE_PUBLISH_WORKFLOW` lists the table, the gate,
    the gate's declared-knob roster at `native/src/gates/mod.rs:1673`, the two
    fixture knob files and the SPEC declarations).
  - The knob-roster emitter and the knob-file validator. Both read the row's
    shape off the table, so an indexed row needs no edit there.
  - The fixture knob files
    (`gate-sdk/gate-tests/check-gate-substrate-parity/{good,bad}/scripts/gate-sdk-config.knobs:5`)
    each set one member, which parses unchanged as a one-element list.
- **The command-position detector (delta 2).** Its one consumer is assertion F's
  per-job count. It will also be the called-script follower's, from the sibling
  debt unit.
- **The clean line's workflow clause.** Its reader is a human. The evidence
  parser reads the gate's status, not this clause.

## Existing sections updated

Rosters produced by `git grep -n GATE_SDK_NATIVE_PUBLISH_WORKFLOW` and by reading
gate-sdk/SPEC.md §check-gate-substrate-parity and §Consumer payload.

- `gate-sdk/SPEC.md` §Layout and configuration: the knob's declaration (`:303`)
  becomes a space-separated list defaulting to the publish workflow (delta 1).
- `gate-sdk/SPEC.md` §check-gate-substrate-parity assertion F: "a `matrix:`
  declaration of `GATE_SDK_NATIVE_PUBLISH_WORKFLOW`" reads "of each workflow
  `GATE_SDK_NATIVE_PUBLISH_WORKFLOW` names". "Invokes `sha256sum`" gains *in
  command position*, and the two absences paragraph reports each absent member
  (deltas 1 and 2).
- `gate-sdk/SPEC.md` §Consumer payload, the one-producer paragraph at `:8593-8602`:
  "the publish workflow's own text" reads "the text of each workflow the knob
  names" (delta 1). Its called-script limit stays, and is the debt unit's to
  narrow.
- `native/src/knobs/gate_sdk.rs:122` and `native/src/gates/gate_substrate_parity.rs`
  (deltas 1 and 2), and the fixture pair's `good/` gains the presence-loop job
  (delta 2).
- `scripts/gate-sdk-config.knobs` (delta 3).
- `.workflow/release-declarations.md` §Behavior changes: one bullet that the knob
  takes a list, appended by the landing session (delta 1).
<!-- update-target-exempt: generated mirror of the kit SPEC, regenerated by its freshness gate's printed command, never hand-edited -->
- `docs/gate-sdk/SPEC.md`.

## Retired spellings

- None — the knob keeps its name, and a scalar value parses as a one-element list.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved** — `substrate-parity-audits-one-producer-of-two` moves to Done
      in the merge commit, a stage before the drain stage.
- [ ] **Green on both workflows** — the battery's `check-gate-substrate-parity`
      reads both members and is clean.
- [ ] **Gaps filed** — any cross-component gap build discovers is resolved that
      session.
