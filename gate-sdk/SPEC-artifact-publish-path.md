# SPEC amendment: native-artifact-publish-path

Queue entry: **`native-artifact-publish-path`**. Companion to
**`native-artifact-install-path`** (which places and verifies what this one
produces). This amendment rules how a compiled gate is **built and published**.

The basename drops the `native-` prefix the slug carries, and that is deliberate
rather than sloppy: the promoted lead line carries the entry's ruled
`[roadmap: now/reliability]` tag, and the full-slug spelling
`SPEC-native-artifact-publish-path.md` measures the line at 106 columns against
`QUEUE_KIT_WRAP_BUDGET`'s floor of 100 (queue-kit/SPEC.md §check-queue-wrap —
tags carry spaces, so the one-unbreakable-token exemption never reaches them).
queue-kit/SPEC.md §bin/roadmap.sh already names this trade: *"a sufficiently long
slug plus spec pointer leaves no room even for that"*, and widening the budget to
buy a public page's tag *"would trade a parse guarantee for a presentation one"*.
The ref resolves either way — `check-amendment-queue` searches a bare basename
tree-wide (canon-kit/SPEC.md §check-amendment-queue) — so the shorter spelling
costs nothing and keeps the ruled curation.

## What this amendment inherits and may not re-litigate

TRAJECTORY.md's closed rulings, in force:

- **The payload ships a prebuilt gate binary, selected by platform.** The
  consumer builds nothing, installs no toolchain, and receives no gate
  implementation source. Building from vendored crate source at install time is
  **void**.
- **Opacity carries an obligation.** *Ship the achievable floor and claim
  nothing beyond it.* The floor is a published per-target digest verified before
  the artifact is written. The bound on prose is exact: a governed surface may
  say **verified against a published digest** and may **not** say
  **reproducible**. **`tarball-build-attestation`** holds the larger ground.
- **The objectives are a direction, not a claim about the tree.** *"No
  user-facing surface may state the dependency floor they aim at as though it
  were reached."* This clause is what §The support-commitment bound below turns
  into a roster rule.

gate-sdk/SPEC.md §Porting a gate to the binary substrate criterion 5 already
states the model in one sentence. This amendment fixes the **wire** that
criterion deliberately left open.

## The support-commitment bound (a constraint, not a note)

**`platform-support-ci-matrix`** stays deferred by operator ruling, so the
target roster this amendment declares is **the only surface asserting platform
support this iteration**. It is bound by two rules, both enforceable:

1. **The roster may not exceed what `docs/install.md` §Requirements already
   states.** That page asserts Unix-first and GNU-first; Linux out of the box;
   **Windows through WSL, not natively** (*"no native-Windows shell path
   exists"*); and macOS as an **adopter action** rather than something the stock
   system delivers. A roster naming a platform the page does not support is a
   support commitment made through the back door.
2. **A target joins the roster only when a green run has produced and exercised
   its artifact.** Not reasoning about a platform — a run.

Applying both, **the roster ships exactly one target: `x86_64-unknown-linux-gnu`.**
The grounds are measured rather than cautious:

- It is the only platform `.github/workflows/publish.yml` has a runner for
  today. Every job is `ubuntu-latest`; there is no `matrix:` key anywhere in
  `.github/workflows/`.
- It is the only platform this repo has any green evidence for at all — every
  gate run, every fixture suite, every consumer smoke, every CI leg is Linux.
- WSL is Linux and resolves to the same target, so objective 2's Windows half
  is served today exactly as far as the page already claims and no further;
  native Windows waits on **`powershell-installer-surface`**.
- A macOS target would have to be built on a runner no workflow has and shipped
  to a platform whose *battery* support **`platform-support-ci-matrix`** records
  as resting on reasoning rather than on a green run. Cross-compiling one from
  `ubuntu-latest` is worse, not better: it publishes an artifact no run has ever
  executed.

**This is not a narrowing of the trajectory.** Objective 2 is the direction; the
roster is the tree. A one-target roster makes the omit-and-declare path
(**`native-artifact-install-path`**) the *normal* path for a macOS adopter on day
one, which is the honest reading of the page as written — and it means that path
ships exercised rather than as dead code the vacuous-green class exists to
refuse.

**The widening trigger is mechanical and named:** a target joins the roster when
**`platform-support-ci-matrix`** lands a leg for its platform. Because the build
matrix is roster-derived (delta 2), widening is one roster line and one runner
mapping — never a workflow rewrite. That cheapness is the whole point of ruling
the roster before the first port.

## What changes

### 1. A tracked target roster, one owner and two readers {design-bearing}

`native/targets.list` — one Rust target triple per line, `#`-comments and blank
lines stripped, the same line grammar `scripts/gates.list` already uses and
`gates_list_members` (gate-sdk/lib/gate.sh:66-68) already implements for it.

Its location is a knob on the kit convention (§Layout and configuration):
`GATE_SDK_NATIVE_TARGETS_FILE`, default **derived** from the existing
`GATE_SDK_NATIVE_CRATE` as `$GATE_SDK_NATIVE_CRATE/targets.list`, exactly as
`GATE_SDK_NATIVE_SRC`'s default already is — the crate's location keeps one
owner. `gate-sdk/lib/gate.sh` gains `gate_native_targets`, the roster's single
reader function.

**The roster is consumer config, not a kit literal.** The knob and the grammar
are gate-sdk mechanism; the *values* — which platforms this project commits to —
live in a consumer-owned file outside every kit root. This is the
`check-graph` / `scripts/graph-vocab.sh` pattern (CLAUDE.md §The provenance
seam), and here it is a seam before it is a convenience: a kit literal spelling
one project's platform-support commitment would ship that commitment as
everyone's.

**Why a roster and not a list in the workflow.** A hand-maintained platform list
inside `publish.yml` is the maintained-roster anti-pattern derivation-first
refuses, and it puts the build's idea of the supported set and the installer's
idea in two files that drift silently. One owner, two readers, and **a target in
the roster that no leg built fails the release** — which is the correct place for
that failure.

**Why the installer needs it at all**, since it could infer support from a
directory's presence: without the roster the installer cannot tell *"this
platform was never committed to"* (omit and declare — a supported outcome) from
*"this platform was committed to and the artifact is missing"* (a broken payload
— a refusal). Collapsing those two silently degrades a supported platform into a
green battery over a smaller roster, which is the exact class
**`native-artifact-install-path`** is forbidden to produce.

### 2. `publish.yml` gains a roster-derived build matrix {design-bearing}

A new `build` job, ahead of `pack`:

- A **prepare** step reads the roster through `gate_native_targets` and emits it
  as a JSON array on a step output. GitHub Actions matrices take JSON, so this
  is the whole of the conversion — the YAML carries **no platform literal**.
- The matrix leg maps a target to its runner. The mapping is the one place a
  platform name may appear in YAML, and it is a *runner selection*, not a
  support declaration: a target absent from the mapping fails the leg rather
  than silently building somewhere wrong.
- Each leg runs `cargo build --release --target <target>` in
  `GATE_SDK_NATIVE_CRATE`, then emits the binary and its digest sidecar
  (delta 3) and uploads both as a per-target run artifact.

`pack` gains `needs: build` and downloads every per-target artifact before
invoking `scripts/pack-installer.sh`. `release` and `npm` still `needs: pack`
and still consume the one assembled tarball — the workflow's stated shape
(*"Add a channel as a job; never weld a second publish into an existing one"*)
is preserved: this adds a **stage**, not a channel.

**The crate's zero-dependency set is what makes the matrix cheap.** `cargo build`
with an empty `[dependencies]` is network-free, which is why
`check-gate-substrate-parity` assertion E already holds it that way.

### 3. The digest is emitted once and never recomputed {design-bearing}

Each build leg writes `<binary>.sha256` beside its binary, in `sha256sum -c`
format with a bare filename inside it — the same shape the `release` job already
uses for the tarball (`.github/workflows/publish.yml`:138) and the same shape
`docs/install.md` already documents a reader verifying.

**One producer, two publications, and no recomputation anywhere.** The bytes the
build leg wrote are the bytes `pack-installer.sh` places in the payload *and* the
bytes `release` attaches to the Release. A second `sha256sum` invocation on a
later job is what lets a published digest and an installed digest diverge while
both look computed, so the workflow is ruled to move the file rather than
re-derive its contents.

**Why per-artifact sidecars rather than a combined `SHA256SUMS` or a JSON
manifest.** The forward reader is **`tarball-build-attestation`**: an in-toto /
SLSA-shaped attestation's subject list is exactly `{name, digest}` pairs, so a
per-artifact sidecar maps onto a subject one-to-one and that entry can later add
an attestation *beside* these files with no migration and no digest value
changing. A JSON manifest would mint a schema, a version key and a reader
migration for the same information; a combined `SHA256SUMS` couples every
target's digest into one file an attestation must then parse apart. The queue
entry names *"what `tarball-build-attestation` can later strengthen without a
migration"* as this delta's open call, and non-migration is what decides it.

**What the digest proves, stated at its honest bound.** It is a transfer- and
substitution-integrity claim: the artifact in the payload is byte-identical to
the one the release built. It is **not** evidence the build host was
uncompromised, and it is not a reproducible build —
`docs/install.md`:198-202 already states that bound for the tarball checksum and
the artifact's story is the same one, one layer in. No governed surface gains a
stronger word.

### 4. `pack-installer.sh` gains an artifact class beside its kit roots {design-bearing}

A second copy loop beside the kit-root loop (`scripts/pack-installer.sh`:74-83),
placing for each roster target:

```
payload/artifact/<target>/<binary>
payload/artifact/<target>/<binary>.sha256
payload/artifact/targets.list
```

`<binary>` is `${GATE_SDK_NATIVE_BIN##*/}` — derived from the existing knob, so
the payload's spelling of the binary's name cannot drift from the one
`gate_command` dispatches to. `targets.list` is the tracked roster copied
verbatim; it is the roster's one publication, not a second spelling.

**Fail-closed, mirroring the `packed -gt 0` guard the kit loop already has:** a
roster target with no artifact directory, a missing sidecar, or a sidecar whose
digest does not match the binary beside it aborts the pack. The dirty-worktree
refusal (`:41`) and the tag-derived version stamp (`:54-62`) are untouched.

**The artifacts are never produced from a working tree.** The script takes them
from the run artifacts the build legs uploaded and refuses to build anything
itself, so a locally-built binary can never substitute for a released one. This
is the same reasoning the vendoring ruling applied to the crate source, arriving
one layer out.

**The one-tarball invariant survives.** Every declared target rides inside the
single payload; the difference against a per-target payload is download size
alone, since the installer writes only the matching binary. The revisit trigger
is a measurement and is already recorded on the queue entry: the roster growing
past what **`platform-support-ci-matrix`** commits to, or one target's binary
ceasing to be small.

### 5. `check-gate-substrate-parity` gains assertion F {design-bearing}

Enforcement-first: the roster's whole value is *no second spelling*, and prose
cannot hold that. The gate that already audits the substrate from the shell side
— staying shell so it never depends on the substrate it audits — gains a fourth
concern rather than this amendment minting a new governed name:

- the roster resolves, is non-empty, and every live line is a well-formed target
  triple;
- `.github/workflows/publish.yml` carries **no platform literal in its matrix
  declaration** — the matrix is roster-derived or the gate reds;
- no digest is computed in more than one workflow step (delta 3's
  no-recomputation rule, held mechanically rather than by review).

Its `# graph:` `couples=` gains the roster path and `publish.yml` so an edit to
either re-fires it.

### 6. `RELEASING.md` step 5's factual narration is corrected {mechanical}

Step 5 currently states the publish path as *"assembles the package once"* and
*"Two sibling jobs then consume that one artifact"*. Both stay true and both
become incomplete: a build stage now runs ahead of the assembly, and the Release
gains the per-target artifacts and their sidecars. The paragraph is rewritten to
narrate three stages rather than two, keeping its imperative voice and its
existing *watch both jobs to green* instruction. §The publish spec is untouched —
no new `npm publish`-shaped line is introduced.

### 7. gate-sdk/SPEC.md records the roster as the support surface {design-bearing}

§Layout and configuration gains `GATE_SDK_NATIVE_TARGETS_FILE` beside the three
existing native path knobs, with its default stated as derived from
`GATE_SDK_NATIVE_CRATE`. §Consumer payload records the roster as the surface
that asserts platform support and the digest sidecar as the published-digest
floor, with the bound of delta 3 stated in place. §Porting a gate to the binary
substrate criterion 5's forward pointer stops saying a second port *waits on*
these artifacts and says what produces them.

## Producers and consumers

**Target roster (new tracked state).**
Producer: hand-authored `native/targets.list`, a tracked file; it is a
declaration, so its producer is an editor and its enabling config is the knob's
default, live in this repo and in every consumer by construction. Consumers,
both named and both real: `.github/workflows/publish.yml`'s prepare step
(delta 2, at tag push) and `installer/lib/init.sh`'s selection step
(**`native-artifact-install-path`**, at install and at every re-run), reading the
payload copy delta 4 places. Third reader: `check-gate-substrate-parity`
assertion F, at every commit touching it. Every live line has exactly one field —
the target triple — and all three readers read that field.

**Per-target binary (new artifact).**
Producer: the `build` matrix leg's `cargo build --release --target <target>`, on
tag push, gated by nothing an adopter can influence. Consumers:
`scripts/pack-installer.sh`'s artifact loop (packs it), the `release` job
(attaches it), and `gate_command` (gate-sdk/lib/gate.sh:96-105) once the
installer has written it. Its enabling config is `GATE_SDK_NATIVE_BIN`, already
live at its default, whose basename delta 4 derives the payload spelling from.

**Per-target digest sidecar (the one new external contract).**
Producer: the same build leg, one `sha256sum` invocation per target, and
**exactly one** — delta 5 holds that mechanically. Consumers, both named at a
named transition: `installer/lib/init.sh`'s **pre-write** verification
(**`native-artifact-install-path`**), and `installer/lib/doctor.sh`'s
re-verification of the binary in place through the lock's `artifact` record. The
Release copy has a third, human reader: a reviewer cross-checking the payload's
digest against the published one by hand. Its content is one
`<hex>  <bare-filename>` line; both fields are read — the hex is the comparison,
the filename is what makes `sha256sum -c` work wherever a reader puts the two
files.

**Payload roster copy.**
Producer: `pack-installer.sh`'s artifact loop, at every pack. Consumer:
`init.sh`'s selection step. It is a *publication* of the tracked roster, not a
second source — copied verbatim, never regenerated or filtered, so assertion F's
one-owner claim holds through the payload.

**Build-matrix JSON.**
Producer: the prepare step's roster read. Consumer: the `build` job's `matrix:`
expression, in the same workflow run. It is a step output — never persisted,
never a file — because a stored copy is a second source for a fact the roster
already carries.

## Existing sections updated

Owned by this amendment, each named with the delta that claims it:

- gate-sdk/SPEC.md §Layout and configuration — `GATE_SDK_NATIVE_TARGETS_FILE`
  joins the knob roster (deltas 1, 7).
- gate-sdk/SPEC.md §Consumer payload — the roster as the support-commitment
  surface, the digest sidecar as the published-digest floor, and the honest
  bound on what it proves (deltas 3, 7).
- gate-sdk/SPEC.md §Porting a gate to the binary substrate — criterion 5's
  forward pointer (delta 7).
- gate-sdk/SPEC.md §check-gate-substrate-parity — assertion F (delta 5).
- gate-sdk/SPEC.md §lib/gate.sh — `gate_native_targets` (delta 1).
- `RELEASING.md` step 5 (delta 6).
- `docs/install.md` §Requirements — the `cargo` bullet's *"a compiled gate is
  ruled to arrive prebuilt for your platform"* becomes a statement about
  artifacts that now exist; the platform claims themselves are **unchanged by
  construction**, which is §The support-commitment bound's whole point
  (deltas 1, 7). `check-install-claim` is the reader that must stay green.
- `CLAUDE.md` §Housekeeping's `native/` bullet — *"No gate is ported today"*
  stays true; *"how a compiled gate arrives is now ruled"* gains that the
  artifacts now exist (delta 7).

## Sequencing this amendment does not own

**Flagged forward to build, not settled here:** `native/` sits in neither
`DELEGATION_KIT_META_PATHS` nor the kit roots, so `check-gate-tamper` refuses a
commit touching `native/` beside any gate file. Delta 1 writes `native/` and
delta 5 writes a gate — a two-commit sequence, not a design change.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The roster did not widen.** It ships one target. A second target is a
      support commitment **`platform-support-ci-matrix`** owns, and adding one
      here would be the silent widening §The support-commitment bound refuses.
- [ ] **No port lands.** No kit gains a `.gate` member; this unit produces
      artifacts, it does not consume them.
- [ ] **No surface says *reproducible*.** The published word is *verified
      against a published digest*, and TRAJECTORY.md's bound is exact.
