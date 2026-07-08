# SPEC amendment: evidence-kit

The one large gap the platform second scan named: lifecycle-kit's stage
evidence proves a stage was *invoked*, never that it produced its green
result. The platform closes it with three coupled mechanisms — a
held-constant test baseline, a committed per-run evidence manifest, and a
codified run contract. This amendment rules the layout: a **new kit**
(`evidence-kit/`), not a lifecycle-kit extension — the evidence manifest is
the wire contract a future external verifier consumes (its format must be
versioned, stable, and hashable independent of the state machine), and the
kit is adoptable by a consumer that runs no iteration lifecycle at all.
lifecycle-kit integration is optional and arrives through one generic knob
on its side of the seam.

## What changes

New kit `evidence-kit/` (README, SPEC.md, fixtures per gate, `smoke/`),
platform surface names as defaults:

- **`lib/evidence.sh`** — config loader. Knobs: `EVIDENCE_KIT_SUITES`
  (ordered suite names), `EVIDENCE_KIT_RUN_<suite>` (the command that runs
  a suite, capturing to a `GATE_SDK_TMP_DIR` log), `EVIDENCE_KIT_PARSER`
  (command mapping a captured log to `<scenario> <pass|fail|ignore>`
  lines), `EVIDENCE_KIT_SCENARIO_GLOBS` (optional, per suite — enables the
  manifest↔disk set-equality assertion), `EVIDENCE_KIT_BASELINE_FILE`
  (default `.workflow/validate-baseline.txt`), `EVIDENCE_KIT_MANIFEST_FILE`
  (default `.workflow/validate-evidence.txt`), `EVIDENCE_KIT_RUN_ID`
  (evidence-line key when no lifecycle queue header exists). Two parsers
  ship as lib adapters: `libtest` (per-test result lines, the platform's)
  and `exit-code` (one scenario per suite, status from the suite command's
  exit — the adapter this repo dogfoods with).
- **Baseline manifest** — held-constant: one line per known scenario,
  `<suite> <scenario> <status> [<slug>]`, a blocking slug required exactly
  when status is `fail`/`ignore`, each slug resolving to a live queue task
  (queue-kit's file knob) or a configured permanent marker. Never edited
  by tooling; promotions are human commits.
- **Evidence manifest** — committed, append-per-run:
  `<iteration> <suite> sha256=<log-hash> pass=<n> fail=<n> ignore=<n>
  verdict=<clean|new-failures> <date>`, superseding the same iteration's
  prior line for a re-run suite. The file header is a
  `# contract: evidence-manifest v1` line — the versioned wire format the
  deferred hosted-attestation service consumes as its attestation payload.
  The `.tmp` log stays uncommitted; its digest pins which run produced the
  counts.
- **`bin/run-validate.sh`** — the codified spine: optional per-suite
  consumer pre-hook (`EVIDENCE_KIT_PRE_HOOK` — the platform's projection
  regen and Docker teardown stay on the platform behind it), run each
  suite foreground, parse, diff against the baseline's suite slice
  per-scenario, append the evidence line. Never edits the baseline, never
  retries, surfaces a non-zero suite verbatim. A log with no parseable
  result is a run failure, not an empty diff.
- **`bin/diff-baseline.sh`** — the runtime diff, situational (takes
  captured logs as arguments, not a precommit gate): a baseline `pass`
  scenario red-or-absent is a new failure; a baseline `fail`/`ignore`
  scenario running green is an unpromoted recovery; per-scenario, so a
  regression and a recovery cannot net to zero. Reads the skip
  side-channel (`.workflow/validate-skips.txt`, truncated per run) to
  demote self-skipped scenarios from `pass`.
- **`checks/check-evidence-baseline`** (precommit) — baseline grammar,
  slug liveness (Done slugs are stale-red), and — when scenario globs are
  configured — manifest↔disk set equality.
- **`checks/check-evidence-manifest`** (precommit, rides commits staging
  the queue file, the manifest, or the state file) — (B) grammar: every
  line the eight-field shape, iteration = current header's (a foreign line
  means boundary truncation was skipped); with lifecycle-kit configured
  also (A) close-entry: a `[stage: close]` header requires the iteration's
  full green block dated on/after its earliest validate stamp, and (C)
  stamp-coupling: a validate stamp demands at least one evidence line,
  re-armed past `[stage: validate]` (the entry flip legitimately precedes
  the suites). Grammar red suppresses A and C.

lifecycle-kit delta (its side of the integration):

- **`LIFECYCLE_KIT_BOUNDARY_TRUNCATE`** — a list of files
  `bin/enter-stage.sh` truncates back to their `# contract:` header at the
  iteration boundary, exactly as it already truncates the state file; this
  repo sets it to the evidence manifest. Generic mechanism — no evidence
  name appears in lifecycle-kit.
- The validate stage skill template gains the run-validate invocation and
  the manifest-commit step (evidence rides a later commit than the entry
  flip, per check-evidence-manifest assertion C's re-arm scoping).

Dogfood config for this repo: suites = the CLAUDE.md battery (the gate
run plus each kit's fixture runner and the guard decision table) under the
`exit-code` parser, baseline all-`pass` — the evidence manifest then proves
per-iteration, at close entry, that the battery actually ran green.

## Producers and consumers

- Evidence line: produced by `run-validate.sh` per suite verdict; consumed
  by `check-evidence-manifest` (A/B/C), by close-stage entry via that
  gate, and — forward — by the hosted-attestation payload (BRIEF-tracked).
  Every field has a reader there: iteration (A/C scoping), suite +
  verdict + counts (A's green-block test), sha256 (audit pinning of the
  producing log), date (A's stamp-ordering floor).
- Baseline line: produced by human commits (initial seed, promotions);
  consumed by `diff-baseline.sh` (the per-scenario diff) and
  `check-evidence-baseline` (grammar/liveness/coverage).
- Skip record: produced by a consumer harness that self-skips; consumed by
  `diff-baseline.sh`. Absent file = no skips (this repo writes none).
- Truncation: produced by `enter-stage.sh` at the scope boundary reading
  `LIFECYCLE_KIT_BOUNDARY_TRUNCATE`; consumed by assertion B's
  foreign-iteration test, which is what makes skipping it visible.

## Existing sections updated

- lifecycle-kit/SPEC.md §bin/enter-stage.sh — the boundary-truncate knob;
  §Layout and configuration — knob listed; §check-stage-evidence — the
  honest limit ("stamp proves invocation, not results") now points at
  evidence-kit as the closure.
- lifecycle-kit/SPEC.md §templates/skills/ — validate template's
  run-validate + manifest-commit steps.
- README.md kit table — evidence-kit row.
- CLAUDE.md battery list — evidence-kit fixture runner line, and
  `scripts/gates.list` registers the two checks.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md evidence-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
