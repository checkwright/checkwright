# SPEC amendment: the packer becomes a non-gate arm

The terminal cut of the native gate-binary port. `scripts/pack-installer.sh` is
the last file in the port oracle's owed column; this amendment moves it in-crate
as a bridged `Arm::Run` non-gate arm and takes that column to zero.

**The two rulings this amendment executes are cited and not restated.** The
port's completion predicate is the operator's, 2026-08-28, literal and admitting
no contributor-side subtraction — TRAJECTORY.md's. The route the cut takes is
ROUTE 1, build the crate inside the release workflow's `pack:` job, ruled by the
operator 2026-09-09 through the AskUserQuestion channel; its disposition, its
refused alternatives and the reachability obligation it discharges already sit in
`installer/README.md` §The packer. Neither ruling's provenance belongs in a kit
SPEC (CLAUDE.md §The provenance seam), so what follows is mechanism.

**What this amendment does *not* decide.** The pack step's clean-tree predicate
is the sibling amendment `installer/SPEC-pack-clean.md`'s, joined to this cut on
shared surface. This one mints the arm; that one scopes the arm's refusal.

**The seam this cut crosses, ruled here because the cut is the first thing to
move mechanism from a repo-private script into an artifact every adopter
receives.** The arm's *mechanism* ships — it lives in the binary the payload
carries — while no adopter path invokes it, which is a combination this repo has
not had before, so the three-way split is stated rather than left to inference.

- **Kit mechanism, and it ships:** assembling a payload out of a repository's own
  kit roots and its `installer/` directory, verifying each declared target's
  artifact against the sidecar its build leg emitted, restoring the executable
  mode the transports drop, and stamping a version and a commit. Every one of
  those is already `gate-sdk/SPEC.md` §Consumer payload's contract, generic over
  any consumer that redistributes kits, and the arm carries no term list, no
  coupling vocabulary and no product constant. Delta 2's new value is kit
  mechanism on the same test — a derived name, not a name of this project's.
- **Repo-private, and it stays out of every kit SPEC:** the *disposition* — that
  this repository's release path is what invokes the arm, from which job, and
  under which ruling. That is `installer/README.md` §The packer's, which is
  repo-root-governed with no owning kit, exactly the placement its own seam
  ruling reached for the shell form. Delta 7 re-subjects that section rather
  than relocating it into a kit, and the `gate-sdk/SPEC.md` edits delta 1 and
  delta 2 make are the arm's roster membership and its knob, never its purpose
  here.
- **Consumer config, unchanged in width:** every input the tool reads stays a
  knob. The four declared names keep their existing contracts and the scratch
  base keeps its spelling, so nothing an adopter or a workflow could set before
  the cut becomes a crate literal after it — the property that distinguishes a
  port from a narrowing.

## What changes

### (1) `scripts/pack-installer.sh` becomes `--pack-installer`, a bridged `Arm::Run`

The packer moves into the crate as `native/src/emit/pack_installer.rs`, reached
by the bare flag `--pack-installer`, resolved in `main` before the registry
lookup and absent from `--list`. {design-bearing}

**The flag spelling is minted here, with its readers.** A spelling written into a
SPEC ahead of its implementation is a reservation, so this is the mint: the
readers are the seven call sites delta 4 re-points, and the spelling keeps the
diagnostic prefix `pack-installer:` the tool already prints, so a reader of a
finished CI log meets one name and not two.

**`Arm::Run` rather than `Arm::Emit`, and the ground is not the status grammar.**
The tool's statuses are 0 and 2 alone, which `Arm::Emit` would carry losslessly —
so the usual forced-family test does not decide it. What decides it is that the
member's product is a **tarball on disk** plus a one-line `PACK:` receipt, not a
document its caller reads; an `Arm::Emit` member returns the document it renders,
and a member whose real output is a side effect would be returning a receipt for
it. `--emit-` is a per-arm spelling rather than a family name (§The non-gate
arm), so a bare flag costs nothing and claims nothing false.

**Bridged-arm table membership is forced, not chosen.** The tool reads the
kit-root set, the target roster and the binary's name, and every one of those is
consumer-overridable — the Windows pack leg steers the roster knob explicitly. A
hardcoded top-level flag is reached by no bridge and would resolve platform
defaults while silently ignoring every override, which §The non-gate arm calls
the difference between working and appearing to.

**The declared knob roster is four names:** `GATE_KIT_ROOTS_REL`, the packed kit
set in the anchored spelling the pack loop needs for a pathspec;
`GATE_SDK_NATIVE_TARGETS_FILE`, the roster whose verbatim copy the payload
carries; `GATE_SDK_NATIVE_BIN`, whose basename the artifact names are built from;
and `GATE_SDK_NATIVE_ARTIFACT_NAMES`, minted by delta 2.

**`INSTALLER_PACK_TMP_DIR` and its `TMPDIR` fallback are absent from that roster
and must be.** Neither carries a kit prefix the bridge can partition by and
neither is defined in any kit library, so declaring either would meet the
undeclared-knob refusal and fail-close the arm on every invocation. Both are read
straight off the process environment instead. This is `--run-demo`'s third shape
exactly — a member declaring what it can while reading what it may not declare —
and it is recorded so the short roster does not read as an omission. The knob's
**name is unchanged**, which is what keeps every call site's existing environment
prefix working across the cut.

**The spawned-program set, stated because this class records each member's.**
`git` — `archive`, `rev-parse`, `describe`, `ls-files` and `status`; `tar`, to
extract the archive stream; and `npm`, to pack. **`jq` leaves with the port**, the
version-and-commit stamp becoming a `serde_json` edit through `json.rs`, and
**`sha256sum` leaves too**, the sidecar verified in-crate through `sha256.rs`.
`mktemp`, `mkdir`, `cp`, `mv`, `chmod` and `rm` all become `std::fs` calls, and
the scratch teardown becomes the arm's own `Drop` rather than an `EXIT` trap —
the shape `--run-demo` already established. `git archive | tar` and `npm pack`
stay spawns deliberately: reproducing either in-crate is a second implementation
of a format, not a port.

**The preflight tool check narrows with that set.** The shell form refuses unless
`npm`, `jq`, `git` and `tar` are all on `PATH`; the arm refuses on `npm`, `git`
and `tar`, because `jq` is no longer reached. A tool check that names a program
nothing runs is the same defect as a documented flag that does nothing.

### (2) The per-target artifact name crosses the bridge as a value, not as a rule

`gate-sdk/lib/gate.sh` gains a bridgeable `GATE_SDK_NATIVE_ARTIFACT_NAMES` — one
element per roster line, each the target triple and the artifact name that
target's build leg emits — computed once from `gate_native_targets` and
`gate_exe_suffix`. The arm reads it and derives nothing. {design-bearing}

**The problem this solves is criterion 6, and a naive port walks into it.** The
shell form builds each artifact name by asking `gate_exe_suffix` for that
*target's* suffix, and gate-sdk/SPEC.md §lib/gate.sh asserts that function is the
executable suffix's single owner — an assertion the crate honours today by
spelling the suffix nowhere at all. The function's other caller of the
target-triple form is `gate-sdk/bin/build-native.sh`, which cannot itself move
in-crate: its body *is* the binary's build, so its arm would predate itself. So
the shell holder is permanent, and a crate-side `*-windows-*` rule would be a
second, permanently dual-implemented owner of a one-line predicate.

**Carrying the value rather than the rule is criterion 6's discharge by
construction, which is stronger than its *unless* clause asks.** The bridge
transports a resolved value across the dispatch seam, so there is exactly one
place the suffix is computed — the kit's shell library — and the binary holds
nothing to drift. This is `GATE_KIT_ROOTS_HERE`'s own precedent, and its own
recorded reasoning: a value a consumer can override is not derivable on the
binary side, so it is resolved once and carried.

**The refused alternative is the *unless* clause's road:** the triple rule
in-crate plus a machine-held cross-substrate comparator. Refused because the
comparator would be a parity arm whose second holder can never empty — a
permanent member minted to hold a single `case` line equal to itself — where the
bridge removes the duplication outright.

**Two properties of the new value, stated because both are easy to lose.** It is
computed **after** `GATE_SDK_NATIVE_TARGETS_FILE` resolves in the same library,
so a caller that steers the roster knob in the arm's environment — the Windows
pack leg does — gets a value derived from the steered roster rather than the
default one. And it costs one small file read per sourcing of `lib/gate.sh`, the
same class of source-time cost `GATE_KIT_ROOTS_HERE`'s parent-directory walk
already carries.

**`exe-suffix-single-spelling-unenforced` is untouched and stays open.** That
entry's subject is the missing *gate* behind the single-owner invariant; this
delta keeps the invariant true rather than enforcing it, and leaves that entry's
literal scan exactly as clean as it is today.

### (3) The release workflow's `pack:` job builds the binary

`.github/workflows/publish.yml`'s `pack:` job gains a
`bash gate-sdk/bin/build-native.sh` step ahead of its assemble step, so the arm
the assemble step now invokes is reachable in the one job that had no build.
{mechanical}

This is ROUTE 1 executed. The obligation, the route and the refusal of the
alternative — resolving `GATE_SDK_NATIVE_BIN` onto a downloaded artifact — are
§The packer's and are not restated. The job runs on the runner's preinstalled
cargo, the disposition the sibling `build:` job's own comment already records for
this workflow.

**The reachability obligation is discharged here and nowhere else.** The consumer
smoke already builds the binary before it reaches any pack call site, and the
macOS install-smoke legs adopt a compiled binary from the build legs' hand-off,
so those callers were never the gap.

### (4) The seven call sites are re-pointed, and the caller now pins two things

Every `bash scripts/pack-installer.sh …` becomes
`bash gate-sdk/bin/run-gates.sh --pack-installer …` with its arguments unchanged.
Two workflow steps and five consumer-smoke scenarios. {design-bearing}

**The cut splits one decision into two, and the callers must pin both.** The
front-end resolves the gate binary and the bridged environment relative to the
git toplevel of the **current directory**. Today the packer is a script inside
the tree it packs and `--root` alone decides which tree that is — the property
§The consumer smoke states as *a clone's copy invoked by absolute path, a second
checkout, a linked worktree: all pack the tree the script belongs to*. After the
cut the **cwd** selects whose tooling and configuration run and **`--root`**
selects which tree is packed and stamped, and the two are no longer the same
decision.

So each of the five smoke call sites runs the front-end in a subshell whose
current directory is the smoke's own script-derived repository root, and keeps
`--root` naming that same root. The two workflow steps already run at the
checkout root and need no change beyond the command.

**This does not reinstate the retired invocation requirement.** That rule bound
the smoke's *caller* — "run it with the current directory inside the tree under
test" — and `--root` retired it. What lands here is the smoke pinning a directory
it derives from its own path, so the caller's directory still selects nothing;
the same shape the smoke already uses one screen earlier to source the gate
library. The property §The consumer smoke promises is preserved, by a different
mechanism, and the section says so rather than leaving a reader to reconcile them.

### (5) The packer's single-tier flag roster retires with the file

§The packer's rule that the flag roster has exactly one tier and it is the tool
itself — `--help` printing it at exit 0 — retires. A bridged arm's usage lives in
`bin/run-gates.sh`'s own help and in the owning surface's prose, which is where
the arm's grammar lands. {mechanical}

The rule is retired rather than quietly dropped because its ground was specific:
with a flag surface that wide, an unknown-argument refusal was too thin a
discovery route to be the only one. The arm keeps the refusal and gains the
front-end's help, so the discoverability the rule bought is preserved and its
one-tier claim, which the file's deletion makes false, is not left standing.

### (6) The arm has one refusal formatter, so no exit path is silent

Every refusal returns a typed error through a single formatter that prints the
`pack-installer:` prefix, the cause and any help lines on stderr and exits 2.
{design-bearing}

The shell form runs under `set -uo pipefail` without `-e`, so it has exit paths
that print nothing at all, and *exited non-zero having printed nothing* is
exactly what made one red master unreadable from a finished run. The arm makes
that state unreachable by construction, which is a dividend of the substrate
rather than a fix authored on top of it.

**The honest limit, stated so this is not read as a diagnosis.** A process killed
by a signal prints nothing whatever its code does, so this delta does not explain
the 2026-09-08 Intel-leg firing. That firing's cause is
`binding-intel-leg-failed-one-run-in-two`'s subject and stays open there.

### (7) §The packer is re-subjected onto the arm, and three `no-port`/`spec` grounds move with it

The section's whole premise — *`scripts/pack-installer.sh` assembles the payload
both transports ship … This section owns that file* — re-subjects onto the arm,
keeping every contract it states and changing what states them. Its port
disposition paragraph, whose subject is a file in the reachable column, is
replaced by the record of the cut that took it. {design-bearing}

Three grounds elsewhere cite the packer *as a shell file* and are re-worded to
the arm in the same commit, because each is the stated reason for a live
declaration rather than incidental prose: `installer/consumer-smoke/run-smoke.sh`
rests its own `# no-port:` on the packer assembling out of the kit roots and
never out of the smoke's directory; `scripts/measured-claims.sh` rests its
`# no-port:` on `scripts/` riding no installer payload, citing the packer for it;
and `scripts/ci-build-artifact.sh`'s `# spec:` names the packer as a co-taker of
one derivation from its single owner — a sentence delta 2 changes the shape of.

**§The packer loses its only source-side `# spec:` citer and gains new ones.**
Nothing reds — `check-spec-pointer` grades that a pointer's target resolves and
never that a section has an incoming one — and the arm's own module carries the
same pointers the script did.

### (8) The thirteen discharged TRAJECTORY.md paragraphs are retired

Thirteen paragraphs of the ruling record carry the identical discharge oracle
*the port oracle's `--tree` trailer reads zero owed*. This cut is the event all
thirteen were written to await, so all thirteen retire in the commit that lands
it. {design-bearing}

**All thirteen, not the one ostensibly about the packer**, and the count is read
off the record rather than estimated: each names the same oracle in its own
`discharge:` line. Retiring a spent ruling is not reversing one, and each
retirement is executed against that paragraph's own discharge sentence rather
than inferred from the event.

**The witness is `--emit-ruling-staleness`, and its limit is why this delta
exists.** That arm reports every dischargeable ruling and its citing sites, and
it is an emitting arm that blocks no commit — so nothing reds if the thirteen are
left standing. A session has to run it. What survives the retirement is a
judgment this delta does not pre-empt: several of the thirteen state rules whose
*subject* outlives the port, and those keep a home on a business-as-usual surface
rather than going with the paragraph.

### (9) §Porting a gate to the binary substrate states which entry a cut hosts on

The section's hosting rule reads *the scoping stage promotes the standing
composer entry itself, carrying that cut's own `[spec:]` amendment ref*. It gains
the case it does not cover: a cut that **has its own entry** — filed when the cut
was deferred behind a design fork, and named by the composer's own text as what
holds that file — hosts there, and the composer entry is the host for a cut with
no other. {design-bearing}

The composer-entry rule exists because a cut ordinarily has nowhere else to live;
it was never a claim that a dedicated host is wrong. This cut has one, and the
ratification that made these two entries this iteration's units is a lead ruling
of 2026-09-10 — so the delta records on the governed surface what is already
ruled, rather than minting a rule. Nothing about the composer's demotion contract
changes.

## Producers and consumers

**New interfaces, each with its producer, its consumer and the transition.**

| interface | producer | consumer | transition |
| --- | --- | --- | --- |
| `--pack-installer` | `main`'s bridged-arm dispatch, before the registry lookup (delta 1) | two workflow steps and five consumer-smoke scenarios (delta 4) | every pack invocation |
| `GATE_SDK_NATIVE_ARTIFACT_NAMES` | `gate-sdk/lib/gate.sh`, at source time, from the roster and `gate_exe_suffix` (delta 2) | the arm's per-target artifact loop, through the config bridge | once per pack invocation, per roster line |
| the built binary in `pack:` | the new `build-native.sh` step (delta 3) | the assemble step in the same job | once per release publish |
| the single refusal formatter | every refusal path in the arm (delta 6) | the callers' captured stderr, and a reader of a finished CI log | on refusal only |

**The producers' enabling configuration is emitted everywhere it must be, and
that is checked rather than assumed.** The four declared knobs are all defined in
`gate-sdk/lib/gate.sh` and therefore resolvable by the bridge; the two undeclared
scratch names are read from the environment and every call site already sets or
defaults them. `GATE_SDK_NATIVE_ARTIFACT_NAMES` is computed after the roster knob
resolves in the same file, so the one caller that steers that knob — the Windows
pack leg — is served the steered value and not the default.

**Every new field has a named reader.** `GATE_SDK_NATIVE_ARTIFACT_NAMES` carries
exactly two fields per element, the target and the artifact name, and the arm
reads both at the same transition: the target selects the artifact directory, the
name selects the binary and its sidecar inside it. No third field is added — the
sidecar's name is the artifact's with a fixed suffix and needs no carrier.

**A checked negative, recorded because its absence is the finding.**
`.claude/settings.json` carries no permission grant naming `scripts/pack-installer.sh`;
the adjacent grant is on the consumer-smoke harness, which the cut does not
rename. So the composer's settings-grant ruling does not bind on this cut and no
settings edit is owed by it.

**Existing integration prose is updated in this amendment**, not left to drift —
the roster below is that update. **Not yet applied**: this stage authors and
build lands, so every passage named below is a proposal until the build stage
merges it.

### Each reader's red condition, because this amendment narrows a corpus

Delta 1 deletes a file from the port oracle's scan corpus and takes its owed
column from one to zero. Per canon-kit/SPEC.md §The causal-completeness check
point 5, a reader is clearable **by inspection** only where its verdict is
monotone in the violation set, so what follows enumerates what makes each reader
**red**. The enumeration is over the whole gate set rather than a hand-picked
subset, and it was bought as a survey rather than reasoned from the shape.

**Non-monotone in class, and the one live wire:**

- `check-measured-claim` — reds when a `measured:` marker's declared value
  disagrees with the value the consumer's emitter computes fresh. That is an
  **exact-value comparison**, the textbook non-monotone shape, and this repo's
  emitter already publishes a key read straight off the `--tree` trailer's owed
  count. **No marker in the tree cites that key today**, so the deletion reds
  nothing — but the day one does, this deletion breaks it. The build re-runs this
  grep immediately before the deleting commit rather than trusting this sentence.

**Monotone, and clearable by inspection — each with the red condition that makes
it so:**

- `check-spec-pointer` — reds when a `# spec:` target does not resolve. It is
  **forward-only**: no arm asserts that a section has an incoming pointer, so
  deleting the file deletes its ten pointers with it and dangles nothing, even
  though §The packer thereby loses its only source-side citer.
- `check-shellcheck` — reds on a warning, and refuses only when the whole target
  set yields no shell file. `scripts/` keeps its remaining members, far from that
  floor.
- `check-comment-tier` — reds on a non-directive comment. One fewer file scanned.
- `check-graph` and `check-reads-couples` — red on a `couples=` field that is
  **declared empty**, never on a glob that resolves to fewer files. No manifest
  row names this file; it is reached only by broad `scripts/*.sh` tokens that do
  not empty.
- `check-docs-cmd` — reds on a **fenced** invoked repo-relative `.sh` path that
  does not resolve. No governed doc fences this one; every mention is
  inline-backtick narrative, outside the gate's corpus.
- `check-gate-exemption-tasks` — reds on a temporary-disposition annotation
  naming a task that is not live. This file carries neither `# no-port:` nor
  `# port-until:`; it was the bare owed row and was never a subject.
- `check-todo-task-liveness` — reds on a `TODO(task:)` marker resolving to a dead
  slug. The file carries none.
- `check-gate-substrate-parity` — reds on descriptor/subcommand disparity and on
  a leftover shell implementation for a **ported gate**. The packer is not a
  registered gate and never was one's shell source; the new arm stays outside
  `--list`, which is what keeps assertion B's equality true in both directions.
- `check-gate-binary-fresh` — an **equality** between the binary's source stamp
  and git's content identity for the crate. Not narrowed by this cut, but every
  commit touching `native/` owes `bash gate-sdk/bin/build-native.sh`, and this
  amendment touches it in three deltas.
- `check-tracking-claim`, `check-unmarked-claim`, `check-manifest-count`,
  `check-manifest-temporal`, `check-install-claim`, `check-payload-claim`,
  `check-crate-arms` — checked and **not readers** of this corpus at all; each is
  named rather than omitted so a later reader can tell *checked and cleared* from
  *never looked at*.

**The one reader that is not a gate**, and it is the reason delta 8 exists:
`--emit-ruling-staleness` reports every ruling whose discharge oracle has fired
and blocks nothing, so the thirteen discharged paragraphs red no commit and are
found only by a session that runs it.

## Existing sections updated

- `installer/README.md` §The packer — the section's subject, its port
  disposition, its single-tier flag-roster rule and its reachability paragraph,
  now discharged (deltas 1, 3, 5 and 7).
- `installer/README.md` §The consumer smoke — the packed-set sentence its
  `no-port` ground rests on, and the invocation property delta 4 preserves by a
  new mechanism (deltas 4 and 7).
- `installer/README.md` §The install boundary — its disclaimer that the packer is
  outside that section's reach and its disposition is §The packer's (delta 7).
- `scripts/pack-installer.sh` — deleted; its body becomes the arm (delta 1).
- `native/src/emit/pack_installer.rs` — new: the arm, its refusal formatter and
  its per-target loop (deltas 1, 2 and 6).
- `native/src/emit/mod.rs` — the bridged-arm table gains the member and its
  four-name declared roster (deltas 1 and 2).
- `gate-sdk/lib/gate.sh` — `GATE_SDK_NATIVE_ARTIFACT_NAMES` and its derivation
  (delta 2).
- `gate-sdk/bin/run-gates.sh` — the arm's usage line in the front-end's own help
  (delta 5).
- `gate-sdk/SPEC.md` — §The non-gate arm's member roster and spawned-program
  record gain this member; §Layout and configuration gains the new knob;
  §lib/gate.sh gains the derived value beside `GATE_KIT_ROOTS_HERE`; §Porting a
  gate to the binary substrate gains the hosting case (deltas 1, 2 and 9).
- `.github/workflows/publish.yml` — the `pack:` job's new build step and its
  re-pointed assemble step (deltas 3 and 4).
- `.github/workflows/gates.yml` — the Windows pack leg's invocation and the four
  comments naming the packer as the reader of the artifact layout (delta 4).
- `installer/consumer-smoke/run-smoke.sh` — five re-pointed call sites, each in a
  cwd-pinned subshell, and the file's own `# no-port:` ground (deltas 4 and 7).
- `scripts/measured-claims.sh` — its `# no-port:` ground, which cites the packer
  for the claim that `scripts/` rides no payload (delta 7).
- `scripts/ci-build-artifact.sh` — its `# spec:` note naming the packer as a
  co-taker of the executable-suffix derivation, which delta 2 changes (deltas 2
  and 7).
- `CLAUDE.md` — §Housekeeping's sentence naming the out-of-tree assembler
  (delta 1).
- `RELEASING.md` — the assemble step's description of what runs (deltas 1 and 3).
- `docs/install.md` — the two sentences resting a prerelease-suffix claim on the
  packer's version regex (delta 1).
- `TRAJECTORY.md` — the thirteen paragraphs whose discharge oracle this cut fires
  (delta 8).
- `TASK-QUEUE.md` — this entry's terminal move, and the citations of the deleted
  path in entries outside this iteration's units (all deltas).
- `docs/gate-sdk/SPEC.md` — the generated site mirror of `gate-sdk/SPEC.md`,
  regenerated by its own command and never hand-edited (all deltas).
- `scripts/git-hooks/pre-commit` — a generated projection, regenerated by its own
  command; it persists each member's emitted argv (all deltas).
<!-- update-target-exempt: a boundary-truncated capture surface whose contract is to record a dated reading, so its mention of the deleted path is a record of what was true at that revision rather than a live citation; named here so the retired-spelling reconciliation does not read it as a missed site -->
- `.workflow/survey-record.md` — the two surveys this spec filed, which name the
  path as the corpus they were taken over.
<!-- update-target-exempt: a historical audit narrative in past tense; its mentions are records of past sweeps and of closed queue slugs, and rewriting them would falsify the record -->
- `.workflow/audit-roster.txt` — past-sweep narration naming the path.

## Retired spellings

- `scripts/pack-installer.sh` — the file and every citation of it as a runnable
  path; the arm replaces it and the roster above names each surviving surface
  (all deltas).
*Two near-spellings are deliberately kept, and it is stated here so a later
reader does not take their survival for a missed site.* The bare token
**pack-installer** stays: delta 1 keeps it as the arm's diagnostic prefix, so a
reader of a finished CI log meets one name across the cut, and four live and
closed queue entries carry slugs that share the prefix without naming the file —
renaming a closed slug would falsify the record it is the handle for. The
scratch-base knob **INSTALLER_PACK_TMP_DIR** keeps its name too, which is what
lets every call site's environment prefix survive the cut and leaves the dated
release note mentioning it true.

## Definition of Done

- [ ] **Causal completeness** — the arm, the new bridged value and the new build
      step each have a named, reachable producer and a named consumer; the new
      value's two fields each have a named reader at a named transition; every
      non-monotone reader above is inspected in the deleting commit, not assumed.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper section of `installer/README.md` and `gate-sdk/SPEC.md`, not
      appended; the merged sections read as one document.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — `check-amendment-retired-spelling` runs the
      declaration above against the whole tracked tree, and the four `no-port`
      and `spec` grounds that cite the packer are re-worded rather than left.
- [ ] **The completion predicate is witnessed** —
      `bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree` reports
      **zero owed**, and `--emit-ruling-staleness` reports the thirteen
      paragraphs delta 8 retires as discharged.
- [ ] **The binary is rebuilt** — `bash gate-sdk/bin/build-native.sh`, in every
      commit touching `native/`, beside the battery.
- [ ] **Gaps filed** — cross-component gaps discovered during the work resolved
      that session, not deferred.
