# SPEC amendment: battery-quiet

## What changes

Every green battery run prints the full per-gate banner roll (~70 banners in
this repo: a `=====` header, the gate's clean line, a PASS line, times every
gate), and the generated pre-commit hook does the same on every commit — so a
session re-running the battery accretes the whole roll in context and a
supervising lead re-reads it on each cold wake: the dominant residue class
observed in the first lead-orchestrated iteration. The principle lands as
**quiet green, loud red**, mechanized on both run surfaces and registered as a
doctrine craft rule.

- **`run-gates.sh` — quiet is the default.** A passing gate prints nothing;
  the run ends with the existing summary line, whose executed-gate count is
  the roster-collapse tripwire (`All N gates passed.` — a battery that
  silently shrank shows a smaller N). A failing or erroring gate prints its
  `===== <name> =====` header and its captured output **verbatim, always** —
  the red path is the feedback channel and never quiets. `--for`'s
  no-coupled-gate notice and the timings file are unchanged.
- **`GATE_SDK_VERBOSE`** (env knob, default unset = quiet; any non-empty
  value = verbose) restores the full banner roll — the on-demand reading for
  the vacuous-pass tripwire (a "0 files scanned" clean line is only visible
  in a gate's own banner). Env over flag: one mechanism serves the
  interactive run, the hook, and any CI wrapper without an argv contract
  change (config-via-env, the kit convention).
- **The generated hooks go quiet the same way.** `gen-pre-commit.sh`'s
  emitted pre-commit and commit-msg hooks gain a capture wrapper in the
  generated header: each gate invocation's output is captured and printed
  only on failure (then `hook_fail` as today); a fully green hook run prints
  one summary line carrying its executed-invocation count. `GATE_SDK_VERBOSE`
  passes through verbatim. The hook stays generated — the wrapper lives in
  the emitter's heredoc, and `check-graph` freshness carries the change into
  the committed hook.
- **`demo/run-demo.sh` stays verbose by design** — its display is the
  payload; it sets `GATE_SDK_VERBOSE=1` around its battery runs so the
  walkthrough still shows the roll.
- **Consumer-facing default flip.** A vendored consumer's next upgrade
  changes green-run output; the release note's behavior-change declaration
  names it (the upgrade contract's phase-B reading), and gate-sdk/SPEC.md
  §run-gates documents quiet as the contract with verbose as the opt-in.
- **Doctrine registration.** doctrine-kit/DOCTRINE.md's engineering-craft
  register gains rule 21, *Quiet green, loud red*: success is one summary
  line carrying its scope counts; failure output is verbatim and never
  quiets. Craft-class — "prints too much on success" is not mechanically
  decidable, so the rule is prompted, not gated; the leaf mechanisms are
  `check-gate-output` (per-gate) and this amendment's two run surfaces.
  Trailer: *Stages:* build. Assertion D of
  doctrine-kit/SPEC.md §check-doctrine-registration covers the new rule's
  trailer automatically; craft rules join no consumer digest.

## Producers and consumers

- **Captured gate output** — producer: the runner/hook wrapper around each
  gate invocation; consumer: the terminal only when that invocation fails or
  `GATE_SDK_VERBOSE` is set; otherwise discarded. Gates themselves are
  untouched — each still prints its single clean line (the gate-output
  contract), so nothing downstream of a gate changes.
- **`GATE_SDK_VERBOSE`** — producer: the invoking environment (operator,
  demo, CI); readers: `run-gates.sh`'s per-gate print step and the generated
  hooks' wrapper. Joins the knob roster in gate-sdk/SPEC.md §Layout and
  configuration.
- **The hook summary line** — producer: the generated hook epilogue;
  consumer: the committing session's context (one line where ~70 banners
  were) and the operator eyeballing a commit.
- No new files, no new exit-code semantics: red set, exit codes, and
  fail-closed behavior are byte-identical to today.

## Existing sections updated

- gate-sdk/SPEC.md §run-gates: output contract rewritten around quiet-green
  default + verbose knob; the summary-line count documented as the
  roster-collapse tripwire.
- gate-sdk/SPEC.md §gen-pre-commit: the emitted-header description gains the
  capture wrapper and summary line.
- gate-sdk/SPEC.md §Layout and configuration: knob roster gains
  `GATE_SDK_VERBOSE`.
- gate-sdk smoke: green-run-is-quiet, red-run-prints-verbatim, and
  verbose-restores-banners cases join the runner's coverage; the hook wrapper
  is exercised through the emitted hook fixture.
- doctrine-kit/DOCTRINE.md + its registration gate fixtures: rule 21 with
  trailer (fixture pair already asserts craft-trailer coverage generically).
- demo/run-demo.sh: explicit `GATE_SDK_VERBOSE=1`.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls <component>/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
