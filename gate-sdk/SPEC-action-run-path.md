# SPEC amendment: action-run-path

A GitHub Actions `run:` step that invokes a deleted script reds nowhere, and CI finds it on the push that runs the step, which spends a push-budget round. This happened at `native-shell-guard`: after that iteration deleted `guard-kit/gate-tests/guard-read-path.test.sh`, the `install-smoke-sh-windows` leg of `.github/workflows/gates.yml` kept the step `run: bash guard-kit/gate-tests/guard-read-path.test.sh`. The full battery was green, and the close's stale-identifier audit found the step and removed it (`git show 7dc96344 -- .github/workflows/gates.yml`). The step was a **single-line plain scalar**, which is the one `run:` form §check-action-run-shell counts and does not read.

**The ruling: a new gate-sdk member, `check-action-run-path`, holds path existence over `run:` bodies.** It reuses two predicates that already exist: §check-action-run-shell's extractor, for the bodies and their dialect, and canon-kit/SPEC.md §check-docs-cmd assertion (A)'s invocation predicate, for which token is an invoked script. Two other homes were weighed and refused.

- **A second failure class on `check-action-run-shell`.** That gate spawns `shellcheck` and refuses at exit 2 where it is absent (§check-action-run-shell, criterion 7), so a consumer without the linter would lose a check that needs no linter. Its invariant is lint-cleanliness, and a gate whose name and reach disagree teaches the wrong boundary, which is that section's own ground for its scan narrowing.
- **A widened corpus for `check-docs-cmd`.** Its corpus is the governed doc set. An Actions file is gate-sdk's subject, which canon-kit neither ships a template for nor owns a contract over.

**Measured at authoring (2026-09-24):**

- **The live `run:` invocations all resolve.** `grep -nE "[A-Za-z0-9_./-]+\.sh\b" .github/workflows/*.yml gate-sdk/templates/gates-workflow.yml site-kit/templates/site-health.yml` lists every `.sh` token. The invoked ones are `gate-sdk/lib/gate.sh`, `gate-sdk/bin/build-native.sh`, `gate-sdk/bin/run-gates.sh`, `scripts/ci-macos-floor.sh`, `scripts/ci-build-artifact.sh` and `installer/consumer-smoke/run-smoke.sh`, and `ls` finds each. The rest are runtime paths under a variable (`"$probe/s.sh"`), which the invocation predicate does not take. The gate registers green.
- **No step sets a working directory.** `grep -n "working-directory" .github/workflows/*.yml gate-sdk/templates/*.yml site-kit/templates/*.yml` returns nothing.
- **Single-line values take quoted forms too.** `gate-sdk/templates/gates-workflow.yml` carries `run: "$GATES"`, a double-quoted scalar.

## What changes

### (1) `check-action-run-path` — every invoked script in a shell `run:` body resolves {design-bearing}

**Not yet applied.** A new gate-sdk/SPEC.md section, `### check-action-run-path`, after §check-action-run-shell:

> Invariant: in every Actions-shaped YAML file, every repo-relative `.sh` path in **invocation position** in a `run:` body whose step runs a shell resolves to a file under the scan root. The class this closes is a step still invoking a script that a later change deleted or renamed. The battery stays green on it because nothing reads a workflow's commands as paths, so CI finds it on the push that runs the step.
>
> **Scan set, subject and bodies are §check-action-run-shell's, consumed whole.** That means the derived `*.yml` / `*.yaml` walk from the scan root with the shared prune set, and the Actions-shape predicate. It also means the extractor with its refusals, its GitHub-expression substitution and its two-axis dialect resolution. A body is read when its resolved dialect is a shell (`bash`, `sh`, `dash`, `ksh`). Any other body is skipped and counted: a `pwsh` or `python` body, a Windows step with no `shell:`, or a step whose `runs-on` cannot be read. The unresolved dialects are §check-action-run-shell's findings already, and one defect owes one reporting gate. The bodies are the literal block scalars and the **single-line values**. A single-line value is read as a plain scalar up to its first ` #` comment, or as a single- or double-quoted scalar with its quotes removed. A double-quoted value carrying a backslash escape is skipped and counted, because unescaping it is the YAML parsing the extractor declines.
>
> **Which token is a script is canon-kit's predicate, called and never copied** (canon-kit/SPEC.md §check-docs-cmd, assertion A). The token is the first word of each `;`/`|`/`&`-separated segment, or the first non-flag argument when that word is `bash`, `sh`, `source` or `.`, and it must be a two-or-more-segment path ending in `.sh`. So a path in argument position, such as a `cp` destination or a file a step writes, is never a finding, and a path under a variable or a GitHub expression carries a `$` and is not a token.
>
> **Resolution is against the scan root**, the checkout root a step's shell starts in. A token resolves when `<scan root>/<token>` is a regular file. A step carrying a `working-directory:` key resolves against that directory joined to the root when the value is literal, and is skipped and counted when the value carries `${{`.
>
> **Red** (exit 1) is one finding per unresolved invocation, `<file>:<line>: run: invokes <path>, which does not resolve under <root>`, where the line is the body line that carries it, with a `help:` line naming the remedy: repoint the step at the script that replaced it, or delete a step whose script is spent. **Clean** reports the bodies read, the invocations resolved, and the skipped count by cause. **Fail-closed** (exit 2) on the extractor's own refusals, an unreadable walked file, and a scan root that is not a directory. A tree holding no YAML exits clean on a zero count, so a consumer running no GitHub Actions pays nothing for it.
>
> Tier `precommit`, **no knob**, **no valve**. Every finding is discharged by repointing or deleting the step, and an exemption marker would only keep a broken step. The `# graph:` couples the walked YAML surface, like §check-action-run-shell's, and triggers on `*.sh` too, since renaming a script is the edit that strands a step. The trigger reaches a rename's new path and a modified script; a *deleted* script is a staged `D`, which the generated hook's `ACMR` filter does not match, so the deletion itself is caught by the full battery and by CI. That is the same partial route context-kit/SPEC.md §check-settings-paths records for its reverse trigger. **Born native**: a crate-carrying tree births a gate native, and none of the three exception classes applies. `install: zero-config`, because the subject is whatever YAML the adopter's tree holds.
>
> The `good/`+`bad/` pair is its oracle. `bad/` has a block-scalar invocation and a single-line plain invocation of an absent script, each beside a resolving one, so a broken body arm shows as a missing finding. `good/` exercises resolving invocations in both forms, a quoted single-line value, a `pwsh` step and an unshelled Windows step (both skipped), a path in argument position, a `$`-rooted path, and a literal `working-directory:`.

### (2) The `run:` extractor becomes shared, and reads single-line values {design-bearing}

**Not yet applied.** In gate-sdk/SPEC.md §check-action-run-shell, the extractor paragraph's opening "**The extractor** is one awk pass per file, keyed on block-scalar indentation, and stays inline in the check script rather than moving to `lib/` — a helper earns its place at a second consumer and **this extractor** has none. The clause is scoped rather than absolute, because the tree now holds one of each answer:" becomes:

> **The extractor** is one pass per file, keyed on block-scalar indentation. It lives in shared crate code because it has two consumers, this gate and §check-action-run-path, on the rule that a helper earns its place at a second consumer. Both of the tree's walks over Actions files have now met that rule:

The rest of that sentence, about §check-action-gh-repo's walk, stands. The later paragraph "**The extractor is the port's content, and its two standing rulings moved with it.**" keeps its dialect ruling and drops the *stays inline* ruling, which this delta discharges.

The *Out of ability* paragraph's plain-scalar sentence becomes:

> **Single-line `run:` values** are extracted, with their text read as §check-action-run-path states, and are not linted. The text is recovered well enough to find an invoked path but not to lint as shell, because a plain scalar's YAML rules (a space-preceded `#` opens a *YAML* comment) differ from a shell's, and reading them exactly means parsing the scalar, the dependency this gate declines. They are counted in the output so the cost is visible on every run, and the class that produces incidents is multi-line blocks.

In the crate, the extractor moves from `native/src/gates/action_run_shell.rs` to a shared module beside `native/src/actions.rs`. A single-line item carries its text, its line and the same step-and-job dialect resolution a block carries, and a step's literal `working-directory:` value is captured beside its `shell:`. `check-action-run-shell`'s verdict does not move: its lint set is still the block scalars, and its clean line's counts are unchanged.

### (3) canon-kit's invocation predicate has a second reader {mechanical}

**Not yet applied.** In canon-kit/SPEC.md §check-docs-cmd, after assertion (A)'s calibration sentence ("…with no whole-file exemption."):

> The predicate has a second reader, gate-sdk's `check-action-run-path`, which applies it to `run:` bodies (gate-sdk/SPEC.md §check-action-run-path). That reader calls this gate's holder rather than restating it, so a change to the scoping above changes that gate's too.

In `native/src/gates/docs_cmd.rs`, the segment split and the invoked-script test are exported to the crate (`pub(crate)`) under one function, which returns the invoked tokens of one command line.

### (4) The gate is registered and its projections regenerated {mechanical}

**Not yet applied.** The gate is registered in `scripts/gates.list` and in gate-sdk's `smoke/install.sh` registry heredoc, and ships `gate-sdk/checks/check-action-run-path.gate` and `gate-sdk/gate-tests/check-action-run-path/{good,bad}/`. The README's gate-roster block and the new-gate fan-out in `docs/site-architecture.md` are then regenerated or brought level.

## Producers and consumers

- **The finding (delta 1).**
  - Producer: `check-action-run-path`, `precommit`, registered here. Its enabling configuration is none, since the scan set is derived, so every deployed tree that registers it reaches it.
  - Consumers: the committing session, through the output contract on the generated hook; `run-gates.sh`; CI. The fixture pair is read through `--run-gate-tests`.
- **The shared extractor (delta 2).** Its producers are the extractor passes. Its consumers are the two gates. Its single-line text field is read by `check-action-run-path` alone. `check-action-run-shell` reads the item's count only, as it does today, so that field has one named reader. The `working-directory` field has one reader too, the new gate.
- **The exported predicate (delta 3).** Two callers: `check-docs-cmd` (A), and the new gate.
- **Roster-holding readers of the new name (point 2).** `scripts/gates.list` and the smoke registry heredoc (`check-gate-substrate-parity` assertion J). `check-gate-fixture-coverage` reads the fixture pair. `check-install-disposition` reads the descriptor's `install:` field. The README gate-roster block has `check-readme-roster` as its reader. The enforcement map rows the gate. None of these is a comment directive.
- **Point 5.** No corpus narrows. `check-action-run-shell`'s lint set is unchanged. Its clean line's counts are unchanged, because the extractor already counted single-line values.
- **Point 6.** Not obliged beyond the live tree, whose invocations are measured above as resolving.

## Existing sections updated

The roster comes from `git grep -n "stays inline\|second consumer\|Single-line plain-scalar\|invoked_script\|scan_a" -- '*.md' '*.rs'` and the new-gate fan-out in `docs/site-architecture.md`, both run 2026-09-24.

- `gate-sdk/SPEC.md` — the new §check-action-run-path (delta 1), and §check-action-run-shell's extractor, port and fidelity paragraphs (delta 2).
- `native/src/gates/action_run_path.rs`, new, with its registry row in `native/src/gates/mod.rs` (delta 1).
- `native/src/gates/action_run_shell.rs` and the new shared extractor module (delta 2).
- `canon-kit/SPEC.md` §check-docs-cmd (delta 3).
- `native/src/gates/docs_cmd.rs` (delta 3).
- `gate-sdk/checks/check-action-run-path.gate` and `gate-sdk/gate-tests/check-action-run-path/` (delta 4).
- `scripts/gates.list` and `gate-sdk/smoke/install.sh` (delta 4).
- `gate-sdk/README.md`, its gate-roster block (delta 4).
- `docs/enforcement.md`, `docs/value.md`'s rollup block, `docs/check-graph.html` and the generated hooks, regenerated by their gates' printed commands (delta 4).
- `.workflow/surface-ceiling.txt`, re-baselined if `check-surface-ratchet` reds on the page's growth (delta 4).
- `.workflow/release-declarations.md` (delta 1). Under Tightened gates: `check-action-run-path` — new: a script a shell `run:` step invokes must exist in the tree.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`, `docs/gate-sdk/README.md` and `docs/canon-kit/SPEC.md`.

## Retired spellings

- None — the deltas add a gate and move a module, and retire no name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the finding, the shared extractor and the exported predicate.
- [ ] **Instruction surfaces: instruction only.** No instruction surface is edited.
- [ ] **Merged with no information lost.** The new section carries the ruling's grounds, and the run-shell paragraphs are re-phrased rather than appended to.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `workflow-run-path-unresolved` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
