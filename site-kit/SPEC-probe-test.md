# SPEC amendment: probe-test

Pairs `site-health-probe-unexecuted`. The `check-action-*` gates lint `templates/site-health.yml`, and the kit's smoke copies it into a scratch tree, but nothing executes its bash before a consumer's scheduled run (site-kit/SPEC.md §templates/site-health.yml, its last paragraphs: "The smoke lints the probe's bash and never executes it"). A regression in the resample loop, in an arm's zero case or in the report step's branches would reach every copier unseen. This amendment adds a tracked test that runs the template's own run blocks under stubs.

**The template is the subject, not a copy of it.** The test reads the run blocks out of `site-kit/templates/site-health.yml` at run time, so an edit to the template is the edit the test sees. A transcribed copy would pass while the template regressed.

**Bespoke test, not a gate.** The template asserts nothing over the tree, so there is no verdict to fixture as a `good/`+`bad/` pair. A tests dir admits bespoke tests beside its pairs (gate-sdk/SPEC.md §run-gate-tests), and `site-kit/gate-tests/` already holds five.

## What changes

### (1) `site-kit/gate-tests/site-health-probe.test.sh` executes the probe and report steps {design-bearing}

A new bespoke test. **Not yet applied.**

- **Bootstrap.** Its first act sources `gate-sdk/lib/test-hermetic.sh` (gate-sdk/SPEC.md §check-test-hermetic, assertion A).
- **Extraction.** It reads the `run: |` block of a step named by its `- name:` line out of the template, dedents it by the block's first line, and ends it at the first line indented less. A step name the template lacks, or an empty block, fails the test with the name it looked for. The test fails rather than skips, so a renamed step reds here instead of passing with nothing run.
- **The scratch tree.** Each case runs in a fresh temp directory holding `docs/CNAME` with a fixture apex and one release note under the glob, carrying a `release:` front-matter key. `GITHUB_OUTPUT` names a scratch file there, and the step env the template sets is exported by the test with fixture values. Both theme-drift values are empty, so that arm takes its skip line.
- **The stubs.** A stub dir goes first on `PATH`, and each stub appends its argv to a call log.
  - `curl` answers each probed URL in the healthy shape: 200 for the apex and for each Release-body URL, and the expected 301 and redirect target for `www`, `http` and the alternate host. Under `-v` it writes an `* expire date:` line to stderr. A case variable makes the apex answer 503 on the first *k* attempts, counted by the stub on each apex request, since the apex is each attempt's first probe.
  - `gh` answers the paginated tag list with the note's tag, and the Release lookup with a body carrying the note's URL. A case variable makes the tag-list call exit non-zero with error text. The report step's `label`, `issue list`, `issue create`, `issue comment` and `issue close` calls succeed, and `issue list` answers an open issue number when the case asks for one.
  - `sleep` returns at once.
  - `date` answers the probe's two forms with fixed epochs, a certificate far from expiry and a fixed now. It answers the report step's `-u +<format>` with a fixed string. So the step needs no GNU `date`, and no case reads the clock.
- **The probe step's cases**, under `RESAMPLE_ATTEMPTS=3` and `RESAMPLE_PAUSE=30` set by the test:
  1. Healthy: `status=pass`, settled on attempt 1, no `sleep` call.
  2. One transient failure: `status=pass`, settled on attempt 2, one `sleep 30`.
  3. Two transient failures: `status=pass`, settled on attempt 3, two `sleep` calls.
  4. A persistent failure: `status=fail`. The report holds the apex finding **exactly once**, since the report is the last attempt's failures and never their sum. Two `sleep` calls.
  5. `FORCE_FAIL=true` on a healthy set: `status=fail`, the report exactly the synthetic line, no `sleep` call.
  6. Resample off, `RESAMPLE_ATTEMPTS=1`, under a persistent failure: `status=fail`, settled on attempt 1 of 1, no `sleep` call.
  7. A misconfigured bound, one case each for `RESAMPLE_ATTEMPTS=0`, `RESAMPLE_ATTEMPTS=x` and `RESAMPLE_PAUSE=-1`: `status=fail`, the report is the knob's own finding, and the `curl` log is empty.
  8. The tag list fails: `status=fail`, the report carries the tag-list finding with the `gh` exit status.
- **The report step's cases**, with `STATUS` and `REPORT` set by the test:
  9. `ISSUE_TRACKING=off` and a failure: exit 1, the report printed, an empty `gh` log.
  10. `off` and a pass: exit 0, an empty `gh` log.
  11. `on`, a failure and no open issue: exit 1, and the `gh` log holds the label create, the issue list and an issue create.
  12. `on`, a pass and an open issue: exit 0, and the `gh` log holds an issue close naming that number.
  13. `ISSUE_TRACKING=yes`: exit 1 with the line naming the value, and an empty `gh` log.
- **The siblings.** The set record lands `site-health-resample-knobs` and `site-health-issue-venue-unwanted` before this unit, so the cases above read their names. Where either has not landed when this delta is built:
  - **No resample knobs:** the step's bounds are the literals `attempts=3` and `pause=30`. Cases 1 to 5 hold as written, the test sets no `RESAMPLE_*`, and cases 6 and 7 are not written.
  - **No issue opt-in:** the step is `file or clear the site-health issue`. Cases 9, 10 and 13 are not written, and cases 11 and 12 run with no `ISSUE_TRACKING`.
- **Output.** One line per case naming it and its verdict, a final count, and exit 1 when any case failed. That is the shape of the kit's other bespoke tests.

### (2) The SPEC states the executing oracle {mechanical}

Replacement for `site-kit/SPEC.md` §templates/site-health.yml, the paragraph opening "The kit's `smoke/install.sh` installs the template verbatim", its last sentence. **Not yet applied.**

> The smoke lints the probe's bash and never executes it. `gate-tests/site-health-probe.test.sh` executes it: it reads the probe and report steps' run blocks out of the template and runs each under `PATH` stubs for `curl`, `gh`, `sleep` and `date`, over a healthy set, transient, persistent and synthetic failures, the resample's bounds and its off setting, a failed tag list, and both issue settings. A regression in the loop, an arm's zero case or the issue switch reds `--run-gate-tests site-kit/gate-tests` before any consumer's scheduled run. The test reads the template, never this repo's filled instance, whose consumer-local arms it does not cover.

## Producers and consumers

- **The test (delta 1).**
  - Producer: `--run-gate-tests site-kit/gate-tests`, which runs every `*.test.sh` in the dir after the pairs (gate-sdk/SPEC.md §run-gate-tests). It is reached by the README's per-kit runner, by the commit-time obligation CLAUDE.md states for an edit reaching the kit, and by the fixture-suites step in `.github/workflows/gates.yml`, which loops over the suites `--emit fixture-suites` derives from every `*/gate-tests` dir.
  - Consumer: the session or CI leg reading the exit status and the per-case lines.
  - Roster-holding readers of the new file, from `git grep -n "test\.sh" -- native/src/gates gate-sdk/lib` and gate-sdk/SPEC.md §check-test-hermetic: `check-test-hermetic` assertion A enumerates every `gate-tests/*.test.sh` and reads the bootstrap line, which delta 1 writes. `check-assertion-strength` reads its guards against the exit contracts of the scripts they invoke (gate-sdk/SPEC.md §check-assertion-strength), and the run blocks declare none. `check-shellcheck` lints it as kit source. None of the three keeps a named roster to update.
- **The extraction's step names (delta 1).** The test reads `probe live site` and either `report the probe result` or `file or clear the site-health issue`. A rename in the template reds the test, which is the intended coupling, and the template's `# spec:` header already cites the section that delta 2 points at the test.
- **Point 5:** no delta narrows a corpus.
- **Point 6:** no delta obliges the members of an enumerable corpus. The test runs one template.

## Existing sections updated

The roster came from `git grep -n "never executes\|lints the probe" -- site-kit` and the reader probe above, over the tracked tree.

- `site-kit/gate-tests/site-health-probe.test.sh`, a new file (delta 1).
- `site-kit/SPEC.md` §templates/site-health.yml (delta 2).
- `docs/site-kit/SPEC.md`, the generated mirror (delta 2).

## Retired spellings

- None — the amendment adds a test and one sentence, and renames nothing.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls site-kit/SPEC-*.md`) once the iteration's last site-kit amendment merges.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The oracle bites** — each case was seen red once: break the loop's bound, the report's last-attempt reset, the knob check and the `off` branch in turn, and the case holding each reds. A case that stays green under its break is rewritten.
- [ ] **Fixture suites** — `--run-gate-tests site-kit/gate-tests` and gate-sdk's suites run green.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, before the drain stage.
