# SPEC amendment: powershell-bootstrap

**Checkwright has no native Windows install path, and the reason is that the one surface which
must run before the binary exists is written only in bash.** This amendment authors the second
hand-kept bootstrap — PowerShell — against the five steps `installer/README.md` §The install
boundary already specifies, gives it its own install-smoke leg, and makes it reachable from both
shipped transports. It is **one increment of a two-half deliverable**: the behind-invoke
relocation is the other half and is not taken here.

**What this amendment asserts, and what it deliberately does not.**

- **Asserted** — the six deltas below, each tree-verifiable and checkable at validate: a
  PowerShell bootstrap implementing the five steps, a leg that exercises all five on a native
  Windows runner, the two transports' entry points, and the record corrections.
- **Not asserted** — that a **full `checkwright init` completes through PowerShell**. It cannot,
  and the reason is structural rather than incidental: the binary carries no `init` arm
  (`native/src/install.rs`:12, `OPS = ["place-artifact"]`), so step 5's verbatim argv forwarding
  hands `init` to a program that does not implement it. That capability arrives with the
  relocation and with nothing else.
- **Not asserted** — that native Windows is a *supported* platform in the sense
  `docs/install.md` §Requirements uses. The roster carries no msvc artifact
  (`native/targets.list`), so an adopter on that host still takes `substrate-unavailable`. This
  amendment ships the bootstrap, not the platform claim.

## The two operator narrowings this amendment implements, which are not the same narrowing

Both were ruled 2026-09-07 by the operator through the AskUserQuestion channel in a lead session
and relayed by the lead, and they are recorded apart because a reader who collapses them loses
which precondition owns which half.

- **Narrowing one — of the same day's take-whole ruling, on 2026-09-03 grounds.** The unit takes
  the PowerShell half only; the relocation stays deferred. Ground: `doctrine-kit/SPEC.md`:213-216,
  the operator's own 2026-09-03 ruling on this identical precondition. This narrowing is about
  **which half ships**.
- **Narrowing two — of the 2026-08-26 ruling's oracle.** The per-bootstrap leg asserts the **five
  bootstrap steps**; **payload-end-to-end is owed with the relocation** and recorded as owed
  rather than dropped. Ground: the drift that ruling guards is between the two **bootstraps**, and
  "end to end" adds **payload** coverage rather than **parity** coverage — the two were conflated
  because both halves were assumed to land together. This narrowing is about **what the leg
  asserts**. **The shortened oracle is not the whole one**, and delta 5 says so in the section
  itself so the next reader cannot take it for the whole one.

**One alternative was put and refused rather than left open:** shipping the PowerShell bootstrap
with **no** leg at all. Refused because an unexercised hand-kept twin is precisely the drift the
2026-08-26 ruling exists to prevent — the ruling's whole mechanism is that parity is held *by
running*, not by generation, so a half with no leg has no parity mechanism whatsoever. Not an
alternative worth revisiting.

## What changes

### (1) `installer/bin/checkwright.ps1` — the second hand-kept bootstrap

A PowerShell implementation of the five bootstrap steps lands beside the bash half, authored
against §The install boundary's step list rather than transliterated from `init.sh`
{design-bearing}.

**It implements the five steps and nothing else.** §The install boundary:387-394 is the whole
specification and this delta adds no step to it:

1. **Resolve the package's own payload directory** — the PowerShell analogue of
   `installer/lib/init.sh`:8-9's `INSTALLER`/`PAYLOAD` pair, resolved from the script's own
   location so a symlinked or npm-shimmed invocation still finds its payload.
2. **Resolve the host to one Rust target triple** — the twin of `target_of_host()`
   (`init.sh`:99-111). It does **not** shell out to `uname`: on a native Windows host with no
   POSIX shell there may be no `uname`, and the standing obligation is to assume none. It reads
   the platform and architecture from PowerShell's own runtime and maps to
   `x86_64-pc-windows-msvc` — the same triple the bash half's `MINGW*|MSYS*|CYGWIN*` arm produces,
   and for the same stated reason: the map answers *which published artifact fits this host*.
   **The arm's verdict is unchanged today and this is stated rather than left to be discovered**:
   `native/targets.list` does not carry that triple, so selection still resolves
   `substrate-unavailable`.
3. **Read the payload's target roster and resolve the artifact and its sidecar, refusing a
   declared target whose pair is incomplete** — the twin of `select_artifact()`
   (`init.sh`:116-154), and it must preserve **all three** selection outcomes byte-for-byte in
   meaning: not in the roster → omit and declare; in the roster with a complete pair → verify then
   proceed; in the roster with nothing or half a pair → **refuse**. Collapsing the first and third
   is the defect §The gate binary:589-597 exists to name, and a twin that collapses them is a
   parity failure even where both halves are individually coherent.
4. **Verify the artifact's SHA-256 against that sidecar** — `Get-FileHash -Algorithm SHA256`.
   **This is the one step where the twin is simpler than the original rather than parallel to it**,
   and the asymmetry is already ruled: §The install boundary:534-536 records that on Windows the
   `digest-unverifiable` branch is **vacuous** because PowerShell carries `Get-FileHash`. So the
   PowerShell half has no hasher-resolution logic, no `sha256sum`/`shasum` fallback, and **no
   `digest-unverifiable` outcome** — where `installer/lib/common/digest.sh`:5-11 resolves between
   two hashers, the twin resolves nothing. Stated here because a reader holding the two halves
   side by side will read the missing branch as an omission, and it is a ruling.
5. **Execute the verified artifact, forwarding argv verbatim** — run in place out of the payload,
   where step 4 has just verified it, never a copy to a scratch path (§The install
   boundary:452-460). Argv forwarding is **verbatim**, which on PowerShell is a real constraint
   rather than a formality: its argument binder splits and re-quotes by default, so the delta
   must specify the invocation form that passes the caller's tokens through unaltered.

**The `--install <op>` call is byte-identical to the bash half's, and that is the point of the
seam.** §The install boundary:478-485 specifies it as an unbridged arm precisely so that a caller
which may not be a POSIX shell can make it: every value arrives as argv, the arm reads no kit
config and no knob. `init.sh`:294-297 shows the exact argv, and the PowerShell half issues the
same one — that file's own comment at `:293` already says so ("the PowerShell twin issues this
same call").

**Two host assumptions are already measured rather than hoped for, and they move here from the
queue entry.** Measured 2026-08-26 at close on a native Windows runner: `[[ -x ]]` **holds** on a
freshly `chmod +x`'d shebang script, which executes directly despite `core.filemode=false`; and it
**holds** on npm's extension-less bin shim, written mode `-rwxr-xr-x` beside its `.cmd` and `.ps1`
siblings. So neither `-x` test needs a Windows special case, and the twin inherits no
mode-detection branch. These land in §The install boundary at merge (delta 5), which is what keeps
them once the entry demotes.

### (2) The PowerShell half is reachable from both shipped transports

`installer/package.json`'s `bin` map and `files[]` gain the PowerShell entry point, and the
Release tarball's layout carries it {design-bearing}.

**Today there is exactly one entry point and it is a shell script.**
`installer/package.json`:11-12 declares `"bin": { "checkwright": "bin/checkwright.sh" }`. npm's
generated Windows shims (`checkwright.cmd`, `checkwright.ps1`) wrap that target, so on a host with
no POSIX shell the shim resolves and then fails at the interpreter — the failure mode the whole
unit exists to remove. A bootstrap nothing can invoke is a bootstrap that does not ship.

**What lands.** The `bin` map gains the PowerShell entry under a name that does not collide with
npm's own generated `.ps1` shim for the bash target — the collision is the trap here, because npm
writes `<name>.ps1` itself, so a second bin named `checkwright` cannot simply point at a `.ps1`.
The delta specifies the resolution and `files[]` gains `bin/checkwright.ps1` so the file is
actually published; a `bin` entry naming a path outside `files[]` ships a broken package.
`installer/README.md` §Layout gains the file, and §The packer's payload assembly carries it into
the tarball.

**`installer/bin/checkwright.sh` is untouched.** It is the verb dispatcher, not the bootstrap
(53 lines, resolving the symlink chain and `exec`ing `lib/<verb>.sh`), and the relocation is what
collapses it into the bootstrap. Touching it here would be taking the other half.

### (3) A per-bootstrap install-smoke leg that exercises the five steps on a native Windows host

`.github/workflows/gates.yml` gains a job that drives the **PowerShell** bootstrap, distinct from
the three existing jobs which all drive the bash half {design-bearing}.

**The three existing legs are three platforms and one bootstrap, which is the counting error §The
install boundary already warns about.** `:437-443` says to count legs *by bootstrap and never by
platform*, and records that `install-smoke`, `install-smoke-windows` and `install-smoke-macos`
"all drive the *bash* half, the Windows one through Git-for-Windows bash" — so a reader counting
platforms concludes the 2026-08-26 oracle is in place when it is not. This delta is the leg that
makes it in place for the bootstrap's own parity.

**What it asserts — the five steps, and this is narrowing two made concrete.** The leg runs
`checkwright.ps1` under `shell: pwsh` on `windows-latest` and asserts, in order: the payload
directory resolved; the host resolved to `x86_64-pc-windows-msvc`; the roster read and the
artifact and sidecar resolved; the SHA-256 verified against the sidecar; and the artifact executed
with argv forwarded verbatim, evidenced by a real `--install place-artifact` call that performs a
placement and emits its two stdout verbs.

**All five are reachable today, and the mechanism is already in the tree rather than minted
here.** `.github/workflows/gates.yml`:395-408 steers the existing Windows leg's roster to
`rustc -vV`'s host triple and packs an msvc artifact for it, under a comment that calls the
steering "measurement scaffolding and not a fix". This leg takes the same re-entry —
`GATE_SDK_NATIVE_TARGETS_FILE` — for the same reason and with the same standing: it exercises the
bootstrap against a real, locally built, digest-verified artifact **while the published payload
still commits only to what `native/targets.list` declares**. The steering is not a roster
widening and this delta does not perform one.

**What it does not assert, recorded on the job itself and not only here:** a completed
`checkwright init`. The job's own comment states that payload-end-to-end is owed with the
relocation, so a later reader extending this leg knows which assertion is missing and why, rather
than reading its absence as an oversight.

**Its `continue-on-error` posture is specified rather than inherited.** The delta states the
posture the job takes and the condition under which it changes, so the leg does not silently
acquire the `install-smoke-windows` posture by proximity — that job's non-binding status is what
let a manifest defect survive seventeen rounds, and repeating it by default would be repeating the
cost.

### (4) `installer/README.md`:442's ordering citation of a retired entry is corrected

The sentence naming `platform-support-ci-matrix` as the owner of the PowerShell leg is rewritten,
that entry having retired 2026-09-06 {mechanical}.

`:441-443` reads "the PowerShell half's leg is owed with that half itself, under
`platform-support-ci-matrix`, and no other leg substitutes for it." Two things are now wrong with
it: the named owner is retired, and the leg is no longer *owed* — delta 3 lands it. The
replacement states that the leg ships with this half, and keeps the clause that is still true and
still load-bearing: **no other leg substitutes for it**, because the three existing legs drive the
other bootstrap.

**This is the ordering-class citation only.** The three *precondition*-bearing citations of the
same retired slug — `installer/README.md`:530 and `native/targets.list`:31 and `:51-52` — are
**deliberately untouched here**: they name that entry as owner of the roster-widening precondition,
which this amendment does not discharge, and they ride
`gate-binary-roster-covers-supported-platforms` (`lead, own-authority` 2026-09-07). Editing them
here would leave a live precondition citing nothing.

### (5) §The install boundary records the narrowed oracle *as* narrowed, and absorbs the measured host facts

`installer/README.md` §The install boundary is updated so the 2026-08-26 ruling's oracle reads
correctly for a two-increment delivery, and so the bootstrap's measured host assumptions live in
the owner doc {design-bearing}.

**The narrowing must be visible at the ruling, not only in the queue.** `:433-437` states the
oracle as "a per-**bootstrap** install-smoke leg, each exercising the payload **end to end** on
the host that bootstrap is for." Left alone, delta 3's leg reads as a leg that failed to meet the
stated oracle. What lands: the oracle is stated in two parts — **bootstrap parity**, which delta
3's leg discharges for the PowerShell half, and **payload coverage**, which is **owed with the
relocation** — together with the ground the operator ruled on, that the drift the 2026-08-26
ruling guards is between the two bootstraps and that "end to end" adds payload coverage rather
than parity coverage. **The paragraph says explicitly that the shortened oracle is not the whole
one**, so a later reader cannot take the increment's oracle for the ruling's.

**The measured host facts land here, and this is where they must live.** The 2026-08-26
native-runner measurements retiring both `-x` assumptions (delta 1) are currently held only by the
queue entry, which **demotes** at this amendment's merge. Everything an entry's body carries goes
with it, so a measurement whose only home is the entry is lost at the disposition — canon-kit's
own consequence of the bidirectional rule, which no gate can catch. They land in §The install
boundary as facts about the bootstrap surface.

**§The gate binary's `digest-unverifiable` paragraph gains the twin's asymmetry.** `:606-610`
describes hasher resolution as the install path's behaviour; with a second bootstrap that has no
such branch, the paragraph gains the sentence naming which half it describes.

### (6) The two bootstraps' equality has a stated reading, so a drift is diagnosable rather than merely detectable

§The install boundary gains a short statement of **what a reader compares** when the two legs
disagree {design-bearing}.

**Parity held by running tells you *that* the halves differ, never *where*.** With one bootstrap
this cost nothing; with two hand-kept halves and two legs, a red on one leg and green on the other
is the routine case and the reader needs a stated order of comparison. What lands is the order —
the five steps, in sequence, each with the observable that distinguishes it — so a session
arriving at a divergence reads the step list rather than diffing two languages. This is the same
economy the truth table in §The consumer smoke buys, and it is stated once here rather than
rediscovered per divergence.

## Producers and consumers

**New interface: `installer/bin/checkwright.ps1`, the PowerShell bootstrap (deltas 1 and 2).**

- **Producer.** Invoked by an adopter on a native Windows host through one of the two shipped
  transports: the npm package's `bin` entry (delta 2 makes it exist) and the Release tarball's
  `bin/` (delta 2 carries it into the packer's payload). Its enabling configuration is the `bin`
  map and `files[]` — **actually emitted**, which is the delta's point: today neither ships the
  file, so a bootstrap authored without delta 2 would be the dead-producer shape, present in the
  tree and reachable from nothing.
- **Consumer.** The gate binary, through the `--install <op>` seam (§The install boundary:478-502)
  — an unbridged non-gate arm whose every value arrives as argv precisely so a non-POSIX caller
  can make the call. The consuming call is `--install place-artifact`, the argv shape being
  `installer/lib/init.sh`:294-300's, and the arm's two stdout verbs (`own`, `kept`) are read back
  by the bootstrap exactly as `:303-309` reads them.
- **Existing integration prose describing the prior flow, updated rather than left to drift.**
  §The install boundary's bootstrap-job list, its two-bootstraps ruling and its leg-counting
  paragraph all describe a world with one authored half; deltas 4, 5 and 6 rewrite each in place.
  §The gate binary's platform-resolution and digest paragraphs describe `target_of_host()` and
  hasher resolution as *the* install path's behaviour; delta 5 names which half each describes.

**New interface: the PowerShell install-smoke leg (delta 3).**

- **Producer.** A job in `.github/workflows/gates.yml`, triggered by the workflow's existing
  push-to-master and pull-request triggers. No new trigger and no new schedule is minted.
- **Consumer.** The workflow's job verdict, and a human reading the job's log. **Read job-keyed,
  never off the workflow's conclusion** — the trap is measured, not predicted: a `continue-on-error`
  job concludes `failure` while its workflow concludes `success`, which is exactly how the
  Windows manifest leg stayed unread. Delta 3 specifies this leg's posture rather than letting it
  inherit one, so the reading rule and the posture are decided together.
- **Every new field has a named reader.** The leg emits no new machine-read field: it produces a
  log and an exit status, both consumed as above. It deliberately does **not** feed the
  `installer_smoke` evidence suite — that suite's `parse-smoke-log` parser derives its row from a
  known set of arm headers (§The consumer smoke:1593-1602), and a new job emitting unrecognized
  headers would be adding a producer with no reader in that suite. Stated so a build does not wire
  it in on the assumption that every smoke leg belongs there.

**Readers whose red condition is named, not merely their subject.**

- **`check-action-run-shell`** — its red condition is a `run:` step on a Windows runner with no
  `shell:` key (`.github/workflows/gates.yml`:195-201 records that it *requires* the key rather
  than assuming bash). Delta 3's job is on `windows-latest` and every step of it must carry
  `shell: pwsh` explicitly. This is the gate most likely to red on delta 3 and it reds on the
  **absence** of a key, so it is not clearable by inspection of the added steps alone — the whole
  job must be read.
- **`check-action-pinning`**, **`check-action-permissions`**, **`check-action-gh-repo`** — each
  reds on a non-immutable `uses:` ref, a missing `permissions:` block, or a bare repo reference
  anywhere in the scanned corpus. All three are **monotone in the violation set** (a `stray`
  vector, non-empty → exit 1), so adding a job can only add violations and never mask one; each is
  clearable by inspecting the added job alone.
- **`check-workflow-tiering`** and **`check-portability-floor`** — read the workflow corpus and
  this repo's own interpreter floor. A PowerShell step is the tree's **first**, so whether either
  gate's corpus or vocabulary assumes bash is a question the build must **run** rather than
  reason about. Flagged as a live risk rather than cleared: this amendment adds a language the
  repository has never carried, and `find . -iname '*.ps1'` returns nothing today.
- **`check-shellcheck`** — reds on a shell script failing shellcheck. A `.ps1` is not a shell
  script and must not enter its corpus; whether the gate's file discovery would pick one up is,
  again, a build-time run rather than a spec-time assumption.
- **`check-install-claim`**, **`check-payload-claim`**, **`check-installer-no-deps`** — read the
  installer's claims and its dependency surface. `check-install-claim`'s red condition is a **zero
  count** on a declaration it expects to find (attested in canon-kit's causal-completeness point
  5), so it is **not** monotone and **not** clearable by inspection; delta 2 changes `files[]` and
  the `bin` map, which is exactly the kind of corpus move that flips it. The build runs it.

**No corpus is narrowed by any delta** — every delta adds a file, a job, a bin entry or a
paragraph. The one edit that removes text is delta 4, which rewrites a sentence in place rather
than pruning a file from any gate's corpus.

**No knob is minted.** Delta 3 *uses* `GATE_SDK_NATIVE_TARGETS_FILE`, which already exists and is
already in CI use, so no `<KIT>_<KNOB>` roster moves. **The provenance seam is not approached:**
the PowerShell bootstrap is generic install mechanism, carries no term list, no vocabulary and no
product constant, and this project's provenance for the ruling stays in `TASK-QUEUE.md` and
`TRAJECTORY.md` rather than entering the amendment's merged prose as a dated stamp.

## Existing sections updated

- `installer/README.md` §The install boundary — the two-bootstraps ruling and its oracle, restated
  in two parts with the narrowing marked as a narrowing (delta 5); the leg-counting paragraph at
  `:437-443`, whose "all three drive the bash half" is still true but no longer the whole roster
  (deltas 3 and 4); `:441-443`'s citation of the retired entry (delta 4); the measured `-x` host
  facts, absorbed from the demoting queue entry (delta 5); and the new statement of what a reader
  compares across the two halves (delta 6).
- `installer/README.md` §The gate binary — the platform-resolution paragraph at `:549-558` and the
  hasher paragraph at `:606-610`, each of which describes the bash half's behaviour as the install
  path's and now names which half it describes; and the selection table's three outcomes, whose
  preservation across the twin is delta 1's obligation (deltas 1 and 5).
- `installer/README.md` §Layout — the file roster, which gains `bin/checkwright.ps1` (delta 2).
- `installer/README.md` §The packer — the payload assembly, which must carry the new file into the
  tarball transport (delta 2).
- `installer/package.json` — the `bin` map and `files[]` (delta 2).
- `.github/workflows/gates.yml` — the new PowerShell install-smoke job (delta 3).
- `TASK-QUEUE.md` — `powershell-installer-surface` promotes to New Features with this file's
  `[spec:]` ref, in the same commit as this file; and at merge it **demotes** rather than reaching
  Done, the relocation half remaining (all deltas).
<!-- update-target-exempt: this amendment ships no msvc artifact and performs no roster widening; delta 3 steers a locally built artifact under an existing knob, which the roster's own header distinguishes from a join. Editing the roster here would widen it on a plan rather than on a run. -->
- `native/targets.list` — **deliberately untouched**, and its three precondition-bearing citations
  of the retired entry ride `gate-binary-roster-covers-supported-platforms`.

## Definition of Done

- [ ] **Causal completeness** — the bootstrap has a reachable producer through both transports
      (the `bin` map and `files[]` actually emitting it, not merely the file existing), a named
      consumer in the `--install` seam, and a named reader for its leg's verdict at a named,
      job-keyed transition.
- [ ] **Five steps, no sixth** — the PowerShell half implements exactly §The install boundary's
      five steps; no conditional install logic is added to either bootstrap, and the standing
      obligation to add no new shell-only install step is not breached in the bash half either.
- [ ] **Three selection outcomes preserved** — the twin distinguishes not-in-roster from
      in-roster-with-a-broken-pair, and refuses on the third; collapsing them is a parity failure
      even where each half is individually coherent.
- [ ] **The digest asymmetry is a ruling, not an omission** — the PowerShell half has no
      hasher-resolution branch and no `digest-unverifiable` outcome, and §The gate binary says why.
- [ ] **Argv is forwarded verbatim** — the invocation form is verified to pass the caller's tokens
      through unaltered, PowerShell's default binder splitting and re-quoting being a real hazard
      rather than a formality.
- [ ] **The `--install` call is byte-identical across halves** — the seam exists so both callers
      make the same call; a diff of the two argv constructions shows no difference in operands.
- [ ] **The leg asserts all five steps and says what it does not assert** — payload-end-to-end is
      recorded as owed with the relocation, on the job itself and in §The install boundary, and
      the shortened oracle is stated as not being the whole one.
- [ ] **No roster widening and no platform claim** — `native/targets.list` is unchanged,
      `docs/install.md` §Requirements still says what it says, and no surface claims native
      Windows is supported.
- [ ] **The first PowerShell surface did not break a bash-shaped gate** — `check-action-run-shell`,
      `check-workflow-tiering`, `check-portability-floor`, `check-shellcheck`,
      `check-install-claim`, `check-payload-claim` and `check-installer-no-deps` are **run**, not
      reasoned about; `check-install-claim`'s zero-count red condition is not clearable by
      inspection.
- [ ] **Battery and suite green** — `bash gate-sdk/bin/run-gates.sh` plus
      `bash gate-sdk/bin/build-native.sh`, neither discharging the other, and the installer
      fixture suite clean.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical
      section, not appended; the measured `-x` facts survive the entry's demotion.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls installer/SPEC-*.md`), discharged at the iteration rather than at the commit, a sibling
      amendment being in flight for this same component.
- [ ] **The entry demotes, it does not close** — `powershell-installer-surface` returns to the
      deferred section under `[design-pending]`, at the position it was promoted from, compressed
      to the per-entry cap, with the relocation half and the owed payload-end-to-end leg intact.
- [ ] **Removals propagated** — grepped for the retired ordering citation and for every surface
      describing the install path as having one bootstrap; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps found during the work filed to the gap inbox.
