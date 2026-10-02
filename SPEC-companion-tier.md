# SPEC amendment: companion-tier

**A third install line per spec toolkit, `complement`: every kit except the ones whose job the toolkit already does, each left-out kit recorded with the toolkit surface it defers to.** Today companion/SPEC.md §The two tiers offers `prose` (canon-kit's document gates) and `full` (every kit), so a Spec Kit or OpenSpec adopter chooses between document gates alone and Checkwright's whole workflow installed beside the toolkit's own. The direction is the operator's of 2026-09-29, lead-relayed (not a ruling), recorded on the queue entry: the tier, its exclusion record, and a recipe per toolkit in its extension where one exists and at minimum on its docs page.

**The tier is named `complement`**, operator direction 2026-10-02, confirmed in the lead session (not a ruling). **It is `full` with a recorded selection, not a fourth profile**, the lead's decision of 2026-10-02: the direction's "profile" names the outcome, and the tiers are companion/SPEC.md's to own. Its line is `init --profile full --without-kit <kit>… --recipe <toolkit>`, on installer/SPEC.md §Selecting kits and gates as it stands, so it mints no installer mechanism. `full` is derived from the payload, so a kit landing later joins the tier with no edit here. A fourth profile would need a hand roster of every kit but the excluded ones, the drift `installer/profiles.list`'s header refuses, or a subtractive row form the profile module does not have. The selection is recorded in the manifest, so a bare re-run and `update` re-apply it, and `doctor` reports it beside the profile.

**Measured at authoring**, at each toolkit's pin in `companion/toolkits.list`, by running the toolkit's own init in a scratch repository with the arguments companion/SPEC.md §Recipes records, and listing what it wrote:

- **Spec Kit 1.0.12** writes `.specify/workflows/speckit/workflow.yml`, a workflow running specify → plan → tasks → implement with a review gate between steps; skills for each step, `tasks` and `taskstoissues` among them, so each feature carries its own task list; and `.specify/memory/constitution.md`, the project principles the plan step checks against. It writes no agent file.
- **OpenSpec 1.13.2** writes commands and skills for propose, apply, archive, explore, sync and update, a change lifecycle in which each change carries its own `tasks.md`; and `openspec/config.yaml`, whose optional `context` and `rules` are the project guidance shown to the agent. It writes no agent file.
- **What each kit seeds**, read off `native/src/installer/recipe.rs`'s seed arms and the `--install queue-source` op: lifecycle-kit seeds the stage machine's state file, queue-kit is the queue template's owner, doctrine-kit writes the doctrine block into the agent file and creates that file when absent, and evidence-kit seeds two manifests under `.workflow/`. canon-kit owes a queue file, so `--install queue-source` answers `-`, the inline skeleton, both for `gate-sdk,canon-kit` (`prose`) and for every kit but lifecycle-kit and queue-kit: **`prose` already seeds a task-queue skeleton**, and §The two tiers' "seeds nothing beside the toolkit's own files" is false. Delta 2 corrects it.
- **The excluded set, by the direction's test: a kit leaves where the toolkit, at its pin, ships the surface the kit's subject governs.** lifecycle-kit's subject is the stage machine and queue-kit's the task tracker, and each toolkit ships its own of both. doctrine-kit's subject is the guidance an agent works under, and each toolkit ships its own: Spec Kit's constitution, OpenSpec's `config.yaml` context and rules. So all three leave, for both toolkits. Two supporting grounds, neither the ground: doctrine-kit's digest carries the rules that route work through the scope stage and the queue's Deferred section (doctrine-kit/DOCTRINE.md, Gap disposition and Scope-gated intake), which this tier leaves out; and its one gate, `check-doctrine-registration`, checks only its own block's registration, so leaving it out loses no gate over the adopter's work. evidence-kit's subject, a held-constant test baseline and a per-run evidence manifest, is neither toolkit's, so it stays. So do guard-kit, delegation-kit and context-kit, whose subjects are the agent session's permissions, dispatch and context budget, and drift-kit and site-kit.
- **The tier owes `bash` 4.3 or later**, since it carries context-kit, drift-kit and guard-kit (`native/src/installer/doctor.rs`: the floor is owed where any of the three is selected).
- **OpenSpec's lifecycle layer cannot ride the tier.** `init` checks a recipe's seams against the selected kit set (`native/src/installer/init.rs`, the `check_seams` call after `selection::kit_set`), and the layer writes `lifecycle-config.knobs`, which no selected kit writes, so `init` refuses it at exit 2. The only kit a `--without-kit` may not name is gate-sdk (`native/src/installer/selection.rs`).

## What changes

### (1) The exclusion record: `companion/exclusions.list`

A new data file beside `toolkits.list` and `native.list`, one `<toolkit> <kit> <surface…>` line per left-out kit {design-bearing} {user-facing: the operator's direction of 2026-09-29 on the entry, "each exclusion recorded with the toolkit it defers to"}. The toolkit field keys the line to a `toolkits.list` line, the kit field names a payload kit root, and the surface is the toolkit's own artifact that does the kit's job, in words an adopter reads. `#` lines and blank lines are ignored. It opens with a `# contract: companion/SPEC.md §The component —` header in the form its two siblings carry.

The rows, as measured above:

```
speckit lifecycle-kit the specify, plan, tasks and implement workflow, .specify/workflows/speckit/workflow.yml
speckit queue-kit each feature's task list, specs/<feature>/tasks.md
speckit doctrine-kit the project constitution, .specify/memory/constitution.md
openspec lifecycle-kit the propose, apply and archive change workflow
openspec queue-kit each change's task list, openspec/changes/<change>/tasks.md
openspec doctrine-kit the project context and rules in openspec/config.yaml
```

The doctrine-kit rows are the lead's decision of 2026-10-02 on this session's escalation, applying the direction's test to the measurement above. The record lives here, in the companion's data, because a `--without-kit` selection carries no reason and the manifest records only the selection.

**Not yet applied.**

### (2) companion/SPEC.md states the third tier

The sections below are rewritten {design-bearing} {user-facing: the operator's direction of 2026-09-29 on the entry; the name `complement`, operator direction 2026-10-02, lead-relayed; the selection spelling, the lead's decision 2026-10-02}. **Not yet applied.**

**The preamble's** second sentence, *It holds, for each supported spec-authoring toolkit, a **recipe** that fits the `prose` and `full` profiles to that toolkit's tree*, becomes:

> It holds, for each supported spec-authoring toolkit, a **recipe** that fits each install tier (§The tiers) to that toolkit's tree, and a fixture tree that proves the recipe.

**§The component** gains, after the `native.list` bullet:

> - `exclusions.list` records the kits each toolkit's `complement` line leaves out, one `<toolkit> <kit> <surface…>` line each: the kit, and the toolkit's own surface that already does its job. The companion arm holds each toolkit's `complement` line to its rows (§The tiers).

**§Recipes**, the rule's second sentence, *Install `prose` or `full` on a fresh tree of the toolkit at its pin*, becomes *Install any tier on a fresh tree of the toolkit at its pin*.

**§Applying a recipe**, from *For Spec Kit both lines are in* to the paragraph's end, becomes:

> For Spec Kit every line is in `speckit/commands/install.md`, each an `sh` fence holding the one-line install the agent runs, and the Spec Kit page's `complement` and `full` lines carry the same words from `init`. Each toolkit's `complement` and `full` lines take a marker pair each (§The tiers). The companion arm runs each line's words from `init` to the line's end, so the documented steps are the tested steps.

**§The two tiers** is retitled **§The tiers** and its first two paragraphs are replaced; the overlap paragraph after them stays as it is:

> Each toolkit has three install lines, each a profile with the toolkit's recipe applied, and each contains the one before it, so moving up only adds (installer/SPEC.md §Profiles). An adopter reaches the next tier by running `init` with its line, or asks the install command for it.
>
> - **`prose`**, the default, puts canon-kit's document gates over the toolkit's tree and owes no `bash`. Of Checkwright's workflow surfaces it seeds only a task-queue skeleton, the file canon-kit's task-liveness gates resolve a marker against (installer/SPEC.md §What init seeds).
> - **`complement`** installs every kit except those whose job the toolkit already does, so every gate the toolkit does not already do runs over the tree. It is `full` with a `--without-kit` per left-out kit (installer/SPEC.md §Selecting kits and gates), so a kit the payload gains joins it. A kit is left out when the toolkit, at its pin, ships the surface the kit's subject governs, and `exclusions.list` records each with that surface; the line's `--without-kit` set is exactly the toolkit's rows. It needs `bash` 4.3 or later, since it carries kits that ship bash files (docs/install.md §Requirements), and seeds evidence-kit's files under `.workflow/` beside the queue skeleton.
> - **`full`** installs every kit. It also needs `bash` 4.3 or later, and seeds Checkwright's own workflow beside the toolkit's: queue-kit's task queue, the stage machine's state file, a doctrine block in the agent file, and evidence files under `.workflow/`.
>
> **A left-out kit is the path to replacing that piece of the toolkit's workflow.** Run the `complement` line again without that kit's `--without-kit`: a run passing any selection flag replaces the recorded selection whole. Each toolkit's page carries its `complement` line between `<!-- companion-complement:begin -->` and `<!-- companion-complement:end -->` and its `full` line between `<!-- companion-full:begin -->` and `<!-- companion-full:end -->`, in the same `text` form as the default line. On OpenSpec the `full` line also applies the lifecycle layer, which no gate `init` registers reads until a stage session runs (§The lifecycle layer). The Spec Kit extension's install command installs any of the three, by the user's input, its `complement` and `full` lines each in its own block.

**Every other citation of §The two tiers** in companion/SPEC.md becomes §The tiers: the one in §The tested claim's lifecycle-leg paragraph, the one in §The fixtures' overlap paragraph, and the one in §The support table's gate-row bullet, beside the sentence delta 2 rewrites there.

**§The lifecycle layer**'s first sentence cites §The tiers, and gains a second:

> The `complement` line leaves lifecycle-kit out, so it carries no layer, and `init` refuses the layer beside it, since the layer writes a seam no selected kit writes (installer/SPEC.md §Payload recipes).

**§The tested claim**'s first sentence becomes:

> The recipes are proved against the defect classes below, each caught by a named gate, on `prose` and `complement` for both toolkits and on `full` for Spec Kit, whose `complement` and `full` legs plant them again.

and the paragraph *The OpenSpec `full` line is held green by the lifecycle leg* gains, before its second sentence:

> Each `complement` leg also asserts the left-out kits absent: the manifest's `kits` holds none of the toolkit's `exclusions.list` kits, and its recorded selection removes exactly them, after the install and after a bare re-run and a bare `update`.

**§The Spec Kit extension**, the `speckit.checkwright.install` bullet's second sentence becomes:

> It runs the one-line install with `CHECKWRIGHT_VERSION` set to `extension.version` and the arguments `init --profile prose --recipe speckit`, or its `complement` or `full` line's when the user's input asks for that tier, naming the tier's costs before running it, then runs the commands `init` printed.

and *so one command offers both tiers* becomes *so one command offers every tier*.

**§The support table**, *since the default line arms and tests it and `full` only adds*, becomes *since the default line arms and tests it and each tier above it only adds*.

**§Honest limits**: *the companion arm runs both of the install command's lines* becomes *the companion arm runs each of the install command's `sh` lines*, and the list gains:

> - The exclusion record is measured at each toolkit's pin, from what its init writes and the commands it ships. A toolkit release that adds or drops a workflow surface is seen only when a pin move measures it again.

### (3) The Spec Kit extension offers the tier

`companion/speckit/commands/install.md`, `companion/speckit/extension.yml` and `companion/speckit/README.md` {mechanical} {user-facing: the operator's direction of 2026-09-29 on the entry, "in the toolkit's extension where one exists"; the name `complement`, operator direction 2026-10-02, lead-relayed}. **Not yet applied.** Instruction text only; the grounds are delta 2's.

`commands/install.md`:

- the front matter's `description` reads `"Install Checkwright's gates, prose, every kit Spec Kit does not replace, or every kit, and apply the Spec Kit recipe"`, and `argument-hint` reads `"prose (the default), complement or full"`;
- **Choose the profile.** becomes **Choose the tier.**, reading: *If the user input asks for `full`, install `full`; if it asks for `complement`, install `complement`; otherwise install `prose`. Before a `complement` install, tell the user what it adds and costs: every kit's gates except the kits Spec Kit's own workflow replaces, which its line names, `bash` 4.3 or later, which stock macOS lacks, and evidence files under `.workflow/`.* The `full` sentence and the refusal sentence follow unchanged;
- *the line for the profile and the host* becomes *the line for the tier and the host*;
- after the `prose` block, a `complement` block in the `full` block's shape:

  ```
  For `complement` on macOS and Linux:

  <!-- companion-complement:begin -->

  ```sh
  curl -fsSL https://checkwright.dev/install.sh | CHECKWRIGHT_VERSION=X.Y.Z sh -s -- init --profile full --without-kit lifecycle-kit --without-kit queue-kit --without-kit doctrine-kit --recipe speckit
  ```

  <!-- companion-complement:end -->
  ```

- after the PowerShell `prose` line, *For `complement`:* and its PowerShell line with the same arguments.

`extension.yml`: the install command's `description` reads `"Install Checkwright's gates (prose, complement or full) and apply the Spec Kit recipe"`.

`README.md`: the `speckit.checkwright.install` bullet names the three tiers, *`prose` by default, `complement` or `full` when you ask*, and gains one sentence: *`complement` adds every kit's gates except the kits Spec Kit's own workflow replaces, and needs `bash` 4.3 or later.*

### (4) The pages carry the lines

`docs/speckit.md`, `docs/openspec.md`, `docs/spec-toolkits.md`, `companion/README.md` and `docs/site-architecture.md` {mechanical} {user-facing: the operator's direction of 2026-09-29 on the entry, "at minimum on its docs page"; the name `complement`, operator direction 2026-10-02, lead-relayed}. **Not yet applied.**

- **`docs/speckit.md`.** The install paragraph's *It installs the `prose` profile, or `full` when you ask it for `full`* becomes *It installs `prose`, or `complement` or `full` when you ask for one*. Before the `full` paragraph, a paragraph **Every kit Spec Kit does not replace.** *Ask the install command for `complement`, or run your system's line from the install page with:* and a `companion-complement` block holding one `text` fence, `checkwright init --profile full --without-kit lifecycle-kit --without-kit queue-kit --without-kit doctrine-kit --recipe speckit`. Then: *`complement` leaves out each kit whose job Spec Kit already does, one `--without-kit` each, and [`companion/exclusions.list`](https://github.com/checkwright/checkwright/blob/master/companion/exclusions.list) records the Spec Kit surface each defers to. It needs `bash` 4.3 or later and seeds evidence files under `.workflow/`. To adopt a left-out kit later, run the line again without its `--without-kit`.* The `full` paragraph's lead becomes **Every kit.**
- **`docs/openspec.md`.** Before the `full` paragraph, a paragraph **Every kit OpenSpec does not replace.** *The second line installs `complement`:* with a `companion-complement` block, `checkwright init --profile full --without-kit lifecycle-kit --without-kit queue-kit --without-kit doctrine-kit --recipe openspec`, and the sentence above with OpenSpec in Spec Kit's place, plus: *It applies no lifecycle layer, since it leaves the stage machine to OpenSpec.* The `full` paragraph's lead becomes **Every kit.** and *The second line* becomes *The third line*.
- **`docs/spec-toolkits.md`.** *Each toolkit has two lines: `prose`, the default, and `full`, every kit.* becomes *Each toolkit has three lines: `prose`, the default; `complement`, every kit whose job the toolkit does not already do; and `full`, every kit.* The legend's *and `full` keeps it* becomes *and `complement` and `full` keep it*, and its `#the-two-tiers` anchor becomes `#the-tiers`. §What is tested gains, after the `full` sentence: *On each toolkit it also installs `complement` with the recipe, asserts the battery green and each defect caught, and asserts the left-out kits absent.* §Limits' *the tests run both install lines the install command contains* becomes *the tests run each install line the install command gives for macOS and Linux*.
- **`companion/README.md`.** The preamble's *fits the `prose` and `full` profiles* becomes *fits each install tier*. §What it holds gains, after the tested-versions bullet: *- **The left-out kits**, in `exclusions.list`: each kit a toolkit's `complement` line leaves out, with the toolkit's own surface that does its job.* The *Each toolkit also has a `full` line* paragraph becomes: *Each toolkit also has a `complement` line, which installs every kit but those whose job the toolkit already does, and a `full` line, which installs every kit and on OpenSpec applies `openspec/lifecycle/` too ([SPEC.md §The tiers](SPEC.md#the-tiers)).*
- **`docs/site-architecture.md`**, §Generated projections' *toolkit install lines* bullet: *carry three marker blocks* becomes *carry the marker blocks below*, and the bullet names the `companion-complement` pair on both pages and in `companion/speckit/commands/install.md`, the extension's an `sh` fence like its `companion-install` block.

### (5) The consumer smoke's companion arm runs the tier

`installer/consumer-smoke/run-smoke.sh`, installer/SPEC.md §The consumer smoke, and `.workflow/validate-baseline.txt` {design-bearing}. **Not yet applied.**

**The arm reads `companion/exclusions.list`** from the repository, as it reads `companion/*/recipe/`, and refuses before any leg, by name:

- a row whose toolkit has no `toolkits.list` line, or which carries no surface;
- a `<toolkit> <kit>` pair twice;
- a kit the packed payload carries no `payload/<kit>/` for;
- a toolkit with a `companion/<toolkit>/recipe/` directory and no row, since every toolkit has a `complement` line and a line leaving out nothing is `full`.

**One `complement` leg per toolkit**, each under its own literal header, `companion arm for speckit complement (…)` and `companion arm for openspec complement (…)`:

1. It reads the toolkit's `companion-complement` block, from `companion/speckit/commands/install.md` for Spec Kit and `docs/openspec.md` for OpenSpec, refusing an absent or empty block or a line carrying no `--profile full`. For Spec Kit, `docs/speckit.md`'s `companion-complement` line must carry the command's words from `init`, and the arm fails naming both when it does not, as the `full` leg does.
2. It asserts that the line's `--without-kit` values, as a set, equal the toolkit's `exclusions.list` kits, naming the kits on either side only.
3. It runs `companion_arm` on the line: one commit, the recorded recipes, the battery green, a bare re-run unchanged, and each planted defect red by its gate.
4. It asserts that the manifest's `kits` carries none of the left-out kits and that `selection.without-kits` equals them as a set.
5. It asserts the exclusion persists: after step 3's bare re-run, and again after a bare `update`, the tree object is unchanged and step 4's two assertions still hold, since both verbs re-apply the recorded selection (installer/SPEC.md §Selecting kits and gates, §update).

**The installer section.** §The consumer smoke's companion-arm paragraph gains, after the sentence on the `companion-full` block:

> It reads each toolkit's `companion-complement` block the same way, from the same file as its `companion-full` block, and `companion/exclusions.list`, refusing a row naming an unknown toolkit or carrying no surface, a repeated pair, a kit the payload lacks, and a toolkit with a recipe and no row. The Spec Kit page's `companion-complement` line must carry the command's words from `init`.

and, after the Spec Kit `full` leg's sentence:

> Each toolkit then runs a `complement` leg in another consumer: `init` with its `companion-complement` line's arguments, the line's `--without-kit` set asserted equal to the toolkit's `exclusions.list` kits, steps 2 to 4 above, and the manifest's `kits` and `selection.without-kits` asserted to hold the left-out kits out, after the install and again after a bare re-run and a bare `update`.

The script's header comment and the companion arm's `# spec:` lines name the new legs. The `# spec:` line citing §The two tiers in `run-smoke.sh` cites §The tiers.

**The baseline.** `.workflow/validate-baseline.txt` gains `installer_smoke companion-arm-for-speckit-complement pass` and `installer_smoke companion-arm-for-openspec-complement pass`, beside the arm's four rows.

**Inferred, cannot run before build:** that each `complement` install is green on its fixture tree and reds each planted defect — no `complement` line exists until deltas 3 and 4 land it. `full` with the same recipe is green on the Spec Kit fixture and `complement`'s registry is a subset of `full`'s, but a gate reading the declared kit-root set could still move, so the leg is the oracle.

### (6) The projections and citations follow

{mechanical} Build regenerates every generated projection the battery reds on, each with the command its freshness gate prints: the site mirrors `docs/companion/SPEC.md`, `docs/companion/README.md` and `docs/installer/SPEC.md`, the graph artifact and `ROADMAP.md` at the entry's Done move. The deferred entry `openspec-delta-base-agreement` cites `companion/SPEC.md §The tiers` in place of §The two tiers, in the commit retitling the section.

## Producers and consumers

- **`companion/exclusions.list`.** Producer: a hand edit, measured at a toolkit's pin (delta 2's honest limit). Consumers: the companion arm, by a read of the tracked file at smoke time (delta 5); and the adopter, through the toolkit pages' link (delta 4). Fields: the toolkit is read by the arm to key the rows and to check `toolkits.list`; the kit is read by the arm against the line, the payload and the manifest; the surface is read by the arm for presence and by the adopter for its words. Roster-holding readers of `companion/`: `git grep -l "native\.list"` finds the support-table arm and `check-support-table-fresh`'s `couples=`, which name `toolkits.list`, `native.list` and the fixture directories one by one, so a new sibling is no finding for either and joins neither, since the table does not read it.
- **The `companion-complement` marker pair.** Producer: the hand-authored blocks of deltas 3 and 4. Consumer: the companion arm, which refuses an absent or empty block (delta 5). Roster-holding reader: `docs/site-architecture.md`'s install-lines bullet (delta 4).
- **The tier itself.** Producer: `init` with the line's arguments, through the existing selection and recipe paths; enabling configuration is the line, which every route to `init` runs. Consumers: the manifest's `selection` and `kits`, read by a bare re-run, `update` and `doctor` (installer/SPEC.md §The manifest, unchanged), and the companion arm's step 4.
- **Red conditions.** The arm's new refusals red on *finding none* in one place, a toolkit with a recipe and no row, which is the intended coverage floor. The lattice assertions are untouched: they run over selection-free installs (installer/SPEC.md §Selecting kits and gates).
- **The extension's `complement` path.** Producer: the agent following `commands/install.md` on the user's input. It is untested beyond its `sh` line, as the two existing paths are (companion/SPEC.md §Honest limits).
- **Every member's satisfying value.** The corpus is the two toolkits in `companion/toolkits.list`: Spec Kit's value is its `exclusions.list` rows and the line in `companion/speckit/commands/install.md`, held equal to `docs/speckit.md`'s; OpenSpec's is its rows and the line in `docs/openspec.md`.

## Existing sections updated

Roster produced by `git grep -n -i "the two tiers\|the-two-tiers\|companion-full\|companion-install"` over the tracked tree, the reads of companion/SPEC.md, installer/SPEC.md §The consumer smoke, the toolkit pages and the extension, and the measurements above.

- `companion/exclusions.list` (delta 1).
- `companion/SPEC.md`: the preamble, §The component, §Recipes, §Applying a recipe, §The two tiers retitled §The tiers, §The lifecycle layer, §The tested claim, §The fixtures, §The Spec Kit extension, §The support table and §Honest limits (delta 2).
- `companion/speckit/commands/install.md` (delta 3).
- `companion/speckit/extension.yml` (delta 3).
- `companion/speckit/README.md` (delta 3).
- `docs/speckit.md` (delta 4).
- `docs/openspec.md` (delta 4).
- `docs/spec-toolkits.md` (delta 4).
- `companion/README.md` (delta 4).
- `docs/site-architecture.md`, the toolkit install-lines bullet (delta 4).
- `installer/consumer-smoke/run-smoke.sh` (delta 5).
- `installer/SPEC.md` §The consumer smoke (delta 5).
- `.workflow/validate-baseline.txt` (delta 5).
- `docs/companion/SPEC.md` (delta 6).
- `docs/companion/README.md` (delta 6).
- `docs/installer/SPEC.md` (delta 6).
- `ROADMAP.md` and the graph artifact (delta 6).
- `TASK-QUEUE.md`: `openspec-delta-base-agreement`'s citation of the retitled section (delta 6).

## Retired spellings

- `The two tiers` — companion/SPEC.md's section heading, retitled §The tiers since the tiers number three (delta 2).
- `the-two-tiers` — that heading's anchor, cited by `companion/README.md` and `docs/spec-toolkits.md` (deltas 2 and 4).

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Remote oracle read** — delta 3 changes the packed extension, so the entry's push-need condition fires: after the landing commit, one push under that line, and the `companion-toolkits` run read green. The entry's Done move lands after that read and before the drain stage.
