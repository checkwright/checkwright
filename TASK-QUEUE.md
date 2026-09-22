# TASK-QUEUE.md — Checkwright work queue

## Iteration: queue-kit-unwrap

  The lifecycle-kit gates read this header's iteration name and the stage
  cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt`
  (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue
  format itself and gates this file. One iteration per hardening or roadmap
  unit; [README.md](README.md) maps the kits.

---

## New Features

## Technical Debt

- **queue-backlog-vocabulary-undeclared** — the tree uses "queue" for the governed file and its
  drain mechanics (`--emit queue-counts`, the drain-entry assertion, `QUEUE_KIT_ENTRY_LINE_CAP`)
  and "backlog" for the accumulating mass and its aging (`kpi-gate-backlog`, the backlog-aging
  findings), and no doc states which word owns which half.
  **Re-verified at this scope:** queue-kit/SPEC.md and lifecycle-kit/SPEC.md still carry usages
  only, no definition. **Deliverable:** one section in queue-kit/SPEC.md declaring the split and
  its ground. It salvages an operator-declined rename of the queue file (operator direction,
  2026-09-18): collapsing the two words would cost the live distinction and leave
  `kpi-gate-backlog` naming something other than the backlog, and the path was always consumer
  config (`QUEUE_KIT_QUEUE_FILE`).
  **Cost while deferred:** each reader infers the distinction, and a rename proposal can recur.
  Filed 2026-09-18 to the gap inbox by the lead after `external-install-evidence`'s close;
  promoted 2026-09-22 into `queue-kit-unwrap` by operator direction (lead-relayed). Debt: it
  declares two names the tree already carries and mints none.

## Deferred

- **shell-textual-absoluteness-single-dialect** [cost: event/low] [surface: gate-sdk] — tracked
  shell sites test a path for absoluteness by a leading `/` alone, so a caller-supplied Windows
  drive-letter path is joined onto a root as if relative. Sites (re-measured 2026-09-22):
  `gate-sdk/lib/test-hermetic.sh`, `gate-sdk/smoke/install.sh`, `installer/bin/checkwright.sh`,
  `installer/consumer-smoke/run-smoke.sh`, `lifecycle-kit/smoke/install.sh`,
  `guard-kit/lib/guard.sh` and `guard-kit/gate-tests/guard-lib-parity.test.sh`.
  Probe: `git grep -n -E '== /\*|/\*\)' -- '*.sh'`.
  **Deliverable:** one shell absoluteness helper both dialects satisfy, the sites routed through it,
  and the shell half of `check-path-dialect`'s locality arm (today crate-only) holding it.
  **Cost while deferred:** a wrong root on a Windows adopter host whenever a caller exports a
  drive-letter path. Filed 2026-09-21 to the gap inbox by gate-sdk-surface-drain's spec session;
  promoted at its close because →fix needs a new shell-side oracle the cwd-anchor amendment kept
  out of its envelope. Owner lookup ran over `absolute` and `locality` in gate-sdk/SPEC.md;
  §check-path-dialect owns the crate-side arm and names no shell arm.

- **armed-by-content-emptiness-shape** [cost: event/low] [surface: gate-sdk] — `check-commit-msg`
  and `check-tree-terms` share `GATE_SDK_MSG_PATTERN_FILES`, a required tracked pattern file, and
  both pass vacuously when that file trims to no non-comment line. `# armed-by:` reads a knob's
  resolved value, never a tracked file's parsed content (gate-sdk/SPEC.md §The install
  disposition), so `doctor` cannot name this disarmed state. **Deliverable — rule one of two:** an
  arming shape reading content emptiness, or a stated refusal naming why a content-emptied pattern
  file stays outside `doctor`. No design is proposed here.
  **Second instance, same limit:** `check-provenance-seam` asserts nothing at the defaults
  (`CANON_KIT_SCAN_KIT_ROOTS=0` with `CANON_KIT_SEAM_SURFACE_GLOBS` empty), a conjunction one
  knob's emptiness cannot state; its dead `armed-by:` on the `0|1` switch came off at
  canon-gate-precision's close. The ruling covers both shapes.
  **Cost while deferred:** an adopter who empties the pattern file gets a silent clean pass with no
  doctor warning, and a default provenance-seam install gets one too. Filed 2026-09-22 to the gap
  inbox by the census discharging
  `armed-by-census-unrun`; promoted at close because →fix needs a new declaration shape. Owner
  lookup ran over `armed-by` in gate-sdk/SPEC.md; §The install disposition owns the declaration.

- **build-work-before-entry-stamp** [cost: event/low] [surface: lifecycle-kit] — a stage session
  can land its work commits before running its own `--enter-stage`, and nothing reds it:
  `check-stage-evidence` catches a stamp committed after the work it preceded, never work preceding
  the stamp, and a same-stage re-entry finds the cursor already on its stage. Measured 2026-09-22 by
  `git log`: canon-gate-precision's second build session merged two amendments, then stamped at the
  second merge's head. The first instance, one iteration earlier and also on the cheaper tier, was
  discarded at scope because §check-stage-evidence states the limit; the recurrence shows that
  stating the limit did not stop it.
  **Deliverable — rule one of two:** an oracle asserting that a session's work commits follow its
  own entry stamp (inferred: commits carry no session id, so how the oracle ties a commit to a
  session is the open question), or a stamp-first line carried in the dispatch path the cheaper
  tier reads. **Cost while deferred:** the stamp's recorded head postdates the stage's work, so
  the stamp misplaces where that session's work began. Filed 2026-09-22 to the gap inbox by the
  canon-gate-precision lead as a recurrence judgment; promoted at its close because →fix needs a new
  oracle or a dispatch-path ruling. Owner lookup ran over `stamp` and `newly introduced` in
  lifecycle-kit/SPEC.md; §check-stage-evidence owns the assertion and its limit.
  recurrence: build-work-before-entry-stamp 2026-09-22

- **doc-path-hardcoded-reads** [cost: event/low] [surface: gate-sdk] — eighteen kit-shipped
  `couples=` literals across thirteen members name a consumer path the member reads that no knob it
  declares configures, so each trigger stays frozen to this repo's layout and a vendoring adopter's
  hook never fires on their real file. Members (re-measured 2026-09-22 by
  `git grep -n couples= -- '*/checks/*.gate'`): the four `check-action-*` gates
  (`docs/_config.yml`), `check-install-claim`, `check-payload-claim`, `check-prose-tells`
  (`docs/*.md`), `check-close-surfaces` (`.claude/commands/*.md`), `check-scratch-citation`
  (`TASK-QUEUE.md`), `check-footprint-fresh`, `check-enforcement-fresh`, `check-surface-ratchet`,
  `check-graph`, `check-surface-duplication` (`VISION.md`) and `check-producer-liveness`
  (`.tmp/*.run`).
  **Deliverable:** per member, a knob the code reads and the registry declares (the literal then
  converts by gate-sdk/SPEC.md §The # graph: manifest's rule), or an undeclared read declared.
  **Cost while deferred:** those triggers stay pinned to this layout in every vendored tree.
  Filed 2026-09-22 to the gap inbox from the doc-path-tokens survey's class 4; promoted at close
  because each member's module needs reading. **DISTINCT from**
  **`check-graph-trigger-consumer-path-reach`** (icebox), whose subject is a `couples=` missing
  `installer/`, not a frozen consumer literal.

- **pipe-membership-corpus-omits-test-suites** [cost: event/low] [surface: gate-sdk] —
  `check-pipe-membership` reads `walk::tracked_shell_tree`, which excludes `*.test.sh`, so the
  tracked suites setting `pipefail` sit outside the gate. Measured 2026-09-22 by the filer: 107 of
  111 tracked suites set `pipefail`; one of the five historically fixed membership sites was a
  suite, `gate-sdk/gate-tests/lib-gate.test.sh`. **Deliverable:** widen the corpus to tracked
  `*.test.sh`, still excluding the pruned fixture trees (their `bad/` cases hold deliberate
  violations), and restate gate-sdk/SPEC.md §check-pipe-membership's corpus sentence; the landing
  amendment's own probe over all `*.sh` finds no current site, so the widened gate lands green.
  **Cost while deferred:** a new membership pipe written in a suite goes unflagged until it flips a
  verdict under load. Filed 2026-09-22 by build batch 4 of gate-sdk-surface-drain on a lead ruling;
  promoted at close because §check-pipe-membership states the suites outside its corpus, so the
  widening is an envelope change for spec, not a drain fix.

- **fence-run-fixed-env-hides-user-gem-dir** [cost: event/low] [surface: canon-kit] — site-kit's
  fixture-suite fence stays unmarked for `check-fence-run`, so the command an adopter pastes from it
  runs unwitnessed. canon-kit/SPEC.md §check-fence-run pins `HOME` inside the scratch and inherits
  only `PATH`, and a Ruby user install keeps `kramdown-parser-gfm` under `HOME`'s user gem dir, so
  the suite finds no gems there. **Measured 2026-09-21:** `gem env gempath` lists the user gem dir
  first and the only kramdown gems sit there.
  **Deliverable — rule one of two:** a declared, consumer-named environment passthrough in the fixed
  environment (a `GEM_PATH`-shaped knob, stated beside the proxy honest limit), or a stated refusal
  naming why a toolchain resolving through `HOME` stays outside fence execution. Then mark the
  fence.
  **Cost while deferred:** one adopter-facing command unwitnessed; the render-fidelity gate itself
  still runs in the battery. Filed 2026-09-21 to the gap inbox by build batch 3 of
  `docs-first-contact`; promoted at its close because →fix needs an envelope change to the fixed
  environment. Owner lookup ran over `fixed environment`, `HOME` and `GEM_` in canon-kit/SPEC.md;
  §check-fence-run item 4 owns the variable list and names no passthrough.
  **DISTINCT from `fence-execution-gate` (landed)**, which built the gate this member cannot reach.

- **queue-citation-line-number-stales-within-its-own-session** [cost: event/low] [surface: queue-kit]
  — a `path:line` cite in a queue body goes stale when the cited file changes above the line, and
  nothing reads it; stale on write when the citing session edits that file itself.
  **Recurred 2026-09-21, across iterations:** `shell-cwd-anchor-clause-has-no-oracle` cited
  `gate-sdk/lib/test-hermetic.sh` at lines that moved +4 when `bespoke-test-path-knob-pinning`
  inserted a block above them; corrected by hand grep at `scratch-hermeticity`'s close.
  **Why not obviously gateable:** resolving a cite to its referent is semantic; the decidable half
  is a cite whose file changed above the cited line since the cite landed. The no-gate
  alternative: cite by content, never by number.
  **Cost while deferred:** each instance costs a later reader a wrong resolution until a hand grep.
  Filed 2026-09-08 by close from the gap inbox and iceboxed; returned to Deferred 2026-09-21 at
  this scope's intake on the real recurrence above (gap bullet dated 2026-09-21, filed after
  `scratch-hermeticity`'s close). Body before eviction: `git log -p -S'<slug>' -- TASK-QUEUE.md`.
  **DISTINCT from `docs-cmd-retired-path-blind-to-queue`**, where the whole cited path is retired.
  Feature — a gate or a citation rule either way; `/spec` authors and promotes it. Operator
  direction 2026-09-22 (lead-relayed): into `queue-kit-unwrap`; heading anchors cover queue
  records only.
  recurrence: queue-citation-line-number-stales-within-its-own-session 2026-09-21

- **recurrence-line-never-ages** [cost: event/low] [surface: queue-kit] — a dated `recurrence:`
  line counts as a live icebox trigger with no age limb, so one recurrence pins a low-cost entry
  out of the icebox indefinitely, even after later re-measurements fail to reproduce it.
  queue-kit/SPEC.md §The icebox tier lists a dated `recurrence:` line among live triggers and ages
  only the entry itself (`QUEUE_KIT_ICEBOX_AGE_DAYS`), never the recurrence date.
  **Attested once:** `worktree-isolated-agent-report-lost-to-a-failed-peer-send` recurred
  2026-08-26, did not reproduce 2026-09-16 or 2026-09-21, was re-costed `event/low`, and stayed
  Deferred on that one date until `delegation-seams` landed it. The instance, not a recurrence.
  **Deliverable — rule one of two:** an age limb on the recurrence date mirroring the icebox age
  knob, or a stated refusal naming why a single recurrence stays live forever.
  **Cost while deferred:** each close's eviction re-judges such entries by hand and keeps them.
  Filed 2026-09-21 to the gap inbox at `release-declaration-coupling`'s close eviction; promoted at
  the next scope's intake the same day. Owner lookup ran over `recurrence:`, `live trigger` and
  `age limb`; §The icebox tier owns the trigger list and states no age rule for it.
  Feature — an age limb mints a knob and a refusal narrows the tier's contract; `/spec` authors and
  promotes it. Operator direction 2026-09-22 (lead-relayed): into `queue-kit-unwrap`.

- **disclaimer-beside-its-own-restatement** [cost: event/low] [surface: canon-kit] — a surface
  that disclaims carrying a rule ("stated there and not restated here") in the same sentence that
  carries it is asserted by nothing, and the disclaimer tells every sweep the copy is not one.
  **Attested once, fixed inline:** README.md §This repo, governed restated the commit-time
  fixture-suite selection rule beside exactly that disclaimer, so correcting CLAUDE.md stranded
  README's copy; `5e0e10f8` de-literalized it. What survives is the class, not the instance.
  **Why reachable when general restatement is not:** the predicate is a disclaimer phrase
  co-located with a content clause — does the sentence around it name the rule's substance
  rather than only its owner. `check-surface-duplication` and `check-shim-restatement` hold
  restatement for their own corpora; neither reads a disclaimer.
  **Deliverable:** a gate, or an assertion joining an existing restatement gate, over that shape.
  A feature by the new-names litmus, so it owes an amendment and passes the enhancement
  admission filter only on an arm its authoring session argues.
  **Cost while deferred:** each such disclaimer is a licence a later reader trusts, and the copy
  beside it rots silently.
  Filed 2026-09-20 to the gap inbox by the close of `adopter-floor-door-remainder`; promoted
  2026-09-21 at the next scope's intake, so the record is late and says so. Owner lookup ran over
  `disclaim`, `not restated here`, `restatement` and the two gates above and found no owner.

- **lead-no-change-decision-has-no-landing-site** [cost: once/low] [surface: lifecycle-kit] — a
  lead decision whose content is "make no change" is invisible to every later stage, because the
  landing rule presupposes a ruling with something to write.
  **Attested once:** a scope escalated whether a recurrence date was owed; the lead declined it
  and told scope nothing further was owed, on the ground that scope's gap bullet carried the
  grounds. That bullet recorded the question as escalated and unresolved, so close re-derived the
  judgment from the owner doc three stages later. Same disposition, one re-derivation — but the
  opposite call would have tripped the recurrence threshold and forced an entry into a unit set,
  a queue change only the lead may rule.
  lifecycle-kit/templates/lead.md already calls the message thread transport and already requires
  a ruling whose acting session is not imminent to be filed durably; both clauses assume an act
  that writes something.
  **Deliverable — rule one of two, neither authored:** a no-change decision appends its
  resolution to the gap bullet that raised it, so the drain reads a closed question; or the lead
  template states that "nothing further is owed" is never the lead's to assert about a durable
  record it did not read. The lead journal is scratch and is not the answer.
  **Cost while deferred:** the next no-change ruling on a threshold-bearing question is
  re-derived by a session that may not rule it.
  **DISTINCT from `record-stamp-encoding-compression` and `precondition-gate-direction-blindness`**,
  the entries the instance happened on; this is the lead protocol's landing rule.
  Filed 2026-09-20 to the gap inbox by the lead of `adopter-floor-door-remainder`; promoted at
  the 2026-09-21 scope intake, a late record that says so. Owner lookup ran over
  `no-change`, `landing site`, `lead decision` and `transport` and found no owner.

- **entry-line-cap-has-no-line-axis-relief** [cost: event/low] [surface: queue-kit]
  — an entry at `QUEUE_KIT_ENTRY_LINE_CAP` buys room for a mandated write by
  compressing prose a later reader needed, and nothing offers relief on the LINE
  axis or rules that the trade is intended.
  **Re-measured at this close off `check-queue-entry-budget`'s verbose headroom
  read, and the squeezed set has GROWN since the filing:** FOUR entries sit at
  exactly 0 lines of headroom — `record-stamp-encoding-compression`,
  `companion-toolkit-profile`, `heterogeneous-agent-delegation` and
  `worktree-isolated-agent-report-lost-to-a-failed-peer-send` (landed since) — with two more at
  1 and four at 2. The 2026-09-20 scope bullet that witnessed it named three at
  0, one at 1 and one at 2.
  **The witness is first-hand:** that scope had to land an operator-ruled
  admission filter onto an entry at 0 headroom and compressed three load-bearing
  paragraphs to make room, so the ruling shipped at three lines rather than the
  two paragraphs its own gap bullet had costed.
  **DISTINCT from `record-stamp-encoding-compression`, cited here rather than
  re-filed.** That entry names both axes itself and its remedy is re-encoding
  record STAMPS — which frees columns in stamp lines, never five lines of
  paragraph prose. Its column axis is witnessed three times; its line axis had no
  witness until 2026-09-20. No `recurrence:` date joins it: today's instance is
  in its CLASS and outside its REMEDY, a neighbouring shape in the same family,
  which files as a new finding on the same discrimination
  `precondition-gate-direction-blindness` applies to its own sibling.
  **DISTINCT from `icebox-eviction-line-budget-squeeze`** (the icebox tier's
  one-line grammar against the column cap) and from
  `lead-line-blocked-by-spec-tag-width-collision` (a lead line's COLUMN width).
  **Deliverable — rule one of two, neither authored:** a line-axis relief the
  owner entry does not propose (a discount class, a relocation rule, or a cap
  read against entry class); or a recorded ruling that compression at the cap IS
  the intended trade, so a squeezed session stops reading it as a defect.
  **Cost while deferred:** every mandated write onto an entry at the cap pays in
  prose, and the payment is invisible afterwards — the entry reads as though it
  was always that terse.
  Filed 2026-09-20 by close's drain as the second-order half of that scope
  bullet; owner lookup ran over `line cap`, `headroom`, `entry budget` and
  `squeeze` across the pool and returned the three entries distinguished above
  and no owner.
  **Reshaped by operator direction 2026-09-22 (lead-relayed), into `queue-kit-unwrap`:** unwrapped
  entries have no line axis, and the hard-wrap entry re-units the cap to code points; the residue is
  whether compression at that cap is the intended trade. `/spec` discharges or authors it.

- **stage-evidence-prefix-doubles-a-separator** [cost: event/low] [surface: lifecycle-kit]
  — `check-stage-evidence`'s path relativizer composes its not-under arm from
  `git rev-parse --show-prefix`, which prints a TRAILING slash, onto a path with
  a separator already between them.
  **Latent rather than live, and that is why it is filed rather than fixed:**
  the doubled separator is collapsed downstream by `walk::normalize_abs`, so no
  verdict is wrong today. Removing the second separator is a behaviour change to
  a COMPARED value, which wants its own measurement rather than a drive-by edit
  at a close.
  **Deliverable:** measure what the compared value is on each arm, then either
  trim the prefix at the site or state at the site that the collapse is relied
  on.
  **Cost while deferred:** none paid today; the exposure is a later edit to
  `normalize_abs`'s collapse turning a latent defect live with nothing asserting
  the coupling.
  Filed 2026-09-20 by close's drain, noticed at the build that routed the site's
  prefix strip onto `walk::rel_under` and left as found; owner lookup ran over
  `show-prefix`, `separator` and `stage-evidence` across the pool and returned
  no owner.

- **delta-instruction-batch-dependence-unmarked** [cost: once/low] [surface: lifecycle-kit]
  — a spec amendment can state a delta instruction in the unconditional voice
  when its satisfying act depends on which units share the build BATCH, and the
  grammar has no way to say so.
  **Attested twice in one iteration, both door amendments:** each carried
  "exactly two of the 59 door sites are adopter-facing and are deliberately left
  red, being the paired debt entry's deliverable" — true only of a batch
  EXCLUDING that debt unit. The lead batched the debt unit IN on a
  producer/consumer read, at which point the correct act was to REPAIR those two
  sites.
  **Measured, not projected:** the dispatched session reported it would have
  left them red on the amendment's letter, and only an explicit prompt-side
  inversion by the lead prevented it — which is prompt-side and dies with the
  lead. A later session reading the merged amendment alone reaches the wrong act
  and calls it correct.
  **Deliverable — neither candidate authored:** a delta-authoring rule that a
  batch-dependent instruction names its condition; or a spec-stage check that a
  delta citing a sibling unit's deliverable states what happens when both land
  together.
  **DISTINCT from every live entry**, and the owner sweep over `delta`,
  `amendment`, `batch` and `unconditional` found none:
  `amendment-refusal-acceptance-parity` concerns an amendment's REFUSAL
  rationale rather than its instruction voice, and
  `amendment-dod-sibling-dependence` concerns a DoD item depending on an unnamed
  sibling rather than a delta's voice.
  **Cost while deferred:** once/low to rule, plus whatever the rule costs to
  gate; until then every batch-dependent delta needs a lead present to invert
  it.
  Filed 2026-09-20 by the iteration lead to the gap inbox at the build batch-2
  dispatch, where inverting the instruction was the act that surfaced it;
  drained and promoted 2026-09-20 at close.

- **probe-before-assertion-doctrine** [cost: event/low] [surface: doctrine-kit]
  — **returned from the icebox on a judged recurrence.** The rule it asked for shipped:
  CLAUDE.md §Delivery doctrine carries `Probe-before-assertion`, and the icebox line recorded the
  remainder as "Rule shipped; mechanizing it is open". What re-fires it is that the shipped rule did
  not hold, and did not hold seven times in one iteration.
  **The seven, all one shape — a probe narrower than the claim it supported.** Three path-scoped
  greps; two filters whose empty return was read as success; an amendment whose printed probe
  over-returned; and a nested-tree probe whose first confirming run used an inconsistent tree. Two
  of the seven produced regressions that reached validate, where they cost a re-tier.
  **The sharpening is the finding, and it is new.** `Probe-before-assertion` asks whether a probe
  was *run*; in all seven a probe ran. What failed is that its **corpus was narrower than the corpus
  the claim ranged over**, which the rule as worded does not reach. The drain reproduced the shape
  on its own inputs: the `door-sweep-reach-stops-at-the-kit-boundary` bullet named nine
  `docs/site-architecture.md` sites where a fresh grep returns thirteen, and missed `docs/index.md`
  altogether.
  **Why design-pending:** whether this is a fourth always-loaded line, a re-wording of the existing
  one, or something mechanizable is the open call — and it is a governed-surface widening, which a
  close does not self-serve. The cheapest candidate worth beating is still the one the original
  filing named: widen an existing rule rather than mint another.
  **Cost while deferred:** measured at seven misses in one iteration, two of them reaching validate.
  recurrence: probe-before-assertion-doctrine 2026-09-20
  Filed 2026-08-07 by close as that iteration's candidate lesson; iceboxed in the machinery-class
  triage slice; returned to the deferred section at `door-binding-sweep`'s close drain on the
  recurrence above. Owner lookup: `probe`, `premise`, `unverified`, `corpus narrower` — this entry,
  plus two **adjacent and distinct** icebox members: the cited-object token sweep's corpus (a
  *gate's* corpus) and `dispatched-child-asserts-an-unverified-base` (a child inheriting a base it
  never probed).

- **inferred-marker-malformed-placement-passes-unseen** [cost: event/low] [surface: lifecycle-kit]
  — `check-stage-entry` assertion D reads an inferred marker only where the full spelling opens a
  physical line (`inferred_marker` in `native/src/gates/stage_entry.rs`), so a marker an author
  places mid-line, or whose spelling a hard wrap splits, is invisible and its unrun claim passes the
  build entry. Both attested instances were mid-line: two of three markers in a
  declined-target-audit amendment, found only because the third, well-formed one redded
  `--simulate build`. A line-start marker whose reason wraps to the next line already reds (empty
  reason), so that case fails closed.
  **Re-verified at the drain:** the unit test `a_marker_is_read_only_at_line_start` passes and
  asserts a mid-line `**Inferred, not run:**` reads as `None`; a line opening with a split spelling
  fails both `strip_prefix` arms.
  **Why design-pending:** lifecycle-kit/SPEC.md §templates/stages/ rules a mid-line mention prose,
  not a marker, and the test pins it. Refusing a bold unbackticked mid-line spelling as a malformed
  marker narrows that rule, and prose mentions in an amendment would red.
  **Cost while deferred:** a misplaced marker's claim reaches build unrun, and only the author's own
  line-start discipline holds.
  Filed 2026-09-18 to the gap inbox at declined-target-audit's align; promoted at its close drain.
  Owner lookup: `inferred marker`, `inferred_marker`, `scan_markers`, `malformed marker` — none.

- **guard-declares-class-correspondence-ungated** [cost: event/low] [surface: guard-kit] — items
  in guard-kit/SPEC.md §The generic ruleset state the inert quoting classes their rule's
  `guard_skeleton` call strips (`Declares sq dq hd`), transcribed by hand, and nothing holds the two
  equal: an edited skeleton call silently stales its SPEC declaration.
  `check-guard-registration` holds roster, definitions and dispatch only, and the class
  correspondence stays outside it because a rule has no single value — rule 6 builds two skeletons,
  rule 18 four, some rules skeletonize inside a helper, and only some items declare. Candidate: a
  declaration shape each skeleton call can satisfy per member (one declaration per call, keyed to
  the call), then a fourth-roster assertion over it.
  **Re-verified at the drain:** the `Declares` lines sit in the section (e.g. guard-kit/SPEC.md
  rules at lines 681, 742, 759) and §check-guard-registration's assertions A-D name no class
  correspondence.
  **Why design-pending:** the declaration shape is a SPEC-grammar change across every declaring
  item plus a gate extension, a spec-stage amendment rather than a drain fix.
  **Cost while deferred:** a skeleton edit leaves a declaration that misstates what a quoted
  mention does; a reader, not the guard, pays.
  Filed 2026-09-18 to the gap inbox during `guard-ruleset-registration-lockstep`; promoted at its
  close drain. Owner lookup: `guard_skeleton`, `Declares`, `inert class`, `skeleton call` —
  none.

- **residency-roster-template-reach-ungated** [cost: event/low] [surface: context-kit] —
  nothing asserts the roster↔template relation context-kit/SPEC.md §The consumer footprint rules:
  an obligation whose bound actor is any session is carried at the consumer's resident tier and is
  **not** also restated in the kit templates those sessions load, with one corollary permitting a
  template statement where the reader's discharge differs and never the channel or disposal-time
  half alone. The ruling landed this iteration and no oracle reads it, so the next template shipping
  an unsanctioned restatement — or an obligation losing its only carrier — reds nothing.
  **Premise corrected at the drain that promoted this:** the filing bullet credited the no-gate
  claim to delegation-kit/SPEC.md §Operative residency. That section makes no such claim; it says
  the opposite ("never a substitute for an oracle where one is buildable"). The claim sits in
  §Verify after every agent commit, whose honest-limit paragraph asserts §Operative residency
  owes no gate "the same structural reason" — a supervisor's choice of command leaving no tracked
  artifact to read. That ground does not reach this defect, which is a grep over tracked files, so
  **that sentence is what narrows** if this lands.
  **Why design-pending:** the assertable direction is the open question. The ruling forbids the
  per-template copy, so the negative direction (no template restates a rostered resident obligation)
  is the one the ruling supports, while the bullet as filed named the positive one (each rostered
  obligation reaches the templates of the readers it binds). Which the corollary's exception can be
  expressed in, and whether a grep can tell a sanctioned discharge-differs statement from a
  restatement, are the two calls. **The worked hard case, found by this close's instruction-tier
  sweep:** `lifecycle-kit/templates/lead.md`'s journal-disposal block states the kfric obligation's
  *timing* half and names no channel, which the corollary permits here only because the lead also
  loads the resident line that does. Any oracle must rule that CLEAN; ruling it a violation is the
  false positive that would make the check unusable.
  **Cost while deferred:** the placement ruling can rot exactly as the obligations it corrected did,
  with no oracle to catch the next template that ships unserved or doubly served.
  Filed 2026-09-17 to the gap inbox at build (gate candidate declined under operator direction,
  2026-09-17, lead-relayed), promoted at this iteration's close drain. Owner lookup: `Operative
  residency`, `consumer footprint`, `footprint`, `residency`, `obligation` — none.

- **baseline-suite-coverage-arm-one-directional** [cost: event/low] [surface: evidence-kit] —
  evidence-kit/SPEC.md §check-evidence-baseline says the suite-coverage arm closes the failure of a
  suite "silently ceasing to run — dropped from the roster, or renamed under a config edit". The arm
  asserts one direction only: every `EVIDENCE_KIT_SUITES` entry carries a baseline row. A *rename*
  is caught, because the new name is rowless; a *pure drop* is not, because nothing reads a row
  whose suite left the roster. So the prose claims an enforcement the gate does not deliver.
  **Attested, and the counterexample is in the tree:** `.workflow/validate-baseline.txt` still
  lists `budget_guard_tests` and `dispatch_guard_tests`, whose producers
  `delegation-kit/bin/run-{budget,dispatch}-guard-tests.sh` were deleted at `7a4da575`
  (2026-08-31, their coverage absorbed into `native_crate`). The battery has been green over them
  for sixteen days.
  **Probed at promotion, not inferred:** `native/src/gates/evidence_baseline.rs:314-327` iterates
  the configured suites and never the rows; `.workflow/validate-evidence.txt` carries no line for
  either name, so a row removal would stale no recorded evidence.
  **DISTINCT from `evidence-baseline-orphan-suite-row`** (icebox), whose subject is the unread rows
  themselves — the instance this entry's fix would catch — and which this drain re-observed rather
  than re-fired. **Adjacent to `gate-spec-claim-assertion-parity`** (icebox, ruled a human-audit
  class): this is one concrete instance with a mechanical oracle, not that class.
  **Why design-pending:** whether the reverse assertion is a row-level red or an advisory, and how
  it interacts with the declared no-suites early-out, are the two calls; the SPEC sentence quoted
  above narrows in the same unit either way.
  **Cost while deferred:** a baseline surface whose whole job is a held-constant comparison keeps
  rows that assert nothing, and the next suite retirement leaves another pair the same way.
  Filed 2026-09-17 at this iteration's close drain, on re-verification of the bullet the lead filed
  from validate's observation. Owner lookup: `EVIDENCE_KIT_SUITES`, `validate-baseline`,
  `suite coverage`, `evidence-baseline`, `orphan` — matched `evidence-baseline-orphan-suite-row`,
  read and ruled distinct above.

- **guard-read-path-windows-unexercised** [cost: event/low] [surface: .github] — no `gates.yml`
  step runs guard-kit's `gate-tests/guard-read-path.test.sh` (its verbatim-bytes and no-added-CR
  assertions) on a Windows leg: fixture suites run only in the Linux `gates` job.
  `floor-jq-guard-lib` deleted the `_guard_lf` CR strip on the ground that those reads return bytes
  verbatim, so the Windows half of that claim is unproven.
  **Re-verified at the drain:** `--run-gate-tests` appears once in `.github/workflows/gates.yml`,
  in the `gates` job; gates run 35455364409's `install-smoke-windows` log has no hit for the test.
  **Why promoted, not fixed:** the fix is a Windows workflow step whose outcome is unknown until a
  Windows run. A red there reopens a landed unit, and the close's one remaining push cannot
  absorb it.
  **Cost while deferred:** if a Windows read adds CR, guard rules misread there, and no leg
  shows it.
  Filed 2026-09-19 by the lead at `adopter-floor-native-rungs`' build; promoted at its close drain.
  Owner lookup: `guard-read-path`, `_guard_lf`, `Windows leg` — none.

- **guard-powershell-tool-unguarded** [cost: event/high] [surface: guard-kit] — on native
  Windows the harness's `PowerShell` tool is on by default beside `Bash`, and guard-kit's hook
  matches `Bash` alone, so a PowerShell-tool call is neither steered nor logged to the friction log.
  Widening the matcher is no fix: every generic rule reads bash grammar.
  **Probed at filing:** the harness tools reference names the tool `PowerShell` and advises
  matching `Bash|PowerShell`; `grep -rn -i 'powershell tool' guard-kit` finds nothing before
  guard-kit/SPEC.md §The hook on native Windows, which states the bypass as an honest limit.
  **Why design-pending:** a PowerShell-grammar guard needs its own skeleton and splitter, rules
  that model PowerShell rather than bash, and a decision table on a Windows leg. None of that is
  designed, and whether a consumer-rule seam belongs on it is open.
  **Cost while deferred:** on a Windows host a command routed through the PowerShell tool
  bypasses every steer and block, and the close-stage triage never sees it.
  Filed 2026-09-18 at `windows-adopter-path`'s spec by operator direction (lead-relayed), when
  `guard-hook-windows-substrate` was designed. Owner lookup: `PowerShell tool`,
  `USE_POWERSHELL_TOOL`, `Bash|PowerShell` — none.

- **stamp-subject-merge-carve-out-unruled** [cost: event/low] [surface: lifecycle-kit] —
  `check-stamp-subject` reds a commit adding stamp lines under git's own `Merge …` subject, which
  has no scope to parse (lifecycle-kit/SPEC.md §check-stamp-subject, honest limits), while
  `check-commit-subject` admits `Merge `, `Revert ` and `fixup! ` / `squash! ` as carve-outs never
  to be reworded (gate-sdk/SPEC.md §check-commit-subject). A consumer merging branches that carry
  state-file stamps meets two gates whose remedies conflict.
  **Why design-pending:** the remedies are (a) exempt git-generated subjects in
  `check-stamp-subject` on `check-commit-subject`'s precedent, or (b) keep the red and name the
  merge-time remedy, a scoped subject supplied with `git merge -m`. Either narrows or holds the
  landed envelope, so an envelope ruling is owed at this entry's scope.
  **Inferred, not run:** that `git merge -m` with a scoped subject clears both gates on a
  stamp-carrying merge.
  **Cost while deferred:** none here, where internal work commits direct to master; a
  multi-operator consumer's stamp-carrying merge is blocked until reworded.
  Filed 2026-09-19 to the gap inbox at `lifecycle-contract-drain`'s build, promoted at its close
  drain by lead decision. Owner lookup: `stamp-subject`, `merge subject`, `git-generated` —
  no entry matched.

- **config-variant-battery-harness** [cost: event/high] [surface: gate-sdk] — nothing shipped lets a customer run the
  battery under a named config-seam variant and see what changes; the fixture pairs prove each
  gate's arm against fixed trees, and the smoke scripts are this repo's harness legs.
  **Operator ruling at consult: file it costed.** Deliverable: a shipped, bridged arm that takes a
  scratch copy of the consumer's tree, applies a named variant of the config seam, runs the
  battery, and prints the per-gate verdict diff against baseline — so an adopter sees which gates
  a knob arms, disarms or reds before committing the knob.
  **Why design-pending:** the variant's declaration form (an env file or a config-dir overlay),
  whether the scratch copy is a worktree or a copy that carries untracked content, and whether
  the diff or a full report is the product. Native, per the interpreter constraint
  (gate-sdk/SPEC.md §The adopter constraints).
  **Refused:** repurposing the smoke scripts, which are install recipes read as text by the
  install-disposition gate and bound to this repo's harness; a shipped directory of shell tests,
  which widens the interpreter surface the adopter constraints shrink.
  **Cost while deferred:** an adopter evaluating a knob edits the seam, commits, and learns from
  the next red; the preview cohort's false-positive dispositions have no cheap rehearsal.
  Filed 2026-09-11 by consult as a direct entry, the test gap the operator named there.
  **Held Deferred by the enhancement admission filter, operator direction 2026-09-21
  (lead-relayed):** a shipped adopter arm reaching none of the three arms.

- **gate-binary-platform-roster-holes** [cost: once/low] [surface: native] — the shipped platform roster held four
  joined triples and two more the installed base plainly wants; **one of the two is discharged and
  this entry is what is left of it.**
  **PRODUCT-class** by the 2026-08-30 witness discriminator — the install path is adopter-facing,
  so the machinery-class icebox default does not reach this and it stays ordinary scope intake.
  **DISCHARGED 2026-09-11 at `52b4b96a`, leg repaired at `644547a6`: `aarch64-unknown-linux-gnu`
  is DECLARED HELD**, with its probed runner mapping and an `install-smoke-linux-arm64` consumer
  leg. Its roster line is deliberately unwritten — the join is the roster header's own predicate
  and never a queue entry's — and every mechanic that unit probed now lives where it belongs
  rather than here: the declaration-before-mapping ordering in `native/targets.list`'s header, the
  pinned-label cost in `native/runners.list`'s, and the answered floor question in the leg's own
  job header.
  **WHAT REMAINS IS `aarch64-pc-windows-msvc` ALONE**, and it is the operator's specific ask.
  Unpriced and unprejudged: its cost turns on whether GitHub's ARM Windows runners have reached
  general availability — re-probe actions/runner-images' table rather than trusting any paragraph
  — and on a runner decision that is the operator's rather than a session's.
  **THE ROUTE IS WORKED NOW, which is what the discharged half bought this one.** A platform is
  declared `held` FIRST: the declaration is what creates its producer leg, the runner mapping is
  only looked up once a declared target exists, and `continue-on-error` derives from the `held`
  bit so both legs land non-blocking on a red-averse master. Whoever takes this half copies
  `52b4b96a`'s shape; nothing in it was arm64-Linux-specific but the label and the floor probe.
  **The `observed-by` tag is DROPPED and the drop is a judgment, not a lapse.** That tag declares
  a completion predicate that is an observation of a remote run rather than a tree state, and its
  one reader of the value is a scope stage at promotion. This half's completion is a TREE state —
  declare, map, add the leg, which is exactly what the discharged half landed green before any run
  existed — and no `gates` run produces it until that work exists, so the producer field would be
  unreadable for its only named reader.
  **DISTINCT from `binding-intel-leg-failed-one-run-in-two`**, whose subject is a joined leg's
  RELIABILITY; this owns which hosts get a binary at all.
  **Cost while deferred: bounded, and one leg now rather than two** — one build leg and one smoke
  leg, plus a platform floor if the ARM Windows image does not carry the class the two bootstrap
  scripts already bootstrap. Runner availability is the open question here rather than the known
  non-blocker it was for arm64 Linux.
  Surfaced 2026-09-10 by the iteration lead at the operator's ask — rescued out of a gitignored
  journal into `packer-port-terminal-cut`'s gap inbox, promoted 2026-09-10 at scope on a fresh
  roster read, specified 2026-09-11 at spec with its scope narrowed to one leg, and demoted
  2026-09-11 at build on canon-kit/SPEC.md:239-250's corpus-versus-increment test — ruled `lead,
  own-authority` 2026-09-11 through the lead's message channel, that lead reversing its own Done
  instruction of the same date after verifying the grounds at source.

- **markdown-hard-wrap-unowned-and-ungated** [cost: once/high] [surface: queue-kit] — this repo's markdown hard-wrapping
  convention is unowned, bimodal and load-bearing for a gate whose manifest does not say so, and
  under enforcement-first the unwrap and its oracle land in one unit or neither does.
  **Measured at 2026-09-10 and RE-VERIFIED at this scope's HEAD.** `TRAJECTORY.md` is ungated —
  `check-queue-wrap`'s manifest reads `couples=TASK-QUEUE.md` and nothing else — and it is bimodal:
  46 lines at 79-81 columns, 94 at 92-100, 5 over 100, of 319. Two conventions inside one file.
  **The gate's real load is written nowhere, and that is the defect under the formatting one.**
  `check-queue-wrap`'s stated ground (so a runaway never reflows to column 0) is the weak one; its
  actual load is being the DENOMINATOR of `check-queue-entry-budget`, which measures an entry's
  extent in LINES against `QUEUE_KIT_ENTRY_LINE_CAP`. Nothing records that coupling, so a session
  reads the weak ground, judges it thin, and deregisters a gate holding up a sibling.
  **Six arms measure in lines and all six re-unit to CODE POINTS**, which is the precondition for
  dropping any cap: `always_loaded.rs`, `overhead_meter.rs`, `scan_prompts.rs`, `footprint.rs`,
  `port_blockers.rs`, `queue_entry_budget.rs`. `md_index.rs` counts lines CORRECTLY — navigation,
  not measurement — and is left alone. Unit ruled `lead, own-authority` 2026-09-10: code points,
  because `cplen` in `queue_wrap.rs` is the in-tree precedent, bytes penalise the em-dashes and
  section marks this prose is full of, and words need a tokenizer decision.
  **The `.metric/` history BREAKS on the unit change** — record the discontinuity and re-baseline in
  the same unit, or the first post-change close reads a tenfold phantom win.
  **`check-tag-lead-line` is a PURE COMPENSATOR for wrapping** and becomes deletable rather than
  merely deregisterable once an entry is one line, every governed tag sitting on the only line its
  readers scan. The `recurrence:` declaration is then the last line-led construct (`ruled:` is
  already retired), and the operator's move is to convert it to a bracketed tag beside
  `blocked-by`, `roadmap`, `spec` and `precondition-ok`, all of which `queue.rs` already scans
  positionally — after which nothing in the queue is line-scoped by design.
  **The new gate is the INVERSE of `check-queue-wrap`** — red when a prose paragraph is broken
  across lines — and both ship, each consumer registering the one it wants. That also answers the
  fixture-rot exposure. Enforcement-first is the operator's own correction here, and it corrected
  the lead: a convention with no oracle is the shape this repo refuses, so stop-maintaining-wrap is
  not a free half. That gate inherits `check-queue-wrap`'s two-tag discount: a deferred lead line's
  cost and surface tags go unmeasured (queue-kit/SPEC.md §check-queue-wrap).
  **Cost while deferred:** not low, and deliberately not claimed to be — this step alone rewrites
  every governed markdown file in the tree and ships a new gate.
  Surfaced 2026-09-10 by the operator through the consult channel in a lead session, relayed by that
  lead into the gap inbox of `packer-port-terminal-cut`'s close and promoted at this scope. The full
  probed body is recoverable: `git log -p -S'bimodal, and load-bearing' -- .workflow/gap-inbox.md`.
  **Operator direction 2026-09-22 (lead-relayed), into `queue-kit-unwrap`:** tracked markdown
  carries no artificial line cap; a tool needing a line-capped form gets one generated from the
  uncapped tracked file. Feature (a new gate); `/spec` authors and promotes it.

- **queue-entry-shape-slugs-headings-links** [cost: once/low] [surface: queue-kit] — the operator's queue-shape sequence:
  ratchet slug length, make each task a third-level heading, then make every cross-task reference a
  real link and retire the bold-code typographic convention. Steps two through five of the
  2026-09-10 sequence, whose ORDER IS BINDING.
  blocked-by: markdown-hard-wrap-unowned-and-ungated
  **Slug length first, and step two MUST precede step four**, because once slugs are anchors every
  rename is a breaking change. Measured 2026-09-10: 419 slugs spanning 17 to 64 characters, only 2
  at or below 20, 234 in the 31-to-40 band, 83 over 40. Ruled `lead, own-authority` 2026-09-10 on
  the operator's delegation: KEEP KEBAB and cap the length, moving the description to the body —
  the heading becomes the anchor and GitHub manufactures the kebab form anyway, so heading, anchor
  and `blocked-by` target become ONE STRING IN THREE ROLES with zero transformation, and
  `is_slug_head`/`is_slug_byte` already parse kebab. A prose title is the two-sources defect in
  friendlier clothes. **A RATCHET, NOT A CAP:** red on a NEW or RENAMED slug over 30 and grandfather
  the rest — a hard cap would evict 317 of 419.
  **Third-level headings, operator 2026-09-10.** That heading level is unused in `TASK-QUEUE.md`, 0
  occurrences, the title being one hash and sections two. Slugs are already one global unique
  namespace across active, deferred, icebox and sub-tasks, gated by `check-task-names`, and the Done
  line carries the slug verbatim — so a fragment reference becomes a PERMANENT ANCHOR surviving
  every section move, Done included. **Constraint:** the heading is the BARE slug with tags on the
  body, because GitHub derives anchors from heading TEXT and a tag inside the heading breaks the
  anchor the moment the tag changes. **Buys:** `--emit md-index` over the queue becomes a derived
  roster for free, `check-md-refs` gains roughly 420 verifiable anchor targets, and the icebox
  tier's no-subsections rule goes MOOT rather than violated — its ground was that grouping is
  presentation, and under this change the heading is identity.
  **References as links, operator 2026-09-10.** `check-md-refs` already validates that internal
  links resolve; nothing demands that a reference BE one. `check-queue-slug-liveness` detects
  references by TYPOGRAPHY — the bold-code form — and `ENV.local.md` carries a hand-written
  workaround because a BINARY NAME could not wear that markup without redding the gate: the attested
  cost of overloading prose markup as a reference marker. Demanding a clickable link separates prose
  from references and retires the convention.
  **Cost while deferred:** bounded and mechanical but wide — everything keying on the column-zero
  moves to heading detection: `is_top_level_bullet`, `is_bullet`, `live_slugs`, the section
  scanners, `check-task-conservation`, `queue-index`, `queue-counts`, `icebox-candidates` and the
  roadmap walk.
  Surfaced 2026-09-10 as above; full body via
  `git log -p -S'TASKS BECOME THIRD-LEVEL HEADINGS' -- .workflow/gap-inbox.md`.
  **Operator direction 2026-09-22 (lead-relayed), into `queue-kit-unwrap`:** records such as tasks
  become link-referenceable headings. Re-verified at scope: 0 third-level headings in this file.
  Feature (a slug ratchet and a heading grammar); `/spec` authors and promotes it.

- **install-smoke-leg-names-mix-two-axes** [cost: event/low] [surface: .github] — the `install-smoke` legs in
  `.github/workflows/gates.yml` spend one suffix slot on two different axes, and two tracked
  surfaces now carry prose whose only job is to undo the misreading that produces.
  **The naming, read off the workflow.** Three legs take a PLATFORM suffix
  (`install-smoke-windows`, `install-smoke-macos`, `install-smoke-macos-intel`), the baseline
  Linux leg takes none, and `install-smoke-powershell` takes a BOOTSTRAP suffix while
  running on `windows-latest` — so it reads as a second Windows platform leg, and a reader
  counting platforms off the leg names counts wrong.
  **THE STRONGEST ARGUMENT IS NEW AND THE LEG THAT MADE IT RECORDED SO.**
  `install-smoke-linux-arm64` landed 2026-09-11, and a SECOND Linux leg makes the baseline's
  unsuffixed `install-smoke` actively ambiguous where the absence of a suffix used to mean
  "the baseline" — the leg's own job header in `.github/workflows/gates.yml` states that it
  owes this entry the point. The leg count is deliberately unwritten here for the reason
  gates.yml's own correcting paragraph gives about platform counts: it moves, and a number
  written in prose goes stale silently.
  **The cost is attested rather than predicted:** `.github/workflows/gates.yml` and
  `installer/SPEC.md` each carry a paragraph whose whole job is to say that a reader counting
  platforms has been reading three legs as covering two halves, which they never did. A name
  needing a paragraph to be read correctly is the defect; the paragraph is the receipt.
  **WHETHER IS RULED, so only the scheme is open.** The operator ruled 2026-09-09, in the lead
  session and through the lead channel, that intuitive design is the aim — which retires the
  correct-but-underexplained disposition, a name a reader must be corrected about not being
  intuitive however well the correction is written. Closing this as no-mechanism is therefore not
  available to a later drain.
  **Why design-pending:** two schemes are live and unranked — bootstrap-first
  (`install-smoke-bash-<platform>` alongside `install-smoke-powershell`), or
  platform-suffix-preserving (rename only the odd leg so it names its bootstrap unambiguously) —
  and the blast radius has to be priced with the scheme rather than discovered after it.
  **THE BLAST RADIUS IS WHOLLY IN-TREE, and this entry asserted otherwise until 2026-09-11.**
  It claimed the rename also reaches the branch-protection required-check names in the local
  ops runbook's desired state, and that that surface breaks silently. Both halves are false
  and were measured false at the drain: the runbook names no `install-smoke` leg at all, and
  this repo's branch-protection desired state is deliberately none, so there are no required
  checks to break. The radius is five tracked files — `.github/workflows/gates.yml`,
  `installer/SPEC.md`, `TASK-QUEUE.md`, `docs/site-architecture.md`, `docs/install.md` —
  with no out-of-band step; other tracked files carry the literal descriptively or in
  historical logs and are not rename targets.
  **WHAT STILL MAKES IT SCOPE WORK, on the corrected radius.** Not reach: the unranked
  schemes. Five files renamed under a scheme nobody chose is a rename done twice, and the
  ranking is the deliverable a drain fix cannot supply.
  **Folded 2026-09-11 from `host-resolution-fail-open-cut`'s close inbox:** the rename lands
  with a check that every backticked `install-smoke-<suffix>` literal in tracked prose names a
  gates.yml job key; free-prose leg descriptions stay beyond its reach, its honest limit.
  **Cost while deferred:** two surfaces keep paying a correcting paragraph at every read, and
  every new reader of the CI matrix starts from a miscount the prose then walks back.
  Filed 2026-09-10 by close from the gap inbox, on a reach premise this drain falsified;
  re-priced 2026-09-11 at close onto the scheme-ranking ground, which survives.
  not-icebox-eligible: install-smoke-leg-names-mix-two-axes 2026-09-09 operator ruled it a defect

- **instrument-leg-expiry-keyed-to-a-green-run-not-its-assertion-set** [cost: event/high] [surface: .github] — an
  instrument leg's `continue-on-error` is dropped by an argued expiry that fires on the leg running
  green ONCE, so widening the leg afterwards re-creates, silently, the never-run condition the
  exemption existed for.
  **ATTESTED, and the cost was the exact one the exemption was written to avoid.**
  `install-smoke-powershell` carried the expiry "drop this line on the run it is first observed
  green and not before". It fired on an EIGHT-check leg; a later delta rewrote the leg to THIRTEEN,
  so five brand-new assertions executed for the first time anywhere on a leg that was by then
  BINDING. Two defects in the thirteenth reddened master at `ab676c7b`. The exemption's own stated
  ground was that "its first run was that code's first run and a binding red would have spent a
  watched push on the very defect the leg exists to surface" — and it spent precisely that push,
  against a one-to-two-push iteration budget.
  **BOTH DEFECTS ARE FIXED and this entry is deliberately the residue.** `74d94c94` provisioned the
  leg to the floor and cast the bare `-match` whose `System.Object[]` killed the step;
  `install-smoke-powershell` is SUCCESS at `273a9d76` in run `34346492761`, re-verified at this
  scope. Master is green and nothing needs reverting — what survives is the keying.
  **Why design-pending, a real fork with no obviously right limb:** an expiry keyed to *the
  assertion set unchanged* needs a stable identity for that set, which the live leg spells as
  inline PowerShell rather than as anything a scanner can count; a re-arm obligation on the
  widening session is a prose rule no gate reads and the same class this tree keeps refusing; and
  the third reading — that widening a binding leg is simply owed a rehearsal round like any other
  new code — costs a push per widening, which the budget above is what bounds. The shape reaches
  every `continue-on-error` instrument leg in `.github/workflows/gates.yml`, not this one.
  **A FOURTH LIMB, ATTESTED RATHER THAN PROPOSED — `lead, own-authority` 2026-09-09 through the
  lead's message channel.** An exemption whose posture is DERIVED — `continue-on-error` read out of
  a per-target roster index rather than hand-carried — makes the binding transition happen on the
  run that licenses it with no edit, and is rehearsed by construction, the green run licensing the
  join being the same assertion set. Witness: `install-smoke-windows` carried verbatim the expiry
  this entry indicts; this iteration's build replaced it with the derived read, and run 34394922502
  shows `native-artifacts (x86_64-pc-windows-msvc, windows-latest, false)` with both Windows legs
  binding and no hand edit. **Its BOUND, where the limb is weaker than it looks:** the derived
  posture needs a roster carrying a held/joined axis. Platform-target legs have one; an instrument
  leg with no target roster — the PowerShell bootstrap leg among them — does not, so the limb may
  resolve the platform subset and leave the shape stated above only partly answered. It is also the
  limb doctrine favours on its face, derivation-first. **Next step, filed and NOT started:**
  re-measure the remaining `continue-on-error` legs against this limb — which have a roster to
  derive from. That is a survey, so scope-gated intake puts pricing it in scope's hands.
  **DISTINCT from `binding-intel-leg-failed-one-run-in-two`**, which owns a leg already binding
  failing non-deterministically with no cause a finished run can reach; this owns the transition
  INTO binding being keyed to the wrong predicate.
  **Cost while deferred:** every widening of a leg that has already expired its exemption is an
  unrehearsed binding assertion set, and the failure mode is a spent watched push — the scarcest
  resource an iteration has.
  Filed 2026-09-09 to the gap inbox by the close of `behind-invoke-relocation`, which no stage of
  that iteration could drain; promoted 2026-09-09 at this iteration's scope intake, so the record
  is late and says so.

- **binding-intel-leg-failed-one-run-in-two** [cost: event/high] [surface: .github] — a leg this project made binding
  failed one of its first two runs, non-deterministically, in a way no finished run can diagnose;
  master is green and nothing needs reverting.
  **Read the resolution first.** Master went red at `adb7379f` and GREEN again at `7329b319`, the
  very next commit, with `install-smoke-macos-intel` PASSING the second time on the same code path
  and the same three declared targets. What is left is the leg, not a fire.
  **What failed.** In run 34267324532 (job 102200720560), the upgrade arm's pack step:
  the packer as it then was, `scripts/pack-installer.sh`, exited non-zero in 1.6
  seconds having printed NOTHING —
  `run-smoke.sh`:877 echoes `PACK_OUT` to stderr and `PACK_OUT` was empty. Every other arm on that
  leg passed (main, toolchain-free, jq-less), and the SIBLING arm64 leg ran the same upgrade arm on
  the same commit with the same three declared targets and finished clean.
  **No cause is asserted.** Regression, a host condition of the `macos-15-intel` runner, and a
  transient are all open. The leg was green on the immediately prior run 34245261556 at `held=true`,
  so the failure itself is new; the roster count cannot be the cause, since both macOS legs carry
  the identical count and only one failed.
  **Scratch witness landed** (`install-smoke-slow-leg`): read a recurrence against the upgrade arm's
  `scratch witness` lines — near-zero free space beside the failed pack confirms exhaustion.
  **Ruled 2026-09-08 by the lead, own-authority: CARRY it, do not unbind** — unbinding would reverse
  this iteration's own delivered predicate on a single sample, and the sibling-leg control already
  excludes the roster count as the cause.
  **DISTINCT in subject from `pack-step-dirty-tree-predicate-unscoped`**, whose subject is that
  refusal's SCOPE alone; this entry's is a red binding leg and what to do about it. The joined cut
  answered the silence incidentally — one refusal formatter, so the arm has no exit path that
  prints nothing — and the diagnosis of THIS firing stays here, because a signal-killed process
  prints nothing whatever the code does. That silence is what made the red unreadable from a
  finished run, and its firing here is the recurrence that retired entry carried.
  `lead, own-authority` 2026-09-10 by escalation reply, relayed by the lead, NOT the operator's.
  **Cost while deferred:** a re-run is the only diagnosis available, and the next firing costs
  another one. Filed 2026-09-08 to the gap inbox by the close of `intel-macos-roster-join`, which no
  stage of that iteration could drain; promoted 2026-09-09 at this iteration's scope intake, so the
  record is late and says so.
  **Joins `install-smoke-slow-leg` — operator direction, 2026-09-17, lead-relayed:** its scratch
  witness only, riding that unit's `run-smoke.sh` upgrade arm; green in 12 master runs since.
  **Re-costed iteration/high to event/high — operator direction, 2026-09-17, lead-relayed:** the
  cost is per firing, and the leg passed in the last 20 master `gates` runs.

- **held-ci-leg-failure-reddens-a-binding-one** [cost: event/high] [surface: .github] — a held producer leg's failure
  fails the workflow through a binding consumer leg, so held-ness is defeated for the pair, and the
  binding leg's measurement goes to zero rather than degrading.
  **Every premise verified at HEAD 2026-09-08 and measured on a live run.** `native-artifacts`
  carries `continue-on-error: ${{ matrix.held }}` over a derived matrix
  (`.github/workflows/gates.yml`:737, :739); `install-smoke-macos` at :858 needs it and is binding,
  stated twice (:838 "It is binding: its red fails this workflow", and again at :1064). The
  coupling is the normalize step at :1012-1016, which exits 1 when the host's artifact is absent —
  and it aborts BEFORE the smoke runs, so a down producer yields no measurement at all rather than
  a degraded one. Measured on run `34200226768`, where that step printed "the producer uploaded no
  complete aarch64-apple-darwin artifact" and those legs were the run's only failures.
  **Second cost, in the same shape.** The leg used to buy an adopter-path measurement from a host
  build; after delta 5 it consumes the producer's upload and buys nothing when the producer is down.
  **Deliberately left open by the hotfix, which is the discriminator against its sibling bullet.**
  The operator was offered the wider option that also decouples a held producer's failure from the
  binding smoke and chose the narrow bootstrap fix over it, so `de662aca` removes the observed
  TRIGGER and leaves this COUPLING untouched and filed.
  **Why the smallest mechanical shape is not the answer.** Falling back to a host build when the
  upload is absent would void criterion 2 of the join predicate `native/targets.list`'s header
  states — a platform install-smoke leg green having CONSUMED that upload, with no host-built
  stand-in anywhere in it — so the six-line edit trades away the evidence the leg exists to buy.
  Whether the leg should degrade or stay binding on its producer is an envelope question no surface
  settles.
  **Cost while deferred:** every future held platform inherits it, so widening the roster widens
  the coupling, and a red master traceable to a leg nobody declared binding costs a fresh diagnosis.
  **ITS OBSERVED TRIGGER CLEARED HOURS AFTER FILING, and the correction is recorded rather than
  left to rot.** Run `34209082851` at `d433fd4a` has BOTH darwin producer legs green, so the
  normalize step passes and the coupling is dormant at HEAD; `install-smoke-macos` is red on a
  different, downstream defect. What the run does NOT show is the coupling repaired — it shows the
  one trigger that had fired going away. The entry stays deferred on the structural ground its
  cost field states, that every future held platform inherits it, and NOT on a red master.
  Filed 2026-09-08 by close from the gap inbox. Promoted rather than fixed because it is an
  envelope question the operator has already declined once in its wider form; promoted rather than
  iceboxed because the coupling is structural and inherited, which no single green run retires.

- **kit-knob-consumer-adapter-convention** [cost: event/low] [surface: evidence-kit] — whether a kit may ship
  a knob whose only working configuration requires the consumer to author an adapter, and whether
  the front-end shape that answer takes should be a named convention rather than each consumer's
  invention.
  **The capability loss that raised it is CLOSED, and this entry is deliberately the residue.**
  `shell-gate-tail-port` deleted the path `DELEGATION_KIT_LIVENESS_CMD` defaulted to, so the
  turn-end liveness probe logged `verdict=unavailable` on every firing. The repair landed at the
  same cut: the kit template dropped a default that pointed at nothing (a path present in no tree
  reads as a shipped capability and is none), the knob kept its contract exactly — a path run with
  the scratch dir as its only argument — and this repo named its own reader
  `scripts/producer-liveness-reader.sh`, reaching the gate through `scripts/gate-exec.sh`. BOTH
  scripts left the tree 2026-09-05, and the negative control at
  `scripts/gate-tests/subagent-stop-reader.test.sh` now asserts `unresolved`, never `unavailable`.
  **The capture log priced the loss and then verified the repair, read at close.**
  `.workflow/subagent-stop-liveness.log` carried **77** `verdict=unavailable` firings, every one
  inside a single 46-minute window between the port that deleted the path and the repair, and
  **zero** after it. So the degradation was real, bounded, silent except for that field, and is
  closed — which is what makes the surviving question a convention question rather than a defect.
  **The precedent this stands on, which is what makes the question general.** evidence-kit met the
  identical break one caller over when the same port turned a pre-flight entry's named path into a
  descriptor, and discharged it with a CONSUMER-SIDE front end resolving the gate name, explicitly
  refusing to teach the kit's knob to resolve a name as "a kit-contract change". Two kits, two
  consumers, one shape, invented twice.
  **Why this needed design:** naming the convention is a cross-kit envelope change. The seam
  INSTANCE it was argued on is spent — `scripts/gate-exec.sh` left the tree 2026-09-05 and its
  successor `run-gates.sh --only` is kit-shipped, so a template could name that one — while the
  question survives on the parser knobs, where any default is still a seam call first.
  **Cost while deferred:** low and adoption-shaped — every consumer configuring such a knob
  rediscovers the adapter shape from scratch, and a consumer that does not gets honest degradation
  rather than a break, which is why nothing forces the issue.
  **The 2026-09-03 parser cut narrowed the question to its naming and answered nothing else.**
  Both `EVIDENCE_KIT_PARSER_<suite>` values this repo configures now name bundled arms, so an
  adopter wanting per-gate or per-arm scenarios writes a knob value instead of authoring a script:
  the *mechanism* stopped being each consumer's invention. The *name* did not — a third built-in
  adapter beside `exit-code` and `libtest` was refused twice over, on the ground that absorbing an
  owed script into a permanently shell library discharges the port's count while defeating its
  objective, and on the ground that naming the convention is this entry's own deliverable, which a
  port cut may not rule (evidence-kit/SPEC.md §The evidence adapters). So the cut DEMOTES this entry
  rather than closing it, returning it to the position the 2026-08-24 promotion took it from.
  **The delegation instance LEAVES the class 2026-09-05 (operator, consult):** the liveness hook
  reaches its gate through its own executable, so the knob is an override over a working default
  (delegation-kit/SPEC.md §The turn-end liveness hook); the question survives on the parser knobs.
  Filed 2026-08-24 to the gap inbox by build in two bullets, the second correcting the first's
  repair premise as measured-false; promoted 2026-08-24 at
  `shell-gate-tail-port-and-completion-oracle`'s close, whose drain confirmed the reader, its
  oracle and the surviving question. The instance above was filed 2026-09-03 to the gap inbox by
  the consult and drained here into this entry rather than into a new one, the port-only run
  barring the drain's promote.
  recurrence: kit-knob-consumer-adapter-convention 2026-09-03

- **plugin-marketplace** [roadmap: later/ecosystem] [cost: once/low] [surface: installer] — harness plugin packaging.
  roadmap-summary: The stage skills and guards installable as a harness plugin.
  Harness plugin/marketplace packaging
  of the stage skills and guards; anti-drift gate shape: manifest ↔ shipped
  surface parity. Design against the live manifest format at promotion — the
  plugin substrate moves fast (the scope-session-routing ruling applies).
  **The install-ownership contract this must package against already exists:**
  `checkwright.lock`, written by the installer's `init` and specified at
  installer/SPEC.md §The manifest — its schema owner is
  `native/src/installer/lock.rs`. A marketplace package that installs kits
  without writing that manifest would be a second install model with no upgrade
  or uninstall story, which is the sequencing risk this entry has always
  flagged; the named contract replaces the re-derivation it used to imply.
  The upgrade/uninstall story itself has shipped as the installer's `update`
  and `uninstall` verbs, specified at installer/SPEC.md §update and
  §uninstall — sequence against those rather than duplicating them.
  **Negative result — the tarball channel's economics do not transfer here.**
  The retired `release-tarball-delivery-channel` was cheap for a structural reason that is
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
- **benchmark-ab-experiment** [roadmap: later/adoption] [cost: once/low] [surface: drift-kit] — a controlled A/B trial.
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
- **hosted-attestation-service** [roadmap: later/commercial] [cost: event/low] [surface: evidence-kit] — hosted attestation.
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

- **heterogeneous-agent-delegation** [roadmap: later/ecosystem] [cost: iteration/low] [surface: delegation-kit] — foreign agents.
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
  **Excluded by the enhancement admission filter (2026-09-19 scope):** idle budget headroom,
  its strongest ground, is none of the three arms; only an operator exception admits it. Its
  citers (`companion-toolkit-profile`, the credential-swap entries) block on none of it.
  **Design-memory amendment (2026-07-25):** the TUI relay buys no session resume or token
  efficiency — both live in the vendor's session store (stateless APIs, the same on-disk
  transcript replayed against the same server-side prompt cache), so interactive-vs-headless
  is rendering, not state. Headless warm-resume by session id and JSONL turn events ship on
  the vendors probed, which makes (1) plumbing.
  **Verification capability (2026-08-02):** those probes ran against **installed binaries**
  (the foreign CLIs are on the development machine), so the executor is verifiable, not
  inferred from vendor docs — a change to the unit's risk under oracle-first: the executor
  ships with a smoke that invokes them. The machine profile (context-kit/SPEC.md
  §bin/env-probe, local-only) owns which CLIs and how.
  **Cost while deferred:** the foregone lever is live — read-heavy audits and mechanical
  sweeps all bill against one vendor's budget while three subscriptions are held — and
  this design memory ages against fast-moving CLIs.
  Surfaced 2026-07-17 in the release-in-lifecycle lead session (operator question).

- **background-credential-swap-support** [cost: event/high] [surface: delegation-kit] — first-class support for
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

- **companion-toolkit-profile** [roadmap: next/ecosystem] [cost: event/high] [surface: lifecycle-kit] — the interop rung.
  roadmap-summary: Gate a tree whose specs another toolkit's workflow wrote.
  Govern a tree whose specs an **external spec-authoring toolkit produced** — a
  consumer profile for when the specs Checkwright gates were written by a second
  toolkit's workflow, not by this one's `spec` stage. It cashes the claim below.
  **The design is already decided and is not what this entry holds.** Two rulings on
  record settle it: `prose-profile` (retired) ruled a profile ships as an adapter
  delivered as optional consumer config and never as a kit literal, and
  `heterogeneous-agent-delegation` rules a kit literal naming a vendor crosses the
  provenance seam outright. So the shape is a consumer-side profile over a declared
  artifact layout — the `check-graph` / `graph-vocab` pattern — with per-toolkit
  specifics in consumer config. What is open is the *substance*: which lifecycle
  assumptions break when the amendment set is authored elsewhere, and whether a tested
  two-toolkit consumer is buildable without a kit ever naming one.
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
  `--agents-md-smoke` arm. That is the shape the three claims below owe.
  **Seam re-verified clean:** no tracked file names an external spec toolkit.
  **Intake provenance:** never declined or costed — the opportunities half of the same
  operator-commissioned review whose *weaknesses* half filed six launch-facing rungs
  2026-07-23, all since landed; the growth half fell outside that intake's stated "top
  pre-announcement gaps" filter rather than being missed, so the gap is the absent record.
  **Enhancement admission filter, engaged 2026-09-20 at scope:** strongest ground is the
  reputational carry below, and it reaches no arm — the trust arm means a gap in what an
  adopter must trust, not a claim weaker than its proof; discharge it by qualifying the claim.
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

- **design-partner-preview** [cost: event/low] [surface: drift-kit] — a narrow external preview
  before any broad announcement: a preview cohort whose composition is ruled in the operator's
  private brief, installs observed live rather than by written feedback, instrumented for
  time-to-first-green, first useful red, false-positive dispositions, and 7/30-day retention
  per kit. It is the first rung on this queue whose deliverable is **evidence from outside
  this tree** rather than a tree change.
  **The TREE HALF landed in `external-install-evidence`** — drift-kit/SPEC.md §The
  install-observation record and §The install-evidence projection, the `--emit file-install`
  capture arm, and `docs/install-evidence.md` behind `check-install-evidence-fresh`. What is
  kept here is the expensive half: operator hours and a calendar window of thirty days or
  more, running beside later iterations and never inside a stage session. What re-promotes
  this entry is an observed install, not another tree change.
  **Sequencing is the load-bearing part.** The preview runs *before*
  `benchmark-ab-experiment`, so pilot findings shape that experiment's task classes and
  metrics rather than being retrofitted to them; per-gate true/false-positive history and
  profile retention are preview deliverables, not pre-launch builds. The full launch ruling
  behind this sequencing is operator material and stays in the local-only private brief; this
  entry carries only the queue-visible rung.
  **The cohort was a named population here until 2026-08-09 and is deliberately no longer
  one** — the composition it stated had since been re-ruled, so the sentence contradicted the
  ruling it was meant to carry; the fix is to name the owner rather than restate a ruling
  this file does not hold.
  **Expected FIRST FINDING, not a precondition:** today's quick start is curl, sha256sum, tar
  and `bash … init` from a repository root; macOS needs GNU bash and coreutils by adopter
  action; native Windows needs Git for Windows. That is why the merged channel gives the
  installer no delta — those host-floor facts are an output of the observation, not an input.
  **Refused, grounds carried forward:** parking behind `native-windows-bash-floor` or the
  git-only-floor discharge (the trigger is what the preview measures); the icebox (the
  highest cost-while-deferred in the intake).
  **Cost while deferred — still the highest of its intake, and now the more exposed half.**
  Every claim that would be strongest with external evidence still rests on internal
  dogfooding, and the channel that would carry it is built and reading zero: the published
  page states an honest `0` on every pass while the volume of unattested governed surface
  keeps growing. Deferring also silently defers `benchmark-ab-experiment`, since running that
  first would fix the wrong metrics.
  Surfaced 2026-08-02 at close, in the same intake pass, as the review's fourth-ranked item.

- **external-gate-quality-evidence** [cost: event/low] [surface: drift-kit] — durable, published
  evidence of **gate quality as experienced outside this tree**: per-gate true/false-positive
  history, the disposition of each red a non-author hit, and whether a red changed behaviour
  or was worked around. The direct answer to the standing threat that a false positive
  converts the enforcement advantage into bypass and distrust — every blocking gate raises
  the stakes of a wrong red.
  **Why it is not just a report.** The tree already publishes evidence projections, so the
  mechanism exists; what does not exist is a *population* to measure. A red in this repo is
  authored and dispositioned by the same party, which cannot distinguish a gate that is right
  from a gate whose author agrees with it.
  **The record's FIELDS landed in `external-install-evidence`**, settling the open design
  question the deferred form carried: the collection surface is a new capture stream — the
  observation record's `red` line, with its closed `<verdict>`, `<disposition>` and
  `<behaviour>` sets — rather than a field on a disposition this tree already records, the
  disposition being measured being the *adopter's*. What is kept here is the evidence itself,
  which accrues only once observations exist; `docs/install-evidence.md` publishes zero reds
  today, and a population is what re-promotes this entry.
  **The bundle with `design-partner-preview` was not convenience.** This entry's own carry
  sentence rules the history unrecoverable — it cannot be retroactively collected, and starts
  accruing only once someone decides to record it — so a protocol landing without the
  gate-quality fields designed into it would have sent the first installs' reds somewhere this
  baseline can never read. There was one chance to fix the record's fields and it is spent.
  **Cost while deferred — moderate, asymmetric, and no longer unrecoverable.** The
  gate-quality claim still rests on fixture pairs and a green battery, which prove a gate does
  what its author specified and say nothing about whether that was the right thing to specify.
  The irreversible half is discharged: the record exists, so the history starts accruing at
  the first observation rather than never. What remains is that the first externally-hit false
  positive is argued from anecdote until a population exists.
  Surfaced 2026-08-02 at close, in the same intake pass, as the last of the growth half.

- **gate-authoring-sdk-surface** [roadmap: next/ecosystem] [cost: event/low] [surface: gate-sdk] — a gate-authoring SDK.
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
  (gate-sdk/SPEC.md §Consumer payload, which owns both). What stays
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
  **Enhancement admission filter, engaged 2026-09-20 at scope:** its strongest ground is
  the cost-while-deferred line above — each further port hardening substrate assumptions
  by habit — and that is an internal design cost, none of the three arms: it neither cuts
  time-to-first-value, closes a trust or supply-chain gap, nor produces external proof.
  Only an operator exception admits it. **Unlike `heterogeneous-agent-delegation`, one
  citer does block on it:** `gate-tamper-exemption-reader-substrate` is design-pending
  precisely on the ruling this entry holds, so declining this entry holds that one too —
  a cost the decline carries knowingly, not a reason to admit it.
  Filed 2026-08-02 by build, on an operator ruling, during `native-gate-dispatch-seam`.

- **gate-tamper-exemption-reader-substrate** [cost: event/low] [surface: gate-sdk] — `check-gate-tamper`'s
  exemption reader has no implementation-side equivalent.
  Split 2026-08-09 at scope by operator ruling from `gate-tamper-roster-native-reach`,
  when that entry narrowed to its meta-path-roster half and promoted; this is the
  exemption half, unchanged in substance. That entry was itself split 2026-08-02 from
  `native-gate-meta-layer-reach`, so this is the second narrowing of one original gap.
  `extract_exemptions()` parses a shell `# exception-list:` array literal, so a ported
  gate's Rust module can carry no exemption the gate is able to read.
  **Why design-pending:** it wants the ruling `gate-authoring-sdk-surface` holds
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

- **session-model-identity-verification** [cost: event/high] [surface: delegation-kit] — a session cannot report or
  verify the model tier it is running at.
  The session-context hook prints iteration, budget and drift; `drift-report` prints neither.
  Nothing surfaces the running model, so a session cannot state its own tier without a human
  hand-reading the harness transcript, and no stage can assert the tier it was dispatched at.
  **Operator-proposed shape, recorded 2026-08-04:** a `usage-verdict`-shaped check — snapshot
  in, exit 0/1/2, fail-soft — reusing the `--emit-session-id` arm's projects-dir
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

- **consult-tier-declaration** [blocked-by: session-model-identity-verification] [cost: event/high] [surface: delegation-kit]
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

- **intra-file-pendency-contradiction-scan** [cost: event/high] [surface: canon-kit] — one file can call the same
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
  **Why design-pending:** the construction vocabulary is the whole gate, and a literal
  phrase list in a kit is drift by construction plus a provenance-seam problem — the
  vocabulary is consumer editorial. It wants the `check-graph` / `graph-vocab.knobs` treatment,
  optional consumer config, which is a design call rather than a size one. Also open: whether
  a legitimate "X landed, Y still waits on it" sentence pair trips it, which decides whether
  the predicate is per-slug or per-slug-per-section.
  **Cost while deferred:** the class stays a sweep whose reach is whoever runs it, and its
  one measured miss cost a full iteration of a governed SPEC contradicting itself in public
  — gate-sdk/SPEC.md is mirrored to the docs site, so the contradiction shipped.
  Filed 2026-08-04 at close; the instances it would have caught were fixed the same session.

- **amendment-refusal-acceptance-parity** [cost: event/low] [surface: canon-kit] — an amendment's refusal rationale can
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
  **Why design-pending:** the decidable predicate is not obvious. "A refusal section citing an
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
  removal of the stated bound now standing in `installer/SPEC.md` §The verbs and §The consumer
  smoke. Taking (1) without (2) leaves a published caveat naming a coverage limit that no longer
  holds.
  **The tree is honest today, only narrower than the refusal argued.** The capability-liveness
  sweep at this close found zero governed surfaces still carrying the wide claim, so nothing
  published overclaims; what is missing is the assertion, not a correction.
  Filed 2026-08-08 by close, draining the gap inbox; found at build. The coverage half was
  escalated as an envelope call and folded in here on the lead's ruling the same day.

- **docs-link-red-remedy-first** [cost: event/high] [surface: site-kit] — `check-docs-link-convention` reds on the
  most ordinary thing a docs subpage author writes, and leads with the diagnosis.
  **Measured 2026-08-09 at spec on a real `init` consumer.** A `docs/` tree whose `index.md`
  carries a `[project README](../README.md)` link reds with "off-root relative link … resolves
  outside docs/".
  **The rule is correct, and changing it is not what this asks for.** For a site served from
  `docs/` alone such a link 404s, and the gate already offers a remedy — the absolute self-repo
  blob form, or a `docs-link-exempt:` comment. It stays on-surface deliberately
  (its descriptor's `install:` line keeps it there), so nothing is broken today.
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

- **baseline-row-prose-coupling-gate** [cost: event/low] [surface: canon-kit] — governed prose asserts what
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
  **A SECOND instance was authored 2026-08-24, at this close's eviction review.** Ruling the
  `installer_smoke` row's attribution put a claim about that file's slug column into two governed
  surfaces at once — `bridged-knob-case-tmp-dir-override-inert`'s body and evidence-kit/SPEC.md
  §Baseline manifest. That pair is no longer live: the entry's body left the queue with its Done
  move (2026-09-12), so the claim now sits on the SPEC section alone.
  **No `recurrence:` date joins:** the
  entry names an unbuilt gate rather than a defect, so authoring a new instance of the class it
  would catch is the class recurring, not the finding re-firing.
  **Cost while deferred:** low and slow, but it recurs on exactly the readers who most need the
  file — a cohort pricing criterion 5 reads the prose first.
  Filed 2026-08-14 by close, from its own gap-inbox drain and staleness review; kept in Deferred at
  the 2026-08-24 eviction review on the trigger above and on the live slug it names.

- **gap-capture-argv-prompt-friction** [cost: event/low] [surface: gate-sdk] — the mandated capture tools take their
  prose as an argv string, so every filing whose prose contains shell punctuation costs an
  out-of-band permission decision.
  recurrence: gap-capture-argv-prompt-friction 2026-08-15
  **Re-measured 2026-08-15: three prompting calls** (`file-survey.sh` twice, `file-gap.sh` once,
  `kfric.sh` none), against six the iteration before. Halved, and the halving is not progress —
  fewer captures were filed, and the per-filing tax is unchanged.
  **Diagnosed rather than allowlisted, per the triage criterion.** `bash
  gate-sdk/bin/run-gates.sh *` is **already** in the committed allowlist and, since the 2026-09-01
  and 2026-09-03 ports made survey, gap and knowledge-friction capture arms, is the one grant
  covering all three — the per-tool `kfric.sh *` grant went with its file — so this is not missing
  coverage. The harness matcher
  refuses a command whose text carries an expansion or a redirect, and gap prose routinely
  carries both: a backticked slug is command substitution, and a bullet describing
  `jq -r … 2>/dev/null` contains a redirect operator inside its quotes. The glob cannot help,
  because the match never gets that far.
  **Measured this iteration:** `file-gap.sh` prompted **4** times, `file-survey.sh` and
  `kfric.sh` once each — six out-of-band decisions on the three tools the repo *mandates* for
  in-the-moment capture, which is the exact path CLAUDE.md says deferred capture ruins.
  **Deliverable, and it is small:** a body-from-file arm — `--emit file-gap --from <path>` reading
  the prose from a scratch file written with the editor tool — is a fully static command the
  matcher can grant. The same shape serves `--emit file-survey` and `--emit kfric`, whose
  free-text fields are the same shape; the capture tool's port to a compiled arm moves where that
  shape is written and settles nothing here. What needs deciding is whether it is per-member or
  a shared helper, and whether the argv form stays (it should — a short gap is one call).
  **Why it is not a guard rule:** there is no better *form* to steer to today, which is what a
  guard rule requires. The form has to exist first.
  **Cost while deferred:** a friction tax that scales with how carefully a bullet is written,
  which taxes exactly the good filings.
  Filed 2026-08-14 by close, from the prompt-friction triage.
  **Held Deferred:** the `--from` arm, this entry's whole deliverable, is excluded by the
  enhancement admission filter (operator direction 2026-09-21, lead-relayed).

- **queue-recovery-pickaxe-wrong-oracle** [cost: event/high] [surface: queue-kit] — every surface that tells a reader how
  to recover an evicted queue body names `git log -S`, which is blind to exactly the eviction it
  documents.
  **Measured, not reasoned, at this close.** `-S` fires only when a literal's occurrence *count*
  changes, and an eviction that leaves the slug behind changes none — a `## Done` move to a bare
  slug line, an icebox one-liner, or any body that spelled its own slug once. Two probes: this
  iteration's own Done move (`4bea9ceb`) leaves the count at 5 before and 5 after, so `-S` does
  not list the evicting commit at all and its newest hit is an unrelated earlier commit; and on a
  real icebox eviction, `-S'scratch-execution-allowlist-bar'` returns 3 commits where `-G`
  returns 5. `-G` matches diff content and reaches both.
  **Three surfaces carry the wrong spelling**, which is why this is one unit rather than a typo
  (a fourth, `queue-entry-evidence-tier`'s body, left with that entry at its 2026-09-17 landing):
  this file's `## Icebox` preamble, queue-kit/SPEC.md §The
  icebox tier, and `check-queue-entry-budget`'s own help text. **Re-verified 2026-09-22 at
  scope:** the help text no longer carries `-S` (`git grep` over `native/src` is empty); the live
  sites are the preamble, queue-kit/SPEC.md §The icebox tier and body recovery recipes here.
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
  Operator direction 2026-09-22 (lead-relayed): into `queue-kit-unwrap`; `/spec` authors and
  promotes it.

- **settings-hook-command-path-gate** [cost: event/high] [surface: context-kit] — a hook registration in
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
  **Why design-pending:** whether the widened subject stays inside `check-settings-paths` or
  mints a second gate name is canon-kit's new-names litmus, and the projection half is a scope
  call on the enforcement page that nobody has taken.
  **Cost while deferred:** a broken hook is invisible until the behaviour it guards silently stops
  happening — the failure mode with no red anywhere and no user-visible symptom.
  Filed 2026-08-22 at spec while surveying context-kit's settings gates; drained at that
  iteration's close, which re-verified the claim and found the enforcement-map reader it missed.

- **guard-rule-number-not-citable-outside-kit** [cost: event/low] [surface: guard-kit] — a guard-kit rule number is a
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
  outside the kit and have every cross-corpus reference name the rule — the way rule 21 already
  cites DOCTRINE.md by name rather than number, for exactly this reason.
  **A SECOND entry's slice now routes to that second disposition — recorded here 2026-09-08, the
  inbound half of a routing whose outbound half already exists.** `canon-kit/SPEC.md`
  §check-amendment-retired-spelling rules the RENUMBER slice — the retired
  `amendment-roster-omission-detection`'s second candidate slice — a stated non-target on a
  decidability ground: after a renumber the retired and the replacing spellings occupy the SAME
  token space, so a survivor scan cannot discriminate. That section states the durable fix in its
  own words ("a cross-corpus citation that names its referent rather than its number has no numeric
  relation left to decay, which dissolves the slice instead of detecting it"), which is the outbound
  half. **Its citation here was wrong on BOTH halves until this close** — there is no
  §The two slices in that SPEC, and its line 362 is about config seeding and carries no pointer;
  re-read at 2026-09-08 and corrected to the section that owns the ruling. So this entry's second
  disposition —
  a cross-corpus citation naming its referent rather than its number — **dissolves that slice
  instead of detecting it**, and discharges two entries' work rather than one. This entry's own
  open question (whether a bare `rule N` outside the kit is gateable at all, and at what
  false-positive budget) is untouched by the routing and is not answered by it.
  **Recurred 2026-08-29, and this entry's own citation was one of the six that rotted.** Rule 19's
  insertion pushed rules 19-23 to 20-24; the renumber sweep held guard-kit but not this file, so
  five citations of the scratch-execution rule by its pre-insertion number, and this entry's own
  citation of the history-rewrite advisory by its pre-insertion number, all pointed
  at the wrong rule. Corrected in place at close, by number rather than by name, because naming
  them here would silently do the second disposition's work and rewrap five capped entries.
  **Why design-pending:** the second is a one-paragraph boundary note plus a sweep of unknown
  size, and whether a bare "rule N" outside the kit is gateable at all needs a false-positive
  budget nobody has measured.
  **THIS ENTRY AND ITS `guard-rule-number-intra-kit-citations-ungated` SIBLING WERE THE (D) ISLAND
  of `citation-liveness-family-convergence`, which landed 2026-09-17 with (A)-(C) only** —
  grounds relocated here 2026-09-08 from that hub, under `check-queue-entry-budget`'s rule that an
  unanswered ground moves to the entry already owning its subject. Slices (A)-(C) widened gates that
  already resolve citations; (D) has no gate to widen and an unmeasured false-positive budget over
  111 intra-kit citations, so it stays open here.
  **Cost while deferred:** a reader follows the number to the wrong rule and reasons from it.
  recurrence: guard-rule-number-not-citable-outside-kit 2026-08-29
  Filed 2026-08-22 at align's cross-audit; drained at that iteration's close, which found the
  bullet had named the wrong slug and located the real entry before dispositioning.

- **guard-rule-number-intra-kit-citations-ungated** [cost: event/low] [surface: guard-kit] — guard-kit cites its own
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
  **Why design-pending:** the corpus is bounded and the numbering is derivable from the
  numbered list, so a gate could assert that every intra-kit `rule N` resolves to an existing
  item and that the derivable rosters — which rules take the raw command, which read a skeleton —
  match the bodies. Which claims are derivable and which are prose is the open question.
  **DISTINCT from `guard-rule-number-not-citable-outside-kit`**, whose dispositions are about
  cross-corpus prose *outside* the kit and which therefore leaves intra-kit numbers citable and
  ungated by construction; and from `guard-ruleset-registration-lockstep`, Done 2026-09-18 as
  `check-guard-registration`, whose subject is the roster/function/dispatch-order triple agreeing,
  not what cites a rule by number.
  **Cost while deferred:** every insertion into the ruleset re-buys a hand sweep whose
  completeness nothing checks, and a stale roster reads as authoritative to the next author —
  which is exactly how the raw-vs-skeleton one survived.
  Filed 2026-08-23 by build; drained at that iteration's close, which re-counted the citations
  and confirmed no gate matches them.

- **precondition-gate-direction-blindness** [cost: event/low] [surface: queue-kit] — `check-queue-prose-precondition`
  reds an entry whose prose says the entry IS the blocker, and three of the four remedies it
  prints are false for that shape.
  **Reproduced at this iteration's scope rather than predicted.** Promoting
  `worktree-reclamation-cause-falsification` to Technical Debt red the member on a paragraph
  reading "UPSTREAM of <slug>" and "two sibling amendments are blocked on", where the entry held
  no precondition of its own and was pickable first by construction.
  **The remedy list is the sharper half, and it was read off the gate at this drain.** The four
  lines it prints are: tag the real blocker, move the entry to Deferred, rephrase past-tense, or
  take the `[precondition-ok:]` valve. For an entry naming ITSELF as the upstream, a blocker tag
  asserts a blocker that does not exist, past-tense rephrasing falsifies an unrun experiment, and
  a move to Deferred undoes an operator-ruled promotion. Only the valve is true, so the gate's
  own help text steers a session toward writing something false unless it stops to reason.
  **Direction, not negation — and the filing's "they do not share a fix" FELL at the drain.**
  `precondition-gate-negation-false-positive` is the NEGATED shape ("not gated on", "no longer
  waiting on") against a bare-substring alternation; this is the unnegated, affirmative sentence
  whose subject is the blocker rather than the blocked, and no phrase-set calibration reaches it.
  But that entry's third candidate deliverable — declare the valve the intended answer and widen
  queue-kit/SPEC.md's calibration paragraph to say so — is this entry's second horn verbatim. The
  two are distinct in TRIGGER and overlap in REMEDY, so whichever is taken first should take
  both; the filing's claim was read against the sibling's trigger set, never its deliverable list.
  **No `recurrence:` date joins that sibling:** its finding is the gate redding a NEGATED
  sentence, and a negated sentence did not re-occur. A neighbouring shape in the same
  false-positive family did, which is a new defect and files as one.
  **Deliverable — rule one of two:** the gate attempts a direction read at all (an author-subject
  test, not a phrase set); or the honest answer is that it cannot, the valve plus a stated cause
  IS the contract, and queue-kit/SPEC.md's calibration paragraph says so — it currently justifies
  the blocking grade on a bounded FP scope naming neither this shape nor the negated one.
  **Cost while deferred:** every entry that names itself as an upstream blocker pays one red and
  one valve, and the surface teaching the repair teaches three wrong ones.
  recurrence: precondition-gate-direction-blindness 2026-09-20
  Filed 2026-08-24 to the gap inbox by scope, which reproduced the red; drained and promoted
  2026-08-25 at close, which read the gate's four help lines first-hand and corrected the filing.
  Feature — either horn changes the gate's asserted behavior or its SPEC contract; `/spec` authors
  and promotes it. Operator direction 2026-09-22 (lead-relayed): into `queue-kit-unwrap`.

- **icebox-eviction-line-budget-squeeze** [cost: event/low] [surface: queue-kit] — the icebox tier's one-line grammar
  and `check-queue-wrap`'s column cap are jointly unsatisfiable above a slug length nothing
  bounds, and nothing says so at the point of eviction.
  **Attested first-hand at the 2026-08-25 close, three failed attempts rather than predicted.**
  Evicting `spec-embedded-source-criterion-4-membership` (icebox) — a 44-character slug — left 31
  columns after the then-mandatory design-pending-tagged lead prefix. All three candidate
  sentences describing the question redded `check-queue-wrap`, and the line that landed says only
  "Its port sizing stays unruled." The tier's contract calls for a self-contained sentence; at 31
  columns that is not achievable, and the grammar gives no relief — `check-queue-entry-budget`
  assertion B makes an icebox entry EXACTLY one line, so wrapping is a violation and not a
  workaround.
  **Why it stays invisible until a session hits it:** the slug is chosen at filing time and the
  eviction is paid iterations later by a different session, so the two constraints never meet in
  one edit until they collide.
  **Deliverable — rule one of three.** Exempt the icebox lead line from `check-queue-wrap` (the
  run-away-reflow hazard the cap exists for does not apply to a line the tools key on by its
  `- ` lead); or cap slug length at filing, which is enforceable and retroactively expensive; or
  state at queue-kit/SPEC.md §The icebox tier that a long-slugged entry's pointer degrades to a
  bare classification and that this is accepted, so a later session stops re-deriving it.
  **DISTINCT from `queue-entry-grammar-single-owner`** (icebox), which is two entry grammars
  disagreeing; this is one grammar whose own two constraints cannot both be met.
  **DISTINCT from `spec-embedded-source-criterion-4-membership`** (icebox), cited here as the
  instance and deliberately not re-filed: that slug's finding is a criterion-4 classification
  question, untouched and still carried in the icebox, while this entry's finding is the eviction
  grammar that could not describe it. The two share nothing but the eviction that surfaced one
  while performing the other, so no `recurrence:` date is owed on it.
  **Cost while deferred:** low and paid at eviction — a session either spends three gate
  round-trips discovering the budget, or declines an otherwise-eligible eviction because it cannot
  describe it, which silently biases the tier against exactly the heavily-specified entries the
  tier was built to drain.
  Surfaced 2026-08-25 at the `turn-end-liveness-seam-and-worktree-cause` close and filed to the
  gap inbox there; promoted 2026-08-25 at this scope's drain of that inbox.
  **IT RECURRED 2026-09-03, on the second of the two branches the cost line names — the one that
  leaves no gate round-trip behind.** The `parser-and-enum-adapter-cuts-with-graph-hotfix` close
  ruled `worktree-cleanliness-assertion-scopes-to-checkout` icebox-eligible on the merits (its own
  cost line reads low and self-correcting, no roadmap tag, no live trigger, and its mitigation had
  already landed) and then DECLINED the eviction, because that 52-character slug leaves 22 columns
  after the mandatory prefix and no sentence in 22 columns is the self-contained one the tier's
  contract asks for. So the eviction was abandoned rather than degraded, which is the silent bias
  this entry predicted: the entry stays in Deferred carrying a costed body, and nothing in the tree
  records why except this paragraph. The first attestation cost three gate round-trips and landed a
  thin line; this one cost one round-trip and landed no line at all. Both branches are now
  attested, which retires the "predicted" half of the deliverable — the three candidate rulings
  stand unchanged and none of them is chosen here.
  **Reshaped by operator direction 2026-09-22 (lead-relayed), into `queue-kit-unwrap`:** past the
  2026-09-17 loosening (no design-pending tag, +17 columns), unwrapped markdown leaves the one-line
  grammar no column cap; `/spec` confirms the hard-wrap and entry-shape amendments discharge it.
  recurrence: icebox-eviction-line-budget-squeeze 2026-09-03

- **site-health-issue-venue-unwanted** [cost: event/low] [surface: site-kit] — the site-health probe files issues on
  the public repo for failures the iteration lifecycle resolves anyway, and the operator does not
  want that venue.
  **Operator-ruled 2026-08-25: the issue-filing path is unwanted.** The objection is to the
  **venue**, not to the probe — and a later session must not read it as the probe being wrong.
  Both firings were true positives on arm #6, the Release body missing its note URL: 2026-08-08
  on `v0.22.0` and 2026-08-24 on `v0.25.0`, each cleared by the probe's own recovery path.
  **Those dates sit in this prose deliberately.** The operator has since deleted both issues —
  probed here, `gh issue list --state all` returns nothing — so the tracker is empty, two dead
  run-log URLs are all that survives of the evidence, and the underlying defect's repair landed
  as `release-body-step-has-no-in-tree-witness` — the publish job now composes the Release body,
  unwitnessed until the first tagged publish run — rather than at any issue that resolves. The
  `site-health` label survives and is harmless: the workflow's label creation is idempotent and
  its open-issue lookup returns empty either way.
  **The deletion is not the fix, and an empty tracker is not the problem going away.** The
  workflow is unchanged and still armed on its `17 6 * * *` cron, so the next arm-#6 failure
  opens a fresh issue. That is the whole reason this entry exists.
  **The fork, unresolved, and it is two changes rather than one.** (1) Repo-copy only: delete the
  step and the `issues: write` scope from `.github/workflows/site-health.yml`. One file — but
  that file is pinned governed repo-meta in `scripts/core-files.list` and is a copy of
  `site-kit/templates/site-health.yml`, so this forks the template a repo governed by its own
  kits dogfoods. (2) Kit-level opt-in: the issue path becomes consumer config defaulting **off**
  under the `<KIT>_<KNOB>` convention, with this repo taking the default — template plus
  site-kit/SPEC.md plus `site-kit/smoke/install.sh`, which copies the template in.
  **Option 2 is the recommendation and it is BLOCKED, operator-class.** The template header states
  the issue path as a standing design ruling — a failed probe opens or updates an issue and
  recovery self-clears — so making it opt-out reverses that ruling, which is the operator's to do
  and neither a stage's nor a lead's. Recorded rather than resolved.
  **The replacement signal is the whole cost, and one half is now probed.** Candidate A, red run
  only: GitHub's documented scheduled-failure notification targets the last modifier of the
  **cron syntax** — not the last committer — and here that is `016d522a`, 2026-07-10, the
  operator, so the channel resolves to the right person today. The limit that cannot be probed
  from the tree is whether their notification settings deliver it. Candidate B, write the failure
  report to the run's job summary: visible in the Actions tab and files nothing, but the workflow
  writes no step summary today, so this is net-new work rather than a redirect.
  **Cost while deferred:** tracker noise on a public repo, and nothing worse — the probe is
  accurate and self-clearing, so no outage goes unseen while this waits.
  Filed 2026-08-25 by scope, operator-directed and relayed through the lead; the tree read
  behind it was re-run here rather than taken on the relay.
  not-icebox-eligible: site-health-issue-venue-unwanted 2026-09-21 operator-ruled, cron-armed

- **shellcheck-analyser-version-unpinned-in-ci** [cost: event/high] [surface: .github] — one battery member's verdict is
  a function of the host, so a green local battery is not evidence of a green CI battery.
  **Attested 2026-08-27 first-hand and expensively at close.** The full battery read 106/106
  locally and the pushed run went RED on `check-shellcheck` alone, with SC2120 against
  `gate-sdk/lib/gate.sh`'s `gate_exe_suffix`. Local shellcheck is 0.11.0 and does not emit that
  finding; the `ubuntu-latest` runner's stock shellcheck does. Nothing in
  `.github/workflows/gates.yml` installs or pins shellcheck, so CI takes whatever the runner image
  ships and that floats under this repo without a signal.
  **The finding was a true positive for the older analyser and a false positive for the code** —
  `gate_exe_suffix`'s argument-passing callers then lived in `gate-sdk/bin/build-native.sh` and
  `scripts/pack-installer.sh` alone, so a per-file analysis could not tell an optional-by-contract
  parameter from an unused one. Silenced inline with a justifying comment, the remedy the gate's
  own help prescribes. **The class is not that finding.** It is that this member wraps an
  external analyser whose rule set changes between releases, which makes 106/106 a claim about
  one machine; every other member is deterministic given the tree.
  **DISTINCT from any entry about the gate's own logic** — the gate behaved correctly and reported
  honestly on both hosts. **DISTINCT from the Windows-host inventory finding that shellcheck is
  ABSENT there**: absence is graded and visible, a version skew is silent and reverses a verdict.
  **Why design-pending:** three uncosted shapes — pin the analyser version in the workflow and
  state it where the gate's contract is specified; have the gate REPORT the version it ran so two
  runs are comparable; or accept the float and say in the gate's SPEC section that this member's
  verdict is host-dependent.
  **Cost while deferred:** the pre-push battery's central promise — that a green local run predicts
  a green remote one — is false for one member, and the failure mode is a burned push.
  Filed 2026-08-27 by scope into this iteration's ledger, draining the gap inbox; attested
  2026-08-27 by the `windows-adopter-unblock` close's own verifying push.

- **harness-project-dir-fold-dialect-unresolved** [cost: event/low] [surface: context-kit] — the harness project-dir
  derivation `check-memory-off` and its two shell twins share folds a repo root's `/` and `.` to
  `-`, and under gate-sdk/SPEC.md §The path-dialect contract's per-substrate dialects the two
  substrates fold the *same* Windows checkout to two different names: the crate reads a
  drive-lettered root and yields one spelling, an MSYS shell reads the `/c/…` spelling and yields
  another. Only one can match the directory the harness itself creates, so on Windows at most one
  of the three sites is right and nothing here says which.
  **The three sites, verified 2026-08-30:** `native/src/gates/memory_off.rs:26-29` (a char fold
  over the raw repo root) and `scripts/session-context.sh:91` /
  `context-kit/templates/session-context.sh:86` (`tr '/.' '-'`, same fold).
  **Why it promotes rather than fixing or iceboxing.** →fix fails on evidence, not on effort: the
  missing fact is *which spelling the harness uses on Windows*, an observation of another program
  on a host this tree has none of, and no command on a Linux box produces it — writing a fold
  without it would be inventing a Windows fact, which is what spec declined to do. →icebox fails
  because a live trigger exists and is dated: the Windows leg `platform-support-ci-matrix` shipped
  before retiring 2026-09-06 still runs on every push to master and is the
  run that can observe it, and the migration that just landed made every *other* producer
  dialect-correct, so these three are now the tree's recorded exception rather than part of a
  uniform unfixed background.
  **Standing exclusion — `lead, own-authority` 2026-09-10 through the lead's message channel, at
  `host-resolution-fail-open-cut`'s scope:** the worklist reads the retired slug and not the live
  CI leg `install-smoke-windows`, so it scores this entry false-eligible;
  `icebox-trigger-blind-to-retired-carrier` owns that predicate defect, DECLINED as a rider then
  with the exposure accepted in writing.
  not-icebox-eligible: harness-project-dir-fold-dialect-unresolved 2026-09-10 live CI-leg trigger
  **Owner is context-kit, not gate-sdk.** The rule's home is context-kit/SPEC.md §Layout and
  configuration; the dialect contract is gate-sdk's. It is that seam, not a migration defect.
  **Pre-existing, not a regression** — the fold is already wrong on a backslash-spelled root
  today, so `msys-dialect-migration` discharged its whole deliverable without answering this.
  **Cost while deferred:** low today and stepwise later — no Windows adopter exists pre-launch, so
  the wrong fold silently disables a memory check nobody is running; it becomes reader-visible the
  first time a Windows session opens, which is the same event that supplies the answer.
  **Deliverable:** the observed harness spelling recorded as a fact with its witness, one fold
  that produces it on both substrates, and a fixture pinning the cross-substrate agreement.
  Filed 2026-08-30 by close, promoted from the gap inbox (spec filed it; the three sites carry a
  recorded `spec:` verdict naming the open question rather than an invented answer).

- **record-stamp-encoding-compression** [cost: event/low] [surface: queue-kit] — buy discrimination in the queue's
  record stamps by RE-ENCODING them rather than by adding text, the deferred pool's per-entry
  budget being what makes added text the wrong trade.
  **Operator-ruled 2026-09-01, and the ruling picked a route none of the three escalated options
  offered.** The escalation asked how to disambiguate two same-day recurrences and proposed, among
  others, an iteration slug beside the date. That was REFUSED: adding a field spends the budget the
  format is trying to protect. The worked example given is `YYYY-MM-DD` → a dashless `YYMMDDHHMM` —
  **the same ten columns, now carrying hour and minute** — which discriminates same-day instances
  outright and needs no slug. Array notation for multiple stamps is named as a further step, and
  the direction is stated to generalize to other task-record components rather than to
  `recurrence:` alone.
  **The envelope is two knobs and compression pays on both axes**, which is why this is not a
  tidiness argument: `QUEUE_KIT_WRAP_BUDGET=100` and `QUEUE_KIT_ENTRY_LINE_CAP=50`
  (queue-kit's in-crate knob defaults) bound columns and lines separately, so a shorter stamp frees
  columns directly and freed columns let prose reflow into fewer lines.
  **The column axis is WITNESSED THREE TIMES, all measured, none projected.** 2026-09-01: `/spec`
  blocked outright — `native-gate-port-remaining-corpus`'s lead line could not hold two `spec:`
  refs under 100 columns, over by two at any legal naming, and the lead ruled around it.
  2026-09-03: the same wall forced MINTING a second host, `drift-kit-bin-port-residue`, fissioning
  the port corpus into per-directory hosts for an encoding reason — a host that emptied and left
  the queue by 2026-09-05, so the wall outlasted the structure it forced.
  2026-09-04: FOUR cuts want four refs against a 66-column base that holds ONE — two shortest
  legal refs measure 117. Two more hosts REFUSED; four per-cut Done-bound entries taken instead.
  **The gain is the ENCODING, not the list, and the entry says so because the format already has
  the list.** queue-kit/SPEC.md:440-442 already defines
  `recurrence: <slug> <YYYY-MM-DD> [<YYYY-MM-DD>…]`, multiple dates on one line today.
  **A second interaction dissolves with it.** queue-kit/SPEC.md:449-456 grounds the self-naming
  slug field partly in `check-queue-hygiene` rejecting exact-duplicate lines, naming same-day
  recurrence on two entries as "exactly the case the declaration exists to record". Under a
  minute-bearing stamp those two lines stop colliding at all, so one of that field's two stated
  grounds is retired by the encoding rather than argued against.
  **The costs, probed rather than listed, because a reader meeting this cold should price it.**
  Date stamps span `recurrence:` and `ruled:` declarations, filed-prose provenance lines, gap-inbox
  bullets, survey-record headings and WORKFLOW-STATE stamps; the evidence manifest's trailing date
  field is OPTIONAL and so is not a cost, correcting the relayed picture. FOUR crate gates carry a
  date predicate (`stage_evidence.rs`, `stage_entry.rs`, `gap_inbox_neutrality.rs`,
  `evidence_manifest.rs`, the first two spelling their own `is_date`), and SEVEN shell tools stamp
  `date +%F` outside fixtures and smoke, none of which stamps a time today. `YY` also drops the
  century, a deliberate trade rather than an oversight to find later.
  **Why design-pending:** the ruling fixes the DIRECTION and not the grammar. Open: which
  components take the new encoding and in what order, whether the change is a migration or a
  read-both-write-new window, and what each date-reading gate asserts across it — a wrong answer
  reds every governed surface at once.
  **Cost while deferred:** low and bounded, and it is the cost of the thing it replaces — every
  entry that needs discrimination keeps buying it with text against a budget that already blocked
  one stage this iteration.
  recurrence: record-stamp-encoding-compression 2026-09-03
  Filed 2026-09-01 by close under CLAUDE.md §Housekeeping's operator-directed exception, staged and
  committed in one motion. FILED AND NOT BUILT: it rides no cut, no iteration since has shared its
  surface (scope's composition test, re-grounded 2026-09-11), and this is no hotfix.

- **docs-cmd-retired-path-blind-to-queue** [cost: event/low] [surface: canon-kit]
  — canon-kit/SPEC.md §check-docs-cmd assertion (C) cannot see a retired path cited from the queue,
  from two sides: its corpus is the manifest set, which excludes TASK-QUEUE.md, and a
  `<path>:<line>[,<line>]` token fails the path shape because `:` is outside the segment class.
  **Attested once:** two Deferred bodies cited deleted kit libraries in that form after
  `config-seam-second-cut` and stayed green; that close found them by grep and corrected them.
  **Why design-pending:** the candidate trims a trailing line suffix before the shape test and
  reads the queue's live sections, and the alternative is a stated reason the queue stays out.
  Which, and whether the queue's retired-work citations then need an exemption, is the call.
  **DISTINCT from the iceboxed `queue-citation-line-number-stales-within-its-own-session`**, where
  the cited file lives and only its line number drifts; here the whole path is retired.
  **Cost while deferred:** each kit-library retirement can strand a queue citation that reads live
  until a hand grep finds it.
  Filed 2026-09-14 by `config-seam-second-cut`'s close into the gap inbox, from its
  stale-identifier-after-retirement audit; promoted at this iteration's scope.

- **manifest-files-configured-branch-unpruned** [cost: event/low] [surface: canon-kit]
  — `spec::manifest_files`' configured branch (`CANON_KIT_MANIFEST_FILES` non-empty) applies no
  prune set, where the default branch's walks prune `GATE_SDK_PRUNE_DIRS` and canon-kit/SPEC.md
  §The shared spec adapters rules that `comment_surface`'s configured branch narrow exactly as its
  default does. So a configured value cannot say `**` without admitting `gate-tests/` fixtures;
  this repo's single-level globs in `scripts/canon-config.knobs` stand in for the prune.
  Verified 2026-09-15 at close: the configured branch is `glob_files` plus an `is_file` test, with
  no `path_pruned` filter.
  **Why design-pending:** the repair narrows a corpus shared by roughly ten readers
  (`check-md-refs`, `check-docs-cmd`, `check-manifest-count`, `check-prose-enum` and the claim
  gates among them), so canon-kit/SPEC.md §The causal-completeness check item 5 binds: each
  reader's red condition is enumerated, a zero-count reader such as `check-install-claim` being
  the attested inversion. Whether any consumer relies on the unpruned form is unprobed.
  **Cost while deferred:** a consumer widening the knob with a multi-level glob governs fixture
  prose as manifest content, or enumerates single-level globs around the gap; and both configured
  expansions (`CANON_KIT_MANIFEST_FILES`, `CANON_KIT_PROSE_SURFACE_GLOBS`) still call
  `walk::glob_files`, so a `**` there stats every entry under `target/` and a concurrent cargo
  build can exit-2 every manifest reader on a clean tree. Repair route: `walk::glob_corpus`
  (gate-sdk/SPEC.md §The port-candidate criteria). Verified 2026-09-19 at close:
  `native/src/spec.rs` lines 197 and 215.
  Filed 2026-09-15 by `couples-field-semantics`' spec into the gap inbox; drained and promoted
  2026-09-15 at close.

- **lead-line-blocked-by-spec-tag-width-collision** [cost: event/low] [surface: queue-kit]
  — an active lead line cannot carry both a spec tag and a blocked-by tag once the slugs are
  long, because queue-kit/SPEC.md §check-queue-wrap discounts only a deferred lead line's cost
  and surface tags, and refuses discounting the repeating blocked-by tag.
  **Attested once:** promoting `couples-dynamic-root-resolution` needed 38 columns for its bold
  lead, 45 for the blocked-by tag naming `couples-glob-semantics-unowned` and 18 for the shortest
  legal spec tag, 101 against the 100-column budget before any prose (recounted at close). The
  spec stage dropped the blocker tag and carried the merge order in the amendment header, so the
  structured blocker a first-unblocked selection reads was lost for the promoted dependent.
  **Why design-pending:** a bounded spec-tag discount, a continuation-line home for the
  blocker (which §check-tag-lead-line forbids today), or a slug-length cap are each a grammar call.
  **DISTINCT from `icebox-eviction-line-budget-squeeze`**, which is the icebox tier's one-line
  grammar against the same cap. Both share the unbounded slug length, so a scope may bundle them.
  **SECOND ATTESTATION 2026-09-20, and it WIDENS the premise above rather than repeating it.** This
  scope promoted `readme-front-door-is-adopter-facing-and-outside-every-sweep` with NO spec tag at
  all, and the blocked-by tag alone measured 130 columns against the budget — so the collision needs
  only two long slugs, not a spec tag beside the blocker as the lead line here still says. Same
  disposition as the attested case, for want of another: the tag was dropped and the block written
  into the dependent's prose, where no selection reads it.
  **Cost while deferred:** a dependent promoted beside its blocker loses the tag its selection
  reads, and the order survives only in amendment prose.
  recurrence: lead-line-blocked-by-spec-tag-width-collision 2026-09-20
  Filed 2026-09-15 to the gap inbox at that spec stage, measured by `check-queue-wrap`'s red at its
  promotion commit; drained and promoted the same day at close.
  **Reshaped by operator direction 2026-09-22 (lead-relayed), into `queue-kit-unwrap`:** with tags
  on a heading's body and no column cap, the collision has no budget left; `/spec` confirms it
  discharges with the hard-wrap and entry-shape amendments or authors the residue.

- **close-surface-row-trackedness-undeclared** [cost: event/low] [surface: lifecycle-kit]
  — a close-surface roster row does not say whether its path is tracked or gitignored, so a
  session deciding whether that row's read is delegable to an isolated agent re-derives the bit
  by hand. Isolation cost (3) makes that bit decide delegability outright: an untracked or
  gitignored corpus is not delegable at all to a type held to isolation.
  **The filing premise was corrected at the drain, and the correction changes the cost.** The
  bullet claimed the emitter already holds a `git check-ignore` verdict per path and drops it.
  It does not: `native/src/emit/close_surfaces.rs` calls `check-ignore` only inside the
  workflow-directory walk that mints `(undeclared)` rows, and its exit-1 arm `continue`s, so the
  verdict exists for gitignored workflow-dir members alone. Rows harvested from declaration
  surfaces get no call at all. A fifth tier field is therefore a widening that adds one
  `check-ignore` per declared row, not a free print of a fact already held.
  **Why design-pending:** whether the field names the tier, the trackedness bit, or nothing
  at all — the constraint living in the protocol template instead — is a grammar call, and
  `check-close-surfaces` reads the same rows.
  **DISTINCT from `delegated-read-blind-to-gitignored-capture`, Done 2026-09-16**, which ruled
  the general constraint onto the protocol template; this is one emitter withholding a fact its
  reader needs, and it stands whether or not any sweep is ever delegated.
  **Cost while deferred:** every delegating session re-derives the bit, and one that skips the
  derivation delegates a read returning absence for content.
  Filed 2026-09-16 to the gap inbox by the spec stage, weighed as that iteration's second
  candidate owner and refused there for repairing one instance of a general class; drained and
  promoted at close, which falsified half its premise.

- **local-only-files-write-back-untriggered** [cost: event/low] [surface: lifecycle-kit] — the
  consumer's local-only companion files have read triggers at three skills and no write-back
  trigger. A consult audit found the private brief still carrying forward memory for two shipped
  rungs, its truncate-on-landing rule run by no stage (close's lesson drain routes only
  private-rule discards there); the ops runbook's desired-state verifier had no recorded run and
  no tracked trigger; and `check-queue-slug-liveness` reads the prose-surface globs by bold code
  only, so a plain-code retired slug on a local-only surface passed every battery.
  **Owed:** (a) a close drain step truncating the consumer's private-brief forward memory for each
  unit shipped, as a template slot naming the consumer's surface; (b) a slot in release-sweep or
  close's audit classes running the consumer's out-of-tree state verifier, slot-bound and never a
  path literal; (c) weigh a retired-slug arm over plain code on the local-only globs.
  **Refused in the consult:** leaving `/consult` as the only maintenance channel.
  **Held Deferred by the enhancement admission filter** (TRAJECTORY.md §The rulings): new
  template slots are an enhancement that cuts no time-to-first-value, closes no trust gap and
  produces no external proof.
  **Cost while deferred:** local-only surfaces drift until a consult happens to audit them.
  Filed 2026-09-18 to the gap inbox by the consult that audited the local-only files, after
  `external-install-evidence`'s close; promoted to Deferred at the next scope.

- **install-smoke-powershell-demo-runs-before-bash-strip** [cost: event/low] [surface: .github] —
  the `install-smoke-powershell` leg runs `checkwright demo` (the `full` battery) in the step BEFORE
  the one that strips every bash from PATH, so the full profile on Windows is witnessed with bash
  present and never without; the no-bash step exercises only the starter init's hooks.
  **Re-verified at this scope:** in `.github/workflows/gates.yml` the demo block sits in the init
  step, ahead of the step named "commit through the generated hooks with no bash on PATH"; gates
  run 35632734000 (`58908d02`) showed both green in that order. installer/SPEC.md §demo claims only
  that the leg runs the verb on a PowerShell adopter's path, so no tracked sentence is false.
  **Deliverable — rule one of two:** run the demo after the strip (or a second demo there), or
  state in installer/SPEC.md §demo that the Windows oracle holds bash on PATH.
  **Why promoted, not fixed:** moving the demo is a Windows step whose verdict is unknown until a
  Windows run — a `full` member spawning bash would red it — and the other limb narrows a claim.
  **Cost while deferred:** a `full`-profile gate needing bash on Windows ships unseen.
  Filed 2026-09-21 to the gap inbox by `docs-first-contact`'s close, reading its push run; promoted
  at the next scope. Owner lookup: `install-smoke-powershell`, `demo`, `bash stripped` — the two
  entries naming that leg own its name and its exemption keying, not its step order.

## Icebox

  Dormant entries, one line each: the cost field said the carry was low, no
  `[roadmap:]` commitment rides on it, and no named event is waiting to
  promote it. Still live work — a legal `[blocked-by:]` target, conserved on
  the way in and on the way back out. The removed body is recoverable from
  the evicting commit (`git log -p -S'<slug>' -- TASK-QUEUE.md`).

- **fixture-suites-never-run-history-less** — No leg runs fixtures history-less.
- **lead-held-block-no-sanctioned-surface** — No route records a lead-held block.
- **agent-file-paragraph-sections-ungoverned** — Brevity gate skips paragraphs.
- **survey-record-filed-after-the-fact** — Order to the work goes wholly unread.
- **amendment-prose-misnumbers-its-delta** — Cites Delta 3 for delta 4's subject.
- **battery-timing-file-overwritten-by-only-run** — A filtered run reports as all.
- **worktree-dispatch-rebuilds-the-gate-binary** — Each dispatch pays a cold build.
- **committed-grant-fallthrough-unexplained** — Three bare grants fell through.
- **derived-count-literal-in-queue-unscanned** — No corpus reaches the queue file.
- **bridged-arm-spawned-program-set-unheld** — Declared set unheld; shape ships.
- **icebox-drops-a-bought-census** — No dormant home for a measured payload.
- **surplus-arg-drop-in-six-emit-arms** — Six emit arms drop surplus args at 0.
- **inline-source-literal-ungateable** — Fence-only oracle; no rename pending.
- **turn-end-refusal-used-as-a-busy-wait** — Sessions busy-wait via the stop hook.
- **site-health-probe-no-retry-on-transient** — A single non-200 files an issue.
- **non-gate-arm-roster-hand-maintained** — The arm class's flag list is ungated.
- **craft-rule-step-has-no-reader** — A broken stage-rules knob reds nothing.
- **runtime-dir-two-tier-detector** — No two-tier proof for file-pattern ignores.
- **done-slug-commit-naming-gate** — Done-moving commits need not name their slug.
- **enter-stage-simulate-no-write-fixture** — Guard present, unpinned by a fixture.
- **stage-lag-disambiguation** — Hook over-firing is accepted, not a defect.
- **metric-dir-admission-unstated** — Ad-hoc scripts persist in .metric/.
- **stage-economics-smoke-jq-arm-dormant** — Its jq-absent arm never runs anywhere.
- **hermetic-bin-roster-config** — Pinning coverage needs a consumer roster seam.
- **split-posture-waiver-writer** — A lead-issued waiver stamp has no writer.
- **supervisor-verification-attestation** — The verification duty is unattested.
- **gate-spec-claim-assertion-parity** — Ruled a human-audit class, not gateable.
- **port-takeability-has-no-instrument** — Takeability hand-read on the tree axis.
- **scope-amendment-authoring-gate** — Scope can do spec's job and stay green.
- **evidence-journal-hash-chain** — Tamper-evidence wanted only by a hosted rung.
- **md-section-near-miss-match** — Empty on a near miss; correct on an exact query.
- **operator-authored-unit-set** — The contract omits operator-authored unit sets.
- **tarball-build-attestation** — The checksum proves transfer only; docs agree.
- **action-run-shell-scan-predicate** — No consumer seam on a correct gate.
- **scratch-execution-allowlist-bar** — Each close re-derives this standing bar.
- **gate-tamper-consumer-gate-coverage** — A glob and a roster audit remain.
- **upgrade-contract-rename-routing-unstated** — One clause leans on it.
- **md-refs-tree-link-resolution** — Unreachable while one generator produces.
- **recurrence-judgment-vs-declaration** — The two share a noun, not a meaning.
- **advisory-lane-draft-state-unswept** — GitHub's notifications are the sweep.
- **amendment-done-move-assertions** — Zero cost while merges are hand-checked.
- **guard-advise-jq-dependency** — Needs jq; the one consumer works around it.
- **survey-record-extension-tier-hybrid** — Paid only by a future workflow author.
- **install-lifecycle-reversibility** — A declined branch; only optionality owed.
- **pack-installer-payload-kit-set-anchor** — Latent --root trap, no caller.
- **installer-jq-usability-probe** — Broken-but-present jq is unobserved.
- **rendered-site-link-monitor** — Rendered-site link rot waits on a launch crawl.
- **kit-index-page-vocabulary-ungated** — Index-page enums are ungated.
- **absence-statement-grammar** — When to state absence, and how, is unruled.
- **contributor-writeback-disposition** — Write-back is dormant pre-launch.
- **context-pressure-signal** — Compaction timing has no per-session signal.
- **post-immutability-machine-read-carveout** — Immutable prose, live machine read.
- **path-pinned-allow-entry-oracle** — No scanner reds a path-naming grant.
- **price-table-roster-coverage-oracle** — An unpriced model id reds nothing.
- **economics-posture-binding-stale** — A shim restates a ruling it should cite.
- **align-context-draw-growth** — Two falls read the draw as work-side.
- **customer-facing-iteration-cadence** — No tracked classifier for the bound.
- **scan-prompts-truncation-quote-desync** — Truncation inflates the scan only.
- **template-out-of-tree-copy-obligation** — Out-of-tree copies are unreachable.
- **queue-entry-grammar-single-owner** — Two entry grammars disagree, latently.
- **installer-artifact-omission-residue** — An omission update strands a binary.
- **installer-graph-artifact-literal** — Init literalises a resolver-owned path.
- **doctrine-rule-number-citation-liveness** — A renumber stales citations.
- **false-ground-citation-propagation** — Nothing re-reads a ground once cited.
- **spec-embedded-source-criterion-4-membership** — Its port sizing stays unruled.
- **lead-dispatch-simulate-optionality** — Dispatch may skip the pre-flight.
- **self-repo-prefix-normalisation-unheld** — Two link-prefix holders, unheld.
- **stage-cursor-rerun-stamp-gap** — A skipped re-run stamp points the cursor back.
- **interpreter-grant-redirect-residue** — Seven redirected shapes stay ungranted.
- **canonicalize-extended-length-prefix** — A Windows `\\?\` root is unconverted.
- **stage-cursor-unread-by-index-check** — A clean index hides who holds the stage.
- **crate-arms-relink-under-worker-pool** — It relinks the binary it runs in.
- **build-native-obligation-unconditional** — A crate-free commit still rebuilds.
- **port-blockers-library-mediated-scan** — A library-mediated spawn reads clean.
- **bridged-arm-requirements-undeclared** — `--needs` omits what an arm spawns.
- **delta-citation-unresolvable** — A delta number names no openable file.
- **scratch-grant-backtick-declined** — Rule 17's own clause voids its use case.
- **walk-entry-model-unstated** — Walk drops symlinks unstated; tree has none.
- **prune-set-matches-walk-root-ancestors** — A leaf above the root prunes it all.
- **evidence-baseline-orphan-suite-row** — A row for a retired suite is unread.
- **port-archaeology-restatement-residue** — Prose narrates deleted shell forms.
- **non-gate-arm-testing-floor-unstated** — A new arm's testing floor is unstated.
- **prune-set-convergence-question** — Two kits' prune sets diverge, unruled.
- **gap-inbox-slug-predicate-ground** — Its anti-cycle premise died unreplaced.
- **emit-arm-usage-unreachable** — Prints only on a refusal; lead-ruled 2026-09-03.
- **check-graph-trigger-consumer-path-reach** — couples= misses installer/.
- **precondition-gate-negation-false-positive** — Reds a true negated precondition.
- **worktree-isolated-dispatch-cannot-reach-the-main-checkout** — Bridge undecided.
- **cited-object-token-sweep-corpus-narrower-than-the-class** — Corpus unruled.
- **worktree-lock-start-time-guard-untaken** — Dormant until a consumer acts on it.
- **worktree-cleanliness-assertion-scopes-to-checkout** — Reds on foreign dirt.
- **release-record-retired-knob** — A removal's basis may not name its own knob.
- **friction-key-segment-selection-unruled** — Which segment to key is unruled.
- **scratch-auto-allow-no-decoration-steer** — Chained writes lose the steer.
- **cost-series-limb-unreadable-inside-close** — Close cannot price its open row.
- **post-build-instrument-edit-unowned** — No stage owns a post-build tree edit.
- **dod-size-figure-stales-in-iteration** — Spec's promotion ages its own DoD size.
- **portability-count-on-two-surfaces** — Hand-spelled census; both true today.
- **wrap-budget-caps-lead-line-tags** — A long slug's tag neither fits nor wraps.
- **smoke-roster-guard-precedes-hand-off** — Guard stricter than its stated reason.
- **edges-retired-block-name-clash** — A live gate name inflates a retired slug.
- **lead-report-is-an-ungated-terminal-act** — May close holding unfiled work.
- **smoke-report-array-carrier-mangling-unexplained** — Witness now needs design.
- **gate-timing-baseline-comparability** — Timing baseline has no comparer.
- **amendment-landing-citation-assertions** — Landing citations go unvalidated.
- **root-doc-roster-registration-parity** — Only one root-doc roster is enforced.
- **enforcement-first-load-trigger** — No stage loads the enforcement-first rule.
- **self-revert-reminder-expectation** — Self-revert reminder reads as injection.
- **co-authored-by-trailer-attribution** — Model trailer is a baked literal.
- **guard-steer-grant-mismatch** — Tree steers unpaired; the kit's are templated.
- **reclaim-precondition-outside-the-tree** — Essay-sink reclaim can never fire.
- **release-drain-ordering-contradiction** — Step 4 opener contradicts its body.
- **amendment-dod-sibling-dependence** — DoD items depend on unnamed siblings.
- **recurrence-resolver-literal-match-only** — Unspelled recurrences file as new.
- **section-prose-outlives-its-entries** — Section preambles outlive Clear-Done.
- **amendment-roster-stale-by-construction** — Sweep rosters stale mid-iteration.
- **unregistered-gate-fixture-coverage** — Unregistered gates skip fixture duty.
- **queue-tier-label-correction-cost** — Fixing a label at the cap costs a trim.
- **rejected-compound-commit-relabel** — A bare retry mislabels staged work.
- **survey-record-supersede-invisible** — Superseded survey blocks look live.
- **consumer-smoke-accounting-spelling-unpinned** — Dual-spelling count unpinned.
- **release-runbook-identity-diagnosis** — Account check is prose, not a step.
- **dispatch-cited-evidence-unverified** — A sweep's quotations go unverified.
- **queue-provenance-restates-git-history** — Provenance prose restates git log.
- **comment-tier-surface-excludes-ci-workflows** — Workflow comments go ungated.
- **consult-rulings-outside-the-authority-roster** — Consult readings lack a slot.
- **lead-ruling-reopen-authority-unstated** — Who reopens a lead ruling is open.
- **ruling-record-prose-staleness-unreachable** — Old rulings evade the probe.
- **close-surface-reclaim-uncoupled-from-read** — Reclaim may wipe unread rows.
- **iceboxed-recurrence-judgment-unrecordable** — No room for a recurrence stamp.
- **post-scope-admission-has-no-promotion-route** — Late debt has no promoter.
- **declaration-shape-outside-header-unreadable** — Inert literals read as live.
- **boundary-preserve-covers-names-not-lifetimes** — Keep-list lists names only.
- **validate-suite-wall-clock-unowned** — Serial smoke suites cost ~16 minutes.
- **overlay-only-oracle-grants-uncommitted** — Oracle grants live off-tree.
- **close-triage-log-reclaim-loss-window** — Truncate after read drops appends.
- **nested-battery-env-inheritance-invisible** — A scoped nested run reads clean.
- **enforcement-first-behavioral-regressions** — Rule under-cues behavior gates.
- **spec-split-promotion-review** — Spec-stage default awaits an economics read.
- **build-stage-tier-economics** — Build tier set by intuition, not a priced A/B.
- **supervision-overhead-unmeasured** — Supervision burn priced; quality unread.
- **gate-battery-result-cache** — Battery reruns all gates on an unmoved tree.
- **state-representation-integrity** — Text-state invariants are gate-held only.
- **rule-reach-before-merits** — Merits argued before a rule's reach is set.
- **template-copy-parity-yaml-widening** — YAML template copies mirror by hand.
- **gate-tests-suite-identity-in-evidence** — Two suites can share one hash.
- **template-spec-restatement-reach** — No gate holds a SPEC off its template.
- **amendment-deletion-content-completeness** — Merges can drop rationale unheld.
- **template-registry-population-predicate** — A name collision would red parity.
- **lead-line-parser-conformance** — Eight lead-line holders; no conformance.
- **breadth-declaration-stale-listing** — Spent breadth declarations stay silent.
- **breadth-declaration-committed-glob-home** — Glob keep-rulings have no home.
- **criterion-4-two-spellings-disagree** — Criterion 4 reads two ways.
- **substrate-parity-assertion-c-reach-unannounced** — C can shrink unannounced.
- **promotion-commitment-stamp-latency** — At-ceiling stamps wait on promotion.
- **recurrence-threshold-counts-dates-not-incidences** — Same-day firings merge.
- **same-stage-journal-append-uncoordinated** — Parallel appends share one file.
- **survey-engagement-trigger-narrower-than-its-class** — Trigger is scope-only.
- **recurrence-declaration-grammar-ungated** — Declaration grammar has no gate.
- **scratch-citation-introducer-form-reach** — Copula pointers evade the scan.
- **threshold-entry-escalation-travel-unruled** — Rider or competitor, unruled.
- **one-motion-commit-race-remains-open** — add-then-commit still races.
- **amendment-target-delta-correspondence-unverified** — Orphans in both ways.
- **dispatched-child-asserts-an-unverified-base** — Children infer, not probe.
- **hermetic-harness-export-masks-the-condition-under-test** — Pins void arms.
- **verbose-battery-idiom-steered-to-its-granted-spelling** — Bare form prompts.
- **exe-suffix-single-spelling-unenforced** — Suffix owner claim has no gate.
- **intra-stage-batch-stamp-unobserved** — A skipped batch stamp goes unseen.
- **boundary-sweep-github-write-skips-identity-step** — Account check unenforced.
- **wait-primitive-and-record-compose-to-false-completion** — Waiters exit early.
- **readonly-dispatch-type-cannot-see-gitignored-surfaces** — No ignored files.
- **kfric-second-field-direction-inverted** — Surface field names the owner.
- **baseline-self-certification-unasserted** — Self-served verdicts unasserted.
- **wait-mandate-template-spelling-unreachable** — Mandated spelling is refused.
- **upgrade-smoke-refuses-inside-a-worktree** — Refuses where .git is a file.
- **pre-grammar-disposition-authority-ambiguity** — Double-named rulers unread.
- **kpi-cost-per-unit** — No KPI prices cost per shipped unit.
- **line-range-citation-stales-inside-its-own-iteration** — Line ranges go stale.
- **amendment-commit-shape-red-conditions** — Prompt lacks a commit-shape class.
- **amendment-correction-density** — Correction density goes unmeasured.
- **probe-evidence-sufficiency** — Rule 12 passes a probe that is no evidence.
- **scratch-citation-skill-surface-reach** — Skill files escape the pointer scan.
- **throughput-and-wait-time-unmeasured** — Wait and throughput are unmeasured.
- **headroom-check-ordering-unruled** — When to read cap headroom is unruled.
- **dispatch-unreadable-target-fallback** — Blind sweeps echo the prompt as PASS.
- **queue-write-side-verb** — The queue has no write-side verb.
- **close-red-push-ownership** — No owner for a close blocked by a red push.
- **expected-permission-mode-undeclared** — No surface states the expected mode.
- **consumer-guard-rule-coverage** — Consumer-only guard rules are untested.
- **scan-prompts-blocking-half-blind** — The KPI cannot see blocked commands.
- **handoff-premise-reverification-placement** — Premise-check placement unruled.
- **amendment-work-class-label-placement** — Work-class tag placement unruled.
- **tracked-to-untracked-pointer-scope** — Untracked-target pointer scope unheld.
- **born-native-flip-enforcement-gate** — Born-native rule has no enforcing gate.
- **msrv-move-clippy-arm-coupling** — Floor moves surface unbudgeted lints.
- **declaration-lib-refusal-output-leak** — Refusal output mixes in good tokens.
- **deferred-pool-identifier-restatement-sweep** — The pool was never swept.
- **fixture-assertion-liveness** — Stale fixture expectations go uncaught.
- **fixture-assertion-coverage-unmeasured** — Driver coverage of arms unmeasured.
- **survey-engagement-residue-untracked** — Engagement residue rests on conduct.
- **metric-dir-member-contract-unheld** — The metric dir accretes leftovers.
- **kit-bin-entry-point-unrostered** — No roster maps bin tools to kits.
- **queue-lib-dead-derivation** — Three queue-lib regexes have no live reader.
- **gate-test-in-tree-invoker-ruling** — Is a gate-test an in-tree caller?
- **survey-oracle-liveness-unasserted** — An oracle may name a wiped path.
- **survey-witness-composed-from-unvalidated-corpus** — Prose corpora pass clean.
- **stage-completion-unattested** — Entry stamps cannot show completion.
- **deferred-entry-time-deixis-rot** — Relative deixis rots in deferred bodies.
- **iteration-scoping-clause-date-ambiguity** — Date scoping names no iteration.
- **assertion-strength-exit-header-reach** — The gate now reaches no script.
- **audit-depth-measure-degrades-under-fanout** — Fan-out depth is self-reported.
- **candidate-list-anchors-a-sweep-obligation** — A relayed list anchors a sweep.
- **spec-pointer-boundary-legality** — Resolving targets may be illegal owners.
- **ruled-line-retirement-provenance-census** — Census of the retirement's cuts.
- **amendment-census-claim-unrun** — Amendment counts skip the shipped oracle.
- **composition-test-scores-a-section-cut-as-one-unit** — A cut scores as one.
- **deleted-runner-anchors-across-ten-entries** — Entries cite deleted runners.
- **port-created-failure-mode-refusal-unruled** — Port-made refusals unruled.
- **removal-propagation-site-argued-out-of-scope** — Found sites argued away.
- **wait-form-unallowlistable-by-construction** — No grant reaches the wait.
- **icebox-trigger-blind-to-retired-carrier** — Blind to retired carriers.
- **rationale-located-by-reading-not-by-grep** — Grep misses paraphrases.
- **smoke-whole-tree-precondition-unscoped** — Any dirty path blocks the smoke.
- **append-grant-decline-cause-unlogged** — Decline cause truncated from the log.
- **spec-internal-identifier-prefix-drift** — SPECs cite internal names, not knobs.
- **lint-scope-hook-trigger** — extra lint dirs skip the commit hook, CI-only.
- **release-note-section-set-derivation** — release gate hand-lists note sections.
- **release-asset-claim-class-owner** — a release-ships-an-asset claim has no gate.
- **knob-default-accessor-singularity** — no gate bars re-spelling a knob default.
- **bin-argv-shape-residual-member** — one bin arm skips the argv-shape contract.
- **docs-corpus-derivation-manifest-divergence** — two docs gate corpora diverge.
- **consumer-gate-roster-unread** — no roster reads a consumer-declared gate.
- **waiter-loop-condition-predicate-gap** — waiting rule misses a lone waiter loop.
- **crate-toolchain-grant-uncommitted** — overlay keeps cargo grants ruled out.
- **prose-uniqueness-claim-unchecked** — no gate checks a prose uniqueness claim.
- **kit-spec-layout-tree-hand-maintained** — kit SPEC layout trees are hand-kept.
- **expansion-rule-backtick-blind** — expansion rule misses backtick substitution.
- **fixture-runner-checks-dir-fails-open** — bad checks-dir arg drops silently.
- **enter-stage-refusal-help-contradicts-its-guard** — help contradicts its hook.
- **artifact-substitution-remedy-has-no-end-to-end-arm** — untested end to end.
- **uninstall-artifact-ownership-asymmetry** — uninstall leaves init's artifact.
- **readme-bin-roster-underived** — no gate holds a kit README's bin/ tool roster.
- **align-in-session-absorption-tier-unruled** — operator-class; awaits a consult.
- **push-account-selection-has-an-explicit-per-command-form** — remedy probed, unranked.
- **survey-locator-review-catches-after-loss** — a gate here reopens a stated refusal.
- **lead-agent-id-compaction-defense** — each claim still wants its own probe.

## Done

## Lessons Learned
