# SPEC amendment: extension-full

The Spec Kit extension installs the `prose` profile only (companion/SPEC.md §The two tiers: *The Spec Kit extension installs the default only*). Its install command runs `init --profile prose --recipe speckit`, so a Spec Kit user who arrives through the catalog meets the document gates, and reaches `full` only by leaving the extension for the page's second line. This amendment lets the install command install either tier. The user's input selects it, `prose` stays the default, and the command names `full`'s two costs before running it. The companion arm proves the command's `full` line.

One queue entry pairs it: [speckit-extension-full-profile](TASK-QUEUE.md#speckit-extension-full-profile).

**The rulings.**

- **A choice between the two, not `full` alone.** `full` costs `bash` 4.3 or later and seeds Checkwright's own workflow surfaces beside Spec Kit's (§The two tiers). The page states those costs to a reader. The extension's user meets them only through the agent, so the command names them before the run and keeps `prose` as the default.
- **The argument is the extension format's own.** A command body receives the user's free text as `$ARGUMENTS`, and a command's `argument-hint` front matter reaches the rendered skill (survey record, the toolkit-format block). So the command reads its input for `full` and needs no second command.
- **The line the command runs is the line the arm runs.** The Spec Kit `full` leg runs the command's own `full` line, as the default leg runs its `prose` line. The page's `full` line is held to carry the same words, so the page and the command cannot name two installs.
- **The seam.** Repo-root only: the companion package, its design record, the smoke and the docs. No kit surface changes.

**Sequencing.** This lands after [companion-spec-to-code-gates](TASK-QUEUE.md#companion-spec-to-code-gates). Both edit `companion/speckit/extension.yml` (that unit the extension's own `description`, this one the `speckit.checkwright.install` command's), `companion/speckit/README.md` (its gate sentence and its install bullet), `docs/speckit.md` and the companion arm. Where that unit has landed in an earlier batch or lands in this one, the text below is written over its edits. Otherwise, each passage it would have changed is left as it stands, and this amendment's edits are applied beside it.

## What changes

### (1) The install command offers the tier {design-bearing}

**Not yet applied.** `companion/speckit/commands/install.md`:

- The front matter gains `argument-hint: "prose (the default) or full"`, and its description becomes *Install Checkwright's gates, prose or every kit, and apply the Spec Kit recipe*.
- A step opens the `## Steps` list, ahead of the install step:

  > **Choose the profile.** If the user input asks for `full`, install `full`; otherwise install `prose`. Before a `full` install, tell the user what it adds and costs: every kit's gates, `bash` 4.3 or later, which stock macOS lacks, and Checkwright's own task queue, a doctrine block in the agent file and evidence files under `.workflow/`, beside Spec Kit's own. If `init` refuses on a missing tool, report its `help:` line and stop.

- The install step's lines are given per profile. The `prose` line stays between the `companion-install` markers. A `full` line, `curl -fsSL https://checkwright.dev/install.sh | CHECKWRIGHT_VERSION=X.Y.Z sh -s -- init --profile full --recipe speckit`, is added between `<!-- companion-full:begin -->` and `<!-- companion-full:end -->` in its own `sh` fence. The PowerShell line is given for each profile, with `--profile full` in the second.

`companion/speckit/extension.yml`'s `speckit.checkwright.install` description takes the front matter's new text.

### (2) The design record {mechanical}

**Not yet applied.** companion/SPEC.md:

- §The two tiers: *The Spec Kit extension installs the default only.* becomes *The Spec Kit extension's install command installs either, `full` when the user's input asks for it, and its `full` line sits in its own `companion-full` block.*
- §Applying a recipe: *For Spec Kit it is in `speckit/commands/install.md`, an `sh` fence holding the one-line install the agent runs.* becomes *For Spec Kit both lines are in `speckit/commands/install.md`, each an `sh` fence holding the one-line install the agent runs, and the Spec Kit page's `full` line carries the same words from `init`.*
- §The Spec Kit extension, the `speckit.checkwright.install` bullet: *and the arguments `init --profile prose --recipe speckit`* becomes *and the arguments `init --profile prose --recipe speckit`, or `--profile full` when the user's input asks for `full`, naming `full`'s costs before running it, then runs the commands `init` printed.*
- §Honest limits' agent-run bullet: *the companion arm runs the install command's install line* becomes *the companion arm runs both of the install command's lines*.

### (3) The companion arm runs the command's `full` line {design-bearing}

**Not yet applied.** `installer/consumer-smoke/run-smoke.sh`'s Spec Kit `full` leg reads its line with `companion_line` from `companion/speckit/commands/install.md`'s `companion-full` block, not `docs/speckit.md`'s. Its header becomes `companion arm for speckit full (the extension install command's full line on its fixture tree)`. Before it runs, the leg reads `docs/speckit.md`'s `companion-full` line the same way, and fails when the page's words from `init` differ from the command's, naming both lines.

installer/SPEC.md §The consumer smoke, the companion paragraph: *It reads each toolkit's `companion-full` block, `docs/speckit.md`'s and `docs/openspec.md`'s* becomes *It reads each toolkit's `companion-full` block, the extension's install command's for Spec Kit and `docs/openspec.md`'s for OpenSpec*. It gains: *The Spec Kit page's `companion-full` line must carry the command's words from `init`, and the arm fails naming both when it does not.*

### (4) The pages {mechanical}

**Not yet applied.**

- `docs/speckit.md` line 15: *It installs the `prose` profile at the release the extension was tested with* becomes *It installs the `prose` profile, or `full` when you ask it for `full`, at the release the extension was tested with*. Line 21: *After the extension's install, or in place of it, run your system's line from the install page with:* becomes *Ask the install command for `full`, or run your system's line from the install page with:*.
- `companion/speckit/README.md`'s `speckit.checkwright.install` bullet: *installs Checkwright's `prose` profile with the one-line install, `init --profile prose --recipe speckit`* becomes *installs Checkwright's `prose` profile, or `full` when you ask for it, with the one-line install, `init --profile prose --recipe speckit`*. The bullet gains *`full` adds every kit's gates, needs `bash` 4.3 or later, and seeds Checkwright's task queue and evidence files beside Spec Kit's; the command says so before it runs.*

## Producers and consumers

Probe: the survey record's toolkit-format block for `$ARGUMENTS` and `argument-hint`; `companion/speckit/commands/install.md`, `companion/speckit/extension.yml`, `companion/speckit/README.md` and `docs/speckit.md` read whole; `grep -n "companion_line\|companion-full"` over `installer/consumer-smoke/run-smoke.sh`; installer/SPEC.md §The consumer smoke's companion paragraph.

- **The user's input** (delta 1). Producer: the Spec Kit user, through the agent. Consumer: the command body, which picks the line. No gate reads it, since Spec Kit hands the body to the agent and executes nothing.
- **The command's `companion-full` block** (deltas 1 and 3). Producer: the tracked command file, packed into the extension zip at the tag. Consumers: the agent at install, and the companion arm's Spec Kit `full` leg. The arm reds on an absent or empty block, a line with no `--profile full`, and a page line whose words differ.
- **Red conditions for adopters.** None: a `prose` install is unchanged, and the command's default stays `prose`.

## Existing sections updated

Roster probe: `git grep -n "installs the default only\|install command's install line\|init --profile prose --recipe speckit\|companion-full"` over `companion`, `docs`, `installer`.

- `companion/speckit/commands/install.md`, `companion/speckit/extension.yml` (delta 1).
- `companion/SPEC.md` — §Applying a recipe, §The two tiers (its default-only sentence, and the paragraph naming `init` with the second line as the adopter's route to `full`, which gains *or asks the install command for `full`*), §The Spec Kit extension, §Honest limits (delta 2).
- `installer/consumer-smoke/run-smoke.sh`, `installer/SPEC.md` §The consumer smoke (delta 3).
- `docs/speckit.md`, `companion/speckit/README.md` (delta 4).
- `docs/companion/SPEC.md`, `docs/installer/SPEC.md`, the generated mirrors, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (deltas 2 and 3).
- `.workflow/release-declarations.md` — Behavior changes gains a bullet led by **the Spec Kit extension's install command**: it installs `full` when asked (delta 1).

## Retired spellings

- None — the deltas add a block, an argument and a smoke comparison, and re-phrase prose; no name, path or token is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Battery green** — the full battery and the installer consumer smoke green on the landing commit, and the `companion-toolkits` job green on the next push, since it packs and installs the extension. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`), once that push's runs are green.
