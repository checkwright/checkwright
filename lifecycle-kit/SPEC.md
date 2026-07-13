# lifecycle-kit — an evidence-stamped iteration lifecycle for stateless agent sessions

An iteration is a self-contained work cycle driven through a configurable
sequence of stages (default: `scope → [align] → build → validate → close`).
The problem the kit solves: a stateless agent session cannot be trusted to
remember, or even re-read, a process document — so the process state lives in
two governed files a gate can read, and every stage transition leaves
machine-checkable evidence.

The kit carries the generic state machine only; a consumer's stage names,
exit conditions, and ritual content are config and skill-template fill-ins.
Requires [gate-sdk](../gate-sdk/) (the gates follow its four contracts and
resolve through its registry).

## The state machine

Two governed surfaces:

- **The header line**, at the top of the consumer's queue file (default
  `TASK-QUEUE.md`):

  ```
  ## Iteration: <name>  [stage: <stage>]
  ```

- **The evidence file** (default `.workflow/WORKFLOW-STATE.txt`): free prose,
  then a `---` separator, then one data line per stage-skill invocation:

  ```
  <iteration> <stage> <session-id> <YYYY-MM-DD>
  ```

The default motion is the linear stage walk; the gate-legal shapes for leaving
it — abandon, split, reopen — are specified in §Deviation transitions, not
improvised.

### The flip+stamp protocol

The **arriving** stage's skill flips the `[stage:]` line to its own stage as
its first step, committed atomically with its evidence stamp — the departing
session never flips it, leaving no uncommitted stage line across the session
boundary. The flip commit stages the queue file + the evidence file, so every
queue/state-coupled gate re-fires on it: the prior stage's machine-expressible
exit is re-verified *at the flip* (`check-stage-evidence`), and
`check-stage-entry` extends that one hop back. A self-asserted "stage
complete" marker would prove a claim, not completion — the kit deliberately
has none.

The **deterministic half** of that first step — read the iteration from the
header, read the id from `session-id.sh`, append the stamp, flip the
`[stage:]` field — is mechanized by `bin/enter-stage.sh <stage>`, the same
writer/asserter split as `gen-pre-commit.sh` ↔ `check-graph`: the skill
invokes it, **judgment stays in the skill** (what the stage means, its exit
condition, when to enter it at all), and the stage gates stay the independent
verifier. The tool takes no `--force` flag, so the compliant path is the easy
one — an operator who intends to override runs the flip+stamp by hand, exactly
as before the tool existed. Committing the flip+stamp remains the skill's
business — **never with `--no-verify`**: `enter-stage.sh` refuses to write
while `check-stage-entry` is red, so the hook a bypass skips is exactly the
battery that would confirm the stamp just written. A stage flip is never the
one-off-with-cause that a bypass is reserved for.

The first stage is the iteration boundary: `enter-stage.sh` *truncates* the
evidence file back to its header (git history is the permanent audit trail;
the gates only read the current iteration) and stamps under the
unnamed-iteration sentinel `—`, rewritten to the real name when the stage
names the iteration. Later stages only append.

**Honest limit:** a stamp proves the stage skill was *invoked*, not that its
work was done faithfully — strictly better than skip-and-no-trace, but not
proof of done. The `<session-id>` field is **read, not hand-picked**:
`bin/session-id.sh` prints the canonical id by a fixed derivation order
(§bin/session-id.sh), which rotates per session (including across a context
clear), and the stage skills stamp exactly what it prints, so each stage's
provenance is observed, not guessed.
`check-stage-evidence`'s invocation floor keys on `<iteration> <stage>`; it
additionally reads the session id to enforce cross-stage distinctness (a stage
flip must carry a fresh session — see §check-stage-evidence).

**The optional lead never becomes a second state source.** An iteration may run
with a live *lead* session (§templates/lead.md) that dispatches its stage
sessions and answers their escalations so a blocked stage resumes in place
rather than restarting. The lead writes no state: every flip and stamp
originates in a stage session through `enter-stage.sh`, exactly as above, so the
flip+stamp protocol stays the only iteration state and a lead crash costs
nothing the tracked surfaces do not already hold. The lead is a boundary skill,
not a stage — it flips nothing and joins no stage set, so the coverage gate
never reads it (the release-sweep precedent, §templates/lead.md).

### Deviation transitions

The stages walk `scope → align → build → validate → close` in order by
default; the gate-legal shapes for leaving that walk are specified here, not
improvised. Each composes mechanism the kit already owns — `enter-stage.sh`'s
boundary reset, canon-kit's amendment pairing, queue-kit's tag algebra — so
**no new tooling, state, stamp grammar, or tag is introduced**, and a
harness-less consumer keeps every shape. `check-stage-entry` and the flip+stamp
protocol bar an ad-hoc abandon, which is why each hatch is spelled against the
existing gates.

**The demote ritual** is the shared step the other shapes compose. To take a
promoted entry out of a live iteration: move it back to the deferred queue
section restoring its design-pending tag, and delete its amendment file in the
same commit. Git history preserves the design — a later scope re-promotes by
resurrecting the file from history rather than re-deriving it. The enforcement
is already on the books: canon-kit's `check-amendment-queue` reds the commit on
a deferred entry that still carries a spec ref, or an orphaned amendment left on
disk. If the validate baseline carries a scenario keyed to the demoted entry,
that scenario is re-scoped or removed in the same commit — a coverage-honesty
obligation, not a gate one (`check-evidence-baseline`'s slug-liveness passes
regardless, since a demoted entry stays a live queue task).

**Abandon** ends an iteration without a close. Disposition every active entry
explicitly — demote it (ritual above) or carry it (it stays active with its
amendment and the next iteration adopts it); sink or delete every Lessons entry
under the existing disposition rules (the first-stage entry refuses a non-empty
Lessons section, so this is already forced). Then the next `enter-stage.sh
scope` *is* the abandon: scope has no mandatory predecessor, so the flip is
gate-legal from any stage, and the boundary reset drops the dead iteration's
stamps exactly as it drops a closed one's (git history is the permanent audit
trail — the existing boundary doctrine, not a new rule). The abandon commit's
subject names the abandoned iteration; no stamp grammar changes.

**Split mid-flight** narrows a live iteration. The iteration name never changes
once set — every stamp already written carries it, and a rename-in-place is
barred because it would orphan those stamps against `check-stage-evidence`'s
header/stamp agreement. So splitting is demotion: demote the split-out subset
via the ritual and drive the remaining queue through the remaining stages; the
subset re-promotes at a later scope under its own iteration.

**Reopen after close** is barred as an in-place edit. Stamps are append-only
within an iteration and scope is the only reset, so there is no gate-legal way
to continue a closed iteration's evidence file — and no history rewrite is
sanctioned to fake one (doctrine-kit rule 16 territory). The sanctioned shape
is a successor iteration: a post-close defect files as a debt entry and the
follow-up iteration proceeds normally; the closed iteration's record stays
immutable.

## Layout and configuration

The kit is vendored beside gate-sdk (conventionally at `lifecycle-kit/`); its
gates are registered in the consumer's `gates.list` by name and resolve
through gate-sdk's multi-kit path (consumer gates dir first, then each kit's
`checks/`). Stage skills adopt `templates/skills/*.md` in one of two modes —
copied into the consumer's agent-skill directory with each named slot
overwritten, or a thin binding shim that references the template (the grammar
and the contract both modes satisfy are §templates/skills/).

The stage machine itself is config with this repo's lifecycle as the
default: copy `templates/lifecycle-config.sh` into the gates dir as
`lifecycle-config.sh` (or point `LIFECYCLE_KIT_CONFIG_FILE` elsewhere) and
set only what you override — this roster owns every knob and its default;
the template carries no second copy. The loader validates the machine
(unknown stages in the map, a waiver token colliding with a stage name, a
non-integer n-gram width, a malformed preflight entry) and exits 2 on a
malformed config — a broken machine must not gate anything.

Knob-rename compat precedent: before the first release tag a knob rename is
compat-free — no read-the-old-name shim, no deprecation window — because no
external consumer can have vendored the kit yet (the first tag is a
launch-comms prerequisite). From the first tag onward a rename owes the
queue-bound deprecation mechanism and a tightened-gates/release-note
declaration (the deprecation-lifecycle and upgrade-path rungs).

- `LIFECYCLE_KIT_STAGES` — the stage roster, in order; default
  `(scope align build validate close)`.
- `LIFECYCLE_KIT_PREDECESSOR` — associative map stage → the predecessor whose
  stamp `check-stage-entry` requires; default `([align]=scope [build]=scope
  [validate]=build [close]=validate)` (`build` keys to `scope` because the
  audit stage is trigger-gated; §check-stage-entry).
- `LIFECYCLE_KIT_FIRST_STAGE` — the stage whose entry is the iteration boundary
  (§bin/enter-stage.sh truncation); default `scope`.
- `LIFECYCLE_KIT_DRAIN_STAGE` — the stage whose entry requires the active queue
  sections empty; default `validate`; empty disables the drain assertion.
- `LIFECYCLE_KIT_ACTIVE_SECTIONS` — the queue sections the drain assertion
  reads; default `("New Features" "Technical Debt")`.
- `LIFECYCLE_KIT_AUDIT_STAGE` — the trigger-gated audit stage assertion C looks
  for; default `align`; empty disables the audit machinery entirely.
- `LIFECYCLE_KIT_AUDIT_ENTRY_STAGE` — the stage whose entry assertion C blocks
  on a cross-component signal with no audit stamp; default `build` when an
  audit stage is set, else empty.
- `LIFECYCLE_KIT_WAIVER_TOKEN` — the stamp token recording the user's explicit
  audit waiver; default `<audit-stage>-waived`; must not collide with a
  stage name.
- `LIFECYCLE_KIT_AMENDMENT_GLOB` / `LIFECYCLE_KIT_ROSTER_BASENAME` — the amendment
  filename shape and the canonical-spec basename assertion C scans
  (template dirs pruned); defaults `SPEC-*.md` / `SPEC.md`.
- `LIFECYCLE_KIT_CONTRACT_TOKENS` — the amendment-body substrings assertion C
  reads as a cross-component contract signal; default `("SPEC.md" "proto/")`.
- `LIFECYCLE_KIT_SKILLS_DIR` — the agent-skill directory
  `check-stage-skill-coverage` scans; default `.claude/commands`.
- `LIFECYCLE_KIT_AGENT_FILE` — the always-loaded agent file
  `bin/install-lifecycle.sh` writes the registration block into and
  `check-lifecycle-registration` reads it back from; default `CLAUDE.md`
  (the `DOCTRINE_KIT_AGENT_FILE` sibling).
- `LIFECYCLE_KIT_QUEUE_FILE` / `LIFECYCLE_KIT_STATE_FILE` — the governed header and
  stamp files, defaulting through gate-sdk's `GATE_SDK_QUEUE_FILE` /
  `GATE_SDK_WORKFLOW_DIR`.
- `LIFECYCLE_KIT_SESSION_ID` — the harness-neutral stamp-id override, source 1
  of the derivation order (§bin/session-id.sh); default unset.
- `LIFECYCLE_KIT_LESSON_EVIDENCE_FILE` — the kit-owned lesson-disposition stamp
  file; default `${GATE_SDK_WORKFLOW_DIR:-.workflow}/lesson-evidence.txt`,
  read by `check-lesson-disposition` and the boundary-reset built-in.
- `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` — extra files reset to their header at the
  iteration boundary; default empty.
- `LIFECYCLE_KIT_ENTRY_PREFLIGHT` — per-stage `<stage>=<command>` entries run
  alongside the built-in pre-flight (§bin/enter-stage.sh); default empty.
- `LIFECYCLE_KIT_SHIM_NGRAM` — the shared-n-gram width `check-shim-restatement`
  trips at (positive integer; §check-shim-restatement); default `9`.
- `LIFECYCLE_KIT_SHIM_DEDUP_CORPUS` — that gate's corpus file list; default
  empty for the computed `CLAUDE.md`-plus-kit-templates default.

## Per-component contracts

### lib/stages.sh

The sourced config loader: consumer config first, defaults fill what it left
unset (an explicitly empty value disables a knob where the contract says so),
then validation. Also owns the shared header adapters
(`lifecycle_header`, `lifecycle_header_iter`, `lifecycle_header_stage`,
`lifecycle_stage_known`) — both gates must parse the header identically, and
a shared adapter removes that drift axis — and `lifecycle_registration_block`,
which renders the resident registration block (§bin/install-lifecycle.sh) from the
live config so `bin/install-lifecycle.sh` and `check-lifecycle-registration`
derive one text and cannot drift. Values and adapters only, never
gate structure (gate-sdk's `lib/gate.sh` rule).

### bin/session-id.sh

Prints the canonical stamp id so a stage skill reads it rather than guessing.
The id derives by a fixed source order, first hit wins, every source ending in
the same normalization — strip a leading `agent-` token if present, then take
the first 8 characters:

1. `LIFECYCLE_KIT_SESSION_ID` — a harness-neutral consumer override: a consumer
   whose harness exposes a session identity by any means wires it here.
2. `CLAUDE_CODE_SESSION_ID` — the shipped default source, harness-specific by
   nature: this harness exports the current session's transcript uuid into every
   Bash environment, identifying the session directly rather than inferring it
   from file mtimes. Taken directly here only when `CLAUDE_CODE_CHILD_SESSION`
   is unset; when the flag is set a lead-dispatched stage session
   (§templates/lead.md) may see the *lead's* uuid here, so source 3 verifies
   the flag before trusting it and can route back to this uuid.
3. The newest transcript under the sessions dir (default
   `<config-home>/projects/<cwd-slug>` — `$CLAUDE_CONFIG_DIR` or `~/.claude`,
   and the cwd with every non-alphanumeric char mapped to `-`; override
   `LIFECYCLE_KIT_SESSIONS_DIR`), the top-level glob widened with
   `<dir>/*/subagents/*.jsonl` so a dispatched session with neither env var
   still resolves without a per-dispatch override. Newest-file selection is the
   documented single-operator assumption (one live session per project tree). A
   dispatched child (source 2 skipped, `CLAUDE_CODE_SESSION_ID` carrying the
   lead's uuid) narrows this scan to `<dir>/<lead-uuid>/subagents/*.jsonl`
   alone, excluding the lead's own top-level transcript — concurrently written,
   and able to out-mtime the dispatched session's — from the candidate set. The
   flag is verified, not trusted, because this harness sets it in top-level
   sessions too: a non-empty narrowed scan is a genuine child (newest subagent
   transcript wins); an empty scan with `<dir>/<lead-uuid>.jsonl` present marks
   the flag spurious — a genuine child's transcript lives under `subagents/`
   while it runs, so an empty scan plus a top-level transcript for the env uuid
   means the uuid names a live top-level session, and the derivation falls back
   to `CLAUDE_CODE_SESSION_ID` (source 2's answer). An empty scan with no such
   top-level transcript exits 2 — only a wrong sessions dir or a broken layout
   still reaches it. Two races are accepted: a genuine child stamping before
   its transcript's first write would fall back to the lead's uuid (theoretical
   — a child's transcript has its first writes by the time it can run a tool
   call), and a spurious-flagged session that dispatched subagents earlier in
   the same session stamps the newest subagent's id (a provenance smudge, not a
   correctness break, unchanged from the prior trusting behavior). An absent dir
   or transcript exits 2.

Not a gate — a `bin/` helper invoked (now internally, by `enter-stage.sh`) for
the `<session-id>` field; the stage skills reach it through `enter-stage.sh`
rather than calling it directly.

### bin/enter-stage.sh

The deterministic writer for a stage transition: `enter-stage.sh <stage>`
appends the invocation stamp and flips the `[stage:]` field in one invocation
(honoring the flip+stamp-ride-together protocol), reading `session-id.sh` for
the id — never an argument, so the no-hand-picking rule rides into the tool.
`<stage>` must be a configured stage; anything else is a usage error (exit 2).
An ordinary stage stamps the iteration under the entered stage and swaps only
the header's `[stage:]` token; the first stage (`LIFECYCLE_KIT_FIRST_STAGE`)
performs the iteration-boundary reset instead — truncating the state file and
every file in `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` back to its contract header and
restarting the header at the unnamed-iteration form. `LIFECYCLE_KIT_BOUNDARY_TRUNCATE`
is a generic per-iteration reset knob — no consumer surface is named in
the kit; a downstream kit whose per-iteration file must start each cycle from
its contract header adds itself here, as evidence-kit's manifest does. The
kit-owned `LIFECYCLE_KIT_LESSON_EVIDENCE_FILE` resets by the same rule as a
**built-in member** — the kit owns that surface, so it does not ride the
consumer knob (git history keeps the retired stamps). The boundary entry also
**refuses outright when `## Lessons Learned` is non-empty** (exit 1, the
untriaged entries printed, nothing written — the same refusal contract as the
built-in pre-flight): an untriaged lesson must not cross into the next
iteration, so no `[attend]` injection (queue-kit §bin/queue-index.sh) can
outlive the iteration that filed it. **Pre-flight,
not enforcement:** before writing, it runs the built-in `check-stage-entry`
for the entered stage plus each `LIFECYCLE_KIT_ENTRY_PREFLIGHT` command whose
stage key matches, each reading the not-yet-written flip off a header-flipped
temp queue under `${GATE_SDK_TMP_DIR}` beside the real state file, and refuses
(exit 1, findings printed, no writes) when any is red; the refusal is advisory
in the same sense the gate is at commit time (no `--force`, so the easy path is
the compliant one). `LIFECYCLE_KIT_ENTRY_PREFLIGHT` is a generic per-stage hook
— no consumer surface is named in the kit; a downstream kit whose gate is the
real precondition for a stage wires itself here (as evidence-kit's manifest gate
does for close entry), turning a would-be pre-commit deadlock into a loud
refusal at the flip. **Idempotent:** if the
state file already ends with a stamp for the same `<iteration> <stage> <id>`,
it reports and exits 0 without appending, so a crashed-and-resumed session
re-runs its entry step safely. It reads the `lib/stages.sh` knobs
(`LIFECYCLE_KIT_QUEUE_FILE`, `LIFECYCLE_KIT_STATE_FILE`, `LIFECYCLE_KIT_STAGES`,
`LIFECYCLE_KIT_FIRST_STAGE`, `LIFECYCLE_KIT_BOUNDARY_TRUNCATE`,
`LIFECYCLE_KIT_LESSON_EVIDENCE_FILE`, and `LIFECYCLE_KIT_ENTRY_PREFLIGHT`). Advisory tooling,
not a gate: no fixture pair is owed; it is exercised end-to-end in
`smoke/install.sh`.

### bin/install-lifecycle.sh

`bin/install-lifecycle.sh [agent-file]` writes the resident registration block
into the always-loaded agent file (`LIFECYCLE_KIT_AGENT_FILE`, default
`CLAUDE.md`; the positional override points a smoke or fixture at a scratch
tree without touching consumer config), idempotently. The block is bounded by
fixed marker lines (`<!-- lifecycle-kit:begin -->` … `<!-- lifecycle-kit:end -->`);
a run replaces the content between the markers when present and appends the
block when absent, so re-running never duplicates. A begin marker without its
end is a malformed target (exit 2, rather than guess the bounds); the agent
file must already exist — the installer edits an always-loaded file, it does
not mint one — so a missing target is exit 2. The marker insert/replace itself
is not the installer's code: it rides gate-sdk's shared `lib/inject.sh`
helper (`inject_marker_block`), the one copy `install-doctrine.sh` also uses,
so no second replace path exists to drift.

The block is pointer-only, its roster derived: `lib/stages.sh`'s
`lifecycle_registration_block` renders the one line that the repo runs the
state machine on `LIFECYCLE_KIT_QUEUE_FILE`, the stage roster as skill
invocations (`/<stage>` for each `LIFECYCLE_KIT_STAGES` member), and the
markdown link to the kit SPEC — never stage prose, and never a hand-listed
roster, so a consumer's reshaped stage set flows into the block by
construction. The installer and `check-lifecycle-registration` share that one
renderer, so the emitted block and the block the gate certifies cannot
diverge. Advisory tooling, not a gate: no fixture pair is owed; it is
exercised end-to-end in `smoke/install.sh`.

### check-lifecycle-registration

Invariant: the configured agent file (`LIFECYCLE_KIT_AGENT_FILE`) carries a
lifecycle-kit marker block whose inner content byte-matches the block
regenerated from the live stage machine (`lifecycle_registration_block`). The
block is derived from the machine, and a reshaped machine — a renamed or
reordered stage, a relocated queue file — or a hand-edit stales it *by
construction*, on the exact path the kit advertises (reshape the config,
re-run the installer): the drift-prone-generated-surface case where a gate is
owed (the enforcement-first weighing). The freshness posture is
`check-doctrine-registration`'s, byte-strict like `check-docs-mirror-fresh`.

A missing block is a finding with the install remedy; a block present but out
of lockstep is a finding printing the diff and the regenerate remedy (both
exit 1). Resolution fails closed: a missing agent file, a begin marker without
its end, or an errored awk capture is exit 2 — a half-written or unreadable
target must not pass as clean. The gate satisfies the four gate-sdk contracts
(gate-sdk/SPEC.md §The gate model): the single `LIFECYCLE-REGISTRATION: clean`
line and a `help:` remedy on each finding path (output); exit 2 on an
unreadable target (fail-closed); a `good/`+`bad/` fixture pair under
`gate-tests/` (byte-lockstep-clean and stale-block) plus a sibling `*.test.sh`
for the block-absent, unpaired-marker, and agent-missing cases the one-pair
harness cannot hold (fixture-pair); and registration in this repo's
`gates.list` where its own always-loaded file is the scan target (self-lint).
Positional form `check-lifecycle-registration.sh [agent-file]` points the
fixtures at a synthetic agent file. Its `# graph:` manifest couples the agent
file and `lib/stages.sh` — the config that feeds the block — so an edit to
either re-fires the gate.

### check-stage-evidence

Invariant: the current iteration's stage has a matching skill-invocation
stamp in the evidence file. Each stage skill appends a stamp as its first
step; the gate asserts the header's current `<name> [stage: <stage>]` pair is
covered — so advancing the stage header without running the skill fails
closed. The stamp file is additionally kept provably bounded: every data line
must be grammatically well-formed — exactly four fields, stage ∈ the
configured stage set plus the waiver token (a stamp token but never a header
stage), date `YYYY-MM-DD`; every data line's iteration must be the current
one, stale lines from a prior iteration are rejected; and the `—`
unnamed-iteration sentinel may appear only while the header itself is unnamed.
It also reads the `<session-id>` field (which it once ignored) for one
cross-stage invariant: within the current iteration, two *different* stages
may not share one session id — a stage flip is a context boundary and demands
a fresh session, so a duplicate (e.g. build == validate) is a self-reported
skip and fails. Same-stage re-entries (a multi-session build) may share or
rotate ids freely, and waiver-token stamps are exempt (never a stage, so never
in the map).

Calibration: the `—` sentinel is the bootstrap name for a new iteration
before the first stage names it. Any stage past the first carrying `—` in the
header is rejected (the header-name guard); admitting `—` at every stage — an
attested bug — would let an unnamed iteration reach the final stage
undetected, so the allowance is stage-scoped, not global. The data section
begins after the first `---` separator; prose above it is not validated
line-by-line. Argument mode `$1 $2` (queue, state) with configured defaults
makes the gate fixture-capable; the sentinel-scoping interplay that exceeds
one good/bad pair is covered by `gate-tests/check-stage-evidence.test.sh`.

Honest limit: the stamp proves the stage skill was *invoked*, never that it
produced its green result — a validate stamp says validate ran, not that the
suites passed. That gap is closed by evidence-kit, which commits a per-run
evidence manifest (a suite verdict per line) and, via the optional
`LIFECYCLE_KIT_BOUNDARY_TRUNCATE` integration, couples a `[stage: close]` entry to
the full green block.

### check-stage-entry

Invariant: the stage being *entered* re-verifies its prior stage's static
exit, extending the invocation-stamp floor `check-stage-evidence` provides
for the current stage one hop back (a shared surface, a distinct invariant:
*current-stage invoked + file grammar* there, *prior-stage invoked +
entered-stage static exit* here). It owns three assertions, (A)
prerequisite-stamp ordering — for header `[stage: X]` the file carries a
stamp for X's configured mandatory predecessor, which closes the "flipped
straight to the last stage with no prior stamp" hole its sibling — asserting
only the *current* stage's stamp — cannot see; (B) drain-entry queue-empty —
a drain-stage header requires the configured active queue sections to carry
no top-level `- ` entry, catching entry-on-incomplete-build; and (C)
audit-trigger — an audit-entry-stage header carrying a cross-component
amendment signal but no `<iter> <audit-stage>` stamp demands either that
stamp or an explicit recorded waiver line, mechanizing the audit stage's
self-reported cross-component trigger so a missed trigger cannot silently
skip the audit. The signal reads the on-disk amendment tree (cwd-relative,
gate-sdk prune set applied, and `templates/` paths excluded — a shipped
`SPEC-amendment.md` skeleton is a copyable stub, not a live amendment, the
same exclusion canon-kit's finders apply): it fires when amendment files span
≥2 component dirs, OR when a single amendment's component set — its own dir ∪
the contract-surface tokens in its body that resolve to a roster dir — is ≥2.
The
waiver rides the same file the stamps do (auditable) and is written only on
an explicit user ruling — never self-issued by the entering session; it
satisfies only assertion C (assertion A's predecessor scan matches the audit
stage exactly, so a waiver is never read as an audit *stamp*).

Calibration: the predecessor map deliberately omits the trigger-gated audit
stage as anyone's predecessor — demanding an audit stamp before every build
would false-fire on an iteration that legitimately skipped it; the
build→align re-check when align *did* run is the build skill's step-0
procedural precondition, not this gate. Assertion C's honest limit: it
approximates "changes ≥2 components' *contracts*" with "touches or names ≥2
component surfaces" — it can over-demand (the cheap waiver valve absorbs
that) and can under-detect a purely semantic cross-component impact; it
converts a silent skip into a stamp, a recorded waiver, or a narrow
false-negative, strictly better than self-report. The good/bad pair covers
assertion A; `gate-tests/check-stage-entry.test.sh` covers B and C over five
sandbox scenarios (two-dir amendments ±waiver, a single-amendment
cross-component body, a single-component amendment, and a `templates/` stub
that must not fabricate a second component). Suite *runs* and other
non-static exits are not re-runnable as pre-commit gates and stay
human-judged at the stage approval; the prerequisite-stamp floor is their
mechanical residual.

### check-stage-skill-coverage

Invariant: the configured stage set and the skills dir (`LIFECYCLE_KIT_SKILLS_DIR`,
default `.claude/commands`; override with the first argument) cover each other,
both directions. Forward: every `LIFECYCLE_KIT_STAGES` member has a `<stage>.md`
skill file — a stage with no skill cannot be entered. Reverse: every skill file
that invokes `enter-stage.sh` names a live stage in the token it passes. The
`enter-stage.sh` invocation is the mechanical marker separating a stage skill
from an ordinary one, so a retired stage's orphan skill (its `.md` still
invoking a now-unknown stage) reddens without false-flagging a non-stage skill
like `/agent-execution`, which never invokes `enter-stage.sh`. A skills dir that
does not exist is fail-closed (exit 2). The `# graph:` couples the skills dir at
`tier=precommit`; the whole-tree `run-gates.sh` battery backstops a stage-set
edit (`lifecycle-config.sh`), which is not itself in the coupled surface.

### check-skill-binding

Invariant: every skill under `LIFECYCLE_KIT_SKILLS_DIR` (default `.claude/commands`;
override with the first argument) that carries a binding directive — `Execute
the template at <path>, applying the bindings below.` — (a) names a template
file that exists and (b) binds exactly that template's slot set: an unbound slot
is red, an orphan binding naming no slot is red. A skill with no directive is
not read, so a copy-and-specialize skill carrying no such line is untouched —
the same directive-as-selector mechanism `check-stage-skill-coverage` uses on
`enter-stage.sh`. A bound skill need not be a stage skill: `/agent-execution`
binds a delegation-kit template, which the gate accepts unchanged (the resolved
template path may point at any kit). Template slots are the `*<slot-name: …>*` opening
tokens; a shim's bindings are the `**slot-name** —` lead lines under
`## Bindings`; the directive's template path resolves relative to the current
directory (the tree root at pre-commit). A skills dir that does not exist is
fail-closed (exit 2). The `# graph:` couples the skills dir, the lifecycle
templates dir, and each out-of-tree bound template (e.g.
`delegation-kit/templates/agent-execution.md`) at `tier=precommit`, so a slot
added to a template or a binding changed in a shim fires the gate. The good/bad pair drives the unbound-slot case;
`gate-tests/check-skill-binding.test.sh` covers the orphan-binding,
missing-template, and skip (no-directive / no-slots) cases the one pair cannot.

### check-shim-restatement

Invariant: no binding shim under `LIFECYCLE_KIT_SKILLS_DIR` shares a normalized word
n-gram of length ≥ `LIFECYCLE_KIT_SHIM_NGRAM` with any surface in the dedup corpus
`LIFECYCLE_KIT_SHIM_DEDUP_CORPUS` — the duplication tripwire under the same
directive-as-selector rule `check-skill-binding` uses (a file with no `Execute
the template …` directive is not a shim and is not read). The corpus defaults to
the consumer's always-loaded agent file (`LIFECYCLE_KIT_AGENT_FILE`, default
`CLAUDE.md`) plus every kit's `templates/**/*.md`
(kit set from `gate_kit_roots`); an explicit `LIFECYCLE_KIT_SHIM_DEDUP_CORPUS` or
positional corpus arguments override it — the latter is the hermetic-fixture
affordance. Comparison normalizes first — lowercase, punctuation stripped to a
word boundary, whitespace collapsed — so cosmetic rewording does not evade the
tripwire; a resolved corpus that yields no n-grams is fail-closed (exit 2), never
a false clean.

`LIFECYCLE_KIT_SHIM_NGRAM` is calibrated to the smallest window with zero false
positives on the post-rewrite corpus, with a floor of 8 words so a citation line
(a path plus a §heading) never fires — this repo's default is 9, the width at
which the 8-word §heading `This repo is governed by its own kits` stops
tripping. Honest limit: the n-gram holds the *copy shape* only. Which tier a
fact belongs to stays semantic judgment — a paraphrase below N words passes the
gate and is still a defect to fix on sight (the same doctrine as
check-comment-tier's floor). The `# graph:` couples the skills dir, `CLAUDE.md`,
and the kit template dirs at `tier=precommit`, so editing a shim or a corpus
surface fires the gate. Because the corpus find recurses but the `kit:templates/*.md`
couple does not, the one kit template *sub*directory that holds bound
templates — `lifecycle-kit/templates/skills/*.md` (the stage-skill templates each
stage shim binds) — is coupled explicitly beside it; a shim's own template is its
likeliest collision surface, so it must re-trigger the gate. A red run names the
shim, the corpus surface, and the shared n-gram, so the fix (delete the
restatement, keep a citation) is mechanical. The good/bad pair drives the plain restatement/clean split;
`gate-tests/check-shim-restatement.test.sh` covers the no-directive skip, the
short-corpus fail-closed, and the below-N paraphrase the one pair cannot.

### check-lesson-disposition

Invariant: every `## Lessons Learned` entry present at HEAD and absent from the
worktree leaves a well-formed disposition stamp in
`LIFECYCLE_KIT_LESSON_EVIDENCE_FILE` — a lesson cannot be cleared without a
recorded rule/task/harvest/discard call. The evidence home is a stamped file,
not the commit body, because the battery runs at pre-commit when no commit
message exists yet, so only a file is mechanically decidable (the
`check-stage-evidence` fail-closed precedent). Each data line is
`<iteration> lesson <rule <file> | task <slug> | harvest <tag> | discard
<reason>> — <lead-line prefix>`; the ` — ` separates the disposition from the
lead-line prefix that joins it to the removed entry (a stored prefix matches a
removed entry when it is a leading substring of that entry's normalized lead
line). Both grammar (each line well-formed, a known disposition kind) and
per-entry matching (every removal has a stamp) hold; an unreadable evidence
surface is fail-closed (exit 2). Shape validation only: `harvest <tag>` is not
checked against `QUEUE_KIT_LESSON_TAGS` — that would cross-couple the kits'
configs; the close skill and `check-tag-lead-line` hold the vocabulary.

Calibration: diffing HEAD against the worktree is fixture-hostile (a committed
fixture has HEAD == worktree, so the removal case has no static representation —
the `check-task-conservation` precedent), so the gate takes optional
`[queue-head] [queue-worktree] [evidence-file]` override args and its good/bad
pair drives all three hermetically (the `check-trajectory-fresh` synthetic-args
precedent); `gate-tests/check-lesson-disposition.test.sh` covers the malformed
grammar, still-present-not-removed, and prefix-join cases the one pair cannot.
The `# graph:` couples the queue file and the evidence file at
`tier=precommit`.

### templates/skills/

The stage-skill templates (`scope`/`align`/`build`/`validate`/`close`) carry
the generic stage spine — the flip+stamp first step (performed by invoking
`enter-stage.sh <stage>` and stating in one line what it does), each stage's
trigger/ordering rules, and its stage-local doctrine — with **named slots**
where the consumer's rule content goes. The templates are the owned surface:
this section states the contract a consumer skill must satisfy and never
restates what a template carries.

A consumer skill adopts a template in one of two modes; either way the executed
skill states in one line what the flip+stamp step does and supplies every
slot's content:

- **Consume-by-reference (the default)** — the consumer skill is a thin
  **binding shim** whose body is a single directive line, `Execute the template
  at <repo-relative path>, applying the bindings below.`, followed by a
  `## Bindings` section with exactly one entry per template slot. The template
  stays the executed surface; the shim carries only consumer content, so generic
  doctrine has one owner and never drifts across a copy. This is the documented
  default because it tracks the kit: a re-vendor reaches the template, and
  `check-skill-binding` + `check-shim-restatement` hold the shim to a thin
  reference. This repo dogfoods it (`.claude/commands/*.md`).
- **Copy-and-specialize (the sanctioned fork)** — the template is copied into
  the consumer's skills dir and each slot overwritten in place; self-contained
  and legible, structure copied not imported, so the skill stands alone
  (gate-sdk's check-skeleton shape). It is a fork with its consequence owned:
  you own the ritual prose, an upgrade's re-vendor does not reach it, and the
  shim gates do not cover it. It is kept deliberately — the blessed escape hatch
  that keeps legitimate structural divergence (different stages, a reshaped
  machine) visible and contained, and the harness-agnostic floor the bare-bash
  upgrade smoke assumes; removing it would drive forks into edits of the
  vendored template, which break Phase-A upgrade determinism with no gate to
  catch them.

A consumer reaching for the fork to express *prose* divergence rather than
structural divergence signals the slot vocabulary is too thin; the fix is
richer slots pulling those cases back under shim protection, not more copying.
No gate or telemetry watches for it — which mode a consumer picks is their tree,
not this one.

**Named slots (template grammar).** Each consumer placeholder is a named slot
`*<slot-name: guidance>*` — `slot-name` matches `[a-z][a-z0-9-]*`, is unique
within its template, and precedes the `:` and the guidance a copy-editor or
shim author replaces. A copy-and-specialize consumer overwrites the whole
`*<…>*` span; a shim binds the slot by name in a `## Bindings` entry
`**slot-name** — <consumer content>` (multi-line content indents under its lead
line), and carries nothing else — doctrine restated from the template in a shim
is the defect the reference mode removes. `check-skill-binding` holds the
shim↔template slot parity.

**Authoring rule (a binding shim binds residue, cites procedure, restates
nothing).** A binding a slot supplies carries only what is local to this
consumer — the residue: which surfaces to sweep, which config knobs, which log
sinks. Procedure and always-loaded fact that a kit template or the consumer's
`CLAUDE.md` already owns are named by a citation (a path plus a §heading), never
copied into the shim: a shim is loaded on every stage invocation, so a
restatement there is a per-session token tax on a fact with an owner, and it
drifts the moment the owner changes. `check-shim-restatement` is the tripwire
for the copy shape; the tier judgment (residue vs owned fact) stays the author's.

Beside the stage skills sits `release-sweep.md` — a **boundary skill**, not a
stage: it invokes no `enter-stage.sh` and stamps no state, so
`check-stage-skill-coverage` never reads it (it governs only the configured
stage set). It is the deprecation disposition walk at a major, forcing every
marker on the `CANON_KIT_DEPRECATION_MARKERS` roster to a stamped disposition —
decommission, carry-forward, or un-deprecate — the `check-lesson-disposition`
contract shape at a release boundary. canon-kit's `check-deprecation-task` holds
each marker bound to a live task between majors; this sweep forces the standing
inventory to a decision at the boundary the deprecations were promised against.
The stamp file is operator evidence riding the release commit — the kit wires no
gate over it (a consumer may), the same `<…>`-placeholder copy-edit shape.

### templates/lead.md

The **iteration lead** template — an optional live session that dispatches an
iteration's stage sessions and answers their escalations, closing the
restart-cost of a stage that would otherwise stop and surface to the user cold
(§The state machine). Like `release-sweep.md` it is a **boundary skill, not a
stage**: it invokes no `enter-stage.sh` and joins no stage set, so
`check-stage-skill-coverage` never reads it. Unlike release-sweep it carries
named slots, so it adopts the binding-shim grammar (§templates/skills/) — a
consumer copies-and-specializes it or binds it through a thin shim, and
`check-skill-binding` holds the slot pairing either way (this repo's
`.claude/commands/lead.md` shim).

The template owns the orchestration protocol whole: the lead model (dispatch a
stage session as a background agent whose prompt is that stage's ordinary skill
invocation), the four-header escalation block (Question / Options /
Recommendation / Evidence), the split-channel design (routine narration to the
resume journal, escalations to the message channel), the compact economics —
the handoff compact plus operator-suggested compacts at the acceptance
boundaries that pay under the cold-wakes-times-compressible-residue rule —
with the dispatch-granularity rule (batch units sharing a kit or SPEC
surface, split on a model-tier change or a delegation-kit split trigger), and
the stamps-authoritative invariant carried from §The state machine as the
design's load-bearing rule. Dispatch safety is not re-owned — it inherits
delegation-kit's protocol by citation (delegation-kit/SPEC.md §The delegation
model: background dispatch, the per-dispatch budget guard, validate after any
agent commit). Consumer residue stays in named slots — the tracked
agent-definition carrying the standing dispatch policy the dispatch names (the
ruling-class roster and everything else true of every dispatch, not improvised
per prompt), and whether the consumer wires the optional escalation-shape guard
(guard-kit/SPEC.md §wakeup-guard) or leaves it inert.
