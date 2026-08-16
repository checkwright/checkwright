# evidence-kit — a held-constant baseline and a committed per-run evidence manifest for validate

lifecycle-kit's stage evidence proves a stage was *invoked*; it cannot prove
the stage produced its green result. evidence-kit closes that gap with three
coupled surfaces — a held-constant test baseline, a committed per-run evidence
manifest, and a codified run contract — so a validate stamp is backed by a
recorded, hashable verdict rather than a claim.

The kit is a **new kit**, not a lifecycle-kit extension, because the evidence
manifest is a wire contract a future external verifier consumes: its format is
versioned, stable, and hashable independent of the state machine, and the kit
is adoptable by a consumer that runs no iteration lifecycle at all.
lifecycle-kit integration is optional and arrives through one generic knob on
its side of the seam (§lifecycle-kit integration).

## Layout and configuration

The kit is vendored beside [gate-sdk](../gate-sdk/) (required); its gates
register in the consumer's `gates.list` by name and resolve through gate-sdk's
multi-kit path. Config follows the established kit pattern: copy
`templates/evidence-config.sh` into the gates dir as `evidence-config.sh` (or
point `EVIDENCE_KIT_CONFIG_FILE` elsewhere) and override any knob; the loader
fills every unset knob with a default, then validates and exits 2 on a malformed
machine (a suite name that is not a valid variable suffix) — a broken config
gates nothing.

Knobs, this repo's surface names as defaults:

- `EVIDENCE_KIT_SUITES` — the ordered suite names.
- `EVIDENCE_KIT_RUN_<suite>` — the command that runs a suite (captured to a log
  under `EVIDENCE_KIT_TMP_DIR`, default gate-sdk's `.tmp`).
- `EVIDENCE_KIT_PARSER` — a parser adapter name or a consumer command mapping a
  captured log to `<scenario> <pass|fail|ignore>` lines; default `exit-code`.
- `EVIDENCE_KIT_PARSER_<suite>` — a per-suite parser override with the same
  value grammar; default unset, an unset suite falling through to the global
  knob. The name mirrors the `EVIDENCE_KIT_RUN_<suite>` convention. Suite
  granularity is a floor, not a ceiling: a suite whose runner reports per-case
  results carries a parser that says so, while its siblings keep the global
  adapter. This repo dogfoods it on the `gates` suite —
  `scripts/parse-gates-log.sh` maps the verbose `run-gates` log to one scenario
  per registered gate, so an existing gate turning red diffs as a new failure
  even while a sibling gate is legitimately held red; the whole-battery
  `exit-code` scenario could not tell those apart.
- `EVIDENCE_KIT_SCENARIO_GLOBS` — optional per-suite globs; configuring one
  arms the manifest↔disk set-equality assertion for that suite.
- `EVIDENCE_KIT_BASELINE_FILE` (default `.workflow/validate-baseline.txt`),
  `EVIDENCE_KIT_MANIFEST_FILE` (default `.workflow/validate-evidence.txt`),
  `EVIDENCE_KIT_SKIP_FILE` (default `.workflow/validate-skips.txt`).
- `EVIDENCE_KIT_LOCK_FILE` — the producer-liveness lock (§The producer-liveness
  lock), default `run-validate.lock` under `EVIDENCE_KIT_TMP_DIR`. It resolves
  *through* the scratch knob rather than beside it, so a consumer that moves the
  scratch dir moves the lock with it and never has to keep two paths in step.
- `EVIDENCE_KIT_QUEUE_FILE` / `EVIDENCE_KIT_STATE_FILE` — the lifecycle surfaces
  read for the manifest's optional close-entry and stamp-coupling assertions;
  they default through gate-sdk's `GATE_SDK_QUEUE_FILE` / `GATE_SDK_WORKFLOW_DIR`.
- `EVIDENCE_KIT_RUN_ID` — the evidence-line key when no lifecycle queue header
  names the iteration.
- `EVIDENCE_KIT_PRE_HOOK` — an optional per-suite pre-run command (projection
  regen, container teardown) kept on the consumer side of the spine.
- `EVIDENCE_KIT_PERMANENT_SLUGS` — blocking slugs that satisfy baseline liveness
  without a live queue task.
- `EVIDENCE_KIT_RUNNER_DOC` (default `README.md`, resolved against the git
  toplevel) — the doc whose battery-roster block `check-battery-roster` holds
  against the suite roster. It is gate-local: nothing in the validate run path
  reads it, so the loader fills no default for it and the gate carries its own.
  The name mirrors gate-sdk's `GATE_SDK_RUNNER_DOC` deliberately — in a tree
  vendoring both kits the two name the same physical doc for two different
  assertions, and a reader who has met one should not have to learn a second
  vocabulary for the other.

## Per-component contracts

### lib/evidence.sh

The sourced config loader: consumer config first, kit defaults fill what it
left unset, then validation. It also owns the shared adapters — `ek_suite_cmd`
(a suite's configured run command, `EVIDENCE_KIT_RUN_<suite>`), `ek_parser_for`
(the per-suite parser resolution, detailed below), `ek_parse` (the
parser dispatch), `ek_diff` (the per-scenario baseline diff, §bin/diff-baseline.sh),
`ek_data_lines`, the `ek_pid_alive` / `ek_lock_read` lock adapters
(§The producer-liveness lock), and the self-contained `ek_queue_iteration` / `ek_run_key`
header readers plus the `ek_state_stage` cursor reader that let the kit read
lifecycle state without a lifecycle-kit dependency. The two axes come from two
surfaces: the queue header names the iteration, the state file's **last data
line** is the stage cursor. `ek_state_stage` returns non-zero on both no-cursor
shapes — an absent state file, and a file truncated to its preamble with no data
line yet — so a caller's `|| true` yields an empty stage for either. Values and adapters only, never tool structure. It
sources gate-sdk's `lib/gate.sh` for `fail_closed`, so evidence-kit requires
gate-sdk vendored beside it.

The parser adapters map a captured log — and, for `exit-code`, the suite's exit
status — to `<scenario> <pass|fail|ignore>` lines: `libtest` reads per-test
result lines (a Rust `cargo test` suite), `exit-code` emits one scenario per
suite keyed off the suite command's exit. Any other value is a consumer command
run on the log.

Which adapter a suite gets is resolved by `ek_parser_for <suite>`
(`EVIDENCE_KIT_PARSER_<suite>`, else the global `EVIDENCE_KIT_PARSER`), and the
dispatch lives inside `ek_parse` — so both spine callers, `run-validate` and
`diff-baseline`, inherit per-suite parsing with no edit of their own. The
resolution is a named helper rather than private to `ek_parse` because
`run-validate`'s produced-no-result diagnostic must name the *effective* parser:
naming the global while an override produced the empty result would misreport
exactly the guard the per-gate baseline leans on.

A consumer parser command receives the log path alone — no suite name, no exit
status. That is deliberate: passing suite+status to every consumer command would
add fields most parsers never read (a field with no reader is removed), and a
consumer needing exit-code semantics for a suite simply leaves that suite on the
global adapter.

Neither adapter is a gate, so the library's branches are covered by
`gate-tests/evidence-lib.test.sh`: the per-suite dispatch with its global
fall-through, and `ek_diff`'s absent-from-baseline triple. The triple's `ignore`
edge carries a test of its own — the narrow side is the half a later session
widens by accident, so it is pinned by an assertion rather than by prose alone.

### Baseline manifest

Held-constant, edited by human commit only. It is a tracked checked projection
of the workflow directory, so its first line is the pointer-form header
`# contract: evidence-kit/SPEC.md §Baseline manifest — held-constant validate
baseline: <suite> <scenario> <status> [<slug>]` (the form ruled by
gate-sdk/SPEC.md §The workflow directory, whose em-dash tail carries the line
grammar). Below it, one line per known scenario,
`<suite> <scenario> <status> [<slug>]`. A blocking `<slug>` is required exactly
when status is `fail` or `ignore` and forbidden when `pass`; each slug resolves
to a live queue task (the queue-file knob) or a configured permanent marker.
Tooling never writes it — a promotion (a held-constant red recovering to pass)
is a human commit, which is what keeps the baseline honest.

A scenario absent from the baseline fails closed: the diff treats its failure
as a new failure, so a missing `pass` row loses no enforcement — its cost is
classification (a regression reads as a new scenario), not a silent green.

The rule keys on `fail`, never on non-pass. For a scenario with no baseline row,
observed `fail` is a new failure; observed `pass` is the classification cost
above, no red; observed `ignore` is **silent** — an ignored test is a non-verdict,
with no assertion here to converge on. Widening to non-pass would redden a
libtest consumer's newly-added `#[ignore]` test: a fail-closed appetite for
absent `ignore` is a separate argued change with its own delta to this section,
never a rider. The skip demotion (§bin/diff-baseline.sh) stays a baseline-row
concern and does not reach absent scenarios — a skip-demoted observed `pass`
absent from the baseline falls under the classification-cost rule.

### Evidence manifest

Committed, written once per run. The file header is a `# contract: evidence-manifest v1`
line — the versioned wire format the deferred hosted-attestation service consumes
as its attestation payload. Each data line is
`<iteration> <suite> sha256=<log-hash> pass=<n> fail=<n> ignore=<n>
verdict=<clean|new-failures> <date>`, and a run supersedes that iteration's
prior line for every suite it ran. The captured log
stays uncommitted under the tmp dir; its digest pins which run produced the
counts. The iteration key scopes the line so the boundary-truncate knob can
clear the manifest at the start of the next iteration.

**The spine touches this file only after its last suite has run**, and that is
contract rather than implementation detail. `run-validate` accumulates its rows
under the tmp dir and folds them in as a single write, dropping this iteration's
prior line for each suite the run covered and re-appending the batch in
configured-suite order.

What that buys is a suite free to sit anywhere in the roster even when its own
precondition is a clean worktree. A spine writing per suite dirties the tree
before such a suite's turn, so every full run reddens it for no reason but
roster position — a collision the writer manufactures and no suite can defend
itself against. Pinning it to the front of the roster is mitigation, not a fix:
nothing asserts the position, so a second such suite re-breaks it silently.
Under one fold nothing pins any suite anywhere.

Line order follows the configured roster rather than run history, so a repeat
run rewrites this iteration's rows where they already were instead of relocating
each to the end — a run whose counts and date are unchanged leaves the file
byte-identical, and a concurrent or repeated producer stops surfacing as a diff
with no content behind it. An aborted run writes nothing: it leaves the manifest
as it found it, which is what the abort's own diagnostic already claims. The
partial manifest that used to survive such an abort was never admissible anyway —
§check-evidence-manifest's close-entry assertion wants a clean line for every
configured suite.

The header is a wire-format version marker, not a doc pointer —
gate-sdk/SPEC.md §The workflow directory rules that as one of the two payload
forms a checked projection may carry, and this section is the statement that
form requires. `check-evidence-manifest` owns it (asserts the first line is
`# contract: <version>`). A consumer that also runs canon-kit's
`check-spec-pointer` over its workflow dir whitelists **this** file there
(`CANON_KIT_COMMENT_WHITELIST`), since a version marker resolves as no path; the
baseline is pointer-form and needs no whitelist entry.

### The producer-liveness lock

Uncommitted, under `EVIDENCE_KIT_LOCK_FILE`. A stage stamp proves invocation and
an evidence line proves a green result, but neither can say a producer is *still
running* — a file read at an instant cannot carry that, and the manifest's own
guarantees do not reach it: §check-evidence-manifest's assertion A already
asserts suite-roster completeness at a close cursor, and the spine's single fold
means a torn read is unreachable. The gap is liveness alone, and the lock is the
artifact that closes it.

The record is one line, `pid=<n> run=<key>`, where `<key>` is the evidence-line
key `ek_run_key` yields. Both fields have named readers and nothing else is
carried: a start timestamp was considered and removed, because once the stale
policy is PID-liveness rather than age it has no reader, and a field with no
reader is removed rather than kept for plausibility.

**The lock is held if and only if the recorded PID is alive**, which makes a
leaked lock self-invalidating — the shape this methodology already relies on for
the session-role marker, whose id match means a stale marker self-invalidates.
An **age-based TTL is rejected outright**: a long validate run outlives any
honest TTL, and a long run is precisely the case the lock exists for, so a TTL
tuned short enough to reclaim a crashed run promptly is guaranteed to declare a
healthy long run dead — restoring the false-green the lock removes.

`ek_pid_alive` is the one predicate all three readers share, and its two legs are
a ruling rather than belt-and-braces. `kill -0` is the cheap same-uid answer, but
it conflates *no such process* with *not yours*: against a producer running under
another uid it reports the live process as dead, which is a false **free**
reading — the one direction this design may not take. `ps -p` answers existence
without needing the permission to signal, so it runs as the fallback and any
evidence of existence means held. Reading `/proc` to confirm process *identity*
is rejected separately: it is unportable, and the OS-reach objective makes a
Linux-only predicate a cost rather than a refinement.

**PID reuse is a named, accepted residual.** A recycled PID yields a false
*held* reading, which refuses a stage entry that could have proceeded — a
fail-closed direction costing one file deletion to clear, against a defect that
would otherwise cost the next session an evidence file changing underneath it.
The same direction reaches the writer's own refusal, which reads the identical
predicate: a recycled PID makes `run-validate` over-refuse by the same
mechanism. One cause, one direction, one clearance — a restated instance of the
residual, not a second one to weigh.

**The claim is atomic create-exclusive, never check-then-write**, and the
asserted property is two-part: it succeeds for exactly one producer, *and* the
record publishes whole. The idiom is the one the spine already uses to publish
the manifest — build the record in a temp file under the same scratch dir, then
`ln` it into place, which fails if the target exists. `mkdir` and a `set -C`
redirect are atomic on the first half only: each leaves a window where the lock
exists and its record does not, which every reader would then have to parse
around. Because the record publishes whole, a reader never has to interpret a
partial lock or decide what an empty one means, so an unparseable lock is
corruption and fails closed rather than reading free.

The atomicity is *required* rather than careful, and what makes it so is the
writer-side refusal (§bin/run-validate.sh). An unconditional claim has no
predicate and so no time-of-check window; adding the refusal adds one, and a
naive read-then-claim would let two producers both observe a clear lock and both
claim, the second's record overwriting the first's. That reintroduces exactly
the two-producer case the refusal exists to close, and defeats conditional
release with it — "still ours" cannot be answered from a record another producer
overwrote. The claim's success *is* the check, so there is no interval between
them to lose.

**The release is conditional — remove only if the lock is still ours.** The
`EXIT` trap compares the recorded PID against the running shell's and removes
nothing on a mismatch. An unconditional `rm -f` reproduces this artifact's own
defect inside its own mechanism: the lock is single-holder, so with an
unconditional claim *and* an unconditional release whichever producer exits
first deletes the survivor's lock, after which a preflight reads free with a
producer still live. Atomicity does not make the condition redundant, and the
residual it closes is **not** the `EXIT`-trap race it first suggests: a trap runs
synchronously as part of its process's own exit and completes before that
process reads as dead, and the one exception — `SIGKILL` — skips the trap
entirely rather than deferring it. So a producer correctly identified as stale
cannot later run its trap. The case that remains is a lock removed by some path
*other than* the reclaim below — an operator deleting an apparently-stuck lock,
or a future code path: producer A is still alive and unaware, producer B claims
the freed slot, and A's unconditional release would delete B's live lock.
Atomicity is what makes "still ours" *answerable*; conditional release is what
acts on the answer.

The reclaim path a runtime artifact owes is three layers, and all three are
asserted: the `EXIT` trap, which covers every exit path the spine has; the
readers' PID-liveness predicate, which makes a leaked file inert; and the
consumer's scratch-boundary wipe, which removes it. No close-surface declaration
is owed — that obligation attaches to capture-tier members of the workflow
directory, and this lock lives in the scratch tier.

**evidence-kit owns the lock at both ends; lifecycle-kit contributes only the
hook it already ships.** The lock is a property of the *producer's run*, and this
kit owns the producer and the scratch directory it lives under. The reader need
not live in lifecycle-kit: `LIFECYCLE_KIT_ENTRY_PREFLIGHT` is already a generic
per-stage hook naming no evidence surface, so a second evidence-kit gate on that
roster adds no new cross-kit dependency at all — the consumer wires it exactly as
it already wires the manifest gate (§lifecycle-kit integration). The rejected
alternative is worth recording: a lock held by lifecycle-kit would force that kit
to know this one's scratch knob — a downward dependency onto one specific
producer kit — and would generalize wrongly, since lifecycle-kit would then have
to model every possible producer's lock rather than one hook any producer's gate
can hang from.

### bin/run-validate.sh

The codified spine, bounded by the producer-liveness lock: the guards, the
claim, then the optional per-suite pre-hook and each suite run
foreground, parsed, diffed against the baseline's suite slice per-scenario, and
recorded as one evidence line whose verdict is `clean` unless the diff
finds a new failure. The lines batch under the tmp dir and reach the manifest in
the single fold §Evidence manifest rules, so a suite never runs against a tree
the spine has already written to. It never edits the baseline, never retries, and surfaces a
non-zero suite exit verbatim. A log with no parseable result is a run failure,
not an empty diff. Not a gate — a `bin/` tool exercised end-to-end in `smoke/`,
with the lock's own behavior pinned by `gate-tests/producer-lock.test.sh`.

**The claim's placement is asserted, not left to the implementer**: after the
preflight guards and the scratch `mkdir` — a run that refuses to start must not
claim — and before the batch file is created, so no evidence work happens
outside the lock's cover. Release is the `EXIT` trap, conditional on the record
still being ours (§The producer-liveness lock owns both properties and the
reasons they are load-bearing). A trap rather than a tail line, because the
script exits from many guard and fail-closed sites besides its terminal exit, and
a tail-line release would leak the lock on every failure path — the population
that matters most, since a crashed run is exactly when a stale lock appears. The
idiom precedent is `lifecycle-kit/bin/enter-stage.sh`, which claims temp files
under a scratch dir with an `EXIT` trap.

**It refuses to start while a live lock is held**, and the refusal falls out of
the atomic claim rather than being a second mechanism: the claim either succeeds
— in which case no holder existed — or fails, and the failure branch is the
refusal. On a failed claim it reads the existing lock; a **live** PID refuses
immediately, naming the blocking run key, with no reclaim attempted. A **dead**
PID (or a lock that has vanished since the claim failed) is reclaimed by removing
the lock and retrying **exactly once**; a second failure refuses rather than
looping, which resolves the two-contender stale case without an unbounded retry —
both contenders may remove and relink, exactly one `ln` succeeds, and the loser's
re-read finds a live PID. A lock that does not parse refuses outright.

This is why the tool has **two** non-zero exits with different meanings: exit 1
when a suite records `new-failures` — the verdict — and the guards' exit 2 when
the run cannot start at all, which a held or unclaimable lock now joins. The
refusal is a start-time verdict about the world, not a result, so it takes the
guards' code and not the verdict's.

The reason a producer that never enters a stage is worth guarding is that the
entry-side reader cannot see it: a session can run this tool without entering a
stage, so an entry-side red alone would leave two producers able to race the
manifest with every stage entry green. A lock the producer itself does not check
is not a mutex.

### bin/diff-baseline.sh

The situational runtime diff, not a precommit gate: it takes captured logs as
arguments, parses each, and diffs against the baseline slice per-scenario. A
baseline `pass` scenario red-or-absent is a new failure; a baseline `fail` or
`ignore` scenario running green is an unpromoted recovery; an observed `fail`
with no baseline row at all is a new failure (§Baseline manifest's fail-closed
rule, which keys on `fail` alone). The split is
per-scenario, so a regression and a recovery cannot net to zero. It reads the
skip side-channel (`EVIDENCE_KIT_SKIP_FILE`, truncated per run) to demote a
self-skipped scenario from pass first, so a self-skip cannot masquerade as a
pass. The shared diff (`ek_diff`) returns non-zero the moment a new failure
fires, which is also how run-validate derives its verdict.

### check-evidence-baseline

Invariant: the held-constant baseline stays grammatical and honest. It asserts
the `<suite> <scenario> <status> [<slug>]` shape, blocking-slug liveness — every
`fail`/`ignore` slug resolves to a live queue task or a permanent marker, and a
slug present only under `## Done` is stale-red — and, for every suite carrying a
configured scenario glob, manifest↔disk set equality (a baseline scenario with
no matching file, or a file with no baseline line, reddens). Argument mode
`$1 $2` (baseline, queue) with configured defaults makes it fixture-capable; the
liveness and coverage branches beyond the one good/bad pair are covered by
`gate-tests/check-evidence-baseline.test.sh`.

### check-evidence-manifest

Invariant: the evidence manifest is well-formed and, where lifecycle drives the
tree, coupled to the stage machine. It owns three assertions, (A) close-entry —
a `close` cursor requires the full green block, a `verdict=clean` line
for every configured suite dated on/after the iteration's earliest validate
stamp; (B) grammar — every line the eight-field manifest shape with the current
iteration (a foreign iteration line means the boundary truncation was skipped);
and (C) stamp-coupling — a validate stamp demands at least one evidence line,
re-armed only once the cursor has advanced past `validate`, since the
entry stamp legitimately precedes the suites. Grammar (B) red suppresses A and C.
An empty cursor disarms A and C entirely, so a consumer running no lifecycle
keeps only the grammar floor. Both no-cursor shapes disarm at the *declared*
early-out rather than by an empty stage falling through two live assertions: a
silent fall-through would read as a green gate in exactly the window where the
gate has nothing to say, which is the failure mode the cursor migration was
ordered to avoid.
Argument mode `$1 $2 $3` (manifest, queue, state) makes it fixture-capable; the
close-entry and stamp-coupling assertions are covered by
`gate-tests/check-evidence-manifest.test.sh`.

### check-battery-roster

Invariant: the configured runner doc's battery-roster block holds name-set
parity with `EVIDENCE_KIT_SUITES`, both directions. The suite roster is
machine-owned and the doc block is a hand copy of it — the
`check-readme-roster` fork (gate-sdk/SPEC.md §check-readme-roster) applied to
the validate battery: the register stays a human-read list carrying per-line
annotation prose an emitter would have to invent, and a gate holds it honest
rather than generating it.

Marker vocabulary follows that gate verbatim in shape: the doc wraps its
register in `<!-- battery-roster:begin -->` / `<!-- battery-roster:end -->`
markers, which may carry leading indentation (the scan trims surrounding
whitespace before matching, since a README nests the block inside a list item).
Inside the markers a **roster line** is a line whose content begins with a bare
lowercase command word (`bash …`, `cargo …`) — the interpreter is not part of
the grammar, so a suite whose runner is not a shell script is rosterable without
widening a literal each time; the fenced-block delimiters and any prose fail the
match by not starting with a lowercase letter. A trailing `#` annotation clause
is prose the gate never reads.
Outside the markers nothing is scanned, so the same command appearing elsewhere
in the doc for a different rhetorical job neither satisfies nor violates the
gate — which is why the block, not the whole doc, is the unit.

A suite's **documented invocation** is `EVIDENCE_KIT_RUN_<suite>` normalized by
stripping a leading `env` token and any leading `VAR=value` assignments, with
or without that token: the run environment is a validate-harness concern (this
repo's `gates` suite runs under gate-sdk's `GATE_SDK_VERBOSE` knob, whose value
that kit's SPEC owns, to emit the per-gate tails its parser reads) and not
something a contributor types. What remains is
compared as an exact string, whitespace-collapsed on both sides so the block's
annotation alignment carries no meaning.

Two assertions over the suite set versus the roster set:

- **(A) every suite is documented** — a member of `EVIDENCE_KIT_SUITES` whose
  normalized invocation matches no roster line is red;
- **(B) every roster line resolves to a suite** — a roster line whose command
  matches no suite's normalized invocation is red, so a retired suite cannot
  leave a stale line telling a contributor to run a command that is not a
  configured suite.

Each finding names the suite (A) or the command and its line (B), and the doc.
A suite with no `EVIDENCE_KIT_RUN_<suite>` configured has no documented
invocation to compare and is passed over: `bin/run-validate.sh` already exits 2
on it, and reporting it here would send the reader to the doc to fix a config
bug.

The overlap with `check-kit-registration` assertion B (gate-sdk/SPEC.md
§check-kit-registration) is deliberate. That assertion requires every kit root
with tracked `gate-tests/` files to have a runner-doc line naming
`<kit>/gate-tests`; because this repo's config derives exactly those roots into
`EVIDENCE_KIT_SUITES` through `gate_fixture_suites`, assertion (A) here is a
superset of that arm for a consumer running both kits. It is kept rather than
retired on a dependency direction: a gate-sdk gate may not require this kit's
config — gate-sdk's enforcement-map emitter reads the suite roster where a
consumer has one, but an assertion cannot, having no honest verdict when it is
absent. So B is the arm that survives a gate-sdk-only adoption, which is the
more common shape. Both sections say so, each naming the other, so the next
reader who notices the redundancy finds the reason instead of re-deriving it.
One omission reported from both sides is a duplicate finding, not a
contradiction — the two name different sets (a kit root, a suite) in their
output.

Config: `EVIDENCE_KIT_RUNNER_DOC` (§Layout and configuration); positional form
`check-battery-roster.sh [runner-doc]` overrides it against a hermetic fixture
tree, the sibling meta-gates' shape. Fail-closed: a configured doc that does not
exist, a doc carrying no marker block, an empty suite roster, or a non-repo cwd
with no positional argument is a misconfiguration (exit 2), never a false clean.
There is no empty-knob valve — a consumer keeping no runner doc opts out by not
registering the gate in its `gates.list`, gate-sdk's registry opt-out shape. The
fail-closed branches and the normalization arms beyond the one good/bad pair are
covered by `gate-tests/check-battery-roster.test.sh`.

### check-producer-liveness

Invariant: no stage entry while the evidence producer is still running. It reads
`EVIDENCE_KIT_LOCK_FILE` and is green when the lock is absent or names a dead
PID, red when it names a live one — printing the blocking run key, so the
operator can tell *wait for that run* from *reclaim a lock whose owner is gone*.
A lock that does not parse is exit 2: the claim publishes the record whole
(§The producer-liveness lock), so an unparseable lock is corruption and never a
free reading.

This is a new gate rather than a fourth assertion on `check-evidence-manifest`,
because that gate's charter is manifest *content* — the close-entry green block,
the grammar, the stamp coupling — and liveness is a different class. A separate
gate also earns its own fixture pair instead of widening an existing gate's
charter.

Argument mode `check-producer-liveness.sh [lock-file]` makes it fixture-capable
and is how the entry hook points it at the lock (§lifecycle-kit integration);
extra arguments are ignored, so the hook's trailing `<queue> <state>` argv passes
through harmlessly.

**It belongs on the entry hook and not in a `gates.list` battery**, and the
reason is structural rather than a matter of taste. Its subject is a transition —
*is a producer in flight right now* — where every battery member's subject is
tree state. A consumer whose validate roster includes its own gate battery (this
repo's does: the `gates` suite *is* the battery) would have `run-validate` invoke
this gate while holding the lock it just claimed, reddening every validate run
against its own record. The kit therefore ships the gate registered nowhere and
wired at the entry, which is also the honest reading of its argument mode: it
takes the lock path because its caller is a stage entry, not a whole-tree sweep.

The fixture pair carries the two static verdicts — a dead PID and a live one.
Its `bad/` case names PID 1, the one PID a checked-in fixture can assert the
liveness of on every platform, which is exactly what `ek_pid_alive`'s `ps -p`
leg makes reliable: under `kill -0` alone, an unprivileged run reads init as
dead and the case would silently invert. Everything the pair cannot hold — a
live PID the test itself owns, the unparseable-lock exit, and the writer-side
behavior — is covered by `gate-tests/producer-lock.test.sh`.

## lifecycle-kit integration

Integration is two generic knobs on lifecycle-kit's side of the seam, each
naming no evidence surface in the kit — the coupling lives entirely in the
consumer's config and this gate's optional assertions.

`LIFECYCLE_KIT_BOUNDARY_TRUNCATE` lists the files `bin/enter-stage.sh` truncates back
to their `# contract:` header at the iteration boundary, exactly as it already
resets the state file. A consumer sets it to the evidence manifest, so a new
iteration starts with a manifest carrying only its contract header — which is
what makes assertion (B)'s foreign-iteration test able to catch a skipped
truncation.

`LIFECYCLE_KIT_ENTRY_PREFLIGHT` carries **both** of this kit's entry-side gates,
at three stage keys between them. `bin/enter-stage.sh` runs each matching entry
against the candidate temp state file (the prospective stamp appended) and the
live queue, appending that `<queue> <state>` argv to whatever the entry names, and
a non-zero exit refuses the entry with nothing written.

`check-evidence-manifest` is wired at `close=`, its command naming the manifest
(`close=…/check-evidence-manifest.sh <manifest>`),
so assertion (A)'s close-entry green-block check fires *before* the
stamp is written — the missing evidence becomes a refusal at the entry (pointing
at run-validate) instead of a self-referential deadlock at pre-commit, where the
`gates` suite that would produce the evidence re-runs this same red gate
against the already-stamped cursor. Belt-and-braces behind the validate
skill's run-validate wiring, not a replacement for it; for a consumer that
wires it, assertion (A)'s enforcement point moves one step earlier, from
commit to entry.

`check-producer-liveness` is wired at `close=` **and** `validate=`
(`<stage>=…/check-producer-liveness.sh <lock-file>`). `close=` is the case the
gate was filed for — a lead dispatching close into a still-running producer.
`validate=` is added because this roster is an exact per-stage match, and a
second validate batch entering while a first batch's `run-validate` is live is
the same hazard with a worse outcome: two producers folding the same manifest.
Which keys a consumer wires is config, not asserted kit behavior — a consumer
whose producer runs at another stage wires its own.

The **read-only `--simulate` mode inherits this gate with no extra wiring**, and
that is the highest-value consumer rather than a bookkeeping detail: it runs
every matching preflight entry, so a lead gating an expensive dispatch with a
simulated entry stops being blind to a live producer. It does **not** make a
dispatch rule redundant, and the boundary has to be stated or the pair reads as
over-built: a simulated entry remains an instantaneous read, so a producer that
starts a second later is still unseen, and the gate covers only producers that
claim this lock. A lead dispatching on artifact state is still dispatching on
artifact state — it merely has one more artifact. A dispatch rule governs what
the lead waits *for*; this gate narrows what survives being wrong about it.

The validate stage records evidence on a commit later than the entry stamp
(assertion C's re-arm scoping): the stamp proves invocation at entry, the
evidence line proves the green result once the suites have run.

## Producers and consumers

- **Evidence line** — produced by `run-validate.sh` per suite verdict; consumed
  by `check-evidence-manifest` (A/B/C), by close-stage entry via that gate, and,
  forward, by the hosted-attestation payload. Every field has a reader there:
  iteration (A/C scoping), suite + verdict + counts (A's green-block test),
  sha256 (audit pinning of the producing log), date (A's stamp-ordering floor).
- **Suite roster** (`EVIDENCE_KIT_SUITES` + `EVIDENCE_KIT_RUN_<suite>`) —
  produced by consumer config, wholly or in part derived there; consumed by
  `run-validate.sh` (what to run), by `check-evidence-manifest` (A's green
  block), by `check-battery-roster` (the doc-parity compare), and — behind a
  `declare -p` probe, so evidence-kit stays optional — by gate-sdk's
  enforcement-map emitter. Every one of them reads it by sourcing the config
  through the loader rather than parsing the file, so a suite a derivation loop
  adds is visible to all of them with no second parse to keep in step. The
  compiled emitter reads it through that same loader at one remove: gate-sdk's
  config bridge sources this kit's library to resolve `EVIDENCE_KIT_SUITES` and
  the `EVIDENCE_KIT_RUN_` family, so the derivation loop still runs in bash and
  nothing re-parses the file (gate-sdk/SPEC.md §lib/gate.sh).
- **Producer-liveness lock** — produced by `run-validate.sh` at the claim point,
  which sits on the ordinary path (the validate stage runs it, and it is the only
  writer of the manifest); its enabling config carries a default in the loader, so
  it resolves in every deployed configuration rather than only under a test
  harness. Consumed by `check-producer-liveness` through the entry-preflight hook
  and, inheriting it with no extra wiring, by that hook's read-only simulate mode.
  Both fields have named readers at named transitions: `pid` at three — the gate
  at the stage-entry transition, `run-validate` at run start on a failed claim
  (the refusal), and its `EXIT` trap at release, compared against the running
  shell to answer *is this still ours*; `run key` at the two transitions where a
  refusal line is composed, whose reader is the operator standing at that refusal
  choosing between waiting and reclaiming. Reclaimed by the `EXIT` trap, by the
  readers' liveness predicate making a leak inert, and by the consumer's
  scratch-boundary wipe.
- **Baseline line** — produced by human commits (initial seed, promotions);
  consumed by `diff-baseline.sh` (the per-scenario diff) and
  `check-evidence-baseline` (grammar, liveness, coverage).
- **Skip record** — produced by a consumer harness that self-skips a scenario;
  consumed by `diff-baseline.sh`. An absent file means no skips.
- **Per-suite parser override** — produced by consumer config
  (`EVIDENCE_KIT_PARSER_<suite>`); read by `ek_parser_for`, and so by the
  `ek_parse` dispatch and `run-validate`'s effective-parser diagnostic, reaching
  both spine tools.
- **Scenario line** — produced by the resolved parser from the captured log;
  consumed by `ek_diff` (the per-scenario diff) and by the evidence line's
  `pass=`/`fail=`/`ignore=` counts. Per-gate granularity changes the line's
  population, not its shape, so those readers are unchanged.
- **Truncation** — produced by `enter-stage.sh` at the scope boundary reading
  `LIFECYCLE_KIT_BOUNDARY_TRUNCATE`; consumed by assertion (B)'s foreign-iteration
  test, which is what makes skipping it visible.
