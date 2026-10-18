# SPEC amendment: harness-reach

Pairs `plugin-harness-reach`. The package is verified on Claude Code alone, and its guards reach no other harness. This amendment takes the entry's three deliverables:

- (1) the skills' load verified on the Codex and Antigravity CLIs;
- (2) a survey of which harnesses read the package, recorded as a section of this SPEC;
- (3) the guard wiring for each harness whose hook protocol the shipped wiring already speaks.

**The reach is decided (2026-10-18, intent oracle, relayed by the lead; not an operator ruling).**

- **What the iteration wires:** Codex, through the existing rendering, verified by a run. That follows the operator direction of 2026-09-28 the entry carries: tier 1 is Claude Code and Codex, tier 2 is Muse Code, Antigravity and Cursor.
- **Codex's two gaps are declared limits**, and each is also filed.
- **What is filed, not built:**
  - an Antigravity hook binding;
  - the Cursor and Muse Code observation;
  - Codex's `apply_patch` silent allow.

  Each was filed at spec as a costed gap bullet, for close's drain to make a Deferred entry (lifecycle-kit/SPEC.md §The committed gap inbox).
- **Refused, with grounds:**
  - *Building the Antigravity binding now.* Its hook runs in the plugin's own directory with no project-directory variable, so it needs a launcher reading the payload to find the repository, and that is interpreter surface (gate-sdk/SPEC.md §The adopter constraints).
  - *Teaching `workflow-state-guard` to read `apply_patch`'s file headers.* That is reader work deliverable (3) does not settle.

**Delta 2 narrowed at build (2026-10-18, intent-oracle decision, relayed by the lead; not an operator ruling): option B.** The build's Codex run disproved delta 2's premise, so the measured limit is Q1's answer applied to it:

- **Run, `codex-cli 0.162.1`, scratch `CODEX_HOME`, a local mock model provider opening the session:** the 13 skills reach the model's prompt. The package's hooks are never discovered while the root `plugin.json` is present: `hooks/list` and `plugin/read` return no hooks, whether or not that manifest carries `extensions.com.openai.hooks` or a `.codex-plugin/plugin.json` sits beside it. A scratch copy without the root manifest has `hooks/hooks.json` discovered. Under it, with hook trust bypassed for the invocation, the rendering blocks in a vendored scratch repository, including from a launch in a subdirectory, and stays silent in an empty one. That answers delta 2's inferred marker.
- **So Codex is not wired.** The Codex row of §The harnesses records both measured halves, each marked **run** with its version. The guard bullets name Claude Code alone, and the honest limits name the discovery gap. The root manifest, §The manifests and Antigravity's route stand.
- **Filed, not built:** Codex guard wiring, costed as tier-1 guard wiring that waits on an upstream discovery fix. It is re-checked when a Codex release discovers hooks with `plugin.json` still at the root. The likely upstream defect is named in the gap bullet only; reporting it upstream is outside this iteration.
- **Refused, with grounds:**
  - *Removing the root `plugin.json`* reverses §The manifests, and Antigravity's validate then fails with "missing plugin.json" (run).
  - *A second, Codex-only package directory* is a new published surface, which is a scope change.

The deltas below are merged as this decision narrows them. Where a delta's quoted text says Codex runs the guards, the decision governs.

**What was run at spec.** None of this touched the user's harness configuration:

- **`codex-cli 0.162.1`, `CODEX_HOME` pointed at a scratch directory.**
  - `codex plugin marketplace add ./` read `.claude-plugin/marketplace.json` unchanged.
  - `codex plugin add checkwright@checkwright` installed the package from the entry's `git-subdir` source at `v0.31.0` into the scratch cache, with 13 skill directories and `hooks/hooks.json`.
  - `codex plugin list` showed it `installed, enabled` at version `1.0.0`, a version neither manifest nor the entry carries.
- **`agy` 1.3.4, `HOME` pointed at a scratch directory.**
  - `agy plugin validate plugin` and `agy plugin install ./plugin` each printed `skills : 13 processed` and `hooks : skipped (not found)`.
  - `agy plugin list` showed `Components: 13 skills`.
- **Not run:** skill registration inside a logged-in session, on either CLI.

Every other fact below is read off the harness's own documentation at the URL named, and is marked so.

## What changes

### (1) A section records which harnesses read the package {design-bearing} {user-facing: plugin-harness-reach's Deliverable (2), "a survey of which harnesses read Agent Plugins 1.0 and their catalogs"}

A new `plugin/SPEC.md` section, **## The harnesses**, after §The component. **Not yet applied.**

> ## The harnesses
>
> The package is one tree, and each harness reads what its own format reaches of it. The standard names no adopter, so a harness is listed here once its own documentation or a run shows it reading the package. Each cell says which: **run** is a command this repository's build ran, and **read** is the harness's documentation, at the URL given below the table.
>
> | harness | skills | hooks | catalog |
> |---|---|---|---|
> | Claude Code | run: `.claude-plugin/plugin.json`, `skills/` | run: `hooks/hooks.json`, this rendering | run: `.claude-plugin/marketplace.json` |
> | Codex | run: the portable `plugin.json`, `skills/` | run: `hooks/hooks.json`, this rendering, once the user trusts it in `/hooks` | run: `.claude-plugin/marketplace.json`, read as Codex's legacy location; a `git-subdir` source |
> | Antigravity | run: the portable `plugin.json`, `skills/`, installed from a local directory | not wired: its protocol differs (§The guards) | read: an official marketplace only |
> | Cursor | read: an Agent Plugin's `plugin.json` and `skills/` | not wired from the plugin: it reads hooks only from its own plugin format, and imports a project's Claude Code settings wiring by default | read: its own `.cursor-plugin/marketplace.json` |
> | Muse Code | read: Claude Code-format plugins | read: a Claude Code-format plugin's hooks, unsupported events skipped; its payload is undocumented | read: `muse plugins marketplace` |
>
> Sources, observed 2026-10-18: developers.openai.com/codex/plugins/build and learn.chatgpt.com/docs/hooks for Codex; the CLI's built-in plugin and hooks documentation for Antigravity; cursor.com/docs/plugins, cursor.com/docs/hooks and cursor.com/docs/reference/third-party-hooks for Cursor; dev.meta.ai/docs/muse-code for Muse Code. A harness not in the table is unread, not unsupported.

§The component's sentence "The harness names live here, in the root marketplace file and on the docs pages" stands, since the section is inside `plugin/`.

### (2) The guards reach Codex through the same rendering {design-bearing} {user-facing: plugin-harness-reach's Deliverable (3), "per-harness guard wiring where a harness has a hook surface", as decided 2026-10-18}

Replacement for `plugin/SPEC.md` §The guards, its first sentence. **Not yet applied.**

> `plugin/hooks/hooks.json` is a rendering of `guard-kit/templates/settings-hooks.json`, for the harnesses that read a plugin's hooks in the shipped binding's protocol (gate-sdk/SPEC.md §The hook contract): Claude Code, and Codex. The portable manifest carries skills alone, since the standard excludes hooks from its core.

**This act depends on the sibling entry `hook-contract-harness-neutral`**, which the set record orders first:

- where its amendment has merged when this delta lands, the sentence cites gate-sdk/SPEC.md §The hook contract, as written;
- where it has not, the parenthesis cites gate-sdk/SPEC.md §The harness-integration arm instead, and that merge repoints it.

The same holds for the field-by-field citation under §Producers and consumers.

A paragraph after the one opening "The prefix differs from the settings rendering's anchor". **Not yet applied.**

> **Codex runs the same rendering, on three terms of its own.**
>
> - **It runs a plugin's hooks only after the user trusts them** in its `/hooks` view, and a changed rendering is skipped until trusted again.
> - **It sets no project-directory variable and runs a hook in the session's working directory**, so the prefix's fallback to `.` is the path it takes, and the toplevel lookup is the same one.
> - **Two of the rendering's blocks reach less there.** No Codex tool matches `ScheduleWakeup|CronCreate`, so the wakeup guard never fires. Its patch tool matches `Write|Edit` but carries the patch text, never a written path, so `workflow-state-guard` reads no path and allows (§Honest limits).
>
> **Antigravity is not wired.** Its hook payload, its decision envelope and its hook file's shape each differ from the shipped binding, and it runs a hook in the plugin's own directory with no project-directory variable, so a command there cannot find the repository's front end without reading the payload. It installs this package with its hooks skipped.

### (3) The validation leg runs the package on Codex and Antigravity {design-bearing}

Replacement for `plugin/SPEC.md` §The validation leg, its last paragraph. **Not yet applied.**

> A build that changes the package runs the same commands locally, then installs it into each harness it verifies, under a scratch configuration home, so the user's own configuration is untouched:
>
> - **Claude Code.** From a scratch marketplace with a relative source into a scratch harness config, to see the skills registered; `CLAUDE_CONFIG_DIR` pointed at a scratch directory. A scratch config carries no login, so the guards are seen in a session that loads the package with `--plugin-dir`, which installs nothing: firing in a vendored scratch repository and silent in an empty one. Two more sessions follow, and the guard fires in both: one launched from a subdirectory of the vendored repository, and one whose Bash tool changes into a subdirectory before its next call.
> - **Codex.** `codex plugin marketplace add` of the repository and `codex plugin add checkwright@checkwright`, `CODEX_HOME` pointed at a scratch directory, then `codex plugin list`. A session then loads the skills, and with the hooks trusted the shell guard fires in a vendored scratch repository and stays silent in an empty one.
> - **Antigravity.** `agy plugin validate plugin` and `agy plugin install ./plugin`, `HOME` pointed at an absolute scratch directory, then `agy plugin list`, which counts the skills.

`agy` refuses a relative `HOME` ("must be an absolute path"), observed at spec.

**Inferred, cannot run before build:** a session opened on a scratch Codex home loads the skills and, once the hooks are trusted, fires the shell guard — a scratch home carries no login, and the one logged-in home is the user's own, whose write is the operator's to approve.

Where a scratch home cannot open a session, the build escalates to the lead before installing into a logged-in home. The write there and its removal (`codex plugin remove`, `codex plugin marketplace remove`) are the operator's to approve.

### (4) The honest limits name each harness's residue {mechanical}

`plugin/SPEC.md` §Honest limits. **Not yet applied.**

- The first bullet, "Only Claude Code's install is run. Every other Agent Plugins client is read off the standard.", reads: "Claude Code's, Codex's and Antigravity's installs are run; every other harness in §The harnesses is read off its own documentation."
- Three bullets are added:

> - On Codex the wakeup guard never fires, and `workflow-state-guard` allows a patch to the state file, since the patch tool carries no written path.
> - Codex records an install at version `1.0.0`, since neither manifest carries `version` (§The manifests). Whether a pin move reaches an installed copy through `codex plugin marketplace upgrade` is unrun.
> - Antigravity, Cursor and Muse Code run no guard from this package.

### (5) The usage tier and the install page name the Codex and Antigravity routes {mechanical} {user-facing: plugin-harness-reach's Deliverable (1) and (3)}

`plugin/README.md`, **not yet applied**:

- **§What it registers**, the guards bullet, reads: "**The guards**, in Claude Code and Codex: guard-kit's shell, wakeup and workflow-state guards. Each does nothing in a repository without Checkwright vendored. Codex runs them once you trust them in `/hooks`, and runs no wakeup guard."
- **§Installing it** gains, after the Claude Code fence:

  > In Codex:
  >
  > ```text
  > codex plugin marketplace add checkwright/checkwright
  > codex plugin add checkwright@checkwright
  > ```
  >
  > In Antigravity, from a clone of this repository: `agy plugin install ./plugin`. It registers the skills and no guard.

- The sentence "Any other Agent Plugins client loads this `plugin/` directory by its own install route." reads "Any other Agent Plugins client loads this `plugin/` directory by its own install route; [`plugin/SPEC.md`](SPEC.md) §The harnesses lists the ones read."

`docs/install.md` §From a plugin marketplace, **not yet applied**:

- The route list's bullet "**In Claude Code:** [the plugin marketplace](#from-a-plugin-marketplace)." reads "**In Claude Code or Codex:**" with the same link.
- The Codex fence above is added after the Claude Code fence, with its lead-in "In Codex:".
- The closing sentence "Any Agent Plugins client loads …" names Codex and Antigravity alongside it and links plugin/SPEC.md §The harnesses.

The hosted route was run at spec. Under a scratch `CODEX_HOME`, `codex plugin marketplace add checkwright/checkwright` resolved the hosted repository, and `codex plugin add checkwright@checkwright` installed from it.

## Producers and consumers

- **§The harnesses (delta 1).**
  - Producer: the build that runs a harness, which writes a **run** cell. A **read** cell becomes **run** only by a later run.
  - Consumers: an adopter choosing a harness; the README's pointer (delta 5); and the filed Deferred entries, whose observation would turn a cell from **read** to **run**.
  - Readers holding a roster: `check-plugin-parity`'s assertions name no section of this SPEC, by its §check-plugin-parity, so a new section reds nothing there.
- **The Codex wiring (delta 2).**
  - Producer: the unchanged rendering in `plugin/hooks/hooks.json`.
  - Consumer: Codex's plugin-hook discovery, which reads `hooks/hooks.json` by default.
  - Field by field (gate-sdk/SPEC.md §The hook contract's binding table), each matching the shipped binding per Codex's hooks documentation:
    - the event name `PreToolUse`;
    - the tool name `Bash`;
    - `tool_input.command`;
    - exit 2 with stderr as a block;
    - `hookSpecificOutput` with `permissionDecision` `allow`, `updatedInput` and `additionalContext`.

    Codex refuses an `ask` on that event, and no member emits one.
  - **Inferred, cannot run before build:** the rendering's shell-form command runs under Codex's hook shell, and its fallback to `.` lands on the session's tree — it needs the same logged-in session as delta 3's Codex run.
- **Point 5, narrowed corpora.** No delta narrows a corpus a gate reads. `check-plugin-parity` (C) holds the rendering unchanged, and delta 2 changes no byte of it.
- **Point 6.** Delta 1 obliges each harness the deliverable names, plus Cursor, which the entry's read-first note names. Each row's value is in the table above.

## Existing sections updated

The roster came from `git grep -n -i "only claude code\|in claude code only\|agent plugins client\|plugin marketplace add" -- plugin docs/install.md README.md installer` over the tracked tree. Its `docs/plugin/README.md` and `docs/llms-full.txt` hits are generated projections.

- `plugin/SPEC.md`: §The harnesses, new (delta 1); §The guards (delta 2); §The validation leg (delta 3); §Honest limits (delta 4).
- `plugin/README.md` §What it registers and §Installing it (delta 5).
- `docs/install.md` §From a plugin marketplace and the route list of its opening section (delta 5).
- `docs/plugin/README.md`, `docs/plugin/SPEC.md` and `docs/llms-full.txt`, the generated projections, regenerated by the commands docs/generated-projections.md names (all deltas).
- `.workflow/release-declarations.md`, `## Behavior changes`: the plugin's guards reach Codex (delta 2).

## Retired spellings

- None — no delta retires a name; the rendering, the manifests and the marketplace file are unchanged.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls plugin/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The runs** — delta 3's Codex and Antigravity runs read, each cell of §The harnesses marked **run** only where its run passed, and the two inferred markers above deleted or kept as the runs answer.
- [ ] **Suites** — `check-plugin-parity`, the plugin validation commands of §The validation leg, and the battery green.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, no later than the stage before the drain stage.
