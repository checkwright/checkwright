# TASK-QUEUE.md — Checkwright work queue

## Iteration: workflow-surface-tiering

  The lifecycle-kit gates read this header's iteration name and the stage
  cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt`
  (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue
  format itself and gates this file. One iteration per hardening or roadmap
  unit; [README.md](README.md) maps the kits.

---

## New Features

- **workflow-dir-tracking-claim** [spec: canon-kit/SPEC-tracking-claim.md] —
  CLAUDE.md §Housekeeping asserts `.workflow/` is committed while three of its
  twelve members are gitignored, on an always-loaded surface every session
  reads. Corrects the claim to a two-tier rule and takes the open gate arm: a
  tracking claim is machine-checkable against `git check-ignore`, so the
  correction lands with the gate that keeps it true. Design, ruled-out
  alternatives, and the gate's honest limit: the amendment.
  Lands **after** `workflow-file-format-convention` — the corrected sentence
  cites the surface contract that unit creates.
  Promoted 2026-07-22 by spec, which authored the amendment and took the gate
  arm scope left open.

- **close-triage-surface-roster** [spec: lifecycle-kit/SPEC-close-surface-roster.md]
  — close's inbound triage surfaces are enumerated only as prose, so a surface
  close never reads leaves no trace anywhere. Replaces the enumeration with a
  derived roster carrying a per-surface forced/advisory mode, plus the closure
  that reports an undeclared capture surface instead of inheriting the hole.
  **Cross-component by design** (lifecycle-kit, queue-kit, guard-kit,
  drift-kit, doctrine-kit): `check-stage-entry` assertion C is armed, and an
  **audit stamp** is the expected disposition at the next stage's entry —
  anticipated, not a waiver.
  Promoted 2026-07-22 by spec, which authored the amendment.

## Technical Debt

- **gap-inbox-post-close-window** — the gap inbox is drained by
  `close` and refuses the next iteration-boundary `scope` entry while non-empty
  (`enter-stage.sh`), which assumes every bullet is filed *during* an iteration.
  A bullet filed after that iteration's close has no drainer left in the state
  machine: the next `scope` is refused and only an out-of-band edit clears it.
  Reproduced live: `enter-stage.sh scope --simulate` refused on two post-close
  filings.
  **Not a gate** (lead ruling): `close` legitimately writes the
  inbox when it drains, so the rule cannot be "block changes" without separating
  close-truncating from anyone-appending; `file-gap.sh` appends without
  committing, so a gate reds at whatever commit next carries the file, blaming
  an actor who did not file the bullet; and refusing capture does not dissolve a
  real finding — it pushes it back into session context, the deferred-capture
  antipattern the inbox exists to prevent. A close that files something it
  genuinely cannot disposition in-session would red on a correct action.
  **Preferred arm:** warn at the point of capture — `file-gap.sh` knows the
  current stamp, so it can tell the filer, while they can still act, that the
  bullet will block the next `scope` unless drained or promoted directly.
  **Second half:** `enter-stage`'s refusal text says only that "the close stage
  must drain every gap", which once close has finished describes a stage that is
  not coming back; it should name the promote-directly recovery. The existing
  refusal *did* detect this correctly — what failed was the message's
  actionability, which is why a second detector is the wrong fix.
  Debt: a real boundary hazard in shipped mechanism, not a new capability.
  **Promoted 2026-07-22 by scope** into `workflow-surface-tiering` (operator
  ruling). Routed as debt, not to the authoring stage: both halves converge
  shipped mechanism onto its own stated contract — a warning line in
  `file-gap.sh` and an actionable refusal message in `enter-stage.sh` — and
  neither arm adds a name to a governed surface. The "open to spec" phrasing
  the entry carried while deferred is dropped: the lead's not-a-gate ruling and
  the named preferred arm are the design, and no arm remains that would need an
  amendment.
  Filed 2026-07-20 by lead, drained from the gap inbox.

## Deferred

- **boundary-scratch-wipe-unowned** [needs-spec] — the iteration-boundary `.tmp/`
  wipe is prescribed as **consumer binding prose** (the `/scope` skill's
  evidence-reset binding) and performed by hand, so every scope session emits an
  ad-hoc destructive shell command. The recurring shape, verbatim:
  `find .tmp -mindepth 1 -not -name session-role -delete && ls -a .tmp/`
  Cadence: once per iteration, on every scope session, permanently. It prompts
  every time and cannot stop prompting — the `&&` makes it a compound no prefix
  entry matches, and `find … -delete` is not allowlisted at all. The allowlisted
  `rm .tmp/*` globs cannot express "preserve `session-role`", which is exactly
  why the hand-written form reaches for `find`.
  **Widening the allowlist is the wrong fix** — it grants a broad destructive
  shape to buy one convenience. The operation has no per-invocation variance, so
  it belongs behind kit mechanism the allowlist matches bare and undecorated,
  which kills the prompt at its source and leaves the allowlist no wider.
  **The home already exists — no new script.** `bin/enter-stage.sh` is already
  the boundary actor: it resolves the scratch dir at `:57`
  (`GATE_SDK_TMP_DIR:-.tmp`), already writes there, already truncates the
  boundary-reset file set at `:176-181`, and already reports what it truncated at
  `:199`. The wipe is the same boundary operation over a different surface.
  **Why this is a feature, not a one-line debt fix — the reason it is filed
  rather than folded in.** The wipe must *preserve* `.tmp/session-role`, and that
  marker is **context-kit's** surface (its session-context hook reads it; its
  lifetime is the lead session's, not the iteration's), not lifecycle-kit's.
  Expressing the exception needs a preserve-list knob — presumably
  `LIFECYCLE_KIT_BOUNDARY_PRESERVE`, following the established
  `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` array pattern. That is a **new name on a
  governed surface**, so the new-names litmus makes it a feature requiring an
  amendment however small the diff looks. Two consequences follow: the amendment
  spans ≥2 component dirs (lifecycle-kit mechanism, context-kit's marker as the
  reason for the exception), which arms `check-stage-entry` assertion C and
  demands an audit stamp at the next stage's entry; and moving the wipe from
  binding prose into kit mechanism is a provenance-seam call in its own right —
  what is mechanism versus what stays consumer config.
  **Open design question:** whether the preserve list is the right shape at all.
  The alternative is that the kit wipes nothing and instead *names* the scratch
  surface it owns, leaving cross-kit markers somewhere the boundary never
  touches — which would dissolve the exception rather than configure it, and may
  be the better answer.
  Debt/feature: adds a governed name; needs an amendment. Not implemented.
  **Cost while deferred:** low, non-rotting, but permanent and paid by the
  **operator** rather than the agent — one unavoidable interruption per
  iteration, on a command whose destructive shape is exactly the kind a reviewer
  should not be trained to wave through. Bounded: nothing breaks, no gate reds.
  Surfaced 2026-07-22 by the operator during the `budget-oracle-honesty` scope,
  naming the exact recurring command; filed by scope at lead instruction after
  verifying the placement claim against `enter-stage.sh`.
  **Evidence added 2026-07-22 by the `workflow-surface-tiering` scope — the
  command drifts between sessions, so the shape recorded above is one observed
  instance, not the canonical form.** That session performed the same prescribed
  wipe as:
  `find .tmp -maxdepth 1 -type f ! -name session-role -delete`
  — compare the shape recorded verbatim near the top of this entry, which no
  session has now reproduced. Three independent divergences for one prescribed
  operation — the traversal
  bound (`-maxdepth 1` vs `-mindepth 1`), a type filter present in one and
  absent in the other (`-type f`), and the negation spelling (`!` vs `-not`) —
  plus the trailing listing dropped. The first two are not cosmetic: the
  observed form deletes only regular files at the top level, while the recorded
  form descends and removes directories too, so the two commands do not even
  agree on what the wipe removes.
  **Why this sharpens the entry's own argument rather than qualifying it:** the
  central claim is that the operation has no per-invocation variance and so
  belongs behind mechanism. That holds — what varies is not the operation but
  the *command*, because binding prose specifies an outcome and each session
  re-authors the shell for it from scratch. A destructive command whose exact
  effect is re-derived per session is a stronger case for mechanism than a
  stable one would be, since the recurring cost is not only the prompt but a
  fresh chance to get the deletion boundary wrong.

- **done-slug-commit-naming-gate** [needs-spec] — `kpi-task-split` reads a Done
  slug's feature/debt class off the commit its message names, via
  `git log -1 --grep=<slug>`. Nothing requires a landing commit to name its
  slug, so a correctly-typed `fix` commit that omits it leaves the row
  **unclassified** and the KPI reports `0f/0d`. Reproduced this iteration on all
  four units: `1dac2f9`, `9c5aeb2`, `a740a7b` are each typed `fix(delegation)`
  and each moves its slug(s) into Done **in the same commit**, yet none names a
  slug in its message — so the newest matching commit for every one of the four
  is the `chore(scope)` opener, and the split read `0f / 0d of 4 done (4
  unclassified)` for an iteration that was unambiguously four debt units.
  **Gap generalization — this class is gateable, and precisely.** The commit-msg
  hook holds both inputs the rule needs: a commit whose diff **adds a slug line
  to the queue's Done section** must name that slug in its message. Both halves
  are mechanical, the trigger is narrow (only Done-moving commits arm it), and
  there is no judgment in the match — the low-false-positive contract looks
  satisfiable without a new vocabulary. The queue-section parse already exists in
  `kpi-task-split`'s awk and in queue-kit's gates.
  **Open design (why `[needs-spec]`, not a build unit):** whether the rule binds
  the *moving* commit or merely requires *some* commit in the range to name the
  slug — a build may legitimately land the fix in one commit and move the queue
  line in another, and the strict form would then red on a correct sequence. That
  choice decides the gate's shape and its false-positive surface, and it is a
  queue-kit-vs-gate-sdk placement call besides (queue-format knowledge versus
  commit-message mechanism).
  **Cost while deferred:** low per-iteration but silently compounding — the
  headline lead KPI reads `n/a`-shaped noise whenever the convention lapses, and
  the lapse is invisible until close reads the report. `trajectory.sh` is
  unaffected (it splits range commits by subject and recorded this iteration's
  debt correctly), so the published evidence stays honest; what degrades is the
  live dashboard the lead steers by. Bounded: nothing breaks, no gate reds.
  Debt: converges a KPI onto a stated convention; adds no governed name unless
  the gate lands. Filed 2026-07-22 by close, from this iteration's own
  unclassified split.

- **scan-prompts-local-overlay-blind** [needs-spec] — `bin/scan-prompts.sh`
  filters the friction log against `GUARD_KIT_SETTINGS` (the committed
  allowlist) only. `GUARD_KIT_SETTINGS_LOCAL` exists in `lib/guard.sh` and
  `compare-settings-allow.sh` reads it, but the ranker never does — so a command
  granted solely by the local overlay is reported as a prompting call although it
  did not prompt. Concretely this iteration: `gh issue list --state open`,
  `gh pr list --state open`, and the bare `gh release create` are all covered by
  `.claude/settings.local.json` globs and all three appear in the ranked
  survivors, so **at least 4 of the 23 reported calls never prompted anyone**
  (a fourth, an exact-string local entry, is counted below). With 90 local
  entries live here the inflation is structural, not incidental.
  **The claim was the defect, and it is already corrected** — guard-kit/SPEC.md
  §scan-prompts said "so only commands that actually prompted remain", which is
  false for any consumer with an overlay; it now states the honest limit and the
  design reason the overlay is excluded (a locally-granted command is exactly the
  triage candidate close must see, to promote or prune). What is *not* settled,
  and is this entry, is the mechanism.
  **Open design:** three arms, genuinely undecided. (a) Keep committed-only
  filtering and rename the output/KPI so nothing claims to count prompts —
  cheapest, but leaves `kpi-prompt-friction` a mixed quantity. (b) Read both
  files and report two numbers — prompted versus committed-uncovered — which is
  the honest shape but doubles the KPI's surface and its fixtures. (c) Read both
  and rank the overlay-covered survivors in a separate, visibly-advisory section,
  which preserves the promote-or-prune worklist while making the prompt count
  true. Note (b)/(c) inherit a second inaccuracy the committed path already has:
  `allowed()` glob-matches the whole command string, so `git *` matches a
  compound the harness would split and refuse — a wider filter is *more* wrong on
  compounds, not less, and any arm that reads more settings must settle the
  compound semantics first.
  **Cost while deferred:** low and non-rotting now that the SPEC states the
  limit; the residue is that the friction trend line the lead steers by is an
  upper bound of unknown slack, so a real regression and an overlay artifact read
  identically. Debt: converges an advisory tool onto its own contract; adds no
  governed name. Filed 2026-07-22 by close, from this iteration's tooling-friction
  triage.

- **scratch-execution-prompt-friction** [needs-spec] — this repo's `bash-guard`
  actively steers every scratch write into repo-local `.tmp/` (CLAUDE.md
  §Housekeeping; the harness scratchpad is refused by name), and nothing
  allowlists *executing* what lands there — so `bash .tmp/<probe>.sh` prompts on
  every run, forever. Measured this iteration: 4 calls on one build probe, plus 3
  more in the close session's audit sweep, on a shape the repo's own guard
  mandates. The loop is self-inflicted: the guard creates the directory
  convention, and the permission posture penalizes it.
  **Widening the allowlist is not obviously the fix, which is why this is
  `[needs-spec]`.** `Bash(bash .tmp/*.sh)` auto-approves running an
  agent-authored script whose *contents* no one reviewed — it converts a visible
  command into an opaque one, which is the opposite of what the prompt exists to
  do. The competing arms: (a) allowlist it and accept that scratch execution is
  agent-trusted, arguing the agent could inline the same code anyway; (b) steer
  to inlining, which works for short probes and fails for genuine multi-line
  sweeps (this close's audit sweep is one — loops and arrays, not inlineable);
  (c) give the boundary a named, gated scratch-runner so the *shape* is
  allowlistable while the content stays visible in the log. Arm (b) is already
  known-insufficient from this session's own experience, which is the evidence
  the entry rests on.
  **Deliberately not folded into `boundary-scratch-wipe-unowned`** — that entry
  is about *wiping* `.tmp/` at the boundary and needs a preserve-list knob; this
  is about *executing* from it and needs a trust ruling. Same directory, unrelated
  questions.
  **Cost while deferred:** low per-call, paid by the operator, and rising with
  build-probe volume — every reproduction harness costs interruptions
  proportional to how many times it is re-run, which penalizes exactly the
  iterative probing that finds real defects. Bounded: nothing breaks.
  Debt/policy: a permission-posture ruling; adds no governed name unless arm (c)
  ships. Filed 2026-07-22 by close, from this iteration's tooling-friction triage.

- **enter-stage-simulate-no-write-fixture** [needs-spec] — add a regression
  fixture asserting `enter-stage.sh --simulate <stage>` leaves the tree
  byte-identical after a *successful* (non-refused) boundary entry. The guard
  now present at `lifecycle-kit/bin/enter-stage.sh:168-171` (the `sim` exit ahead
  of every write) has no test pinning it, so a future refactor could silently
  re-introduce the success-path write. This is the second half of the now-closed
  enter-stage-simulate-writes entry (Done): the behavioral fix is verified
  present, and only its regression guard is missing. Consider also asserting the
  `(simulate)` marker prints on the success path so an honoured flag stays visible.
  Debt: adds a fixture to shipped mechanism, no governed name.
  **Cost while deferred:** low and non-rotting — the fix is present; the exposure
  is a future un-caught regression on a rarely-touched dry-run path.
  Filed 2026-07-20 by scope, the closed entry's own second ask (operator ruling).


- **guard-read-compound-carveout** [needs-spec] — tighten `guard_rule_cat_read` /
  `guard_rule_find_glob` so an all-reads compound does not slip their
  composition carve-out. Both rules deliberately exempt a `cat`/`find` that
  pipes, redirects, or feeds a consumer — correct, that is real composition. But
  a compound whose every segment is a bare read (`cat A; echo ===; cat B`) is not
  composition; it is two file reads batched through the shell to save a
  round-trip, which is exactly what the Read/Glob steer exists to prevent, and it
  currently falls through to a permission prompt. Observed 6x `cat` + 4x `find`
  in one iteration's friction log. Fix: treat a `;`-separated compound as
  steerable when every segment is itself a bare read, leaving genuine
  pipe/redirect composition untouched; ships with the good/bad fixture pair.
  **Recurred 2026-07-22** (`budget-oracle-honesty` close triage): 3x `cat` + 2x
  `find`, plus two `;`-joined all-`git`-read compounds. Three of the five were
  emitted by the close session itself while reading four small config files —
  the strongest form of the argument, since a session executing the triage that
  owns this entry still reached for the batched read. One `find` is the separate
  `boundary-scratch-wipe-unowned` wipe and is not this class. Two iterations of
  recurrence now, with no counter-evidence.
  Debt: refines an existing guard rule, adds no governed name. Filed 2026-07-20
  by tooling-friction triage during the `render-fidelity-leak-coverage` close.

- **queue-selection-order-implicit** [needs-spec] — `queue-kit/SPEC.md`
  documents section order as selection order, so the default section sequence
  silently makes `New Features` outrank `Technical Debt` in what scope picks
  first. Unlike the spec-tag requirement that `CANON_KIT_FEATURE_SECTIONS`
  places on `New Features` alone, which is principled and argued, this ordering
  policy is embedded in section sequence with no stated argument anywhere.
  Either state the argument or make selection order explicit rather than
  positional. Surfaced while ruling on whether the two active sections should
  collapse into one — they should not, but this rides along on their sequence.
  Debt: latent policy with no owner doc.
  Filed 2026-07-20 by lead while ruling on the active-section question.

- **rendered-site-link-monitor** [needs-spec] — durable coverage for the
  reader-facing link liveness of the rendered checkwright.dev site. Internal
  and external link rot recurs, and the tree-side reference gates
  (check-md-refs, check-docs-nav-reachable, check-docs-render-fidelity) plus
  the site-health.yml deployment probe cover render and deployment truth but
  not the rendered-site external-URL crawl a reader actually hits. A hermetic
  gate is ruled out on record: site-kit/SPEC.md §The monitor boundary —
  external-link liveness reds on causes no commit produced (DNS, a moved
  target, an incident), breaking the low-false-positive contract. So the
  durable form is a **monitor**, a scheduled crawl step extending site-kit's
  site-health.yml, signalling through an issue and its own red run, never a
  blocked merge. Demand-gated like the other rungs: promote when the one-time
  launch crawl (launch-readiness-gate validate) shows recurrence worth
  automating. Surfaced 2026-07-16 in the launch triage that scoped
  launch-readiness-gate.
- **plugin-marketplace** [needs-spec] — harness plugin/marketplace packaging
  of the stage skills and guards; anti-drift gate shape: manifest ↔ shipped
  surface parity. Design against the live manifest format at promotion — the
  plugin substrate moves fast (the scope-session-routing ruling applies).
  Surfaced 2026-07-09 in adoption-track's split; evidence artifact retained:
  upstream Claude Code issue #75214 (project config can't lift the Task
  ask-first default), surfaced dogfooding the delegation nudge 2026-07-07.
- **benchmark-ab-experiment** [needs-spec] — the controlled differential
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
- **prose-profile** [needs-spec] — the non-code universality rung: a third
  consumer shaped as a prose/documentation repo (no build, no test suite)
  stress-tests whether the kits govern non-code work. Core dilution is ruled
  out on record — if pursued, this is an adapter/profile delivered as
  optional consumer config, never a kit literal (the provenance seam).
  Demand-gated: it attests only when a non-code consumer actually vendors a
  kit and hits friction; until then this entry is the roadmap marker. Seeds:
  gate-sdk, guard-kit, context-kit, drift-kit, and canon-kit's
  one-owner/coupling core are workflow-agnostic today; lifecycle-kit's stage
  semantics, evidence-kit's test baseline, and canon-kit's spec framing are
  software-coupled — the abstraction axis is "code + spec" artifacts
  generalizing to "governed surface". `check-prose-tells` (the
  launch-readiness-gate build) is the first concretely prose-shaped kit
  mechanism and the natural profile seed. Surfaced 2026-07-16 in the same
  launch triage that scoped launch-readiness-gate.
- **hosted-attestation-service** [needs-spec] — the team/paid rung: gates
  verified server-side by a party the committing agents cannot touch —
  hosted gate runs as a neutral attestation, cross-repo drift dashboards,
  maintained rulesets. A service, not code: cloning the kits does not clone
  the neutrality or the ops. Demand-gated — this entry is the public
  roadmap marker, not a scaffold; hosting and sequencing decisions are on
  record in the operator's local brief, and multi-operator-semantics
  is its prerequisite mechanism. Surfaced 2026-07-07.
- **spec-internal-identifier-prefix-drift** [needs-spec] — SPEC prose naming a
  script's **internal** variable spelling where the public knob is the contract
  name. Found by the config-seam-hardening close audit of the
  `internal-identifier-restatement` roster class, and fixed there: seven sites
  in delegation-kit/SPEC.md named `PAUSE_PCT`, `PAUSE_PCT_7D`, `LOGIN_WINDOW`,
  `REFRESH_CMD`, `REFRESH_MIN_AGE` — each of which exists in
  `bin/usage-verdict.sh` only as a local assigned straight from its
  `DELEGATION_KIT_`-prefixed env knob. The same doc's §Layout roster names the
  prefixed spelling correctly, so the drift was prose-vs-roster *within one
  file*.
  **Cost while deferred:** the fix is a rename away from rotting — renaming the
  local in the script silently falsifies the prose, and only the roster class's
  audit cadence catches it, at iteration granularity.
  **The manual fix is demonstrably incomplete — an eighth site surfaced
  2026-07-19.** The `tooling-signal-honesty` close audit swept the class
  mechanically and found `DELEGATION_KIT_STALE_AGE` cited as bare `STALE_AGE` at
  delegation-kit/SPEC.md:200, a survivor the config-seam-hardening pass missed
  while correctly fixing seven neighbours in the same file (the §Layout roster two
  hundred lines below had the prefixed spelling right the whole time). Fixed at
  that close. This is the demand evidence the entry was waiting on: a hand sweep
  of one file missed one instance in eight, so the audit cadence is catching
  what review does not — and a ~20-line scan expressed the whole class, which is
  itself evidence the low-false-positive boundary is tighter than feared (one hit,
  zero false positives, across every kit SPEC in the tree).
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

- **stage-lag-disambiguation** [needs-spec] —
  narrow the session-context hook's accepted over-firing by distinguishing a
  first-of-stage session from a restarted predecessor session, using the session
  id the stage-cursor migration puts in the state file's last stamp.
  **The rule was never wrong, only wrongly argued.** context-kit's cursor-lag
  rule (context-kit/SPEC.md §The session-context hook) already survived the
  stage-cursor migration on its *lag* ground: the hook fires at session start,
  before the arriving skill's
  first step writes the cursor, so a first-of-stage session still reads the
  predecessor's value. What the migration retired is the rule's former stated
  **justification** — "no header value distinguishes [a first-of-stage session]
  from a restarted predecessor session" — and the over-firing cost it accepted
  on that basis. The header carried one value; the state file's last stamp
  carries `<iter> <stage> <session-id> <date>`, so the id distinguishes exactly
  the two cases the rule says are indistinguishable: ids match = restarted
  session of the stamped stage, ids differ = new session whose stage has not
  stamped yet. Found by the stage-cursor-extraction align audit; deliberately
  left out of that amendment's envelope (a behavior widening coupled to a
  mechanical migration muddies what its fixture proves).
  **Cost — larger than the comparison it looks like.** The session id *is*
  reachable from the hook payload, so the premise holds: the session-role
  signal already reads it (`scripts/session-context.sh`, spec at
  context-kit/SPEC.md §The session-context hook), and the stamp's 8-char id
  already matches the
  `${hook_sid:0:8}` comparison shape that code uses. But that read sits inside
  the `[[ -f "$ROLE_FILE" && ! -t 0 ]]` guard, and **stdin is consumable
  exactly once** — so the payload read must be hoisted to the unconditional
  path before a stage derivation can use it. (The stage-cursor migration added
  a *named-file* cursor read ahead of that guard, deliberately consuming no
  stdin, so it left this hoist untouched.) The hoist is the real work, and it
  lands on a hook whose contract is "never fails a session" and "signal absent =
  byte-identical output": a read that today happens only for lead-marked
  sessions would happen every fire. Note the audience inversion that makes the
  hoist unavoidable rather than incidental — the sessions this entry serves are
  stage sessions, which carry *no* role marker, i.e. precisely the case the
  current guard skips. Scope: the hoist plus its no-payload/`-t 0` fallbacks,
  the comparison, and a fixture pair proving both the first-of-stage and
  restarted-predecessor cases; both kit template and consumer copy.
  **Cost while deferred:** low and non-rotting — the over-firing is the
  documented accepted cost, not a defect, and the rule stays correct on its lag
  ground. Unblocked: `stage-cursor-extraction` has reached Done, so this entry
  is pickable.

- **heterogeneous-agent-delegation** [needs-spec] — cross-vendor stage dispatch:
  a Claude Code lead delegating a stage (e.g. `/build`) to a foreign coding agent
  (Codex, etc.), extending the homogeneous multi-agent / multi-operator model to a
  heterogeneous fleet. Cashes the public "no IDE/model/harness lock-in"
  positioning claim, and is the purest expression of the thesis — governance
  enforced at the git/gate boundary, not by trusting the author. It splits along a
  **two-substrate line**. *Already agent-neutral:* the verification substrate (git
  + the gate battery + the bash stamp state machine) does not care who
  authored the diff — a foreign agent's commit is gated identically; and the
  concurrent-agent coordination primitive is the shared git-index/HEAD
  serialization, vendor-neutral already. *Homogeneous today — the real work,
  worst-first:* (1) the **escalation resume model** is the gating sub-problem —
  the lead's whole value is resuming a paused stage in place instead of
  cold-restarting; a foreign agent cannot be `SendMessage`-resumed, so a foreign
  stage either runs fully autonomously (no mid-stage escalation) or escalates
  through a committed/polled channel with cold restarts, forfeiting the lead's
  cost asymmetry — the part that is not plumbing. (2) **dispatch transport** —
  today the harness `Agent`/`SendMessage`/task-notification; a foreign agent needs
  a transport-neutral handoff (committed worklist, issue, spawned process). (3)
  **budget oracle** — `usage-verdict.sh` is Anthropic-OAuth-specific; a
  heterogeneous fleet has N vendor-keyed oracles (the same seam as this session's
  credential-swap / token-usage tasks). (4) **stage-contract expression** —
  `/build` is a Claude Code skill (markdown + tool bindings); the lifecycle
  machinery is already neutral bash but the skill prose is not, so the contract
  needs an agent-agnostic form.
  **Seam ruling (on record):** generic mechanism only — the dispatch transport,
  budget oracle, and escalation channel become consumer-config seams (harness-native
  one adapter, foreign-agent another). A kit literal naming a vendor crosses the
  provenance seam and is ruled out, same pattern as `prose-profile`. **Economic
  why:** extends the existing per-batch model-tiering lever (the split-lead
  posture's whole point) across vendors — route each stage to whichever vendor's
  model wins. **Prerequisite cluster:** interacts with `hosted-attestation-service`
  (its neutral-party angle; multi-operator-semantics its named prerequisite),
  `plugin-marketplace` (the harness-absorption hedge), and this session's
  credential-swap budget-oracle tasks. **Demand-gated:** promote when a concrete
  cross-vendor dispatch need attests; until then this is the roadmap marker.
  Surfaced 2026-07-17 in the release-in-lifecycle lead session (operator question
  on external-agent delegation).

- **background-credential-swap-support** [needs-spec] — first-class support for
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
  never blocks (delegation-kit/SPEC.md §The staleness contract, lines 62-64 and
  207-208), so this is signal quality, not a dispatch-blocking defect. Two
  points: the window default is 600s while the SPEC's own stated server-lag is
  "about a minute" (SPEC line 245), a ~10x margin worth tightening; and it is a
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

- **stage-economics-smoke-jq-arm-dormant** [needs-spec] — drift-kit's smoke
  asserts the jq-absent degradation of `bin/stage-economics.sh`, but the
  assertion never executes. `drift-kit/smoke/install.sh:171` branches on the
  **host's** jq (`command -v jq`), so the degradation arm at `:189` — "without
  jq must emit its degradation notice" — is reached only on a jq-less machine.
  Neither this machine nor CI is one (the gates workflow image carries jq), so
  the arm is **dormant on every runner that actually runs it**: a correct
  assertion that no run has ever evaluated. **This is a testability gap, not a
  defect** — the stage-economics-report validate verified the degradation by
  hand (jq masked → exit 0, `jq not found` notice, 0 rows logged), so the
  behavior is known-good; what is missing is the *automation* of that check.
  Fix direction: exercise the arm unconditionally rather than conditionally —
  a second tool run under a PATH sandbox with jq masked, asserted alongside the
  jq-present run, so both arms evaluate on every host. **Scope: drift-kit's
  smoke only.** Deliberately not generalized to a smoke-authoring rule — one
  instance is not evidence for a general rule, and gate-sdk's own jq consumers
  do not assert their degradation at all, so a blanket rule would manufacture
  work against a pattern nobody has shown to be wrong.
  **Cost while deferred:** low and non-rotting. The asserted behavior was
  hand-verified at ship, and the arm cannot *false-green* anything — it is
  skipped, not passed. The real cost is narrow: a future regression in the
  jq-absent path (say a reordering that emits the notice after an unguarded
  `jq` call) lands unnoticed, because the only mechanism watching that path
  never runs. Bounded by the degradation path being small and rarely touched.
  Filed 2026-07-18 by lead ruling at the stage-economics-report close.

- **hermetic-bin-roster-config** [needs-spec] — `check-test-hermetic` assertion
  B catches *partial* credential pinning but not *absent* pinning. B arms only
  when the smoke script itself contains a `*_CRED_FILE=` assignment, so a smoke
  script that calls a credential-consuming own-kit bin and pins nothing at all
  is never flagged. The narrow trigger was deliberate and correct — an
  unconditional every-own-bin-call-must-pin rule false-positives across the
  credential-free kits — so the hole is a known limit of the trigger, not a bug
  in it.
  **Design question (why deferred, not a build fix):** closing it needs a
  per-kit roster of credential-consuming bins, and that roster is exactly the
  content the provenance seam keeps out of a kit — it is optional consumer
  config (the `check-graph` / `scripts/graph-vocab.sh` pattern), never a
  gate-sdk literal. So the deliverable is a config seam and its fall-open
  default, an align-shaped design pass rather than an assertion tweak.
  **Remaining scope is the roster design only — the one-line leak was split out
  and fixed separately.** The concrete `delegation-kit/smoke/install.sh`
  threshold leak this entry recorded is now its own promoted debt unit,
  `delegation-smoke-threshold-pin` (2026-07-21, operator ruling): it needed no
  config seam, only the export the sibling test script already uses. What stays
  here is the part that genuinely earns a design pass — the per-kit roster of
  credential-consuming bins and its fall-open default. Do not re-file the pin.
  **Cost while deferred:** low and bounded — the gate still catches the partial
  case that actually regressed here; the uncovered case is a kit shipping a
  credential-consuming smoke with no pin at all — reproduced 2026-07-21 (was:
  "no kit does today" as of 2026-07-19).
  Cost to close: roughly one iteration. Surfaced 2026-07-19 by the validate
  re-entry on `derivation-by-precedent`, downstream of that iteration's
  operator-authorized hermeticity fix rather than of its precedent-doctrine
  envelope.
  **Reproduced 2026-07-21** by the `lifecycle-rule-placement` validate
  re-entry: `delegation-kit/smoke/install.sh` pins `DELEGATION_KIT_CRED_FILE`
  for its live 95%-reading `usage-verdict.sh` assertion but never pins
  `DELEGATION_KIT_PAUSE_PCT`/`_7D`, so it inherits whatever the ambient
  session env carries. That session's env happened to carry
  `DELEGATION_KIT_PAUSE_PCT=100` / `_7D=100` (an operator override for an
  unrelated stale pre-login budget reading, later reverted in config but
  already exported into the live process — reverting the file cannot unexport
  it from inherited child env), which pushed the 95% reading's expected PAUSE
  (exit 1) to OK (exit 0) — turning demo, consumer_smoke, upgrade, and
  agents_md_smoke red, since all four share that installer. Controlled by
  re-running with both vars unset (no code change): all four suites passed,
  confirming the ambient-env leak, not a code regression, was the cause. The
  one-line fix: `smoke/install.sh` should `export DELEGATION_KIT_PAUSE_PCT`/
  `_7D` around its 95%-reading assertion the same way
  `delegation-kit/bin/run-usage-tests.sh` already pins
  `DELEGATION_KIT_PAUSE_PCT=0` around its own. Still build-routed tech-debt —
  not fixed at this validate re-entry.

- **release-body-url-form** [needs-spec] — *residue only: the two cheap
  deliverables are done (see below); what remains is the monitor-shaped half,
  promotable only together with `rendered-site-link-monitor`.* The `v0.6.0`
  GitHub Release body linked
  `https://checkwright.dev/posts/2026-07-18-checkwright-v0-6-0/` **with a trailing
  slash, which 404s** — the site serves posts without one. Confirmed 2026-07-19:
  the no-slash form returns 200, and `v0.4.0`/`v0.5.0` both use the correct form,
  so this is a one-off regression in the most recent release rather than a
  long-standing convention error. `v0.7.0` hit the same trap while being cut and
  was corrected before this entry was filed, which is the evidence that the trap
  is live rather than historical.
  **Two deliverables — both DONE 2026-07-19, operator-authorized out of stage**
  (the entry's own "pinning the form in the runbook is the cheap half and stands
  alone" is the authority for landing them without a scoped iteration):
  (a) the `v0.6.0` body is repaired — a one-line `gh release edit`, deliberately
  *not* done while closing `derivation-by-precedent` because editing a previously
  published release is outside that iteration's envelope and the envelope had
  already been widened once by ruling; re-verified 200 after the edit, and the
  note's prose is byte-identical apart from the URL. `v0.4.0`/`v0.5.0` re-checked
  and already correct, so no other published body carries the trap.
  (b) RELEASING.md step 5 now pins the no-slash form and makes opening the link a
  named verification; it previously said only that the body "points at the post's
  `https://checkwright.dev/` URL", and that ambiguity is what both slips came
  through.
  **Gap generalization — why no gate.** A release body lives on the host, not in
  the tree, so the battery cannot reach it; and the link is an external URL whose
  liveness reds on causes no commit produced, which site-kit/SPEC.md §The monitor
  boundary already rules out of gate shape on the low-false-positive contract. So
  the durable form is **monitor-shaped, not gate-shaped** — the same disposition
  and the same reasoning as `rendered-site-link-monitor`, over a different surface
  (links *in release bodies pointing at* the site, versus links *on* the rendered
  site). Promote the two together if either earns automation; pinning the form in
  the runbook is the cheap half and stands alone.
  **Cost while deferred:** one dead link in one published release body, plus a
  recurring per-release chance of repeating it until the runbook pins the form.
  Surfaced 2026-07-19 by the close-stage release step for `derivation-by-precedent`,
  verifying that the URL its own release body advertises actually resolves.
- **enforcement-first-behavioral-regressions** [needs-spec] — the always-loaded
  enforcement-first rule ("the fix and the gate that catches it land in one unit;
  removing the duplication outranks gating it") anchors its second clause — and
  every neighbouring doctrine example (content-tiering, de-literalization) — in
  the SSOT/duplication domain, so it under-cues the incident→gate reflex for a
  *behavioral* regression that has nothing to do with duplicated content. Design
  shape: tighten the always-loaded line (or the doctrine-kit section behind it) to
  name the generalize-to-class-then-gate reflex for runtime/behavioral defects,
  not only duplication — a `doctrine-kit/DOCTRINE.md` change, re-vendored to
  upgrade, so it sits outside an incident-fix commit's envelope. **Cost while
  deferred:** each behavioral bug fixed in a maintenance turn risks shipping
  without its paired low-FP gate until a reviewer prompts. Surfaced 2026-07-19 by
  the check-graph `maxEdges` fix (the coupling graph outgrew Mermaid's 500-edge
  render cap): the one-line render fix landed, but the paired render-cap gate —
  exactly the low-FP gate enforcement-first says to land in the same unit — was
  added only on explicit request.

- **spec-split-promotion-review** [needs-spec] — after the six-stage roster has
  run **≥N iterations with `spec`-stage economics actually recorded on the
  trajectory**, re-run the `/economics` read; if the split shows the projected
  cache/context win, **promote the `spec` stage to the kit default** via a new
  amendment (else keep it consumer-config or revert). The `stage-posture-split`
  amendment shipped the stage as optional, demand-gated consumer config
  precisely so this promotion is an evidence-gated follow-up rather than a
  default flip on projection alone.
  **Precondition — hard dependency on `trajectory-stage-roster-hardcode`:** until
  that lands, `drift-kit/bin/trajectory.sh` silently drops every `spec` stamp
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
  **Cost while deferred:** one queue line; the backlog-aging review re-raises it
  every iteration until the data exists to run it. Filed 2026-07-19 by lead
  ruling at the `stage-posture-split-tuning` close — the split shipped on
  projected economics, and this is the loop that confirms or retires that
  projection with recorded data.

- **split-posture-waiver-writer** [needs-spec] — in the split-lead posture the
  lead issues stage rulings (e.g. an align waiver) but commits no lifecycle
  state by design (the lead stamps nothing; only stage sessions write), so a
  lead-issued **waiver stamp** has no clean writer. This iteration's
  `trajectory-stage-roster-hardcode align-waived cbc94da8 2026-07-19` line was
  carried into the *build* session's entry commit (`6b22f38`) as a piggyback,
  because no stage owns the waiver and the lead cannot commit it. The protocol
  worked here only because the build session was told to carry it; nothing in
  the stage skills or lifecycle SPEC prescribes *which* stage commit a pending
  waiver rides, nor guarantees the carry happens at all.
  **Design question (why deferred, not fixed inline):** the fix is a protocol
  ruling on the seam, not a code tweak — options include (a) codify in the
  next-stage skill/entry contract that a stage session carries any pending
  lead-issued waiver into its entry commit (formalize the piggyback that
  happened), (b) a lead-invokable waiver-writer that appends the stamp without
  the lead becoming a lifecycle-state writer (tension with the split posture's
  "lead writes none" invariant), or (c) route the waiver through the same
  committed-worklist channel the dispatch uses. Choosing among these is a
  split-posture-vs-state-machine seam ruling that belongs in
  lifecycle-kit's SPEC / the lead template, so it earns a scope pass.
  **Cost while deferred:** low and non-rotting — the waiver *did* land correctly
  here and `check-stage-evidence` accepts the line, so the machine is not
  broken. The cost is recurrence risk: every future split-posture waiver
  re-litigates which stage commit carries it, and a stage session that stamps
  its entry without carrying a pending waiver would leave the waiver
  uncommitted — living only in the message channel, which is transport, never a
  store (CLAUDE.md §How to escalate). Surfaced 2026-07-19 by the
  `trajectory-stage-roster-hardcode` close, filed by lead-dispatch instruction.

- **supervisor-verification-attestation** [needs-spec] — the resume-journal
  recovery contract now rests on an **unattested** supervisor duty. The
  `resume-journal-done-marker-compliance` amendment (this iteration) rescoped the
  DONE-absence clause so that on the ordinary path "the supervisor consumed the
  agent's return and ran its post-commit verification (§Validate after every agent
  commit)" *is* the recovery contract, making the `DONE` marker redundant. That
  promotion is correct — DONE was false-reading completed runs as interrupted —
  but it moves the load onto a step nothing checks: no mechanism records that the
  supervisor actually ran the verification before deleting the journal, so a
  skipped verification is indistinguishable from a performed one.
  **Deliberately out of the amendment's envelope:** the operator ruled plain (b),
  which excluded marker enforcement and gate mechanization by name — the amendment
  fixes the *reading* of the contract, and mechanizing the *check* is separate
  work, filed rather than smuggled in.
  **Design question (why [needs-spec], not a build unit):** the verification
  happens **after** the commit it verifies, so the pre-commit battery cannot reach
  it — a gate is the wrong shape on ordering grounds alone, the same
  wrong-shape finding as `rendered-site-link-monitor`'s. The open design is what
  an attestation would even be: a supervisor-written stamp (another
  self-asserted-completion marker, which lifecycle-kit/SPEC.md §The stamp protocol
  rules out as proving a claim rather than completion), a next-dispatch preflight
  that refuses until the prior batch's verification is on record, or an honest
  ruling that the duty stays unmechanized and the SPEC's existing
  **Honest limit** paragraph is widened to say so. Note `check-gate-tamper`
  already mechanizes one slice (agent gate-edit *shape*) and its own text concedes
  the by-eye review remains a supervisor duty — so the precedent in-kit is a
  partial floor plus a stated limit, not full mechanization.
  **Cost while deferred:** low today, structurally rising. Under a single
  attentive supervisor the verification does happen (this iteration's every batch
  was verified); the exposure grows with dispatch volume and with the split-lead
  posture, where the lead accepting a stage session's return is exactly the
  unattested step. Bounded by the delete being idempotent and the journal being
  scratch — a missed verification costs a late-caught regression, never lost work.
  Filed 2026-07-19 by the `tooling-signal-honesty` close, as the follow-up the
  plain-(b) ruling named.

- **assertion-strength-exit-header-reach** [needs-spec] —
  `check-assertion-strength` is armed by callee `# exit:` headers, and **two
  scripts in the tree declare one** (`delegation-kit/bin/usage-verdict.sh`,
  `delegation-kit/bin/usage-trend.sh`; the other two hits are the gate's own
  fixtures). Of those, `usage-trend.sh` declares its codes in prose with no
  uppercase token, so it yields an **empty** token→code map — leaving
  `usage-verdict.sh`'s `PAUSE`→1 / `STALE`→2 as the gate's entire live
  vocabulary, over 2 call sites out of 54 scanned scripts. Recorded as an
  honest limit in gate-sdk/SPEC.md §check-assertion-strength.
  **What the close sweep changes about the premise.** The limit reads as though
  the reach were inherently narrow; it is narrow **by adoption**. A count of the
  bin roster found **29 of 64 scripts exit with a code >1**, i.e. carry a
  three-valued contract a header could declare. That is a materially larger
  candidate surface than "one script" suggests.
  **Design question (why [needs-spec], and why the value is the open part):**
  the mechanism is trivial — add `# exit:` headers across the bin roster. The
  unresolved question is whether it buys reach or ceremony. The gate fires only
  when a guard's *failure message names a verdict token*, and for most of the 29
  the code >1 arm is a generic error exit (usage error, unreadable input) rather
  than a named verdict a caller relays, as `usage-verdict.sh`'s is. Headers
  declaring `2 USAGE` / `2 UNREADABLE` would arm the gate against a smoke guard
  claiming "fails on unreadable input" while checking only truthiness — a
  plausible real defect class, but one nobody has shown to have occurred. So the
  design pass must establish the value before prescribing the convention;
  concluding "not worth it" and widening the SPEC's honest limit to say *why* is
  a legitimate outcome of this entry.
  **Cost while deferred:** low and non-rotting — the gate is correct and clean
  over what it reaches, and the SPEC states the limit rather than overclaiming.
  The cost is that each reader re-derives the adoption-vs-inherent distinction,
  as this close did. Filed 2026-07-20 by the `verdict-reader-honesty` close, by
  lead instruction.

- **gate-spec-claim-assertion-parity** [needs-spec] — the generalization the
  `render-fidelity-inline-span-leak` entry flagged as a deliberately-unsettled
  `/spec` question, ruled here: a gate whose **SPEC prose names a failure class
  its assertions do not implement**. This iteration's defect is the instance —
  `check-docs-render-fidelity`'s section described the severed-span defense
  (`gettalong/kramdown#843`) while its assertion matched only a multi-backtick
  fence run, so the prose claimed a defense the code omitted.
  **Ruling — the general class is a human-audit class, not gateable.** The two
  in-kit precedents that look adjacent both key on a **structured token**:
  `check-assertion-strength` reads callee `# exit:` codes and
  `check-gate-assertion-strength` reads the runtime **failure message** string.
  A SPEC's claimed-defense is unstructured natural-language prose with no such
  token, so correlating prose intent to code behaviour is the by-eye
  faithful-artifact residue `check-gate-tamper` and `check-assertion-strength`
  already concede as a stated limit plus partial floor — not a deterministic,
  low-false-positive assertion. No gate is built for the general class.
  **Why `[needs-spec]` and not closed:** a *narrower structured sub-class* could
  be gateable via a **claimed-defense annotation** convention — an author tags a
  claimed failure class with a machine-readable marker and a gate correlates the
  marker to a present assertion. That is a genuine new-gate initiative, filed
  here per `new-initiative-filing-default` (a broader new-gate build is filed,
  never smuggled into this iteration's lean single-unit envelope), demand-gated.
  The open design is whether the annotation buys reach or ceremony — the same
  reach-vs-ceremony question as `assertion-strength-exit-header-reach`; the
  honest outcome may be to record the general class as a permanent human-audit
  limit in `gate-sdk/SPEC.md` near `check-assertion-strength` and build nothing,
  a widest-true-tier placement call in its own right (gate-sdk honest limit vs a
  meta-gate doctrine line).
  **Cost while deferred:** low and non-rotting — this iteration's instance is
  fixed and the general class stays a review tripwire; the recurrence cost is
  that a future gate can overclaim in prose versus its assertions and only human
  audit catches it. Debt/analysis, adds no governed name unless the annotation
  convention is built. Filed 2026-07-20 by the `render-fidelity-leak-coverage`
  spec, settling the flagged `/spec` question.

- **build-stage-tier-economics** [needs-spec] — measure whether the `build`
  stage downgrades from Opus to Sonnet net-positive rather than flipping on
  intuition; a ruling-config tier re-judgment (`.claude/agents/stage-session.md`
  / the lead template's ruling-config, which invites re-judging every tier).
  Grounding, **corrected** against priced rows (the earlier token-only reading
  below predates both the price table and the split-lead posture, and overstated
  build's lead): under the current split-lead posture the priced spread is build
  $2.59–10.96, close $5.83–7.81, validate-on-Sonnet $0.54–1.44. Two readings
  follow, and neither matches the original grounding. **Close is comparable to
  build, not an order of magnitude below it** — so close is a tier candidate in
  its own right, arguably ahead of build, and the premise that build is the
  single highest-value lever no longer holds. And the already-adopted
  validate→Sonnet downgrade **demonstrably works**: validate is the cheapest
  stage by a wide margin with no observed quality cost, which is the affirmative
  precedent this A/B is testing for build. Treat every figure here as provisional
  until the prerequisite below lands — they carry the same attribution defects.
  The superseded token-only reading: build ~100–175k output, 5–25M cache-read per
  run versus close/validate ~7–49k output.
  **Two design blockers:** (1) ~~the decision metric is uninstrumented~~ —
  **resolved**: a price table now exists and the meter prices instead of
  reporting `cost=n/a`. It is replaced by a sharper blocker: the figures are
  priced but **mis-attributed**, so `stage-economics-attribution-honesty` is
  now this task's hard prerequisite. Running the A/B on today's rows would
  compare two noisy numbers — one session bearing two stamps is billed to both
  stages in full, unstamped continuation sessions vanish, and the lead's own
  supervision burn lands in no row at all. Sequence that task first. (2) the
  metric must be **net delivered-work cost** — price-weighted tokens + rework
  round-trips + the supervisor's by-eye gate-diff burden + escalation load
  shifted onto the Opus lead — not single-pass token price; a cheaper builder
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
  ruling-config tier by data, adds no governed name. Filed 2026-07-20 by lead
  ruling during the `render-fidelity-leak-coverage` spec, from an operator
  question.

- **upgrade-smoke-phase-a-regen-derivation** [needs-spec] —
  `gate-sdk/bin/upgrade-smoke.sh` phase A swaps in the tip-of-tree kits then
  regenerates a **hardcoded roster** of generated artifacts — pre-commit,
  check-graph, and (as of `cd5dd59`) install-doctrine's digest — plus a **literal
  `CLAUDE.md`** in the determinism whitelist. Both are hand-maintained copies of a
  fact the vendored kits already own: which artifacts each kit's installer
  regenerates. A kit that adds or renames a generated artifact (as doctrine-kit
  just did, reddening `check-doctrine-registration` in phase B until `cd5dd59`
  patched the roster) silently falls out of phase A until someone hand-edits the
  roster — the maintain-a-derivable antipattern derivation-first rules out.
  **Design direction:** derive the regen set from the vendored kits' own installers
  (each kit owns its generated-artifact targets), so phase A re-runs whatever the
  vendored kits install rather than a literal list, and the determinism whitelist
  follows from those targets rather than naming `CLAUDE.md` by hand. This is the
  interim flagged in `cd5dd59`'s message.
  Debt: converges an existing smoke onto a derived roster; adds no governed name.
  **Cost while deferred:** low but rot-prone — each new or renamed generated
  artifact in any vendorable kit re-opens the same phase-A staleness, caught only
  when a downstream phase-B gate reds and someone re-derives the cause (as
  `cd5dd59` did). Cost to close: roughly one iteration. Filed 2026-07-21 by build
  during `lifecycle-rule-placement`'s model correction.

- **stage-tiering-unit-is-the-batch** [needs-spec] — the lead binding tiers
  models by **stage** (`lifecycle-kit/templates/lead.md` §Economics, "Tier each
  stage to its work class"), but work class varies **within** a stage. This
  iteration is the proof: `build` ran three batches, and batch 2a (a one-line
  hermeticity pin) and batch 2b (a new KPI plugin plus four queue filings) sat in
  the same `build` stage with opposite work classes. Stage is the wrong tiering
  unit; the **batch** is the right one — and the lead already owns the batch
  split, since the neighbouring bullet in that same section says to "split where
  the model tier changes", which presupposes a per-batch tier the per-stage rule
  never provides.
  **This is a reconciliation, not a new policy.**
  `delegation-kit/templates/agent-execution.md` already mandates selection at the
  unit level: "Match the dispatched model and effort to the unit's shape";
  selection "sits with the dispatching session"; the class ladder derives "from
  the harness's **live model roster at dispatch time**"; and "a standing choice
  lands in a tracked agent-type definition, never per-dispatch habit". The lead
  binding sits *under* that doctrine, so its per-stage tiering is a **coarsening
  that contradicts the finer rule already in force**. That framing also indicts
  this iteration's own success: the Sonnet dispatch for 2a was the right answer
  reached by the wrong route — per-dispatch habit, the anti-pattern the doctrine
  names by name. The deliverable is convergence of the lead binding onto the rule
  it already inherits, not the invention of a tiering policy.
  **The substantive design content — `/spec` emits a work-class label per delta,
  never a model recommendation.** A model name written into an amendment is drift
  by construction against a churning roster (the doctrine's own reason for
  deriving the ladder at dispatch time), and a spec-time model *recommendation*
  attaches to a batch that does not exist yet, because the lead cuts batches at
  build. A work-class label — **mechanical** vs **design-bearing** — is durable,
  roster-independent, and is genuinely information spec holds that the lead does
  not: spec knows what each delta demands, the lead knows only what the queue
  entry says. The lead then maps class → live model at dispatch time, which keeps
  the roster dependency where the doctrine already puts it. This iteration's own
  amendment is the worked example: Delta B and the hermeticity rider were
  mechanical; Delta A was not.
  **Evidence, stated honestly — n=1, confounded, and smaller than it looks.**
  Batch 2a cost $1.4131 on Sonnet against build 1's $9.3255 on Opus, but that gap
  is **not** $7.9 of tier saving: cost here is dominated by cache reads (2a
  2,797,771; build 1 9,541,533), so most of the spread is unit size, not tier.
  Repricing 2a's *own* token volume at Opus rates gives a $3.5329 counterfactual,
  so the tier itself saved **$2.1198** on that unit — with quality holding, since
  2a caught an error in the lead's own dispatch brief and mutation-tested its own
  fix. One unit, one iteration, no control: this is a plausibility argument for
  the mechanism, not a measured effect size.
  **A second honesty caveat the price table forces.** `scripts/price-table.tsv`
  carries a KNOWN CLIFF: the Sonnet row is introductory pricing, and past its
  `prices-valid-through: 2026-08-31` the Opus:Sonnet ratio falls from 2.5x to
  ~1.67x on every column. Repricing 2a post-cliff gives a $1.41 saving rather than
  $2.12 — a third of the benefit evaporates on a calendar date, with no code
  change. Any tier ruling this entry reaches must be re-read against the table's
  headers rather than against these figures, which is precisely what
  `kpi-price-table-age` (shipped this iteration) exists to raise.
  **Why the figures are usable at all:** this is the first iteration whose meter
  output is trustworthy, because it is the iteration that fixed the meter's
  attribution (`stage-economics-attribution-honesty`, Done). Every prior
  per-stage figure in the queue predates that fix and should not be compared
  against these.
  Debt/analysis: reconciles a consumer binding to the doctrine above it and adds
  a label to `/spec`'s output contract; adds no governed name.
  **Cost while deferred:** low per iteration, structurally compounding — each
  build stage either pays the Opus premium on mechanical batches or reaches the
  cheaper tier by habit, and habit is the failure mode the doctrine already
  forbids. Sibling to `build-stage-tier-economics`, which asks whether *the build
  stage* downgrades; this entry argues that question is malformed because the
  stage is not the unit.
  Filed 2026-07-22 by close, from a lead-side economics review of this
  iteration's own priced rows.

- **supervision-overhead-unmeasured** [needs-spec] — the `supervision` row is
  now the iteration's third-largest line and has never been examined. Re-derived
  at this close: supervision **$6.9870 of a $37.26 iteration total, 18.8%** —
  larger than every stage but build 1 ($9.3255), and larger than scope ($5.3941),
  spec ($5.2158), and align ($3.2266) individually. It went unexamined because
  until Delta B landed *this iteration* it was **not a distinct row at all** —
  the lead's burn was attributed nowhere. Note the scale comparison that makes
  this the priority: `stage-tiering-unit-is-the-batch` (above) fights over ~$2 of
  build-tier spread; this line is more than three times that.
  **First experiment, to run as recorded here:** run the **next** iteration's
  lead session on Sonnet and measure it against this iteration's Opus baseline of
  $6.9870. The `supervision` row makes that a clean A/B, which is exactly what
  Delta B was built to enable. It could not be tried mid-iteration — a session
  cannot re-tier itself — so the binding change necessarily applies to a future
  lead session, and the baseline above is the number it is measured against.
  **Window declined 2026-07-22 for `budget-oracle-honesty` — deliberately, not
  missed (operator ruling at that iteration's scope).** The experiment was live
  and expiring: a lead session cannot re-tier itself, so running it meant
  restarting the already-Opus lead at iteration open. Declined on the asymmetry
  between the two axes — the cost side now has 17 priced Opus supervision rows
  and the quality side has **no read at all**, and this entry's own risk
  paragraph says a bad ruling would score as a *saving*. Running a cost-only A/B
  on the tier that makes rulings would produce a number that cannot be
  interpreted. This iteration's Opus supervision cost therefore lands as another
  baseline row, and the experiment stays blocked on the quality-read design this
  entry already owes rather than on an absent opportunity.
  **Baseline premise corrected 2026-07-22 by the scope survey — larger than
  filed.** This entry frames the A/B against the single $6.9870 figure. In fact
  `.metric/stage-economics-log.txt` carries **17 priced Opus `supervision` rows**
  spanning ~15 iterations, $3.2016 to $19.1464. The comparison is therefore
  against a distribution, not a point, which both strengthens the eventual A/B
  and sharpens the caveat below: a single Sonnet run must be read against that
  spread, and a spread that wide may swallow the effect entirely.
  **Window declined again 2026-07-22 for `workflow-surface-tiering` (operator
  ruling at that iteration's scope), on the same grounds as the first
  declination — the quality read this entry owes still does not exist, and a
  cost-only A/B on the tier that makes rulings returns a number that cannot be
  interpreted. Second consecutive declination; that iteration's Opus supervision
  cost lands as another baseline row.**
  **Premise sharpened at that scope — the spread argument cuts harder than this
  entry states, and it re-ranks the entry's own open work.** Re-counted:
  `.metric/stage-economics-log.txt` now holds **24 `supervision` rows**, the
  priced Opus ones still running $3.2016 to $19.1464 — a **6x** spread. Read
  against that, a single Sonnet lead session is not merely underpowered at the
  margin; it is uninterpretable in principle, because any plausible tier effect
  sits well inside the existing variance. So the experiment as filed would
  return an unusable number **even if the quality read existed**. The
  consequence is a re-ordering this entry does not currently carry: it reads as
  though the quality read were the sole blocker and the cost read were ready,
  when in fact **both legs are blocked**. The cost leg needs either many
  repeated runs (n≫1, at one lead session per iteration) or a
  variance-controlled comparison that normalizes for iteration size — the
  obvious candidate being cost per unit delivered rather than cost per
  iteration. Design that normalization *before* the quality read: an
  uninterpretable cost axis makes the quality axis moot, and the reverse does
  not hold. Landed here so the next scope reads the corrected case rather than
  re-deriving it from the log.
  **A measurement caveat on the baseline itself:** supervision is the only row
  still growing while close runs, so any figure quoted for it is a snapshot. The
  lead's own mid-close read was $6.4552 (17.6% of $36.72); this close's read is
  $6.9870. The A/B must therefore compare rows read at the *same* lifecycle
  point, not two convenience snapshots.
  **The risk this experiment carries, which is a different class from
  validate's.** Supervision is where **rulings** happen. This iteration's
  highest-judgment act was a supervision-axis ruling — the intent oracle's
  finding that age *inverts* at a cliff, which redirected the whole
  `price-table-age-kpi` unit and is the reason it shipped an expiry header rather
  than an age-only KPI. Downgrading the tier that makes rulings is not the
  already-vindicated validate downgrade, whose rows are mechanical
  oracle-running; the failure mode is not a bigger bill but a **bad ruling that
  costs a rebuild**, which the cost row would score as a *saving*. So the
  experiment needs a **quality read alongside the cost read**, and the design
  question this entry owes is what that read is — a rebuild count, an escalation
  correctness sample, or an honest ruling that the axis is unmeasurable at n=1.
  **What cannot be harvested by delegation.** Supervision splits internally the
  same way `build` did — mechanical routing/verification versus genuine rulings —
  so the batch-tiering answer looks transferable. It is not: the verification
  half is **not delegable away from the supervising session**, because the
  supervisor re-running the battery and diffing every agent commit *is* the
  protocol (`delegation-kit/templates/agent-execution.md`, "Validate after every
  agent commit"). A supervision split can therefore be tiered but not delegated,
  which narrows the available levers to the tier question this experiment tests.
  **Why the figures are usable at all:** same provenance as the entry above —
  this is the first iteration whose meter output is trustworthy, being the
  iteration that fixed the meter's attribution, and the `supervision` row exists
  at all only because of that fix.
  Debt/analysis: measures an unexamined cost line and may re-tier a lead binding;
  adds no governed name.
  **Cost while deferred:** the largest unexamined line in the iteration budget,
  paid every iteration, with no evidence either way about whether it is bought or
  wasted. Bounded and non-rotting — nothing breaks, and the row now accumulates
  per-iteration baselines whether or not the experiment runs.
  Filed 2026-07-22 by close, from the same lead-side economics review.

## Done

- friction-log-merge
- workflow-file-format-convention

## Lessons Learned
