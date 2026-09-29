# SPEC amendment: adoption-prompts

No shipped template drives a coding agent through adoption. The plugin's `install` skill chooses a profile and runs `init`. What follows is left to the adopter's agent reading pages: fitting the knobs to the adopter's layout, and deciding what each first red means.

This amendment adds that half as one harness-neutral kit template, `gate-sdk/templates/adopt.md`, and ships it twice. The plugin carries it as an `adopt` skill, rendered from a binding shim like every other plugin skill, which the `install` skill hands to once `init` is done. Any other coding agent is handed the vendored file itself, and the install page gives the prompt that does so.

One queue entry pairs it: [adoption-prompt-templates](TASK-QUEUE.md#adoption-prompt-templates).

**The rulings.**

- **A sibling, not an extension of `install`.** The queue entry left this open. `install` runs before anything is vendored, so it cannot run a vendored template; it carries its own steps and no template directive (plugin/SPEC.md §The skills). Knob configuration and red triage run after `init`, over files `init` wrote, so they are a vendored template's job. Two skills keep each half on the side of `init` where its inputs exist. `install`'s last step hands to `adopt`.
- **gate-sdk owns the template.** Every profile carries gate-sdk, `starter` included (`installer/profiles.list`), and the template's subject is gate-sdk's: the knob seam, the gate output contract, the registry. A template in any other kit would be missing from some install.
- **The plain route is the vendored file.** After `init`, the template sits at `gate-sdk/templates/adopt.md` in the adopter's tree, so any agent can be told to follow it. Before `init`, the install page is the procedure. The page's prompt names the install steps by linking the page's own sections, and names the template by path, so it restates neither.
- **It files no install evidence.** drift-kit's install-observation record cannot yet tell an author's install from anyone else's, so an agent filing reds from the author's own adoptions would pass them off as external evidence. The template does not file.
- **The seam.** The template is kit mechanism and names no toolkit, harness, publisher location or project. The shim, the plugin skill, the install skill and the install page are repo-root. The template names the binary's own arms and knobs, which are kit content.

## What changes

### (1) The template {design-bearing}

**Not yet applied.** `gate-sdk/templates/adopt.md`. It is a boundary template, like lifecycle-kit's `upgrade.md`: it stamps no state and runs in any session after `init`. It has no slots, so the plugin's fallback, each slot's own text, has nothing to bind. Its opening states its exit condition: *the full battery is green, or every red carries a disposition the user accepted.* Its steps, each an instruction with no grounds:

1. **Confirm the install.** `checkwright.lock` exists at the repository root and the worktree is clean. If the lock is missing, Checkwright is not installed here, so say so and stop. Read the lock for the profile, the kits and any selection.
2. **Run the battery** on the gate binary `GATE_SDK_NATIVE_BIN` names, with `--run`, and keep its output. Each red is a triage item. Each clean line that counts zero files, pages or entries names a gate that reads nothing on this tree, and is a configuration item.
3. **Fit the knobs to the layout.** For each configuration item and each red whose finding names a path the gate should not read, find the knob that sets its corpus or its exemptions. `--emit knob-roster` lists every knob with its default, and `--emit knob-values <NAME>` gives its resolved value here. Survey the tree for where its specs, docs, sources and generated or third-party directories are. Tell the user each change and the paths that ground it, then write it into the kit's knob file in the gates directory. Never edit a vendored kit file.
4. **Triage each red that remains.** Read the finding, its `help:` line and the section its `spec:` pointer names. Then give it one disposition, in this order of preference:
   - a defect in the tree: fix the tree;
   - a legitimate exception the gate provides a valve for: apply the valve, with its reason;
   - a gate of no value to this repository: remove it with `init --without-gate <gate>`, which the lock records, or replace it with a gate of the same name in the gates directory;
   - a red the gate should not raise here: dispose of it as above, and report the finding to the kit's publisher, where its README says.

   Never bypass a hook, and never edit a vendored gate. Tell the user each disposition before applying it.
5. **Commit** the knob changes and dispositions in one commit naming them, after the battery is green or each remaining red's disposition is accepted.

gate-sdk/SPEC.md gains `### templates/adopt.md` after §templates/gates-workflow.yml. It states that the template is the post-install adoption walk, which surfaces it reads and writes (the lock, the battery, the knob roster and values arms, the knob files in the gates directory, `init --without-gate`), and that it files no install evidence. gate-sdk/README.md's template list gains `templates/adopt.md` — *the walk an agent follows after `init`: knobs fitted to your layout, first reds triaged*.

### (2) The binding shim {mechanical}

**Not yet applied.** `.claude/commands/adopt.md`, in the shim form: its one line is `Execute the template at gate-sdk/templates/adopt.md, applying the bindings below.`, and a `## Bindings` heading follows with no binding under it. `lifecycle-kit/checks/check-skill-binding.gate`'s `couples=` gains `gate-sdk/templates/adopt.md` beside `delegation-kit/templates/agent-execution.md`, since the gate couples each out-of-tree bound template by name.

### (3) The plugin skill {mechanical}

**Not yet applied.** `plugin/skills/adopt/SKILL.md`: front matter `name: adopt` and a `description` of the form plugin/SPEC.md §The skills sets, *Fits Checkwright's knobs to this repository's layout and triages the first reds after an install, with the user. Run it only when the user asks for it.* Its body is §The skills' fixed rendering for `gate-sdk/templates/adopt.md`, which `check-plugin-parity` assertion B derives from the shim and holds byte for byte.

### (4) `install` hands to `adopt` {mechanical}

**Not yet applied.** `plugin/skills/install/SKILL.md` gains step 5: *Then run the `adopt` skill, which fits the knobs to this repository and triages the first reds.* plugin/SPEC.md §The skills: *`install` tells the agent four things* becomes *five*, and the list gains *hand to the `adopt` skill*. plugin/README.md's line on asking for the `install` skill gains *, then for the `adopt` skill*.

### (5) The install page gives the plain prompt {mechanical}

**Not yet applied.** docs/install.md §From a plugin marketplace: *Ask for its `install` skill …* gains *, then its `adopt` skill, which fits the knobs to your layout and triages the first reds.* §Install gains `### With another coding agent`, after §From a plugin marketplace:

*Give your agent this prompt:*

```text
Install Checkwright in this repository. Choose a profile with me from https://checkwright.dev/install.html#choosing-a-profile, then install it as https://checkwright.dev/install.html#install describes for this host, and run each command init prints. Then follow gate-sdk/templates/adopt.md.
```

*The last step is the same walk the plugin's `adopt` skill runs.*

### (6) The release declaration {mechanical}

**Not yet applied.** `.workflow/release-declarations.md`, drained to its header, gains a `## Behavior changes` heading and a bullet whose lead token is the bolded template path, in the grammar installer/SPEC.md §The upgrade contract gives: *- **gate-sdk/templates/adopt.md** — a walk an agent follows after `init` to fit the knobs to your layout and triage the first reds. Nothing to do.*

## Producers and consumers

Probe: `ls .claude/commands plugin/skills`; `grep -n "templates/" lifecycle-kit/checks/check-skill-binding.gate lifecycle-kit/checks/check-shim-restatement.gate context-kit/checks/check-footprint-fresh.gate context-kit/checks/check-surface-ratchet.gate`; `grep -v '^#' installer/profiles.list`; plugin/SPEC.md §The skills and §check-plugin-parity read whole; gate-sdk/SPEC.md §The release declaration surface (the producer and Behavior-changes rules) read. The `git grep -n "kit:templates/\*"` over the `.gate` descriptors names the kit-template readers: `check-footprint-fresh`, `check-surface-ratchet`, `check-shim-restatement`, `check-door-binding` and `check-release-change-declared`.

- **The template** (delta 1). Producer: gate-sdk. Consumers: the `adopt` skill's rendering, the adopter's agent through the plain prompt, and this repository's shim. Roster readers: `check-skill-binding`, which resolves the directive and reds a slot mismatch; `check-shim-restatement`, which reds a shim sharing an n-gram with any kit template; `check-footprint-fresh`, whose measured set holds `templates/` markdown, so `docs/footprint.md` and `docs/value.md`'s rollup grow; `check-surface-ratchet`, which reds a kit template past its ceiling row. `check-door-binding` reads every kit's `templates/`: its assertion A reds a front-end stub named as a command to run, and B a path to the gate binary. So the template names the binary as the one `GATE_SDK_NATIVE_BIN` names, the voice lifecycle-kit's templates use, and spells no path to it.
- **The shim** (delta 2). Producer: this repository. Consumers: `check-skill-binding`; `check-plugin-parity` assertion B, which derives the plugin roster from the shims, so the `adopt` skill directory is required from the shim's commit.
- **The plugin skill** (delta 3). Consumers: the harness; the plugin validation leg's `skills-ref validate` and `claude plugin validate`, run on the mid-iteration push.
- **Step 5 and the prompt** (deltas 4 and 5). Consumer: the agent. `install` stays a front-door page, and its new line advertises no verb. The prompt is a `text` fence whose line opens with `Install`, so `check-front-door-verbs` reads no route in it.
- **The declaration** (delta 6). Consumer: the close that composes the release note.

The two adoptions the operator runs first are this template's first run, and the clean-container rehearsal can drive the same walk unattended. Neither is a delta.

## Existing sections updated

Roster probe: `git grep -n "install\` skill\|skills/install\|templates/check-skeleton.sh\|templates/gates-workflow.yml"` over `plugin/`, `docs/install.md`, `gate-sdk/README.md` and `gate-sdk/SPEC.md`.

- `gate-sdk/templates/adopt.md`; gate-sdk/SPEC.md — the new §templates/adopt.md; `gate-sdk/README.md` (delta 1).
- `.claude/commands/adopt.md`, `lifecycle-kit/checks/check-skill-binding.gate` (delta 2).
- `plugin/skills/adopt/SKILL.md` (delta 3).
- `plugin/skills/install/SKILL.md`, plugin/SPEC.md — §The skills, `plugin/README.md` (delta 4).
- `docs/install.md` — §From a plugin marketplace and the new §With another coding agent (delta 5).
- `.workflow/release-declarations.md` (delta 6).
- `docs/footprint.md` and `docs/value.md`'s rollup block, regenerated with `bash gate-sdk/bin/run-gates.sh --emit footprint > docs/footprint.md` and `bash gate-sdk/bin/run-gates.sh --emit value-rollup --write` (delta 1).
- The generated mirrors `docs/gate-sdk/SPEC.md`, `docs/gate-sdk/README.md`, `docs/plugin/SPEC.md` and `docs/plugin/README.md`, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (deltas 1 and 4).
- `docs/check-graph.html` and the generated hooks, since a descriptor's couples move: `bash gate-sdk/bin/run-gates.sh --emit git-hooks --write`, then `bash gate-sdk/bin/run-gates.sh --emit graph > docs/check-graph.html` (delta 2).
- `.workflow/surface-ceiling.txt` — the grown rows re-stamped with `bash gate-sdk/bin/run-gates.sh --emit always-loaded --ceiling` in the growing commit (deltas 1 and 5).

## Retired spellings

- None — the deltas add a template, a shim, a skill, a step, a page section and a declaration; no name, path or token is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — the template, the shim, the skill and the prompt carry no grounds; the deltas place them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` above holds, checked by `check-amendment-retired-spelling`.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Walked** — the template followed once by an agent over a scratch `prose` install of a small repository, its knob and triage steps each reached, before the landing commit.
- [ ] **Battery green** — the full battery, the root fixture suite and `cargo test` green on the landing commit, and the plugin validation leg green on the mid-iteration push. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`), once that push's runs are green.
