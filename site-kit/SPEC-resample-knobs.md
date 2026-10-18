# SPEC amendment: resample-knobs

Pairs `site-health-resample-knobs`. The probe step of `templates/site-health.yml` resamples its probe set up to a bounded number of attempts with a pause between them (site-kit/SPEC.md §templates/site-health.yml, *The resample*). Both bounds are literals in the step's run body, `attempts=3` and `pause=30`, outside the env block that holds every other template knob, and the SPEC says the template owns them as literals. doctrine-kit/DOCTRINE.md Policy-as-choice asks a calibration to ship as a consumer-selectable set with off among it. This amendment lifts both into the probe step's env block, one attempt meaning off.

**Step env, the template's knob form.** The template's knobs are step-level workflow env set in the copied file, never `SITE_KIT_*` knob-file knobs (site-kit/SPEC.md §Layout and configuration), so the two new names take that form. The defaults stay `3` and `30`, so a consumer copying the template after this change meets the behaviour it met before.

## What changes

### (1) The resample's two bounds become probe-step env {mechanical} {user-facing: site-health-resample-knobs' Deliverable, "both lifted into the probe step's env block, one attempt meaning off, the SPEC section stating the set"}

Replacement for `site-kit/SPEC.md` §templates/site-health.yml, the paragraph opening **The resample.**, its third sentence. **Not yet applied.**

> The bounds are two values in the probe step's env block: `RESAMPLE_ATTEMPTS`, a positive integer where `1` turns the resample off, and `RESAMPLE_PAUSE`, the whole seconds between attempts, `0` admitted. The template ships them at `3` and `30`. A value outside its set is a misconfigured knob and therefore a finding: the step reports it as the set's one failure and runs no probe.

`site-kit/templates/site-health.yml`: the probe step's env block gains `RESAMPLE_ATTEMPTS: '3'` and `RESAMPLE_PAUSE: '30'` under one `# spec: site-kit/SPEC.md §templates/site-health.yml — the resample's bounds; RESAMPLE_ATTEMPTS: '1' turns it off` directive. The run body's `attempts=3` and `pause=30` assignments are replaced by reading the two values. Before the loop, a check that the attempt count matches `^[1-9][0-9]*$` and the pause `^[0-9]+$`, and on a miss the knob's own finding naming the value it got. The loop's `attempts`/`pause` locals and its census lines are unchanged.

`.github/workflows/site-health.yml`, this repo's filled instance, takes the same lift at the template's values. The repo keeps the shipped bounds.

### (2) The template's knob roster names the two {mechanical}

Replacement for `site-kit/SPEC.md` §Layout and configuration, the paragraph opening "`templates/site-health.yml` is governed by none of these", its first sentence. **Not yet applied.** The act depends on whether `site-health-issue-venue-unwanted` lands in the same build batch:

- **Batched together, or this one second:** the sentence names all three additions.

  > Its knobs (`ALT_DOMAIN`, the three `RELEASE_NOTE_*`, the two `HIGHLIGHT_*`, the two `RESAMPLE_*` and `ISSUE_TRACKING`, all below) are **step-level workflow env** set in the copied file itself, not `SITE_KIT_*` knobs, and no knob file names them.

- **This one lands alone:** the sentence names the two `RESAMPLE_*` only, and the sibling's amendment adds `ISSUE_TRACKING` when it lands.

  > Its knobs (`ALT_DOMAIN`, the three `RELEASE_NOTE_*`, the two `HIGHLIGHT_*` and the two `RESAMPLE_*`, all below) are **step-level workflow env** set in the copied file itself, not `SITE_KIT_*` knobs, and no knob file names them.

`site-kit/README.md` §Install, item 5: after the arm groups, one sentence saying the two `RESAMPLE_*` values bound the resample and that `RESAMPLE_ATTEMPTS: '1'` turns it off, pointing at the SPEC for the set. **Not yet applied.**

## Producers and consumers

- **`RESAMPLE_ATTEMPTS` and `RESAMPLE_PAUSE` (delta 1).**
  - Producer: the probe step's env block in the consumer's copy. The template sets both, so every copy has them set without any edit, and this repo's instance sets them (delta 1).
  - Consumer: the probe step's run body, by environment, at the step's start, once per run. The attempt count bounds the loop and the pause is the `sleep` operand between attempts.
  - Roster-holding readers, from `git grep -n "attempts\|pause\|RESAMPLE" -- site-kit .github/workflows/site-health.yml` and `git grep -n "ALT_DOMAIN" -- '*.md' ':!docs/posts'` over the tracked tree: site-kit/SPEC.md §Layout and configuration's knob sentence (delta 2) and site-kit/README.md item 5 (delta 2). No gate holds a roster of the template's env names. The `check-action-*` gates read the workflow's `uses:`, `run:`, `permissions:` and `gh` repository context, so neither name reaches them.
  - Fields: each value is read once, by the check and then the loop.
- **The misconfigured-knob finding (delta 1).** Producer: the pre-loop check. Consumer: the failure path the template already has, with the knob finding as the report and `status=fail` as the step's output. No new field.
- **Point 5:** no delta narrows a corpus.
- **Point 6:** delta 1 obliges each copy of the template in this tree. The members, from `git grep -l "name: site-health" -- '*.yml'`, are `site-kit/templates/site-health.yml` and `.github/workflows/site-health.yml`, and both satisfy it at `3` and `30`.

## Existing sections updated

The roster came from `git grep -n "attempts\|pause\|as literals\|RESAMPLE\|ALT_DOMAIN" -- site-kit .github/workflows/site-health.yml '.workflow/*.md'` over the tracked tree.

- `site-kit/SPEC.md` §templates/site-health.yml, *The resample* (delta 1).
- `site-kit/templates/site-health.yml`, the probe step (delta 1).
- `.github/workflows/site-health.yml`, the probe step (delta 1).
- `site-kit/SPEC.md` §Layout and configuration (delta 2).
- `site-kit/README.md` §Install (delta 2).
- `.workflow/release-declarations.md` — a `## Knob changes` bullet naming the two new template env values and their defaults, since the template is kit-shipped (deltas 1 and 2).
- `docs/site-kit/SPEC.md` and `docs/site-kit/README.md`, the generated mirrors (all deltas).

## Retired spellings

- None — `attempts` and `pause` stay the loop's locals and the census lines keep their wording; only the two literal assignments move into the env block.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls site-kit/SPEC-*.md`) once the iteration's last site-kit amendment merges.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Fixture suites** — `--run-gate-tests site-kit/gate-tests`, gate-sdk's `check-action-*` suites, and the consumer smoke, which copies the template, run green.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, before the drain stage.
