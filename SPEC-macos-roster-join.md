# SPEC amendment: macos-roster-join

**The observation the predecessor amendment specified has fired, and this amendment executes it.**
`SPEC-platform-evidence.md` built the release-shaped producer, declared the supported set on a
readable surface, and wrote the join predicate down mechanically so that "the session that observes
the green executes rather than re-derives". The green exists. This amendment writes
`aarch64-apple-darwin` into `native/targets.list`, and — because a roster line is a *release*
commitment and not a CI one — repairs the release path that line would otherwise break. It then
buys the second target's missing half by adding an Intel `install-smoke` leg.

**What it deliberately does not do:** it does not write `x86_64-apple-darwin`'s line. That triple's
producer half is already discharged; its consumer half is what this amendment builds, and the run
that discharges it does not exist yet. Writing both lines here would be the exact failure
`native/targets.list`'s header names — a roster widened on a plan.

## The rulings this amendment executes

- **Unit shape: the aarch64 join PLUS a new Intel `install-smoke` leg** — ruled by the OPERATOR
  2026-09-08, asked and answered in a `/lead` session and lead-relayed. The leg's cost was put
  explicitly and accepted: **a new job name, and doubled macOS minutes on every push**. The aarch64
  half is the floor and lands whatever the Intel leg does.
- **The join predicate itself is not reopened.** `native/targets.list`'s header owns it and this
  amendment quotes no part of it. The 2026-09-07 operator ruling behind it — that one green macOS
  install-smoke leg licenses nothing on its own — is untouched and is what the aarch64 join
  satisfies rather than sidesteps.
- **`.github/workflows/gates.yml`'s licensing paragraph is out of this amendment's reach.** Its
  release for repair belongs to the promoted debt unit
  `gates-yml-macos-leg-ground-stale-and-self-contradicting`, was given for that repair and no wider,
  and delta 6 below touches a *different* paragraph for a *different* falsity. §Where this
  amendment abuts that debt unit says so precisely, because the two edits sit within a few lines of
  each other and a batch that mixes them will do by accident what the ruling refused by relay.

## The licence, verified first-hand at this stage rather than relayed

`gates` run `34212264301` at `dd6dcf54`, read job-keyed off the finished run for free. Both limbs of
the predicate, on one run, at a commit later than the paragraph that states the predicate:

1. `native-artifacts (aarch64-apple-darwin, macos-latest, true)` — **success**. Binary and `.sha256`
   sidecar produced by the release's own build command and uploaded.
2. `install-smoke-macos` — **success**, `host triple: aarch64-apple-darwin`, having consumed that
   upload under `INSTALLER_SMOKE_ARTIFACTS_DIR` and reached the artifact-present branch, whose own
   words in the log are `adopted checkwright-gates for aarch64-apple-darwin from the hand-off,
   sidecar and all — nothing rebuilt, nothing rehashed`, followed by `INSTALLER-SMOKE: clean` and
   `installer_smoke on macOS exited 0`.

Produced and exercised, with no host-built stand-in anywhere in it. **The same run also discharges
limb 1 for `x86_64-apple-darwin`** — `native-artifacts (x86_64-apple-darwin, macos-15-intel, true)`
was green — which is why that triple's only missing half is the consumer leg delta 5 builds, and why
its join becomes buyable on the next run rather than on a new design turn.

## What changes

### (1) `aarch64-apple-darwin` joins, on the three surfaces that move together

The join `native/targets.list`'s header specifies, executed and not re-derived {mechanical}:
`native/targets.list` gains the triple's line; `docs/install.md` §Requirements' `platforms:begin`
block flips that triple from `held:` to `joined`; `native/runners.list` already names its runner and
takes no edit. `check-install-platforms` holds the first two in lockstep in both directions, so
these land in one commit or the commit reds.

The declaration bullet's prose is rewritten with the state word rather than left beside it: its
current continuation says the platform "does not yet have … a smoke leg that installs the result and
reaches its artifact-present branch", which the run above falsifies. A flipped state word over prose
that still argues the hold is the drift the block exists to end.

### (2) `native/targets.list`'s header stops arguing from a one-target roster

Three passages in that header take the roster's size as a premise and go false at delta 1
{design-bearing}: "It ships one target, and the ground is evidence rather than capability";
`x86_64-unknown-linux-gnu` "is the only platform with a discharged precondition"; and "a one-target
roster makes the omit-and-declare path the normal path for a macOS adopter on day one".

**The predicate is untouched and the second argument survives in a narrower form**, which is the
distinction this delta has to hold. What licenses a line is unchanged; what changes is that two
lines now hold it. And omit-and-declare stays an exercised path rather than becoming dead code —
Intel macOS is still held, native Windows is still undeclared, and every host outside the block
still meets it — so the paragraph is re-grounded on the *held remainder* rather than on the roster
having one line. The header's own accounting of what a second line costs is likewise re-grounded:
it is written as an unpaid cost, and delta 3 is where it is actually paid.

### (3) The release build leg and the CI producer become one body, and the release leg gains the macOS floor it has never had

**This is the delta that makes the roster line honest, and nothing has stated it before.**
`native-artifacts` exists on the ground — its own header's words — that "the build command, the
sidecar's format and the directory layout are `.github/workflows/publish.yml`'s build leg's, so what
CI exercises is what a release would publish". **That claim is false at HEAD**, and delta 1 turns
the falsity into a broken release {design-bearing}:

- `publish.yml`'s build leg carries **no** bash-floor or GNU-userland bootstrap, where
  `native-artifacts` carries one under `if: runner.os == 'macOS'`. `gate-sdk/lib/gate.sh`'s knob
  defaults use the `[[ -v ]]` unary, a bash 4.2 construct, and a `macos-latest` runner's `bash` is
  3.2 — which is not predicted here but **attested**: that bootstrap step exists because run
  `34200226768` measured both darwin legs dying on their first executed line with `conditional
  binary operator expected`, having compiled nothing.
- `publish.yml`'s digest step calls `sha256sum` unconditionally, where the producer resolves a
  hasher because stock macOS ships `shasum` and no `sha256sum`.

Today neither costs anything, because `native/targets.list` names one Linux triple and the release
matrix is roster-derived. **Delta 1 is what puts a `macos-latest` leg into the release matrix**, so
without this delta the first release after the join fails on its second build leg — on a path
nothing exercises until a tag, which is the worst place to discover it.

**The repair is to remove the divergence rather than to copy it a second time**, on the ground the
producer's own header states: two copies of one build body is how a release comes to build an
artifact CI never measured. Two tracked scripts, each invoked by both workflows:

- **the macOS floor** — the `brew install` of the declared bash floor and GNU userland plus the
  `$GITHUB_PATH` ordering, which must stay its own *step* because a `$GITHUB_PATH` write takes
  effect only for later steps;
- **the artifact body** — `rustup target add`, `build-native.sh --target`, the copy into
  `<outdir>/<target>/`, and the single digest emission with its resolved hasher. It takes the target
  and the output directory as arguments rather than reading `RUNNER_TEMP`, so it is runnable — and
  is to be run — on a developer machine before the push.

**Do not read this as a third bootstrap.** `install-smoke-macos`'s `brew` step is deliberately not
folded in: that step's package set is an *adopter claim* held equal to `docs/install.md`
§Requirements in both directions, and this one is scaffolding held to what the producer's own path
executes. Its header already draws that line; this delta preserves it.

**The residual risk is stated rather than left to be met.** This edit touches the producer leg that
delta 1 makes binding, so a mistake reds master and spends the second push. What contains it is that
the extraction is locally verifiable in full on the Linux arm — the build body is the same script on
every leg — and the floor step is a verbatim move of a body already attested green on the same
runner class.

### (4) `native-artifacts-roster` publishes the declaration keyed by target

The roster step gains a second output, a JSON object mapping each declared triple to its runner and
its held-ness — the same two facts it already computes for the matrix, indexed instead of listed
{mechanical}. Nothing about the existing `legs` output or the matrix it feeds changes.

The index exists because a GitHub Actions expression can index an object and cannot filter an array,
and delta 5 needs one platform's two facts at *job* level, where no matrix context exists.

### (5) `install-smoke-macos-intel`, whose posture is derived and not declared

A new job buying `x86_64-apple-darwin`'s missing consumer half {design-bearing}: it `needs` both
`native-artifacts` and `native-artifacts-roster`, and takes **both** its runner and its
`continue-on-error` from delta 4's index at its own triple. Its steps mirror `install-smoke-macos` —
the GNU-userland install, the runner probe, the reporting-only crate probe, the pattern download,
the normalize onto `<artifacts>/<target>/`, and the smoke under `INSTALLER_SMOKE_ARTIFACTS_DIR` — and
it spells no host triple in any of them, deriving the host from `rustc -vV` exactly as its sibling
does.

**Three properties are load-bearing:**

- **Its posture is derived, so the join needs no second edit and can have no third state.** A held
  platform's leg is `continue-on-error` for the reason `native-artifacts`' header already gives — a
  red on a held platform is why it stays held, not a reason master goes red — and the day
  `x86_64-apple-darwin` flips to `joined`, this leg becomes binding with nobody remembering to make
  it so. A hard-coded `continue-on-error: true` is the maintained copy that goes wrong exactly once,
  silently, in the direction of a joined platform whose smoke red reds nothing.
- **It therefore mints no new instance of `held-ci-leg-failure-reddens-a-binding-one`.** That entry
  is about a *held* producer defeating held-ness through a *binding* consumer; this consumer is held
  while its producer is, so the pair moves together. Delta 1 also retires the one live instance the
  entry recorded, by making the aarch64 producer binding — that pair is now binding-on-binding, and
  what remains of the entry for it is the second cost it names, that the consumer's measurement goes
  to zero rather than degrading. The entry stays deferred on its own structural ground; this
  amendment neither closes it nor re-scopes it, and says this much so the next reader of it knows
  which half moved.
- **A triple spelled at a job's own level is a read of the declaration, not a second assertion of
  support.** This is the one platform literal in either workflow, and it is admitted because the
  alternative is worse in a way the download-by-pattern rule beside it is not: that rule avoids
  *asserting* a platform in CI, and this expression *reads* one. If the triple ever leaves the
  declaration the index yields nothing, the job cannot resolve a runner and the workflow reds —
  loudly, which is the right direction for a support retraction that forgot its leg.

**What the leg cannot do, stated so no reader expects it.** It cannot join its own target: criterion
1 is `native-artifacts`' and criterion 2 is this leg's, and the write is a later session's on a run
that carries both. Its first green is that run's second half.

### (6) The macOS leg's steering precondition stops counting the roster

The `install smoke on a macOS host` step's header argues its steering from `native/targets.list`
carrying "one line, `x86_64-unknown-linux-gnu`" {mechanical}. Delta 1 falsifies the count while
leaving the argument sound — an unsteered smoke on that host still meets the roster/host refusal,
because the roster still names a platform that host is not. The sentence is de-literalized to say
that rather than to say a number, which is the repair the queue's own
`queue-citation-line-number-stales-within-its-own-session` finding generalizes.

## Where this amendment abuts the promoted debt unit

`gates-yml-macos-leg-ground-stale-and-self-contradicting` owns the repair of a **false ground** in
two passages of `.github/workflows/gates.yml`. Delta 6 owns a **false count** in one of them. The two
are different sentences and different falsities, and the operator's release covers only the debt
unit's subject.

**Two consequences for whoever cuts batches.** First, landing these in one batch is cheapest: the
debt unit's queue entry already records that its own cited line numbers no longer resolve, and delta
6 moves them again. Second, the licensing paragraph — the one the release names — is a few lines
below delta 6's sentence, so a batch holding both must keep the ruling's boundary explicitly in
view: the released repair is that paragraph's ground sentence, never its verdict, and never a wider
reading of the paragraph.

## The one edit this amendment does not authorize

`TRAJECTORY.md`'s 2026-08-14 ruling that a new gate is born native unless a stated cause says
otherwise records the cost it was weighed against, in a sentence that ends "recorded so it is not
re-argued": *a `.gate`-declared member is omitted on a platform `native/targets.list` carries no
artifact for, and that roster is one target, so the flip attaches that omission to **every macOS
adopter** for every new gate.*

**Delta 1 falsifies both halves of that ground** — the roster stops being one target, and the
omitted set stops being every macOS adopter, narrowing to Intel Macs and to the hosts the
declaration does not carry at all. The ruling itself is untouched and, if anything, cheaper than it
was costed at. But the sentence is a recorded ruling's stated grounds, and CLAUDE.md routes a change
to one to the operator **however well-grounded the finding and however plainly the arithmetic has
moved** — the strength of the case is not a reason to skip the routing, which is the whole point of
the rule. So this amendment states the finding and authorizes no edit to that file.

It does repair the same arithmetic where it appears as a **live claim in a governed spec** rather
than as a record: `gate-sdk/SPEC.md` §The port-candidate criteria's criterion-5 bullet, listed
below under delta 1. Until the escalation is answered the two surfaces disagree, and that is the
honest state rather than a defect to paper over — a spec states what is true now, a ruling record
states what was weighed then.

## What is kit mechanism, what is this project's own, and what is config

Ruled here because the exit condition asks for it, and because every delta below the join sits on a
kit-adjacent surface:

- **Kit mechanism (gate-sdk):** the join bound and its produced-and-exercised predicate, the
  declaration-block *shape*, and the widening-cost rule delta 3 corrects. All of it is stated
  undated and without this project's runs, job names or platform choices.
- **This project's own, and it stays out of every kit:** which platforms are declared, the runner
  labels, the job names, the two `scripts/ci-*` bodies, and the `native-artifacts-roster` output
  delta 4 adds. gate-sdk/SPEC.md already rules that "which jobs and which log line spell it are that
  project's CI and not kit mechanism", and this amendment adds nothing to a kit that would name one.
- **New consumer config: none.** No delta introduces a `<KIT>_<KNOB>`, and the two new scripts take
  arguments rather than knobs precisely so they stay this repo's CI rather than becoming a
  configurable surface a kit would then owe a roster entry for.

## Producers and consumers

**New state — `aarch64-apple-darwin` as a live roster line (delta 1).**
Producer: hand-authored, licensed by the run named above and by nothing else. Consumers, every one
of them already existing and reached by this line with no further edit: `publish.yml`'s roster job,
which derives the release build matrix from `gate_native_targets` and now emits a second build leg —
**this is the consumer delta 3 exists for, and the only one whose enabling configuration was
missing**; `scripts/pack-installer.sh`, which refuses a declared target with no artifact directory
and so now demands the macOS artifact in the payload; the payload's own `artifact/targets.list`
copy, read by `installer/lib/init.sh` to tell "never committed to" from "committed to, artifact
missing"; and `check-install-platforms`, whose arm B reds on a roster line the block does not
declare `joined` — which is why delta 1's two edits are one commit.
The consumer smoke is deliberately **not** on that list: it steers itself at a one-line host roster
by default, so a second roster line reaches it as nothing at all. That is the re-entry the
predecessor amendment built, and this delta is the first thing to actually rely on it.

**New state — the declaration's `joined` word for that triple (delta 1).**
Producer: hand-authored in `docs/install.md` §Requirements. Consumers: `native-artifacts-roster`'s
awk, at the transition where `joined` sets `held=false` — so this flip **also flips that triple's
producer leg from `continue-on-error` to binding on master**. That is a support commitment taking
effect rather than an incidental change, and it is stated here because the header derives it and
does not say it: the leg is green today, so nothing reds at the landing, and the cost arrives the
first time an Apple-silicon build breaks. The block's other consumers are unchanged
(`check-install-platforms`' arms A and C, and a human reading the page).

**New interface — delta 4's per-target index.**
Producer: the `native-artifacts-roster` step, on every `gates` run, reachable with no configuration
because it derives from a tracked page. Consumer: delta 5's job, at job-resolution time, before any
step runs. Fields and readers: **runner** — read by that job's `runs-on`; **held** — read by that
job's `continue-on-error`. Two fields, two readers, and no third field is added: the target itself is
the index key rather than a field, and nothing else the roster step computes has a reader here.

**New event — the Intel leg's own run (delta 5).**
Producer: the job, on every `gates` run, needing no configuration. Consumers: a human reading the
run job-keyed, which is criterion 2 of the join predicate and the whole purpose of the leg; and
`native/targets.list`'s header, as the surface that says what that reading licenses. It emits no
artifact and no output any other job consumes — deliberately, since a leg whose green is read by a
later human and by nothing else is exactly what a measurement leg is.

**New interface — the two `scripts/ci-*` bodies (delta 3).**
Producers: hand-authored scripts. Consumers: `publish.yml`'s build leg and `gates.yml`'s
`native-artifacts` job, both real deployed configurations rather than test-only ones — which is the
point of naming them, since the release consumer is the one that has never been exercised on macOS
and is the reason this delta exists at all. The floor script's consumers each guard it with
`if: runner.os == 'macOS'`; the artifact script's consumers each pass a target and an output
directory.

**Existing integration prose updated, and no delta narrows a corpus.** Point 5's red-condition
enumeration is not owed here: every delta *widens* — a roster line, a declaration state, a workflow
job, two scripts — and the one reader whose verdict is non-monotone under a narrowing,
`pack-installer.sh`'s refusal of a declared target with no artifact, is being handed **more**
subject rather than less. Its planted witness in the consumer smoke is untouched.

## Existing sections updated

- `gate-sdk/SPEC.md` §Consumer payload, its "Widening is cheap on the publish path and not free
  elsewhere" paragraph — the cheap half is falsified by delta 3, which found the publish path's
  second platform costing a runner bootstrap the first platform never needed; the paragraph gains
  that cost, stated as generic mechanism and not as this repo's incident (delta 3).
- `native/targets.list`'s header — the two size-premised passages, and its accounting of what a
  second line costs, which delta 3 pays (deltas 2 and 3).
- `docs/install.md` §Requirements — the declaration bullet whose state word flips and whose prose
  still argues the hold (delta 1).
- `docs/site-architecture.md` §Generated projections and their freshness gates, the
  install-platforms parity-contract row — it states what the declaration's two readers do with the
  state word, and delta 4 makes that word decide a consumer leg's binding posture as well as a
  producer leg's (deltas 4 and 5).
- `.github/workflows/publish.yml`'s build-leg header — it states this leg's runner is an expression
  and reasons about the day a Windows target is declared, without noticing that the day a *macOS*
  target is declared arrived first and needed a floor (delta 3).
- `.github/workflows/gates.yml`'s `native-artifacts` header — its "what CI exercises is what a
  release would publish" ground becomes true by construction rather than by inspection, and should
  say which surface makes it so (delta 3).
- `installer/README.md` §The consumer smoke, its job-keyed-log-read paragraph — it names
  `install-smoke-macos` as "the slowest sibling" a close would otherwise wait on, which a second
  macOS leg makes a claim about the wrong number of jobs (delta 5).
- `installer/README.md`'s two-bootstraps paragraph — it counts the workflow's install-smoke jobs
  outright ("three install-smoke jobs … a fourth drives the PowerShell half … the other three drive
  the other bootstrap"), and its whole point is that the count be read by bootstrap rather than by
  platform, so a fifth job that adds nothing to the bootstrap side is exactly the arithmetic it
  cannot get wrong (delta 5).
- `installer/README.md`'s `INSTALLER_SMOKE_ARTIFACTS_DIR` knob entry — it names the macOS
  install-smoke leg as the knob's single live setter and says it is "unset everywhere else", which
  a second setting leg falsifies (delta 5).
- `gate-sdk/SPEC.md` §The port-candidate criteria, criterion 5's bullet — its stated price is
  "`native/targets.list` shipping one target, so the uncovered set is every macOS adopter rather
  than a narrow hypothetical", and delta 1 halves that set: Apple-silicon adopters stop being in
  it and Intel ones remain. The mechanism is untouched; the arithmetic narrows, and this is the
  surface where that number is a live claim rather than a record (delta 1).
- `gate-sdk/SPEC.md`'s host-derived-artifact-name passage — it justifies a per-line derivation
  "with the roster still one line", a present-tense count delta 1 falsifies while leaving the
  derivation itself correct (delta 1).
- The generated `docs/` mirrors of the kit SPECs — regenerated, never hand-edited, and stale the
  moment any delta touching a kit SPEC lands (all deltas).

<!-- update-target-exempt: escalated to the lead at this stage's close and unresolved here — TRAJECTORY.md is the recorded-ruling surface, and CLAUDE.md routes a change to a ruling's recorded grounds to the operator however well-grounded the finding, so no delta of this amendment may claim it -->
- `TRAJECTORY.md`'s 2026-08-14 born-native-default ruling, its weighed-cost sentence — see §The one
  edit this amendment does not authorize.

## Retired spellings

- None — no delta retires a name. The join replaces a declaration's state word, delta 3 moves two
  step bodies into scripts under their existing spellings, and every other delta adds a surface
  rather than removing one.

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
