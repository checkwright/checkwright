# SPEC amendment: plugin

Checkwright is in no harness plugin catalog, and adopters now find enforcement there. This amendment adds a plugin package that registers the stage skills and wires the guards, and a marketplace that lists it. It is a **discovery surface**: installing the plugin installs no kit. Its install skill runs the installer's `init`, which remains the only thing that vendors, and the gates stay in the binary outside every harness. It pairs [plugin-marketplace](TASK-QUEUE.md#plugin-marketplace), the second pick of the `catalog-then-plugin` ruling (TRAJECTORY.md).

**The target formats, read at authoring.** The entry's standing ruling is that the plugin substrate moves fast and the design is made against the live format at promotion. What follows was read from the published specifications and the harness's own documentation, and the local rows were run.

- **Agent Plugins 1.0** (agent-plugins.org, the specification and its schemas). The manifest is `plugin.json` at the package root. It requires `$schema` and `name`, and admits `version`, `description`, `author`, `homepage`, `repository`, `license`, `keywords` and a reverse-domain `extensions` object. Skills are the immediate subdirectories of `skills/` holding a `SKILL.md`, in the Agent Skills format (agentskills.io): YAML front matter requiring `name`, equal to the directory's name, and `description`, 1 to 1024 characters. Hooks, commands and agents are outside the v1 portable core. The standard defines no marketplace, no install from a repository and no validator; it leaves distribution to each client.
- **Claude Code's plugin format** (code.claude.com, the plugin, manifest, marketplace and skills references). The manifest is `.claude-plugin/plugin.json`, requiring only `name`. It reads the same `skills/<name>/SKILL.md` layout and a standard-layout `hooks/hooks.json`. A marketplace is `.claude-plugin/marketplace.json` at a repository's root, listing plugins by `name` and `source`, and a `git-subdir` source names a repository, a path in it and a `ref`. `/plugin marketplace add <owner>/<repo>` reads that root file. Its documentation does not mention Agent Plugins, so the two manifests are two documents.
- **Run locally with Claude Code 2.1.283**, on a scratch package carrying both manifests, one skill and a hooks file, and on a scratch marketplace. `claude plugin validate <package>` passed, with warnings for no `version` and no `author` in `.claude-plugin/plugin.json`. `claude plugin validate <marketplace-root>` passed with an entry whose source is `git-subdir` with a `ref`; it checks that entry's shape and fetches nothing. `--strict` turns every warning into a failure.

**The rulings.**

- **One package, two manifests, one skill tree.** The two formats read the same `skills/` layout, so the package carries the portable `plugin.json` and the harness's `.claude-plugin/plugin.json` side by side over one set of skills. A client implementing either reads its own manifest.
- **The component is `plugin/`, repo-root-governed and not a kit.** It carries no `checks/` and no `smoke/`, so no kit-root resolver admits it. It holds `SPEC.md` (the design record) and `README.md` (the usage tier), as `companion/` does. Vendor names live here, in the root marketplace file and on the docs pages, and never in a kit SPEC or kit literal (gate-sdk/SPEC.md §The provenance seam).
- **A skill runs a vendored template; it binds nothing.** A stage template's slots are the consumer's to bind in its own skill. That is rule content, and a package serving every adopter cannot carry it. So each plugin skill names its template, defers to a skill of the repository's own that binds it, and otherwise takes each slot's own text as its binding. Where the template is absent, the skill says Checkwright is not installed and points to the install skill.
- **The skill roster is this repository's own.** The plugin registers exactly the templates this repository binds in `LIFECYCLE_KIT_SKILLS_DIR`: what it ships is what its own iterations run. It adds one skill, `install`. The roster is derived and gated, never listed.
- **The guards ride the harness's extension, not the portable core.** The roadmap summary promises the guards, and the standard excludes hooks from its core. So `hooks/hooks.json` carries guard-kit's hook wiring for the one harness that reads it, and the portable manifest carries skills alone. Each hook command first tests for the vendored front end and exits 0 without it, because a plugin's hooks fire in every repository where the plugin is enabled.
- **The version rides the marketplace pin, never a tracked manifest.** The marketplace entry pins the plugin to a release tag with `ref` and states the same version. Neither manifest carries `version`: in Claude Code a `plugin.json` version wins over the entry's, so a tracked one would be a second version line to bump by hand, and a stale one would stop updates. The pin is the hosted install pin (installer/SPEC.md §The hosted install pin), so the plugin and the one-line install always serve one release.
- **Publication is a release.** The marketplace lands with the package, pinned to the current install pin, a tag that carries no `plugin/`. Delta 6's assertion E reds that and admits it on the pending terms `check-front-door-verbs` already uses. So this iteration's close either releases, and the pin move after the tag makes the marketplace live, or withdraws the marketplace file. The entry's push need already names that tag.

**Refused.**

- **A kit carrying the package.** A kit literal naming a harness crosses the provenance seam.
- **Vendoring through the plugin.** A second install model with no manifest, update or uninstall story, which the entry has always ruled out. The install skill hands to `init`.
- **Binding slots in the plugin's skills.** A slot's value is a consumer's rule content, and a default chosen here would be this repository's calibration shipped as everyone's.
- **A tracked `version`.** Above.
- **Validating the manifests with a reader of our own.** It re-implements schemas that drift. The harness's CLI and the published schema are the oracles (delta 8). The parity gate checks only what no external tool knows: that the package matches this repository.
- **The session-start brief in the package.** context-kit's session-start wiring is a context brief, not a guard, and the roadmap promise is the guards.

## What changes

### (1) The component

**Not yet applied.** Create `plugin/` with its design record and usage tier, and bring it under governance {design-bearing}.

- `plugin/README.md` — the usage tier: what the plugin registers, the marketplace commands a Claude Code user runs (`/plugin marketplace add checkwright/checkwright`, then `/plugin install checkwright@checkwright`), that any Agent Plugins client loads `plugin/` by its own install route, and that the install skill runs `init`.
- `plugin/SPEC.md` — the design record, holding the rulings above as current rules: the two manifests and what each carries; the skill roster and body (delta 3); the hooks rendering (delta 4); the marketplace pin (delta 5); the parity gate's assertions (delta 6); the validation leg (delta 8). Its honest limits:
  - only Claude Code's install is run, and every other Agent Plugins client is read off the standard;
  - a skill takes slot text as bindings, which is weaker than a repository's own bound skill;
  - an adopter who also merged guard-kit's settings wiring runs each guard twice, and the friction log counts each fall-through twice, so the plugin README says to keep one of the two;
  - the fail-open prefix means a tree whose front end is missing runs unguarded, and says nothing about it.
- `scripts/root-allowlist.list` gains `plugin` and `.claude-plugin`, each under a comment: the plugin package, not a kit; the marketplace manifest the harness reads at the repository root.
- `scripts/canon-config.knobs`: `CANON_KIT_SEAM_SURFACE_GLOBS` gains `plugin/SPEC.md`, beside `companion/SPEC.md`. `CANON_KIT_MANIFEST_FILES` gains `plugin/README.md`. `CANON_KIT_PROSE_SURFACE_GLOBS` gains `plugin/skills/*/SKILL.md`, on the Spec Kit commands' precedent.

### (2) The two manifests

**Not yet applied.** Write `plugin/plugin.json` and `plugin/.claude-plugin/plugin.json` {design-bearing}.

- `plugin/plugin.json`, the portable manifest: `$schema` `https://agent-plugins.org/schemas/1.0.0/plugin.schema.json`, `name` `checkwright`, a one-sentence `description`, `author` `{"name": "Checkwright", "url": "https://checkwright.dev"}`, `homepage` `https://checkwright.dev`, `repository` `https://github.com/checkwright/checkwright`, the repository `installer/package.json` names, and `license` `Apache-2.0`. No `keywords`, no `extensions` and no `version`.
- `plugin/.claude-plugin/plugin.json`: the same `name`, `description`, `author`, `homepage`, `repository` and `license`. No `version`, and no component paths, since the standard layout already locates `skills/` and `hooks/hooks.json`. `claude plugin validate plugin` then warns only on the missing version, which is the pin's ruling (above), and passes.

### (3) The skills

**Not yet applied.** One `plugin/skills/<name>/SKILL.md` per binding directive in `LIFECYCLE_KIT_SKILLS_DIR`, named for its shim, plus `plugin/skills/install/SKILL.md` {design-bearing}.

- **The roster.** Probed with `grep -l 'Execute the template at' .claude/commands/*.md`: `agent-execution`, `align`, `build`, `close`, `consult`, `economics`, `lead`, `release-sweep`, `scope`, `spec` and `validate`. Each skill's `<path>` is the template its shim's directive names.
- **Front matter.** `name` is the directory's name. `description` is authored per skill: what the skill does, in one or two sentences, and that it runs only when the user asks for it. No other key, so the file stays inside the Agent Skills base set.
- **The body is one fixed rendering.** Its only variable is `<path>`, and delta 6 holds it byte for byte:

  ```
  Execute the template at `<path>` in this repository.

  - If this repository has a skill of its own that binds `<path>`, run that skill instead.
  - Otherwise, take each `*<slot: …>*` placeholder's own text as its binding.
  - If `<path>` does not exist, Checkwright is not installed here, or not with the kit that ships this template. Say so, and stop: the `install` skill installs it.
  ```

- **`install`.** Its body tells the agent four things:
  - Choose a profile with the user, from the installer README's *Choosing a profile* section, linked by its GitHub URL.
  - Confirm the worktree is clean.
  - Run the one-line install for the host with `init --profile <profile>` as its arguments: `curl -fsSL https://checkwright.dev/install.sh | sh -s -- init --profile <profile>`, and on Windows its PowerShell twin in the form docs/install.md gives.
  - Run each command in the `next:` block `init` prints (installer/SPEC.md §init). It carries no template directive.

### (4) The guards

**Not yet applied.** Write `plugin/hooks/hooks.json` as a rendering of `guard-kit/templates/settings-hooks.json` {design-bearing}.

- The rendering takes each block under `hooks`: its event, its matcher and its hook entries, in order.
- It rewrites each `command` from `<command>` to `test -f gate-sdk/bin/run-gates.sh || exit 0; <command>`.
- It drops the template's `//` note, which is merge advice for a settings file.
- The harness runs a hook command from the project root under a POSIX shell, Git for Windows' bash on native Windows (guard-kit/SPEC.md §The hook on native Windows). So the test resolves the vendored front end where it exists and exits 0 where it does not.

**Inferred, cannot run before build:** that a plugin hook's working directory is the project root, as the template's relative command already assumes for settings hooks — the package does not exist until build lands it, and delta 8's scratch install settles it.

### (5) The marketplace and its pin

**Not yet applied.** Write `.claude-plugin/marketplace.json` at the repository root, and move its pin with the install pin {design-bearing}.

- The file: `name` `checkwright`, `owner` `{"name": "Checkwright"}`, a one-sentence `description`, and one plugin entry. The entry has `name` `checkwright`; a `source` of `{"source": "git-subdir", "url": "checkwright/checkwright", "path": "plugin", "ref": "v<pin>"}`; `version` `<pin>`; and the `description` and `author` of delta 2.
- `<pin>` is `docs/install.sh`'s pin. The file lands at build with the current pin, which delta 6's assertion E admits as pending.
- RELEASING.md, the step that moves the install pin after the tag: the same commit moves the entry's `ref` and `version`.
- installer/SPEC.md §The hosted install pin gains one sentence: the plugin marketplace's pin moves with it, held by `check-plugin-parity` (plugin/SPEC.md).

### (6) `check-plugin-parity`

**Not yet applied.** A repo-root gate, born native: `native/src/gates/plugin_parity.rs`, `scripts/check-plugin-parity.gate`, a `good/`+`bad/` pair under `scripts/gate-tests/check-plugin-parity/`, and a line in `scripts/gates.list` {design-bearing}.

- **(A) Manifests agree.** Both parse. `name`, `description`, `author`, `homepage`, `repository` and `license` are equal across the two. Neither carries `version`. The portable manifest's `$schema` names a published Agent Plugins schema URL.
- **(B) Skills match the repository.** The skill directories equal, as a set, the binding-directive shims in `LIFECYCLE_KIT_SKILLS_DIR` plus `install`. Each `SKILL.md` carries front matter whose `name` is its directory and whose `description` is 1 to 1024 characters. Each non-install body equals delta 3's rendering for the template its shim names, and that template exists.
- **(C) Hooks match the template.** `plugin/hooks/hooks.json` equals delta 4's rendering of `guard-kit/templates/settings-hooks.json`, compared as JSON.
- **(D) The marketplace is pinned.** `.claude-plugin/marketplace.json` carries exactly one entry, named as the manifests. Its source is `git-subdir` with `path` `plugin` and `url` the manifests' repository slug. Its `ref` is `v<pin>` and its `version` is `<pin>`, where `<pin>` is read by `check-install-pin`'s own pin reader.
- **(E) The pinned release carries the package.** `v<pin>:plugin/.claude-plugin/plugin.json` resolves. This is admitted while `.workflow/release-disposition.txt` carries no line for the queue header's iteration, or a line whose field is a release, through `check-front-door-verbs`' reader; a `none` or `deferred:` field reds it. It is dormant, and says so, where `v<pin>` does not resolve.
- **Red messages** name the file, the field and the expected value. The (B) red prints the expected body. A missing file or unparsable JSON exits 2.
- **Wiring.** It reads `LIFECYCLE_KIT_SKILLS_DIR`, declared on its registry row. It is `precommit`, and its `couples=` names `plugin/`, `.claude-plugin/`, the skills dir, `guard-kit/templates/settings-hooks.json`, `docs/install.sh`, `.workflow/release-disposition.txt` and its module.
- **Arguments.** A positional form points it at a fixture tree, with the pinned tag's answer read from a file, on `check-front-door-verbs`' precedent. The `bad/` case is a skill body drifted from the rendering; the `good/` case is a matching tree.

### (7) The front door's verbs

**Not yet applied.** The install skill advertises `init` on a route, so it joins the front-door pages {mechanical}.

- installer/SPEC.md §The front door's verbs, invariant B's page list, gains `plugin/skills/install/SKILL.md`.
- `native/src/gates/front_door_verbs.rs` gains that path on its page roster.

### (8) The validation leg

**Not yet applied.** A `plugin-validate` job in `.github/workflows/gates.yml`, each tool pinned by version, beside the `companion-toolkits` job {design-bearing}.

- **The harness's CLI.** It installs `@anthropic-ai/claude-code` from npm at a pinned version and runs `claude plugin validate plugin` and `claude plugin validate .`. Each must pass. The missing-version warning is expected, and `--strict` is not used. Measured at authoring: `npm view @anthropic-ai/claude-code version` answers `2.1.283`, the version the local runs above used.
- **The portable manifest.** It fetches the published schema and validates `plugin/plugin.json` against it. The schema declares JSON Schema draft 2020-12. Measured at authoring on the scratch manifest: `npx -p ajv-cli@5.0.0 -p ajv-formats@3.0.1 ajv validate --spec=draft2020 -c ajv-formats -s plugin.schema.json -d plugin.json` printed `valid`.
- **The skills.** It validates each skill directory with the Agent Skills reference validator, `skills-ref validate <dir>`. That library says it is for demonstration and not production, so its commit is pinned and a finding it raises is read against the Agent Skills specification before a skill is changed for it.
- **The build session runs the same commands locally.** It also installs the plugin into a scratch harness config from a scratch marketplace with a relative source. That run shows the skills registered, and a guard firing in a vendored scratch repository and staying silent in an empty one.

**Inferred, not run:** the Agent Skills validator installs from its repository's subdirectory with pip — pip install "git+https://github.com/agentskills/agentskills@<sha>#subdirectory=skills-ref" && skills-ref validate plugin/skills/scope

Measured at authoring: `CLAUDE_CONFIG_DIR=<scratch> claude plugin marketplace list` printed `No marketplaces configured` where the unset form lists the operator's own, so a scratch install leaves the operator's config untouched.

### (9) The docs

**Not yet applied.** docs/install.md gains a section, *From a plugin marketplace*, and the front door gains one link {design-bearing}.

- The section gives the two Claude Code commands, says the plugin's install skill runs the same `init` this page documents, and that any Agent Plugins client loads the repository's `plugin/` directory by its own install route.
- `README.md`'s install paragraph links it.
- The generated on-site mirror gains `docs/plugin/SPEC.md` and `docs/plugin/README.md`, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`, and `scripts/docs-offnav.list` gains `docs/plugin/README.md`, as it carries `docs/companion/README.md`.

## Producers and consumers

- **The package** (`plugin/`).
  - Producer: hand edits, held by delta 6.
  - Consumers: Claude Code, reading `.claude-plugin/plugin.json`, `skills/` and `hooks/hooks.json` at the pinned tag; any Agent Plugins client, reading `plugin.json` and `skills/`; the validation leg (delta 8).
  - Every manifest field has a reader. `name` keys install; `description`, `author`, `homepage`, `repository` and `license` are what a listing shows a user deciding to install; `$schema` is read by the portable client and by the schema check. `keywords` is left out because no reader of it was found.
- **A skill.**
  - Producer: a hand edit whose body delta 6 holds.
  - Consumer: the harness, on the user's invocation. The skill's own consumer is the vendored template, or the repository's bound skill, or the install skill.
  - Red condition: (B), on a roster drift or a body drift.
- **The hooks file.**
  - Producer: a hand edit, held by (C) to its source.
  - Consumer: Claude Code, at each matching tool call.
  - Red condition: (C), on any drift from `guard-kit/templates/settings-hooks.json`, including a new block that template gains.
- **The marketplace file.**
  - Producer: a hand edit at build, then the pin move at each release.
  - Consumer: `/plugin marketplace add`, and the harness's update check, which reads `version`.
  - Red conditions: (D) and (E).
- **`check-plugin-parity`.**
  - Producer: the battery and the generated pre-commit hook, which runs it on a change to any coupled path.
  - Consumer: the committing session.
- **Roster-holding readers of the minted names.**
  - `check-root-tiering` reads `scripts/root-allowlist.list` (delta 1).
  - `check-gate-fixture-coverage`, `check-gate-substrate-parity` and `check-reads-couples` read the new gate's descriptor, pair and declared read (delta 6).
  - `check-front-door-verbs` reads the install skill (delta 7).
  - `check-docs-mirror-fresh` reads every top-level directory holding a `SPEC.md` (delta 9).
  - The canon-kit gates read the three knob globs (delta 1).
  - `check-kit-ref-liveness` reads every tracked path segment, which the component's naming satisfies.

## Existing sections updated

Roster probes, over the tracked tree: `ls plugin .claude-plugin` (both absent); `grep -l 'Execute the template at' .claude/commands/*.md`; `git grep -n` for `companion` over `scripts/*.knobs`, `scripts/*.list` and `docs/site-architecture.md`; `grep -n 'pin'` over RELEASING.md; `grep -n 'README.md\|docs/index.md'` over `native/src/gates/front_door_verbs.rs`.

- `plugin/README.md`, `plugin/SPEC.md`, `scripts/root-allowlist.list`, `scripts/canon-config.knobs` (delta 1).
- `plugin/plugin.json`, `plugin/.claude-plugin/plugin.json` (delta 2).
- `plugin/skills/` (delta 3).
- `plugin/hooks/hooks.json` (delta 4).
- `.claude-plugin/marketplace.json`, `RELEASING.md`, `installer/SPEC.md` — §The hosted install pin (delta 5).
- `native/src/gates/plugin_parity.rs`, `scripts/check-plugin-parity.gate`, `scripts/gate-tests/check-plugin-parity/`, `scripts/gates.list`, `scripts/git-hooks/pre-commit` (regenerated), and the coupling graph (delta 6).
- `installer/SPEC.md` — §The front door's verbs; `native/src/gates/front_door_verbs.rs` (delta 7).
- `.github/workflows/gates.yml` (delta 8).
- `docs/install.md`, `README.md`, `scripts/docs-offnav.list` (delta 9).
- `docs/plugin/SPEC.md`, `docs/plugin/README.md`, `docs/installer/SPEC.md` — the generated on-site mirrors (deltas 1, 5, 7 and 9).
<!-- update-target-exempt: the Definition of Done's Done move regenerates it, and no delta edits it -->
- `ROADMAP.md` — regenerated at the Done move, which drops the entry's roadmap tags.

## Retired spellings

- None — the unit adds a component, two manifests, a gate and a job, and retires no name.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; no root-level amendment remains for this unit (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Run** — the full battery green with `check-plugin-parity` registered, and delta 8's commands and scratch install run locally, their output recorded in the build commit.
- [ ] **The entry closes with the build.** The merge moves [plugin-marketplace](TASK-QUEUE.md#plugin-marketplace) to Done in the build commit that deletes this file, before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`), and the same commit regenerates `ROADMAP.md` with `--emit roadmap --write`, since the move takes the entry's roadmap tags with it. Publication is the close's release disposition, which assertion E holds. The `catalog-then-plugin` discharge stays with the ruling record.
