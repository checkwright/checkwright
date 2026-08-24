# TASK-QUEUE.md — Checkwright work queue

## Iteration: turn-end-liveness-seam-and-worktree-cause

  The lifecycle-kit gates read this header's iteration name and the stage
  cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt`
  (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue
  format itself and gates this file. One iteration per hardening or roadmap
  unit; [README.md](README.md) maps the kits.

---

## New Features

- **turn-end-liveness-exit-two-conflation** [spec: SPEC-liveness-exit-classes.md] — the
  SubagentStop hook reads the gate substrate's fail-closed "could not run" as a malformed launch
  record, so every worktree-isolated agent is refused at turn end.
  **OPERATOR-RULED 2026-08-24 and relayed through the lead at the close of
  `port-remainder-disposition-and-worktree-reclamation`: TAKEN NEXT ITERATION, ahead of normal
  scope intake.** Formally a FOURTH yield of TRAJECTORY.md §PRIORITY DIRECTIVE, and like the
  three before it it spends one iteration's turn alone and makes no claim on the sequence's next
  turn — NOT a reversal, demotion or re-scoping of that directive. What distinguished it: it
  ENABLES rather than competes, since the refusal disables exactly the delegated read-heavy
  audits CLAUDE.md pre-authorizes and the port's own remaining work would use.
  **The cost is wider than the bullet that filed it.** `scripts/delegation-config.sh` declares
  `audit-sweep` the read-only dispatch type and `scripts/agent-dispatch-guard.sh` blocks any
  `audit-sweep` dispatch that is not worktree-isolated, so the sanctioned type is forced into
  exactly the shape the refusal kills.
  **The operator ruled the PRIORITY and not the choice, and spec ruled the choice.** All three
  candidate dispositions, the mechanism re-verified against source, the fifteen-of-fifteen
  measurement of the refusing shape, and the fourth conflation spec found on the MESSAGE axis
  are in the amendment; this entry deliberately keeps none of that ruling, the entry format
  having no home for causal completeness.
  **Cost while deferred:** every read-heavy audit delegation is pre-authorized for is either
  un-dispatchable or must be re-shaped onto a type that makes no read-only claim, and the
  refusal message points at the one change that would disable the check entirely.
  Surfaced 2026-08-24 at the `port-remainder-disposition-and-worktree-reclamation` close, filed
  to the gap inbox there and corroborated by all three of that close's audit dispatches; drained
  and promoted to the deferred pool 2026-08-24 at that scope, which re-verified the mechanism
  first-hand; promoted to a feature 2026-08-24 at spec, which attested the refusal in a live
  worktree dispatch of its own.

- **agent-worktree-boundary-disposition** [spec: SPEC-worktree-liveness.md] — the iteration
  boundary refuses on registered agent worktrees, but nothing reaps them and nothing surfaces
  one mid-iteration.
  **The finding, operator-directed 2026-08-18.** Four worktrees survived an iteration's close,
  registered in `git worktree list` and invisible to every gate.
  **Its stated blocker is DISCHARGED, and that is why it was authorable.** The entry held that
  a worktree has no liveness signal and that inventing one was the design work. The upstream
  experiment `worktree-reclamation-cause-falsification` ran at spec and found the signal already
  published by git, so nothing is invented and the reap is designable against a mechanism. The
  experiment's result, the classification it makes possible and the seam ruling that keeps one
  harness's vocabulary out of the kit are in the amendment.
  **The cheap half was TAKEN 2026-08-24** — `agent-worktree-reclamation-unenforced` landed the
  boundary refusal, which surfaces the paths and leaves removal a session act. What is left is
  removal and the mid-iteration surfacing.
  **Cost while deferred:** low, and lower since the refusal. Worktrees cost only disk, and the
  carry is the reap itself: a session meeting the refusal still re-derives by hand whether each
  survivor is safe to remove.
  Filed 2026-08-18 by the lead under the operator-directed direct-to-queue exception, the four
  having been removed once verified empty and clean; promoted to a feature 2026-08-24 at spec,
  once its upstream experiment returned.

- **subagent-stop-agent-id-attribution-doubt** [spec: SPEC-agent-id-doubt.md] —
  delegation-kit/SPEC.md's `session` field bullet forwards `agent_id` as the discriminator that
  *would* attribute a firing to one agent, and the first read of real payloads puts that forward
  claim in doubt.
  **The observation, and it is an observation rather than a settled measurement.** Five
  `SubagentStop` firings inside one session carried five DISTINCT top-level `agent_id` values,
  none matching the stable identifier the same payloads' `background_tasks` array reported for
  the one live dispatched agent. If that holds, the field is per-*firing* rather than
  per-*agent*.
  **The tracked record cannot settle it.** `.workflow/subagent-stop-liveness.log` records the
  payload's top-level key set and no values, by the no-values privacy ruling, so every line
  carries `agent_id` as a key and none as a value. Settling it means reading raw payloads, which
  that ruling holds operator-class.
  **STANDING OPERATOR RULING carried into spec: it lands as *state the doubt, drop the forward
  claim*.** The raw-payload read was NOT granted, and the amendment is authored so that no delta
  needs one; needing one to proceed is an escalation rather than a licence.
  **DISTINCT from `subagent-stop-payload-background-tasks-read`**, which is Done and whose
  finding is what the harness ENUMERATES; this is about what one field IDENTIFIES.
  **Cost while deferred:** low and forward-looking — nothing is wrong today, and the first thing
  that goes wrong is a later variant building an attribution grammar on a field that cannot
  attribute, which reads as a working feature until two agents are compared.
  Filed 2026-08-23 to the gap inbox by build; promoted to the deferred pool 2026-08-24 at
  `shell-gate-tail-port-and-completion-oracle`'s close; promoted to a feature 2026-08-24 at
  spec, which re-corroborated the `session_id` half first-hand across two checkouts.

- **delegation-provenance-floor** [spec: SPEC-provenance-floor.md] — a dispatching session can
  narrate findings from a subagent whose output it never received, and nothing reds.
  recurrence: delegation-provenance-floor 2026-08-18 2026-08-19
  **Four attested instances, every one self-caught, and they are not all one shape.** Three are
  a dispatcher relaying a return that never arrived — the fabrication sitting in the dispatcher's
  own REASONING rather than in quoted child text, so a floor checking relayed output alone misses
  it. The fourth is an invented IDENTIFIER: a fabricated short hash written into the survey
  record to make a dated census look precise.
  **The failure leaves no signature in the text**, which makes it a gap rather than a lapse: the
  prose reads identically whether the return was held or invented.
  **Distinct from its two neighbours:** `dispatch-cited-evidence-unverified` is a sweep's
  *quoted evidence* being untraceable, `dispatch-unreadable-target-fallback` is a *child*
  fabricating a verdict on an unreadable target; this is the *parent* relaying a return that
  never arrived.
  **TAKEN 2026-08-24, operator-ruled at scope's threshold proposal: the SIX holds are DISCHARGED
  rather than overruled.** Every one stood on the surface criterion and this iteration is that
  surface; the receiving-side half having landed as delegation-kit/SPEC.md §Resume journal, only
  the doctrine line was open.
  **The entry's own concession does not fully survive, and the amendment says where.** It held a
  gate very likely unbuildable. That is true of the relayed-return shape and false of the
  minted-identifier shape, whose predicate already exists in this tree one field away from where
  the attested fabrication landed — so the unit ships the doctrine line AND an oracle rather than
  conceding enforcement-first wholesale.
  **Cost while deferred:** it lands on exactly the dispatches delegation is pre-authorized for,
  and it defeats normal trust rather than merely being wrong — a relayed finding is the signal a
  reader uses to decide a claim has already been checked.
  Filed 2026-08-13 by close, draining the gap inbox on operator direction to file, not promote;
  promoted to a feature 2026-08-24 at spec, which added a fifth instance from its own dispatch
  and recorded it as one where the floor worked.

- **stage-journal-contract-unoracled** [spec: SPEC-stage-journal.md] — a granted resume journal
  that is never written is indistinguishable from a session that had nothing to say, so the
  durable narration channel fails silently.
  **Observed, not hypothesised.** At one iteration's validate the dispatch granted a journal path
  spelled absolutely into the main checkout, and at stage end the file did not exist — while
  every other stage that iteration wrote its own, six of six.
  **The mitigation that failed is the one the protocol itself prescribes.**
  `delegation-kit/templates/agent-execution.md` already carries the caveat that a background
  agent's sandbox may block the write and that the agent then falls back silently to its return
  message. The granted path was named absolutely in the main checkout — the caveat's own remedy —
  and it failed anyway.
  **The cost landed on precisely the case the journal exists for.** That was the one stage of the
  iteration interrupted mid-work; had the session died rather than merely ending its turn, its
  reasoning would have been unrecoverable.
  **Distinct from the two nearest entries.** `stage-completion-unattested` is the stage's own
  **deliverable** absent behind a live entry stamp; this is the narration channel, and a stage
  can lose either without the other. `delegation-provenance-floor` is a parent relaying a
  child's return that never arrived — its body RECOMMENDS the shape this entry takes, so it is
  prior art rather than a subsumer.
  **The fork is ruled in the amendment, and the enabling move is a derivation rather than an
  assertion**: a path granted ad hoc in a prompt is unreachable by any oracle, so the path
  becomes computable from the stage before anything can assert it.
  **Cost while deferred:** every dispatched stage can lose its reasoning silently, and the loss
  is invisible until someone needs it.
  Filed 2026-08-19 by close from the gap inbox, filed by the lead because the absence is
  observable to the dispatcher and not to the session; promoted to a feature 2026-08-24 at spec,
  which probed the sandbox caveat against a live worktree dispatch and found the write capable.

## Technical Debt

- **worktree-reclamation-cause-falsification** [precondition-ok: upstream-not-blocked] — nothing
  has established WHY an agent worktree survives, so no reaper can be designed against the
  mechanism rather than a guess at it.
  **The valve's cause, stated because a terse tag is not an audit trail.**
  `check-queue-prose-precondition` reds this entry on its sequencing paragraph, reading "upstream
  of" and "blocked on" as a forward precondition this entry waits behind. The direction is
  inverted: this entry is the BLOCKER, holds no precondition of its own, and is pickable first by
  construction — which is what the paragraph says. Declaring a blocker tag would assert a blocker
  that does not exist, and rephrasing past-tense would falsify an unrun experiment, so the valve
  is the only one of the gate's four remedies that is true here.
  **The hypothesis, unrun.** Reclamation may be tied to the DISPATCHING session's lifetime
  rather than the child's, so a parent that ends abnormally strands the child's worktree. Cheap
  to falsify: dispatch an isolated read-only agent, end the parent abnormally, read what stays
  registered.
  **SEQUENCED FIRST inside this iteration, and the ordering is the reason it was taken.** It is
  UPSTREAM of `agent-worktree-boundary-disposition`, whose second refusal ground is this same
  hypothesis written as an unverified candidate, and it is upstream of the seam question inside
  `turn-end-liveness-exit-two-conflation`'s third disposition. Both read its answer, so the
  experiment runs before either amendment is authored rather than at build — a spec session
  authoring against an unrun hypothesis would be designing against the guess this entry exists
  to retire.
  **DISTINCT from `agent-worktree-reclamation-unenforced`, which a prior iteration completed.**
  That unit bought the residue's DECLARATION, its UNREACHABILITY to the two index walkers, and
  a per-iteration BOUND at the boundary refusal — every delta hypothesis-independent by
  construction, which is why it landed without the cause and why this is not a re-filing of it.
  This buys the cause: none of those deltas needed it and none of them establishes it.
  **What the answer buys:** it separates a dispatch-protocol fix from a sweep. If reclamation
  is parent-lifetime-tied the fix is in how a dispatch ends; if it is not, the fix is a reaper
  — different owners, different seams, and nothing today says which.
  **Class:** it runs an experiment and mints no name and no gate, so the litmus makes it debt;
  what it produces is the input two sibling amendments are blocked on.
  Filed 2026-08-24 to the gap inbox by spec with an explicit distinct declaration; drained
  2026-08-24 at close, which verified the completed sibling in `## Done` and found the upstream
  relation the filing did not name; promoted 2026-08-24 at the next scope on an operator ruling.

- **no-port-cause-class-generalization** — one file's `# no-port:` cause is true of a whole
  `scripts/` class, and declaring the class moves the completion predicate by roughly a sixth.
  **The cause, and the two halves it is made of.** `scripts/measured-claims.sh` declares
  `# no-port:` on CLAUDE.md §The provenance seam, and the declaration is a conjunction: (a)
  `scripts/` rides no installer payload, `pack-installer.sh` assembling from the kit roots
  alone; (b) the file's keys are this repo's private claim vocabulary, so porting it ships that
  vocabulary in every adopter's binary.
  **OPERATOR-RULED 2026-08-24, relayed through the lead at the close of
  `port-remainder-disposition-and-worktree-reclamation`: the cause generalises on half (b)
  ALONE.** Declare the config-and-vocabulary files, which hold the private claim vocabulary;
  leave the mechanism files owed, since they hold none. The operator's ground: the cause's real
  basis is the provenance seam, and TRAJECTORY.md's 2026-08-14 ruling already ACCEPTED the
  payload cost for mechanism, so declaring on half (a) alone would contradict a trade that
  ruling took. NOT a reversal, demotion or re-scoping of it: its stated subject is gates under
  `scripts/`, and it records that `scripts/` now keeps no gate script at all, so every owed row
  this touches falls outside that subject.
  **Re-measured at scope rather than carried:** half (a) holds for every owed row under
  `scripts/`; half (b) splits them fifteen vocabulary rows against ten mechanism rows. Re-derive
  that split by reading each header rather than citing it — the class is real and NOT uniform,
  so one undifferentiated sweep over-declares.
  **What is NOT ruled and stays with this iteration:** whether the declarations land as one
  sweep or per owning entry, and the cause text each row carries. Class: it declares an existing
  marker on existing files and mints no name and no gate, so the litmus makes it debt.
  **What the delay costs:** the predicate keeps about a sixth of its rows ruled-in-principle and
  unruled-in-fact, so `--tree`'s owed count overstates the remaining work and every cut composed
  off it is mis-sized by that margin.
  Filed 2026-08-24 to the gap inbox by spec and held there deliberately rather than asked
  mid-iteration, since ruling the class would have moved the owed column underneath the
  amendment then landing; drained 2026-08-24 at close, which re-measured both halves and
  escalated the ruling; promoted 2026-08-24 at the next scope once the operator ruled it.

- **icebox-candidate-eligibility-unapplied** — the eviction worklist lists rows the icebox
  eligibility rule already forbids, so every close re-derives the same exclusions by hand.
  **The split is backwards, and that is the finding rather than a calibration complaint.**
  queue-kit/SPEC.md §The icebox tier states that a roadmap-tagged entry is not icebox-eligible
  as a hard rule with structural enforcement behind it — `check-roadmap-fresh` reds a one-line
  entry that cannot carry a `roadmap-summary:` — while the `queue-index --icebox-candidates` arm
  filters on age and low-cost-opener alone. The mechanically-decidable half of eligibility is
  unapplied while the judgment half is applied by hand, which is the wrong split.
  **DISTINCT from any calibration question.** queue-kit/SPEC.md deliberately rules the age and
  cost filters non-load-bearing because miscalibration only lengthens the list; that reasoning
  does not extend to a rule the SPEC states as binding either way.
  **The fork the build stage rules:** one predicate in the emitter that drops the row, or a
  printed ineligibility reason per row if listing-then-explaining is preferred to filtering. The
  second keeps the worklist a census and costs the close a read; the first makes it a pick list
  and hides the tier's reasoning from the session that most needs to have read it. Class:
  neither shape mints a governed name or a gate, so the litmus makes it debt.
  **Measured across four closes rather than predicted, and the last reading is the one that
  settles it.** Two closes each ran the worklist and got a byte-identical five rows, 100%
  ineligible — three by the roadmap-tag rule and two on the live-promotion-trigger clause. A
  third ran it after the age floor dropped and got 27 rows with the same three ineligible, an
  11% rate that was the hold's ground. The fourth measured 8 rows, all 8 ineligible, and scope
  re-ran the emitter at the next boundary and got the same 8. The five non-roadmap rows are each
  held by a **live unbuilt queue slug**, which the 2026-08-23 operator narrowing makes a live
  promotion trigger — and a slug-liveness test is exactly what the queue adapter already
  implements for `check-task-names` and `check-roadmap-fresh`. So **both** halves of the
  eligibility rule are mechanically decidable, the judgment residue the 11% reading rested on is
  not judgment at all, and the worklist's false-positive rate is 100% against a predicate the
  emitter could have computed. Re-derive these figures rather than citing them.
  **TAKEN 2026-08-24, operator-ruled at scope's threshold proposal on three recurrence dates**,
  one hold after the lead came closest to taking it. The rate-collapse ground that hold stood on
  is answered above; the surface cost was stated to the operator rather than elided — this lands
  on the queue emitter and not on this iteration's turn-end files — and the ruling took it
  anyway, against TRAJECTORY.md §The closed rulings' 2026-08-23 operator action on the pool's
  measured 8:1 intake-to-exit ratio. The bundling with `deferred-cost-class-opener-vocabulary`
  on one surface stands, and is now an argument for reading that entry beside this one rather
  than against taking it.
  Surfaced 2026-08-19 at the `takeable-tier-batch-and-installer-noop` close's icebox eviction
  review and filed to the gap inbox there; promoted to the deferred pool 2026-08-20 by the next
  iteration's scope, and to this iteration 2026-08-24 on the recurrence rule.

## Deferred

- **native-gate-port-remaining-corpus** [design-pending] [roadmap: now/reliability]
  — the whole battery onto the binary, and the shell surface down to its residue.
  roadmap-summary: The gate battery becomes a native binary — precompiled, or built from source.
  The entry demotes at build rather than moving to `## Done`: it is the **whole corpus**, the oracle
  below still counts gates owed, and a Done move would assert a finished port and silently drop it
  from the **public** roadmap projection, which reads `[roadmap:]` tags off live entries.
  **Operator-ruled 2026-08-09: complete the port, ASAP** — the ruling, its grounds and its
  supersession of the 2026-08-06 measurement-locus clause are [TRAJECTORY.md](TRAJECTORY.md)
  §PRIORITY DIRECTIVE's, with the scope, both install paths and the bootstrap residue; this entry
  is the work, and designing the bootstrap is still its. It inherits gate-sdk/SPEC.md §Porting a
  gate to the binary substrate for the procedure and §Consumer payload for the payload rule.
  **Two objections answered:** wall-clock is the weaker case (the win is retiring the shell
  *sources* the payload carries), and the toolchain-free arm rides the pre-compiled path. Its old
  third clause — `powershell-installer-surface` shrinking to the bootstrap — is DELETED as measured
  false 2026-08-24; that entry's re-scope records the ~350 uncovered `init.sh` lines.
  **Every closed cohort and cut — members, counts, holds, grounds, price — is recorded at
  gate-sdk/SPEC.md §The first cohort, and the rule that selects the next, so this entry states
  what remains.** Cut widths are ruled **per cut and never inherited**, members are **selected by
  running** `port-blockers.sh --group`, and the size arm is **permanently** exhausted, so the
  budget arm is the only composer left.
  **The residue's three-way shape is SPENT, and both oracles were re-run at this boundary rather
  than cited.** The 2026-08-23 ruling retired the permanently-shell class outright; `--group` now
  reads 106 scanned, **0 owed and 0 takeable** (none permanent, none held — the battery is
  finished) and `--tree` read 153/0/0/**153 owed**, superseded 2026-08-24 when
  `port-oracle-instrument-self-disposition` landed the first three declarations: it now reads
  153/2/1/**150**. Re-run it, never cite. Only `--tree` is the predicate; `--group`'s zero is a
  finished battery not a finished port; `ported-gate-members`, a registry walk, answers neither.
  **Cost while deferred:** large and known — the **owed** remainder (the trailer's own arm,
  never the unported count) plus the runners and the install-lifecycle layer; since the
  2026-08-14 born-native default (TRAJECTORY.md §The closed rulings) a gate landed meanwhile no
  longer adds shell to it. Not a single-iteration delta; scope owns the decomposition, and the
  criterion-relaxation question is closed at gate-sdk/SPEC.md §The port-candidate criteria — an
  ordering signal, never an eligibility screen. Both battery entries said the port subsumes
  them; `gate-battery-parallel-execution` closed on that, leaving `gate-battery-result-cache`.
  **TEN cuts are delivered**, each one's members, findings and price staying at its own
  §check-<gate> section; the last two are the tail's first two, which emptied the takeable tier.
  **The PRIORITY DIRECTIVE has yielded THREE times, each for one named iteration alone and each
  operator-ruled through the lead — 2026-08-22; 2026-08-24 to the two entries already committed to
  that boundary; and 2026-08-24 to this entry's own remainder decomposition. NONE is a reversal,
  demotion or re-scoping**: the directive and its tail ordering stand as ruled, a yield spending one
  iteration's turn and never the sequence's claim on the next.
  Filed 2026-08-06 at spec; re-scoped 2026-08-09 by close on operator direction, under the
  direct-filing exception; cohorts ruled 2026-08-11 and 2026-08-12 at scope. Since then it is
  promoted at spec and demoted at build once per increment on the entry-outlives-the-amendment
  branch, each cut's record staying at its SPEC section.
  **Tail ruled 2026-08-23, re-recorded 2026-08-24** (TRAJECTORY.md §PRIORITY DIRECTIVE): no member
  is permanently shell; the sequence is `battery-runner-port`, `shell-gate-tail-port`, then the
  bootstrap — the first two LANDED, and the bootstrap is now ONE member,
  `powershell-installer-surface`, `install-step-relocation` having retired as mooted.

- **port-oracle-instrument-self-disposition** [design-pending] — the tool that measures
  the completion predicate is still shell; its disposition is ruled and declared, and its PORT
  is what remains.
  **The disposition increment LANDED 2026-08-24** in the port-remainder iteration, merged into
  gate-sdk/SPEC.md §port-blockers, §The non-gate arm, §gen-pre-commit and §The
  `# graph:` manifest. `gate-sdk/bin/port-blockers.sh` declares `# port-until:` against THIS slug;
  `scripts/measured-claims.sh` — the value of `CANON_KIT_MEASURED_CLAIMS_CMD`, in a directory
  `pack-installer.sh` never packs — declares `# no-port:` on the provenance seam instead, which is
  where the two instrument files parted. `--tree` moved 153/0/0/153 to 153/2/1/150 with the hook
  regenerated behind it. The entry DEMOTED rather than moving to Done because a Done move would
  strand that declaration at a dead slug and red `check-gate-exemption-tasks`.
  **Deliverable — the port itself:** `port-blockers.sh` becomes a **bridged-arm table member** of
  the non-gate arm class, the family gate-sdk/SPEC.md §The non-gate arm already forces for a tool
  reading the gates dir, the kit roots, the prune set and the program floor. Route ruled, port
  unbought.
  **Why [design-pending]:** the arm's flag spelling is deliberately UNMINTED — a spelling written
  ahead of its caller is the reservation the closed-roster rule refuses — so the porting unit mints
  it WITH its reader, which is an amendment's act rather than a mechanical one.
  **Two constraints the porting unit inherits rather than discovers**, both recorded at §The
  non-gate arm. (i) `measured-claims.sh` reads the `--tree` trailer by spawning `bash` today and
  must move to the arm IN THE SAME COMMIT, or the consumer's claim oracle exits 2. (ii) Most of the
  substrate exists — the registry walk, the prune resolution and the `--tree` corpus rule's
  compiled face are already shared crate mechanism, so what the port owes is the header-block read
  and the three arms' trailers. What exists, what is owed, and the promote-or-duplicate choice are
  enumerated at §The non-gate arm and are deliberately not re-listed here.
  **The reflexivity is priced and is not a problem:** the arm's shell implementation and the arm's
  own row leave the tree in the same commit, so the measurement is not a fixed point and its last
  act is to remove itself from its own corpus. The arm reads headers, not behaviour, so there is no
  fixed point to converge on.
  **DISTINCT from `native-gate-port-remaining-corpus`**, whose deliverable is the remaining corpus
  and its demotion rule; this is one named file with a route already ruled.
  **Cost while deferred:** low and legible — the tool stays shell, so the port is not complete
  however the owed column reads, and `--tree`'s held tier carries exactly this one row. The
  declaration already bought the legibility half: the row reads as named work, not unexamined work.
  Filed 2026-08-24 by build to the gap inbox, promoted at
  `shell-gate-tail-port-and-completion-oracle`'s close, ruled at spec in `SPEC-owed-column.md`,
  and demoted 2026-08-24 by build once the disposition increment had landed.

- **validate-suite-wall-clock-unowned** [design-pending] — the validate run is about seventeen
  minutes and no entry owns the sixteen of them that are not the gate battery.
  **Measured 2026-08-23 from `.tmp/validate-*.log` mtimes:** five serial smoke suites carry
  about 80% of the run — `installer_smoke` 5.6m, `consumer_smoke` 4.0m, `upgrade` 1.9m, `demo`
  1.8m, `agents_md_smoke` 1.4m — against a 52s gate battery.
  **`run-validate.sh` is serial by construction and says so**: its spine declares "run each
  suite foreground" (evidence-kit/SPEC.md §bin/run-validate.sh) and the loop parses and diffs
  each suite before the next starts. Nothing instruments per-suite cost either — the figures
  above came from file mtimes, which is the tell.
  **Why `[design-pending]`:** the spine's ordering is load-bearing in one direction and not the
  other. The manifest fold is explicitly after the last suite, so evidence accumulation
  parallelizes trivially; the pre-hook and any suite that writes the tree do not. Which suites
  are genuinely independent is unmeasured, and a wrong answer corrupts evidence rather than
  merely running slow.
  **DISTINCT from the two battery entries, and from the arm already discharged.**
  The two battery entries own the battery's own cost and never reached the suite roster;
  `battery-runner-port` discharged `gate-battery-parallel-execution`, now Done, while
  the other stays parked. `check-crate-arms`'s source-stamp cache is **landed** — the gate carries
  its `.green` stamp today — and ruled at gate-sdk/SPEC.md §check-crate-arms, so the cargo half
  of the original finding is discharged and this entry is the suites alone.
  **Cost while deferred:** about sixteen minutes per validate, paid serially, on a stage every
  iteration runs at least once.
  Filed 2026-08-23 to the gap inbox by the post-close consult of
  `leak-guard-and-assertion-meta-gate-port`, and promoted at the next scope intake the same day,
  with the crate-arms half re-verified as landed and dropped from the entry.

- **overlay-only-oracle-grants-uncommitted** [design-pending] — four oracles this tree runs
  constantly are granted only by an untracked overlay, so a fresh clone re-buys every one.
  **SPLIT from `session-mechanic-grants-uncommitted` on 2026-08-23 at spec, lead-authorized**,
  as the first application of the split criterion `SPEC-entry-split.md` mints: that entry
  carried a ruled design and this unruled half, two deliverables taking two dispositions on
  one slug. Its ruled half promoted the same day; this one did not, which is the test.
  **The set, unchanged from the parent and not to be re-derived:** `cargo test`,
  `cargo build` / `clippy`, the release binary, and `gh auth` — each read-only or a build of
  this tree's own crate, none of them reaching the network or a foreign repo.
  **Why `[design-pending]`:** what is open is not whether the four are safe but **which
  surface should carry them**. A committed `.claude/settings.json` grant is the obvious
  answer and is **operator-class** under the 2026-08-22 ruling (TRAJECTORY.md §The closed
  rulings), so no stage session can land it; a guard-kit rule is landable but would encode a
  Rust toolchain's command shapes into a kit whose ruleset admits **no project's toolchain**
  by its own stated clause. That tension is the design question, and neither horn is costed.
  **Distinct from the parent**, whose subject is a redirect target no settings rule can
  express, and from `guard-grant-review`, whose subject is narrowing grants that already
  exist rather than minting ones that do not.
  **Cost while deferred:** every fresh clone and every dispatched session on one pays an
  out-of-band decision per oracle, and the cost is invisible on the operator's own machine
  because the overlay hides it there — which is the reading that makes it worth filing
  rather than absorbing.
  Filed 2026-08-23 at spec by the split above; never promoted, never held on merit.

- **grant-argument-bounding-mechanism** [design-pending] — two committed grants reach a destructive
  form and no allow-glob narrowing can stop either.
  **Both findings are verified rather than surmised**, at build 2026-08-22 by a read-only worktree
  sweep over all 105 committed `Bash(` entries.
  **(1) The `.tmp/` rm family escapes the scratch dir.** `Bash(rm .tmp/*)`, `Bash(rm -f .tmp/*)`
  and `Bash(rm -rf .tmp/*)` each match `rm -rf .tmp/../.git`, and reach any irreplaceable untracked
  local file — `BRIEF.local.md`, `OPS.local.md`, `ENV.local.md`. `guard_rule_rm_tracked` fires only
  on a TRACKED target, so nothing in the guard covers this.
  **(2) `Bash(git rm -q *)` reaches `git rm -q -f <modified file>`**, destroying uncommitted work
  irrecoverably; the sweep demonstrated it in a scratch repo rather than reasoning about it. No
  guard rule covers that either — `guard_rule_rm_tracked` matches a bare `rm` only.
  **That grant is LOAD-BEARING, which is what makes this hard.** `guard_rule_rm_tracked` STEERS
  every tracked-file deletion INTO `git rm -q`, so narrowing the grant taxes a mechanic the guard
  itself mandates. It is not a grant anyone may simply delete.
  **Why no allow-glob fixes it.** A Bash rule's `*` "matches any sequence of characters including
  spaces" (vendor permissions doc), so it spans `/` and `..` and cannot bound an argument. That
  same doc warns outright that "Bash permission patterns that try to constrain command arguments
  are fragile", and offers exactly two remedies: deny rules, or a PreToolUse hook.
  **Candidate shapes, none costed:** a committed `deny` list, which outranks allow but inherits the
  fragility the doc names; extend the guard's rules, the vendor's own remedy and the one this repo
  already owns the hook for; or accept and declare, recording the reach rather than removing it.
  **Why [design-pending]:** the three trade differently against a boundary this repo has never
  used — `.claude/settings.json` carries no `deny` list at all today — and the first is an
  operator-class edit besides, so the mechanism choice decides who may even land it.
  **Cost while deferred:** two live paths to irrecoverable data loss, one of them reachable by a
  single mistyped path inside a grant every session uses for routine scratch cleanup.
  Filed 2026-08-22 by build, split out of `guard-grant-review` on the lead's ruling that choosing
  the mechanism is design work and scope-gated intake makes it a costed Deferred entry by default.

- **grant-path-traversal-exposure** [design-pending] — the committed script-runner globs match a
  traversing path, and whether that is in the narrowing ruling's scope is unsettled.
  **The exposure, stated plainly:** `Bash(bash */checks/check-*.sh)` matches
  `bash ../../evil/checks/check-x.sh`, because a Bash rule's `*` spans `/`. Its siblings carry the
  same shape — `bash */bin/run-*-tests.sh`, `bash */gate-tests/*.test.sh`, `bash */smoke/install.sh`
  and the `drift-kit/kpis/*.sh` pair. Verified at build 2026-08-22.
  **Ruled OUT of scope for the 2026-08-20 narrowing, by the lead 2026-08-22**, on the reading that
  that ruling is scoped to data-loss forms and that declining to widen a recorded ruling is the
  conservative move. Recorded here so the exposure is not lost along with the decision.
  **Why it is genuinely weaker than a data-loss finding:** reaching a destructive script needs a
  second precondition the allowlist cannot evidence — a hostile script must already exist at a
  matching path. `scripts/bash-guard.sh` also blocks `bash .tmp/…` outright, closing the one path a
  session may write to freely, and routes it through `scratch-run.sh`, which resolves the real path
  and refuses anything outside the scratch dir.
  **Why [design-pending]:** whether "destructive form" covers a code-execution class at all is a
  question about the 2026-08-20 ruling's own scope, and only the operator may widen it. The
  engineering question — whether these globs can be re-spelled without breaking the battery they
  exist to run — is untouched and unbought.
  **Cost while deferred:** low and precondition-bound, but it spans the whole script-runner
  surface rather than one grant, so a later change making a matching path writable would arm the
  whole family at once, and would do it without a signal anyone reads.
  Filed 2026-08-22 by build on the lead's ruling; surfaced by the same sweep that produced
  `grant-argument-bounding-mechanism`, which is the data-loss half of the one audit.

- **config-bridge-resolution-cost** [design-pending] — the array-knob config bridge still costs
  about 640 ms on every invocation that resolves it, and no entry owns the residue.
  **RE-SCOPED 2026-08-23 at `battery-runner-port`'s close, on a lead ruling: correcting a false
  claim in an entry is not a descope.** Three of this entry's four load-bearing statements moved
  under `df42c13b`, and the retired ones are deleted rather than annotated.
  **Its blocking design question is ANSWERED, by an executed fact rather than a preference.** It
  asked whether one knob's resolved value may legitimately differ between two members in the same
  run. It may not: resolution is **member-independent**, verified by reading the resolver rather
  than the SPEC — `gate-sdk/lib/gate.sh`'s `_gate_knob_emit` takes the gate name as a parameter and
  uses it at exactly one site, a refusal message, so no resolver reads the requesting member.
  Batch 1 then built on that property. The tag nevertheless STAYS: canon-kit/SPEC.md §The amendment
  lifecycle makes `[design-pending]` a section-membership invariant ("every entry in the set carries
  the tag"), so it comes off at promotion and not on a closed question — probed, not assumed,
  since removing it reds `check-amendment-queue` outright.
  **Its third candidate shape is BUILT** — "resolve each kit's declared-knob set once per run
  rather than once per knob" is what `gate_knob_env_set` and `_gate_knob_kit_emit` now do, one
  subshell per owning kit. The other two shapes are untaken and stay open.
  **What actually remains, measured at this close, best-of-three warm.** A single-gate run of a
  gate that does almost no work (`check-core-files`) costs **640 ms**, essentially all of it one
  bridge resolution — that is the floor every bridged invocation pays. `gen-pre-commit.sh --emit`
  is **4104 ms**, down from the 6119/6203/6243 ms this entry used to carry. The old
  92%-of-`check-graph` framing is retired with those figures and must not be revived.
  **Why it does not close.** 4104 ms is still the largest non-cargo single cost in the tree, and
  the 640 ms floor is paid by every hook regeneration, every `install-hooks.sh`, and every
  single-gate run a session makes while iterating. What the batch removed was the per-knob
  multiplier, not the per-kit subshell.
  **Nothing else owns the residue.** The surviving `bash` spawn is owned by gate-sdk/SPEC.md
  §gen-pre-commit, where its disposition is recorded, and was ruled 2026-08-23 not to fall to this
  port — only its price did.
  **Cost while deferred:** every bridged invocation pays 640 ms it cannot avoid, and the cost
  scales with the number of owning kits rather than with the number of members, so a new kit
  raises the floor for everyone while a new member no longer does.
  Filed 2026-08-21 twice, by spec and by build; promoted at `graph-port-and-config-seam`'s close;
  re-scoped here after the batch landed, with every retired figure deleted.

- **installer-graph-artifact-literal** [design-pending] — `installer/lib/init.sh` spells the graph
  artifact's path as a literal where a resolver owns it.
  **The two sites are exact**, re-verified at the drain: `installer/lib/init.sh` line 311 (the
  `GENERATED` roster) and line 317 (the emit redirect) both write `$GATES_DIR/CHECK-GRAPH.html`,
  while `gate-sdk/lib/gate.sh` resolves `GATE_SDK_GRAPH_ARTIFACT` for the same file.
  **DISTINCT from `upgrade-smoke-graph-artifact-literal`**, which the graph port discharged: that
  entry was scoped to `gate-sdk/bin/upgrade-smoke.sh`, and this is a second site the port's caller
  sweep surfaced.
  **It is also weaker, and the weakness is the whole cost question.** `init` writes a freshly
  initialised consumer whose config seam `init` itself authors, so the default holds by construction
  on a first run. The live exposure is a **re-run** over a tree whose adopter has since set
  `GATE_SDK_GRAPH_ARTIFACT`: `init` would write the default path and record it in the manifest while
  the consumer's own gate looks elsewhere. The `GATE_SDK_GATES_DIR` half is already handled,
  `$GATES_DIR` being init's own variable.
  **Why `[design-pending]`:** the fix is not obviously "resolve the knob" — `init` runs before the
  consumer's gate library is necessarily sourceable, which is why it literalises at all, and whether
  the installer may reach into a vendored kit's resolver at that point is an installer-layering
  question this entry does not settle.
  **Cost while deferred:** low and bounded to the re-run case, but it is exactly the standing
  invitation the discharged sibling entry named on its way out.
  Filed 2026-08-21 by build into the gap inbox; promoted at `graph-port-and-config-seam`'s close,
  which re-verified both line numbers and the resolver against the tree.

- **amendment-reader-roster-undercount** [design-pending] — an amendment's reader/caller roster is a
  dated measurement presented as a roster, and it undercounted twice in one iteration.
  **Both misses were the same spec session, one iteration.** `SPEC-graph-port.md` delta 5 scoped ONE
  caller of a file it deleted where the tree held twelve — nine kit `smoke/install.sh` arms,
  `context-kit/smoke/agents-md.sh`, `bin/upgrade-smoke.sh` and `installer/lib/init.sh`.
  `SPEC-stamp-head.md` delta 5 asserted "exactly one parser breaks" and named it, missing FOUR
  end-anchored `grep` assertions in `lifecycle-kit/smoke/install.sh` plus 7 of 10 cases in
  `check-stage-evidence.test.sh` that flip to "malformed stamp" and stop reaching what they assert.
  **One shape, and the spec method structurally cannot see it.** The hit is BUILT BY INTERPOLATION —
  the date is a shell variable — so a literal-string, field-count or length sweep returns nothing;
  and the breakage is CONSEQUENTIAL rather than assertional, so "the tests still pass" reads true
  and is false. Re-verified at the drain against `git diff` on both files: every named site is there
  and every one was rewritten by the build unit that found them.
  **This is a method finding about a STAGE, not a defect count.** Delta 5 itself says the build unit
  re-runs the sweep against the tree rather than trusting the roster, and that sentence is what
  caught both — carried by an individual amendment rather than by any contract that outlives it.
  **Candidate deliverables, none ruled:** make the re-sweep obligation a build-stage contract line
  rather than a sentence an amendment may or may not carry; or an oracle for the interpolated-hit
  class that greps the assertion IDIOM rather than the literal; or drop the roster from amendments
  and keep only the sweep instruction, since a roster nobody may trust is context cost with a
  false-confidence coupon attached.
  **Why `[design-pending]`:** the three are not variants of one fix — a template edit, a new gate,
  and a deletion that removes a real aid — and choosing needs the counter-case this iteration did
  not supply: an amendment whose roster was RIGHT and whose reader was thereby spared the sweep.
  **Cost while deferred:** a spec session pays for a census the build session must buy again, and
  the roster's authority runs inversely to its accuracy.
  Filed 2026-08-21 by build into the gap inbox; promoted at `graph-port-and-config-seam`'s close.

- **close-triage-log-reclaim-loss-window** [design-pending] — the close-stage triage of a capture
  log reads it and later truncates it as two separate acts, and anything appended between the two
  is lost with no trace.
  **The window is in the templates, not in one session's sequencing** — re-verified at this drain
  by reading both: `guard-kit/templates/close-triage.md` step 5 and
  `drift-kit/templates/close-knowledge.md` step 3 each prescribe a read-and-triage pass and then
  name `: > .workflow/<log>` as the reclaim, with the whole triage in between.
  **Measured, not predicted:** at the `wide-budget-batch-and-hold-declaration` close the
  diagnosing sweep recorded `.workflow/prompt-friction.log` growing 688 -> 694 lines *while it
  read*, so at least six fall-throughs from a concurrent session were then discarded by the
  reclaim.
  **DISTINCT from the gap inbox**, whose lifecycle-kit/SPEC.md §The committed gap inbox answer is
  merge=union on a **committed** surface: these two logs are gitignored per-clone capture with no
  merge semantics available to them, so union is not the fix and the shape of one does not
  transfer. Distinct too from `scan-prompts-truncation-quote-desync`, a per-line 500-character
  truncation defect in parsing the log rather than a loss of whole lines from it.
  **Why `[design-pending]`:** three shapes and none ruled — truncate to the byte offset actually
  read rather than to empty; rotate rather than truncate; or state in both templates that the
  reclaim is lossy under concurrency and let the KPI carry the caveat, which is the cheapest and
  buys the least.
  **Cost while deferred:** a lower bound that is already declared a lower bound gets quietly
  lower, and the loss is invisible — nothing records that a truncation discarded unread lines, so
  the class cannot be measured after the fact, only prevented.
  Surfaced 2026-08-18 at the `wide-budget-batch-and-hold-declaration` close's tooling-friction
  triage and filed to the gap inbox there; promoted 2026-08-18 by the next iteration's scope,
  draining that inbox.

- **relayed-ruling-provenance-unrecorded** [design-pending] — a relayed operator ruling lands in a
  tracked governance surface as "operator-directed" with no provenance a later reader can check.
  **Found by the harness's own security review**, which flagged the 2026-08-18 `gh` account-restore
  ruling as possible instruction poisoning: the recording session had landed an operator direction
  authorizing an elevated-credential account switch and a push to master, deviating from the local
  release runbook, while its own transcript contained no operator message at all.
  **The authorization was genuine** — the operator selected it through the harness question
  mechanism in the lead session — so this is not an incident report; the flag was correct to fire
  on what it could see.
  **The gap:** under the split-posture lead architecture (lifecycle-kit/templates/lead.md) an
  operator ruling reaches a stage session as a peer message and is landed in the queue as
  "operator-directed". Nothing in the tracked record distinguishes a genuinely relayed ruling from
  one a compromised or confused lead invented, and the reviewing layer cannot see the lead's
  transcript. The class most often relayed this way is the highest-consequence one — credentials,
  pushes, releases, runbook deviations.
  **DISTINCT from `delegation-provenance-floor`**, which is a *parent* relaying a *child's* return
  that never arrived; this is a *child* recording a *parent's* ruling into a permanent tracked
  surface, where the artifact outlives every session that could attest it.
  **Why `[design-pending]`, and it is envelope-class:** three shapes, none ruled — the lead cites
  the authorization channel and turn in its relay and the recording session records that citation
  alongside the ruling; or operator-class rulings are landed by the operator directly rather than
  relayed; or a provenance field on ruling records naming how the authorization arrived, so an
  unverifiable one is visibly unverifiable rather than indistinguishable.
  **The first shape was PRACTISED 2026-08-19, one day after filing, and the trial is worth more
  than the entry's prose.** The lead relaying five rulings stated the authorization channel
  unprompted — three questions put to the operator through the harness's question mechanism, each
  answered by selecting the recommended option, not free text and not the lead's inference — and
  the recording session carried that citation onto each ruled entry, where an auditor now has it.
  **Two limits the trial exposed, both owed an answer.** The citation is still the relaying
  party's own word about a channel the tracked record cannot reach, so it raises the cost of
  inventing a ruling without making an invented one detectable. And it **did not fit twice**:
  `close-entry-baseline-bootstrap-deadlock` and `stage-stamp-ordering-unenforced` took their
  rulings at 0–1 lines of headroom and carry no channel, the displacement class
  queue-kit/SPEC.md §check-queue-entry-budget's split criterion now decides.
  recurrence: relayed-ruling-provenance-unrecorded 2026-08-24
  **A FOURTH SHAPE, AND IT CONSTRAINS THE OTHER THREE: the field must record WHO RULED, with
  "lead, own authority" a first-class value rather than an absence.** All three above record only
  HOW an authorization arrived, and 2026-08-24 fired in the direction none catches — a lead's own
  ruling recorded as the OPERATOR's, corrected at gate-sdk/SPEC.md §check-gate-fail-closed, which
  names the lead and says commit `0f9cc492` misattributes it and is deliberately not rewritten.
  The same iteration also caught the lead attributing a lead ruling to the operator: both
  over-attribute upward, and since "operator-ruled" marks a decision a later session may not
  reverse alone, this direction is a false FLOOR on re-ruling where the filed shape is a false
  CEILING on verifiability.
  **Cost while deferred:** silent and audit-side. Every relayed ruling already in the queue carries
  the same unverifiable provenance, the tracked record is all a later auditor or a fresh session
  has, and it will not red a gate — it surfaces as a security flag on an honest session, which is
  where it surfaced.
  Surfaced 2026-08-18 by the harness security review of `wide-budget-batch-and-hold-declaration`'s
  close and filed to the gap inbox there; promoted 2026-08-18 by the following scope.

- **boundary-wipe-preserve-basename-reach** [design-pending] — the iteration-boundary scratch wipe
  matches its preserve list by **basename at any depth**, so one nested `.gitkeep` makes a whole
  scratch tree immortal and the wipe still reports success.
  `lifecycle-kit/bin/enter-stage.sh`'s boundary block runs
  `find "$tmpdir" -mindepth 1 -depth ! -name .gitkeep [! -name <preserve>…] -print -delete`.
  `! -name` is unanchored, so a `.gitkeep` at any depth survives, its parent's delete then fails
  as non-empty, and every ancestor up to the scratch root survives with it.
  **Attested at this very boundary, not reasoned:** `.tmp/upgrepro/` survived this session's wipe
  intact — two full vendored kit payload copies — because
  `.tmp/upgrepro/{up,base}/package/payload/context-kit/gate-tests/check-memory-off/good/memory/.gitkeep`
  sits inside it.
  **The collision is structural rather than freak:** kit payloads ship `.gitkeep` files, and an
  upgrade-smoke reproduction is a copy of a kit payload, so the shape recurs whenever scratch holds
  one.
  **The failure is silent by design.** The same `# spec:` comment above that `find` suppresses its
  stderr deliberately, and the run reports what it wiped and never what it failed to wipe; `.tmp/`
  is gitignored, so no gate sees the residue either.
  **Why `[design-pending]`:** the preserve contract's intent is stated for the scratch dir's own
  scaffolding (`<tmpdir>/.gitkeep`, "a consumer that tracks its scratch dir's scaffolding"), so the
  fix is to anchor the match to the scratch root's immediate children — but whether
  `LIFECYCLE_KIT_BOUNDARY_PRESERVE` entries are basenames or root-relative paths is a kit contract
  change a consumer inherits, and whether the wipe should *report* its failures instead of
  suppressing them is a second, separable call. That inherited contract is what makes this a
  **unit rather than a one-line patch**, and it is why the 2026-08-19 disposition was to leave it
  filed rather than widen `budget-batch-and-account-identity-kind` onto it.
  **The attesting residue was cleared** once this entry held the evidence: reproducing it costs
  `mkdir -p .tmp/x/y && touch .tmp/x/y/.gitkeep` plus a boundary run, so keeping two vendored kit
  payload copies alive across every future boundary bought nothing the entry does not state.
  **Cost while deferred:** scratch accumulates across iteration boundaries without bound while the
  one mechanism chartered to reclaim it reports success — the boundary reset's own claim is false
  in exactly the case a consumer is most likely to hit.
  Found 2026-08-18 by this iteration's scope at its own entry, from the surviving directory rather
  than from a reading of the code; filed under scope-gated intake rather than fixed in-session.

- **gap-inbox-commit-ownership** [design-pending] — nothing says who **commits** a gap-inbox
  bullet, so a filed bullet can reach the next iteration uncommitted and be carried by whichever
  session happens to stage next.
  **Re-tiered out of the icebox 2026-08-19 on a falsified premise, ruled by the iteration lead
  rather than the operator because the entry's own cost field is what decides it.** Icebox
  membership asserts "no named event is waiting to promote it"; a named event has now occurred.
  **The firing, probed rather than inferred.** `lifecycle-kit/bin/file-gap.sh` appends the bullet
  and never commits — no `git` invocation anywhere in it. `git log -- .workflow/gap-inbox.md`
  shows the `wide-budget-batch-and-hold-declaration` close's first two bullets each landing in
  their own `chore(gap):` commit while the third was written and left in the working tree, so it
  reached a **new iteration's** scope session as an uncommitted modification and was carried into
  history by that session's boundary stamp commit.
  **The distinction the firing turns on:** committing a bullet is **not** its disposition. The
  carrying session had to say so explicitly in the stamp commit's message, because the drain
  contract (lifecycle-kit/SPEC.md §The committed gap inbox) gives a bullet exactly one
  disposition — promoted, fixed inline, or discarded with cause — and none of them is "committed".
  A surface named *the committed gap inbox* whose filer never commits is the gap in one phrase.
  **Why `[design-pending]`:** the shapes differ in who pays. Have `file-gap.sh` commit its own
  bullet — cheapest, but it makes a capture affordance a committing tool, and it contends on a
  shared index with whatever stage session is mid-commit. Oblige the filing session to commit in
  the same turn — no tool change, but it is an instruction, and this iteration is the evidence
  that instructions of that shape get missed. Or rule the carry legitimate and state that the
  next stamping session commits it, which is what happened here by accident rather than by
  contract.
  **Cost while deferred:** small per instance and silent — a bullet outside git until some later
  session notices, invisible to `git log` and to any reader who is not looking at a dirty tree,
  and lost outright if that tree is reset. It is also the one class the inbox's own
  merge=union answer cannot cover, since union protects a committed surface and this bullet was
  never on one.
  Filed 2026-07-25 by close draining its own gap inbox, and born in the icebox rather than demoted
  to it — `git log -S` over the slug returns that one commit, which is the whole of its history
  before this move. Fired 2026-08-18 and re-tiered 2026-08-19 by the scope session the
  uncommitted bullet reached, which is also the session that carried it into history.

- **crate-test-cwd-process-global-race** [design-pending] — the crate's test guard covers the knob
  environment and nothing else, while a second process-global is written by a test and read by
  production paths a sibling test may be running concurrently.
  **Re-verified at this drain rather than taken from the bullet; all three sub-claims hold.**
  `native/src/gates/mod.rs:1333`/`:1338` calls `std::env::set_current_dir` per fixture case inside
  `every_registry_member_declares_the_roots_it_walks`; `std::env::current_dir()` is read in
  production paths of `walk.rs`, `spec.rs`, `emit/trajectory.rs`, `emit/docs_mirror.rs`,
  `gates/docs_nav_reachable.rs`, `gates/assertion_strength.rs` and `gates/docs_link_convention.rs`;
  and `native/src/knobenv.rs:41` declares `ENV_WRITE_APIS = ["set_var", "remove_var"]`, so the
  machine-side roster names the knob environment alone.
  **Why it is latent rather than live.** The one cwd-writing test happens to hold
  `knobenv::lock()` across its whole loop (`mod.rs:1318`), so the existing guard serializes it by
  accident of where the lock was taken. Nothing states that, and nothing stops the next
  cwd-changing test from taking no guard — which is the defect, not the current schedule.
  **Distinct from `crate-test-env-knob-race`, which landed this iteration**: that entry owned
  the knob environment, and its fix (`f2701ff4`) landed the guard this entry calls too narrow. The
  finding
  is what the fix did not reach, so it files as a new defect rather than a recurrence.
  **Deliverable, and why `[design-pending]`:** widening `knobenv` from "the knob environment" to
  "process-global test state" renames the module's charter and its roster, and whether the roster
  should enumerate APIs (`set_current_dir` joining the two) or assert over a class is the open
  choice. A rename that outruns its assertion is the failure mode to avoid.
  **Cost while deferred:** one more 1-in-N false red on `check-crate-arms`, paid by every port
  commit — the same tax `crate-test-env-knob-race` was fixed to remove, re-armed on a second axis.
  Filed 2026-08-18 by close, draining the gap inbox; re-verified by probe, not by prose.

- **baseline-move-stales-evidence-line** [design-pending] — promoting a task and moving a suite's
  baseline is not enough to close: the evidence line already recorded against the *old* baseline is
  stale, and nothing says so until the entry gate refuses a second time for a different reason.
  **The second face of `close-entry-baseline-bootstrap-deadlock`**, which owns the first (close is
  the only stage that may file the blocking slug, and cannot enter without it). That entry's
  candidate fixes all address the first face and leave this one standing, which is why it is filed
  apart rather than folded in.
  **The mechanism.** A recorded verdict is *relative* to whichever baseline was live when the suite
  ran, so moving a suite from `pass` to a slug-carrying `fail` invalidates every line computed
  before the move. `check-evidence-manifest`'s close-entry assertion then still refuses on "no clean
  evidence line" with the promotion and the baseline row both correctly landed — the refusal a
  session reads as the first fix having failed.
  **Attested, not predicted:** hit at this iteration's validate/close boundary. The first refusal
  was fixed by `f8c34c20` + `f5664bbf`; a `--simulate close` recheck still refused, for a second
  reason nothing had flagged, forcing a round trip the recipe would have saved.
  **Re-verified at this drain; both claims hold.** No promote → baseline → fresh-evidence recipe
  exists anywhere in `evidence-kit/SPEC.md`, and `check-evidence-baseline`'s own help
  (`native/src/gates/evidence_baseline.rs:224`) names the line grammar, the liveness requirement
  and the human-commit rule — never the evidence manifest as the other surface a promotion stales.
  **Deliverable, enforcement-first shaped:** document the three-step recipe wherever the
  deferred-known-red path is described, *and* widen that help text to name the manifest, so the
  session that reaches the second refusal is told by the gate rather than by a round trip.
  **Cost while deferred:** one wasted close-entry round trip per iteration that ends non-clean,
  landing on top of the operator interrupt the first face already charges for the same boundary.
  Filed 2026-08-18 by close, draining the gap inbox; the first face stamped as a recurrence there.

- **nested-battery-env-inheritance-invisible** [design-pending] — a suite whose harness re-execs
  batteries inside nested sandboxes inherits the parent environment wholesale, and a contaminated
  scoped run still emits a well-formed `verdict=clean` row.
  **What makes it a real defect rather than a caveat: the contamination is invisible where the
  reader looks.** The manifest row carries pass/fail counts, a fresh `sha256` and correct grammar,
  so nothing in it distinguishes a clean run from one whose nested batteries reddened on a
  dispatch-harness error. The evidence is only in the captured suite log, which the runner hashes
  and never prints.
  **Re-verified at this drain: the mechanism holds.** `installer/consumer-smoke/run-smoke.sh`
  contains zero occurrences of `EVIDENCE_KIT`, and re-execs `bash gate-sdk/bin/run-gates.sh` in the
  scratch consumer at L169, L296 and L302, so any evidence-kit knob set as an env-var prefix on the
  parent `run-validate.sh` reaches every nested battery. A relative scratch path resolves against
  the wrong cwd inside the sandbox and reds `check-evidence-baseline` / `check-evidence-manifest`
  there — a defect of the harness, not of the suite.
  **Attested twice at this iteration's validate**, both times caught only by reading the suite log
  directly rather than trusting the summary line; a full `run-validate.sh` sets no suite-scoping
  env var and is unaffected, so it was used instead.
  **The generalisation, and the distinction that must survive a fix.** The supersede-not-truncate
  manifest mechanics (evidence-kit/SPEC.md §Evidence manifest — a run overwrites only the rows for
  suites it covered) are sound and reusable on their own. It is specifically the **scoped-run
  path** that is unsafe, and only for a suite whose own harness re-execs a battery in a nested
  tree. A fix that throws out the mechanism with the path has overpaid.
  **Deliverable, two candidates and neither improvised:** sanitize `EVIDENCE_KIT_CONFIG_FILE` and
  its siblings inside `run-smoke.sh` before it re-execs into a nested sandbox, or drive
  suite-scoping by something other than an inherited env var. `[design-pending]` because the second
  changes a knob contract and the first names a roster that must not drift from it.
  **Distinct from `consumer-smoke-subset-accounting-verdict`**, whose scoped run produces a false
  *FAIL* a reader can at least see; this one produces a false *clean* a reader cannot.
  **Cost while deferred:** re-recording one suite's evidence stays unavailable in practice — the
  only safe spelling is a full battery — and any future scoped run risks a green row with nothing
  behind it, the one failure the manifest exists to make impossible.
  Filed 2026-08-18 by close, draining the gap inbox; the harness's env handling probed directly
  rather than inferred from the bullet.

- **born-native-omission-accumulation** [design-pending] — the born-native flip attaches
  criterion 5's omission to every future gate, and nothing measures the pile.
  **This is the costed residue of a closed ruling, not a challenge to it.** TRAJECTORY.md
  §The closed rulings records the 2026-08-14 flip to native-by-default; this entry exists
  because that ruling's own cost has no owner, and the gap-disposition rule forbids
  flagging it and moving on.
  **The mechanism, from the surfaces that already state it.** gate-sdk/SPEC.md §The
  port-candidate criteria, criterion 5: a `.gate`-declared member is **omitted** from the
  `gates.list` of a consumer whose host the roster carries no artifact for, arriving as a
  declared absence rather than a broken battery. For a *port* that trade is neutral, since
  the shell form is deleted either way. For a born-native gate it is "a real subtraction
  against the alternative of shipping shell" — the SPEC's own words, written when
  born-native was the exception.
  **What changes under the flip, and it is a rate rather than an event.** Every new gate is
  now born native unless an exception is argued, so the omitted set on an uncovered host
  grows monotonically at the rate the battery grows. `native/targets.list` ships **one**
  target, `x86_64-unknown-linux-gnu`, and states its own grounds: it is the only platform
  any workflow has a runner for. So the uncovered set today is not a hypothetical Windows
  adopter — it is **every macOS adopter**, for whom omit-and-declare is already the normal
  path on day one.
  **Why the existing entries do not cover it.** `gate-binary-target-roster-widening` owns
  widening the roster and `platform-support-ci-matrix` owns the CI leg that is its stated
  trigger; both are about *closing* the gap. Neither owns the *accumulation* the flip
  creates while the gap stays open, and neither would redden if it grew without bound.
  **What is genuinely open:** whether the right instrument is a measurement (a count of
  omitted members per uncovered target, so the pile is visible), a bound (a policy ceiling
  on how far the omitted set may grow before widening is forced), or nothing beyond the
  exception rule the flip's own amendment must state — in which case this entry closes
  against that amendment rather than shipping.
  **Cost while deferred:** an adopter class the project has never observed installing
  silently loses more of the battery every iteration, and the first evidence of how much
  would arrive from a macOS preview adopter rather than from the tree — the exact
  order the launch readiness rule exists to avoid.
  Filed 2026-08-14 at scope, dispositioning the criterion-5 consequence of the same
  session's born-native ruling, under the gap-disposition rule.

- **powershell-installer-surface** [design-pending] — a native Windows install path, and the
  relocation that has to precede it.
  The installer is bash end to end, so native Windows is unreachable:
  `native-gate-binary-port` states the limit in its own words — "`init` hands to bash, so
  Windows stays blocked" — and `platform-support-ci-matrix` covers Windows only as a
  **WSL** CI leg, which is a Linux userland wearing a Windows badge rather than a native
  path. No entry claims this ground today.
  **Ordered by the operator's trajectory pivot 2026-08-03**, whose objective 2 is every
  major OS including Windows and whose objective 6 is a script-interpreter surface that is
  minimal and dual-implementable — bash for Linux and macOS, PowerShell for Windows. The
  objective set is recorded in TRAJECTORY.md.
  **RE-SCOPED 2026-08-24 by the operator, and the correction is a size rather than a
  direction.** This entry used to say the vendoring ruling had already fixed the shape to
  resolve-the-platform, place-the-binary, invoke — a three-step bootstrap whose relocation half
  `install-step-relocation` owned and whose size depended on that landing first. Both halves of
  that sentence were false at HEAD. `install-step-relocation` is **retired as mooted**
  (at this iteration's scope, 2026-08-24): its graph step already dispatches to the binary and its
  `gen-pre-commit.sh --write`
  step is refused relocation by the closed operator-ratified ruling at gate-sdk/SPEC.md
  §gen-pre-commit. **So the relocation is this entry's now, and it is most of the work.**
  **Measured at the scope that re-verified it, not estimated.** The three-step shape describes
  `installer/lib/init.sh:102-147` and `:277-308` — roughly eighty lines, the platform resolve plus
  the digest-verified placement `installer/README.md` §The gate binary already calls irreducible.
  The remaining ~350 lines of that file — kit-source vendoring, manifest and lock I/O,
  `gates.list` and queue seeding, the commit flow — are conditional install logic that no ruling
  puts behind the invoke, and `doctor.sh` (164), `uninstall.sh` (214), `diff.sh` (66) and
  `update.sh` (34) sit beside it unaccounted for. Sizing this against the three-step phrasing
  understates it by roughly fourfold.
  **Why it is design-pending, and the fork is now two questions rather than one.** First, what
  moves behind the invoke and what genuinely cannot — the second is the one that binds, because a
  step that cannot move is a step PowerShell must re-implement. Second, the original fork,
  unchanged: two hand-kept bootstraps held in parity by a smoke leg, versus one bootstrap
  generated from a single declaration. The second question is only answerable once the first
  fixes how many steps there are.
  **Cost while deferred:** the pivot's OS-reach objective stays unmet on the one platform it
  names that no current path reaches, and every install-path change is authored bash-first,
  which is the habit that made this entry necessary.
  Filed 2026-08-03 by spec, on the operator's trajectory pivot; re-scoped 2026-08-24 at scope,
  on the operator's ruling, after a first-hand read of the install path rather than an inherited
  premise.

- **instruction-surface-bash-focus** [design-pending] — the always-loaded surfaces assume bash.
  `CLAUDE.md` and the instruction surfaces beside it are written around a shell battery:
  the gate-authoring conventions, the fixture idiom, the housekeeping rules and the
  delegation guidance all name bash mechanisms as the default case. Under the trajectory
  pivot the default case becomes a native binary behind a minimal dual-implementable
  bootstrap, so those surfaces teach a shape the project is leaving. No entry claims this
  ground today.
  **Ordered by the operator's trajectory pivot 2026-08-03**; the objective set is recorded
  in TRAJECTORY.md, and objective 6 is what names this rewrite as needed without starting
  it.
  **Why it is design-pending:** the rewrite is not a find-and-replace. An always-loaded
  surface is costed per session, so the question is which bash specifics stay resident
  because they are still the common case, which move behind a load trigger, and which are
  deleted — and that depends on how far the port has actually got, which makes the trigger
  a threshold rather than a date.
  **Cost while deferred:** every session is oriented by a surface describing the substrate
  the project is moving off, and the correction is paid per session in re-derivation rather
  than once in an edit.
  Filed 2026-08-03 by spec; the pivot names this rewrite and does not start it.

- **plugin-marketplace** [design-pending] [roadmap: later/ecosystem] — harness plugin packaging.
  roadmap-summary: The stage skills and guards installable as a harness plugin.
  Harness plugin/marketplace packaging
  of the stage skills and guards; anti-drift gate shape: manifest ↔ shipped
  surface parity. Design against the live manifest format at promotion — the
  plugin substrate moves fast (the scope-session-routing ruling applies).
  **The install-ownership contract this must package against already exists:**
  `checkwright.lock`, written by the installer's `init` and specified at
  installer/README.md §The manifest — its schema owner is
  `installer/lib/common/lock.sh`. A marketplace package that installs kits
  without writing that manifest would be a second install model with no upgrade
  or uninstall story, which is the sequencing risk this entry has always
  flagged; the named contract replaces the re-derivation it used to imply.
  The upgrade/uninstall story itself is `installer-lifecycle-verbs` below —
  sequence against it rather than duplicating it.
  **Negative result — the tarball channel's economics do not transfer here.**
  `release-tarball-delivery-channel` is cheap for a structural reason that is
  absent from this rung: `.github/workflows/publish.yml`'s `pack` job already
  assembles and stamps one tarball and uploads it as the run's artifact, so a
  new channel is a sibling job that `needs: pack` and consumes that artifact.
  A marketplace package cannot consume it. Its unit of delivery is the
  harness's own plugin manifest format, not a packed npm assembly, and its
  subject is the stage skills and guards rather than the eleven-kit tree — so
  it shares neither the assembly nor the artifact. Recorded because the reflex
  at promotion will be to cost this by analogy from the tarball's sibling-job
  cheapness and arrive at the wrong number.
  **Open question a promoting scope answers first — deliberately undecided
  here.** Whether the marketplace package vendors kits at all, or merely
  registers the skills and delegates all vendoring to the installer's `init`.
  Under the second answer it stops being a distribution channel and becomes a
  **discovery surface**, and `checkwright.lock` ceases to be a contract it must
  *honour* and becomes one it must not *violate* — the materially cheaper
  answer, and the one that dissolves most of the sequencing risk above. It is
  not settled here because it is downstream of this entry's standing ruling
  that the plugin substrate moves fast and the design must be made against the
  live manifest format at promotion; deciding it now would be deciding it
  against a format that will have moved. Recorded 2026-07-26 by close
  (`activation-path`).
  **Cost while deferred:** zero mechanism rots — the install-ownership contract
  this must package against is already written and maintained by the installer's
  `init`; what is foregone is a discovery surface, and the plugin substrate's
  motion means a design taken early would be retaken at promotion anyway.
  Surfaced 2026-07-09 in adoption-track's split; evidence artifact retained:
  upstream Claude Code issue #75214 (project config can't lift the Task
  ask-first default), surfaced dogfooding the delegation nudge 2026-07-07.
- **benchmark-ab-experiment** [design-pending] [roadmap: later/adoption] — a controlled A/B trial.
  roadmap-summary: A controlled experiment measuring drift with and without governance.
  **Cost while deferred:** zero — the self-referential drift-trajectory route
  already carries the claim this rung would upgrade, and the measurement half it
  consumes ships independently; what is foregone is an externally-comparable
  number the project does not currently claim.
  The controlled differential
  experiment: same model, same dependent-task series, two arms (ungoverned
  loop vs Checkwright-governed), drift *accumulation across the series* as
  the metric — a governance layer's effect, not a model leaderboard number.
  Metric axis: Drift-Bench's "satisfiable drift". Substrate/vocab primaries:
  seqBench (arXiv 2509.16866), Drift-Bench (arXiv 2602.02455 — real title
  "Diagnosing Cooperative Breakdowns in LLM Agents under Input Faults via
  Multi-Turn Interaction"; the "Decomposing Reasoning Into Failure Types"
  expansion is confabulated, do not repeat it), Lost-in-Conversation /
  FlowBench as prior art. Surfaced 2026-07-08 inside adoption-track; split
  out 2026-07-09 — the self-referential route (drift-trajectory) ships
  first and this rung upgrades the claim only if demand attests it.
  The experiment's measurement half — per-stage, per-model, price-weighted
  token burn off harness transcripts — is the stage-economics-report tool
  filed above; this rung consumes it rather than rebuilding it. Nearer use of
  that tool: verifying the split-lead posture's savings
  (lifecycle-kit/templates/lead.md §Economics). Surfaced 2026-07-15 by the
  per-stage budget analysis that motivated that posture.
- **hosted-attestation-service** [design-pending] [roadmap: later/commercial] — hosted attestation.
  roadmap-summary: Gate runs verified by a neutral party no committing agent can touch.
  The team/paid rung: gates
  verified server-side by a party the committing agents cannot touch —
  hosted gate runs as a neutral attestation, cross-repo drift dashboards,
  maintained rulesets. A service, not code: cloning the kits does not clone
  the neutrality or the ops. Demand-gated — this entry is the public
  roadmap marker, not a scaffold; hosting and sequencing decisions are on
  record in the operator's local brief, and multi-operator-semantics
  is its prerequisite mechanism. Surfaced 2026-07-07.
  **Cost while deferred:** zero — this is a service rather than tree mechanism,
  so nothing rots; the residue is that gate runs stay self-attested, which binds
  only when a party the committing agents cannot touch is asked to trust them.
- **spec-internal-identifier-prefix-drift** [design-pending] — SPEC prose naming a
  script's **internal** variable spelling where the public knob is the contract
  name. Found by the config-seam-hardening close audit of the
  `internal-identifier-restatement` roster class, and fixed there: seven sites in
  delegation-kit/SPEC.md named locals of `bin/usage-verdict.sh` that exist only as
  assignments from their `DELEGATION_KIT_`-prefixed env knobs, while the same
  doc's §Layout roster had the prefixed spelling right — prose-vs-roster drift
  *within one file*.
  **Cost while deferred:** the fix is a rename away from rotting — renaming the
  local in the script silently falsifies the prose, and only the roster class's
  audit cadence catches it, at iteration granularity.
  **The manual fix is demonstrably incomplete — an eighth site surfaced
  2026-07-19** and was fixed at that close. That is the demand evidence the entry
  was waiting on: a hand sweep of one file missed one instance in eight, so the
  audit cadence catches what review does not, and a ~20-line scan expressed the
  whole class.
  The parent class is on the audit roster precisely because it is **un-gateable**
  (public contract names are legitimate citations). This entry is the narrower
  sub-class that does look gateable: a backtick-quoted `^[A-Z][A-Z0-9_]*$` token
  in a kit SPEC that appears in that kit's source *only* as a local assigned from
  a `<KIT>_`-prefixed env var must be cited by its prefixed spelling. The
  prefixed counterpart's existence is what bounds the false-positive surface — an
  internal constant with no public counterpart never fires. Needs spec because
  that boundary is the whole design: proving it is tight enough for a
  low-false-positive gate is the open work, and if it is not, the honest outcome
  is to record that here and leave the class to the audit cadence. Seam: the gate
  is generic mechanism; the `<KIT>_` prefix is already each consumer's config.
  **Premise updated 2026-07-20 by the `verdict-reader-honesty` close audit — two
  corrections, both of which the next scope would otherwise re-derive.**
  (1) *The class is currently clean.* The eighth site was fixed at this
  iteration's scope, and a fresh mechanical sweep of every kit SPEC found zero
  genuine hits. A gate built for this class today would land **greenfield** — it
  proves the boundary rather than fixing a backlog, so the demand argument now
  rests entirely on recurrence, not on outstanding drift.
  (2) *The false-positive bound above is looser than stated.* The sweep produced
  one hit — `GRAPH_VOCAB` at gate-sdk/SPEC.md:883 — which is **correct prose**:
  it names the array a consumer declares inside `<gates-dir>/graph-vocab.sh`,
  a public contract in its own right, while `GATE_SDK_GRAPH_VOCAB` is the
  *separate* knob naming that file's path. So a `<KIT>_`-prefixed counterpart
  can exist and denote a **different contract**, which the "prefixed counterpart
  bounds the false positives" claim does not anticipate. The gate must therefore
  also establish that the bare token and the prefixed one denote the *same*
  contract — the discriminator the design pass actually owes.

- **heterogeneous-agent-delegation** [design-pending] [roadmap: later/ecosystem] — foreign agents.
  roadmap-summary: Dispatch a stage to any vendor's coding agent, gated identically.
  Cross-vendor stage dispatch: a lead delegating a stage to a foreign coding agent,
  extending the homogeneous multi-agent model to a heterogeneous fleet. It cashes the
  public no-lock-in claim and is the purest expression of the thesis — governance enforced
  at the git/gate boundary, not by trusting the author. *Already agent-neutral:* the
  verification substrate (git, the gate battery, the bash stamp state machine) does not
  care who authored the diff, and the coordination primitive is the shared git-index/HEAD
  serialization. *Homogeneous today — the real work, worst-first:* (1) the **escalation
  resume model** collapses into (2) as a property of the chosen transport, per the 2026-07-25
  amendment below; (2) **dispatch transport** — today the harness
  `Agent`/`SendMessage`/task-notification; a foreign agent needs a transport-neutral
  handoff. The adapter contract is "open / prompt / permission-request / resume" spoken over
  each vendor's structured **machine plane, never its TUI**: a screen-scrape relay is the
  adapter of last resort for a vendor shipping no machine interface at all — it yields
  rendered frames not turn events, answers dialogs by heuristic, and bets on the vendor's
  least-stable surface. (3) **budget oracle** — the verdict tool is
  Anthropic-OAuth-specific; a heterogeneous fleet has N vendor-keyed oracles, the same seam
  as the credential-swap entries, and the vendors' JSONL event streams carry the token-usage
  events a TUI path would scrape from a status bar. (4) **stage-contract expression** — the
  lifecycle machinery is neutral bash but the stage-skill prose is not.
  **Seam ruling (on record):** generic mechanism only — transport, budget oracle, and
  escalation channel become consumer-config seams; a kit literal naming a vendor crosses
  the provenance seam and is ruled out, the `prose-profile` pattern. It extends the
  per-batch model-tiering lever across vendors, and interacts with
  `hosted-attestation-service`, `plugin-marketplace`, and the credential-swap entries.
  **Demand-gated — demand attested (2026-07-23):** the operator holds working
  foreign-vendor subscriptions and wants read-heavy delegation routed to them for budget
  headroom, and with three vendors live the N-keyed oracle seam is no longer hypothetical.
  First slice at promotion: a foreign-CLI executor for the already-pre-authorized
  read-heavy audit / mechanical-sweep class over a spawned non-interactive CLI process,
  one adapter per vendor as consumer config — not full stage dispatch.
  Promotion-eligible at the next scope session.
  **Design-memory amendment (2026-07-25):** the TUI-relay alternative was probed for
  session resume and token efficiency. Ruling: those benefits live in the vendor's session
  store, not the TUI — the APIs are stateless and both modes replay the same on-disk
  transcript against the same server-side prompt cache, so interactive-vs-headless is a
  rendering choice, not a state choice. Headless warm-resume by session id and JSONL turn
  events ship today on the vendors probed, which is what makes (1) plumbing.
  **Verification capability (2026-08-02):** those probes ran against **installed binaries**
  — the foreign CLIs are present on the development machine — so the executor is verifiable
  rather than inferred from vendor documentation. Under oracle-first that is a change to
  the unit's risk, not a convenience, and it sharpens the first slice: the executor ships
  with a smoke that actually invokes them, the shape every kit already uses. The machine
  profile (context-kit/SPEC.md §bin/env-probe, local-only) owns which CLIs and how.
  **Cost while deferred:** the foregone lever is live — read-heavy audits and mechanical
  sweeps all bill against one vendor's budget while three subscriptions are held — and
  this design memory ages against fast-moving CLIs.
  Surfaced 2026-07-17 in the release-in-lifecycle lead session (operator question).

- **background-credential-swap-support** [design-pending] — first-class support for
  swapping the Anthropic OAuth credential out from under in-flight agents (to
  spread burn across accounts), which the budget oracle does not model today.
  Four components, worst-first; all delegation-kit SPEC+code, all demand-gated
  (no one swaps in background yet — this is the roadmap marker).
  **(a) Detection.** usage-verdict's auth-change reroute fires only on
  CRED_FILE mtime, so an out-of-band / env-var / path token swap that does not
  rewrite that file bypasses it — the verdict trusts the prior account's
  snapshot and the poller re-fetches the stale file's token. Broaden the reroute
  to also fire when the live account identity (oauthAccount.accountUuid /
  subscriptionType) differs from the snapshot's `account=` / `tier=`, forcing a
  re-poll on any swap.
  **(b) Evidence.** the `.metric/` trend samples already carry `account=` /
  `tier=`, but the wave-over-wave burn projection reads the tail
  **unpartitioned**, so a swap reads as a spurious used% drop that corrupts the
  projection and masks aggregate load. Segment usage analysis by `account=` and
  mark the swap boundary in the trend log so the evidence is per-account-honest.
  **(c) Safety.** the budget guard's premise is one account = one rate window
  per wave; background rotation moves the wall in-flight agents bill against and
  lets rotation collectively exceed what any single account's 5h/7-day PAUSE
  would allow while each account stays individually under threshold. Add a
  cross-account aggregate view so supported swapping cannot silently blow past
  the true combined ceiling.
  **(d) Signal-quality refinement (advisory, not a bug).** the post-login
  reroute (`DELEGATION_KIT_LOGIN_WINDOW`) is correctly advisory-only — STALE
  never blocks (delegation-kit/SPEC.md §usage-verdict, which also states the
  server lag the next point turns on), so this is signal quality, not a
  dispatch-blocking defect. Two
  points: the window default is 600s while the SPEC's own stated server-lag is
  "about a minute", a ~10x margin worth tightening; and it is a
  **blanket** time-window where an **account-keyed** check is sharper — trust
  `usage.txt` when its `account=` matches the current credential's account AND
  `updated_at > login_at`, with a short (~90s) settling floor for the server
  lag. That restores the true reading in ~1 min instead of 10 and stops 10 min
  of STALE samples polluting the trend log (`.metric/usage-history.log`) — which
  directly sharpens (b).
  **Cost while deferred:** any background swap today silently corrupts the burn
  projection and can breach the combined budget ceiling with every account
  reading individually safe; and the login window over-STALEs by ~10x.
  **Seam:** all four are generic delegation-kit mechanism — the account-id is
  already on the `usage.txt` contract; nothing consumer-specific is added. This
  is the budget-oracle prerequisite cluster heterogeneous-agent-delegation
  cross-references. Surfaced 2026-07-17 in the release-in-lifecycle session
  (kfric plus one operator-raised refinement).

- **enforcement-first-behavioral-regressions** [design-pending] — the always-loaded
  enforcement-first rule ("the fix and the gate that catches it land in one unit;
  removing the duplication outranks gating it") anchors its second clause — and
  every neighbouring doctrine example (content-tiering, de-literalization) — in
  the SSOT/duplication domain, so it under-cues the incident→gate reflex for a
  *behavioral* regression that has nothing to do with duplicated content. Design
  shape: tighten the always-loaded line (or the doctrine-kit section behind it) to
  name the generalize-to-class-then-gate reflex for runtime/behavioral defects,
  not only duplication — a `doctrine-kit/DOCTRINE.md` change, re-vendored to
  upgrade, so it sits outside an incident-fix commit's envelope.
  **Cost while deferred:** each behavioral bug fixed in a maintenance turn risks
  shipping without its paired low-FP gate until a reviewer prompts.
  Surfaced 2026-07-19 by
  the check-graph `maxEdges` fix (the coupling graph outgrew Mermaid's 500-edge
  render cap): the one-line render fix landed, but the paired render-cap gate —
  exactly the low-FP gate enforcement-first says to land in the same unit — was
  added only on explicit request.

- **spec-split-promotion-review** [design-pending] — after the six-stage roster has
  run **≥N iterations with `spec`-stage economics actually recorded on the
  trajectory**, re-run the `/economics` read; if the split shows the projected
  cache/context win, **promote the `spec` stage to the kit default** via a new
  amendment (else keep it consumer-config or revert). The `stage-posture-split`
  amendment shipped the stage as optional, demand-gated consumer config
  precisely so this promotion is an evidence-gated follow-up rather than a
  default flip on projection alone.
  **Precondition — hard dependency on `trajectory-stage-roster-hardcode`:** until
  that lands, the `trajectory` emit arm silently drops every `spec` stamp
  (hardcoded 5-stage roster), so **no `spec`-stage economics data is recorded on
  the trajectory at all** — the re-run this task prescribes has nothing to read.
  This task cannot start before that fix.
  **Premise corrected 2026-07-20 by the undirected scope survey — the named
  dependency has landed and this entry is still blocked, by a different one.**
  `trajectory-stage-roster-hardcode` reached Done, and the trajectory table now
  renders `sp` for three iterations — so this entry reads as unblocked and
  pickable. It is not. The trajectory and the *economics* are two surfaces, and
  only the first recovered: `.metric/stage-economics-log.txt` holds **zero `spec`
  rows**, because `bin/stage-economics.sh` reads the boundary-truncated live state
  file rather than committed history. The precondition this entry actually needs
  is *recorded spec-stage economics*, and that is blocked on
  **`stage-economics-truncation-durability`** — the real blocker, named here so no
  future scope re-derives it or promotes a review with nothing to read.
  **Premise corrected again 2026-07-21 by the scope survey — that blocker has
  dissolved and this entry is now genuinely pickable.** The "zero `spec` rows"
  reading above was taken before the meter's history union had been exercised;
  `.metric/stage-economics-log.txt` in fact holds five `spec` rows covering five
  distinct iterations, all recovered *after* their boundary truncations. So the
  data this entry prescribes reading **exists**, and the named blocker is being
  retired inside `stage-economics-honesty` rather than repaired. Two carry-overs
  before the re-run is worth anything: (1) wait for that iteration to land, since
  the figures this review reads are exactly the ones
  `stage-economics-attribution-honesty` is correcting — re-running against
  mis-attributed rows would settle a tier question on noise; (2) the `≥N
  iterations` threshold this entry leaves unset still needs a value, and five
  recorded iterations is the number now on the table.
  **Carry-over (1) discharged 2026-08-01 by the undirected scope survey:**
  `stage-economics-honesty` has landed, attribution fix included, so the rows are
  no longer mis-attributed and this entry is **pickable with nothing left
  blocking it**. Only carry-over (2) — the unset `≥N` — stands.
  **Cost while deferred:** one queue line; the backlog-aging review re-raises it
  every iteration until the data exists to run it. Filed 2026-07-19 by lead
  ruling at the `stage-posture-split-tuning` close — the split shipped on
  projected economics, and this is the loop that confirms or retires that
  projection with recorded data.

- **build-stage-tier-economics** [design-pending] — measure whether the `build`
  stage downgrades from Opus to Sonnet net-positive rather than flipping on
  intuition; a ruling-config tier re-judgment (`.claude/agents/stage-session.md`
  / the lead template's ruling-config, which invites re-judging every tier).
  Grounding, **corrected** against priced rows: the split-lead spread is build
  $2.59–10.96, close $5.83–7.81, validate-on-Sonnet $0.54–1.44, and neither
  reading it yields matches the original grounding. **Close is comparable to
  build, not an order of magnitude below it** — so close is a tier candidate in
  its own right, arguably ahead of build, and the premise that build is the
  single highest-value lever no longer holds. And the already-adopted
  validate→Sonnet downgrade **demonstrably works**: validate is the cheapest
  stage by a wide margin with no observed quality cost, which is the affirmative
  precedent this A/B is testing for build. Re-read the figures from the log
  rather than trusting them as transcribed, and read the `cr` column rather than
  `cost` — a falling per-token rate makes a growing draw read flat in dollars.
  The superseded token-only reading is in history.
  **One design blocker remains; the first two are answered.** Both the
  uninstrumented metric and the mis-attributed rows are **discharged** — a price
  table exists, the meter prices instead of reporting `cost=n/a`, and
  `stage-economics-honesty` landed the one-transcript-one-row attribution fix on
  2026-08-01, so this task is **unblocked** and self-labelled Debt below. What
  stands: the metric must be **net delivered-work cost** — price-weighted tokens
  + rework round-trips + the supervisor's by-eye gate-diff burden + escalation
  load shifted onto the Opus lead — not single-pass token price; a cheaper builder
  that fails the battery or emits a subtly-wrong gate the supervisor must catch
  can invert the saving.
  **Design direction:** a deliberate A/B on representative *low-judgment* builds
  (convergence on an already-authored contract), holding the unit class
  comparable; high-judgment builds that write gate assertions or touch the
  provenance seam likely stay Opus regardless, so the honest outcome may be a
  per-build-class tier rule, not a blanket flip. Sibling to
  `spec-split-promotion-review` (evidence-gated tier promotion) and
  `benchmark-ab-experiment` (which holds model constant and varies governance —
  this holds stage constant and varies model). Debt/analysis: settles a
  ruling-config tier by data, adds no governed name.
  **Datum, twice now: the per-batch lever paid nothing.** `native-cohort-activation` (2026-08-07,
  four units) and `native-cohort-canon-kit` (2026-08-12, three batches) each classed every delta
  before dispatch and found every unit design-bearing, so none was downgradeable and all rode Opus.
  Evidence *for* the per-build-class rule (classification is cheap and correctly returns *no*), and
  a warning that a whole iteration can be design-bearing, so the lever's value tracks the mix scope
  cuts. **Not yet measurable either:** the log keys on (iteration, stage), so a batch-split build's
  single stamp hides per-batch cost — `batch-split-stamp-ownership` owns that fix.
  **Cost while deferred:** low and non-rotting — validate's adopted downgrade
  already banks the affordable half of this lever, and the prerequisite that
  made the rows provisional has now landed; the residue is that
  build's and close's tiers stay set by intuition rather than by a priced A/B.
  Filed 2026-07-20 by lead
  ruling during the `render-fidelity-leak-coverage` spec, from an operator
  question.

- **supervision-overhead-unmeasured** [design-pending] — supervision is roughly a
  fifth of an iteration's priced burn and has never been examined. It went
  unexamined because it was not a distinct row at all until the attribution fix
  landed; the figures live in `.metric/stage-economics-log.txt` and are read from
  there rather than restated here. The obvious experiment — run a lead session on
  Sonnet against the Opus baseline — has been **declined twice at scope**
  (2026-07-22, both times on the same grounds), and the premise has since
  sharpened enough to re-rank the entry's own open work.
  **Both legs are blocked, which this entry used to obscure.** Re-counted at that
  scope: 24 `supervision` rows, the priced Opus ones spanning a **6x** range. Read
  against that spread a single Sonnet lead session is not merely underpowered, it
  is uninterpretable in principle — any plausible tier effect sits well inside the
  existing variance — so the experiment as filed returns an unusable number *even
  if the quality read existed*. The cost leg therefore needs either many repeated
  runs (n≫1, at one lead session per iteration) or a variance-controlled
  comparison normalizing for iteration size, the obvious candidate being **cost
  per unit delivered** rather than cost per iteration. **Design that
  normalization before the quality read:** an uninterpretable cost axis makes the
  quality axis moot, and the reverse does not hold. A further constraint on any
  comparison: supervision is the only row still growing while close runs, so rows
  must be read at the *same* lifecycle point, never at two convenience snapshots.
  **The risk is a different class from validate's.** Supervision is where
  **rulings** happen, so the failure mode is not a bigger bill but a **bad ruling
  that costs a rebuild** — which the cost row would score as a *saving*. That is
  why a quality read is owed at all, and the design question this entry carries is
  what that read is: a rebuild count, an escalation-correctness sample, or an
  honest ruling that the axis is unmeasurable at n=1. It is also why the two
  declinations were correct rather than missed windows: the cost side has many
  priced Opus rows and the quality side has no read at all.
  **What cannot be harvested by delegation.** Supervision splits internally the
  way `build` does — mechanical routing/verification versus genuine rulings — so
  the batch-tiering answer looks transferable. It is not: the verification half is
  **not delegable away from the supervising session**, because the supervisor
  re-running the battery and diffing every agent commit *is* the protocol
  (`delegation-kit/templates/agent-execution.md`). A supervision split can be
  tiered but not delegated, which narrows the levers to the tier question.
  Debt/analysis: measures an unexamined cost line and may re-tier a lead binding;
  adds no governed name.
  **Cost while deferred:** the largest unexamined line in the iteration budget,
  paid every iteration, with no evidence either way about whether it is bought or
  wasted. Bounded and non-rotting — nothing breaks, and the row now accumulates
  per-iteration baselines whether or not the experiment runs.
  Filed 2026-07-22 by close, from the same lead-side economics review.

- **platform-support-ci-matrix** [design-pending] [roadmap: next/reliability] — a leg per platform.
  roadmap-summary: A CI install-smoke leg per supported platform, or an honest label.
  The per-platform half carved out
  of `platform-support-contract` when that entry was scoped down to the floor
  contract for the `activation-path` iteration: a CI install-smoke leg per
  supported platform (Linux / macOS / Windows-WSL), or an explicit experimental
  label where no leg exists. The floor contract states *what* the toolchain
  floors are; this entry is the mechanism that proves a platform actually meets
  them, and the support matrix's per-platform rows are only as honest as the
  legs behind them.
  **Why deferred rather than built alongside the floors:** the repo runs three
  workflows (`.github/workflows/gates.yml`, `publish.yml`, `site-health.yml`), none a
  matrix, so this is new runner spend on macOS and WSL images — and **no macOS
  or WSL adopter exists** to attest the spend. Demand-gated on exactly that,
  like the other adoption rungs.
  **Re-verified 2026-08-02 at scope, and a second un-defer trigger named.** The
  workflow count moved (two → three); the substance is unchanged — still zero
  non-Linux legs, no `matrix:` key anywhere, and the adopter trigger unfired. The
  second trigger: **`native-gate-binary-port` reaching distribution** — refined the
  same day, because "prerequisite" first stated it too strongly. The two are
  *different jobs* sharing only runner spend: that entry needs binary build+smoke
  legs in `publish.yml`, this one specs install-smoke of the bash battery. So
  neither blocks the other's start, and the port's per-platform obligation attaches
  to its artifacts clause — the release boundary, not the first ported cohort.
  **The un-defer trigger, carried so the reason is not re-derived:** the gap
  that motivates the legs is now *stated* rather than contradicted —
  `platform-support-contract` landed the floor contract, so `docs/install.md`
  §Requirements says outright that stock macOS ships bash 3.2 and a BSD
  userland and that a GNU toolchain is an adopter action there. That statement
  is the whole of what the floor contract can do; nothing proves whether the
  battery actually runs on a Mac so prepared until a leg runs there, and the
  page's macOS support claim rests on reasoning until one does. So this entry
  un-defers on the first
  of: a macOS or WSL adopter appearing, or the support matrix wanting to promote
  a platform from experimental to supported.
  **Cost while deferred:** the support matrix's non-Linux rows rest on
  reasoning rather than on a green run, so a macOS claim is unfalsified in both
  directions; zero until a non-Linux adopter exists, then it becomes the
  dominant support load, which is the same shape the parent entry recorded.
  Bounded and non-rotting — the floor contract lands the falsifiable half.
  Debt: CI configuration over an already-stated contract; adds no governed name
  unless a platform label becomes a gated surface.
  Filed 2026-07-26 by scope, operator ruling at the `activation-path` unit-set
  escalation, split from `platform-support-contract`.

- **lint-scope-hook-trigger** [design-pending] — `GATE_SDK_LINT_EXTRA_DIRS` widens
  what `check-shellcheck` scans but cannot widen when the generated hook fires
  it. The hook's trigger is expanded from the gate's `# graph:` couples
  (`scripts/*.sh,kit:*.sh`), and `kit:` resolves to kit roots — a set no
  consumer knob joins — so a directory only the knob names is linted by the
  full battery and by CI, never at commit time. gate-sdk/SPEC.md
  §check-shellcheck now states this limit; this entry owns the fix.
  **Deliverable:** a consumer-expandable `lint:` token beside the existing
  `kit:` token in `gate_expand_couples`, expanded at hook-generation time from
  `GATE_SDK_LINT_EXTRA_DIRS`, so a consumer's added lint directories reach the
  generated hook without any kit literal naming them — the same shape the
  `check-graph` / `graph-vocab.sh` pattern already uses to keep consumer
  vocabulary out of kit source.
  **Why deferred rather than taken with the knob:** it changes gate-sdk/SPEC.md
  §The `# graph:` manifest — a gate-sdk contract the `activation-installer`
  amendment's §Existing sections updated does not list, so it sits outside that
  envelope. Scope-gated intake, not a batch side errand.
  **Two cheaper-looking options are already dead; do not re-derive them.**
  Widening the gate's `couples=` with entries like `kit:lib/*.sh` is a no-op:
  the hook matches with a pattern comparison in which `*` spans `/`, so
  `<kit>/*.sh` already reaches every subdirectory of every kit — measured, and
  the reason the kit half of the originally-reported gap turned out not to
  exist. Setting `trigger=*` works but costs the full gate on every commit:
  `check-shellcheck` runs about five seconds, the second-slowest gate in the
  battery behind `check-docs-render-fidelity`.
  **Cost while deferred:** consumer-named lint directories — this repo's
  installer scripts and its runnable walkthrough — are covered one tier later
  than every other script in the tree. CI catches a lint regression there; the
  commit does not, so the feedback arrives after the push rather than before
  it. Non-rotting and bounded to directories the consumer chose to add.
  Filed 2026-07-26 by build, on the lead's ruling during the `activation-path`
  doctor + manifest batch, after measuring both the glob behavior and the
  gate's runtime.

- **gate-file-coverage-closure** [design-pending] — the missing check class behind a
  hole this close fixed inline: nothing asserts that every gate script in the
  tree is matched by some `DELEGATION_KIT_GATE_FILES` glob, so a gate can sit
  outside `check-gate-tamper`'s assertion-A coverage silently. It did: this
  repo's consumer config declares the array, which **replaces** the kit default
  rather than extending it (`delegation.sh` guards it with `declare -p … ||`),
  and the declaration named only `*/checks/*.sh` — leaving all nine
  `scripts/check-*.sh` consumer-resident gates uncovered. Close restated the
  default's glob in `scripts/delegation-config.sh` and corrected
  delegation-kit/SPEC.md §Layout and configuration, which had described the
  knob as *widening* the default — the inverse of the mechanism.
  **The gate:** enumerate gate scripts (the `gates.list` registry resolved to
  files), and red any whose path no `DELEGATION_KIT_GATE_FILES` glob matches.
  Cheap and mechanically decidable — the coverage set and the glob set are both
  already in hand at gate time. Needs a `gate-sdk` fixture pair and a home
  (delegation-kit, since it reads that kit's knob).
  **Why `[design-pending]`:** it makes coverage-completeness a delegation-kit
  contract, which is a SPEC assertion, not just a new script.
  **Cost while deferred:** the config is correct today but unheld — the next
  consumer-resident gate, or the next kit-glob edit, can reopen the identical
  hole with nothing to catch it. Exactly the replace-vs-extend footgun the SPEC
  now documents but does not enforce.
  Filed 2026-07-26 by close (`activation-path`), generalizing the
  knowledge-friction captures that surfaced the replace-vs-extend semantics.

- **gate-battery-result-cache** [design-pending] — re-run the battery over an unchanged tree and
  every gate redoes its work. This iteration ran it well past half a dozen times,
  nearly always over trees where most gates' coupled inputs had not moved.
  **The cache key is already derivable, which is the whole case.** Every gate
  declares its inputs in its `# graph: couples=` manifest — the same source
  `gen-pre-commit.sh` projects the hook from — so a key is a content hash over
  the expanded couples plus the gate's own source. Nothing new to maintain, and
  `check-graph` already gates the manifest's freshness.
  **Opening question, to be answered rather than assumed:**
  **`docs-renderer-batch-contract`** ruled a content-hash cache *out* inside its
  own scope 2026-08-01. Whether that reasoning generalises from one renderer to
  the battery is this entry's first work — inherit it or overturn it on evidence,
  never ignore it.
  **The risk is invalidation, not speed.** A gate whose real inputs exceed its
  declared couples would be skipped while stale — a false green, the exact
  failure class the battery exists to prevent. `check-reads-couples` is the
  existing oracle for that gap and would become load-bearing rather than
  advisory, so its own coverage is a precondition, not a detail.
  **`battery-runner-port` landed 2026-08-23 and delivers the precondition, not
  the closure.** The couples-keyed bookkeeping this entry's key would read is
  now an in-process map (the crate's registry/couples machinery) rather than
  shell bookkeeping, which is ordinary engineering exactly as expected — but the
  unit built no cache, so this entry's own invalidation question stands exactly
  where it was: whether a gate whose real inputs exceed its declared `couples=`
  would be skipped while stale, and whether `docs-renderer-batch-contract`'s
  2026-08-01 content-hash refusal generalizes from one renderer to the battery.
  Both stay this entry's first work (gate-sdk/SPEC.md §lib/gate.sh).
  **Cost while deferred:** repeated full-battery runs inside one session are the
  dominant waste — a stage that validates after each commit re-runs every gate
  over a tree it just proved. Nothing is incorrect while deferred.
  Debt: no governed name. Filed 2026-08-01 by the lead at operator direction.

- **state-representation-integrity** [design-pending] — repo state lives in text
  files whose invariants — slug uniqueness, cross-entry `blocked-by` and `spec`
  targets resolving, one evidence record per suite — are properties of the
  readers, not of the format. Each is enforced by a gate that greps, so an
  invariant with no gate is silently unenforced rather than merely unchecked.
  **Measured evidence (2026-08-01, operator-directed), to be weighed at a future
  scope entry and opening nothing now.** Asked whether an embedded database should
  replace the text surfaces. Against `release-step-verification`'s eleven defects,
  a store with constraints prevents **two**: the interrupted validate run leaving
  a half-written evidence file (a transaction rolls it back), and
  `gate-tests-suite-identity-in-evidence` (a primary key makes an unidentifiable
  record unrepresentable). The other nine are CI-checkout and API-pagination
  semantics, assertion design, and missing oracles — untouched by storage. The
  counterweight belongs in the entry too: `check-task-conservation` *did* catch a
  deleted slug, as a grep over text.
  **Diffability is load-bearing here, not incidental.** `TASK-QUEUE.md` and the
  gap inbox are reviewed in diffs, merged across concurrent sessions by
  `merge=union`, and carry paragraphs of human reasoning. A binary store ends
  `git diff`, `git blame`, line-level review and union-merge; prose in a TEXT
  column buys no integrity over the part that matters most.
  **So split by shape, never convert wholesale.** Machine-written append-only
  records — the evidence manifest, `WORKFLOW-STATE.txt`, `tightened-gates.txt`,
  `gates.list` — are tables pretending to be files, but their concrete need is
  atomic append and self-identifying records, a far smaller fix than a database.
  Human-authored documents stay diffable text.
  **Deliverable, and it converges with `native-gate-binary-port`:** both reduce to
  *parse once into a typed model and validate there* — one parser and a schema
  replacing N hand-rolled greps. A store is one way to reach that model, a real
  deserializer over typed structs another. Cost them together or the same work is
  counted twice. The middle path is this repo's derivation-first rule one level
  further: keep text canonical and generate a freshness-gated queryable index, as
  `check-graph`, `ROADMAP.md` and the enforcement map already do, so gates run
  uniqueness and referential-integrity queries against the projection. That keeps
  diffs, centralises parsing, and trades *prevention* for after-the-fact
  validation — which is the bargain the gate model already makes.
  **The strongest argument for a store is one nobody has made yet:** there is no
  concurrency primitive. Shared-index contention, `merge=union` as a workaround,
  and a live session's journal clobbered mid-iteration are symptoms of its
  absence. Weigh it apart from the integrity case.
  **Cost while deferred:** gates keep hand-rolling parsers over state surfaces and
  integrity stays gate-enforced rather than structural — caught at commit where a
  gate exists, silent where none does. Nothing breaks today.
  Filed 2026-08-01 by operator request at close, on evidence the same iteration
  produced; a sibling rather than an addition because `check-queue-entry-budget`
  refused the combined body.

- **rule-reach-before-merits** [design-pending] — the iteration's recurring
  failure mode, in both its forms, with no durable home. Six times a question
  that presented as a merits call turned instead on the governing rule's **scope**.
  **Form one — a rule invoked that does not reach the unit (five instances).** At
  scope the operator's pre-launch bar was the Enhancement decision rule and three
  of four units never reached it; at spec the knob-rename compat precedent reached
  neither rename; at align the release note's Renamed-knobs section proved scoped
  to own-config knobs twice over and so took neither rename either. In none was an
  exception owed or taken — and logging an exception where the rule never reached
  corrupts the record for the next reader.
  **Form two — a governing mechanism that DID reach and went unread (one
  instance).** Three sessions designed new mechanism for the close-entry refusal —
  an early tag, a rescoped preflight, a new `verdict=` state, a smoke redesign —
  before anyone read evidence-kit's held-constant-red baseline idiom, which already
  covered the case exactly and needed no new mechanism at all.
  They are one failure mode seen from two sides: **establish what a rule governs
  before arguing where its line falls, and read the governing mechanism before
  designing its replacement.**
  **Why `[design-pending]`:** placement is the whole question. As stated it is a
  delivery-doctrine rule — `doctrine-kit/DOCTRINE.md`, re-vendored, so it owes a
  release-note bullet — with an always-loaded one-liner above it; but form two sits
  close enough to the existing Oracle-first and Spec-over-precedent rules that it
  may belong as a clause on one of them rather than as a new rule. Minting a rule
  that restates a neighbour is the exact defect the doctrine's own content-tiering
  rule forbids, so the split is the design.
  **A third candidate rides the same placement question, and only that question.**
  Twice this iteration an oracle's *satisfaction* was read as the spec: the
  upgrade smoke's demand for one Tightened-gates bullet where the contract wanted
  three, and the budget guard's `OK` read as a dispatch decision. Generalized —
  *an oracle's pass is a floor, not the specification* — that is a doctrine rule
  sitting beside Oracle-first, and it is named here rather than filed separately
  because whoever settles new-rule-versus-clause for the two forms above settles
  it for this one in the same motion. The concrete half is already landed in
  delegation-kit/SPEC.md at the guard's contract; only the doctrine tier is open.
  **Cost while deferred:** the pattern recurred six times in one iteration and is
  paid in rework rather than in tree state — a wrong scope read spends a session's
  design effort on a rule that was never in play. Non-rotting; nothing degrades.
  Surfaced across scope, spec, align, build and validate of
  `pre-adoption-grammar-break`; drained from the gap inbox by close, which merged
  the original three-instance lesson with its inverted-form correction.

- **prose-filename-citation-liveness** [design-pending] — a bare `<name>.md`
  filename cited in governed prose can name no tracked file and nothing reds.
  `check-md-refs` resolves markdown *links* only; `check-spec-pointer` resolves
  `spec:`/`contract:` directives and free-prose `<path>.md §<heading>`
  citations, so a citation with **no** `§heading` and no link syntax falls
  between them.
  **Instance, fixed at this close:** `site-kit/SPEC.md` and its `docs/` mirror
  cited `SPEC-os-support.md` as owning `docs/install.md`'s Requirements ruling.
  The amendment is deleted, so the pointer dangled. Close deleted the
  parenthetical rather than repointing it — the same sentence already names
  `docs/install.md`'s Requirements prose as the stater, and that page carries
  the ruling. Same class as the seven citations the `release-assertion-honesty`
  build sweep cleaned, which is what makes this recurring rather than a one-off:
  every merged amendment deletes a file that governed prose may still name.
  **Why `[design-pending]`:** the predicate needs care, and a naive one reds
  correct prose. `docs/install.md` names `AGENTS.md` four times as a harness
  convention file that is deliberately untracked here — a legitimate citation of
  a filename with no in-tree target. Candidate narrowings: bind only the
  `SPEC-*.md` amendment-naming convention (narrow, decidable, covers the whole
  observed class), or extend `check-spec-pointer`'s free-prose extractor to the
  headingless form behind an exclusion roster. Which, and whether it is a new
  gate or an assertion inside an existing one, is the open call.
  **Cost while deferred:** one dangling citation per merged amendment that
  governed prose names, found only by a hand sweep somebody remembers to run;
  the citations read as live pointers until then.
  Debt: one gate plus its fixture pair, or an assertion added to an existing
  gate. **That line states the unit's size, not its class** — on the shipped
  path it mints a script name and a `scripts/gates.list` registration, so
  canon-kit/SPEC.md's new-names litmus makes it a **feature** owing an
  amendment. It promotes into a feature section; only the
  assertion-on-an-existing-gate variant would be debt.
  Filed 2026-07-31 at close as the gap-generalization owed by the inline fix
  above; the `check-md-refs` blind spot and the `AGENTS.md` false-positive case
  were both verified against source before filing.

- **lead-iteration-open-authorization** [design-pending] — the iteration lead may
  open an iteration on its own inference, and opening one is the operator's call.
  lifecycle-kit/templates/lead.md §Opening an iteration requires only the
  operator's standing **directive** — a theme bounding scope's survey — and
  explicitly permits proceeding without even that ("absent a directive, the lead
  dispatches scope undirected"). The directive slot presupposes the decision to
  open has already been made and is silent on who makes it; re-verified at this
  close, the file contains no authorization word at all, and
  `lifecycle-kit/bin/enter-stage.sh` carries no floor either — its first-stage
  branch differs only in stamping `—` for the iteration name.
  **The live instance.** The operator said "fix all" about four outstanding
  release defects, two of them edits to governed surfaces; the lead read that as
  authorization to open an iteration, reset the boundary, dispatch scope, and
  stamp — and the operator then stated plainly that they had not asked for one
  and that opening one requires their explicit approval. The inference was
  defensible on the *content* (governed-surface edits do route through the
  lifecycle) and wrong on the *scale*, which was the operator's to choose.
  **Deliverable:** state in lead.md that opening an iteration requires explicit
  operator authorization, distinct from and prior to the standing directive, and
  that an ambiguous instruction to fix filed work is not that authorization — the
  lead asks. **Why `[design-pending]`:** whether a mechanical floor is buildable
  is the open question. Prose requests and guards enforce, so the shape worth
  costing is an `enter-stage.sh` first-stage precondition; against it, the
  authorization is a fact about a conversation, and every encoding of it
  (a flag, a stamped operator line) is forgeable by the same session it binds.
  **Cost while deferred:** a lead can spend an iteration's full opening cost on a
  scale the operator never chose, and the only detector is the operator noticing
  after the stamp has landed. Recurs per iteration opened under ambiguity.
  Debt: one prose rule, optionally one precondition; adds no governed name
  unless the floor lands.
  Filed 2026-08-01 at close from the gap inbox, operator-directed, from a live
  instance in this iteration's own opening.

- **workflow-permissions-scope-oracle** [design-pending] — no gate parses a
  workflow `permissions:` block, so every scope a workflow needs is landed on
  reading alone. Re-verified exhaustively at this close: no check script matches
  `permissions:` at all, and every `permissions` hit across the tree is `jq` over
  the harness `settings.json`, an unrelated concept. The failure mode is concrete
  and this iteration supplied it: a `permissions:` block is an **allowlist**, an
  undeclared scope makes the read come back as an HTTP 404, and a 404 on a read is
  indistinguishable from an absent resource — the site-health release-body arm
  needs `contents: read` and would have reported "no such Release" without it.
  On a public repository the omission stays invisible until a private-repo
  consumer copies the workflow.
  **Precedent, stated precisely** (the gap's own framing overstated it):
  gate-sdk/SPEC.md declares **GitHub-expression injection** a non-goal and defers
  it to "a dedicated workflow-security linter". It does *not* rule the whole
  workflow-security category out of scope, so that line is supporting precedent
  for keeping the gate narrow, never a standing exclusion to argue against.
  **Deliverable:** a gate far narrower than a security linter — a workflow job
  whose `run:` bodies invoke `gh`, or that checks out, must declare the scopes
  those calls consume, starting with `contents: read`. Tree-local, hermetic,
  cheap: one gate, a `good/`+`bad/` fixture pair, a `gates.list` row, a graph
  manifest.
  **Cost while deferred:** charged against every workflow that gains an API call;
  the detector today is a 404 read as a missing object, which is the exact
  misreading `release-credential-precondition-scope-vs-permission` was filed for.
  Size: one gate plus fixtures.
  **Feature-shaped — self-label corrected 2026-08-04 at close.** This line read
  "Debt" while naming the governed gate name it mints; canon-kit/SPEC.md §The
  amendment lifecycle's litmus makes that a feature, so promoting it authors an
  amendment.
  Filed 2026-08-01 at close from the gap inbox, confirmed at this iteration's
  align audit against all nine gates that read workflow YAML.

- **template-copy-parity-yaml-widening** [design-pending] — a kit `.yml` template
  and this repository's copy of it are mirrored by hand and a missed half is caught
  by nothing. Re-verified at this close: `check-template-copy-parity` globs
  `*/templates/*.sh` against the gates dir — every pair it finds is shell and none
  is YAML — and no other gate compares two hand-maintained copies of anything. Every
  other parity or freshness gate compares a **generated** projection against its
  source, which is a different problem. The duplication cannot be removed, because
  kit-template-plus-consumer-copy *is* the distribution model, so enforcement-first
  prefers a remedy that is unavailable here and gating is the fallback — and the
  fallback is absent.
  **Why byte parity is the wrong assertion.** The two `site-health.yml` files
  diverge today across eight hunks, and the divergence is largely *correct*:
  `ALT_DOMAIN` is `alt.example.com` in the template and this repo's own host in the
  copy, and the template's "copy verbatim, then set or delete this arm" adoption
  comments have no place in an instance. The shape that fits is the declared-
  divergence contract `check-template-copy-parity` already implements for `.sh`,
  widened from that extension to a registered template/copy pair of any extension —
  which also means the copy side stops being hard-wired to the gates dir.
  **Cost while deferred:** charged against every future edit to either copy. This
  iteration widened the exposure rather than creating it — the release-body arm
  adds a second hand-mirrored block to the same pair, and its own amendment stated
  outright that nothing catches a missed half.
  Size: one existing gate widened plus a pair registry.
  **Feature-shaped — self-label corrected 2026-08-04 at close**, on the same read as
  its `workflow-permissions-scope-oracle` sibling: the label read "Debt" while naming
  the consumer knob it mints, and canon-kit/SPEC.md §The amendment lifecycle's litmus
  makes any such name a feature, so promoting it authors an amendment.
  Filed 2026-08-01 at close from the gap inbox, confirmed at this iteration's align
  audit against the full gate roster.

- **gate-tests-suite-identity-in-evidence** [design-pending] — the validate
  manifest's per-suite hash cannot tell two suites apart, so it certifies that
  *some* suite ran clean rather than that *this* one did. evidence-kit's
  `run-validate.sh` hashes each suite's captured stdout+stderr as the manifest's
  identity check, but the hash carries no suite name or fingerprint of its own — it
  is only as discriminating as the wrapped tool's output, and
  `gate-sdk/bin/run-gate-tests.sh` ends with `GATE-TESTS: clean ($pairs pairs,
  $unit unit tests)`, naming neither the kit nor the files it ran. Two kits with
  matching counts therefore produce byte-identical logs and identical hashes.
  Confirmed live this iteration and re-verified at close: `evidence_kit` and
  `site_kit` both carry
  `sha256=1065526da8cb11a7fd6176adfde255374b9bcb95a8240a6591ea47fa43367c6d` in the
  committed evidence, reproduced across independent re-runs. Verified
  genuine-by-construction rather than a mis-wire — the two suites resolve distinct
  commands against their own correct kit directories and genuinely have matching
  counts (2 pairs, 3 unit tests each).
  **Why `[design-pending]`:** two fixes with different reach. Having
  `run-gate-tests.sh` name its kit in the success line sharpens the hash for free
  and helps every log reader, but it fixes only this producer; hashing something
  suite-identifying (the invoked command) alongside the log bytes fixes the
  manifest for every wrapped tool, including ones this repo does not own. They are
  not exclusive and the second is the one that generalizes.
  **Cost while deferred:** the manifest is weak evidence exactly where it is
  load-bearing — it cannot separate "suite X's correct result" from "some other
  suite emitting the same generic message", so a silently-skipped or mis-pointed
  suite reads as certified. Charged per validate run.
  Debt: a success-line change and/or a hash-input change; adds no governed name.
  Filed 2026-08-01 at close from the gap inbox, filed by this iteration's validate.

- **always-loaded-brevity-reach** [design-pending] — `check-brevity` guards the
  tidiest section of the always-loaded surface while the section that actually
  grows is outside its reach, so the tier ratchets with no oracle. The gate's
  target is a single named section (`CONTEXT_KIT_BREVITY_SECTION`, this repo's
  conventions block, 17 lines); §Housekeeping is 59 of `CLAUDE.md`'s 189 lines —
  31% of the surface — and no gate reads it. Measured at this close: the
  always-loaded meter reads 197 lines against a committed baseline of 172,
  **+25 (+14.5%)**, and the baseline stamp dates to `v0.8.0`, twelve releases
  back. Every close's brevity pass is therefore the only detector, which is a
  session's attention standing in for a gate — the shape enforcement-first exists
  to refuse.
  **Deliberately not reset.** Re-stamping the baseline at this close would clear
  the drift signal by blessing the growth, which is the one thing the baseline is
  for. It stays where it is until the ratchet is actually addressed.
  **What this close found and left.** Four bullets carry a doc pointer and then
  restate what is behind it — the `.tmp/`/`.metric/`/`.workflow/` bullet (13
  lines, mostly mechanism the owning SPECs own, with only the `*.local.md` roster
  genuinely resident-worthy), the `installer/` bullet (8 lines naming
  `installer/README.md` as layout owner then carrying packing mechanics and a
  scratch knob), the `demo/` bullet (compresses a `README.md` paragraph), and
  `reserve/` (2 lines guarding a mistake no session is near). Each needs content
  *relocated*, not merely deleted, which is unit-shaped rather than pass-shaped —
  filed here rather than done under release pressure.
  **Why `[design-pending]`:** pointing the gate at the whole file needs a budget
  model it does not have. A single whole-file cap either strangles a legitimately
  dense file or is set so loose it never fires; per-section budgets need a section
  roster that is itself maintained. The candidate worth costing first is a
  **ratchet** assertion rather than a cap — red when the surface grows against its
  committed baseline without the baseline being deliberately re-stamped — which
  needs no budget at all and makes the growth visible where it happens.
  **The series, against the same unmoved baseline: 197 (+25) → 213 (+41) → 213 (+41).**
  The middle reading advanced 16 lines and looked like compounding drift. The third,
  at `native-cohort-activation`'s close, is the first that did **not** advance — and
  the mechanism is worth more than the number: the iteration added exactly one line to
  the surface and that close's pass recovered exactly that one line, both times from
  the same `native/` bullet restating what gate-sdk's SPEC owns. So a pass-shaped fix
  holds the line **only** against a pass-shaped increment. It recovered one of
  forty-one when the growth was unit-shaped, and one of one when it was not. The
  baseline stays unstamped through all three.
  **Cost while deferred:** compounding directly in the tier the whole methodology
  is trying to hold down, and paid by every session in this repo and every
  consumer that vendors context-kit. The detector is a close-stage read, so it is
  as reliable as the attention of whoever runs it.
  Debt: one gate widened or one assertion added; adds one knob if a budget lands.
  Filed 2026-08-01 by close's brevity pass, which measured the ratchet it could
  not close.

- **batch-split-stamp-ownership** [design-pending] — who stamps the per-session
  audit trail when a live lead splits one stage across several batch sessions is
  unowned. This iteration recorded **one** `build` line in
  `.workflow/WORKFLOW-STATE.txt` for five batch sessions.
  **Premise corrected 2026-08-01 by the undirected scope survey — the filed
  diagnosis was wrong and the next reader would otherwise re-derive it.** The
  entry said "the two surfaces disagree in effect", with
  `lifecycle-kit/templates/stages/build.md`'s "Every session still stamps"
  paragraph against a lead that "tells batches 2..n not to re-stamp". **The
  surfaces agree.** build.md:45 does say every build session re-runs
  `enter-stage.sh build`; `lifecycle-kit/templates/lead.md`:226-230 says batches
  are "N sibling stage sessions … each entering through `enter-stage.sh` as a
  same-stage re-entry … each leaving its own stamp and the cursor staying put" —
  the same instruction, not its opposite, and present since 2026-07-17, two weeks
  before the divergence. So the gap is **practice against instruction**, not
  instruction against instruction: the batches simply did not run the entry.
  Nor is the id the obstacle — dispatched stage sessions do resolve distinct
  session ids (this survey's own stamp differs from its lead's), so
  `bin/session-id.sh` was never the blocker.
  **Practice confirmed correct once, 2026-08-01.** This iteration's two build
  batches each wrote their own stamp — two `build` lines with distinct session
  ids in `.workflow/WORKFLOW-STATE.txt` — so a per-batch trail is achievable
  under the instruction both templates already carry, and the fork below is a
  live choice rather than a repair. A second limit confirmed alongside it:
  `bin/enter-stage.sh --simulate` **cannot predict a distinct session's entry**,
  so it never answers whether a sibling batch will stamp — it gates *this*
  session's entry only, which bounds any lead-side pre-dispatch check.
  **Not a defect today.** Nothing gates on the missing lines, and the stage
  cursor is the *last* stamp, so it is correct either way — this is doctrine
  drift, not breakage.
  **The ruling the entry wants,** restated against the corrected diagnosis: is a
  per-session trail worth restoring at all — the evidence file's stated contract
  is one line per stage-skill invocation, which a batch session *is*, so the fork
  is stamping per batch or narrowing the contract to per stage. If per batch, the
  second half is what makes an instruction both templates already carry actually
  bind, since prose alone demonstrably did not.
  **Why `[design-pending]`:** it either narrows a shipped stamp contract and
  `check-stage-evidence`'s reading of it, or adds an oracle where two templates
  now rely on a dispatched session's compliance; and it recurs on every batched
  stage, not just build.
  **Regressed 2026-08-02:** nine build batches, one stamp — the once-correct
  practice did not hold, and since economics attributes one row per transcript,
  the lead binding's per-batch tier lever is unmeasurable while the trail is not.
  **Cost while deferred:** the trail silently under-reports session count on
  every batched stage, so the evidence file cannot answer "how many sessions did
  this stage take" — an economics question the drift KPIs would otherwise want.
  Debt: one contract narrowed, or one oracle added; adds no governed name unless
  the oracle lands.
  Filed 2026-08-01 at close from the gap inbox, filed by this iteration's build.

- **template-spec-restatement-reach** [design-pending] — the gap generalization
  behind this close's resume-journal fix: `check-shim-restatement` holds
  **binding shims** (`.claude/commands/*.md`) to an n-gram-disjointness contract
  against a dedup corpus, and nothing holds the far heavier **template ↔ owning
  SPEC** pair to anything. Attested cost: `delegation-kit/SPEC.md` §Resume
  journal and `templates/agent-execution.md`'s resume-journal bullet drifted into
  near sentence-for-sentence restatement, and one build unit wrote the retention
  resolution and the `DONE`-as-last-line clause into **both** in parallel because
  no oracle held them to one owner. Close deduplicated them by hand; nothing
  stops the next lifetime edit doing the same thing again.
  **The mechanism already exists.** `check-shim-restatement`'s implementation module
  (`native/src/gates/shim_restatement.rs`) is the whole of it — normalize, emit every
  N-word window, intersect against a corpus index. The unit is a second (surface,
  corpus) pairing of that same machine, not new code.
  **Why `[design-pending]`, and the honest objection.** The shim contract is
  *bind consumer residue, cite kit-owned procedure* — a shim legitimately
  restates nothing. A SPEC legitimately restates its template's **contract
  clauses**, because a consumer reading the SPEC alone must learn what the kit
  requires. So a naive port reds on correct text, and the design question is what
  the exemption is: an explicit contract block the gate skips, a higher `N` for
  this pairing, or a direction-sensitive rule (the SPEC may cite the template,
  never the reverse). Answering that is the unit; porting the gate is an
  afternoon.
  **Cost while deferred:** paid per lifetime edit to any kit whose SPEC and
  template both discuss the same rule, and paid as a *silent* two-surface edit —
  the failure mode is a one-surface edit nobody catches, i.e. exactly the
  contradiction that build unit had to resolve.
  Debt: one gate widened or forked plus its fixture pair; adds one knob if the
  exemption is configurable.
  Filed 2026-08-01 by close, generalizing the gap it fixed inline.

- **guard-command-prefix-wrapper** [design-pending] — a transparent prefix
  displaces the token the guard matches on, so an already-allowlisted read-only
  command prompts anyway. Two shapes, one mechanism, measured at this close's
  prompt-friction triage as **35 of 108 prompting calls (~32%) — the largest
  class by a wide margin**: `time bash <allowlisted-script>` and
  `/usr/bin/time -f '<fmt>' bash <allowlisted-script>` (31), where the leading
  token is `time` rather than `bash`; and `git -c core.pager=cat <subcommand>`
  (4+), where the read-only-subcommand recognition reads `-c` instead of
  `status`/`log`/`tag`.
  **Why a guard rule and not an allowlist entry** — the triage criterion's own
  test. `Bash(time *)` would grant *anything* under `time`, which is the guard
  defeated rather than configured; the decision needs logic no static glob can
  express (strip the wrapper, then re-test the *wrapped* command against the
  committed allowlist with the matcher the guard already has).
  **Deliverable:** a generic-ruleset rule in `guard-kit/lib/guard.sh` that
  strips a recognized transparent prefix and re-tests via `guard_allow_match`,
  plus a `guard-tests/cases.tsv` pair and its SPEC rule entry. `scan-prompts.sh`
  already has half of it — its `strip_prefix()` handles `sudo `/`timeout ` for
  ranking purposes only, so the live hook and the scanner disagree about what is
  covered, which is its own small defect.
  **Why `[design-pending]`:** the wrapper roster is the design question. `time`
  and `git -c` are safe because they are transparent, but `env VAR=v <cmd>`,
  `nice`, `nohup`, and `xargs` are not uniformly so (`env` can replace `PATH`),
  so the rule needs a stated closed roster and a reason for its boundary rather
  than a "strip anything that looks like a wrapper" heuristic.
  **Cost while deferred:** roughly a third of all permission prompts in an
  iteration, all of them on commands the operator already blessed — the pure
  interruption cost this loop exists to retire, and it grows with profiling work
  (this iteration was a performance iteration, which is why `time` dominated).
  **Tier: feature, owing an amendment — corrected 2026-08-13 at close.** The entry long carried
  "Debt: … adds no governed name to a shipped surface"; that self-declaration is false against
  the tree. guard-kit's generic ruleset is a numbered SPEC roster backed by named
  `guard_rule_*` functions in `lib/guard.sh`, so a new rule adds a name to a
  governed surface, a contract consumers honour, and a closed transparent-prefix roster.
  Only the tier label is corrected here; the entry's substantive claims were not re-examined.
  Filed 2026-08-01 by close's prompt-friction triage; tier corrected 2026-08-13 by close from the
  gap-inbox drain, after the operator ruled the entry out of that iteration's unit set.

- **release-note-section-set-derivation** [design-pending] — the release note's
  fixed-section set is held by `check-release-bump` as parallel
  hardcoded calls, so a fifth section is added by copying a call rather than by
  extending one roster, and **no gate asserts that the gate's set equals the
  page's**.
  **The disease this is the residue of.** The section *count* was hand-maintained
  as a literal across `gate-sdk/bin/upgrade-smoke.sh`,
  `check-tightened-gates-grammar`, `check-release-bump`'s spec
  comment, `gate-sdk/SPEC.md` and several `docs/install.md` sentences, while the
  set itself was separate hardcoded calls with no array and no length. Nothing
  derived the number and nothing reddened when it was wrong — the test "if this
  number were wrong tomorrow, does anything red?" answered no at every site. It
  had already rotted: adding one section took a by-hand three-to-four edit at
  every site, and align's census of those sites — itself a hand roster — omitted
  `upgrade-smoke.sh`, which build's re-run caught. **The census only had to exist
  because nothing derived it.** The prose half is closed: every count is gone and
  the prose now names two *classes*, with `docs/install.md` §The upgrade contract
  owning the roster by enumerating it.
  **A ruling that must not be re-derived.** The obvious consolidation — put the
  set in `gate-sdk/lib/declaration.sh` beside the shared container — is **ruled
  out**: those section names are consumer content, and a kit literal carrying one
  project's release-note vocabulary crosses the provenance seam (CLAUDE.md §The
  provenance seam), the same ground on which the note-parity gate stays out of
  gate-sdk. So the unit is consumer-side: one rostered set in this repo's
  `scripts/`, derived from or gated against the page.
  **Why `[design-pending]`:** the seam ruling fixes *where*, not *how* — derive
  the gate's set from the page at runtime, or hold both and gate the parity. The
  two differ in fail-closed behavior when the page is unreadable, which is the
  open call.
  **Cost while deferred:** a fifth section is added by copy, and the gate's set
  can silently diverge from the roster the page owns — the exact shape that
  survived a dedicated audit stage once already.
  Debt: converges a duplicated set onto one roster; adds no governed name unless
  the parity arm becomes its own gate, which the promoting scope call settles.
  Filed 2026-08-01 at close from the gap inbox, filed by this iteration's build
  on an operator-raised doctrine check.

- **amendment-deletion-content-completeness** [design-pending] — a closed
  amendment can be **deleted with part of its content landing in no canonical
  spec**, and the merge rule has no completeness oracle at deletion time.
  **Observed, not hypothetical.** `SPEC-supply-chain-trust-baseline.md`'s
  causal-chain rationale (its A1) survives only in git history, so the next
  amendment on that surface **re-derived** it rather than inheriting it — the
  re-derivation is the cost, paid in full.
  **Why `[design-pending]`:** spec-over-precedent makes git history a
  non-canonical tier, so "recoverable from the deleting commit" is not an
  answer here the way it is for an icebox eviction — an evicted queue body is
  dormant work, whereas a merged rationale is *live doctrine* a later reader is
  entitled to read forward. The oracle is the hard part: deciding that every
  claim in a deleted amendment landed somewhere canonical means diffing prose
  meaning, not tokens. Cheaper shapes to weigh first — require the deleting
  commit to name each destination section per amendment section, or forbid
  deletion outright and require an explicit `merged-into:` field the gate can
  resolve.
  **Cost while deferred:** unbounded and silent — each deletion may drop
  rationale, and the loss is invisible until someone re-derives it, which is
  exactly what happened here and cost this iteration a spec cycle.
  Filed 2026-08-01 at close from the gap inbox.

- **template-registry-population-predicate** [design-pending] — a **contingent**
  residual in `check-template-registry-parity`'s population predicate
  (gate-sdk/SPEC-template-registry-parity.md, closed): a `templates/<X>.list`
  enters the parity population iff a sibling `<X>/` directory exists. If a kit
  ever ships a `.list` template of *consumer rule content* whose basename
  happens to match one of its own directories of kit-shipped `.sh` artifacts,
  the predicate admits a private vocabulary into a parity check it can never
  satisfy. No such case exists: two `.list` templates exist tree-wide
  (`kpis.list`, `msg-patterns.list`), and the predicate is sound against both.
  Why this is filed rather than fixed: the direction is a **false red**, which
  announces itself the moment the colliding template lands, and the obvious
  pre-emptive fix — an exception naming the private-vocabulary template — is
  the kit literal the provenance seam refuses, which is precisely why the
  predicate is structural in the first place. The residual is the price of the
  seam-respecting design, not a defect in it.
  The one thing that is not free: the gate is `valve=none`, so a consumer who
  hits the collision has no suppression path short of renaming their own
  template. If this is ever worth work, the work is a valve rather than a
  widened predicate.
  **Cost while deferred:** near-zero and non-rotting — a self-announcing false
  red on a case no tree has produced. Not iceboxed only because a named event
  waits: the next `.list` template landing in any kit is the moment to re-read
  this entry.
  Surfaced 2026-08-01 by `kit-template-registry-completeness` while writing the
  predicate; flagged then as contingent, not a present defect, and re-verified
  against the shipped gate at close.

- **companion-toolkit-profile** [design-pending] [roadmap: next/ecosystem] — the interop rung.
  roadmap-summary: Gate a tree whose specs another toolkit's workflow wrote.
  Govern a tree whose specs an **external spec-authoring toolkit produced** — a
  consumer profile for when the specs Checkwright gates were written by a second
  toolkit's workflow, not by this one's `spec` stage. It cashes the claim below.
  **The design is already decided and is not what this entry holds.** Two
  rulings on record settle it: `prose-profile` rules that a profile ships as an
  adapter delivered as optional consumer config and never as a kit literal, and
  `heterogeneous-agent-delegation` rules that a kit literal naming a vendor
  crosses the provenance seam outright. So the shape is a consumer-side profile
  over a declared artifact layout — the `check-graph` / `graph-vocab` pattern —
  and any per-toolkit specifics stay in consumer config. What is open is the
  *substance* — which lifecycle assumptions break when the amendment set is authored
  elsewhere, and whether a tested two-toolkit consumer is buildable without a kit
  ever naming one.
  **Survey run 2026-08-02 at scope — three corrections, so a spec pass starts here.**
  (1) *Cheaper than filed:* the load-bearing knobs already exist as consumer config
  (`CANON_KIT_SPEC_NAME`, `_AMENDMENT_GLOB`, `_QUEUE_FILE`, `_DOD_MODE`;
  `LIFECYCLE_KIT_AMENDMENT_GLOB`, `_CONTRACT_TOKENS`). The work is *proving them
  sufficient*, not inventing a profile format. (2) *The sharpest break is a silent
  one:* `check-stage-entry` assertion C reads literal `SPEC.md`/`proto/` substrings
  inside amendment bodies as its cross-component signal, so a foreign layout makes it
  **never fire** — the align audit is skipped with no red. An interop consumer is
  not merely unsupported, it is silently under-gated. `check-spec-pointer` breaks the
  same way on non-markdown artifacts. (3) *The deepest coupling is process, not
  config:* `check-spec-derivable-section`/`check-spec-embedded-source` assume the
  canonical-spec-plus-short-lived-amendment model itself, which a toolkit keeping
  many living per-feature specs does not fit at any knob setting.
  **The discharge pattern already exists in-tree.** docs/positioning.md §The tiered
  compatibility claim says "This is tested, not asserted" and cites context-kit's
  `smoke/agents-md.sh`. That is the shape the three claims below owe.
  **Seam re-verified clean:** no tracked file names an external spec toolkit.
  **Intake provenance:** never declined or costed — the opportunities half of the
  same operator-commissioned external review whose *weaknesses* half was filed
  2026-07-23 as six launch-facing rungs, all since landed. That intake's filter was
  the review's "top pre-announcement gaps", so the growth half fell consciously
  outside a stated filter rather than being missed; the gap here is the absent
  intake record, not that session's judgment.
  **Cost while deferred — not zero, and this is the entry's sharpest fact.**
  `README.md`:16-17 and `docs/index.md`:17-18 both already assert, on the first
  screen, "It complements the workflow you already run. Keep your spec process, your
  prompts, your harness." — and docs/orchestration.md:22-23 makes the same move for
  orchestration ("It complements your orchestration setup; it does not replace it"),
  a third site found 2026-08-02 at scope. **No queue or roadmap entry backs any of
  the three with a tested consumer.** Not false — "complements" is far weaker than
  "integrates with X" — but *published and unproven*, in a project whose whole pitch
  is that claims are mechanically proven rather than asserted. The carry is
  reputational and front-door-resident, accruing on every reader rather than with
  time. Surfaced 2026-08-02 at close, intake pass over the review's unfiled half.

- **design-partner-preview** [design-pending] — a narrow external preview before
  any broad announcement: a narrow external preview cohort, its composition ruled
  in the operator's brief, installs observed live rather than by written feedback,
  instrumented for
  time-to-first-green, first useful red, false-positive dispositions, and 7/30-day
  retention per kit. It is the first rung on this queue whose deliverable is
  **evidence from outside this tree** rather than a tree change.
  **The cohort was a named population here until 2026-08-09 and is deliberately no
  longer one.** The composition it stated had since been re-ruled, so the sentence
  contradicted the ruling it was meant to carry; the fix is to name the owner rather
  than to re-state a ruling this file does not hold. The rung's own sequencing is
  unchanged, and it now depends on `prose-profile` shipping.
  **Sequencing is the load-bearing part.** The preview runs *before*
  `benchmark-ab-experiment`, so pilot findings shape that experiment's task
  classes and metrics rather than being retrofitted to them; per-gate
  true/false-positive history and profile retention are preview deliverables,
  not pre-launch builds. Broad announcement waits on the activation path, the
  trust baseline, and two externally observed defect stories. The full launch
  ruling behind this sequencing is operator material and stays in the local-only
  private brief; this entry carries only the queue-visible rung.
  **Intake provenance:** item 4 of the external review's own priority order, and
  like `companion-toolkit-profile` it fell outside the 2026-07-23 intake's
  "top pre-announcement gaps" filter rather than being judged and declined —
  consciously out of scope for that pass, with no record left behind.
  **Cost while deferred:** the highest of this intake, and it compounds. Every
  claim in the tree that would be strongest with external evidence — false
  positives, retention, time-to-first-value — stays supported by internal
  dogfooding alone, and no external install has ever been observed. That is not
  a rot risk that ages passively: each iteration adds governed surface whose
  quality is unattested outside, so the volume of unattested claim grows while
  the evidence stays at zero. Deferring also silently defers
  `benchmark-ab-experiment`, since running it first would fix the wrong metrics.
  Surfaced 2026-08-02 at close, in the same intake pass, as the review's own
  fourth-ranked priority.

- **external-gate-quality-evidence** [design-pending] — durable, published
  evidence of **gate quality as experienced outside this tree**: per-gate
  true/false-positive history, the disposition of each red a non-author hit, and
  whether a red changed behaviour or was worked around. The review's own
  suggested new initiative, and the direct answer to the standing threat that a
  false positive converts the enforcement advantage into bypass and distrust —
  every blocking gate raises the stakes of a wrong red.
  **Why it is not just a report.** The tree already publishes evidence
  projections, so the mechanism exists; what does not exist is a *population* to
  measure. A red in this repo is authored and dispositioned by the same party,
  which cannot distinguish a gate that is right from a gate whose author agrees
  with it. That makes this entry structurally downstream of
  `design-partner-preview` — not blocked by it in the tag sense, since a
  narrower internal cut is conceivable, but the honest version needs external
  reds. The open design question is which of the two it should be, and whether
  the collection surface is a per-gate field the disposition already records or
  a new capture stream.
  **Intake provenance:** the third unfiled item of the external review's growth
  half, outside the 2026-07-23 intake's stated pre-announcement filter rather
  than declined by it.
  **Cost while deferred:** moderate and asymmetric. The gate-quality claim is
  currently supported by fixture pairs and a green battery, which prove a gate
  does what its author specified and say nothing about whether that was the
  right thing to specify — exactly the gap this measures. The carry is that the
  first externally-hit false positive will be argued from anecdote, because no
  baseline exists to argue from. It does not rot, but it cannot be
  retroactively collected either: the history it wants starts accruing only once
  someone decides to record it.
  Surfaced 2026-08-02 at close, in the same intake pass, as the third and last
  of the growth half's unfiled items.

- **lead-line-parser-conformance** [design-pending] — eight independent holders
  re-implement queue-kit's bullet lead-line predicate, coupled only by SPEC
  prose, and no mechanism catches divergence. The census and the residue
  accounting are owned by gate-sdk/SPEC.md §check-gate-exemption-tasks and are
  deliberately not restated here; this entry owns the *mechanism*, which that
  section explicitly leaves to a different unit.
  **The re-implement-and-cite-from-both-ends rule is NOT being re-litigated.**
  queue-kit/SPEC.md §The queue format and the gate-sdk section above both state
  it, for two different reasons (a cross-kit cycle for drift-kit, a layering
  inversion for gate-sdk), and that ruling stands. What is missing is any oracle
  over agreement.
  **The hard constraint on any candidate, and the reason a naive one reds on
  wanted behavior:** the holders diverge on the section **span** deliberately —
  drift-kit's `kpi-deferred-age` resets on an unknown heading and so excludes the
  icebox tier, `check-gate-exemption-tasks` does not reset and so includes it,
  and **both behaviors are wanted**. So a conformance mechanism must scope to the
  **line predicate alone**.
  **Second divergence axis, verified at this close and not in the original
  filing:** the `queue-index` arm's default index-mode walk matches lead lines
  with a column-0 `/^-[[:space:]]/` anchor rather than the tolerant
  `^[[:space:]]*-[[:space:]]+` one, so indented sub-task bullets are invisible to
  it. That is *sanctioned* — the gate-sdk section rules the indent level the
  reader's choice and the predicate not — but it means a conformance mechanism
  must also decide which level it holds, or it reds on a holder exercising a
  sanctioned narrowing. A whole-holder text-identity assertion is therefore
  already known to be wrong.
  **Candidate shapes, none ruled:** a conformance test asserting every holder
  returns the same slug set for one fixture queue; a gate asserting the line
  predicate's text is identical across declared holders; or accept the residue and
  merely roster the holders so the next format change is findable. The third is
  also the form that would retire the hand-maintained census in the gate-sdk
  section, which is the Derivation-first half of the prize.
  **Why `[design-pending]`:** the holders span four kits with a layering ban
  between two of them, so where the shared fixture or the declared-holder roster
  *lives* is a cross-kit ruling before it is a script.
  **Cost while deferred:** a format change costs all eight edits and endangers
  the few that build sets — a set builder with a wrong predicate fails silently,
  in wrong membership, which is `check-gate-exemption-tasks`' own shipped defect
  class. Two of the holders are inline scans no function-name grep finds, which
  is how the census was twice under-stated before a whole-tree survey ran.
  Non-rotting while the format holds still; the exposure is entirely at the next
  format change.
  Filed 2026-08-02 at spec by the causal-completeness reader survey for
  `gate-exemption-live-slug-derivation`, explicitly scoped out of that amendment;
  promoted at close from the gap inbox, its census re-verified against the tree
  rather than re-derived from the filing.

- **unqualified-section-citation-liveness** [design-pending] — a bare `(§Heading)`
  citation in governed prose — the same-file form, with no `<path>.md` prefix —
  resolves to nothing and no gate reds. The two citation gates each miss it from
  a different side: `check-md-refs` resolves markdown *links* only, and
  `check-spec-pointer`'s prose extractor requires a `<path>.md §` prefix before it
  will look (the `match(s, /…\.md[[:space:]]*§/)` guard), so the qualified form is
  held and the unqualified one is not.
  **Instance, fixed at this close:** gate-sdk/SPEC.md cited `§check-fixture-pair`
  as the authority for "a registered gate still owes its `good/`+`bad/` pair"; no
  such section exists — the gate is `check-gate-fixture-coverage`, named correctly
  seven lines later in the same paragraph. A reader chasing the load-bearing half
  of that sentence landed nowhere.
  **The class is large and currently unverified:** a mechanical sweep of the kit
  SPECs plus `CLAUDE.md` and `DOCTRINE.md` counted 171 parenthesized unqualified
  citations, against which a crude same-file resolver flagged 11 — one genuine
  (the instance above) and the rest artifacts of the crude predicate. Those
  artifacts are the design: they are the false-positive classes a real gate must
  answer for.
  **Why `[design-pending]` — three named false-positive classes, all observed in
  that sweep.** (1) **Cross-file antecedent:** `site-kit/SPEC.md` cites
  `(§run-gates)`, a gate-sdk section, and gate-sdk/SPEC.md cites
  `(§check-spec-pointer)`, a canon-kit one — bare, with the owning path named
  earlier in the paragraph. `check-spec-pointer` already implements a blank-line
  paragraph join for the qualified form, so the antecedent rule has an
  implementation to extend rather than invent, but whether a bare citation should
  bind to the nearest prior path or be required to qualify itself is the ruling
  this entry owes. (2) **Prose tail:** `(§Bundled KPIs / §Layout and
  configuration)`, `(§run-gates owns the contract)` — where the heading name ends
  and prose resumes is not syntactic. (3) **Possessive and inflected forms:**
  `(§check-stage-entry's trigger-gated-stage calibration)`.
  **Shape:** an assertion inside `check-spec-pointer` rather than a new gate — it
  already owns heading resolution in both exact and prefix modes, already walks
  the manifest set, and already carries the paragraph join. Sibling to
  `prose-filename-citation-liveness`, which is the same family (a citation form
  falling between the two gates) from the other end: that one is a path with no
  heading, this one a heading with no path. A promoting scope should cost them
  together and may find one predicate covers both.
  **Cost while deferred:** 171 live unqualified citations across the governed doc
  set, held by nothing. Each merged amendment and each renamed section can strand
  one, and the only detector is a reader who follows the pointer — which is how
  the instance above was found, one iteration after the paragraph was rewritten.
  Debt: an assertion added to an existing gate and its fixture arm; adds no
  governed name — the new-names litmus does not fire, which is what separates it
  from `prose-filename-citation-liveness`' shipped-path variant.
  Filed 2026-08-02 at spec to the gap inbox as the dangling pointer; promoted at
  close as the gap-generalization that inline fix owed, with the coverage gap
  verified against both gates' source and the class sized by sweep.

- **gate-authoring-sdk-surface** [design-pending] [roadmap: next/ecosystem] — a gate-authoring SDK.
  `.gate` as the substrate-neutral surface. **Operator-surfaced during
  `native-gate-dispatch-seam` build; filed so the framing outlives the session that
  saw it.** Horizon set 2026-08-02 on the operator's steer: this is ecosystem work
  on `companion-toolkit-profile`'s rung, not "make our own gates fast and opaque".
  roadmap-summary: Author a gate in any language behind one substrate-neutral descriptor.
  **The observation:** because the manifest lives outside the implementation, the
  graph, hook, and meta-gate layers never learn what implements a gate. That is not
  a Rust seam that happens to work — it is a **language-agnostic** one. A gate could
  be written in any language behind a descriptor, and slice 1 already avoided baking
  a language into the descriptor format, the resolution path, and
  `check-gate-substrate-parity` assertion D (whose comment-leader match is `#`, `//`
  and `/*` deliberately). `GATE_SDK_NATIVE_SRC` is a path knob, not a language knob,
  for the same reason.
  **Why it is an SDK question and not a port question:** the port asks "how does
  *this* gate move"; this asks what a **third party** needs to author a gate on any
  substrate — the descriptor contract, the subcommand calling convention, the output
  contract, and the fixture-pair obligation, which are already the four things
  gate-sdk holds. The kit is most of an SDK already; what is missing is the statement
  that the substrate is a parameter.
  **Boundary against the two questions already settled, so this one does not sprawl:**
  how a compiled gate *arrives* and what it *discloses* are both ruled and recorded
  (TRAJECTORY.md §The closed rulings; gate-sdk/SPEC.md §Consumer payload). What stays
  open is what a gate *is* independent of substrate, and that is this entry alone. The
  distinction it supplies outlived those rulings and is why it was worth keeping — a
  descriptor discloses a gate's **shape** without its **predicate**, which is the line
  the disclosure ruling drew.
  **Not started, and deliberately not widened into slice 1**: building it would have
  meant generalizing a seam with exactly one instance, which is the shape of a design
  that fits nothing later.
  **Cost while deferred:** each further port hardens substrate-specific assumptions
  by habit rather than by ruling, and the cheapest moment to keep the seam neutral is
  before the second language exists — not after.
  Filed 2026-08-02 by build, on an operator ruling, during `native-gate-dispatch-seam`.

- **queue-entry-evidence-tier** [design-pending] — **not "how is cut detail recovered"
  — that is solved. The gap is that a reader cannot tell detail was ever cut.**
  `check-queue-entry-budget`'s own help names the recovery path
  (`git log -p -S'<slug>'`), which is tracked, public-safe and needs no session
  transcript. What no surface carries is the *signal*: a compressed entry looks
  identical to an entry that was always short, so a session with a question it cannot
  answer has no way to know an evicting commit holds the answer, nor which one.
  Reproduced 2026-08-02: one scope session pushed `native-gate-binary-port` over the
  cap three times and compressed it back, and nothing in the entry now records that
  the worked arguments exist upstream.
  **The budget is gated and the gate works — this is not ungated drift.** The cap is
  50 and it is measured on the entry's **raw extent**, blanks included (assertion A,
  `n = bound - o_start[i]`), not on content lines; content lines are counted only for
  the icebox shape rule. It red thrice on that entry this session and forced each
  compression, so the discipline held. What it cannot do is say where the cut went.
  **A design constraint that is not obvious from the queue's prose, verified in the
  gate's awk:** a nested sub-task is counted into every open parent
  (`for (i = 1; i <= nopen; i++) o_nb[i]++`), so **sub-tasks do not relieve a parent's
  budget — only a companion top-level entry does.** A mechanism that routed overflow
  into sub-tasks would be defeated by the gate it has to live under.
  **One more constraint:** the repo is public and CLAUDE.md forbids internal session
  references in tracked files, so no entry may point at a transcript. And under
  spec-over-precedent an evicting commit evidences *that* a thing was costed, never
  *that the costing is right* — so the signal must lead to a ruling surface, not stop
  at history.
  **Empirical support, measured 2026-08-02 by spec.** Recovery via `git log -p -S`
  **worked**: every ground dropped from this entry's own reproduction case came back
  intact from the filing diff. What failed was *signal* — nobody knew to look, and the
  arguments were supplied from memory twice the same day instead. That is evidence for
  the narrowing already applied here, not grounds to re-open the shape.
  **One candidate mechanism, evaluated at close 2026-08-02 and recorded unproven.**
  The gate cannot judge loss, but it may be able to detect the *shape of the
  sanctioned remedy*: a commit that shrinks a deferred entry past some delta while
  adding no linked entry in the same commit. Two known costs, both real: it needs
  history access no battery gate has today, and it false-positives exactly where a
  genuinely answered ground is deleted — which is the case the authorization ruling
  in queue-kit/SPEC.md §check-queue-entry-budget already governs. Recorded so a
  later session weighs it rather than re-invents it; it is not a design decision.
  **Cost while deferred:** compression stays silently lossy in practice — the bytes
  survive in git, and the knowledge that they are worth fetching does not.
  Filed 2026-08-02 by scope on operator intake. The operator previously proposed an
  evidence-submission mechanism and it was declined in favour of boxed entries; this
  narrower shape is the ground for revisiting that ruling.

- **gate-tamper-exemption-reader-substrate** [design-pending] — `check-gate-tamper`'s
  exemption reader has no implementation-side equivalent.
  Split 2026-08-09 at scope by operator ruling from `gate-tamper-roster-native-reach`,
  when that entry narrowed to its meta-path-roster half and promoted; this is the
  exemption half, unchanged in substance. That entry was itself split 2026-08-02 from
  `native-gate-meta-layer-reach`, so this is the second narrowing of one original gap.
  `extract_exemptions()` parses a shell `# exception-list:` array literal, so a ported
  gate's Rust module can carry no exemption the gate is able to read.
  **Why `[design-pending]`:** it wants the ruling `gate-authoring-sdk-surface` holds
  — whether a meta-gate reads a substrate-neutral descriptor or learns each
  substrate — and that entry is horizon-set to ecosystem work, so this one waits.
  **The coupling was checked at the split rather than inherited.** It is true of this
  half and was not true of the roster half: which paths a tamper roster covers is
  configuration, where how a meta-gate reads an exemption across substrates is exactly
  the substrate-neutrality question the SDK entry holds.
  **Cost while deferred:** zero until a ported gate needs an exemption; no first-cohort
  member carries an exemption list, which gate-sdk/SPEC.md §Meta-gate conservation for
  the binary substrate records in its `check-gate-tamper` row.
  Filed 2026-08-02 at close from the gap inbox; found by build. Split out 2026-08-09
  at scope.

- **gate-timing-baseline-comparability** [design-pending] — a baseline with no
  comparer decays into a number nobody can use. `battery-baseline-capture` shipped
  `.workflow/gate-timing-baseline.txt` this iteration — 94 per-gate rows × 3 runs,
  captured deliberately *before* any gate ports off bash. Its extension and tier
  are correct (checked projection, `# contract:` pointer header, a stated
  field-wise line grammar — audited at close under the roster's
  `workflow-surface-extension` class). What it has is **no reader**: nothing in the
  tree parses it, and `kpi-gate-runtime` reads a single live timings file rather
  than a baseline-versus-current pair, though `DRIFT_KIT_TIMINGS_FILE` shows the
  seam where a comparer would attach.
  **Why that is not merely "not yet" — and the premise has since expired, which
  raises it rather than settling it.** The comparison moment is the second port. It
  was argued here as far off, on the ground that it waited on the operator-gated
  first tag publishing binaries. **Both halves of that ground are discharged:** the
  tag was cut at `v0.22.0` (2026-08-07) and the second port is built and proved
  (gate-sdk/SPEC.md §What is retained, and where the second port stands — every
  member of the first cohort ships its rule as a compiled subcommand). So the moment this entry
  says the baseline "must be ruled before" has **arrived**, and the entry is now
  overdue rather than parked. Corrected in place at the 2026-08-09 close's
  premise-rot review; the re-disposition is scope's to make, not close's.
  Meanwhile the baseline's own validity conditions decay. Two already have,
  measured at close the same iteration the file landed: the header pins a
  `gates.list` sha256 that no longer matches (the 95th gate landed after capture),
  and the header's environment line pins a kernel, bash build and CPU count that a
  later comparison will almost certainly not reproduce. The pins are doing their
  job — they make the drift *visible* — but nothing reads them, so nothing will
  refuse an apples-to-oranges comparison when it is finally made.
  **Why `[design-pending]`:** the choice is between a comparer that normalizes
  (re-measuring the shared gate set on the current box, which makes the stored
  numbers advisory) and a freshness gate that invalidates the baseline on roster
  or environment change (cheap, but it deletes the only pre-port measurement the
  repo will ever be able to take). Those are opposite answers, not a size question.
  **Held out of the `native-port-unblocking` iteration 2026-08-02 by operator
  ruling — a trigger, not a dismissal.** That iteration unblocks a second port
  without landing one, and this entry's surfaces (`.workflow/`, drift-kit) share
  nothing with the gate-sdk SPEC surface the iteration amends, so it buys no
  amortization. It becomes urgent the moment a second port is *scheduled*: that is
  the comparison moment, and the baseline must be ruled before it arrives.
  **Staleness measured rather than asserted, recomputed 2026-08-02 at scope so the
  next reader need not:** the header pins gates.list sha256
  `bd6c1cc8f8b89154d03fd80d4609d2b29efe82b5971dded6017564366923d5c9`; the file today
  hashes `19074f2be101376afb8b490a7ad08cf3cef303b4e97e40bec6f3619247a7800e`. The
  registry stands at 95 entries against the baseline's 94 rows.
  **Cost while deferred:** the port's headline justification stays unmeasurable in
  the one tree that could measure it — the before-numbers exist and quietly stop
  being comparable to any after-numbers, with no surface saying so.
  Filed 2026-08-02 at close by the `workflow-surface-extension` roster audit.

- **meta-gate-conservation-record-reach** [design-pending] — the conservation record
  under-describes what `check-gate-tamper` refuses, on both halves.
  `gate-sdk/SPEC.md` §Meta-gate conservation records the `native/`-excluded roster as
  refusing "a commit editing a gate's Rust implementation alongside its descriptor",
  mitigated by "neither binds slice 1". Verified 2026-08-03 by the
  `native-gate-meta-layer-reach` build, which the gate blocked.
  **Both halves under-describe it.** The refused co-staging was the crate alongside a gate
  *script*, not a descriptor — the rule is every gate file, and the recorded wording names
  only the descriptor case a reader is least likely to hit. And it now binds *routinely*
  rather than not at all: every crate-plus-gate change of the shape this iteration opened
  must land as two commits.
  **Deliverable:** the wording correction, plus a decision on whether the two-commit split
  is the intended contract or a roster gap.
  **Why `[design-pending]`:** the decision is the entry. The split is arguably the right
  outcome — a meta-gate that let an implementation and its own guard land together would
  be conserving nothing — but that is not what the record predicts, so correcting the
  prose without ruling the contract only relocates the surprise.
  **Cost while deferred:** a reader planning a port budgets against a record that
  understates both the rule's reach and its frequency, and meets the two-commit tax at the
  pre-commit hook instead.
  Filed 2026-08-03 at close from the gap inbox; found by build batch 1.

- **couples-dynamic-root-resolution** [design-pending] — the skipped-and-counted bucket is
  where trigger-drift hides, and its dominant subset is decidable.
  Verified 2026-08-03: `check-reads-couples`' root resolver handles a bare quoted literal,
  a KIT-prefixed token and a REPO_ROOT-prefixed token, and nothing else — so a walk root
  held in a *variable* falls to skipped and is never checked.
  **The assertion-D instance has since LANDED — corrected 2026-08-14 at scope**, which the
  prose below claimed open and no longer does. `check-gate-substrate-parity`'s manifest now
  reads `couples=…,native/*,…`; the generated hook guards it with `staged_matches …
  'native/*' …`, and `staged_matches` compares with bash `[[ "$f" == $pat ]]`, where `*`
  crosses `/` — so a crate-source edit **does** re-fire the gate. Assertion E's `couples=`
  widening is evidently what paid it. Kept because a later reader meets the same trap: the
  entry read as a live enforcement hole for an unknown period after the hole closed, and a
  scope ranking off it ranks a fixed instance as currently-broken.
  **The class is untouched and is now this entry's whole substance** — the resolver, not any
  one gate. A pinning instance must therefore be chosen from the still-skipped set rather
  than inherited from assertion D; which one is open, and enforcement-first still binds
  whichever is chosen to the check that catches the class.
  **The decidable subset, with no false-positive surface:** an assignment whose value is a
  parameter expansion with a literal default — the universal kit idiom CLAUDE.md
  §Conventions declares. Following one such assignment in the same file would have caught
  assertion D.
  **Deliverable:** that resolver extension, a pinning instance drawn from the still-skipped
  set, and a named cadence for the residue.
  **Why `[design-pending]`:** the residue is the open design. A truly dynamic root stays
  undecidable, so per the enforcement-first false-positive carve-out it needs a named
  cadence rather than a bare counter — and today the count is printed and nothing reviews
  it. Whether that cadence is a roster class, a close step or a threshold is unruled.
  **Cost while deferred:** every gate whose walk root is a variable is uncoupled and
  silently under-triggered, with no way to tell which. Measured 2026-08-14: `check-reads-couples`
  reports 0 resolvable walk(s) covered against 37 undecidable skipped-and-counted, and
  nothing reviews the 37 — so the next such hole is found the way assertion D's was, by
  someone noticing.
  Filed 2026-08-03 at close from the gap inbox, merging the assertion-D instance into its
  class; both found by build batch 2.

- **amendment-landing-citation-assertions** [design-pending] — nothing validates an
  amendment's landing citations, and align fixed five instances bare.
  Align corrected five amendment-citation defects with no mechanism named and none landed
  — the enforcement-first stop signal (doctrine-kit/DOCTRINE.md: a green instance fix is
  the stop signal to ask what check should have caught it). `check-amendment-queue` is the
  only registered amendment gate and asserts the bidirectional queue-amendment rule and
  spec-readiness only.
  **The class splits by decidability, and two thirds of it is gateable at zero FP.**
  *(1)* Every delta the amendment marks as landing carries an "Existing sections updated"
  entry — decidable from the amendment file alone, and it catches four of the five
  (vendoring deltas 3 and 9, meta-layer deltas 1 and 2). *(2)* Every citation's named file
  exists and contains the named heading — the same shape as the repo's existing `spec:`
  pointer resolution. *(3)* **Not gateable, high FP:** whether a delta prescribes an edit
  its target actually needs. Vendoring delta 11 prescribed a cargo-bullet treatment for
  `installer/README.md` §Requirements, which carries no such bullet at all. Per the
  false-positive carve-out this becomes a stated manual duty whose cadence event is the
  align stage — which already performs this tree-verification by hand.
  Also in the class: `gate-payload-disclosure-ruling`'s crate-path knob was credited to
  `native-gate-vendoring-model`, which declares no such knob.
  **Deliverable:** assertions (1) and (2) inside `check-amendment-queue` — extending beats
  adding, and its `couples=` already reach `TASK-QUEUE.md` and the amendment glob — plus
  (3) written into the align stage template as a named duty.
  **Why `[design-pending]`:** (3)'s placement is the open call. A duty stated in a stage
  template is only as good as the stage that reads it, and this iteration's own evidence
  (`enforcement-first-load-trigger`) is that a stage template is not a reliable carrier.
  **Cost while deferred:** an amendment can claim a delta landed, name a file that does not
  carry it, and merge green — after which the claim is deleted with the amendment and
  nothing records that it was false.
  **The deferred cost was paid 2026-08-04, exactly as written.** A build batch found a
  false citation in an amendment that align's audit had already passed as
  zero-divergence — assertion (2)'s case, and the second stage to miss it by hand.
  That align passes this class by eye is now measured, not assumed, which retires the
  argument that the manual duty under (3) can also carry (2).
  Filed 2026-08-03 at close from the gap inbox; found by align.

- **amendment-owner-position-citation** [design-pending] — prose may cite a merged
  amendment as *settled history*, never as a live *owner*; today it does both.
  recurrence: amendment-owner-position-citation 2026-08-06
  Amendments are deleted on merge (canon-kit/SPEC.md §Merging an amendment), so a citation
  naming one in owner position dangles the moment it merges — by construction, not by
  oversight. This is the inverse direction of the class
  `amendment-landing-citation-assertions` covers (amendment-to-spec landing citations) and
  of `amendment-deletion-content-completeness` (content that fails to land): here the
  canonical side is the one that goes stale.
  **Re-verified 2026-08-03 at close against the tree, and half the filing was already
  false.** The `gate-sdk/SPEC.md` instances the base bullet named are **gone** — build
  batch 5 repointed them to `TRAJECTORY.md` when the ruling record landed, and the slug now
  has zero occurrences in that file. The live residue is entirely inside `TASK-QUEUE.md`
  deferred bodies, and the sweep found **five**, not the two validate filed: three more
  naming `gate-payload-disclosure-ruling` and one naming `native-gate-meta-layer-reach`,
  all phrased in present tense against slugs that are now bare `## Done` lines.
  `check-spec-pointer` catches none — it matches path-form only, and all five are slug-form.
  **Recurred 2026-08-06 on a second surface that re-verification did not cover, falsifying its
  "entirely inside `TASK-QUEUE.md`" scoping.** Sibling amendments *inside one iteration* cite
  each other by path, so merge order decides which side dangles: `SPEC-verify-verb.md` cited
  `evidence-kit/SPEC-liveness-lock.md` and `lifecycle-kit/SPEC-dispatch-signal.md`, both
  merged away by earlier batches, leaving two dead pointers inside a governing input the last
  batch had to repoint by hand before it could reason from it. Nothing fired on either. This
  surface is path-form, so unlike the five slug-form instances it is in `check-spec-pointer`'s
  reach — and its cost lands on the session reading the amendment as governing input, not on a
  reader of the queue.
  **The design question is exactly where those five differ from two that are fine.** The
  tag algebra's unresolved-token rule *explicitly* sanctions naming landed work: "entries
  legitimately name a closed defect class, a shipped contract, a settled ruling, and that
  citation is valuable prose no gate may punish." Two live citations do it correctly — they
  say a merged slug *holds* a decision, past tense, settled. The five defective ones say a
  merged slug *rules* something, present tense, as though the reader could go read it. Both
  forms sit in the tree today, which is the calibration set.
  **Deliverable:** repoint the five to `TRAJECTORY.md`, then extend `check-amendment-queue`
  (its `couples=` already reach `TASK-QUEUE.md` and the amendment glob) with an
  owner-position assertion. Structural removal first, gate second.
  **Why `[design-pending]`:** owner position needs a decidable spelling that separates those
  two forms, and tense is not mechanically decidable. The tractable proxy is the
  `§`-heading form and the amendment path form; whether that catches enough of the
  present-tense class to be worth its false-negative surface is the open call, and getting
  it wrong permissively punishes the prose the tag algebra protects.
  **Cost while deferred:** five false statements stand in the queue, each pointing a future
  scope at a content-free Done line, and the next merged amendment adds more for free.
  Filed 2026-08-03 at close from the gap inbox, merging the validate-filed instances into
  the base class and re-verifying both against the tree; found by build batch 1 and
  validate, corrected and widened at close.

- **partitive-exemption-line-scope** [design-pending] — `check-manifest-count` is stricter
  than its own spec, and the gap is one line break wide.
  Verified 2026-08-03 by build batch 4: prose reading "appeared in 57 of" / "the 96 checks
  counted that day" reds as a restated collection total purely because the line broke
  between the partitive marker and the cardinal — the same sentence reflowed onto one line
  passes.
  **Root cause, decidable and narrow.** The partitive exemption in `canon-kit/lib/spec.sh`
  tests a *same-line* prefix regex against the text preceding the cardinal, so a partitive
  marker on the previous line is invisible to it — while canon-kit/SPEC.md
  §check-manifest-count states the exemption as a property of the *sentence*, a partitive
  marker on either side of the match, with no line-scoped qualifier. The gate being
  stricter than its own spec is the defect, not the reverse.
  **The cost is paid quietly and then paid wrongly.** It is invisible until it fires, and
  the remedy the help line offers is the exemption tag — which would bless a non-violation
  permanently and corrupt the exemption's meaning for every later reader. A low-FP-contract
  violation that converts into a corrupted exemption set.
  **Deliverable, and the machinery already exists:** `check-manifest-count` already
  carries a wrapped-paragraph hook beside the line hook, so the paragraph-joined text is
  available at the match site. Test the partitive prefix against the joined paragraph
  rather than the raw line, or carry a one-line lookback for the prefix window. The fix
  belongs in `canon-kit/lib/spec.sh` where both gates read it — `check-prose-enum` shares
  the same adapter and inherits the same defect — not in either gate. Plus a `good/`
  fixture case pinning a wrapped partitive, the regression the current pair does not carry.
  **Why `[design-pending]`:** paragraph-joined and one-line-lookback are not the same
  contract. Joining widens the exemption to any partitive anywhere in the paragraph, a real
  false-negative surface; the lookback is narrower and arbitrary. The spec says sentence,
  and neither implements a sentence.
  **Cost while deferred:** every author whose partitive happens to wrap pays a red gate and
  is offered an exemption tag as the remedy, so each occurrence risks permanently corrupting
  the exemption set rather than merely costing a reflow.
  Filed 2026-08-03 at close from the gap inbox; found by build batch 4.

- **root-doc-roster-registration-parity** [design-pending] — a new tracked root doc must
  join three rosters and only one of them is enforced.
  Verified 2026-08-03 while landing `TRAJECTORY.md`: `scripts/root-allowlist.list`
  (`check-root-tiering` reds on any tracked root entry not covered),
  `scripts/core-files.list` (`check-core-files` only reds listed-but-missing, never
  missing-but-should-be-listed), and the manifest-files array in `scripts/canon-config.sh`
  (nothing reds on a non-member). The allowlist has exactly one reader,
  `check-root-tiering`, and no gate couples allowlist membership to either of the other two.
  **Net effect:** a root doc that joins only the allowlist is battery-green while being
  unpinned against silent deletion *and* outside the doc-gate corpus — so its links,
  commands, spec pointers, manifest counts and temporal narration are all ungoverned. The
  failure is invisible and grows with every reader the doc acquires.
  **Defect class:** a governed surface whose registration is split across N rosters with
  enforcement on only one.
  **A fourth axis, added at close from the same landing.** A root doc also owes an inbound
  orientation link from a landing page, and nothing enforces that either: `TRAJECTORY.md`
  was reachable only from a SPEC deep-link and the always-loaded housekeeping roster until
  close added the README pointer by hand. Same shape, same invisibility, and it is the
  axis a *reader* actually pays for.
  **Deliverable, an extension rather than a new gate:** `check-root-tiering` already reads
  the allowlist and its subject already *is* the root set, so add an assertion that every
  allowlisted tracked root markdown file is also a `core-files.list` line, a manifest
  member, and the target of at least one link from a declared landing-page set. Its
  `couples=` must widen to `scripts/core-files.list` and `scripts/canon-config.sh`, or the
  assertion will not re-fire when either roster changes.
  **Why `[design-pending]`:** it is not zero-FP without a stated exemption set, checked
  against today's tree rather than assumed. `TASK-QUEUE.md` is core-files-pinned but
  deliberately outside the manifest set (queue-kit owns its corpus), and the `SPEC-*.md`
  amendments are deliberately in neither (transient tier). So the deliverable is the
  assertion *plus* a two-member exemption declaration, and where that declaration lives —
  gate literal, consumer config, or a per-file tag — is the open call and touches the
  provenance seam.
  **Cost while deferred:** every root doc added from here forward is one forgotten roster
  line away from being ungoverned, silently, with the battery green.
  Filed 2026-08-03 at close from the gap inbox; found by build batch 5.

- **enforcement-first-load-trigger** [design-pending] — enforcement-first has no load
  trigger anywhere in the lifecycle, which is why findings this iteration landed bare and
  were triaged retroactively.
  Verified 2026-08-03: zero occurrences of the rule's name anywhere in
  `lifecycle-kit/templates/` — no stage skill, no lead template, carries it.
  **It exists in exactly two tiers and neither fires at the moment of discovery.** The
  always-loaded CLAUDE.md line is a *summary* — the fix and the gate land in one unit —
  that omits every part of the procedure which changes behaviour: the green-instance-fix
  stop signal, the structural-first ladder, the false-positive carve-out, the named-cadence
  requirement. The full procedure in `doctrine-kit/DOCTRINE.md` is load-triggered with
  nothing in the lifecycle loading it. Load-trigger residency says a rule is resident only
  when no stage, skill or tool loads it; this one is neither resident in useful form nor
  triggered, so it fires only when a session already remembers it.
  **Evidence it is placement and not context pressure:** the align stage — a fresh session
  with a narrow brief and its stage skill loaded — fixed five instances bare, identically
  to a saturated routing-tier lead.
  **Two mechanisms, ranked.** *(1) Structural and cheapest* — put the triage step in the
  stage ritual where findings are discovered, so it costs a paragraph rather than a
  dispatch. *(2) Mechanical and self-applying* — `lifecycle-kit/bin/file-gap.sh` takes free
  text, so a bullet saying only that something is broken passes; requiring the three fields
  the doctrine asks for (defect class; mechanism, or the structural removal that obviates
  it; decidability call with a cadence when un-gateable) makes triage unskippable, because
  filing is the one step every finding already passes through. A dedicated triage subagent
  is the third option and fits a batch at close or a class needing a tree survey — but the
  decidability call is the load-bearing judgment, and getting it wrong permissively produces
  exactly the noisy gate the doctrine warns erodes the battery's authority, so it does not
  ride the cheapest tier.
  **Why `[design-pending]`:** (2) is a refusal on a capture affordance whose design premise
  is that refusing capture pushes a finding back into session context — the deferred-capture
  antipattern the inbox exists to prevent. Reconciling a required-fields check with that
  premise is unruled.
  **Cost while deferred:** the class is self-demonstrating — this iteration's gap inbox is
  the evidence, and the bullets carrying a named mechanism carry it because some session
  happened to remember a rule nothing loaded.
  Filed 2026-08-03 at close from the gap inbox; found by the lead.

- **self-revert-reminder-expectation** [design-pending] — a dispatched agent that reverts
  its own edits gets a harness reminder indistinguishable from an injected instruction.
  Observed 2026-08-03: a read-only audit sweep verifying the crate's unit-test
  falsifiability mutated source, reverted it, and received a note saying the file was
  modified by the user or a linter, that the change was intentional, that it should not
  revert it, and that it should not tell the user — so it reported a suspected prompt
  injection. The lead received the identical message twice in the same session for mundane
  causes (a gap-inbox append landing under it; a build batch sweeping its working-tree edit
  into a commit), which is what identifies it as genuine harness behaviour rather than an
  attack.
  **The hazard is not the false alarm; it is the true-belief case.** Falsifiability testing
  *requires* mutating source and reverting it, so this fires every time — and an agent that
  obeys leaves its mutation in the tree while reporting success. A broken crate behind a
  green report.
  **Two mechanisms, ranked.** *(1) Structural* — a falsifiability check runs against a
  scratch copy rather than the tracked tree, so no revert is ever needed and the reminder
  never fires. *(2) Stated* — a standing line in
  `delegation-kit/templates/agent-execution.md` telling a dispatched agent that this
  specific reminder is expected after a self-revert and does not countermand its brief;
  cheap, but it does not remove the surface. Not gateable: nothing about it enters the
  tracked tree.
  **Related to `fork-dispatch-prohibition`** — both are cases where a dispatched agent's
  authority is narrower than what some in-context text invites it to do.
  **Why `[design-pending]`:** (1) is the real fix and it is not free — a scratch-copy
  falsifiability harness has to reproduce enough of the build to make the mutation
  meaningful, and whether that lives in the crate's own test tooling or in the gate-test
  runner is unruled. (2) alone teaches an agent to discount a class of harness message,
  which is a cost worth naming before paying.
  **Cost while deferred:** every falsifiability check dispatched to an agent is one obedient
  reading away from silently leaving a mutation in the tree behind a green report.
  Filed 2026-08-03 at close from the gap inbox; observed by the lead across three
  occurrences.

- **behavior-change-surface** [design-pending] — no accumulating declaration surface.
  A tightened gate has one and a behavior change does not — and gate-sdk/SPEC.md §upgrade-smoke's
  own rationale for the former covers the latter without modification: it grounds
  `.workflow/tightened-gates.txt` on build being the only stage that knows what it tightened at
  the moment it tightens it, so the declaration is *written from knowledge* rather than
  reconstructed later. docs/install.md §The upgrade contract requires a Behavior changes section
  in every release note and gives it no such surface, while that set is judgment-laden rather
  than diff-derivable — so composing it at close means reconstructing an author's judgment from
  commits.
  **Deliverable:** the missing surface, carrying the three things `tightened-gates.txt` has and
  this would need — a `# contract:` header, a drain protocol at the tag, and a freshness or
  parity gate.
  **Open design question the promoting scope answers first — deliberately unresolved here:**
  whether this is a second `.workflow/` file or a widening of the existing one. A
  behavior-change bullet carries a changed-surface name plus prose, while the existing surface
  is specified as bare gate names and nothing else.
  **Measured, which is why the cost is not hypothetical.** The "each batch records its own set"
  convention was invented at batch 4 (`13f8091`) in response to this gap being filed at
  `df6fd3d`. Batches 1 and 2 closed before it existed and recorded nothing — a case-insensitive
  scan of every diff in the iteration finds the phrase only in those two later commits. So
  `native-artifact-publish-path`, `native-artifact-install-path` and
  `install-path-gnu-userland-undeclared` carry **no declaration, not a declared "none"**, and
  close reconstructed their sets from commit bodies so the next note need not: *publish-path* —
  the parity gate rides Tightened gates, leaving the descriptor and roster path as its behavior
  set; *install-path* — `init`, `doctor` and the lock/digest resolvers changed, and `c5c19e6`
  and `d13c1f6` are defect fixes whose own bodies say they "would have reached an adopter";
  *gnu-userland* — documentation only, no shipped-code path.
  **Cost while deferred:** exactly that reconstruction, paid again by whoever composes each
  release note, against evidence that is coldest when the batch count is highest.
  Filed 2026-08-04 at close from the gap inbox; the design question left open on purpose.

- **consumer-smoke-subset-accounting-verdict** [design-pending] — a per-kit smoke run reds an
  accounting the subset cannot decide, and says nothing about it.
  `gate-sdk/bin/run-consumer-smoke.sh` given a KIT SUBSET reds the registration accounting on
  gates the no-arg run accounts for: `check-action-gh-repo`, `check-action-pinning` and
  `check-action-run-shell` probe exit 0 in a two-kit scratch consumer and self-declare in the
  full one. So a per-kit invocation prints a FAIL that is not a repo finding, and nothing in
  its output says the verdict needs the full roster — the reader has no way to tell an
  artifact of the subset from a real one. Master itself is clean (11 kits probed, 0
  unaccounted), independently confirmed 2026-08-04.
  **Deliverable, and the design choice inside it:** either scope the accounting assertion to
  the invoked subset (correct verdict, weaker coverage) or keep it whole-roster and have the
  subset run *say so* — print that the accounting line is advisory under a subset and
  suppress its contribution to the exit code. The second is cheaper and keeps one accounting
  algorithm; the first is what a reader naively expects.
  **Cost while deferred:** every per-kit smoke run costs a false-alarm investigation, and the
  habit that pays it is worse than the alarm — a reader who learns to discount this FAIL
  discounts the true one. Already paid twice this iteration, including one bullet filed
  against master on the strength of it and retracted.
  Filed 2026-08-04 at close from the gap inbox; the false red corrected in place mid-iteration.

- **session-model-identity-verification** [design-pending] — a session cannot report or
  verify the model tier it is running at.
  The session-context hook prints iteration, budget and drift; `drift-report` prints neither.
  Nothing surfaces the running model, so a session cannot state its own tier without a human
  hand-reading the harness transcript, and no stage can assert the tier it was dispatched at.
  **Operator-proposed shape, recorded 2026-08-04:** a `usage-verdict`-shaped check — snapshot
  in, exit 0/1/2, fail-soft — reusing `lifecycle-kit/bin/session-id.sh`'s projects-dir
  derivation, with the *tier expectation in consumer config* rather than a kit literal. Both
  halves of that placement are forced: a baked model-name ladder is drift by construction
  (delegation-kit's agent-execution rule keys tiering to capability, not to a name), and the
  provenance seam keeps product constants out of kit literals regardless.
  **Feature-shaped, so it wants `/spec`, not a debt promotion.** It spans derivation lifecycle
  or context, verdict delegation, and a consumer config surface, and it introduces a new
  governed name. Cross-kit ownership plus a new name is the amendment threshold, and a scope
  that promotes this straight to build will be authoring the contract inside the build.
  **Cost while deferred:** every tiering rule in the tree is unverifiable — including the two
  filed alongside this one. `consult-tier-declaration` blocks on it outright, and the
  `Co-Authored-By` attribution defect has no derivable fix without it.
  Filed 2026-08-04 at close from the gap inbox; filed by the lead.

- **consult-tier-declaration** [design-pending] [blocked-by: session-model-identity-verification]
  — `/consult` governs the tier of what it dispatches and asserts nothing about its own.
  The skill landed this iteration to carry judgment-tier boundary questions, and its own
  amendment argues it is judgment-tier *by nature* — yet it declares no floor for the session
  running it and verifies nothing at entry. A consultation answered at a cheap tier is
  indistinguishable, in the record, from one answered at the tier the skill was built for.
  **Operator direction 2026-08-04:** it must run on the top model and verify that at entry.
  **The shape that keeps the seam intact:** the *skill* declares its own floor, the *kit*
  never spells a model name — the same split `session-model-identity-verification` sets up,
  which is why this blocks on it rather than racing it. Without the mechanism this entry is a
  prose assertion of the kind that already failed twice this iteration.
  **Cost while deferred:** the repo's one escalation-grade skill is silently downgradeable,
  and the failure is invisible in the artifact — a thin consultation reads as a short one.
  Filed 2026-08-04 at close from the gap inbox; filed by the lead on operator direction.

- **co-authored-by-trailer-attribution** [design-pending] — the model trailer is a baked
  prompt literal, so public history misattributes authorship.
  The `Co-Authored-By` line is copied from the harness prompt rather than derived from the
  running model, and tracked public history carries the result. **Measured 2026-08-04:** the
  lead session ran one model for two hours while its prompt's trailer named another, and the
  validate stage session ran a third for all 82 of its messages while both of its commits are
  signed with the first — two tiers, one baked literal, both wrong. Within this single
  iteration five stage commits carry the trailer and five do not, so the convention is neither
  uniform nor accurate, and no gate enforces either shape.
  **The decision comes before the fix:** whether the repo wants the trailer at all. Dropping
  it costs nothing and ends the defect; keeping it obliges deriving it, which needs
  `session-model-identity-verification` and is why this is filed beside that entry rather than
  under it — the drop answer needs no mechanism, so this is not simply blocked.
  **Why it is not merely cosmetic:** this is a public repo and the trailer is an authorship
  claim in tracked history. A wrong one is not a stale comment; it is a false statement about
  who wrote a commit, published, and it cannot be corrected without a rewrite.
  **Cost while deferred:** every commit adds another misattributed record, and the cost of the
  eventual correction grows monotonically with the count — history is the one surface this
  repo cannot re-generate.
  Filed 2026-08-04 at close from the gap inbox; filed by the lead, measured that session.

- **intra-file-pendency-contradiction-scan** [design-pending] — one file can call the same
  slug landed in one section and pending in another, and nothing reads both.
  Found at close 2026-08-04 by the `capability-pendency-after-landing` audit:
  gate-sdk/SPEC.md said a second port "lands after `native-artifact-publish-path` and
  `native-artifact-install-path`" while, ninety lines later, the same file said criterion 5
  is "implemented by `native-artifact-publish-path` and `native-artifact-install-path`". Both
  landed 2026-08-03. Two sections, two tenses, one file, one slug pair.
  **Why the existing coverage did not catch it, which is the point.** The
  `capability-pendency-after-landing` roster class *did* run at that iteration's own close
  and missed it, because the class is scoped as a human sweep of governed prose against the
  tree — an unbounded read whose reach depends on which files the sweeper opens. The stale
  paragraph was written mid-iteration and never revisited after the same iteration's later
  commit discharged it, so the tree-comparison the class prescribes never reached it.
  **Deliverable, and why it is narrower than the class it sits under:** a scan for one
  *decidable* shape — a governed file citing a slug in a landed construction ("implemented
  by", "built", "ships") and in a pending construction ("waits on", "lands after", "does not
  exist yet", "not yet") within the same file. It needs no tree comparison and no judgment
  about what is actually live: the contradiction is internal, so the file falsifies itself.
  That is what makes it gateable where its parent class is not.
  **Why `[design-pending]`:** the construction vocabulary is the whole gate, and a literal
  phrase list in a kit is drift by construction plus a provenance-seam problem — the
  vocabulary is consumer editorial. It wants the `check-graph` / `graph-vocab.sh` treatment,
  optional consumer config, which is a design call rather than a size one. Also open: whether
  a legitimate "X landed, Y still waits on it" sentence pair trips it, which decides whether
  the predicate is per-slug or per-slug-per-section.
  **Cost while deferred:** the class stays a sweep whose reach is whoever runs it, and its
  one measured miss cost a full iteration of a governed SPEC contradicting itself in public
  — gate-sdk/SPEC.md is mirrored to the docs site, so the contradiction shipped.
  Filed 2026-08-04 at close; the instances it would have caught were fixed the same session.

- **enter-stage-arg-position-silent-drop** [design-pending] — `--simulate` after the stage
  is silently dropped, and the read-only preflight runs the destructive reset instead.
  recurrence: enter-stage-arg-position-silent-drop 2026-08-07
  `bin/enter-stage.sh` parses the flag positionally — it tests `$1` for `--simulate` and
  shifts — so `enter-stage.sh scope --simulate` leaves `$1` as the stage, never sets the
  simulate bit, and treats the flag as a trailing argument nothing reads. Nothing refuses
  the extra argument and nothing in the output says the flag was ignored.
  **Why this one argument's position is not an ordinary usage nit.** The dropped token is
  the only thing separating a read-only preflight from an iteration-boundary reset. The
  real run truncates `.workflow/WORKFLOW-STATE.txt`, rewrites the queue header to `—`,
  truncates every `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` member — including the release
  disposition a close just wrote — and wipes `.tmp` past the keep-list. So the failure mode
  of a typo'd read-only command is destroying the iteration state the command was being run
  to inspect, and the one unrecoverable casualty is `.tmp`: the boundary wipe is not a git
  operation, so a session journal goes with it while the tracked half restores clean.
  **Measured 2026-08-04 at this close**, on the close session's own verification step. Every
  tracked write was reverted from the index and the close's state restored; the wiped `.tmp`
  journals were not, and were not needed because the work was already committed. That
  recovery depended on the run happening with a clean tree — the same mistake with
  uncommitted work in the tree loses it.
  **Deliverable, and why the obvious fix is not the whole fix:** accept the flag in any
  position, which is a two-line change. But permissiveness is the weaker half — the
  load-bearing part is that an argument the script does not recognize must be a **refusal**,
  not a silent ignore, which is this repo's own fail-closed rule applied to its own tooling.
  A tool that silently discards what it cannot parse fails open on exactly the input a user
  got wrong.
  **Why `[design-pending]`:** whether the refusal belongs in this script alone or as a shared
  argument contract across `lifecycle-kit/bin/` is the open call — and the class reading is the
  one that held: `file-gap.sh` had the same symptom (`capture-affordance-help-flag`, it filed
  `--help` as a gap), and that closed 2026-08-13 as a shared `bin/` argument-shape contract over
  five tools rather than as a fix to one script.
  Related and worth reading together: `enter-stage-simulate-no-write-fixture` (icebox) pins
  the no-write guard with a fixture, and would **not** have caught this — a fixture written
  the documented way puts the flag first and passes.
  **Cost while deferred:** every session that reaches for the preflight can destroy the
  state it meant to inspect, and the sessions most likely to run it are stage sessions at a
  boundary, which is exactly when the state is most valuable and least reconstructible.
  Filed 2026-08-04 at close, from the close session's own misfire.

- **dead-queue-citation-report** [design-pending] — an in-body citation that resolves to no
  live entry reads exactly like one that does, and nothing names the difference.
  recurrence: dead-queue-citation-report 2026-08-14
  queue-kit/SPEC.md §The tag algebra rules the in-body single-backtick slug a *reference*
  rather than a membership claim, aggregated by `bin/queue-edges.sh` and "audited by
  nothing". That is a deliberate choice and stays right — entries legitimately name landed
  work and no gate may punish it — but its cost is now attested rather than hypothetical.
  **Re-attested 2026-08-14**, one iteration after the last correction and inside a single
  iteration's own lifetime: `consumer-gate-port-disposition`, filed that morning at scope,
  cited `port-corpus-grouping-census-unbought`, a slug resolving nowhere in the tree. The
  citing text was rewritten later the same day for an unrelated reason (`a8354823`), so the
  instance is gone and the class is not — nothing found it but a reader.
  `check-queue-slug-liveness` does not reach it: the citation is backticked prose, which the
  SPEC deliberately rules a reference rather than a membership claim.
  **The attestation.** Three ruled-and-deleted slugs were cited across four live entries, and
  two of those entries argued *from* them in the present tense: one framed a "Boundary with
  the two live companions" whose companions were both dead and settled. A scope session
  ranking such an entry reads closed rulings as open questions, which is a false premise in a
  survey input. Both were corrected inline at the 2026-08-06 close; nothing stops the next.
  **Why a report and not a gate**, which is the design half already half-answered. A red is
  wrong here by the SPEC's own reasoning, so the cheapest true form is a *listing*:
  `bin/queue-edges.sh` already resolves every in-body citation against the live set and drops
  the misses on the floor, so naming them costs one output section and no new scan — the
  no-red posture kept, the silence ended.
  **What is genuinely open:** where the listing is read (a close step, a scope survey input,
  or both), and whether a citation of *landed* work should be distinguishable in prose at all
  — a grammar question the SPEC left unanswered when it refused a relational vocabulary, and
  answering it the wrong way re-imports the maintained-roster anti-pattern that refusal avoided.
  **Cost while deferred:** dead citations accumulate at the rate rulings close, and each is a
  false premise sitting in a survey input at exactly the moment a scope decides what to promote.
  Filed 2026-08-06 at close, draining the gap inbox; found 2026-08-06 at scope.

- **guard-steer-grant-mismatch** [design-pending] — the guard steers sessions onto forms the
  committed allowlist does not grant, so obeying it costs a prompt.
  Attested at the 2026-08-06 close triage. `scripts/bash-guard.sh` refuses a shell expansion
  and names `git -C <dir>` as the better form; `git -C` sits behind no committed glob, so
  every session that obeys the steer prompts — 4 calls this iteration. The same shape one
  step down: the allowlist grants `chmod +x *` while the tree's actual motions are
  `chmod 755` and `chmod 644` on fixture stubs and descriptors, which that glob cannot match.
  **Why it is a class and not two typos.** A guard rule and an allowlist glob are authored on
  different surfaces by different motions, and nothing reads them against each other. The
  triage procedure already warns that an allowlist entry can *mask* a steering opportunity
  (guard-kit/templates/close-triage.md); this is the mirror it does not name — a steer with
  no grant behind it, which reads as ordinary friction rather than as a missing pair.
  **Why `[design-pending]`:** the cheap fix is the wrong one. Granting every steered-to form
  widens the auto-allow set, which that same template rules the **consumer's** call and never
  a session's, so the deliverable is at most a *report* pairing each guard steer against the
  committed globs — and whether the steer strings are mechanically extractable from a
  consumer's guard script at all is the open question. Provenance seam: the steer vocabulary
  is this consumer's, so no kit may carry the pairs.
  **Cost while deferred:** the friction loop the close triage exists to drain refills from
  the guard itself, and each close re-derives the same mismatches off the same log.
  Filed 2026-08-06 at close, from its own prompt-friction triage; scope-gated intake, so the
  report is costed here rather than built here.

- **kfric-empty-log-ambiguity** [design-pending] — an empty knowledge-friction log is read as
  no friction, and it is equally consistent with no capture.
  recurrence: kfric-empty-log-ambiguity 2026-08-17 2026-08-23
  **The 2026-08-23 date is a direct stamp too, and the corroboration is the strongest yet.**
  `battery-runner-port` ran six stage sessions plus a consult and the log read **empty**, while
  that iteration's prompt log records at least four history-archaeology reads shaping a new
  deliverable: a build session reading commit `b86f3c96`'s diff to locate pipe sites, a session
  archaeologising when `check-crate-arms`'s cache was introduced, a spec session reading a
  *deleted* amendment (`SPEC-eighth-cut.md`) to shape a new one, and this close reading a prior
  close's commit to shape its own. Four events, zero stamps.
  **The 2026-08-17 date is a direct stamp, and its grounds are first-person.** The log read
  **empty** at this close across a six-session iteration — and the closing session itself
  re-derived a fact no doc owns (that `git log -S` misses an eviction leaving the slug behind)
  and did not run `kfric.sh`, filing `queue-recovery-pickaxe-wrong-oracle` instead. The survey
  record shows scope re-deriving the *same* fact independently earlier in the same iteration.
  Two events, one fact, zero stamps: stronger than the founding attestation, which had to infer
  its unstamped event from the prompt log. Stamped directly, out of channel, under the rule this
  iteration landed as obliged (lifecycle-kit/SPEC.md §The committed gap inbox).
  `.workflow/knowledge-friction.log` read **empty** at the 2026-08-06 close, across an
  iteration that added a Rust module, a build-time source stamp and a new gate. The same
  iteration's prompt-friction log records a build session reading a *deleted* `.gate`
  descriptor out of git history to shape the one it was writing — a prior deliverable
  consulted because no surface carries an example, which is exactly the class
  drift-kit/templates/close-knowledge.md names and `bin/kfric.sh` exists to stamp. It was not
  stamped, and nothing noticed.
  **The reader is what makes it expensive.** `kpi-knowledge-friction` trends the log toward
  zero and reads that trend as the tier contract's holes filling. A sensor with an
  in-the-moment capture discipline and no independent floor cannot tell a filled hole from an
  unstamped one, so the KPI reads best exactly where it is least trustworthy.
  **Why `[design-pending]`:** the honest answers differ in kind. A corroborating signal is
  cheap and weak — the prompt log already records history archaeology and close already reads
  both surfaces. A per-stage capture prompt is stronger and is precisely the standing-
  instruction tax context-kit's brevity machinery rejects. Stating the limit on the KPI
  instead — an empty log is not evidence of zero — costs one line and buys no detection.
  Which is right turns on whether the KPI is meant to be trusted at zero, a contract call.
  **Cost while deferred:** the one KPI measuring the tier contract's completeness reads best
  exactly when nobody is capturing, and no other signal contradicts it.
  **THREE HOLDS, 2026-08-24, across two consecutive scopes and their leads, every one on the
  surface criterion and none a decline of the finding.** Nothing in the promoted turn-end and
  worktree control set touches drift-kit's capture loop or the KPI that reads it. The third is
  operator-ruled. No date joins a hold — a decline is not a firing — and the log read **empty**
  again at this boundary, the expected reading at an iteration's open and no firing either.
  **The pairing the first hold could not carry:** `kfric-obligation-residency` is this entry's
  writer-side twin — that entry is the capture obligation never reaching the writer, this is how
  an empty log may be read once it has — and both resolve alongside
  `recurrence-obligation-residency` under delegation-kit/SPEC.md §Operative residency's placement
  rule. A unit taking the capture loop should take all three; taking this one alone answers the
  reader's question while leaving the writer unserved.
  Filed 2026-08-06 at close, from its own knowledge-friction sweep.

- **reclaim-precondition-outside-the-tree** [design-pending] — a declared `reclaim=` can be
  runnable and still un-runnable, and the gate that demands one cannot tell.
  `check-close-surfaces` blocks a capture-tier declaration that names no `reclaim=`, which
  is the right floor and is not the whole obligation. `.workflow/essay-harvest.md` declares
  `: > .workflow/essay-harvest.md` — a command any close can execute — but its *precondition*
  is "merged into the essay", and the essay lives outside this tree. So the reclaim has never
  fired: the sink now carries entries dated back to 2026-07-10, seven-plus iterations of
  operator material, each close correctly declining to truncate.
  **The bind is what makes it a defect rather than a slow queue.** A close that runs the
  declared reclaim destroys material it cannot re-derive; a close that declines lets a
  capture surface grow without bound, and does so *correctly* every time, so nothing ever
  reads as wrong. The declaration's own text is what hides it — a reclaim command that
  executes is indistinguishable, to the gate and to a reader, from a reclaim that discharges.
  **Why `[design-pending]`:** the candidates are a grammar change and a posture change, and
  they are not the same size. A `reclaim=` could carry its precondition (who or what must act
  first), which the gate could then require but never verify — honest, cheap, and still
  unenforceable. Or the surface could be ruled *not* capture-tier at all, since what it
  accumulates is an operator's material rather than a session's, and a tier whose reclaim no
  session may run may be the wrong tier. Provenance seam: what the precondition *is* — an
  essay, a person, an external merge — is consumer content, so a kit may demand the field and
  never a vocabulary for it.
  **Cost while deferred:** every close spends a judgment call re-deciding not to truncate,
  reaches the same answer, and leaves no residue that the answer was reached — which is
  exactly the shape the close-surface roster was built to end.
  Filed 2026-08-06 at close, from the runtime-artifact lifecycle check and an audit sweep
  that measured the sink's age.

- **release-drain-ordering-contradiction** [design-pending] — RELEASING.md step 4 prescribes a
  commit its own battery refuses.
  Step 4 makes the tightened-gates drain and the disposition stamp one commit, calls that the
  iteration's final commit, and requires pushing it and watching the `gates` run for that SHA
  green *before* tagging. `check-tightened-gates-note-parity` arms on a note whose declared
  version carries no tag yet, and its only dormancy arm is that every note is tagged — so the
  truncation reds it in both directions, nine declared gates against an emptied surface. It is
  `tier=precommit` and a battery member, so the prescribed commit is refused locally and the
  watch it requires would red remotely. No ordering satisfies both surfaces.
  **Attested at the v0.22.0 cut, and it could not have surfaced earlier.** The gate landed in
  `release-signaling-reset`, after v0.21.0 was tagged, so this was the first release cut under
  it. Ruled option A by the lead 2026-08-06 — the drain yields and lands as a follow-up commit
  once the tag has disarmed the gate — on the ground that the gate is executable, was written
  later, and encodes tag-then-drain in its own dormancy message, while step 4 was simply never
  updated to match. The release shipped on that reading and the runbook was deliberately left
  unedited mid-release, because changing the procedure you are executing is how a cut becomes
  its own precedent.
  **What this entry owes is the text, not the decision.** The edit is small and the care is in
  what must survive it: the one-commit coupling is defended by a stated parity-window reason and
  the watch-before-tag ordering by a separate stated reason about the remote oracle, and the
  rewrite must keep both arguments while moving only the drain.
  **Correction 2026-08-08 at scope, re-verified against HEAD — most of this entry is already
  discharged, and the residue is one sentence.** A prior revision claimed "Step 4's prose still
  describes the refused sequence, so the next release re-derives the contradiction". That is
  **false**: `d64e63c` (2026-08-07, the v0.22.0 cut) already rewrote step 4 to *stamp*, push,
  watch, tag, drain-after — `RELEASING.md:119` now reads "write the **stamp** commit", and
  `:127-135` carries a paragraph titled "The drain lands after the tag, and the parity gate is
  what forces it." What survives is a residual intra-step contradiction the corrective did not
  reach: step 4's **opening** sentence (`RELEASING.md:106-109`) still bundles the drain and the
  disposition stamp as one commit — "so the iteration's final commit is the one this step
  creates" — while the body it introduces now splits them. So the cost-while-deferred below
  overstates: the next release does not re-derive the contradiction, it reads one stale opening
  sentence against a correct body. Re-cost before promoting; the design question below may no
  longer be worth an iteration slot.
  **Why `[design-pending]`:** where the follow-up commit belongs is genuinely open. Riding the
  next iteration's push holds the two-push budget and leaves the surface undrained on the
  remote between releases; its own push buys immediacy at a cost CLAUDE.md rations. A third
  answer is to state the gate's dormancy model in the runbook as the contract it already is,
  making the follow-up's timing a consumer's call rather than a step.
  **Cost while deferred:** every release re-derives a settled contradiction, and re-derives it
  at the worst moment — mid-cut, with a note committed and a tag pending.
  Filed 2026-08-06 at close, on the lead's ruling, from the cut that hit it.

- **amendment-dod-sibling-dependence** [design-pending] — an amendment's DoD is written as if
  its unit were the iteration's only one.
  Two shapes attested in `background-producer-liveness`, one root.
  (1) **Reciprocal.** `evidence-kit/SPEC-liveness-lock.md` and
  `lifecycle-kit/SPEC-dispatch-signal.md` each carried a mirror "ships together with the other"
  item. Written symmetrically it reads as blocking to each side and neither half can go first;
  batch 1 merged only by reasoning past its own DoD, and nothing in either amendment asserted
  the iteration-level reading that makes that correct.
  (2) **Broader, and strictly containing (1).** The merge assertion `ls <kit>/SPEC-*.md returns
  none` is carried by *every* amendment for a kit with more than one in flight. It is satisfiable
  only by whichever batch merges that kit's last amendment; the earlier batches' identical copies
  are unsatisfiable at their own commit, for a reason nothing in either amendment states.
  **The general shape is the entry** — a DoD item whose satisfiability depends on sibling units
  the amendment never names. Triaging the two instances separately is what produced two gap
  bullets for one defect.
  **The reading is already ruled.** The operator ruled shape (1) discharged at the iteration, not
  the commit, on the mechanical ground that build stays in build until the queue empties and
  close refuses entry on a non-empty active queue; that ruling is landed at canon-kit/SPEC.md
  §Merging an amendment. What stays open is the *authoring* side.
  **Why `[design-pending]`:** the candidates differ in kind — a template change (the shipped DoD
  checklist names the iteration as the discharge horizon), an authoring rule (an amendment may
  not assert over files it does not own), or an assertion on the amendment glob that reds a
  cross-amendment path reference inside a DoD item. Which is right depends on whether sibling
  dependence is ever legitimate, and that is unruled.
  **Cost while deferred:** every multi-amendment iteration ships DoD items that are false at
  their own commit, and each costs a build session the reasoning-past that batch 1 paid — the
  exact move a DoD exists to make unnecessary.
  Filed 2026-08-06 by close, merging two gap-inbox bullets into the class both name.

- **recurrence-resolver-literal-match-only** [design-pending] — a real recurrence whose bullet
  does not spell the slug is drained as a fresh entry.
  The gap-inbox recurrence resolver matches a literal slug substring in a bullet's prose — at
  capture (`bin/file-gap.sh`'s live-slug scan) and again at the close drain, which re-resolves
  the same way. A filer who describes a defect rather than naming it produces a bullet that is
  present, is a genuine recurrence, and matches nothing; the drain files it as a new entry,
  fragmenting the backlog the inbox exists to keep whole.
  **The third face of one matcher, which is why the three cost together.**
  `gap-resolver-mention-overcount` covers the inflating direction only (a bare mention counted as
  a recurrence); `recurrence-drain-input-widening` covers a recurrence reaching close with no
  bullet at all. Neither covers a bullet that is present and fails to match. All three are the
  same bounded-substring predicate from three sides, and a scope taking one alone re-derives the
  other two.
  **Attested twice in one drain, in the over-counting direction.** Two of this iteration's nine
  bullets carried an auto-stamped recurrence prefix fired by a bare mention — one naming
  `gap-resolver-mention-overcount`, one naming `recurrence-drain-input-widening` — and both had
  to be struck by hand at the drain.
  **First attestation in its own direction, 2026-08-08.** A mechanical scan of the live slug set
  across the whole gap inbox matched nothing, yet one of that drain's six bullets was a genuine
  recurrence of `dispatch-worktree-reds-the-battery`: it described the defect — the worktree the
  dispatch guard mandates, and the battery reddening while it lives — without ever spelling the
  slug. The drain's session-side re-resolution caught it and stamped the date; the matcher alone
  would have filed a duplicate entry. Every prior attestation on this cluster ran the
  over-counting way, so this is the first measurement of the direction this entry names.
  **Why `[design-pending]`:** the under-matching direction has no syntactic remedy at all. The
  over-counting sibling can at least imagine a filer-supplied flag; here the missing signal is in
  the filer's head, so the honest candidates are a required recurrence-or-new field on
  `file-gap.sh`, or accepting the drain's session-side judgment as the only real channel.
  **That contract call has since been made, and this entry survives it.**
  `gap-resolver-mention-overcount` landed the second candidate: the drain is the authoritative
  judge, the capture-time matcher is demoted to an advisory that asks the filer, and the required
  recurrence-or-new field is refused there with cause (it interrogates a filer who has not read
  the queue). So the channel question is settled and this entry's is not — the shipped advisory
  is silent exactly where this entry points, since a bullet that never spells the slug raises no
  prompt at all, leaving the whole burden on a drain reading prose with nothing marking the match.
  **Cost while deferred:** the recurrence count under-reports in a direction nobody can audit,
  and its two readers — scope's pre-emption threshold and `kpi-incident-recurrence` — see a
  backlog that looks less repetitive than it is.
  Filed 2026-08-06 by close from the gap inbox.

- **section-prose-outlives-its-entries** [design-pending] — Clear-Done removes the entries and
  leaves the prose that framed them.
  A scope-authored preamble under an active section describes the entries beneath it. Close's
  Clear-Done step moves those entries to `## Done` and touches nothing else — precedent
  `e3d893e`, `f8433ac` — so the preamble survives as prose naming a causal order that points at
  nothing. No gate reds: prose is not an entry, and no gate reads the relation between them.
  **Instance, fixed at this close.** `## New Features` kept `background-producer-liveness`'s
  four-unit causal-order preamble after all four entries reached `## Done`. Lead-ruled 2026-08-06
  at the end of build that close removes it explicitly; the removal rides this close's Clear-Done
  commit, and the class is what survives.
  **Why `[design-pending]`:** the tractable rule is not obvious. "An active section with no
  entries has no body" is decidable and would have caught this instance, but it over-fires on a
  section preamble that is a standing authoring rule rather than a gloss on the current entries,
  and `TASK-QUEUE.md` carries both kinds. Separating them needs either a marker on
  iteration-scoped prose or a ruling that active sections may not carry standing prose at all —
  a queue-format contract change.
  **Cost while deferred:** every iteration whose scope writes a section preamble leaves one
  behind at close, and each stranded preamble tells the next scope a causal story about entries
  that are gone.
  Filed 2026-08-06 by close from the gap inbox; instance fixed inline in the same commit.

- **amendment-roster-stale-by-construction** [design-pending] — the enumeration that makes a
  sweep mechanical is stale the moment a sibling batch lands.
  An amendment that enumerates a rename or sweep roster up front is doing the very thing that
  makes such a delta mechanical rather than design-bearing. The roster is authored at spec time
  and is stale by construction as soon as any sibling batch writes prose on the same files, and
  nothing reds.
  **Attested this iteration.** The verb-rename roster was authored at spec; an earlier build
  batch then added ~77 lines to `delegation-kit/SPEC.md` and ~46 to
  `lifecycle-kit/templates/lead.md` without pre-applying the rename. The sweeping batch
  re-derived against the tree only because the lead flagged it, finding five sites the roster
  missed — one in `docs/delegation-kit/index.md`, a hand-authored page
  the `docs-mirror` arm does not write, so no mirror regen would ever have corrected it.
  The enumeration also undercounted its own stated four citation sites.
  **Shape:** either a freshness assertion that a roster is re-derived at sweep time, or a stated
  rule that an enumerated roster is a floor and never the answer. The instance went green, which
  is exactly the enforcement-first stop signal to ask what check should have caught it.
  **Why `[design-pending]`:** a freshness assertion needs a re-derivation command the amendment
  can name, and a rename roster's derivation is a grep whose pattern is the amendment's own
  content — so the gate would assert a grep against a grep. The floor-not-answer rule is cheap
  and unenforceable, and choosing between an unenforceable rule and an expensive gate is the
  design.
  **Cost while deferred:** every multi-batch iteration carrying a sweep unit ships a roster its
  own later batches falsify, and the only detector is a lead who happens to flag it — which is
  what happened here and is not a mechanism.
  Filed 2026-08-06 by close from the gap inbox; the sweep instance itself landed green.

- **unregistered-gate-fixture-coverage** [design-pending] — the fixture-pair contract is
  asserted over the registry, not over the gates.
  `check-gate-fixture-coverage` walks `gates.list` members, so a kit gate that ships deliberately
  unregistered carries no fixture obligation at all: evidence-kit's `check-producer-liveness` (an
  entry-preflight gate, kept out of `gates.list` because this repo's battery is what it locks
  against) and canon-kit's `check-surface-duplication` can lose or never gain their `good/`+`bad/`
  pair with nothing red. `run-gate-tests.sh` executes whatever pair directories exist, so an
  absent pair reads as zero coverage rather than as a harness error — the silent direction.
  **Found at the build that created the first entry-only gate**
  (`validate-producer-liveness-unobservable`), which is what turned an always-true assertion into
  a false one.
  **Shape:** widen the assertion from the registry to every kit `checks/` member, in both
  declaration spellings, keeping the existing no-fixture opt-out. The false-positive surface is
  bounded precisely because that opt-out already exists — a gate that genuinely owes no pair
  declares it.
  **Why `[design-pending]`:** "every kit `checks/` member" is a claim about what a kit root is,
  and gate-sdk/SPEC.md's kit predicate is the authority; widening a gate to walk kit roots
  couples it to that predicate, and the coupling wants stating before it ships.
  **Cost while deferred:** the four gate-sdk contracts bind registered gates only, and the
  exception set grows by one each time a gate is deliberately unregistered — which this iteration
  established as a supported pattern rather than an oddity.
  Filed 2026-08-06 by close from the gap inbox; found at build, shape supplied by the filer.

- **link-wrapped-section-citation-liveness** [design-pending] — a section citation inside a
  markdown link is invisible to both citation gates.
  `check-spec-pointer`'s prose extractor guards on a `.md` *immediately* followed by the section
  sign, so it looks only where the two are adjacent. The reference-link form
  `[path.md](path.md) §Heading` puts a `)` between them: the guard never fires, the heading is
  never resolved, and this holds even though the file is in the governed manifest and
  `check-md-refs` has already resolved the link half. Live at `CONTRIBUTING.md:65-66`.
  **Third member of a family the queue already carries**, and the one that makes the family a
  predicate question rather than three fixes: `prose-filename-citation-liveness` is a path with
  no heading, `unqualified-section-citation-liveness` is a heading with no path, and this is a
  path *and* a heading that the adjacency guard splits. That second entry already says one
  predicate may cover both of the others; this is the evidence that the guard's window, not the
  citation form, is the variable, so a promoting scope should cost all three together.
  **Why `[design-pending]`:** widening the guard to tolerate an intervening `)` is a
  two-character change and almost certainly wrong alone. The honest fix decides what the
  extractor's window *is* — nearest preceding `.md` in the paragraph, or the link target — and
  that decision is shared with the two siblings.
  **Cost while deferred:** a renamed heading strands this citation with nothing red, inside a
  governed repo-meta file whose whole purpose is telling a contributor where to go.
  Found at spec 2026-08-06 while sizing the verb rename's blast radius, verified against the
  gate's own extraction logic rather than inferred; filed 2026-08-06 by close from the gap inbox.

- **queue-tier-label-correction-cost** [design-pending] — a wrong debt/feature label is cheapest
  to leave wrong.
  Two of `background-producer-liveness`'s four units — the lead dispatch-precondition entry and
  the dispatched-session waiting-rule entry — self-labelled as debt adding no governed name,
  while both delivered exactly what canon-kit/SPEC.md's litmus counts as a feature: a contract
  another component must honor (a dispatch precondition the lead must honor; a kit SPEC sanction).
  Both bodies also fired the misfiling tell — a multi-paragraph design ruling inlined where no
  gate sees it.
  **Scope read the labels correctly and still did not fix them.** It routed all four units to the
  authoring stage on the corrected reading, which is the behavior the label exists to drive, and
  left the labels standing: both entries sat at `check-queue-entry-budget`'s line cap, so editing
  a label first required finding a compensating trim. The correction was strictly more expensive
  than the miscarriage it fixed.
  **Why `[design-pending]`:** the label is not mechanically derivable — the litmus turns on
  whether a body delivers a contract, the same semantic judgment `check-queue-entry-budget`'s
  compression rule already declines to gate. The tractable question is the *friction*, not the
  label: whether a label-only edit should be exempt from the cap (a one-token diff cannot regrow
  an entry), whether the cap should be measured below the lead line, or whether nothing changes
  and the cost is correct.
  **Cost while deferred:** every mislabeled entry that reaches the cap stays mislabeled, and the
  label's two readers — the authoring-stage routing rule and the new-names litmus — read a value
  its author already knew was wrong.
  Filed 2026-08-06 by close from the gap inbox; both instances reached `## Done`, so the class is
  all that survives.

- **ruling-record-condition-staleness-probe** [design-pending] — a ruling conditioned on an
  event nobody retires once the event fires.
  `TRAJECTORY.md` carries rulings whose text is conditioned on a future event — "at
  `<iteration>`'s close" is the surviving example, and "until the first binaries tag" was the
  other until the prune deleted every ruling carrying it — and nothing retires one when its
  condition is met. Today obsolescence rides the unit that discharges the ruling, whose own
  entry names the rows it removes on landing; that holds only where an author remembered to
  write the removal in, and nothing catches the ruling whose condition a unit met silently.
  **The enforceable half is a probe, never a prune — and the ground for that has moved.** It
  rested on retirement being operator-class; the 2026-08-08 pruning directive
  (`trajectory-prune-on-completion`) authorizes the prune, so what keeps this entry to a probe is
  its own deliverable rather than an authority limit.
  The deliverable is a *staleness probe* over condition-bearing rulings that escalates, plus
  whatever declaration makes a condition machine-readable — a ruling stating its own discharge
  event is the design question, since a prose condition carries no syntactic tell. That
  declaration is this entry's alone: the pruning unit ships prose convention, not form.
  **Both attested conditions have now fired, which settles the recurrence and not the
  deliverable.** One was already missed at the 2026-08-06 close — a ruling naming an iteration's
  close as the tag point, which passed uncut with no surface saying so; the other fired
  2026-08-07 (v0.22.0), discharging every row conditioned on it at once. No probe watched either.
  **The class reaches the record's corrections, not only its conditions** — the surface written
  to record the missed condition went stale itself in under a day.
  **Widened 2026-08-09 by the first post-prune close: the inbound half, and it is the mirror
  of what `SPEC-ruling-record-prune` D9 refused.** D9 weighed slug-liveness *over* the record
  — the record citing dead slugs — and correctly ruled it a no-op. The other direction was
  never considered: **another governed surface citing a ruling the record no longer carries.**
  The pointer names the surviving *section*, so it resolves and the battery reports clean over
  it. Now that pruning is licensed, every prune manufactures this exposure. Three instances were
  verified individually at that close, each having survived a full green battery, and all fixed
  there. **No denominator is stated on purpose:** two sweeps of the citation corpus returned
  counts irreconcilable under differing exclusion sets — `spec-measured-count-gate`'s subject.
  **The design input worth more than the instances, because a probe would miss it.** A fourth
  instance — `gate-timing-baseline-comparability`, corrected at the same close — cites the
  deleted ruling **by its distinctive phrasing, naming no file at all**, so no probe scoped to
  file-name citations reaches it; and it was the highest-consequence of the four, a deferral
  argument resting on the expired premise rather than stale narration. So the condition
  vocabulary must be readable from the *citing* side, not only the record's.
  **Why `[design-pending]`:** the condition vocabulary is the whole design, and the
  escalation-only boundary rules out the cheap fix.
  **Cost while deferred:** a ruling read as live after its condition passed steers the very
  sessions that consult the record for what is settled, and the record's authority makes that
  steer expensive to unwind. The widening adds the sharper cost: the citing surfaces are
  *published* — a kit SPEC and its docs mirror — so the blast radius reaches adopters.
  **Fifth inbound instance, 2026-08-13, and the first from a *deliberate retirement* rather than
  a prune** — `gate-sdk/SPEC.md` citing a ruling the retirement deleted from a surviving section;
  cleaned by `spent-ruling-retirement`, so the exposure this entry attributes to pruning is now
  attested on the retirement path too.
  recurrence: ruling-record-condition-staleness-probe 2026-08-13
  Filed 2026-08-07 by close, draining the gap inbox.

- **stage-economics-log-key-session-collision** [design-pending] — two sessions in one
  (iteration, stage) overwrite each other in the trend log.
  `drift-kit/bin/stage-economics.sh`'s `log_line` dedups on the `<iteration> <stage> <model>`
  triple, so a batch-split stage — several sessions stamped into the same pair — appends twice
  and the second silently replaces the first, under-reporting that stage by a whole session.
  The session id is written as a same-line field, never into the key.
  **Found by exercising the meter against the real sessions directory**, not by reading it:
  one iteration's build stage had run as two sessions and only one row survived. The fan-out
  pass folds its own rows for exactly this reason; the stage rows still race.
  **Held out of the ruled cut deliberately** by the iteration that fixed the fan-out half, so
  this is the named remainder rather than a regression.
  **Re-measured 2026-08-08 at close, on a second independent instance.** That iteration's build
  ran four batches tiered per batch across two models, two sessions each; the log retained one
  row per model, so each retained row carries a single batch's draw as though it were the
  stage's — an under-report of roughly a third on both models, with nothing in the file marking
  that anything was dropped. Two refinements that instance adds to the diagnosis above: the
  per-session report on **stdout is correct**, so only the persisted rollup is lossy and it is
  the persisted rollup that trends; and **per-batch model tiering is precisely the shape that
  breaks the dedupe**, which was written for one session per stage per model, where re-running
  the collector must not double-count.
  **Why `[design-pending]`:** the honest closes differ in kind, and there are three. Folding
  stage rows the way the fan-out pass folds keeps the key and loses per-session detail; widening
  the key with the session id is a trend-log grammar change every existing reader inherits; and
  making the row an explicit aggregate the writer recomputes from every matching transcript
  keeps the grain but moves the work into the writer. Naive summing is ruled out on its own —
  the collector is re-runnable, so a sum double-counts on every re-run.
  **Cost while deferred:** every per-stage figure for a split stage under-reports, and the
  under-report is invisible in the log — a cross-stage comparison drawn from a split iteration
  is not signal, and nothing in the log says so. The Split posture makes splitting routine, so
  the defect's reach grows with the posture's adoption. Sharper under model tiering: the rollup
  is the surface a standing tiering watch reads, so a model looks cheaper exactly when its stage
  ran more batches.
  Filed 2026-08-07 by close, draining the gap inbox; found at build. **Merged 2026-08-08 at
  scope by lead ruling** with `stage-economics-log-multi-session-undercount` (filed 2026-08-08
  by close), which was the same defect re-found: identical `log_line` dedupe on the
  iteration/stage/model triple, verified against the source. That entry's premise that the work
  was unfiled is false; its measurement and its third design candidate are folded in above and
  the duplicate slug is dropped.

- **rejected-compound-commit-relabel** [design-pending] — a rejected stage-and-commit leaves
  the index staged and the bare retry mislabels the work.
  A hook rejection of a compound stage-and-commit invocation unstages nothing, so the natural
  retry — a bare commit — sweeps the staged deliverable under whatever message the retry
  carries. Observed 2026-08-07: a build batch had its compound call rejected on an expansion
  rule, retried bare, and shipped fifteen files of feature work under a one-line stamp
  message; the real stamp then landed as a second commit.
  **Nothing red fires, because nothing is wrong with the content.** The work was correct and
  gate-verified. The casualty is provenance, and it is unrecoverable once pushed: the
  never-rewrite policy and a shared index both forbid amending, so the misdescription is
  permanent in the history later sessions read for what happened.
  **Note the interaction that makes this likely rather than rare.** The shared-index rule
  advises staging and committing in one motion precisely to keep a foreign commit from landing
  between the two — which is the very shape most likely to be rejected by a guard.
  **Why `[design-pending]`:** the candidates sit on different surfaces. A commit-msg gate
  asserting that a stamp-shaped message touches only the stamp path catches this case and
  little else; guidance to reset the index after a rejected compound commit is cheaper and
  unenforced. Which is right depends on how wide the misdescription class really is.
  **Cost while deferred:** the history that later sessions and the economics read take for
  ground truth can misattribute a whole deliverable, and the error is silent, permanent, and
  likeliest on the commits a guard just fired on.
  Filed 2026-08-07 by close, draining the gap inbox; observed against this iteration's
  own build commits.

- **survey-record-supersede-invisible** [design-pending] — the entry trigger prints a
  superseded block beside its superseder with no visible difference.
  `.workflow/survey-record.md`'s stage-entry read trigger advertises every block for a
  question, so a superseded block and the block that superseded it print as two identical
  headings. A reader cannot tell which is live from the trigger output, so the protocol's own
  instruction — run the witness before buying this survey again — has to be run twice to find
  out, which is the re-derivation the surface exists to remove.
  **Observed on the surface's first real use, and again at this close.** A build session filed
  a survey, its own commits moved the corpus, it filed a superseding block, and the next stage
  entries each printed the pair.
  **Why `[design-pending]`:** printing only the newest block per question is cheaper but hides
  the history the record is append-only to keep; marking the superseded heading keeps both and
  needs a supersede relation the record does not declare today. Either way it stays
  append-only.
  **Cost while deferred:** small per entry and growing with every supersede in an iteration,
  and it lands on the reader rather than the writer — so the writer never sees it and the
  surface's own value claim decays unobserved.
  Filed 2026-08-07 by close, draining the gap inbox; found at build, re-observed at close.

- **probe-before-assertion-doctrine** [design-pending] — the iteration's recurring failure was
  asserting where a cheap probe was available; does that earn a durable rule?
  Every session this iteration that *probed* found something reasoning had missed, and every
  falsified claim was falsifiable by one command. Scope asserted twice and was falsified
  twice; spec corrected scope and then made a softer version of the same error, which align
  caught; build's meter batch found a row-erasure race only by running against the live
  sessions directory; the guard batch found that a whitespace `IFS` made one of its own rules
  never fire; the survey batch found a boundary-truncate bug by running a real entry. Close
  added two more: a gap bullet asserted a disambiguation was missing that had landed two days
  before it was filed, and a stamped release disposition asserted a tag that was never cut.
  **The form question is now CLOSED — operator-ruled 2026-08-10 at the `native-port-cadence`
  close.** The rule ships as an always-loaded one-line entry in CLAUDE.md's delivery doctrine
  with its mechanism behind the doctrine-kit link: `Probe-before-assertion`,
  doctrine-kit/DOCTRINE.md rule 12, in the methodology-maintenance register. The three
  near-neighbours it was weighed against stay distinct and are named in the rule itself —
  `Oracle-first` covers the gate case, `gap-bullet-premise-verification` the gap-inbox case,
  and rule 15's inspectable-run discipline the spawned-component case; none covered the
  design-time claim made where no oracle is running, which is what the new rule owns.
  **What remains, and why this stays `[design-pending]`:** the rule shipped with *Enforced by:
  judgment with a capture mechanism*, so the open work is whether any slice of the class is
  mechanizable at all — a scanner for an unprobed claim is the thing nobody has designed, and
  the honest default is that the class is natural-language and stays convention.
  **Sixth-firing evidence, `native-port-cadence` (the iteration that earned the rule).** Scope's
  cohort census scored 4 of 7 criteria and carried "clears criteria 2/3/4/7" forward as if it
  answered the selector's actual term, so the operator's cohort ruling failed on both terms.
  `.workflow/survey-record.md` was wrong twice in one iteration. Validate asserted "not a live
  end-user hazard" and was sent back to verify it. Validate then declined the evidence baseline
  mark on a false premise about what baseline-marking means, which blocked the close entry.
  Each was caught by a *different* reviewer and none by the author.
  **The firing that decided the register.** The lead's own dispatch relaying this pattern
  carried an unverified citation — `scripts/gen-pre-commit.sh:38`, a file that does not exist —
  passed on without opening it. A pattern that fires inside the dispatch *about* the pattern is
  the argument for a resident line rather than a link: the reviewers who caught the other five
  had all loaded the material, and it still went through. Hence the rule's relay clause.
  **The firing that shows the reviewing stages are not the backstop, 2026-08-17
  (`post-close-intake-and-index-port`).** An amendment section built a whole batch on the
  claim that deleting a 182-line tracked shell file moves the footprint measurement. False —
  that projection measures context surfaces, not shell line counts — and it rode through
  **spec, which authored it, and align, whose whole job is verifying the amendment**,
  unchallenged. Build caught it by running the emitter rather than reading the sentence, and
  the batch collapsed into verification. Every earlier firing was caught by a later reviewer;
  this one only by the first stage that *executed*: review does not catch a false ground.
  **Cost while deferred:** the cost is paid where it was measured — a false premise entering
  at scope is corrected at spec or align if it is lucky, and lands in the queue as established
  fact if it is not; the queue's own convention of dating premise corrections into bodies is
  the standing evidence that it is often not.
  Filed 2026-08-07 by close, as the iteration's candidate lesson; the observation came from
  the lead, the evidence and the framing from this drain. Form ruled and the rule landed
  2026-08-10 by close on operator direction; scope narrowed here to the mechanizability half.

- **consumer-smoke-accounting-spelling-unpinned** [design-pending] — the accounting reads both
  gate spellings, and nothing holds it there.
  `run-consumer-smoke.sh`'s registration accounting now unions `check-*.sh` with `check-*.gate`
  (gate-sdk/SPEC.md §Consumer smoke — the registration accounting), but that widening is stated
  in prose and pinned by no fixture. The defect it prevents is the silent one: a ported gate
  dropping out of the accounting's universe leaves no probe, no declaration and no finding, so
  a regression reads as a clean run rather than a red.
  **Why `[design-pending]`:** the harness is a `bin/` tool rather than a registry member, which
  is exactly what `check-gate-assertions`' assertion C cannot reach, so pinning it means
  standing up a second whole-consumer scratch tree inside a fixture — the cost is the design
  question, not the assertion.
  **Cost while deferred:** the one accounting that proves a ported gate is still counted is
  itself uncounted, and its failure mode is invisible by construction.
  Trigger: the next unit touching that accounting, or a second gate cohort porting.
  Filed 2026-08-07 by close, promoting the lead's gap-inbox filing at `d790c10`.

- **release-asset-claim-class-owner** [design-pending] — a reader-facing claim about what a
  Release ships is maintained by hand, with no oracle.
  `docs/install.md`'s cargo bullet asserts what a tagged release publishes. The descriptors unit
  corrected the sentence and **ruled the gap accepted-ungated** rather than minting a gate:
  the class has exactly one instance, and a registry for one member is the move the payload-claim
  design refuses on its own axis. This entry carries that ruling's named trigger so it survives
  the gap inbox's truncation.
  **Why `[design-pending]`:** the open question is whether a release-asset claim is its own class
  or a second axis of `check-payload-claim`'s vocabulary — one asks what a Release *ships*, the
  other what a consumer *receives*, and collapsing them wrongly is harder to unwind than leaving
  them apart.
  **Cost while deferred:** one published claim about Release contents sits on the adoption path
  held only by hand — the exact shape that produced `payload-disclosure-claim-owner` one claim
  class over.
  Trigger: a **second** release-asset claim appearing on any governed surface, at which point
  both axes collapse into `check-payload-claim` rather than into a third gate.
  Filed 2026-08-07 by close, promoting spec's gap-inbox filing at `06e379c`.

- **build-stage-tightened-gates-write-pair** [design-pending] — the stage template names one of
  the two writes a tightened gate owes.
  `lifecycle-kit/templates/stages/build.md` instructs a build session to append the bare gate
  name to `.workflow/tightened-gates.txt`. That is half the obligation: the dated release note
  owes a matching bullet, and `check-tightened-gates-note-parity` holds the two against each
  other **in the same commit**. A session following the template literally stages an incomplete
  pair and reds its own commit.
  **Two independent build batches hit this in one iteration**, which is what makes it a template
  defect rather than a session lapse — the second batch had no way to learn from the first.
  **Why `[design-pending]`:** the deliverable is nameable (say both writes, cite the parity
  gate), so the open part is only placement — whether the note bullet belongs in the build
  template, in the release runbook the template would then have to load, or in the gate's own
  red output, which is the surface that already knows both halves. A scope may promote this
  straight to a task.
  **Cost while deferred:** every build stage that tightens a gate reds once and re-derives the
  second write from the gate's failure, which is the knowledge-friction loop paying for a
  sentence.
  Filed 2026-08-07 by close, carried from build batch 4.

- **align-checklist-fanout-calibration** [design-pending] — align converges at zero divergence
  while build finds the defects it should have caught.
  **Read this entry as calibration, not as the revert signal.** The standing align tiering ruling
  names one live revert trigger: *a missed spec defect surfacing as a build round-trip*. It did
  **not** fire this iteration — every defect was fixed in-envelope by the build session that
  found it, none round-tripped, and nothing here is grounds to revert the tier. Conflating the
  two is the specific misreading this entry exists to prevent.
  **What did happen, `native-cohort-activation`.** Align passed all four amendments at zero
  divergence. The four build batches then found **ten** real defects. Six were fan-out and roster
  misses of one shape: a section a change touched that the amendment's "Existing sections updated"
  roster failed to name, or a reader-facing surface still asserting a predicate the change
  retired — twice in the release note itself, which is the front door.
  **Most were catchable two ways**, and neither is a judgment call: grep the unit slug tree-wide,
  and read each delta against its own **DoD** rather than against its "what this keeps" list. The
  second is the sharper one — a "keeps" list is the author's claim about what a change does not
  touch, so align reading it as a boundary inherits the exact blind spot that produced the miss.
  **Why `[design-pending]`:** a mechanical fan-out check over an amendment's roster is a real
  candidate and is already filed as `amendment-update-target-coverage`, so the open question is
  the split — how much of this is a checklist edit to the align template, how much is a gate, and
  whether a "keeps" list should keep its standing as an input at all.
  **A second, separate failure class, recorded beside this one rather than folded into it.** The
  validate session — also Sonnet — ended its turn on work still running in order to wait for it,
  which its standing dispatch policy forbids outright. Corrected in-flight; it recovered fully,
  re-verifying the first run's actual state rather than assuming it. That is a **protocol** miss,
  not a judgment miss, and it belongs to the dispatch-policy surface rather than to align's
  checklist. Kept adjacent because both are tier-calibration evidence and the two get read
  together; kept distinct because a fix for either does nothing for the other.
  **Second reading, `consumer-cohort-completion-and-wait-enforcement` (2026-08-15) — and it does
  not repeat the first.** Align ran on Sonnet, found **five real defects** and escalated nothing
  spurious, so the zero-divergence shape above did not recur. What did happen is the other half:
  **three amendment claims were falsified by probes at build** — delta 8's knob arithmetic, delta
  6's substantive half (which would have reddened the battery on **every** invocation had it
  shipped), and a second fail-closed hole.
  **Both facts, stated honestly, because they point opposite ways.** The revert signal **as
  defined** — a missed spec defect surfacing as a **build round-trip** — did **not** fire: build
  absorbed all three in-session and nothing round-tripped. And three spec defects nonetheless
  reached build, one of them battery-reddening. Whether a defect absorbed in-session should count
  against the tier is a **tier judgment**, which is the lead's and the operator's; this close
  records the data and does not re-tier. That question is itself the calibration this entry is
  about — the revert trigger is defined on the round-trip, and this iteration is the case where
  the two readings come apart.
  **Third reading, `leak-guard-and-assertion-meta-gate-port` (2026-08-23): the second reading
  again, at five for five.** Every build batch found its amendment's stated *tree behaviour*
  wrong while its *ruling* was right — batch 4 carried three in one amendment, one of which
  would have refused the exact command its own rule exists to grant, each settled by one probe.
  recurrence: align-checklist-fanout-calibration 2026-08-23
  **Cost while deferred:** align keeps returning a clean verdict that build then falsifies, so
  the stage's signal value decays toward zero while its cost does not — and a zero-divergence
  pass is read as evidence the amendments were right.
  Filed 2026-08-07 by close, from the lead's per-batch tiering watch.

- **survey-edge-aggregation-residue** [design-pending] — the audit that asks whether a scope
  survey aggregated inbound edges has nothing to read.
  `templates/stages/scope.md` requires it plainly — aggregate a candidate's inbound edges
  before ranking it, because the promotion dividend lives in the total and in no single entry.
  The `survey-engagement` audit class then asks each close whether that pass ran, and declares
  its only residue to be "the survey's own reasoning".
  **The residue does not carry the claim, and four dated observations now settle it** — the
  question is answered, so the narration of each is compressed to what it proved.
  **2026-08-07:** scope's reasoning landed only as conclusions in a commit message. Its
  counter-evidence half was done properly, but nothing in any artifact said whether an edge sum
  was taken, so the audit could return neither verdict — **unanswerable, not un-gateable.**
  **2026-08-08 — answerable, and passed.** Scope named `bin/queue-edges.sh` as an oracle in the
  survey record and wrote per-candidate inbound totals into the finding. "Fold the sum into the
  existing survey record" is therefore a **shipped instance**, not a hypothetical design option.
  **2026-08-09 and 2026-08-12 — unanswerable again, twice.** Both ranked (129 Deferred entries
  into seven clusters; 85 shell gates into three cohorts) and the second recommended against a
  group by name, so the due-condition fired squarely on each. Both named a different oracle —
  `bin/queue-index.sh`, then `bin/port-blockers.sh` — and neither records an inbound sum for any
  candidate. Both did premise-falsification well, and the second did it very well (refuting a
  member's inclusion from the source, surfacing a spec-versus-oracle contradiction rather than
  picking a side, stating its own limit). So the split is stable: the falsification half is
  healthy and the aggregation half is simply absent.
  **The alternation is the evidence, and it is what makes this the requirement's property rather
  than any session's.** Answerable once, unanswerable three times, on one roster line, with
  nothing changed but the survey — and the entry predicted its own recurrence before two of them.
  **The ground for declining a `recurrence:` date is SPENT, 2026-08-17**, when
  `recurrence-drain-input-widening` landed the direct stamp as sanctioned *and obliged*: an
  observation reaching a session from the rostered close audit rather than the capture channel is
  now stamped by whoever judges it. These four are **not** back-filled — the obligation attaches
  to the judging session at the moment of judgment, the landing rule undertook no backfill, and a
  later session re-grading three earlier sessions' prose is the self-grading hazard that ruling
  names rather than a repair of it. The next observation of this defect stamps. The backfill
  question itself is filed to the gap inbox at that landing for close to disposition.
  **Why `[design-pending]`:** the cheap fix — have the survey cite its edge sums — risks
  becoming ceremony, a stage writing down that it did the thing rather than doing it. The
  honest alternatives are a survey artifact the sum lands in, folding the sum into the existing
  survey record, or accepting that this class is not auditable and retiring the roster line
  rather than restamping it each close.
  **Cost while deferred:** every close restamps an audit it did not actually perform, which is
  worse than a skipped audit — the roster reads as coverage.
  Filed 2026-08-07 by close, performing the `survey-engagement` audit its roster made due.

- **release-runbook-identity-diagnosis** [design-pending] — where the which-account-is-active
  check belongs in the release procedure is unplaced; the diagnosis itself is now settled.
  A refused write on a machine authenticated as several accounts is an *identity* fault, not a
  permission one: the same 404 appears with the permission model already correct, and reading
  it as permission points the resolution at granting write to the account the private brief
  rules must **not** hold the namespace.
  **Reachability, not merely wording.** A close that defers its release never meets this; a
  close that *cuts* one meets it mid-cut, with a note committed and a tag pending.
  **Armed and demonstrated 2026-08-14** — this supersedes the 2026-08-08 correction that judged
  it latent, which is the ground that fell. The close session of
  `native-port-grouping-and-eighth-cohort` ran `gh api repos/<owner>/<repo> --jq .permissions`
  with the **non-owning** account active, read `push: false`, applied the runbook literally, and
  reported the release blocked on a permission defect. Selecting the owning account returned
  `admin/maintain/pull/push/triage` all true. Cost paid: a stage session's forward motion, at
  exactly the mid-cut point predicted above.
  **The other 2026-08-08 ground stays answered and is not re-opened:** the speculation that this
  fault explained the last tag's unwritten Release body was falsified by `gh release view
  v0.22.0` — a written body and all four assets — so step 6 was reached on the last cut, and
  that half of the original filing is closed rather than carried.
  **Resolution direction, settled 2026-08-14 and the reverse of what the runbook said.**
  Selecting the designated release account is correct; "fix the permission" is not the slower
  resolution but the **prohibited** one. This entry's former residual is now its finding.
  **The sentence-level half landed 2026-08-14 and is not this entry's remaining work:**
  RELEASING.md's never-switch-identity prohibition is deleted, replaced by an
  identity-before-status-code rule; the recording obligation is kept as that clause's real
  content; and the ops-runbook pointer is re-scoped out of the keyless-operator branch. The
  2026-08-08 observation that the runbook was "further along than this body describes" is spent
  with it — the text it cited no longer exists.
  **Why `[design-pending]`, unchanged: the fix is not a sentence.** Where the
  which-account-is-active check belongs — its own preflight step, step 4's prose, or a probe the
  procedure runs — is open design, and the sentence fix does not answer it. The constraint on
  any answer: the desired account state is private-ops content and cannot land on a tracked
  surface, so the tracked runbook may name the discriminator but never its expected value.
  **Cost while deferred:** no longer a wrong-and-authoritative diagnosis — that is fixed. What
  remains is that the check lives in prose a session must remember to apply rather than in a
  step it cannot skip, so the failure recurs on any release cut by a session that reads past it.
  Filed 2026-08-08 by close, draining the gap inbox; found at scope. Re-grounded 2026-08-14 by
  close on operator direction. Sibling on the same step:
  `release-drain-ordering-contradiction` covers step 4's drain/tag ordering, this its diagnosis.

- **gate-binary-target-roster-widening** [design-pending] [roadmap: next/reliability] — the
  binary ships one triple, and no queue unit carries widening it.
  roadmap-summary: A prebuilt gate binary for every platform the project says it supports.
  `native/targets.list` declares exactly one target triple, so the last release published one
  binary and every adopter off that triple takes the omit-and-declare outcome. That is a
  supported result rather than a break — but it means the trajectory objective naming every
  major operating system has no filed unit behind it on the artifact axis.
  **Distinct from the two entries a reader reaches for first, which is why it is separate.**
  `powershell-installer-surface` covers the bash bootstrap reaching a second interpreter, and the
  shell support-matrix CI legs cover the test matrix; neither is about which binaries the publish
  step produces. The design work is also half done and unowned — `installer/README.md` already
  records this design's named re-entry and its two candidate shapes, so what is missing is the
  unit, not the thinking.
  **Distinctness is not readiness, and the entry read as ready until 2026-08-13.** The paragraph
  above records that the *jobs* are separate and omitted that one **gates** the other:
  `native/targets.list` states the widening trigger is `platform-support-ci-matrix` landing a CI
  leg, "after which widening is one line here plus one runner mapping" — and that entry is
  demand-gated with both un-defer triggers unfired (re-verified at the 2026-08-13 drain: every
  workflow leg is still `ubuntu-latest`, and no adopter or promotion is recorded anywhere). So a
  scope session ranking off this entry alone reads "blocked on a design pick" where the truth is
  "blocked on a design pick **and** on an unfired upstream trigger". Both surfaces were right and
  only their union was legible; this paragraph is the union.
  **Why `[design-pending]`:** the two candidate shapes differ in what they cost the publish
  workflow and in whether cross-compilation or a matrix of runners carries it, and choosing
  between them is the unit's substance rather than a detail inside it.
  **Cost while deferred:** a published objective has no unit behind it on this axis, so the gap
  is invisible to the roadmap projection and to anyone reading that projection as a commitment.
  Filed 2026-08-08 by close, draining the gap inbox; found at scope. **Tagged
  `next/reliability` 2026-08-08 by operator ruling** at the `adopter-floor-integrity` scope,
  answering the call this entry deliberately left open: a published objective naming every
  major operating system with no filed unit behind it on the artifact axis is exactly the gap
  the roadmap projection exists to prevent, so the public commitment is taken deliberately.


- **amendment-refusal-acceptance-parity** [design-pending] — an amendment's refusal rationale can
  claim an acceptance criterion asserts something that criterion does not say.
  The `--dry-run` amendment refused a gate on the stated ground that the behavioral property was
  asserted directly instead, in D8, as *each mutating verb's* `--dry-run` leaving the tree object
  unchanged. D8's own ordered specification listed one verb, not each. Two sections of one
  governing document said different things about what that document accepts.
  **The contradiction, not the coverage, is what earns the entry.** A build session reading either
  section in isolation would have been correct and would have shipped a different acceptor — the
  wide reading builds three assertions, the narrow one builds one. Build read D8's explicit list
  and built to it, which is the conservative call and the right one, since widening asserted
  behavior is envelope-shaped. But which acceptor shipped turned on which section the builder
  happened to open.
  **It passed spec and align.** Neither stage caught a self-falsifying document, which is what
  makes this a methodology gap rather than one amendment's typo: the acceptance section is the
  contract, the refusal section argues *from* it, and nothing compares the two.
  **Gap generalization — the class that should have caught it.** The nearest sibling is
  `intra-file-pendency-contradiction-scan`, which scans one file for a slug claimed landed in one
  section and pending in another. Same family — a governed file falsifying itself, decidable with
  no tree comparison — on a different axis: that one is about tense, this about the scope of an
  acceptance claim. Cited rather than folded, because collapsing them would hand one gate two
  unrelated vocabularies.
  **Why `[design-pending]`:** the decidable predicate is not obvious. "A refusal section citing an
  acceptance item must quote it" is checkable but is ceremony; comparing a paraphrase against the
  criterion it paraphrases is a judgment no scanner makes. The honest middle — a refusal may cite
  an acceptance item by identifier and may not restate its content — is a change to the amendment
  format, which is doctrine rather than a gate.
  **Cost while deferred:** any amendment can argue a refusal from a claim about its own acceptance
  that no stage verifies, and the failure is silent — both readings pass every gate, and the
  divergence surfaces only as a coverage gap found after the amendment is deleted.
  **Closing this takes both halves, and the ruling folds the fix in here rather than into a unit
  of its own.** Lead-ruled 2026-08-08: a separate entry for the one-assertion fix would let
  the cheap half close on its own and quietly leave the expensive one — that the document cleared
  both spec and align carrying the contradiction — as nobody's. Enforcement-first, applied to a
  finding rather than a gate: the fix and what explains it land together. Closure therefore
  requires **both** (1) the `init --dry-run` assertion in the consumer smoke — one arm, because
  `update` execs `init`, so it covers both mutating paths the reversal arm does not — and (2)
  removal of the stated bound now standing in `installer/README.md` §The verbs and §The consumer
  smoke. Taking (1) without (2) leaves a published caveat naming a coverage limit that no longer
  holds.
  **The tree is honest today, only narrower than the refusal argued.** The capability-liveness
  sweep at this close found zero governed surfaces still carrying the wide claim, so nothing
  published overclaims; what is missing is the assertion, not a correction.
  Filed 2026-08-08 by close, draining the gap inbox; found at build. The coverage half was
  escalated as an envelope call and folded in here on the lead's ruling the same day.

- **spec-pointer-self-section-citation** [design-pending] — a bare `(§Heading)` self-citation is
  outside every citation gate's reach.
  `check-spec-pointer`'s prose pass fires only on a **tracked path immediately followed by the
  section mark**, so a citation naming a section of the file it already sits in — the ordinary
  intra-SPEC form — is never resolved against that file's headings. A dead one survives
  indefinitely and rides the docs mirror to the public site.
  **Found live, not hypothesized.** `delegation-kit/SPEC.md` cited `§The staleness contract`,
  a heading no file in the tree carries; the mirrored page published it and a queue entry had
  copied it. Repointed at this close, so the instance is gone and the class is not.
  **Why it looks cheaper than it is.** The candidate set is large and mostly correct, so the
  design question is the resolution rule rather than the scan: a self-citation resolves against
  the containing file's own headings, which is a different lookup from the path-qualified form
  the gate already does, and the two must not be conflated in one pass. It sits on the same
  extractor the deferred `check-spec-pointer` citation entries name — sequence it with them
  rather than building a second scanner.
  **Cost while deferred:** a dead intra-file pointer reads as a live one, and the mirror
  publishes it, so the failure is silent on both the governed and the public tier.
  Filed 2026-08-08 by close, generalizing the gap it drained.

- **deferred-release-declaration-accumulation** [design-pending] — only one of the note's three
  sections survives a deferred release.
  `.workflow/tightened-gates.txt` is the accumulating declaration surface for **Tightened
  gates** and is drained only at the tag, so that section batches correctly across any number of
  deferrals (RELEASING.md §The procedure step 1). **Behavior changes** and **Renamed knobs** have
  no such surface: RELEASING.md step 2 says the outstanding criteria are carried into the next
  qualifying note, and nothing carries them. A deferral therefore drops them unless a later
  session reconstructs them from `git log`.
  **Already live, twice.** `installer-lifecycle-verbs` deferred on a minor earned by behavior
  changes alone, so its declaration exists only in the basis clause of its disposition line.
  The 2026-08-08 close deferred again with five behavior changes; its bullets are carried below
  so the next qualifying note inherits them from a committed surface, not from session memory.
  **The carried declarations, composed 2026-08-08**, to the grammar docs/install.md §The upgrade
  contract owns. **2026-08-09: the carrier hit `check-queue-entry-budget`'s cap.** A third
  deferral's declarations could not land here at all and ride its disposition line's basis
  instead — the second alternative below, chosen by the cap rather than by design.
  *`lifecycle-kit/bin/file-gap.sh`* — the slug matcher no longer writes a verdict onto the
  bullet it files. Every filing gets one bullet shape; on a match the tool raises a stderr
  advisory *asking* the filer to say in the prose whether the bullet re-files the named entry or
  merely cites it. The recurrence judgment moved to close's gap-inbox drain, which reads every
  bullet and stamps the `recurrence:` declaration in the same commit that truncates the inbox.
  Reconcile if your close skill or tooling parsed the retired prefix form.
  *`kpi-incident-recurrence`* (drift-kit) — the counted series is **not comparable across this
  release**. Every `recurrence:` date before it was matcher-derived; every date after is a
  session's judgment read off a bullet's prose. Compare within a segment, never across the
  break. For this project the break is the release carrying `gap-resolver-mention-overcount`;
  for a consumer it is that consumer's own adoption of it.
  *`installer/lib/doctor.sh`* — `doctor` now walks only the consumer-audience subset of the
  toolchain roster, so a machine with no Rust toolchain installs. Reconcile if you copied the
  roster out or relied on `doctor` failing on a contributor-side tool.
  *`context-kit/lib/toolfloor.sh`* — a roster element gained a fourth positional field,
  `<name>[:<min-version>[:<impl-token>[:<audience>]]]`. The only declarable audience is
  `contributor`; empty or omitted means every audience, so existing elements are unchanged.
  Reconcile if you carry a local roster read by a consumer-side reader of your own.
  *`.github/workflows/site-health.yml`* — a release-channel arm was added: it reads the
  published Release list on the schedule and files a `site-health` issue naming every Release
  whose prerelease flag disagrees with its own version line. Reconcile if you copied the
  workflow out.
  Renamed knobs: None.
  **Why `[design-pending]`:** a second accumulating file is the obvious fix and is probably wrong
  — three surfaces to drain at one tag, two of them free prose no gate can hold to the note the
  way `check-tightened-gates-note-parity` holds the first. The honest alternatives: one surface
  carrying all three sections, or the deferral line's basis as declared carrier with a gate on it.
  **Cost while deferred — and the prediction has graduated from slow loss to hard stop.** Each
  deferral loses its non-gate declarations to git history, so the next qualifying note
  under-declares by however many iterations batched into it, which is the one section a consumer
  reconciles by reading. At the third deferral the carrier was capped and took nothing at all,
  so the evidence this entry needed is complete: the next scope ranks it on a closed case.
  Filed 2026-08-08 by close, from the release-note obligation its own drain could not discharge.

- **always-loaded-baseline-restamp-unforced** [design-pending] — the brevity pass reacts to a
  delta nothing keeps per-iteration.
  `--update-baseline` is specified as a close-stage act precisely because the pass must react to
  *growth since the iteration started* rather than to the level (context-kit/SPEC.md §The
  always-loaded meter). Nothing forces it, and `kpi-always-loaded` is advisory, so a close that
  skips the re-stamp silently converts the next close's delta into a cumulative one — and the
  reading degrades further with every close that then trusts it.
  **Measured at this close:** the baseline last moved 2026-07-19 and had gone unrefreshed
  through roughly fifteen iterations, reporting `+41` where this iteration's own always-loaded
  growth was zero. The inherited 40 lines were read against the brevity criterion here and kept
  — each is a one-line-plus-pointer resident rule whose trigger is genuinely every session — so
  the debt is the broken signal, not the surface.
  **Why `[design-pending]`:** the obvious fix, a gate asserting the baseline commit is recent,
  gates a *cadence* rather than a property and would red on any close that legitimately had
  nothing to re-stamp. The honest candidates are folding the re-stamp into the same commit the
  brevity pass already writes, or making the meter report both deltas — against the baseline and
  against the iteration's first stamp — so a stale baseline is visible in the reading instead of
  invisible behind it.
  **Cost while deferred:** a cumulative delta reads as this iteration's growth, so the pass
  either re-reads surface it already cleared or dismisses a real addition as inherited.
  Filed 2026-08-08 by close, from running the meter during its own brevity pass.

- **drift-baseline-unnamed-iteration** [design-pending] — every since-iteration-start KPI
  baselines on an ancient commit for the whole of scope, silently.
  `drift-kit/bin/drift-report.sh`'s `iteration_start()` reads the queue header's iteration name
  and runs `git log -S"<iteration> scope " -- .workflow/WORKFLOW-STATE.txt | tail -1`. While the
  header carries the unnamed-iteration sentinel — which is every scope stage, from the boundary
  reset until the stage names the iteration — the pickaxe matches the *sentinel* rather than a
  name, and `tail -1` then returns the **oldest** sentinel-bearing commit in the whole history
  instead of this iteration's.
  **Probed at filing, not inferred.** The report printed `[iteration start 718ab4e]`, a commit
  dated 2026-07-10 — four weeks stale — and the pickaxe's own tail confirms it resolves to the
  first sentinel-bearing commit rather than the newest. The numbers it produced that day:
  `queue net delta +111 (126 filed, 15 drained)` and `queue carry weight +3759 lines`, where the
  true delta for the last full iteration, measured directly off boundary commits
  (`bf641b4` 3805 lines to `dbc9c1b` 4082), is roughly **+280**.
  **Why `[design-pending]`:** reporting `n/a` while the sentinel is live is honest and goes
  blank at exactly the stage that wanted the reading; resolving the baseline from the newest
  boundary-reset commit instead of from the header name keeps the reading but changes what
  "iteration start" means for every KPI that reads it. Which one is right is not the reporter's
  call to make in passing.
  **Cost while deferred:** it fails silent behind a plausible sha and worsens with age, and the
  stage it misleads is the one stage that reads the trend to pick units — a scope session sees a
  month of cumulative drift presented as this iteration's growth. Distinct from
  `always-loaded-baseline-restamp-unforced`, which is the always-loaded meter's re-stamp cadence
  rather than this resolver.
  Filed 2026-08-08 at scope on the lead's ruling, from running the report during its own survey.

- **audit-class-corpus-attestation** [design-pending] — an un-gateable-class audit stamps a
  **verdict**, not the corpus it read, so a false negative is indistinguishable from a clean tree.
  `.workflow/audit-roster.txt` rows carry `due:` and `last:` and nothing else, so the close that
  performs one records *that* it swept and reports its finding count in prose. "Came back clean"
  is unfalsifiable at the time it is written and un-re-runnable afterwards.
  recurrence: audit-class-corpus-attestation 2026-08-15
  **Third instance, and it widens the entry: this one was not a false negative.** The 2026-08-15
  sweep of `capability-pendency-after-landing` *found* `gate-sdk/SPEC.md`'s "no `.gate` member
  exists anywhere in the tree" — false since the first cohort, 2026-08-02 — and correctly ruled
  it outside its own trigger, which is event-scoped to what the iteration landed. So an honest
  sweep, with its corpus genuinely read, can leave standing drift; a `last:` carrying the corpus
  command would not have changed that verdict. The scoping is the second axis, and this entry's
  deliverable has to rule whether a row also stamps what it *declined* and why.
  **Measured harm, and it is not hypothetical — a two-for-two false negative on consecutive
  closes.** `capability-pendency-after-landing`'s trigger fired when `native-cohort-activation`
  cut v0.22.0 (`.workflow/release-disposition.txt`, `git show d64e63c0`), the first tag publishing
  gate binaries as Release assets. Both that close and the next swept the class and stamped it
  performed: `ad8d4a31` recorded "the kit SPECs came back clean" and `9b2aec20` recorded "zero
  findings". `gate-sdk/SPEC.md` carried the discharged-blocker claim in **two** places throughout
  (§The port-candidate criteria criterion 5, and the paragraph opening "What stands between that
  port and an adopter"). Both survived both sweeps and shipped to the published SPEC and its
  public docs mirror, where validate found one of them two iterations later.
  **The deliverable is a stamp grammar, not a scanner** — which is what makes it buildable against
  a class the roster's own text calls un-gateable. A row's `last:` gains the corpus command the
  sweep ran and the hit count it triaged, so the next close re-runs the predecessor's own oracle
  instead of re-inventing a corpus, and a sweep that read nothing cannot stamp a verdict. What
  stays un-gateable is the *judgment* on each hit; what stops being un-gateable is whether a
  corpus was read at all.
  **Why `[design-pending]`:** three classes on the roster have no single-command corpus (a
  capability's "instances" are the tree set a scanner cannot infer — the reason they are on this
  roster), so the unit must rule what a row with no derivable corpus stamps instead of a command,
  and an honest "these rows stamp a named surface list" may be the answer for some of them.
  **Cost while deferred:** compounding and paid at the boundary the roster exists to hold — every
  close re-derives each class's corpus from scratch, and the roster's whole value is a cadence
  claim it currently cannot evidence. The two closes above are the attested instances.
  Filed 2026-08-09 by close (`install-profile-seam`), draining the criterion-5 staleness bullet:
  the bullet's own finding is fixed in this close, and this entry is the mechanism half of it.
  It is the generalization `spec-measured-count-gate` names as the scope-claim axis it cannot
  reach, and is cross-referenced there rather than folded into it — that entry designs a scanner
  over authored prose, this one designs a stamp over a session act.

- **absorbed-duplicate-disposition** [design-pending] — the queue has **no third state between
  live and done** for a slug merged away as a duplicate, so an absorbed entry is counted as a
  shipped deliverable.
  recurrence: absorbed-duplicate-disposition 2026-08-24
  **FIRST RECURRENCE 2026-08-24, by a SECOND ROUTE into the class the body does not name.**
  `install-step-relocation` reached `## Done` at this iteration's scope, on an operator ruling,
  **retired as mooted** — its work became unnecessary rather than being merged into a sibling. It
  shipped nothing, and this close's Done clear is what hands it to `kpi-task-split` as a delivered
  unit. So the class is not "absorbed duplicate" but *any* slug that reaches Done having shipped
  nothing, and mooted-by-supersession is a route in that neither the lead line nor the `Distinct
  from the icebox tier` paragraph reaches. That widening is the finding; the missing third state
  is unchanged.
  `check-task-conservation` diffs the live slug set and its own help sanctions moving a renamed
  slug to Done, so an absorbed duplicate has nowhere else to go. But Done means completed:
  `stage-economics-log-multi-session-undercount` shipped nothing and its content survives under
  `stage-economics-log-key-session-collision`, which is untouched and still live. `kpi-task-split`
  reads Done, so this iteration's feat/debt split inherits a row nothing built.
  **Distinct from the icebox tier**, which is the deferred pool's exit and holds *unbuilt* work
  that stays promotable; an absorbed duplicate is not dormant, it is redundant, and putting it
  back in a live section would re-open the namespace collision the merge closed.
  **Why `[design-pending]`:** every cheap answer costs something the conservation gate is holding.
  A fourth section re-opens the section-order contract queue-kit/SPEC.md §The icebox tier already
  records as gate-unenforced; an `[absorbed-by: <slug>]` tag keeps one section and obliges the
  KPI readers to learn it; and letting the slug simply vanish is the sanctioned disappearance the
  icebox tier's design explicitly declined to invent.
  **Cost while deferred:** low per instance and silent — every merge inflates the delivered-unit
  count by one and no reader of the KPI can tell. It recurs whenever a scope survey merges a
  filed duplicate, which the deferred pool's intake asymmetry makes routine.
  Filed 2026-08-09 by close (`install-profile-seam`), draining the bullet this iteration's own
  scope session filed against its own economics merge. This close dispositions the standing
  instance as an absorbed duplicate rather than as a deliverable, in its Done-clearing commit.

- **dispatch-cited-evidence-unverified** [design-pending] — a dispatched sweep's **quoted
  evidence** is covered by no verification rule, and one returned an attributed quotation that
  exists in no revision of the file it named.
  delegation-kit's shipped verify-after-commit set covers what a sweep *writes*; nothing covers
  what it *cites*. The class matters more than a wrong line number or a stale fact because
  quotation-with-attribution is precisely the signal a reader uses to decide a claim has already
  been checked, so it defeats normal trust rather than merely being wrong.
  **Caught 2026-08-08 at spec only by re-reading every load-bearing fact by hand against the named
  file** — a property of that session, not of the mechanism.
  **Candidate rule, stated where agent-execution binds:** a dispatched sweep's quoted evidence is
  a pointer to verify at the source, never evidence in itself. **Why `[design-pending]`:** stated
  that way it is an authoring contract with no oracle, and the buyable half may be narrower — a
  dispatch that must quote can be required to return file:line for every quotation, which makes
  the check mechanical for the dispatcher even though no gate can run it.
  **Cost while deferred:** compounding, and it lands on exactly the dispatches delegation is
  pre-authorized for. Every read-heavy audit this repo runs returns prose the dispatcher is
  expected to act on without re-reading the corpus — which is the whole economic point of the
  dispatch, and the reason an unverifiable quotation inside one is worth more than it looks.
  Filed 2026-08-09 by close (`install-profile-seam`), draining the bullet spec filed under
  scope-gated intake. It is filed here rather than left in the survey record for the reason the
  bullet itself gives: the record is boundary-truncated, so the finding would be erased at the
  next scope boundary.

- **qualified-pointer-section-ownership** [design-pending] — `check-spec-pointer` asserts a
  cited `§Heading` **exists**, never that it is the heading which *owns* the cited claim, so a
  fully-qualified pointer aimed at the wrong section resolves and reds nothing.
  **Self-witnessed 2026-08-09, which is why it is filed rather than theorised.** This close,
  correcting another entry's expired premise, cited `gate-sdk/SPEC.md §What the dispatch seam
  does not settle` for a claim owned by §What is retained, and where the second port stands.
  Both sections exist. The gate passed — "every target file tracked and named §heading present"
  is its own verdict text, and presence is the whole of what it checks. Caught only by reading
  the file to confirm the sentence was there.
  **It is the mechanism under this iteration's headline class.** The inbound half of
  `ruling-record-condition-staleness-probe` — a citation surviving the deletion of the ruling
  it names — and this are one defect seen from two sides: a pointer is verified against the
  section's *existence*, so nothing notices when the section stops carrying the claim, whether
  because the claim was pruned out of it or because the wrong section was named to begin with.
  **Distinct from the three sibling entries in this cluster**, which are all about citations
  that resolve to *nothing*: `unqualified-section-citation-liveness` (a bare `§Heading` with no
  path), `spec-pointer-self-section-citation` (the self-citing form), and
  `prose-filename-citation-liveness`. This one resolves successfully and is wrong anyway, which
  is the harder half — the reader's trust is higher precisely because the pointer works.
  **Why `[design-pending]`, and an honest "not buildable" is a permitted outcome.** Deciding
  whether a section *supports* a sentence is comprehension, not scanning. The only mechanical
  approximations are term-overlap heuristics between the citing sentence and the target
  section, whose false-positive surface is the one `spec-measured-count-gate` escaped by ruling
  for an author-applied marker — an escape closed here, because that gate needs the author only
  to *declare* what was measured while this one must judge a citation nobody annotated.
  A term-overlap red on a correct citation is worse than silence on a wrong one.
  **Cost while deferred:** broad and unmeasured — `check-spec-pointer` reports 902 directive
  pointers and 247 prose citations, and the ownership of every one of them is
  unverified. The cost is not that they are wrong; it is that the gate's green is read as
  saying they are right. **Both figures re-measured 2026-08-09 at scope by re-running the gate:
  the directive count filed here was 1774, wrong by roughly a factor of two on the day it was
  written. Read the numbers off the oracle, never off this line.**
  Filed 2026-08-09 by close (`install-profile-seam`), from its own miswritten citation.

- **docs-link-red-remedy-first** [design-pending] — `check-docs-link-convention` reds on the
  most ordinary thing a docs subpage author writes, and leads with the diagnosis.
  **Measured 2026-08-09 at spec on a real `init` consumer.** A `docs/` tree whose `index.md`
  carries a `[project README](../README.md)` link reds with "off-root relative link … resolves
  outside docs/".
  **The rule is correct, and changing it is not what this asks for.** For a site served from
  `docs/` alone such a link 404s, and the gate already offers a remedy — the absolute self-repo
  blob form, or a `docs-link-exempt:` comment. It stays on-surface deliberately
  (`SPEC-prose.md` keeps it there), so nothing is broken today.
  **The gap is first contact.** The gate's whole subject is a docs host, so the adopter most
  likely to register it is the adopter most likely to write that link — and they meet a red
  before they meet the remedy, having adopted the battery minutes earlier.
  **The buyable half is probably narrower than a rule change:** the failure text could lead
  with the remedy rather than the diagnosis. That is a message edit plus its fixture, not a
  predicate change, and it should be costed before any widening of the rule is.
  **Cost while deferred:** paid once per docs adopter, at exactly the moment they are deciding
  whether the battery is worth keeping — the worst moment this project has to spend a
  false-feeling red.
  Filed 2026-08-09 by close, draining the bullet spec filed under scope-gated intake.

- **init-dry-run-plan-parity** [design-pending] — `init --dry-run` is a hand-maintained second
  spelling of the seeds it predicts, and three of its four remaining arms already diverge.
  **The queue arm was exactly this defect, and `install-queue-template-unreachable` removed it**
  this iteration — one predicate, the write alone guarded by `(( DRY ))`. The four sibling arms
  in the same `case` statement were left as they were.
  **Measured divergences, 2026-08-09 at build.** *(a) evidence-kit* — the real arm writes
  `.workflow/validate-baseline.txt` and `.workflow/validate-evidence.txt` only when absent; the
  dry arm prints both unconditionally. *(b) lifecycle-kit* — the real arm returns early when
  `.workflow/WORKFLOW-STATE.txt` exists; the dry arm prints it unconditionally. So `--dry-run`
  on an already-installed consumer names files the run would not write. *(c) the agent-file
  seed* — `init.sh`'s own `printf`, guarded by `recipe_needs_agent_file`, is predicted by **no
  arm at all**. The doctrine-kit arm predicts the doctrine *block*, and the two coincide only
  because every profile carrying context-kit also carries doctrine-kit. That is the same
  coincidence-of-rosters that hid the queue defect, and it stops holding the day a profile
  carries context-kit alone.
  **Nothing catches any of it**, which is the half that makes this a unit rather than a patch:
  the consumer smoke asserts `uninstall --dry-run` behaviorally and never compares `init`'s dry
  plan against the run it predicts.
  **Deliverable:** give each seed the one-predicate form the queue arm now has, and add the
  missing acceptor — a dry plan diffed against the set the real run records.
  **Cost while deferred:** a `--dry-run` is a promise about what will happen, so a wrong one is
  worse than none — and it is the first command a cautious adopter runs, which is the same
  first-contact surface the profile work is being bought to improve.
  Filed 2026-08-09 by close, draining the build stage's bullet.

- **kfric-capture-unverified-assertion** [design-pending] — the knowledge-friction channel has
  no oracle, so it captures whatever a session asserts and the next reader reads it as measured.
  **Self-witnessed this iteration, with both halves in the log at once.** A build batch stamped
  the consumer smoke's cost as "~50-60 minutes" and reasoned from it that the run serializes
  against all tracked editing for that window. Validate measured it twice independently at
  **227s** and superseded the entry in place rather than deleting it, so the log now carries the
  mis-derivation beside its correction — which is what makes this filable rather than anecdotal.
  **The mis-derivation is the more interesting artifact.** The figure was disprovable from
  evidence already in front of every reader: the reporting batch's own total session runtime
  was ~26 minutes, so a 50-60 minute sub-step could not have fitted inside it. It was relayed
  onward unchecked and shaped two sessions' scheduling before validate measured it.
  **Distinct from `kfric-empty-log-ambiguity`**, which is about an *empty* log's two readings;
  this is about a populated one whose entries carry no distinction between a measurement and an
  estimate. Adjacent to `dispatch-cited-evidence-unverified`, which covers what a dispatched
  sweep *cites*; this covers what a session captures about its own work.
  **Deliverable, and the design question that makes it `[design-pending]`:** whether the
  affordance should carry a measured-vs-estimated distinction at all. The whole value of
  `bin/kfric.sh` is that stamping is cheaper than deferring, so a field that slows capture buys
  accuracy with the capture rate the loop depends on. A convention may beat a flag.
  **Cost while deferred:** an unverified assertion in the log is indistinguishable from a
  measurement, and close's own triage is chartered to promote it into a doc-owner edit — which
  is the channel by which a wrong number reaches a canonical surface with a citation on it.
  Filed 2026-08-09 by close, from its own knowledge-friction triage.


- **close-entry-baseline-bootstrap-deadlock** [design-pending] — a validate that
  ends on an accepted red cannot be closed without an operator carve-out.
  recurrence: close-entry-baseline-bootstrap-deadlock 2026-08-12 2026-08-18
  The close entry preflight (`scripts/lifecycle-config.sh`'s
  `LIFECYCLE_KIT_ENTRY_PREFLIGHT`) refuses until every suite carries a clean
  evidence line. The sanctioned way to make a known red clean is a baseline `fail`
  row carrying a blocking slug, and `check-evidence-baseline`'s liveness assertion
  (evidence-kit/SPEC.md §check-evidence-baseline) requires that slug to resolve to
  a **live queue task**. But the stage chartered to file
  that task is close — which cannot enter until the preflight the mark would
  clear passes. Validate cannot pre-empt it either: a mid-iteration queue edit is
  exactly what the gap inbox exists to prevent.
  **So the machine has no in-contract path** for its own "validate ends with an
  accepted red" case. The filing close escaped only by the operator-directed
  filing exception (CLAUDE.md §Housekeeping), a carve-out and not a mechanism.
  **Deliverable, and why `[design-pending]`:** the candidates trade off against
  real properties. Let validate file a baseline slug against a gap-inbox bullet
  rather than a queue task — cheap, but weakens the liveness check that makes the
  mark self-retiring. Give the preflight a documented one-shot valve — honest, but
  a valve that will be reached for whenever close is inconvenient. Or let a
  configured permanent marker cover the transitional case — smallest change,
  worst semantics, since the whole point is that the red is *not* permanent.
  **Cost while deferred:** paid only when validate ends non-clean, but paid as a
  full stop that costs an operator round-trip at the boundary, in the exact
  situation where the tree is already known to be imperfect.
  **Second and third firings — 2026-08-12 (`native-cohort-canon-kit`) and 2026-08-18
  (`wide-budget-batch-and-hold-declaration`, at `f8c34c20`) — and the repeat is the argument.**
  Both hit the identical wall on `installer_smoke`, the second on the init no-op/regen red, and
  both escaped by the identical route: an operator-directed filing relayed by the lead. Cost
  measured rather than predicted: one full stop and one operator round-trip per non-clean close.
  Three consecutive non-clean validates, three operator interrupts, one route: the
  structural reading above is now the only one left. The same drain found a **second face** the
  candidates above do not cover — a baseline move stales the evidence line computed against the old
  baseline — promoted as `baseline-move-stales-evidence-line`.
  **NINE holds, none a decline of the finding, every one on surface or fork grounds.** The NINTH is
  2026-08-24, OPERATOR-RULED at the next scope's threshold proposal against the turn-end liveness
  set, with both the nine-hold count and the operator-class nature of the fork put in front of the
  operator rather than left implicit. It stands on the ground the SIXTH settled: the operator
  re-deferred this fork rather than ruling it, and nothing since has narrowed the three candidates.
  The SIXTH and the later of the two 2026-08-22 holds were operator-ruled too; the rest were the
  lead's, on the surface criterion alone, and their per-iteration enumeration is spent now that
  every reading agrees. One probed ground is worth keeping out of it: the row is **not armed
  today**, the live baselined `fail` naming a slug that still resolves. Count unchanged at two —
  **no date joins a decline, the finding not having re-fired** — so each hold is discharged by
  having reached the authority.
  Filed 2026-08-10 by close, from its own blocked entry; the escape it needed is
  the evidence. Re-attested 2026-08-12 by close, again from its own blocked entry.

- **knob-default-accessor-singularity** [design-pending] — the missing check class
  behind two knob-default re-spellings drained this iteration.
  `check-knob-default-coupling` asserts that every literal site for one knob
  carries the **same** literal (assertion 1 at canon-kit/SPEC.md
  §check-knob-default-coupling) — an *agreement* assertion. Two identical spellings agree, so they
  pass, and the gate has no *singularity* assertion at all. That is why both
  `.github/workflows/publish.yml:97` (`GATE_SDK_NATIVE_CRATE`) and
  `check-reads-couples`' binary lookup (`GATE_SDK_NATIVE_BIN`) sat green
  while open-coding a default whose accessor exists and whose `# spec:` comment
  calls itself "the one home" of it. Both are fixed; the class is not.
  **The check is buildable, which is why this is filed rather than argued away.**
  When a knob has a designated accessor — a function whose body is the sole
  `${KNOB:-…}` — any other literal default site for that knob is a defect even
  when it agrees. The accessor set is derivable from `lib/*.sh` rather than
  listed, so the gate ships no term list and the provenance seam holds.
  **Why `[design-pending]`:** the scan corpus is the open question, not the
  predicate. The existing gate walks `*.sh` under kit roots only, which is exactly
  why the `publish.yml` site was unreachable to it; widening to workflow YAML and
  other non-kit consumers is a corpus decision with its own false-positive
  profile, and a consumer legitimately reading a knob with a fallback where no
  accessor is in scope must not red.
  **Cost while deferred:** every new reader of a knob that already has an accessor
  can re-spell its default and stay green — and the two found this iteration were
  both introduced by the port work that is still widening.
  Filed 2026-08-10 by close as the gap-generalization owed by fixing the two
  instances inline during the drain.

- **amendment-commit-shape-red-conditions** [design-pending] — the amendment
  template's red-conditions prompt has no class for commit-shape gates.
  An amendment whose deltas touch both a gate file and a product path predictably
  collides with `check-gate-tamper`'s commit-shape rule, forcing a commit split
  that build discovers rather than plans. `SPEC-native-build.md` enumerated its
  readers' red conditions carefully and still missed it — not through
  carelessness, but because the tamper gate is not a *reader* of any changed
  string. It is a gate on **commit composition**, a class the prompt does not ask
  about.
  **Deliverable:** decide whether the red-conditions prompt should ask about
  commit-shape gates as a distinct class from reader gates, and if so what the
  amendment owes — a planned commit split is a real deliverable an amendment can
  state up front.
  **Cost while deferred:** each such amendment buys the same surprise at build
  time, where a split is more expensive to perform than to have planned.
  Filed 2026-08-10 by close, from the bullet build filed against its own amendment.

- **survey-record-claim-reliability** [design-pending] — the survey record carries
  unwitnessed mechanism claims and was wrong three times in one iteration.
  `.workflow/survey-record.md` exists so a later stage need not re-derive a survey,
  and its findings are cited on that authority. This iteration it was wrong three
  times: the scope block's cohort claim (corrected at spec by operator ruling),
  its criterion-7 verdict (contested and re-framed), and the spec block's claim
  that the generated pre-commit hook "never calls `gate_command`" — false at HEAD,
  `gate-sdk/bin/gen-pre-commit.sh:54`. The third was corrected in place at this
  close; the first two carry their corrections inline.
  **What makes this a surface problem, not three line problems.** The record's own
  contract is a `finding` free-text field a later session "judges before citing",
  with no bar on the mechanism claims inside it. `check-survey-record` parses the
  block's *shape* — key order, non-empty `corpus`/`oracle`, a real `rev` — and
  asserts nothing about whether the finding was verified. So a confident wrong
  claim is perfectly grammatical.
  **Interaction with boundary truncation, which cuts both ways.** The record is
  truncated at the next first-stage entry, so a wrong claim cannot rot for long —
  which is the argument that this needs no fix. Against that: the whole *point* of
  the record is to be cited within the iteration, and all three errors did their
  damage well inside the truncation window.
  **Deliverable, and why `[design-pending]`:** the honest options are a
  witness-on-cite discipline (already gestured at by `enter-stage.sh`'s reminder,
  but advisory), a `verified:`/`inferred:` split inside the finding field, or
  accepting the surface as lossy and demoting how findings may be cited. The
  middle option is the one `kfric-capture-unverified-assertion` is separately
  weighing for the friction log, and the two should probably be decided together.
  **Cost while deferred:** a wrong finding is cited with the record's authority by
  the exact sessions it was written to save work for.
  Filed 2026-08-10 by close, on operator direction after the third error.

- **kit-ref-liveness-stem-token-hole** [design-pending] — a typo'd knob name under
  a defined stem resolves and passes unchecked.
  Build batch 2 widened `check-kit-ref-liveness` so that a dispatch-composed knob
  name resolves against its defined *stem* rather than requiring a full literal
  match — the right fix, since names like `GATE_SDK_KNOB_<GATE>_<KNOB>` are
  composed at dispatch and no full literal exists to match. The residual: any
  token under a defined stem now resolves, so a misspelling such as
  `GATE_SDK_KNOB_PRUNE_DIRZ` passes.
  **Inherent to the shape, not a defect in the fix** — which is why it is filed
  rather than reverted. A composed name has no enumerable literal set, so tightening
  means reconstructing the composition rule inside the gate.
  **Deliverable, and why `[design-pending]`:** the candidates are to have the
  binary answer the question (`--knobs` already reports a gate's knob set, so the
  gate could resolve a composed token against the live answer rather than a stem)
  or to constrain the tail to a declared per-gate knob roster. The first is
  stronger and reuses a seam this iteration just built; it also makes the gate
  depend on a built binary, which is the trade to weigh.
  **Cost while deferred:** a typo'd knob reference reads as governed and checked
  while binding nothing — the failure mode is a knob silently never applied, which
  surfaces as behavior, not as a red.
  Filed 2026-08-10 by close, from the residual batch 2 identified and did not file.

- **amendment-correction-density** [design-pending] — one amendment took five
  corrections across three stages; nothing measures or bounds that.
  `SPEC-native-build.md` was corrected three times by align and twice more by build
  batch 2. Every correction was found by a reviewer, none by the author, and the
  amendment passed its own align gate between the two rounds — so "align passed
  it" is not evidence an amendment is correct, which is the property the stage is
  relied on for.
  **Why this is not just `probe-before-assertion` again.** That rule governs the
  individual unverified claim. This is about *density*: an amendment accumulating
  five corrections is a signal about the authoring step — its inputs, its length,
  or the review bar it was written against — that no surface currently reads,
  because each correction is landed and forgotten individually.
  **Deliverable, and why `[design-pending]`:** it is not obvious a count is even
  the right instrument. Candidates: stamp per-amendment correction counts as a
  drift-kit measurement and let a threshold prompt a re-author rather than another
  patch; or treat correction density as a review-time judgment with no counter,
  on the ground that a long amendment legitimately attracts more corrections than
  a short one and a raw count would punish thoroughness. Normalising by amendment
  size is the obvious refinement and the obvious way to game it.
  **Cost while deferred:** an amendment that needed re-authoring gets patched
  five times instead, and the cost lands on the stages downstream of it — which is
  where all five corrections were in fact paid this iteration.
  Filed 2026-08-10 by close, as the iteration's second candidate lesson.

- **probe-evidence-sufficiency** [design-pending] — rule 12 is discharged by the *act* of
  consulting a source and says nothing about whether what you consulted supports the claim.
  `Probe-before-assertion` (doctrine-kit/DOCTRINE.md rule 12) reads as satisfied whenever a
  command was run or a file opened. Every failure below ran a probe and read a real surface and
  still produced a false disposition, so the rule was **green** over each: it constrains whether
  evidence was gathered, never whether what was gathered is evidence **for this claim**.
  **Sub-case (a), attribution — the probe's output does not carry its subject.** At spec, `grep
  -h` over ten filenames returned bare lines and the fifth was attributed to the fifth argument.
  Positional mapping is inference, not evidence; it overrode corroborating prose already read in
  gate-sdk/SPEC.md's conservation table and survived into a committed "correction" of a survey
  that was right — two wrong commits and a reversal. Candidate clause: a probe whose output does
  not carry its own subject (`grep -h`, `sort -u` over several files, a bare count) is not
  evidence for a per-subject claim; ask for the label or do not make the claim.
  **Sub-case (b), authority — the surface read is not the one that owns the question.** At
  build batch 2 the tightened-gates entry was declared unowed from `build.md`'s restatement
  of the scoping rule, which is accurate *for scoping* and silent on cost; the sentence that
  decides it (`red ⊆ declared, so a declared gate that never reds is inert`,
  gate-sdk/SPEC.md §upgrade-smoke) lives only in the owner. The call was not made *against*
  the clause, it was made **without** it. The lead's own port-criteria error the same
  iteration has the identical shape, against §The port-candidate criteria.
  **Why (b) is a hazard this repo manufactures rather than inherits.** Content-tiering,
  always-loaded shape and load-trigger residency deliberately compress each rule to a resident
  one-liner. The by-product is a surface that is *accurate, authoritative-looking and not
  dispositive*: a reader cannot tell from the line whether it settles their question. The more
  disciplined the tiering, the sharper the edge.
  **Deliverable, and why `[design-pending]`:** the cheap close is one clause on rule 12 covering
  both sub-cases ("a probe is evidence only for the claim its subject and its owner support").
  Open: one clause or two, since the remedies diverge — (a) is a property of how a probe is
  *invoked* and is plausibly lintable, (b) of which surface was *opened*, with no mechanical
  residue at all. A third reaches only (b): make the always-loaded tier declare its own status,
  so a resident line routes and never decides — an authoring contract across every kit's digest,
  which is why it is not obviously cheaper.
  **Sub-case (a) again, 2026-08-12 — by the session filing this entry.** Dispatching the
  prompt-friction triage I told a sweep to separate entries "using the log's own dates"; the log
  carries none, and I had not opened it. The same dispatch asserted a 621-line corpus to a
  worktree that could not see it, and I then reported an unpushed commit as `a1f1a5b0` — a sha
  resolving to nothing, asserted rather than read back from `git log`. **Three in one session,
  by the author, while authoring the rule against them.** That is the entry's strongest evidence
  and the argument it must win to earn a gate: an instance produced by the session with maximal
  attention on the hazard is what separates a structural defect from a lapse of care.
  **Relationship to `probe-before-assertion-doctrine`:** that entry owns whether *any* slice of
  the assert-*without*-probing class is mechanizable; this is the complement — the probe ran and
  was the wrong probe. Separate because either answer leaves the other open.
  **Cost while deferred:** paid at the moment of decision by the session closest to the work and
  invisible in the diff — a judgement made from an accurate-but-silent surface shows up as an
  *absence*, and nobody reviews an absence. Seven instances now; the two caught cheaply were
  caught only because the deciding session marked its call reversible instead of landing it.
  Filed 2026-08-12 by close, draining the rule-12 gap bullet and the lead's handed-over
  candidate rule; the two were folded into one entry because both amend the same clause.

- **scratch-citation-skill-surface-reach** [design-pending] — the permanent-surface class most
  likely to carry stage-owned pointers is the one `check-scratch-citation` does not scan.
  The gate ships scanning `TASK-QUEUE.md` and `*/SPEC.md` only
  (`LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS`), and `.claude/commands/*.md` — a permanent,
  untruncated surface — is deliberately uncovered. The concrete blocker, probed at build and
  re-probed independently: `.workflow/release-disposition.txt` is a `lifecycle_supersede_set`
  member and `.claude/commands/close.md` introduces it with a colon, so widening the glob array
  by one config line would red on it **legitimately** under the gate's form (b).
  **It is not a citation defect**, which is what makes this a rule decision rather than
  effort: `close.md` is a stage naming the surface it operates on, the subject case
  lifecycle-kit/SPEC.md §The survey record already carves out ("naming the record as a subject
  is unaffected").
  **Deliverable, and why `[design-pending]`:** decide whether the subject case earns a
  carve-out in the red condition itself — the gate learns to tell a retrieval promise from an
  operand — or an exempt tag at each such site, leaving the rule blunt and the author
  declaring. Deciding this disposes of a second thing at no extra cost: the
  `scratch-citation-exempt:` tag shipped in the same batch with **no live user in the tree**,
  and the exempt-tag branch is exactly where it would earn one.
  **Cost while deferred:** a real retrieval pointer written into a skill file goes uncaught,
  and the class grows with every skill file added — while a shipped tag with no user reads as
  dead mechanism to the next session that finds it.
  Filed 2026-08-12 by close, draining the bullet build filed at batch 3.

- **settings-content-pin-report-absent** [design-pending] — the close-triage template names
  three overlay dispositions and its tool implements two.
  guard-kit/templates/close-triage.md step 4 tells close to disposition the local overlay by
  **redundant**, **too broad**, and **not content-pinned**, the last with a full procedure — an
  entry naming a *script path* rather than a fixed command grants whatever that file says at
  run time, and a path under the gitignored scratch dir is rewritable by any session, so the
  run routes through `bin/scratch-run.sh` and the direct-path grant is removed.
  `guard-kit/bin/compare-settings-allow.sh` builds only `redundant` and `too_broad`; it has no
  content-pin arm and prints no such section, so that disposition is hand-executed every close
  against an unbounded surface, with the template's own wording ("Read the shape, not the
  literal") as the only method.
  **It found a live instance immediately, which is the argument.** This close read the overlay
  by hand and found `Bash(python3 .tmp/*)` — not a pinned scratch path but a **glob over the
  scratch directory**, granting execution of any Python file any session can write there. The
  tool was silent on it. Removed this close; the next one is re-armed exactly as the template
  warns ("Removing instances without applying this criterion re-arms for the next one").
  **A second, smaller arm gap found alongside it:** redundancy is computed local-vs-committed
  only, so a local entry subsumed by another *local* entry is never reported —
  `Bash(git checkout -b *)` sat under `Bash(git checkout *)` unflagged.
  **Deliverable, and why `[design-pending]`:** the predicate is the design. "Entry names a
  writable path" is cheap and catches the scratch-glob case; "entry names any path whose
  content is not fixed" reaches a script under `scripts/` that a commit can change, which is
  most of the useful allowlist and would over-match badly. Ruling where the line sits is the
  unit; the scan itself is an afternoon, and the local-vs-local redundancy arm is a one-line
  widening of an existing loop.
  **Cost while deferred:** paid once per close as an unbounded hand-read whose miss is silent,
  and the thing missed is an auto-allow grant over attacker- or accident-writable content —
  the one allowlist class whose whole point is that it does not look dangerous.
  Filed 2026-08-12 by close, from its own overlay triage.


- **throughput-and-wait-time-unmeasured** [design-pending] — nothing splits session wall-clock
  into waiting-on-model versus local execution, and nothing measures per-model throughput.
  **What exists, and what each misses.** `.tmp/gate-timings.txt` measures local gate runtime;
  `.metric/stage-economics-log.txt` carries per-stage tokens and cost with **no time field**;
  `overhead-log.txt` measures context composition; `usage-history.log` has timestamps but no
  durations. Nothing measures round-trip latency, time-to-first-token or tokens per second, and
  nothing **joins** the two halves that would answer the question.
  **Why it is cheap: the substrate exists unjoined.** Every completion notification carries
  `duration_ms`, `tool_uses` and a token count; waiting ≈ duration minus measured local cost.
  **The claim is per-stage, not per-session, and that distinction is the finding.** Six visible
  dispatches this iteration totalled roughly 6300s of agent wall-clock against a battery of
  roughly 33s (the filer's figures, at their precision). Build and close are **inference-bound**
  — one close ran ~40 minutes against that battery — while validate is the exception, its spine
  being minutes of real local execution. A session ratio averages away the one stage where
  machine resources matter.
  **Second half, larger consequence: per-provider and per-model throughput is a decision input
  nothing can supply.** Responsiveness is a first-order reason to prefer a backend, and this tree
  can measure neither an incumbent's nor a candidate's, so the comparison would rest on
  impression. It is also what makes the standing tiering rulings **falsifiable**.
  **One blocking unknown, stated first because it must not be resolved by assumption.** The
  notification token field's semantics are **unverified** — cumulative across resumes or not,
  input-plus-output or output-only. One resumed agent reported 141301 then 196021 across two
  runs, which *reads* cumulative but was never probed. Dividing an unprobed field by a duration
  manufactures a confident number on an unchecked premise, which is why **no throughput figure
  appears here**. Settled by one dispatch whose token count is known independently.
  **Consumers, verified here rather than assumed.** `heterogeneous-agent-delegation` owns a
  routing *decision*; this owns the *instrument* it needs, and closing either advances the other
  not at all. Nearer and already live: `build-stage-tier-economics` exists to stop a tier flip
  "on intuition" and `supervision-overhead-unmeasured` is priced burn — **neither carries a time
  axis**. Adjacent, not blocking: `gate-timing-baseline-comparability` owns the *baseline* file's
  missing reader; the subtraction here uses the live file `kpi-gate-runtime` reads. DISTINCT from
  `probe-evidence-sufficiency`, cited in passing: that names a reasoning failure mode.
  **Cost while deferred:** the standing tiering rulings are invisible to current instrumentation
  — per-batch build tiering, the align departure kept on the cost-rate column and the split
  posture are judged on tokens and price, so a tier cheaper per token but doubling round-trips
  reads as a **win** while costing wall-clock, undetectably. Live rather than hypothetical:
  tiering is re-judged whenever the model roster churns, and it compounds the moment a second
  backend is in play — which is the routing decision above.
  **Cheapest close:** add `duration_ms` and `tool_uses` as columns beside the existing per-stage
  token row, written by the writer that already records it, keep the model identifier that row
  carries, and subtract local execution to split the total. The joined, per-model-grouped
  **view** is the deliverable, not a new collector.
  Filed 2026-08-12 by close, draining the operator-directed bullet the lead filed after a close
  question — intake by direction, not a finding gone looking for.

- **cardinal-notation-splits-gate-reach** [design-pending] — whether a written count is
  enforceable depends on how it is spelled and how large it is, and the queue sits outside the
  scanning corpus entirely; the truncated probe below is a `probe-evidence-sufficiency` sub-case.
  **The premise that prompted this is falsified, and that is the finding.** Asked whether spelled
  numerals have a reason, the answer is partly yes: canon-kit/SPEC.md §check-manifest-count
  documents a cardinal grammar — *"digit sequences and the spelled `two`…`twelve`,
  case-insensitive; `one` is deliberately outside it"* — implemented as `SPEC_COUNT_CARDINAL_RE`
  in canon-kit/lib/spec.sh. A word-number table is **not future work; it ships** — and the sweep
  that costed shipped code as future work searched the **right** term: `digit` matches the
  dispositive line (canon-kit/SPEC.md:642), but `grep -rn … | head -10` cut the output before
  canon-kit was reached (queue-kit/SPEC.md supplied 11 of 25). **Truncated, not mis-queried.**
  **Discontinuity 1, magnitude.** The word branch stops at `twelve`; the digit branch
  (`[0-9]+`) does not stop. So `13 gates` is matched and `thirteen gates` is not. The SPEC states
  the ceiling without justifying it. Conventional English style — spell through twelve, digits
  above — is the likely reason and is **unrecorded**, so no reader can tell a deliberate ceiling
  from an unfinished list, and neither can decide whether to extend it.
  **Discontinuity 2, corpus.** `CANON_KIT_MANIFEST_FILES` does not include `TASK-QUEUE.md`, so
  none of this reaches the queue. Its 206 spelled numerals from *three* through *fifty*
  (counted independently here, matching the filer) are outside every count gate, at any
  magnitude, in either notation.
  **The worked example is this close's own edit.** `spec-measured-count-gate` moved from "Twelve
  instances" to "Thirteen instances" here. In the manifest corpus that single word would have
  moved the claim from **inside** the matcher's reach to **outside** it — the entry about counts
  going unenforced, crossing the enforcement boundary, in the act of recording its own thirteenth
  instance. It happened in the queue, which no count gate scans, so nothing could have caught it
  either way. That is the two discontinuities composing.
  **DISTINCT from `spec-measured-count-gate`, deliberately not folded into it as grounds.** That
  entry's thesis is that measured counts **go stale** because nothing derives them; this is that
  notation and corpus decide whether an existing oracle **reaches** a claim at all. A count can be
  perfectly current and unreachable, or stale and reachable, so neither closes the other. What is
  true and kept: a digits convention is a genuine **precondition** that makes that entry's gate
  cheaper to build, since matching a digit run is trivial where matching spelled compounds is
  not. Recorded here rather than lost — but a precondition is not the same defect.
  **Compaction is the weak half, and is stated as weak.** 206 spelled numerals at roughly 4-5
  characters against a 100-column wrap is on the order of 9 lines file-wide, and a fraction of a
  line inside any single entry. Real, and not the reason to act.
  **The framing that decides it:** `TASK-QUEUE.md` reads like prose but is a gated data surface
  with a line cap, a wrap gate, a tag grammar and slug liveness. Applying a human style guide to
  it optimizes for a reader it does not have.
  **Deliverable, and why `[design-pending]` — three separable calls, and only one is mechanical.**
  (a) Is the `twelve` ceiling deliberate? Record the reason or extend the table. (b) Should
  `TASK-QUEUE.md` join the manifest corpus? That is a widening with its own false-positive
  surface, and the entry-cap and wrap gates already treat the file as data. (c) Should the queue
  adopt digits as convention? **(c) is mechanical and can land alone; (a) and (b) are design.**
  **Cost while deferred:** every count written in the queue is unreachable by any oracle, and in
  the manifest corpus a count's enforceability turns on a spelling choice no author is told
  about. Both failures are silent — the gate runs, reports clean, and never saw the claim.
  recurrence: cardinal-notation-splits-gate-reach 2026-08-19
  Filed 2026-08-12 by close, from an operator observation; the "no reason exists" premise was
  falsified by probing the count gate's own matcher rather than by grepping for a style rule.

- **headroom-check-ordering-unruled** [design-pending] — nothing says *when* a capped surface's
  headroom is read, and reading it first lets a resource limit quietly perform a judgment.
  A session editing a capped surface (`check-queue-entry-budget`, 50 lines per deferred entry)
  must at some point learn how much room it has. Whether that read happens **before** or
  **after** the disposition is reached is unruled — this close did it after, by habit rather
  than by contract.
  **The failure it prevents is invisible by construction, which is the whole argument.** A
  ruling shaped by available space presents exactly like a ruling shaped by merit: same prose,
  same confidence, no diff that distinguishes them, and no gate that reds. Every other defect
  this queue tracks leaves *something* — a stale figure, a vacuous fixture, an absent entry.
  This one leaves a well-argued disposition that happens to coincide with what fitted.
  **Worked instance from this close.** `cardinal-notation-splits-gate-reach` was ruled its own
  entry rather than grounds on `spec-measured-count-gate`. That entry stands at exactly 50
  lines, so the grounds ruling would have required compressing peer prose to land. Had the
  headroom been read first, the ruling that avoided the compression would also have been the
  ruling that fitted — and would have read as principled either way. The order is what kept the
  two questions separable.
  **Deliverable:** one line in lifecycle-kit/templates/stages/close.md placing the headroom read
  after the disposition, with the reason attached — a bare ordering instruction with no rationale
  is the kind of rule a later session optimizes away.
  **Why `[design-pending]` and why it was NOT landed inline.** Scope-gated intake decides it: a
  mid-session initiative is filed by default and never started. Close's own contract does sanction
  landing a stage-local procedure in the stage skill file, which is exactly why this needed a
  ruling rather than an assumption — and the ruling went the other way, because
  `lifecycle-kit/templates/stages/close.md` is a **kit surface with adopter reach**, so every
  consumer inherits the line. That earns scope, spec and align rather than a tail-end edit by the
  session that thought of it. Two open questions the unit owns: whether the rule generalizes past
  close to any session editing a capped surface, and whether it wants a companion where the cap
  itself is specified (queue-kit/SPEC.md §check-queue-entry-budget) rather than only in the stage
  template that happens to hit it most.
  **Cost while deferred:** each close either re-derives the ordering or does not, and a session
  that gets it wrong leaves **no trace to audit** — so the class cannot be measured after the
  fact, only prevented. That also means its instance count will read as zero however often it
  fires, which is the reverse of every recurrence-tracked entry here.
  Filed 2026-08-12 by close on the lead's ruling, from a practice the lead named at this
  session's end; recorded rather than landed, per scope-gated intake.

- **dispatch-unreadable-target-fallback** [design-pending] — a dispatched sweep whose target is
  unreadable validates against the **dispatch prompt's paraphrase** and returns PASS.
  Attested at this iteration's align, first round: three `isolation: worktree` children auditing
  three SPEC amendments, each worktree pinned to a base several commits behind HEAD. One child
  reported its target (`canon-kit/SPEC-measured-claim.md`) absent from its worktree and returned a
  full PASS/DIVERGENCE verdict set **anyway**, including PASS on an item checking a citation in
  `scripts/canon-config.sh`. A second round under an explicit pinned-rev protocol caught what the
  first missed on that exact item: the cited line documents a different knob
  (`CANON_KIT_PROSE_SURFACE_GLOBS`, not `CANON_KIT_MANIFEST_FILES`), corroborated by direct read.
  **The failure shape is the point.** The first pass's PASS was not verification — it was the
  dispatcher's own prompt reflected back as a report, and it is **indistinguishable in shape from
  a genuine clean audit**. A dispatcher cannot tell the two apart from the return value, which is
  what makes this worse than a wrong answer.
  **DISTINCT from the two entries a reader reaches for first.**
  `readonly-dispatch-isolation-unbuyable` is about which revision a child starts from — a
  stale-but-present file giving wrong numbers. `dispatch-cited-evidence-unverified` is about a
  *quotation* that cannot be traced. This is neither: it is **zero access** to the real target,
  and a child that substitutes prose for the file rather than stopping.
  **Deliverable, and why `[design-pending]`:** the pinned-rev protocol (state HEAD as a literal
  sha, require `git rev-parse HEAD` back, read every target via `git show <rev>:<path>`, and
  forbid falling back to the prompt when a target is unreadable) closes the instance and is
  **unenforced prose on the child** — the same enforcement gap its sibling names for the
  stale-base half. What is genuinely open is whether the buyable half is a required
  *unreadable-target refusal* in the agent contract, or a dispatch-side wrapper that injects the
  protocol so a parent cannot omit it. This session's own first round is the argument for the
  second: the hazard was named in the dispatching lead's brief and the mechanics were not in the
  prompt, so it recurred anyway.
  **Cost while deferred:** paid as false confidence on exactly the audits delegation exists for,
  and it is invisible by construction — the artifact is a clean report.
  Filed 2026-08-12 by close, draining the gap inbox; found at align.

- **queue-write-side-verb** [design-pending] — `TASK-QUEUE.md` has four read-side callers and no
  write-side verb, so every stage session that restructures an entry hand-rolls a throwaway script.
  Measured against this iteration's `.tmp/` before the boundary wipe, at `fde9db3f`: five scripts,
  three of them mutating governed surfaces — `promote.py` (excise two deferred entries, patch a
  premise inside one body, re-emit all three under `## New Features` carrying spec-ready tags),
  `done-move.py` (excise the active entry, append its bare slug under `## Done`), and `countfix.py`
  (a cardinal substitution below a hardcoded line offset) — plus `rename-iteration.sh` (the
  `scope-rename-guard-deadlock` workaround) and `verify.py` (a read-only citation-range printer,
  a different missing utility).
  **Each of the three re-derives the same primitives** — locate an entry by slug, excise its body,
  re-insert it elsewhere — and each re-derives the section grammar from scratch, guarded only by
  hand-written asserts over raw list-slice arithmetic.
  **queue-kit already parses that grammar**: `queue_live_slugs` plus the section classifier behind
  `check-queue-sections` and `check-task-conservation`, with `bin/` shipping `queue-counts`,
  `queue-edges`, `queue-index` and `roadmap` over it. The write side is not missing knowledge,
  only a caller — **the grammar has one reader and N ad-hoc writers**.
  **This is a correctness argument, not a convenience one.** `check-task-conservation` exists
  because entries get lost in exactly these moves, so gating the damage after a hand-rolled
  `del lines[start:end+2]` is the enforcement-first inversion: removing the duplication outranks
  gating it.
  **Deliverable, and why `[design-pending]`:** a slug-addressed `queue-kit/bin/queue-edit.sh` with
  promote/done/defer/icebox verbs reusing queue-kit's own classifier and running the conservation
  check over its own output before writing — the write-side counterpart to the `queue-index`
  arm. What is open is which grammar it writes against, since
  `queue-entry-grammar-single-owner` records that queue-kit carries two and a verb must pick a
  side.
  **RELATED, NOT DUPLICATE:** `amendment-done-move-assertions` designs a *gate* for the Done-move
  contract a verb would make unreachable; `scope-rename-guard-deadlock` was this same missing-verb
  shape on `.workflow/WORKFLOW-STATE.txt`, and it closed 2026-08-13 with `enter-stage.sh --rename`
  — worked precedent for the verb this entry wants, on the sibling surface.
  **Cost while deferred:** every stage session that restructures the queue writes and debugs a
  one-off mutator against a grammar it does not own, under a conservation gate that catches the
  loss only after the fact.
  Filed 2026-08-12 by close, draining the gap inbox; raised by the operator mid-iteration off
  noticing `done-move.py`, and verified against the tree rather than taken from the observation.

- **bin-argv-shape-residual-member** [design-pending] — one `bin/` tool was left outside the
  argument-shape contract this iteration landed, and it is the only member of its own class.
  `SPEC-bin-argv.md` shipped the contract against a five-member census;
  `gate-sdk/bin/run-gate-tests.sh` is a sixth with the same arity-only-not-shape defect, found at
  align and **deliberately not folded in** — expanding a pending amendment's member count mid-align
  widens its asserted deliverable past align's stage contract.
  **The defect, re-verified at close rather than carried:** `TESTS_DIR=${1:-…}` and
  `GATE_DIRS=(${@:2})` take free-text positional directory paths gated only by `$# -gt 1`, with no
  `-h`/`--help` handling anywhere in the file. Witness: `bash gate-sdk/bin/run-gate-tests.sh --help`
  prints `no fixture tree at --help` — a misleading nonexistent-directory error where the contract
  wants usage text.
  **Milder than the amendment's five, and the difference is what makes it deferrable**: it writes
  nothing durable, so a bad argument fails a `-d` existence check rather than corrupting a governed
  surface. That is severity, not correctness — the contract still does not hold across the roster it
  claims.
  **Deliverable, and why `[design-pending]`:** apply the shipped contract to the sixth member, then
  decide whether the roster is closed. The census that found five and the sweep that found the sixth
  used different screens, so what is open is whether `bin/` membership is derivable — a roster gate
  over every kit's `bin/` — or stays a hand-curated set that the next tool added silently escapes.
  **Cost while deferred:** the contract reads as roster-complete while one member fails it, which is
  the shape a later session trusts rather than re-screens.
  Filed 2026-08-13 by close, draining the gap inbox; found at align 2026-08-12, escalated to the
  lead in the same session, and corroborated by a survey filed that session.

- **instruction-motivation-owner** [design-pending] — the what-vs-why rule for instruction
  surfaces has an owner for *restatements* and no owner for original motivation.
  **Two halves of this were already answered and this entry does not re-ask them**, corrected at
  close 2026-08-13 against the source: the general rule is delegation-kit/SPEC.md §Operative
  residency — (a) unreachable trigger, (b) imperative only, (c) adjacent citation — and that same
  SPEC already **rules it owes no gate**, with the structural reason stated so a later session does
  not try to build one that cannot exist.
  **The residual is a reach gap, verified by reading the rule's own scope.** §Operative residency
  opens *"A rule may be restated as an imperative in a surface that does not own it"*, so it
  presupposes an owner elsewhere. Original motivation authored **in** a surface that owns no rule
  falls outside it: there is nothing being restated, and (b)'s "the reasoning stays with the owner"
  has no owner to name. That is the exact shape operator ruling 2026-08-13 corrected by hand in
  `delegation-kit/templates/agent-execution.md`, where the fix was to *create* the owner
  (delegation-kit/SPEC.md §The delegation model) and then point at it.
  **Nothing applies (b)/(c) at authoring time either.** Both conditions are read by an author or a
  reviewer, and no stage, template or roster puts that read in front of anyone authoring a template
  surface — so the conditions are correct and unfired.
  **Deliverable, and why `[design-pending]`:** widen the rule to reach original motivation, or state
  in the SPEC why the two cases stay separate, and give (b)/(c) an authoring-time reader. What is
  open is which surface owns the widened rule, since the failure spans kits while §Operative
  residency is delegation-kit's.
  **No gate is owed and that is settled, not deferred** — the line above rules it, and this entry
  inherits the ruling rather than reopening it.
  **Cost while deferred:** every template surface authored keeps accreting motivation the owning
  SPEC should hold, and the corrected instance is one file out of a corpus of twenty.
  Filed 2026-08-13 by close, draining the gap inbox; the bullet's own claim that the rule and its
  gateability were unfiled is false on both halves and is corrected here rather than inherited.

- **instruction-surface-sweep** [design-pending] [blocked-by: instruction-motivation-owner]
  — one full pass applying the what-vs-why rule across the instruction-file corpus, once.
  **Operator-ruled 2026-08-13**, and deliberately moved out of that session's scope into close's
  filing rather than started mid-iteration.
  **The corpus is derived, never carried:** `git ls-files "*/templates/*.md" ".claude/agents/*.md"`
  minus the `gate-tests` fixture copies — **20 files, 2020 lines** at 2026-08-13, re-derived at
  filing time. Re-derive it at promotion; the command is the roster, the numbers are a dated
  measurement.
  **What the sweep applies** is the rule as ruled: an instruction surface carries the operative
  instruction, not the reasoning that justified it. The already-worked example is commit `bdfaed3e`
  on `agent-execution.md`, which is the calibration for how much a section loses — lead-ins,
  dispositions and commands stay; the grounds move to the owning SPEC.
  **The move is relocation, not deletion, and not annotation.** Grounds go to the surface that owns
  the mechanism; a `spec:` or exempt tag that keeps the prose in place blesses the restatement,
  which is itself the defect (CLAUDE.md §This repo is governed by its own kits).
  **Deliverable:** every one of the twenty read against the rule, each violation either relocated to
  a named owning section or explicitly ruled compliant, with the compliant calls stated so the next
  differential sweep inherits a baseline rather than re-deciding.
  **Cost while deferred:** the rule is ruled and applied to one file, so the corpus is nineteen
  files of unswept prose under a rule that already binds it.
  Filed 2026-08-13 by close, on operator direction; blocked on the reach question above, because a
  sweep for violations of a rule whose scope does not reach the corpus has no stated predicate.

- **close-differential-instruction-sweep** [design-pending] [blocked-by: instruction-surface-sweep]
  — after the full sweep, close keeps the corpus swept differentially instead of re-reading it
  whole.
  **Operator-ruled 2026-08-13** as the second half of the sweep shape: one full pass, then close
  runs differential passes in future iterations.
  **The worklist is mechanically derivable and the judgment is not**, and naming that split is the
  point — no gate is claimed over the what-vs-why call, only the set of files to look at:
  `git diff --stat <prev-close>..HEAD -- '*/templates/*.md' '.claude/agents/*.md'`. This does not
  contradict the standing ruling that the rule owes no gate, because a derived worklist is not an
  oracle over the judgment applied to it.
  **It self-applies, which constrains the deliverable's own shape.** The obligation lands in
  `lifecycle-kit/templates/stages/close.md`, itself one of the governed surfaces, so the paragraph
  installing it must be imperative-only with a pointer to the owning SPEC or it fails the rule it
  installs.
  **Deliverable, and why `[design-pending]`:** the close-stage step, plus its `<prev-close>`
  resolution. What is open is where that ref comes from — the stamp file carries stage history, so
  the previous close's commit is derivable, but which surface owns the derivation is not settled.
  **Cost while deferred:** the full sweep's result decays from the next template edit onward, so a
  one-time pass with no differential successor buys a baseline that expires.
  Filed 2026-08-13 by close, on operator direction.

- **close-red-push-ownership** [design-pending] — the state machine has no owner for a close blocked
  by a red remote push.
  lifecycle-kit/SPEC.md never mentions the push at all — the push-to-green rule lives only in
  CLAUDE.md — so the two remedy shapes the SPEC does define, §Reopen after close and §The
  interstitial mitigation, are keyed to a **post-close defect** rather than to close's own
  verification failing at the remote.
  **Unspecified, all three:** whether a close stamp written before the push is valid while the
  remote is red, what re-verification close owes after an interstitial fix lands, and whether the
  fix rides close's iteration range or the next one.
  **Instance:** `gate-dispatch-stderr-becomes-argv`, where the fix commits landed after the close
  commit — the sequence the SPEC has no words for.
  **Deliverable:** a SPEC section, **not a gate.** The remedy mechanism already exists and needs no
  new tooling, stamp grammar, or tag; the missing half is the trigger and the stamp-validity ruling.
  The debt-vs-feature litmus and the refusal of a parallel hotfix track are already settled and are
  inputs here, not questions.
  **Cost while deferred:** each close that hits it improvises the protocol under time pressure, at
  the one boundary where an improvised state write is least recoverable.
  Filed 2026-08-13 by close, draining the gap inbox; filed by the lead mid-iteration 2026-08-13.

- **settings-pins-live-suite-coverage** [design-pending] — `check-settings-pins` is exercised by
  its fixtures and by no live suite, so the vendored path is unproven end to end.
  **Scoped against the tree rather than taken from the report that raised it.** The fixture pair
  does cover the real branches: `context-kit/gate-tests/check-settings-pins/{good,bad}/` each ship
  a `settings-pins.conf` and a `settings.json`, so the pass and the legible-violation dispositions
  both run. The claim that the gate only ever reaches its trivial branch is true of the **live**
  suites and false of the fixture suite, and the difference is the whole entry.
  **What no live suite reaches:** grepping `CONTEXT_KIT_SETTINGS_PINS` and `settings-pins.conf`
  across the tree returns the two checks and one gate-test and nothing else — no installer profile,
  no consumer smoke, no upgrade suite ever writes a pins file. So in every consumer tree the
  battery exercises only the absent-pins-file clean skip, and a vendoring defect that broke the
  gate against a real settings file would ship green.
  **This is the craft rule's own shape** (doctrine-kit/DOCTRINE.md, *Test from the real consumer's
  runtime*): the lower layer is covered and the higher one is not, which is the inverse of the
  usual finding and is why it reads as adequate coverage at a glance.
  **Deliverable, and why `[design-pending]`:** have a smoke profile pin a key and assert the gate
  bites on a violated pin. What is open is which profile owns it — a pin is consumer config, so the
  suite must author one without asserting that any particular key is pinnable, or it re-couples the
  kit to this repo's own pin set and breaks the provenance seam.
  **This iteration added a pin**, which is what made the hole visible; the pin is correct and
  independently fixture-covered, so nothing here is a red.
  **Cost while deferred:** a green battery in an adopter's tree carries no evidence that this gate
  ran against anything.
  Filed 2026-08-13 by close, from the roster sweep; raised by the lead and re-scoped here after
  reading the fixtures.

- **vendored-library-identifier-reach** [design-pending] — De-literalization's reach test gives
  two answers for a vendored `lib/*.sh` function, and the corpus holds both populations.
  The rule bans prose restating a source's **internal** identifier roster while allowing public
  contract names. Two readings of "internal" are both defensible and they disagree:
  **file-level** — a shipped vendored library is a public contract by construction, so its
  owning `§lib/*.sh` section may name its exports — or **kit-level**, the reach test an audit
  actually applies: nothing outside the kit calls it, so it is internal.
  **Both populations exist, which is why no reading can be adopted silently.**
  `gate-sdk/lib/gate.sh`, `guard-kit/lib/guard.sh` and `gate-sdk/lib/declaration.sh` have
  verified callers in other kits and in `scripts/` — public on either reading, so they decide
  nothing. `canon-kit/lib/spec.sh` (`spec_manifest_files` and the adapters beside it) and
  `evidence-kit/lib/evidence.sh` (eleven adapters, inventoried in one sentence at
  evidence-kit/SPEC.md §lib/evidence.sh) have **zero callers outside their own kit**: public
  under the first reading, and textbook "a SPEC subsection that inventories internal helpers"
  under the second.
  **What turns on it:** the kit-level reading makes those two inventories findings and buys a
  large corrective across two SPECs; the file-level reading clears them and matches the
  doctrine's own carve-out for a SPEC naming public functions as contracts.
  **Deliverable:** rule it in doctrine-kit/DOCTRINE.md's De-literalization rule so the reach test
  is stated rather than re-derived per audit, then sweep whichever population the ruling makes
  findings.
  **Why `[design-pending]` and not simply decided:** the governing docs do not resolve it and
  only precedent would, which is the case the spec-over-precedent rule says not to settle from
  history. Escalated to the lead at this close alongside filing.
  **Not blocking the audit it came from:** all seven findings that close's
  `internal-identifier-restatement` sweep fixed fail **both** readings, so the ruling changes
  nothing already landed.
  **A third population, found 2026-08-14 and settled by neither reading — the cross-kit
  citation.** `gate-sdk/SPEC.md` §The canonical-spec `spec_canonical_specs` cohort names and
  describes the algorithmic fault of `_spec_prune_kit_roots`, which is **canon-kit's**
  underscore-prefixed helper with zero callers outside `canon-kit/lib/spec.sh`. The fork above is
  posed as a kit's own SPEC inventorying its own helpers; here one kit's governed doc names
  another kit's private helper, and the file-level reading — "a shipped vendored library is a
  public contract, so *its owning* section may name its exports" — is silent on a section that
  does not own it. Recorded rather than ruled, and the entry is **not** widened by fiat: whichever
  reading the deliverable adopts must say what it does with a citation that crosses kits, because
  a sweep will keep meeting this shape. Re-escalated to the lead at the 2026-08-14 close.
  **Cost while deferred:** every run of that rostered audit re-derives the same fork over the
  same two libraries, and two auditors can reach opposite verdicts on identical prose. With the
  cross-kit shape added, the fork now has a branch no reading answers at all.
  Filed 2026-08-13 by close, raised by the delegated identifier sweep, which declined to rule it;
  third population added 2026-08-14 by close from the same rostered sweep, which declined again.

- **docs-corpus-derivation-manifest-divergence** [design-pending] — two gates declare a
  byte-identical `# graph:` couple and walk different corpora, so the manifest asserts a
  sameness the code does not honour.
  `check-docs-link-convention` derives its corpus with a bare `find` over
  the docs root — a filesystem walk that sees **untracked** files, a shape its 2026-08-18 port
  carried across the substrate deliberately rather than tidying — while
  `check-docs-render-fidelity` derives its own from `git ls-files`, tracked only, plus an
  underscore-directory exclusion and a `gate_path_pruned` filter the first gate has neither of.
  **The byte-identity premise NO LONGER HOLDS, re-measured 2026-08-24, and the entry's subject
  survives it.** Both once declared
  `couples=docs/*.md,docs/*/index.md,docs/posts/*.md dir=one valve=none tier=precommit`,
  identical to the byte. `shell-gate-tail-port`'s delta 8 ported render-fidelity, and each
  descriptor now also couples its own crate module — `native/src/gates/docs_render_fidelity.rs`
  against `native/src/gates/docs_link_convention.rs,native/src/fresh.rs` — so the two `couples=`
  fields differ on their module suffixes and agree on the docs prefix. The **divergence** this
  entry is about (find-over-untracked against `git ls-files` plus the exclusions) is untouched
  and still live; what needs re-taking is the framing, since the manifests no longer assert
  sameness outright and instead assert it over the corpus half alone.
  **Demonstrated, not reasoned:** at the 2026-08-13 drain an isolated checkout gained one
  untracked markdown page under the docs root; link-convention's page count went 70 → 71 and
  render-fidelity's stayed at 70. An untracked page is inside one gate's corpus and outside the
  other's with nothing in either manifest showing it.
  **Why `[design-pending]`:** which derivation is *correct* is the substance, and both have a
  live reason. Render-fidelity gates what the published site will contain, which is the tracked
  set; link-convention gates authoring conventions, where catching a page before it is staged is
  arguably the point. Converging them silently would weaken one gate; declaring the divergence in
  the manifests needs a `# graph:` field that does not exist. Adjacent second finding from the
  same read, reported as observed: link-convention calls bare `find` rather than the shared
  `gate_find` wrapper, so it does not honour the SDK prune set — verified null in effect today
  (no directory under the docs root matches the default prune set), a latent inconsistency
  rather than a live symptom.
  **Cost while deferred:** the graph artifact and the enforcement map both read `couples=`, so
  two gates with materially different reach still present as interchangeable to every derived
  projection over the docs half. The port-cohort bite is **spent** — both members are ported, and
  their module suffixes now separate them, so no cohort selected on identical couples can merge
  them; what remains is the projection reading, which the differing suffixes do not repair.
  Filed 2026-08-13 by close, draining the gap inbox; surfaced by a scope census and re-verified
  at the drain against both sources plus the empirical probe above.

- **expected-permission-mode-undeclared** [design-pending] — the repo governs its allowlist and
  its bash guard but states nowhere which permission mode it expects to run under.
  `.claude/settings.json` carries a 120-entry `permissions.allow` and no `defaultMode`, and no
  tracked surface names an expected mode either — re-verified at the 2026-08-13 drain by a tree
  grep, whose only hits were queue prose *about* the user-level setting. Neither the allowlist
  nor the guard can therefore be judged correct or stale without an out-of-band probe.
  **The failure this was measured from:** a lead session ran roughly two hours in `acceptEdits`
  while the user settings specified `auto`, across four stage boundaries, and neither operator
  nor lead noticed — the symptom presented as "auto mode is not reducing prompts". The root
  cause is harness-side and filed upstream; what stays repo-side is that the guard's own blocking
  rationale is mode-**conditional** — false under `auto`, true under `acceptEdits` and default —
  and no doc records which mode it is written against.
  **Why `[design-pending]`:** the unit could declare an expected mode in tracked settings, or
  make the guard read the mode rather than assume it (`permission_mode` arrives on every
  `PreToolUse` payload and `scripts/bash-guard.sh` discards it), and those differ in whether the
  repo *constrains* the operator or *adapts* to them. Distinct from
  `bash-guard-auto-mode-rationale`, which reworded the guard's messages to be mode-independent
  and is done: this is the repo declaring an expectation, a different assertion.
  **Relationship to `guard-grant-review`, stated without pre-empting it:** the mode is an input
  to that entry's open narrow-or-declare direction, since it changes whether a grant is load-
  bearing at all. Naming the dependence is not choosing the direction, which is operator-class
  and unruled.
  **Cost while deferred:** every session that reasons about the allowlist or the guard re-derives
  the mode by probe, and a mode drift costs hours before it presents as anything legible.
  Filed 2026-08-13 by close, draining the gap inbox; filed by the lead and re-verified here.

- **lead-state-durable-home** [design-pending] — the lead template forbids the lead from writing
  lifecycle state and gives lead state no durable home either, so it lives only in the
  conversation.
  recurrence: lead-state-durable-home 2026-08-14
  **Re-attested 2026-08-14, and sharpened from a missing feature to a defect.** The absence is
  not merely unprovided-for: delegation-kit/templates/agent-execution.md binds its durability
  rule to whoever holds findings they will act on and closes the role loophole in its own
  words — *"Neither the supervising role nor a session running outside any dispatch is
  exempt: the axis is what the session can do at this moment, never what it is"* — and a lead
  can commit. So the rule **names** the lead and hands it no discharge path, which is a
  broken obligation rather than an unbuilt convenience. Re-verified at the 2026-08-14 drain:
  the four `journal` mentions in `lifecycle-kit/templates/lead.md` are all the *stage
  session's* journal, and nothing anywhere mints, requires or checks a lead-side one.
  Measured cost that iteration: six operator rulings, three stage returns and an
  operator-corrected premise carried with nothing durable behind them, and no journal existed
  until the operator asked at the fourth stage boundary — by which point a compaction had
  made one stage session unreachable by name. That loss is worse than the 2026-08-13
  attestation's and is why the recurrence is stamped rather than the entry merely re-read.
  `lifecycle-kit/templates/lead.md` §Channel design assigns the **resume journal** to the *stage
  session* — the lead reads it and is forbidden to delete it — and §Stamps are authoritative
  rules out every lifecycle write. Re-verified at the 2026-08-13 drain by reading the template
  end to end: across 438 lines there is no lead-side journal, state file, or capture obligation
  of any kind. The claim is about an absence and that read is what settles it.
  **What has no home:** the batch roster and its tiering rationale, findings carried between
  batches, operator items awaiting a ruling, and anything learned mid-iteration that no stage
  owns. Measured, not inferred: a full investigation's findings existed nowhere but a transcript
  until the operator prompted the lead to write them down — after that session's second
  compaction.
  **Why `[design-pending]`:** the fix shape is a lead-journal obligation with a stated cadence
  (on dispatch, on completion notification, on any ruling or finding), but its *home* is the open
  question: `.tmp/` is swept at the scope boundary, which is right for per-iteration state and
  wrong for anything crossing it, and a tracked home collides with the invariant that the lead
  writes no governed state. Distinct from the gap inbox and the survey record, which are capture
  channels for findings a *stage* will drain, not a place a lead's own working state persists.
  **Cost while deferred:** compounding with session length — every lead compaction degrades or
  loses state, and the loss is silent, since a degraded roster reads exactly like a short one.
  Filed 2026-08-13 by close, draining the gap inbox; a lead filing, re-verified at the drain by
  reading the template end to end, the search an absence claim needs.

- **ro-bins-write-option-bypass** [design-pending] — `GUARD_KIT_RO_BINS` membership is tested as
  "the segment leads with this binary", but leading with a roster binary does not make the
  invocation read-only, and the read-only-pipeline rule's safety argument assumes it does. The rule
  is named rather than numbered here: guard-kit renumbers on insertion, and this citation had gone
  stale twice over before close 2026-08-22 corrected it.
  **Probed through the live hook at close, not reasoned:** `grep foo a.md | sort -o out.txt` and
  `sort -o tracked.md tracked.md | head` are both auto-allowed today, and each overwrites a named
  file. That rule's redirect check inspects `>`/`>>` targets only, so a write expressed as a
  binary's own option is invisible to it. `sort` is the shipped instance.
  **The `xargs` half of this finding is already discharged and must not be re-filed**: the
  guard-context-matching unit landed `_guard_is_ro_xargs` (guard-kit/lib/guard.sh), and the same
  close probe confirms `find . -type f | xargs rm -rf` and `grep -rln foo src | xargs sed -i …`
  now fall through rather than auto-allow, while `find . -name '*.sh' | xargs grep -l foo` still
  allows. What survives is the general predicate the discriminator solved case-by-case.
  **Distinct from `guard-command-prefix-wrapper`** (transparent prefixes for allowlist matching)
  and from the two consumer-side guard entries below: this is the roster knob's own predicate
  being weaker than the grant that reads it.
  **Fix shape, uncosted:** either a per-binary write-option denylist, or generalize the
  executor-shaped discriminator into the membership test itself.
  **Cost while deferred:** a standing auto-allow that overwrites tracked files with no prompt —
  the narrowest live hole in the permission surface, and the one a reviewer of the roster would
  never see, because the roster reads as a list of safe programs.
  Filed 2026-08-13 by close, draining the gap inbox; every allow/fall verdict above re-probed
  against HEAD at the drain rather than taken from the bullet.

- **consumer-guard-rule-coverage** [design-pending] — the guard decision table has no reach into
  the consumer copy, so this repo's most destructive guards are its untested ones.
  `guard-kit/bin/run-guard-tests.sh` drives `guard-kit/templates/bash-guard.sh`, so the three
  project rules in `scripts/bash-guard.sh` — the hook-bypass block, the harness-scratchpad path
  block and the `git clean -x` block — carry zero behavioral coverage. It was four until
  `scratch-execution-control-is-bash-only` generalized the scratch-run steer into rule 22 and
  deleted the copy's arm, which is the one exit this entry has ever had: a rule that qualifies for
  the generic lane leaves the uncovered one.
  `check-template-copy-parity` is their only gate and it reads the copy-divergence declaration
  shape, never what the rules do.
  **Distinct from `template-copy-parity-yaml-widening`**, whose predicate is byte-parity between a
  template and its consumer copy; this is the absence of firing/non-firing cases for rules that
  exist only in the copy.
  The kit-side rule is that every generic rule owes a firing and a non-firing case; the consumer
  side inherits the discipline and no mechanism.
  **Cost while deferred:** a copy-divergence edit that silently stops matching reds nowhere, on
  the four rules whose failure mode is a destructive command running unprompted.
  Filed 2026-08-13 by close, draining the gap inbox; the runner's target re-read at the drain.

- **guard-ruleset-registration-lockstep** [design-pending] — guard-kit's generic ruleset exists in
  three places and nothing holds them in lockstep.
  The numbered roster in `guard-kit/SPEC.md` §The generic ruleset, the set of named
  `guard_rule_*` functions in `lib/guard.sh`, and the fixed dispatch order inside
  `guard_generic_rules` must agree, and no gate asserts it.
  **Re-verified at the drain:** the SPEC names neither the `guard_rule_` prefix nor the
  dispatcher — a grep for either over `guard-kit/SPEC.md` returns nothing — so the only statement
  of the convention is a code comment in `lib/guard.sh` citing a section that does not carry the
  fact. (The gap bullet cited that comment at line 89; it is now line 262, the file having grown
  at build — the citation was already stale when filed, which is the class's own failure mode.)
  `check-lifecycle-registration` and `check-doctrine-registration` are the analogue gates and
  guard-kit has none.
  **Declined once, on stated grounds**, inside the guard-context-matching amendment's delta 9: the
  convention is unwritten, so gating it means authoring it, on a surface the operator ruled out of
  the 2026-08-13 unit set — and the gate would have been green before and after every delta in
  that amendment. The amendment is deleted at merge, so the decline survives only here.
  **A fourth correspondence sits beside the three**, surfaced by the close audit that reviewed
  this surface: each rule's SPEC subsection states the inert classes it declares
  (`Declares sq dq hd`), transcribed from the rule's own `guard_skeleton` call site, and nothing
  holds those in step either. Judged legitimate rather than a restatement defect — the class
  vocabulary is `guard_skeleton`'s public parameter contract and which classes a rule treats as
  inert is a behavioral fact about the rule — but it is a fourth un-gated correspondence, and
  whether a vendored library's exports read as public or internal is the open question
  `vendored-library-identifier-reach` owns. Size this unit against that ruling, not ahead of it.
  **Cost while deferred:** a rule added to the roster and not to the dispatcher, or the reverse,
  ships silently; the three-way correspondence is exactly what a registration gate is for.
  Filed 2026-08-13 by close, draining the gap inbox; the SPEC re-grepped for both identifiers.

- **scan-prompts-blocking-half-blind** [design-pending] — guard-kit's own KPI cannot see the
  failure mode the guard exists to produce, so a change trading a prompt for a block reads as
  pure improvement.
  `guard_block` exits 2, and `guard_log_fallthrough` is the **last** line of both the template and
  the consumer hook — re-verified at the drain: `guard-kit/lib/guard.sh` blocks at its
  `guard_block`, and the fall-through log write is line 42 of 43 in `scripts/bash-guard.sh` and
  line 17 of 18 in the template. So a blocked command never reaches `GUARD_KIT_LOG`.
  `scan-prompts` reads that log alone, so its count is fall-throughs only and every refusal the
  guard itself issues is invisible to it.
  **Distinct from `scan-prompts-truncation-quote-desync`**, a false-positive defect in parsing the
  log's existing lines; this is a whole class of event never written to the log at all.
  **Cost while deferred:** `exit-echo-decoration-guard-vs-habit` had to name the decision table as
  its success instrument because the KPI cannot measure it, and drift-kit's prompting KPI
  understates guard friction by the entire blocking half.
  Filed 2026-08-13 by close, draining the gap inbox; the two hook tails re-read at the drain.

- **spec-section-title-collision** [design-pending] — two sections in one SPEC may carry the same
  title, and a `§`-pointer then resolves to the wrong one from the day it is written.
  Live instance, re-verified at the drain: `guard-kit/SPEC.md` carries **Consumer rules** at the
  `##` level (line 244) and again at the `###` level (line 508). Section-resolving readers take
  the first match, so every `§Consumer rules` pointer binds to the `##` one — including both
  pointers in `scripts/bash-guard.sh`, whose second one means the `###` section.
  `check-spec-pointer` stays green, because the section exists.
  **Distinct from `qualified-pointer-section-ownership`**, whose predicate is whether a pointer's
  named section still *owns* the claim — a staleness question about one resolving target. This is
  a resolution-*ambiguity* question: two targets carry one name, so no amount of ownership
  freshness fixes it. Cited because the citation-liveness family is one predicate question and
  this is a second predicate that family would have to absorb.
  **Cost while deferred:** a pointer that reads as precise and resolves elsewhere, which is worse
  than a broken pointer because nothing signals it.
  Filed 2026-08-13 by close, draining the gap inbox; both SPEC headings re-read at the drain.

- **handoff-premise-reverification-placement** [design-pending] — `Probe-before-assertion` is
  resident in the always-loaded doctrine and did not bind, so the open question is where a
  premise obligation actually holds.
  **The measurement is this iteration's own.** Six premises stated on governed surfaces or in
  dispatches were false, each one cheap command from being caught: gate-sdk/SPEC.md's `declare -A`
  bridge claim; the scope survey's rejected-rival sizings; a gap count of six where four landed;
  `exit-echo-decoration-guard-vs-habit`'s stated grounds; and two guard-amendment deltas. Two more
  were caught inside close itself — a delegated sweep returned a queue slug
  (`command-prompts-transparent-wrapper`) that exists nowhere in the tree, and this session's own
  first grep for a queue claim missed it on a line wrap and nearly recorded a false negative.
  **Every one was caught downstream, by a session that re-verified instead of complying.**
  Detection worked; prevention did not — which is the finding, because the rule aimed at
  prevention was already loaded in every one of those sessions.
  **The hypothesis the unit tests:** what binds is a *receiving-side* obligation at a **named
  handoff**, not a standing instruction at the point of assertion. The tree has exactly one such
  obligation today — close's per-bullet gap re-verification — and it is the one that keeps firing.
  Candidate handoffs to place a sibling at: the lead→stage dispatch relay, and survey citation.
  **Note the survey witness does not cover this**: it diffs the corpus since the survey's rev and
  re-runs its oracle, which is a *staleness* test and cannot detect a claim that was never derived.
  **Why `[design-pending]` and not a rule edit:** placing the obligation everywhere converts one
  re-derivation into a permanent per-handoff tax, which is the cost the always-loaded tier exists
  to refuse. The unit owes a placement argument, not a slogan.
  **Distinct from `delegation-provenance-floor`** above, which is one specific unobservable
  (whether a return arrived). This is the general placement question, and it stands whichever way
  that one is ruled.
  **Cost while deferred:** the rate is measured and non-trivial — six on governed surfaces in one
  iteration — and each survived a competent session's summary before something downstream caught it.
  Filed 2026-08-13 by close, from its own Lessons judgment.

- **single-kit-smoke-precondition** [design-pending] — a per-kit consumer-smoke invocation carries
  an undocumented cross-kit precondition, so a maintainer debugging one kit hits a failure that is
  not their change.
  **Re-verified by running it at the drain**, not inferred: `bash
  gate-sdk/bin/run-consumer-smoke.sh lifecycle-kit` fails at `install-lifecycle` with
  `agent file not found: CLAUDE.md`, because that file is written by an earlier kit's smoke in the
  full run. Only the full run is valid, and nothing says so.
  Pre-existing rather than introduced by the sixth cohort; surfaced there because that batch ran
  the single-kit form.
  **Distinct from `consumer-smoke-subset-accounting-verdict`**, whose predicate is a wrong
  registration count under a kit subset. This run does not mis-count — it errors outright, on an
  unstated ordering precondition.
  **Cost while deferred:** a false red charged to whoever next runs a single kit's smoke, which
  is the debugging path the per-kit form exists to serve — and it is charged to a maintainer
  already debugging something else, which is when a spurious failure is most expensive.
  Filed 2026-08-13 by close, draining the gap inbox; re-verified by running the command.

- **amendment-work-class-label-placement** [design-pending] — the amendment work-class label is
  mandated with no placement, so each spec session re-establishes it from a deleted precedent.
  `lifecycle-kit/templates/stages/spec.md` requires every delta to carry a work-class tag and is
  silent on where it goes. Two placements are attested one iteration apart: a heading suffix on
  each What-changes subsection, and an inline bracket tag after the bolded delta sentence.
  **Both amendments are deleted at merge**, so the precedent is recoverable only from git history
  — which means the convention is re-derived, not read, every time.
  **Cost while deferred:** the lead reads these labels at batch-cut time, so a placement it
  cannot find is a label that does not work — the tag is authored every iteration and consumed
  by nobody whenever it lands where the reader is not looking.
  Filed 2026-08-13 by close, draining the gap inbox; both placements read off git history.

- **close-eviction-refiles-without-checking** [design-pending] — close's backlog-eviction step
  files its finding without checking whether a prior close already filed it, and has now done so.
  **Self-demonstrating instance, found at this close:** the queue carried two entries for one
  finding — `icebox-worklist-roadmap-blind` (filed 2026-08-09) and
  `icebox-candidate-roadmap-filter` (filed 2026-08-13) — with the *same* three-row measurement in
  both bodies. Merged at this close into the elder slug, which is the one carrying an inbound
  citation.
  **The shape generalizes past that step.** Any close-stage sweep that files from a *recurring*
  worklist will re-file on the next iteration unless something checks; the eviction worklist is
  simply the sweep that recurs most reliably, because its input is stable by construction.
  `check-task-conservation` does not catch it — two distinct slugs carrying one finding is
  conserved.
  **Candidate fixes, none ruled:** a duplicate-finding check at filing time (needs a similarity
  oracle, probably not buildable); or the cheaper direction — have the recurring sweeps *state*
  their prior filing, so the next close reads a pointer instead of re-deriving. The second is the
  same receiving-side shape `handoff-premise-reverification-placement` argues for.
  **Cost while deferred:** one duplicated entry per recurring sweep per close, each of which then
  has to be found and merged by a later close reading 5000 lines of queue.
  recurrence: close-eviction-refiles-without-checking 2026-08-23
  **FIRST RECURRENCE, 2026-08-23, self-demonstrating TWICE in one close and generalizing the
  entry past its own step.** `leak-guard-and-assertion-meta-gate-port`'s close filed two gap
  bullets that each re-derived a live entry, and withdrew both once an audit sweep surfaced the
  owners. One came from the eviction step, re-filing `icebox-candidate-eligibility-unapplied`
  with the same roadmap-tag measurement for the FOURTH time across four closes. The other came
  from the PROMPT-FRICTION triage, re-deriving `guard-read-steer-tool-coverage`'s awk question
  from the log — which is the paragraph above confirmed rather than merely restated: the shape
  is any close-stage sweep whose input recurs, and the eviction step is only its most reliable
  instance. It also settles which candidate fix is reachable. A similarity oracle is not needed
  to catch either: both owners were found by a plain slug-and-subject grep of the queue, so the
  cheap direction is not merely cheaper but sufficient — the missing step is a *lookup before
  filing*, and neither sweep performed one. The contaminated measurement that came with the awk
  bullet is a second cost the entry had not priced: a re-derived finding also re-measures, and
  a worse measurement can overwrite a better one if the duplicate is promoted rather than caught.
  **THIRD INSTANCE, 2026-08-23, and the SHARPENING is the datum rather than the count.**
  `battery-runner-port`'s close read the audit-roster row `close-surface-actually-read` — which
  already carried the prior close's finding that reading a capture surface is not the same act as
  checking the queue for the owner — then filed five bullets that DID grep for an owner and one
  that did not. The one that skipped it arrived off the **eviction worklist** rather than a capture
  log. So the rule is neither unwritten nor unread: it was applied per-bullet by habit instead of
  as a step, and the bullet arriving through the surface the row does not name is the one that
  missed. The generalizable form is narrower and sharper than that row's wording — **the
  owner-check is owed by every filing, not only by one read from a capture log** — and it is a
  *lookup step*, which is exactly the cheap candidate fix above rather than a new one.
  Filed 2026-08-13 by close, from its own backlog-eviction step.

- **consumer-smoke-targeted-mode-registrar-scope** [design-pending] — the consumer smoke's
  targeted single-kit form fails on a cross-kit registration dependency that the targeting severs.
  **Re-verified live at this close**, at HEAD, on a clean tree: `bash
  gate-sdk/bin/run-consumer-smoke.sh gate-sdk` exits 1 with `7 unregistered gate(s) probed (3
  self-declared, 1 hand-declared, 3 unaccounted)`, naming `check-action-gh-repo`,
  `check-action-pinning` and `check-action-run-shell`, each `scratch exit 0`. The untargeted run
  is clean. **Pre-existing and unrelated to this iteration** — the lead confirmed both forms
  independently at HEAD.
  **Cause, probed rather than relayed.** An earlier telling blamed a `# smoke-unregistered:`
  declaration honoured only from the shipping kit; that is wrong — no such line names any
  `check-action-*` gate. The three gates are **shipped** by gate-sdk but **registered** by
  site-kit's `smoke/install.sh`, the kit that writes the workflow surface they read. The
  accounting derives the shipping kit from the `checks/` walk and honours registration from
  whichever kit wrote it, so the untargeted run — which installs site-kit — is clean. Targeted at
  gate-sdk alone, site-kit never installs, nothing registers them, and they probe exit 0, so
  neither exemption opens: self-declaration needs a scratch exit 2, hand-declaration needs a line
  from the shipping kit, which gate-sdk does not carry for them.
  **Options, neither ruled:** the targeted mode carries the kits that register the restricted
  kit's gates, or the accounting goes root-aware (a gate whose registrar is outside the selected
  roots is out of scope, not unaccounted).
  **Cost while deferred:** the trap is misattribution, and it has already nearly fired — this
  iteration's batch-B session almost charged the pre-existing failure to its own change. Any later
  session running the targeted form after touching a `smoke/install.sh` will read the same red as
  its own.
  Filed 2026-08-14 by close, draining the gap inbox (filed there mid-iteration).

- **install-disposition-smoke-accounting-split** [design-pending] — the precommit gate checks smoke
  registration for `zero-config` gates only, so an `on-surface` gate's missing registration is
  caught one stage late, at validate.
  `check-install-disposition` skips every non-`zero-config` disposition outright
  (`native/src/gates/install_disposition.rs:203` — `if value != ZERO_CONFIG { continue }`; the
  member became a `.gate` descriptor plus that module at `shell-gate-tail-port`'s delta 3, and the
  skip survives the port verbatim), and its clean line counts only the zero-config half. The full
  accounting — every shipped gate
  either registered in its kit's `smoke/install.sh` or carrying a `# smoke-unregistered:` line with
  a reason — lives in `gate-sdk/bin/run-consumer-smoke.sh`, which this repo runs as the evidence-kit
  `consumer_smoke` validate suite and never at precommit.
  **The instance, measured 2026-08-22.** Batch A landed `check-unmarked-claim` (`install:
  on-surface`) without registering it in `canon-kit/smoke/install.sh`. The precommit battery passed
  at 105 and then at 106 across four commits and three independent lead verifications; validate's
  `consumer_smoke` caught it, fixed in one line at `93cda68d`. The same iteration's batch-B gate was
  `zero-config` and WAS registered, so the seam is the disposition split rather than a careless
  batch.
  **Why `[design-pending]`:** the fix shape needs a ruling, not a build. Either widen
  `check-install-disposition` to run the full accounting for every disposition, or move the
  accounting out of `consumer_smoke` into a precommit member — the second buys the coverage but may
  re-buy smoke cost at every commit, which is the trade nothing here settles.
  **Distinct from `consumer-smoke-targeted-mode-registrar-scope`**, whose axis is the targeted
  single-kit mode severing a cross-kit registrar. This one is about which TIER holds the accounting
  at all, and it fires on the untargeted run that neighbour reports clean.
  **Cost while deferred:** one stage of latency on a mechanical zero-judgement condition a precommit
  gate could hold, plus a validate red that presents as a build defect — the batch session reads a
  registration omission as its own gate misbehaving.
  Filed 2026-08-22 by close, draining the gap inbox; the lead filed the bullet at validate and this
  drain re-verified the skip at its source rather than off the gate's `spec:` line.

- **spec-prune-normalisation-shell-oracle** [design-pending] — the shell twin of the
  `_spec_prune_kit_roots` normalisation repair is covered by no standing oracle.
  `canon-kit/lib/spec.sh`'s `_spec_prune_kit_roots` compared unnormalised paths, so a `..` scan
  root pruned nothing at all and silently widened every caller's corpus; the eighth cohort's
  edge-root parity run caught it and repaired it with `_spec_norm_abs`. The only standing
  assertion is `check-spec-dod-singleton.test.sh`'s prune-through-dotdot case, and that member now
  dispatches to the binary, so it holds the **crate's** normalisation, not the shell's.
  **Re-verified at this close**, not relayed: `canon-kit/checks/` carries only
  `check-spec-dod-singleton.gate` and `check-spec-derivable-section.gate` (both `.sh` files
  deleted), while `canon-kit/lib/spec.sh:171-201` still routes three surviving shell gates through
  the repaired prune — `check-surface-duplication.sh:34` (via `spec_canonical_specs`),
  `check-spec-embedded-source` (both readers) and `check-amendment-queue` (via
  `spec_amendments`), plus the README reader in that library. None of the three has a `.test.sh`
  scenario runner. The bullet named two of the three; the third is `check-amendment-queue`.
  **Since that re-verification** the fifth budget batch ported `check-amendment-queue` and the
  sixth ported `check-spec-embedded-source`, leaving one shell caller, `check-surface-duplication`.
  **THE COUNT IS NOW ZERO, and the DELIVERABLE AS WRITTEN IS MOOT — measured 2026-08-24 at
  `shell-gate-tail-port`'s cut.** Delta 9 ported that last caller. A grep over every tracked `.sh`
  finds no caller of `_spec_prune_kit_roots`, `spec_canonical_specs` or `spec_amendments` outside
  `canon-kit/lib/spec.sh` itself; `check-spec-dod-singleton.test.sh` names them in comments only.
  A scenario runner "for one still-shell caller" cannot be built for a caller that does not exist.
  **What is NOT disposed of by that**, and is what a scope re-takes: the *library still ships*, so
  a consumer's own shell gate sourcing `canon-kit/lib/spec.sh` meets the unrepaired-prune
  regression with nothing asserting against it. The repaired behaviour is held on the crate side
  by the eighth cohort's edge-root fixture, which is the alternative a re-scope folds this into.
  **The control discipline the cohort paid for survives either shape:** a *symmetric* break of the
  normaliser is invisible to this assertion, so the oracle must be an asymmetric one.
  **Cost while deferred:** the shell prune can regress to the pre-repair behaviour with a green
  battery, silently widening every surviving caller's corpus on any consumer leaving
  `CANON_KIT_SCAN_KIT_ROOTS` at 0. This repo now has **no** shell caller, so the cost lands
  entirely on adopters and this tree's battery cannot see it even in principle.
  Filed 2026-08-14 by close, draining the gap inbox; the bullet came from the eighth cohort's
  build session, which paid for the repair and declined to absorb its coverage hole.

- **baseline-row-prose-coupling-gate** [design-pending] — governed prose asserts what
  `.workflow/validate-baseline.txt` holds, and nothing checks it against the file.
  **The instance that bought this entry** was fixed at this close, not deferred: `gate-sdk/SPEC.md`
  claimed in two places that the baseline carried a held `installer_smoke fail` row. It was flipped
  to `pass` in `97683db2`, so a cohort pricing criterion 5 read a pointer to a mechanism it could
  not find, and the cheapest wrong conclusion was that the row had been dropped rather than earned
  out. Both sentences were re-worded at this close.
  **Why it is gateable, unlike its neighbours.** The general class — prose making claims about
  machine surfaces — is the human-audit class `gate-spec-claim-assertion-parity` already rules
  ungateable. This slice is not: a sentence naming `.workflow/validate-baseline.txt` and quoting a
  `<suite> <verdict>` pair is a decidable pattern, and the live file is a two-column lookup. The
  scanner reds when a quoted verdict disagrees with the row.
  **Deliverable:** a canon-kit gate over governed prose citing that file, with the `good`/`bad`
  fixture pair, plus a ruling on the past-tense form — a sentence deliberately recording a
  *retired* row (both repaired sentences are now exactly that) must not red, so the predicate needs
  a tense or a citation convention to key on. That convention is the design question.
  **A SECOND live instance was authored 2026-08-24, at this close's eviction review, and it is
  recorded because it moved this entry out of eviction range.** Ruling the `installer_smoke` row's
  attribution put a claim about that file's slug column into two governed surfaces at once —
  `bridged-knob-case-tmp-dir-override-inert`'s body and evidence-kit/SPEC.md §Baseline manifest —
  so the coupled prose is no longer one repaired sentence but a live pair, and the class now has a
  reader that a future re-attribution would silently falsify. **No `recurrence:` date joins:** the
  entry names an unbuilt gate rather than a defect, so authoring a new instance of the class it
  would catch is the class recurring, not the finding re-firing.
  **Cost while deferred:** low and slow, but it recurs on exactly the readers who most need the
  file — a cohort pricing criterion 5 reads the prose first.
  Filed 2026-08-14 by close, from its own gap-inbox drain and staleness review; kept in Deferred at
  the 2026-08-24 eviction review on the trigger above and on the live slug it names.

- **tracked-to-untracked-pointer-scope** [design-pending] — a tracked doc pointing at an
  untracked one can scope the pointer to the wrong reader, and nothing holds the scope
  against what the target actually owns.
  **The instance, fixed 2026-08-14 rather than deferred.** RELEASING.md's credential
  precondition read *"only a genuinely keyless sandbox defers these to the operator, **whose**
  push mechanics live in the local ops runbook"*. The possessive attached to the operator, so
  the pointer appeared to belong to the keyless branch — while the ops runbook actually owns
  the push transport and the release-account selection for the **credentialed closing
  session's own** push, which is its primary reader. A careful session reads the clause, is
  never sent to the runbook, and stops.
  **Attested, not hypothesised.** This close read `gh api repos/<owner>/<repo> --jq
  .permissions` as `push: false`, applied the precondition literally, and escalated the
  release as blocked on a permission defect. The machine carries two `gh` logins and the
  non-writing one was merely *active*; selecting the maintainer account returned full write.
  The runbook already carried the account roster, the working transport, and the line *"check
  the pushing identity, not the `gh` login, if this regresses again"* — the session never
  reached any of it.
  **Both surfaces were repaired in that same commit**, so this entry is not the fix: the
  pointer was re-scoped, the precondition now states that `permissions.push` answers for
  whichever account is *active* and that a machine may carry several logins, and the
  never-switch-identity prohibition was **deleted** rather than narrowed — it was wrong in the
  case that occurs here, and `release-runbook-identity-diagnosis` owns why. What replaced it is
  an identity-before-status-code rule plus the recording obligation the old clause was really
  carrying.
  **What survives the fix, and is why this is filed.** No gate holds a tracked pointer's
  *scope* against what its untracked target owns. This is the second surface where that gap
  has cost a session — the ops runbook's SSH-transport entry was the first — and the class is
  wider than releases: any tracked-to-untracked pointer can name *which branch may read it*
  instead of *what the target owns*, and only the second is checkable against the target.
  **Deliverable candidate, not ruled:** a stated rule that a tracked-to-untracked pointer
  names the target's ownership rather than its readership, plus whatever partial oracle that
  admits. The honest limit is that the target is untracked by design, so no scanner can read
  it; what may be gateable is the *shape* of the pointing sentence, not its truth.
  **Cost while deferred:** low frequency, high per-occurrence — the two known firings each
  cost a stage session's forward motion, and both were resolved only by a second reader who
  happened to hold the environmental fact.
  Filed 2026-08-14 by close, on operator direction, and drained in the same session that
  filed it — the cursor was already at close, so no later stage existed to drain it.

- **ops-push-transport-leaves-tracking-ref-stale** [design-pending] — the sanctioned push
  transport pushes to an explicit URL, so it never updates the remote-tracking ref, and the
  ahead-count reads wrong right after a successful push.
  The private ops runbook's push command targets an explicit HTTPS URL rather than the `origin`
  remote, because the credential helper has to be named on the command. A push that way updates
  the remote branch and leaves `refs/remotes/origin/master` untouched.
  **Attested at the v0.23.0 release close, not hypothesised.** Immediately after pushing 33
  commits successfully, `git rev-list --count origin/master..HEAD` still reported **34** ahead.
  Real remote state was confirmed out-of-band with `gh api
  repos/<owner>/<repo>/git/refs/heads/master`, and the count corrected to 1 by an explicit
  `git fetch <url> master:refs/remotes/origin/master`. Nothing reds: the push genuinely
  succeeded and the tree is correct — only the local view of it is wrong.
  **Cost while deferred, and why it is worse than cosmetic.** The ahead-count is the exact
  reading a close session uses to decide whether it still owes a push, and the stale form says
  "you have not pushed" *after* a successful push. One failure mode is a redundant push, which
  under this repo's one-to-two-push budget burns a whole `gates` run plus a
  `pages-build-deployment`. The other is a session that learns to distrust the number and
  re-derives remote state by hand at every boundary.
  **Candidate fixes, neither ruled:** have the runbook's push also update the tracking ref (a
  refspec that writes it, or the fetch above appended); or set the `origin` remote's `pushurl`
  so a plain `git push origin master` uses the credential helper and the tracking ref updates
  for free. The second looks better because it deletes the special-case command rather than
  lengthening it — but it touches local git config rather than a tracked surface, which makes
  it an ops-runbook decision rather than a repo one. That placement question is the
  `[design-pending]` half.
  **DISTINCT from `tracked-to-untracked-pointer-scope`:** that entry is a pointer failing to
  send a reader to the runbook at all; this is a defect in what the runbook's command leaves
  behind once it has been followed correctly.
  Filed 2026-08-14 by close at the release boundary, and drained in the same session — the
  cursor was already at close, so no later stage existed to drain it.

- **born-native-flip-enforcement-gate** [design-pending] — the born-native default is held by
  discipline alone, and the cheapest enforcing shape is now priced rather than refused.
  **This is the enforcement residue of a closed ruling, not a challenge to it.** TRAJECTORY.md
  §The closed rulings records the 2026-08-14 flip; gate-sdk/SPEC.md §The port-candidate
  criteria states the ground as **cost rather than impossibility** and files the disposition
  here rather than flagging and skipping it.
  **What changed at the merge, and it was measured rather than reasoned.** The shape was first
  refused on the ground that no discriminator exists while the port runs — a newly authored
  shell gate being indistinguishable from one still awaiting its port. A build session probed
  that ground and found it **false**: `git log --diff-filter=A` dates every declaration file,
  so demanding the `# substrate: shell — <cause>` header only of a member whose declaration
  entered history after the flip's own commit needs **zero** retrospective declarations,
  against the sixty the per-gate-header shape demands. The measurement is on the survey record
  (2026-08-14 build): 1697 ms over 72 paths at two `git log` calls each, 5/72 rename
  false-positives, all five resolved by `--follow`. `gates.yml` already checks out at full
  depth for `check-trajectory-fresh`, so the CI precondition is paid.
  **What it still costs, stated so the entry is not read as shovel-ready.** `--follow` is a
  heuristic and not a guarantee; the anchor is a literal commit no derivation produces; and as
  weighed the shape is **publisher-local**, because in a vendored tree every declaration file
  was added by the vendoring commit and the assertion would redden a consumer's whole registry.
  That last cost is smaller than it looks — the default's domain is exactly the crate-carrying
  tree — but making the shape consumer-safe is design work it has not had, and that work is
  what holds this entry at `[design-pending]`.
  **DISTINCT from `born-native-omission-accumulation`**, which owns criterion 5's omission pile
  and would not redden if the flip were enforced perfectly. This entry owns whether the flip is
  enforced at all. The two refused shapes stay refused on their own grounds (a maintained
  baseline roster rots at every cohort; the per-gate header becomes cheap only once the residue
  is small, which is its own revisit condition).
  **Cost while deferred:** every gate authored while the port runs is a coin flip against a
  rule nothing reads, and the failure is invisible — a shell gate born after the flip looks
  exactly like one of the sixty awaiting a port, which is the same indistinguishability the
  refusal was built on and the discriminator dissolves.
  Filed 2026-08-14 by close, draining two gap-inbox bullets — the batch-1 filing whose "no
  discriminator exists" premise the batch-2 probe falsified, merged into the corrected one.

- **msrv-move-clippy-arm-coupling** [design-pending] — raising the crate's toolchain floor
  un-suppresses clippy lints against unchanged code, and no surface budgets the pass.
  **Measured 2026-08-14, by a controlled single-line experiment at HEAD rather than inferred.**
  Clippy suppresses a lint whose suggested API postdates the declared `rust-version`, so moving
  `native/Cargo.toml`'s floor changes what `check-crate-arms` reports without a line of the
  crate changing. The 1.56 → 1.71 move surfaced **four findings in three modules the ninth
  cohort never edited**.
  **The coupling is now stated and is held by nothing.** context-kit/SPEC.md carries the
  sentence, in its own words machine-checked by nothing. Before this iteration no surface
  coupled the floor to the lint arm at all.
  **Why `[design-pending]`:** the obvious gate — red when `rust-version` moves without a clippy
  run in the same unit — asserts over a *commit shape* rather than a tree state, which is the
  class this repo's gates deliberately avoid, and a freshness stamp would be a maintained
  artifact derivation-first refuses. The plausible alternative is a *stage* obligation
  (a floor move budgets the pass) rather than a gate, in which case this entry closes against
  that rule instead of shipping one — and saying so is the design call.
  **Cost while deferred:** a floor move is rare and its surprise is total. The next one lands
  an unbudgeted lint worklist on whichever unit moves the floor, which is never the unit that
  planned for it.
  Filed 2026-08-14 by close, draining a gap-inbox bullet; the context-kit sentence was
  re-verified at the drain.

- **validate-baseline-suite-coverage** [design-pending] — two validate suites carry no
  held-constant-red baseline row, so a regression in either passes the baseline arm silently.
  **Verified at the 2026-08-14 drain by diffing the two files' suite columns**, not taken from
  the bullet: `.workflow/validate-baseline.txt` carries rows for **22** suites while
  `run-validate.sh` runs **24**. The two with no row are `dispatch_guard_tests` and
  `native_crate`.
  **Pre-existing, and an omission rather than lag.** Both suites predate the baseline's last
  edit (`eb94126b`, 2026-08-13), so nothing about this iteration introduced it.
  **The consequence is narrow and real.** `diff-baseline` flags a *new* failure against a
  *listed* scenario, so a regression in either suite is caught only by the suite failing
  outright — which is exactly the coverage the held-constant mechanism exists to add on top of.
  `native_crate` is the sharper half: it is the crate's own 92-test suite, it now spans a
  dependency graph and an MSRV floor that both moved this iteration, and it is the arm the next
  port cohort leans on hardest.
  **Why `[design-pending]` and not a two-line fix:** which scenarios are worth holding constant
  is an evidence-kit judgment, not a mechanical backfill — a baseline row is a *claim* that a
  verdict is expected to stay put, and adding twenty-four rows because twenty-four suites exist
  is the maintained-roster shape derivation-first refuses. The design call is whether the
  baseline is per-suite or per-scenario for a suite this size.
  **DISTINCT from `baseline-row-prose-coupling-gate`**, which is prose asserting what the
  baseline holds; this is the baseline not holding it.
  **Cost while deferred:** the two suites keep their weakest protection while the crate is the
  fastest-moving surface in the tree.
  **Second instance, one granularity down — 2026-08-19, and it is the entry's own open call.**
  The suite-level claim above re-verified unchanged (22 rows for 24 suites, the same two absent).
  What is new: *within* the `gates` suite the run reports 104 scenarios while the baseline names
  **75**, so 29 registered gates hold no row. Probed at the source rather than assumed
  (`evidence-kit/lib/evidence.sh:137,156,166`): the diff is directional, so a rowless gate going
  red is still a new-failure — what a `pass` row alone buys is that the scenario going **absent**
  reds too. The uncovered 29 can therefore be deregistered, renamed or silently skipped with the
  battery still green, which is a different loss from the one the suite-level half names.
  recurrence: validate-baseline-suite-coverage 2026-08-19
  Filed 2026-08-14 by close, draining a gap-inbox bullet the lead filed after the validate
  session declined to act on its own finding unilaterally.

- **gap-capture-argv-prompt-friction** [design-pending] — the mandated capture tools take their
  prose as an argv string, so every filing whose prose contains shell punctuation costs an
  out-of-band permission decision.
  recurrence: gap-capture-argv-prompt-friction 2026-08-15
  **Re-measured 2026-08-15: three prompting calls** (`file-survey.sh` twice, `file-gap.sh` once,
  `kfric.sh` none), against six the iteration before. Halved, and the halving is not progress —
  fewer captures were filed, and the per-filing tax is unchanged.
  **Diagnosed rather than allowlisted, per the triage criterion.** `bash
  lifecycle-kit/bin/file-gap.sh *` is **already** in the committed allowlist, alongside
  `file-survey.sh *` and `kfric.sh *` — so this is not missing coverage. The harness matcher
  refuses a command whose text carries an expansion or a redirect, and gap prose routinely
  carries both: a backticked slug is command substitution, and a bullet describing
  `jq -r … 2>/dev/null` contains a redirect operator inside its quotes. The glob cannot help,
  because the match never gets that far.
  **Measured this iteration:** `file-gap.sh` prompted **4** times, `file-survey.sh` and
  `kfric.sh` once each — six out-of-band decisions on the three tools the repo *mandates* for
  in-the-moment capture, which is the exact path CLAUDE.md says deferred capture ruins.
  **Deliverable, and it is small:** a body-from-file arm — `file-gap.sh --from <path>` reading
  the prose from a scratch file written with the editor tool — is a fully static command the
  matcher can grant. The same arm serves `file-survey.sh` and `kfric.sh`, whose `<finding>`
  fields have the same shape. What needs deciding is whether the arm is per-tool or a shared
  helper in `lifecycle-kit/lib/`, and whether the argv form stays (it should — a short gap is
  one call).
  **Why it is not a guard rule:** there is no better *form* to steer to today, which is what a
  guard rule requires. The form has to exist first.
  **Cost while deferred:** a friction tax that scales with how carefully a bullet is written,
  which taxes exactly the good filings.
  Filed 2026-08-14 by close, from the prompt-friction triage.

- **allowlist-path-existence-unchecked** [design-pending] — an allowlist entry naming a script
  path that does not exist grants a call that can only ENOENT, and nothing detects it, so the
  grant silently blesses the typo it encodes.
  **Measured 2026-08-15: seven of this repo's 112 local-overlay `Bash(` entries named
  nonexistent paths** — five under a `bin/` directory for tools that live under `checks/` or in a
  different kit. Two sessions this iteration actually called one, `gate-sdk/bin/md-section.sh`,
  whose real home is `context-kit/bin/md-section.sh` and is already committed-allowlisted; both
  got ENOENT instead of the prompt that would have named the wrong path.
  **Why the existing tool cannot see it.** `compare-settings-allow.sh` reports redundancy (a
  committed glob already grants the entry) and breadth (a declared probe the glob auto-allows).
  Both compare *globs against globs*; neither resolves a target. A dead entry is redundant with
  nothing and broad over nothing, so it reports clean — and it reported clean on all seven.
  **The failure mode is inverted, which is what earns this a check.** A missing grant costs one
  prompt and teaches the session the right form. A grant for the *wrong* path costs no prompt
  and teaches nothing: the session reads ENOENT as "the tool is gone" rather than "I named it
  wrong", and the entry survives to mislead the next one. Friction is the correcting signal
  here, and a dead grant suppresses precisely the signal that would have corrected it.
  **Why `[design-pending]`:** the corpus is the open part, and it is a tracked-versus-local
  question. The committed allowlist is tracked and gateable; the overlay is gitignored and
  per-machine, so a *gate* over it would red on a state no commit can fix — the shape a
  pre-commit gate must not have. Candidates: an advisory arm on `compare-settings-allow.sh`,
  which already reads both files, is already advisory, and would cost no new surface and no
  gate; a committed-only gate plus that advisory arm; or resolving only entries whose first
  token is a known interpreter followed by a repo-relative path, which is the subset decidable
  at all — a grant naming a system binary or a path outside the tree is not this check's
  business. **The check class is nameable and buildable**, which is why no "no scanner is
  possible" line appears here: it is *allowlist entry whose repo-relative target does not
  resolve*, and the first candidate builds it.
  **Cost while deferred:** paid per stale entry per session and paid silently, and it compounds
  — an overlay is append-mostly, so dead entries accumulate and the file degrades as a statement
  of what is actually granted. Seven were found at first look, by a session that was not
  looking for them.
  Filed 2026-08-15 by close, from its own prompt-friction triage; the seven were pruned in the
  same session, which removes the instances and not the hole.

- **bridged-knob-owner-for-consumer-gate** [design-pending] — the config bridge resolves a knob
  by the knob's own name, so a consumer-declared ported gate that needs a consumer-owned knob
  fails closed on every invocation with no library able to answer it.
  **The mechanism, read at the drain rather than taken from the filing.**
  `gate-sdk/lib/gate.sh`'s `_gate_knob_owning_kit` tries each `gate_kit_roots` basename,
  upper-cased with hyphens to underscores, as a `<KIT>_` prefix on the knob, and falls back to
  `gate_sdk_root`; `_gate_knob_value` then sources only that kit's `lib/*.sh` in a subshell. A
  knob no kit's prefix claims therefore resolves to gate-sdk, whose library does not define it —
  the bridge's first refusal, exit 2, on every invocation of that member.
  **The filing's mechanism was wrong in a load-bearing way, and the correction narrows this.**
  The bullet read it as "a gate in the consumer gates dir has no owning kit"; the gate's
  *location* plays no part, and a consumer gate declaring `GATE_SDK_WORKFLOW_DIR` resolves
  today. What is actually absent is the consumer's own config seam — the `gate-sdk-config.sh`
  in the gates dir that `lib/gate.sh` auto-sources for layout knobs — from `_gate_knob_value`'s
  search path. The gap is one missing source, not a missing owner.
  **Visible today as an asymmetry rather than a red.** `native/src/gates/release_bump.rs` and
  `native/src/gates/tightened_gates_note_parity.rs` each hardcode a workflow-dir const while
  `gate-sdk/bin/upgrade-smoke.sh` resolves the same file through `GATE_SDK_WORKFLOW_DIR`. The
  tenth cohort's three members declare no knobs, so nothing fails yet.
  **DISTINCT from `consumer-gate-port-disposition`, which it cites rather than re-files.** That
  entry owns the *declaration* question — the owner column and the conservation row, authored
  this iteration — and this owns the *dispatch* question, which that amendment names and
  deliberately leaves unanswered because no member of its first tranche declares a knob.
  **Why `[design-pending]`:** three candidates trade off. Add the consumer config seam to the
  resolver's search path (widest, but a consumer file's globals then enter the same subshell as
  the kit libraries, which the per-knob subshell exists to keep apart). Require a ported consumer
  gate to spell its knob with an existing kit's prefix (costs nothing to build, and makes a
  consumer-owned knob wear a kit's name, a provenance-seam inversion). Or let the `.gate`
  descriptor name its resolving library, which is precise and adds a descriptor field whose only
  user would be this case.
  **Cost while deferred:** paid in full by the first knob-declaring member of the remaining
  consumer tranche, and paid as exit 2 on every invocation — a gate that cannot run rather than
  one that answers wrongly, so it surfaces loudly rather than silently. Zero until then, which
  is why it files rather than fixes: nothing is wrong in the tree today.
  Filed 2026-08-15 by close, draining the gap inbox; mechanism re-derived against
  `gate-sdk/lib/gate.sh` at the drain and the bullet's account corrected here.

- **consumer-gate-roster-unread** [design-pending] — no roster reader covers a consumer-declared
  gate, so one added to or deleted from the consumer gates dir drops out of nothing.
  **Verified at the drain.** `check-readme-roster`'s corpus is `gate_kit_roots`, and it skips any
  root carrying no `checks/` directory. The consumer gates dir is neither, so its members are
  named by no README roster in either direction — the kit-README marker block that makes a kit's
  own roster falsifiable has no counterpart for them.
  **Re-verified 2026-08-15**, after the eleventh cohort emptied that dir of shell: it now holds
  thirteen `.gate` members and no `check-*.sh` at all. The substrate moved, the predicate did
  not — a `.gate` descriptor is exactly as unrostered as the script it replaced, so the entry's
  corpus changed shape without changing size-of-gap, and the finding stands unaltered.
  **What does cover them, and why none of it is a roster.** They register in `gates.list`, each
  carries a `# graph:` manifest, and each owes a fixture pair. Every one of those is a
  *mechanism* obligation discharged by a machine-readable artifact; none asserts that a
  human-readable document names the gate, which is exactly what the marker block buys.
  **Pre-existing rather than opened by the tenth cohort's port** — measured while surveying that
  port's readers, and unchanged by it in either direction.
  **DISTINCT from `consumer-gate-port-disposition`, which owns porting these gates and not their
  documentation coverage.** Porting all thirteen would leave this exactly where it is: the corpus
  predicate is the kit root, not the substrate, so a ported consumer gate is as unrostered as a
  shell one.
  **Why `[design-pending]`:** the roster's home is the open question, and it is a seam question.
  A vendoring consumer has no kit README to hold the block and may have no README at all, so
  widening `check-readme-roster`'s corpus to the gates dir needs a consumer-named document knob
  — a kit gate acquiring a consumer-shaped surface. The alternative routes the coverage through
  a *generated* projection instead: the enforcement map already enumerates every registered gate,
  so a check over generated output would be freshness rather than parity, and derivation-first
  reads that as the cheaper answer. Which is right turns on whether the roster is meant to be
  *authored* — a claim a human can get wrong, and so worth gating — or merely *available*.
  **Cost while deferred:** a consumer gate can be added or removed with no prose anywhere
  noticing, so "what does this repo enforce beyond its kits" is answerable only by listing a
  directory. Paid by every reader who asks it, and paid silently — there is no red.
  Filed 2026-08-15 by close, draining the gap inbox.

- **declaration-lib-refusal-output-leak** [design-pending] — both declaration-library token
  walkers emit resolved tokens onto the same stream as their refusal, so a container mixing a
  readable and an unreadable bullet reports the readable one as unreadable.
  **Verified at the drain, in both holders.** In `gate-sdk/lib/declaration.sh`,
  `decl_section_tokens` prints each good token as it walks, then appends the offending lines and
  returns 1; `decl_record_tokens` has the identical shape. One stream carries two meanings and
  the caller cannot tell which line is which.
  **Consequence, and why nothing is red.** `check-tightened-gates-grammar`'s finding list names
  a readable bullet as having an unreadable lead token, and
  `check-tightened-gates-note-parity`'s exit-2 diagnostic carries the same pollution. **Verdicts
  are unaffected** — a clean container still resolves clean and a polluted one still refuses —
  so the defect lives entirely in the text a reader is sent to act on.
  **Preserved rather than repaired at the tenth cohort's port, deliberately.** Delta 4 of that
  amendment rules that a port proves parity and does not fix rules, so
  `native/src/declaration.rs` reproduces the leak faithfully and
  `gate-sdk/gate-tests/declaration-lib-parity.test.sh` holds both holders to it. The doctrine
  deferred the repair out of the port; it did not schedule it.
  **The repair is known and cannot be one-sided:** buffer the resolved tokens until the loop
  ends and emit them only on success, **in both holders in one unit**, or the parity test reds.
  It rides a binary rebuild and whatever the two gates' fixture pairs assert about their output.
  **Why `[design-pending]` rather than a ready task:** the repair *site* is genuinely open.
  Buffering in the library changes a shipped output contract on two substrates at once; filtering
  at the two readers changes one substrate and leaves the library's stream ambiguous for the next
  caller. The first is correct and wider, the second smaller and defers the ambiguity.
  **Cost while deferred:** paid only when a mixed container actually refuses — a rare shape,
  since the declaration file is machine-appended — and paid as a reader sent to the wrong line by
  the one output whose job is to say where the defect is. Low, and non-rotting in content; but it
  does not decay either, because the parity test now pins the defect across both substrates.
  Filed 2026-08-15 by close, draining the gap inbox; not fixed inline because the change is
  shipped-output work across two substrates, which scope-gated intake files rather than starts.

- **deferred-pool-identifier-restatement-sweep** [design-pending] — the deferred pool entered
  `internal-identifier-restatement`'s corpus on 2026-08-15 and has never been swept under it, so
  the tree's longest-lived prose surface carries an unmeasured restatement debt.
  **The corpus ruling that created this.** The class's roster line was widened at the 2026-08-15
  close: the 2026-08-09 exclusion is a **conjunction** — a surface is out when recording is its
  whole contract *and* it is boundary-truncated — and the deferred pool fails the second half.
  The survey record stays out on both. The roster line carries the ruling; this entry carries
  the work it created.
  **The predicate is narrow, and the narrowness is the whole point.** What is swept for is a
  restated *call chain, roster or algorithm* — never a named identifier, since
  de-literalization's own digest is "prose cites names". A private helper named as the
  **subject** of a pending decision is a citation of instance and is clean. A sweep that reports
  every underscore-prefixed name it finds has misread its predicate and produced noise, which is
  the failure mode worth naming in advance because the corpus invites it.
  **Why the debt is plausibly real rather than theoretical, priced off the iteration that filed
  it.** Both restatements found at that close had rotted in place — a false call chain in a kit
  SPEC for six days, and a tree-state claim for roughly seven closes. A deferred entry has no
  truncation and no boundary, so its rot window is bounded only by how long the entry waits.
  Two entries filed at that same close name private helpers deliberately and cleanly, which is
  evidence the corpus is **rich** in the identifiers the predicate must distinguish rather than
  evidence it is dirty; the sweep's cost is in the judging, not the grepping.
  **Why `[design-pending]`:** the remedy is open in a way it is not for a SPEC. An entry's job is
  to carry a pending decision's grounds, so the chain may belong there — compressing it to a
  pointer can leave the decision unmakeable by the scope that inherits it. The unit must rule
  what a finding here *becomes*: a rewrite, a pointer, or an accepted carry with its rot window
  stated. Ruling that ahead of reading the corpus decides against a shape nobody has measured.
  **DISTINCT from `vendored-library-identifier-reach`, and deliberately not folded into it.**
  That entry owns *which identifiers count as internal* — the file-level versus kit-level reach
  question — and the corpus ruling left it un-narrowed on purpose. This owns *sweeping a surface
  that ruling just added*. Either may land first; neither closes the other.
  **Cost while deferred:** the class's `last:` stamp names an iteration whose sweep read the
  pre-widening corpus only, so the roster carries a cadence claim narrower than the class it
  labels. The roster line states that gap explicitly rather than leaving it inferable — but a
  stated caveat is not a sweep, and this is the same stamp-implies-coverage shape
  `audit-class-corpus-attestation` records. Paid at every close that reads the roster.
  Filed 2026-08-15 by close, on the lead's corpus ruling; scope-gated intake, so it is filed
  costed rather than swept by the session that received the ruling.

- **consumer-guard-rule-verification-lane** [design-pending] — a consumer's own bash-guard rules
  have no verification lane anywhere in the tree.
  guard-kit ships the decision table `guard-tests/cases.tsv` and guard-kit/SPEC.md §Testing
  requires every **generic** rule to carry a firing and a non-firing case. But
  `bin/run-guard-tests.sh` feeds `guard-kit/templates/bash-guard.sh`, which ships no consumer rule
  by design (§Consumer rules). Re-verified 2026-08-24: `scripts/bash-guard.sh` carries exactly
  three project rules — the `--no-verify` block, the harness-scratchpad steer and the
  `git clean -x` block — and no test file anywhere names any of them. The fourth, a scratch-script
  steer, left at `scratch-execution-control-is-bash-only`'s build by qualifying for the generic
  lane, where the table reaches it.
  **Why `[design-pending]`:** the seam is the open part, not the tests. A consumer table needs a
  runner entry point that feeds the *consumer's* guard rather than the template's, and that is
  either a knob on `run-guard-tests.sh`, a second table the kit reads from a consumer-named path,
  or a shipped harness the consumer calls with its own guard as the argument — and which one is
  right depends on whether a consumer table may also assert on generic rules, which is unruled.
  Provenance seam: the rules themselves are consumer content and must not become kit literals.
  **Cost while deferred:** every consumer rule is a hand-verified block whose narrowing cannot be
  measured, and §Testing's own point — that the decision table is the instrument precisely
  because a blocked command never reaches the friction log — applies to consumer rules with no
  table behind them. Enforcement-first wants the fix and its check in one unit; for a consumer
  rule there is currently nowhere to put the check.
  Filed 2026-08-15 by close, draining a gap-inbox bullet filed at spec.

- **waiter-loop-condition-predicate-gap** [design-pending] — guard rule 12 declines on a
  standalone loop-condition waiter, which is the shape its founding instance most likely took.
  **Lead-ruled at build 2026-08-15 and filed with its cost and its conflict already stated; this
  drain lands it as filed and does not re-open the ruling.** Rule 12 fires only when the
  `pgrep`/`pkill -f` pattern literal recurs elsewhere in the same command, so a standalone waiter
  issued as its own tool call falls through the recurrence test, and rule 13 declines on that
  same command because the sleep sits inside a `do…done` span. **Both new rules miss it** —
  re-verified by probe at this drain: the standalone `until`-form and the `while`-form both
  return exit 0 through `scripts/bash-guard.sh`, while the compound launch-and-wait form returns
  exit 2. Coverage is partial and honest rather than absent.
  **Founding-instance argument.** The attested instance recorded beside
  `waiter-predicate-self-match` is build batch A running two such waiters, plausibly the
  standalone shape — so the enforcement half landed 2026-08-15 may not cover the failure that
  motivated it.
  **Proposed predicate:** also fire when the `pgrep`/`pkill -f` segment sits in loop-condition
  position, between an `until`/`while` head and its `do`. Mechanically decidable off the token
  walk the bare-sleep rule already performs, so the machinery exists and the cost is one clause
  plus its firing and non-firing case in the decision table.
  **Why `[design-pending]` — a conflict a later unit must resolve by ruling rather than ignore.**
  The merged amendment states as a conservative direction that a `pgrep -f` whose pattern occurs
  nowhere else in the command is a genuine query and is untouched, and loop-condition firing newly
  blocks exactly that shape; and rule 12's own block message sanctions `kill -0` against a
  recorded PID wherever liveness is the condition, a legitimate case the widened predicate would
  reach. That message's clause was **widened 2026-08-16** by `waiting-rule-carrier-reach` — it now
  covers a child the session backgrounded itself, which is the shape a self-backgrounding waiter
  actually has, so the conflict this entry names got broader rather than narrower; the firing
  predicate itself is untouched and stays this entry's subject.
  **Cost while deferred:** the founding shape stays unblocked and its failure mode is silent —
  nothing reds, the work finishes correctly, and the only symptom is the foreground cap absorbing
  an unbounded wait, which reads from outside as a fixed cap-length wait.
  Filed 2026-08-15 by close, draining the gap-inbox bullet build filed on the lead's ruling;
  scope-gated intake, so it was filed costed rather than taken in flight.

- **overhead-meter-measures-the-lead** [design-pending] — under a live lead the overhead meter
  measures the supervising session, never the stage session its own binding names.
  **Probed at close 2026-08-15, both halves.** `lifecycle-kit/bin/session-id.sh` is
  delegation-aware: with `CLAUDE_CODE_CHILD_SESSION` set it scans the lead's `subagents/` dir
  alone and returned this close session's own id. `drift-kit/bin/overhead-meter.sh` resolves
  "newest transcript" by a bare glob over the project dir with no such branch, and returned the
  **lead's** id — twice, with a growing byte total, so it was tracking the live supervising
  transcript rather than a stale one. Two tools, the same phrase, different answers.
  **What it costs, and why it is not a rounding error.** drift-kit/SPEC.md §The overhead meter
  names the producer as the consumer's close-stage binding, "invokes the meter on the closing
  session". Under the lead-orchestrated posture this repo runs, that is exactly the case where
  the meter cannot reach the closing session, so every close-stage row taken under a live lead is
  the lead's own governance/task split filed under the lead's key. `kpi-overhead` reads `pct`,
  `gate` and `total` off those rows, and the lead's shape — dispatch prompts, notifications,
  little tool work — is not the shape a stage session has. The trend is not corrupt so much as
  **measuring a different population than it names**, which is the harder failure to notice.
  **Not a data-repair task.** Each logged row is a truthful measurement of a real session, keyed
  by its own `session8`, so nothing needs deleting and re-measuring cannot fix it: the meter run
  again from the same session resolves the same wrong transcript.
  **DISTINCT from the mis-pick limit lifecycle-kit/SPEC.md §bin/session-id.sh already owns**,
  and deliberately not folded into it: that limit is about `session-id.sh`'s *fallback* picking a
  just-finished subagent when the env uuid is absent, and it is documented at
  `lifecycle-kit/templates/lead.md`. This is the opposite direction — the env uuid is present and
  correct, one tool consults it and the other does not.
  **Why `[design-pending]`:** the seam is the open call. drift-kit could grow its own
  delegation-aware resolution (duplicating a derivation lifecycle-kit already ships and owns), or
  take a transcript path from its caller so the close binding passes what it already knows, or
  depend on lifecycle-kit's tool across a kit boundary that no kit dependency currently spans.
  The third is the cheapest and the least obviously admissible, which is exactly why it wants a
  ruling rather than a patch. Whether the lead's own overhead is separately *worth* measuring —
  it may be, and the accidental rows are the only such data that exists — is a second question
  this entry raises and does not answer.
  **Cost while deferred:** an account-bearing trend under `.metric/` keeps accumulating rows
  attributed to the wrong population, and every efficiency claim read off `kpi-overhead` while
  the lead posture is live is a claim about supervision cost wearing a stage-session label.
  Filed 2026-08-15 by close, probed while reconciling its own meter output against its own stage
  stamp; scope-gated intake, so it is filed costed rather than fixed in flight.

- **fixture-assertion-liveness** [design-pending] — nothing catches a fixture asserting a message
  no gate emits, or a fixture citing a script already deleted from the tree.
  **Re-verified at the drain.** `check-gate-fixture-coverage` is the only fixture-subject gate in
  `scripts/gates.list`, and its invariant is that a pair *exists* — never that the pair's expected
  strings are still emitted. `run-gates.sh` runs gates, not their fixture pairs, so a gate whose
  message changed still passes itself while its own `bad/expect.txt` goes stale.
  **Attested, not hypothetical.** `f8795712` changed `check-value-rollup-fresh`'s stale message
  and left the fixture asserting the old string; the battery stayed green and the defect surfaced
  a stage later as the only red in a 24-suite validate manifest (`b7e47707`), repaired by
  `83f79e7d`. The commit that broke it was the commit that could have caught it.
  **Why `[design-pending]`:** the oracle is the hard part. A lint cross-referencing each
  `*/gate-tests/*/bad/expect.txt` literal against its gate's own source works for a shell gate
  and gets harder for a ported one, where the string lives in a compiled binary and would have to
  be read back out of it. A liveness check on any script path a fixture's `args`/`projection.txt`
  names is the cheap half and is worth costing separately.
  Class: mints a gate name, so canon-kit/SPEC.md's new-names litmus makes it a **feature**.
  **Cost while deferred:** paid once per iteration that changes a gate's user-facing text, and
  paid at the worst place — a validate red, one stage after the commit that caused it.
  Filed 2026-08-16 by close from the gap inbox; filed by the build re-entry that fixed the
  instance rather than designing the gate in flight.

- **fixture-assertion-coverage-unmeasured** [design-pending] — no surface records which of a
  gate's assertions its fixture pair actually reaches, and no gate derives it.
  **gate-sdk/SPEC.md §The port-candidate criteria states the discharge condition well** — criterion
  4's pair must carry every arm of the derivation being ported — but the condition is asserted per
  port and verified by nobody, so a port can satisfy the sentence and ship the hole it points at.
  **The cost is attested rather than theorised.** `check-graph` shipped for months with a
  `good/`+`bad/` pair whose two `args` files were both `--amend-only`: exactly ONE of its nine
  assertions was fixtured, and A/B/C/D/E/F/H/I rested on a live tree that was green because it was
  clean. A human reading the two args files at spec 2026-08-21 found it, no oracle did, and the
  widening to nine-of-nine was that iteration's build work (gate-sdk/SPEC.md §check-graph). Every
  other multi-arm member is in the same unmeasured position and nobody knows which.
  **Re-verified at promotion, and it NARROWS the candidate set.** Both `args` files still read
  `--amend-only` today: the nine-of-nine discharge was bought with **bespoke drivers**
  (`check-graph-tree.test.sh` for A-F, `check-graph-refs`/`-cap` for H and I), not by widening the
  pair — because a case runs one argv, and only `--amend-only` reaches an assertion without also
  reaching D and E. So an oracle phrased as *the pair's `expect.txt` must cover every assertion* is
  falsified by the very member that motivated it; the unit of coverage is the member's whole driver
  set, which is what makes the derivation hard.
  **Candidate shapes, none costed:** an assertion-id roster per gate that the driver set must
  cover; a coverage arm on `check-gate-fixture-coverage` comparing exercised paths against the
  module's assertion set; or a stated per-member coverage line in each gate's SPEC section —
  derivation-first's weakest arm, since it is maintained rather than derived.
  **DISTINCT from three siblings.** `unregistered-gate-fixture-coverage` asks whether a member has
  a pair at all; `gate-file-coverage-closure` asks whether a gate is inside `check-gate-tamper`'s
  glob; `fixture-assertion-liveness` asks whether a pair's expected strings are still emitted.
  This asks which of a member's assertions any driver reaches.
  **Why `[design-pending]`:** the three shapes differ in kind — a maintained roster, a new gate
  arm, and a SPEC convention — and choosing needs the assertion-identity question answered first,
  a Rust `// assertion X` comment being no machine-readable id today.
  Class: the first two shapes mint a governed name, so canon-kit's litmus makes it a **feature**.
  **Cost while deferred:** criterion 4's discharge is asserted per port and verified by nobody, so
  every remaining port re-buys a hand audit or ships without one.
  Surfaced 2026-08-21 draining the knowledge-friction log at `graph-port-and-config-seam`'s close;
  promoted 2026-08-22 at scope from the carried gap inbox, which re-verified both args files and
  the discharge record against the tree.

- **done-slug-ownership-citation-report** [design-pending] — governed prose says a queue slug
  "owns" an open question in the present tense, and nothing notices when that slug lands.
  **Two live instances, both found by hand at this close's audit sweep and both fixed here.**
  `gate-sdk/SPEC.md` asserted that the shipped install path "degrades silently, which
  `installer-jq-silent-degradation` owns", and again that the slug "still owns it" — text
  *added by this same iteration*, weeks-fresh, and falsified by the same iteration's later
  commits (`bd8ef299`, `97b65bdb`, `047c7426`) that landed the unit and moved the slug to
  `## Done`. A reader arrives at a settled question dressed as an open one.
  **DISTINCT from `dead-queue-citation-report`**, and deliberately not folded into it: that
  entry's corpus is the queue's own bodies and its subject is a slug resolving *nowhere*. This
  one's corpus is governed SPEC prose and its subject is a slug resolving in `## Done` — live,
  findable, and closed. Different scan, different reader, different remedy.
  **Why `[design-pending]`:** the tempting form is a red, and a red is wrong for the same
  reason it is wrong for its sibling — governed prose legitimately names landed work, and
  telling a historical citation from a present-tense ownership claim means reading tense. The
  cheaper true form is that sibling's own conclusion: a **listing**, every governed-prose
  citation of a Done-resolving slug, reported rather than redded. Where the listing is read is
  the same open question, and taking either entry should cost both.
  **This narrows a class the audit roster carries as un-gateable.** `capability-pendency-after-
  landing` is un-gateable because a scanner cannot infer which tree set discharges a prose
  claim. The slug sub-case is the exception: the discharging set is the queue's own `## Done`,
  which is machine-readable, so this is the gateable slice of an ungateable class.
  Class: mints a name for a report rather than a gate; the promoting scope settles feature-vs-debt.
  **Cost while deferred:** paid by every reader of a governed SPEC who takes a closed ruling
  for an open one, and the audit that catches it is a per-close human sweep with no oracle.
  Filed 2026-08-16 by close, from the two instances its own roster sweep found.

- **survey-engagement-residue-untracked** [design-pending] — nothing OBLIGES a survey or its lead
  to leave durable engagement residue, so the `survey-engagement` audit passes or fails on practice.
  **RE-FRAMED 2026-08-24 at close, on the operator ruling that widened the class, and the old
  framing is corrected rather than annotated because three of its grounds went false at once.**
  The entry used to claim the class "cannot be performed a stage later under this repo's
  lead-orchestrated posture", resting on the roster's then-reading that residue is "the survey's
  OWN reasoning" — relayed to the lead, landing in a `.tmp/` journal the boundary reset wipes, an
  evidence lifetime shorter than the audit's cadence. **Option (c) of three was ruled 2026-08-24
  by the operator:** durable, git-logged reasoning satisfies the class and a lead's ruling commit
  is admitted, so the class IS performable and was performed and stamped at that same close.
  **Two of the three candidate fixes need their costing corrected, and one loses its premise.**
  (1) Widening `bin/file-survey.sh` so a *refusal* files like a census was rejected on the ground
  that the survey record "is itself boundary-truncated, so it buys one iteration, not an audit
  trail" — **that ground is FALSE and was re-probed here:** `.workflow/survey-record.md` is
  TRACKED (`git ls-files` resolves it), so the boundary reset truncates the working copy while
  git history keeps every block. It is an audit trail already, which makes this the cheapest
  option rather than the disqualified one. (2) A scope-stage obligation to record each refusal's
  engaged ground in the entry it refuses is now ATTESTED PRACTICABLE rather than hypothetical —
  `execution-control-reach-and-turn-end-blocking`'s five holds each did exactly this, and the
  at-cap growth it was costed against was paid there by answering grounds. (3) Retiring the class
  as unauditable has **lost its premise** and is no longer a live option.
  **What survives the ruling, and it is the whole entry now:** (c) changed what COUNTS as residue,
  never whether residue is SYSTEMATICALLY captured. The iteration that earned the first stamp
  earned it by conduct — a lead that wrote its grounds down — and a lead that does not leaves the
  audit with nothing, with no gate, template line or tool to say so. The class is now auditable
  and still not *reliably* auditable, which is a different and smaller defect than the one filed.
  **A second-order cost the widening introduces**, recorded here because the roster row states the
  limit and this entry owns the fix: the class no longer distinguishes a survey that engaged
  nothing from a lead that engaged thoroughly, so a mechanism landed under (1) or (2) should
  attribute the residue to its author rather than only capture it.
  **Cost while deferred:** the stamp now rests on whichever session happened to write its
  reasoning down, so the audit reads cleanest exactly when a diligent lead masks an incurious
  survey — the failure it cannot see is the one it was built for.
  Filed 2026-08-16 by close, from the audit it could not discharge; re-framed 2026-08-24 at close
  on the operator ruling, with one rejected option's ground re-probed and overturned.

- **in-crate-module-coupling-derivation** [design-pending] — a ported gate's descriptor can omit
  the crate modules its own verdict depends on, and no gate says so.
  recurrence: in-crate-module-coupling-derivation 2026-08-19
  **THE CORPUS IS 51 OF 89, measured 2026-08-19 at build and re-derived at this drain**: 51 `.gate`
  descriptors carry a `couples=` naming no `native/` path and no `*.rs` glob, and every one of the
  51 has a like-named crate module — so the hook never re-runs them on the edit that moves their
  verdict. `check-manifest-count` is the worked case: trigger `*SPEC*.md,*README.md,CLAUDE.md`,
  rule in `native/src/gates/manifest_count.rs`. §The fourth budget batch records eight descriptors
  written against the tree's older reading; nothing owns the retro-fit and the count has grown with
  each ported cohort. The retro-fit is mechanical per descriptor — the member's own module plus the
  shared rule-carrying modules it reaches, stopping at the universal layers — but 51 hand edits
  will not stay correct, so the gate this entry is designing is what makes it durable.
  **Attested this iteration, twice, and neither instance was found by a gate.**
  `check-value-rollup-fresh`'s module calls `enforcement_map::measure()` and
  `footprint::measure()` in-process, but its `couples=` named none of the three `emit/` modules;
  the lead found it by reading, and the fixing session found a **fourth** module the lead had
  missed (`native/src/marker.rs`, used on both sides of the compare, so a `read_block` change
  moves the verdict). Its two sibling members had their descriptors updated at port time, so the
  omission was inconsistent with the pattern the same cohort established.
  **Why it is silent rather than loud:** the generated pre-commit hook derives its
  `staged_matches` trigger from `couples=`, so an under-declared descriptor means the gate never
  runs on the edit that broke its projection. It passes because it did not execute. Only a full
  battery reaches it, and `run-gates.sh --for <path>` targeting misses it too.
  **`check-reads-couples` does not reach this**, and the distinction is the design's crux: that
  gate covers **walk roots** — what a gate reads off the filesystem — and an in-process call to a
  sibling module changes no walk root at all. This is a *source* coupling, a different axis.
  **Why `[design-pending]`, though the derivation looks easy:** a `--deps` arm reporting each
  gate module's transitive in-crate dependencies is the obvious producer and has a precedent in
  `--reads` (§The non-gate arm), but transitive closure over a shared crate reaches `walk.rs`,
  `proc.rs` and every common helper, so a literal reading couples every gate to most of the
  crate and the trigger set stops discriminating. Where to cut — a declared boundary set, or
  first-party modules only, or the emit/marker layer alone — is the unit's real question.
  **Cost while deferred:** paid once per remaining member of the freshness family as its emitter
  lands in the crate beside it, and paid as a stale published projection rather than a red. The
  interim is prose: gate-sdk/SPEC.md §The non-gate arm now states the obligation.
  Class: mints a gate name if it lands as one, so canon-kit/SPEC.md's litmus makes it a
  **feature**; debt only as an assertion folded into an existing meta-gate.
  Filed 2026-08-16 by close, from the lesson the porting cohort generalized rather than from a
  fresh finding — the two instances are already fixed.

- **metric-dir-member-contract-unheld** [design-pending] — `DRIFT_KIT_METRIC_DIR` states a member
  contract and nothing holds it, so the persistent dir accretes whatever a session leaves there.
  drift-kit/SPEC.md §Layout and configuration says metric-dir members "are append-only trend logs
  that survive scratch wipes" — a contract, distinguishing it from `DRIFT_KIT_TMP_DIR` precisely
  by retention. The distinction is what makes the dir exempt from the scratch wipe, so a
  non-conforming member there has **no reclaim path at all**: it survives every boundary forever.
  **Found at this close's runtime-artifact lifecycle check, and it is a live instance rather than
  a shape.** This clone's `.metric/` carries three Python scripts beside its two trend logs. They
  are referenced by no tracked surface — a grep across `*.sh`, `*.md` and `*.json` finds each
  name zero times — and they predate this iteration, so they are prior sessions' analysis scratch
  written to the one gitignored directory that never gets swept.
  **Why `[design-pending]` rather than a gate:** the dir is per-clone and gitignored, so a gate
  reds on one operator's local state and never on anything a commit produced — the
  low-false-positive contract site-kit/SPEC.md §The monitor boundary rules on for a different
  subject. The candidate shapes are a shape assertion inside `drift-report.sh` (advisory, where
  the reader already is), a wipe-non-conforming-members arm on the meter, or ruling the contract
  advisory and saying so. Which one turns on whether the contract is a privacy rule or a
  housekeeping rule; the gitignore already carries the privacy half.
  **Cost while deferred:** unbounded accretion in the one directory with no reclaim path, paid
  as a dir a later reader cannot tell trend data from leftovers in.
  Class: mints no governed name unless it lands as a gate, so canon-kit's litmus makes it
  **debt** in two of the three shapes.
  Filed 2026-08-16 by close, from step 6's write-path/reclaim-path question; the instance was
  probed (grep for each name across the tracked tree) before it was called orphaned.

- **knob-shape-flip-undetected** [design-pending] — a crate reading a knob as an array cannot
  tell that its consumer has since redeclared it `declare -A`: the values arrive as
  `key=value` strings and pass.
  The reverse direction *is* caught — the map reader refuses an element with no `=` — so this
  is the one open half of the keyed arm, named by gate-sdk/SPEC.md §Porting a gate to the
  binary substrate as the residue the arm leaves rather than discovered later. That section
  owns the grounds and is cited here, never restated.
  **Re-verified at this drain rather than taken on the filing's word.** Both crate readers were
  read at HEAD: the array reader tab-splits and asserts no shape, the map reader errors naming
  the offending element. Every associative knob in the tree — `LIFECYCLE_KIT_PREDECESSOR`,
  `EVIDENCE_KIT_SCENARIO_GLOBS`, `QUEUE_KIT_LESSON_SINKS` — is read by key wherever a crate
  reads it, so the hazard is a **future** flip with no live instance today. The filing's
  mechanism claim held; only its silence about liveness needed correcting.
  **Why `[design-pending]`:** closing it at the wire means transporting the reader's expected
  shape back to the producer, the maintained declaration the derived-shape rule deliberately
  declined to mint. So the candidate close is the *auditor* shape instead — assert that no
  knob a crate reads as an array is declared associative in its owning kit's lib. That is
  `check-gate-substrate-parity`-shaped work, and whether it lands as a further assertion there
  or as its own member is the open call this entry cannot make from the wire alone.
  **Cost while deferred:** silent, and it presents as a gate reading plausible-looking garbage
  rather than as a refusal — the same failure shape the keyed arm was written to end. Bounded
  by needing a consumer to change a shipped knob's *grammar*, itself a kit-SPEC-governed
  contract change rather than a configuration edit.
  Class: lands as a gate assertion, so canon-kit's litmus makes it **debt**.
  Filed 2026-08-16 at spec from the amendment's own residue section; drained and promoted
  2026-08-17 by close.

- **kit-bin-entry-point-unrostered** [design-pending] — no surface says which kit owns which
  `bin/` entry point, so a session guesses the path and buys an out-of-band decision per guess.
  **Measured at this close's triage, off the log rather than from impression.** Five prompting
  calls this iteration invoked a kit-bin path that does not exist, across three sessions:
  `canon-kit/bin/md-section.sh` and `gate-sdk/bin/section.sh` (three calls) for the extractor
  context-kit actually owns, `lifecycle-kit/bin/drain-stage.sh` for the gating read that
  `enter-stage.sh --simulate` performs, and a `scripts/checks/` path for a shell gate this
  iteration's own port had already deleted. The correct `context-kit/bin/md-section.sh` appears
  three times in the same log, so the *script* is known and its *owning kit* is what gets guessed.
  **What this entry is NOT.** Not every invented form at the triage. `run-gates.sh <gate-name>`
  was also tried and is a habit rather than a hole: the positional argument is a gates-dir and
  the path-scoped selector is owned in prose by gate-sdk/SPEC.md §run-gates. Only the cross-kit
  ownership fact is genuinely unowned — each kit SPEC documents its own `bin/` members and
  nothing spans them.
  **Three candidate closes, disagreeing on tier.**
  (a) A generated roster projection, freshness-gated like every other — derivation-first, since
  one listing derives it; costs a projection and its whole staling fan-out for a rarely-read fact.
  (b) A convention on an always-loaded surface telling a session to list before invoking — free
  to build and a permanent per-session tax, the shape the brevity machinery exists to reject.
  (c) A `bash-guard.sh` steer: an invocation naming a kit-bin path that does not exist gets the
  real owner back, found by basename. Enforcement-first — it fires at the moment of the guess,
  costs no resident bytes, and the guard is already wired into every session.
  **Why `[design-pending]`:** (c) dominates on cost but has the guard answering a *discovery*
  question rather than steering a form, which is a widening of what that instrument is for, and
  that is the operator's call rather than this entry's.
  **Cost while deferred:** about one out-of-band decision per guess, five this iteration, paid by
  whichever session reaches for a kit tool it has not used recently — and the guesses land as
  silent no-ops where the session falls back to grep instead of noticing.
  Class: mints no governed name and adds no gate in arms (b) and (c), so canon-kit's litmus makes
  it **debt**.
  Filed 2026-08-17 by close from its own tooling-friction and knowledge-friction triages; each of
  the five paths was probed for existence before the count was asserted.

- **kit-spec-consumer-config-literal** [design-pending] — nothing stops a kit SPEC from spelling
  out a value that belongs to a consumer's config, so the seam leaks by worked example.
  The provenance seam (CLAUDE.md) forbids a kit literal carrying consumer content, and
  de-literalization forbids prose owning a value. Both were crossed by one sentence this
  iteration: gate-sdk/SPEC.md illustrated the keyed wire by enumerating this repo's live
  `LIFECYCLE_KIT_PREDECESSOR` pairs, so a *vendoring* consumer read another tree's stage graph
  asserted as the shape of their own. Fixed in place at this close; the class is what is filed.
  **Why nothing caught it.** `check-tree-terms` is a banned-pattern leak guard and the leaked
  string is not a banned term; no other gate reads a kit SPEC against a consumer config at all.
  Found instead by the close-stage `internal-identifier-restatement` audit, which is rostered
  un-gateable for a *different* reason (public contract names are legitimate citations) — that
  reason does not reach this narrower shape, which is why the gap is filed rather than absorbed.
  **Why it looks buildable, and where the design is owed.** The two tiers are already separate
  files: a kit's own default lives in `<kit>/lib/*.sh` and a consumer's override in the
  consumer's config dir, so a kit SPEC quoting a value that appears only in the *override* tier
  is mechanically decidable. What is owed is the false-positive boundary — a kit SPEC that
  documents its own default legitimately, and a consumer whose override happens to equal it,
  are the same two strings — plus whether the subject is any value or only a multi-element
  roster, the shape actually found here.
  **Cost while deferred:** one leaked example per authoring session that reaches for a live
  value to illustrate a wire format, each one shipping a consumer's configuration inside a kit
  and going stale against it silently.
  Class: lands as a gate, so canon-kit's litmus makes it a **feature**.
  Filed 2026-08-17 by close from its own audit-roster review; the instance was fixed at this
  close and the absent-gate claim probed against `scripts/gates.list` before it was asserted.

- **queue-recovery-pickaxe-wrong-oracle** [design-pending] — every surface that tells a reader how
  to recover an evicted queue body names `git log -S`, which is blind to exactly the eviction it
  documents.
  **Measured, not reasoned, at this close.** `-S` fires only when a literal's occurrence *count*
  changes, and an eviction that leaves the slug behind changes none — a `## Done` move to a bare
  slug line, an icebox one-liner, or any body that spelled its own slug once. Two probes: this
  iteration's own Done move (`4bea9ceb`) leaves the count at 5 before and 5 after, so `-S` does
  not list the evicting commit at all and its newest hit is an unrelated earlier commit; and on a
  real icebox eviction, `-S'scratch-execution-allowlist-bar'` returns 3 commits where `-G`
  returns 5. `-G` matches diff content and reaches both.
  **Four surfaces carry the wrong spelling**, which is why this is one unit rather than a typo:
  this file's `## Icebox` preamble, `queue-entry-evidence-tier`'s body, queue-kit/SPEC.md §The
  icebox tier, and `check-queue-entry-budget`'s own help text — that last one is a compiled
  subcommand now, so the fix costs a crate edit and a rebuild rather than a string swap.
  **It does not re-open `queue-entry-evidence-tier`'s narrowing, and that is deliberate.** That
  entry ruled recovery solved and signalling the gap, on a 2026-08-02 measurement of the
  *compression* case — a shrinking body whose slug count does drop, where `-S` genuinely worked.
  The case that fails is *eviction*. The narrowing stands; only the oracle spelling is wrong.
  **Re-derived independently twice inside one iteration**, which is the cost showing rather than
  an argument for it: scope hit it while counting recurrence-stamping commits (the record notes
  `-S` undercounted and `-G` surfaced nine more), and this close hit it again re-verifying a gap
  bullet whose own recovery command was the broken one. Neither derivation had a doc to read.
  **Not started here, per Enforcement-first:** the fix and the gate that catches it land in one
  unit, and the gate half looks cheap — a literal-pattern check over the governed doc set for a
  `-S` prescribed as a body-recovery recipe. What is owed is whether that gate earns its slot
  against a four-line prose fix, which is the design call this entry holds.
  **Cost while deferred:** a session sent to recover an evicted body runs the documented command,
  gets a short list that does not contain the evicting commit, and reads the absence as the
  content never having existed — a silent wrong answer, not a visible failure.
  Class: mints a governed name and lands a gate, so canon-kit's litmus makes it a **feature**.
  Filed 2026-08-17 by close, from re-verifying a gap-inbox bullet; both probes were run before
  the claim was asserted and the compression-vs-eviction split checked against the entry it
  would otherwise have contradicted.

- **crate-toolchain-grant-uncommitted** [design-pending] — one clone's overlay grants a hand-run
  `cargo` path the sanctioned path never uses, so what is uncommitted is a widening nobody needs.
  **The premise as filed was backwards, corrected 2026-08-20 against a fresh probe.** It read "a
  session in a fresh clone meets a mandatory toolchain with nothing granting it". It does not: both
  sanctioned paths are granted in the committed `settings.json` — `run-gates.sh` at lines 11-13 and
  `build-native.sh` at 32-33. `check-crate-arms` *does* shell out to `cargo` directly
  (`native/src/gates/crate_arms.rs`; the member was a shell check at `gate-sdk/checks/…:40,47`
  when this was probed and ported at `shell-gate-tail-port`, the spawn surviving unchanged), and
  that settles the question **toward** the
  correction rather than against it, because the allowlist matches the top-level `Bash` call and
  not its subprocesses. Verified behaviourally rather than reasoned: that gate is registered at
  `scripts/gates.list:14`, so every `bash gate-sdk/bin/run-gates.sh` run this session executed it,
  and not one prompted for `cargo`.
  **What is true instead:** the committed file carries **107** `Bash(` entries and no `cargo` in
  any form — the 115 first filed was a dated count — while `.claude/settings.local.json`
  auto-allows `cargo build`, `cargo test` and `cargo clippy`, grants that reach only the
  unsanctioned hand-run path, the obligation being discharged through the two wrappers.
  **RULED — PRUNE THE OVERLAY AND GRANT NOTHING NEW; operator 2026-08-20, relayed by the lead.**
  A pruning, not a widening, and the deliverable shrinks to match.
  **The design question is ANSWERED by that ruling**, and it is why this was never a one-line
  settings edit: the wrapper grant is the correct form, it already exists committed, and the direct
  `cargo` entries are the habit to steer away from. Pruning is inside a session's own authority —
  guard-kit's triage contract (guard-kit/templates/close-triage.md) lets a session prune the
  overlay and forbids only widening its own auto-allow set.
  **Cost while deferred:** small, and no longer clone-shaped — the corrected premise removes the
  fresh-clone harm outright, both sanctioned paths being committed-granted. What remains is that
  one clone's overlay silently permits a hand-run path the project does not sanction: a standing
  grant nobody reviews, rather than friction anyone feels.
  Class: mints no governed name and lands no gate, so canon-kit's litmus makes it **debt**.
  Filed 2026-08-17 by close from its own tooling-friction triage; both allowlist sets were read
  before the absence was asserted, and the overlay was confirmed carrying zero redundant and
  zero over-broad entries by `compare-settings-allow.sh` at the same triage.

- **recurrence-obligation-residency** [design-pending] — the recurrence stamp became an
  obligation on **every** session this iteration, and its statement still reaches only the two
  stages that already loaded it.
  `recurrence-drain-input-widening` ruled the direct stamp sanctioned *and obliged*, attaching
  the duty to the judgment rather than to the channel, and ruled prospective-only at this close.
  So a build, align or validate session that observes a recurrence now owes a stamp. A live grep
  over the surfaces those sessions actually load — CLAUDE.md, doctrine-kit/DOCTRINE.md,
  context-kit's session-context templates, every stage template, delegation-kit's templates and
  the agent definitions — finds the word only in `close.md` and `scope.md`, and scope's is the
  *reader* half (the pre-emption threshold counting dates), not the duty to stamp one.
  **The remedy is already specified, which is why the gap is filed rather than argued.** This is
  delegation-kit/SPEC.md §Operative residency's exact shape and its condition (a) holds: the
  bound actor fires no trigger that loads lifecycle-kit/SPEC.md §The committed gap inbox, and a
  trigger that exists but that the actor never fires is, for that actor, no trigger. So a bounded
  imperative plus an adjacent citation is sanctioned — (b) and (c) bound what may be copied.
  **Attested from history, not predicted.** This iteration's spec survey counted the stamping
  commits and found direct stamps already produced by `chore(build)` twice, `chore(align)` once
  and `chore(scope)` once — the stages the original ruling never contemplated as producers. The
  writes are being made by sessions that never read the rule, which is the residency gap
  measured rather than theorised.
  **Second attestation, first-person, from this close.** The closing session made two direct
  stamps and reached the obligation only by reading §The committed gap inbox for an unrelated
  reason; neither its stage skill's dispatch nor any resident surface named the duty.
  **Why `[design-pending]` — the placement is the open half and it is a real trade.** The
  candidates differ in kind: one line in CLAUDE.md buys every session at a standing per-session
  cost on the tier the brevity machinery guards; a clause in each of the four unserved stage
  templates costs nothing resident but multiplies the restatement by four and drifts four ways;
  a clause in the dispatch-side agent definition serves only dispatched sessions and misses a
  standalone run. Widest-true-tier placement decides it and no reading is obviously right.
  **Cost while deferred:** the obligation is unenforceable and silently unmet for four of six
  stages — the failure mode is a recurrence *seen and not recorded*, which is the exact failure
  the counter exists to end, so the ruling's own purpose is what erodes.
  Class: relocates one imperative and adds no gate, so canon-kit's litmus makes it **debt**.
  Filed 2026-08-17 by close from its own brevity-and-residency pass; the loaded-surface corpus
  was grepped before the absence was asserted and the history attestation read off the survey
  record rather than inferred.

- **stale-identifier-after-retirement** [design-pending] — governed prose citing a deleted path
  whose capability moved intact to a new holder: a class the close-stage audit roster does not
  name.
  Found as a five-instance out-of-class residue — two in queue-kit/SPEC.md and three in live
  deferred entries, all naming `bin/queue-index.sh` after the port deleted it. **The instances
  were fixed; the class was not.**
  **Why both rostered siblings read past it.** It is neither
  capability-pendency-after-landing — nothing is claimed outstanding — nor
  capability-liveness-after-descope — nothing was descoped. Both are tense-and-inference classes
  over a *claim*; here the capability survived and only its holder changed, so the citation is
  stale in its **subject** rather than in its claim, and no roster line covers that axis.
  **What makes it different in kind: it is PARTLY GATEABLE.** A backticked path in governed
  prose either exists on disk or does not, and deciding that infers no intent — the exact ground
  both existing classes are rostered un-gateable on. The class does not merely widen the roster;
  it moves an instance out of the human-audit tier.
  **Three readings, and the choice is real — each costed here rather than left to the unit.**
  Widen `prose-filename-citation-liveness`, the decisive neighbour, which owns the same dangling
  -backtick shape but is bounded to bare `<name>.md` filenames and states its own reason for
  staying narrow (its `AGENTS.md` false positive); or mint a new audit class; or rule it a
  `check-docs-cmd` widening — and that third arm is dearer than it looks, because assertion A
  scans **invocation position only** and calls that its deliberate calibration, so the widening
  reopens a stated design decision rather than extending a corpus.
  **Attested cost, not estimated:** commit `19098b08` swept for the qualified
  `bin/queue-index.sh` spelling and missed every bare-basename one, so a hand sweep has already
  failed at this once.
  **Cost while deferred:** every retirement that relocates a capability leaves citations no audit
  line claims, and the failure is silent — the prose reads correct and names a path that is gone.
  Class: mints a gate name and a roster line on the mint-a-class arm, so canon-kit's litmus makes
  it a **feature** there; debt only as a widening of a line already carried.
  Filed 2026-08-17 into the gap inbox by the `post-close-intake-and-index-port` close, from its
  capability-pendency audit; promoted 2026-08-17 at scope, the disposition landing one iteration
  after the finding.

- **queue-lib-dead-derivation** [design-pending] — three derivations in `queue-kit/lib/queue.sh`
  outlived the shell tool that read them, and their only surviving reader is a gate-test.
  **Re-verified at this scope rather than carried from the bullet, and the premise holds at
  HEAD.** `QUEUE_ACTIVE_RE`, `QUEUE_DEFERRED_RE` and `QUEUE_ICEBOX_RE` (lib/queue.sh:71-87) are
  read by nothing but their own definitions and `queue-kit/gate-tests/queue-lib-parity.test.sh`,
  the shell tool that read them having been deleted by the queue-index port. Their siblings
  `QUEUE_TASK_RE` and `QUEUE_SECTION_RE` **do** still have live readers —
  `bin/queue-counts.sh`, `bin/queue-edges.sh`, and `queue_live_slugs` in
  the library itself — so this is a **partial** death and a blanket section deletion is wrong.
  **queue-kit/SPEC.md already rules the three internal**, which is what makes the deletion look
  safe: §The queue-index arm says the derived regexes "were never a configuration surface, only
  that library's internal spelling of these knobs".
  **What is NOT settled, and it is the whole entry:** whether the parity test's reads are
  load-bearing coverage of a live contract or the last consumers of a dead one. That is
  `gate-test-in-tree-invoker-ruling`'s predicate asked about a derivation instead of an arm — one
  unruled question wearing two hats — and this entry deliberately does not pre-empt it.
  **Cost while deferred:** three dead derivations and a parity arm carried against the port's own
  dependency-floor objective, and every reader who wonders re-runs the same grep to learn they
  are dead.
  Class: deletes internal names and mints none, so canon-kit's litmus makes it **debt**.
  Filed 2026-08-17 by the `post-close-intake-and-index-port` close into the gap inbox, found by
  its internal-identifier-restatement audit; promoted 2026-08-17 at scope with the reader set
  re-probed at HEAD before the claim was carried forward.

- **gate-test-in-tree-invoker-ruling** [design-pending] — nothing rules whether a gate-test counts
  as an in-tree invoker, and two live governed claims rest on the answer.
  gate-sdk/SPEC.md §The non-gate arm calls the queue-index arm's extent mode the class's worked
  instance of a mode whose only caller is a session. `queue-kit/gate-tests/queue-index.test.sh`
  invokes it, landed in the very commit that authored the prose, off a survey premise recorded at
  scope before the test existed. The sentence was corrected at the last close to say nothing in
  tree invokes it **but its own fixture** — true without deciding the general question, and
  deliberately so.
  **The corpus does not answer it.** That section's own named-caller set is a regen command, a
  comparator calling `emit()`, a stage step, and a gate reaching it in process; it lists no test.
  **Why the answer reaches past one sentence.** §The queue-index arm's refusal to ship a
  queue-mutating tool rests on the caller being a session, so a fixture counting as a caller needs
  that refusal's ground restated too. And `queue-lib-dead-derivation` is this same predicate on a
  derivation rather than an arm, which is why the two are filed as a pair rather than folded.
  **Cost while deferred:** two governed claims stand on an unruled predicate, and every session
  meeting a fixture-only caller re-derives the same undecided question — the close that found it
  escalated rather than picked, for exactly this reason.
  Class: rules a reading of prose already shipped and mints no name, so canon-kit's litmus makes
  it **debt**; a feature only if the ruling lands a new declaration.
  Filed 2026-08-17 into the gap inbox by the `post-close-intake-and-index-port` close, which
  escalated rather than picked; promoted 2026-08-17 at scope.

- **spec-embedded-source-criterion-4-membership** [design-pending] — whether a diff-reference
  corpus counts as "scanned as content" for criterion 4 is unsettled by the two sections that
  would rule it.
  gate-sdk/SPEC.md's conservation table rules `check-spec-embedded-source` **"survives
  unchanged — reverse trigger of the same shape"**, reading its `couples=` extension roster as
  a language list "not a reference to gate declarations" whose "scanned corpus is the canonical
  specs and amendments". The implementation disagrees in shape:
  the member builds its candidate set by walking every `*.sh`/`*.rs`/… file in the tree and
  diffs each file's actual line content against every spec's fenced blocks, so every gate
  declaration path is inside that set and its content is **read**, not merely triggered on.
  **The sixth budget batch ported the member and did NOT rule this class** — it took the
  binding verdict for that member alone, on the ground that a conservative verdict costs a
  fixture widening and cannot be wrong in the harmful direction (gate-sdk/SPEC.md §The sixth
  budget batch). This entry still owns the class, and that disposition is not its answer.
  **Why [design-pending]:** criterion 4 (gate-sdk/SPEC.md §The port-candidate criteria) binds
  where "a registry member's declaration path lies inside the corpus the gate scans as
  content", and distinguishes a reverse-trigger couple, never read as content, from a content
  couple. A **diff-reference** corpus — opened and content-compared, but not the gate's own
  assertion target — is neither, and neither section's text as written resolves which it joins.
  The verdict decides whether this gate **binds** criterion 4 the way `check-tree-terms` does —
  a fixture widening priced into the port, never a hold, per that section's 2026-08-22 ruling —
  so it is a port-sizing call rather than a wording one.
  **Cost while deferred:** low and bounded — nothing in this iteration's batch rested on the
  verdict, which is why it was filed rather than picked by precedent. The carry is that the
  next cohort cut re-opens the same question at selection time, where a wrong answer buys a
  mis-sized port rather than a wrong one.
  Surfaced 2026-08-17 in the spec stage's survey record, flagged unsettled rather than
  adjudicated there, and escalated at align on spec-over-precedent.
  Filed 2026-08-18 by close from the gap inbox, whose cited SPEC line number the drain
  re-verified — the row moved this iteration, content unchanged.

- **threshold-recurrence-routing-residency** [design-pending] — where the threshold-recurrence
  routing clause lives, now that its only carrier has left the live tree.
  **The clause, carried here verbatim so it does not spend by attrition:** *"a third threshold
  recurrence routes to the operator, not to a third decline; two is where lead discretion
  ends."*
  **OPERATOR-RULED 2026-08-17: file for scope to decide a permanent home**, rather than leave it
  in history or move it to TRAJECTORY.md now. It was exercised once in
  `port-selector-permanence-and-batch` and ruled LIVE AND UNSPENT at the promotion relay, so
  its disappearance from the live tree would spend it by attrition — the outcome that ruling
  refused.
  **ANSWERED 2026-08-19 at scope: the clause is GENERAL, not entry-specific** — a
  threshold-recurrence routing rule every recurrence-carrying entry inherits, not prose belonging
  to the entry whose subject (guard-kit rule 14) is resolved. Ruled by the iteration lead as a
  routing call rather than a fresh envelope one: the 2026-08-17 operator ruling delegated the home
  decision to scope, so recording the answer here **discharges** that delegation. Grounds: the
  clause completes the scope contract's own threshold paragraph, which already puts a
  threshold collision "in front of the authority this stage already escalates to" and stops short
  of saying where that authority changes. Exercised again at this very scope, which is the
  evidence rather than the argument: `stage-stamp-ordering-unenforced` stands at three
  recurrences and its last two declines were the operator's, not a lead's.
  **What is NOT decided here, deliberately.** The clause is not landed in lifecycle-kit's scope
  contract by this stage or this iteration; that stays a feature-shaped unit for a later one, and
  this entry stays its carrier meanwhile.
  **Two homes were probed and refused at build**, and the refusals are not rejections of the
  options — recorded that way so a later reader does not mistake one for the other. A live queue
  entry was blocked at that build because the candidate host stood at zero lines of headroom under
  `check-queue-entry-budget`; **that host has since closed and left the live tree**, so the option
  is now absent rather than full — a stronger refusal on a different ground, corrected 2026-08-24
  at close rather than left reading as a cap problem a later reflow could solve.
  TRAJECTORY.md is refused by CLAUDE.md's own scoping sentence, which admits
  **closed** operator rulings while this one is explicitly open — choosing it means amending
  that sentence, a governance edit rather than a move. lifecycle-kit's scope contract was
  refused as envelope-class: that was a **build session correctly declining an envelope call it
  had no authority to make**, and with the general/entry-specific question now answered it is the
  live candidate home rather than a closed one.
  **DISTINCT from `waiting-rule-fourth-firing-post-fix`**, which is Done: that entry owned the
  residency rule's enforcement and got it; this owns where its escalation-routing clause lives,
  which the Done move is what puts at risk.
  **Cost while deferred:** low while this entry exists and unbounded without it — the entry IS
  the carrier, so deferring the *home* decision costs only that a possibly-general rule reads
  as one parked entry's prose; not filing at all would have cost the clause to git history.
  Filed 2026-08-18 by close from the gap inbox on the 2026-08-17 operator ruling; the drain
  re-verified that no permanent surface carries the clause — the only hits were the inbox this
  drain truncates and the survey record the next first-stage entry truncates.

- **lead-dispatch-simulate-optionality** [design-pending] — the lead's dispatch contract makes
  `--simulate` optional, so the liveness gate wired into it runs only when the lead chooses to
  pay for it.
  **This entry is what survived a corrected premise, and the correction is the point.** It was
  filed as "detection costs a lead a hand-run gate it must remember to run", off the FOURTH
  ATTESTED FIRING of the turn-end residency rule (2026-08-17, the first after this iteration
  shipped its enforcement): validate ended its turn to wait on a background loop, the harness
  fired a completion notification reading "finished" while the producer was still writing the
  evidence manifest, and a lead taking that at face value would have dispatched close over a
  half-written manifest.
  **The machine held, and the drain established more than the bullet claimed.**
  `lifecycle-kit/bin/enter-stage.sh` runs `LIFECYCLE_KIT_ENTRY_PREFLIGHT` under `--simulate`
  too, and `scripts/lifecycle-config.sh` wires `check-producer-liveness` against the scratch
  directory at **every** stage — widened this iteration. So the bullet's candidate, that the
  lead's dispatch path run the liveness gate before dispatching stage N+1, is **already
  satisfied whenever the lead simulates**, and lifecycle-kit/templates/lead.md already states
  that qualification in prose.
  **What actually survives is one word.** That template says the lead "dispatches and trusts
  `enter-stage.sh`'s fail-closed refusal, **or** gates an expensive dispatch cheaply first with
  `--simulate`". A lead taking the first branch dispatches over a live producer; the stage
  session's own entry then refuses, one dispatch later.
  **Why [design-pending]:** making `--simulate` mandatory is an envelope change to a kit
  template binding every consumer, and it trades a cheap always-run probe against that rule's
  own stated reason for the optional branch — the lead must not re-derive what the machinery
  rules on. Whether an always-run `--simulate` is re-derivation or is exactly the machinery is
  the call.
  **Cost while deferred:** low, and now honestly low. The harm is a wasted dispatch caught one
  stage later, not lost evidence — both chokepoints that covered the fourth firing, guard rule
  14 and the widened preflight, stay live.
  Filed 2026-08-18 by close from the gap inbox; the drain probed the `--simulate` preflight
  path and the consumer wiring, which is what narrowed the entry from the bullet's shape to
  this one.

- **stage-cursor-rerun-stamp-gap** [design-pending] — a stage re-run that skips its stamp leaves
  the cursor naming an earlier stage, and nothing reds.
  **Observed in `port-selector-permanence-and-batch` with the battery green.** The iteration's
  last stamp before close was a `build` stamp, but the re-validate that followed the repair
  round-trip committed a full 24-suite evidence manifest without stamping — the state file's
  history shows validate's earlier stamp and no second one, so the cursor pointed backwards
  across two commits of validate work.
  **The two readings of "the previous stage" diverge here.** CLAUDE.md declares the cursor "has
  exactly one source, the last stamp"; `check-stage-entry`'s assertion A tests that the
  mandatory predecessor's stamp **exists for this iteration**, never that it is the last one.
  Close was therefore admitted from a `build` cursor — correctly by the gate, wrongly by the
  doc.
  **The tree's own practice is that a re-run stamps** — build stamped three times this
  iteration, once per batch — so this is a discipline the machine does not hold rather than a
  convention nobody follows.
  **The missing check class is nameable and buildable**, which is why this files as a task and
  not a note: for each commit in the iteration, the last stamp at that commit must name the
  stage its `chore(<stage>):` subject declares. Both halves already exist —
  `check-commit-subject` parses the subject and the state file is the cursor — so the
  deliverable is a comparator over the two, not new substrate.
  **Why [design-pending]:** the honest fix may be the doc rather than the gate. Either the
  cursor sentence narrows to "the stamp set", or a re-run owes a stamp; that ruling is what
  decides whether the comparator above is enforcement or ceremony.
  **DISTINCT from `stage-stamp-ordering-unenforced`**, and explicitly not judged a recurrence of
  it: that entry's subject is a stamp landing **after** commits already made under it. Here the
  stamp precedes its own run correctly, and it is a **later run of the same stage** that leaves
  no mark at all.
  **Cost while deferred:** low and quiet. Nothing was lost this iteration — the manifest is
  correct and validate is green — but the cursor is what a lead and `--simulate` both read, so
  a wrong cursor buys a wrong dispatch, and the failure is silent by construction.
  Filed 2026-08-18 by close, generated by this session's own entry read rather than drawn from
  the gap inbox, and dispositioned in the drain that was already open rather than routed into
  an inbox this same close is about to truncate.

- **lead-specifies-constraint-not-mechanism** [design-pending] — whether the lead contract should
  say that a lead states the constraint and a stage session finds the mechanism.
  **The claim, which is what a later scope rules on:** a supervision layer that specifies
  *mechanism* spends its sessions' verification discipline against its own unverified guess. The
  asymmetry is already recorded at lifecycle-kit/templates/lead.md — the lead writes no state and
  so has no verification discipline, while a stage session is held to oracle-first, fixture pairs
  and a validate battery. A lead that hands down a fix converts a verified actor into a typist,
  which spends the one asymmetry the split posture exists to exploit.
  **Two earlier worked instances, both in `port-selector-permanence-and-batch` and both the
  lead's own account.** Validate inverted the lead's diagnosis of the `upgrade` red — the defect
  was in the check itself, in a hand-held allowlist older than the change the lead suspected;
  build then rejected the derived form the lead gestured at and deleted the roster outright. In
  both, the lead's contribution was the constraint and the session's was the mechanism.
  recurrence: lead-specifies-constraint-not-mechanism 2026-08-23
  **THIRD WORKED INSTANCE 2026-08-23, and the first where the over-specified mechanism was
  DESTRUCTIVE rather than merely wrong** — again the lead's own same-turn self-report, which is
  what makes it citable. Relaying three operator rulings at this iteration's scope, the lead
  attached a mechanism: "re-enter scope as a sibling session (the cursor is still on scope; a
  same-stage re-entry stamps its own row and moves nothing)". The constraint was right, the
  mechanism was not. Probed rather than argued: `enter-stage.sh --simulate scope` exits 1, since
  scope IS the iteration boundary, so a second scope entry is a NEW-ITERATION entry refused for
  want of a release-disposition line naming the closing iteration. Forced, it would have truncated
  `.workflow/WORKFLOW-STATE.txt` to its header and reset the queue header to the unnamed sentinel,
  destroying the state the instruction existed to build on.
  **IT SHARPENS THE HONESTY BOUNDARY BELOW RATHER THAN ONLY ADDING TO IT.** The cost was NOT
  self-limiting: what was avoided was a state-file truncation, and what avoided it was the
  receiving session running `--simulate` first — the scope template's own do-not-force-the-entry
  rule, a STAGE-side control and not a lead-side one. So "recoverable" holds only where the
  receiving stage happens to carry a refusal for that exact act; **where it does not there is no
  backstop at all**, which is the generalizable form.
  **Why [design-pending], and it is the whole reason this is filed rather than landed:**
  lifecycle-kit/templates/lead.md is a kit template binding every consumer, so a rule added there
  is an envelope change. Close refused to make it alone and the lead adopted the refusal rather
  than substituting its own call. What scope owes is whether the claim generalizes past this
  repo's posture at all, and if so whether it belongs beside the relay-never-assert rule it is
  the mirror of, or is too soft to sit in a contract.
  `relayed-rule-role-scope-unchecked` and `dispatch-claim-evidentiary-tier-unmarked` want the same
  template on the same envelope-change reasoning; a scope ruling any of the three should rule all.
  **THE HONESTY BOUNDARY IS PART OF THE ENTRY, not a caveat on it.** Two instances is an
  **anecdote, not a measurement**, and **nothing reds on a lead over-specifying** — the rule is
  unenforceable by construction, since a dispatch never enters the tracked tree. Both instances
  were caught because the sessions were bounded well, not because any mechanism fired. A scope
  reading this must not take the rule as established.
  **Cost while deferred:** low in frequency and **no longer self-limiting in the worst case** —
  three instances in three iterations, and the third would have cost an iteration's lifecycle
  state rather than a turn. The posture is already the lead contract's implicit shape, so what is
  missing is the explicit statement, not the practice; the carry is that each lead re-derives it.
  A longer narrative version is staged as operator material in the essay-harvest sink; the two
  are different tiers and both stand. Filed 2026-08-18 by close on the lead's ruling, which
  adopted close's own refusal to take the envelope change alone.

- **guard-steer-names-absent-tool** [design-pending] — a guard refusal steers the session onto
  a harness tool its own toolset does not carry, so the remedy it names is unreachable.
  **PROBED at the 2026-08-18 close and re-probed at this scope entry.** `guard-kit/lib/guard.sh`
  emits two such steers: the bare-`find` refusal says "use the Glob tool" and the `git grep`
  refusal says "use the Grep tool". A dispatched stage session's toolset is
  Agent/Artifact/Bash/Edit/Read/Skill/ToolSearch/Write plus the deferred set, and
  `ToolSearch 'select:Grep,Glob'` matches nothing — neither tool exists to reach. The `cat` and
  `sed` steers name the Read tool and are correct, which is why the class reads as ordinary
  friction rather than as a defect.
  **The bullet undercounted the surface, and the correction is what widens this.** It named
  `scripts/bash-guard.sh` and the Grep steer alone; the text lives in **guard-kit's generic
  ruleset** and Glob has the identical shape. So the reach is every vendoring consumer whose
  agent shape lacks those tools, not this repo's own steer vocabulary.
  **NOT a permission-coverage defect and NOT an allowlist widening.** Bare `grep` and `find`
  are already committed grants, so the working form was available all along; only the named
  remedy is wrong.
  **DISTINCT from `guard-steer-grant-mismatch`**, which is a steer whose target form the
  allowlist does not grant. Here the grant exists and the *tool* does not — the same surface's
  opposite half, worth designing together, neither subsuming the other.
  **Why `[design-pending]`:** the guard cannot see a caller's toolset, so a conditional message
  has no input to condition on. The candidate shapes are naming the fallback in the text ("use
  the Grep tool, or bare `grep`, which is allowlisted") or making the steered-to tool a
  consumer-config line, and which one holds is a guard-kit steering-message contract question
  rather than an edit.
  **Cost while deferred:** small and paid per search — a session follows the steer, finds no
  tool, spends a ToolSearch, and re-derives the allowlisted bare form.
  Surfaced 2026-08-18 at the `port-selector-permanence-and-batch` close, in its tooling-friction
  triage; promoted from the gap inbox at this iteration's scope.

- **deferred-entry-defer-date-unasserted** [design-pending] — a deferred entry whose provenance
  date is malformed reads as **undated**, and no gate reds on it.
  recurrence: deferred-entry-defer-date-unasserted 2026-08-24
  **FIRST RECURRENCE 2026-08-24, in a SHAPE no prior instance covers: the spelling was CANONICAL
  and a LINE WRAP broke it.** At this close a new deferred entry ended a body line on the word
  `Filed` with `2026-08-24` beginning the next, and the arm listed it `(undated)`. Probed the same
  way this entry's founding measurement was: moving the marker and the date onto one line dropped
  the row from the worklist on re-measure. The reading that widens the class — the parse is
  LINE-SCOPED, so the defect is reachable by ordinary reflow and not only by careless spelling,
  which means any session that rewraps an entry can silently create one.
  **PROBED both ways at the 2026-08-18 close, on entries that close itself wrote.** Two of its
  seven new deferred entries spelled the date `filed 2026-08-18 by close` (lowercase, mid-line)
  and `Filed at build 2026-08-18` (a word between the marker and the date). Both were listed by
  `run-gates.sh --emit queue-index --icebox-candidates` with defer-date `(undated)`; rewriting
  the two lines to the canonical form dropped both from the worklist on re-measure.
  **The failure is silent by construction.** Such an entry is well-formed to every other gate —
  the cost field is present, the budget gate is clean, the battery is green.
  **Cost while deferred:** an undated entry never ages out of the age filter, so it is a
  permanent icebox candidate *and* is invisible to drift-kit's deferred-age KPI, both readers of
  the one definition (queue-kit/SPEC.md §The queue format).
  **Why the fix looks cheap, and what the design still owes.** Every top-level deferred entry
  resolves a defer date, and `check-queue-entry-budget` already walks every one of them to
  enforce the cost field — so this reads as one more assertion on an existing walk, the shape of
  its own assertion (C): a required field whose absence is invisible. What is open is the
  **holder set**: that same SPEC section names three re-implementations of the definition
  (queue-kit's, drift-kit's KPI, gate-sdk's `check-gate-exemption-tasks`), so an assertion on one
  leaves the others parsing the same malformed line their own way.
  Class: an assertion inside a shipped gate mints no name and is **debt** on that path; a new
  gate or knob would make it a feature, and the promoting scope call settles it.
  Surfaced 2026-08-18 at the `port-selector-permanence-and-batch` close, at its backlog-eviction
  step; promoted from the gap inbox at this iteration's scope.


- **survey-oracle-liveness-unasserted** [design-pending] — a survey record's `oracle:` field can
  name a boundary-wiped path, and the record's own gate accepts it.
  **The instance, self-caught at this iteration's scope.** `bin/file-survey.sh` accepted
  `.tmp/verify-cluster.sh` as the oracle and printed it back as "the witness a later stage runs",
  but `.tmp/` is wiped by `enter-stage.sh`'s boundary reset (CLAUDE.md §Housekeeping), so the
  witness was dead before any later stage could run it — and dead inside this iteration for any
  stage after a re-entry.
  **Why it defeats the record rather than merely annoying it.** lifecycle-kit/SPEC.md §The survey
  record rests on the consuming stage re-running the oracle, so an unrunnable oracle converts a
  carried survey into an uncheckable assertion — the exact failure the record was minted against.
  **Re-verified at the 2026-08-18 drain, with one correction to the filed premise.**
  `check-survey-record` does parse the field and asserts non-emptiness only
  (`native/src/gates/survey_record.rs:150`), so the gap is real. The bullet said
  `check-scratch-citation`'s reach "defaults to the queue file alone"; that is the **kit** default
  (`lifecycle-kit/lib/stages.sh:62`), while this repo configures `("TASK-QUEUE.md" "*/SPEC.md")`
  (`scripts/lifecycle-config.sh:26`). The record sits outside either, so the conclusion survives.
  **Two candidate shapes, neither ruled:** add `.workflow/survey-record.md` to
  `LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS`, reusing `check-scratch-citation`'s existing
  `lifecycle_supersede_set` derivation whole; or assert the oracle path inside
  `check-survey-record`, which already parses the block. The first is cheaper and drags the
  record's whole prose under the no-retrieval-pointer rule; the second is targeted and leaves the
  prose alone. Choosing between them is the unit's first deliverable.
  **Distinct from `scratch-citation-skill-surface-reach`**, whose subject is which *permanent*
  surfaces the rule reaches. This is one committed surface whose field is machine-read and
  boundary-fragile; recurrence declined at the drain on those grounds.
  **Cost while deferred:** silent and delayed — a dead oracle presents as a survey that cannot be
  witnessed, at the moment a later stage is already relying on it.
  Surfaced 2026-08-18. Filed 2026-08-18 by close, draining the gap inbox.

- **lead-held-block-no-sanctioned-surface** [design-pending] — a lead-held block is invisible to
  the queue and its gates, so the next session picks a latently-blocked entry as first-unblocked.
  **The instance, and it did not bite.** This iteration the lead held two units blocked on an
  operator decision about a permissions file, recorded the block in dispatch prompts and a
  boundary-wiped `.tmp/` journal, and left both active entries carrying no blocked state. The
  operator ruled before the block could cost anything, which is why it is filed rather than
  forgotten.
  **All three routes are closed, and the build session probed this rather than reasoning it.**
  Stating the precondition in entry prose reds `check-queue-prose-precondition`, whose message
  names the failure exactly — an active entry with a prose precondition and no tag is latently
  blocked yet mechanically pickable. The `blocked-by` tag must name a real queue slug
  (queue-kit/SPEC.md §The tag algebra), which is a scope write and not a lead's. The
  `precondition-ok` opt-out asserts the opposite of the truth. Re-verified at the drain: the
  gate is registered at `scripts/gates.list:64`, the tag grammar at queue-kit/SPEC.md:264.
  **So the observed default is to hold the block in prompts** — which
  `lifecycle-kit/templates/lead.md` already forbids in principle (the message thread is transport,
  never a store) while offering no mechanism in practice.
  **Candidate shapes, none ruled:** a `blocked-by` target that may name an operator decision
  rather than a slug; a lead-writable block marker outside the queue that `check-stage-entry`
  reads; or a ruling that a lead may not hold a block at all and must escalate it to a scope write
  the moment one appears.
  **Distinct from `check-queue-prose-precondition`**, which closed one route and works as
  designed — the gap is the absence of an open route, not a defect in that gate. **Recurrence of
  `lead-state-durable-home` declined at the drain:** that entry's axis is durability (lead state
  dying with the conversation, fix shape a lead journal), and a lead journal would not fix this,
  because the session computing first-unblocked reads the *queue*. Adjacent, not the same finding.
  **Cost while deferred:** low frequency and silent, landing only when a lead session dies or
  hands off while holding a block — precisely when no one can recover the fact.
  Surfaced 2026-08-18. Filed 2026-08-18 by close, draining the gap inbox; a lead filing about the
  lead's own conduct, re-verified here.

- **self-repo-prefix-normalisation-unheld** [design-pending] — the origin-to-blob-prefix
  normalisation has two holders in the crate with no gate or test holding them equal.
  **The filed premise was falsified at the drain, in the direction that mattered.** The bullet
  said two holders; there were three — `native/src/emit/mod.rs:16` (pub), a byte-identical private
  copy at `native/src/emit/enforcement_map.rs:74` shadowing its own parent module's, and
  `native/src/gates/md_refs.rs:284` (private, `Result`-returning). The emit pair diffed identical
  apart from visibility and the `proc::run` path spelling, so it was a port leftover rather than a
  design fork: `mod.rs`'s own comment says the copy was hoisted "because two arms render self-repo
  links, and a second copy of the normalisation is a second identity to disagree about".
  **The leftover was fixed inline at this drain** — the `enforcement_map.rs` copy deleted and the
  parent's imported; `--emit enforcement-map` byte-matched before and after, so the dedup is
  behaviour-preserving. What remains is a design question rather than a sweep: `md_refs.rs`'s
  variant returns `Result<String, String>` where emit's returns `String` and degrades to empty.
  The signatures encode different fail postures — a gate wants the error, an emitter wants the
  degradation — so unifying them is a decision, not a deletion.
  **Deliverable:** either one holder with a fail posture both callers can take, or two holders
  with a gate or shared test asserting they agree on the same input. `Enforcement-first` says
  removing the duplication outranks gating it, so the one-holder shape is favoured and unproven.
  **Cost while deferred:** low and bounded — both copies are exercised, and a disagreement
  presents as a wrong link rather than a wrong verdict. It is on the list because the third copy
  appeared without anyone deciding to add one, which is the mechanism that produces a fourth.
  Surfaced 2026-08-18. Filed 2026-08-18 by close, draining the gap inbox; premise corrected here.

- **stage-completion-unattested** [design-pending] — the stage stamp marks entry, so a stage that
  is entered and abandoned is indistinguishable in the tree from one that finished.
  **Observed this iteration at validate, not hypothesised.** The session backgrounded
  `evidence-kit/bin/run-validate.sh`, reported a background wait armed on the PID, and its turn
  ended anyway — the harness fires a completion notification precisely when an agent stops with no
  live background children, so the arming did not hold the session open. What it left behind: the
  entry stamp committed, a clean tree, a green battery still running, and no evidence manifest,
  which is validate's actual deliverable.
  **Why it is a class and not a slip.** Every artifact-level check a later session could run —
  stamp present, tree clean, gates green — returns exactly what a finished validate returns. The
  cursor has one source, the last stamp, and the stamp is written at stage *entry*
  (`lifecycle-kit/bin/enter-stage.sh:339` appends it), so the cursor cannot separate
  entered-and-abandoned from entered-and-completed. Re-verified at the drain: no exit-stamp
  concept exists anywhere in lifecycle-kit. The only signal separating the two here was a
  completion notification held by the dispatching lead and by nothing durable.
  **Candidate shapes, none ruled:** an exit stamp distinct from the entry stamp, so cursor motion
  and stage completion stop sharing one mark; an assertion in `check-stage-evidence` that a
  stage's own deliverable exists before the next stage's entry stamp may be written; or a doctrine
  line that a stage session must not background a producer it is itself the consumer of.
  **Distinct from `stage-stamp-ordering-unenforced`**, whose axis is the stamp landing *after* the
  work it authorizes — there the deliverable exists and the mark is mistimed; here the mark is on
  time and the deliverable is absent. **That sibling LANDED in `graph-port-and-config-seam` and
  handed its remainder here**: an entry stamp bound to its own commit cannot reach a session that
  works, commits, then stamps, and with the per-stage surface roster refused there, the exit mark
  this entry would mint is the only declared boundary left. That case closes as a composition of
  the two entries, never as more work inside the sibling.
  **Distinct from `delegation-provenance-floor`**, which is a
  parent unable to attest a *child's* return; this is a session's own completion unobservable in
  the tree. Both recurrences declined at the drain on those grounds.
  **Cost while deferred:** silent, and it lands hardest when recovery is most expensive — a lost
  or compacted lead is exactly the case where the notification that would have caught it is gone.
  Surfaced 2026-08-18. Filed 2026-08-18 by close, draining the gap inbox; filed by the lead
  because the evidence is a notification the stage cannot observe about itself.

- **projection-trigger-witness** [design-pending] — the generated-projections roster states each
  projection's staleness trigger in prose, and nothing checks the prose against the emitter.
  **The missing check class, named because a staleness fix without one forfeits it.** This close
  found five false trigger statements in `docs/site-architecture.md` §Generated projections, all
  about `docs/footprint.md`: that a KPI script is new token cost, that a prose-only SPEC edit reds
  it (stated twice, false both times), that a new gate script is new token cost, and a
  staging-order hazard that in fact belongs to `check-gate-binary-fresh` alone. Each was settled
  by one cheap perturbation — edit a file of the claimed class, re-emit, diff — and each had been
  wrong long enough to be copied into a second row.
  **Deliverable:** a differential witness. Per roster row, perturb one member of the class the row
  says stales it and assert the projection's bytes move; and in the negative direction, perturb a
  class the row excludes and assert they do not. This is a property test over the emitters rather
  than a prose scanner, which is what makes it buildable where a claim-parity gate is not.
  **Why `[design-pending]`:** the row-to-class binding is the open part. The roster names its
  triggers in English, so the witness needs a machine-readable trigger declaration per row, and
  adding one is a docs-surface change carrying its own freshness question.
  **Adjacent to `gate-spec-claim-assertion-parity`**, iceboxed as a human-audit class: that asks
  whether a SPEC's prose claim matches its gate in general. This is one bounded family with a
  mechanical oracle — re-emit and diff — which is why it is deferred rather than iceboxed.
  **Cost while deferred:** measured at five, in one roster, found only because a port made one of
  them conspicuous. A wrong trigger costs either a regen nobody runs or a hunt for a red the named
  command cannot clear. The negative direction is the expensive half to build and caught four of
  the five errors here.
  Surfaced 2026-08-18. Filed 2026-08-18 by close, as the gap generalization owed by the
  `docs/site-architecture.md` staleness fixed in the same commit.

- **cited-script-path-liveness-inline** [design-pending] — `check-docs-cmd` sees a deleted `.sh`
  path only inside a fence, so the same path in an inline code span survives a port unflagged.
  **Measured at this close, seven instances in one iteration.** The emitter-tail port deleted
  `drift-kit/bin/trajectory.sh` and `queue-kit/bin/roadmap.sh`, and the battery stayed green at
  104/104 while seven governed surfaces still named them in the present tense: an `Interface:`
  line in drift-kit/SPEC.md, a queue-kit/README.md intro sentence, a `.gate` descriptor's
  `# spec:` line, three gate-sdk/SPEC.md paragraphs and a `# spec:` comment in
  gate-sdk/lib/gate.sh. All seven were inline code spans or bare prose; all seven are fixed in
  the commit that files this.
  **The gate is not absent, its corpus is narrower than its name reads.** Its own module states
  the reach — "every **fenced** invoked repo-relative .sh path and every backticked/fenced
  kit-prefixed env knob" (`native/src/gates/docs_cmd.rs:1`) — so an inline span is scanned for
  knobs and not for paths. gate-sdk/SPEC.md already predicts the fenced half working: it will
  "correctly — not vacuously — red on a doc still fencing a deleted `.sh` path after a port."
  It does. The unfenced half is the hole.
  **Why `[design-pending]`, and the design is the valve rather than the scan.** The fence *is*
  the current exemption: the gate's own help says a hypothetical example goes outside a fence.
  Widening to inline spans removes that valve and needs a replacement, and the replacement has
  to admit the case this close met repeatedly — prose that names a deleted path **deliberately**,
  as history. Two shapes, neither ruled: a per-line exemption tag, or a tense-blind rule paired
  with a convention that historical mentions cite the commit rather than the path.
  **Not the same as the audit class that caught these.** `capability-liveness-after-descope` is
  a human audit on the roster because judging live-vs-historical prose is a session act; this is
  the mechanical half underneath it — does the cited path exist at all — which is decidable and
  today only half-scanned. Landing it narrows what that audit must read; it does not retire it.
  **Cost while deferred:** measured, recurring, and it lands exactly when the tree is most
  trusted — a green 104/104 battery over prose that names files the same commit deleted.
  Surfaced 2026-08-18. Filed by close 2026-08-18, discharging the gap generalization owed by
  the staleness the `capability-liveness-after-descope` audit turned up in this same commit.

- **shipped-bin-removal-deprecation-path** [design-pending] — deleting a kit-shipped `bin/` tool
  needs no deprecation marker, so that arm of the major-bump criterion is unreachable by design.
  **The instance, probed rather than assumed.** The `freshness-cohort-roadmap-hold-and-batch`
  iteration deleted `drift-kit/bin/trajectory.sh` and `queue-kit/bin/roadmap.sh`. Both shipped:
  `scripts/pack-installer.sh` recursively copies each enumerated kit root into the payload, `bin/`
  included, so a consumer who scripted a direct invocation now gets file-not-found. And
  `CANON_KIT_DEPRECATION_MARKERS` defaults empty in `canon-kit/lib/spec.sh`, so no marker ever
  rode either script and none could.
  **Why the criterion cannot see it.** docs/install.md §Versioning defines a major as removing a
  DEPRECATED surface, or a change the two-phase upgrade contract cannot reconcile from the release
  note alone. Neither half fires. The first presupposes a marker that never existed; the second is
  satisfied because phase A replaces kit directories wholesale, so the deletion propagates with no
  consumer action, and the residual breakage is a consumer's OWN script calling the removed path,
  which no phase-B gate scans. The bump was a correct minor — the defect is that the release-sweep
  constraint that no marker rides into the next major undispositioned is anchored to a roster that
  has always been empty in this tree.
  **The gap is the missing path, not the bump.** Nothing obliges a session deleting a shipped
  `bin/` tool to mint a deprecation marker for it, so the roster stays empty by construction.
  **Three candidate shapes, none ruled, and the choice is envelope-class.** A gate asserting that
  a path deleted under a kit root's `bin/` was marker-covered in a prior release; a widening of
  the major criterion to name shipped-surface removal directly; or an explicit ruling that a
  `bin/` tool is not a declared surface and its removal rides a minor forever, which would at
  least make today's behaviour intentional rather than accidental.
  **Distinct from `cited-script-path-liveness-inline`**, which shares the instance and not the
  axis: that entry is about governed prose still naming a deleted path, a staleness question
  inside this tree; this one is about what an ADOPTER is owed when a shipped path disappears.
  **Cost while deferred:** silent and consumer-side. It lands on an adopter who automated around
  a kit tool, and it lands as a broken script rather than a red gate — the failure class the
  two-phase upgrade contract exists to convert into a worklist.
  Surfaced 2026-08-18 in the gap inbox by `freshness-cohort-roadmap-hold-and-batch`'s close,
  whose release-disposition step postdates the drain; promoted 2026-08-18 at scope.

- **prose-uniqueness-claim-unchecked** [design-pending] — no check reaches a prose UNIQUENESS
  claim over a governed roster: a superlative selecting a predicate-defined subset of it.
  **The instance, found 2026-08-19 at close's staleness read and fixed inline at `efd74265`.**
  docs/site-architecture.md §Generated projections asserted the gate binary was the sole rostered
  projection carrying a staging-order hazard. The generated pre-commit hook carries such a hazard
  by another route: `check-prose-enum`'s enum-set emitter derives its `*-gate-test` members with
  `git ls-files` (`scripts/enum-sets.sh`), so an untracked new `gate-tests/*.test.sh` sibling
  passes a whole-tree battery and reds the hook at commit time on `git add`.
  **Why the nearest gates do not reach it.** `check-manifest-count` refuses a bare cardinal
  quantifying a governed collection noun; `check-prose-enum` holds an enumeration against a
  derived set. Both read claims about the *members*. "The only X that Y" is a claim about the
  **complement** — every member of the roster the predicate does not select — and neither gate
  has any reading of the complement at all.
  **Why `[design-pending]`: whether a scanner is buildable is the open question.** The selecting
  predicate is prose ("carries a staging-order hazard"), so the derivable half may be only the
  roster the claim quantifies over. If that is right the shape is a **marked-claim** class in
  `check-measured-claim`'s lineage — the author declares the roster, the check holds the
  superlative against it — rather than a scan that finds the claims itself.
  **Cost while deferred:** a uniqueness claim is the shape a reader most reasonably stops reading
  at, so a false one silently narrows the next session's search; and it goes false by the roster
  growing, which is the one event nobody re-reads the prose for.
  Filed 2026-08-19 into the gap inbox by the `budget-batch-and-account-identity-kind` close, whose
  staleness read hit the instance; promoted at the following scope's drain of that inbox.

- **guard-read-steer-tool-coverage** [design-pending] — the bash-guard's read-steer covers `cat`
  and `sed` and not `awk`, so a line-range read of a tracked file is decided out of band.
  recurrence: guard-read-steer-tool-coverage 2026-08-23
  **FIRST RECURRENCE, 2026-08-23, and the DOMINANT SHAPE FLIPPED — which redirects the design.**
  22 `awk` programs in that log: **8** line-range, **12** section-pattern (`/^## <title>/,/^## /`),
  2 genuine stream transforms, against 19-of-22 line-range below. So the majority then wanted a
  DIFFERENT target — `context-kit/bin/`'s section extractor, already committed-allowlisted — and a
  steer built to the earlier measurement would have sent it to `Read`'s offset/limit, the wrong
  tool. What the unit owns grows by one: the parser must also decide WHICH steer to emit.
  **THIRD MEASUREMENT, 2026-08-23 at `battery-runner-port`'s close — the dominant shape flipped
  BACK, so the mix is ITERATION-SHAPED rather than trending.** Of the 25 file-reading `awk` calls
  in that log, **23** are the line-range form and 2 the section-pattern one, with about 5 further
  genuine stream transforms beside them — so `Read`'s offset/limit is the majority's right target
  again and the section extractor is the minority answer. Read the three together — 19-of-22
  line-range on 2026-08-19, 8-of-20 earlier on 2026-08-23, 23-of-25 now: a queue-and-survey-heavy
  iteration reads sections and a SPEC-and-source-heavy one reads line ranges. That argues for the
  which-steer clause above more strongly than any single reading argues for one target, and it
  proposes no new deliverable. **No `recurrence:` date joins:** the finding did not re-fire, a
  measurement the entry carries was superseded.
  **FOURTH MEASUREMENT 2026-08-24, and it CONFIRMS the iteration-shaped reading rather than
  superseding it: 72 line-range `NR>=` against 22 section-pattern, in a SPEC-and-source-heavy
  iteration exactly as the hypothesis predicts. No `recurrence:` date joins, on this entry's own
  third-measurement precedent — a further measurement of a carried quantity is not a firing.**
  **Measured 2026-08-19 off the log:** 22 `awk` calls, 19 the exact `awk 'NR>=X && NR<=Y' <file>`
  shape, and every one of the 22 read a FILE rather than transformed a stream — which is the whole
  premise, and it is the shape the guard already steers `cat` and `sed` away from.
  **It CORRECTED a proposal `session-mechanic-grants-uncommitted` carried**, and that entry has
  since moved to Done, so the contest has no live holder and this entry is its only survivor. That
  one listed `awk` among the absent grants to put to the operator, its disposition (a); on this
  measurement the right disposition is (b), the steer. Granting it would bless the form the tree is
  retiring — the masking `guard-kit/templates/close-triage.md`'s criterion warns of.
  **A third disposition, added 2026-08-23 at close:** (c) rule the steer spelling-shaped by design
  and say so in the rule's own honest-limit paragraph, so a later triage stops re-deriving this.
  It is the cheapest of the three and the only one that costs no parser; it is also the only one
  that leaves the out-of-band decisions standing, which is what the choice trades.
  **Distinct from both steer-defect neighbours**, re-read here rather than assumed:
  `guard-steer-grant-mismatch` is a steer whose target form nothing grants;
  `guard-steer-names-absent-tool` is a steer naming a tool that is not there. This is a read form
  with **no steer at all**.
  **Why `[design-pending]`, and why it is not a one-liner.** The `sed` steer is a dedicated
  segment parser (`guard-kit/lib/guard.sh`, `_guard_sed_segment`) separating the script argument
  from file operands through `sed`'s own option grammar. `awk`'s grammar differs — `-F`, `-v`,
  `-f`, and the first non-option word is the program unless `-f` is given — so it needs its own
  parser plus a `guard-read-path.test.sh` arm. What the unit owns is whether a third per-tool
  parser is the right shape, or whether the three want one "first-word-is-the-program,
  rest-are-operands" abstraction.
  **Cost while deferred:** an out-of-band decision per range read, on the busiest read shape in
  the tree, while the guard reads as though it covers the class.
  Filed 2026-08-19 into the gap inbox by the `budget-batch-and-account-identity-kind` close's
  prompt-friction triage; promoted at the following scope's drain, both neighbours re-read there.

- **deferred-entry-time-deixis-rot** [design-pending] — relative time-deixis rots in the deferred
  pool and no gate reaches it, because a deferred entry outlives its filing iteration by design.
  **Measured 2026-08-19 at close's staleness read.** 165 lines of `TASK-QUEUE.md` matched
  `this iteration|this batch|last iteration|next iteration|this close|this session`, and every one
  of them silently re-points each time the queue header moves. Re-run at the following scope's
  drain the same grep returns 152 — the iteration boundary cleared the active sections, not the
  pool — so the count is a live oracle rather than a number this entry holds.
  **The live instance that found it, fixed inline; the class is not.** A deferred entry read
  "VACUOUS UNTIL THIS BATCH ... before this iteration ported `check-commit-msg` and
  `check-gate-fail-closed`", filed 2026-08-18 by validate about the iteration then closing — and
  one iteration later it asserted the vacuity ended with a different batch's ports instead. That
  reading tells the next reader a baselined red is a FRESH regression rather than a known one.
  165 lines is a unit and not a triage, which is why the class is filed rather than swept.
  **`check-manifest-temporal` is the nearest gate and reaches neither half.** Its corpus is the
  `spec_manifest_files` finder — canonical SPECs, `README.md` at any depth, `CLAUDE.md` — which
  excludes `TASK-QUEUE.md`; its markers are narration markers (*previously*, *formerly*) rather
  than deixis.
  **Two candidate shapes, trading against different things.** Widen that gate's corpus and marker
  set, which collides with the queue being a work record where "this iteration" is meaningful AT
  FILING TIME. Or rule that a deferred entry names its iteration and hold it with a queue-side
  scan — narrower, and matching the widening already ruled for
  `deferred-pool-identifier-restatement-sweep` over the same pool and for the same rot-window
  reason.
  **DISTINCT from `installer-init-noop-regen-conflict`**, which owns an installer no-op defect and
  is only the entry this class was caught inside.
  **Cost while deferred:** every promotion, drain and audit reading the pool prices a dated claim
  against the wrong iteration, and the error is invisible — the sentence stays grammatical and
  stays plausible.
  Filed 2026-08-19 into the gap inbox by the `budget-batch-and-account-identity-kind` close;
  promoted at the following scope's drain, the matching grep re-run there before the claim moved.

- **iteration-scoping-clause-date-ambiguity** [design-pending] — a clause scoping a ruling to one
  iteration by DATE cannot name it, because this tree opens several iterations a day.
  **The instance, measured rather than argued.** `native-gate-port-remaining-corpus` carries "the
  PRIORITY DIRECTIVE yields ... for the 2026-08-22 iteration alone". Two iterations scoped on
  2026-08-22 — `substrate-claim-and-update-target-coverage` and
  `hold-ground-oracle-and-takeable-holds` — probed at this drain against the stamp HISTORY, since
  `.workflow/WORKFLOW-STATE.txt` is boundary-truncated and only `git show` reaches the earlier one.
  It cost the 2026-08-22 scope one escalation question and will cost the next reader one.
  **The same defect hits the `HELD at the <date> promotion` form**, two live instances, both dated
  2026-08-22 and neither able to say which of that day's two promotions it records.
  **Deliverable: a clause scoping a ruling to one iteration names the ITERATION, not its date.**
  The iteration name is unique by construction — it is the queue header and the state file's first
  field — where the date is not.
  **Do NOT re-scope the yield ruling itself — operator-class.** Rewriting that clause to name an
  iteration would DECIDE which one it meant, which is substance and not spelling, and is why this
  drain promoted rather than fixed.
  **DISTINCT from `deferred-entry-time-deixis-rot`** (relative deixis — "this iteration" — silently
  re-pointing as the header moves) **and from `deferred-entry-defer-date-unasserted`** (a
  provenance date malformed or absent). Both are a date stale or missing; this is a date that is
  present, correct and still ambiguous. The adjacency worth composing against: deixis-rot's own
  second candidate shape is "rule that a deferred entry names its iteration", the same REMEDY for a
  different defect, so a scope promoting either should price them together.
  **Why `[design-pending]`:** whether this is gateable is open. A scanner for `the <date> iteration`
  and `at the <date> promotion` is cheap, but which clause forms fall in the class is a ruling the
  strings do not carry, and the corpus is the same pool deixis-rot names.
  **Cost while deferred:** every one-iteration scoping clause written from here on is ambiguous the
  day a second iteration opens, and the ambiguity is invisible — the sentence stays grammatical, the
  date stays correct, and the reader who misresolves it has no tell.
  Filed 2026-08-22 into the gap inbox by scope; promoted at this iteration's close drain, the
  two-scope-stamps claim re-verified against git history before the promotion.

- **spec-lib-dead-derivation** [design-pending] — three section-builder regexes in
  `canon-kit/lib/spec.sh` have no reader left in the tree, and nothing rules what they are.
  **Derived rather than inferred, 2026-08-19 at close's capability-pendency audit.**
  `SPEC_FEATURE_RE`, `SPEC_ACTIVE_RE` and `SPEC_DEFERRED_RE` are matched by nothing but their own
  definition lines: a grep for the three names across every `*.sh` in the tree returns those lines
  and nothing else.
  **It surfaced from a prose claim that had gone false, and the prose is already corrected.**
  canon-kit/SPEC.md asserted `check-amendment-queue` "still reads `SPEC_ACTIVE_RE` and its
  siblings here"; that gate ported to the crate, where the module classifies a section name
  against `CANON_KIT_ACTIVE_SECTIONS` directly and builds no regex at all. The residue is what is
  left uncorrected, and it is this entry.
  **The disposition is genuinely open, which is why this is a unit and not a deletion.**
  `canon-kit/lib/spec.sh` is a KIT library: a consumer's own shell gate may legitimately source
  these, so *unread in this tree* is not *unused*. Deleting them is a kit-surface removal with an
  upgrade-contract cost — a Renamed-knobs `old -> nothing` declaration under docs/install.md's
  grammar — rather than a cleanup.
  **Three shapes.** Delete them and declare the removal. Keep them and state in canon-kit/SPEC.md
  that they are consumer surface with no in-tree reader, which makes the absence a fact rather
  than a smell. Or hold them until the port's shell residue is dispositioned wholesale, since more
  of this library loses its last caller as the remaining members port.
  **The same class as `queue-lib-dead-derivation`, one library over, and deliberately not folded
  into it.** That entry's regexes retain a gate-test reader and queue-kit/SPEC.md already rules
  them internal, so its open question is whether a parity arm is live coverage. These have **no**
  reader of any kind and no ruling behind them, so the question is the opposite one: whether they
  are surface at all.
  **DISTINCT from `in-crate-module-coupling-derivation`** (descriptors under-declaring their
  couples) and from `native-gate-port-remaining-corpus` (what the port still owes, not what it
  leaves behind unread).
  **Cost while deferred:** the port keeps generating this residue at the rate it lands members,
  and every reader who wonders re-runs the same grep to learn the same thing.
  Filed into the gap inbox 2026-08-19 by the `budget-batch-and-account-identity-kind` close, at
  its capability-pendency audit; promoted at the following scope's drain, the grep re-run at HEAD
  there and the three names still matched by their own definition lines alone.

- **couples-glob-semantics-unowned** [design-pending] — one manifest field, three readers, two
  incompatible glob semantics, and no surface owns which reader is entitled to which.
  **Probed at all three sources, not read off the bullet.** `check-gate-substrate-parity`
  assertion C matches with a bash `[[ p == g ]]`, where `*` crosses `/` — verified by execution:
  `gate-sdk/*.sh` matches `gate-sdk/checks/check-x.sh`. `check-reads-couples`' own couple matcher
  requires **equal segment count** and never crosses `/`. `check-graph` assertion B reads the
  same field a **third** way — exact-token subset membership against `trigger=`, invoking no glob
  matcher at all — and the generated hook's staged-path test takes the slash-spanning form.
  **What the drain corrected in the filing's own claim.** The bullet said the divergence is
  stated nowhere; it is stated in **one** place, the compiled couple matcher's own comment, which
  names its narrowness as deliberate and cites the slash-spanning matcher it differs from. That
  narrows the gap without closing it: a comment inside one reader is not a contract any other
  reader or any descriptor author reads, and the third semantics is undocumented outright.
  **Why `[design-pending]`:** normalising is not obviously right. `check-reads-couples`' narrow
  matcher is correct **as specified** and its spec says so, so the call is whether `couples=` has
  one semantics with stated exceptions, or is a field whose meaning is per-reader and must
  therefore be declared per reader. Only the second is cheap; only the first is safe.
  **Cost while deferred:** it already bit once, at the sixth budget batch, which reproduced both
  forms deliberately rather than normalising them. The crate now carries a component-wise matcher
  and a slash-spanning one side by side, so a porting session reaching for "the" crate glob
  matcher flips a verdict on one side and no gate anywhere would say which side.
  **Half discharged 2026-08-21 at spec:** `SPEC-graph-port.md` states check-graph assertion B's
  four coverage branches in gate-sdk/SPEC.md and forbids the port reaching for either crate
  matcher, so the third semantics stops being undocumented and the port's own exposure closes.
  What remains is this entry's real question: one semantics with stated exceptions, or a
  per-reader meaning declared per reader.
  Filed 2026-08-19 by close from the gap inbox; the drain executed all three matchers rather
  than reading them.

- **prose-tell-threshold-validation** [design-pending] — `check-prose-tells`' numeric thresholds
  are read unvalidated on both substrates, so a typo turns a calibrated gate into a silent no-op
  or a wall of noise, confidently and with no diagnostic.
  **The count in the filing was wrong and the drain corrected it: five, not six.**
  `canon-kit/lib/spec.sh` defaults `CANON_KIT_PROSE_TELL_EMDASH_MAX`, `_CONTRAST_MAX`,
  `_RHYTHM_MIN_SENTENCES`, `_RHYTHM_CV_MIN` and `_TRICOLON_MAX` with a bare `[[ -v ]] ||` and
  validates none of them, while the same file's validator block checks fourteen other knobs for
  range and shape. `_GLOBS` is the sixth knob the bullet counted and it is an array, not a
  threshold — a different validation question.
  **The failure is silent in both directions.** The value is coerced by its leading numeric
  prefix, so a non-numeric max becomes zero and every paragraph reds, and a non-numeric minimum
  becomes zero and its assertion can never fire. The compiled form reproduces the coercion
  **deliberately** — a refusal the shell never made would be a verdict change across the seam.
  **Why `[design-pending]`:** the repair is one validation in `canon-kit/lib/spec.sh`, which is
  criterion 6's discharge-by-construction — one computation both substrates read. What is not
  settled is what a malformed threshold should *do*: refuse the gate at exit 2, matching every
  other knob in that validator, or fall back to the documented default and report. The first is
  consistent; the second is kinder to an adopter mid-edit.
  **Cost while deferred:** a consumer typo produces a confidently wrong verdict, and neither
  failure mode names its cause.
  Filed 2026-08-19 by close from the gap inbox, which carried it twice — once from the sixth
  batch's port survey and once from the port itself; the drain read the validator and counted.

- **template-copy-parity-knobless-refusal** [design-pending] — `check-template-copy-parity`
  refuses the whole gate at exit 2 on any paired file carrying no knob-with-default idiom.
  **The behavior, reproduced by execution at the drain rather than read.** When a paired file
  carries no knob-with-default token at all, the surface derivation reports *could not classify*
  rather than *no knobs*, and the fail-closed wrapper turns that into exit 2 for the whole gate,
  with a message naming an internal step rather than the file.
  **The tree is green by exclusion, not by correctness.** The two knob-less files in the corpus
  are dropped before the derivation runs — one by the `*-config.sh` rule, one by being unpaired.
  Vendoring an unpaired knob-less template into the gates dir turns every run into that refusal.
  **Reproduced rather than repaired at the sixth budget batch**, and the crate module says so in
  its own spec comment: a refusal the shell form never made is a verdict change across the seam,
  which the parity run holds invariant. Both substrates now carry it identically, proven by a
  differential run.
  **Why `[design-pending]` rather than a two-line fix:** the repair itself needs no ruling — a
  knob-less file has an **empty** knob class, not an unreadable one. What needs one is the seam:
  this is the first deliberate defect-reproduction the port has filed for later repair, so the
  unit has to establish how a repaired verdict lands on both substrates at once without either
  side briefly disagreeing.
  **Cost while deferred:** the first consumer to vendor a template with no defaulted-env read
  gets exit 2 on a file that is fine, and no gate anywhere would have predicted it.
  Filed 2026-08-19 by close from the gap inbox, which carried it twice — from the port survey and
  from the port; the drain reproduced the refusal rather than reading for it.

- **installer-uninstall-diff-stale-hash-coverage** [design-pending] — the reverse-to-pre-install
  property is asserted only where a stale carried hash cannot arise, so the class that produced
  `installer-init-noop-regen-conflict` would land again on a green suite.
  **The filing's premise was wrong and the drain corrected it before promoting.** The bullet
  said both smoke arms run `uninstall` against a fresh install. They do not: the install
  assertion ends with an idempotent **re-run** of `init`, and the reversal assertion runs on that
  same consumer immediately after, so `uninstall` already meets a twice-initialised tree — and
  the seam arm drives init, an edit, a re-run, `diff` and `uninstall` in sequence as well.
  **What survives the correction, narrowed to what is actually uncovered.** Every reversal runs
  after a **same-version** re-run, where the payload is byte-identical and the written set cannot
  change. The upgrade arm is the only place a re-run carries a *different* payload — a relinquish
  hop and a re-add hop — and it asserts no reversal at all. So the hop that can actually move a
  recorded hash is the one hop whose reversal nothing asserts.
  **Why `[design-pending]`:** whether to chain a reversal onto the upgrade arm's second hop or
  give it its own arm is a smoke-cost judgment, and that arm is already the most expensive in the
  suite.
  **Cost while deferred:** the property the whole adoption story rests on is asserted on the
  paths where it cannot fail and unasserted on the one where it can.
  Filed 2026-08-19 by close from the gap inbox, discharging the operator's refusal of the widen
  at `installer-init-noop-regen-conflict`; the drain read the smoke's call order and the bullet's
  central claim fell.

- **pipeline-membership-idiom-latent** [design-pending] — the SIGPIPE-under-pipefail membership
  idiom that produced `installer-init-noop-regen-conflict` has no gate, so nothing stops the next
  site being written.
  recurrence: pipeline-membership-idiom-latent 2026-08-23
  **The idiom.** A quiet `grep` reading an array printed into a pipe under `set -o pipefail`:
  `grep` exits on its first match while the writer is still writing, the writer takes SIGPIPE,
  and `pipefail` makes the pipeline's status the signal rather than `grep`'s zero — so a present
  member reads as **absent**.
  **The 2026-08-23 recurrence, and what it falsified.** This entry's 2026-08-19 drain recorded a
  sweep of every shell file returning "exactly two survivors", both latent. `battery-runner-port`
  found and fixed **five** sites — the then-shell `check-gate-substrate-parity` (3, at a
  declaration path `shell-gate-tail-port`'s delta 4 has since deleted),
  `gate-sdk/bin/upgrade-smoke.sh` (1), `gate-sdk/gate-tests/lib-gate.test.sh` (1) — so the sweep
  undercounted by three, and one of them was not latent at all: it produced a 1-in-3 red at that
  iteration's build once the worker pool raised the load. The rule is now stated at
  gate-sdk/SPEC.md §run-gates. Re-probed at this drain: a tree sweep for the
  `printf … | grep -q` shape returns nothing, so **zero sites remain and still no gate exists**.
  **What remains is now the whole deliverable.** The 2026-08-19 filing left one open call —
  whether the mechanical repair was the deliverable, or whether enforcement-first made it a gate
  over the idiom. `battery-runner-port` took the repair and could not take the gate inside its
  envelope, which settles the call by elimination: the gate is what is owed, and it is the only
  shape that stops a seventh site.
  **Why `[design-pending]`:** born-native per CLAUDE.md — a Rust module matching a
  producer-into-consumer pipe over an array/set membership idiom under `set -o pipefail`, a
  `.gate` descriptor, a `good/`+`bad/` fixture pair, and `gates.list` registration. The design is
  the predicate: separating this idiom from a deliberate early-exit pipe without flooding.
  **Cost while deferred:** a correctness cliff with no warning track, now demonstrated rather than
  reasoned. Nothing degrades gradually; a check reports a present member as absent on the run
  where its roster crosses the 64K pipe buffer (onset measured between 400 and 800 single-token
  members), and in a **parity** gate that false absence reads as a real parity finding rather than
  as a fault in the check. The port is what grows the subcommand roster, and the port is the
  tree's standing direction.
  Filed 2026-08-19 by close from the gap inbox; recurrence judged and stamped at
  `battery-runner-port`'s close, whose drain re-ran the sweep and found the corpus empty.

- **deferred-cost-class-opener-vocabulary** [design-pending] — the icebox cost filter reads the
  first word of a free-prose field, so it is undecidable for most of the pool.
  **Censused at this intake, 2026-08-23**, and recorded with its oracle at
  `.workflow/survey-record.md`: 238 `Cost while deferred:` fields, of which about 40 open with a
  token the `--icebox-candidates` arm recognizes — `low` 35, `zero` 5 — and the rest open with
  prose: `the` 44, `every` 35, `a` 32, `paid` 12, `one` 8, `an` 8, `each` 6, `silent` 5,
  `small` 4, `compounding` 4. queue-kit/SPEC.md names the recognized set (`low`, `zero`,
  `bounded`, `cosmetic`); roughly four fifths of the pool answers none of them.
  **The filter under-selects silently rather than mis-selecting**, which is why no gate ever
  reddened on it: an entry whose cost is genuinely low but whose field opens "the" is invisible
  to the eviction worklist forever, and nothing distinguishes that from a costly entry. So the
  2026-08-23 ruling lowering the age floor to seven days bought less than it looks — the age
  axis widened while the cost axis stayed shut over most of the rows.
  **Enforcement-first shape, unruled:** a controlled opener vocabulary gated over the
  `## Deferred` section, plus a one-time reclassification sweep of the existing fields. Class:
  the vocabulary is a governed name and the gate is a new contract, so canon-kit's litmus makes
  this a **feature** and an amendment is owed at promotion.
  **Why `[design-pending]`:** whether the class rides the field's head or a separate tag is
  open and the two trade differently. A head token keeps one line and makes every existing
  field a migration; a `cost-class:` tag leaves the prose untouched and spends a line on each
  entry against `check-queue-entry-budget`'s 50-line cap.
  **DISTINCT from `icebox-candidate-eligibility-unapplied`**, the same arm's other half: that
  entry is the *eligibility* rules the arm fails to apply — roadmap tags, live promotion
  triggers — and its fix filters rows the arm already produced. This is the *cost* predicate
  producing too few rows to filter. One surface, so they bundle well; neither subsumes the other.
  **Cost while deferred:** the eviction worklist derives from about a sixth of the pool, so the
  8:1 intake-to-exit ratio the 2026-08-23 ruling acted on has no lever reaching the rest.
  Filed 2026-08-23 by the consult held after `leak-guard-and-assertion-meta-gate-port` closed,
  and promoted out of the gap inbox at the following scope intake on the same date.

- **retired-slug-live-pointer-citation** [design-pending] — governed prose can point at a retired
  queue slug as if it were a live surface, and the gate that owns slug citations permits that by
  design rather than by omission.
  **The rule is stated in two halves and only one is enforced.** queue-kit/SPEC.md
  §check-queue-slug-liveness says prose about landed work must drop the bold-code form **and** cite
  the owning SPEC instead; the gate's token grammar is the bold-code form alone, so a plain-code
  slug is legal prose about landed work whether or not the sentence around it still points
  anywhere real.
  **One live instance, found 2026-08-19 at the generating close's staleness review and fixed in
  place.** TRAJECTORY.md's port-sequence directive said a ruling's grounds, its accepted cost and
  its discharging tranche "are the queue entry's", present tense, for an entry that had completed
  13 of 13 and retired several iterations earlier — and the same sentence's closing clause
  anticipated that retirement. That is what makes it the sharp case: the author saw the retirement
  coming, recorded the ruling in TRAJECTORY.md precisely so it would survive, and still left the
  pointer aimed at the vanishing surface. Re-pointed at gate-sdk/SPEC.md §The consumer remainder
  cohort, which is where TRAJECTORY.md reads today.
  **Probed rather than assumed:** a scan of every plain-code slug-shaped token across the
  top-level governed docs against the live slug set returned exactly one true positive, so the
  class is real and rare rather than a wave.
  **DISTINCT from `stale-identifier-after-retirement`**, which is a deleted *path* whose capability
  moved intact. This is a retired *slug* cited as a live pointer, and the two differ in what a
  reader can do about it: a dead path is checkable against disk, while a retired slug resolves to
  nothing anywhere, the Done section being cleared every iteration.
  **Why `[design-pending]`:** the mechanical half is a scan for a plain-code slug-shaped token that
  matches no live slug but does match a slug in the queue's own history — buildable, and it needs a
  false-positive budget, since gate names share the slug grammar and dominate the token
  population. Whether that budget is affordable, and whether the second half — does the sentence
  still point anywhere — is decidable at all, is the ruling this entry owes.
  Class: mints a gate name if the oracle lands, so canon-kit's new-names litmus makes it a
  **feature** on that path; debt only if it lands as a further assertion inside
  `check-queue-slug-liveness`.
  **Cost while deferred:** low frequency, silent failure, and it lands hardest on the ruling record
  — the one surface whose whole purpose is outliving the queue entries it was extracted from.
  Surfaced 2026-08-19 at the same close's staleness review, filed to the gap inbox beside the entry
  above; promoted 2026-08-20 by the next iteration's scope, which drained that inbox to its header.

- **queue-status-parenthetical-liveness** [design-pending] — a queue cross-citation's section-status
  parenthetical goes stale when the cited entry moves section, and nothing reads it.
  **The instance, found 2026-08-22 by two independent close audits.** The
  internal-identifier-restatement and capability-pendency sweeps reached
  `amendment-roster-omission-detection` by different heuristics and returned the same line —
  `amendment-update-target-coverage` tagged `(icebox)`, written while that entry was iceboxed and
  left behind when this iteration promoted, built and closed it. Fixed in place by dropping the
  status and naming the landed gate, which is what queue-kit/SPEC.md §check-queue-slug-liveness
  already prescribes for prose about landed work. Both sweeps proposed keeping a status tag; that
  would have been stale again within the hour, since the same close cleared the Done section.
  **Why this slice IS gateable where its neighbour is not.** `retired-slug-live-pointer-citation`
  needs a false-positive budget because gate names share the slug grammar and dominate the token
  population. This one needs none: the trigger is a closed vocabulary of section names in
  parentheses immediately after a backticked token, in the queue's own file, and the queue's own
  sections are the oracle. Resolution is single-file and total, with no cross-corpus reach.
  **DISTINCT from `retired-slug-live-pointer-citation`** above, whose subject is a slug cited as a
  live pointer after retirement. Here the slug is cited correctly as an instance and only the
  status beside it is false — the reader is told the work is unbuilt while the gate it names runs.
  **Why `[design-pending]`:** the open question is the vocabulary's edges rather than the scan. A
  second live citation — `enter-stage-simulate-no-write-fixture` tagged `(icebox)`, verified
  accurate at this same drain — uses the bare section name; whether a richer spelling should be
  admitted, rejected or normalized, and what a status tag on a NON-slug token means, decides
  whether this is one more assertion on `check-queue-slug-liveness` or a member of its own.
  **Cost while deferred:** low frequency and silent, but it INVERTS the reader's conclusion rather
  than blurring it — a stale `(icebox)` says the work is not done, which is the opposite of true —
  and the population is small enough that hand sweeps keep finding it one instance at a time.
  Filed 2026-08-22 by close, discharging its staleness review's gap-generalization obligation: the
  fix landed inline and the scanner that would have caught it did not, so it is filed not forfeited.

- **settings-hook-command-path-gate** [design-pending] — a hook registration in
  `.claude/settings.json` whose `command` names a renamed or deleted script reds nowhere and
  fails silently at run time.
  **Probed at the drain, not reasoned:** `check-settings-paths` resolves command tokens for
  `permissions.allow[]` only (`native/src/gates/settings_paths.rs`, whose `allow_entries` reads
  `/permissions/allow`); the sole other reader of `/hooks` in the tree is the **emitter**
  `native/src/emit/enforcement_map.rs`, which renders `PreToolUse` and `SessionStart` command
  paths into the enforcement map **without resolving them against the tree** — a deleted script
  still renders a row — and does not read `/hooks/SubagentStop` at all, so this iteration's new
  registration is invisible to the projection as well as to every gate.
  **Two halves, and the second is the cheaper one.** Path resolution is the walk
  `check-settings-paths` already owns, so widening its subject from one JSON pointer to two is a
  small port-side change; extending the enforcement map's hook-event roster is a docs-projection
  ruling about what belongs on that page, not a gate.
  **Why `[design-pending]`:** whether the widened subject stays inside `check-settings-paths` or
  mints a second gate name is canon-kit's new-names litmus, and the projection half is a scope
  call on the enforcement page that nobody has taken.
  **Cost while deferred:** a broken hook is invisible until the behaviour it guards silently stops
  happening — the failure mode with no red anywhere and no user-visible symptom.
  Filed 2026-08-22 at spec while surveying context-kit's settings gates; drained at that
  iteration's close, which re-verified the claim and found the enforcement-map reader it missed.

- **breadth-declaration-stale-listing** [design-pending] — a `GUARD_KIT_BREADTH_DECLARED` entry can
  outlive its subject with nothing noticing: the knob records a ruling about one glob and never
  verifies that glob is still in the local allowlist, still over-broad, or still real.
  **Probed at the drain:** the declaration lookup sits *inside* the local-allow-by-breadth-probe
  loop (`guard-kit/bin/compare-settings-allow.sh`), so a declaration keyed on an entry that has
  left the overlay is never looked up — it is not reported stale, it is not reported at all.
  **A stale-declaration *red* was already weighed and refused** inside the amendment that shipped
  the knob, because it would print every declaration naming a committed glob as stale. The cheap
  true form is a **listing** rather than a red — the conclusion
  `done-slug-ownership-citation-report` reached for its own class; the remedy shape is borrowed,
  the finding is not.
  **DISTINCT from `breadth-declaration-committed-glob-home`**, the same loop's other face: that
  entry asks where a *committed* glob's keep-ruling lives at all, this one asks what happens to a
  *local* declaration once its subject leaves. Settling that entry bounds this one's corpus.
  **Why `[design-pending]`:** whether the listing rides `compare-settings-allow`'s existing
  advisory output or earns its own arm is unsettled, and so is whether a listing nobody reads
  beats nothing at all.
  **Cost while deferred:** a spent ruling reads as live and silences nothing visibly.
  Filed 2026-08-22 at spec; drained at that iteration's close, which re-verified the loop scoping.

- **breadth-declaration-committed-glob-home** [design-pending] — a keep-the-breadth ruling about a
  **committed** allow glob has nowhere durable to live: `GUARD_KIT_BREADTH_DECLARED` cannot hold
  it, because `compare-settings-allow`'s breadth loop iterates the **local overlay** only, so a
  declaration keyed on a committed glob is a key nothing ever reads — the shape guard-kit/SPEC.md
  itself refuses when it says a field with no named reader is removed rather than shipped.
  **The concrete casualty:** `Bash(git rm -q *)` is a committed glob whose breadth build
  2026-08-22 ruled KEEP on real grounds — `guard_rule_rm_tracked` steers every tracked-file
  deletion into it, so narrowing it would tax a mandated mechanic — and that ruling evaporated.
  **NOT a re-filing of `settings-allow-intended-breadth-declaration`**, which shipped the knob and
  is closed: the knob does exactly what its own spec says. The gap is that `guard-grant-review`'s
  corpus (the committed set) and the knob's corpus (the overlay) were never the same corpus, which
  nothing noticed until a committed-glob keep-ruling needed a home.
  **Three dispositions, none ruled:** state in guard-kit/SPEC.md that the knob is
  local-overlay-scoped by design and give committed-glob keep-rulings a different home; widen the
  breadth question to the committed set, which the iteration lead has ruled a closed reading and
  would have to re-open; or accept that a committed glob's breadth is ruled by the fact that it is
  committed and reviewed, and record nothing.
  **Why `[design-pending]`:** the three differ in what they assert about the committed set's
  reviewability, which is a security-posture call and not a mechanism choice.
  **Cost while deferred:** a security review of the committed set produces keep-rulings that
  evaporate — the exact failure mode `guard-grant-review` was re-filed to stop.
  Filed 2026-08-22 by build; drained at that iteration's close, which re-verified the loop.

- **guard-rule-number-not-citable-outside-kit** [design-pending] — a guard-kit rule number is a
  stable-looking identifier that is not stable: rules renumber on every insertion, and the
  renumbering sweep's roster covers SPEC prose, lib comments, the test tables and the runner —
  **cross-corpus prose has never been in it**.
  **Measured, not assumed:** `ro-bins-write-option-bypass` cited the read-only-pipeline rule as
  "rule 15 ... rule 13 when this entry was filed" while it was in fact 17 — stale twice over, and
  already stale before the renumbering that moved rules 15-20 to 16-21. That one citation was
  corrected in place at close 2026-08-22 by naming the rule instead of numbering it; the durable
  rule is this entry's.
  **Two dispositions and neither is free:** sweep cross-corpus prose on every renumber, a corpus
  nobody has costed; or state in guard-kit/SPEC.md that a rule number is not a citable identifier
  outside the kit and have every cross-corpus reference name the rule — the way rule 20 already
  cites DOCTRINE.md by name rather than number, for exactly this reason.
  **Why `[design-pending]`:** the second is a one-paragraph boundary note plus a sweep of unknown
  size, and whether a bare "rule N" outside the kit is gateable at all needs a false-positive
  budget nobody has measured.
  **Cost while deferred:** a reader follows the number to the wrong rule and reasons from it.
  Filed 2026-08-22 at align's cross-audit; drained at that iteration's close, which found the
  bullet had named the wrong slug and located the real entry before dispositioning.

- **criterion-4-two-spellings-disagree** [design-pending] — one port criterion is stated in two
  spellings that give opposite verdicts on the same member.
  **Both sentences sit inside gate-sdk/SPEC.md §The port-candidate criteria, criterion 4.** One
  binds where "a registry member's declaration path lies inside the corpus the gate scans as
  content"; the other where the corpus reaches "the gate's own declaration path". Under the
  first, `check-gate-assertions` bound today, pre-port, in every configuration — eight of its
  nine live enumerated contracts already resolved their markers out of `native/src/gates/*.rs`.
  Under the second it cleared, because §check-gate-assertions carries no
  count-word-plus-labelled-span and so its own heading is filtered out.
  **The eighth cut settled the member and deliberately did not settle the criterion** — it took
  the conservative binding verdict and ended the member's contingent immunity by making it
  self-auditing; widening the spine past the ruled unit set was refused, and §The port-candidate
  criteria records the non-reconciliation in its own prose, citing this slug.
  **Why `[design-pending]`:** picking a spelling changes that section's own rule with reach
  across every registry member, so the deliverable is the reconciliation plus a re-read of every
  member's criterion-4 verdict under the chosen spelling — the re-read is the unsized half.
  **Cost while deferred:** the divergence outlives the cut, and the next gate-source auditor
  meets a criterion that answers its own question two ways.
  Filed 2026-08-23 by the lead on spec's finding; drained at that iteration's close, which
  re-read both spellings in place and confirmed the SPEC still cites this slug as the filing.

- **backgrounded-shell-child-run-record-unenforced** [design-pending] — the launch-time liveness
  record is advised and never required.
  **The drain corrected this finding's premise.** It was filed claiming "prompts request, guards
  enforce, and here only the prompt exists". guard-kit generic rule 15
  (`guard_rule_background_no_record`) had landed the day before, on 2026-08-22, and does fire on
  the write side — but what it calls is `guard_advise`, additional context at exit 0, never
  `guard_block`. The gap is an advisory that is not a floor, not an absent rule.
  **Attested this iteration:** the align stage session backgrounded two full gate-battery runs
  and wrote no `.run` record for either, self-reporting the omission afterward. No harm resulted
  — it awaited each completion notification before any git-writing command — but the omission was
  invisible to every other actor: the lead, reading the scratch dir for liveness at that moment,
  could not distinguish a finished producer from one that never registered. Generic rule 14's
  block on index/worktree/ref-writing git commands is a no-op against a producer that never
  announced itself.
  **Why `[design-pending]`:** promoting the advisory to a block needs a detection shape a guard
  can hold — it cannot read whether a command *will* write the record without inspecting its
  text, the same text-shaped limit `wait-loop-exemption-blind-behind-a-script-name` records on
  the exemption side. A session-end check over the scratch dir is the other candidate and has no
  false-positive budget yet.
  **DISTINCT from `session-mechanic-grants-uncommitted`**, whose subject was an out-of-band
  permission decision on the journal-append write path — a grant question, not this enforcement
  one. That entry closed this iteration and its subject shipped as guard-kit/SPEC.md §The generic
  ruleset rule 17, which is the adjacent surface; nothing of it is re-filed here.
  **Cost while deferred:** rule 14's reach stays opt-in on the launching session's diligence, so
  a commit can be taken beside a live producer that never announced itself.
  Filed 2026-08-23 by the lead; drained at that iteration's close, which dated rule 15 against
  the filing and read `guard_advise` to establish that it never blocks.

- **interpreter-floor-gawk-residue-empty** [design-pending] —
  the published GNU-awk requirement's *stated ground* is dead while the floor itself still stands.
  **Measured, not reasoned — and the measurement was scoped narrower than the conclusion drawn
  from it, corrected 2026-08-23 at scope.** Probed at the eighth port cut with a `gawk --posix`
  shim on PATH, which refuses every gawk extension: `check-action-run-shell` ran clean at
  exit 0, holding no gawk extension at all; `check-gate-assertions` died loudly ("match: third
  argument is a gawk extension", exit 2). That member ported at this cut. **The probe covered
  those two members only**, so "the shell residue is empty" overreached its own corpus: it was
  not — `check-docs-render-fidelity` was registered in `scripts/gates.list`, was still shell, and
  ran GNU-awk-only `BEGINFILE`/`ENDFILE`/`ARGIND` in live program text. Both members were probed
  at their then-shell declaration paths, deleted since by the port below; the probe records stand
  as history and no live path is cited here.
  **THE STATED PRECONDITION IS DISCHARGED, 2026-08-24 — the entry is UNBLOCKED and its slug is
  now true.** "The floor keeps one live holder and `shell-gate-tail-port` is what retires it":
  that port landed, delta 8 taking `check-docs-render-fidelity` to a `.gate` descriptor plus
  `native/src/gates/docs_render_fidelity.rs`, so the GNU-awk residue is empty **by measurement**
  and no shell holder of the floor survives. The rename this entry's own body says is owed with
  the unblock is now due with it.
  gate-sdk/SPEC.md §check-gate-assertions, §check-action-gh-repo and §The first cohort record
  the probe with its scope, corrected at the same session; the stale header on the then-shell
  `check-action-run-shell` was corrected at the generating close.
  **What the cut did NOT change, and this entry owns.** `awk::GNU` is still an element of
  `context-kit/lib/toolfloor.sh`'s probe roster, held to docs/install.md §Requirements by
  `check-install-toolchain` in both directions — and that section still justifies the requirement
  by "the 3-argument `match()` in `check-gate-assertions`", which the port retired. The stated
  ground is dead while the requirement stands.
  **Why `[design-pending]`:** narrowing a user-facing requirement is an envelope change, so the
  deliverable is a decision — narrow `awk (GNU)` to `awk`, or drop awk from the floor entirely —
  taken only after re-probing every remaining shell gate and both generated hooks under
  `--posix` — a re-probe that now collapses to the two hooks, the shell gate corpus being empty.
  The docs/install.md edge is operator-class; the re-probe is not.
  **Cost while deferred:** low, and no longer conditional on an unblock — an adopter is told to
  install gawk on a ground that no longer holds, for a floor **no gate** genuinely needs any
  more, so the published requirement is neither right by accident nor justified on its face.
  Filed 2026-08-23 by build; drained at that iteration's close, which re-read the probe roster
  and found docs/install.md's stated ground retired by the cut that ported its named holder;
  unblocked 2026-08-24 at `shell-gate-tail-port-and-completion-oracle`'s close, whose drain
  re-verified both cited declaration paths absent and the last shell holder ported.

- **guard-rule-number-intra-kit-citations-ungated** [design-pending] — guard-kit cites its own
  rule numbers everywhere and nothing holds a single citation to the ruleset.
  **Measured at the rule-17 insertion, 2026-08-23**, which shifted rules 17-21 to 18-22: the
  kit's own surfaces carry 111 `rule N` citations — SPEC.md 63, `lib/guard.sh`'s `spec:` comments
  17, `guard-tests/cases.tsv` 28, `bin/run-guard-tests.sh` 3 — and no gate matches any of them.
  The amendment authoring the change enumerated six sites; the sweep found roughly five times
  that, and the merge was correct only because the sweep was run instead of the enumeration
  trusted.
  **Worse, one number-bearing roster was already stale and no renumber caused it:** SPEC.md's
  raw-vs-skeleton paragraph named two rules that carry no raw-command test and omitted three that
  do, drifting silently from the function bodies until it was re-derived against them at build.
  **Why `[design-pending]`:** the corpus is bounded and the numbering is derivable from the
  numbered list, so a gate could assert that every intra-kit `rule N` resolves to an existing
  item and that the derivable rosters — which rules take the raw command, which read a skeleton —
  match the bodies. Which claims are derivable and which are prose is the open question.
  **DISTINCT from `guard-rule-number-not-citable-outside-kit`**, whose dispositions are about
  cross-corpus prose *outside* the kit and which therefore leaves intra-kit numbers citable and
  ungated by construction; and from `guard-ruleset-registration-lockstep`, whose subject is the
  roster/function/dispatch-order triple agreeing, not what cites a rule by number.
  **Cost while deferred:** every insertion into the ruleset re-buys a hand sweep whose
  completeness nothing checks, and a stale roster reads as authoritative to the next author —
  which is exactly how the raw-vs-skeleton one survived.
  Filed 2026-08-23 by build; drained at that iteration's close, which re-counted the citations
  and confirmed no gate matches them.

- **wait-loop-grant-lost-its-carrier** [design-pending] — the mandated in-turn wait's grant is
  owned by no queue entry, and became unowned by a completion rather than by a ruling.
  **`session-mechanic-grants-uncommitted` carried it** as the third member of its class — 19
  measured out-of-band calls across two sessions in this iteration alone, every one a
  `kill -0 "$pid"` loop condition that a `Bash(...)` glob provably cannot match, because the
  mandated form *is* a loop condition and so is decorated by construction. That entry's promotion
  deliberately excluded the wait-loop grant, sequencing it behind the wait-primitive measurement
  on the stated ground that a grant minted around the currently-sanctioned form would be shaped
  to a primitive the measurement might correct. The exclusion was right and is not questioned.
  **What the sequencing did not account for** is that the entry itself completed at this build
  and moved to Done, so the grant lost its carrier. The refusal survives at guard-kit/SPEC.md
  §The generic ruleset rule 17 — but a SPEC paragraph is not a work item.
  **The precondition is discharged.** The measurement returned at that same build: the ordering
  stands and the polarity was the bug, so rule 17 now records that a grant is shaped to
  `while kill -0 "$pid" …` as well as `until <cond> …`. Nothing here needs re-deriving.
  **Why `[design-pending]` rather than ready-to-build:** the deliverable is guard-kit code and is
  landable by a stage session — rule 17's own landing demonstrated that end to end — but the
  grant's shape (a rule 6 narrowing, a new generic rule, or a raw-command exemption) is unruled.
  **DISTINCT from `turn-end-chokepoint-and-wait-primitive`**, the measurement half, whose
  completion is this entry's precondition rather than this work; and from
  `overlay-only-oracle-grants-uncommitted`, the parent's other split half, whose subject is four
  Rust/gh toolchain oracles and not the wait form.
  recurrence: wait-loop-grant-lost-its-carrier 2026-08-24
  **FIRST RECURRENCE, 2026-08-24, and the PRICE IS THREE TIMES WHAT THIS ENTRY CARRIED.** Measured
  at close's prompt-friction triage off the log rather than impression: **60** `while kill -0 <pid>`
  occurrences across this iteration, ranked as the log's third-heaviest prompting pattern, against
  the nineteen the cost line below was written on. Eight `until` calls beside them. The mechanic did
  not change and the rule did not change; the iteration simply backgrounded more producers, which is
  what a methodology mandating in-turn waits produces as it is followed more closely. So the carry
  is not a fixed nineteen-per-iteration tax — **it scales with compliance**, which is the worst
  shape a friction cost can have and is the reading a later scope should price against.
  **Cost while deferred:** rule 6 decides the mandated `kill -0 "$pid"` loop out of band on
  *every* call — sixty decisions in this iteration alone, for a mechanic the methodology
  mandates — and the price is now invisible to scope because no entry carries it.
  Filed 2026-08-23 by build and ruled by the lead to stay a filed gap for this drain; drained at
  that iteration's close, which confirmed rule 17 carries the discharged precondition; re-measured
  at `shell-gate-tail-port-and-completion-oracle`'s close.

- **wait-loop-exemption-blind-behind-a-script-name** [design-pending] — guard rule 15's wait-loop
  exemption is command-text-shaped, so a wait loop inside a script draws the advisory anyway.
  **Measured 2026-08-23 at build:** eight backgrounded arms of
  `delegation-kit/bin/wait-probe.sh`'s waiter — whose body *is* `until <cond>; do sleep 1; done`
  — each drew the recording advisory, whose own closing sentence says a backgrounded wait loop
  "owns no work a commit could corrupt and owes no record".
  **Why:** the exemption is detected by the `do … done` span walk
  `guard_rule_background_no_record` performs over the skeletonized command text; at `PreToolUse`
  the script body is not readable, so the span walk cannot see it. That is the honest limit of a
  text-shaped predicate rather than a bug — but the population is not marginal: a reusable wait
  helper is exactly the shape a methodology that mandates in-turn waiting will grow, and every
  invocation of one pays an advisory saying the opposite of what the rule means.
  **Why `[design-pending]`:** the two candidate dispositions are uncosted and point opposite ways
  — widen the exemption to a leading roster member whose name the consumer declares, which rule
  15 explicitly refuses as consumer vocabulary; or accept the limit and say so in the rule's own
  honest-limit paragraph so the next reader does not re-derive it.
  **DISTINCT from `backgrounded-shell-child-run-record-unenforced`**: that entry is the false
  negative — the advisory is not a floor — and this one the false positive, where the exemption
  cannot see through a script name. One text-shaped predicate, two opposite failures, two fixes.
  **Cost while deferred:** the advisory's credibility decays — a session that meets it wrongly
  once learns to read past it, which is the erosion that makes an unenforced rule cheap.
  Filed 2026-08-23 by build; drained at that iteration's close, which read the span walk to
  confirm the limit is structural rather than a detection bug.

- **kit-spec-layout-tree-hand-maintained** [design-pending] — every kit SPEC's Layout tree block
  is a hand-maintained roster of the kit's own filesystem, and no gate asserts it matches.
  **Found 2026-08-23 at build** by a contract sweep before adding
  `delegation-kit/bin/wait-probe.sh`: every `bin/`, `lib/`, `checks/`, `gate-tests/`,
  `templates/` and `smoke/` member is listed by hand with an inline annotation, and a member
  added without editing the block reds nothing. The population is the whole kit set, not
  delegation-kit alone.
  **This is a derivation-first defect by the doctrine's own words** — a roster is derivable, so
  it is derived and freshness-gated, never maintained. What makes it worth an entry rather than
  absorbing is the direction of the silent failure: an omitted member leaves the SPEC
  *understating* what the kit ships, which is exactly the surface a vendoring consumer reads to
  know what it got.
  **Why `[design-pending]`:** the obvious shape is a marker-block projection emitted from the
  filesystem with a byte-gate, as ROADMAP.md and the docs mirror already take — but the
  annotations are hand-authored prose per line, so the emitter has to preserve them across a
  regeneration. That is the design question and the reason this is not mechanical.
  **DISTINCT from `check-readme-roster`**, whose subject is the README's gate roster and which
  reaches no `bin/` or `templates/` member.
  **Cost while deferred:** every kit-member addition re-buys a hand edit nothing checks, on the
  surface a consumer trusts to enumerate what it vendored.
  Filed 2026-08-23 by build; drained at that iteration's close, which confirmed no gate reads
  the block.

- **crate-arms-relink-under-worker-pool** [design-pending] — `check-crate-arms` rebuilds the
  dispatch artifact its sibling members exec, so one battery member mutates another's interpreter
  mid-run.
  **The mechanism, re-probed at the drain rather than taken on the bullet's word, and re-spelled
  2026-08-24 after `shell-gate-tail-port` ported the member.** `native/src/gates/crate_arms.rs:79`
  reads `GATE_SDK_CARGO_TARGET_DIR` and runs cargo clippy/test into it; the fallback to the
  crate's own `target` moved surface as well as line — it is `gate-sdk/lib/gate.sh:94` that now
  supplies it, so the default is resolved once in the config bridge rather than at the member's
  read site. gate-sdk/SPEC.md §Layout and configuration rules that default deliberately, so the
  arms reuse `build-native.sh`'s warm cache. `gate-sdk/lib/gate.sh` dispatches every ported member
  through `native/target/release/checkwright-gates` — the artifact cargo relinks in that same dir.
  Under the battery's worker pool that is a member rewriting an artifact its siblings exec, the
  concurrency contract's own defect class (gate-sdk/SPEC.md §run-gates).
  **The port STRENGTHENS the mechanism, and the sentence above understates it now.**
  `check-crate-arms` is itself `.gate`-dispatched since that port, so the artifact it relinks is
  no longer only a sibling's interpreter — it is **the process the member is running in**. The
  window's narrowness below still holds (cargo removes then hard-links a fully written file), but
  the exposure is one class wider than "one battery member mutates another's interpreter".
  **Why the window is narrow.** Cargo removes then hard-links a fully written file, so an exec
  landing in the window gets ENOENT rather than a torn binary, and no flake has been observed from
  it. The 1-in-3 red found at `battery-runner-port`'s build was the abandoned-pipe-producer class
  — a **different** defect, fixed in place, and this entry is not a re-filing of it.
  **Why `[design-pending]`, and lead-ruled 2026-08-23 to stay filed rather than fixed.** The
  candidate fix is a build cache of the gate's own under the pruned target dir
  (`native/target/arms`), costing a second ~550MB cargo cache and one cold build, and it
  **narrows the recorded shared-warm-cache ruling** in gate-sdk/SPEC.md §Layout and configuration.
  It was built, tried at that build, and did not fix the flake it was reached for. So the
  deliverable is a ruling on the warm-cache trade before it is a patch.
  **Cost while deferred:** low, and paid only during crate development, as a possible spurious
  dispatch-harness error on one member of a battery run that also builds the crate.
  Filed 2026-08-23 by build; drained at that iteration's close, which re-read both the knob
  default and the dispatch path in place.

- **binary-less-dispatch-loop-retirement** [design-pending] —
  `run-gates.sh`'s binary-less dispatch loop and `installer/consumer-smoke`'s binary-less
  green-battery assertion both retire at zero contract cost once no `.sh` member is left, and
  nothing today schedules that retirement.
  **Distinct from its blocker, verified at the drain.** `shell-gate-tail-port`'s body was re-read
  in place: it names the six registered shell gates, the two unregistered kit-shipped ones, the
  wrapper shape each port takes and the dependency-declaration design question — and says nothing
  about this cleanup. So the retirement is the follow-up its landing *enables*, not a re-filing of
  the port itself.
  **Why it is sequenced and not open.** The loop stays by a 2026-08-23 ruling because criterion
  5's stranded-platform branch still needs it while any `.sh`-dispatched member exists. At zero
  `.sh` members a binary-less consumer has no gates at all, so the branch the loop serves has
  nothing left to serve, and the smoke assertion it backs asserts an empty battery.
  **THE CONDITION HAS ARRIVED, AND THE ENTRY IS RE-SCOPED — LEAD-RULED 2026-08-24 as
  `shell-gate-tail-port`'s criterion-5 price, filed rather than taken at the cut because it
  changes what an adopter is TOLD they receive.** **The measurement**, taken from clean checkouts
  of both sides reached by path: pre-unit the binary-less leg exits 0 declaring 24 omitted
  members; post-unit it FAILS on `run-gates: scripts/gates.list names no gates`. The prose
  profile's seeded roster is 26 members on both sides — 24 `.gate` plus exactly two `.sh`,
  `check-install-disposition` and `check-shellcheck`, which were the entire battery a binary-less
  adopter still had. Residual 24-of-26 to 26-of-26. **Not profile-specific:** no `.sh` gate
  declaration exists anywhere after that unit, so every profile's artifact-free install seeds an
  empty registry, and the leg's own re-scoping clause ("re-scope it on the profile whose roster
  criterion the binary-gated class empties") has **no profile left to be re-scoped onto** — itself
  part of what this unit must repair.
  **The shape.** The leg's assertion moves from "the battery is green on the consumer init just
  made" to "the registry declares its omissions and, where nothing survives, says so".
  **What must NOT change, and it is why the obvious small fix is refused:** `run-gates`' "names no
  gates" at exit 2 stays a **refusal**. It exists to stop a vacuous pass, and turning a
  fail-closed into a fail-open on the one tree with no other signal is the worst trade available
  — the smallness of that diff is not a mitigation. The work is in what the LEG asserts and what
  `init` tells the adopter, never in relaxing the runner.
  **Open questions the unit owes:** whether an all-omitted install should still be called an
  install or should refuse at `init` with its own message; what `doctor` reports for it; and
  whether the leg keeps a non-zero omission count as its completeness assertion now that the
  count is the whole roster.
  **Cost while deferred:** an adopter on an uncovered platform installs successfully, receives a
  `gates.list` that declares every omission honestly, and then finds the battery cannot run — the
  disclosure is correct and the experience is a broken install. Held visible meanwhile by the
  `installer_smoke installer_smoke fail binary-less-dispatch-loop-retirement` row in
  `.workflow/validate-baseline.txt`, which validate writes, per criterion 5's own UNPAID-price
  mechanism. The loop's own carry is unchanged and secondary — roughly 180 duplicated dispatch
  lines in `run-gates.sh`, held honest by an executed byte-comparison against the `--run` arm
  (`gate-sdk/gate-tests/run-arm-contract.test.sh`), so it is gated duplication rather than drift.
  Filed 2026-08-23 by build; drained at that iteration's close, which re-read the blocker's body
  to confirm the two deliverables are distinct; re-scoped 2026-08-24 at
  `shell-gate-tail-port-and-completion-oracle`'s close, whose drain re-ran the leg and confirmed
  the ruled empty-registry outcome on a clean tree.

- **pack-installer-vendors-untracked-scratch** [design-pending] — `scripts/pack-installer.sh`
  vendors kit roots with a verbatim `cp -R`, so any git-ignored artifact sitting under a kit's
  tracked tree rides into the installer payload and breaks `checkwright init` on the consumer.
  **The failure it produces.** `init` stages every vendored file with a plain `git add`, which
  refuses when the consumer's own `.gitignore` also ignores that path ("paths are ignored by
  one of your .gitignore files"), so init dies with "could not stage the vendored files".
  **The class is live, and the instance is live AGAIN — both re-probed at this drain.**
  `scripts/pack-installer.sh:110` still vendors each kit with a bare `cp -R` and the script
  holds no tracked-path filter anywhere, so the class is live.
  **The 2026-08-23 "the observed instance is gone" claim is RETRACTED as stale**, and with it
  the retraction of the original cost line. Running the gate-sdk fixture suite STANDALONE — not
  inside the battery, where the earlier re-probe ran and where suite ordering masked it —
  regenerates `check-crate-arms`'s ignored source-stamp cache under its tracked `good/` case
  every time. The absolutization credited with the fix reaches SHELL-dispatched members only,
  so it never protected this one; the mechanism is
  `bridged-knob-case-tmp-dir-override-inert`'s, already filed, live, and self-declared upstream
  here.
  **Why `[design-pending]`:** the candidate fix is one filter at the vendoring step, vendoring
  git-tracked paths only, but it needs a decision about consumers vendoring from a non-git
  payload — packaging semantics rather than a patch.
  **Cost while deferred:** paid NOW rather than on recurrence. Every from-scratch validate
  needs a manual removal plus a clean-tree `installer_smoke` re-run — the original filing's
  cost line, reinstated because the retraction that replaced it has itself been falsified — and
  this iteration's own validate paid it, the leaked artifact feeding the pack step and
  reddening `installer_smoke` for the wrong reason.
  recurrence: pack-installer-vendors-untracked-scratch 2026-08-24
  Filed 2026-08-23 by validate and re-dispositioned by validate the same day; the 2026-08-23
  close drain read both bullets as one disposition and re-ran its probes inside the battery
  alone, which was not isolated enough to see this; the 2026-08-24 close drain re-probed the
  fixture suite standalone, reproduced the artifact, and retracted the retraction.

- **bespoke-test-path-knob-pinning** [design-pending] — a bespoke gate-test's cwd sandbox is
  isolated only while `GATE_SDK_TMP_DIR` and `GATE_SDK_WORKFLOW_DIR` happen to hold relative
  values in the invoker's environment, which is an ambient default rather than anything the test
  owns.
  **Surveyed, and the survey re-run at the drain.** 16 bespoke `*/gate-tests/*.test.sh` build
  cwd-relative `.tmp`/`.workflow` sandboxes; 11 pin one of those knobs explicitly, so pinning is
  already the majority idiom. The 7 that do not: `canon-kit/check-comment-tier`,
  `evidence-kit/producer-lock`, and lifecycle-kit's `check-stage-entry`, `check-merge-attrs`,
  `check-survey-record`, `check-stage-evidence`, `check-close-surfaces`.
  **Probed, not inferred (2026-08-23 validate).** With `GATE_SDK_TMP_DIR` pointed at an absolute
  dir holding a live-pid `run-validate.lock`, `producer-lock.test.sh` reds on 4 assertions because
  its inner run-validate reads the real lock; with `GATE_SDK_WORKFLOW_DIR` absolute it reds on 4
  more, reading a foreign `validate-evidence.txt`. That is the same failure `0c29e19f`'s
  process-wide export produced and `8f2000f0` narrowed away — an export was one way to supply an
  ambient absolute value, not the only one. `producer-lock` is the sharpest case: the evidence_kit
  suite runs inside the spine *while a real run-validate producer is live by construction*.
  **Why `[design-pending]`:** two fix shapes, and choosing is the deliverable. Have each exposed
  test pin its own path knobs (~7 one-line edits, no new mechanism, conforming to the majority
  idiom); or widen `gate-sdk/lib/test-hermetic.sh` to neutralize the path knobs the way it already
  neutralizes `<KIT>_CONFIG_FILE` — one place, and precedented, since that file already pins
  `GATE_SDK_NATIVE_BIN` absolute for the *same* reason (a relative default resolving to nothing
  from a sandbox cwd) — but it changes a bootstrap every bespoke test sources and needs each
  pinning test re-checked for an override it currently gets from the ambient value.
  Distinct from `hermetic-bin-roster-config`, which is credential pinning in smoke scripts under
  `check-test-hermetic` assertion B: a different knob class on a different surface.
  **Cost while deferred:** low and conditional — nothing is red today and the spine is green with
  the case-scoped pin; the exposure is that any future harness or operator exporting an absolute
  `GATE_SDK_TMP_DIR` or `GATE_SDK_WORKFLOW_DIR` silently converts a test sandbox into live-state
  access, which reads as a mystery red in an unrelated kit's suite rather than a configuration
  fact.
  Filed 2026-08-23 by validate; the close drain re-ran the survey oracle and got 16/11/7 with the
  same seven names.

- **validate-tier-premise-mechanical-only** [design-pending] — the ruling-config assigns `validate`
  the cheaper model on the premise that its batches are uniformly mechanical oracle-running, and one
  iteration falsified that premise.
  **RULED BY THE OPERATOR 2026-08-23 — (b): KEEP THE TIER, ADD AN ESCALATE-ON-DISCOVERY CLAUSE.**
  **FIRST EVIDENCE THE CLAUSE WORKS, 2026-08-24:** carried as a one-off in this iteration's
  validate dispatch, it fired as intended — validate met a real defect and ESCALATED, not ground.
  `validate` stays on the cheaper model, and a validate that discovers it must **fix** what it found
  gets a named, cheap transition to the judgment tier rather than an improvisation. That CLOSES this
  entry's design fork: what remains is delivery, not design. **Deliberately NOT promoted into
  `shell-gate-tail-port-and-completion-oracle`** — the operator set that unit set and this is not in
  it, so the entry is a unit with a settled shape awaiting a later scope's attention.
  **The `[design-pending]` tag STAYS, and this line exists so a later session does not strip it on
  the strength of the paragraph above.** It is a section-membership invariant (canon-kit/SPEC.md
  §The amendment lifecycle — every entry in the set carries it), so it marks the section rather than
  an open design question and comes off at PROMOTION. Probed rather than argued at the 2026-08-23
  drain: removing it reds `check-amendment-queue`.
  **OPERATOR-CLASS, and this entry is a CARRIER rather than a proposal.** The tier is a recorded
  ruling carried in the lead binding, so reversing, demoting or re-scoping it is the operator's;
  the scope that promoted this escalated it and ruled nothing. What follows is the observation.
  **The instance, from `battery-runner-port` 2026-08-23.** Validate's spine went red on an
  INHERITED defect — a gate depositing runtime state inside the tracked fixture corpus it is the
  oracle for — and closing it took a diagnosis, a fix, a self-caught overreach (the first cut
  exported the pin process-wide and broke `producer-lock.test.sh`'s sandbox, narrowed at `8f2000f0`)
  and three spine runs. The lead re-dispatched the stage on the more capable tier for exactly that
  reason, so the corrective already happened; what is unrecorded is why.
  **The distinction worth keeping, and it is the whole content: the TIER may well be right and the
  PREMISE is what needs re-judging.** A validate that only runs oracles is mechanical; a validate
  that has to FIX what it finds is not, and nothing in the current framing distinguishes the two or
  says what a session should do on discovering it is in the second kind.
  **The surface this bears on, NAMED and STILL not corrected — now under the ruling rather than
  under restraint.** `build-stage-tier-economics` carries "the already-adopted validate→Sonnet
  downgrade **demonstrably works** ... with no observed quality cost" as the affirmative precedent
  its A/B tests for. That sentence has an operator-acknowledged counter-instance and the tier
  survived it anyway. The operator ruled on the TIER, not on that entry's prose, so the correction
  rides whenever that entry is worked and no session edits it on the strength of this one.
  **DISTINCT from `build-stage-tier-economics`**, which is the BUILD stage's tier and its per-batch
  split; this is validate's, and the two share only the meter that priced them.
  **The three shapes that were open, and which was taken:** re-tier validate — refused; keep the
  tier and add the escalate-on-discovery clause — **TAKEN**; record the premise's honest limit and
  leave the tier alone — refused. Kept rather than deleted because a later session weighing a
  re-tier meets what was already weighed against it.
  **Cost while deferred, now that the shape is settled:** the exposure is unchanged and the carry is
  narrower — a validate meeting a real defect on the cheaper tier still either escalates for a
  re-dispatch, costing a lead turn and a restart, or does not, which is the case nobody sees. What
  is no longer carried is the design question; what is carried is an unbuilt clause.
  Filed 2026-08-23 to the gap inbox at `battery-runner-port`'s close, offered by the lead and
  deliberately not ruled there; promoted 2026-08-23 at the next iteration's scope drain, which
  escalated the ruling and did not take it; ruled by the operator the same day and recorded here by
  that same scope, which authored no part of the ruling it records.

- **relayed-rule-role-scope-unchecked** [design-pending] — a rule relayed through a dispatch can be
  scoped to the RELAYER's role and bind nobody it is relayed to, and nothing in the relay path
  checks that.
  **Two attested instances, both from `battery-runner-port` and both the lead's own self-report.**
  (1) A dispatch instructed a stage to write `recurrence:` stamps in a shape queue-kit/SPEC.md
  forbids that stage to write. (2) A dispatch told validate not to re-run `run-validate.sh` — a
  LEAD-SIDE verification rule, the lead's job being to check the evidence rather than regenerate it
  and the stage's being the opposite.
  **The second cost something measurable rather than merely being wrong:** it produced a spliced
  evidence manifest the lead then had to reject, and the correct manifest needed a second full
  spine run. The failure mode is that a role-scoped rule relayed as a general one makes the
  receiving session do the wrong thing CONFIDENTLY — a rule arriving through the dispatch channel
  reads as authoritative by construction.
  **The generalization, in the lead's own words:** before relaying a rule, check whose role it
  binds.
  **Why `[design-pending]`:** the home is nameable — lifecycle-kit/templates/lead.md, whose relay
  duties the rule would qualify — but it is a kit template binding every consumer, so the wording is
  an envelope change. The harder half is whether anything can CHECK it: a dispatch prompt is prose
  and the roles are defined across three surfaces (lead.md, the stage templates, each kit's SPEC),
  so a scanner would have to resolve a relayed sentence to a role, which is not obviously decidable.
  The cheaper shape is a stated relay discipline naming the surfaces a lead consults before
  relaying, which costs a template line and buys no detection.
  **DISTINCT from three neighbours, all re-read at the promotion.**
  `lead-specifies-constraint-not-mechanism` is a lead handing down a MECHANISM where a constraint
  was its remit; `dispatch-claim-evidentiary-tier-unmarked` is a lead handing down a CLAIM whose
  evidentiary tier was silently upgraded in transit; this is a lead handing down a RULE that was
  never its to hand down. `relayed-ruling-provenance-unrecorded` is a relayed ruling's
  authorization channel being untraceable, not whom the rule binds. The first two share this
  entry's home and its envelope-change reasoning, so a scope ruling any of the three should rule
  all three — and a MERGE of the three was asked for at the 2026-08-24 drain and DECLINED with
  cause, each carrying independently attested evidence near the entry cap.
  **Cost while deferred:** low frequency, high per occurrence, and invisible from the receiving end
  — a stage that follows a mis-scoped relayed rule produces work that looks compliant and is wrong,
  and only the relayer can tell.
  Filed 2026-08-23 to the gap inbox at `battery-runner-port`'s close, on the lead's self-report and
  deliberately not written into the template there; promoted 2026-08-23 at the next iteration's
  scope drain.

- **build-native-obligation-unconditional** [design-pending] — CLAUDE.md states the commit-time
  obligation as the full battery **plus** `bash gate-sdk/bin/build-native.sh`, with neither
  discharging the other and no source-touched condition, while the only cost that build buys is
  source-conditional.
  **The mismatch.** A commit touching no crate source rebuilds a binary `check-gate-binary-fresh`
  has just certified green in the same battery, and in a batched build stage that rebuild can race
  a sibling session working the crate through the shared target dir.
  **The live instance, 2026-08-23, and it is why this is filed rather than argued.** Build batch 1
  skipped the build on exactly that reasoning; the lead REVERSED the act while ENDORSING the
  reasoning. Under spec-over-precedent the obligation as written held, so a session reasoning
  correctly about the cost is currently reasoning against the text. The re-run afterwards
  confirmed the prediction: no tracked change.
  **Deliverable:** make the text carry the condition it means, or state plainly why it must stay
  unconditional. The second is a real option — an unconditional rule needs no judgement at the
  moment a session is least able to make one, which is the argument the current wording is
  presumably standing on and is nowhere written down.
  **Why `[design-pending]`:** CLAUDE.md is always-loaded, so the wording is paid per session, and
  the choice between a condition and a stated absolute is a doctrine call rather than an edit.
  **Cost while deferred:** low per commit and steady — a redundant cargo build on every crate-free
  commit, plus a standing invitation for a session to reason its way out of a stated obligation.
  Filed 2026-08-23 to the gap inbox by build. Promoted 2026-08-24 at
  `shell-gate-tail-port-and-completion-oracle`'s close, whose drain re-read CLAUDE.md and found
  the obligation still stated with no source-touched condition.

- **kit-knob-consumer-adapter-convention** [design-pending] — whether a kit may ship a knob whose
  only working configuration requires the consumer to author an adapter, and whether the front-end
  shape that answer takes should be a named convention rather than each consumer's invention.
  **The capability loss that raised it is CLOSED, and this entry is deliberately the residue.**
  `shell-gate-tail-port` deleted the path `DELEGATION_KIT_LIVENESS_CMD` defaulted to, so the
  turn-end liveness probe logged `verdict=unavailable` on every firing. The repair landed at the
  same cut: the kit template dropped a default that pointed at nothing (a path present in no tree
  reads as a shipped capability and is none), the knob kept its contract exactly — a path run with
  the scratch dir as its only argument — and this repo named its own reader
  `scripts/producer-liveness-reader.sh`, reaching the gate by name through `scripts/gate-exec.sh`.
  Proved by negative control at `scripts/gate-tests/subagent-stop-reader.test.sh`, which fails
  with exactly `verdict=unavailable` against the dead default.
  **The capture log priced the loss and then verified the repair, read at close.**
  `.workflow/subagent-stop-liveness.log` carries **77** `verdict=unavailable` firings, every one
  inside a single 46-minute window between the port that deleted the path and the repair, and
  **zero** after it. So the degradation was real, bounded, silent except for that field, and is
  closed — which is what makes the surviving question a convention question rather than a defect.
  **The precedent this stands on, which is what makes the question general.** evidence-kit met the
  identical break one caller over when the same port turned a pre-flight entry's named path into a
  descriptor, and discharged it with a CONSUMER-SIDE front end resolving the gate name, explicitly
  refusing to teach the kit's knob to resolve a name as "a kit-contract change". Two kits, two
  consumers, one shape, invented twice.
  **Why `[design-pending]`:** naming the convention is a cross-kit envelope change and it collides
  with the provenance seam — a kit template cannot name this repo's `scripts/gate-exec.sh`, the
  front end evidence-kit/SPEC.md §check-evidence-manifest rules an entry must reach a gate
  through, so any default is a seam question before it is an ergonomics one.
  **Cost while deferred:** low and adoption-shaped — every consumer configuring such a knob
  rediscovers the adapter shape from scratch, and a consumer that does not gets honest degradation
  rather than a break, which is why nothing forces the issue.
  Filed 2026-08-24 to the gap inbox by build in two bullets, the second correcting the first's
  repair premise as measured-false; promoted 2026-08-24 at
  `shell-gate-tail-port-and-completion-oracle`'s close, whose drain confirmed the reader, its
  oracle and the surviving question.

- **substrate-parity-assertion-c-reach-unannounced** [design-pending] —
  `check-gate-substrate-parity` assertion C is one-directional by design, so its derived set can
  shrink to a fraction of its former reach with a green battery and nothing anywhere says so.
  **Measured at `shell-gate-tail-port`'s cut, in clean checkouts of both sides** (`git worktree` at
  the pre-unit head and at the unit's HEAD), with the derivation validated against the gate's own
  clean line before it was believed: the substrate-sensitive set falls from 29 members to 9.
  Twenty left; none entered. The record and its cost are at gate-sdk/SPEC.md §Meta-gate
  conservation for the binary substrate.
  **The cause is one thing.** Every departing member was selected by a `*.sh`-shaped `couples`
  token — `kit:*.sh`, `scripts/*.sh`, `gate-sdk/checks/*.sh`, `kit:*/*.sh` — and the unit emptied
  the tree of shell gate declarations, so no `*.sh` glob covers a declaration path any more.
  **The gap.** Assertion C reds on a derived member with no disposition and never on a disposition
  for a member no longer derived, so a shrinking set can never red. A later meta-gate author
  reaching for `kit:*.sh` — before this cut the commonest route into the derived set — now
  receives NO assertion-C coverage and no signal saying so. That is precisely the
  silent-end-of-an-assertion failure the conservation section exists to prevent, arriving through
  the derivation's own input rather than through a port.
  **Why `[design-pending]`:** the repair is a change to the derivation, and choosing among the
  candidates widens a shipped gate's asserted behaviour. Derive over both declaration spellings
  regardless of which the tree currently holds; or report the set's size and its movement so a
  fall is visible; or assert the converse direction against a recorded-not-derived marker.
  **Cost while deferred:** silent and forward-looking — nothing is wrong today, and the first
  thing that goes wrong is a new meta-gate over gate source landing with no disposition and no
  red, the precise event the section was built to make impossible.
  Filed 2026-08-24 to the gap inbox by build; promoted 2026-08-24 at
  `shell-gate-tail-port-and-completion-oracle`'s close.

- **check-graph-trigger-consumer-path-reach** [design-pending] — a consumer's measured-claim keys
  ride the generated pre-commit hook's baked knob values, and `check-graph`'s trigger does not
  reach every path that consumer's own emitter depends on.
  **Measured, not predicted.** `scripts/git-hooks/pre-commit` bakes
  `GATE_SDK_KNOB_CANON_KIT_MEASURED_VALUES` verbatim, so the tree-shell-owed key puts a
  whole-tree-derived count into a byte-gated artifact. `check-graph`'s `couples=` reaches
  `scripts/*.sh` and `kit:*.sh` but not `demo/run-demo.sh` or the 11 `installer/**.sh` — 12 files,
  enumerated by running the expansion. A commit touching only those 12 stales the baked value and
  fires `check-graph` in neither hook; nothing else catches it, because a key that no inline
  measured-marker names is invisible to all three `check-measured-claim` arms.
  **NOT repairable in the kit, and that is the design input this carries.**
  `gate-sdk/checks/check-graph.gate` ships to consumers, so writing `installer/` and `demo/` into
  its `couples=` would publish this repo's layout as kit mechanism (CLAUDE.md §The provenance
  seam). Nor is the hole generically closable: `CANON_KIT_MEASURED_CLAIMS_CMD` names a consumer's
  own emitter, which may depend on anything. In THIS tree the full-battery-before-every-commit
  rule covers it, which is why the unit stated the bound at gate-sdk/SPEC.md §port-blockers rather
  than repairing it.
  **Deliverable — rule which of three:** (a) a consumer-side shadowing descriptor for
  `check-graph` carrying this repo's paths, which the registry's consumer-first resolution already
  permits; (b) a couples-extension knob so a consumer adds trigger paths without forking a
  descriptor, the `graph-vocab` pattern applied to a new field; or (c) accept the bound as stated
  and close, on the ground that the mandated full battery already covers it and the hook is a
  convenience tier.
  **Cost while deferred:** low and bounded — a stale baked value in the committed hook between a
  12-file edit and the next full battery run.
  Filed 2026-08-24 to the gap inbox by build. Promoted 2026-08-24 at
  `shell-gate-tail-port-and-completion-oracle`'s close, whose drain re-ran the couples expansion
  and got the same 12 paths.

- **port-blockers-library-mediated-scan** [design-pending] — `port-blockers`' command-position scan
  does not follow a call into a kit library, so a requirement reached through a shared helper is
  invisible even for a REGISTERED, in-corpus member and the default arm reports "clean" for a gate
  that genuinely requires an off-floor program.
  **A limit of the SCAN, not of the corpus**, so neither the `--tree` arm nor any widening of a
  walk closes it. It is recorded as a standing limit at gate-sdk/SPEC.md §port-blockers.
  **Its attested instance is DISCHARGED and the limit is not, which is exactly why this is filed.**
  `check-producer-liveness` reached `ps` only through `ek_pid_alive` in evidence-kit's library and
  was the worked example for the blind spot; `shell-gate-tail-port` ported it, so it declares `ps`
  outright through `--needs` and no longer demonstrates anything. The next member to reach a
  program through a helper will be reported clean by a report that cannot see it, and the tree no
  longer carries a live instance to make the hazard visible.
  **Deliverable — cost the widening and rule whether it lands.** The shape: follow a
  command-position call whose name resolves to a function defined in a sourced kit library, and
  resolve that function's own command positions transitively. Its price is a false-positive
  surface the current scan does not have — a library function's spawn is not necessarily on the
  caller's path, so a member would be charged for a requirement a branch it never takes might
  reach — plus a termination question on mutually recursive helpers.
  **The cheaper alternative, and the clause that OVERSTATES it, corrected at the drain.** A
  per-member `--needs` unit test holds observed-subset-declared for every ported member, so the
  compiled side looks covered by execution. It is not covered everywhere: that test runs each
  member over its own `gate-tests/<name>/{good,bad}/` cases, a case runs one argv and so selects
  one mode, and `check-graph`'s two cases are both `--amend-only`, which returns before the
  generator is resolved. Its observed set is EMPTY and the subset assertion vacuous over it. The
  tree-wide shape is "any member whose fixture pair is deliberately narrowed to one hermetic
  assertion", so the compiled side is not the solved side, and the widening may be worth more than
  the cheap-alternative argument allows. The class bound is stated at gate-sdk/SPEC.md §The
  `graph:` manifest beside the subset ruling.
  **Cost while deferred:** low today and silent — a member whose off-floor requirement is
  library-mediated ports with an undeclared dependency and fails first on a consumer's machine.
  Filed 2026-08-23 and re-filed with the correction 2026-08-24 to the gap inbox by build and by
  spec; promoted 2026-08-24 at `shell-gate-tail-port-and-completion-oracle`'s close, whose drain
  read both fixture `args` files and the mode dispatch before believing the vacuity claim.

- **bridged-knob-case-tmp-dir-override-inert** [design-pending] — `run-gate-tests.sh`'s
  `CASE_TMP_DIR` absolutization protects a SHELL-dispatched gate only, so a bridged/native gate
  writes its scratch into the tracked fixture corpus it is the oracle for.
  **The mechanism, measured.** `gate-sdk/bin/run-gate-tests.sh` resolves a bridged gate's knobs
  through `gate_command`'s `mapfile` at line 50 — BEFORE the `GATE_SDK_TMP_DIR="$CASE_TMP_DIR"`
  override at line 84 ever applies. A native gate reads `GATE_SDK_KNOB_GATE_SDK_TMP_DIR`, which
  `_gate_knob_emit` bakes from the ambient, un-overridden shell variable at that line-50 call, so
  the line-84 override is inert for every bridged member. It only ever protected `.sh`-dispatched
  gates.
  **Reproduced live rather than reasoned.** `check-crate-arms` ported from `.sh` to `.gate` plus a
  crate module this iteration, which silently reopened exactly the corpus-pollution class
  `0c29e19f`/`8f2000f0` closed. With both fixture `.tmp` dirs deleted, a plain `gate_sdk`
  fixture-suite run — no manual invocation — regenerated
  `gate-sdk/gate-tests/check-crate-arms/good/.tmp/crate-arms-<hash>.green`. Watched twice.
  **DISTINCT from `pack-installer-vendors-untracked-scratch`, and upstream of it.** That entry's
  `cp -R` filter gap is the VENDORING symptom; this is the cache-pollution CAUSE, in the harness
  itself, for every future native-ported gate that declares the `GATE_SDK_TMP_DIR` knob against a
  tracked fixture pair — not only this one member.
  **Why `[design-pending]`:** the candidate fix is to put the case-scoped `CASE_TMP_DIR` in scope
  at `gate_command`'s own `mapfile` call rather than only at line 84's inert `env` prefix, but the
  config bridge is a single-producer surface and moving where a knob's value is computed for one
  caller is the criterion-6 question in miniature.
  **Cost while deferred:** a fixture-suite run leaves scratch inside a tracked fixture corpus,
  which `pack-installer.sh` then vendors, breaking `checkwright init` at `git add` and reddening
  `installer_smoke` for the WRONG reason — which is what masked a ruled scenario at this
  iteration's validate and cost a diagnosis.
  **The baseline row is NOT re-attributed to this entry, ruled 2026-08-24 and recorded so the
  next validate does not re-litigate it.** `.workflow/validate-baseline.txt`'s `installer_smoke`
  row keeps `binary-less-dispatch-loop-retirement`: that slug is the standing unpaid price the row
  exists to hold visible, and this one is a transient masking cause stacked on top of it — exactly
  the split evidence-kit/SPEC.md §Baseline manifest now rules generally, which is where the ruling
  was landed rather than left as this instance's precedent. This entry's cost line above is where
  the masking diagnosis belongs, and it stays here.
  Filed 2026-08-24 to the gap inbox by validate, with an explicit distinct declaration; promoted
  2026-08-24 at `shell-gate-tail-port-and-completion-oracle`'s close, which deleted the
  regenerated instance in the same commit; the attribution question its recurrence put to
  `execution-control-reach-and-turn-end-blocking`'s close was ruled there.

- **dispatch-claim-evidentiary-tier-unmarked** [design-pending] — a dispatch prompt can upgrade an
  inference into a finding, and the receiving session cannot see the evidence base to discount it.
  recurrence: dispatch-claim-evidentiary-tier-unmarked 2026-08-24
  **THIRD INSTANCE, in a SUB-SHAPE the two below do not reach and the highest-stakes one yet: a
  relayed one-shot AUTHORIZATION, carrying no spent/unspent state.** At the
  `port-remainder-disposition-and-worktree-reclamation` close a dispatch relayed an operator
  authorization for an outward-facing write against a named public artifact, declaring it "LIVE and
  UNSPENT" and its target defective. Both halves were false at HEAD: the target had been repaired
  and the authorization spent hours earlier, by the relaying side, and the tracked issue's own
  comment thread records the fix with a probe. The write was NOT made. What differs from the two
  instances below is that the claim was never an inference — it was a fact that had gone stale
  between the act and the relay, so evidentiary-tier marking would not have caught it; a grant needs
  a consumed state, not a tier. What is the SAME, for the third time, is that the receiving session
  caught it by declining to trust the prompt, and nothing else could have. The stakes differ in kind
  too: the two below cost a session's work, and this one would have been an unrecoverable write to a
  public surface.
  **Two attested instances, both self-reported by the lead and both caught by the RECEIVING stage
  rather than by the lead.** (1) At build, a dispatch said "assume deltas 10 and 11 carry a false
  premise", converting a verification instruction into a quota. The session came within ONE EDIT
  of landing a fabricated premise correction into a governed surface and caught itself; a sibling
  session had independently recorded the same mechanism one delta earlier with the expectation
  supplied by a STREAK rather than a dispatch — "a fourth consecutive hit reads as a rule, and the
  fifth check is what keeps it a measurement". (2) At validate, a dispatch asserted that
  `installer_smoke`'s failure WOULD BE the ruled empty-registry outcome. It was not: the suite
  died earlier at `checkwright init`'s `git add` on an already-filed defect and never reached the
  ruled codepath. What build had MEASURED was that outcome from clean worktree checkouts; the lead
  carried it forward as a prediction about a different run, which is a different claim.
  **Why it is not self-correcting.** The lead writes no lifecycle state and is held to no oracle,
  fixture pair or battery, while its dispatches steer what stage sessions land — so an unmarked
  inference costs a stage session's work and nothing reds. Both instances were caught by a session
  declining to trust the prompt, which is the discipline this repo teaches and not a mechanism.
  The quota form is worse than the streak form only because it arrives with authority: the session
  cannot see the base rate it is being handed.
  **Deliverable, and the harder half is the second:** whether a dispatch prompt can be held to
  marking a claim's evidentiary tier (measured / inferred / expected), and what if anything can
  CHECK it, given that a dispatch is prose and lifecycle-kit/templates/lead.md is a kit surface. A
  cheap shape is a stated relay discipline; it costs a template line and buys no detection.
  **THE THIRD OF A FAMILY, AND A SCOPE RULING ON ONE SHOULD RULE ALL THREE.**
  `relayed-rule-role-scope-unchecked` is a RULE scoped to the relayer's role;
  `lead-specifies-constraint-not-mechanism` is a MECHANISM the relayer had no standing to choose;
  this is a CLAIM whose evidentiary status was silently upgraded in transit. All three share the
  home, the envelope-change blocker and the undecidability limit. A MERGE was asked for at the
  drain and DECLINED with cause: each carries independently attested evidence near the entry cap,
  so one entry would exceed it on arrival and the compression would spend exactly the evidence a
  ruling needs.
  **Cost while deferred:** low frequency, high per occurrence, invisible from the receiving end —
  a session that follows an upgraded inference produces work that looks compliant and is wrong.
  Filed 2026-08-24 to the gap inbox by build and again by the lead; promoted 2026-08-24 at
  `shell-gate-tail-port-and-completion-oracle`'s close, which judged the merge question the second
  filing put to it.

- **single-gate-front-end-form-unruled** [design-pending] — the tree mandates one front end for a
  single-gate run and grants a different one, and which form it reinforces is unruled.
  **Re-verified at this intake rather than carried:** the committed `.claude/settings.json` holds
  **106** `Bash(` entries and **zero** naming `scripts/gate-exec.sh` in any form, while
  `bash gate-sdk/bin/run-gates.sh *` is granted at lines 11-14 and covers `--only <gate>`. So a
  granted alternative genuinely exists and this is a choice between forms, not an absent grant.
  **The mandate's scope is NARROWER than the filing bullet claimed, corrected here.**
  evidence-kit/SPEC.md §check-evidence-manifest binds the **pre-flight caller** —
  `LIFECYCLE_KIT_ENTRY_PREFLIGHT` execs with no interpreter word, so a `.gate` descriptor cannot
  ride the exec bit and the discharge is a consumer-side front end resolving a gate *name* through
  `gate_command`. It does not rule that every evidence entry reaches a gate through that front end.
  The tension survives the correction: the roster the front end serves is the consumer's whole
  pre-flight set, so a session obeying it hand-runs an ungranted path.
  **Why `[design-pending]`:** the two dispositions bless opposite futures. Proposing the committed
  grant reinforces a front end the port may retire — `scripts/gate-exec.sh` is 22 lines and reads
  `owed` on `port-blockers.sh --tree` — and the grant itself is **operator-class** under
  TRAJECTORY.md §The closed rulings (2026-08-22), so no stage session may land it. Steering onto
  `run-gates.sh --only` is landable in guard-kit today and makes the SPEC's own discharge the
  unreinforced path. Neither horn is costed, and the port disposition of the front end decides it.
  **DISTINCT from `guard-steer-grant-mismatch`**, a steer whose target nothing grants: no guard
  rule steers to `gate-exec.sh` at all, the mandate living in a kit SPEC. **DISTINCT from
  `overlay-only-oracle-grants-uncommitted`**, whose four oracles are granted in an untracked
  overlay rather than nowhere, and whose question is which surface carries a grant all parties
  agree is wanted. **DISTINCT from `crate-toolchain-grant-uncommitted`**, ruled toward pruning.
  **Cost while deferred:** an out-of-band decision per single-gate run for any session that obeys
  the SPEC, invisible on a clone whose overlay has absorbed it, and a standing risk that a session
  reads the ungranted form as unsanctioned and stops using the discharge the SPEC designed.
  Filed 2026-08-24 to the gap inbox by `shell-gate-tail-port-and-completion-oracle`'s close, from
  its prompt-friction triage; promoted 2026-08-24 at this iteration's scope intake, its premise
  re-verified and one claim corrected above.

- **promotion-commitment-stamp-latency** [design-pending] — between a promotion commitment and the
  boundary that pays it, an at-ceiling entry accrues firings only prose can hold.
  **The case, and the ceiling's stated remedy does not cover it.** queue-kit/SPEC.md §The tag
  algebra rules that a `recurrence:` declaration reaching `check-queue-wrap`'s 100 columns resolves
  to **promotion** and never to a wider `QUEUE_KIT_WRAP_BUDGET`. At
  `shell-gate-tail-port-and-completion-oracle`'s close two entries hit that ceiling at once —
  `turn-end-chokepoint-and-wait-primitive` and `scratch-execution-control-is-bash-only`, each 96
  columns, each 107 with one more date — and **both were already promotion-committed**. The
  prescribed remedy had been applied and the undercount persisted, because promotion is a future
  act and the stamp is owed now.
  **The window is now measured rather than predicted, which the filing bullet could not do.**
  Both commitments were paid at this boundary: turn-end committed 2026-08-22 and paid 2026-08-24
  across two unstampable judged firings; scratch-execution committed 2026-08-23 and paid 2026-08-24
  across one. So the observed latency is two days and three firings, and `kpi-incident-recurrence`
  undercounted by three for its duration. The window is short and the loss inside it is total.
  **Why `[design-pending]`:** three candidates trading different properties. Stamping into the
  promoted entry at promotion time from a carried record keeps the count honest and needs a carrier
  the boundary can read. A KPI-side annotation for a declared-at-ceiling entry costs nothing and
  detects nothing. Accepting the latency and saying so at the tag algebra costs one line and stops
  a later close re-deriving this — which is the outcome this entry may correctly reach.
  **The prose fallback is itself capped**, which is what makes doing nothing expensive:
  `scratch-execution-control-is-bash-only` sat at **0 lines of headroom** under
  `check-queue-entry-budget` and had to compress to record its own seventh measurement.
  **DISTINCT from `recurrence-obligation-residency`** (where the stamp obligation lives across
  stages), from `threshold-recurrence-routing-residency` (routing an entry that reaches the
  promotion threshold), and from `queue-entry-grammar-single-owner`. It adds no recurrence date to
  any of them and re-files none.
  **Cost while deferred:** paid only when an at-ceiling entry is promotion-committed, and paid then
  as a silent KPI undercount on exactly the entries the project has already judged most recurrent.
  Filed 2026-08-24 to the gap inbox by that same close, from its recurrence-stamp attempt;
  promoted 2026-08-24 at this iteration's scope intake, with the window measured above.

- **friction-key-segment-selection-unruled** [design-pending] — the friction ranking keys the first
  segment of a compound, so a row whose friction lives downstream is filed under a command that
  caused none.
  **The filing bullet's premise FELL at the drain, and the corrected one is narrower.** The bullet
  claimed `scan-prompts.sh` splits compounds for the allowlist filter and not for the key — an
  internal inconsistency. `pattern_of` DOES split: it reads
  `guard_split_compound "$skel" | head -1`, and taking segment 1 is a deliberate rule rather than
  an oversight. guard-kit/SPEC.md §scan-prompts states why: pulling a redirect from anywhere in the
  line would key `mkdir -p .tmp && cat > x` as `mkdir >` and attribute a write to a command that
  performs none. The residual is therefore not "split the line" but **which** segment to key.
  **The SPEC already names it open**, in that same paragraph: "*Which* segment should be keyed when
  the friction-bearing one is not the first is a separate axis — which segment, not which shape —
  and is not settled here." This entry is the queue's carrier for that sentence.
  **Re-measured at the drain, and it bites harder than when filed.** On the live 438-line log 5
  rows key under `mkdir`, and **4 of 5** are `mkdir -p .tmp && cat > .tmp/<journal>.md <<EOF` — the
  friction is the journal write, the key names the directory create. The filing measurement was 2
  of 3.
  **Why `[design-pending]`:** the candidate rules are not obviously ordered. Keying the LAST segment
  inverts the current bias without removing it; keying the segment that actually fell through needs
  a per-segment record the log does not carry; emitting one row per segment changes the ranking's
  unit and steps the KPI numerator the way the write-shape axis already did once. Each is a
  different answer to "what is a prompting call", which is the ranking's definitional question and
  not a tuning knob.
  **DISTINCT from `prompt-ranking-command-word-shape-blind`**, landed this iteration: that unit's
  axis is which SHAPE a segment has and this one's is which SEGMENT is read; a shape-aware key over
  the wrong segment is still wrong. Adds it no recurrence date and re-files none of it.
  **Cost while deferred:** bounded and self-limiting — it mis-files only compound rows, and it
  mis-files them onto `mkdir`, a low row nobody triages, so the effect is an under-count on the
  write class rather than a wrong steer. It grows with the journal-writing discipline the method
  mandates, which is what makes it worth carrying rather than iceboxing.
  Filed 2026-08-24 to the gap inbox by spec; drained 2026-08-24 at this iteration's close, which
  re-read `pattern_of` at HEAD, found the filing premise superseded by work landed after it, and
  re-measured the residual.

- **file-authoring-act-ungoverned** [design-pending] — writing a file has no steer, no grant and no
  owner, so authoring one costs an out-of-band permission decision every time.
  **It is the largest single class in the ranking, measured at the drain.** On the live 438-line
  friction log `cat >>` is 36 and `cat >` is 5 — **41 of 185** prompting calls, the top two rows by
  a wide margin. The filing bullet measured the pre-landing form of the same thing (19 of 19
  `cat`-led lines); the write-shape axis landed this iteration is what makes the class legible as a
  class rather than an argument about one.
  **What the mandated alternatives do NOT cover, which is the whole gap.**
  `guard-kit/bin/scratch-run.sh` governs EXECUTING a scratch script and `git commit -F` governs
  CONSUMING a message file. Both act on a file that already exists. Nothing governs bringing one
  into being, and the sessions paying hardest are the ones the method requires to write most:
  resume journals, scratch scripts, commit messages.
  **This is the disposition half of a two-half finding whose instrument half landed.**
  `prompt-ranking-command-word-shape-blind` made the row READ correctly and deliberately left the
  act ungoverned; guard-kit/SPEC.md §scan-prompts carries the instrument half and says nothing about
  governing the act, so without this entry that half has no home at all. Adds that unit no
  recurrence date and re-files none of it.
  **Why `[design-pending]`, and one of its three shapes is not a stage session's to take.** A
  committed permission grant is a settings edit and therefore **operator-class** under
  TRAJECTORY.md §The closed rulings (2026-08-22) — this entry files it and does not land it. A
  stated habit is not a mechanism. A guard steer onto the Write tool is buildable and is the only
  self-served shape, but it is a wide behavioural change with a named collision: it must not fire on
  the `>>`-under-scratch shape rule 17 already auto-allows, which is the shape the mandated
  resume-journal write itself uses.
  **Cost while deferred:** one out-of-band decision per authored file, paid by the sessions doing
  the most durable recording, and invisible to every gate — the friction log is advisory, so nothing
  reds however far the class grows.
  Filed 2026-08-24 to the gap inbox by spec, as the surviving half of a split its sibling recorded;
  drained 2026-08-24 at this close, which confirmed the act half is recorded nowhere at HEAD and
  re-measured the class against the live log.

- **expansion-rule-backtick-blind** [design-pending] — guard rule 6 blocks the modern
  command-substitution spelling and passes the archaic one, so a session that meets the block learns
  the spelling rather than the rule.
  **Probed live at the drain through the consumer hook, not read off the regex.** The match at
  `guard-kit/lib/guard.sh:352` has four alternatives — `${`, `$(`, `<(`, `$IDENT` — and no backtick.
  A crafted PreToolUse payload spelling a command substitution with `$(…)` exits 2 with rule 6's
  message; the IDENTICAL command spelled with backticks exits 0 and falls through.
  **Under-coverage rather than style, on the rule's own stated grounds.** Rule 6 exists because the
  harness's allowlist matcher refuses every expansion, and the matcher refuses a backtick
  substitution exactly as it refuses `$(…)`. So the fall-through costs the out-of-band decision the
  rule exists to pre-empt, while telling the session nothing.
  **Why `[design-pending]` rather than a one-line regex edit — the entry's real content.**
  Rule 22 is BUILT ON rule 6 not reaching backticks. guard-kit/SPEC.md §The generic ruleset rules
  that rule 22 declines on every expansion because rule 6 blocks those shapes already, but
  deliberately does NOT decline on a backtick "since it is the one body-source spelling rule 6 does
  not reach and declining there would ship the hole the rule exists to close". Closing rule 6's hole
  makes rule 22's backtick arm unreachable by dispatch order and stales three SPEC paragraphs that
  argue from the current split. The unit is that re-argument, not the alternative.
  **The mechanical half is small and known:** one alternative in the regex, a firing and a
  non-firing case in `guard-kit/guard-tests/cases.tsv`, no message change. One interaction to watch:
  a backtick is not a matcher glyph like the one rule 7 handles, and `guard_skeleton`'s `sq`/`hdq`
  modes already strip the spans where a literal backtick could sit innocently.
  **DISTINCT from every unit landed this iteration and it re-files none of them.** It was found
  while probing `scratch-execution-control-is-bash-only`'s shape set, and that unit's rule covers
  the substitution shape under its own body-source predicate in BOTH spellings — so this is about
  rule 6's coverage of every OTHER command a backtick substitution can appear in. Adds no recurrence
  date to anything.
  **Cost while deferred:** one silent permission prompt per backtick substitution, plus the teaching
  effect, which is the worse half — the guard's whole contract is that meeting a block teaches a
  rule, and here it teaches a workaround.
  Filed 2026-08-24 to the gap inbox by spec, after build deliberately left it alone for the blast
  radius above; drained 2026-08-24 at this iteration's close, which re-probed both spellings at HEAD
  and confirmed the rule-22 interaction in the SPEC.

- **fixture-runner-checks-dir-fails-open** [design-pending] — `run-gate-tests.sh` refuses a missing
  tests dir by name and drops a missing checks dir in silence, so the wrong second argument reports
  as a corpus-wide fixture defect.
  **Probed at this close, not inferred.** `bash gate-sdk/bin/run-gate-tests.sh scripts/gate-tests
  scripts/checks` emits `GATE-TESTS: 13 harness/fixture error(s) (malformed fixtures — could not
  test)` and two `HARNESS: <gate> resolves in none of:` lines per gate, each with an EMPTY list
  after the colon. Nothing in that output names the argument, and "malformed fixtures" points the
  reader at the corpus, which is intact.
  **The mechanism is a one-line asymmetry.** `$1` is guarded by
  `[[ -d "$TESTS_DIR" ]] || { …; exit 2; }`. `${@:2}` REPLACES the `gate_check_dirs` default and is
  filtered by `[[ -d "$d" ]] && resolved+=(…)`, so a non-existent member is dropped rather than
  refused and an all-missing set resolves to empty. The filter is right for the DEFAULT set, where
  an absent kit dir is normal; it is wrong for an EXPLICIT one, where the caller named it.
  **Why `[design-pending]` and not a two-line patch.** The two sets need different strictness from
  one code path, and which distinction to key on is the design: explicit-vs-default argv provenance,
  or a `--strict` opt-in, or refusing only when the resolved set is empty. The third is cheapest and
  the weakest — it still passes a run where three of four named dirs are typos. The runner is also
  the oracle every kit's fixture suite rides, so a strictness change is felt by every consumer
  invocation at once, including the vendored ones.
  **This repo is the case that surfaces it**, because its consumer remainder keeps no `checks/`
  directory since the port — so the roster's consumer line takes the tests dir alone while every
  kit line takes two, and the natural generalization from a kit line is the failing form.
  **Cost while deferred:** one session's diagnosis per occurrence, paid by whoever generalizes the
  two-argument form; low frequency, and self-limiting once the SPEC paragraph above is found, which
  is exactly what makes it a documented hazard rather than a fixed one.
  Filed 2026-08-24 at this iteration's close, from the knowledge-friction triage that gave the fact
  its home in gate-sdk/SPEC.md §run-gate-tests; filed rather than fixed under the scope-gated intake
  rule, the harness being validate-critical and this close post-validate.

- **inline-body-interpreter-grant-absent** [design-pending] — `python3` with a body carried IN the
  command string is the tree's standard scratch-computation form, is safe by the tree's own stated
  argument, and is granted by nothing.
  **Measured at this close's prompt-friction triage.** 45 `python3`-led fall-throughs on the live
  438-line log, the largest binary-level class after the write class. The inline shapes are the bulk
  — `python3 -` 30, `python3 -c` 5, `python3 <<<` 1, `python3 /dev/stdin` 1 — and the remainder is
  `python3 .tmp/x.py` 3, `python3 <` 2, `python3 tools/gen.py` 2.
  **Granted NOWHERE, checked both surfaces rather than assumed:** no `Bash(python3*)` entry in the
  committed `.claude/settings.json` and none in the local overlay. The one that used to exist,
  `Bash(python3 .tmp/*)`, was removed at a 2026-08-12 close as a glob over writable scratch and is
  recorded in `settings-content-pin-report-absent`; nothing replaced it for the inline shapes.
  **Disposition (a), and the tree already argued the safety case.** guard-kit/SPEC.md §The generic
  ruleset rules that rule 22 deliberately does not fire on a body carried in the command string,
  because "the permission prompt shows it to the approver verbatim and the friction log records it.
  There is nothing for a compensating control to compensate for." A grant over exactly that shape
  reinforces the form the ruleset already blesses — the opposite of the masking risk the triage
  criterion warns of, which is why (a) is right here and (b) is not.
  **Why it sits in the queue rather than being taken:** a committed grant is a settings edit and
  therefore **operator-class** under TRAJECTORY.md §The closed rulings (2026-08-22). This entry is
  the proposal's durable form; no stage session may land it.
  **The design question surviving the operator decision** is the glob's shape. `Bash(python3 -*)`
  reaches `python3 -c` and `python3 -` and stops at a bare file operand, which is the split that
  matches the safety argument — but a Bash allow-glob's `*` spans `/` and `..`
  (`grant-argument-bounding-mechanism`), so it cannot be relied on to bound anything, and the
  scratch-operand shapes rule 22 now blocks must stay blocked by the guard rather than by the glob.
  **DISTINCT from `overlay-only-oracle-grants-uncommitted`**, whose four oracles are granted in the
  overlay and whose question is which surface carries them; this shape is granted on no surface at
  all. DISTINCT from `file-authoring-act-ungoverned`, which governs bringing a file into being where
  this governs executing a visible body. It re-files neither and adds neither a recurrence date.
  **Cost while deferred:** roughly forty out-of-band decisions per iteration, paid by whichever
  session is computing over the tree, and invisible to every gate — the friction log is advisory.
  Filed 2026-08-24 at this iteration's close, from its prompt-friction triage, both grant surfaces
  read rather than inferred.

- **kfric-obligation-residency** [design-pending] — the knowledge-friction capture obligation is
  declared for "any session" and reaches no surface a kit-template session actually loads.
  recurrence: kfric-obligation-residency 2026-08-24
  **FIRST RECURRENCE, and it is the attested shape repeating in a SECOND iteration with a second
  lead.** At `port-remainder-disposition-and-worktree-reclamation` the lead re-derived off
  implementation source that `run-gate-tests.sh`'s case-scoped scratch override exports the PLAIN
  knob name while the crate's bridged reader resolves only the `GATE_SDK_KNOB_`-prefixed one — a
  fact no doc owned — and carried it to close in a DISPATCH PROMPT rather than stamping it. Probed
  at that close, not inferred: `.workflow/knowledge-friction.log` was 0 bytes. Same session shape,
  same carrier, same silent zero, one iteration later.
  CLAUDE.md §Housekeeping binds capture to **any session** with "deferred capture is no capture",
  and that sentence is the obligation's only statement in this tree. A live grep over the kits'
  own templates — `lifecycle-kit/templates/`, `delegation-kit/templates/`,
  `drift-kit/templates/` — finds `kfric` and `knowledge-friction` **only** in
  drift-kit/templates/close-knowledge.md (the close-side triage, a *reader*) and in
  `drift-kit/templates/kpis.list`. `lead.md` and `agent-execution.md` carry no line at all.
  **Attested rather than predicted, and the instance is the worst-case shape.** At the
  `shell-gate-tail-port-and-completion-oracle` iteration a **lead** session re-derived that
  `run-gate-tests.sh` takes one argument for the consumer-remainder tree and carried the fact to
  close in a dispatch prompt instead of stamping it. Probed at that close, not inferred:
  `.workflow/knowledge-friction.log` was 0 bytes at the close entry, so `kpi-knowledge-friction`
  read zero re-derivations for an iteration that demonstrably had one, and the KPI's own
  lower-bound hedge absorbed the miss silently. The fact itself is now homed at
  gate-sdk/SPEC.md §run-gate-tests and its fix filed, so this entry is the CAPTURE PATH alone.
  **A lead is the shape that re-derives most and writes least**, and it is exactly the shape no
  template serves: it runs off `lifecycle-kit/templates/lead.md`, meets no stage template, and
  reaches the rule only through a consumer's always-loaded file a vendoring adopter need not have
  written at all — so for that adopter the obligation ships with no carrier.
  **DISTINCT from `kfric-empty-log-ambiguity`**, which asks how an EMPTY log should be READ and
  whether the KPI may be trusted at zero; this is the obligation never reaching the writer.
  **DISTINCT from `dispatch-claim-evidentiary-tier-unmarked`**, where a claim's evidentiary tier
  is upgraded in transit — the relay here was accurate and the capture simply never happened.
  **PAIRS with `recurrence-obligation-residency` on one surface**: both are an every-session
  obligation stated only where some sessions look, both resolve under
  delegation-kit/SPEC.md §Operative residency's placement rule, and both face the identical trade
  between one resident line and N template restatements. A unit taking either should take both.
  **Why `[design-pending]`:** placement is the whole question and the candidates differ in kind —
  a bounded imperative in the two unserved kit templates, a line on drift-kit's own dispatch
  surface, or the widest-true-tier answer that disposes of the recurrence twin in the same motion.
  Class: relocates one imperative and mints no name and no gate, so canon-kit's litmus makes it
  **debt**.
  **Cost while deferred:** one silently uncounted re-derivation per lead session that hits one,
  and a KPI that reads cleanest exactly when the least-instrumented session shape re-derives most.
  Surfaced 2026-08-24 by the `shell-gate-tail-port-and-completion-oracle` close and filed to the
  gap inbox there; promoted 2026-08-24 at this scope's drain, which re-ran the template grep
  first-hand and found both files still empty.

- **release-body-step-has-no-in-tree-witness** [design-pending] — the one release step whose
  product lives off the tree is the one that was skipped, and its only backstop is next-day.
  RELEASING.md step 6 fills the GitHub Release body with the note post's apex URL by hand. It is
  the only release step whose artifact never touches the tree, so no gate, no fixture and no
  validate suite can see it; the runbook says exactly that and installs a monitor instead —
  `site-health.yml`'s release-body arm, daily and issue-shaped
  (site-kit/SPEC.md §templates/site-health.yml).
  **The step was skipped at the v0.25.0 cut, measured rather than inferred.**
  `gh release view v0.25.0 --json body` returns an **empty** body, against v0.24.0's well-formed
  one, while `docs/posts/2026-08-23-checkwright-v0-25-0.md` has been on the site since the cut.
  The monitor fired as designed and filed the issue the following morning.
  **What the firing proves, and what it does not.** It proves the backstop works. It does not
  close the gap, because the latency is precisely the window the runbook itself names as the
  reason the hand-check stays — "you are the only actor who can fix the body before anyone reads
  it". A cut whose session skips step 6 is public, wrong and unnoticed for up to a day, on the
  surface an evaluator reaches first.
  **DISTINCT from `release-drain-ordering-contradiction`** (step 4's drain/tag ordering) and from
  `release-runbook-identity-diagnosis` (which account is active); neither reads step 6, and both
  concern steps whose evidence is in the tree.
  **Why `[design-pending]`:** three shapes trade real properties. Generate the body from the note
  post inside the `release` job — retires the hand step, but puts release-note text on a CI path
  the battery never runs. Have the release-sweep skill emit the exact body text as a copy-ready
  artifact — cheap, keeps the human in the loop, still skippable. Or move the monitor's cadence
  toward the cut, which shortens the window without closing it.
  **Cost while deferred:** one wrong public front door per skipped cut, for up to a day, plus the
  standing fact that the release's most reader-facing artifact is the least witnessed one.
  Surfaced 2026-08-24 by GitHub issue #2 and promoted at this scope's boundary sweep, whose probe
  re-read the v0.25.0 Release body directly rather than trusting the issue text.

- **precondition-gate-negation-false-positive** [design-pending] — `check-queue-prose-precondition`
  reds on a sentence asserting the OPPOSITE of a precondition, teaching authors away from a
  true one.
  **Reproduced at this drain rather than cited.** A probe queue whose active entry reads "The
  experiment is filed as its own work rather than blocked on anything" reds the member. The
  trigger set is a phrase-set alternation of bare substrings, so every negating prefix the
  filer named — "not gated on", "no longer waiting on", "rather than contingent on" — matches
  by construction rather than by accident; that half was read off the knob's own value, not
  probed four more times.
  **What the shipped calibration already says, and where it stops.** queue-kit/SPEC.md
  §check-queue-prose-precondition calls the gate FP-bearing by construction and justifies the
  blocking grade on a bounded scope plus an attested silent pick; the `[precondition-ok:]`
  valve is named one paragraph earlier. What no surface says is whether a NEGATED phrase is an
  FP that calibration accepts or a miss it would fix — which makes this an authoring question
  about that paragraph as much as a code one.
  **Deliverable — rule one of three:** the trigger gains a negation lookbehind; or the
  calibration accepts the negated FP explicitly, on the phrase-set position other prose gates
  already take; or the valve is declared the intended answer and the calibration paragraph is
  widened to say so.
  **DISTINCT from a matcher-narrowing refusal already on the record**, found at this drain.
  lifecycle-kit/SPEC.md refuses narrowing the gap inbox's RECURRENCE matcher by exempting a
  negation, on the ground that an affordance which must be phrased around is miscalibrated.
  That refusal is about an ADVISORY prompt whose authority was the defect; this is a BLOCKING
  gate whose recall is. The same move has opposite value on the two tiers, and whichever
  surface is widened should say so.
  **Cost while deferred:** low and self-correcting — one round-trip per occurrence, and the fix
  is a rephrase — but it is a gate steering authors off a true sentence, and a rephrase leaves
  no trace, so the occurrence count is unmeasurable and the cost cannot rise into view.
  Filed 2026-08-24 to the gap inbox by spec immediately after the refusal it describes; drained and
  promoted 2026-08-24 at close, which reproduced the red against a probe corpus before
  dispositioning.

- **prune-set-convergence-question** [design-pending] — two kits carry two prune sets that now
  overlap heavily and differ deliberately, and no surface rules whether they should converge.
  **Both definitions read at this drain rather than cited.** context-kit's set
  (`scripts/context-config.sh`) and gate-sdk's (`gate-sdk/lib/gate.sh`) SHARE FOUR leaves and
  differ in exactly four: context-kit omits the scratch dir and the fixture-corpus dir, and
  carries two build-output dirs gate-sdk does not.
  **CORRECTS THE FILING**, which said the two share one leaf. One leaf is what they NEWLY share
  — the second-copy leaf this iteration added to both — and the shared core was already three
  deep before it. Read as a shared-set claim the filing understates the overlap fourfold, which
  matters because the size of the shared core is most of the argument for deriving it.
  **Deliverable:** rule whether the two sets converge, stay two, or share a derived core with
  per-kit extensions — the shape gate-sdk's extra-dirs knob already provides on one side.
  **Why it is not free even though it looks it.** Adding the scratch or fixture-corpus dirs to
  the index walkers is a corpus NARROWING with its own readers — the index-tests goldens, and a
  session that legitimately wants a fixture corpus indexed — so convergence costs coverage on
  the side that gains consistency.
  **Cost while deferred:** low, and correctness-free: no exposure, since the second-copy leaf
  that motivated the question is in both sets. The carry is two arrays a reader must not assume
  agree, and a later author who adds a leaf to one and not the other with nothing in the tree
  to catch it. Filed 2026-08-24 to the gap inbox by spec; drained and promoted 2026-08-24 at
  close, which re-read both definitions and corrected the shared-leaf count.

- **recurrence-threshold-counts-dates-not-incidences** [design-pending] — the pre-emption threshold
  reads a date count as an incidence count, so two incidences in one day count as one and the
  rule fires late.
  **The two halves, both read at this close rather than cited.** lifecycle-kit/SPEC.md's knob
  roster says the threshold is read against the DATE COUNT of an entry's `recurrence:`
  declaration, and glosses it as "two recorded re-filings is a third incidence of the same
  finding" — an incidence claim resting on a date count. The same file rules stamping
  **idempotent per (slug, date)**, and rules it deliberately, on the ground that the day is the
  only resolution the bullet's grammar has and that a finer one would claim precision the
  channel does not carry.
  **NOT A REVERSAL of that ruling, and deliberately not a proposal to date more finely.** The
  ruling governs what the CAPTURE channel may claim. This entry is about what a DOWNSTREAM
  READER may infer from what the channel recorded — which that ruling does not address and
  which no surface states.
  **The residual is measurable rather than theoretical, and this repo is its worst case.**
  Iterations here routinely close several to a calendar day — three on 2026-08-24 alone — so
  two independent incidences in two separate iterations collapse to one date. An entry at
  genuine incidence three can read as date count two or one, and the scope stage's pre-emption
  rule, which admits a threshold entry regardless of the standing directive's theme, simply
  does not fire.
  **Attested at this close rather than hypothesised.** Three entries were dispositioned against
  this exact edge here: one stamp taken, and two declined partly because the entry already
  carried, or would have had to carry, the same calendar day as its own filing.
  **Why `[design-pending]`:** the candidates differ in kind and none is obviously right. Read
  the count against an ITERATION named beside each date, which the grammar does not carry
  today; or state the limit on the threshold — a date count is a LOWER BOUND on incidences —
  and leave the number alone, which costs one line and buys no detection; or leave it and
  record that same-day collapse is accepted. The first mints grammar, the second is a hedge,
  the third is a ruling.
  **Cost while deferred:** the one rule that promotes work on evidence rather than theme
  under-reads its evidence, silently and in the direction of doing nothing, and no gate can see
  it — the count is a judgment record and is not re-derivable by any predicate. Filed
  2026-08-24 by close from its own surface triage rather than from the gap inbox, after an
  owner grep over the queue found the class unowned; both SPEC halves were read first-hand.

- **dated-measurement-restatement-class** [design-pending] — whether a DATED measurement of a
  live file's line spans is inside the internal-identifier-restatement class or outside it.
  **The instance that forced the question, named as an instance rather than re-filed:**
  `powershell-installer-surface` restates two live internal line RANGES of
  `installer/lib/init.sh` plus per-file line counts for four sibling scripts. The fifteenth
  `internal-identifier-restatement` sweep judged it LOW-MEDIUM and filed the call rather than
  taking it. That entry stays live and unchanged either way, and its subject — the uncovered
  install surface — is untouched by this one.
  **What makes it borderline rather than clear.** Every figure was verified EXACT at HEAD, and
  the paragraph is dated and headed "Measured ... not estimated", the form prior sweeps of the
  class have consistently cleared. Against that: a line RANGE is the most rot-prone form the
  class takes, it sits in the deferred pool — the longest rot window in the tree, and the corpus
  the class was widened to cover 2026-08-15 — and the install path is live code any installer
  edit moves.
  **Why it generalises past one entry:** the ruling reaches every "measured at <date>" paragraph
  in the pool, which is why the sweep escalated rather than editing one entry. The audit
  roster's own reading of the shape is the ground to rule against — a ruled-but-unbought port is
  its ideal host, because such a paragraph's whole job is to price a port against a source the
  author has read and the reader has not, and the anchor cannot yet exist.
  **The candidate disposition if the answer is INSIDE, narrow and reversible:** drop the two
  spans and the four parenthetical counts, keep the two order-of-magnitude figures the argument
  rests on, and append a re-derive instruction. If the answer is OUTSIDE, the roster row says so
  once and every later sweep stops re-deciding it.
  **Why `[design-pending]`:** it rules the boundary of a governed audit class, an authoring
  question about the roster row rather than a code change.
  **Cost while deferred:** the figures rot silently, and the entry carrying them is the one
  sizing an unbought port, so a reader prices the work off numbers nothing re-checks — while
  every future sweep spends the same judgment again on the same shape.
  Surfaced 2026-08-24 at the `port-remainder-disposition-and-worktree-reclamation` close's
  internal-identifier-restatement sweep and filed to the gap inbox there; drained and promoted
  2026-08-24 at this scope.

## Icebox

  Dormant entries, one line each: the cost field said the carry was low, no
  `[roadmap:]` commitment rides on it, and no named event is waiting to
  promote it. Still live work — a legal `[blocked-by:]` target, conserved on
  the way in and on the way back out. The removed body is recoverable from
  the evicting commit (`git log -p -S'<slug>' -- TASK-QUEUE.md`).

- **runtime-dir-two-tier-detector** [design-pending] — No two-tier proof for file-pattern ignores.
- **done-slug-commit-naming-gate** [design-pending] — Done-moving commits need not name their slug.
- **enter-stage-simulate-no-write-fixture** [design-pending] — Guard present, unpinned by a fixture.
- **stage-lag-disambiguation** [design-pending] — Hook over-firing is accepted, not a defect.
- **stage-economics-smoke-jq-arm-dormant** [design-pending] — Its jq-absent arm never runs anywhere.
- **hermetic-bin-roster-config** [design-pending] — Pinning coverage needs a consumer roster seam.
- **split-posture-waiver-writer** [design-pending] — A lead-issued waiver stamp has no writer.
- **supervisor-verification-attestation** [design-pending] — The verification duty is unattested.
- **assertion-strength-exit-header-reach** [design-pending] — Wider header reach may be ceremony.
- **gate-spec-claim-assertion-parity** [design-pending] — Ruled a human-audit class, not gateable.
- **scope-amendment-authoring-gate** [design-pending] — Scope can do spec's job and stay green.
- **evidence-journal-hash-chain** [design-pending] — Tamper-evidence wanted only by a hosted rung.
- **md-section-near-miss-match** [design-pending] — Empty on a near miss; correct on an exact query.
- **operator-authored-unit-set** [design-pending] — The contract omits operator-authored unit sets.
- **tarball-build-attestation** [design-pending] — The checksum proves transfer only; docs agree.
- **action-run-shell-scan-predicate** [design-pending] — No consumer seam on a correct gate.
- **scratch-execution-allowlist-bar** [design-pending] — Each close re-derives this standing bar.
- **gate-tamper-consumer-gate-coverage** [design-pending] — A glob and a roster audit remain.
- **upgrade-contract-rename-routing-unstated** [design-pending] — One clause leans on it.
- **md-refs-tree-link-resolution** [design-pending] — Unreachable while one generator produces.
- **advisory-lane-draft-state-unswept** [design-pending] — GitHub's notifications are the sweep.
- **amendment-done-move-assertions** [design-pending] — Zero cost while merges are hand-checked.
- **guard-advise-jq-dependency** [design-pending] — Needs jq; the one consumer works around it.
- **survey-record-extension-tier-hybrid** [design-pending] — Paid only by a future workflow author.
- **install-lifecycle-reversibility** [design-pending] — A declined branch; only optionality owed.
- **pack-installer-payload-kit-set-anchor** [design-pending] — Latent --root trap, no caller.
- **drift-kpis-default-two-homes** [design-pending] — Two literals, one string, never diverged.
- **installer-jq-usability-probe** [design-pending] — Broken-but-present jq is unobserved.
- **rendered-site-link-monitor** [design-pending] — Rendered-site link rot waits on a launch crawl.
- **kit-index-page-vocabulary-ungated** [design-pending] — Index-page enums are ungated.
- **absence-statement-grammar** [design-pending] — When to state absence, and how, is unruled.
- **contributor-writeback-disposition** [design-pending] — Write-back is dormant pre-launch.
- **context-pressure-signal** [design-pending] — Compaction timing has no per-session signal.
- **readme-roster-enum-coverage** [design-pending] — Prose rosters drift outside every parity gate.
- **post-immutability-machine-read-carveout** [design-pending] — Immutable prose, live machine read.
- **path-pinned-allow-entry-oracle** [design-pending] — No scanner reds a path-naming grant.
- **price-table-roster-coverage-oracle** [design-pending] — An unpriced model id reds nothing.
- **economics-posture-binding-stale** [design-pending] — A shim restates a ruling it should cite.
- **align-context-draw-growth** [design-pending] — Two falls read the draw as work-side.
- **customer-facing-iteration-cadence** [design-pending] — No tracked classifier for the bound.
- **scan-prompts-truncation-quote-desync** [design-pending] — Truncation inflates the scan only.
- **template-out-of-tree-copy-obligation** [design-pending] — Out-of-tree copies are unreachable.
- **queue-entry-grammar-single-owner** [design-pending] — Two entry grammars disagree, latently.
- **installer-artifact-omission-residue** [design-pending] — An omission update strands a binary.
- **doctrine-rule-number-citation-liveness** [design-pending] — A renumber stales citations.
- **false-ground-citation-propagation** [design-pending] — Nothing re-reads a ground once cited.
- **amendment-roster-omission-detection** [design-pending] — Only a grep catches a short roster.

## Done

## Lessons Learned

