# SPEC amendment: intel-roster-join

**Nothing in this amendment is applied.** Every passage below is a proposal for the build stage to
land; where replacement wording is given it is marked **Not yet applied** at the passage itself.
`spec` authors, build lands, and a reader arriving here mid-iteration must not read a quoted
sentence as one already in the tree.

`x86_64-apple-darwin`'s join predicate has fired. The predecessor amendment built the Intel
`install-smoke` leg and said in terms what it could not do — *it cannot join its own target; its
first green is that run's second half*. That run exists. This amendment writes the third roster line
and repairs the surfaces that argue from a hold the line ends.

**It is deliberately small on the mechanism and not small on the prose**, and that asymmetry is the
unit rather than an accident of it. The producer leg, the consumer leg, the runner mapping, the
release build body and its macOS floor were all bought by the two iterations before this one, so the
executable half is two file edits and a rebuild. What is left is that roughly a dozen sentences
across five surfaces are *arguments from a held remainder*, and the remainder goes to zero here.

## The licence, verified at this stage rather than relayed

`gates` run `34245261556`, job `102126302115`, read job-keyed off a finished run for free — the
observation costs no push (CLAUDE.md §This repo is governed by its own kits). Both limbs on one run,
at a commit later than the paragraph stating the predicate:

1. `native-artifacts (x86_64-apple-darwin, macos-15-intel)` — **success**, binary and `.sha256`
   sidecar produced by the release's own build command and uploaded.
2. `install-smoke-macos-intel` — **success**, having consumed that upload under
   `INSTALLER_SMOKE_ARTIFACTS_DIR`, reached the artifact-present branch, emitted
   `INSTALLER-SMOKE: clean` and exited 0.

**The predicate itself is not restated and not reopened.** `native/targets.list`'s header owns it,
this amendment quotes no part of it, and the 2026-09-07 operator ruling behind it — that one green
macOS install-smoke leg licenses nothing on its own — is what this join *satisfies* rather than
sidesteps. The scope stage recorded the same finding as a survey block; the witness for citing it
rather than re-buying it was run at this stage: `git diff --stat` over that block's corpus since its
recorded rev is empty, and its oracle is the finished run above.

## What this join is not

**Native Windows is out of scope, and the ground is a product commitment rather than a cost.**
`docs/install.md` §Requirements routes Windows through WSL and carries `x86_64-pc-windows-msvc` in
neither join state, so §Consumer payload's first bound — a roster line may not exceed what the
install page states — refuses the line before any run is considered. Composing it would be declaring
a platform, which is operator-class. This amendment does not compose it, and delta 2 keeps that
triple as the header's worked example of the distinction.

## What changes

### (1) `x86_64-apple-darwin` joins, on the surfaces that move together

The join `native/targets.list`'s header specifies, executed and not re-derived {mechanical}:

- `native/targets.list` gains the triple's live line;
- `docs/install.md` §Requirements' `platforms:begin` block flips that triple from
  `(held: …)` to `(joined)`, and its continuation prose is **rewritten rather than left beside the
  new state word**. The bullet currently argues the hold — "`macos-latest` is arm64, so the macOS
  smoke leg measures Apple silicon and asserts nothing here; an Intel leg is a separate green" —
  and that separate green is what the line now rests on. The rewrite names what the line rests on,
  in the shape the `aarch64-apple-darwin` bullet beside it already uses;
- `native/runners.list` needs **no mapping edit**: it already maps the triple to `macos-15-intel`
  (verified at HEAD). The header's three-edits-together clause therefore resolves to two edits
  here, and that is the clause being satisfied rather than bent. That file's *header* does take an
  edit, for an unrelated reason delta 3 owns.

`check-install-platforms` holds the roster and the block in lockstep in both directions, so the two
edits land in one commit or that commit reds.

**The obligation the roster edit incurs, which is not discharged by the battery.**
`check-gate-binary-fresh` stamps the binary against git's content identity for every tracked file
under the crate root, and `native/targets.list` is one of them. So the roster line reds that gate
with no Rust touched, and the same commit owes `bash gate-sdk/bin/build-native.sh`
(CLAUDE.md §Housekeeping: the battery and the build discharge each other in neither direction).

### (2) `native/targets.list`'s header stops asserting a falsehood and stops arguing from a remainder it no longer has

Three passages in that header fail, and they fail for two different reasons — one is already false
at HEAD, two are falsified by delta 1 {design-bearing}:

- **Already false, independently of this amendment.** The "standing case" bullet says
  `x86_64-apple-darwin` "is still not a line here, because no single run has yet carried both
  halves". Run `34245261556` carried both. **This repair belongs to this unit rather than
  filing as a side finding**, because the sentence's whole function is to say why the line is
  absent, and delta 1 is what makes the line present: repairing it separately would mean landing a
  roster line under a header still explaining its absence.
  Its replacement subject is the triple the header already discusses at length —
  `x86_64-pc-windows-msvc`, which the declaration carries in neither state — so the bullet keeps a
  *worked* instance of "evidence rather than capability" instead of losing its example with its
  subject.
- **The held-remainder paragraph.** It grounds the omit-and-declare path's liveness on
  "the HELD REMAINDER and not the roster's size: Intel macOS is still held, native Windows is not
  declared at all, and every host outside … §Requirements' declaration block still meets that path
  on day one." Delta 1 takes the **first** of those three clauses to zero — after it, the
  declaration block carries **no held platform at all**. The other two clauses survive intact and
  are sufficient, so the paragraph is re-grounded on the *undeclared complement* rather than on any
  hold. This is the delta's judgment call and it is stated as one: the honest reading is that
  omit-and-declare is now exercised only by hosts the page never declared, which is a narrower
  ground than the paragraph has ever rested on and is still a real population.
- **The second-line cost accounting.** The header prices "what a second line costs, now that one has
  landed and the figure is paid rather than estimated", and predicts that "a further line on a
  platform class those two already bootstrap costs a runner mapping again". Delta 1 is that further
  line and the prediction is **measured**: the runner mapping was already tracked, the release
  matrix is roster-derived, `scripts/ci-macos-floor.sh` resolves its paths through `brew --prefix`
  rather than hard-coding a prefix, and the publish leg guards that step on `runner.os == 'macOS'`.
  So this line cost **zero new surface on the release path**. The accounting is updated to record a
  paid figure rather than a predicted one, which is the same move the aarch64 line's accounting made.

**What is untouched, said explicitly because a header edit invites the wider read.** The join
predicate, the refusal of cross-compilation, the runner-availability paragraph, and the ruling
stamp at the header's middle are not this delta's subject and take no edit.

### (3) `native/runners.list`'s header loses its worked example, in the same shape delta 2 handles

That file's separateness from the roster is argued from an instance {design-bearing}: "a mapping may
name a platform the roster deliberately may not. **Both of the darwin lines below are exactly that
case — neither triple is on the roster**, and both must be buildable anyway, because the whole point
of the `native-artifacts` job is to measure a platform BEFORE it joins."

**Half of that sentence is already false at HEAD** — `aarch64-apple-darwin` joined last iteration
and its mapping line stayed — and delta 1 falsifies the other half. After it, **every** mapped
triple is a roster line, so the file holds no live instance of the property its own header rests on.

The property is not weakened by that and must not be restated as one that is. The repair grounds it
on the **permission** the separateness preserves rather than on a member that happens to hold it:
the map is what lets the *next* probed platform be built and measured before it may be declared, and
both darwin lines are the attested path through that sequence rather than a standing exception to
the roster. The probed-labels paragraph below it is untouched.

### (4) The declaration block reaches zero held, and one prose surface overclaims what reads `held`

The state word is an input to more than documentation, and delta 1 empties one of its two values
{design-bearing}. Two readers meet that, and **only one of them takes an edit**:

- **`native-artifacts-roster`'s per-target index**, whose `held` field feeds `continue-on-error` on
  both the Intel producer leg and `install-smoke-macos-intel`. Both flip to **binding** at the
  landing commit, with no workflow edit — the derived-posture property working as designed, and
  this join's whole real cost. §What this join makes binding states it as a risk rather than
  leaving it to be discovered.
- **`check-install-platforms`' arm D**, which reports the omitted-member count **per held
  platform** and therefore goes quiet. **The gate itself takes no edit**, and that was reviewed
  independently at this stage rather than assumed: the empty-set case is designed and correctly
  labelled — `omitted_report` returns the empty string, the red-line block is guarded on a non-empty
  held set, and the clean line prints `0 held with a stated precondition` rather than reading
  vacuously true. A gate whose report has no subject because nothing is held is reporting
  correctly.

**What does take an edit is the prose that calls that arm standing.**
`docs/site-architecture.md`'s install-platforms row calls arm D "the standing instrument
gate-sdk/SPEC.md §The port-candidate criteria's aggregate cost names". It is not standing — it
reports only while something is held, and this join silences it — and it is not what that criterion
names, whose observer is the **binary-less leg**. The row gains both limits. Its state-word
paragraph also gains its first worked instance, delta 1 being the first flip to make a consumer leg
binding.

**`gate-sdk/SPEC.md` takes no edit here, and that was checked rather than assumed — it is the
surface a reader will expect on this list.** §The port-candidate criteria states the uncovered set
as "`native/targets.list`'s complement", de-literalized on exactly the ground that "a count here has
gone stale on a roster join once already and would again on the next one". This is that next one,
and the de-literalization holds.

### (5) The Intel leg's prose stops arguing from a hold, without losing the argument that survives

`.github/workflows/gates.yml`'s `install-smoke-macos-intel` job carries three passages written from
inside the hold {design-bearing}:

- its header's closing paragraph, *"What this leg cannot do, stated so no reader expects it … Its
  first green is that run's second half"* — the leg did it, and the sentence now describes a
  discharged predicate rather than a standing limit;
- the suite step's comment, *"this leg is held, so a non-zero does not fail the workflow. Read a red
  here as the reason `x86_64-apple-darwin` is still absent from native/targets.list"* — both halves
  false after delta 1;
- the verdict step's echoed text, whose "while `$host` is held" clause and "A zero here is criterion
  2 only" line describe a posture the leg no longer has.

**The derived-posture argument is what must survive the edit, and it is the reason this delta is not
a find-and-replace.** The header's load-bearing claim is that `runs-on` and `continue-on-error` are
read out of the declaration so that *nobody has to remember* to make the leg binding. Delta 1 is the
first exercise of that mechanism, so the header should record it as demonstrated rather than delete
the argument along with the hold that motivated it. The sibling `install-smoke-macos` leg's binding
verdict wording is the shape to converge on; the deliberate divergence the comment records was a
divergence *from a held posture*, and its ground goes with the hold.

## What this join makes binding

Stated as a risk this amendment accepts rather than as a change to notice.

At the landing commit `.github/workflows/gates.yml` carries **no non-binding platform leg except the
Windows install-smoke leg**: all three `native-artifacts` producer legs and all three platform
`install-smoke` legs become binding on master. The legs are green today — that is what the licence
above establishes — so nothing reds at the landing, and the cost arrives the first time an Intel
macOS build or install path breaks.

**This has an ordering consequence for the iteration's other half, and the lead cuts the batches.**
`windows-smoke-manifest-cr-survives-repair` edits `installer/consumer-smoke/run-smoke.sh`, which is
the script all three platform install-smoke legs run. Before this join, a regression that script
introduced on macOS would have reddened one binding leg (`install-smoke-macos`); after it, two. The
join does not block that unit and that unit does not block the join — the queue entry's
`NOT STAGE-BLOCKING` finding is unchanged — but a batch landing both wants the join's binding
posture in view when it touches the shared script.

## What this join does to a neighbouring deferred entry

`held-ci-leg-failure-reddens-a-binding-one` rests on a structural ground — "every future held
platform inherits it, so widening the roster widens the coupling" — and records that its observed
trigger had already cleared. Delta 1 takes its **live instance count to zero**: with no held
platform left, there is no held producer whose failure can defeat held-ness through a binding
consumer, and a binding producer failing a binding consumer is that pair working correctly.

**The entry's ground is untouched and this amendment re-scopes nothing.** Its ground is about a
*future* hold, which this join does not retire — the next declared-and-held platform inherits the
coupling exactly as the entry says. The finding is recorded here so the entry's next reader knows
which half moved, and a change to the entry itself is a queue decision this stage does not take.

## The one edit this amendment does not authorize

`TRAJECTORY.md`'s 2026-08-14 born-native-default ruling records the cost it was weighed against, and
that recorded figure currently reads: *with `aarch64-apple-darwin` joined the set is Intel Macs plus
the hosts the platform declaration does not carry at all.* **Delta 1 falsifies the "Intel Macs"
half**, leaving only the undeclared hosts.

**The prior correction's authorization was bounded and is spent, and where each half of that fact
lives matters.** The figure was corrected once already. `TRAJECTORY.md` itself records only the
*extent* — "Only that figure is corrected — the verdict, its grounds and its date stand". That the
correction was **operator-authorized, and authorized for that one figure and explicitly for nothing
else**, is recorded in the landing commit's message and on no governed surface, which is why this
amendment had to read a commit to learn it (captured as knowledge friction at this stage).

A bounded authorization is not a standing licence for the figure to track the roster: writing the
extent into the surface was that correction's own requirement precisely so a later reader could not
read it as one. So the second correction needs its own routing, and CLAUDE.md sends a change to a
recorded ruling's grounds to the operator **however plainly the arithmetic has moved** — the
strength of the arithmetic is the reason the routing exists, not a reason to skip it.

**No delta of this amendment may touch that file.** The finding is escalated at this stage's close.
Until it is answered, `TRAJECTORY.md`'s recorded figure and the tree disagree, and that is the
honest state rather than a defect to paper over: a spec states what is true now, a ruling record
states what was weighed then.

## What is kit mechanism, what is this project's own, and what is config

The seam ruling this stage's exit condition asks for:

- **Kit mechanism (gate-sdk).** The join bound and its produced-and-exercised predicate; the
  declaration block's *shape* and its two join states; the rule that a roster line may not exceed
  what the install page declares. Delta 3 adds one generic clause to that mechanism and no more —
  that a report keyed on held platforms has an empty subject once nothing is held, so a surface
  claiming it as a standing instrument overclaims. All of it stated undated, with no run id, no job
  name and no platform choice.
- **This project's own, and it stays out of every kit.** Which platforms are declared; the runner
  labels; the job names; the workflow prose deltas 4 and 5 repair; `native/targets.list`'s header
  arguments. gate-sdk/SPEC.md already rules that which jobs and which log line spell the pair are
  that project's CI and not kit mechanism, and this amendment adds nothing to a kit that would name
  one.
- **New consumer config: none.** No delta introduces a `<KIT>_<KNOB>`, and none is warranted: the
  arm-D conditional is a verdict-line repair inside an existing gate, not a new configurable.
- **Private rule content: none crosses.** The private brief was read and holds no unit-specific
  design memory for this join; nothing from it is quoted, paraphrased or cited here.

## Producers and consumers

**New state — `x86_64-apple-darwin` as a live roster line (delta 1).**
Producer: hand-authored, licensed by the run named above and by nothing else. Consumers, every one
already existing and reached with no further edit — each verified at HEAD rather than carried from
the predecessor amendment:

- `.github/workflows/publish.yml`'s `roster` job, which derives the release build matrix through
  `gate_native_runner` and now emits a **third** build leg on `macos-15-intel`. Its enabling
  configuration is present and was the specific thing checked: `runs-on` is `matrix.runner`, the
  macOS floor step is guarded on `runner.os == 'macOS'`, and `scripts/ci-macos-floor.sh` derives
  every path it writes from `brew --prefix` rather than hard-coding an Apple-silicon prefix. This
  is the consumer the predecessor amendment's build-body extraction existed for, and this join is
  the first thing to rely on it for a **second** macOS host class.
- `scripts/pack-installer.sh`, which refuses a roster target with no artifact directory and so now
  demands the Intel artifact in the payload.
- The payload's `artifact/targets.list` copy, read by `installer/lib/init.sh` to tell "never
  committed to" from "committed to, artifact missing"; that reader's host map already answers
  `Darwin/x86_64` with this triple.
- `check-install-platforms` arm B, which reds on a roster line the block does not declare `joined`
   — which is why delta 1's two edits are one commit.
- `check-gate-binary-fresh`, whose source stamp covers the file — the obligation delta 1 states.

The **consumer smoke is deliberately not on that list**: it derives a one-line host roster for
itself unless its caller sets `GATE_SDK_NATIVE_TARGETS_FILE`, so a third roster line reaches it as
nothing at all.

**New state — the declaration's `joined` word for that triple (delta 1).**
Producer: hand-authored in `docs/install.md` §Requirements. Consumers: `native-artifacts-roster`'s
reader, at the transition where `joined` sets `held=false`, which flips **two** legs binding — the
Intel producer through the matrix and `install-smoke-macos-intel` through the per-target index at
job-resolution time, before any of its steps run; `check-install-platforms` arms A and C; and a
human reading the page. No new field is added to the index and none is removed: its two fields,
`runner` and `held`, keep the two readers the predecessor amendment named.

**No new interface, no new event, no new message, and no delta narrows a corpus.** Deltas 2, 3, 4
and 5 are prose repairs on surfaces whose readers are unchanged, and delta 4 explicitly declines the
one code edit it considered. Point 5's red-condition enumeration
is not owed: every delta widens — a roster line, a declaration state, a build-matrix leg, a binding
posture — and the one reader whose verdict is non-monotone under a narrowing,
`pack-installer.sh`'s refusal of a declared target with no artifact directory, is handed **more**
subject rather than less. Arm D's report is not an assertion and cannot red in either direction.

## Existing sections updated

- `native/targets.list`'s header — the standing-case bullet asserting no run has carried both
  halves, the held-remainder paragraph, and the second-line cost accounting (deltas 1 and 2).
- `docs/install.md` §Requirements' `platforms:begin` block — the `x86_64-apple-darwin` bullet's
  state word and its continuation prose (delta 1).
- `native/runners.list`'s header — the separateness argument whose worked instance was both darwin
  lines being off the roster, half of which is already false at HEAD (delta 3).
- `docs/site-architecture.md` §Generated projections and their freshness gates, the
  install-platforms parity-contract row — its arm-D "standing instrument" claim, which this join
  silences, and its state-word paragraph, which now has its first worked instance of a flip making
  a consumer leg binding (deltas 1 and 4).
- `.github/workflows/gates.yml`'s `install-smoke-macos-intel` job — its header's closing paragraph,
  its suite step's comment and its verdict step's echoed text (delta 5).
- The generated `docs/` mirrors of the kit SPECs — regenerated, never hand-edited, and stale the
  moment any delta touching a kit SPEC lands (all deltas).

<!-- update-target-exempt: escalated at this stage's close and unresolved here — TRAJECTORY.md is the recorded-ruling surface, the prior correction's authorization was bounded to that one correction and is spent, and CLAUDE.md routes a change to a recorded ruling's grounds to the operator however plainly the arithmetic has moved, so no delta of this amendment may claim it -->
- `TRAJECTORY.md`'s 2026-08-14 born-native-default ruling, its weighed-cost figure — see §The one
  edit this amendment does not authorize.

## Retired spellings

- None — no delta of this amendment retires a spelling. Delta 1 flips a declaration's **state word**
  between two values the grammar keeps (`held:` stays live for the next declared-and-held platform);
  deltas 2, 3, 4 and 5 rewrite prose arguments under existing names; and no file, knob, job, script or
  contract name is removed or renamed anywhere in the set.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in
      `## Retired spellings` above, and `check-amendment-retired-spelling` runs
      each declaration against the whole tracked tree, not against the specs
      alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
