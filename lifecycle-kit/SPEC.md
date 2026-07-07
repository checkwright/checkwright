# lifecycle-kit — an evidence-stamped iteration lifecycle for stateless agent sessions

An iteration is a self-contained work cycle driven through a configurable
sequence of stages (default: `scope → [align] → build → validate → close`).
The problem the kit solves: a stateless agent session cannot be trusted to
remember, or even re-read, a process document — so the process state lives in
two governed files a gate can read, and every stage transition leaves
machine-checkable evidence.

Extracted from the governance meta-layer of a private production platform.
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
one — an operator who intends to override runs the stamp+flip by hand, exactly
as before the tool existed. Committing the flip+stamp remains the skill's
business.

The first stage is the iteration boundary: `enter-stage.sh` *truncates* the
evidence file back to its header (git history is the permanent audit trail;
the gates only read the current iteration) and stamps under the
unnamed-iteration sentinel `—`, rewritten to the real name when the stage
names the iteration. Later stages only append.

**Honest limit:** a stamp proves the stage skill was *invoked*, not that its
work was done faithfully — strictly better than skip-and-no-trace, but not
proof of done. The `<session-id>` field is **read, not hand-picked**:
`bin/session-id.sh` prints the canonical id — the first 8 hex of the newest
agent-session transcript under the sessions dir, which rotates per session
(including across a context clear) — and the stage skills stamp exactly what
it prints, so each stage's provenance is observed, not guessed.
`check-stage-evidence`'s invocation floor keys on `<iteration> <stage>`; it
additionally reads the session id to enforce cross-stage distinctness (a stage
flip must carry a fresh session — see §check-stage-evidence).

## Layout and configuration

The kit is vendored beside gate-sdk (conventionally at `lifecycle-kit/`); its
gates are registered in the consumer's `gates.list` by name and resolve
through gate-sdk's multi-kit path (consumer gates dir first, then each kit's
`checks/`). Stage skills are copy-edits of `templates/skills/*.md` into the
consumer's agent-skill directory; the `<…>` placeholders hold the consumer's
ritual content.

The stage machine itself is config with the platform's lifecycle as the
default: copy `templates/lifecycle-stages.sh` into the gates dir as
`lifecycle-stages.sh` (or point `LIFECYCLE_KIT_STAGES_FILE` elsewhere) and
override any knob — stages, predecessor map, first/drain/audit stages, active
queue sections, waiver token, amendment/roster shapes, governed-file paths
(`LIFECYCLE_QUEUE_FILE` / `LIFECYCLE_STATE_FILE`, defaulting through
gate-sdk's `GATE_SDK_QUEUE_FILE` / `GATE_SDK_WORKFLOW_DIR`). Knob semantics
are documented in the template; the loader validates the machine (unknown
stages in the map, a waiver token colliding with a stage name) and exits 2 on
a malformed config — a broken machine must not gate anything.

## Per-component contracts

### lib/stages.sh

The sourced config loader: consumer config first, defaults fill what it left
unset (an explicitly empty value disables a knob where the contract says so),
then validation. Also owns the shared header adapters
(`lifecycle_header`, `lifecycle_header_iter`, `lifecycle_header_stage`,
`lifecycle_stage_known`) — both gates must parse the header identically, and
a shared adapter removes that drift axis. Values and adapters only, never
gate structure (gate-sdk's `lib/gate.sh` rule).

### bin/session-id.sh

Prints the canonical stamp id so a stage skill reads it rather than guessing:
the first 8 hex of the most recently written transcript under the sessions dir
(default `<config-home>/projects/<cwd-slug>` — `$CLAUDE_CONFIG_DIR` or
`~/.claude`, and the cwd with every non-alphanumeric char mapped to `-`;
override `LIFECYCLE_SESSIONS_DIR`). Newest-file selection is the documented
single-operator assumption (one live session per project tree); an absent dir
or transcript exits 2. Not a gate — a `bin/` helper invoked (now internally,
by `enter-stage.sh`) for the `<session-id>` field; the stage skills reach it
through `enter-stage.sh` rather than calling it directly.

### bin/enter-stage.sh

The deterministic writer for a stage transition: `enter-stage.sh <stage>`
appends the invocation stamp and flips the `[stage:]` field in one invocation
(honoring the flip+stamp-ride-together protocol), reading `session-id.sh` for
the id — never an argument, so the no-hand-picking rule rides into the tool.
`<stage>` must be a configured stage; anything else is a usage error (exit 2).
An ordinary stage reads the iteration from the header, appends
`<iteration> <stage> <id> <date>`, and swaps only the header's `[stage:]`
token; the first stage (`LIFECYCLE_FIRST_STAGE`) performs the
iteration-boundary reset instead — truncate the state file to its header,
stamp under `—`, set the header to the unnamed-iteration form. **Pre-flight,
not enforcement:** before writing, it runs `check-stage-entry` for the entered
stage — a header-flipped temp queue under `${GATE_SDK_TMP_DIR}` plus the real
state file, through the gate's existing positionals, the gate itself
untouched — and refuses (exit 1, findings printed, no writes) when it is red;
the refusal is advisory in the same sense the gate is at commit time (no
`--force`, so the easy path is the compliant one). **Idempotent:** if the
state file already ends with a stamp for the same `<iteration> <stage> <id>`,
it reports and exits 0 without appending, so a crashed-and-resumed session
re-runs its entry step safely. No new config — it reads the existing
`lib/stages.sh` knobs (`LIFECYCLE_QUEUE_FILE`, `LIFECYCLE_STATE_FILE`,
`LIFECYCLE_STAGES`, `LIFECYCLE_FIRST_STAGE`). Advisory tooling, not a gate: no
fixture pair is owed; it is exercised end-to-end in `smoke/install.sh`.

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
header is rejected (the header-name guard); admitting `—` at every stage — the
original platform bug — would let an unnamed iteration reach the final stage
undetected, so the allowance is stage-scoped, not global. The data section
begins after the first `---` separator; prose above it is not validated
line-by-line. Argument mode `$1 $2` (queue, state) with configured defaults
makes the gate fixture-capable; the sentinel-scoping interplay that exceeds
one good/bad pair is covered by `gate-tests/check-stage-evidence.test.sh`.

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
same exclusion spec-kit's finders apply): it fires when amendment files span
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

### templates/skills/

The stage-skill templates (`scope`/`align`/`build`/`validate`/`close`)
— copy-edits into the consumer's skill directory, like gate-sdk's
check-skeleton. Each carries the generic spine (the flip+stamp first step,
performed by invoking `enter-stage.sh <stage>` and stating in one line what it
does; scope's truncate-and-bootstrap protocol; align's trigger-gating and waiver
rule; build's step-0 audit recheck and stamp-per-session/flip-once rule;
validate's baseline-diff discipline; close's disposition-per-lesson rule)
with `<…>` placeholders where the consumer's rule content goes. Structure is
copied, not imported, so a consumer's skills stay legible and self-contained.
