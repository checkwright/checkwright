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
multi-kit path. Config is a sourced file, `evidence-config.sh` in the gates dir
(or `EVIDENCE_KIT_CONFIG_FILE` elsewhere); the loader fills every unset knob
with a platform default, then validates and exits 2 on a malformed machine (a
suite name that is not a valid variable suffix) — a broken config gates nothing.

Knobs, platform surface names as defaults:

- `EVIDENCE_KIT_SUITES` — the ordered suite names.
- `EVIDENCE_KIT_RUN_<suite>` — the command that runs a suite (captured to a log
  under `EVIDENCE_KIT_TMP_DIR`, default gate-sdk's `.tmp`).
- `EVIDENCE_KIT_PARSER` — a parser adapter name or a consumer command mapping a
  captured log to `<scenario> <pass|fail|ignore>` lines; default `exit-code`.
- `EVIDENCE_KIT_SCENARIO_GLOBS` — optional per-suite globs; configuring one
  arms the manifest↔disk set-equality assertion for that suite.
- `EVIDENCE_KIT_BASELINE_FILE` (default `.workflow/validate-baseline.txt`),
  `EVIDENCE_KIT_MANIFEST_FILE` (default `.workflow/validate-evidence.txt`),
  `EVIDENCE_KIT_SKIP_FILE` (default `.workflow/validate-skips.txt`).
- `EVIDENCE_KIT_QUEUE_FILE` / `EVIDENCE_KIT_STATE_FILE` — the lifecycle surfaces
  read for the manifest's optional close-entry and stamp-coupling assertions;
  they default through gate-sdk's `GATE_SDK_QUEUE_FILE` / `GATE_SDK_WORKFLOW_DIR`.
- `EVIDENCE_KIT_RUN_ID` — the evidence-line key when no lifecycle queue header
  names the iteration.
- `EVIDENCE_KIT_PRE_HOOK` — an optional per-suite pre-run command (projection
  regen, container teardown) kept on the consumer side of the spine.
- `EVIDENCE_KIT_PERMANENT_SLUGS` — blocking slugs that satisfy baseline liveness
  without a live queue task.

## Per-component contracts

### lib/evidence.sh

The sourced config loader: consumer config first, platform defaults fill what it
left unset, then validation. It also owns the shared adapters — `ek_parse` (the
parser dispatch), `ek_diff` (the per-scenario baseline diff, §bin/diff-baseline.sh),
`ek_data_lines`, and the self-contained `ek_queue_iteration` / `ek_queue_stage`
/ `ek_run_key` header readers that let the kit read a lifecycle header without a
lifecycle-kit dependency. Values and adapters only, never tool structure. It
sources gate-sdk's `lib/gate.sh` for `fail_closed`, so evidence-kit requires
gate-sdk vendored beside it.

The parser adapters map a captured log — and, for `exit-code`, the suite's exit
status — to `<scenario> <pass|fail|ignore>` lines: `libtest` reads per-test
result lines (the platform's Rust suites), `exit-code` emits one scenario per
suite keyed off the suite command's exit (the adapter this repo dogfoods). Any
other value is a consumer command run on the log.

### Baseline manifest

Held-constant, edited by human commit only: one line per known scenario,
`<suite> <scenario> <status> [<slug>]`. A blocking `<slug>` is required exactly
when status is `fail` or `ignore` and forbidden when `pass`; each slug resolves
to a live queue task (the queue-file knob) or a configured permanent marker.
Tooling never writes it — a promotion (a held-constant red recovering to pass)
is a human commit, which is what keeps the baseline honest.

### Evidence manifest

Committed, append-per-run. The file header is a `# contract: evidence-manifest v1`
line — the versioned wire format the deferred hosted-attestation service consumes
as its attestation payload. Each data line is
`<iteration> <suite> sha256=<log-hash> pass=<n> fail=<n> ignore=<n>
verdict=<clean|new-failures> <date>`, and a re-run of a suite within the same
iteration supersedes that iteration's prior line for the suite. The captured log
stays uncommitted under the tmp dir; its digest pins which run produced the
counts. The iteration key scopes the line so the boundary-truncate knob can
clear the manifest at the start of the next iteration.

The header is a wire-format version marker, not a doc pointer; `check-evidence-manifest`
owns it (asserts the first line is `# contract: <version>`). A consumer that
also runs spec-kit's `check-spec-pointer` / `check-comment-tier` over its
workflow dir whitelists the baseline and manifest there (they are data files,
`SPEC_KIT_COMMENT_WHITELIST`), so the `contract:` header is read as the wire
marker it is rather than a dangling spec pointer.

### bin/run-validate.sh

The codified spine: the optional per-suite pre-hook, then each suite run
foreground, parsed, diffed against the baseline's suite slice per-scenario, and
recorded as one appended evidence line whose verdict is `clean` unless the diff
finds a new failure. It never edits the baseline, never retries, and surfaces a
non-zero suite exit verbatim. A log with no parseable result is a run failure,
not an empty diff. Its own exit is non-zero when any suite records
`new-failures`. Not a gate — a `bin/` tool exercised end-to-end in `smoke/`.

### bin/diff-baseline.sh

The situational runtime diff, not a precommit gate: it takes captured logs as
arguments, parses each, and diffs against the baseline slice per-scenario. A
baseline `pass` scenario red-or-absent is a new failure; a baseline `fail` or
`ignore` scenario running green is an unpromoted recovery; the split is
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
a `[stage: close]` header requires the full green block, a `verdict=clean` line
for every configured suite dated on/after the iteration's earliest validate
stamp; (B) grammar — every line the eight-field manifest shape with the current
iteration (a foreign iteration line means the boundary truncation was skipped);
and (C) stamp-coupling — a validate stamp demands at least one evidence line,
re-armed only once the header has advanced past `[stage: validate]`, since the
entry flip legitimately precedes the suites. Grammar (B) red suppresses A and C.
The state file's absence means no lifecycle integration and disarms A and C
entirely, so a consumer running no lifecycle keeps only the grammar floor.
Argument mode `$1 $2 $3` (manifest, queue, state) makes it fixture-capable; the
close-entry and stamp-coupling assertions are covered by
`gate-tests/check-evidence-manifest.test.sh`.

## lifecycle-kit integration

Integration is two generic knobs on lifecycle-kit's side of the seam, each
naming no evidence surface in the kit — the coupling lives entirely in the
consumer's config and this gate's optional assertions.

`LIFECYCLE_BOUNDARY_TRUNCATE` lists the files `bin/enter-stage.sh` truncates back
to their `# contract:` header at the iteration boundary, exactly as it already
resets the state file. A consumer sets it to the evidence manifest, so a new
iteration starts with a manifest carrying only its contract header — which is
what makes assertion (B)'s foreign-iteration test able to catch a skipped
truncation.

`LIFECYCLE_ENTRY_PREFLIGHT` runs this gate as a close-entry pre-flight: a
consumer sets `close=…/check-evidence-manifest.sh <manifest>`, and
`bin/enter-stage.sh` appends the header-flipped temp queue and state file, so
assertion (A)'s close-entry green-block check fires *before* the flip is
written — the missing evidence becomes a refusal at the flip (pointing at
run-validate) instead of a self-referential deadlock at pre-commit, where the
`gates` suite that would produce the evidence re-runs this same red gate
against the already-flipped header. Belt-and-braces behind the validate
skill's run-validate wiring, not a replacement for it; for a consumer that
wires it, assertion (A)'s enforcement point moves one step earlier, from
commit to flip.

The validate stage records evidence on a commit later than the entry flip
(assertion C's re-arm scoping): the stamp proves invocation at entry, the
evidence line proves the green result once the suites have run.

## Producers and consumers

- **Evidence line** — produced by `run-validate.sh` per suite verdict; consumed
  by `check-evidence-manifest` (A/B/C), by close-stage entry via that gate, and,
  forward, by the hosted-attestation payload. Every field has a reader there:
  iteration (A/C scoping), suite + verdict + counts (A's green-block test),
  sha256 (audit pinning of the producing log), date (A's stamp-ordering floor).
- **Baseline line** — produced by human commits (initial seed, promotions);
  consumed by `diff-baseline.sh` (the per-scenario diff) and
  `check-evidence-baseline` (grammar, liveness, coverage).
- **Skip record** — produced by a consumer harness that self-skips a scenario;
  consumed by `diff-baseline.sh`. An absent file means no skips.
- **Truncation** — produced by `enter-stage.sh` at the scope boundary reading
  `LIFECYCLE_BOUNDARY_TRUNCATE`; consumed by assertion (B)'s foreign-iteration
  test, which is what makes skipping it visible.
