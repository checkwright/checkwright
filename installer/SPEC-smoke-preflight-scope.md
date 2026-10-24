# SPEC amendment: smoke-preflight-scope

Pairs `smoke-whole-tree-precondition-unscoped`. The entry's operator direction (2026-10-24, lead-relayed, not a ruling) reopens the leave-it-filed and asks this stage to rule between narrowing the consumer smoke's clean-tree preflight (B) and ruling the whole-tree predicate correct (C). **The ruling is B, narrowed by exclusion rather than inclusion:** the preflight stops reading the workflow directory and still reads every other path.

**What the tree does today**, read at spec:

- The preflight is one `git status --porcelain` over the whole tree, before the first arm (`preflight` in `native/src/emit/installer_smoke/roster.rs`). Its refusal reads "the worktree is dirty — the pack step refuses to stamp a commit the payload does not match". That ground has fallen: the packer's own refusal is footprint-scoped and re-runs on every pack (§The packer), so it already covers the stamp.
- **The smoke carries no post-run clean assertion over the tree under test.** The entry's 2026-09-10 premise, that B "moves together" with such an assertion, does not hold. `grep -n porcelain native/src/emit/installer_smoke/*.rs` finds the preflight as the one status read rooted at the tree under test. Every other one is rooted at a scratch consumer. §The consumer smoke's sentence "The smoke writes nothing inside the worktree beyond the footprint witness's plant" describes what the smoke writes; it asserts nothing.
- **C's ground is real, but it does not reach the whole tree.** Outside the payload's footprint the smoke reads oracle inputs straight from the worktree. A dirty one would test the committed payload against expectations the commit does not carry, a false verdict in either direction. Probed with `grep -n 'state.root' native/src/emit/installer_smoke/*.rs`:
  - `docs/index.md` (the profile loop);
  - `docs/speckit.md`, `docs/openspec.md` and `companion/` (the companion arm);
  - the crate directory `GATE_SDK_NATIVE_CRATE` names (the build arm's freshness stamp);
  - each held kit's canonical spec, which the payload withholds (`expected` in `carry.rs`, the close-surface arm's expected set);
  - the knob files the binary reads at the toplevel.

  These are scattered and unenumerated. A read roster held beside them would under-refuse silently the day an arm reads a new path, which is the failure §The packer's derived footprint exists against.
- **The workflow directory is read by nothing the smoke runs.** No smoke arm reads the tree's workflow directory: `WORKFLOW_DIR` in `consumer.rs` names the scratch consumer's. Nor is it a footprint member (§The packer). Yet it is the one directory sessions write mid-iteration: the gap inbox, the survey record, and the evidence manifest a red `--run-validate` folds (evidence-kit/SPEC.md §Evidence manifest). That fold is this entry's recurrence.
- `git status --porcelain -- ':(top)' ':(top,exclude)<dir>'` lists every dirty path outside `<dir>`, untracked directories included, and lists nothing when only `<dir>` is dirty. Probed at spec in a scratch repository, from the root and from a subdirectory.

**Why exclusion and not inclusion.** An exclusion errs toward refusing: a writer the predicate does not know about still refuses, which is today's behaviour, a cost and never a false verdict. An inclusion roster errs toward passing a dirty read. Under fail-closed (gate-sdk/SPEC.md §Fail-closed contract), the predicate drops only a set it can name and show unread.

## What changes

### (1) The preflight passes a dirty workflow directory {design-bearing}

In `native/src/emit/installer_smoke/roster.rs`, the dirty-tree check moves into a function the preflight calls. It takes the root and the resolved workflow directory, and returns the dirty entries outside that directory. **Not yet applied.**

- **The excluded member** is `GATE_SDK_WORKFLOW_DIR`, read through the binary's knob resolution (`walk::knob_scalar`), as the smoke reads every knob of the tree it runs in. It is excluded only when the value names a path strictly below the toplevel. Such a path is relative, non-empty, not `.`, and climbs out through no `..` component. Any other value, including a missing directory, excludes nothing, and the check is today's whole-tree one.
- **The call** is `git -C <root> status --porcelain -- ':(top)' ':(top,exclude)<dir>'`, and `':(top)'` alone when nothing is excluded.
- **The refusal** stays exit 2, before the first arm. It reads `the worktree is dirty outside <dir> — the smoke reads the tree under test beyond the payload, so a dirty path there makes its verdict describe a tree no commit holds. Commit or stash first.`, with ` outside <dir>` dropped when nothing is excluded. Then it names the entries in the packer's bounded form: `DIRTY_SHOWN` of them (`native/src/emit/pack_installer.rs`, 10 today), then `(and <n> more)`. The bound is shared, not held as a second constant.
- A `# spec: installer/SPEC.md §The consumer smoke` directive above the function names the excluded member.

### (2) Unit tests hold the predicate {mechanical}

`roster.rs`'s test module gains one test over a scratch repository: `git init`, a committed file at the root and one under `.workflow/`. **Not yet applied.** Each assertion is a named red:

- a modified and an untracked file under `.workflow/` alone leave the check clean;
- a modified root file then refuses, naming it, and an untracked directory outside `.workflow/` is named too;
- with the excluded directory given as `.`, as `../x` and as the empty string, the same `.workflow/` edit refuses.

The scratch repository is made under `std::env::temp_dir()`, as the module's log test makes its file there.

### (3) §The consumer smoke's preflight paragraph {mechanical}

Replacement for `installer/SPEC.md` §The consumer smoke, the paragraph opening **The tree must be clean, and the preflight says so before the first arm.** **Not yet applied.**

> **The tree under test must be clean outside the workflow directory, and the preflight says so before the first arm.** Beyond the payload, the smoke reads oracle inputs straight from the worktree: front-door pages, the companion fixtures, the kits' canonical specs, the crate's source stamp and the knob files. A dirty path among them would test the committed payload against expectations no commit carries. Those reads are scattered, and a roster of them would pass a dirty read the day an arm adds one, so the predicate excludes rather than includes. It passes the one directory no arm reads and every session writes, the one `GATE_SDK_WORKFLOW_DIR` names when that is a path below the root, and refuses any other dirty path, naming the entries. A writer elsewhere still refuses: a cost, never a false verdict. Beside that check the preflight requires `git`, `npm` and `tar`, plus `sh` on a unix host or `pwsh` on native Windows, and refuses at exit 2 on any absent one. It requires no `cargo`, `rustc` or `jq`: the driver builds nothing and parses the manifest and `package.json` with the crate's JSON reader. The packer keeps its own footprint-scoped refusal, since a concurrent edit can trip it on any of the run's several packs. That late refusal names the root it resolved and states that the check is per-invocation, so it reads as a precondition met at an awkward moment rather than as a broken installer.

### (4) The two sentences resting on the old ground {mechanical}

Each a replacement in `installer/SPEC.md`. **Not yet applied.**

- §The consumer smoke, in the paragraph opening **It reads three environment values undeclared**, the last sentence "So the clean-worktree precondition keeps its meaning, which the pack step needs, refusing to stamp a commit the payload does not match." becomes: "So the preflight's verdict on the tree under test still holds at the last arm."
- §The packer, the paragraph opening **A dirty path the payload neither ships nor reads does not abort the pack**:

  > **A dirty path the payload neither ships nor reads does not abort the pack.** `.workflow/gap-inbox.md`, which mid-iteration sessions write, is one. The consumer smoke's preflight, which every validate run reaches before any pack call site, passes such a path inside the workflow directory and refuses one anywhere else, since the smoke reads the tree beyond the payload (§The consumer smoke).

## Producers and consumers

- **The excluded member** (delta 1). Producer: the knob resolution at the preflight, on every smoke run. This tree's configuration reaches it, since the smoke runs only in a checkout and the default `.workflow` names a directory this tree carries. Consumers:
  - the porcelain call, which reads it as an exclude pathspec;
  - the refusal text, which names it.
- **The narrowed refusal** (delta 1). Its red condition is a dirty or untracked path outside the excluded directory. It reds on *finding* such a path and never on finding none, so narrowing its corpus can only remove refusals (causal-completeness point 5). The readers that relied on the old breadth:
  - **The footprint witness** (`footprint` in `staging.rs`) plants an untracked file at the root and one under `installer/`, after the preflight. A dirty workflow directory is outside the packer's footprint, so the root plant still packs clean, and the `installer/` plant still refuses naming itself, since the packer's diagnostic lists footprint entries only.
  - **Every arm that packs** meets the packer's footprint refusal, which never read the workflow directory.
  - **No arm reads the tree's workflow directory**, per the probe above. The clean-path verdicts are unchanged.
- **`--run-validate`'s rerun after a red suite** is the producer the recurrence named. Its fold writes `.workflow/validate-evidence.txt`, inside the excluded directory, so the rerun's `installer_smoke` preflight passes. evidence-kit is unchanged, and the route of keeping the fold out of a preflight's path is not taken: the predicate no longer reads that path.
- **The validate evidence the run records** describes a tree equal to `HEAD` on every path the smoke reads, as before. A dirty evidence manifest is the one difference, and the run supersedes it at its own fold.

## Existing sections updated

- `native/src/emit/installer_smoke/roster.rs`, `preflight` and its test module (deltas 1 and 2).
- `installer/SPEC.md` §The consumer smoke, the preflight paragraph (delta 3) and the undeclared-values paragraph's last sentence (delta 4).
- `installer/SPEC.md` §The packer, the dirty-path paragraph (delta 4).
- `docs/installer/SPEC.md`, the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (deltas 3 and 4).

The passages were found by `grep -n -i "whole-tree precondition\|tree must be clean\|clean-worktree precondition" installer/SPEC.md` and `git grep -n "the pack step refuses to stamp"` over the tracked tree.

## Retired spellings

- None — no delta retires a name. The refusal text delta 1 rewrites is spelled by no other surface.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The local oracle** — `bash gate-sdk/bin/build-native.sh`, the battery, the crate's tests and the `installer_smoke` suite green, the last run once with an uncommitted edit under `.workflow/` (it must pass) and once with one at the root (it must refuse at the preflight, naming the path).
- [ ] **The Done move** lands before the drain stage, with the merge. It rides `personal-adoption`'s mid-iteration push, whose install-smoke legs run the narrowed preflight on every host.
