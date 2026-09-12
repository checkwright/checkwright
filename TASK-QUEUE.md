# TASK-QUEUE.md — Checkwright work queue

## Iteration: —

  The lifecycle-kit gates read this header's iteration name and the stage
  cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt`
  (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue
  format itself and gates this file. One iteration per hardening or roadmap
  unit; [README.md](README.md) maps the kits.

---

## New Features

## Technical Debt

## Deferred


- **config-seam-static-format** [design-pending] [cost: event/high] [surface: gate-sdk] — the knob seam
  is executable bash: a knob's value is computed by sourcing the owning kit's `lib/*.sh` and the
  consumer's `<gates-dir>/<kit>-config.sh` (gate-sdk/SPEC.md §lib/gate.sh), so the battery, hook
  generation and every harness hook enter through bash before the binary runs.
  **Operator ruling at consult: file it costed.** The `# no-port` audit of 2026-09-12 read the
  oracle at 70 declared, 0 held, 0 owed, and found every declaration sound on its own ground;
  34 of them — the config seam, the kit libraries and the three bridge front-ends — are one ground
  restated: the seam's format. No ruling weighed the format itself. A static knob file the binary
  parses (env or TOML) collapses those three classes and moves each kit's defaults in-crate.
  **The tension it closes.** TRAJECTORY objective 6 rules a surviving script surface
  dual-implementable, bash and PowerShell; the runtime bridge is neither and cannot be, since a
  PowerShell twin is the second producer criterion 6 refuses. Native Windows therefore rides
  Git-for-Windows bash for every battery run and tool call, while the PowerShell half covers
  install alone. Measured: the bridge costs under a second per battery and 50 ms per guard
  call, so the case is platform, never speed.
  **Why [design-pending]:** the format must express arrays, keyed pairs and the seven `*_CMD`
  command strings; the 92 fixture suites source `lib/test-hermetic.sh` in-shell; and the harness
  guard's rule content, now bash, becomes data or stays the one shell hook.
  **Refused:** a PowerShell twin of the bridge (second producer); porting the 34 files under the
  present seam (each ruling's ground holds while the seam is bash); a generated knob projection
  (refused at gate-sdk/SPEC.md §gen-pre-commit on per-fixture grounds).
  **Cost while deferred:** objective 6 reads false on the runtime path and no surface says so;
  every native-Windows adopter carries a bash floor the install docs attribute to git alone.
  Filed 2026-09-12 by consult, an operator-directed direct entry.

- **harness-moved-background-task-unrecorded** [design-pending] [cost: event/high] [surface: delegation-kit] — a command the
  harness moves to the background on its timeout is a live producer no liveness record names.
  **Split out of `backgrounded-shell-child-run-record-unenforced` — operator direction, 2026-09-11,
  lead-relayed**, since landed, which kept the explicit-launch block and the recorded-launch grant.
  **Attested twice.** At `installer-trial-lifecycle-repair`'s close, `stage-economics.sh` exceeded
  its foreground timeout, the harness backgrounded it, and it kept writing `.metric/` with no `.run`
  record; no session act could have written one, since the launch was never a session act. Spec
  reproduced the move on 2026-09-11: the result reads "moved to the background (ID: …)", with its
  output under the session scratchpad.
  **Why no `PreToolUse` rule reaches it.** The call was a foreground call when the guard saw it, so
  guard-kit rule 15 has no launch to refuse and rule 14 has no record to read. No `PreToolUse`
  payload carries task state, and a search of that session's transcripts found no line recording
  the move.
  **Candidate, operator-class:** widen delegation-kit's turn-end hook to refuse while its payload's
  `background_tasks` array shows a running harness `shell` task. §What `background_tasks` carries
  rules that view a supplement to the record set and never a substitute, and it holds no pid; the
  hook's refusal set took a separate authorization (§The turn-end liveness hook), so widening it is
  the operator's to rule.
  **Why design-pending:** whether a finished task leaves the array, and how often a running task at
  an intermediate `SubagentStop` would refuse, are both unmeasured.
  **Cost while deferred:** a harness-moved producer can outlive its session, and a commit can land
  beside it; rule 14 and the stage-entry liveness check see neither, because nothing recorded it.
  Filed 2026-09-11 by spec on the operator's split direction.

- **payload-withholds-kit-specs** [design-pending] [cost: event/high] [surface: installer] — the customer payload packs every kit root
  whole (`git archive` per root at `native/src/emit/pack_installer.rs`), so the kit SPECs ride
  along as its bulk. **Operator ruling at consult, recorded in TRAJECTORY.md under this slug:**
  the payload withholds each kit's SPEC and its `smoke/`, and a shipped gate's pointer resolves
  to the site's per-kit SPEC mirror under docs/.
  **Measured, not reasoned.** Tracked payload bytes: 4.51 MB. The eleven kit SPECs: 2.65 MB,
  38,653 lines, 59% of it; gate-sdk's alone 1.2 MB and 17,224 lines, of which the porting section
  is 5,406 and 50 of its 103 sections are cited by no shipped file (53 are; 73 more only from
  the crate, which never ships). The smoke scripts refuse a bare invocation by their own header
  and run only this repo's suites.
  **Not load-bearing for a customer:** canon-kit's finders prune vendored kit roots, so the
  spec-pointer gate never resolves a vendored gate's pointer; what explains a red offline is the
  descriptor's one-line invariant, which ships. This tree keeps its SPECs; nothing here changes.
  **Deliverable, five parts.** (a) A packer exclusion — kit-root SPEC and smoke, a declared shape
  applied per root inside the tracked-set copy, never a per-kit list. (b) The amendment to
  gate-sdk/SPEC.md §Consumer payload: the second member becomes the pointer, resolvable on the
  published mirror, with the restatements at docs/install.md §What a gate discloses and
  installer/README.md §What this package is corrected in the same unit. (c) Kit README links to
  their own SPEC (gate-sdk 3, lifecycle-kit 11, canon-kit 1, delegation-kit 1) point at the site.
  (d) The site keeps the mirror as an unlinked reference tier reached from a pointer, and its
  front nav leads with install and the per-kit READMEs. (e) The consumer smoke asserts a
  vendored tree carries no SPEC.
  **Why [design-pending]: two shapes are open.** Whether the exclusion is a packer literal or a
  per-kit declaration of non-shipping paths; and whether the pointer line gains a URL form or the
  site resolves a kit path plus heading by convention.
  **Refused, with grounds:** a generated extract of only the cited sections — offline reading for
  a reader the site already serves, at a projection plus a freshness gate; keep shipping whole
  SPECs — 59% of the payload for that same reader, and the seam-leak surface
  `kit-spec-seam-content-half-unswept` and `kit-spec-consumer-config-literal` track stays
  customer-visible.
  **Cost while deferred:** every tarball and npm package ships its engineering record as its
  bulk, and the seam-leak surface stays in customers' trees.
  Filed 2026-09-11 by consult, an operator-directed direct entry.

- **installer-readme-usage-tier-split** [design-pending] [cost: event/high] [surface: installer] — `installer/README.md` is the
  npm-visible package README and a 3,182-line, 216 KB design record, so the package page and the
  tarball lead with mechanism grounds where usage belongs.
  **Operator ruling at consult: split it.** A short README — requirements, quick start, the verbs
  table, profiles, and where the design lives — and the design record renamed to the installer's
  own SPEC. Seventeen files carry pointers into fourteen of its sections, a mechanical rename
  sweep (delegable); CLAUDE.md §Housekeeping's layout pointer and docs/install.md's references
  move with it.
  **Why [design-pending]: the usage tier's owner.** docs/install.md already carries quick start
  and the verbs, so the README either points at the site (content-tiering: point, never restate)
  or carries the copy an offline npm reader needs; and an installer SPEC joins canon-kit's
  governed spec set by name, which is right but changes which gates read it.
  **Cost while deferred:** every npm page view and tarball reader meets a design record at the
  activation surface's front door, against the time-to-first-value objective.
  Filed 2026-09-11 by consult as a direct entry, beside `payload-withholds-kit-specs`.

- **config-variant-battery-harness** [design-pending] [cost: event/high] [surface: gate-sdk] — nothing shipped lets a customer run the
  battery under a named config-seam variant and see what changes; the fixture pairs prove each
  gate's arm against fixed trees, and the smoke scripts are this repo's harness legs.
  **Operator ruling at consult: file it costed.** Deliverable: a shipped, bridged arm that takes a
  scratch copy of the consumer's tree, applies a named variant of the config seam, runs the
  battery, and prints the per-gate verdict diff against baseline — so an adopter sees which gates
  a knob arms, disarms or reds before committing the knob.
  **Why [design-pending]:** the variant's declaration form (an env file or a config-dir overlay),
  whether the scratch copy is a worktree or a copy that carries untracked content, and whether
  the diff or a full report is the product. Native, per the interpreter objective.
  **Refused:** repurposing the smoke scripts, which are install recipes read as text by the
  install-disposition gate and bound to this repo's harness; a shipped directory of shell tests,
  which widens the interpreter surface the objectives shrink.
  **Cost while deferred:** an adopter evaluating a knob edits the seam, commits, and learns from
  the next red; the preview cohort's false-positive dispositions have no cheap rehearsal.
  Filed 2026-09-11 by consult as a direct entry, the test gap the operator named there.

- **gate-binary-platform-roster-holes** [design-pending] [cost: once/low] [surface: native] — the shipped platform roster held four
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

- **markdown-hard-wrap-unowned-and-ungated** [design-pending] [cost: once/high] [surface: queue-kit] — this repo's markdown hard-wrapping
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

- **queue-entry-shape-slugs-headings-links** [design-pending] [cost: once/low] [surface: queue-kit] — the operator's queue-shape sequence:
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

- **gates-must-not-bind-to-document-paths** [design-pending] [cost: once/high] [surface: gate-sdk] — a gate may know a document's SHAPE
  and never its PATH; path is always config. The discriminator is the operator's, ruled 2026-09-10,
  and it REPLACES the lead's earlier one.
  **The lead's refused reading, recorded because it is the trap.** The lead's discriminator was that
  a grammar-reading gate belongs to its document — which quietly licensed freezing a consumer's
  filename into a gate manifest. The operator's is that grammar-coupling justifies knowing the
  shape, never the path.
  **The worked case.** `check-queue-wrap`'s logic knows nothing about queues — `cplen`, `is_fence`,
  `is_table_row` — and reaches `queue::` only for config, so it is generic mechanism MISFILED in
  queue-kit and should be a `check-line-length` over a configurable target with a configurable
  budget. `check-brevity` is ALREADY that lego (`CONTEXT_KIT_BREVITY_FILE` plus an argument
  override) and the AGENTS.md adapter is ALREADY built and smoke-tested in `agents_md_smoke.rs`,
  which converts and then asserts always-loaded and footprint both measure `AGENTS.md`. A lead claim
  that `check-brevity` was correctly coupled to `CLAUDE.md` was FALSE and is corrected here.
  **The real defect is the GRAPH FORMAT.** `couples=` tokens must be syntactically valid glob or
  path — literals, globs, a `kit:` prefix, no knob indirection — so a configurable-target gate is
  FORCED to freeze one consumer's filename in its manifest. Cheap fix is
  `couples=CLAUDE.md,AGENTS.md`; the principled fix is knob indirection in the format.
  **The survey arm is the entry's first deliverable, and the class is larger than the two members
  the discriminator has been applied to.** Probed at this scope over every `.gate` manifest:
  `CLAUDE.md` appears in 14 `couples=` token positions, `TASK-QUEUE.md` in 23, `docs/install.md` in
  3, `.workflow/WORKFLOW-STATE.txt` in 4, `scripts/gates.list` in 5, beside further per-file
  literals. Apply the operator's discriminator across that corpus rather than case by case.
  **Cost while deferred:** bounded for the survey; unbounded until it runs, since its own output
  how many gates move and whether the format change is owed.
  Surfaced 2026-09-10 as above; the full probed body via
  `git log -p -S'GATES MUST NOT BIND TO DOCUMENT TYPES' -- .workflow/gap-inbox.md`.

- **install-smoke-leg-names-mix-two-axes** [design-pending] [cost: event/low] [surface: .github] — the `install-smoke` legs in
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
  `installer/README.md` each carry a paragraph whose whole job is to say that a reader counting
  platforms has been reading three legs as covering two halves, which they never did. A name
  needing a paragraph to be read correctly is the defect; the paragraph is the receipt.
  **WHETHER IS RULED, so only the scheme is open.** The operator ruled 2026-09-09, in the lead
  session and through the lead channel, that intuitive design is the aim — which retires the
  correct-but-underexplained disposition, a name a reader must be corrected about not being
  intuitive however well the correction is written. Closing this as no-mechanism is therefore not
  available to a later drain.
  **Why `[design-pending]`:** two schemes are live and unranked — bootstrap-first
  (`install-smoke-bash-<platform>` alongside `install-smoke-powershell`), or
  platform-suffix-preserving (rename only the odd leg so it names its bootstrap unambiguously) —
  and the blast radius has to be priced with the scheme rather than discovered after it.
  **THE BLAST RADIUS IS WHOLLY IN-TREE, and this entry asserted otherwise until 2026-09-11.**
  It claimed the rename also reaches the branch-protection required-check names in the local
  ops runbook's desired state, and that that surface breaks silently. Both halves are false
  and were measured false at the drain: the runbook names no `install-smoke` leg at all, and
  this repo's branch-protection desired state is deliberately none, so there are no required
  checks to break. The radius is five tracked files — `.github/workflows/gates.yml`,
  `installer/README.md`, `TASK-QUEUE.md`, `docs/site-architecture.md`, `docs/install.md` —
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
  re-priced 2026-09-11 at close onto the scheme-ranking ground, which survives. Not iceboxed,
  because the operator's ruling makes the current naming a defect to close rather than a
  state to accept.

- **toolchain-floor-spawn-on-native-windows** [design-pending] [cost: event/high] [surface: gate-sdk] — a compiled gate binary should not
  need a GNU userland on the host at all, and the floor roster it advertises is wider than the set
  it actually spawns.
  **RE-SCOPED 2026-09-09 at scope, `lead, own-authority` through the message channel, because the
  entry's filed premise was ALREADY FALSE when it was written.** It was filed at 15:44 asserting
  that `native/src/toolfloor.rs` spawns each floor tool by bare name, that a System32-first
  resolution leaves the floor unmeetable however the runner is provisioned, and that "the
  discriminating probe has not been run". `d9c7a004` landed at 14:01 the same day and is exactly
  that fix: `proc::resolve_floor_tool` resolves outside the Windows system directory and is taken
  by all three floor probes — the comparator, doctor's banner and the env-probe emitter — its
  ground and its single fall-back stated at gate-sdk/SPEC.md §check-graph. The PowerShell leg went
  green on it. So the provisioning reading is CLOSED rather than deferred, and the slug now names
  the half that survives.
  **Its "blocks `windows-roster-join`'s leg going green" sequencing is FALSE and is deleted rather
  than left standing beside a correction.** That leg's floor is met — its own run resolves `sort`
  to GNU coreutils and scaffolds shellcheck — and the single assertion reddening it belonged to
  `smoke-harness-mapfile-inherits-host-line-terminator`, since Done, measured at this scope off
  the finished run's free log.
  **WHAT SURVIVES IS THE PRODUCT READING, and it is objective 1's rather than Windows'.** A binary
  requiring GNU `sort`, `date` and `stat` on the host is bash-era residue the pivot exists to
  collapse; the roster should shrink to what the binary's gates actually spawn, and on a native
  host that may be git alone.
  **Why `[design-pending]`:** the roster is a published adopter claim as well as a runtime
  precondition, so narrowing it is an envelope change rather than a measurement — and what the
  binary spawns is itself moving while the port's remainder lands, so choosing the corpus to
  measure against is the design call.
  **Cost while deferred:** the front door advertises a dependency floor objective 1 exists to
  remove, which is the shape §What the objectives are not names as the front-door false claim.
  Filed 2026-09-09 by the consult session beside `windows-roster-join`, on the same ruling;
  re-scoped the same day at scope, its filed premise having been overtaken by a commit that
  preceded the filing.

- **instrument-leg-expiry-keyed-to-a-green-run-not-its-assertion-set** [design-pending] [cost: event/high] [surface: .github] — an
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
  **Why `[design-pending]`, a real fork with no obviously right limb:** an expiry keyed to *the
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

- **binding-intel-leg-failed-one-run-in-two** [design-pending] [cost: iteration/high] [surface: .github] — a leg this project made binding
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
  **UNTESTED HYPOTHESIS with a cheap witness, offered as a hypothesis and not a finding:** the Intel
  runner is markedly slower and reaches the upgrade arm 30 minutes in with several full payloads
  already in SCRATCH, so scratch exhaustion would produce exactly a fast silent non-zero; a `df` and
  a `du` in that leg before the upgrade pack would settle it for nothing.
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

- **substrate-parity-audits-one-producer-of-two** [design-pending] [cost: event/high] [surface: gate-sdk] — the parity gate's release-path
  assertions read a single named workflow, and the tree now has two workflows that build and hash a
  published-shaped artifact.
  **Verified at source 2026-09-08.** `native/src/gates/gate_substrate_parity.rs`:696 resolves
  `GATE_SDK_NATIVE_PUBLISH_WORKFLOW` as a SCALAR, defaulted to the publish workflow, and both of
  assertion F's checks sit inside the block guarded by that one path — the roster-derived-matrix
  check at :706 and the one-producer-per-digest check at :711-713. `.github/workflows/gates.yml`
  is never opened.
  **What is unaudited, measured rather than asserted.** That workflow now carries a roster-derived
  matrix (:737) and TWO digest-computing sites in different jobs, :494 and :819-822 — the filing
  bullet said one. The assertions' stated subject is the publish path; the tree has two paths of
  that shape and one of them is audited.
  **Cheapest candidate shape, unpriced:** widen the knob from a scalar to a workflow SET rather
  than adding a second gate. **Why that is not a small edit:** a knob's arity is an adopter-facing
  contract, so the change reaches the kit's shell default, both SPEC declarations, the crate
  registry, the member itself and both fixture trees' config files.
  **Cost while deferred:** a second release-shaped producer can drift from the parity contract with
  no verdict, and the gate reads green while covering half the corpus its own assertions name.
  Filed 2026-09-08 by close from the gap inbox. Promoted rather than fixed because a knob-arity
  change is adopter-facing design; promoted rather than iceboxed because the second producer is
  live in the tree today.

- **held-ci-leg-failure-reddens-a-binding-one** [design-pending] [cost: event/high] [surface: .github] — a held producer leg's failure
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


- **overhead-meter-resolves-the-newest-transcript-not-its-own** [design-pending] [cost: iteration/high] [surface: drift-kit] — a bare
  `--emit overhead-meter` resolves the NEWEST candidate transcript, so any session that delegates
  and then meters measures its child; the close stage is that session by construction.
  **Measured, not inferred.** Run 2026-09-06 while a dispatched grandchild was live, the bare arm
  resolved a session whose id prefixes the live GRANDCHILD's agent id — total=171739, 54 per cent
  governance — rather than the metering close session's. Passing the close session's transcript
  explicitly then measured total=1605018, 76 per cent governance: a 22-point spread on the one
  number the health triad's third member reads.
  **The SPEC and the implementation disagree.** drift-kit/SPEC.md §The overhead meter says a bare
  invocation resolves "the transcript the invoking session is itself running in", and that "a
  delegated session resolves its own subagent transcript rather than its lead's". The implemented
  rule is the two-tier scan's NEWEST CANDIDATE WINS, and a live child is always newer than its
  parent, so the two readings coincide only where nothing was delegated.
  **The close stage loses that race by construction:** the housekeeping binding tells it to meter
  itself, and delegation is pre-authorized for the sweeps it runs first.
  **Why `[design-pending]`:** two shapes, and they are not one. (a) Teach the resolver
  session-ancestry — exclude a transcript whose agent id descends from the invoking session — which
  needs a descendant test the two-tier scan does not have. (b) Rule the bare invocation
  under-determined at a delegating session and make the transcript operand required there, which is
  cheap and moves the burden onto every caller. The SPEC sentence changes either way, so the
  amendment is the unit rather than a wording fix beside a patch.
  **Cost while deferred:** `kpi-overhead`'s trailing window silently absorbs subagent rows in place
  of close rows, and `.claude/commands/close.md`'s health triad reads its third member off a
  population it was not defined over. Honest limit: only this one firing is measured, so whether
  every prior close metered itself correctly is unread.
  **Product-class under the 2026-08-30 witness discriminator, not machinery-class:** drift-kit
  ships the meter, so an adopter who delegates and then meters receives the same wrong number off
  the same SPEC sentence — the payload witnesses it, wherever the fix lands.
  Filed 2026-09-06 by the close of `index-runner-hold-release-and-windows-smoke-comparison` into
  the gap inbox, which no stage of that iteration could drain; carried into this iteration's scope
  intake and promoted here, so the record is late and says so.

- **measured-marker-cannot-sit-mid-paragraph** [design-pending] [cost: event/low] [surface: canon-kit] — `check-measured-claim` binds its
  marker to the line above the claim, so a claim standing mid-paragraph can carry no marker and
  goes stale unwatched; TRAJECTORY.md's port figures are the attested instance.
  **The staleness is attested, not predicted.** The ordering paragraph read "thirty-five owed
  files" and "28.6 per cent" while a sibling sentence in the same file, edited the same iteration,
  had already moved to 34. A hand sweep at the 2026-09-04 close caught it; no gate did.
  **The oracle already exists**, which is what makes this a contract question and not a build:
  `scripts/measured-claims.sh` emits `tree-shell-owed` off `--emit port-blockers --tree`, and
  `CANON_KIT_MEASURED_SURFACE_GLOBS` reaches TRAJECTORY.md through `CANON_KIT_MANIFEST_FILES` — yet
  the file carries ZERO `measured:` markers.
  **THE DESIGN QUESTION.** canon-kit/SPEC.md §check-measured-claim binds the marker as a full-line
  HTML comment on the line immediately above the claim, and every shipped instance sits above a
  bullet or a paragraph. This claim is MID-PARAGRAPH: inserting the comment there splits the
  rendered paragraph, and hoisting it to the paragraph's top attaches it to a sentence about
  reopenability rather than to the count. Neither placement is available without a contract change,
  which is why the fix is not a marker add.
  **THE SECOND HALF, which a blanket marking policy would lose.** Most TRAJECTORY.md figures are
  deliberately FROZEN dated attestations, so marking by figure would fight that file's own rule
  that a dated attestation freezes the claim. The discriminator is a paragraph that self-declares
  as corrected where it stands at each later reading — which this one does in its own text and its
  neighbours do not. A sweep keyed on the figure rather than on that declaration is wrong.
  **Cost while deferred:** low as a carry — a stale port figure in the ruling record misleads the
  next session sizing a cut, and only a hand sweep finds it.
  not-icebox-eligible: measured-marker-cannot-sit-mid-paragraph 2026-09-12 witness discriminator.
  TRAJECTORY.md's own instance is machinery-class, but the contract limit is canon-kit's and ships:
  an adopter whose governed prose carries a mid-paragraph measured claim receives a gate that
  silently does not reach it, and a gate's verdict is a product witness under TRAJECTORY.md's
  2026-08-30 discriminator. The one-line tier would also drop the second half above, which is the
  half a later ruling turns on.
  Filed 2026-09-04 by the close of `enter-stage-cut-and-file-authoring-act` into the gap inbox,
  which no stage of that iteration could drain; carried into this iteration's scope intake and
  promoted here, so the record is late and says so.

- **kit-knob-consumer-adapter-convention** [design-pending] [cost: event/low] [surface: evidence-kit] — whether a kit may ship
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
  port cut may not rule (evidence-kit/SPEC.md §lib/evidence.sh). So the cut DEMOTES this entry
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

- **spec-authoring-self-check-pass** [design-pending] [cost: iteration/low] [surface: lifecycle-kit] — spec-stage amendment authoring asserts tree
  facts a one-command probe refutes, and every instance is caught downstream rather than at
  authoring exit.
  **RETURNED FROM THE ICEBOX 2026-08-31 on a judged recurrence**, by the round trip
  queue-kit/SPEC.md §The icebox tier conserves: a dated `recurrence:` line is a live trigger, and a
  one-line entry has nowhere to carry one. Iceboxed 2026-08-30 at `74018ceb` on the ground that
  nothing shipped wrong. That ground still holds and is not what returned it — the rate did.
  **The recurrence, measured across one iteration's amendment set.** Six loose grounds in
  `port-declaration-cohort-and-windows-leg`, spread over at least three of the four amendments spec
  authored, each caught by a stage AFTER the one that wrote it: align repaired one, build batches A,
  B, C and D the rest. The per-amendment CONCENTRATION hypothesis the first re-file offered is
  dead — the spread is wider than that bullet recorded, not narrower.
  **What survives the concentration hypothesis is CLASS, and separating the classes is what a
  costed look owes first, because their costs differ by an order.** A PREMISE defect is
  load-bearing — a class ruling's ground, or a contract sentence quoted to a section that never says
  it. An ILLUSTRATION moves no oracle row and the ruling stands on its own derivation. A SPENT DELTA
  is an instruction whose predicate the authoring commit had ALREADY satisfied, and its hazard runs
  the other way: a build session that trusts it hunts a discrepancy that does not exist, or
  "corrects" a correct figure. A rate measured over all three together overstates the risk.
  **The cheapest shape is named, and it is not the one the original filing proposed.** Probing every
  asserted tree fact before landing an amendment is open-ended and expensive. Re-running each
  delta's OWN predicate at authoring exit is bounded by the amendment's own text, and it catches the
  spent-delta class outright — the one class fully visible from the commit that wrote it. Whether
  the same pass reaches the premise class at all is the design question this entry owes.
  **DISTINCT from the iceboxed `scope-amendment-authoring-gate`**, which is scope doing spec's job.
  This is spec doing its own, at a rate that has now been measured twice.
  **Cost while deferred:** low and downstream-absorbed — nothing has shipped wrong, because the
  pipeline caught every attested instance. What it costs is the later stage's re-probe, and on the
  spent-delta class a hunt for a discrepancy that is not there.
  recurrence: spec-authoring-self-check-pass 2026-08-31
  Filed 2026-08-30 by scope directly into the icebox under that tier's direct-filing rule; returned
  to Deferred 2026-08-31 by close, judging the recurrence off three gap-inbox bullets that between
  them report one iteration's instances and correct each other's readings of them.

- **path-dialect-clauses-unenforced** [design-pending] [cost: event/high] [surface: gate-sdk] — the two clauses gate-sdk/SPEC.md §The
  path-dialect contract gained 2026-08-30 are held by review alone, and neither is shaped like the
  form scan `check-path-dialect` already runs.
  **Clause one: the cwd anchor has no oracle.** That section obliges a script to anchor its own
  shell in the crossed spelling. `check-path-dialect`'s vocabulary is platform-native PRODUCERS
  (`GIT_FLAGS`, `RUST_FORMS` in `native/src/gates/path_dialect.rs`) and carries no `pwd` form, by
  the contract's own reasoning that a shell builtin produces no foreign value. The hole is that a
  builtin PROPAGATES one: an absolute `cd` leaves `$PWD` in the argument's dialect, and a later
  relative `cd … && pwd` concatenates onto it. That is how `gen-pre-commit.sh`'s prologue died on
  the Windows leg. A gate must pair two facts a script exhibits — it derives a root from
  `BASH_SOURCE` with a relative `cd`, and it composes two roots by string arithmetic — rather than
  scan for one form.
  **Clause two, whose filed premise is CORRECTED here rather than carried forward.** The claim was
  that `walk.rs` is the crate's sole path SPELLER and nothing holds it there. Re-verified at this
  drain: it is not. `.join(` appears widely outside `walk.rs`, and a real share of those compose
  filesystem paths — `proc.rs`'s program probe and its capture files, `marker.rs`'s scratch dir. An
  assertion on that spelling, in the shape of the sibling unit test that bars a walk outside
  `walk.rs`, would therefore red a large and overwhelmingly benign population. What DOES hold is
  narrower and is the property worth enforcing: `walk.rs` is the sole producer of the path STRINGS
  a gate reports or matches on, and `DirEntry::path` outside it is genuinely absent. The predicate
  is escape into a reported or compared value, not composition.
  **Why one entry rather than two.** Both are the same landed contract's unenforced half, both were
  filed the same day by the unit that landed it, and both need a pairing predicate rather than a
  form scan. Splitting them files one shape twice.
  **Cost while deferred:** a Windows adopter's gate verdict, which is a cost already paid once —
  `check-install-disposition` read its whole corpus as unregistered on round 5 because a composed
  separator escaped into a compared value, and a Linux battery cannot show it.
  Filed 2026-08-31 by close, draining two 2026-08-30 gap bullets. Fix was tried first and refused
  (the assertion the second bullet proposes reds a benign population); icebox second, refused
  because a gate's verdict on an adopter's host is adopter-facing.

- **design-pending-tag-restates-its-own-section** [design-pending] [cost: iteration/low] [surface: queue-kit] — the tag every deferred entry
  carries is derivable from section membership alone, which is the two-sources defect
  queue-kit/SPEC.md refuses by name for a challenger while exempting the incumbent.
  **The argument is the spec's own, not a filer's.** queue-kit/SPEC.md:190-194, ruling out an
  `[icebox]` tag for the icebox tier: "No `[icebox]` tag is minted: section membership *is* the
  state, and a tag restating its own section is the two-sources defect. What generalizes across
  the amendment lifecycle is the design-pending **section set**, not its tag set." This tag
  restates its own section set exactly as the refused one would have, and derivation-first says
  the same independently. The coupling is total and BIDIRECTIONAL —
  `native/src/gates/amendment_queue.rs` reds a deferred entry lacking the tag (:201) and an
  active entry carrying it (:191, :197) — which is a proof of derivability, not a protection.
  **Mechanical cause of the misreading, the part a rename fixes and a warning does not:** the
  identical string names two things. On a lead line it is a structural section marker carrying no
  information; in a body, `Why [design-pending]` is a conventional field (queue-kit/SPEC.md:32)
  whose stated job is "what the open design actually is" — genuinely semantic. Re-derived at the
  2026-09-06 drain: 4 body fields against 382 lead-line tags, 579 occurrences file-wide;
  re-derive rather than compare a number, which drifts every demotion.
  **Measured confusion cost, one iteration, two sessions:** a lead read the tag as a semantic
  claim and proposed a stale-tag correction on an entry an operator ruling had already settled;
  a scope session dropped the tag and TOOK THE RED learning the same thing, finding its own drain
  prose contradicting itself in the process.
  **The accidental-move defence does not survive contact, and it was this filer's own.** The
  claimed value is a checksum making a cross-section move detectable. The promote direction is
  ALREADY guarded by `amendment_queue.rs:205`, which reds a design-pending-section entry carrying
  a spec-ref tag — keyed on a marker that carries real information. And the tag CREATES the
  error class it detects: without it, relocating an entry is moving lines and there is nothing to
  forget. That is a redundant field, not a guard. **The one honest residual, owed inside the
  unit:** the demote direction keeps no equivalent checksum, and nobody has measured whether an
  accidental demotion ever occurred — one `git log -L` over a section boundary answers it.
  **Costed, mechanical and wide rather than deep:** the file-wide occurrences above; three checks
  in `amendment_queue.rs` plus `tag_lead_line.rs` and `task_conservation.rs`; the grammar block
  at queue-kit/SPEC.md:187 and the icebox paragraph whose argument would invert; four gate-test
  fixture pairs under `queue-kit/gate-tests/`; and the docs mirrors. Far past fix-inline.
  **PRODUCT-CLASS by the 2026-08-30 discriminator, without needing its counterfactual clause:**
  shipped queue-kit grammar every adopter's queue wears and every adopter's gate enforces.
  **Three alternative shapes, and this entry rules none — the choice is operator-class.** Remove
  the tag and rely on section membership as the icebox tier does; or RENAME so the string cannot
  be read as the body field's semantics, preserving the demote-direction checksum at the same
  sweep cost; or keep as-is and document the trap, cheapest and leaving the confusion in place.
  **This entry wears the tag it questions, and that is correct rather than a joke** — the grammar
  binds until it is changed, and an entry exempting itself would be the second source it names.
  **Cost while deferred:** every reader meets a string that is a section marker in one place and
  a semantic field in another; the two sightings above are what that costs per iteration, and the
  sweep grows by one lead line per demotion.
  Filed 2026-09-06 by the lead at the spec dispatch boundary on an operator question about
  whether the tag earns its keep; the operator noted having proposed removal before and gave no
  ruling. Promoted 2026-09-06 by close — too wide to fix inline, live trigger bars the icebox.

- **entry-compression-contract-unenforced** [design-pending] [cost: event/low] [surface: queue-kit] — the compression
  relief that queue-kit/SPEC.md §check-queue-entry-budget mandates most often
  is enforced by nothing, and the failure is invisible where it lands.
  **The concession is the spec's own, twice**, re-read at the 2026-08-30
  drain: queue-kit/SPEC.md ("it sees an entry's current extent, and
  judging whether a removed line was answered or discarded is semantic") and
  again for the recording-in-the-moment rule it names. So the rule that
  compression proceed by ANSWERING grounds, never by dropping them, has no
  oracle at all.
  **Why the failure is invisible**: extent is the only artifact, and a
  compressed entry reads identically whether its missing grounds were answered
  or silently discarded. A later reader cannot tell that grounds it lacks were
  ever written.
  **Candidate shape, unpriced and not started**: a gate arm reding a commit in
  which a deferred entry's counted extent SHRINKS while that same commit adds
  a ruling or recurrence line, unless the commit also carries the relocation
  citation the section already specifies. Both halves are machine-readable,
  which is what makes it a proxy for "a mandated write displaced grounds";
  what it cannot decide is the semantic half, so it reds a shape rather than a
  judgment.
  **This SUPERSEDES rather than re-files `ruling-accretion-outgrows-the-entry-cap`**,
  whose three shapes were all refused by the owning spec and which is now Done —
  wontfix, operator-ratified 2026-09-09, executed at the 2026-09-10 close, its
  boundary note landed in queue-kit/SPEC.md §check-queue-entry-budget and its
  grounds recoverable from git history. That entry proposed re-pricing the cap. This
  one's subject is the enforceability of the relief the spec ALREADY rules
  correct, which stays live precisely because those three were refused.
  **FIRST ATTESTED INSTANCE OF THE CANDIDATE ARM'S OWN SHAPE, judged a recurrence at the
  2026-09-08 drain and verified by diff rather than inferred.** At `fcf51555` the entry
  `amendment-roster-omission-detection` carried its propose-once record verbatim; `40477bdb`
  compressed it out while that same commit added ruling lines, and `40477bdb`'s own body states
  the mechanism ("the ruling lines are paid for by compress-by-answering rather than by
  widening"). The consequence was measured at the next scope, which had to recover the spent
  route from git to avoid re-escalating an answered member. **The instance is now moot and the
  class is not** — the compressed entry has since left Deferred, so the dangling antecedent is
  gone with it; what the arm would have caught is unchanged.
  **RULED IN CONSULT 2026-09-09 (`consult, own-authority`, operator-convened, lead-relayed): COSTED
  AND FILED, NO MECHANISM OWED.** The candidate arm's shape — extent shrinks while a ruling line
  lands — is also the shape of the relief the spec mandates (compress by answering), so it reds the
  correct act as often as the defect: a classifier no gate can honestly run, the 2026-08-30 ground.
  The lever is content tiering — a ruling lands on the entry as a pointer to TRAJECTORY.md or the
  owning SPEC, never as prose — and this iteration's scope paid its rounds because four landed as
  prose. **Declined at threshold at scope — operator direction, 2026-09-11 (lead-relayed):** no
  mechanism is owed, so nothing is takeable.
  recurrence: entry-compression-contract-unenforced 2026-09-07 2026-09-09
  **Cost while deferred:** every mandated write onto a saturated entry pays
  the same unenforced honour-system compression, and the queue's most-ruled
  entries are exactly the ones paying it.
  Filed 2026-08-30 by close from the gap inbox; both of the owning spec's
  concessions were re-read at the drain.

- **config-bridge-resolution-cost** [design-pending] [cost: session/high] [surface: gate-sdk] — the array-knob config bridge still costs
  about 640 ms on every invocation that resolves it, and no entry owns the residue.
  **RE-SCOPED 2026-08-23 at `battery-runner-port`'s close, on a lead ruling: correcting a false
  claim in an entry is not a descope.** Three of this entry's four load-bearing statements moved
  under `6d813968`, and the retired ones are deleted rather than annotated.
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
  **The residue is now paid TWICE on one gate, measured at this close rather than estimated.**
  Cut B's enum-sets port made `CANON_KIT_ENUM_SETS_CMD`'s value itself a bridged arm that
  re-sources `lib/gate.sh` and resolves its own knobs before exec'ing the binary, so
  `check-prose-enum`'s whole resolve-and-run went **366 ms -> 1236 ms** (build read a 1223 median
  at the cut; this drain re-read 1210/1211/1236/1237/1237). The nesting is what the excess is: the
  same five-run method reads **743-765 ms for `check-core-files`**, one bridge resolution over
  almost no work, so `check-prose-enum` carries about one further floor on top of it. The
  commit-hook path pays NOTHING — `gen-pre-commit` bakes the resolved arrays — so the exposure is
  whole-tree battery runs, `--only` runs, and hook regeneration. The repair refused, named so it
  is not re-derived: an in-process call from the gate to the bundled emitter would resolve the
  BUNDLED producer for a consumer who configured a different one, which is the extension point
  that knob exists to protect.
  **Cost while deferred:** every bridged invocation pays the floor it cannot avoid — 640 ms
  best-of-three on 2026-08-23, 743 ms best-of-five at this close, indicative of drift rather than
  a regression claim and not to be re-quoted without re-running it — and the cost scales with the
  number of owning kits rather than of members, so a new kit raises the floor for everyone while a
  new member no longer does. A nested bridge, as `check-prose-enum` now has, pays it twice.
  Filed 2026-08-21 twice, by spec and by build; promoted at `graph-port-and-config-seam`'s close;
  re-scoped here after the batch landed, with every retired figure deleted.
  recurrence: config-bridge-resolution-cost 2026-09-03

- **amendment-reader-roster-undercount** [design-pending] [cost: iteration/high] [surface: lifecycle-kit] — an amendment's reader/caller roster is a
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
  **THE COUNTER-CASE IS NOW OVERDUE AND THE POOL WENT THE OTHER WAY — FOUR FOR FOUR in a second
  iteration**, which is the datum the `[design-pending]` fork was waiting on and it arrived
  falsifying, not supplying, the sparing case. `test-harness-cut-seam-sweep`: batch A's gate-tests
  amendment named three callers and missed the CI workflow and its shipped template; batch B's
  guard-tests amendment claimed one caller "verified this session over the tracked tree" against an
  EXECUTABLE CI step plus six prose sites, and its agents-md amendment predicted two docs pages
  needed no edit when both named the deleted path; batch C's cardinal-reach DoD asserted "grepped
  every spec, config and gate source; nothing dangles" against eight live references. **The battery
  caught every one and reading caught none** — so of the three candidates the two that reach an
  ORACLE (the build-stage re-sweep line, the idiom grep) are now separated from the third by
  evidence rather than by taste, and a session picking this up starts there.
  **AND THE DOC SIDE IS THE SAME MISS.** This close's `internal-identifier-restatement` and
  `capability-liveness-after-descope` sweeps independently found five present-tense prose sites
  naming the three shell files this iteration deleted, four of them in files the landing unit had
  itself opened and edited. Grammatical position predicts them: every repaired occurrence sat in
  subject or object position, every missed one in an appositive, an attributive modifier or a
  coordinated list member. A roster derived by grep would have caught all five.
  recurrence: amendment-reader-roster-undercount 2026-09-05
  **Cost while deferred:** a spec session pays for a census the build session must buy again, and
  the roster's authority runs inversely to its accuracy.
  Filed 2026-08-21 by build into the gap inbox; promoted at `graph-port-and-config-seam`'s close.

- **boundary-wipe-preserve-basename-reach** [design-pending] [cost: iteration/high] [surface: lifecycle-kit] — the iteration-boundary scratch wipe
  matches its preserve list by **basename at any depth**, so one nested `.gitkeep` makes a whole
  scratch tree immortal and the wipe still reports success.
  `gate-sdk/bin/run-gates.sh --enter-stage`'s boundary block walks the scratch tree depth-first and
  skips any entry whose **basename** equals `.gitkeep` or a preserve-list member (`wipe_walk`,
  `native/src/emit/enter_stage.rs`). The test is on the basename alone with no path anchor, so a
  `.gitkeep` at any depth survives, its parent's directory removal then fails as non-empty, and
  every ancestor up to the scratch root survives with it.
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
  recurrence: boundary-wipe-preserve-basename-reach 2026-09-04
  **Cost while deferred:** scratch accumulates across iteration boundaries without bound while the
  one mechanism chartered to reclaim it reports success — the boundary reset's own claim is false
  in exactly the case a consumer is most likely to hit.
  Found 2026-08-18 by this iteration's scope at its own entry, from the surviving directory rather
  than from a reading of the code; filed under scope-gated intake rather than fixed in-session.

- **baseline-move-stales-evidence-line** [design-pending] [cost: iteration/low] [surface: evidence-kit] — promoting a task and moving a suite's
  baseline is not enough to close: the evidence line already recorded against the *old* baseline is
  stale, and nothing says so until the entry gate refuses a second time for a different reason.
  **The second face of `close-entry-baseline-bootstrap-deadlock`**, which owned the first (close is
  the only stage that may file the blocking slug, and cannot enter without it) and has since been
  ruled and retired. Its fix addressed the first face and left this one standing, which is why this
  was filed apart rather than folded in, and is why the retirement does not carry it away.
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

- **plugin-marketplace** [design-pending] [roadmap: later/ecosystem] [cost: once/low] [surface: installer] — harness plugin packaging.
  roadmap-summary: The stage skills and guards installable as a harness plugin.
  Harness plugin/marketplace packaging
  of the stage skills and guards; anti-drift gate shape: manifest ↔ shipped
  surface parity. Design against the live manifest format at promotion — the
  plugin substrate moves fast (the scope-session-routing ruling applies).
  **The install-ownership contract this must package against already exists:**
  `checkwright.lock`, written by the installer's `init` and specified at
  installer/README.md §The manifest — its schema owner is
  `native/src/installer/lock.rs`. A marketplace package that installs kits
  without writing that manifest would be a second install model with no upgrade
  or uninstall story, which is the sequencing risk this entry has always
  flagged; the named contract replaces the re-derivation it used to imply.
  The upgrade/uninstall story itself has shipped as the installer's `update`
  and `uninstall` verbs, specified at installer/README.md §update and
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
- **benchmark-ab-experiment** [design-pending] [roadmap: later/adoption] [cost: once/low] [surface: drift-kit] — a controlled A/B trial.
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
- **hosted-attestation-service** [design-pending] [roadmap: later/commercial] [cost: event/low] [surface: evidence-kit] — hosted attestation.
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

- **heterogeneous-agent-delegation** [design-pending] [roadmap: later/ecosystem] [cost: iteration/low] [surface: delegation-kit] — foreign agents.
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

- **background-credential-swap-support** [design-pending] [cost: event/high] [surface: delegation-kit] — first-class support for
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


- **gate-file-coverage-closure** [design-pending] [cost: event/high] [surface: delegation-kit] — the missing check class behind a
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

- **prose-filename-citation-liveness** [design-pending] [cost: iteration/low] [surface: canon-kit] — a bare `<name>.md`
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
  recurrence: prose-filename-citation-liveness 2026-09-06

- **batch-split-stamp-ownership** [design-pending] [cost: iteration/low] [surface: lifecycle-kit] — who stamps the per-session
  audit trail when a live lead splits one stage across several batch sessions is
  unowned. This iteration recorded **one** `build` line in
  `.workflow/WORKFLOW-STATE.txt` for five batch sessions.
  **Premise corrected 2026-08-01 by the undirected scope survey — the filed
  diagnosis was wrong and the next reader would otherwise re-derive it.** The entry said the two
  surfaces disagree in effect. **They agree**, and have since 2026-07-17: build.md's
  "Every session still stamps" paragraph and lead.md's batch paragraph carry the same instruction,
  so the gap is **practice against instruction**, not instruction against instruction — the batches
  simply did not run the entry. Nor is the id the obstacle: dispatched sessions resolve distinct
  ids.
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
  **Recurred 2026-08-29 in a new sub-shape: directed, not omitted.** Three build batches, one
  `build` stamp, under an explicit lead instruction not to re-stamp — the shape
  `lifecycle-kit/templates/lead.md` has named as the failure since 2026-08-04, so prose has now
  failed against a lead that had it in context. That is the second half of the fork's own argument.
  recurrence: batch-split-stamp-ownership 2026-08-29
  Filed 2026-08-01 at close from the gap inbox, filed by this iteration's build.

- **companion-toolkit-profile** [design-pending] [roadmap: next/ecosystem] [cost: event/high] [surface: lifecycle-kit] — the interop rung.
  roadmap-summary: Gate a tree whose specs another toolkit's workflow wrote.
  Govern a tree whose specs an **external spec-authoring toolkit produced** — a
  consumer profile for when the specs Checkwright gates were written by a second
  toolkit's workflow, not by this one's `spec` stage. It cashes the claim below.
  **The design is already decided and is not what this entry holds.** Two
  rulings on record settle it: `prose-profile` (retired) ruled that a profile ships
  as an adapter delivered as optional consumer config and never as a kit literal, and
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
  `--agents-md-smoke` arm. That is the shape the three claims below owe.
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

- **design-partner-preview** [design-pending] [cost: iteration/high] [surface: installer] — a narrow external preview before
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
  unchanged, and the `prose-profile` dependency it named landed 2026-08-09.
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

- **external-gate-quality-evidence** [design-pending] [cost: event/low] [surface: evidence-kit] — durable, published
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

- **unqualified-section-citation-liveness** [design-pending] [cost: iteration/low] [surface: canon-kit] — a bare `(§Heading)`
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

- **gate-authoring-sdk-surface** [design-pending] [roadmap: next/ecosystem] [cost: event/low] [surface: gate-sdk] — a gate-authoring SDK.
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
  Filed 2026-08-02 by build, on an operator ruling, during `native-gate-dispatch-seam`.

- **gate-tamper-exemption-reader-substrate** [design-pending] [cost: event/low] [surface: gate-sdk] — `check-gate-tamper`'s
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

- **couples-dynamic-root-resolution** [design-pending] [cost: session/high] [surface: gate-sdk] — the skipped-and-counted bucket is
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

- **amendment-owner-position-citation** [design-pending] [cost: iteration/low] [surface: queue-kit] — prose may cite a merged
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

- **partitive-exemption-line-scope** [design-pending] [cost: event/high] [surface: canon-kit] — `check-manifest-count` is stricter
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

- **behavior-change-surface** [design-pending] [cost: event/high] [surface: gate-sdk] — no accumulating declaration surface.
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
  **A live instance, 2026-08-25 at build in commit `04f81ad8`, and it sharpens the open question
  above.** `evidence-kit/bin/diff-baseline.sh` gained a refusal: each argument group is now
  `<suite> <logfile> [<status>]`, and a suite on the exit-code parser named *without* a status is
  refused at exit 2 rather than handed a hardcoded 0. A vendored consumer invoking the old pair form
  against such a suite now gets an exit 2 where it used to get a verdict. Nothing owes a
  declaration for it: `.workflow/tightened-gates.txt`'s own contract line takes *one bare
  kit-shipped gate name* per data line, and `diff-baseline.sh` is a `bin` tool and not a registered
  gate — so the surface cannot hold it even in principle. That is the reconstruction cost this entry
  predicts, arriving in the half the open question is about: the tightening is real and the existing
  surface's grammar, not just its scope, is what excludes it.
  **SECOND INSTANCE 2026-09-06 at build, and it comes at the grammar from the OTHER side.** The
  2026-08-25 witness was a surface that could not hold the *name*. Here it held the name and could
  not hold the *remedy*: minting the `open-authorization-channel` binding slot reds
  `check-skill-binding` in every vendoring consumer until that consumer binds it, so build appended
  the bare name — correctly — and the one-line remedy a consumer needs had no route from build to
  the composing session. Two witnesses, both landing on "name plus prose versus bare names", is the
  open design question above asked twice rather than a second question.
  **At threshold, declined again at scope — operator direction, 2026-09-11 (lead-relayed):** the
  release-note surface is unshared, its design question open on purpose, and releases deferred.
  recurrence: behavior-change-surface 2026-08-25 2026-09-06
  Filed 2026-08-04 at close from the gap inbox; the design question left open on purpose.

- **session-model-identity-verification** [design-pending] [cost: event/high] [surface: delegation-kit] — a session cannot report or
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

- **consult-tier-declaration** [design-pending] [blocked-by: session-model-identity-verification] [cost: event/high] [surface: delegation-kit]
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

- **intra-file-pendency-contradiction-scan** [design-pending] [cost: event/high] [surface: canon-kit] — one file can call the same
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

- **link-wrapped-section-citation-liveness** [design-pending] [cost: event/low] [surface: canon-kit] — a section citation inside a
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


- **amendment-refusal-acceptance-parity** [design-pending] [cost: event/low] [surface: canon-kit] — an amendment's refusal rationale can
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


- **deferred-release-declaration-accumulation** [design-pending] [cost: iteration/high] [surface: RELEASING.md] — only one of the note's three
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

- **audit-class-corpus-attestation** [design-pending] [cost: iteration/high] [surface: lifecycle-kit] — an un-gateable-class audit stamps a
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
  It is the generalization `check-measured-claim` names as the scope-claim axis it cannot
  reach, and is cross-referenced there rather than folded into it — that entry designs a scanner
  over authored prose, this one designs a stamp over a session act.

- **qualified-pointer-section-ownership** [design-pending] [cost: event/high] [surface: canon-kit] — `check-spec-pointer` asserts a
  cited `§Heading` **exists**, never that it is the heading which *owns* the cited claim, so a
  fully-qualified pointer aimed at the wrong section resolves and reds nothing.
  **Self-witnessed 2026-08-09, which is why it is filed rather than theorised.** This close,
  correcting another entry's expired premise, cited `gate-sdk/SPEC.md §What the dispatch seam
  does not settle` for a claim owned by §What is retained, and where the second port stands.
  Both sections exist. The gate passed — "every target file tracked and named §heading present"
  is its own verdict text, and presence is the whole of what it checks. Caught only by reading
  the file to confirm the sentence was there.
  **It is the mechanism under the ruling-staleness class.** The inbound half that class
  diagnoses — a citation surviving the deletion of the ruling it names, now designed at
  lifecycle-kit/SPEC.md §The ruling-staleness probe — and this are one defect seen from two
  sides: a pointer is verified against the section's *existence*, so nothing notices when the
  section stops carrying the claim, whether because the claim was pruned out of it or because
  the wrong section was named to begin with. That section also records the measurement narrowing
  the inbound reading: a probe over citation targets reaches almost none of the real cohort,
  because the heading survives and only the body under it is rewritten.
  **Distinct from the three sibling entries in this cluster**, which are all about citations
  that resolve to *nothing*: `unqualified-section-citation-liveness` (a bare `§Heading` with no
  path), `spec-pointer-self-section-citation` (the self-citing form), and
  `prose-filename-citation-liveness`. This one resolves successfully and is wrong anyway, which
  is the harder half — the reader's trust is higher precisely because the pointer works.
  **Why `[design-pending]`, and an honest "not buildable" is a permitted outcome.** Deciding
  whether a section *supports* a sentence is comprehension, not scanning. The only mechanical
  approximations are term-overlap heuristics between the citing sentence and the target
  section, whose false-positive surface is the one `check-measured-claim` escaped by ruling
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

- **docs-link-red-remedy-first** [design-pending] [cost: event/high] [surface: site-kit] — `check-docs-link-convention` reds on the
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

- **init-dry-run-plan-parity** [design-pending] [cost: event/high] [surface: installer] — `init --dry-run` is a hand-maintained second
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

- **kfric-capture-unverified-assertion** [design-pending] [cost: iteration/high] [surface: drift-kit] — the knowledge-friction channel has
  no oracle, so it captures whatever a session asserts and the next reader reads it as measured.
  **The DRAIN-side axis is LANDED**, 2026-09-07, in `drift-kit/SPEC.md` §The knowledge-friction
  loop and `templates/close-knowledge.md`: the drain re-verifies both limbs of a capture — the fact
  and the ownership clause its surface field asserts — and records the outcome in the close commit.
  **What returns here is the capture-side question alone, and it is unruled:** whether the
  affordance should carry a measured-vs-estimated distinction at all. The whole value of
  `--emit-kfric` is that stamping is cheaper than deferring, so a field that slows capture buys
  accuracy with the capture rate the loop depends on. A convention may beat a flag — and
  lifecycle-kit/SPEC.md §The committed gap inbox has **already refused both obvious shapes** for
  the sibling channel, a filing-time prompt and a fact-versus-inference grammar, so a proposal
  here argues against a recorded refusal or finds a third shape.
  **Threshold decline at scope, 2026-09-11 — operator direction (lead-relayed):** the
  drain-side half landed 2026-09-07 and what remains argues against a recorded refusal, which is
  design work rather than a takeable unit; drift-kit shares no surface with this iteration.
  recurrence: kfric-capture-unverified-assertion 2026-08-28 2026-09-06
  **Three attested instances, each falling on a different limb.** A build batch stamped the
  consumer smoke's cost as "~50-60 minutes" and reasoned that the run serializes against all
  tracked editing for that window; validate measured **227s** twice. The figure was disprovable
  from evidence already in front of every reader — the reporting batch's own session runtime was
  ~26 minutes — and was relayed onward unchecked, shaping two sessions' scheduling.
  2026-08-28, a wrong MECHANISM rather than a wrong number: a kfric asserted that
  `check-stage-entry` assertion C's component dir is one holding `LIFECYCLE_KIT_ROSTER_BASENAME`,
  and close's remediation wrote that into lifecycle-kit/SPEC.md §check-stage-entry as a
  definition. It is false against `native/src/gates/stage_entry.rs`, whose multi-file arm returns
  on `amend_dirs.len() >= 2` with no roster test. Corrected in the same close — so the harm is
  attested on a governed kit SPEC, not only on a scratch log.
  2026-09-06, a wrong OWNERSHIP claim rather than a wrong fact: the fact held and the clause beside
  it, "no gate-sdk/SPEC.md section states it", did not — that section having stated it since
  `121e76cb`, two days earlier, landed by this same triage loop.
  Distinct from `kfric-empty-log-ambiguity`, retired, which is about an *empty* log's two readings;
  this is about a populated one whose entries carry no distinction between a measurement and an
  estimate. Adjacent to `dispatch-cited-evidence-unverified`, which covers what a dispatched
  sweep *cites*; this covers what a session captures about its own work.
  **Cost while deferred:** an unverified assertion in the log is still indistinguishable from a
  measurement. The drain axis caught the channel by which a wrong number reaches a canonical
  surface with a citation on it; it did not make the log itself readable, so every consumer of a
  captured line before the drain still reads an estimate as a measurement.
  Filed 2026-08-09 by close, from its own knowledge-friction triage. Promoted 2026-09-07 by spec
  on the drain axis alone — `lead, own-authority`, relayed in that iteration's dispatch — and
  demoted at merge with that axis landed and this one untaken.


- **survey-record-claim-reliability** [design-pending] [cost: iteration/high] [surface: lifecycle-kit] — the survey record carries
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
  **FOURTH instance, 2026-09-08, and it moves the class off the `finding` field.** The scope
  recurrence survey's `edges:` recorded `prompt-ranking-ungrantable-shape-class 1`; the real inbound
  count is TWO, at that survey's own cited rev and at HEAD. So a *derived* field is wrong the same
  way the free-text ones were — `--emit file-survey` takes `edges` as a hand-typed argument rather
  than deriving it, which no candidate deliverable above addresses. The survey's headline finding is
  unaffected (it reads `recurrence:` dates, not this figure). Judged a recurrence at close after
  re-running the count; not corrected in the record, which the next first-stage entry truncates.
  **FIFTH instance, 2026-09-11, and the wrong finding travelled into a direction.** The scope
  survey called `bridged-knob-case-tmp-dir-override-inert`'s premise STALE: the shell harness was
  gone and the native arm absolutizes the case scratch. Scope routed the entry to close to retire.
  The close re-ran the entry's own reproduction and the residue regenerated, because the arm's pin
  misses the knob form a bridged gate reads. The survey never ran the reproduction the entry names.
  recurrence: survey-record-claim-reliability 2026-09-08 2026-09-11
  **Declined at threshold at scope:** its design pairs with `kfric-capture-unverified-assertion`'s
  capture-side question, itself declined, and its cheapest part — deriving the `edges` slot
  rather than taking it hand-typed — shares no surface with the `instruction-surface-tier` set.
  Filed 2026-08-10 by close, on operator direction after the third error.

- **kit-ref-liveness-stem-token-hole** [design-pending] [cost: event/high] [surface: canon-kit] — a typo'd knob name under
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


- **close-differential-instruction-sweep** [design-pending] [cost: iteration/low] [surface: lifecycle-kit]
  — after the full sweep, close keeps the corpus swept differentially instead of re-reading it
  whole. It waited on `instruction-surface-sweep`, which landed the full pass at build 2026-09-12;
  that pass's compliant calls are in its landing commit's message, which the differential range
  yields through `git log`.
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

- **settings-pins-live-suite-coverage** [design-pending] [cost: session/high] [surface: context-kit] — `check-settings-pins` is exercised by
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

- **vendored-library-identifier-reach** [design-pending] [cost: iteration/low] [surface: doctrine-kit] — De-literalization's reach test gives
  two answers for a vendored `lib/*.sh` function, and the corpus holds both populations.
  The rule bans prose restating a source's **internal** identifier roster while allowing public
  contract names. Two readings of "internal" are both defensible and they disagree:
  **file-level** — a shipped vendored library is a public contract by construction, so its
  owning `§lib/*.sh` section may name its exports — or **kit-level**, the reach test an audit
  actually applies: nothing outside the kit calls it, so it is internal.
  **Both populations exist, which is why no reading can be adopted silently.**
  `gate-sdk/lib/gate.sh` and `guard-kit/lib/guard.sh` have
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


- **guard-ruleset-registration-lockstep** [design-pending] [cost: event/low] [surface: guard-kit] — guard-kit's generic ruleset exists in
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

- **spec-section-title-collision** [design-pending] [cost: event/low] [surface: canon-kit] — two sections in one SPEC may carry the same
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

- **close-eviction-refiles-without-checking** [design-pending] [cost: iteration/high] [surface: lifecycle-kit] — close's backlog-eviction step
  files its finding without checking whether a prior close already filed it, and has now done so.
  **Self-demonstrating instance, found at this close:** the queue carried two entries for one
  finding — `icebox-worklist-roadmap-blind` (filed 2026-08-09) and
  `icebox-candidate-roadmap-filter` (filed 2026-08-13) — with the *same* three-row measurement in
  both bodies. Merged at this close into the elder slug, which was the one carrying an inbound
  citation; both have since retired.
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
  (since retired) with the same roadmap-tag measurement for the FOURTH time across four
  closes. The other came
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

- **install-disposition-smoke-accounting-split** [design-pending] [cost: event/low] [surface: gate-sdk] — the precommit gate checks smoke
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
  `consumer_smoke` caught it, fixed in one line at `1e18d154`. The same iteration's batch-B gate was
  `zero-config` and WAS registered, so the seam is the disposition split rather than a careless
  batch.
  **Why `[design-pending]`:** the fix shape needs a ruling, not a build. Either widen
  `check-install-disposition` to run the full accounting for every disposition, or move the
  accounting out of `consumer_smoke` into a precommit member — the second buys the coverage but may
  re-buy smoke cost at every commit, which is the trade nothing here settles.
  **Distinct from `consumer-smoke-targeted-mode-registrar-scope`** (merged 2026-09-11 into the
  iceboxed `consumer-smoke-subset-accounting-verdict`), whose axis is the targeted
  single-kit mode severing a cross-kit registrar. This one is about which TIER holds the accounting
  at all, and it fires on the untargeted run that neighbour reports clean.
  **Cost while deferred:** one stage of latency on a mechanical zero-judgement condition a precommit
  gate could hold, plus a validate red that presents as a build defect — the batch session reads a
  registration omission as its own gate misbehaving.
  **RECURRED 2026-08-27**, and the recurrence is exact rather than analogous.
  `check-action-permissions` landed at `windows-artifact-proof` build batch 1 carrying
  `install: on-surface`, unregistered; the precommit battery passed across five commits and
  validate's `consumer_smoke` caught it, fixed in one line at `d0b496fa`. Two independent
  instances now, both `on-surface`, both one line, both one stage late — so the seam is the
  disposition split and nothing about either batch. What the second instance ADDS to the
  first: the registrar was a *different kit* from the shipping one (gate-sdk ships the
  `check-action-*` family; site-kit registers it, being the kit that writes the workflow
  surface those gates read), so whichever tier ends up holding the full accounting must
  resolve registration cross-kit rather than in the gate's own kit — a constraint the
  2026-08-22 instance did not expose.
  recurrence: install-disposition-smoke-accounting-split 2026-08-27
  Filed 2026-08-22 by close, draining the gap inbox; the lead filed the bullet at validate and this
  drain re-verified the skip at its source rather than off the gate's `spec:` line.

- **spec-prune-normalisation-shell-oracle** [design-pending] [cost: event/high] [surface: canon-kit] — the shell twin of the
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

- **baseline-row-prose-coupling-gate** [design-pending] [cost: event/low] [surface: canon-kit] — governed prose asserts what
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

- **gap-capture-argv-prompt-friction** [design-pending] [cost: event/low] [surface: gate-sdk] — the mandated capture tools take their
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

- **bridged-knob-owner-for-consumer-gate** [design-pending] [cost: event/high] [surface: gate-sdk] — the config bridge resolves a knob
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
  `native/src/emit/upgrade_smoke.rs` resolves the same file through `GATE_SDK_WORKFLOW_DIR`, a
  knob it declares. The tenth cohort's three members declare no knobs, so nothing fails yet.
  **DISTINCT from `consumer-gate-port-disposition`, landed, which it cites rather than re-files.**
  That entry owned the *declaration* question — the owner column and conservation row, authored
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



- **done-slug-ownership-citation-report** [design-pending] [cost: iteration/low] [surface: queue-kit] — governed prose says a queue slug
  "owns" an open question in the present tense, and nothing notices when that slug lands.
  **Two live instances, both found by hand at this close's audit sweep and both fixed here.**
  `gate-sdk/SPEC.md` asserted that the shipped install path "degrades silently, which
  `installer-jq-silent-degradation` owns", and again that the slug "still owns it" — text
  *added by this same iteration*, weeks-fresh, and falsified by the same iteration's later
  commits (`bd8ef299`, `97b65bdb`, `047c7426`) that landed the unit and moved the slug to
  `## Done`. A reader arrives at a settled question dressed as an open one.
  **DISTINCT from `dead-queue-citation-report`, shipped**, and not folded into it: that entry's
  corpus was the queue's own bodies and its subject a slug resolving *nowhere*. This
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

- **in-crate-module-coupling-derivation** [design-pending] [cost: event/low] [surface: gate-sdk] — a ported gate's descriptor can omit
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

- **knob-shape-flip-undetected** [design-pending] [cost: event/high] [surface: gate-sdk] — a crate reading a knob as an array cannot
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

- **kit-spec-consumer-config-literal** [design-pending] [cost: event/high] [surface: gate-sdk] — nothing stops a kit SPEC from spelling
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

- **queue-recovery-pickaxe-wrong-oracle** [design-pending] [cost: event/high] [surface: queue-kit] — every surface that tells a reader how
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


- **stale-identifier-after-retirement** [design-pending] [cost: event/low] [surface: canon-kit] — governed prose citing a deleted path
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
  recurrence: stale-identifier-after-retirement 2026-09-09

- **threshold-recurrence-routing-residency** [design-pending] [cost: once/low] [surface: lifecycle-kit] — where the threshold-recurrence
  routing clause lives, now that its only carrier has left the live tree.
  **The clause, carried here verbatim so it does not spend by attrition:** *"a third threshold
  recurrence routes to the operator, not to a third decline; two is where lead discretion
  ends."*
  **BREACHED ONCE (2026-09-04, `dated-measurement-restatement-class`): a lead ruled its third.**
  **OPERATOR-RULED 2026-08-17: file for scope to decide a permanent home**, rather than leave it
  in history or move it to TRAJECTORY.md now. It was exercised once in
  `port-selector-permanence-and-batch` and ruled LIVE AND UNSPENT at the promotion relay, so
  its disappearance from the live tree would spend it by attrition — the outcome that ruling
  refused.
  **ANSWERED 2026-08-19 at scope: the clause is GENERAL, not entry-specific** — a
  threshold-recurrence routing rule every recurrence-carrying entry inherits, not prose belonging
  to the entry whose subject (guard-kit rule 14) is resolved. Ruled by the iteration lead as a
  routing call, not a fresh envelope one: the 2026-08-17 operator ruling delegated the home decision
  to scope, so recording the answer here **discharges** it. Grounds: the clause completes the scope
  contract's own threshold paragraph, which already puts a collision "in front of the authority
  this stage already escalates to" and stops short
  of saying where that authority changes. Its exercising instance,
  `stage-stamp-ordering-unenforced`, has since landed and retired — recover it from git history.
  **THE TWO-STEP READING IS SETTLED — `lead, own-authority` 2026-09-07 at scope, derived from the
  surfaces and not from precedent:** the scope contract's "regardless of theme" governs the
  PROPOSAL step, TRAJECTORY.md §PRIORITY DIRECTIVE's joining ground the PROMOTION step.
  **What is NOT decided here, deliberately.** The clause is not landed in lifecycle-kit's scope
  contract by this stage or this iteration; that stays a feature-shaped unit for a later one, and
  this entry stays its carrier meanwhile.
  **Two homes were probed and refused at build**, and the refusals are not rejections of the
  options. A live queue entry was blocked at that build because the candidate host stood at zero
  headroom under `check-queue-entry-budget`; **that host has since closed and left the live tree**,
  so the option is absent rather than full — a stronger refusal on a different ground, corrected
  2026-08-24 at close rather than left reading as a cap problem a reflow could solve.
  TRAJECTORY.md is refused by CLAUDE.md's own scoping sentence, which admits **closed** operator
  rulings while this one is explicitly open — choosing it means amending that sentence, a governance
  edit rather than a move. lifecycle-kit's scope contract was refused as envelope-class: a **build
  session correctly declining an envelope call it had no authority to make**, and with the
  general/entry-specific question answered it is the live candidate home rather than a closed one.
  **DISTINCT from `waiting-rule-fourth-firing-post-fix`**, which is Done: that entry owned the
  residency rule's enforcement and got it; this owns where its escalation-routing clause lives,
  which the Done move is what puts at risk.
  **Cost while deferred:** low while this entry exists and unbounded without it — the entry IS
  the carrier, so deferring the *home* decision costs only that a possibly-general rule reads
  as one parked entry's prose; not filing at all would have cost the clause to git history.
  **Out of the icebox until a permanent home lands:** the cost line prices the CARRY, never the
  eviction, and eviction deletes the verbatim clause — the 2026-08-17 ruling's refused attrition.
  not-icebox-eligible: threshold-recurrence-routing-residency 2026-08-17 eviction spends the clause
  Filed 2026-08-18 by close from the gap inbox on the 2026-08-17 operator ruling; the drain
  re-verified that no permanent surface carries the clause — the only hits were the inbox this
  drain truncates and the survey record the next first-stage entry truncates.

- **lead-specifies-constraint-not-mechanism** [design-pending] [cost: event/high] [surface: lifecycle-kit] — whether the lead contract should
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

- **deferred-entry-defer-date-unasserted** [design-pending] [cost: iteration/low] [surface: queue-kit] — a deferred entry whose provenance
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

- **projection-trigger-witness** [design-pending] [cost: event/low] [surface: docs] — the generated-projections roster states each
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

- **cited-script-path-liveness-inline** [design-pending] [cost: event/high] [surface: canon-kit] — `check-docs-cmd` sees a deleted `.sh`
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
  **THIS ENTRY AND `stale-identifier-after-retirement` ARE THE (C) SLICE of
  `citation-liveness-family-convergence`** — grounds relocated here 2026-09-08 from that hub under
  `check-queue-entry-budget`'s rule that an unanswered ground moves to the entry already owning its
  subject, this one having already held the open ruling. The slice widens `check-docs-cmd` from
  fenced-only to inline spans and is priced at likely ONE ticket for the pair; the open ruling is
  which gate holds it. Two of the family's 37 inbound edges land here.
  **Cost while deferred:** measured, recurring, and it lands exactly when the tree is most
  trusted — a green 104/104 battery over prose that names files the same commit deleted.
  Surfaced 2026-08-18. Filed by close 2026-08-18, discharging the gap generalization owed by
  the staleness the `capability-liveness-after-descope` audit turned up in this same commit.

- **shipped-bin-removal-deprecation-path** [design-pending] [cost: event/high] [surface: canon-kit] — deleting a kit-shipped `bin/` tool
  needs no deprecation marker, so that arm of the major-bump criterion is unreachable by design.
  **The instance, probed rather than assumed.** The `freshness-cohort-roadmap-hold-and-batch`
  iteration deleted `drift-kit/bin/trajectory.sh` and `queue-kit/bin/roadmap.sh`. Both shipped:
  the `--pack-installer` arm recursively copies each enumerated kit root into the payload, `bin/`
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


- **spec-lib-dead-derivation** [design-pending] [cost: event/low] [surface: canon-kit] — three section-builder regexes in
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
  couples) and from `native-gate-port-remaining-corpus`, retired 2026-09-10 at zero owed (what the
  port owed, not what it leaves behind unread).
  **Cost while deferred:** the port keeps generating this residue at the rate it lands members,
  and every reader who wonders re-runs the same grep to learn the same thing.
  Filed into the gap inbox 2026-08-19 by the `budget-batch-and-account-identity-kind` close, at
  its capability-pendency audit; promoted at the following scope's drain, the grep re-run at HEAD
  there and the three names still matched by their own definition lines alone.

- **couples-glob-semantics-unowned** [design-pending] [cost: event/high] [surface: gate-sdk] — one manifest field, three readers, two
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

- **prose-tell-threshold-validation** [design-pending] [cost: event/high] [surface: canon-kit] — `check-prose-tells`' numeric thresholds
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

- **template-copy-parity-knobless-refusal** [design-pending] [cost: event/high] [surface: gate-sdk] — `check-template-copy-parity`
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

- **pipeline-membership-idiom-latent** [design-pending] [cost: event/high] [surface: gate-sdk] — the SIGPIPE-under-pipefail membership
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

- **retired-slug-live-pointer-citation** [design-pending] [cost: event/low] [surface: queue-kit] — governed prose can point at a retired
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
  **A second true positive, 2026-09-06, and it falsifies the rarity half of the probe above**, not
  the class: `gate-sdk/SPEC.md` claimed an open installer-and-probe trade "stays
  `install-path-gnu-userland-undeclared`'s" for a slug landed several iterations earlier. Published
  surface, single-backtick form, and `check-queue-slug-liveness`' corpus does not reach it — so the
  false-positive budget question the entry owes now has two instances to size against, both on
  surfaces whose whole purpose is outliving the queue.
  **THE FALSE-POSITIVE BUDGET IS NOW MEASURED, 2026-09-07 at close over `--emit queue-edges`.** Its
  retired block ranks 84 retired slugs across 182 citing lines, 152 distinct (citer, retired-slug)
  pairs. Exactly ONE of the 84 is a live registered gate name — `check-spec-pointer`, whose slug
  once headed an entry — so the rate is 1/84, but that single false positive HEADS the ranking at 15
  inbound, ahead of every true positive. A ranker must therefore filter against `scripts/gates.list`
  before it reads the block at all; the budget is cheap and the ORDERING is what the collision
  breaks. Of the 152 pairs, most already spell "retired" inline and are correct prose; the drain's
  own corrective reach is this iteration's own retirements, which is what a close can honestly hold.
  recurrence: retired-slug-live-pointer-citation 2026-09-06

- **settings-hook-command-path-gate** [design-pending] [cost: event/high] [surface: context-kit] — a hook registration in
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

- **guard-rule-number-not-citable-outside-kit** [design-pending] [cost: event/low] [surface: guard-kit] — a guard-kit rule number is a
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
  **Why `[design-pending]`:** the second is a one-paragraph boundary note plus a sweep of unknown
  size, and whether a bare "rule N" outside the kit is gateable at all needs a false-positive
  budget nobody has measured.
  **THIS ENTRY AND ITS `guard-rule-number-intra-kit-citations-ungated` SIBLING ARE THE (D) ISLAND
  of `citation-liveness-family-convergence`, and its LONG POLE rather than an equal quarter** —
  grounds relocated here 2026-09-08 from that hub, under `check-queue-entry-budget`'s rule that an
  unanswered ground moves to the entry already owning its subject. Slices (A)-(C) widen gates that
  already resolve citations; (D) has no gate to widen and an unmeasured false-positive budget over
  111 intra-kit citations. Never average it into that family's size floor.
  **Cost while deferred:** a reader follows the number to the wrong rule and reasons from it.
  recurrence: guard-rule-number-not-citable-outside-kit 2026-08-29
  Filed 2026-08-22 at align's cross-audit; drained at that iteration's close, which found the
  bullet had named the wrong slug and located the real entry before dispositioning.

- **guard-rule-number-intra-kit-citations-ungated** [design-pending] [cost: event/low] [surface: guard-kit] — guard-kit cites its own
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

- **bespoke-test-path-knob-pinning** [design-pending] [cost: event/high] [surface: gate-sdk] — a bespoke gate-test's cwd sandbox is
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
  more, reading a foreign `validate-evidence.txt`. That is the same failure `c1375e99`'s
  process-wide export produced and `80d74291` narrowed away — an export was one way to supply an
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
  **THE PREDICTED EXPOSURE FIRED, 2026-09-05, and it widens fix shape 2 rather than this entry.**
  The cost field above said "nothing is red today"; this iteration produced the red — a bespoke
  test's isolation defeated by ambient env it did not own, reaching `--run-validate`'s verdict.
  The mechanism was a different knob class (a bridged `GATE_SDK_KNOB_*` scalar inherited from a
  sibling arm, not an absolute path knob from an operator export), so the instance is filed as its
  own entry, `run-validate-child-env-knob-leak`. What it changes HERE is the fork: shape 2 —
  widening `gate-sdk/lib/test-hermetic.sh` to neutralize the knobs — now covers two knob classes
  rather than one, while shape 1's per-test pinning covers only whichever class each author
  anticipated. The choice was made on 7 one-line edits against one bootstrap change; it should be
  re-made on that.
  recurrence: bespoke-test-path-knob-pinning 2026-09-05
  Filed 2026-08-23 by validate; the close drain re-ran the survey oracle and got 16/11/7 with the
  same seven names.

- **validate-tier-premise-mechanical-only** [design-pending] [cost: event/low] [surface: lifecycle-kit] — the ruling-config assigns `validate`
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
  exported the pin process-wide and broke `producer-lock.test.sh`'s sandbox, narrowed at `80d74291`)
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

- **bridged-knob-case-tmp-dir-override-inert** [design-pending] [cost: session/high] [surface: gate-sdk] — the `--run-gate-tests`
  arm's case scratch pin protects no bridged gate, so a bridged/native gate writes its scratch into
  the tracked fixture corpus it is the oracle for.
  recurrence: bridged-knob-case-tmp-dir-override-inert 2026-08-25
  **FIRST RECURRENCE, 2026-08-25 at close, and it re-fired in the ATTESTED shape rather than a
  variant.** A battery run regenerated the scratch, and the next `installer_smoke` run died at
  `checkwright init`'s `git add` on the ignored path — masking the ruled binary-less scenario
  exactly as the cost line predicts. The masking cost a diagnosis a second time: only deleting the
  regeneration and re-running showed the four profiles clean and the failure landing where the
  baseline says it does. TWO READINGS THE RECURRENCE ADDS. First, the regeneration is not
  occasional — the producing gate runs in the battery, so ANY session that runs the battery before
  the installer suite meets the mask, which makes the masked reading the default ordering rather
  than the unlucky one. Second, the 2026-08-24 attribution ruling was VINDICATED by this firing
  rather than merely upheld: because the baseline row still names the standing cause, deleting the
  transient one exposed the real red immediately instead of leaving a re-attributed row that would
  have read as satisfied.
  **The mechanism, re-measured 2026-09-11 after the harness port.** The `--run-gate-tests` arm
  resolves a case's argv by running `gate_command` inside the case dir, which bakes the ambient,
  repo-relative `GATE_SDK_KNOB_GATE_SDK_TMP_DIR=.tmp` into its `env` prefix. The arm's scratch pin
  sets the bare `GATE_SDK_TMP_DIR`, which no native gate reads, so `.tmp` resolves inside the case
  dir. Probed on the resolved argv, and one clean `gate_sdk` fixture-suite run regenerated the
  residue. The shell harness this entry first measured had the same shape.
  **Reproduced live rather than reasoned.** `check-crate-arms` ported from `.sh` to `.gate` plus a
  crate module this iteration, which silently reopened exactly the corpus-pollution class
  `c1375e99`/`80d74291` closed. With both fixture `.tmp` dirs deleted, a plain `gate_sdk`
  fixture-suite run — no manual invocation — regenerated
  `gate-sdk/gate-tests/check-crate-arms/good/.tmp/crate-arms-<hash>.green`. Watched twice.
  DISTINCT from `pack-installer-vendors-untracked-scratch`, retired, upstream of it. That entry's
  `cp -R` filter gap is the VENDORING symptom; this is the cache-pollution CAUSE, in the harness
  itself, for every future native-ported gate that declares the `GATE_SDK_TMP_DIR` knob against a
  tracked fixture pair — not only this one member.
  **Why `[design-pending]`:** the candidate fix is to pin the case scratch in the knob form the
  member reads rather than only the bare name, but the config bridge is a single-producer surface
  and moving where a knob's value is computed for one caller is the criterion-6 question in
  miniature.
  **Cost while deferred:** a fixture-suite run leaves ignored scratch inside a tracked fixture
  corpus. The packer now archives the tracked set alone, so the vendoring that cost the diagnoses
  above no longer follows from it; any other reader of that corpus that walks ignored paths does.
  **The baseline row was NOT re-attributed to this entry, ruled 2026-08-24 and recorded so the
  next validate does not re-litigate it.** While the loop was shell, that `installer_smoke` row
  kept `binary-less-dispatch-loop-retirement` as the standing unpaid price it held visible, and
  this one was a transient masking cause stacked on top of it — exactly
  the split evidence-kit/SPEC.md §Baseline manifest now rules generally, which is where the ruling
  was landed rather than left as this instance's precedent. This entry's cost line above is where
  the masking diagnosis belongs. The loop has since retired and that row now reads pass.
  **Re-verified at close and kept.** Operator direction, 2026-09-11, lead-relayed: re-verify, then
  retire. It was routed on a survey reading the premise stale; the re-run held it, so no retire.
  Filed 2026-08-24 to the gap inbox by validate, declared distinct; promoted that day at
  `shell-gate-tail-port-and-completion-oracle`'s close, deleting the regenerated instance; its
  recurrence's attribution was ruled at `execution-control-reach-and-turn-end-blocking`'s close.

- **dispatch-claim-evidentiary-tier-unmarked** [design-pending] [cost: event/high] [surface: lifecycle-kit] — a dispatch prompt can upgrade an
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

- **file-authoring-act-ungoverned** [design-pending] [cost: session/low] [surface: guard-kit] — the file-authoring writes no glob can reach,
  and a prepared settings diff no stage session may apply.
  **What landed 2026-09-04 and what did not.** Guard rule 17 was narrowed and widened in one unit:
  its substitution decline moved onto the `hdq` view, so the quoted-delimiter heredoc body every
  journal append spells its slugs in stops defeating the grant; and its `>>`-only test went, so a
  create to a gitignored target is granted where truncate-then-append already granted it in two
  calls. Measured at that build: sixteen of sixteen locatable journal appends carried a backtick,
  and the class was about half of every prompting call in the snapshot. The record is
  guard-kit/SPEC.md §The generic ruleset rules 16 and 17 and §scan-prompts, not restated here.
  **WHAT KEEPS THIS ENTRY ALIVE IS TWO THINGS, and neither is design this project can self-serve.**
  **(1) A PREPARED SETTINGS DIFF AWAITS OUT-OF-BAND OPERATOR APPLICATION**, recorded here under
  guard-kit/SPEC.md §compare-settings-allow, which lets a session derive a diff and state its
  grounds and ends its remit there. Six committed `permissions.allow` entries:
  `Bash(date *)`, `Bash(find *)`, `Bash(git merge-base *)`, `Bash(git config *)`, `Bash(mkdir *)`,
  and `Bash(: > .workflow/subagent-stop-liveness.log)`. SECURITY GROUNDS: the first five are
  read-only or scratch-creating, take no path outside the tree that a `*` does not already reach in
  a sibling grant, and each ranked in the live friction log; the sixth is a fixed literal whose two
  sibling reclaim paths are already granted, so its absence is an asymmetry rather than a decision.
  **These do NOT fix this entry's own class** and are not offered as doing so. They ride the same
  operator application, and this entry is the queue's carrier for that obligation — which is why the
  2026-09-04 build DEMOTED it rather than moving it to `## Done`, where a bare slug would have taken
  every tag and the obligation with them.
  **(2) THE RESIDUE THE LANDED GRANT DOES NOT REACH:** a write to a path OUTSIDE the gitignored
  scratch set — a heredoc to a tracked file, a commit-message file, a scratch script under another
  root. Rule 17 declines every one on its target test, by design. The Write-tool steer was RECORDED
  AS REFUSED-FOR-NOW at that build: the landed deltas removed its subject, and a steer firing on
  writes the same commit had just granted is a rule arguing with its neighbour. Two facts were
  bought so a later session need not re-buy them — `.claude/settings.json` already carries a
  `Write|Edit` `PreToolUse` matcher, so a Write-side rule needs no new matcher shape; and a steer is
  not a distinct primitive, `guard_block` being stderr plus exit 2 for a block and a steer alike.
  A stated habit was the shape filed beside those two and it stays refused: it is not a mechanism.
  **The operator ruling that scoped it, and its dated series, which lives once and here.** The class
  read 41 of 185 prompting calls, then 56 of 139 at the 2026-09-04 close — 22 per cent to 40 per
  cent — then 48 across 22 patterns on the mid-iteration snapshot the promoting spec measured afresh
  under the ruling's re-costing rider. The ruling REOPENED the port-only run for a second non-port
  unit. **A later reading of this class will fall for a reason other than fewer writes**: a granted
  call never reaches the friction log, so widening rule 17 shrank the log's own corpus.
  guard-kit/SPEC.md §scan-prompts carries that caveat, and a close attributing the drop to fewer
  writes is reading the instrument's own boundary as a result.
  **Cost while deferred:** one out-of-band decision per file authored outside the scratch set, plus
  the six ungranted entries above, and invisible to every gate — the friction log is advisory, so
  nothing reds however far the residue grows.
  Filed 2026-08-24 to the gap inbox by spec, as the surviving half of a split its sibling recorded;
  drained 2026-08-24 at that close; scoped 2026-09-04 at close; promoted, built and demoted on
  2026-09-04 within one iteration.



- **kfric-obligation-residency** [design-pending] [cost: iteration/low] [surface: delegation-kit] — the knowledge-friction capture obligation is
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
  DISTINCT from `kfric-empty-log-ambiguity`, retired, which asks how an EMPTY log should be READ and
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

- **release-body-step-has-no-in-tree-witness** [design-pending] [cost: event/high] [surface: RELEASING.md] — the one release step whose
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

- **precondition-gate-direction-blindness** [design-pending] [cost: event/low] [surface: queue-kit] — `check-queue-prose-precondition`
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
  Filed 2026-08-24 to the gap inbox by scope, which reproduced the red; drained and promoted
  2026-08-25 at close, which read the gate's four help lines first-hand and corrected the filing.

- **gate-command-status-conflation-third-caller** [design-pending] [cost: event/low] [surface: gate-sdk] — a third call site conflates
  `gate_command`'s harness-error exit with a resolution failure, and its guard for the real case
  is dead.
  **Both defects read off the source at this drain rather than cited.** In
  `gate-sdk/bin/run-gate-tests.sh`, `run_case` reaches `gate_command` through a PROCESS
  SUBSTITUTION, so the function's exit 2 for a harness error — an absent dispatch binary, a
  refused knob bridge — kills only the subshell and reaches the caller as an empty argv. The
  `if !` arm around it is dead, because `mapfile`'s status is `mapfile`'s own and not the
  substitution's, so only the empty-argv arm ever fires; both arms print the same
  resolves-in-no-check-dir line, which is FALSE for the binary-absent cause — the gate resolved
  perfectly well and merely could not be built.
  **Distinct from the closed exit-class unit, and NOT a re-filing of it.** That unit repaired
  this shape at `scripts/gate-exec.sh` only and enumerated its blast radius as the name-addressed
  pre-flight callers; this caller sits in gate-sdk's own `bin/` and outside that unit's audited
  update-target roster, which is why it was filed rather than folded in.
  **The verdict is already right and the defect is on the MESSAGE axis alone** — `run_case`
  returns 2 and the run counts a harness failure, so nothing silently passes. But that is the
  same axis the closed unit exists for, and gate-sdk/SPEC.md §lib/gate.sh now states the caller
  obligation generically, so this is a stated-rule violation rather than an inconsistency.
  **Shape:** `run-gates.sh`'s existing pattern is prior art in the same directory — capture
  through a command substitution, keep the status, and name resolves-in-no-check-dir only on
  status 1. It also owes an update to gate-sdk/SPEC.md §run-gate-tests, which is what puts it
  past a mechanical sweep and made it a scope change rather than in-envelope calibration.
  **Cost while deferred:** low and diagnostic — a fixture run against a stale binary tells its
  reader the gate does not exist, pointing at a registration fix instead of at
  `bash gate-sdk/bin/build-native.sh`.
  **The authority named on the Filed line below is ITSELF A FILED QUESTION — read it there rather
  than re-deriving it:** `pre-grammar-disposition-authority-ambiguity` owns whether an ungrammared
  disposition naming an operator CLASS and a LEAD ruler is read as one or the other, and the
  operator ruled 2026-09-03 that the ambiguity is FILED rather than settled. This entry's own
  deferral turns on neither reading, so a drain meeting this row disposes of it without escalating.
  Filed 2026-08-24 by build while landing the exit-class unit; DISPOSITIONED BY OPERATOR-CLASS
  RULING at the 2026-08-25 close — the lead ruled it STAYS DEFERRED on CLAUDE.md's
  scope-gated-intake rule, and directed it be promoted as a filing rather than started as work.

- **amendment-update-target-overcount-undetected** [design-pending] [cost: event/low] [surface: canon-kit] — an amendment's
  `## Existing sections updated` roster is unchecked in the OVER-count direction, so a rostered
  target with nothing to update is discovered only by a session that goes looking.
  **The instance, and the correction this drain made to it.** The stage-journal amendment rostered
  `lifecycle-kit/templates/stages/` as a delta-1 target for "every stage template's
  dispatch-facing text that names a journal path", and its DoD named hand-spelled journal paths
  as the strings to chase. No stage template spells one, re-verified here. But the filing's
  stated sweep result — "NOT ONE tracked file spells a journal path ... anywhere else" — is FALSE
  as written: `guard-kit/guard-tests/cases.tsv` spells one six times. Those are guard fixture
  INPUTS with no reader, so the substantive conclusion holds while the sweep's reported reach
  does not, and the entry carries both rather than the tidier half.
  **Why the correct outcome is still the defect.** The roster bullet was vacuous and the DoD item
  was discharged by finding nothing, which at read time is indistinguishable from a skip.
  **The filing's "neither entry's fix reaches the other's direction" FELL at the drain.**
  `amendment-reader-roster-undercount` is the under-count direction, where the failure is a
  missed edit; this is the over-count, where the failure is a session either fabricating an edit
  to satisfy a bullet or silently dropping it. But two of that entry's three candidate
  deliverables — make the build-stage re-sweep a contract line, or drop the roster and keep only
  the sweep instruction — reach BOTH directions squarely. The two are distinct in FAILURE MODE
  and overlap in REMEDY, so whichever is taken first should take both.
  **No `recurrence:` date joins that sibling:** its finding is a roster that UNDERCOUNTED, and an
  undercount did not re-occur. The mirror direction is a new defect and files as one.
  **Deliverable:** decide whether `check-amendment-update-target` can assert that a rostered path
  exists AND that the amendment's own cited string is present in it, or whether the honest answer
  is that a roster is a claim and the merging session owes a stated finding when one comes up
  empty.
  **Cost while deferred:** low per instance and paid at the worst moment — a build session either
  buys a sweep the amendment implied was already scoped, or writes something to make a vacuous
  target non-vacuous.
  **JUDGED RECURRENCE 2026-09-05 at the close drain, on a second roster.** `SPEC-preflight-cut.md`
  rostered eleven reader rows for ten readers: a quote living at `evidence-kit/SPEC.md:836` was
  also attributed to `delegation-kit/SPEC.md:836`, and align verified the citation's TEXT without
  checking that the file named holds it. Same finding, one axis over — a rostered row resolving to
  nothing in the file it names, undetected — and the deliverable below already covers it verbatim,
  so no entry is minted. It widens the deliverable's corpus from the update-target roster to any
  amendment roster carrying a path.
  recurrence: amendment-update-target-overcount-undetected 2026-09-05
  Filed 2026-08-25 by build; drained and promoted 2026-08-25 at close, which re-ran the sweep and
  corrected its reported reach.

- **readme-roster-enum-coverage** [design-pending] [cost: iteration/high] [surface: canon-kit] — a kit README enumerating a
  **derivable** set is outside every parity gate, so it drifts silently while the
  battery stays green.
  recurrence: readme-roster-enum-coverage 2026-08-25
  `check-readme-roster` holds one roster per README — the
  `checks/` basenames — and nothing else; `check-prose-enum` holds only the sets
  the `--emit-enum-sets` arm declares, which is the queue tag vocabulary plus two
  derived roster families over the kit tree, none of them a behavioural enum.
  **Three instances now, every one found by close's step-5 staleness read rather
  than by an oracle.** (1) drift-kit/README.md omitted a bundled lead KPI shipped
  that iteration and registered in `scripts/kpis.list` — a registry that is
  exactly an enum-set source. (2) queue-kit/README.md's `## Use` block omitted a
  `queue-index` invocation the SPEC states outright, while the README is the only
  invocation surface a reader gets. (3) THE THIRD, 2026-08-25: delegation-kit's
  README said the turn-end liveness hook refuses on `red` or `corrupt` while the
  iteration's own landing widened the refusal guard to a third verdict, verified
  against the consumer script's own disjunction rather than off the diff. All
  three were corrected by hand at the close that found them, which is the
  Enforcement-first shape the doctrine bars — the fix landing without the gate.
  **The third instance MOVES this entry back out of the icebox, and it also
  falsifies half its own cost claim.** "Low and non-rotting" was written when both
  instances were omissions from a roster that never became false; this one is a
  README stating a live refusal contract that the tree had already widened, in an
  ADOPTER-FACING install step, in a file the widening diff never opened. So the
  class does not merely cost a close's attention — between closes it ships a
  false contract to a consumer wiring the hook, which is a different and higher
  cost than an incomplete list.
  **It also sharpens the shape.** The first two instances were rosters derived
  from a registry FILE. This one is a set that exists only as a `[[ ]]`
  disjunction inside a shell guard, so no registry read reaches it and an
  extractor would have to be written against that one script. That is either the
  case that makes the enum-set survey worth buying, or the case that bounds it.
  **Why `[design-pending]`:** an enum set is cheap to declare and expensive to
  land, because declaring one obliges **every** prose enumeration of that set,
  tree-wide, to be complete. The unit owes a survey of what each candidate set
  would red before it is declared, plus a ruling on whether a behavioural set
  living in a conditional is an enum set at all or wants a different parity
  shape. The count half of this class is `check-measured-claim`'s, not this
  entry's: a bare cardinal qualifying a roster is a different scanner from a
  membership check.
  **Cost while deferred:** paid once per close by the staleness read, which is
  the only detector — and, as the third instance shows, paid by a consumer in
  between when the drifted roster is a contract rather than a list.
  Surfaced 2026-07-31 by close's top-level staleness review, which found the
  first two instances; filed rather than fixed because the enum-set survey is the
  work. Evicted to the icebox on a low, non-rotting cost; returned to Deferred
  2026-08-25 on a judged recurrence, the tag algebra's own icebox exit.

- **icebox-eviction-line-budget-squeeze** [design-pending] [cost: event/low] [surface: queue-kit] — the icebox tier's one-line grammar
  and `check-queue-wrap`'s column cap are jointly unsatisfiable above a slug length nothing
  bounds, and nothing says so at the point of eviction.
  **Attested first-hand at the 2026-08-25 close, three failed attempts rather than predicted.**
  Evicting `spec-embedded-source-criterion-4-membership` (icebox) — a 44-character slug — left 31
  columns after the mandatory `- **<slug>** [design-pending] — ` prefix. All three candidate
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
  recurrence: icebox-eviction-line-budget-squeeze 2026-09-03

- **worktree-isolated-agent-report-lost-to-a-failed-peer-send** [design-pending] [cost: event/high] [surface: delegation-kit] — an isolated
  read-only sweep's final report reaches its dispatcher as a bare `.`, because the child sends to
  a peer name it cannot resolve and the harness returns only the last assistant message.
  **Reproduced twice at one close, 2026-08-25, not predicted.** Two `audit-sweep` dispatches
  carrying `isolation: worktree` each completed substantial work (46 and 49 tool uses, ~163k and
  ~97k child tokens) and each returned a single period. Both had to be resumed with an explicit
  "put it in your final assistant message, do not use SendMessage" instruction, after which both
  reported in full. The work is not lost; it is paid for twice, and the second payment is a whole
  extra dispatch round-trip.
  **The cause is stated by the harness itself and is not a guess.** The agent-dispatch guard
  already warns at dispatch time that a grandchild has no upward channel and that neither level
  knows its own address; one child said so outright in its recovered report — it could not resolve
  the dispatcher's name and had no roster tool to find a ref.
  **NOT the guard's defect.** Its warning is accurate and fires at the right moment; what is
  missing is that nothing carries the warning INTO the child, so the child learns its own
  isolation only by failing.
  **Deliverable — rule one of three.** The dispatch-shape guard appends a return-value-only
  instruction to a prompt whose type is read-only and whose isolation is worktree, the one shape
  that provably cannot message back; or delegation-kit/templates/agent-execution.md states the
  return-value-only obligation as a contract the dispatching session spells into such a prompt,
  which costs one line and no code; or the agent-type definition for read-only sweep types carries
  it, which reaches every dispatch of that type without touching any dispatcher.
  **DISTINCT from `worktree-isolated-dispatch-cannot-reach-the-main-checkout`**, deliberately not
  re-filed here: that entry is about a child's WRITES — a binary-dispatched gate it cannot resolve
  and a capture log it writes into a doomed worktree — and its bridge is `git rev-parse
  --git-common-dir`. This is the child's RETURN VALUE, it has no filesystem half, and that bridge
  does not touch it. The two share the isolation flag and share no fix.
  **Cost while deferred:** one wasted dispatch round-trip per isolated sweep, paid by the
  dispatcher at the moment it is waiting on the result — and silent, since a bare `.` reads as an
  agent that found nothing rather than as an agent whose report was dropped. That last reading is
  a correctness risk rather than an efficiency one, and it is the expensive half.
  **The floor's coverage is now measured, and it is half of shape one.**
  The `agent-dispatch-guard` arm's D2 rule refuses a read-only type dispatched WITHOUT
  `isolation: worktree`, and its D3 rule appends the return-value-only advice — but that second
  branch fires only when the dispatcher is ITSELF a dispatched agent (a nested dispatch). A
  top-level lead dispatching the same read-only sweep gets the isolation refusal and no
  return-value instruction at all. So the guard already reaches the ISOLATION half of shape one and
  is silent on the CHANNEL half, which is the half this entry is about.
  **A further ground, three sessions paid for it 2026-08-26.** For a read-only fan-out the RETURN
  VALUE is the contract; the resume-journal path a dispatcher grants is for agents that MUTATE.
  Worktree isolation and the journal answer different questions, and granting the journal to a
  read-only child buys nothing while making the dropped return look like a channel that was
  offered. `delegation-kit/templates/agent-execution.md` already draws the distinction; nothing
  makes a dispatcher pay it, which is shape two of the deliverable restated as an observed cost.
  recurrence: worktree-isolated-agent-report-lost-to-a-failed-peer-send 2026-08-26
  Surfaced 2026-08-25 by the `turn-end-liveness-seam-and-worktree-cause` close, which reproduced
  it twice while dispatching its own sweeps, and filed to the gap inbox there; promoted
  2026-08-25 at this scope's drain of that inbox.

- **site-health-issue-venue-unwanted** [design-pending] [cost: event/low] [surface: site-kit] — the site-health probe files issues on
  the public repo for failures the iteration lifecycle resolves anyway, and the operator does not
  want that venue.
  **Operator-ruled 2026-08-25: the issue-filing path is unwanted.** The objection is to the
  **venue**, not to the probe — and a later session must not read it as the probe being wrong.
  Both firings were true positives on arm #6, the Release body missing its note URL: 2026-08-08
  on `v0.22.0` and 2026-08-24 on `v0.25.0`, each cleared by the probe's own recovery path.
  **Those dates sit in this prose deliberately.** The operator has since deleted both issues —
  probed here, `gh issue list --state all` returns nothing — so the tracker is empty, two dead
  run-log URLs are all that survives of the evidence, and the underlying defect is tracked at
  `release-body-step-has-no-in-tree-witness` rather than at any issue that still resolves. The
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
  Operator-directed filing 2026-08-25, relayed through the lead at this scope; the tree read
  behind it was re-run here rather than taken on the relay.

- **citation-liveness-family-convergence** [design-pending] [cost: iteration/high] [surface: canon-kit] — the citation-liveness deferred family
  is FOUR gate-touch points, not one resolver and not fourteen tickets; the measurement lives here.
  **Why the survey lives here:** `.workflow/survey-record.md` is boundary-truncated and
  `check-scratch-citation` reds a permanent pointer into it. **Witness re-run at scope 2026-09-08,
  all sixteen named slugs LIVE** — corpus the two design-pending sections, `scripts/gates.list` and
  `native/src/gates/`; oracle `--emit queue-edges` and a citation grep; rev `f2308550`.
  **The finding.** Thirteen live members, eleven Deferred and two Icebox, none blocking on an
  operator-class fork; **size floor** four touch points, eight to ten assertions, two reports, all
  native modules. (A) `check-spec-pointer` absorbs `prose-filename-citation-liveness`,
  `unqualified-section-citation-liveness`, `link-wrapped-section-citation-liveness` and
  `spec-pointer-self-section-citation` as ONE resolves-to-nothing predicate — those entries say the
  guard WINDOW, not the citation form, is the variable — plus `spec-section-title-collision` and
  `qualified-pointer-section-ownership` as two harder predicates on the same gate, the latter
  self-declaring an honest not-buildable as a permitted outcome. (B) `check-queue-slug-liveness`
  takes about two assertions for `retired-slug-live-pointer-citation` and
  `queue-status-parenthetical-liveness`, plus ONE report-only deliverable riding the queue-edges
  arm's resolution, `done-slug-ownership-citation-report` — report-not-gate under the SPEC's
  reference-vs-membership ruling, its twin `dead-queue-citation-report` shipped. (C) is
  `cited-script-path-liveness-inline` plus `stale-identifier-after-retirement`, likely one ticket.
  **(D), the two `guard-rule-number-*` entries, is ANSWERED and out**, both slices' grounds having
  relocated 2026-09-08 to the entries already owning them.
  **Two members are content-unverified though both re-verified live above:** Icebox's
  `false-ground-citation-propagation` is title-only and `doctrine-rule-number-citation-liveness` a
  forward bet, a 2026-08-27 probe having found DOCTRINE.md carrying ZERO `rule N` forms — both
  one-liners with nowhere to hold a ground, which is why these stay and (D)'s did not.
  **Excluded with cause:** `scratch-citation-skill-surface-reach` (self-disclaims, glob-coverage),
  `kit-ref-liveness-stem-token-hole` (env-knob tokens), `fixture-assertion-liveness`,
  `survey-oracle-liveness-unasserted`. **Adjacent, unfolded:**
  `amendment-landing-citation-assertions` and `amendment-owner-position-citation` ride
  `check-amendment-queue` over a corpus deleted at merge. **Fold-or-exclude, both filed after the
  rev:** a FIFTH point `scratch-citation-introducer-form-reach`, an EIGHTEENTH member
  `queue-citation-line-number-stales-within-its-own-session`.
  **MACHINERY-class HOLDS by the 2026-08-30 discriminator** — the family's only demand witness is
  this repo's delivery process — **but the class bar is ANSWERED, the operator having carved this
  entry out 2026-09-08**, so COST alone blocks and a costed look owes the honestly-low fields first.
  **NOT THIS WINDOW, TWICE, AND THE CARVE-OUT'S REACH IS SETTLED — `lead, own-authority` 2026-09-08,
  then operator 2026-09-09 (AskUserQuestion in a lead session, lead-relayed): the carve-out buys
  ADMISSIBILITY, never admission.** The three joining grounds apply unchanged and this family waits
  for a cut sharing its surface; 2026-09-09's, the behind-invoke relocation, shares none of the four
  points above. Answered, so no boundary carries the reach forward as open.
  **The zero-inbound RANKING ground this entry carried is FALSIFIED and deleted rather than left
  standing:** summed 2026-09-08 the family carries **37 inbound over seventeen live members**
  against the hub's 1, four times the largest single entry. **THE HEALTH TRIAD'S QUEUE LIMB READS
  THE WRONG WAY** — 301 Deferred + 101 Icebox against 292 + 84 on 2026-09-05 — so (A)+(B)+(C), nine
  entries over three gate widenings, is the largest lever the pool offers on it.
  **Cost while deferred:** the expensive half of the pool's largest measured exit dies at every
  first-stage entry that finds it uncarried, and re-buying it costs a full deferred-pool sweep plus
  a read of four gate sources — the re-derivation the survey record exists to prevent. Filed
  2026-08-25 by close, draining the gap inbox; survey bought at that iteration's scope.

- **wait-record-self-deadlock** [design-pending] [cost: event/high] [surface: delegation-kit] — a backgrounded **wait** that registers itself as
  a producer makes its own exit condition unsatisfiable, and blocks every concurrent session's
  commits while it spins.
  **Attested live 2026-08-26.** This iteration's validate session backgrounded
  `until bash gate-sdk/bin/run-gates.sh --enter-stage --simulate validate;
  do sleep 15; done` and, per the
  standing launch-liveness rule, wrote `.tmp/validate-entry-wait.run` naming its own pid. This repo
  wires `check-producer-liveness .tmp` as a validate entry pre-flight, so the poll refused on the
  record the poll itself had written; the only thing still blocking the loop was the loop.
  **Second-order harm, wider than the filing claimed — corroborated at this close from a surface
  the filer never cited.** The tracked-tree-mutation rule correctly refuses every git index,
  worktree or ref write in *every* session while a record names a live pid, so the build session
  could not commit the queue drain the waiter was waiting for. And
  `.workflow/subagent-stop-liveness.log` shows the SAME record refusing SubagentStop 21 times
  between 07:05Z and 07:15Z on 2026-08-26 (`live=yes verdict=red records=1 decision=refuse`), so
  the wedge reached the turn-end path too, not only the poll. Three consumers, one record.
  **The distinction the rule does not draw, and one surface already draws it.** A *producer* writes
  artifacts a reader must not race and owes a record; an *observer* writes nothing and owes none.
  Guard-kit's backgrounded-launch rule now exempts an inline wait loop from its record block, so
  the attested spelling owes none; a wait behind a script name still does, and the agent-execution
  template draws the split only by pointing at that rule — the attested instance followed no split.
  **Why `[design-pending]`, three candidate fixes differing in kind:** state the producer/observer
  split in the agent-execution rule so a wait never registers; have `check-producer-liveness` ignore
  a record whose run key names the stage being entered; or refuse the self-naming record at write
  time.
  **DISTINCT from `close-entry-baseline-bootstrap-deadlock`**, closed this iteration: that one is a
  circularity in what the close-entry evidence manifest demands, with queue and baseline content on
  both sides. This is a liveness record invalidating its own waiter, with neither involved.
  **Cost while deferred:** the rule as written walks a session into a wedge that costs that session
  and every concurrent one, and the only escape is deleting a record that still names a live pid —
  the one act the rule names as retracting a statement that is still true.
  Filed 2026-08-26 by close, draining the gap inbox; found 2026-08-26 at build, observing validate.


- **account-noun-plural-slips-the-shape** [design-pending] [cost: event/high] [surface: gate-sdk] — the account-identification pattern
  matches a singular account noun only, so the plural form passes both readers.
  **Probed rather than reasoned, at this close.** Feeding a three-line sample through
  `grep -nE -f scripts/msg-patterns.list` matches the singular line and matches NEITHER plural: the
  noun alternation is `(account|login|username|handle)` followed by a required non-letter, and a
  trailing `s` is a letter, so the boundary fails in BOTH orderings of the shape. The reach is
  wider than the limit was reported as — `accounts` slips with `logins`.
  **THE HOLE IN THE FORM THAT MATTERS, because the plural is not an exotic phrasing but the
  natural way to write the exact leak this pattern exists to catch.** A sentence of the shape
  *"the accounts were X and Y"*, with X and Y handle-shaped and backticked, passes both readers
  clean — while the singular *"the account is X"* reds. So the guard is strongest against the
  phrasing that names ONE identity and weakest against the phrasing that names SEVERAL, which is
  the correlation case, and inverts the pattern's own stated rationale that naming an account
  correlates identities. The bad-fixture sentence a fix should pin is that one, not a contrived
  minimal pair.
  **Where the limit currently lives, and why that is the filing's whole point.** It is stated in
  commit `3763bc3e`'s body ("the plural form slips: tolerating it was measured and still costs
  three rewordings of prose that is not wrong") and NOWHERE in `gate-sdk/SPEC.md`
  §check-commit-msg, which records only the OTHER limit that commit names — that account topology
  is a proposition no token pattern reaches. Verified by grepping that commit's own SPEC diff. So
  the limit is held in history, and history answers what happened, never what is correct
  (CLAUDE.md §Delivery doctrine, spec-over-precedent).
  **The tolerance was a measured choice, not an oversight**, and it is the reason this is
  `[design-pending]` rather than a patch: extending the noun set to plurals reds three tracked
  sentences that are not wrong, and the same tree-exact calibration that governs the singular form
  says rewriting prose to satisfy a heuristic inverts the rule the heuristic serves. The open call
  is which of the three moves to take — accept the three rewordings, add a plural arm scoped to the
  MESSAGE reader only (whose over-refusal economics differ, per §check-commit-msg), or leave the
  gap and record it in the SPEC where the sibling limit already sits.
  **Cost while deferred:** the leak class the pattern was built for reaches public history through
  one letter, and nobody reading the SPEC learns that — the section's stated scope reads as
  complete.
  Filed 2026-08-26 by close, triaging a build finding relayed through the lead.

- **shipped-config-tightening-undeclared** [design-pending] [cost: event/high] [surface: gate-sdk] — a kit-shipped pattern-list change
  tightens two gates in an adopter's tree, and no surface says which release section owns it or
  who may declare it once the landing stage is gone.
  `3763bc3e` added an account-identification pattern to `gate-sdk/templates/msg-patterns.list`,
  the config template the installer's `init` seeds into a consumer's gates dir. **The gate code
  did not change** — that commit's `native/src/gates/commit_msg.rs` diff is entirely inside
  `mod tests`, so the whole tightening is data. `.workflow/tightened-gates.txt` carries neither
  `check-commit-msg` nor `check-tree-terms`.
  **CALL 1 IS RULED — BOTH SECTIONS, ALWAYS. Operator ruling 2026-08-26**, relayed through the
  iteration lead at this scope's escalation. A tightening that ships as kit template data
  declares in **both** `Tightened gates` **and** `Behavior changes`. The ground is the adopter
  split below: it is real, so the ruling removes the choice rather than making it, at one extra
  line per event. Executing it — the edits to gate-sdk/SPEC.md §upgrade-smoke and
  docs/install.md — is a spec stage's act when this entry promotes, deliberately not taken here.
  **Call 1 CANNOT BE EXECUTED until call 2 is filled, and this coupling is why the ruling is not
  self-discharging.** "Declare in both sections" names no one who may append when the discovering
  stage is not build, so `3763bc3e`'s tightening stays undeclared until a producer exists and
  surfaces at the next release tag as an adopter meeting a red the note never named.
  **The adopter split the ruling rests on.** `claim()`
  (`native/src/installer/init.rs:123`) rewrites a seeded path whose recorded hash still matches, so
  an adopter who never edited their copy TAKES the new pattern on upgrade and both gates can red on
  a clean run — the Tightened-gates allowed-red set's subject exactly. An adopter who did edit it
  keeps their copy and diverges, which docs/install.md folds into Behavior changes by name:
  "a template you have copied out that then changed *is* depended-on behavior diverging from your
  copy — it is behavior-folded, not dropped". That folding rule reaches the edited population
  only; it never contemplated the unmodified-seeded-copy case, where init writes through. Two
  populations, two sections; the ruling above now requires both.
  **OPEN CALL — who may declare a late-discovered tightening. Left open 2026-08-26 by lead ruling**
  for a later spec stage: it is a hole inside the existing envelope, and filling a hole in an owner
  doc is spec's work rather than the operator's. gate-sdk/SPEC.md §upgrade-smoke
  names the build stage the producer, on the ground that build alone "knows what it tightened at
  the moment it tightens it". A tightening found after that stage closed therefore has **no
  declared producer at all**, and every stage that could append is out of contract.
  **Cost while deferred:** nothing can red for it. `check-tightened-gates-note-parity` compares
  the note against the declaration surface, never against the tree, so an omission passes now and
  passes again at composition. It surfaces as an adopter meeting a red the release note never
  named — the defect docs/install.md assigns to the release rather than to the adopter's work.
  Surfaced 2026-08-26 to the gap inbox by the close of the `platform-reach-and-target-roster`
  iteration, while re-probing that close's release disposition; promoted 2026-08-26 at scope.

- **shellcheck-analyser-version-unpinned-in-ci** [design-pending] [cost: event/high] [surface: .github] — one battery member's verdict is
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
  **Why `[design-pending]`:** three uncosted shapes — pin the analyser version in the workflow and
  state it where the gate's contract is specified; have the gate REPORT the version it ran so two
  runs are comparable; or accept the float and say in the gate's SPEC section that this member's
  verdict is host-dependent.
  **Cost while deferred:** the pre-push battery's central promise — that a green local run predicts
  a green remote one — is false for one member, and the failure mode is a burned push.
  Filed 2026-08-27 by scope into this iteration's ledger, draining the gap inbox; attested
  2026-08-27 by the `windows-adopter-unblock` close's own verifying push.

- **prompt-ranking-ungrantable-shape-class** [design-pending] [cost: iteration/low] [surface: guard-kit] — the friction ranking's unit mixes
  rows an allowlist entry could retire with rows no entry can ever match, so each close re-triages a
  class no action retires.
  **The structural ground, and it is why this is not a tuning complaint** — the guard says so in its
  own refusal text: a `Bash(...)` entry matches a bare command, so any chaining, redirect or
  expansion breaks the match and the call is decided out of band whatever the allowlist holds.
  **All three of the triage's dispositions are wrong for these rows.** An allowlist entry cannot
  reach them; a guard steer already covers the read shapes while the write shapes are legitimate
  journal and scratch appends, which guard rule 17 now auto-allows outright; and a habit change
  leaves the row ranking identically. The honest disposition is a fourth the ranking does not
  offer: structurally ungrantable, retired from the actionable set.
  **DISTINCT from `friction-key-segment-selection-unruled`**, whose axis is *which* segment of a
  compound is keyed: it changes which name a row files under and leaves every row ranking. DISTINCT
  from `file-authoring-act-ungoverned`, which owns whether the authoring act is governed at all —
  its guard-steer shape retires these rows by changing behaviour, where this entry asks what the
  ranking should say while it has not.
  **The design question is ANSWERED by the fourth measurement below — partition the ranked output by
  SHAPE**, as it already partitions overlay-covered rows. Grading each row against the allowlist's
  own matching rule was more accurate and duplicated that rule; a guard-kit/SPEC.md note was
  cheapest and bought least. What stays open is only where the partition lands.
  **THE INSTRUMENT IS CONTAMINATED and every measurement here is a MULTI-WINDOW read until it is
  cleared — 2026-09-06, probed at scope.** `.workflow/prompt-friction.log` still carried a PRIOR
  iteration's close calls verbatim, so no emitted figure denominates one window. Clearing the log is
  a precondition of the next measurement.
  **Cost while deferred:** one re-triage of the same unretireable rows per close, on the surface a
  close reads to decide where friction is; the top row is always one of them.
  **FOURTH MEASUREMENT 2026-09-05, and it ANSWERS the design question.** 126 calls, 27 patterns, 498
  fall-throughs; head `python3 -` heredocs at 61x, `cat >>` second at 24x — down from 50x, so the
  earlier GROWING read tracked the command word, not the class. A THIRD word takes the head across
  four samples while the SHAPE never moves: partitioning by shape is what the samples buy.
  recurrence: prompt-ranking-ungrantable-shape-class 2026-08-28 2026-09-03 2026-09-08
  **THE ROUTING AXIS IS ANSWERED — operator direction, 2026-09-09, lead-relayed: a newly judged
  recurrence does NOT restart the port-first run's proposed-once clause**, since reading a fresh
  date as a reset defeats the clause that stops a member cycling; the threshold history before
  it (2026-09-03 to 2026-09-05) is in git. **Declined again at scope — operator direction,
  2026-09-11 (lead-relayed):** that direction stands, and the friction ranking shares no surface
  with this iteration.
  **THE `cat >> .tmp/*` GRANT WAS GRANTED THE SAME DAY AND IS NOT LANDED HERE:** it edits
  `.claude/settings.json`, and a stage session may not touch a permission surface on a relayed
  authorization, so it was routed back. The limit it was granted on, recorded here because JSON
  carries no comment: the glob is a PREFIX match, reaching any path under `.tmp/`.
  **THE SECOND AND THIRD MEASUREMENTS (2026-08-28, 2026-09-03) ARE ANSWERED by the fourth**, each a
  gap bullet re-deriving this diagnosis without finding this entry. One live residue survives them:
  the second **falsified a tempting sub-case** — ten `GATE_SDK_VERBOSE=1` calls read as a
  missing-`env`-word steer were all piped into `grep`, so the prefix changes nothing. Filed
  2026-08-27 to the gap inbox by the windows-artifact-proof close; promoted 2026-08-27 by the next
  scope. Also ruled on: lead 2026-09-04 (own-authority). Fifth and SIXTH re-derivations drained at
  scope 2026-09-06 and 2026-09-08, each discarded, the instrument half kept above.



- **readme-bin-roster-underived** [design-pending] [cost: event/low] [surface: gate-sdk] — every kit README's **gate** roster is held by a
  gate and its **`bin/` tool** roster is held by nothing, so a tool added, renamed or retired leaves
  a hand-authored list silently stale.
  **The scope limit is the gate's, stated and verified rather than inferred.** `check-readme-roster`
  asserts name-set parity in both directions over exactly two sets: the names inside a README's
  gate-roster marker block, and the kit's `checks/` basenames (gate-sdk/SPEC.md
  §check-readme-roster; the descriptor's `couples=` field and the compiled rule agree). A `bin/`
  tool sits outside both sets, so nothing reds.
  **The witness is dated now rather than live, and it still does its work.**
  `gate-sdk/bin/upgrade-smoke.sh` shipped in `gate-sdk/bin/` and appeared nowhere in that
  README's hand-authored `bin/` prose, from before the surfacing iteration until its 2026-09-02
  port. The roster was wrong across that whole span: the omission is a class, not a one-off.
  **This is Derivation-first in the shape the doctrine names:** a roster maintained rather than
  derived, with a freshness gate sitting one directory away that does not reach it.
  **Why `[design-pending]`, two dispositions trading different properties.** Make the `bin/` roster
  a **generated projection**, rostered in docs/site-architecture.md with a trigger and a regen
  command like every other — which buys derivation but puts a marker block and a freshness gate on
  a surface whose annotation prose is the part a reader wants. Or **widen `check-readme-roster`**
  to a second marker block over `bin/` basenames — cheaper, reuses the shape the gate already
  has, but asserts parity over a set whose membership rule ("a shipped tool") is less crisp than
  `checks/`'s, since `bin/` mixes adopter-facing entry points with internal helpers and the gate
  would have to rule which are roster-owed.
  **DISTINCT from `readme-roster-enum-coverage`**, whose subject is the gate's own enum coverage
  inside the block it already reads. This one is a corpus the gate never reaches at all.
  **Cost while deferred:** one stale roster line per tool retired or added, each unattributable by a
  later reader, on the surface an adopter reads first to learn what a kit ships. The tree carries
  none today — the 2026-09-02 port deleted the omitted tool, which fixes the instance and leaves
  the class exactly where it was.
  Relayed 2026-08-28 by the lead on the spec session's behalf (`file-gap.sh` contended on an
  uncommitted inbox); promoted 2026-08-28 by close at this boundary's drain, its scope claim
  re-verified against the descriptor and the compiled rule and its witness found here.

- **harness-project-dir-fold-dialect-unresolved** [design-pending] [cost: event/low] [surface: context-kit] — the harness project-dir
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

- **record-stamp-encoding-compression** [design-pending] [cost: event/low] [surface: queue-kit] — buy discrimination in the queue's
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
  (`queue-kit/lib/queue.sh:37,39`) bound columns and lines separately, so a shorter stamp frees
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
  **Why `[design-pending]`:** the ruling fixes the DIRECTION and not the grammar. Open: which
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

- **editor-diagnostic-unruled** [design-pending] [cost: event/high] [surface: doctrine-kit] — the harness LSP channel contradicts the tree's
  own oracles on crate-touching commits, and no surface rules that channel out as an oracle.
  **Re-tiered out of the icebox 2026-09-01 by close's gap drain, on a dated recurrence.** It was
  born in the icebox at `9845f2c3` this same iteration, on the stated basis that the channel had
  "contradicted the tree's own oracle TWICE ... one candidate shape is to do nothing, which is what
  makes this dormant". Two further instances have since fired, so the eviction's premise is spent
  and §The icebox tier's own round trip applies. **The ground is ELIGIBILITY, not grammar** — the
  stronger reading, added by the lead sustaining the move: queue-kit/SPEC.md:220-224 requires an
  iceboxed entry to have no live promotion trigger and names "a dated `recurrence:` line" as one, so
  stamping the judged recurrence makes the entry ineligible for the tier by its own eligibility
  rule. The move out is FORCED, which is exactly why the drain is not promoting anything. The
  grammatical reading — assertion (B) barring a body line from a one-line entry — is a second,
  weaker fence around the same conclusion. Not a reversal of the evicting session's judgment.
  **The four instances, all 2026-09-01.** (1) and (2) are the icebox basis. (3) At `64cce1d9` the
  channel reported nine rustc errors — E0603 private-import on `emit/mod.rs`'s `targets`,
  `corpus`, `relative` and `read_text` from `md_index.rs` and `pub_index.rs`, and E0277 missing
  `Debug` on `pub_index::Extractor` — plus dead_code on `walk.rs`'s new `Links` variant and
  `find_link_entries_with_prune`. (4) At `65e22a28`, the same shape. Both times the lead ran the
  oracles rather than reasoning about them: `run-gates.sh` read 108/108 including
  `check-crate-arms`, and `build-native.sh` compiled the release profile clean.
  **What the frequency adds.** Instances 3 and 4 are the first two crate-module-adding commits of
  a new iteration, so the divergence is a standing property of such a commit rather than an
  incident. The exposure is the direction of trust: a session taking the editor channel as ground
  would "repair" correct code to silence a stale index, inverting oracle-first at a channel that
  rule does not name.
  **Cost while deferred:** low but no longer dormant — it blocks no stage entry and no push, and
  the whole cost lands on a session that trusts the wrong channel, which is unbounded when that
  happens and zero when it does not.
  **Why `[design-pending]`:** the candidate shapes are unchanged and one of them is to do nothing
  — name the channel in the oracle-first rule as a non-oracle, or leave it to session judgment —
  so picking one is a doctrine call rather than a defect repair.
  recurrence: editor-diagnostic-unruled 2026-09-01
  Filed 2026-09-01 at scope's boundary drain and born in the icebox; re-tiered the same day by
  close on instances 3 and 4, which postdate the eviction. Both fell on one calendar day, so the
  single-date spelling here rests on lifecycle-kit's `(slug, date)` idempotence and the ambiguity
  is escalated rather than closed by this session.

- **docs-cmd-invariant-inline-scope-imprecise** [design-pending] [cost: event/low] [surface: canon-kit] — `check-docs-cmd`'s invariant
  sentence promises that inline-backticked `.sh` paths are scanned while its own assertion (A)
  scans only fenced ones, so the sentence over-promises and the class it names stays ungated.
  **The imprecision, read at the source rather than argued.** `canon-kit/SPEC.md`'s invariant is
  ONE sentence introducing BOTH assertions, so its "or inline backticks" is satisfied by (B),
  which genuinely does cover inline — loose prose rather than a flat self-contradiction. (A)
  carries an explicit justification for its narrowness where the invariant sentence carries none,
  and `native/src/gates/docs_cmd.rs` matches each assertion precisely: the path scan runs only
  inside a fence, the knob scan runs in-fence and over inline code spans.
  **The two repairs are ASYMMETRIC and this entry starts from the narrow one**, ruled by the lead
  on all four sites read directly, which corrected an earlier framing of it as a symmetric
  coin-flip. (b) correcting the invariant sentence to say paths are fence-only is the ACCURACY fix
  and the presumptive default. (a) widening (A) and the implementation to inline code spans is
  enforcement-first, but it must win a POSITIVE case against that calibration paragraph and it
  re-arms over the whole governed doc set at once.
  **The live class that prompted it is DISCHARGED and does not ride this entry.** The eight inline
  citations the two port cuts made dead were swept as a Definition-of-Done item inside the units;
  re-verified by grep at this drain, the sharpest of them — `canon-kit/SPEC.md`'s own invoked
  `bash scripts/enum-sets.sh` — is gone, and what survives in `gate-sdk/SPEC.md` is a dated
  history paragraph naming the deleted scripts deliberately. This is the gate-precision half alone.
  **THE POSITIVE CASE LIMB (a) WAS WAITING FOR ARRIVED 2026-09-05, and it does not settle the
  fork.** Two independent close audits (`internal-identifier-restatement`,
  `capability-liveness-after-descope`) each found the SAME five present-tense inline-backticked
  `.sh` paths in published kit SPECs, naming three files that iteration deleted; a third sweep's
  corpus-wide existence probe returned 38 non-resolvers over 68 literals. Every one is inline, so
  assertion (A) saw none of them and `check-docs-cmd` stayed green throughout. That is
  enforcement-first's case measured rather than argued. What keeps the fork open is the SAME
  probe's other column: its non-resolvers are dominated by legitimate past-tense port narrative,
  dated release posts and explicit placeholders — exactly the false-positive surface (b) prices.
  **Why `[design-pending]`:** choosing between (a) and (b) is a doctrine call between
  enforcement-first and a stated calibration, not a coding one.
  **Cost while deferred:** an invoked `.sh` path in inline backticks stales silently across the
  governed doc set while a reader of the invariant sentence is told otherwise — an adopter reading
  a kit SPEC to learn what a gate checks witnesses that, so it is product rather than machinery.
  **DISTINCT from `prose-filename-citation-liveness`**, which owns bare `.md` filenames falling
  between `check-md-refs` and `check-spec-pointer`; this is `.sh` paths and the precision of one
  gate's own invariant sentence.
  Filed at spec 2026-09-03 and reframed the same day on the lead's four-site read; drained here.

- **registry-needs-conflates-requirement-and-spawn** [design-pending] [cost: event/high] [surface: gate-sdk] — the crate's registry
  declares a member's HOST REQUIREMENT while the test guarding it compares that declaration
  against the literal program string a spawn used, so a member that resolves its interpreter is
  undeclarable by construction.
  **THE SYMPTOM THAT MADE IT NON-LATENT IS GONE, and only the symptom.** It briefly stopped
  being latent when `graph.rs` spawned `proc::resolve_interpreter("bash")`'s resolved absolute
  path against a bare `bash` declaration that `declaration_covers` matches by EXACT equality.
  `host-resolution-fail-open-cut` repaired that 2026-09-11: `native/src/gates/graph.rs` and
  `native/src/installer/init.rs` no longer resolve at the call site, so the recorder observes
  the bare literal `native/src/gates/mod.rs` declares and the guarding assertion matches again.
  **THIS ENTRY IS NOT THEREBY CLOSED**, and the note exists so a later drain does not read a
  vanished symptom as a vanished subject: the repair moved one member back into agreement and
  changed nothing about the grammar, so the next member that resolves an interpreter is
  undeclarable on exactly the same construction.
  **The assertion that would have said so cannot reach the spawn.** Both `check-graph` fixtures
  pass `--amend-only`, which returns before the generator arm, so the recorder observes nothing
  and `every_registry_member_declares_the_programs_it_spawns` passes VACUOUSLY on the one member
  the hotfix changed. The fixture's own comment states why the alternative is hard: the whole-tree
  generator run anchors to the real repo root and is unfixturable. The vacuity is UNTOUCHED by
  the 2026-09-11 repair above — the assertion still cannot reach that spawn, so it would not
  have caught the disagreement and will not catch the next one.
  **Two halves, and only the first is cheap.** The grammar half — teaching the comparison that a
  resolved path satisfies a declared program name — is a small change to a crate-wide test made on
  behalf of every registry member. The vacuity half needs a fixture that reaches the generator arm
  and may not be buildable at all.
  **Why `[design-pending]`:** whether the declaration is a REQUIREMENT (a resolved path matches by
  its name) or a LITERAL argv[0] (then a resolving member can declare nothing host-independent) is
  a contract question gate-sdk/SPEC.md §The `# graph:` manifest owns, and the two answers differ
  in what `--needs` promises a consumer's machine must carry.
  **Cost while deferred:** `--needs` is the roster an adopter provisions from, so a member whose
  declaration silently stops matching its spawn under-reports it; that payload-facing claim is the
  witness, which puts this on the product side of the 2026-08-30 discriminator.
  **DISTINCT from `crate-interpreter-resolution-residue`, retired**, whose deliverable is the
  spawn sites themselves; this is the declaration grammar every registry member shares.
  Filed at spec 2026-09-03; drained here with the disagreement re-read at the source.

- **kit-spec-provenance-seam-sweep-remainder** [design-pending] [cost: iteration/high] [surface: canon-kit] — FIVE kit SPECs still carry this
  project's ruling provenance, plus the seam gate that goes green only once the last is swept.
  gate-sdk landed with the parent split; context-kit and guard-kit landed as the first slice, 14
  sites; lifecycle, delegation and queue landed 2026-09-06 as the second, ~38 sites. **STILL
  OWING: canon, doctrine, drift, site, evidence.**
  **IT BELONGS IN `Deferred`, NEVER `Done`, AND IT DEMOTES AT BUILD, NEVER AT CLOSE** (`lead,
  own-authority`, 2026-09-06): the deliverable is a corpus, so the next cut re-promotes with its
  own amendment (canon-kit/SPEC.md §Merging an amendment), and `check-stage-entry` assertion B
  refuses a validate entry on a non-empty active queue. TRAJECTORY.md §The closed rulings'
  `kit-spec-provenance-seam` grants scope the by-kit split, so a slice needs no waiver.
  **OWED ON A RE-GROUNDED BASIS, 2026-09-06 — operator, direct answer to the lead session's ask,
  lead-relayed:** the 2026-09-03 standing direction is SPENT and the operator chose RE-GROUNDING
  over renewing it, so what carries this now is product-class with a live trigger on its own
  accretion.
  **DO NOT RE-AUTHOR THE PREDICATE OR RE-RUN THE SCAN — RECOVER IT** from the parent half's merged
  amendment, `git show d190c2f6^:gate-sdk/SPEC-seam-sweep.md`: delta 1 is the discriminator (the
  seam decides the VOICE, never the content), delta 2 the taxonomy, delta 3 the move-versus-delete
  test. Delta 1's COROLLARY is the sizing lever — the test is the IDENTIFIER, never the tense — and
  delta 2 excludes a dated MEASUREMENT, whose date is its freeze and must survive. That is also why
  a date scan MISSES: the swept mass is event deixis — queue slugs, cut ordinals, port chronology.
  **FOUR MORE RULES, recovered the same way from the second slice's `SPEC-seam-slice.md`:** a dated
  attestation used as a rule's own SPECIMEN is not a stamp and stays; a LIVE procedural status is
  swept and routes to the gap inbox, never migrated into the ruling record; where an attribution
  abuts UNDATED numeric evidence the cut lands BETWEEN them; and a passage's self-declared sole-home
  claim is evidence about the MECHANISM, never about the stamp.
  **THE RANK ORDER ORDERS AND NEVER COUNTS** — lifecycle, delegation, queue, canon, doctrine,
  drift, site, evidence. It failed quantitatively at ranks 3 and 6 over the first slice and held
  monotonically across the second's three (20 > 11 > 7), adjudicating nothing about the five left.
  **MOVE-VERSUS-DELETE IS ANSWERED THROUGH THE SECOND SLICE: NO SWEPT SITE WAS SOLE-HOME**, and
  nothing migrated to TRAJECTORY.md from it either. drift-kit is STILL LAST TO OWE IT — its half of
  the guard-kit §scratch-run spent pair, plus its spelling's fixture shape, a parenthetical date.
  **THE POINTER CLASS STAYS OPEN and has now shipped intact for THREE consecutive units** — ten,
  then nine, then two — of a class re-verified 2026-09-06 as still EXACT at 17 across five kit
  SPECs (gate-sdk 10, guard-kit 4, canon-kit/delegation-kit/lifecycle-kit 1 each; queue-kit zero),
  plus a `docs/`- and `BRIEF.local.md`-class the same question reaches. **RULED `lead,
  own-authority` 2026-09-06 for the second slice: they STAY STANDING.** Answerable for every kit
  SPEC at once or not at all, so **THE ANSWER IS OWED TO THE OPERATOR BEFORE THE SLICE THAT LANDS
  THE GATE** and not before any earlier one — it reaches the provenance seam, a privacy boundary
  before it is a design one. Right each time, and the count is the debt.
  **THE GATE IS UNBUILT and every landed sweep ships ungated meanwhile.** It must encode the
  sanctioned exemption for gate-sdk/SPEC.md §The decisions this substrate already closed (`lead,
  own-authority` 2026-09-05), which retires only if
  `provenance-ownership-recording-rule-assigns-into-kit-specs` lands first.
  **DISCHARGE:** TRAJECTORY.md's directive paragraph discharges on "that unit lands and the gate
  is green", so it survives until this entry lands WHOLE.
  **Cost while deferred:** five published kit SPECs and every adopter's vendored copy carry private
  ruling history as mechanism and pointers a consumer cannot follow; product-class and accreting.
  Filed 2026-09-05 at scope with the split that created it; promoted and demoted 2026-09-06;
  re-promoted at spec and demoted again at build 2026-09-06 for the three-kit slice.

- **provenance-ownership-recording-rule-assigns-into-kit-specs** [design-pending] [cost: iteration/high] [surface: TRAJECTORY.md]
  — TRAJECTORY.md's recording rule does not merely permit the provenance class the seam sweep
  removes; at least once it POSITIVELY ASSIGNS a closed ruling's ownership to a kit SPEC, which
  is the class's stated cause rather than one of its instances.
  **BOTH ANCHORS ARE RETIRED BY `ruling-record-shrink-to-bau`, 2026-09-09, and the entry is
  corrected rather than left pointing at deleted prose.** The attested instance was the ruling
  record's two pointer paragraphs — the Rust-versus-Go refusal and the bash-portability-floor
  costing, assigned to gate-sdk/SPEC.md §The decisions this substrate already closed "because
  the component that depends on them must be readable alone". That sweep deleted both, the home
  carrying them unchanged, so the assignment stands while the paragraph asserting it is gone.
  **The general rule it quoted went with them**: "a ruling whose mechanism already has a
  canonical home is registered here with a pointer to that home rather than restated" was the
  §Where the grounds live sentence, and the admission test that replaced it splits mechanism
  from provenance in terms — mechanism to the owning kit SPEC with its engineering grounds and
  undated, and date, authority, channel and refusals to the landing commit. What this entry
  asked for is therefore ANSWERED IN THE RULE and open only in the tree; whether that is enough
  is the doctrine call below, and the entry's disposition is next scope's.
  **Why `[design-pending]`:** the shape is a doctrine call with at least three candidates — the
  rule gains a clause splitting mechanism from provenance; or a kit SPEC may host a ruling's
  mechanism while TRAJECTORY.md keeps its stamp; or the readable-alone objection is answered
  some third way, since it is a real one and a blanket ban would leave a component's dependants
  unable to read it alone. All three touch what a kit SPEC may contain.
  **OPERATOR-CLASS WHEN WORKED, and that is why this is filed rather than started.** Reversing
  the 118-130 assignment reverses a recorded ruling; and the rule reaches the provenance seam
  itself, which is a privacy boundary before it is a design one. Filed under scope-gated intake;
  a session picking it up escalates before editing TRAJECTORY.md's rule.
  **Cost while deferred:** the class's cause stays live, so a swept kit SPEC re-accretes at the
  next cut that records a ruling with a canonical home — measured once already, two new sites in
  one iteration from cuts that were not editing provenance. It also holds one carve-out open:
  gate-sdk/SPEC.md §The decisions this substrate already closed cannot be swept while this
  stands, so the seam gate must ship an exemption for it (recorded on
  `kit-spec-provenance-seam-sweep-remainder`, which owns the gate).
  Filed 2026-09-05 by spec, on the lead's ruling (`lead, own-authority 2026-09-05`) that the
  carve-out stands and its real fix is the recording rule.

- **bin-tool-help-arm-absent-tree-wide** [design-pending] [cost: event/low] [surface: gate-sdk] — most shipped `bin/` tools answer
  `-h`/`--help` with something other than usage on stdout at exit 0; the count is derived below
  and deliberately not frozen in this sentence.
  **THE CENSUS IS DERIVABLE AND THIS IS ITS COMMAND**, stated on the entry because a count whose
  oracle is unstated is unmaintainable by anyone but its author. Run
  `git ls-files '*/bin/*.sh' | grep -v '/gate-tests/' | xargs grep -L -- '--help'`. Dropping the
  `grep -v` re-admits the gate fixtures the census excludes. The proxy is static and a **lower
  bound** — a file merely mentioning `--help` in a comment counts as having an arm — but it was
  checked exact at the 2026-09-05 reading: the only matching tools, `gate-sdk/bin/run-gates.sh`
  and `installer/bin/checkwright.sh`, each carry a real `-h | --help)` dispatch branch.
  **RE-DERIVED 2026-09-06 at close, and the drift is this entry's own lesson landing on itself:**
  the command returns **5 of 7**, in **three** kits (context-kit, doctrine-kit, gate-sdk). The
  corpus was already 9 at that iteration's start, so two tools had left before the port cuts took
  the other two, and guard-kit has now left the `bin/` corpus entirely as drift-kit did. The lead
  line's frozen count was falsified twice over and is de-literalized here. Run the command.
  **Every earlier figure on this entry is superseded by that command and none is restated**, which
  is the whole reason the command is here: a 2026-09-04 measurement stated no pattern and no later
  session could reproduce it: re-derive rather than compare a bare number.
  **Measured behaviourally, 2026-09-04, and two of the three probes have since left the corpus.**
  `scratch-run.sh --help` answered `scratch-run: no such script: --help` at exit 2;
  `compare-settings-allow.sh --help` prints usage on **stderr** at exit 2, the unrecognized-option
  refusal branch and not a help arm; `stage-economics.sh --help` **ignored the argument entirely
  and ran the full meter**. ALL THREE shell paths are now deleted (the first and third 2026-09-05,
  the middle 2026-09-06), so all three stay as attested shapes and none is a current instance.
  **Why they survived, and it is not "no gate reads the contract".** That is ruled and deliberate
  (gate-sdk/SPEC.md §The bin/-tool contract), and the ruling names its own substitute in the same
  breath — *"Each member's coverage follows it"*, behavioral coverage in `smoke/`, on the
  `enter-stage.sh --simulate` precedent. gate-sdk, lifecycle-kit and drift-kit smokes each carry it;
  **context-kit's and doctrine-kit's smokes carry none**, one `bin/` tool each, measured at the
  2026-09-06 close; guard-kit's carried none either and its `bin/` left with the cut. The gap
  is the kits that never took the ruled substitute, not a missing scanner.
  **Why `[design-pending]`: the contract's own scope is the design question.** §The bin/-tool
  contract states its three behaviors under a free-text-positional rule, yet
  `gate-sdk/bin/run-gates.sh --enter-stage`'s own note reads the HELP half as binding on a
  membership-validated tool too. Whether it binds on a tool taking **no** positionals — which is
  most of the census — is unstated, and the answer sets the corpus before any member is fixed.
  **Cost while deferred:** one wrong answer instead of usage per session that probes a tool for its
  modes, and the attested shapes are silently-wrong rather than merely unhelpful.
  not-icebox-eligible: bin-tool-help-arm-absent-tree-wide 2026-09-12 live per-session trigger.
  **The remainder is also owed to the port**, so a cut can apply the split per member — but only
  once the scope question is answered, since it decides which members owe an arm. `build-native.sh`,
  `gen-pre-commit.sh` and `run-consumer-smoke.sh` are declared `no-port` and will never ride a cut,
  so their arm has no cut to ride and needs its own. That split re-derives off the census command
  joined with `--emit port-blockers --tree`.
  Filed 2026-09-04 to the gap inbox at spec as a guard-kit-local two-tool finding; WIDENED at that
  drain from 2 tools to a tree-wide census and from "no gate" to "no smoke coverage", after →fix
  failed on the unsettled scope question and →icebox failed on the live per-session trigger.
  Census command and count landed 2026-09-05 by close, on the lead's ruling that a close moving
  the number without landing its measurement pattern reproduces the defect one iteration later;
  both re-derived 2026-09-05 by build when its own cuts moved the corpus.

- **audit-roster-row-carry-unruled** [design-pending] [cost: iteration/high] [surface: lifecycle-kit] — every close appends its sweep reading to
  the audit-roster row it swept and nothing ever compacts one, so the file has grown past the
  point where the close step that must read it can.
  **Why the read is degraded, not merely long.** The Read tool refuses the whole file at a 256KB
  cap, so the close skill's mandated **Audit-roster review** sub-step
  (`.claude/commands/close.md:36`) can only reach a row through hand-built substring probes — it
  reads the slug, `due:` and `last:` while structurally NOT reading the accumulated body those
  fields exist to be judged against.
  **The design question this waits on, and why no session may just pick one.** Three shapes are
  visible and none is costed: compact each row to its current standing reading and let git history
  hold the retired ones; split one row per file under a roster directory; or cap a row and force
  the appending close to compress. Which is right turns on whether a row's accumulated prior
  readings are load-bearing for a later judge or are history — and
  doctrine-kit/DOCTRINE.md's Enforcement-first cadence clause, which owns this roster, says
  nothing about the roster's own carry. Any of the three adds a name to a governed surface, so
  this is feature-class and owes an amendment.
  **A second fact about the same surface, folded in because a remedy for one touches the other:**
  the file is TRACKED, so the close-surface derivation's capture-tier arm never reaches it and it
  appears in no `--emit close-surfaces` row, declared or otherwise. That is the situation
  CLAUDE.md §Housekeeping handles for `.workflow/preflight-valve.txt` with an explicit
  `close-surface:` declaration and a stated reason; the audit roster has none. It is not UNREAD,
  so this is a declaration asymmetry between two same-class surfaces and not a missed read.
  **Cost while deferred:** recurring and accumulating — each close adds roughly 7KB and performs
  a degraded review, so the mandated step's evidentiary value falls every iteration while the
  roster keeps its authority. That live per-close trigger is why the icebox tier's
  "no named event waiting to promote it" test fails here, and the machinery-class default
  (TRAJECTORY.md, 2026-08-30) is defeated on the tier's own eligibility rule rather than ignored.
  **NOT ASSERTED:** nobody has measured how much of a row's body a later judge actually uses.
  **The measured series, each point taken at its own boundary and none forecast:** 270392 bytes /
  11 lines at this drain; **280394 / 11 on 2026-09-04**, plus 10002 over one close and ABOVE the
  roughly 7KB the cost line forecasts, so that forecast is understated rather than generous;
  **214955 / 12 on 2026-09-06**, 65KB BELOW it, so growth is not monotone once a compaction lands
  and the cost line prices appends rather than the file; **231607 / 11 on 2026-09-07**, one row's
  single line at 65680 characters, about 80k tokens to read whole.
  **The second point bears on WHICH shape is right, not merely on size.** Those 10KB were one
  close's appends to FIVE rows that came due at once, and they came due because that iteration
  deleted a shell file and recorded a ruling. So append size is driven by ITERATION SHAPE rather
  than by the row, and capping a row per close — the third shape — would truncate hardest in
  exactly the case the roster exists for.
  **The FIRST shape has been run by accident and it lost a mandatory field — 2026-09-05.** A close
  compressed the `internal-identifier-restatement` row from 72073 to 2588 characters and the
  header-mandated `due:` field went with the prose, unreadable until the same close restored it
  (the field sat 43311 characters into the row, at `b2cedcd3`). So "compact each row" is not safe
  unattended: a compression pass reads an accreted row as prose, and the grammar is graded by
  nothing.
  recurrence: audit-roster-row-carry-unruled 2026-09-07
  Surfaced 2026-09-04 in the gap inbox by the `usage-verdict-cut` close's own later steps and
  drained at the next iteration's scope entry, which is why its disposition is dated after it.
  RE-FILED 2026-09-07 as a fresh gap bullet by that iteration's close — same subject, no new axis —
  and drained here as a `recurrence:` stamp with the figure refreshed rather than as a second entry
  (`lead, own-authority` 2026-09-07, on this scope's escalation).
- **upgrade-smoke-producer-leaks-worktrees-on-signal** [design-pending] [cost: event/high] [surface: gate-sdk] — the upgrade-smoke arm
  removes its worktrees on its own exit paths and traps no signal, so a run killed from outside
  leaks every checkout it created.
  **The producer is in-crate and its cleanup is `Drop`.** `native/src/emit/upgrade_smoke.rs`
  declares `impl Drop for Scratch` (:65-78) over a `worktrees: Vec<String>`, and its own comment
  at :56-57 says the shell form's `trap` is what that `Drop` replaces. Rust runs no destructor on
  SIGTERM or SIGKILL, so the claimed equivalence holds on every ordinary exit and fails on exactly
  the case a trap exists for. `gate-sdk/lib/consumer-smoke.sh` carries no `trap` at all — only
  `mktemp -d` at :42.
  **Re-verified at the drain rather than taken on the filer's word**, which is what the filing
  bullet itself asked for: a grep for `trap` and `signal` over both surfaces establishes it, and
  the two orphans the filing close reaped were stranded by a self-imposed timeout SIGTERM.
  **Not free residue.** The `--enter-stage` arm's boundary refusal
  (lifecycle-kit/SPEC.md §bin/enter-stage.sh) REFUSES an iteration-boundary
  entry behind any linked worktree, writing nothing, while away from the boundary the same scan is a
  mid-iteration advisory — so one killed run converts into a blocked boundary for whoever arrives
  next, which is why the filing close met it only as a warning.
  **DISTINCT from every open worktree entry, checked rather than asserted.**
  `worktree-lock-pid-is-not-agent-liveness` (retired), `worktree-lock-start-time-guard-untaken` and
  `worktree-cleanliness-assertion-scopes-to-checkout` are all about the DETECTOR's judgment, and
  `upgrade-smoke-refuses-inside-a-worktree` is about running INSIDE one. This is the PRODUCER
  never cleaning up. It re-files none of them and adds no recurrence date to any.
  **Why `[design-pending]`, and it is why this is not fix-shaped.** `native/src/` carries no
  signal handling anywhere and `native/Cargo.toml:14-16` lists one dependency, `serde_json` — so a
  SIGTERM trap costs either a new dependency or raw unsafe `sigaction`, and the crate's dependency
  BAR is engineering judgment gate-sdk/SPEC.md owns, which removes the no-dependency prohibition
  without touching that bar. Two further undecided shapes: whether cleanup belongs to the producer
  at all rather than to a reaper the boundary check already implies, and whether the
  consumer-smoke library owes the same trap.
  **Deferred, not active, on scope's composition test** (re-grounded 2026-09-11): no iteration
  since its filing has shared its surface. The icebox tier cannot take it either, because it
  blocks an iteration-boundary entry and the machinery-class default is conjunctive.
  **Cost while deferred:** every externally killed validate spine converts one lost run into a
  refused iteration boundary for the next session, and the remedy is a two-`--force` reap that
  session has to be told about.
  Surfaced 2026-09-04 by the close of `wait-probe-cut-and-stage-journal-absence`; drained
  2026-09-04 at this iteration's scope entry, the boundary having carried it.

- **worktree-reap-unasserted-at-dispatching-turn-end** [design-pending] [cost: event/high] [surface: delegation-kit] — nothing asserts the
  worktree reap at the DISPATCHING session's own turn end, so an orphan minted mid-iteration
  survives every later session until an iteration boundary meets it.
  **This is shape three of the retired `worktree-lock-pid-is-not-agent-liveness`**, whose shapes
  one and two landed 2026-09-06 and whose slug is now out of the pool, so that pointer resolves
  to history rather than to live work; the substance is restated here rather than cited.
  **Ruled out of that unit's ENVELOPE, not refuted — operator, 2026-09-06, through an interactive
  prompt in the lead session, relayed by the lead.** The ruling scoped one iteration's work and
  said nothing about the shape's merit; the parent entry recorded it in exactly those terms,
  "deferred rather than refuted".
  **The record's own argument for it, and it is the evidence that ranked the shapes.** The fourth
  sighting, 2026-09-06: an orphan minted at the prior iteration's spec stage survived align, three
  build sessions, validate and close without any of six sessions noticing, and only the boundary
  refusal surfaced it. Those five later sessions had no reason to look, and the party who knew
  what the worktree was for was gone six sessions before the refusal fired. Reaping at the
  dispatching turn end is the only shape that puts the judgment where the knowledge is.
  **DISTINCT from what landed and from what already exists.** Shape one gave the boundary
  refusal's LIVE branch a loss-gated reap remedy, which reaches a session that ALREADY MET the
  refusal. Shape two told delegation-kit's isolation bullet that a lock reason's pid is the
  harness's and not an agent liveness signal. Neither reaches an orphan nobody has met yet, and
  delegation-kit's reap-both-halves rule says WHAT to delete while staying silent on WHEN.
  **Candidate shapes, none costed:** assert the reap in the dispatching session's turn-end
  obligations beside the existing liveness-record rule; or have the isolation arm register the
  worktree so a later sweep reaps it without the dispatcher; or leave it to the boundary and
  accept the latency, which is today's behaviour.
  **Cost while deferred:** an orphan minted at any stage rides to the next iteration boundary and
  converts there into a refused entry for a session with no context on it, which is the one moment
  the loss question is hardest to answer; measured at six sessions of carry on the sighting above.
  Filed 2026-09-06 by a build session to the gap inbox so the ranked evidence would survive the
  parent's move out of the pool, explicitly as a record rather than a re-opening, and called
  icebox-class by default there. Promoted to Deferred instead at the 2026-09-06 close —
  `lead, own-authority`, 2026-09-06, relayed to the close session in its dispatch — on the ground
  that the parent had already recorded the shape as deferred rather than refuted, which is a live
  trigger the icebox tier's no-live-trigger predicate refuses.

- **ere-matcher-capture-groups-unowned** [design-pending] [cost: iteration/low] [surface: gate-sdk] — the crate's POSIX ERE matcher reports
  spans and cannot report a capture group, so the first consumer needing one shells out to bash.
  **The sizing this falsifies, and the axis it falsifies it on.** gate-sdk/SPEC.md §The POSIX ERE
  matcher rules the owed engine "a POSIX ERE matcher with leftmost-longest span reporting, and no
  substitution engine or capture-group replacement", argued from the nine cohort-held members —
  eight apply their pattern as a match test and one extracts a span. That argument is about GATE
  members and holds for them. The `--enter-stage` arm is a non-gate arm, and the worktree-lock pid
  pattern it reads is consumer config whose whole classification turns on the captured pid, so the
  API foreclosure below it ("no future consumer can turn a match test into a substitution") was
  written against a corpus that did not yet contain one.
  **What shipped instead, ruled 2026-09-04 by the lead at the cut.** `capture_group_one` in
  `native/src/emit/enter_stage.rs` runs the match through `bash -c` and reads `BASH_REMATCH[1]`.
  Parity by construction, bash's `[[ =~ ]]` being the semantics the shell original had; the residue
  is TWO ERE interpreters live on the stage-entry path, which can disagree on a pattern neither this
  repo nor an adopter is barred from writing. Asserted at the cut, enforced by nothing afterwards.
  **Verified at the close drain rather than taken from the filing bullet.** `native/src/ere.rs`
  `enum Inst` is Byte/Split/Jmp/Bol/Eol/Match — there is no `Save`, so the gap is structural rather
  than an unexposed API, and twenty crate modules reference `ere::`.
  **COST OF THE FIX, which is why this is a unit and not a line item in a port cut.** A Pike-VM
  upgrade: an `Inst::Save`, per-thread slot vectors, and a STATED POSIX leftmost-longest
  subexpression rule — the sub-match rule is the half POSIX specifies and ordinary leftmost-first
  engines get wrong — against a governed matcher carrying its own acceptance oracle.
  **Deferred on scope's composition test** (re-grounded 2026-09-11): no iteration since its filing
  has shared its surface. The icebox tier cannot take it either, the trigger being live — every
  stage entry runs the path.
  **Cost while deferred:** one bash spawn per iteration-boundary worktree row, and a divergence
  invisible once green, since nothing compares the two interpreters.
  Filed 2026-09-04 to the gap inbox by build; drained 2026-09-04 at this iteration's close.

- **release-note-removal-declaration-uncoupled** [design-pending] [cost: event/high] [surface: gate-sdk] — no oracle couples a removed
  adopter-facing entry point to its release-note declaration, so a note that omits one passes
  green and the adopter meets the omission as a broken invocation at upgrade time.
  **The instance is measured, not predicted.** The `evidence-runner-trend-and-install-hooks-cuts`
  iteration deleted four documented kit `bin/` tools — `evidence-kit/bin/run-validate.sh`,
  `evidence-kit/bin/diff-baseline.sh`, `delegation-kit/bin/usage-trend.sh`,
  `gate-sdk/bin/install-hooks.sh` — and re-spelled each as a bridged arm.
  **Why the version contract does not catch it.** Under docs/install.md §Versioning that is a
  MINOR under the pre-1.0 qualifier and not a major: decommission is scoped there to removing a
  DEPRECATED surface over the `CANON_KIT_DEPRECATION_MARKERS` roster, and none of the four carried
  a marker. So the whole protection an adopter gets is that the note declares the move and phase B
  of the upgrade contract reconciles it — and nothing checks that the note actually declares it.
  **Probed rather than assumed.** `check-release-bump`'s subject is note ordering and section
  presence: it reds a patch-only bump whose note declares behavior changes, and never reads the
  DIFF. `check-docs-cmd`'s invoked-path scan runs only inside a fence, so it cannot see a removal
  at all.
  **The candidate oracle is cheap and derivable:** diff the tracked `*/bin/*.sh` set between the
  newest released tag and the release commit, and require every disappeared path to appear in the
  pending note's declaration-bearing sections.
  **DISTINCT from the printed-follow-up unit LANDED 2026-09-07** (installer/README.md §The consumer
  smoke): that owned a printed command inside installer shell source with no gate corpus reaching
  it, and this owns the coupling between a REMOVAL and a NOTE — another surface, another oracle.
  **Product-class under TRAJECTORY.md's 2026-08-30 witness discriminator**, which is what keeps it
  off the one-line icebox tier: it blocks no stage entry and no push, but its demand witness is an
  adopter upgrade rather than this repo's own accounting.
  **Cost while deferred:** each release that decommissions an entry point re-buys a hand review
  nobody is obliged to run, and the failure surfaces in an adopter's tree rather than in ours.
  Filed 2026-09-05 to the gap inbox by the close of `evidence-runner-trend-and-install-hooks-cuts`;
  promoted here at this iteration's scope, →fix refused (a new gate is build work, not scope's) and
  →icebox refused on the product witness above. Deferred and not active on scope's composition
  test, re-grounded 2026-09-11: no iteration since has shared its surface.

- **fail-open-arm-status-second-source** [design-pending] [cost: event/high] [surface: gate-sdk] — the stub holds the bridged arms'
  unavailable exit status as a two-name shell test, and nothing holds that set in lockstep with
  the status each arm's own contract prose states.
  **Re-verified at the drain rather than taken from the bullets.** `gate-sdk/bin/run-gates.sh` is
  70 lines after the cut; line 20 sets `ARM_UNAVAILABLE_STATUS=2` and line 42 flips it to `0` for
  `--hook | --statusline` alone. The shell must keep holding it: the value is read exactly when
  the binary is absent and cannot be asked, so this duplication cannot be removed the way the
  dispatch loop's was — only asserted.
  **Two bullets, one gap.** This drain merged the spec filing, which names the crate's per-arm
  contract prose as the second source, with batch 3's, which names each arm's own SPEC section.
  They are two spellings of one second source and a single assertion closes both.
  **Why →fix failed at the drain:** the closure is a parity assertion over the fail-open set, and
  a new gate here is born native — a Rust module, a `.gate` descriptor and a `good/`+`bad/`
  fixture pair — which is build work, not a close-session edit.
  **Why →icebox failed:** the trigger is live. `--hook` and `--statusline` are the whole fail-open
  set today and the port run keeps landing non-gate arms (gate-sdk/SPEC.md §The non-gate arm), so
  the first harness-integration arm added without the two-name test wedges a binary-less adopter
  at the exact point a hook grades a user action.
  **Product-class on TRAJECTORY.md's 2026-08-30 witness discriminator**, which is what keeps this
  off the one-line tier: `run-gates.sh` is the adopter entry point, so the demand witness is a
  vendored tree with no binary rather than this repo's own accounting.
  **Deliverable:** an assertion that the shell's fail-open name set equals the set of arms whose
  owning SPEC section declares status 0, including which of the two surfaces is authoritative when
  they disagree.
  **Cost while deferred:** the set is two names and correct today, so the whole cost is future — a
  divergence lands silently and is found by an adopter rather than by the battery.
  Filed 2026-09-05 to the gap inbox at spec and again at build's batch 3; promoted at this close.
  Deferred and not active on scope's composition test, re-grounded 2026-09-11: no iteration
  since has shared its surface.

- **manifest-family-couples-misses-the-consumer-widened-corpus** [design-pending] [cost: session/high] [surface: canon-kit] — eleven gates
  read a consumer-configured doc corpus and none of them triggers on the members the consumer
  added, so `couples=` under-covers its own runtime reads by construction rather than by omission.
  **Measured 2026-09-06 with the selection oracle, and it is a CLASS rather than one file.**
  `run-gates.sh --for TRAJECTORY.md` selects EIGHT gates, of which only `check-measured-claim` and
  `check-unmarked-claim` read the file; `--for RELEASING.md` and `--for CONTRIBUTING.md` select the
  identical eight. Meanwhile eleven canon-kit descriptors declare
  `couples=*SPEC*.md,*README.md,CLAUDE.md` and scan the whole of
  `scripts/canon-config.sh`'s `CANON_KIT_MANIFEST_FILES`, which this consumer widened to add
  `TRAJECTORY.md`, `RELEASING.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`,
  `ROADMAP.md`, `doctrine-kit/DOCTRINE.md` and the `docs/` pages. Every one of those is scanned by
  the citation, temporal, link and count gates and triggers none of them. That is a direct breach of
  gate-sdk/SPEC.md §Porting a gate to the binary substrate's stated authoring rule — `couples=` must
  cover every path the gate reads at runtime, never a subset — and `check-reads-couples` cannot see
  it, because the corpus arrives through a consumer array rather than a statically resolvable walk.
  **Why `[design-pending]`, and why the drain's fix disposition FAILED.** The obvious repair —
  write the consumer's file names into the eleven kit descriptors — puts consumer content in a kit
  literal, which the provenance seam refuses (CLAUDE.md §The provenance seam). The kit's default
  triple is CORRECT for the kit's default corpus; what is missing is a lever by which a consumer
  that widens `CANON_KIT_MANIFEST_FILES` also widens the trigger. Three shapes are live and
  unranked: a consumer-side trigger-extension knob the hook emitter reads, a `couples=` token that
  resolves a knob the way `kit:<glob>` resolves the kit set, or per-consumer descriptor shadowing.
  Picking one at a drain would settle a kit/consumer seam question by accident.
  **Why NOT icebox:** the trigger is live and this iteration made it materially more expensive —
  close now holds ruling-record repair authority, so `TRAJECTORY.md` became a surface close writes
  routinely, and a routine writer of a file no doc gate fires on is the worst case for the class.
  **Cost while deferred:** every commit touching one of those members runs the two whole-corpus
  claim gates and skips the four that grade its citations, links, temporal narration and counts —
  silent, and paid at the moment the surface is least reviewed.
  Filed 2026-09-06 by close from the gap inbox, after re-verification widened the bullet: the bullet
  said four gates trigger and the fix was three `couples=` triples; the oracle reads eight
  triggering and the fix is not three triples at all.

- **dod-parks-a-queue-transition-at-a-stage-that-cannot-perform-it** [design-pending] [cost: iteration/high] [surface: lifecycle-kit] — an
  amendment's definition of done names a stage for the entry's queue transition without reading the
  configured drain stage, and nothing refuses the DoD until a session tries to execute it.
  **Measured 2026-09-05 across spec and build.** spec authored a DoD parking
  `platform-support-ci-matrix`'s queue transition at CLOSE; the lead confirmed it. Both missed that
  `lifecycle-kit/lib/stages.sh:29` sets `LIFECYCLE_KIT_DRAIN_STAGE=validate` in this tree, so the
  DoD was not merely late but UNEXECUTABLE: the pre-dispatch `--enter-stage validate --simulate`
  refused, and build had to demote the entry on a resume after its push was already spent.
  **The cost is measured, not estimated:** one resume round trip, and a demotion taken under time
  pressure rather than as the planned exit.
  **NEW, not a recurrence, and both nearby candidates were read before saying so.**
  `amendment-dod-sibling-dependence` is a DoD item whose satisfiability depends on SIBLING UNITS the
  amendment never names; there is no sibling here, and the dependence is on a config value.
  `observation-predicate-entry-cannot-drain-in-its-own-iteration`, retired, is about WHETHER
  the observation arrives before the drain; this is about the DoD naming the wrong stage for
  the drain at all, and
  it would have fired on an entry with no observation predicate.
  **Why `[design-pending]`:** the candidates differ in kind — an authoring clause in the spec stage
  template (a DoD's queue transition names the configured drain stage, never a literal), or an
  assertion over the amendment glob reading a DoD's stage word against `LIFECYCLE_KIT_DRAIN_STAGE`.
  The second is mechanical and the first is cheaper; which is right depends on whether a DoD may
  legitimately name a non-drain stage, and that is unruled.
  **Cost while deferred:** every iteration whose unit has a queue transition in its DoD can buy the
  same resume, and the defect surfaces only at the dispatch that the wrong stage refuses.
  Filed 2026-09-05 by close, routed by the lead as a finding neither spec nor build had filed.

- **kit-spec-seam-content-half-unswept** [design-pending] [cost: event/high] [surface: gate-sdk] — the provenance seam has two halves and
  the sweep that ran carried a discriminator for only one, so gate-sdk/SPEC.md is swept of private
  VOICE and unswept of private CONTENT.
  **The discriminator that ran was ATTRIBUTION** — an authority, a date, a channel, an internal
  identifier — which by construction cannot see a block that attributes nothing. CLAUDE.md §The
  provenance seam bars "private rule content" as well: term lists, coupling vocabularies, product
  constant sets, and a consumer's configuration where a kit literal should be optional config.
  **TWO INSTANCES, both re-verified live at HEAD by this drain rather than carried from the
  filing.** (1) gate-sdk/SPEC.md:2058-2078 documents THIS REPO'S QUEUE PRACTICE as gate-sdk
  mechanism — which entry a cut rides, the scoping stage's promote and the entry's own build-stage
  demotion, lead-line amendment-tag arithmetic against `check-queue-wrap`'s column budget, and a
  `git log -S` re-derivation — and survived the sweep fully de-attributed. It is a content-tier
  fault independently of the seam: queue practice is queue-kit's subject, not gate-sdk's. (2)
  gate-sdk/SPEC.md:5398-5401 enumerates six literal `couples=` values of this consumer
  (`scripts/git-hooks/*`, `.claude/agents/*.md`, two delegation-kit files, `CLAUDE.md` plus
  `doctrine-kit/DOCTRINE.md`, `.workflow/*,.gitignore`) inside a kit SPEC.
  **Why `[design-pending]` rather than a sweep:** each instance has a real defence and they differ.
  The queue-practice block exists because only half of it was derivable, so deleting it re-buys a
  `git log -S` at every cut; the `couples=` list is a RECORD of one past batch's derivation, so
  de-literalizing it destroys the thing it is for. Whether either is "private rule content" at all
  is the seam question, and answering it SETS an envelope rather than applying one.
  **DISTINCT from `kit-spec-provenance-seam-sweep-remainder`**, whose corpus is the ten OTHER kit
  SPECs under the attribution discriminator; this is the already-swept SPEC under the other
  discriminator, and the two overlap in neither corpus nor test.
  **Cost while deferred:** the seam is a privacy boundary before it is a design one and this repo
  is public — a kit literal carrying a consumer's configuration publishes it, and every adopter
  vendors the copy. Product-class.
  Surfaced 2026-09-05 by build batch A's provenance census, which flagged both and deliberately
  edited neither; drained here with both instances re-verified live.

- **directive-minting-delta-roster-obligation** [design-pending] [cost: event/low] [surface: canon-kit] — a delta that mints a machine
  comment directive obliges canon-kit's built-in directive roster, and nothing in the amendment
  lifecycle prompts the author to list that surface.
  **The instance is measured.** The portability-floor amendment's `## Existing sections updated`
  roster missed the `native/src/gates/comment_tier.rs` + canon-kit/SPEC.md §check-comment-tier row
  through spec AND align, and align had already corrected two other completeness gaps in the same
  roster. Build closed it in-session only because delta 3's own grammar forced it: two of the six
  valve sites sit on line-continuation statements that carry no trailing comment, so the marker had
  to be a full-line one, which `check-comment-tier` governs.
  **Why the gate cannot see it.** canon-kit/SPEC.md §check-amendment-update-target asserts the
  decidable half by design — every LISTED target is owned — and says so in as many words, leaving
  roster completeness to align. Arm B catches a target listed and unowned, never one never listed.
  The amendment is deleted on merge, so no later reader catches the omission either.
  **Why this is a THIRD narrow slice and not a re-file.** `amendment-roster-omission-detection`
  (retired) owned the general class and named two candidate gateable slices, literal-substitution
  and renumber, with "whether one mechanism covers both" as its open question. This is a slice
  neither reaches and the only one whose evidence is IN the amendment: a delta body minting a
  `<word>:` token absent from the directive rosters is decidable without reading the tree. That
  entry was at its 50-line cap when it retired, which is why the slice landed here rather than
  inside it; its recurrence date carries the instance.
  **Product-class under TRAJECTORY.md's 2026-08-30 witness discriminator** — canon-kit ships the
  amendment lifecycle and `check-comment-tier`, so an adopter minting a directive in their own
  amendment meets the same silent omission and their own gate's verdict witnesses it.
  **Cost while deferred:** one roster row per directive-minting delta, caught by a later grep or not
  at all; directives are rare, so the class is low-frequency and silent rather than loud.
  Filed 2026-09-07 by build batch B to the gap inbox; promoted here at this iteration's close drain,
  →fix refused (a prose nudge with no gate is the move §check-amendment-update-target already
  refused) and →icebox refused on the live trigger and the product witness above.

- **portability-floor-adopter-on-ramp-unstated** [design-pending] [cost: event/low] [surface: gate-sdk] — a vendored consumer gets
  `check-portability-floor` registered and permanently disabled, and nothing tells them that naming
  their install path is what turns it on.
  **Probed, not inferred; RE-PROBED against the port 2026-09-09 and unchanged.**
  `native/src/installer/recipe.rs:159-167` seeds `templates/msg-patterns.list` into an adopter's
  gates dir and seeds no portability roster; a grep for the roster name across the five ported
  verbs returns nothing, so `doctor` still reports nothing about the disabled state either.
  `GATE_SDK_PORTABILITY_PATHS` defaults empty, so the gate's clean line reports the
  disabled-and-unconfigured state on every run and nobody reads it as a cue.
  **The not-seeding is ruled and is NOT the gap.** Build ruled it deliberate — a roster with no
  corpus scans nothing, and the kit cannot know an adopter's install path — and gate-sdk/SPEC.md
  §check-portability-floor states the degradation with its honest limit. What is unclosed is the
  on-ramp: neither `init`, `doctor` nor the docs name the knob as the thing to set.
  **Three candidate answers, none costed here:** seed the roster with an empty corpus; have `doctor`
  report the disabled-and-unconfigured state; or a paragraph under docs/install.md §Requirements.
  The second is the only one that reaches an adopter who never opens the SPEC.
  **Product-class under TRAJECTORY.md's 2026-08-30 witness discriminator** — the install path is a
  named witness, and the whole subject is what an adopter's tree does after `init`.
  **Cost while deferred:** every adopter who vendors gate-sdk carries a registered gate asserting
  nothing, and the failure mode is silence rather than a red.
  Filed 2026-09-07 by build batch B; promoted here at this iteration's close drain, with →fix
  refused (choosing among the three candidates is design work an amendment owes) and →icebox
  refused on the adopter witness above.

- **substrate-parity-digest-assertion-stops-at-the-workflow-text** [design-pending] [cost: event/high] [surface: gate-sdk] — assertion F
  reads the publish workflow's own text for the digest producer, and the producer moved out into a
  called script.
  **Re-probed at this drain, and it fell harder than it was filed.** `grep -n sha256sum
  .github/workflows/publish.yml` returns one hit, `:209`, and that is the release job's **tarball**
  digest rather than the gate binary's; `grep -n sha256sum scripts/ci-build-artifact.sh` returns
  `:44-45`. So the build job's per-job `computes_digest` count
  (`native/src/gates/gate_substrate_parity.rs:285-299`, read at `:708-715`) is now **0** and not 1 —
  which means the case the assertion used to catch, a second emission added beside the first at
  1→2, is now 0→1 and passes clean. The filed bullet said the case was no longer caught; the count
  is why.
  **The honest half landed at this close and is NOT what stays deferred.** gate-sdk/SPEC.md
  §Consumer payload claimed the rule was "held mechanically by §check-gate-substrate-parity
  assertion F rather than by review" and stopped there; that clause now states the assertion's
  reach and what factoring a build body out into a called script does to it. What stays open is the
  ENFORCEMENT: widen assertion F to follow a `run:` line's called script, so a shared body sits
  inside the corpus that holds one-producer-per-digest.
  **Why the landed coverage limit is not the fix.** A stated limit on a gate whose whole subject is
  a supply-chain invariant puts that invariant back on review, which is the thing the section's own
  sentence says it is not on.
  **Cost while deferred:** the one-producer rule holds by construction — one shared body, one
  emission — and by nothing else, so a workflow step re-adding an emission beside the call, or a
  second `sha256sum` inside the script, ships green.
  Filed 2026-09-08 by build to the gap inbox; promoted here at this iteration's close drain, →fix
  taking only the overclaim clause (the widening is a Rust change owing a `good/`+`bad/` fixture
  pair, which a close cannot land test-and-doc-complete) and →icebox refused on the live trigger.

- **macos-adopter-package-set-copied-per-leg** [design-pending] [cost: event/high] [surface: .github] — the adopter-claim brew set now
  exists twice in gates.yml and nothing holds either copy equal to the page it claims to mirror.
  **Re-probed at this drain, and the census is three-way rather than the filed two-way.**
  `grep -n 'brew install' .github/workflows/*.yml scripts/ci-macos-floor.sh`: gates.yml `:899` and
  `:1166` both run `brew install bash coreutils gawk shellcheck` — the two install-smoke legs'
  adopter claim — while `scripts/ci-macos-floor.sh:16` runs `brew install bash coreutils gawk`, a
  DIFFERENT set with no shellcheck, which is the build legs' runner floor and not an adopter claim
  at all. So a reader comparing copies must first know which two of the three are meant to agree,
  and no surface says.
  **The step's own header rules the set an adopter claim held equal to docs/install.md §Requirements
  IN BOTH DIRECTIONS**, so the drift that matters is the silent one: a leg quietly gaining or losing
  a formula goes green over an adopter path that stayed broken.
  **Not a delta-5 omission.** The amendment's delta 3 names a third `scripts/ci-*` body as the thing
  not to create, and mirroring the sibling leg is what it specified.
  **Two shapes, and they are not equivalent.** Extract the shared adopter step into one script both
  legs call, accepting the third body delta 3 refused; or gate the package set against
  §Requirements' list. The second is enforcement-first and reaches the page as well as the legs,
  which the first does not.
  **THE GATE SHAPE'S OWN ENTRY WAS MERGED HERE 2026-09-11 BY THE POOL TRIAGE**, and
  `macos-leg-brew-set-vs-documented-requirements` is Done; its grounds ride on here. That gate
  needs a floor-member-to-brew-formula mapping surface that does not exist: §Requirements derives
  its toolchain block from context-kit's `PROBE_SET`, whose floor members map to brew formulae
  neither one-to-one nor derivably. The legs are BINDING, so drift greens `master` over a broken
  adopter path. Scope cut that entry from the 2026-09-07 set on sizing, never merit, as first back
  in at the next window.
  **Product-class under TRAJECTORY.md's 2026-08-30 witness discriminator** — the subject is what a
  documented adopter host needs, and a §Requirements list gone wrong is an adopter's own witness.
  **Cost while deferred:** two copies and a near-copy, none of them held; the failure mode is a
  green leg over a broken install page.
  Filed 2026-09-08 by build to the gap inbox and promoted at this close's drain: →fix refused, since
  choosing between extraction and a gate is design work and delta 3 already refused the extraction
  half once; →icebox refused on the adopter witness above.

- **stage-journal-path-unsourced-mid-stage** [design-pending] [cost: event/high] [surface: lifecycle-kit] — a stage session dispatched into an
  ALREADY-ENTERED stage has no mechanical source for that stage's journal path, so it invents a
  discriminated filename and the successor's entry assertion then refuses.
  **Attested twice, 2026-09-10 and again 2026-09-11:** each time `--enter-stage validate` refused
  because `.tmp/build-journal.md` carried only its opening line while build's lead-cut batch
  sessions had written `-batchN-` journals, three the first time and four the second, and each
  time a batch paid a hand-written stand-in to clear it.
  **Not a cardinality defect — that first reading was corrected the same day.**
  lifecycle-kit/SPEC.md §The state machine already rules the multi-session case: one journal per
  stage, every session appending under a heading naming itself, and the entry assertion satisfied
  by ANY session of the predecessor having written. `--enter-stage`'s journal open appends and
  never overwrites for exactly that reason, so nothing is owed on that axis.
  **What IS owed is the seam:** the canonical path is printed by `--enter-stage` alone, and the
  second or third batch of an already-entered stage never runs that arm.
  **THE CHEAPEST CANDIDATE IS A MEASURED FAILED CONTROL, not an unbuilt option.** Obliging the
  batch-dispatching lead to grant the derived path IS ALREADY IN THE TEMPLATE and has been since
  2026-08-25: lifecycle-kit/templates/lead.md §Channel design says the lead "still spells it out
  in the dispatch prompt" and refuses a per-batch filename by name. Seven batch sessions across
  the two attestations violated it. The 2026-09-11 instance closes the last excuse available to
  that shape — the dispatching lead RELAYED the resume-journal discipline into every prompt in
  the same breath as naming the per-batch path, so the control did not fail for want of being
  read, and a discoverability repair drafted at that close (a §Economics pointer to §Channel
  design) was WITHDRAWN unlanded as an eighth instance of the same failing control.
  **Why `[design-pending]`:** the surviving shapes differ in kind and none is costed — a read-only
  `--emit stage-journal-path` arm any session can run without stamping; the same obligation moved
  into the stage-session agent contract, where a non-entering session actually reads it; or a
  detection half, now that the prompt-side rule is measured not to hold. Choosing is scope's.
  **DISTINCT from the concurrency limit the same SPEC paragraph files** (two sessions of one stage
  appending at once, untested and not claimed safe): this is a sequential session that does not
  know the path at all.
  **Cost while deferred:** the failure lands on the NEXT stage's entry, cold, holding only whatever
  escape was written down — a session that did nothing wrong pays for one that did.
  **NOT ICEBOX-ELIGIBLE on its `[cost:]` class:** `event/high` is outside the low class
  queue-kit/SPEC.md §The icebox tier admits. The 2026-09-10 lead decision read the since-retired
  prose opener and machinery-class conjunction; the class carries the same verdict now.
  Filed 2026-09-10 by close, draining the gap inbox. →fix was refused on the re-verification above:
  the obvious cheap fix is already landed, and already failed.
  recurrence: stage-journal-path-unsourced-mid-stage 2026-09-11
- **icebox-standing-cause-truncated-on-board** [design-pending] [cost: iteration/low] [surface: queue-kit]
  — `--icebox-candidates` prints about three characters of a standing row's grounds.
  An ineligible row's reason is capped at `CAUSE_CAP`, 48 characters
  (native/src/emit/queue_index.rs), and the fixed prefix `[standing] not-icebox-eligible <date> — `
  spends 44 of them, so a declared row shows its date, three characters and an ellipsis.
  **Re-verified at close:** both standing rows on the 2026-09-11 worklist render `— evi…` and
  `— liv…`. The cap no longer touches any cost text: an eligible row prints its `[cost:]` class
  whole, so only the ineligibility reasons are cut.
  **Candidate repairs, unranked:** a shorter fixed prefix, the `✗` mark already saying ineligible;
  or exempting the grounds tail from the cap. Either changes the printed format queue-kit/SPEC.md
  §The queue-index arm owns, which is why close did not fix it inline.
  **Cost while deferred:** a close reviewing the standing declarations pays one anchored grep per
  declared row to read grounds the worklist was meant to show, growing with the declarations.
  Filed 2026-09-11 by close, draining the gap inbox; →fix refused on the printed-format ground.
- **prose-enum-common-word-tag-members** [design-pending] [cost: event/low] [surface: canon-kit]
  — check-prose-enum reads two adjacent ordinary words as a partial list of the queue's task tags.
  The consumer set `queue-task-tag` (native/src/emit/enum_sets.rs) now carries two task tags
  whose names are common English words, beside `spec`. canon-kit/SPEC.md §check-prose-enum
  matches a member bare or bracketed and chains members separated by whitespace, so prose that
  happens to put the two words side by side reads as a hand list and reds on every omitted tag.
  **Re-verified at close:** the first instance is in range, a historical release post exempted by
  a `prose-enum-exempt:` comment when the tags landed.
  **The obvious fix is not free.** Requiring the bracketed or backticked spelling for common-word
  members would stop the gate reading the one surface its attested drift lives on: README.md's
  queue-kit row spells the tag algebra bare and slash-joined. A per-member match mode is also a
  change to the set grammar, which that section keeps at two fields and refuses a third.
  **Cost while deferred:** an occasional false red on unrelated prose, each remedied by an
  exemption comment that dilutes the exemption set.
  Filed 2026-09-11 by close, draining the gap inbox; →fix refused on the set-grammar ground.
- **install-smoke-slow-leg-residue** [design-pending] [cost: iteration/high] [surface: installer]
  — the native Windows install smoke, still the slowest leg, sets every close watch.
  **Measured at close off the first run carrying the roster hash batch:** install-smoke-windows took
  31m46s against 42m48s to 45m58s over the five runs before it; macOS Intel took 13m59s and the
  rest of the workflow under eight minutes. Three terms remain, each with a design question.
  **1. The harness still hashes each recorded file with its own `git hash-object` child**
  (installer/consumer-smoke/run-smoke.sh, the manifest assertion's `got`). That is deliberate: the
  loop is the independent second reading of the recorded value and installer/README.md §The
  manifest names the smoke's comparison as its own invocation, so batching it re-authors that
  operand and has to decide whether a `--stdin-paths` reading still discriminates. The filer's
  measure, not re-run at close: about 250 s of the Windows leg.
  **2. The arms run serially on one core** while the Windows and macOS runners carry several.
  Ordering is load-bearing: the printed arm headers are the `installer_smoke` scenario roster
  (evidence-kit/SPEC.md §Layout and configuration); `ENTRY`, `RUN_PATH` and `VALUE_RED` are
  globals each arm reassigns; the cross-version reversal arm consumes the payload path the upgrade
  arm relinquishes. The cheapest concurrent shape that keeps the roster and every assertion is not
  designed.
  **3. Two large terms are unprofiled, carried as the filer's claim:** the vendored battery (about
  63 s per 40-gate run, several per profile and twice in the value arm) and the non-hash work of a
  full-profile init (about 60 s). No spawn count exists for either, so neither has a known lever;
  a profile is the prerequisite.
  **Cost while deferred:** the slowest smoke leg sets the watch wait on every push, every term
  scales with the profile count, and Windows pays each spawn at MSYS fork cost.
  Filed 2026-09-11 by close, draining three gap-inbox bullets left by the smoke-leg hash batch.

- **align-in-session-absorption-tier-unruled** [design-pending] [cost: event/low] [surface: lifecycle-kit]
  — does a spec miss that build absorbs **in session** count against align's model tier?
  **OPERATOR-CLASS, and that is the whole reason it is filed rather than answered.** Answering it
  re-scopes a recorded tiering reading, which lifecycle-kit/SPEC.md §The steering vocabulary puts
  with the operator through `/consult` and with no session, lead or stage.
  **What is on the record and what is not.** The revert signal in the lead binding (ruling-config)
  names only a **build round-trip** as align's tier evidence. Nothing rules on a miss build absorbs
  without one, so the two readings — a silently-absorbed miss is align failing, or it is align
  working at the tier it was bought at — are both consistent with every surface today.
  **DISTINCT from `align-checklist-fanout-calibration`, Done 2026-09-12**, which closed on the
  checklist residue and left this half out of SPEC-align-claims.md on purpose.
  **Cost while deferred:** every align tier decision is taken on an unstated reading, and the two
  readings recommend opposite moves when the next miss lands.
  Filed 2026-09-12 at spec to the gap inbox; promoted here by close, →fix refused as operator-class.

- **spec-growth-restraint-unstated-at-authoring-surfaces** [design-pending] [cost: iteration/low] [surface: canon-kit]
  — operator direction, 2026-09-12: a SPEC is not expected to grow each iteration; a new feature or
  an ambiguity is preferably handled by re-phrasing existing instructions for clarity and brevity,
  adding text only where it adds value, and brief phrase-shaped instructions read better to an LLM
  than verbose prose.
  **The direction reached this iteration's five amendments and stops there.** The authoring surfaces
  that decide the *next* merge carry no statement of it: canon-kit/SPEC.md §Merging an amendment
  (step 2 says integrate rather than append, but prefers no re-phrasing over adding), the shipped
  amendment template's Definition of Done, the spec and build stage templates, and
  context-kit/templates/close-brevity.md, which accepts growth that earns its cost.
  **Cost while deferred:** later authoring and later merges default to appending, which is the
  behaviour the direction was given against, and each iteration's default compounds.
  **The shape is a re-phrase, not an addition** — applying the direction to the surfaces that state
  it is the first test of it, and a delta that grows those five files fails its own subject.
  Filed 2026-09-12 at spec to the gap inbox; promoted here by close.

- **instruction-sweep-grounds-residue-unrelocated** [design-pending] [cost: event/low] [surface: lifecycle-kit]
  — three grounds passages the 2026-09-12 instruction sweep's per-passage enumeration missed, which
  the differential successor cannot reach because it derives its worklist from a git diff range and
  these lines were never touched.
  **The passages, verified present at 2026-09-12 close** (the close's own brevity pass took the
  *history* half of the residue and left these): lifecycle-kit/templates/lead.md — the
  prompt-answered-signal paragraph's stated-here-rather-than-incident-lore meta-ground, the
  intra-stage-split bullet's nests-a-second-supervisor ground, and the
  Lead-does-stamping-is-ruled-out grounds sentence under §Stamps are authoritative.
  **Why close did not take them.** Each is a *ground*, so the doctrine rule
  (doctrine-kit/DOCTRINE.md, Content-tiering) relocates it to the section owning the
  mechanism rather than deleting it, and each needs a per-passage read of whether
  lifecycle-kit/SPEC.md already
  carries it — a spec-stage judgment, not a brevity-pass deletion.
  **DISTINCT from `close-differential-instruction-sweep`**, the forward mechanism over future edits,
  which is unchanged by this and cannot see these lines.
  **Cost while deferred:** three grounds slabs ride the lead template at every dispatch, inside the
  surface ratchet's ceiling for that file.
  Filed 2026-09-12 at build to the gap inbox; promoted here by close after →fix took only the
  history half.

## Icebox

  Dormant entries, one line each: the cost field said the carry was low, no
  `[roadmap:]` commitment rides on it, and no named event is waiting to
  promote it. Still live work — a legal `[blocked-by:]` target, conserved on
  the way in and on the way back out. The removed body is recoverable from
  the evicting commit (`git log -p -S'<slug>' -- TASK-QUEUE.md`).

- **lead-held-block-no-sanctioned-surface** [design-pending] — No route records a lead-held block.
- **survey-record-filed-after-the-fact** [design-pending] — Order to the work goes wholly unread.
- **amendment-prose-misnumbers-its-delta** [design-pending] — Cites Delta 3 for delta 4's subject.
- **always-loaded-baseline-freshness** [design-pending] — A close may skip the re-baseline; no gate.
- **stage-economics-log-redates-rows** [design-pending] — Re-running the meter re-dates live rows.
- **battery-timing-file-overwritten-by-only-run** [design-pending] — A filtered run reports as all.
- **audit-roster-row-body-unbounded-growth** [design-pending] — Row bodies never compress.
- **worktree-dispatch-rebuilds-the-gate-binary** [design-pending] — Each dispatch pays a cold build.
- **committed-grant-fallthrough-unexplained** [design-pending] — Three bare grants fell through.
- **derived-count-literal-in-queue-unscanned** [design-pending] — No corpus reaches the queue file.
- **audit-roster-grammar-ungated** [design-pending] — Row fields ungraded; waits on the format.
- **bridged-arm-spawned-program-set-unheld** [design-pending] — Declared set unheld; shape ships.
- **icebox-drops-a-bought-census** [design-pending] — No dormant home for a measured payload.
- **surplus-arg-drop-in-six-emit-arms** [design-pending] — Six emit arms drop surplus args at 0.
- **inline-source-literal-ungateable** [design-pending] — Fence-only oracle; no rename pending.
- **turn-end-refusal-used-as-a-busy-wait** [design-pending] — Sessions busy-wait via the stop hook.
- **site-health-probe-no-retry-on-transient** [design-pending] — A single non-200 files an issue.
- **non-gate-arm-roster-hand-maintained** [design-pending] — The arm class's flag list is ungated.
- **craft-rule-step-has-no-reader** [design-pending] — A broken stage-rules knob reds nothing.
- **runtime-dir-two-tier-detector** [design-pending] — No two-tier proof for file-pattern ignores.
- **done-slug-commit-naming-gate** [design-pending] — Done-moving commits need not name their slug.
- **enter-stage-simulate-no-write-fixture** [design-pending] — Guard present, unpinned by a fixture.
- **stage-lag-disambiguation** [design-pending] — Hook over-firing is accepted, not a defect.
- **metric-dir-admission-unstated** [design-pending] — Ad-hoc scripts persist in .metric/.
- **stage-economics-smoke-jq-arm-dormant** [design-pending] — Its jq-absent arm never runs anywhere.
- **hermetic-bin-roster-config** [design-pending] — Pinning coverage needs a consumer roster seam.
- **split-posture-waiver-writer** [design-pending] — A lead-issued waiver stamp has no writer.
- **supervisor-verification-attestation** [design-pending] — The verification duty is unattested.
- **gate-spec-claim-assertion-parity** [design-pending] — Ruled a human-audit class, not gateable.
- **port-takeability-has-no-instrument** [design-pending] — Takeability hand-read on the tree axis.
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
- **recurrence-judgment-vs-declaration** [design-pending] — The two share a noun, not a meaning.
- **interpreter-floor-gawk-residue-empty** [design-pending] — Its ground died; awk stands.
- **inline-interpreter-heredoc-unsteered** [design-pending] — No rule steers `python3 -`.
- **advisory-lane-draft-state-unswept** [design-pending] — GitHub's notifications are the sweep.
- **amendment-done-move-assertions** [design-pending] — Zero cost while merges are hand-checked.
- **guard-advise-jq-dependency** [design-pending] — Needs jq; the one consumer works around it.
- **survey-record-extension-tier-hybrid** [design-pending] — Paid only by a future workflow author.
- **install-lifecycle-reversibility** [design-pending] — A declined branch; only optionality owed.
- **pack-installer-payload-kit-set-anchor** [design-pending] — Latent --root trap, no caller.
- **installer-jq-usability-probe** [design-pending] — Broken-but-present jq is unobserved.
- **rendered-site-link-monitor** [design-pending] — Rendered-site link rot waits on a launch crawl.
- **kit-index-page-vocabulary-ungated** [design-pending] — Index-page enums are ungated.
- **absence-statement-grammar** [design-pending] — When to state absence, and how, is unruled.
- **contributor-writeback-disposition** [design-pending] — Write-back is dormant pre-launch.
- **context-pressure-signal** [design-pending] — Compaction timing has no per-session signal.
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
- **installer-graph-artifact-literal** [design-pending] — Init literalises a resolver-owned path.
- **doctrine-rule-number-citation-liveness** [design-pending] — A renumber stales citations.
- **false-ground-citation-propagation** [design-pending] — Nothing re-reads a ground once cited.
- **spec-embedded-source-criterion-4-membership** [design-pending] — Its port sizing stays unruled.
- **lead-dispatch-simulate-optionality** [design-pending] — Dispatch may skip the pre-flight.
- **self-repo-prefix-normalisation-unheld** [design-pending] — Two link-prefix holders, unheld.
- **stage-cursor-rerun-stamp-gap** [design-pending] — A skipped re-run stamp points the cursor back.
- **interpreter-grant-redirect-residue** [design-pending] — Seven redirected shapes stay ungranted.
- **canonicalize-extended-length-prefix** [design-pending] — A Windows `\\?\` root is unconverted.
- **stage-cursor-unread-by-index-check** [design-pending] — A clean index hides who holds the stage.
- **crate-arms-relink-under-worker-pool** [design-pending] — It relinks the binary it runs in.
- **build-native-obligation-unconditional** [design-pending] — A crate-free commit still rebuilds.
- **port-blockers-library-mediated-scan** [design-pending] — A library-mediated spawn reads clean.
- **bridged-arm-requirements-undeclared** [design-pending] — `--needs` omits what an arm spawns.
- **delta-citation-unresolvable** [design-pending] — A delta number names no openable file.
- **scratch-grant-backtick-declined** [design-pending] — Rule 17's own clause voids its use case.
- **walk-entry-model-unstated** [design-pending] — Walk drops symlinks unstated; tree has none.
- **prune-set-matches-walk-root-ancestors** [design-pending] — A leaf above the root prunes it all.
- **evidence-baseline-orphan-suite-row** [design-pending] — A row for a retired suite is unread.
- **port-archaeology-restatement-residue** [design-pending] — Prose narrates deleted shell forms.
- **non-gate-arm-testing-floor-unstated** [design-pending] — A new arm's testing floor is unstated.
- **prune-set-convergence-question** [design-pending] — Two kits' prune sets diverge, unruled.
- **gap-inbox-slug-predicate-ground** [design-pending] — Its anti-cycle premise died unreplaced.
- **emit-arm-usage-unreachable** [design-pending] — Prints only on a refusal; lead-ruled 2026-09-03.
- **check-graph-trigger-consumer-path-reach** [design-pending] — couples= misses installer/.
- **precondition-gate-negation-false-positive** [design-pending] — Reds a true negated precondition.
- **worktree-isolated-dispatch-cannot-reach-the-main-checkout** [design-pending] — Bridge undecided.
- **cited-object-token-sweep-corpus-narrower-than-the-class** [design-pending] — Corpus unruled.
- **worktree-lock-start-time-guard-untaken** [design-pending] — Dormant until a consumer acts on it.
- **worktree-cleanliness-assertion-scopes-to-checkout** [design-pending] — Reds on foreign dirt.
- **release-record-retired-knob** [design-pending] — A removal's basis may not name its own knob.
- **friction-key-segment-selection-unruled** [design-pending] — Which segment to key is unruled.
- **scratch-auto-allow-no-decoration-steer** [design-pending] — Chained writes lose the steer.
- **cost-series-limb-unreadable-inside-close** [design-pending] — Close cannot price its open row.
- **post-build-instrument-edit-unowned** [design-pending] — No stage owns a post-build tree edit.
- **dod-size-figure-stales-in-iteration** [design-pending] — Spec's promotion ages its own DoD size.
- **portability-count-on-two-surfaces** [design-pending] — Hand-spelled census; both true today.
- **wrap-budget-caps-lead-line-tags** [design-pending] — A long slug's tag neither fits nor wraps.
- **smoke-roster-guard-precedes-hand-off** [design-pending] — Guard stricter than its stated reason.
- **edges-retired-block-name-clash** [design-pending] — A live gate name inflates a retired slug.
- **lead-report-is-an-ungated-terminal-act** [design-pending] — May close holding unfiled work.
- **smoke-report-array-carrier-mangling-unexplained** [design-pending] — Witness now needs design.
- **queue-entry-evidence-tier** [design-pending] — Nothing signals an entry was compressed.
- **gate-timing-baseline-comparability** [design-pending] — Timing baseline has no comparer.
- **amendment-landing-citation-assertions** [design-pending] — Landing citations go unvalidated.
- **root-doc-roster-registration-parity** [design-pending] — Only one root-doc roster is enforced.
- **enforcement-first-load-trigger** [design-pending] — No stage loads the enforcement-first rule.
- **self-revert-reminder-expectation** [design-pending] — Self-revert reminder reads as injection.
- **consumer-smoke-subset-accounting-verdict** [design-pending] — Kit-subset smoke reds falsely.
- **co-authored-by-trailer-attribution** [design-pending] — Model trailer is a baked literal.
- **guard-steer-grant-mismatch** [design-pending] — Guard steers onto forms no allowlist grants.
- **reclaim-precondition-outside-the-tree** [design-pending] — Essay-sink reclaim can never fire.
- **release-drain-ordering-contradiction** [design-pending] — Step 4 opener contradicts its body.
- **amendment-dod-sibling-dependence** [design-pending] — DoD items depend on unnamed siblings.
- **recurrence-resolver-literal-match-only** [design-pending] — Unspelled recurrences file as new.
- **section-prose-outlives-its-entries** [design-pending] — Section preambles outlive Clear-Done.
- **amendment-roster-stale-by-construction** [design-pending] — Sweep rosters stale mid-iteration.
- **unregistered-gate-fixture-coverage** [design-pending] — Unregistered gates skip fixture duty.
- **queue-tier-label-correction-cost** [design-pending] — Fixing a label at the cap costs a trim.
- **rejected-compound-commit-relabel** [design-pending] — A bare retry mislabels staged work.
- **survey-record-supersede-invisible** [design-pending] — Superseded survey blocks look live.
- **probe-before-assertion-doctrine** [design-pending] — Rule shipped; mechanizing it is open.
- **consumer-smoke-accounting-spelling-unpinned** [design-pending] — Dual-spelling count unpinned.
- **build-stage-tightened-gates-write-pair** [design-pending] — Template names one of two writes.
- **release-runbook-identity-diagnosis** [design-pending] — Account check is prose, not a step.
- **always-loaded-baseline-restamp-unforced** [design-pending] — Nothing forces the meter restamp.
- **drift-baseline-unnamed-iteration** [design-pending] — Scope-time KPIs baseline on old commit.
- **dispatch-cited-evidence-unverified** [design-pending] — A sweep's quotations go unverified.
- **queue-provenance-restates-git-history** [design-pending] — Provenance prose restates git log.
- **comment-tier-surface-excludes-ci-workflows** [design-pending] — Workflow comments go ungated.
- **consult-rulings-outside-the-authority-roster** [design-pending] — Consult readings lack a slot.
- **lead-ruling-reopen-authority-unstated** [design-pending] — Who reopens a lead ruling is open.
- **ruling-record-prose-staleness-unreachable** [design-pending] — Old rulings evade the probe.
- **close-surface-reclaim-uncoupled-from-read** [design-pending] — Reclaim may wipe unread rows.
- **iceboxed-recurrence-judgment-unrecordable** [design-pending] — No room for a recurrence stamp.
- **queue-citation-line-number-stales-within-its-own-session** [design-pending] — Line cites stale.
- **post-scope-admission-has-no-promotion-route** [design-pending] — Late debt has no promoter.
- **declaration-shape-outside-header-unreadable** [design-pending] — Inert literals read as live.
- **inline-interpreter-substrate-census** [design-pending] — Scratch computations may be tooling.
- **boundary-preserve-covers-names-not-lifetimes** [design-pending] — Keep-list lists names only.
- **validate-suite-wall-clock-unowned** [design-pending] — Serial smoke suites cost ~16 minutes.
- **overlay-only-oracle-grants-uncommitted** [design-pending] — Oracle grants live off-tree.
- **close-triage-log-reclaim-loss-window** [design-pending] — Truncate after read drops appends.
- **nested-battery-env-inheritance-invisible** [design-pending] — A scoped nested run reads clean.
- **enforcement-first-behavioral-regressions** [design-pending] — Rule under-cues behavior gates.
- **spec-split-promotion-review** [design-pending] — Spec-stage default awaits an economics read.
- **build-stage-tier-economics** [design-pending] — Build tier set by intuition, not a priced A/B.
- **supervision-overhead-unmeasured** [design-pending] — Supervision burn priced; quality unread.
- **gate-battery-result-cache** [design-pending] — Battery reruns all gates on an unmoved tree.
- **state-representation-integrity** [design-pending] — Text-state invariants are gate-held only.
- **rule-reach-before-merits** [design-pending] — Merits argued before a rule's reach is set.
- **template-copy-parity-yaml-widening** [design-pending] — YAML template copies mirror by hand.
- **gate-tests-suite-identity-in-evidence** [design-pending] — Two suites can share one hash.
- **template-spec-restatement-reach** [design-pending] — No gate holds a SPEC off its template.
- **amendment-deletion-content-completeness** [design-pending] — Merges can drop rationale unheld.
- **template-registry-population-predicate** [design-pending] — A name collision would red parity.
- **lead-line-parser-conformance** [design-pending] — Eight lead-line holders; no conformance.
- **queue-status-parenthetical-liveness** [design-pending] — Stale status tags beside cited slugs.
- **breadth-declaration-stale-listing** [design-pending] — Spent breadth declarations stay silent.
- **breadth-declaration-committed-glob-home** [design-pending] — Glob keep-rulings have no home.
- **criterion-4-two-spellings-disagree** [design-pending] — Criterion 4 reads two ways.
- **relayed-rule-role-scope-unchecked** [design-pending] — Relayed rules may bind the wrong role.
- **substrate-parity-assertion-c-reach-unannounced** [design-pending] — C can shrink unannounced.
- **promotion-commitment-stamp-latency** [design-pending] — At-ceiling stamps wait on promotion.
- **recurrence-threshold-counts-dates-not-incidences** [design-pending] — Same-day firings merge.
- **same-stage-journal-append-uncoordinated** [design-pending] — Parallel appends share one file.
- **survey-engagement-trigger-narrower-than-its-class** [design-pending] — Trigger is scope-only.
- **recurrence-declaration-grammar-ungated** [design-pending] — Declaration grammar has no gate.
- **scratch-citation-introducer-form-reach** [design-pending] — Copula pointers evade the scan.
- **threshold-entry-escalation-travel-unruled** [design-pending] — Rider or competitor, unruled.
- **one-motion-commit-race-remains-open** [design-pending] — add-then-commit still races.
- **amendment-target-delta-correspondence-unverified** [design-pending] — Orphans in both ways.
- **dispatched-child-asserts-an-unverified-base** [design-pending] — Children infer, not probe.
- **hermetic-harness-export-masks-the-condition-under-test** [design-pending] — Pins void arms.
- **verbose-battery-idiom-steered-to-its-granted-spelling** [design-pending] — Bare form prompts.
- **exe-suffix-single-spelling-unenforced** [design-pending] — Suffix owner claim has no gate.
- **intra-stage-batch-stamp-unobserved** [design-pending] — A skipped batch stamp goes unseen.
- **boundary-sweep-github-write-skips-identity-step** [design-pending] — Account check unenforced.
- **wait-primitive-and-record-compose-to-false-completion** [design-pending] — Waiters exit early.
- **readonly-dispatch-type-cannot-see-gitignored-surfaces** [design-pending] — No ignored files.
- **kfric-second-field-direction-inverted** [design-pending] — Surface field names the owner.
- **baseline-self-certification-unasserted** [design-pending] — Self-served verdicts unasserted.
- **design-pending-boilerplate-reds-its-own-promotion** [design-pending] — Reds on promotion.
- **wait-mandate-template-spelling-unreachable** [design-pending] — Mandated spelling is refused.
- **consumer-smoke-single-kit-run-not-self-sufficient** [design-pending] — Per-kit smoke fails.
- **upgrade-smoke-refuses-inside-a-worktree** [design-pending] — Refuses where .git is a file.
- **pre-grammar-disposition-authority-ambiguity** [design-pending] — Double-named rulers unread.
- **kpi-cost-per-unit** [design-pending] — No KPI prices cost per shipped unit.
- **line-range-citation-stales-inside-its-own-iteration** [design-pending] — Line ranges go stale.
- **amendment-commit-shape-red-conditions** [design-pending] — Prompt lacks a commit-shape class.
- **amendment-correction-density** [design-pending] — Correction density goes unmeasured.
- **probe-evidence-sufficiency** [design-pending] — Rule 12 passes a probe that is no evidence.
- **scratch-citation-skill-surface-reach** [design-pending] — Skill files escape the pointer scan.
- **throughput-and-wait-time-unmeasured** [design-pending] — Wait and throughput are unmeasured.
- **headroom-check-ordering-unruled** [design-pending] — When to read cap headroom is unruled.
- **dispatch-unreadable-target-fallback** [design-pending] — Blind sweeps echo the prompt as PASS.
- **queue-write-side-verb** [design-pending] — The queue has no write-side verb.
- **close-red-push-ownership** [design-pending] — No owner for a close blocked by a red push.
- **expected-permission-mode-undeclared** [design-pending] — No surface states the expected mode.
- **consumer-guard-rule-coverage** [design-pending] — Consumer-only guard rules are untested.
- **scan-prompts-blocking-half-blind** [design-pending] — The KPI cannot see blocked commands.
- **handoff-premise-reverification-placement** [design-pending] — Premise-check placement unruled.
- **amendment-work-class-label-placement** [design-pending] — Work-class tag placement unruled.
- **tracked-to-untracked-pointer-scope** [design-pending] — Untracked-target pointer scope unheld.
- **born-native-flip-enforcement-gate** [design-pending] — Born-native rule has no enforcing gate.
- **msrv-move-clippy-arm-coupling** [design-pending] — Floor moves surface unbudgeted lints.
- **allowlist-path-existence-unchecked** [design-pending] — Dead-path grants go undetected.
- **declaration-lib-refusal-output-leak** [design-pending] — Refusal output mixes in good tokens.
- **deferred-pool-identifier-restatement-sweep** [design-pending] — The pool was never swept.
- **fixture-assertion-liveness** [design-pending] — Stale fixture expectations go uncaught.
- **fixture-assertion-coverage-unmeasured** [design-pending] — Driver coverage of arms unmeasured.
- **survey-engagement-residue-untracked** [design-pending] — Engagement residue rests on conduct.
- **metric-dir-member-contract-unheld** [design-pending] — The metric dir accretes leftovers.
- **kit-bin-entry-point-unrostered** [design-pending] — No roster maps bin tools to kits.
- **recurrence-obligation-residency** [design-pending] — Stamp duty reaches two of six stages.
- **queue-lib-dead-derivation** [design-pending] — Three queue-lib regexes have no live reader.
- **gate-test-in-tree-invoker-ruling** [design-pending] — Is a gate-test an in-tree caller?
- **survey-oracle-liveness-unasserted** [design-pending] — An oracle may name a wiped path.
- **survey-witness-composed-from-unvalidated-corpus** [design-pending] — Prose corpora pass clean.
- **stage-completion-unattested** [design-pending] — Entry stamps cannot show completion.
- **deferred-entry-time-deixis-rot** [design-pending] — Relative deixis rots in deferred bodies.
- **iteration-scoping-clause-date-ambiguity** [design-pending] — Date scoping names no iteration.
- **assertion-strength-exit-header-reach** [design-pending] — The gate now reaches no script.
- **audit-depth-measure-degrades-under-fanout** [design-pending] — Fan-out depth is self-reported.
- **candidate-list-anchors-a-sweep-obligation** [design-pending] — A relayed list anchors a sweep.
- **spec-pointer-boundary-legality** [design-pending] — Resolving targets may be illegal owners.
- **ruled-line-retirement-provenance-census** [design-pending] — Census of the retirement's cuts.
- **amendment-census-claim-unrun** [design-pending] — Amendment counts skip the shipped oracle.
- **composition-test-scores-a-section-cut-as-one-unit** [design-pending] — A cut scores as one.
- **deleted-runner-anchors-across-ten-entries** [design-pending] — Entries cite deleted runners.
- **audit-roster-grammar-and-stored-probe-set** [design-pending] — Probe set is a stored constant.
- **port-created-failure-mode-refusal-unruled** [design-pending] — Port-made refusals unruled.
- **removal-propagation-site-argued-out-of-scope** [design-pending] — Found sites argued away.
- **wait-form-unallowlistable-by-construction** [design-pending] — No grant reaches the wait.
- **icebox-trigger-blind-to-retired-carrier** [design-pending] — Blind to retired carriers.
- **rationale-located-by-reading-not-by-grep** [design-pending] — Grep misses paraphrases.
- **smoke-whole-tree-precondition-unscoped** [design-pending] — Any dirty path blocks the smoke.
- **run-validate-child-env-knob-leak** [design-pending] — Suite children inherit bridged knobs.
- **append-grant-decline-cause-unlogged** [design-pending] — Decline cause truncated from the log.
- **spec-internal-identifier-prefix-drift** [design-pending] — SPECs cite internal names, not knobs.
- **lint-scope-hook-trigger** [design-pending] — extra lint dirs skip the commit hook, CI-only.
- **release-note-section-set-derivation** [design-pending] — release gate hand-lists note sections.
- **release-asset-claim-class-owner** [design-pending] — a release-ships-an-asset claim has no gate.
- **spec-pointer-self-section-citation** [design-pending] — same-file section cites go unchecked.
- **knob-default-accessor-singularity** [design-pending] — no gate bars re-spelling a knob default.
- **bin-argv-shape-residual-member** [design-pending] — one bin arm skips the argv-shape contract.
- **docs-corpus-derivation-manifest-divergence** [design-pending] — two docs gate corpora diverge.
- **consumer-gate-roster-unread** [design-pending] — no roster reads a consumer-declared gate.
- **waiter-loop-condition-predicate-gap** [design-pending] — waiting rule misses a lone waiter loop.
- **crate-toolchain-grant-uncommitted** [design-pending] — overlay keeps cargo grants ruled out.
- **prose-uniqueness-claim-unchecked** [design-pending] — no gate checks a prose uniqueness claim.
- **kit-spec-layout-tree-hand-maintained** [design-pending] — kit SPEC layout trees are hand-kept.
- **expansion-rule-backtick-blind** [design-pending] — expansion rule misses backtick substitution.
- **fixture-runner-checks-dir-fails-open** [design-pending] — bad checks-dir arg drops silently.
- **enter-stage-refusal-help-contradicts-its-guard** [design-pending] — help contradicts its hook.
- **artifact-substitution-remedy-has-no-end-to-end-arm** [design-pending] — untested end to end.
- **uninstall-artifact-ownership-asymmetry** [design-pending] — uninstall leaves init's artifact.

## Done

## Lessons Learned
