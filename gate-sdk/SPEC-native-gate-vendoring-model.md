# SPEC amendment: native-gate-vendoring-model

Queue entry: **`native-gate-vendoring-model`**. Companion to
**`gate-payload-disclosure-ruling`** (what a compiled gate discloses) and
**`native-gate-meta-layer-reach`** (the meta-layer's binary-side reach). This one
rules how a compiled gate **arrives** in a consumer tree.

The iteration's envelope is **unblock, not port**. This amendment makes
gate-sdk/SPEC.md §Porting a gate to the binary substrate's criterion 5 — *its
vendored form stays runnable* — a satisfiable condition. It lands no port, and
landing one is explicitly outside its envelope.

## The objectives this ruling serves

**Ruled 2026-08-03 by the operator as a project-trajectory pivot.** This section
is the durable record of it, and it is the ground every ruling below stands on;
a later session weighing an alternative weighs it against these, not against the
constraint set the substrate seam was built under.

The pivot: **port the battery to native binaries, and reduce what a consumer
must have to git alone.** Six objectives, each of which decides something below:

1. **The dependency floor collapses to git.** What a consumer needs today —
   bash, awk, the GNU userland — stops being acceptable. git stays, shelled out
   rather than embedded.
2. **All major operating systems, Windows included.** A bash-only install path
   fails native Windows, so it fails the objective.
3. **Opacity is a goal, not a side effect.** Withholding a gate's implementation
   source is wanted: it favours *execution* of a gate over *analysis* of it by
   the coding agents the gate exists to hold. This reverses the direction the
   substrate seam was argued under, where opacity was explicitly not claimed.
4. **Footprint is a first-class cost.** An adopter installs, tries, and
   uninstalls without growing their managed code base or their dependency set.
   The trial lifecycle is part of the product, not an afterthought.
5. **Non-technical adopters are a design constraint.** A prose-profile consumer
   will not install a toolchain and may not have one. Any step that assumes a
   developer machine excludes them.
6. **The script-interpreter surface shrinks to the unavoidable.** Where an
   interpreter is genuinely unavoidable it must be dual-implementable — bash for
   Linux and macOS, PowerShell for Windows — and everything else moves into the
   binary. Designing that Windows surface is **`powershell-installer-surface`**'s
   work, not this amendment's; this amendment's obligation is to add no new
   shell-only install step and to assume no POSIX shell.

Objective 3 is the one that reverses a prior ruling rather than adding to it,
and **`gate-payload-disclosure-ruling`** owns its consequences. Objectives 1, 2
and 5 are jointly what voids building from vendored source at install time.

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

## What the ruling may and may not claim

The pivot is a trajectory; this iteration is one step on it. The tree still runs
a shell battery and every gate in it is shell, so the ruling below is the
**model** the substrate installs under, and it is inert until a gate ports —
which this iteration's envelope forbids.

That distinction is binding on the prose deltas: no user-facing surface may
state the dependency floor the objectives aim at as though it were reached. A
requirement page that claims git-only today is false, and the front door is
exactly where a false claim costs the most.

## The ruling

**The payload ships a prebuilt gate binary, selected by platform. The consumer
builds nothing, installs no toolchain, and receives no gate implementation
source.**

Building from vendored crate source at install time was ruled and is void:
it adds a Rust toolchain to a dependency floor objective 1 is collapsing, it is
unreachable for the non-technical adopter objective 5 admits, and it ships the
very source objective 3 wants withheld. Recorded rather than deleted, because the
next session to reach for the cheap answer should find it already costed.

### Weighing the two costs a prebuilt payload actually carries

Both were real objections before the objectives were visible. They are weighed
here against the objectives rather than against nothing.

**Cost 1 — the single-artifact model.** `scripts/pack-installer.sh` assembles one
payload that both transports consume. Two shapes survive the pivot: **one payload
carrying every declared target**, or **one payload per target**, the
optional-dependency shape the npm ecosystem uses.

*Ruled: one payload, carrying every declared target.* The **installed** footprint
is one binary either way — the installer writes only the matching target — so
objective 4 is served identically, and the difference is download size alone,
bounded by the target roster. Against that, the per-target shape multiplies the
publish path, the digest set, and the attestation surface by the roster size,
which is precisely what cost 2 makes expensive. Collapsing N attestation
surfaces to one is worth a bounded download.

*The revisit trigger is stated so it is a measurement and not a taste:* revisit
when the payload's download size becomes an adoption barrier — when the roster
grows past the platforms **`platform-support-ci-matrix`** commits to, or when a
single target's binary stops being small. Objective 4 makes that a number
somebody can watch, not an opinion.

**Cost 2 — the obligation opacity buys.** A consumer who cannot read the gate has
only the publisher's word for what it does, so the integrity story stops being
garnish and becomes the whole of what replaced reading the source. That
obligation is now **taken on deliberately** rather than avoided.

*Ruled: ship the achievable floor and claim nothing beyond it.* The floor is a
published per-target digest and an installer that verifies the artifact against
it before writing (delta 6). A genuinely reproducible build — a third party
rebuilding to the same bytes — is a larger program, and the queue already holds
its ground: **`tarball-build-attestation`**, whose one line says the checksum
proves transfer only and the docs agree. It is the entry this obligation lands
on, and the pivot is what turns it from a nicety into the thing that replaced
reading the source.

The honesty rule binds the prose, and today's release is the reason it has to:
`.github/workflows/publish.yml`'s checksum step says of itself that what it
proves is *same origin, same TLS session* — corruption and truncation, never a
compromised host. A governed surface may therefore say *verified against a
published digest*, and may **not** say *reproducible*, until that entry lands.

## What this iteration builds, and what is filed

**Ruled 2026-08-03 by the operator: split at the publish/consume seam.** This
iteration lands the **model** — the deltas below as ruled — plus only the
mechanism that is exercisable today. The implementation that produces and places
artifacts is filed as its own costed work, citing this amendment. The pivot is a
trajectory and this iteration is one step on it; the step does not swallow the
track.

The seam is sharp because nothing in the tree carries a `.gate` member and the
envelope forbids adding one, so every artifact-side mechanism would be built
against zero instances. What *is* exercisable today is exercisable against real
instances: the crate exists, the reference-only implementation exists, and the
kit roots exist.

| Delta | Disposition |
|---|---|
| 3, 8, 9, 10, 11 | **Landed 2026-08-03** and merged out of this file. Homes: gate-sdk/SPEC.md §Porting a gate to the binary substrate (criterion 5, §What is retained), §What the dispatch seam does not settle, §The extensibility model (new), §lib/gate.sh; `installer/README.md` §init; `docs/install.md` §Requirements; `CLAUDE.md` §Housekeeping. |
| 1, 2, 6 | **Filed: `native-artifact-publish-path`** — the target roster, the release build matrix, the per-target build, and the published digest. Publish side of the seam. |
| 4, 5, and the consume half of 2 and 6 | **Filed: `native-artifact-install-path`** — platform resolution, artifact selection, pre-write digest verification, the lock's artifact class, and omit-and-declare. Consume side, blocked on the publish side because it has nothing to select or verify until artifacts exist. |

Delta 7 files itself by construction — it names **`install-step-relocation`** and
builds nothing.

The deltas are **not** deleted into those entries. They are the ruled model and
they stay here, because an entry that has to re-derive its own design is the
compression loss this project keeps paying for; each filed entry cites this
amendment as its design source rather than restating it.

**Both filed entries inherit the objectives and the closed rulings**
(§The objectives this ruling serves). A session picking either one re-verifies
facts against the tree and does not re-litigate the rulings — prebuilt payload,
git-only floor, opacity-as-goal and the Rust substrate are closed, and only the
operator reopens a closed ruling.

## What changes

### 1. The substrate ships prebuilt, and its source does not ship {design-bearing}

`native/` stays a non-kit — it gains no `checks/` and no `smoke/`, the predicate
that makes a root directory a kit (CLAUDE.md §Housekeeping) — and, decisively,
**the crate root stays outside every kit root**, so the vendoring set
`gate_kit_roots_rel` derives can never reach the implementation. That is
objective 3 held by structure rather than by rule, and
**`gate-payload-disclosure-ruling`** delta 5 is the assertion that keeps it
structural.

`scripts/pack-installer.sh` gains an **artifact class** beside its kit roots: the
gate binary, built once per declared target by the release workflow and placed in
the payload under its target name. It is not copied from the working tree — no
build output is tracked, and the release is the only thing that produces it.

The **target roster is a tracked declaration with one owner** and exactly two
readers: the release workflow reads it to know what to build, and the installer
reads it to know what to look for. One roster, two readers, no second spelling —
a hand-maintained duplicate of a platform list in a workflow file is the
maintained-roster anti-pattern derivation-first refuses.

**The size of this change is stated rather than implied.**
`.github/workflows/publish.yml` has no build matrix, no OS-specific runner and no
compiled artifact anywhere: `pack` runs on one runner, and one
platform-agnostic tarball reaches both transports. The roster, the matrix, the
per-target build and the per-target digest are all new, and none of them is a
tweak to an existing mechanism.

### 2. Platform resolution, and the bootstrap that is left {design-bearing}

The installer resolves the host to a target name and selects the matching
artifact. That resolution is the **irreducible interpreter surface**: the binary
cannot select itself, so something outside it must run first.

Objective 6 binds the shape of what is left rather than its existence. The
bootstrap's whole job is: **resolve the platform, place the matching binary,
invoke it.** It is small enough to be written twice — `uname -s`/`uname -m` on
the bash path, the equivalent environment read on the PowerShell path — because
everything conditional lives on the other side of the invoke. Designing the
PowerShell path is **`powershell-installer-surface`**'s work and is explicitly
not done here.

This amendment's binding obligation to that objective is negative and checkable:
**it adds no new shell-only install step.** The prior ruling's install-time
`cargo build` was exactly such a step; the ruling that replaces it is a file
copy.

### 4. An unavailable platform omits and declares {design-bearing}

The mechanism the prior ruling introduced survives with its trigger replaced: not
*the toolchain is missing* but **no artifact exists for this platform**.

The install still succeeds — objectives 4 and 5 both refuse an install that can
fail on something the adopter did not choose. The affected members' descriptors
are not written and the members are not seeded into the consumer's `gates.list`;
each is recorded there as `# omitted: <name> substrate-unavailable`, a comment
line `gates_list_members` (gate-sdk/lib/gate.sh:66-68) already strips from the
live set. Named readers, unchanged: `gate-sdk/bin/run-gates.sh` prints the count
and the remedy as a **separate** line beside its summary — separate because
`gate-sdk/bin/run-consumer-smoke.sh`:60-68 greps `All N gates passed` verbatim —
and `installer/lib/doctor.sh` reports it against the platform that caused it.

**The honest limit, stated because it worsens as the pivot proceeds.** With one
gate ported, an unsupported platform loses one gate. With the battery ported, it
loses the battery. So the target roster **is** the support contract rather than a
build convenience, and **`platform-support-ci-matrix`** owns which platforms that
contract names. A consumer on a platform outside it gets a working install and an
honest, counted statement of what it cannot run — never a green battery over a
silently smaller roster.

### 5. The artifact is recorded in `checkwright.lock` as its own class {design-bearing}

`checkwright.lock` today records every written path — vendored and generated
alike — under one flat `files` map, path to content hash, with no class
discriminator; the vendored/generated split is a write-time behavior
(`claim()`'s hash-guard applies to vendored paths only, and a generated path is
unconditionally regenerated and recorded) rather than a distinction the schema
itself names. The binary is neither of the two behaviors: it is not authored
text the install claimed, and it is not locally reproducible the way the
pre-commit hook is. It joins as a new field — an **artifact** entry carrying
the target name and the digest, the schema's first named class.

Two readers, one record, which is why the class earns its place:

- **Uninstall exactness.** Objective 4's trial lifecycle needs the uninstall to
  remove precisely what the install wrote, and a binary the lock does not name is
  a file uninstall either leaves behind or guesses at.
  **`installer-lifecycle-verbs`** owns the verbs; this record is what makes the
  uninstall verb able to be exact.
- **Integrity re-verification.** The same digest lets `doctor` tell a consumer
  whether the binary they are running is the one that was installed.

### 6. The artifact is verified against a published digest before it is written {design-bearing}

Cost 2's floor, made mechanism. The release publishes a per-target digest; the
installer computes the digest of the artifact it is about to write and refuses on
a mismatch — refuses at that step specifically, rather than proceeding and
recording a bad hash, because a consumer who cannot read the gate has nothing
else standing between them and a substituted binary.

This is the one place the ruling *adds* a requirement rather than removing one,
and it is added because opacity is a goal rather than in spite of it.

### 7. Install-time logic moves toward the binary — named, not built {design-bearing}

Objective 6 is served long-term by shrinking what the bootstrap must do, and the
shell steps the install runs today are the candidates:
`gen-pre-commit.sh --write` and `check-graph.sh --emit`, both invoked
consumer-side by `installer/lib/init.sh`. Each is a pure function of tracked text
and each is a natural subcommand of a binary that is already present by the time
they run.

That relocation is **filed as `install-step-relocation`, not built here** — it is
gate-sdk work with its own contract questions, and doing it inside a vendoring
ruling would widen this iteration's unit set.

What binds here is only the negative: no new shell-only install step, and no
design that would have to be unbuilt when the relocation lands. The ordering
constraint the prior ruling introduced disappears with the build step it existed
for — a copied binary is present before any post-write step runs.

## Producers and consumers

**Target roster (new tracked declaration).**
Producer: authored once, tracked, with the crate. Consumers, both named and both
real: the release workflow's build matrix, and the installer's artifact
selection. Its enabling config is its own tracked presence — nothing sets it, so
there is no configuration under which a producer exists and no reader is
reachable. A target added to the roster and not built fails the release, which is
the correct place for that failure.

**Payload artifact (new payload class).**
Producer: the release workflow, once per declared target, at publish time.
Consumer: `installer/lib/init.sh`'s selection-and-copy step. It is never produced
from a working tree and never tracked, so no local build can substitute for it.

**Platform resolution (derived, never stored).**
Producer: the installer bootstrap's platform probe — **new, and new is the
finding**: no platform-triple derivation exists in the tree.
`context-kit/lib/toolfloor.sh` does no OS or architecture logic at all, and
`context-kit/bin/env-probe.sh`'s single `OS:` field is an unparsed
`uname -s -r -m` string rendered for a human to read in `ENV.local.md`. That
field is explicitly **not** the producer: making a human-read free-text line into
a machine-consumed selector is how a display string becomes a contract nobody
declared. Consumer: the artifact selection step. The resolution is a local — a
stored copy would be a second source for a fact the host answers, and would go
stale the first time a tree moved between machines.

**Artifact record in `checkwright.lock` (new state, one new field pair).**
Producer: `installer/lib/init.sh` after a verified write. Consumers: the
uninstall verb (**`installer-lifecycle-verbs`**) reads the path to remove exactly
what was written; `installer/lib/doctor.sh` reads the digest to re-verify the
binary in place. The record carries the target name and the digest and nothing
else, because those two are what the two readers read — no field is reserved
against a future reader.

**Published digest (new release output).**
Producer: the release workflow, beside each artifact. Consumer: the installer's
pre-write verification (delta 6), and `doctor`'s re-verification through the lock
record. This is the one new external contract the ruling creates, and it has a
named reader at a named transition on both sides.

**Omitted-member record.**
Producer: `installer/lib/common/recipe.sh`'s roster seeding, when the host's
platform has no artifact. Consumers: `gate-sdk/bin/run-gates.sh`'s summary line
and `installer/lib/doctor.sh`'s report. One field beyond the member name — the
reason token `substrate-unavailable` — read by the remedy text those two print.

**Existing integration this changes.** `installer/lib/init.sh`'s post-write
sequence gains a write step and loses nothing; `scripts/pack-installer.sh` gains
an artifact class; the release workflow gains a build matrix and a digest
emission. `context-kit/lib/toolfloor.sh`'s `cargo` member keeps its
contributor-and-CI role and acquires no install-time consumer — the prior ruling
gave it one and this ruling takes it back, which is a removal to propagate rather
than an addition to make.

## Existing sections updated

Owned by this amendment, each named with the delta that claims it. The landed
deltas' entries are struck from this list as they land, so what remains here is
exactly what the two filed entries still owe.

- gate-sdk/SPEC.md §Porting a gate to the binary substrate — the port's own
  justification, which this ruling's grounds change (delta 1). Criterion 5 and
  §What is retained, and what a second port must do first are **landed**
  (delta 10).
- gate-sdk/SPEC.md §Layout and configuration — the target-roster declaration and
  any knob the artifact selection takes (deltas 1, 2).
- gate-sdk/SPEC.md §Consumer smoke — the vendored-tree runnability statement,
  criterion 5's other reader (delta 4).
- gate-sdk/SPEC.md §run-gates — the summary's omitted-member line (delta 4).
- gate-sdk/SPEC.md §upgrade-smoke — the artifact's treatment in the
  determinism diff, which must not read a platform-selected binary as drift
  (delta 5).
- `installer/README.md` §The manifest — the field table gains a new field for
  the artifact entry (target name, digest). **Re-verified against the tree:**
  today's table has one `files` row, a flat path→hash map covering both
  vendored and generated paths with no class discriminator; the
  vendored/generated split delta 5 opens with is a write-time behavior
  (`claim()`'s hash-guard applies to vendored paths only) rather than an
  existing schema-level class. The artifact is still a new field the table
  does not carry today, which is delta 5's point regardless (delta 5).
- `RELEASING.md` — the release gains a build matrix, per-target artifacts and a
  digest emission, so the runbook that drives it changes with them (deltas 1, 6).

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
      retired; nothing dangles. The install-time toolchain role the prior ruling
      introduced is one of them.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **No port lands.** The iteration's envelope is unblock; no `.gate` member
      is added to any kit.
- [ ] **No surface claims a dependency floor the tree has not reached.** The
      objectives are the direction; the requirement pages state the tree.
