# SPEC amendment: issue-opt-in

Pairs `site-health-issue-venue-unwanted`. On a failed probe, `templates/site-health.yml` opens or updates a `site-health` issue, and recovery closes it (site-kit/SPEC.md §templates/site-health.yml). The entry's option 2, the direction it records, makes the issue path consumer config that defaults **off**, with this repo taking the default. This reverses the shipped design, and the amendment states it as a reversal. What is reversed is the **venue**: the probe and its red run are unchanged, and the red run alone is the signal by default.

**Why off is a default the kit can ship.** A failed scheduled run already notifies the person who last changed the workflow's cron line, and the run log carries the full report. An issue is a second, public record of the same failure. On a repository whose own lifecycle repairs what the probe finds, it is tracker noise. So the issue path is a calibration, and Policy-as-choice (doctrine-kit/DOCTRINE.md) ships it as a selectable set with off among it, and off as the default.

**Step env, the template's knob form.** The entry's option asks for consumer config under the config-via-env convention. The template's knobs are step-level workflow env in the copied file, never `SITE_KIT_*` knob-file knobs (site-kit/SPEC.md §Layout and configuration, which gives the grounds: a monitor must not reach into the gates dir). So the switch is a step env value, `ISSUE_TRACKING`.

**Least privilege follows the default.** The workflow's `permissions:` block is an allowlist (§templates/site-health.yml), and nothing in a workflow can make a scope depend on an env value. With the path off, `issues: write` is a write scope nothing uses, so the template drops it. Opting in is therefore two edits in the copy: the switch and the scope. A copy that sets the switch and forgets the scope fails its first `gh issue` call, which reds the run with the API's own error, so the half-done opt-in is visible.

## What changes

### (1) The report step: a red run always, an issue only on opt-in {design-bearing} {user-facing: site-health-issue-venue-unwanted's option 2, "the issue path becomes kit-level consumer config under `<KIT>_<KNOB>`, defaulting off, and this repo takes the default"}

`site-kit/templates/site-health.yml`, the step now named `file or clear the site-health issue`. **Not yet applied.**

- **The step is renamed `report the probe result`.** It runs on every probe outcome as it does now, and it is still the step that reds the run.
- **Its env gains `ISSUE_TRACKING: 'off'`**, under the directive `# spec: site-kit/SPEC.md §templates/site-health.yml — the issue path: 'on' also needs issues: write in the permissions block`.
- **Its run body checks the value first.** A value other than `on` or `off` prints a line naming `ISSUE_TRACKING` and the value it got, then exits 1, so a misconfigured switch reds the run whatever the probe found.
- **With `off`**, the body makes no `gh` call. On `STATUS=fail` it prints the report and a line saying the issue path is off, then exits 1. On a pass it prints `all probes green; issue path off` and exits 0.
- **With `on`**, the body is today's body unchanged: it creates the label, finds the open issue, comments on it or opens one and exits 1 on a failure, and closes it on recovery.
- **The workflow's `permissions:` block** keeps `contents: read` and drops `issues: write`. The block's directive becomes `# spec: site-kit/SPEC.md §templates/site-health.yml — the permissions block is an allowlist; ISSUE_TRACKING: 'on' adds issues: write`.
- **The `force_fail` input's description** becomes `Inject a synthetic probe failure to exercise the failure path`.

`.github/workflows/site-health.yml`, this repo's filled instance, takes the same edit and keeps `ISSUE_TRACKING: 'off'`. Its consumer-local arms, release-channel and About, feed the same `failures` array, so their findings also red the run and file nothing.

### (2) The SPEC states the reversed venue {mechanical} {user-facing: site-health-issue-venue-unwanted's option 2, as delta 1}

Replacements in `site-kit/SPEC.md`. **Not yet applied.**

- §The monitor boundary, third paragraph, its second sentence:

  > It ships as `templates/site-health.yml`, a scheduled workflow a consumer copies, and signals through a red run of its own, and through an issue where the consumer opts in, never a blocked merge.

- §templates/site-health.yml, the paragraph opening "A failure opens or updates", its first sentence:

  > A failure reds the run. The issue path is opt-in: with `ISSUE_TRACKING` set to `on` in the report step's env and `issues: write` added to the permissions block, a failure also opens or updates a single `site-health` issue and recovery closes it. The template ships it `off`, and any value but `on` or `off` is a misconfigured knob the report step reds on.

- *The resample*, the bullet **A standing defect is resampled too**: "so it reaches the issue later by the pauses" becomes "so it reaches the failure path later by the pauses".
- The paragraph opening "The workflow's `permissions:` block is an **allowlist**", its second sentence:

  > So reading the tag list and the Release bodies needs `contents: read` declared, the opt-in issue path needs `issues: write` beside it, and the probe step needs its own `GH_TOKEN`.

- §Layout and configuration, the knob sentence: `ISSUE_TRACKING` joins the roster. The act depends on whether `site-health-resample-knobs` lands in the same build batch. Batched, or that one first, take the three-addition form that amendment's delta 2 gives. Landing alone, add `and ISSUE_TRACKING` after the two `HIGHLIGHT_*`.

### (3) The surfaces outside site-kit that cite the issue {mechanical} {user-facing: site-health-issue-venue-unwanted's option 2, as delta 1}

Replacements. **Not yet applied.**

- `site-kit/README.md` §Install, item 5: "It opens/updates/closes a `site-health` issue on its own schedule." becomes "A failure reds the run. Set the report step's `ISSUE_TRACKING: 'on'` and add `issues: write` to the permissions block to have it also open, update and close a `site-health` issue."
- `installer/SPEC.md` §The release channel, the paragraph opening **C is held by a monitor**, its first sentence:

  > **C is held by a monitor, not by the battery**: the release-channel arm of `.github/workflows/site-health.yml` reads the Release list from the API on its schedule and reds its run, naming in the run's report every Release whose flag disagrees with its own version line.

## Producers and consumers

- **`ISSUE_TRACKING` (delta 1).**
  - Producer: the report step's env block in the consumer's copy. The template sets it to `off`, so every copy has it set without an edit, and this repo's instance sets it to `off` (delta 1).
  - Consumer: the report step's run body, by environment, once per run, before any `gh` call.
  - Roster-holding readers: the knob sentence in site-kit/SPEC.md §Layout and configuration (delta 2), and README item 5 (delta 3), found by `git grep -n "ALT_DOMAIN" -- '*.md' ':!docs/posts'`. No gate holds a roster of the template's env names.
  - Fields: one value, read once.
- **The red run without an issue (delta 1).** Producer: the report step's `exit 1` on `STATUS=fail`, now reached on both settings. Consumer: GitHub's scheduled-failure notification, and the reader of the run log.
- **The dropped `issues: write` scope (delta 1).** Readers: `check-action-permissions`, whose arm A holds a checkout-armed job to `contents:` and is satisfied by `contents: read` alone (gate-sdk/SPEC.md §check-action-permissions). Probe: `git grep -n "issues" -- native/src/gates/action_permissions.rs gate-sdk/gate-tests/check-action-permissions` names only fixture and test content, so no gate requires the scope.
- **Readers of the issue wording**, from `git grep -n -i "site-health\` issue\|site-health issue\|an issue and a red run\|opens or updates\|issues: write\|issue path\|opens/updates" -- ':!TASK-QUEUE.md' ':!docs/posts'` over the tracked tree:
  - the template and this repo's instance (delta 1);
  - site-kit/SPEC.md at §The monitor boundary, the issue sentence, the resample bullet and the permissions paragraph (delta 2);
  - site-kit/README.md item 5 and installer/SPEC.md §The release channel (delta 3);
  - their generated mirrors under `docs/`.

  `site-kit/smoke/install.sh` copies the template and keeps no issue wording (`git grep -n -i issue -- site-kit/smoke` matches nothing), so delta 1 leaves it unedited. `.workflow/release-declarations.md` names the issue in a bullet already declared for a shipped release, which stays as written. The fixture `gate-sdk/gate-tests/check-action-gh-repo/good/tree/workflows/arms.yml` spells its own `gh issue` calls and does not copy the template.
- **Point 5:** delta 1 narrows the template's permission set, and the reader whose verdict could move is `check-action-permissions`. It reds a checkout-armed job that lacks a `contents:` scope at `read` or `write`. The `probe` job keeps `contents: read`, so it stays clean.
- **Point 6:** delta 1 obliges each copy of the template in this tree. The members, from `git grep -l "name: site-health" -- '*.yml'`, are `site-kit/templates/site-health.yml` and `.github/workflows/site-health.yml`, and both take `ISSUE_TRACKING: 'off'` with no `issues: write`.

## Existing sections updated

The roster is the reader probe above.

- `site-kit/templates/site-health.yml`, the report step and the permissions block (delta 1).
- `.github/workflows/site-health.yml`, the same edit on this repo's instance (delta 1).
- `site-kit/SPEC.md` §The monitor boundary, §templates/site-health.yml and §Layout and configuration (delta 2).
- `site-kit/README.md` §Install (delta 3).
- `installer/SPEC.md` §The release channel (delta 3).
- `.workflow/release-declarations.md`: a `## Behavior changes` bullet saying that a copy of the template no longer files issues by default, and naming the two edits that restore them, plus a `## Knob changes` bullet naming `ISSUE_TRACKING` (deltas 1 and 2).
- `docs/site-kit/SPEC.md`, `docs/site-kit/README.md` and `docs/installer/SPEC.md`, the generated mirrors (all deltas).
## Retired spellings

- `file or clear the site-health issue` — the report step's old name, renamed `report the probe result` (delta 1).

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls site-kit/SPEC-*.md`) once the iteration's last site-kit amendment merges.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Fixture suites** — `--run-gate-tests site-kit/gate-tests`, gate-sdk's `check-action-*` suites, the consumer smoke and installer's suites run green.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, before the drain stage. No remote run is its oracle: the `off` branch is held by the probe test `site-health-probe-unexecuted` lands, never by a dispatched run.
