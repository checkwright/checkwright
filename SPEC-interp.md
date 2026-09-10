# SPEC amendment: interp

**APPLIED at `e65b893a` by build batch 1 of `host-resolution-fail-open-cut`.** Every delta below is
in the tree, and the per-passage **Not yet applied** markers are superseded by this line rather than
edited one by one — read a quoted replacement sentence as one the tree now carries, not as a
proposal. **Nothing in this amendment's own Definition of Done is outstanding:** the census was
re-run at `e65b893a` rather than inherited and is filed as a survey record, and the registry
assertion passes green with `check-graph`'s declaration unchanged. What remains is the terminal
move alone — this file's deletion and the queue's Done move — which batch 2 owns, held to that
batch because canon-kit/SPEC.md §Merging an amendment step 3 makes the none-remain assertion
unsatisfiable while sibling amendments for this component are still in flight. Ruled
`lead, own-authority` 2026-09-11, through the lead's message channel.

**This amendment depended on SPEC-run.md's delta 2 and the ordering was honoured:** that delta
funnels every `proc::run*` spawn through one resolution on Windows, and this one decides what that
resolution does for a name class the funnel alone cannot serve. Both landed in one build batch,
SPEC-run.md's first at `5961ea8e`.

## The census is re-run at this stage, and it supersedes the entry's own numbers *and* its file list

The entry records "20 sites across 15 files, 11 shipped and 9 test-only" and names the shipped
eleven. Its own text tells a later reader to distrust that: *"Read the four figures above as of
their date and re-run the oracle rather than trusting them; the recorded total is not maintained
here, deliberately."* Re-run at HEAD over `native/src` with the entry's own oracle:

**33 sites across 16 files — 25 shipped, 8 test-only**, every site classified by reading its
enclosing `#[cfg(test)]` scope in context rather than inferring from a filename.

Two things matter about the delta beyond the arithmetic. **Four of the eleven files the entry names
as shipped hold no `"bash"` literal at all today** — `hook/budget.rs`, `hook/stop_liveness.rs`,
`emit/kpi/always_loaded.rs` and `emit/kpi/prompt_friction.rs` — so the roster is wrong in kind and
not only in count, and `emit/always_loaded.rs` (a live hit) beside `emit/kpi/always_loaded.rs` (a
stale listing) is a same-basename trap set for the next reader. And **the two pointed repairs
hold**: `installer/init.rs` and `gates/graph.rs` are confirmed absent from the oracle, each now
calling `proc::resolve_interpreter("bash")`.

The corrected roster is deliberately **not** copied into this file. Derivation-first: the oracle is
the roster, this paragraph is its verdict as of 2026-09-11, and a build session re-runs it rather
than reading a list that will be wrong again.

## Why the three recorded shapes are all wrong, and the fourth the census produced

The entry says the shape is unruled among *"one holder with a governed name, a call-site sweep, or a
lint that reds a bare interpreter name."*

- **The call-site sweep is refused on a measurement.** The same census counted the whole
  bare-literal shipped-path spawn population: `git` **111** sites, `bash` 24, `date` 7, `mktemp` 5,
  `jq` 2, and one each of `uname`, `tar`, `npm`, `curl`, `cp`, `ps`. A sweep over that is not a
  shape anyone can cost. It is also what the recorded doctrine already rules out — §check-graph's
  two prior adoptions each landed *"on its own witnessed red and never on a sweep"* — and the
  measurement is now the second, independent reason.
- **A lint that reds a bare interpreter name is refused on its own terms**, and this is the
  non-obvious one. Under the funnel the call sites **keep** spawning the literal `"bash"`, correctly
  — resolution is the owner's job and the literal is what the registry declaration is compared
  against. A lint forbidding the literal would red 24 correct sites and push every one of them into
  the call-site resolution that broke `declaration_covers` in the first place. Enforcement-first is
  still owed; it is owed against a **different** predicate, which is delta 3.
- **"One holder with a governed name" is the surviving shape**, and the census tells us the holder
  is not one *function* but one **roster** the funnel consults.

**The fourth shape, which none of the three reaches: the funnel closes the class with zero
call-site edits.** That is the whole yield. Twenty-five shipped sites are repaired by a change to
the owner they already call, no site is pointed, no site is swept, and no site's literal moves.

## Why the funnel alone is not enough, which is what keeps this a unit

SPEC-run.md's funnel resolves through `which()`, which walks `PATH` in the operating system's own
order. On the attested Windows host `%SystemRoot%\System32` **precedes** Git-for-Windows'
`usr/bin`, and the only `bash.exe` Windows ships there is the WSL launcher. So a funnelled bare
`bash` still reaches the launcher: the funnel fixes the probe/spawn *disagreement* and does nothing
about the wrong-program *homonym*. The system-directory rejection is program-class-specific, and
supplying it is this amendment's work.

## What changes

### (1) The governed roster: which program names carry a Windows system-directory homonym, and what a host offering only that homonym earns

`proc.rs` gains one roster, and it is the single owner of a question two functions answer separately
today {design-bearing}.

Each member is a **name** plus a **no-resolution disposition**, and both existing dispositions are
carried over with the grounds they were argued under rather than re-argued:

| name | disposition when nothing resolves outside the system directory | the ground, already on record |
|---|---|---|
| `bash` | **Refuse**, by name, saying what was skipped and why | gate-sdk/SPEC.md §check-graph — a shell that is not a shell is worse than not running, and the refusal *"arriv[es] with a cause rather than as the bare refusal that cost a CI round"* |
| `sort` | **Fall back** to the bare name | context-kit/SPEC.md §bin/env-probe — *"the verdict is then the roster's own absent or wrong-impl, which is the true reading of such a host and the fail-closed direction"* |

**The membership criterion is stated, so the roster is extensible without a second design round:** a
name belongs here when the Windows system directory ships a program of that name that is **not** the
program the payload wants. §check-graph already states this for the two members and generalises it
— *"That ground was stated for one member and holds for the roster"* — so this delta writes the
roster the sentence already presumes.

**Two functions become two faces of one roster and neither is retired.** `resolve_interpreter` and
`resolve_floor_tool` keep their names, their callers and their behaviour; what changes is that the
name-to-disposition question moves out of the two call sites and into the roster, so a third member
cannot be added to one and forgotten in the other. `resolve_floor_tool` additionally keeps an
identity the funnel cannot absorb — it is a **reporting** resolver, its value rendered in doctor's
banner and the env-probe emitter, not only spawned.

### (2) The funnel consults the roster, and the two pointed call sites hand their resolution back to it

`gates/graph.rs:380` and `installer/init.rs:719` stop resolving and spawn the bare name again
{design-bearing}.

Under delta 1 the funnel gives every `bash` spawn the rejecting resolution, so a call site that
resolves first is doing the owner's job — and doing it in the one place where it is observably
harmful. `proc::recorder::note` records the argument **as passed**, and
`every_registry_member_declares_the_programs_it_spawns` compares that record against the registry's
declared requirement by exact equality. Those two call sites are why
`registry-needs-conflates-requirement-and-spawn` records a live disagreement today. Reverting them
onto the funnel **repairs that disagreement** as a consequence of getting the ownership right.

**This amendment does not answer that entry's open question and takes care not to.** Whether a
`# graph:` declaration is a host REQUIREMENT or a literal argv[0] is a contract question
gate-sdk/SPEC.md §The `# graph:` manifest owns, and that entry stays `[design-pending]` on it. What
changes is that the question loses its live instance: under the funnel the recorded name and the
declared name are the same string whichever way the contract is later read. The entry's **vacuity**
half — that both `check-graph` fixtures pass `--amend-only` and never reach the generator arm, so
the assertion passes vacuously — is untouched here and stays wholly that entry's.

**Twenty-three shipped sites change no bytes at all.** They spawn `"bash"` today and spawn `"bash"`
after; the funnel is what changed underneath them. That is the shape's whole argument, and it is
also why the delta needs the assertion in delta 4 — a repair invisible in the diff is a repair
nothing would notice regressing.

### (3) The enforcement-first half: the bypass lint's corpus widens from one directory to the crate

`no_gate_module_constructs_a_subprocess_itself` (`proc.rs:1106`) scans `src/gates` alone; its corpus
becomes `native/src` minus `proc.rs`, with a declared exception class {design-bearing}.

The funnel is only a funnel if nothing goes around it, and the lint that would say so is scoped to a
tenth of the crate. Probed at HEAD, two shipped `Command::new` sites sit outside `proc.rs` and
outside the lint's reach, and they are **different classes**:

- `native/src/emit/wait_probe.rs:245` takes `std::os::unix::process::CommandExt`. That is a shape
  `proc::run*` genuinely cannot carry, and it is the **declared exception** — named, with its cause,
  on the tree's existing valve convention, so the next reader meets a decision rather than an
  oversight.
- `native/src/emit/enter_stage.rs:1306-1324` is a private `run_merged_with_env`: a re-implementation
  of `proc::run_merged_in` with its own refusal string and a **concatenated** rather than
  interleaved capture. There is no shape argument; it duplicates the owner. It is folded onto
  `proc::run_merged_in` by this delta, which also gives it the interleaved capture the owner's
  `try_clone` technique buys.

**The lint's own false claim is corrected in the same delta.** `proc.rs:1` calls this module *"the
crate's one spawn site"*, which is not true at HEAD and was not true when the lint was written with
a one-directory corpus. **Not yet applied** — the comment states the property the widened lint
actually holds, naming the exception class rather than asserting a purity the tree does not have.

### (4) The repair is asserted from a host that cannot execute it

Two injected-input cases, on the shape `proc.rs`'s existing Windows assertions already use
{design-bearing}.

`the_attested_windows_path_resolves_past_the_wsl_launcher` proves the *resolver*. What no case
proves is that a **bare-name spawn** now reaches it, which is precisely what deltas 1 and 2 change
and precisely what leaves no trace in twenty-three call sites' bytes. The cases: a funnelled spawn
of a roster member with a `Refuse` disposition, on a PATH offering only the system directory, yields
the named refusal; and a funnelled spawn of a `FallBack` member on the same PATH yields the bare
name. The cannot-exercise-locally doctrine (§Fail-closed contract) is what makes both runnable on
the Linux host that develops them, and it is cited rather than restated.

## Producers and consumers

**One new interface — the governed roster — and no new state, event, file or emitted artifact.**
Surveyed across the whole component set: `native/src` in full, the gate registry in
`native/src/gates/mod.rs`, `installer/`, and both kit SPECs that own a disposition, with no stderr
suppressed on any path grep.

- **The roster** (delta 1). *Producer:* a compile-time table in `proc.rs`, the crate's single owner
  of what an installed program may be named. **Enabling config: none, and the absence is a
  decision.** A knob listing homonym names would let a consumer's host disagree with the crate about
  which programs Windows ships in `System32`, which is a fact about the platform and not about the
  consumer — the same reason `WINDOWS_SYSTEM_DIR_VIEWS` is a constant rather than a knob. *Consumers,
  each named with its transition:* the funnel, at every Windows spawn, to select the resolution;
  `resolve_interpreter`, at a pointed interpreter resolution, for its disposition;
  `resolve_floor_tool`, at a floor probe, for its disposition and its rendered value.
- **Every field has a named reader.** The roster's rows carry exactly two fields. `name` is read by
  the funnel at spawn (exact match against the unresolved program). `disposition` is read by
  `resolve_outside_system_dir`'s caller at the no-resolution branch, and by nothing else. A third
  field naming *why* a member is on the roster was considered and **refused**: its only reader would
  be a human, prose already carries it at the membership criterion, and a field read by no code is
  one the DoD says to remove.
- **The reverted call sites** (delta 2). *Producer:* `gates::graph::generator_emit` and
  `installer::init::run_vendored`, each now emitting the unresolved name into `proc::recorder`.
  *Consumer:* `every_registry_member_declares_the_programs_it_spawns`, at the crate test arm, which
  compares that record against `gates/mod.rs`'s declared requirement — the reader whose exact-match
  comparison is failing today.
- **Existing integration prose describing the prior flow** is updated in three places, listed below;
  the important one is §check-graph's *"each on its own witnessed red and never on a sweep"*
  passage, which describes a flow this amendment ends.

**Producer/consumer edges inside batch A, for the lead's batch cut.** This unit **consumes**
SPEC-run.md's funnel (delta 2 there) and cannot land before it. It has no edge to SPEC-host-detect.md
or SPEC-arm64-linux.md, which touch the installer's host map and the workflow rather than the
crate's spawn path. **No edge crosses to batch B.**

## Existing sections updated

- `gate-sdk/SPEC.md` §check-graph, the paragraph beginning *"The interpreter assertion D spawns is
  resolved, not named"* — its two-pointed-readers roster and its *"each on its own witnessed red and
  never on a sweep"* clause describe the flow deltas 1 and 2 replace. Rewritten to state the roster and the
  funnel (deltas 1 and 2). **This is a rewrite a later reader could mistake for a reversal, so it
  carries its own sentence saying it is not:** the recorded ruling narrowed an envelope *"a resolver
  over the whole residue triages as a feature, a feature is a yield"* and left the residue unpointed
  *"under its queue entry"*. That entry is promoted as a feature at this iteration's scope, which is
  the route the ruling itself named. The ruling is **discharged, not reversed**, and no delta here
  reopens it.

  **Whose ruling is being discharged, stated because the answer changes who may discharge it.** It
  is the **lead's**, not the operator's — `TASK-QUEUE.md`'s own record of it reads *"The lead ruled
  the envelope narrow"*. A later reader looking for an operator ruling behind this rewrite will find
  none and should stop looking; there never was one. The operator-class carve-out on reversing a
  recorded ruling therefore never reached this question, and it is a discharge besides — CLAUDE.md's
  own line is that retiring a spent ruling is not reversing one.

  **Ruled `lead, own-authority` 2026-09-11, through the lead's message channel.** The authoring
  stage escalated the discharge-or-reversal judgment anyway rather than acting on it unrouted,
  offering three dispositions — discharge as authored, treat it as a reversal and relay to the
  operator, or narrow to one further pointed site — and recommended the first. The lead verified the
  route against the recorded wording and took it, noting that routing a possible reversal is the
  right instinct even where the answer is that it is not one. The narrowing option was refused on
  measurement: the shipped bare-literal spawn population is `git` 111 sites, `bash` 24 and nine
  other programs, so pointing one further site closes nothing.
- `context-kit/SPEC.md` §bin/env-probe, the clause recording that a floor probe *"falls back to the
  bare name where these two refuse"* — the departure is now a roster **disposition** rather than a
  property of one function, and the clause says so while keeping its ground verbatim (delta 1).
- `native/src/proc.rs:1`, the module's own `# spec:` comment claiming *"the crate's one spawn
  site"* — replaced by the property the widened lint holds (delta 3).
- `native/src/proc.rs`'s `# spec:` comments on `resolve_interpreter` and `resolve_floor_tool` — each
  today names its own callers and its own disposition; both now cite the roster (deltas 1 and 2).
- `TASK-QUEUE.md`, the entry `registry-needs-conflates-requirement-and-spawn` — its recorded live
  disagreement is repaired by delta 2 while its contract question and its vacuity half stand. The
  entry needs one paragraph saying so, or a later drain re-reads a symptom that is gone and judges
  the whole entry closed. **Not yet applied**, and it is a *deferred* entry's body, so build files
  it through the gap inbox rather than editing the queue mid-iteration (delta 2).

## Retired spellings

- None — no delta of this amendment retires a spelling. `resolve_interpreter` and
  `resolve_floor_tool` both survive with their names, their callers and their behaviour; delta 1
  moves the name-to-disposition question behind them rather than replacing them, precisely so no
  sweep is owed. Delta 3 renames nothing: `run_merged_with_env` is **deleted** rather than
  retired-and-replaced, its two callers moving to the existing `proc::run_merged_in`, so there is no
  new spelling and the old one has no survivors to chase.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and
      a named consumer; every new field has a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); the merged spec reads as one coherent document a reader who never saw
      the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root
      (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired
      spellings` above, and `check-amendment-retired-spelling` runs each declaration against the
      whole tracked tree.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
- [ ] **The census is re-run, not copied** — the build session re-runs the entry's oracle at its own
      HEAD and records the figures with their date, rather than inheriting this amendment's.
- [ ] **The registry disagreement is verified repaired** —
      `every_registry_member_declares_the_programs_it_spawns` passes with `check-graph`'s
      declaration unchanged, which is the observable half of delta 2 and the one a green suite can
      actually show.
