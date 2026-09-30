# SPEC amendment: hook-root-resolution

**Every hook wiring the kits ship names the front end by a path relative to the hook's working directory, and that directory is not the repository root often enough to matter.** `guard-kit/templates/settings-hooks.json` runs `bash gate-sdk/bin/run-gates.sh --hook <member>`. `plugin/hooks/hooks.json` renders the same command behind `test -f gate-sdk/bin/run-gates.sh || exit 0;`. A build session measured the plugin's guards silent in a session launched from a vendored tree's `docs/` (Claude Code 2.1.283).

**Measured at authoring, Claude Code 2.1.285, `claude -p`, in a scratch git repository whose project settings wire logging hooks:**

- **A hook runs in the session's current directory, and that directory follows the session's own `cd`.** In a session launched at the root, the `PreToolUse(Bash)` hook ran at the root. After the session's Bash tool ran `cd sub`, the next hook ran in `sub`. So the ordinary launch is not safe: any session that changes directory into a subdirectory loses a relatively spelled hook from its next call on. The settings rendering then exits 127, which the harness treats as a non-blocking error, and the plugin rendering falls open. Either way the guard is silent.
- **`CLAUDE_PROJECT_DIR` holds the launch directory, and a `cd` does not move it.** It held the root before and after the session's `cd`. It was set for `SessionStart`, `UserPromptSubmit`, `PreToolUse` on the Bash and Agent matchers, and `SubagentStop`.
- **A subdirectory launch loads no project settings.** Launched from `sub/deeper`, the project's `.claude/settings.json` hooks did not fire at all. A hook supplied another way (`--settings`, the path a user-scope or plugin hook takes) did fire, with its working directory and `CLAUDE_PROJECT_DIR` both `sub/deeper`, while `git rev-parse --show-toplevel` answered the repository root.
- **Resolving the root costs 0.8 ms**: the mean of 50 runs of `git -C docs rev-parse --show-toplevel` in this checkout.

**So the two renderings take different resolutions, each the cheapest that is correct for where it loads.** The settings rendering loads only in a root launch, where `CLAUDE_PROJECT_DIR` is the root and survives every `cd`. So the variable resolves it with no spawn, and every reader of the wiring already parses it. The plugin loads in every launch, subdirectories included, where the variable names the subdirectory. So it resolves the toplevel through git from the variable, one spawn per call, git being the one unconditional member of the adopter floor. This premise on the queue entry is settled by the second and third bullets: "whether settings hooks also run from the launch directory, where the relative command would fail rather than fall open". The entry's inferred marker is deleted in the promoting commit.

**What stays as it is.** The front end's own resolution: `gate-sdk/bin/run-gates.sh` already `cd`s to the git toplevel of its working directory, so once it is found, a subdirectory working directory is harmless. The fail-open arm exemption in `check-door-binding`, which keys on the `--hook` token wherever the path sits. The statusline and session-start wirings, which spell a repository-relative command outside the front-end hooks this unit covers. They are filed as a gap rather than swept here.

## What changes

### (1) The settings rendering resolves the front end through `CLAUDE_PROJECT_DIR`

`guard-kit/templates/settings-hooks.json`'s three commands become `bash "${CLAUDE_PROJECT_DIR}/gate-sdk/bin/run-gates.sh" --hook <member>` {mechanical}. **Not yet applied.** That spelling is the one both wiring readers already parse:

- `check-settings-paths` strips a leading `${CLAUDE_PROJECT_DIR}` as the repository root and resolves the rest (`native/src/gates/settings_paths.rs`, the `hook_path` unit tests covering the quoted braced form).
- The hook registration parser reads the member after any path whose basename is the front end (`native/src/hook/mod.rs`, the unit test registering `"${CLAUDE_PROJECT_DIR}/gate-sdk/bin/run-gates.sh" --hook x`).

guard-kit/SPEC.md §The shell guard's sentence "`templates/settings-hooks.json`'s `Bash|PowerShell` matcher group runs `bash gate-sdk/bin/run-gates.sh --hook shell-guard`" names the new spelling and gains: "The path is anchored at the harness's project directory, because a hook runs in the session's current directory, which the session's own `cd` moves." guard-kit/SPEC.md §The recommended allowlist's sentence "**The template assumes the kits are vendored at the repository root**, as `settings-hooks.json` does" stays true and stays.

### (2) The plugin rendering resolves the toplevel through git

plugin/SPEC.md §The guards' rewrite rule changes its prefix from `test -f gate-sdk/bin/run-gates.sh || exit 0; ` to `r=$(git -C "${CLAUDE_PROJECT_DIR:-.}" rev-parse --show-toplevel 2>/dev/null) && test -f "$r/gate-sdk/bin/run-gates.sh" || exit 0; CLAUDE_PROJECT_DIR=$r; ` {design-bearing}. **Not yet applied.** The template command follows it unchanged, so the rendering stays the template's command behind a fixed prefix. The prefix resolves the toplevel of the project directory, falling back to the working directory where the variable is unset. It exits 0 where there is no repository or no vendored front end. It then rebinds `CLAUDE_PROJECT_DIR` to the toplevel for the template command it prefixes, so one template spelling serves a root launch, a subdirectory launch and a `cd` alike. The rebinding is local to the hook's shell and its children.

- `native/src/gates/plugin_parity.rs`'s `FAIL_OPEN` constant becomes the new prefix, and `plugin/hooks/hooks.json` is regenerated to match (`check-plugin-parity` assertion C).
- `scripts/gate-tests/check-plugin-parity/{good,bad}/` carry the new rendering, and `bad/expect.txt`'s printed prefix moves with it.
- plugin/SPEC.md §The guards' paragraph "…It runs it from the directory the session was launched in, which is the repository root on an ordinary launch: …" becomes: "It runs it in the session's current directory, which a subdirectory launch or the session's own `cd` moves off the root, so the prefix resolves the repository's toplevel from the harness's project directory: the shell-guard fires anywhere inside a vendored repository and stays silent outside one."
- plugin/SPEC.md §Honest limits drops the bullet's sentence "A session launched from a subdirectory is such a tree, since the hook resolves the front end against the launch directory."

**Inferred, cannot run before build:** the rendered prefix resolves under Git for Windows' bash, where `CLAUDE_PROJECT_DIR` arrives in a Windows spelling that `git -C` accepts and `git` answers with a forward-slash toplevel that `test -f` accepts — no Windows host is available here, and delta 4's probe is what runs on one.

### (3) Every hook-wiring instruction names the anchored spelling

Each instruction telling an adopter to wire a front-end hook spells `bash "${CLAUDE_PROJECT_DIR}/gate-sdk/bin/run-gates.sh" --hook <member>` {mechanical}. **Not yet applied.** The roster, from `git grep -n 'run-gates.sh --hook' -- ':!docs' ':!native' ':!*gate-tests*'`, with each site's value:

- `guard-kit/README.md` — the wiring step's `bash gate-sdk/bin/run-gates.sh --hook shell-guard` (and the `--hook wakeup-guard` and `--hook escalation-guard` alternatives): re-spelled.
- `delegation-kit/README.md` — steps 4 and 5 (`--hook agent-budget-guard`, `--hook subagent-stop-liveness`): re-spelled.
- `delegation-kit/SPEC.md` — the two "Wire `bash gate-sdk/bin/run-gates.sh --hook …`" sentences: re-spelled.
- `lifecycle-kit/README.md` — step 5 (`--hook workflow-state-guard`): re-spelled.
- `lifecycle-kit/SPEC.md` — "It is invoked as `bash gate-sdk/bin/run-gates.sh --hook workflow-state-guard`": re-spelled.
- `guard-kit/SPEC.md` §Testing's smoke paragraph and installer/SPEC.md's installed-guard bullet name `bash gate-sdk/bin/run-gates.sh --hook shell-guard` as the command the smoke *drives* from the consumer root, not as a wiring, and stay.
- gate-sdk/SPEC.md §run-gates' `run-gates.sh --hook <member>` describes the arm's dispatch and stays.

### (4) Each rendering is probed from a subdirectory

The settings rendering gets a hermetic probe in guard-kit's consumer smoke, and the plugin rendering gets a harness probe in the plugin's validation run {design-bearing}. **Not yet applied.**

- **`guard-kit/smoke/install.sh`**, after its existing `cd_compound` assertion, reads the merged `Bash|PowerShell` group's shell-guard `command` string. It executes it with `bash -c` from a subdirectory the smoke creates in the scratch consumer, with `CLAUDE_PROJECT_DIR` set to the consumer root and the `cd deploy && ls` payload on stdin, and asserts the same block carrying rule `cd_compound`'s steer. The existing assertions drive the member directly. This one drives the wiring string, so a relative spelling reds it: from the subdirectory it exits 127 and blocks nothing. guard-kit/SPEC.md §Testing's smoke paragraph gains the sentence.
- **plugin/SPEC.md §The validation leg**'s local run, a build that changes the package, adds two sessions to "firing in a vendored scratch repository and silent in an empty one". One is launched from a subdirectory of the vendored repository. In the other, the session's Bash tool changes into a subdirectory before its next call. The guard fires in both.

### (5) This repository's own wiring follows, prepared rather than applied

`.claude/settings.json`'s five front-end hook commands (`--hook subagent-stop-liveness`, `shell-guard`, `agent-budget-guard`, `agent-dispatch-guard` and `workflow-state-guard`) take the delta 1 spelling {mechanical}. **Not yet applied.** Hook wiring is a permission-surface write: CLAUDE.md §Housekeeping has a delegated session only prepare the diff, and this one rewires every guard, so it is high-impact. The build session prepares the diff and the operator applies it. The entry moves only once it is applied, because until then this repository's own sessions keep the defect.

## Producers and consumers

- **The anchored settings command.** Producer: the adopter's merge of `templates/settings-hooks.json`, and this repository's settings file. Consumer: the harness, which runs it with `CLAUDE_PROJECT_DIR` set on every hook event measured above. Readers of the string: `check-settings-paths` (member check, unchanged), the registration parser and its callers (the enforcement map, the guard-kit grant reader), `check-door-binding` (fail-open exemption, unchanged), `check-plugin-parity` (through the template), and the consumer smoke (delta 4).
- **The plugin prefix.** Producer: the rendering rule, held by `check-plugin-parity`'s `FAIL_OPEN` constant. Consumer: the harness, in every repository where the plugin is enabled. The shell variable `r` is read by the `test` and the rebinding, and the rebound `CLAUDE_PROJECT_DIR` by the template command after it. The prefix has no other field.
- **The subdirectory probes.** The smoke's is produced on every `--run-consumer-smoke` run and read through its exit status. The plugin's is produced by the build session that changes the package, as the validation leg's local run already is.

## Existing sections updated

- `guard-kit/templates/settings-hooks.json` — the three commands (delta 1).
- `guard-kit/SPEC.md` — §The shell guard (delta 1), §Testing's smoke paragraph (delta 4).
- `plugin/hooks/hooks.json`, `native/src/gates/plugin_parity.rs`, `scripts/gate-tests/check-plugin-parity/` — the rendering and its holder (delta 2).
- `plugin/SPEC.md` — §The guards and §Honest limits (delta 2), §The validation leg (delta 4).
- `guard-kit/README.md`, `delegation-kit/README.md`, `delegation-kit/SPEC.md`, `lifecycle-kit/README.md`, `lifecycle-kit/SPEC.md` — the wiring instructions (delta 3).
- `guard-kit/smoke/install.sh` — the subdirectory probe (delta 4).
- `.claude/settings.json` — this repository's wiring, as a prepared diff (delta 5).
- `docs/` — the generated mirrors of every SPEC and README above, regenerated by the arm `check-docs-mirror-fresh` prints (all deltas).

## Retired spellings

<!-- retired-spelling-exempt: the relative front-end spelling stays lawful where a smoke or a test drives the member from the root, in fixtures, and in SPEC prose describing the arm's dispatch -->
- `bash gate-sdk/bin/run-gates.sh --hook` — retired as a wiring spelling (deltas 1, 3 and 5).
<!-- retired-spelling-exempt: the queue entry quoting the prefix leaves the queue body at its Done move, and a check-plugin-parity unit test may keep it as the prior form its assertion rejects -->
- `test -f gate-sdk/bin/run-gates.sh || exit 0; ` — the plugin's prefix (delta 2).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for both spellings and both probes.
- [ ] **Every roster site has its value** — each delta 3 site re-spelled or kept as stated, re-derived with the roster's `git grep`.
- [ ] **Merged with no information lost** — the measurements above land in guard-kit/SPEC.md §The shell guard and plugin/SPEC.md §The guards as the grounds for each spelling.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — `check-amendment-retired-spelling` runs both declarations.
- [ ] **Gaps filed** — the statusline and session-start wirings, filed with `--emit file-gap` at authoring.
- [ ] **The entry moves** — `plugin-guards-subdir-launch` moves to Done once delta 5's diff is applied, a stage before the drain stage.
