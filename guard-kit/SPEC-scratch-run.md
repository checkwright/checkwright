# SPEC amendment: scratch-run

<!-- Delta for the `scratch-execution-prompt-friction` unit of the
     permission-posture-reconciliation iteration. Merges into guard-kit/SPEC.md
     on completion; delete this file then. -->

## What changes

guard-kit **steers** every scratch write into the repo-local scratch dir
(`GATE_SDK_TMP_DIR`, default `.tmp/`) — the guard's cat/find/redirect rules push
probes and multi-line sweeps there, and the harness scratchpad is refused by
name. Nothing allowlists *executing* what lands there, so `bash .tmp/<probe>.sh`
prompts on every run, forever — a loop the repo's own guard creates and its
permission posture penalizes. The friction is real and recurring (measured
multi-times per iteration on build probes and audit sweeps).
**First-hand, from this iteration's align session:** auditing whether any kit
ships a bespoke test without a fixture pair needed one `for`-loop over
`*/gate-tests` with a command substitution per kit — a genuine multi-line sweep,
the arm-(b)-defeating case. The guard refused it for carrying expansions, and the
runner this unit specifies did not yet exist to take it, so the audit was re-cut
into weaker single-purpose commands. The tool's absence taxed the very session
auditing the tool.

New tool **`guard-kit/bin/scratch-run.sh <script> [args…]`** — a content-agnostic
scratch runner that, given a path inside the scratch dir:

1. **Echoes the script's contents to stdout**, annotated (a header line naming
   the path, then the body), so the executed code is visible **at the point of
   execution** in the Bash output log — not only in an earlier `Write` call.
2. **Executes it** (`bash <script> [args…]`), passing through stdout/stderr and
   the child exit code.
3. **Refuses** (fail-closed, non-zero) a target **outside** the scratch dir, so
   the tool cannot be turned into a general "run this arbitrary path without a
   prompt" bypass — its reach is exactly the scratch surface the guard already
   steers into.

Its fixed path makes it **allowlistable**: this consumer adds
`Bash(bash guard-kit/bin/scratch-run.sh *)` to committed `.claude/settings.json`,
killing the prompt while the echo keeps every executed line on the evidence
trail (and thus in the supervisor's post-commit review surface).

**Seam.** The runner is generic mechanism — content-agnostic, reads the scratch
dir from the existing `GATE_SDK_TMP_DIR` knob, carries no rule content. The
allowlist entry is this consumer's settings, not a kit literal.

**Ruled out.** *Arm (a)* — bare `Bash(bash .tmp/*.sh)` allowlist. Rejected: it
auto-approves executing any script at that path with its contents visible only in
a *prior* `Write` call, converting a visible command into a silent+opaque one —
the opposite of the prompt's purpose. The runner's echo-at-execution is the
review-value delta that makes silent execution *self-documenting* rather than
opaque, which is why this unit is a feature (a named tool) rather than a
one-line settings widening. *Arm (b)* — steer to inlining. Rejected as
known-insufficient: it works for a short probe but not for genuine multi-line
sweeps (loops, arrays), which is exactly the case that recurs.

## Producers and consumers

- **New interface: `bin/scratch-run.sh`.**
  - *Producer:* the agent invokes `bash guard-kit/bin/scratch-run.sh
    .tmp/<probe>.sh` in place of a bare `bash .tmp/<probe>.sh`. Enabling config:
    the committed allowlist entry (so the invocation does not prompt) plus
    `GATE_SDK_TMP_DIR` (the scratch dir the target must sit within). No kit-side
    config knob is added — the tool takes the dir from the existing gate-sdk knob.
  - *Consumer:* two readers. (1) the child `bash` process receives the script and
    its args and produces the run's real output/exit code; (2) the **transcript /
    supervisor**, which reads the echoed contents inline with the execution — the
    evidence surface the post-commit verification (delegation-kit's "validate
    after every agent commit") scans.
- **New field: the target path argument.**
  - *Named reader:* the runner's in-scratch-dir guard reads it at invocation to
    decide echo-and-run vs refuse; the `bash` child reads it to execute. A path
    outside `GATE_SDK_TMP_DIR` is refused before any echo or execution.
- **New field: pass-through args (`[args…]`).**
  - *Named reader:* forwarded verbatim to the `bash` child; the runner does not
    interpret them.

## Existing sections updated

- **guard-kit/SPEC.md** — the permission-posture / steering section gains the
  runner: guard-kit steers writes into scratch, and this tool closes the loop by
  making scratch *execution* allowlistable-yet-visible. State the fail-closed
  in-scratch-dir contract and the echo-then-exec evidence property.
- **guard-kit landing checklist** — the tool ships with a test asserting
  echo-then-exec on an in-scratch target, pass-through of exit code and args, and
  fail-closed refusal of an out-of-scratch target. (A bin/tool, not a gate — no
  good/bad gate-fixture pair, but shipped mechanism needs a test.) Its home is a
  new `guard-kit/gate-tests/scratch-run.test.sh`, **not** `guard-kit/guard-tests/`:
  the latter is the hook decision table (`cases.tsv`, expected-decision `<TAB>`
  command, fed through the template guard as hook JSON by `bin/run-guard-tests.sh`
  — guard-kit/SPEC.md §Testing), whose row grammar cannot express any of the three
  assertions. The bespoke-unit-test lane is gate-sdk's:
  `<tests-dir>/*.test.sh` run after the fixture pairs by
  `gate-sdk/bin/run-gate-tests.sh` (gate-sdk/SPEC.md §run-gate-tests), which needs
  no fixture pair present **once gate-sdk/SPEC-unit-only-tests-dir.md lands**.
  *Align audit — that qualifier is load-bearing and was missing.* As written
  today the runner exits 2 (`no gate fixture dirs under <dir>`) on a tests dir
  with no `<gate>/` subdirectory, before the `*.test.sh` lane it would reach, so a
  `guard-kit/gate-tests/` holding only `scratch-run.test.sh` would red the battery
  line this amendment obliges and the test would never run. The original clause
  was an inference from §run-gate-tests' "unit tests run after the pairs", which
  states an ordering and never licensed a pair-less dir. Operator-authorized
  remedy: the runner exits 2 only when the dir holds *neither* pairs *nor* unit
  tests — specified in the paired gate-sdk amendment, which this unit now carries
  alongside this one. Sequencing for build: the runner change lands before or with
  the test, never after.
  Two obligations ride along, both mechanical:
  the test sources `gate-sdk/lib/test-hermetic.sh` as its first act or carries a
  `# hermetic-exempt:` marker (`check-test-hermetic`, gate-sdk/SPEC.md
  §check-test-hermetic); and guard-kit's first `gate-tests/` dir obliges a
  `run-gate-tests.sh guard-kit/gate-tests` line in the fixture-runner battery
  (`check-kit-registration` reds a kit shipping gate-tests with no battery line).
  *Align audit — both verified at the read site, and both do fire.*
  `check-test-hermetic`'s assertion A enumerates `<kit-root>/gate-tests/*.test.sh`
  from `gate_kit_roots`, and guard-kit is a kit root despite shipping no `checks/`
  (the discovery rule keys on `checks/` *or* `smoke/` — guard-kit/SPEC.md §Testing),
  so the bootstrap-or-marker obligation reaches this test. `check-kit-registration`'s
  assertion B tests the dir with `git ls-files`, not the filesystem, so it stays
  silent until the test is staged and then reds in the pre-commit hook — the
  battery line belongs in the same commit as the test, never a follow-up.
- **Consumer surface (this repo, not kit):** `.claude/settings.json` gains
  `Bash(bash guard-kit/bin/scratch-run.sh *)`. Lands at build; named here for
  causal completeness. (`compare-settings-allow.sh` / `scan-prompts.sh` read that
  allowlist — the sibling `scan-prompts-local-overlay-blind` debt unit of this
  same iteration touches how that read is reported.)

## Definition of Done

- [ ] **Causal completeness** — the runner has a reachable producer (agent
      invocation + allowlist entry), named consumers (bash child, supervisor
      transcript), and every argument a named reader; the out-of-scratch refusal
      is specified.
- [ ] **Merged with no information lost** — the runner integrated into
      guard-kit/SPEC.md's posture section; the merged SPEC stands alone.
- [ ] **Amendment deleted** — this file removed on merge (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — no name retired by this change.
- [ ] **Gaps filed** — any cross-component gap found during build resolved that
      session.
