# SPEC amendment: arm64-linux

**Nothing in this amendment is applied.** Every passage below is a proposal for the build stage to
land; where replacement wording is given it is marked **Not yet applied** at the passage itself.
`spec` authors, build lands, and a reader arriving mid-iteration must not read a quoted sentence as
one already in the tree.

**Scope, narrowed at dispatch and stated here so the narrowing is not re-litigated.** The entry
`gate-binary-platform-roster-holes` names two holes. This amendment discharges the
**`aarch64-unknown-linux-gnu`** leg alone. `aarch64-pc-windows-msvc` — the operator's specific ask,
whose cost turns on whether GitHub's ARM Windows runners have reached general availability and on a
runner decision that is the operator's rather than a session's — stays deferred on the same entry,
untouched, and this unit neither prices nor prejudges it.

## What this unit is not, and the confusion it exists to prevent

**This unit does not join a platform.** gate-sdk/SPEC.md §Consumer payload states the bound and
`native/targets.list`'s header states its mechanical predicate: a triple joins when ONE run carries
both a `native-artifacts` green for it and a platform install-smoke leg green that consumed that
upload. That header also states, in the sharpest wording anywhere in the tree, that *"removing a
blocker is not the granting of a permission"* and that the failure mode is specific and likely —
*"an iteration named unblock invites its next reader to read the removal of a blocker as the arrival
of a permission, and a roster widened on that reading is widened on a plan."*

This unit is exactly such an unblocking. It builds the producer and the consumer; the run they
produce is what joins, and no delta below writes `native/targets.list`. A build session that lands
these deltas and then adds the roster line has widened the roster on a plan.

## The causality this unit turns on, and it is the reverse of what the roster header implies

Probed at HEAD rather than inherited. `native/targets.list`'s header says a joining triple's *three
edits* include "native/runners.list **already** names its runner", which reads as though the runner
mapping is the first move and the declaration follows. The workflow says otherwise:

- `.github/workflows/gates.yml`'s `native-artifacts-roster` job derives the producer legs by parsing
  **`docs/install.md`**'s `<!-- platforms:begin -->` block — the target list *and* each target's
  joined/held state. Its own comment states why: *"The matrix is the DECLARATION, not the roster.
  Deriving it from `native/targets.list` would build only what is already joined and measure
  nothing."*
- Only once it holds a target does it call `gate_native_runner` (gate-sdk/lib/gate.sh) to look that
  target's label up in `native/runners.list`, **refusing loudly** for a target with no mapping.

Three consequences the deltas below are shaped by. A `runners.list` line alone creates nothing. The
`held:` **declaration** is what creates the producer leg, so it and its runner mapping land in one
commit or the job fails on the first push. And `continue-on-error` is derived from the declaration's
`held` bit, so a held platform's producer leg *and* its smoke leg are both non-blocking — which is
the mechanism that lets this unit land on a red-averse master without risking a sixth required
green. That last property is why the unit is safe to land before the observation it waits for.

## What changes

### (1) `native/runners.list` gains the arm64-Linux mapping, with its label probed rather than recalled

One line, in the file's existing two-column grammar {mechanical}.

    aarch64-unknown-linux-gnu ubuntu-24.04-arm

**Probed 2026-09-11 against actions/runner-images' own available-images table**, which is the
discipline that file's header already imposes on the macOS labels (*"PROBED, not remembered"*, read
2026-09-08) and the discipline this line inherits. What the table carries: `ubuntu-24.04-arm` and
`ubuntu-22.04-arm` both **Ready**, `ubuntu-26.04-arm` in **Preview**. `ubuntu-24.04-arm` is the
newest Ready label and is the one taken.

**There is no `ubuntu-latest`-shaped label for this platform**, and the line's comment says so:
`ubuntu-latest` resolves x64, so unlike the Linux x64 line this one carries a **pinned** label that
ages. That is a real recurring cost and it is the same one both macOS lines already pay; naming it
here is what stops a later reader reading the pin as an oversight. The header's re-probe instruction
covers it — *"Re-probe that table rather than trusting this paragraph if a leg ever fails to find a
runner: an unmatched label does not fail fast, it queues."*

### (2) `docs/install.md`'s platform declaration gains the first `held` line this repo has ever written

The block gains one declaration and the paragraph below it loses a sentence that this delta makes
false {design-bearing}.

**Not yet applied.** Added inside `<!-- platforms:begin -->`:

> - `aarch64-unknown-linux-gnu` (held: one `gates` run carrying both a `native-artifacts` green for
>   this triple and a green `install-smoke-linux-arm64` that consumed that upload and reached its
>   artifact-present branch) — Linux on arm64. The installer's host detector already maps this host,
>   so an adopter on it reaches a refusal rather than a wrong artifact; what is missing is the run.

**Not yet applied.** The paragraph at `docs/install.md`:120-126 currently reads *"No platform is
held today — every declared one is joined — and the `held` state stays in the grammar because it is
what lets the next platform be declared before it is published rather than appearing fully formed."*
Its first clause becomes false the moment this delta lands. Replacement:

> One platform is held today. The `held` state is in the grammar precisely so the next platform can
> be declared before it is published rather than appearing fully formed, and this is the first use
> of it: a held line is a stated precondition and an explicit *not yet*, never a support claim.

This is design-bearing rather than mechanical because the **precondition text is the licence**. It
is what a later session reads to decide whether the join may be taken, and `check-install-platforms`
arm C asserts only that a precondition is *present*, never that it is the right one. A vague
precondition passes the gate and licenses a premature join; the wording above restates the roster
header's two mechanical halves so the licence and the bound cannot drift apart.

### (3) The sixth `install-smoke` leg, named under the status quo, with its cost to an unsettled scheme recorded

`.github/workflows/gates.yml` gains `install-smoke-linux-arm64` {design-bearing}.

Its shape is the one `install-smoke-windows` and `install-smoke-macos-intel` already carry and it is
adopted rather than invented: `needs: [native-artifacts, native-artifacts-roster]`; `runs-on` and
`continue-on-error` read out of `native-artifacts-roster`'s `outputs.index` at this triple, never
hard-coded; `actions/download-artifact` **by pattern**, never by a name spelling the triple, on the
ground those legs already state — a second spelling of the triple in the workflow is a second
support commitment; the normalize step deriving the host triple from `rustc -vV`; and the suite step
writing a one-line local roster and invoking `installer/consumer-smoke/run-smoke.sh` under
`GATE_SDK_NATIVE_TARGETS_FILE`, `INSTALLER_SMOKE_TMP_DIR` and `INSTALLER_SMOKE_ARTIFACTS_DIR`. Every
`run:` step names `shell: bash` explicitly, for the reason those legs state: `runs-on` is an
expression, so no tree-local reader can infer the dialect.

**One step is an observation rather than an authored input, and build must not guess it.** The
baseline Linux leg carries **no** platform-floor step, because `ubuntu-latest` ships the floor. The
arm64 image is a different image and whether it does likewise is not derivable from this tree. The
delta therefore lands with **no** floor step, and the leg's first run is the probe; if it reds on a
missing floor member the fix is a floor step of the shape `scripts/ci-macos-floor.sh` already has,
and `native/targets.list`'s header has already priced that case (*"one on a class they do not costs
its own floor first"*). Stated so the first red reads as the measurement it is rather than as a
defect in this amendment.

**The name is minted under the status quo and that is a deliberate non-decision.**
`install-smoke-leg-names-mix-two-axes` records two live, unranked schemes — bootstrap-first
(`install-smoke-bash-<platform>`) and platform-suffix-preserving — and no name is stable under both,
so no scheme-neutral name exists to reach for. Minting under the convention already in the file
presupposes neither: whichever scheme wins, this leg is renamed with the other five and the marginal
cost is one more line in a rename that already touches five. Blocking instead would hold a
PRODUCT-class adopter-facing platform hole behind a machinery-class prose-ergonomics entry, which
inverts the discriminator both entries are classified by.

**What this delta owes that entry, recorded rather than left to be discovered.** A *second* Linux
leg makes the baseline's unsuffixed name `install-smoke` actively ambiguous — one Linux leg with no
suffix beside one with an architecture suffix, where before there was one Linux leg and the absence
of a suffix meant "the baseline". That is the strongest new argument the naming entry has and it is
this unit's own doing, so it is stated here as a promotion trigger a later scope reads.

### (4) The correcting paragraph is corrected, being already wrong before this unit touched it

`.github/workflows/gates.yml`'s comment above `install-smoke-powershell` {mechanical}.

It reads *"the three above are three platforms driving ONE bootstrap"*. There are **four** such legs
today — `install-smoke`, `install-smoke-windows`, `install-smoke-macos`, `install-smoke-macos-intel`
— the count having gone stale when the Intel leg landed after the comment was written. This unit
makes it five. **Not yet applied.** The count is replaced by the property that does not age:

> Count these legs by bootstrap and never by platform (installer/README.md §The install boundary):
> every leg above drives ONE bootstrap — the Windows one through Git-for-Windows bash — so a reader
> counting platforms has been reading a growing set of legs as covering a growing set of bootstraps,
> which they never did. The platform count is deliberately not written here: it moves whenever the
> platform declaration does, and a number written here goes stale silently, as this sentence's did.
> No other leg substitutes for this one.

De-literalization: the prose cites the property, the workflow owns the count. `installer/README.md`
§The install boundary already took this shape (*"The platform side is deliberately not given a
number here"*) and this delta brings the workflow comment into line with its own cited owner rather
than inventing a rule.

## Producers and consumers

**No new state, no new event, no new file, and no new field.** Every artifact this unit touches
already exists with a named reader; what changes is one more row in each. Surveyed across the whole
component set — the workflow, both roster files, the install page, the gate, the bootstrap — with no
stderr suppressed on any path grep.

- **`native/runners.list`'s new line.** Producer: hand-authored, delta 1. Consumers, both existing:
  `native-artifacts-roster` in `.github/workflows/gates.yml` (at CI dispatch, to resolve `runs-on`)
  and `.github/workflows/publish.yml`'s roster job (at release, to derive the build matrix), both
  through `gate_native_runner` in gate-sdk/lib/gate.sh. **Enabling config: none** — the file is read
  through `gate_native_runners_file`, whose layout default is this path.
- **`docs/install.md`'s new declaration.** Producer: hand-authored, delta 2. Consumers, all three
  named with their transition: `native-artifacts-roster`'s awk parse (at CI dispatch, to derive both
  the producer matrix and each leg's `held` bit); `check-install-platforms` arms B, C and D (at every
  battery run, to assert lockstep with the roster and to print the held platform's omitted-member
  count); and the widened arm this iteration's sibling unit adds (SPEC-host-detect.md), which is why
  the two units share a build-batch constraint recorded below.
- **The new leg.** Producer: the `gates` workflow on a push to master. Consumer: a human reading the
  run, and the `held → joined` decision that reading licenses. This is the unit's whole point and it
  is the reason the entry carries `[observed-by: gates]`.
- **The field with a named reader.** The one new *field-shaped* thing is delta 2's `held:`
  precondition string. Its reader is `check-install-platforms` arm C at battery time (presence and
  non-emptiness only) and a scope-stage session at the join decision (the value). Both named; the
  gate's limit — that it reads presence and never correctness — is stated on delta 2 rather than
  left for a later session to discover by trusting a green.

**One producer/consumer edge crosses inside this iteration's batch A and the lead sequences it.**
SPEC-host-detect.md's widened `check-install-platforms` binds the installer's host detector to this
declaration block. The detector already emits `aarch64-unknown-linux-gnu` and the block does not
declare it, so **that gate reds the moment it is widened, and delta 2 is what clears it**. The two
units land in one commit, or this one lands first. Landing the widened gate alone reds the battery.

**No edge crosses to batch B.** That batch touches `lifecycle-kit` and `queue-kit`; this one touches
the workflow, the rosters, the install page and the consumer gate. Nothing either emits is the
other's input. The one thing this unit *consumes* from batch B is a tag spelling, not a value.

## Existing sections updated

- `docs/install.md` §Requirements, the `<!-- platforms:begin -->` block — one declaration added
  (delta 2).
- `docs/install.md` §Requirements, the paragraph below that block — the no-platform-is-held sentence
  replaced (delta 2).
- `.github/workflows/gates.yml`, the comment above `install-smoke-powershell` — the stale platform
  count replaced by the property (delta 4).
- `native/runners.list`'s header — the probe date and the pinned-label cost recorded beside the new
  line, on the same terms the macOS labels already carry (delta 1).
- `native/targets.list`'s header, the three-join-edits sentence — one clause **added**, not
  corrected (delta 1). The sentence is not false: *"native/runners.list already names its runner"*
  is a true statement about the state at join time. What it does not say is what lands **first**,
  and a reader who supplies the natural order gets it backwards, since the declaration is what
  creates the producer leg and the mapping is only looked up once a declared target exists. The
  added clause states that ordering. **Not yet applied**; the header's 2026-09-07 operator ruling
  and its attribution are untouched and no delta of this amendment reopens them.
<!-- update-target-exempt: no delta writes this file — it is named to record deliberately that the roster line is withheld until the observation, which is this amendment's central non-target -->
- `native/targets.list`'s live roster lines — **deliberately unwritten by every delta**.

## Retired spellings

- None — no delta of this amendment retires a spelling. Delta 4 replaces a stale *count* inside a
  sentence rather than a name, and delta 2 replaces a clause; neither leaves a token for a sweep to
  chase. `install-smoke`, the baseline leg's name, is explicitly **not** retired here: delta 3
  records that it has become ambiguous and leaves its disposition to
  `install-smoke-leg-names-mix-two-axes`, which owns the scheme.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and
      a named consumer; every new field has a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical
      surface (not appended); the merged surfaces read as one coherent document a reader who never
      saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root
      (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired
      spellings` above, and `check-amendment-retired-spelling` runs each declaration against the
      whole tracked tree.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
- [ ] **The roster line is NOT written** — `native/targets.list` is unchanged by this unit, and the
      build session confirms it rather than assuming it. The join is a later act on an observation.
- [ ] **The freshness fan-out is paid** — a tracked file added or changed under the crate root reds
      `check-gate-binary-fresh`, so `bash gate-sdk/bin/build-native.sh` runs in the same commit;
      neither it nor the battery discharges the other.
