# SPEC amendment: macos-leg

**A macOS install-smoke leg joins `.github/workflows/gates.yml` as a third, non-blocking
probe-then-suite job, and the one live GNU-only defect on the adopter install path is fixed
inside it.** The leg and the fix are one unit because the fix is what the leg would otherwise
spend its first run discovering, and because the fix's own entry says the defect is live whether
or not a macOS leg is ever bought — so neither half waits on the other.

**The amendment's own evidence base is a filed survey, not a fresh reading.** The install-path
sweep and the leg-shape reading behind every measured claim here are recorded on
`.workflow/survey-record.md` at this iteration's `spec` rev, with the witness a later stage runs
to re-license citing them. Nothing below re-derives what that record already holds.

**Two things this amendment deliberately does not buy, stated at the top because both are the
natural misreading:** it does not widen the toolchain floor, and it does not widen
`native/targets.list`. Deltas 5 and 6 own those and each states its own trigger.

## What changes

### (1) The leg is a third job in the filled workflow instance, non-blocking, probe-then-suite

`install-smoke-macos` joins `gates.yml` beside `install-smoke` and `install-smoke-windows`
{design-bearing}. It copies the **Windows** leg's posture and not the Linux leg's, and the two
are genuinely different postures rather than two spellings of one:

- The Linux leg is **binding** — its verdict decides the workflow, driven through
  `--diff-baseline installer_smoke`, because a green baseline exists to diff against.
- The Windows leg is an **instrument** — `continue-on-error: true`, four steps that report and
  never fail, then one suite step that propagates its real exit status into a job the workflow
  does not read.

Nothing has ever run green against macOS, so there is no baseline to diff and a binding leg
would red `master` on the push that introduces it. The instrument posture is therefore the only
one available on the first run, and it is also the one the standing file-and-defer ruling on the
sibling entry assumes: a red cause is filed and not looped on, which a blocking leg makes
impossible to honour.

**The promotion to binding is a later act with a stated trigger, and the trigger is copied
verbatim from the sibling** — the `continue-on-error` line carries the same instruction the
Windows leg's does: drop it on the run it is first observed green and not before. Stated here so
the leg arrives with its own exit condition rather than acquiring one later.

**The runner label floats, and the honest-label boundary is stated with it.** `macos-latest`,
matching `windows-latest` and `ubuntu-latest` beside it: the leg's subject is the platform an
adopter meets *today*, and a pinned label would freeze the instrument against a host adopters
have already left. The consequence is that `macos-latest` is **arm64**, so this leg measures
arm64 and nothing else. An Intel leg is a separate job this one neither buys nor blocks, and the
amendment says so rather than letting one green arm64 run read as a macOS claim — the same
honest-label discipline the unit's own roadmap summary asks for.

**Timeout.** The Windows leg's ninety minutes is bought by MSYS emulating `fork` on every one of
the suite's thousands of spawns, and its own comment says the Linux figure is not the planning
basis there. On macOS that penalty does not exist — spawns are native — so the Linux leg's
observed runtime *is* the right basis, widened for a cold Homebrew install and a cold cargo
build. Forty-five minutes, revisable on the first observed run, which is the only measurement
that can settle it.

### (2) The leg installs the GNU userland the install page already documents, and the run is the point

The recorded runner-image measurement is that the macOS image ships bash 3.2.57 with no
coreutils and no gawk, while cargo, rustc, jq, node and Homebrew are present {design-bearing}.
So the leg `brew install`s bash, coreutils and gawk and PATH-orders them ahead of `/usr/bin`.

**That step is not scaffolding — it is the assertion.** It is precisely the adopter action
`docs/install.md` §Requirements documents, and the whole value of this leg is that the
documented remedy stops being a reading and becomes a run. A leg that reached green by
installing something the page does not name would be measuring a host no adopter has.

**Which is why the leg must not `brew install findutils`, and this is the delta's sharp edge.**
Delta 3 removes the install path's only findutils dependency, so after it lands the documented
list is complete. If the leg installed findutils anyway it would go green over an adopter path
that stayed broken for everyone who installed exactly what the page names — which is the
near-miss the defect's own entry identifies, met from the CI side instead of the reading side.
The leg's package set and §Requirements' list are therefore held equal by construction, and a
future divergence between them is a defect in whichever moved.

### (3) The vendoring enumeration becomes portable, and an empty enumeration becomes a refusal

`installer/lib/init.sh:201` runs `find . -type f -printf '%P\n'` once per kit in the
unconditional vendoring loop, with stderr **not** suppressed {design-bearing}. `-printf` is a
GNU findutils primary; where `find` refuses it the `while`-read loop receives nothing and the
kit vendors zero files. Two changes land, and the second is the one that outlives the first:

- **The construct becomes portable.** The loop already `cd`s into the kit's payload directory
  and already sorts, so the enumeration needs no GNU primary to produce the same
  payload-relative names in the same order.
- **A kit that enumerates zero files becomes a refusal.** The defect's real shape is not the
  primary — it is that a *failed* enumeration is indistinguishable from an *empty* kit, so the
  install proceeds on a false reading either way. Naming the empty case and refusing it makes
  the fix survive the next construct that fails for a different reason, on a host nobody has
  tested yet.

**The refusal has exact in-tree precedent and takes its shape from it rather than inventing
one.** `scripts/pack-installer.sh` already refuses a payload whose kit enumeration came back
empty, and gate-sdk/SPEC.md §Consumer payload rules the same shape one layer out — a roster
target no build leg produced *fails the release*, "which is the correct place for that failure".
A kit the profile named and the payload carries that vendors nothing is that failure arriving at
install time.

**The entry's own account of the failure mode is corrected on promotion, and the correction
matters to what the leg asserts** {design-bearing}. The entry costs this defect as "a green
`init` over an empty vendor tree, discovered later as missing kits rather than at install time".
Read off the control flow, that is not what happens: `gate-sdk` is itself one of the vendored
kits, so an empty loop leaves no hook generator under the install root, and `init.sh:301-302`
reaches `die "gate-sdk's hook generator failed"` **before** the `git add` and commit block. The
real failure is a loud partial die over a half-written tree — worse in one way the entry did not
claim (the tree is left dirty and uncommitted) and better in another (it is not silent). This
reading is control-flow, taken on a machine with no Mac, and the leg's first run is the oracle
that settles it; the entry is corrected to say that rather than to assert the new shape as
measured.

### (4) The smoke is steered at a single-host roster, which is a precondition and not a preference

The macOS leg must set `GATE_SDK_NATIVE_TARGETS_FILE` to a single-host roster before running the
suite, exactly as the Windows leg does {mechanical}. This is not stylistic parity:
`native/targets.list` carries one line, `x86_64-unknown-linux-gnu`, and gate-sdk/SPEC.md
§Consumer payload rules that a consumer smoke building its artifact from the host it runs on
cannot satisfy a roster naming a platform that host is not — so an unsteered smoke on a macOS
host fails on the roster before it reaches anything this leg is measuring. Recorded as a delta
rather than left to the build because it is the one step whose omission produces a failure that
looks like the platform finding the leg exists to collect.

### (5) The toolchain floor is not widened, and the ground is that the path no longer needs it

`context-kit/lib/toolfloor.sh`'s `PROBE_SET` gains no findutils member and `docs/install.md`
§Requirements gains no findutils bullet {design-bearing}. The defect's entry left this open as
the second of two candidate fixes differing in kind — name GNU findutils on the floor, or
replace the construct — and asked whether the floor should widen anyway "since this loop is not
the only GNU-ism the install path may carry".

**The survey answers the premise the open call rested on.** The install path's other GNU-only
constructs are exactly two — `sort -V` and `realpath --relative-to` — and `docs/install.md`
already names both, the second by the construct that binds it. Every other candidate the sweep
looked for has zero live shell invocations anywhere in the tree. So after delta 3 the path
carries no findutils extension at all, and a floor member would oblige every adopter to install
a package nothing on their install path calls, and every `doctor` run to probe for it.

**The floor names what the path needs.** Widening it past that is not a cheap safety margin: it
is a user-facing requirement bought with nothing behind it, and it would put the floor and the
install page in the position of documenting a dependency the code does not have — which is the
same drift class from the opposite side.

### (6) The leg does not widen the target roster, and this non-target is stated because the SPEC predicts the misreading

Landing this leg licenses **no** line in `native/targets.list` {design-bearing}. gate-sdk/SPEC.md
§Consumer payload rules the join bound directly — a target joins only when a green run has
**produced and exercised** its artifact, not when a platform is reasoned about and not when a
provider offers a runner — and it goes on to rule that removing a blocker is not the granting of
a permission, naming the exact failure: an iteration that makes a target *possible* invites its
next reader to read the removal of a blocker as the arrival of a permission.

**This leg is that iteration, so the clause is quoted into the leg's own comment rather than
left in the SPEC to be found.** And the discriminator is sharper here than the general rule
makes it look: the smoke builds its artifact **from the host it runs on**, which §Consumer
payload already classifies as "a harness stand-in, not this rule relaxing". A green macOS smoke
therefore proves the adopter path works on macOS and proves nothing about a released macOS
artifact, because no released artifact was involved. The roster's own file states the same bound
from its side.

### (7) The surfaces that named this leg as owed are swept

Four surfaces describe a world in which no macOS leg exists {mechanical}: the workflow's own
comment naming the macOS leg as still owed and belonging to this unit; `TRAJECTORY.md`'s
sentence about the per-platform install-smoke legs holding the two bootstraps in parity;
`installer/README.md` §The install boundary's per-platform-leg prose; and the two queue entries,
whose `[roadmap:]` tags reach `ROADMAP.md` through its own freshness gate with no hand edit.

**`docs/install.md` gains no macOS support claim** — that is delta 6's bound applied to the
documentation surface, and its trigger is the same first observed green.

## Producers and consumers

**The leg (a new workflow job).** *Producer* — GitHub Actions, on the `push`/`pull_request` to
`master` triggers the file already declares at workflow level; there is no per-job trigger in
Actions, so the leg's enabling configuration is the one this workflow already carries and needs
no new arming. *Consumers* — three, and only the third is a decision: a session reading the run
log for the platform findings; `--diff-baseline`'s recorded baseline, which this leg does
**not** feed while it is non-blocking; and the workflow's own conclusion, which
`continue-on-error: true` deliberately keeps this job out of until the trigger in delta 1 fires.

**The leg as a gate subject.** Two gates take the new job as a new subject the moment it exists,
and both are named here because a new job is exactly the shape that introduces a violation:
`check-workflow-tiering` and `check-action-gh-repo`. The job checks out, so it is armed, and the
enabling configuration it must carry is a `permissions:` block with `contents:` in scope — the
siblings each carry `contents: read` at job level, and a job-level block *replaces* the
workflow-level one rather than adding to it, so the leg carries its own. Every `uses:` ref is
pinned to a 40-hex commit SHA, which the checkout action in all three sibling jobs already is.

**The portable enumeration and its refusal.** *Producer* — `installer/lib/init.sh`'s
unconditional per-kit vendoring loop, which runs on every non-dry `checkwright init` on every
profile, so the producer is reachable on the only path an adopter takes and not merely in a
test. *Consumer* — `copy_in`, which receives one payload-relative name per line and is the sole
reader of the enumeration; and, for the refusal, the adopter, who receives a named cause instead
of a half-written tree. The refusal's transition is the end of one kit's loop, before the next
kit begins, so a broken host fails on the first kit rather than after writing several.

**No new field, no new message, no new state file, no new knob.** `GATE_SDK_NATIVE_TARGETS_FILE`
is read at its existing contract and gains no meaning here.

**This delta set narrows one corpus — the prose in four surfaces that name the leg as owed
(delta 7) — and it *adds* two subjects (a workflow job, a refusal path). So the
causal-completeness check's point 5 binds, and each reader's RED condition is enumerated rather
than its subject, in both directions.**

- **`check-workflow-tiering`** reds on **finding** an armed job with no `contents:` scope in
  scope. Adding a job can only introduce subjects, so this reader is **not** monotone under this
  change and is run, not inspected.
- **`check-action-gh-repo`** reds on **finding** a `uses:` ref that is not an immutable 40-hex
  SHA. Same direction, same conclusion: run it.
- **`check-install-claim`** holds the primary-install-path claim and its red condition is a
  **zero count** — the attested case in canon-kit/SPEC.md's own point 5, where pruning the file
  holding a declaration's sole instance flipped it green to red. Delta 7 edits install-adjacent
  prose and delta 5 declines to edit `docs/install.md`, so the count could move in either
  direction; it is run.
- **`check-comment-tier`** reds on a comment that is not a directive. Deltas 1, 2 and 6 each
  place load-bearing comments in `gates.yml` — the drop-on-first-green instruction, the
  package-set equality, the roster non-permission — and each must earn its place as a directive
  rather than as narration. Run, and the deltas are written so the comments are instructions.
- **`check-measured-claim`** reds when a `measured:` marker disagrees with its oracle and
  **fails closed on an unknown key**. Delta 3's correction of the entry's failure-mode account
  must not delete a marker's bound claim out from under it.
- **`check-md-refs`** reds on a reference resolving to nothing. Delta 7 rewrites four
  cross-referencing sentences, so a surviving `§` citation can be re-pointed at a section that
  does not exist; monotone under pure deletion, not under rewrite, so build runs it.
- **`check-roadmap-fresh`** byte-gates `ROADMAP.md` against the queue's curated `[roadmap:]`
  tags. It is monotone in nothing and clears only by regenerating; the queue-entry disposition
  in delta 7 is its trigger.
- **`check-core-files`** governs `.github/workflows/gates.yml` as pinned repo-meta, so the
  workflow is edited as a governed doc rather than as CI scaffolding.

## Existing sections updated

- **`.github/workflows/gates.yml`** — the new job, and the shared job-group comment above
  `install-smoke` that names the macOS leg as still owed and assigns it to this unit
  (deltas 1, 2, 4, 6, 7).
- **`installer/lib/init.sh`, the unconditional per-kit vendoring loop** — the portable
  enumeration and the empty-enumeration refusal (delta 3). Its port disposition is unchanged: the
  file stays `owed` behind the install boundary's relocation, and a defect fix is not a cut.
- **`installer/README.md` §The install boundary** — its per-platform-leg prose, which describes
  the parity mechanism in terms of the legs that exist (delta 7).
- **`context-kit/lib/toolfloor.sh` and `docs/install.md` §Requirements** — read, and
  deliberately **not** changed; delta 5 records why, so the next reader does not take the silence
  for an oversight. `docs/install.md` is read a second time under delta 6's bound — it gains no
  macOS support claim, on the same first-observed-green trigger as the roster non-widening — so
  the file's own non-edit is recorded under both deltas rather than delta 5 alone (deltas 5, 6).
- **`native/targets.list`** — read, and deliberately **not** changed; delta 6 records the bound
  and its trigger (delta 6).
- **`TRAJECTORY.md`**, the sentence stating that the two hand-kept bootstraps are held in parity
  by the per-platform install-smoke legs (delta 7).
- **`TASK-QUEUE.md`** — `macos-install-smoke-ci-leg` and `init-vendoring-assumes-gnu-findutils`,
  the second carrying delta 3's correction of its failure-mode premise (deltas 3, 7).
- <!-- update-target-exempt: a generated root projection with its own freshness gate and regen command, rostered in docs/site-architecture.md §Generated projections and their freshness gates; its content is derived from the queue's [roadmap:] tags, which delta 7 edits, so no delta owns the projection itself --> `ROADMAP.md`.

## Definition of Done

- [ ] **Causal completeness** — the new job names its producer (the workflow's existing
      triggers) and its consumers, carries the `permissions:` block its arming demands, and the
      enumeration's refusal names the reader that receives it; the non-monotone readers above are
      run rather than inspected.
- [ ] **Merged with no information lost** — the Windows leg's instrument reasoning is cited, not
      re-derived; the roster non-permission and the floor non-widening each land with their
      trigger stated, so neither reads as an oversight.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the repo root
      (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every surface naming the macOS leg as owed re-checked;
      `ROADMAP.md` regenerated; `check-md-refs` and `check-install-claim` green.
- [ ] **The leg's own exit condition is written into it** — the `continue-on-error` line carries
      the drop-on-first-observed-green instruction, so the instrument's promotion to a gate is
      recorded where the next reader of the job meets it.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed to the gap inbox;
      a red cause from the leg's first run is filed and not looped on.
