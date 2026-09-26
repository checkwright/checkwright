# SPEC amendment: done-claim-demo

The front door promises that unsupported *done* claims become failing checks, and `checkwright demo` shows a mistyped relative link caught instead (§demo, act 3). This amendment makes act 3 the claim the headline names: a task reported done, with the stage stamps saying validate ran and the iteration closed, and no evidence behind either. The battery the `full` install registers reds on it, and act 4 withdraws the claim and returns to green.

**The ruling: the claim is caught by evidence coupling, not by a stage stamp.** A fresh `full` install registers no lifecycle-kit gate, since every one is `on-surface` or `never` (§What init seeds), so `check-stage-evidence` is not in the battery the demo runs. It would not catch this claim anyway: it asserts stamp grammar and name-axis agreement, and its own honest limit hands the green result to evidence-kit (lifecycle-kit/SPEC.md §check-stage-evidence). The member that reds is evidence-kit's `zero-config` `check-evidence-manifest`, assertion C: a validate stamp with no evidence line, once the cursor has moved past validate (evidence-kit/SPEC.md §check-evidence-manifest). Assertion C does not read `EVIDENCE_KIT_SUITES`, so it arms on a fresh install's empty suite roster. The act still names no gate: it is judged by what the red says, not by which member said it.

**Refused: act 4 records the evidence** by running `--run-validate`. The spine refuses an empty suite roster, so the scratch would first need a suite roster, a run command and a baseline row, which is consumer configuration act 1 says an adopter's own `init` does not write. It would also run the battery a second time inside the spine. Withdrawing the claim is the fix on the value arm's ground: the fix is the defect, never the corpus or the gate.

**Refused: keeping the link defect as a further act.** It adds two battery runs to the first minute and shows none of the three failures the headline names. The consumer smoke's value arm keeps the link defect on every profile (§The consumer smoke), so the link gate stays proven where it was proven before.

**Measured at authoring:**

- **Which gates a `full` install registers.** `grep -l "^# install: zero-config" */checks/*.gate` lists `evidence-kit/checks/check-evidence-manifest.gate` and no `lifecycle-kit/checks/` member; `lifecycle-kit/checks/check-stage-evidence.gate` carries `# install: on-surface`.
- **Assertion C arms with no suite configured.** `native/src/gates/evidence_manifest.rs` tests `have_validate && stage != "validate" && !have_line_for_iter` with no read of the suite roster. Driven over a scratch queue named `demo`, a state file whose last stamps are `demo validate` then `demo close`, and a header-only manifest (`run-gates.sh --only check-evidence-manifest -- <manifest> <queue> <state>`), it exits 1 and prints `iteration 'demo' has a validate stamp but no evidence line — validate ran and recorded nothing`.
- **What a fresh `full` install seeds.** `native/src/installer/recipe.rs` seeds the state file header-only under `.workflow/WORKFLOW-STATE.txt` and the manifest header-only; the queue comes from `queue-kit/templates/TASK-QUEUE.md`, which carries `## Iteration: —` and a `## Done` section. With no stamp, assertion C is disarmed, which is why act 2 is green today.
- **The demo is unreleased.** `.workflow/release-declarations.md` still carries the verb's new-verb declaration under `## Behavior changes`, so its arc is rewritten there rather than declared a second time.

## What changes

### (1) The contract: act 3 is a done claim, act 4 withdraws it {design-bearing}

**Not yet applied.** In installer/SPEC.md §demo, act 1's sentence "It is chosen because it reddens on the defect below where the lattice minimum does not." stays. Items 4 and 5 of **The arc** become:

> 4. **Act 3, a caught done claim.** The verb commits what an agent claiming done writes, and no evidence: the queue's iteration header names a demo iteration, the `## Done` section gains a demo task, and the state file gains that iteration's `validate` stamp and then its `close` stamp, each well-formed (lifecycle-kit/SPEC.md §The state machine) with `HEAD`'s abbreviation as `<head>`. The files are the queue and state file `init` seeded. A seeded queue carrying no iteration header or no `## Done` section cannot stage the act, so that is the env verdict. The battery must exit non-zero, and some reddened block must name the demo iteration. **No gate is named**: the claim is that the done claim was caught, not which member caught it. Every reddened block is quoted back through its invariant line by `--run-demo`'s excerpt rule. The commit passes `--no-verify`, because the battery run is what the act shows catching it.
> 5. **Act 4, withdrawn.** The verb restores the queue and state file to the contents act 3 found, commits, and re-runs the battery, which must be green again. The fix is the claim, never the corpus or the gate.

In the verdict paragraph, "the defect is not caught" becomes "the done claim is not caught", and "a failed `git`, or `init` exiting non-zero" becomes "a failed `git`, a seeded queue act 3 cannot stage, or `init` exiting non-zero".

### (2) The verb stages the claim {design-bearing}

**Not yet applied.** In `native/src/installer/demo.rs`, the link defect's constants and act 3 are replaced:

- Act 3 reads the seeded queue at `QUEUE_FILE` and the seeded state file, and keeps both bodies for act 4. The state file's path becomes a constant in `native/src/installer/mod.rs` beside `QUEUE_FILE`, read by both the lifecycle-kit seed arm in `recipe.rs` and this verb, so its path is spelled once.
- It rewrites the `## Iteration:` line to name the demo iteration, adds the demo task as a done bullet (`- <slug>`, queue-kit/SPEC.md §check-task-conservation's done grammar) under `## Done`, its slug one the seeded queue does not carry, since `check-task-names` holds slugs unique across the file, and appends the `validate` and `close` stamps. The session-id field is a fixed demo token, the date is today's, and `<head>` comes from `git rev-parse --short HEAD`, so `git` stays the verb's only spawned program besides itself. A missing header or `## Done` heading is `Outcome::Refuse`.
- The content assertion keys on the demo iteration's name in the quoted blocks, where it keyed on `README.md`. Its failure line says the done claim did not turn the battery red, or that the battery went red without naming the claimed iteration.
- Act 4 writes both kept bodies back and commits through the existing `commit` helper.
- The usage text, the act 3 and act 4 banners and narration, and the closing arc line say *done claim caught* and *withdrawn* where they said *defect caught* and *fix*. The `// spec:` directives above the constants and `commit` are restated for the claim.

### (3) The consumer smoke reads the red in the output {mechanical}

**Not yet applied.** In installer/SPEC.md §The consumer smoke, the demo arm's assertion list gains, after the `DEMO: clean` bullet:

> - a `FAIL:` verdict line between the act 3 and act 4 banners, so the red is read from the output rather than inferred from the verb's own exit;

`installer/consumer-smoke/run-smoke.sh`'s demo arm gains that assertion as its own named failure line, beside the `DEMO: clean` check. The `FAIL:` tail is the runner's output contract (gate-sdk/SPEC.md §run-gates), so the smoke couples to no gate name.

### (4) The unreleased verb's declaration {mechanical}

**Not yet applied.** In `.workflow/release-declarations.md`, the `checkwright demo` declaration's "a green battery, one mistyped link caught, the fix" becomes "a green battery, a task marked done with no validate evidence caught, the claim withdrawn".

### (5) The install page's arc {mechanical}

**Not yet applied.** docs/install.md's sentence "Add `demo` in place of `init` to watch the whole arc — install, a green battery, one mistyped link caught, the fix — in a scratch repository first; it installs nothing." depends on the sibling queue entry `front-door-demo-unreachable`, whose deliverable collapses docs/install.md's demo descriptions into one paragraph under §Install.

- **Sibling in the same build batch:** the collapsed paragraph describes act 3 as a done claim with no evidence, caught, and act 4 as the claim withdrawn. This delta writes nothing of its own.
- **Sibling not in the batch:** "one mistyped link caught, the fix" becomes "a task marked done with no evidence caught, the claim withdrawn", and the rest of the sentence stays.

## Producers and consumers

- **The act 3 claim commit.**
  - Producer: the `demo` verb, in its own scratch repository after act 2. Enabled on every run; the verb takes no operand.
  - Consumer: the battery `init` registered, through `check-evidence-manifest`, which `full` registers because it is `zero-config` in evidence-kit. It reads the queue header for the iteration and the state file's last stamp for the cursor, both at evidence-kit's defaults, which are the paths `init` seeded. The verb's content assertion reads the quoted blocks for the iteration name.
  - Fields: the iteration name is read by assertion C's finding and by the content assertion. The done bullet is read by no gate; it is the narrated claim, and `check-task-conservation` passes it, since it conserves live slugs only. The `validate` stamp arms assertion C and the `close` stamp moves the cursor past it. `<head>` and the date keep the stamps well-formed for a reader that registers `check-stage-evidence` later; no member the demo runs reads them.
- **The act 4 restore commit.**
  - Producer: the verb, from the bodies act 3 kept.
  - Consumer: the battery, which must be green; the state file has no stamp again, so assertion C is disarmed.
- **The smoke's `FAIL:` assertion.**
  - Producer: the verb's quoted excerpt, whose blocks end at the verdict line (gate-sdk/SPEC.md §Consumer smoke owns the excerpt rule, and §run-gates the reserved two-space verdict prefix).
  - Consumer: the demo arm of `run-smoke.sh`, in the `installer_smoke` validate suite.
- **Downstream reader.** The queue entry `front-door-demo-unreachable` captures its proof block from act 3's output, so this unit lands first or with it.

## Existing sections updated

Roster probe: `git grep -n -i -e "one mistyped link" -e "value-arm defect" -e "defect caught"` over the tracked tree, plus `git grep -n "WORKFLOW-STATE.txt" -- native/src/installer`.

- `installer/SPEC.md` — §demo, acts 3 and 4 and the verdict paragraph (delta 1); §The consumer smoke, the demo arm's assertion list (delta 3).
- `native/src/installer/demo.rs` — the constants, act 3, act 4, the usage text and the closing line (delta 2).
- `native/src/installer/mod.rs` — the state-file path constant beside `QUEUE_FILE` (delta 2).
- `native/src/installer/recipe.rs` — the lifecycle-kit seed arm reads that constant (delta 2).
- `installer/consumer-smoke/run-smoke.sh` — the demo arm's `FAIL:` assertion (delta 3).
- `.workflow/release-declarations.md` — the `checkwright demo` declaration (delta 4).
- `docs/install.md` — the demo sentence, or the sibling's collapsed paragraph (delta 5).
- `docs/installer/SPEC.md` — the generated mirror, regenerated by the command its freshness gate prints (deltas 1 and 3).

## Retired spellings

- `one mistyped link` — the demo's arc description (deltas 2, 4 and 5).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The walkthrough ran** — `bash installer/consumer-smoke/run-smoke.sh` green, its demo arm printing the new assertion; the entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`), once that run is read.
