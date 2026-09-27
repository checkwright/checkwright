# SPEC amendment: comment-actions

`check-comment-tier` governs every full-line comment on the comment surface, and the surface never reaches a CI workflow. `comment_surface` (`native/src/spec.rs`) draws its files from `CANON_KIT_COMMENT_SURFACE`'s globs, or from `.sh`, `.gate` and `.rs` sources when the knob is empty, and adds the tracked tier of `GATE_SDK_WORKFLOW_DIR`, which is `.workflow/` and not `.github/workflows/`. A workflow is shell-bearing CI code, and its comments are the same tier as a script's. The operator asked for them to be governed on 2026-09-09, 2026-09-25 and 2026-09-27, under the standing direction "We should aim to have all technical assets covered by our gates" (operator direction, 2026-09-27). This amendment adds actions-shaped YAML to the surface by content, behind a consumer-selectable knob. This repo binds it on and sweeps its workflows.

**Why by content and behind a knob.** A glob cannot express the corpus. `**/*.yml` never descends into `.github`, because the glob walk keeps bash's leading-dot rule. It also catches `docs/_config.yml`, which is Jekyll configuration and not CI. `actions_shaped` (`native/src/actions_run.rs`) already names the corpus by content: a YAML file with a top-level `jobs:` or `runs:` key. `check-action-run-shell` and `check-action-run-path` read that corpus through `walk::find_files(root, ["yml", "yaml"])` filtered by the predicate. Governing it by default for every adopter would red each adopter's workflows on upgrade, and the calibration is a consumer's to choose (policy-as-choice). So the kit ships `off` and `on`, defaulting to `off`, and a consumer binds its own.

**Measured at authoring (2026-09-27), with the surface widened through a local overlay of `CANON_KIT_COMMENT_SURFACE` that spells `.github/workflows/*.yml` literally:**

- `check-comment-tier` findings per file: `.github/workflows/gates.yml` 953, `publish.yml` 147, `site-health.yml` 111, `site-kit/templates/site-health.yml` 98, `gate-sdk/templates/gates-workflow.yml` 35, `installer/action.yml` 0 (its one comment is a `spec:` directive). 1344 in all. `git ls-files '*.yml' '*.yaml'` read against `actions_shaped` finds exactly these six actions-shaped files outside `gate-tests/`. `.github/ISSUE_TEMPLATE/*.yml` and `docs/_config.yml` are not actions-shaped.
- **`check-spec-pointer`'s first reading of the widened surface: 0 findings.** The three `spec:` directives in the newly reached files (`installer/action.yml:1`, and `.github/workflows/publish.yml` at the `check-action-gh-repo` and §Versioning steps) resolve. `check-todo-task-liveness` and `check-deprecation-task`, the surface's other two readers, also report 0.
- The classifier (`native/src/gates/comment_tier.rs` `classify`) sends an unlisted extension to its default arm: `#` style, the shell roster, positional rescue on. So a `.yml` member is read as shell with no classifier change. It has no YAML structure, so a `#` line inside a `run: |` body is judged like any other full-line comment. That is correct, because such a line is a shell comment.
- The workflow valves `# gh-repo-exempt: <reason>` (gate-sdk/SPEC.md §check-action-gh-repo) and `# action-permissions-exempt: <reason>` (§check-action-permissions) are not on the built-in roster (`SHELL_COLON`). `git grep -n 'gh-repo-exempt:\|action-permissions-exempt:' .github installer gate-sdk/templates site-kit/templates` finds no use today.
- The four surface readers' declarations: `COMMENT_SURFACE_ROOTS` and `SPEC_POINTER_ROOTS` in `native/src/gates/mod.rs` declare the knob's glob branch and the `else:` extension branch. The four descriptors' `couples=` carry `*.sh,*.gate,*.rs`, the workflow tier and `knob:CANON_KIT_COMMENT_SURFACE`.

## What changes

### (1) The comment surface gains an actions tier behind `CANON_KIT_COMMENT_ACTIONS` {design-bearing}

**Not yet applied.** A new scalar knob, `CANON_KIT_COMMENT_ACTIONS`, takes `off` (the default) or `on`. Any other value refuses at exit 2 and names the knob and its two values. With `on`, `comment_surface` adds every actions-shaped YAML file under the root to the selected set in both branches, before the `templates/` rule, the kit-root prune and the sort. So every narrowing the corpus defines applies to the tier, as canon-kit/SPEC.md §The shared spec adapters requires of each branch. A file both a glob and the tier select appears once. With `off`, nothing is added and the surface is unchanged.

The tier is found by one helper, `actions_run::actions_files(root)`, which runs the walk and predicate `check-action-run-shell` and `check-action-run-path` each run today. Those two gates call it too, so the three readers cannot disagree about which files are actions-shaped. The helper keeps the gates' fail-closed behaviour: an unreadable file or an unwalkable root is exit 2 for every caller.

### (2) The knob is rostered and this repo binds it {mechanical}

**Not yet applied.**
- `native/src/knobs/canon_kit.rs` gains `Row::scalar("CANON_KIT_COMMENT_ACTIONS", "off")` beside `CANON_KIT_COMMENT_SURFACE`.
- In canon-kit/SPEC.md §Layout and configuration, the comment-surface bullet gains, after the `CANON_KIT_COMMENT_SURFACE` clause:

  > `CANON_KIT_COMMENT_ACTIONS` — `off` (default) or `on`: `on` adds every actions-shaped YAML file (`actions_shaped`, a top-level `jobs:` or `runs:` key) to the surface whichever branch selected it.

  The same bullet's "plus the `${GATE_SDK_WORKFLOW_DIR:-.workflow}/*.txt` state files" becomes "plus every tracked member of `${GATE_SDK_WORKFLOW_DIR:-.workflow}`". That brings it in line with §The shared spec adapters, which states the tier as "every tracked member … whatever its extension". The bullet predates that wording.
- `scripts/canon-config.knobs` sets `CANON_KIT_COMMENT_ACTIONS = on` under a `# spec: canon-kit/SPEC.md §check-comment-tier` directive.

### (3) The two workflow valves join the built-in roster {mechanical}

**Not yet applied.** `gh-repo-exempt:` and `action-permissions-exempt:` join `SHELL_COLON` in `native/src/gates/comment_tier.rs`, for the reason `path-dialect-exempt:` is there: each is kit mechanism, read at its own site by the gate whose section mints it. Without them, the first legitimate valve in a governed workflow reds as prose. canon-kit/SPEC.md §check-comment-tier's machine-directive list gains, after the `door-contributor:` clause:

> `gh-repo-exempt: <reason>` and `action-permissions-exempt: <reason>` are read at their own site by `check-action-gh-repo` and `check-action-permissions` and join the built-in roster on the same ground, gate-sdk/SPEC.md §check-action-gh-repo and §check-action-permissions;

### (4) The readers declare and couple the new walk {mechanical}

**Not yet applied.**
- `COMMENT_SURFACE_ROOTS` and `SPEC_POINTER_ROOTS` in `native/src/gates/mod.rs` each gain the unconditional root `(".", "ext:lit:yml,yaml", "", "")`. The walk runs only under `on`, and an unconditional declaration over-approximates it, which `check-reads-couples` accepts: it asserts observed ⊆ declared. The `else:` guard is the wrong form, because it marks a walk taken when a selector resolves empty, and this walk is taken when one resolves `on`.
- The four descriptors (`canon-kit/checks/check-comment-tier.gate`, `check-spec-pointer.gate`, `check-todo-task-liveness.gate`, `check-deprecation-task.gate`) add `*.yml,*.yaml,knob:CANON_KIT_COMMENT_ACTIONS` to `couples=`. The generated hooks and the graph artifact are regenerated with the commands `check-graph` prints.

### (5) The SPEC states the tier {mechanical}

**Not yet applied.** In canon-kit/SPEC.md §The shared spec adapters, the governed-comment-surface sentence "Its file set spans **both gate declaration spellings, the ported implementation, and the workflow directory's tracked tier**: `*.sh`, the `*.gate` descriptor, `*.rs`, and every tracked member of `GATE_SDK_WORKFLOW_DIR` whatever its extension." becomes:

> Its file set spans **both gate declaration spellings, the ported implementation, the workflow directory's tracked tier** and, under `CANON_KIT_COMMENT_ACTIONS=on`, every actions-shaped YAML file: `*.sh`, the `*.gate` descriptor, `*.rs`, every tracked member of `GATE_SDK_WORKFLOW_DIR` whatever its extension, and each file `actions_run::actions_files` returns, the same set `check-action-run-shell` lints.

In §check-comment-tier's calibration paragraph, the sentence opening "The default surface is shell (`#`) — `templates/` stubs included:" gains, between "exempts them as placeholders-by-design —" and "with the workflow directory's **tracked** members":

> an actions-shaped YAML member reads as shell too, a `#` line inside a `run:` body being a shell comment governed as one, since the classifier reads no YAML structure —

### (6) The fixture pair exercises the tier {mechanical}

**Not yet applied.** `check-comment-tier`'s `bad/` case gains an actions-shaped workflow carrying a prose comment, with a case-local knob file binding `CANON_KIT_COMMENT_ACTIONS = on`. Its `good/` case gains, under the same binding, an actions-shaped workflow whose comments are directives, a `run: |` body with a `# shellcheck disable=` line, and a non-actions `.yml` carrying prose, which is not read. A crate unit test in `spec.rs` asserts three things: `off` adds nothing, `on` adds exactly the `actions_files` set, and any other value refuses.

**Inferred, cannot run before build:** the classifier's heredoc skip matches a heredoc whose terminator is indented inside a `run:` body, and if it does not, the lines after such a heredoc are skipped, a coverage loss rather than a false red — only the new `good/` case, which build adds with an indented heredoc, exercises the skip on a workflow, and build fixes the skip if that case shows the loss.

### (7) This repo's actions-shaped files are swept to directives {design-bearing}

**Not yet applied.** The five files with findings are swept until `check-comment-tier` is clean: `.github/workflows/gates.yml`, `publish.yml`, `site-health.yml`, `gate-sdk/templates/gates-workflow.yml` and `site-kit/templates/site-health.yml`. Each comment run takes the first rule that fits it:

1. **Restates** the step's name, its code, or a SPEC section it could cite instead: delete it.
2. **Carries a why a SPEC section owns**: replace it with a one-line `# spec: <SPEC> §<section>` binding. Where the pointer would sit on a `run:` line or a `uses:` line, it goes on the line above the step.
3. **Carries a why no SPEC owns and a later reader needs**: rewrite the owning SPEC section to carry it, re-phrased into that section and not appended (canon-kit/SPEC.md §Merging an amendment's rule applied to prose). Then point at it as in rule 2. Measured run ids, dates and "measured on run N" provenance go to neither place. Git history keeps them, and a kit SPEC may not carry them (gate-sdk/SPEC.md §The provenance seam).
4. **States a genuinely local fact below SPEC altitude**: `# comment-tier-exempt: <reason>`, used sparingly. An exemption that blesses a restatement is the defect canon-kit/SPEC.md §check-comment-tier names.

A template's comments are an adopter's instructions. They take rule 2 against the kit SPEC the template ships with, and a template's `spec:` line resolves against the vendored kit path.

**The sweep's oracle is local and mechanical, and it runs before the push.** For each swept file, the file with every full-line comment stripped (lines whose first non-blank character is `#`) is byte-identical before and after the sweep, and `ruby -ryaml -e 'YAML.load_file(ARGV[0])'` parses it. So a comment edit cannot change what a workflow runs, and `publish.yml`, which runs only on a tag, and `site-health.yml`, which runs only on its schedule, are verified before either workflow runs again. `check-action-pinning`, `check-action-permissions`, `check-action-run-shell` and `check-action-run-path` stay green in the same battery.

## Producers and consumers

- **`CANON_KIT_COMMENT_ACTIONS`.** Producer: the consumer's `canon-config.knobs`, which this repo sets to `on`. The kit's own template sets nothing, so an adopter resolves `off`. Consumers: `comment_surface`, on every run of its four callers. Roster-holding readers of the new name: the knob table in `canon_kit.rs` (delta 2), which `--emit knob-roster` and `check-knob-citation` read; the four descriptors' `couples=` (delta 4); and canon-kit/SPEC.md §Layout and configuration (delta 2).
- **`actions_run::actions_files`.** Producer: the one walk over `yml`/`yaml` under the root, pruned by `GATE_SDK_PRUNE_DIRS`, filtered by `actions_shaped`. Consumers: `comment_surface` under `on`, `check-action-run-shell` and `check-action-run-path`. For the two action gates this is a refactor and their corpus is unchanged.
- **The two roster tokens.** Consumer: `comment_tier.rs`'s blessing. Their meaning stays with the gates that read them.
- **Point 5.** No corpus narrows. The surface widens for four readers. `check-todo-task-liveness` and `check-deprecation-task` red only on a marker they find, and none exists in the new files (measured 0). `check-spec-pointer` reds on an unresolved pointer, and the three it finds resolve (measured 0). `check-comment-tier` reds on every undirected comment, which is what delta 7 clears.
- **Point 6.** The corpus the sweep obliges is enumerable: the six actions-shaped files above. Each one's satisfying value is `check-comment-tier` clean over it. For `installer/action.yml` that is already true.
- **Ordering with the sibling units.** Scope ordered this unit before the iteration's new CI legs: `crate-tests-other-triples`, and the musl legs `linux-musl-artifacts` and `glibc-floor-lowering` add. So those legs are written under the gate. If one lands in the same build batch as this unit, its job comments are written as directives in its own commit. If it lands in a later batch, the gate reds any prose it adds.
- **Oracle.** The battery at commit, the strip-and-compare and YAML parse above before the push, and the mid-iteration push's `gates` run for `gates.yml`.

## Existing sections updated

Roster from `grep -n "comment_surface(" -r native/src`, `git grep -n "CANON_KIT_COMMENT_SURFACE" -- canon-kit native scripts`, `grep -n "SHELL_COLON" native/src/gates/comment_tier.rs` and the widened-surface gate runs above, run 2026-09-27.

- `native/src/spec.rs` `comment_surface`, `native/src/actions_run.rs`, `native/src/gates/action_run_shell.rs` and `native/src/gates/action_run_path.rs` (delta 1).
- `native/src/knobs/canon_kit.rs`, canon-kit/SPEC.md §Layout and configuration, and `scripts/canon-config.knobs` (delta 2).
- `native/src/gates/comment_tier.rs` and canon-kit/SPEC.md §check-comment-tier (deltas 3 and 5).
- `native/src/gates/mod.rs` and the four descriptors in `canon-kit/checks/` (delta 4).
- `scripts/git-hooks/pre-commit`, `scripts/git-hooks/commit-msg` and the graph artifact, regenerated (delta 4).
- canon-kit/SPEC.md §The shared spec adapters (delta 5).
- `canon-kit/gate-tests/check-comment-tier/` and `native/src/spec.rs` tests (delta 6).
- `.github/workflows/gates.yml`, `.github/workflows/publish.yml`, `.github/workflows/site-health.yml`, `gate-sdk/templates/gates-workflow.yml`, `site-kit/templates/site-health.yml`, and whichever SPEC sections rule 3 of delta 7 rewrites (delta 7).
- The on-site mirror of `canon-kit/SPEC.md`, and of any SPEC delta 7 rewrites, regenerated by `check-docs-mirror-fresh`'s printed command (deltas 2, 3, 5 and 7).
- `.workflow/release-declarations.md` gets two Behavior changes bullets. The first: `CANON_KIT_COMMENT_ACTIONS` is new and set to `on` extends `check-comment-tier`, `check-spec-pointer`, `check-todo-task-liveness` and `check-deprecation-task` to every actions-shaped YAML file; it defaults to `off`, so nothing changes until you set it. The second: the `gh-repo-exempt:` and `action-permissions-exempt:` valves are now blessed directives (deltas 1, 2 and 3).

## Retired spellings

- None — no delta renames or deletes a spelling. The §Layout bullet's `/*.txt` clause is a stale description corrected in place, not a spelling any surface carries.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness check holds for the knob, the helper and the two tokens.
- [ ] **Instruction surfaces: instruction only.** The two swept templates carry directives and no grounds.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines, and each rule-3 relocation rewrites its target section rather than appending.
- [ ] **Amendment deleted.** This file is removed on merge (`ls canon-kit/SPEC-*.md`).
- [ ] **Entry moved.** `comment-tier-surface-excludes-ci-workflows` moves to Done after the mid-iteration push's `gates` run is read green over the swept `gates.yml`, at a stage before the drain stage. `publish.yml` and `site-health.yml` are held by the local strip-and-compare oracle, since neither runs on a push.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the sweep goes to the gap inbox.
