# SPEC amendment: native git hooks

Pairs `native-executable-git-hooks`. Sited at the repository root because it changes gate-sdk's and the installer's contracts together.

**Direction this amendment executes** (operator direction 2026-10-06, lead-relayed, not a ruling): ship the native hook probe-gated. `--install-hooks` places untracked per-clone native hooks on every OS and the tracked sh hooks retire; the sh handoff stays, on every OS, if the Windows probe is red. Where the hook lives was left to this amendment and is ruled in delta 3.

**The gate on every delta after the first.** Delta 1's probe runs on the iteration's one mid-iteration push, and deltas 2 through 7 are built before that push so the Windows legs judge the probe and the implementation in one run.

- **Windows legs green:** deltas 2 through 7 stand as landed.
- **Windows legs red on the hook start:** deltas 2 through 7 are reverted, on every OS, since a native hook on some systems and a shell hook on others is the Windows-only exception the direction refuses. Delta 1 stays as a witness, and gate-sdk/SPEC.md §gen-pre-commit gains one sentence stating what the run printed. Its assertion step is reduced to what git alone answers, whether a native file under the hook's name is started, inverted on the legs that printed the refusal. The refused-with-a-gate's-name half rests on delta 2's launcher and leaves with it, and the probe step times a stand-in executable.

## What changes

### (1) A probe of whether git starts a native hook, and what a hook start costs

Every OS leg of the `gates` workflow that installs a consumer gains two steps: an assertion that git starts a native hook, which fails the leg, and a probe of the start cost, which reports and never fails as installer/SPEC.md §The consumer smoke rules for a probe step. {design-bearing}

Both run in a scratch repository with `core.hooksPath` naming a scratch directory. The assertion step holds the first two points below and the probe step the third.

- A copy of the gate binary named `pre-commit`, with the host's executable suffix, is started by `git commit` with no interpreter: the commit lands with a clean staged change, and is refused with a gate's name when the change violates a registered member.
- On Windows the same holds with every `sh.exe` under the Git for Windows root renamed away, the existing probe's own setup.
- The start cost: the median of repeated runs of the hook with nothing staged, where the arm exits 0 printing nothing, for the native hook and for a two-line POSIX sh handoff to the same binary. Both figures print with the leg's OS and architecture.

**Inferred, cannot run before build:** Git for Windows resolves a hook named `pre-commit` to a file `pre-commit.exe` in the hooks directory and starts it as a program — no Windows host is reachable from the authoring session, and this step is the claim's first run.

**Inferred, cannot run before build:** a native start costs less than the bundled sh start on Windows, on x64 and on Arm — the timing half of this step is what measures it.

Run at authoring, on Linux x86-64 with git 2.55.0: git started a native executable named `pre-commit` under `core.hooksPath` and honoured its status both ways; a lone `pre-commit.exe` was not looked up; a sh handoff cost 1448 µs per start against 651 µs direct, over 200 starts.

### (2) The binary answers to a hook's name

The gate binary, started under a file name whose stem is a git hook it serves, acts as that hook's launcher. {design-bearing} {user-facing: operator direction 2026-10-06, lead-relayed — a hook git starts as a native executable, one shape on every OS}

- **The served set is closed**: `pre-commit` and `commit-msg`, the two names §git-hook takes as its operand. The stem is read from the name the process was started under, with the host's executable suffix removed.
- **A launcher resolves the binary `GATE_SDK_NATIVE_BIN` names and starts it** as `<bin> --git-hook <hook> <the arguments git passed>`, waits, and returns its status. It always starts it, even when the two are one file. A placed hook therefore never judges: after an update replaces the binary, the hook left behind is an older build acting only as a launcher, and the judging build is the installed one.
- **A binary that cannot be started refuses the commit**, naming the path it resolved and the two remedies: place or build the binary, or `git commit --no-verify`. This is the refusal an absent binary gave the sh handoff.
- **A `GATE_SDK_NATIVE_BIN` whose own stem is a served hook name is refused** at exit 2, since the launcher would start itself.
- The launcher adds no knob read beyond `GATE_SDK_NATIVE_BIN`. The work-tree pin, the selection and the output strings are §git-hook's, unchanged.

### (3) `--install-hooks` places the hooks, untracked, in the clone

`--install-hooks` writes the hook files itself and points the clone at them. {design-bearing} {user-facing: operator direction 2026-10-06, lead-relayed — hooks installed untracked per clone on every OS, the tracked sh hooks retiring; the directory's name and the link mechanics below are a lead decision of 2026-10-06 inside that direction, which left the location to this amendment}

- **The hooks directory is `gate-hooks` under the repository's common git directory**, the path `git rev-parse --git-common-dir` answers. `core.hooksPath` is set to its absolute path.
  - It is under the git directory because a hook file is a per-platform binary, which a tracked text hook cannot be, and because a directory in the working tree is absent from every linked worktree, where git then runs no hook and says nothing.
  - It is its own directory, never git's default `hooks`, so an adopter's existing hooks are not overwritten and `git config --unset core.hooksPath` stays the per-clone off switch.
  - The path is absolute because a relative `core.hooksPath` resolves against each worktree's top.
- **Both served hooks are always placed.** Each is a hard link to the binary `GATE_SDK_NATIVE_BIN` names, under the hook's name with the host's executable suffix, and a copy where the link cannot be made. On a host with an executable bit the file carries it.
- **`--git-hook commit-msg` with no `tier=commit-msg` member registered exits 0 printing nothing.** The conditional that decided whether a `commit-msg` hook existed has no remaining reader, and a gate registered at that tier later is reached with no re-install. An adopter with no such gate met no `commit-msg` hook before and meets a silent one now, so the arm's summary line for an empty selection is suppressed rather than printed on every commit.
- **The arm refuses at exit 2 when the binary it would link is absent**, in place of the refusal on an absent hooks directory.
- A re-run replaces both files and is otherwise idempotent. The `blame.ignoreRevsFile` write, the `check-identity` rung and its three branches, and the exit contract are unchanged. The `Active hooks:` receipt keeps its shape and lists the placed files, so its names carry the host's executable suffix. The arm's usage text and the `--help` paragraph are restated to placement.

### (4) The tracked hooks, their emitter and their two holders retire

The generated hook files, the arm that wrote them and the assertions that held them are deleted in one unit. {mechanical} {user-facing: operator direction 2026-10-06, lead-relayed — the tracked sh hooks retire}

- The `--emit git-hooks` arm and its module are deleted, with its row in the arm table.
- `GATE_SDK_HOOKS_DIR` is deleted from gate-sdk's table. A consumer knob file still setting it is refused at its first read by the table's own rule, naming the knob.
- `check-graph` loses assertion D and the two hook paths in its `couples=` and `# projection:` lines. Its tree test loses the three hook cases.
- `check-hook-exec-bit` is deleted: descriptor, module, registry row, fixture pair and every registration.
- The tracked hook files under this repository's gates directory are deleted, with their line in the core-files roster.
- gate-sdk/SPEC.md §gen-pre-commit is deleted. What survives of it moves: the staged-set-against-working-tree limit and the stage-then-regenerate rule to §git-hook, the knob-derived trigger rule to §The `# graph:` manifest. Every `§gen-pre-commit` citation is repointed to the section its sentence now rests on.
- **No tracked artifact holds the binary knob's text any longer**, the hooks having been the only one. Three statements resting on that lose their ground and are restated or deleted:
  - guard-kit/SPEC.md §The rule roster's knob-prefix rule keeps its steer and loses the respelled-path clause, with the message tail the rule prints for it.
  - installer/SPEC.md §Placement's two paragraphs, the stable relative path and the ordering against hook generation.
  - gate-sdk/SPEC.md §Consumer smoke's rule that a kit smoke regenerates the hook with the graph, and docs/projection-fan-outs.md's paragraph on what the hooks carry.
- installer/SPEC.md §The consumer smoke's `bash`-less arm rested its unmasked `/bin/sh` on git running hooks with it. The mask is unchanged, since the unix bootstrap still runs under it, and the sentence is restated to that ground.

### (5) `init`, `uninstall` and `doctor` follow the hooks into the clone

The installer stops generating hooks and learns where the placed ones are. {design-bearing} {user-facing: operator direction 2026-10-06, lead-relayed — hooks installed untracked per clone by `--install-hooks`}

- **`init`'s generation step emits the coupling graph alone.** Its follow-up block still prints the `--install-hooks` line, the one step that puts a hook in the clone.
- **`uninstall` removes the hooks directory** and, when `core.hooksPath` names it, prints the `git config --unset core.hooksPath` line. The directory is this tool's own and holds nothing an adopter wrote; git config stays outside the ownership roster.
- **`doctor` reports the hook state on one line and fails on none of it**: not opted in, opted in with both hooks present, or opted in with a hook missing, naming `--install-hooks` as the remedy. The opt-in is the adopter's choice, so its absence is no failure.

### (6) The published surfaces say what the hook now is

Every adopter-facing statement of the hook is rewritten to the placed native form. {mechanical} {user-facing: operator direction 2026-10-06, lead-relayed — replaces the tracked-hook review story}

- installer/SPEC.md §Reviewing the pre-commit hook: the hook is the digest-verified gate binary under the hook's name, placed in the clone's git directory; opting in changes no tracked file; what a commit runs is the installed binary and the tracked declarations it reads.
- installer/SPEC.md's other statements of the hook follow: §Implementation's POSIX-subset ground, §Requirements' *What runs without `bash`*, §Selecting kits and gates' and §Profiles' description of what the gate SDK brings, §What runs behind the invoke's generation step, and §Vendoring without the installer's opt-in step. In §Requirements' bullet this delta owns the sentences on the git hooks and the sibling entry owns the closing sentence on guard-kit's hook; in context-kit/SPEC.md §bin/env-probe delta 4 owns the sentence on the hooks staying POSIX sh and the sibling owns the library-reader sentence. Neither rewrites the other's, so the two land in either order.
- Every sentence calling the hook *generated* is restated to *placed*: the declared spellings under §Retired spellings enumerate them, and the forms no literal reaches, a hook that *is generated* or *generated from* the manifests, are in docs/value.md, gate-sdk/README.md and the demo arm's narration.
- Each kit README's hook step names `--install-hooks` in place of the retired write.
- docs/requirements.md: the Git for Windows prerequisite row drops the bundled sh as the hook's runner, and §What a commit costs replaces the unmeasured-sh sentence with the statement that the hook starts with no shell, citing the probe step for the per-leg figures.
- The Git for Windows row's closing bash clause is the sibling entry `windows-kit-bash-files`'s. If that entry's wiring delta lands in the same batch the row is rewritten once, to both results; if it does not, this delta removes the sh clause alone and leaves the bash clause as it reads.

### (7) The Windows witness inverts

The `gates` workflow's step that renames the bundled sh away now asserts the commit runs its hooks and lands. {mechanical}

Its failure message names a hook that could not start without sh. The PowerShell leg's hook steps read the hooks directory from `core.hooksPath` as they do now, and its follow-up-block parse is unchanged.

## Producers and consumers

- **The launcher (delta 2).** Producer: git, starting the placed file at commit, enabled by the `core.hooksPath` write of delta 3, which `init`'s printed follow-up line reaches in every install. Consumer: the installed binary's `--git-hook` arm, by spawn; its status returns to git.
- **The hooks directory (delta 3).** Producer: `--install-hooks`. Consumers: git through `core.hooksPath`; `uninstall` and `doctor` (delta 5), each resolving the same common-directory path; the workflow steps of deltas 1 and 7.
- **The silent `commit-msg` exit (delta 3).** Producer: `--git-hook commit-msg` on a registry with no member at that tier. Consumer: git, which reads the status alone.
- **The probe's figures (delta 1).** Producer: the workflow step. Consumer: the reader of a run's log, and docs/requirements.md by citation; no file transcribes them.
- **Narrowed corpora, each reader's red condition (delta 4).**
  - `check-graph`: its remaining assertions red on a stale graph artifact or an invalid manifest, never on a count of hooks, so dropping the hook paths removes subjects and no floor.
  - `check-core-files`: reds on a rostered path that is absent, so the roster line and the file leave in one commit.
  - `check-gate-fixture-coverage` and `check-kit-registration`: red on a registered member with no pair or no declaration, so `check-hook-exec-bit`'s registration, descriptor and pair leave together.
  - Any other roster naming the deleted gate reds on a name that resolves nowhere, so every registration leaves in the same commit; the build enumerates them with `git grep -l check-hook-exec-bit`.
- **Roster-holding readers of a minted name.** `gate-hooks` is a path component under the git directory and lands on no rostered surface. No knob, tag or arm is minted.

## Existing sections updated

Produced by `git grep -l -F` over the tracked tree for the five spellings under §Retired spellings, minus the paths the retired-spelling exclusion knob holds out, plus the sections the deltas name. The align audit added the surfaces a delta changes that carry no declared spelling, found by `git grep -n -i hook installer/SPEC.md` and by `git grep -l -i` for `hooksPath`, `install-hooks` and `pre-commit` over `native/src`. The build re-derives it.

- `gate-sdk/SPEC.md` — §gen-pre-commit deleted and its survivors rehomed (delta 4); §git-hook gains the launcher and the silent `commit-msg` exit (deltas 2 and 3); §install-hooks restated to placement (delta 3); §check-graph loses assertion D, §check-hook-exec-bit is deleted, §Layout and configuration loses the knob, and §Consumer smoke loses the hook from its regeneration rule (delta 4); each sentence calling the hook generated (delta 6).
- `installer/SPEC.md` — §init, §uninstall, §doctor (delta 5); §Placement's two paragraphs and §The consumer smoke's `bash`-less arm (delta 4); §Reviewing the pre-commit hook, §Implementation, §Requirements' *What runs without `bash`*, §Selecting kits and gates, §Profiles, §What runs behind the invoke and §Vendoring without the installer (delta 6); §The consumer smoke's hook steps and its hooked move arm (deltas 1, 3 and 7).
- `.github/workflows/gates.yml` — the two new steps and the inverted witness (deltas 1 and 7); the comment naming the shell the hooks run under (delta 4).
- `docs/requirements.md` — the prerequisite row and the cost paragraph (delta 6).
- `CLAUDE.md` — the generated-hook line, this repository's own statement of a tracked hook (delta 4).
- `README.md` — the governed-repo paragraph's opt-in sentence (delta 6).
- `guard-kit/SPEC.md` — §The rule roster's knob-prefix rule (delta 4); §The recommended allowlist's and §scratch-run's mentions of the generated hook (delta 6).
- `native/src/guard/rules/spelling.rs` — the respelled-path message tail (delta 4).
- `doctrine-kit/DOCTRINE.md` — the Oracle-first rule's two mentions (delta 6).
- `drift-kit/SPEC.md` — §The overhead meter's mention (delta 6).
- `queue-kit/SPEC.md` — §check-roadmap-fresh's mention (delta 6).
- `installer/README.md` — the starter profile's description (delta 6).
- `installer/profiles.list` — the comments naming the generated hook and the hook generator (delta 6).
- `docs/value.md` — the sentence that the hook is generated from the manifests (delta 6).
- `docs/gate-sdk/index.md` — the kit landing page's mention (delta 6).
- `native/src/main.rs` — the start-name read ahead of the argument parse (delta 2).
- `native/src/emit/git_hook.rs` — the silent `commit-msg` exit and the leading binding's wording (deltas 3 and 6).
- `native/src/runner.rs` — the usage line and the `--help` paragraphs naming the generated hooks (deltas 3 and 6).
- `native/src/installer/uninstall.rs` (delta 5)
- `native/src/installer/doctor.rs` (delta 5)
- `native/src/installer/selection.rs` — the refusal text naming the hook generator (delta 6).
- `native/src/emit/demo.rs` — the narration that a hook was generated (delta 6).
- `native/src/emit/installer_smoke/moves.rs` — the hooked move arm's executable-hook assertion (delta 3).
- `gate-sdk/gate-tests/check-graph-tree.test.sh` — the three hook cases (delta 4).
- `docs/generated-projections.md` — the graph artifact's set loses the hooks (delta 4).
- `docs/projection-fan-outs.md` — the first-commit-msg-gate fan-out line (delta 4).
- `.claude/commands/agent-execution.md` — the shared-file roster's hook path (delta 4).
- `scripts/core-files.list` (delta 4)
- `scripts/gates.list` (delta 4)
- `scripts/git-hooks/commit-msg` — deleted (delta 4).
- `scripts/git-hooks/pre-commit` — deleted (delta 4).
- `gate-sdk/checks/check-graph.gate` (delta 4)
- `gate-sdk/checks/check-hook-exec-bit.gate` — deleted (delta 4).
- `native/src/emit/git_hooks.rs` — deleted (delta 4).
- `native/src/gates/hook_exec_bit.rs` — deleted (delta 4).
- `native/src/emit/install_hooks.rs` (delta 3)
- `native/src/emit/mod.rs` (deltas 2 and 4)
- `native/src/emit/upgrade_smoke.rs` (delta 4)
- `native/src/gates/graph.rs` (delta 4)
- `native/src/gates/mod.rs` (delta 4)
- `native/src/installer/init.rs` (delta 5)
- `native/src/knobs/gate_sdk.rs` (delta 4)
- `native/src/knobs/mod.rs` (delta 4)
- `native/src/registry.rs` (delta 4)
- `gate-sdk/README.md` (delta 6)
- `canon-kit/README.md` (delta 6)
- `context-kit/README.md` (delta 6)
- `delegation-kit/README.md` (delta 6)
- `doctrine-kit/README.md` (delta 6)
- `evidence-kit/README.md` (delta 6)
- `lifecycle-kit/README.md` (delta 6)
- `queue-kit/README.md` (delta 6)
- `site-kit/README.md` (delta 6)
- `canon-kit/SPEC.md` — each sentence carrying a retired spelling (delta 4).
- `context-kit/SPEC.md` — §bin/env-probe's sentence that the generated hooks stay POSIX sh (delta 4).
- `lifecycle-kit/SPEC.md` — the writer and asserter analogy naming the retired arm (delta 4).
- `canon-kit/smoke/install.sh` (delta 4)
- `context-kit/smoke/install.sh` (delta 4)
- `delegation-kit/smoke/install.sh` (delta 4)
- `doctrine-kit/smoke/install.sh` (delta 4)
- `evidence-kit/smoke/install.sh` (delta 4)
- `gate-sdk/smoke/install.sh` (delta 4)
- `guard-kit/smoke/install.sh` (delta 4)
- `guard-kit/smoke/violation.sh` (delta 4)
- `lifecycle-kit/smoke/install.sh` (delta 4)
- `queue-kit/smoke/install.sh` (delta 4)
- `site-kit/smoke/install.sh` (delta 4)
- `docs/check-graph.html` — regenerated (delta 4).
- `docs/enforcement.md` — regenerated (delta 4).
- `docs/gate-sdk/SPEC.md` — mirror, regenerated (all deltas).
- `docs/installer/SPEC.md` — mirror, regenerated (all deltas).
- `docs/canon-kit/SPEC.md` — mirror, regenerated (delta 4).
- `docs/context-kit/SPEC.md` — mirror, regenerated (delta 4).
- `docs/lifecycle-kit/SPEC.md` — mirror, regenerated (delta 4).
- `docs/gate-sdk/README.md` — mirror, regenerated (delta 6).
- `docs/canon-kit/README.md` — mirror, regenerated (delta 6).
- `docs/context-kit/README.md` — mirror, regenerated (delta 6).
- `docs/delegation-kit/README.md` — mirror, regenerated (delta 6).
- `docs/doctrine-kit/README.md` — mirror, regenerated (delta 6).
- `docs/evidence-kit/README.md` — mirror, regenerated (delta 6).
- `docs/lifecycle-kit/README.md` — mirror, regenerated (delta 6).
- `docs/queue-kit/README.md` — mirror, regenerated (delta 6).
- `docs/site-kit/README.md` — mirror, regenerated (delta 6).
- `docs/guard-kit/SPEC.md` — mirror, regenerated (deltas 4 and 6).
- `docs/drift-kit/SPEC.md` — mirror, regenerated (delta 6).
- `docs/queue-kit/SPEC.md` — mirror, regenerated (delta 6).
- `docs/doctrine-kit/DOCTRINE.md` — mirror, regenerated (delta 6).
- `docs/installer/README.md` — mirror, regenerated (delta 6).
- `docs/posts/2026-08-01-checkwright-v0-21-0.md` — a dated release post, left standing as history (delta 6).
- `docs/posts/2026-08-06-checkwright-v0-22-0.md` — a dated release post, left standing as history (delta 6).
- `docs/posts/2026-08-23-checkwright-v0-25-0.md` — a dated release post, left standing as history (delta 6).
- `docs/posts/2026-09-26-checkwright-v0-26-0.md` — a dated release post, left standing as history (delta 4).
- `docs/posts/2026-09-28-checkwright-v0-27-0.md` — a dated release post, left standing as history (delta 4).
- `docs/posts/2026-09-29-checkwright-v0-30-0.md` — a dated release post, left standing as history (delta 4).

## Retired spellings

- `GATE_SDK_HOOKS_DIR` — the hooks directory is no longer consumer-placed (delta 4)
- `--emit git-hooks` — no hook file is generated (delta 4)
- `check-hook-exec-bit` — no tracked hook carries a mode to hold (delta 4)
- `scripts/git-hooks` — this repository's tracked hooks directory (delta 4)
- `§gen-pre-commit` — the section is deleted and its citations repointed (delta 4)
- `generated pre-commit hook` — the hook is placed, never generated (delta 6)
- `generated hook` — as above (delta 6)
- `generated git hook` — as above (delta 6)
- `generated pre-commit and commit-msg hooks` — as above (delta 6)
- `hook generator` — no arm generates a hook (delta 6)

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **The probe read before the verdict** — the mid-iteration push's Windows legs are read, and the amendment's gate above is applied in the direction they print.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component.
- [ ] **Entry moved** — to Done before the drain stage, once the remote run is read, as the build stage's remote-oracle rule says, never in the merge commit.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
