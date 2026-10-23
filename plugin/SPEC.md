# Checkwright plugin — design record

The plugin package lists Checkwright in a harness's plugin catalog. It registers Checkwright's lifecycle stage skills and wires guard-kit's guards, and a marketplace at the repository root lists it. It is a **discovery surface**: installing the plugin installs no kit. Its install skill runs the installer's `init`, which remains the only thing that vendors ([installer/SPEC.md §init](../installer/SPEC.md#init)), and the gates stay in the binary, outside every harness. `plugin/README.md` is the usage tier in front of this file.

## The component

`plugin/` is repo-root-governed and not a kit. It carries no `checks/` and no `smoke/`, so no kit-root resolver admits it and no install payload carries it. The harness names live here, in the root marketplace file and on the docs pages, and never in a kit SPEC or a kit literal ([gate-sdk/SPEC.md §The provenance seam](../gate-sdk/SPEC.md#the-provenance-seam)).

The package serves two formats from one tree:

- **Agent Plugins 1.0** (agent-plugins.org). The manifest is `plugin.json` at the package root, requiring `$schema` and `name`. Skills are the immediate subdirectories of `skills/` holding a `SKILL.md` in the Agent Skills format (agentskills.io). Hooks, commands and agents are outside the standard's portable core, and it defines no marketplace and no install route, leaving distribution to each client.
- **Claude Code's plugin format.** The manifest is `.claude-plugin/plugin.json`, requiring only `name`. It reads the same `skills/<name>/SKILL.md` layout and a standard-layout `hooks/hooks.json`. A marketplace is `.claude-plugin/marketplace.json` at a repository's root, and `/plugin marketplace add <owner>/<repo>` reads it.

The two formats read the same `skills/` layout, so one package carries both manifests over one set of skills, and a client implementing either reads its own manifest.

## The harnesses

The package is one tree, and each harness reads what its own format reaches of it. The standard names no adopter, so a harness is listed here once its own documentation or a run shows it reading the package. Each cell says which: **run** is a command this repository's build ran, and **read** is the harness's documentation, at the URL given below the table.

| harness | skills | hooks | catalog |
|---|---|---|---|
| Claude Code | run: `.claude-plugin/plugin.json`, `skills/` | run: `hooks/hooks.json`, this rendering | run: `.claude-plugin/marketplace.json` |
| Codex | run: the portable `plugin.json`, `skills/` | not wired: `codex-cli` 0.162.1 discovers none beside the root `plugin.json` (run); discovered, this rendering blocks (run, 0.162.1; §The guards) | run: `.claude-plugin/marketplace.json`, read as Codex's legacy location; a `git-subdir` source |
| Antigravity | run: the portable `plugin.json`, `skills/`, installed from a local directory | run: `hooks.json` at the plugin root, its own rendering (§The guards), in a workspace-local copy; read: an installed copy and native Windows | read: an official marketplace only |
| Cursor | read: an Agent Plugin's `plugin.json` and `skills/` | not wired from the plugin: it reads hooks only from its own plugin format, and imports a project's Claude Code settings wiring by default | read: its own `.cursor-plugin/marketplace.json` |
| Muse Code | read: Claude Code-format plugins | read: a Claude Code-format plugin's hooks, unsupported events skipped; its payload is undocumented | read: `muse plugins marketplace` |

Sources: developers.openai.com/codex/plugins/build and learn.chatgpt.com/docs/hooks for Codex; the CLI's built-in plugin and hooks documentation for Antigravity; cursor.com/docs/plugins, cursor.com/docs/hooks and cursor.com/docs/reference/third-party-hooks for Cursor; dev.meta.ai/docs/muse-code for Muse Code. A harness not in the table is unread, not unsupported.

A **read** cell becomes **run** only by a later run, written by the build that runs that harness.

## The manifests

- `plugin/plugin.json`, the portable manifest, carries `$schema` (the published Agent Plugins schema URL), `name`, `description`, `author`, `homepage`, `repository` and `license`.
- `plugin/.claude-plugin/plugin.json` carries the same six fields after `$schema`, and no component paths, since the standard layout already locates `skills/` and `hooks/hooks.json`.

Every field has a reader. `name` keys the install. `description`, `author`, `homepage`, `repository` and `license` are what a listing shows a user deciding to install. `$schema` is read by the portable client and by the validation leg's schema check. `keywords` and `extensions` are left out because no reader of them was found.

**Neither manifest carries `version`.** The marketplace entry carries it (§The marketplace pin). In Claude Code a `plugin.json` version wins over the entry's, so a tracked one would be a second version line to bump by hand, and a stale one would stop updates. `claude plugin validate plugin` warns on the missing version and passes.

## The skills

The plugin registers exactly the templates this repository binds in its own skills: one skill per binding shim in `LIFECYCLE_KIT_SKILLS_DIR` (a skill whose file carries lifecycle-kit's binding directive, [lifecycle-kit/SPEC.md §check-skill-binding](../lifecycle-kit/SPEC.md#check-skill-binding)), named for its shim, plus one skill of its own, `install`. What the plugin ships is what this repository's own iterations run. The roster is derived from the shims and held by `check-plugin-parity`, never listed.

- **Front matter.** `name` is the directory's name. `description` says what the skill does, in one or two sentences, and that it runs only when the user asks for it. No other key, so the file stays inside the Agent Skills base set.
- **The body is one fixed rendering.** Its only variable is `<path>`, the template the shim's directive names:

  ```
  Execute the template at `<path>` in this repository.

  - If this repository has a skill of its own that binds `<path>`, run that skill instead.
  - Otherwise, take each `*<slot: …>*` placeholder's own text as its binding.
  - If `<path>` does not exist, Checkwright is not installed here, or not with the kit that ships this template. Say so, and stop: the `install` skill installs it.
  ```

**A skill runs a vendored template and binds nothing.** A stage template's slots are the consumer's to bind in its own skill. That is rule content a package serving every adopter cannot carry: a default chosen here would be this repository's calibration shipped as everyone's. So the skill defers to a skill of the repository's own that binds the template, and otherwise takes each slot's own text as its binding.

**`install`** tells the agent to:

- choose a profile with the user, from the installer README's *Choosing a profile* section;
- confirm the worktree is clean;
- run the one-line install for the host with `init --profile <profile>` as its arguments, the PowerShell twin on Windows in the form docs/install.md gives;
- run each command in the `next:` block `init` prints;
- hand to the `adopt` skill.

The skill carries no template directive. It advertises `init` on a route, so it is a front-door page ([installer/SPEC.md §The front door's verbs](../installer/SPEC.md#the-front-doors-verbs)).

**`adopt`** is `install`'s sibling rather than its extension. Fitting the knobs and triaging the first reds run over files `init` wrote, so they are a vendored template's job, [gate-sdk/SPEC.md §templates/adopt.md](../gate-sdk/SPEC.md#templatesadoptmd), while `install` runs before anything is vendored. `adopt` is an ordinary shim-derived skill, and `install`'s last step hands to it.

Vendoring stays with `init`. A plugin that vendored kits itself would be a second install model with no manifest, update or uninstall story; the install skill hands to the installer, whose `checkwright.lock` the plugin never writes.

## The guards

`plugin/hooks/hooks.json` is a rendering of `guard-kit/templates/settings-hooks.json`, for the harness that reads a plugin's hooks in the shipped binding's protocol (gate-sdk/SPEC.md §The hook contract), Claude Code. The portable manifest carries skills alone, since the standard excludes hooks from its core. The rendering:

- takes each block under `hooks`, its event, its matcher and its hook entries, in order;
- reads each hook's member through the registration parser (gate-sdk/SPEC.md §The harness-integration arm) and composes its `command` as the fail-open prefix followed by the shell-form front-end call of that member: `r=$(git -C "${CLAUDE_PROJECT_DIR:-.}" rev-parse --show-toplevel 2>/dev/null) && test -f "$r/gate-sdk/bin/run-gates.sh" || exit 0; CLAUDE_PROJECT_DIR=$r; bash "${CLAUDE_PROJECT_DIR}/gate-sdk/bin/run-gates.sh" --hook <member>`, dropping the template's `args`;
- fails at exit 2 on a template hook that is no member registration, since the plugin has no command to give it;
- drops the template's `//` note, which is merge advice for a settings file.

A plugin's hooks fire in every repository where the plugin is enabled, so each command first tests for the vendored front end and exits 0 without it. That test takes a shell, so the plugin's hooks keep the shell form where the template's registrations take the exec form. **The plugin therefore owes bash on every host**: the harness runs a shell-form hook command under a POSIX shell, Git for Windows' bash on native Windows. It runs it in the session's current directory, which a subdirectory launch or the session's own `cd` moves off the root, so the prefix resolves the repository's toplevel from the harness's project directory. The shell-guard fires anywhere inside a vendored repository and stays silent outside one, and declines, through the front end, where git refuses or skips the repository the session stands in.

The prefix differs from the settings rendering's anchor because the plugin loads where project settings do not. Launched from a subdirectory, a session loads no project settings, and a hook supplied any other way runs with its working directory and `CLAUDE_PROJECT_DIR` both naming that subdirectory. So the variable cannot name the root here, and the prefix asks git for the toplevel, one spawn per call measured at under a millisecond, git being the adopter floor's one unconditional member. It falls back to the working directory where the variable is unset, exits 0 where there is no repository or no vendored front end, and rebinds `CLAUDE_PROJECT_DIR` to the toplevel for the front-end call after it. The rebinding is local to the hook's shell and its children, so one rendered command serves a root launch, a subdirectory launch and a `cd` alike.

**Codex speaks the shipped binding's protocol and runs no guard from this package.** `codex-cli` 0.162.1 discovers no plugin hooks while the package root carries `plugin.json`, whether or not that manifest names its hooks under `extensions.com.openai` or a `.codex-plugin/plugin.json` sits beside it. A copy without the root manifest has `hooks/hooks.json` discovered. The root manifest stays, since it is the portable core and Antigravity refuses a package without it (§The manifests). Where Codex discovers the rendering, it runs it unchanged, on three terms of its own:

- **It runs a plugin's hooks only after the user trusts them** in its `/hooks` view, and a changed rendering is skipped until trusted again.
- **It sets no project-directory variable and runs a hook in the session's working directory**, so the prefix's fallback to `.` is the path it takes. Run on that copy, the shell guard blocks in a vendored repository, from a subdirectory launch too, and stays silent in an empty one.
- **The wakeup guard never fires there**, since no Codex tool matches `ScheduleWakeup|CronCreate`. Its patch tool matches `Write|Edit` and carries the patch as the command, and `workflow-state-guard` reads each path the patch's file headers name (gate-sdk/SPEC.md §The hook contract).

**Antigravity reads `plugin/hooks.json`**, a second rendering of the same template in its own binding (gate-sdk/SPEC.md §The hook contract, the Antigravity table). It reads hooks only from a `hooks.json` at the plugin root and never from `hooks/hooks.json`, and no other harness reads the root file: `codex-cli` 0.163.0's `hooks/list` over the package names no hook and no warning. The rendering:

- is one hook named for the package, carrying one `PreToolUse` group per template group whose tools have an Antigravity spelling. Its matcher is those spellings joined by `|`, the documented regular-expression alternation: the shell group becomes `run_command`, and the file-write group becomes the five file-write tools. A group with no such tool, the wake group, is left out, and so is the PowerShell tool;
- composes each `command` as `rem=1; ` followed by the Antigravity prefix, then the member, then the suffix.

The prefix reads the payload once and takes the first `workspacePaths` entry with `sed`. It asks git for that path's toplevel and tests for the vendored front end and for `bash`, and it exits 0 with nothing on stdout where any of the three fails. Otherwise it enters the toplevel and hands the payload to the front end as `--hook <member> --binding antigravity`. The suffix prints the front end's stdout only where it opens with the deny envelope, and exits 0 whatever happened. So the front end's own operator notice, which Antigravity would read as a deny, never reaches it, and the prefix never fails a call.

The hook runs in the plugin's own directory, so it finds the repository from the payload rather than from the working directory. On native Windows Antigravity runs the command under `cmd`, which reads the leading `rem` as a comment and exits 0 with nothing on stdout. There the package installs and runs no guard. The oracle is `plugin_parity.rs`'s Windows-only unit case, which runs one composed command under `cmd /c` on a payload and holds exit 0 and an empty stdout. It composes the command line as Go's `syscall.EscapeArg` composes one argument, which is inferred: the hooks page names `cmd /c` and nothing more. A red there withdraws `plugin/hooks.json` and (C)'s Antigravity half, since a rendering `cmd` fails would fail every matched call on Windows.

context-kit's session-start wiring is not rendered: it is a context brief rather than a guard.

## The marketplace pin

`.claude-plugin/marketplace.json` carries `name` `checkwright`, `owner`, a one-sentence `description`, and one plugin entry: `name` `checkwright`, a `source` of `{"source": "git-subdir", "url": "checkwright/checkwright", "path": "plugin", "ref": "v<pin>"}`, `version` `<pin>`, and the manifests' `description` and `author`.

`<pin>` is the hosted install pin ([installer/SPEC.md §The hosted install pin](../installer/SPEC.md#the-hosted-install-pin)), so the plugin and the one-line install always serve one release. The pin moves in the commit after the tag that moves the install pin (RELEASING.md), and the entry's `ref` and `version` move with it.

**Publication is a release.** The package ships from the pinned tag, so a package the pinned release does not carry is published by the next release's pin move, and until then the marketplace names a tag the harness cannot install from. `check-plugin-parity` assertion E holds that to the close's release disposition.

## check-plugin-parity

A repo-root gate (this repo's `scripts/`), born native in `native/src/gates/plugin_parity.rs`. It checks only what no external tool knows, that the package matches this repository; the manifests' own validity is the validation leg's, whose oracles are the harness's CLI and the published schema, since a reader of our own would re-implement schemas that drift.

- **(A) The manifests agree.** Both parse. `name`, `description`, `author`, `homepage`, `repository` and `license` are present and equal across the two. Neither carries `version`. The portable manifest's `$schema` is `https://agent-plugins.org/schemas/<X.Y.Z>/plugin.schema.json`.
- **(B) The skills match the repository.** The skill directories equal, as a set, the binding shims in `LIFECYCLE_KIT_SKILLS_DIR` plus `install`. Each `SKILL.md` carries front matter whose `name` is its directory and whose `description` is 1 to 1024 characters. Each value written unquoted is one a YAML plain scalar can carry, so a slip the validation leg's `skills-ref` step would reject reds before a push. Such a value opens with no indicator character (`[]{},#&*!|>'"%@` or a backtick, or `-`, `?` or `:` followed by a blank), and carries no `:` before a blank or at its end and no `#` after a blank. The quoted form carries any of them. A line whose first non-blank character is `#` is a comment, never a field. Each non-install body, after the front matter and its following blank lines, equals §The skills' rendering for the template its shim names, byte for byte, and that template exists.
- **(C) The hooks match the template.** `plugin/hooks/hooks.json` equals §The guards' rendering of `guard-kit/templates/settings-hooks.json`, and `plugin/hooks.json` equals its Antigravity rendering, each compared as JSON. So a block the template gains reds until both files carry it, or until the Antigravity rendering leaves it out by rule.
- **(D) The marketplace is pinned.** `.claude-plugin/marketplace.json` carries exactly one entry, named as the manifests. Its source is `git-subdir`, with `path` `plugin` and `url` the manifests' repository slug. Its `ref` is `v<pin>` and its `version` is `<pin>`, `<pin>` read by `check-install-pin`'s own pin reader.
- **(E) The pinned release carries the package.** `v<pin>:plugin/.claude-plugin/plugin.json` resolves. A tag lacking it is admitted on the pending terms of [installer/SPEC.md §The front door's verbs](../installer/SPEC.md#the-front-doors-verbs), through that section's reader: while `.workflow/release-disposition.txt` carries no line for the queue header's iteration, or a line whose field is a release. A `none` or `deferred:` field reds it, which leaves the close two remedies, releasing or withdrawing the marketplace file. Where `v<pin>` does not resolve, as in a shallow checkout, E is dormant and the clean line says so.
- **(F) The plugin carries the license text.** `plugin/LICENSE` equals the repository's root `LICENSE` byte for byte, since the marketplace installs `plugin/` alone. A red prints the remedy, `cp LICENSE plugin/LICENSE`.

Each red names the file, the field and the expected value; a (B) body red and a (C) red print the expected rendering. A missing or unreadable file, unparsable JSON, a missing skills dir, and the pin's own refusals exit 2. The marketplace file is the one exception: its absence is the withdrawn state E's remedy leaves, so D and E are dormant there and the clean line says so. It reads `LIFECYCLE_KIT_SKILLS_DIR`, declared on its registry row, and runs at `precommit`, coupled to `plugin/`, `.claude-plugin/`, the skills dir's shims, the hooks template, `docs/install.sh`, the disposition file, the root `LICENSE` and its module. Its positional form, `check-plugin-parity [plugin-dir marketplace skills-dir hooks-template install-sh disposition queue pinned license]`, points it at a fixture tree, `license` naming the root file F compares against. `pinned` is a file whose first word, `carries` or `lacks`, stands for the tag's answer, or `-` for a tag that does not resolve, so the fixture holds still as this repository's tags move.

## The validation leg

The `plugin-validate` job in `.github/workflows/gates.yml` runs each external oracle at a pinned version:

- **The harness's CLI.** It installs `@anthropic-ai/claude-code` from npm at a pinned version and runs `claude plugin validate plugin` and `claude plugin validate .`. Each must pass. The missing-version warning is expected (§The manifests), and `--strict` is not used, since it turns that warning into a failure. The marketplace check reads the entry's shape and fetches nothing.
- **The portable manifest.** It fetches the published schema and validates `plugin/plugin.json` against it with `ajv-cli` under JSON Schema draft 2020-12, the draft the schema declares.
- **The skills.** It validates each skill directory with the Agent Skills reference validator, `skills-ref validate <dir>`, installed at a pinned commit. That library says it is for demonstration and not production, so a finding it raises is read against the Agent Skills specification before a skill is changed for it.

A build that changes the package runs the same commands locally, then installs it into each harness it verifies, under a scratch configuration home, so the user's own configuration is untouched:

- **Claude Code.** From a scratch marketplace with a relative source into a scratch harness config, to see the skills registered; `CLAUDE_CONFIG_DIR` pointed at a scratch directory. A scratch config carries no login, so the guards are seen in a session that loads the package with `--plugin-dir`, which installs nothing: firing in a vendored scratch repository and silent in an empty one. Two more sessions follow, and the guard fires in both: one launched from a subdirectory of the vendored repository, and one whose Bash tool changes into a subdirectory before its next call.
- **Codex.** `codex plugin marketplace add` of the repository and `codex plugin add checkwright@checkwright`, `CODEX_HOME` pointed at a scratch directory, then `codex plugin list`. `codex debug prompt-input` shows the skills the model is given, and the app server's `hooks/list` shows which hooks are discovered. A scratch home carries no login, so a session is opened through a model provider configured in that home and served from the local host, which answers with the tool call under test. Where the hooks are discovered, the shell guard fires in a vendored scratch repository and stays silent in an empty one.
- **Antigravity.** `agy plugin validate plugin` and `agy plugin install ./plugin`, `HOME` pointed at an absolute scratch directory, since `agy` refuses a relative one, then `agy plugin list`, which counts the skills and the hook. Where `agy` is signed in, a vendored scratch repository holding a copy of the package under `.agents/plugins/checkwright/` is opened with `agy -p`. There the shell guard denies a `cat` of a tracked file, `workflow-state-guard` denies a `write_file` to the state file, and both stay silent in an empty repository. A scratch `HOME` carries no login, so the session runs in the signed-in home with the package workspace-local, and writes nothing to that home's configuration. Headless mode refuses a `run_command` it cannot prompt for after a hook takes no decision, so the shell half reads the hook's deny in the vendored repository and the harness's own refusal in the empty one.

## Honest limits

- Claude Code's, Codex's and Antigravity's installs are run; every other harness in §The harnesses is read off its own documentation.
- Codex runs no guard from this package, and once it discovers them, still no wakeup guard (§The guards).
- Codex records an install at version `1.0.0`, since neither manifest carries `version` (§The manifests). Whether a pin move reaches an installed copy through `codex plugin marketplace upgrade` is unrun.
- Cursor and Muse Code run no guard from this package, and Antigravity none on native Windows.
- A write through an Antigravity file-write tool whose arguments carry neither `Path` nor `TargetFile` reaches `workflow-state-guard` with an empty path, and the guard allows it; which tools have never fired is the Antigravity table's measured record (gate-sdk/SPEC.md §The hook contract). Any edit tool the matcher does not name is unguarded.
- Under Antigravity the guards neither grant nor advise: a grant becomes no decision, its safety resting on the shipped binding's matcher, and no advise channel was found (gate-sdk/SPEC.md §The harness-integration arm). The prefix's `sed` fails on a workspace path whose JSON escapes a quote or backslash, or a multi-line payload (live ones were compact), unguarding that session.
- Whether an installed copy's hooks fire is inferred from the workspace copy's, which share Antigravity's plugin loader. While the CLI's Agent Plugins flag is on, a package declaring the Agent Plugins `$schema` may load without its hooks, since that standard excludes hooks.
- (B)'s front-matter reader is a subset of YAML: a quoted value's escapes and a value continued onto a later line are not read, so the validation leg still catches a slip in either.
- A skill takes each slot's own text as its binding, which is weaker than a repository's own bound skill.
- An adopter who also merged guard-kit's settings wiring runs each guard twice, and the friction log counts each fall-through twice, so the README says to keep one of the two.
- The fail-open prefix means a tree whose front end is missing runs unguarded and says nothing about it.
- On native Windows the harness hands the hook `CLAUDE_PROJECT_DIR` in a Windows spelling. That `git -C` accepts it and `test -f` accepts the forward-slash toplevel git answers is inferred rather than run: no Windows host has driven the prefix.
