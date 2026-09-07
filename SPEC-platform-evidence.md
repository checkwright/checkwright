# SPEC amendment: platform-evidence

**`native/targets.list` carries one triple against a documented supported set of Linux and macOS,
the widening is owned by a retired entry, and fifteen of the sixteen files the port oracle still
calls `owed` chain to this one precondition.** This amendment builds the **producer** whose green
run can discharge the join bound for a second target, declares the supported set on a surface a
gate can read, and states — mechanically, so no later session re-derives it — what licenses the
roster line to be written.

**It does not write the roster line, and §Why the roster line is not asserted here says why in
full.** Asserting a target on a run that has not happened is the one failure
`gate-sdk/SPEC.md` §Consumer payload names by name: "a roster widened on that reading is widened
on a plan".

## The ruling this amendment implements, and the reading it does not take

**Ruled by the OPERATOR 2026-09-07, through the lead's question relay in a lead session and
lead-relayed.** The join bound for a second target is discharged by a **release-shaped producer**:
a `native-artifacts` matrix job in `.github/workflows/gates.yml` mirroring `.github/workflows/publish.yml`'s
build leg, uploading the binary and its `.sha256` sidecar, with the platform smoke consuming that
upload instead of building from the host it runs on.

**`.github/workflows/gates.yml`:790-801 stands unamended.** Its verdict — that a zero on
`install-smoke-macos` licenses no line in `native/targets.list` — is left exactly as authored, and
this amendment neither rewrites nor argues with it. What licenses a line under this ruling is a
predicate naming **two** jobs, stated at §What licenses the roster line below, and that predicate
is new rather than a reinterpretation of the paragraph.

**The alternative reading was put and refused, and the grounds are recorded so it is not
re-argued.** The textual case for joining `aarch64-apple-darwin` on the existing binding leg's
green is real: `gate-sdk/SPEC.md`:7580-7586 carries no "released" qualifier, its stand-in clause
at :7638-7646 sits inside the pack/publish passage and exempts the smoke from *that* rule, and
`TRAJECTORY.md`:289-291 states no trigger content of its own. It was refused anyway, on the ground
that `gates.yml`:790-801 was authored one day before the reading it pre-refuses, and that
reversing a deliberate anti-misreading guard on strong-looking grounds is exactly the class of
move an operator ruling exists for. **The text is not what lost; the reversal is.** A later
session finding the textual case persuasive is reading a settled question.

**A recorded residue, filed rather than fixed here.** Once delta 5 lands, `gates.yml`:790-801's
stated *ground* — "the smoke builds its artifact from the host it runs on" — is no longer true of
that leg, while its *verdict* stays correct because the licence never came from that leg alone.
The paragraph is left unamended by the ruling above; correcting its parenthetical ground is not
this unit's work and is filed to the gap inbox rather than carried silently.

## The target set this amendment asserts, and what it records as owed

**Ruled `lead, own-authority` 2026-09-07**, correcting a false premise on the queue entry rather
than re-scoping any ruling.

- **Joined, asserted:** `x86_64-unknown-linux-gnu`, unchanged.
- **Owed, `aarch64-apple-darwin`** — precondition: the run described at §What licenses the roster
  line. Under the operator's ruling this target is **not** joined on landing; the entry's own
  framing, which read the existing binding macOS green as sufficient, does not survive that
  ruling.
- **Owed, `x86_64-apple-darwin`** — precondition: the same predicate, on its own Intel leg. The
  `install-smoke-macos` leg runs on `macos-latest`, which is arm64, and that leg's own comment
  refuses reading one green arm64 run as a macOS claim. Intel macOS is a separate leg and a
  separate green.
- **Owed, `x86_64-pc-windows-msvc`** — precondition: `docs/install.md` §Requirements documenting
  native Windows as supported, **which it does not**. That page says Windows runs through WSL,
  "not natively", and `installer/SPEC-powershell-bootstrap.md` (merged at `bd633f51`) explicitly
  non-asserted the platform claim when it shipped the PowerShell bootstrap: "This amendment ships
  the bootstrap, not the platform claim." The queue entry's premise — that shipping that half made
  native Windows supported — is false at HEAD, and the first of §Consumer payload's two bounds
  refuses the triple on that ground alone, independently of any run.

## What changes

### (1) `docs/install.md` §Requirements declares its supported platforms in a marker block

The install page gains a `<!-- platforms:begin -->` / `<!-- platforms:end -->` block naming, per
documented supported platform, its Rust target triple and its **join state** {design-bearing}.

Two states and no third: **`joined`**, meaning the triple is a live line in `native/targets.list`;
and **`held: <precondition>`**, meaning the platform is documented as supported, is not in the
roster, and carries the named run that would join it. A held entry's precondition is mandatory —
a hold with no stated cause is how a pile grows silently, which is the subject of the sibling unit
`born-native-omission-accumulation`.

**Why this block and not a second roster file.** `gate-sdk/SPEC.md`:7580-7581 bounds a roster line
by "what the project's own install documentation already states", and today that bound is held by
prose alone: nothing can compare the roster to the page. Declaring the set *on the page it is
already stated on* makes that bound mechanizable without minting a second place where platform
support is asserted, and the page already carries a marker block of exactly this shape —
`<!-- toolchain:begin -->`, read by `check-install-toolchain` — so this is that established
convention applied to a second axis rather than a new one.

The block's content at landing: `x86_64-unknown-linux-gnu` joined; `aarch64-apple-darwin` and
`x86_64-apple-darwin` held, each with the precondition §The target set names.
`x86_64-pc-windows-msvc` is **absent**, because the block declares what the page states as
supported and the page does not state it — its owed-ness lives in prose here and in delta 6's
roster header, not in a block that would then be asserting a support claim the page refuses.

### (2) `native/runners.list` — the target-to-runner map, one surface with two readers

The `declare -A runner=(…)` map inlined at `.github/workflows/publish.yml`:54-56 moves to a tracked
file read through a new `gate_native_runner` helper in `gate-sdk/lib/gate.sh`, beside the existing
`gate_native_targets` {design-bearing}.

The move is forced by delta 3 rather than chosen: option B mints a **second** consumer of that
map, and two workflows each carrying their own copy is precisely the maintained duplication
derivation-first refuses — with the failure mode that the release builds a target on one runner
class and CI measures it on another, which is invisible until a published artifact behaves
differently from the one that was exercised.

The map's separation from the roster is unchanged and its stated reason survives the move:
`publish.yml`:44-48 rules that a runner mapping "is a runner selection, so it may name a platform
where the matrix declaration may not". That is why it stays a distinct file rather than a column
on the roster — under this amendment it must name `aarch64-apple-darwin`, which the roster
deliberately does not.

### (3) `.github/workflows/gates.yml` gains `native-artifacts`, the release-shaped producer

A matrix job mirroring `publish.yml`'s build leg {design-bearing}: for each platform the delta-1
block declares — **joined and held alike** — it runs `bash gate-sdk/bin/build-native.sh --target
"$TARGET"` on the runner delta 2's map names, writes `<binary>.sha256` beside the binary in
`<artifacts>/<target>/` with a bare filename inside it, and uploads the target directory.

Three properties are load-bearing and each is stated because the cheap version of this job would
drop it:

- **The producer is the release's own, not a second one.** The build command, the sidecar's
  format and the directory layout are `publish.yml`:98-121's, so what CI exercises is what a
  release would publish. A job that built the binary a different way would measure a different
  artifact and discharge nothing.
- **One producer, no recomputation.** The bytes this job writes are the bytes the consuming smoke
  installs, because delta 4 moves the file rather than re-deriving it. §Consumer payload's
  standing rule — "a second `sha256sum` on a later job is exactly what lets a published digest and
  an installed digest diverge while both look computed" — reaches this path unchanged.
- **The matrix is the declaration, not the roster.** Deriving it from `native/targets.list` would
  build only what is already joined and measure nothing, which is the whole point of the job. CI
  measures every documented platform; publish builds only the joined ones. That asymmetry is the
  mechanism by which a held platform can ever stop being held.

### (4) The consumer smoke consumes a prebuilt artifact directory, and steers itself by default

`installer/consumer-smoke/run-smoke.sh` gains `INSTALLER_SMOKE_ARTIFACTS_DIR`, and — separately —
stops depending on the caller to steer its roster {design-bearing}.

**The knob.** When `INSTALLER_SMOKE_ARTIFACTS_DIR` is set, the smoke does not build: it takes that
directory as the `--artifacts` operand it today assembles itself, and its `cargo`/`rustc`
preflight relaxes **on that path alone** — a host consuming a prebuilt artifact has not been asked
to compile anything, so refusing it for a missing compiler would refuse the exact case this delta
exists to serve. The preflight is unchanged everywhere else, and
`installer/README.md`:2466-2470's ground for it ("a machine that cannot compile the crate has not
falsified the install path") is preserved by being made conditional on the smoke actually
building.

**The default steering, and this is the half that decides whether a second roster line is
affordable.** `installer/README.md`:2439-2451 records that "the moment the roster declares a second
target the build step blocks rather than passing… a second declared target stops the whole smoke
rather than only its last arm", and names two exits: steer at a narrowed roster through
`GATE_SDK_NATIVE_TARGETS_FILE`, or give the step a cross-compiling build. **Steering is ruled;
the cross-build is refused**, on grounds already in the tree rather than on cost:
`native/targets.list`:21-23 refuses cross-compiling because "it would publish an artifact no run
has ever executed", which is the same bound this whole amendment is built around.

So the smoke derives a one-line roster from `rustc -vV`'s host triple and points
`GATE_SDK_NATIVE_TARGETS_FILE` at it **unless the caller has already set that knob** — making the
three platform legs' hand-rolled steering redundant rather than wrong, and making a local
`bash installer/consumer-smoke/run-smoke.sh` survive a two-line roster on a developer machine.

**What is given up, stated rather than discovered.** The Linux `install-smoke` leg today asserts,
incidentally, that the *shipped* roster is packable — it steers nothing and its host happens to be
the roster's one line. Under default steering that incidental assertion is gone. What replaces it
is stronger: delta 3 builds **every declared platform on every run**, so the shipped roster's
producibility is asserted directly instead of by coincidence, and `pack-installer.sh`'s refusal of
a declared target no leg built keeps its meaning at release.

### (5) `install-smoke-macos` exercises the uploaded artifact rather than a host build

The leg gains `needs: native-artifacts`, downloads that job's `aarch64-apple-darwin` upload, and
points `INSTALLER_SMOKE_ARTIFACTS_DIR` at it {design-bearing}. Its roster steering is unchanged in
effect and now redundant with delta 4's default. Its "probe the crate build" step stays, reporting
only, because a crate that stops compiling on macOS is still the first thing to fail and the first
thing a reader of a red leg needs.

`gates.yml`:790-801 is **not** touched — see §The ruling this amendment implements.

### (6) `native/targets.list`'s header discharges the retired owner and states the new predicate

The header is rewritten to name a live owner for the widening and to carry §What licenses the
roster line's predicate verbatim {design-bearing}.

**Three citation sites, not four.** `platform-support-ci-matrix` retired 2026-09-06 while still
named as the owner of this live, undischarged precondition. The sites, re-resolved by content at
this stage: `native/targets.list`:31, `native/targets.list`:51-52 and
`installer/README.md`:611-612. The queue entry's claim of a fourth at `installer/README.md`:442 and
a further one at :530 does not survive the file — it carries **exactly one** occurrence of that
slug, at :611-612.

`installer/README.md`:611-612's sequencing clause is repointed at delta 1's declaration block,
which is a permanent surface, rather than at any queue entry — an entry is a lifetime that ends,
and pointing a live precondition at one is what produced this defect in the first place.

### (7) The roster line's write is specified mechanically, so it needs no second design turn

This amendment states the three edits that join a target and the predicate that licenses them,
so the session that observes the green executes rather than re-derives {mechanical}. Both live in
§What licenses the roster line below and in delta 6's rewritten header.

## What licenses the roster line

A target `<T>` joins `native/targets.list` when **one run, at or after this unit's landing commit,
carries both** of:

1. `native-artifacts` green for `<T>` — its binary and `.sha256` sidecar produced by the release's
   own build command and uploaded; and
2. a platform install-smoke leg green having **consumed that upload** — `INSTALLER_SMOKE_ARTIFACTS_DIR`
   pointed at the downloaded directory — and having reached the artifact-present branch, which its
   log states as `artifact: <T> verified in place at scripts/checkwright-gates, recorded with the
   seam, nothing omitted`.

That is *produced and exercised* with no host-built stand-in anywhere in it, which is what the
operator's ruling bought.

On that observation, three edits and nothing else: the roster gains `<T>`'s line; `native/runners.list`
already names its runner (delta 2); and delta 1's block flips `<T>` from `held` to `joined`. The
sibling unit's gate holds those three in lockstep, so a partial write reds at pre-commit.

### Why the roster line is not asserted here

**It cannot be, and the arithmetic is worth stating rather than leaving to be rediscovered.**
The licence above is a green that must **precede** the write. Under CLAUDE.md's one-to-two-pushes
budget, the first watched push buys that green (deltas 1-6), and the roster line can then ride the
second. That is the whole budget with no slack, and a red on the first push spends the second
re-buying it — in which case the entry's terminal move is a **demotion** rather than a Done, with
its precondition narrowed to the observed green.

**Delta 4's default steering is what makes the second push cheap enough to attempt at all.**
Without it, the roster line would also have to steer the binding Linux leg in the same commit that
widens the roster — putting the widened roster and an untested change to the most load-bearing leg
in one unwatched write. With it, the roster line changes nothing any `gates` leg does, and the
second push's green is a confirmation rather than an experiment.

**Recorded honestly:** whether the second push is spent on the roster line is a batch-cut decision
the lead holds at build, not a decision this amendment can make, because it turns on an
observation that does not exist yet.

## Producers and consumers

**New state — the platform declaration block (delta 1).**
Producer: hand-authored in `docs/install.md` §Requirements; no code writes it, exactly as the
`toolchain:begin` block beside it is hand-authored. Consumers: (a) `native-artifacts`'s matrix
derivation (delta 3), which reads every declared triple regardless of state; (b) the sibling
unit's `check-install-platforms`, which holds the block and the roster in lockstep at
`precommit` tier; (c) a human reading the install page, for whom the block is invisible HTML
comment content and the surrounding prose is unchanged.
Fields and readers: **triple** — read by delta 3's matrix and by the sibling gate's arms A and B;
**state** — read by the sibling gate's arms A, B and C to decide which of "must be in the roster",
"must not be" applies; **precondition** — read by the sibling gate's arm C, which reds on an empty
one, and by the session at §What licenses the roster line. No field is added that no reader
consumes.

**New state — `native/runners.list` (delta 2).**
Producer: hand-authored; its enabling configuration is nothing, since both readers resolve it
through `gate_native_runner` with the repo layout as the default, per this repo's config-via-env
convention. Consumers: `publish.yml`'s roster job, which today inlines the same map and fails the
release on a target it has no entry for — that refusal is preserved verbatim through the helper —
and `gates.yml`'s `native-artifacts` job (delta 3), which takes the identical refusal for the
identical reason.

**New event — `native-artifacts`'s per-target upload (delta 3).**
Producer: the matrix leg, on every `gates` run, reachable with no configuration because the matrix
is derived from a tracked file. Consumer: `install-smoke-macos` (delta 5) by
`actions/download-artifact`, and — as the second half of §What licenses the roster line — a human
reading the run.
Fields and readers: the **binary** is read by delta 4's pack path, which installs it into every
consumer profile; the **`.sha256` sidecar** is read by the installer's step-4 digest verification
inside that smoke, which is the transition the artifact-present branch's log line reports. Neither
is written twice: delta 4 moves the file rather than recomputing it, so §Consumer payload's
one-producer rule holds across the new hop.

**New interface — `INSTALLER_SMOKE_ARTIFACTS_DIR` (delta 4).**
Producer: `gates.yml`'s `install-smoke-macos` step (delta 5) — a real deployed configuration and
not a test-only one, which is the point of naming it here. Consumer: `run-smoke.sh`'s pack path,
at the transition where it today assembles the artifact directory itself. Unset everywhere else,
where the smoke's behaviour is byte-for-byte what it is today.

**Existing integration prose updated:** `installer/README.md` §The consumer smoke's honest-limit
paragraph describes the prior flow — a single host build satisfying `--artifacts`, and the
re-entry the second roster line forces — and delta 4 resolves exactly that re-entry, so it is
updated here rather than left to describe a shape the tree no longer has.

**Delta 4 narrows a corpus, so each affected reader's RED condition is enumerated rather than its
subject.** The narrowing: the smoke's roster goes from the shipped file to a one-line host roster,
so the pack step stops comparing the shipped roster against the host.

- `install-smoke` (Linux, **binding**) — reds on a non-zero `run-smoke.sh`. Monotone in the
  violation set: the narrowing can only remove the roster/host mismatch refusal, and that refusal
  cannot fire today, since the host is the roster's one line. Clearable by inspection.
- `install-smoke-macos` (**binding**) — same red condition; it already steers explicitly, so its
  behaviour is unchanged by the default.
- `install-smoke-windows` (`continue-on-error`) and `install-smoke-powershell` — same red
  condition, both already steering explicitly.
- `pack-installer.sh`'s refusal of a **declared target no leg built** — this is the one reader
  whose verdict is *not* monotone under the narrowing, because it reds on finding a target rather
  than on finding none, and a narrowed roster removes its subject. It is cleared by construction
  rather than by inspection: the smoke **plants** that case explicitly and asserts the refusal, and
  the assertion's own log line is `declared target with no artifact: refused, not omitted`. So the
  refusal keeps a live witness under the narrowing instead of becoming unreachable.
- The sibling unit's `check-install-platforms` — reds on a roster line the block does not declare
  `joined`, and on a `joined` declaration absent from the roster. Delta 4 touches neither surface,
  so its verdict is unmoved.

## Existing sections updated

- `gate-sdk/SPEC.md` §Consumer payload, its "Widening is cheap on the publish path and not free
  elsewhere" paragraph — it names the smoke's re-entry as unbuilt and offers two exits; delta 4
  builds one and refuses the other, and installer/README.md is named there as the owner that
  "records which of the two is built" (delta 4).
- `gate-sdk/SPEC.md` §Consumer payload, its two-bound paragraph — bound (i) stops being held by
  prose alone once delta 1 gives it a readable surface, and the paragraph says which surface
  (delta 1).
- `gate-sdk/SPEC.md` §Layout and configuration — the roster knob's neighbour, gaining
  `gate_native_runner`'s file and its resolution (delta 2).
- `installer/README.md` §The consumer smoke — the honest-limit paragraph on a single host build
  satisfying `--artifacts`, and the knob roster, which today states `INSTALLER_SMOKE_TMP_DIR` as
  the smoke's only knob (deltas 4 and 5).
- `installer/README.md` §The install boundary:610-612 — the sequencing clause naming a retired
  entry as the owner of a live precondition, repointed at delta 1's block (delta 6).
- `docs/site-architecture.md` §Generated projections and their freshness gates — delta 1's block
  is a second parity-contract surface on the install page, and the roster there is where a reader
  looks for what holds it (delta 1).
- `docs/install.md` §Requirements — the page that gains the block, whose prose already states the
  supported set the block declares (delta 1).

<!-- update-target-exempt: the sibling unit born-native-omission-accumulation authors check-install-platforms against delta 1's block; the gate is that unit's deliverable and no delta here defines it, so citing a delta of this amendment for it would be a claim this amendment does not make -->
- `SPEC-platform-coverage.md` — the sibling amendment, which reads delta 1's block.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
