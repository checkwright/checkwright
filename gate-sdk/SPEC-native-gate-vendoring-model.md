# SPEC amendment: native-gate-vendoring-model

Queue entry: **`native-gate-vendoring-model`**. Companion to
**`gate-payload-disclosure-ruling`** (what a compiled gate discloses) and
**`native-gate-meta-layer-reach`** (the meta-layer's binary-side reach). This one
rules how a compiled gate **arrives** in a consumer tree.

The iteration's envelope is **unblock, not port**. This amendment makes
gate-sdk/SPEC.md §Porting a gate to the binary substrate's criterion 5 — *its
vendored form stays runnable* — a satisfiable condition. It lands no port, and
landing one is explicitly outside its envelope.

## The constraint set the ruling has to satisfy

Four facts, each measured against the tree rather than argued:

- **Kits vendor as text with zero build step.** `installer/lib/init.sh`:127-133
  writes every file of every selected kit through an unfiltered
  `find . -type f` loop, and a profile selects **kits**, never files
  (`installer/lib/common/profile.sh`:35-49; `installer/profiles.list` rows are a
  strict `<profile><TAB><kit>` pair with no third field).
- **The binary has no route into a consumer tree.** `gate_kit_roots`
  (gate-sdk/lib/gate.sh:111-127) admits a root as a kit only if it carries
  `checks/` or `smoke/`; `native/` carries neither, so
  `scripts/pack-installer.sh`:74-83 never packs it. The `.gate` descriptor sits
  under a kit root and therefore vendors. The descriptor arrives, the
  implementation does not.
- **The resulting failure is a battery kill, not a gate failure.**
  `gate_command` (gate-sdk/lib/gate.sh:96-103) exits 2 on an absent binary, and
  exit 2 is a dispatch-harness error: it takes down the calling battery, not just
  its own member. A freshly vendored consumer's pre-commit battery died on
  invocation, and `demo`, `consumer_smoke` and `agents_md_smoke` all reproduced
  it. That is the measured cost the reverted port paid for.
- **git is the sole runtime dependency, shelled out rather than embedded.** The
  crate declares no dependencies at all (`native/Cargo.toml`, empty
  `[dependencies]`), which is what makes every option below cheap or expensive.

## What changes

### 1. The payload gains a component class beside kits {design-bearing}

`scripts/pack-installer.sh` packs two things instead of one: **kit roots**
(unchanged, `gate_kit_roots_rel`-derived) and, new, **payload components** —
named repo-root directories that are not kits and whose vendoring is conditional.
The substrate crate is the class's first and only member.

`native/` **stays a non-kit.** It gains no `checks/` and no `smoke/`, because that
predicate is what makes a root directory a kit (CLAUDE.md §Housekeeping) and
`gate-sdk/bin/run-consumer-smoke.sh`:38-44 refuses any kit root shipping no
`smoke/install.sh`. Making the crate a kit to give it a ride would put a Rust
crate inside the vendoring set unconditionally, which is the opposite of the
ruling below.

The crate root is a knob on the kit convention (§Layout and configuration):
`GATE_SDK_NATIVE_CRATE`, default `native`. The existing `GATE_SDK_NATIVE_SRC`
default is **derived** from it (`$GATE_SDK_NATIVE_CRATE/src`) rather than
restated, so the crate's location has one owner.

The component is copied **from `git ls-files`**, not `cp -R`. The kit loop's
`cp -R` is safe because a kit root holds no build output; the crate root holds
`target/`, which is gitignored and would otherwise ride a locally-built tree into
a published payload. Deriving the packed set from tracking status makes that
unrepresentable rather than remembered.

### 2. Substrate need is derived from the selected payload {design-bearing}

`installer/lib/init.sh`, after `KITS` resolves (`:93`) and before any write,
scans the selected kits' payload dirs for `*.gate`. One or more means the
selection needs the substrate; zero means it does not.

**No new declaration anywhere.** Not a third `profiles.list` column, not a
per-kit capability file, and above all not a descriptor field: the descriptor
"carries no field that lacks a reader, reserving nothing against a future reader"
(§The `# graph:` manifest), and its existence is already the dispatch
declaration. A kit needs the substrate exactly when it carries a member that
dispatches to it, which is a fact the payload already states. Derivation-first —
a declared requirement is a second source that drifts from the descriptor set it
describes.

### 3. Substrate resolution moves to install time, and never blocks the install {design-bearing}

This is the ruling. `init` resolves the substrate **before it writes**, using the
toolchain floor's existing `cargo` member (`context-kit/lib/toolfloor.sh`'s
`PROBE_SET`, the same probe `installer/lib/doctor.sh` already renders, whose
verdict set — `ok`, `absent`, `below`, `wrong-impl`, `uncomparable` — is already
closed). Two outcomes, and neither is a refusal:

- **Toolchain present.** The crate component is written alongside the kits, built
  once as a post-write step, and the built binary joins `GENERATED`
  (`init.sh`:206-213) beside the pre-commit hook and the graph artifact — a
  generated file the install records, never a vendored file it `claim`s and
  hashes.
- **Toolchain absent or below floor.** The install proceeds and **the `.gate`
  descriptors are not written**. The affected members are not seeded into the
  consumer's `gates.list`; each is recorded there instead as a structured comment
  line (delta 4). Nothing in the consumer tree references a binary that is not
  there.

**Why not refuse the selection.** A governance tool whose install can fail on a
toolchain the adopter did not know it wanted is a time-to-first-value regression
paid by every adopter to serve the few who want a compiled gate. The install
keeps its zero-build-step guarantee unconditionally; the substrate is an upgrade a
consumer takes, not a toll the front door charges.

**Why not degrade to a shell fallback.** A descriptor declaring a shell
implementation to fall back to is the cheapest thing on the table and it is
wrong twice: it obliges every ported gate to keep two implementations and a
parity proof between them for as long as it lives, and it hands the consumer the
predicate as a side effect, settling **`gate-payload-disclosure-ruling`** by
accident rather than by ruling. That entry rules the payload; this one must not
pre-empt it with an install mechanism.

**Why not vendor a prebuilt binary.** `init.sh`'s copy loop is unfiltered and
binary-safe, so a prebuilt artifact placed in a kit dir would vendor with no code
change at all — which is precisely why the refusal has to be explicit. It
multiplies one payload by a platform matrix, and `scripts/pack-installer.sh`
builds exactly one tarball that both transports consume
(`.github/workflows/publish.yml` publishes the same artifact to npm and the
Release). It also converts this project into a binary distributor, taking on a
reproducible-build and provenance obligation that
**`gate-payload-disclosure-ruling`** names as the precondition for shipping any
artifact at all. The digest on the Release asset and npm's publish provenance
cover the *package*, not a per-platform gate binary's reproducibility.

**Ordering is load-bearing.** The substrate build runs **before**
`gen-pre-commit.sh --write`, because the hook's `run_gate` lines come from
`gate_command`, which exits 2 on an absent binary. `check-graph.sh --emit`
(`init.sh`:212) reads declaration paths as text and is unaffected by the
ordering either way.

**`gate_command`'s exit 2 is unchanged**, and this ruling is what makes it
unreachable in a correctly vendored tree. It stays the fail-closed backstop for a
tree that took the substrate and then lost or stale-ed its binary — which is the
one place that error belongs.

### 4. An omitted member is declared and counted, never silent {design-bearing}

A gate that is not running must not be indistinguishable from a gate that is
running clean — the vacuous-green class §Meta-gate conservation for the binary
substrate exists to refuse.

The record rides the registry, not a new file: `installer/lib/common/recipe.sh`'s
roster seeding writes an omitted member into the consumer's `gates.list` as
`# omitted: <name> substrate-absent`, a comment line `gates_list_members`
(gate-sdk/lib/gate.sh:66-68) already strips from the live set. Its named readers:

- `gate-sdk/bin/run-gates.sh` counts those lines and prints the count and the
  one-command remedy as a **separate line** beside its summary — separate because
  `gate-sdk/bin/run-consumer-smoke.sh`:60-68 greps the existing
  `All N gates passed` line verbatim, and that assertion stays intact.
- `installer/lib/doctor.sh` reports the same count against the toolchain verdict
  that caused it, which is where an adopter looks for the remedy.

The record sits in the consumer's tracked `gates.list`, so it is reviewable in
their history rather than buried in install-time stdout. Re-running `init` on a
machine that has since gained the toolchain converts the comment back into a live
member with no hand edit.

### 5. The crate's empty dependency set becomes an enforced precondition {design-bearing}

An install-time build is acceptable only while it is network-free and bounded. A
single dependency turns a post-write step into a registry fetch inside somebody
else's repository, on a machine whose network policy this project cannot see.

`check-gate-substrate-parity` gains **assertion E**: the crate manifest declares
no `[dependencies]` and no `[build-dependencies]`, and the crate carries no
`build.rs`. Its `# graph:` `couples=` gains the crate manifest path so an edit
re-fires it.

A crate unit test was weighed and refused for this one: the claim is consumed by
the **shell** install path and must hold in a tree with no toolchain, and a
`cargo test` cannot run where cargo is the thing in question. The parity gate is
already the auditor that stays shell so it never depends on the substrate it
audits (§check-gate-substrate-parity).

### 6. Upgrade rebuilds unconditionally {design-bearing}

On upgrade into a tree that took the substrate, the crate source is rewritten and
the binary is **rebuilt unconditionally** before hook regeneration. Consumer-side
crate source changes only ever through the installer, so an unconditional rebuild
closes the consumer half of binary freshness by construction rather than by
comparison.

This does **not** absorb **`native-binary-freshness-ungated`**, which is the
in-repo development half: a contributor editing Rust source and skipping the
rebuild. That entry stays deferred and stays true.

`gate-sdk/bin/upgrade-smoke.sh`'s phase-A determinism diff compares the vendored
tree against itself; the built binary and the crate's `target/` are generated
output and join the exclusion the generated hook and graph artifact already
carry.

### 7. The extensibility model is ruled, with its rejections recorded {design-bearing}

The **script escape hatch stays first-class**: a consumer authors gates as shell
in their own resolve dir, and consumer-first resolution with `.sh` beating
`.gate` within a dir (gate-sdk/lib/gate.sh:70-85) means a consumer can shadow a
ported kit gate with their own shell script and win. That is recorded today only
as a deliverable line; it becomes the ruled model, with the two alternatives
refused rather than dropped:

- **A declarative check DSL.** Refused: a DSL is a language carrying none of a
  language's tooling — no debugger, no test framework, no editor support — and
  this repo's own battery is the evidence against its expressible set, since the
  gates carrying real judgment are exactly the ones a rule language would not
  hold. The declarative half a DSL is wanted for already exists as the `# graph:`
  manifest, which is data every reader greps as text.
- **Native plugins.** Refused: a dynamically-loaded plugin ABI is a stability
  contract this project would own forever and a supply-chain surface it would own
  in every consumer tree. The multi-call binary plus the shell hatch already
  spans both ends of the substrate range, and **`gate-authoring-sdk-surface`**
  owns the neutral-surface question that a plugin ABI would foreclose.

### 8. Criterion 5 and the second-port prerequisite are restated as ruled {mechanical}

§Porting a gate to the binary substrate criterion 5 states the ruled condition —
a vendored form stays runnable because the substrate is resolved at install time
and a member whose substrate is absent is omitted and declared — in place of the
"unsatisfiable until `native-gate-vendoring-model` rules" text. The
second-port-prerequisite paragraph (§What is retained, and what a second port
must do first) records the vendoring half as satisfied. §What the dispatch seam
does not settle's vendoring paragraph is rewritten to state the ruled model.

### 9. The install path's requirement claims are corrected {design-bearing}

`docs/install.md` §Requirements carries a `cargo` bullet claiming the floor is
contributor-and-CI only and that "a gate on that substrate shells out to git at
runtime and embeds nothing". The first half stops being true for a consumer whose
selection carries a `.gate` member; the second half stays true and is worth
keeping. The bullet is rewritten to state the conditional install-time role, the
zero-build-step guarantee that survives it, and the omit-and-declare outcome.

`installer/README.md` §Requirements gains the same conditional in its
delivery-path framing. `CLAUDE.md` §Housekeeping's `native/` bullet, which states
that a second port is blocked on this entry, records the ruled install model
instead.

## Producers and consumers

**Payload component (new payload class).**
Producer: `scripts/pack-installer.sh`'s component loop, `git ls-files`-derived,
run at every release pack. Consumer: `installer/lib/init.sh`'s write step.
Enabling config: `GATE_SDK_NATIVE_CRATE` (default `native`), set nowhere and
therefore live at its default in this repo and in every consumer — the crate is
at `native/` in the payload by construction, since the component's packed name is
its basename exactly as the kit loop's is.

**Substrate-need signal (derived, not stored).**
Producer: `init.sh`'s `*.gate` scan over the selected kits' payload dirs, which
runs on every install and every upgrade. Consumer: the same function's branch on
it. It is a local, never persisted — nothing else may read it, because a stored
copy would be a second source for a fact the payload already carries.

**Toolchain verdict.**
Producer: `context-kit/lib/toolfloor.sh`'s `tool_floor_check` on the `cargo`
member of `PROBE_SET`, already invoked by `installer/lib/doctor.sh`. Consumer:
`init.sh`'s two-outcome branch (delta 3), and `doctor.sh`'s report (delta 4).
The verdict set is closed and already handles the fail-closed `uncomparable` arm,
so no new state is introduced — an existing producer gains a second consumer.

**Built binary.**
Producer: `init.sh`'s post-write build step. Consumers: `gate_command`
(gate-sdk/lib/gate.sh:96-105) at every battery run, and
`check-gate-substrate-parity` assertion B's `--list` read. Its enabling config is
`GATE_SDK_NATIVE_BIN`, already live at its default. It is recorded in `GENERATED`
and therefore in the install's record of generated files, and it is **not**
`claim`ed — a generated artifact is not a vendored file and must not be hashed as
one.

**Omitted-member record.**
Producer: `installer/lib/common/recipe.sh`'s roster seeding, on an install whose
substrate resolution failed. Consumers, both named and both new readers of an
existing file: `gate-sdk/bin/run-gates.sh`'s summary line, and
`installer/lib/doctor.sh`'s report. The record has exactly one field beyond the
member name — the reason token `substrate-absent` — and its reader is the remedy
text those two consumers print. No further field is added, because none has a
reader.

**Assertion E's input.**
Producer: the crate manifest, a tracked file. Consumer:
`check-gate-substrate-parity`, whose `couples=` gains the path so the assertion
re-fires on an edit. Its verdict has no downstream state.

## Existing sections updated

Owned by this amendment, each named with the delta that claims it:

- gate-sdk/SPEC.md §Porting a gate to the binary substrate — criterion 5, and
  §What is retained, and what a second port must do first (delta 8).
- gate-sdk/SPEC.md §What the dispatch seam does not settle — the vendoring
  paragraph (delta 8).
- gate-sdk/SPEC.md §Layout and configuration — `GATE_SDK_NATIVE_CRATE` joins the
  knob roster and `GATE_SDK_NATIVE_SRC`'s default becomes derived (delta 1).
- gate-sdk/SPEC.md §check-gate-substrate-parity — assertion E (delta 5).
- gate-sdk/SPEC.md §run-gates — the summary's omitted-member line (delta 4).
- gate-sdk/SPEC.md §upgrade-smoke — the generated-output exclusion (delta 6).
- gate-sdk/SPEC.md §Consumer smoke — the vendored-tree runnability statement,
  which is criterion 5's other reader (delta 3).
- context-kit/SPEC.md §bin/env-probe — the `cargo` member's role gains the
  conditional install-time reading beside its build-and-CI one (delta 9).
- `installer/README.md` and `docs/install.md` §Requirements (delta 9);
  canon-kit's `check-install-claim` holds the primary-install-path claim across
  governed install sections and is the reader that must stay green.
- `CLAUDE.md` §Housekeeping — the `native/` bullet (delta 9).

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
- [ ] **No port lands.** The iteration's envelope is unblock; a second port is
      out of scope and no `.gate` member is added to any kit.
