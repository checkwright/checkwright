# SPEC amendment: companion

The front door says Checkwright complements the workflow an adopter already runs, and no tested consumer backs it. This amendment builds the first slice's build half of that consumer, for the two spec-authoring toolkits the ruled slice names: a Spec Kit catalog extension, and a recipe for OpenSpec. Both are packaged outside the kits, in a new top-level component, `companion/`. Each toolkit gets a **recipe**: the few knob lines and the gate removals that make the prose profile govern that toolkit's spec tree. The consumer smoke proves each recipe on a fixture tree in the toolkit's own layout: green as installed, red on each planted defect. The pages the catalog entry will link to are written or polished to the page-authoring rules.

One queue entry pairs it: [companion-toolkit-profile](TASK-QUEUE.md#companion-toolkit-profile). The catalog submission stays on that entry, gated on a published tag carrying the extension and on the observed install.

**The rulings.**

- **The component is `companion/`, repo-root-governed and not a kit.** It carries no `checks/` and no `smoke/`, so no kit-root resolver admits it. It holds `SPEC.md` (the design record) and `README.md` (the usage tier), so the on-site mirror publishes both, as it does `installer/`'s.
- **The seam.**
  - Kit mechanism: none is added. The recipes are consumer config for the prose profile, written in knobs that already exist, and the survey's first correction said as much: the work was to prove the knobs sufficient, not to invent a profile format.
  - Vendor names live in `companion/`, the docs pages and the front door, never in a kit SPEC or kit literal. The Release asset is named `checkwright-companion-{version}.zip`, because its template lives in gate-sdk/SPEC.md §Consumer payload, a kit SPEC.
  - No private rule content is involved.
- **A recipe is three files and one procedure.** `gate-sdk-config.knobs` and `canon-config.knobs` hold lines appended to the consumer's seam files of those names. `unregister.list` names gates dropped from the consumer's `gates.list`. Each line carries, in `companion/SPEC.md`, the toolkit idiom it answers. The procedure appends, drops, regenerates the hook and graph, runs the battery and commits. It is one marked block on the landing page, and the smoke runs that block verbatim, so the documented steps are the tested steps.
- **The recipes are measured, not guessed.** A fresh tree of each toolkit, with the published v0.26.0 prose profile installed, was run at authoring (below). Each recipe line answers a red that run produced.
- **The tested claim is four defect classes per toolkit, each caught by a named gate**: a broken relative or anchored link (`check-md-refs`), a dangling section citation or a title carried twice in one spec (`check-spec-pointer`), an unclosed fence (`check-spec-fence-balance`), and a documented command invoking a missing script (`check-docs-cmd`). A recipe may drop a gate only where a toolkit idiom reds it. Dropping one of these four narrows the claim, so a build that finds it must drop one escalates rather than dropping it.
- **The fixtures are authored here, in each toolkit's layout.** Their section structure follows the toolkit's own templates at the pinned version, and no template text is copied. Each fixture carries every idiom a recipe line answers, so a dropped line reds the smoke's green leg. The OpenSpec fixture must pass `openspec validate --all --strict` at the pin, which makes the toolkit its own oracle. Spec Kit ships no validator, so its fixture's fidelity rests on the pinned templates' structure, an honest limit stated in `companion/SPEC.md`.
- **The extension installs its own release.** Its install command runs the one-line install with `CHECKWRIGHT_VERSION` set to the version its `extension.yml` carries, so a listing installs the kits it was tested with.
- **The version is stamped at pack time.** The tracked `extension.yml` carries `version: "0.0.0"`, as `installer/package.json` does, and the pack step writes the tag's version into the packed copy. A tracked version would be a second version line to bump by hand.
- **The toolkits' own tools are oracles, run locally at build and on every push.** A gates-workflow job installs the pinned Spec Kit CLI, then installs the packed extension through the archive path a catalog install takes. The same job validates the OpenSpec fixture with the pinned OpenSpec CLI. The entry's oracle is the build session's local run of the same commands. The job's first green is read by the closing push's watch, so no push beyond the entry's recorded push need is spent.
- **The front door's `README.md` is this unit's to convert.** Its citations become links and it joins `CANON_KIT_CITATION_LINK_PAGES` here, as [docs-ux-authoring-rules](TASK-QUEUE.md#docs-ux-authoring-rules) states.

**Refused.**

- **An `init --recipe` flag.** It would make the procedure one command, but a recipe written by `init` becomes `init`'s file, rewritten on the next run unless the manifest records the recipe too. That is manifest, update and uninstall design for a first slice that needs none.
- **A shell script shipped with each recipe.** An adopter-facing script owes a PowerShell twin under the interpreter policy. The block runs under `/bin/sh` or Git for Windows' shell, which every supported host already has.
- **Validating `extension.yml` with a reader of our own.** It re-implements the toolkit's schema, which drifts; the toolkit's CLI is the oracle.
- **The GitHub tag archive as the download.** A tag archive wraps the whole repository in one directory, so Spec Kit would look for `extension.yml` at the repository root.
- **Copying the toolkits' templates into the fixtures.** Authored content in their structure tests the same gates and ships no third-party text.
- **Unregistering `check-spec-pointer` for OpenSpec** over repeated scenario titles. A citation of the second binds to the first, so the gate's rule holds for a citable spec, and the landing page states it as the one convention the recipe adds.

**Measured at authoring.** Each run used the published v0.26.0 prose profile, installed with `sh install.sh init --profile prose` into a scratch repository, and `./scripts/checkwright-gates --run`.

- **Spec Kit 1.0.12**, `specify init --here --non-interactive --integration claude --script sh`, which writes `.specify/` (scripts, templates, `memory/constitution.md`) and `.claude/skills/speckit-*/`:
  - As installed, 2 of 36 gates red, both on Spec Kit's own scripts under `.specify/scripts/bash/`: `check-comment-tier` (about 240 comments) and `check-path-dialect` (three absoluteness tests). `GATE_SDK_PRUNE_EXTRA_DIRS = .specify` clears both.
  - With a feature written from the templates under `specs/001-demo/` and `CANON_KIT_PROSE_SURFACE_GLOBS[] = specs/**/*.md`, `check-fence-command-head` reds on the tasks template's `bash` fence of `Task:` lines, and `check-graph` reds until the hook and graph are regenerated. Dropping `check-fence-command-head` and regenerating gives 35 of 35 green.
  - Planted defects: a mistyped relative link reds `check-md-refs`; a fenced `bash scripts/missing-helper.sh` reds `check-docs-cmd`; `specs/001-demo/spec.md §Nonexistent Section` reds `check-spec-pointer`; an unclosed fence reds `check-spec-fence-balance`. A file-relative `spec.md §…` citation is skipped by `check-spec-pointer`, whose path form is repo-relative, so the fixture's bare citations are repo-relative and its other citations are links.
- **OpenSpec 1.13.2**, `openspec init --tools claude --no-animation --no-copilot-cloud .`, plus an authored spec and one change, validated with `openspec validate --all --strict`:
  - As installed, 36 of 36 green, since no default corpus reads `openspec/`.
  - With `CANON_KIT_PROSE_SURFACE_GLOBS[] = openspec/**/*.md`, `check-manifest-temporal` reds on `(Previously: 30 minutes)` in a MODIFIED delta, the form OpenSpec's own docs show, and `check-spec-pointer` reds on a scenario title carried twice in one spec. `CANON_KIT_TEMPORAL_EXEMPT_PATHS[] = openspec/changes/**` clears the first, since change deltas and the archive are history. Renaming the scenario apart clears the second. The result is 36 of 36 green.
  - Planted defects: a mistyped link reds `check-md-refs`; a fenced missing script reds `check-docs-cmd`; an anchored link to a missing requirement reds `check-md-refs`. A bare `§Requirement: <name>` citation of a missing requirement passes `check-spec-pointer`, through its lead-clause rule, and is filed as a gap.
- **The extension's archive path.** A zip written with `python3 -m zipfile -c`, one top-level directory holding `extension.yml` and `commands/`, served from a local web server, installs with `specify extension add checkwright --from <url>` after the untrusted-URL prompt is answered `y`. It writes `.specify/extensions/checkwright/`, registers the hook in `.specify/extensions.yml`, and renders `.claude/skills/speckit-checkwright-install/` and `-check/`. The claude integration renders skills, not commands. The manifest carried `requires.speckit_version: ">=1.0.12"` and was accepted.
- **The one-line install's argument rule.** `sh -s -- --profile prose` exits 2 on the published v0.26.0 (`run-gates: unrecognized option: --profile`), while `sh -s -- init --profile prose` installs. Every command this unit writes spells the verb; the defect itself is filed as a gap.
- **The brand-token gate.** `check-kit-ref-liveness` reds a `<name>-kit` path segment naming no kit root, so a URL segment `/spec-kit/` between two slashes reds. The extension's directory is `companion/speckit/`, and a page links Spec Kit's repository root only.

## What changes

### (1) The component {design-bearing}

**Not yet applied.**

- `companion/README.md` — the usage tier: what the component holds, the two routes (the Spec Kit extension, the OpenSpec recipe), and a link to the landing page (delta 9).
- `companion/SPEC.md` — the design record, holding the rulings above as current rules:
  - the recipe format and the procedure's steps;
  - each recipe line with the idiom it answers and the run that measured it;
  - the tested claim and the escalation rule for a gate in it;
  - the fixture rules;
  - the extension's contract (delta 4), the pack step (delta 7) and the toolkit legs (delta 8);
  - the honest limits: Spec Kit's fixture fidelity rests on its templates' structure; the agent-run parts of the extension (its commands and its hook) are exercised by no harness, since Spec Kit hands them to the agent; the Windows route is documented and not run; and a pinned toolkit version says nothing about the next one.
- `companion/toolkits.list` — one `<toolkit> <package> <version>` line per toolkit: `speckit specify-cli 1.0.12` and `openspec @fission-ai/openspec 1.13.2`, with a `# contract:` header citing `companion/SPEC.md`. The CI job (delta 8) and the extension's floor check read it.
- `scripts/root-allowlist.list` gains `companion` under a comment naming it the companion package, not a kit.
- `scripts/canon-config.knobs` brings the new prose under governance: `CANON_KIT_MANIFEST_FILES` gains `companion/*/README.md`; `CANON_KIT_PROSE_SURFACE_GLOBS` gains `companion/speckit/commands/*.md`; `CANON_KIT_SEAM_SURFACE_GLOBS` gains `companion/SPEC.md`, beside `installer/SPEC.md`.
- `scripts/gate-sdk-config.knobs` sets `GATE_SDK_PRUNE_EXTRA_DIRS = fixtures`, since the fixture trees (delta 3) carry, on purpose, each idiom a gate reds.

### (2) The two recipes {mechanical}

**Not yet applied.** The lines are the measured ones above.

- `companion/speckit/recipe/gate-sdk-config.knobs`: `GATE_SDK_PRUNE_EXTRA_DIRS = .specify`.
- `companion/speckit/recipe/canon-config.knobs`: `CANON_KIT_PROSE_SURFACE_GLOBS[] = specs/**/*.md`.
- `companion/speckit/recipe/unregister.list`: `check-fence-command-head`.
- `companion/openspec/recipe/canon-config.knobs`: `CANON_KIT_PROSE_SURFACE_GLOBS[] = openspec/**/*.md` and `CANON_KIT_TEMPORAL_EXEMPT_PATHS[] = openspec/changes/**`.
- `companion/openspec/recipe/unregister.list`: no gate; the file holds its header alone.

Each file opens with a `#` comment line citing `companion/SPEC.md`, which the knob grammar and the drop step both read as a comment. A line the smoke (delta 6) shows unnecessary is deleted, and one it shows missing is added, each with its idiom recorded in `companion/SPEC.md`.

### (3) The fixtures {design-bearing}

**Not yet applied.** `companion/fixtures/speckit/` and `companion/fixtures/openspec/` each hold:

- `layout/` — a tree in the toolkit's layout at the pin:
  - **Spec Kit:** `.specify/memory/constitution.md`, a `.specify/scripts/bash/` script carrying ordinary comments and a `/*` absoluteness test, and one feature under `specs/001-<name>/` with `spec.md`, `plan.md` and `tasks.md`, whose `tasks.md` carries a `bash` fence of `Task:` lines.
  - **OpenSpec:** `openspec/config.yaml`, one capability spec under `openspec/specs/`, and one change under `openspec/changes/` with a proposal, tasks and a delta whose MODIFIED requirement carries a `(Previously: …)` line.
  - Both carry cross-file links and at least one repo-relative section citation that resolves.
- `defects/<gate>/` — one directory per claimed gate, holding the files that replace their layout counterparts to plant that gate's defect: the four classes the rulings name. OpenSpec's `check-spec-pointer` defect is a scenario title carried twice in one spec, and its `check-md-refs` defect includes an anchored link to a missing requirement.

The OpenSpec layout passes `openspec validate --all --strict` at the pin (delta 8). No toolkit template text is copied.

### (4) The Spec Kit extension {design-bearing}

**Not yet applied.** `companion/speckit/`:

- `extension.yml`:
  - `schema_version: "1.0"`;
  - `extension`: `id: checkwright`, `name: Checkwright`, `version: "0.0.0"`, a description under 200 characters, `author: Checkwright`, the repository URL, `license: Apache-2.0`, `homepage: https://checkwright.dev`;
  - `requires`: `speckit_version: ">=<pin>"`, with `<pin>` the `speckit` line of `companion/toolkits.list`, and `tools`: git, required;
  - `provides.commands`: `speckit.checkwright.install` (`commands/install.md`) and `speckit.checkwright.check` (`commands/check.md`);
  - `hooks.after_implement`: `speckit.checkwright.check`, `optional: true`, with a prompt asking whether to run the gates;
  - `tags`: two to four lowercase tags.
- `commands/install.md` — an agent instruction, frontmatter `description` then `## User Input` (`$ARGUMENTS`) and `## Steps`:
  - From a clean worktree at the repository root, run the one-line install for the host, spelled with the verb, `init --profile prose`, and with `CHECKWRIGHT_VERSION` set to this extension's `extension.version`. The curl line serves macOS and Linux, and the PowerShell script-block form serves Windows.
  - Apply the recipe at `.specify/extensions/checkwright/recipe/` by the procedure block of delta 5, carried here verbatim.
  - Run the commands `init` printed, then the battery.
  - Report each red with its gate's `help:` line. Never edit a vendored file, drop a gate the recipe does not name, or commit with `--no-verify`.
- `commands/check.md` — run `./scripts/checkwright-gates --run` from the repository root and report the verdict: on red, each failing gate, its finding and its `help:` line. It fixes nothing unasked.
- `README.md` — what the extension adds, its two commands and the hook, the one-line install it runs, and links to the landing page and the repository. The catalog requires a README carrying install and usage.
- `recipe/` — delta 2's files.

### (5) The recipe procedure {design-bearing}

**Not yet applied.** The landing page (delta 9) carries the procedure as one `sh` fence between `<!-- companion-recipe:begin -->` and `<!-- companion-recipe:end -->`. It is POSIX sh, run from the repository root after `init`, with `recipe` set to the recipe directory. It:

1. appends each recipe `*.knobs` file to the gates directory's file of the same name, skipping the comment header;
2. drops from `scripts/gates.list` every gate `unregister.list` names;
3. regenerates the hooks and the graph through the installed binary, `./scripts/checkwright-gates --emit git-hooks --write` and `--emit graph` redirected into the graph artifact, which is what `check-graph` reds on after a knob or roster change and runs with no `bash` (both measured in the Spec Kit scratch tree);
4. runs `./scripts/checkwright-gates --run`;
5. commits the gates directory.

`commands/install.md` carries the same block with `recipe=.specify/extensions/checkwright/recipe`. The smoke (delta 6) runs the page's block for every recipe and the command's block for the Spec Kit recipe, so both copies are tested. docs/site-architecture.md §Generated projections and their freshness gates gains a row for the marker block, beside the install blocks, naming its reader.

### (6) The tested consumer: the companion arm {design-bearing}

**Not yet applied.** installer/SPEC.md §The consumer smoke gains a **companion arm**, after the demo arm, and `installer/consumer-smoke/run-smoke.sh` implements it. For each recipe directory under `companion/*/recipe/`, a derived set, the arm:

1. makes a scratch consumer, commits the matching `companion/fixtures/<toolkit>/layout/` into it, and runs `init --profile prose` from the packed tarball;
2. extracts the page's `companion-recipe` block, sets `recipe` to the recipe directory, and runs it; the Spec Kit recipe also runs `commands/install.md`'s block in a second scratch consumer;
3. asserts the battery green, reading its summary;
4. for each `defects/<gate>/`, overlays its files, asserts the battery red with a `FAIL: <gate>` line, and restores the commit;
5. asserts the recipe's defect set covers the four claimed gates, so a missing defect directory reds rather than shrinking the claim.

It prints one parsed arm header per recipe, so the `installer_smoke` validate suite gains a scenario per toolkit and `.workflow/validate-baseline.txt` gains their rows at the validate stage. `run-smoke.sh`'s header `# spec:` line names the arm.

### (7) Packing the extension and the Release asset {design-bearing}

**Not yet applied.**

- `scripts/ci-pack-extension.sh <version> <out-dir>` builds `checkwright-companion-<version>.zip` and its `.sha256` sidecar:
  - it extracts `companion/speckit/`'s tracked files at `HEAD` with `git archive` into a scratch `checkwright-companion-<version>/`, adding the repository's `LICENSE`;
  - it rewrites the one `version: "0.0.0"` line to `<version>` and refuses when the line is absent or occurs twice;
  - it zips the directory with `python3 -m zipfile -c`, so one top-level directory holds `extension.yml`, and writes the sidecar in the form the other assets use.
  - It probes `git`, `python3` and the hasher before any step, refusing by name, and writes nothing inside the worktree.
- `.github/workflows/publish.yml`'s pack job runs it with the tag's version, and the Release job uploads both files.
- gate-sdk/SPEC.md §Consumer payload's `release-assets` declaration gains `checkwright-companion-{version}.zip checkwright-companion-{version}.zip.sha256`, which `check-release-assets` then holds against the workflow.

### (8) The toolkit legs {design-bearing}

**Not yet applied.** `.github/workflows/gates.yml` gains a `companion-toolkits` job on `ubuntu-latest`, its actions pinned by SHA and its tools set up by pinned setup actions rather than assumed on the image. It reads `companion/toolkits.list` and:

1. installs the pinned `specify-cli`, runs `specify init --here --non-interactive --integration claude --script sh` in a scratch directory, packs the extension with delta 7's script at version `0.0.0`, serves the zip from a local web server, and installs it with `specify extension add checkwright --from <url>`, answering the trust prompt. It asserts exit 0, `.specify/extensions/checkwright/extension.yml` present, and the hook registered in `.specify/extensions.yml`;
2. asserts `extension.yml`'s `requires.speckit_version` is `>=` the pinned version;
3. runs the pinned `openspec validate --all --strict` inside a copy of `companion/fixtures/openspec/layout/`.

The build session runs the same three steps locally before the entry moves, with a Python virtual environment for the Spec Kit CLI and `npx` for OpenSpec, which is the entry's oracle. `check-action-pinning`, `check-action-permissions` and `check-action-run-shell` hold the job's shape at commit.

### (9) The landing pages {design-bearing}

**Not yet applied.** Its act depends on whether [docs-ux-authoring-rules](TASK-QUEUE.md#docs-ux-authoring-rules) has landed its delta 2 (the citation gate and its knob) when this delta lands:

- **It has:** write and polish every page below under its rules and gates, and add `README.md` to `CANON_KIT_CITATION_LINK_PAGES` in `scripts/canon-config.knobs`.
- **It has not:** write every page below to the rules as that amendment states them, and leave the binding to it. Its delta 4 then adds `README.md`, since the front door is already converted.

The pages:

- **A new page, `docs/spec-toolkits.md`**, title *Spec Kit and OpenSpec*, the H1 stating what it covers, placed in the nav as a child of Install (`docs/install.md` gains a `nav_id`). It carries:
  - what the gates catch in a spec tree, as the four claimed classes;
  - the Spec Kit route: add the extension, run its install command;
  - the OpenSpec route: install, then the procedure block of delta 5 with the recipe's files fetched from the repository at the release's tag;
  - the one convention OpenSpec specs meet: a title unique within its spec;
  - what is tested, and against which versions, by link to `companion/toolkits.list` rather than by restating them;
  - the honest limits from delta 1;
  - links to Spec Kit's and OpenSpec's repository roots only, and to this repository's files by self-repo blob link.
- **`README.md`**: the claim that Checkwright complements the workflow an adopter runs links the new page, and its three citations become links.
- **`docs/index.md`**: the same claim links the new page.
- **`docs/positioning.md` §The tiered compatibility claim** gains one sentence: the enforcement tier is tested beside two spec-authoring toolkits, with a link to the new page.
- **`docs/install.md`** names the new page in the sentence that introduces profiles, for a reader whose specs another toolkit writes.

`check-front-door-verbs` reads `README.md` and `docs/install.md`, so a route spelled on a front-door page names a released verb, which `init` is.

## Producers and consumers

- **A recipe directory.**
  - Producer: hand edits under `companion/<toolkit>/recipe/`.
  - Consumers: the companion arm (delta 6), which derives the recipe set from the directories present; the procedure block (delta 5), run by an adopter or by the extension's install command; the pack step (delta 7), which ships the Spec Kit recipe inside the zip.
  - Red conditions: the arm's green leg reds on a recipe that no longer suffices, and its coverage check reds on a recipe missing a claimed defect.
  - Each member's value: delta 2's lines.
- **`companion/toolkits.list`.**
  - Producer: a hand edit when a pin moves.
  - Consumers: the `companion-toolkits` job, which reads the package and version per toolkit, and its floor assertion, which reads the `speckit` line.
  - Every field has a reader: the toolkit name keys the line, the package is what the job installs, and the version is what it pins and asserts.
- **`extension.yml`'s version.**
  - Producer: the pack step, from the tag.
  - Consumers: Spec Kit's install, the catalog's update check once submitted, and the install command, which passes it as `CHECKWRIGHT_VERSION`.
  - Red condition: the pack step refuses a missing or doubled placeholder line.
- **The `companion-recipe` marker block.**
  - Producer: the landing page.
  - Consumers: the companion arm, which runs it. The new docs/site-architecture.md row names that reader in prose, as the install-block rows do; it carries no `projection:` key, since no freshness gate generates the block, so `check-projection-roster` does not read it.
  - Red condition: the arm refuses an absent or empty block rather than skipping it.
- **The `checkwright-companion-{version}.zip` asset.**
  - Producer: `publish.yml` on a tag.
  - Consumers: `check-release-assets`, which holds the declaration at the tag; the catalog, whose entry's `download_url` names it once submitted.
- **`GATE_SDK_PRUNE_EXTRA_DIRS` in this repository.**
  - Consumer: every walk adapter reading the prune set.
  - Red condition, a narrowed corpus: every gate whose walk the prune set bounds stops reading `companion/fixtures/`. No tracked path sits under a directory named `fixtures` today (`git ls-files '*fixtures/*'` lists none), so the prune removes nothing any reader counts now. `check-exec-bit` already prunes `fixtures` through its own knob.
- **Roster-holding readers of the minted names.**
  - `check-root-tiering` reads `scripts/root-allowlist.list` (delta 1).
  - The `installer_smoke` parser reads the arm headers (delta 6).
  - `check-release-assets` reads the `release-assets` declaration (delta 7).
  - `check-kit-ref-liveness` reads every tracked path segment, which the component's naming satisfies.

## Existing sections updated

Roster probes, over the tracked tree: `git grep -n` for `complements`, `§The tiered compatibility claim`, `release-assets:`, `The demo arm`, `root-allowlist` and `installer/SPEC.md`; `ls companion` (absent); and the probe runs above.

- `companion/README.md`, `companion/SPEC.md`, `companion/toolkits.list`, `scripts/root-allowlist.list`, `scripts/canon-config.knobs`, `scripts/gate-sdk-config.knobs` (delta 1).
- `companion/speckit/recipe/`, `companion/openspec/recipe/` (delta 2).
- `companion/fixtures/` (delta 3).
- `companion/speckit/extension.yml`, `companion/speckit/commands/install.md`, `companion/speckit/commands/check.md`, `companion/speckit/README.md` (delta 4).
- `docs/site-architecture.md` — §Generated projections and their freshness gates, the marker-block row (delta 5).
- `installer/SPEC.md` — §The consumer smoke, the companion arm (delta 6).
- `installer/consumer-smoke/run-smoke.sh` (delta 6).
- `.workflow/validate-baseline.txt` — the companion arm's scenario rows, written at validate (delta 6).
- `scripts/ci-pack-extension.sh`, `.github/workflows/publish.yml` (delta 7).
- `gate-sdk/SPEC.md` — §Consumer payload's `release-assets` declaration (delta 7).
- `.github/workflows/gates.yml` — the `companion-toolkits` job (delta 8).
- `docs/spec-toolkits.md`, `README.md`, `docs/index.md`, `docs/positioning.md`, `docs/install.md` (delta 9).
- `docs/companion/SPEC.md`, `docs/companion/README.md`, `docs/installer/SPEC.md`, `docs/gate-sdk/SPEC.md` — the generated on-site mirrors, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (deltas 1, 6 and 7).

## Retired spellings

- None — the unit adds a component, files and a job, and retires no name.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; no root-level amendment remains for this unit (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Run** — the full battery green; the consumer smoke green with the companion arm; delta 8's three steps green locally at the pins; `check-docs-render-fidelity` green over the new and polished pages.
- [ ] **The entry demotes, it does not close.** The submission half outlives this amendment, so the merge returns [companion-toolkit-profile](TASK-QUEUE.md#companion-toolkit-profile) to the deferred section at the position its promoting commit took it from, with its `[cost:]` and `[surface:]` tags restored from that diff and its roadmap tags kept (canon-kit/SPEC.md §Merging an amendment, step 4). The body is cut to the submission half: the catalog issue, gated on a published tag carrying the asset and on the observed install. The demotion lands before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`).
