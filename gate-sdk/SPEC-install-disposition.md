# SPEC amendment: install-disposition

**`zero-config` is re-drawn at the line between a defect and a house rule, and every kit's members are re-judged against it.** §The install disposition says a `zero-config` gate reads only surfaces `init` writes and the adopter does not author. installer/SPEC.md §Profiles promises the opposite for two profiles: `starter`'s gates "red on real defects in your tree", and `prose` "arms link, claim, staleness and pointer governance over every `README.md`". Both promises are kept by gates that read adopter-authored content. The two owner documents disagree, and the tree follows neither: 54 members declare `zero-config`, and a fresh install reds on ordinary adopter files.

**The ruling this amendment carries.** Operator direction, relayed by the iteration lead on 2026-10-01 (not a `/consult` ruling), choosing among the options the spec stage put:

- **Defect versus house rule.** A gate stays `zero-config` over adopter-authored content only when its one red there is a defect by that content's own terms, or when it fires only on a construct the methodology introduces. A gate holding adopter content to a house rule an ordinary file breaks without being wrong moves to `on-surface`. That moves twelve members.
- **`check-spec-pointer` splits by a knob, not by a move.** Its dangling-citation check is a defect and stays `zero-config` in every `prose` and `full` install; its one-title-per-file rule, the house rule that had moved the whole gate, goes behind `CANON_KIT_SPEC_POINTER_TITLE_ONCE`, off by default (delta 11). A later direction of the same day, relayed the same way, revising the first.
- **The leak patterns move with them.** `check-tree-terms` and `check-commit-msg` go `on-surface`; trimming `init`'s seeded pattern list to keep them at install was offered and not chosen.
- **`never` for `check-knob-default-coupling`**, whose input a vendored tree withholds.
- **`seeded-ci-gates-on-surface` takes its boundary-note branch**: `check-action-pinning` and `check-action-permissions` assert a supply-chain house rule, so they stay `on-surface`, permanently.

**Measured at authoring.**

- *The census.* `grep -l '^# install: zero-config' */checks/*.gate` lists 54: canon-kit 26, gate-sdk 15, queue-kit 6, evidence-kit 2, one each in context-kit, delegation-kit, doctrine-kit, guard-kit and site-kit. Every one is a `.gate` descriptor.
- *Fresh-install probes*, over a payload packed with `--pack-installer` from this tree and installed with `init --profile <p>` into scratch repositories. A bare repository is green in `starter`, `delegation`, `prose` and `full`. A repository holding plain adopter files reds:
  - in `starter`, `check-path-dialect` (a `ROOT=$(git rev-parse --show-toplevel)` line) and `check-tree-terms` (a `/home/<name>` path in a README);
  - in `prose`, also `check-comment-tier` (a plain `#` comment in a `.sh`), `check-manifest-temporal` (a README line opening *Previously*), `check-manifest-count` (*Follow these 3 rules*), `check-fence-command-head` (an `npm install` fence), `check-spec-pointer` (one heading title used twice in a README), `check-spec-dod-singleton` (a `SPEC.md` with no Definition of Done), `check-amendment-queue` (an unrelated notes file named on the amendment glob) and `check-md-refs` (a link to an absent file);
  - in `delegation`, `check-brevity` exits 2 on an adopter `CLAUDE.md` lacking `## Shared conventions`.
  - The survey record's block of 2026-10-01 carries the probe and its oracle.
- *The per-member read*, from each member's descriptor, knob defaults and module (§Producers and consumers, point 6, names every member's value).

## What changes

### (1) §The install disposition re-draws the classes

gate-sdk/SPEC.md §The install disposition is rewritten at its three bullets and the paragraph under `on-surface` {design-bearing}. **Not yet applied.**

> - **`zero-config`** — the gate registers in a fresh consumer, because nothing it can red on the tree `init` makes is a rule the adopter has not chosen. Every surface it reads is one `init` writes or a kit ships, its subject is a construct only the methodology introduces (an amendment's `## What changes`, a `TODO(task:)` tag, a knob a consumer sets), or its one red on content the adopter authored is a **defect by that content's own terms**: a link or path that resolves nowhere, an unclosed fence, a claim the tree falsifies, a pipeline that can lose its match.
> - **`on-surface`** — the gate's subject is one the adopter authors (a glossary, a docs host, a stage attestation, their own workflows), or it holds adopter-authored content to a **house rule** ordinary content breaks without being wrong (a comment must be a directive, no temporal narration, an action ref pinned by SHA). It arms when the adopter opts in rather than at install. `init` may seed one member of such a subject, as it seeds a CI workflow. That member is held at the publisher, where the installer's smoke runs the gate over it green, not in the adopter's battery (installer/SPEC.md §The consumer smoke).
>
> A house rule reaching content `init` never wrote is the arming `on-surface` defers: the adopter's first red would be a rule they never picked, on files that were not wrong. A defect is not deferred, since the content is wrong on its own terms and the red is the install's first value. A gate carrying both takes its house-rule arm behind a knob that defaults off, so the defect arm installs alone; one whose house rule is its whole subject is `on-surface`.
> - **`never`** — the gate is not auto-registered on any tree, because its subject cannot exist in a vendored tree at all, because its input is withheld there so it could only skip, or because it is declared never-registered. §Consumer smoke's declaration valve recognises this class from the other side.

The two paragraphs after the vocabulary (the install-time-reachability paragraph and the derived-roster one) are unchanged.

### (2) Twelve members move to `on-surface`, one to `never`

Each descriptor's `# install:` line changes {mechanical}:

- to `on-surface`: `canon-kit/checks/check-manifest-temporal.gate`, `check-manifest-count.gate`, `check-fence-command-head.gate`, `check-comment-tier.gate`, `check-spec-dod-singleton.gate`, `check-spec-derivable-section.gate`, `check-spec-embedded-source.gate`, `check-amendment-queue.gate`; `gate-sdk/checks/check-path-dialect.gate`, `check-tree-terms.gate`, `check-commit-msg.gate`; `context-kit/checks/check-brevity.gate`;
- to `never`: `canon-kit/checks/check-knob-default-coupling.gate`.

`canon-kit/checks/check-spec-pointer.gate` keeps `zero-config`, its house rule moving behind a knob instead (delta 11). This repository's `scripts/gates.list` registers every one by hand and is unaffected.

### (3) §Profiles and `profiles.list` say what each profile now arms

installer/SPEC.md §Profiles {mechanical}. **Not yet applied.**

- `starter`'s bullet keeps its sentence: it still arms `check-pipe-membership`, a defect gate over the adopter's own shell, and the gate family's meta-gates.
- `prose`'s second sentence, "It arms link, claim, staleness and pointer governance over every `README.md` at any depth and your agent file, real for a documentation repository, whose docs tree is usually a tree of READMEs.", becomes:

> It arms link, citation, command and claim checks over every `README.md` at any depth and your agent file (a dangling link or section citation, a fenced command that cannot run, an unclosed fence, a tracking claim the tree falsifies), real for a documentation repository, whose docs tree is usually a tree of READMEs. canon-kit's house rules, such as temporal narration, counts, one title per file and comment tiers, are yours to turn on (`--with-gate`, or a knob), since they red prose that is not wrong.

`installer/profiles.list`:

- The `starter` comment's "a tracked script missing its exec bit, a mutable action ref in their workflows, a malformed commit subject" names three `on-surface` gates. It becomes "a pipeline under `pipefail` that can lose its match to a closed pipe".
- The `prose` comment's "link, claim, staleness and pointer governance over every README.md" becomes "link, citation, command and claim checks over every README.md", followed by "canon-kit's house rules are the adopter's to turn on".

### (4) §check-action-pinning rules its disposition permanent

gate-sdk/SPEC.md §check-action-pinning {mechanical}. **Not yet applied.** Its disposition sentence, "`install: on-surface` (§The install disposition).", becomes:

> `install: on-surface`, permanently: a tag ref runs, so pinning is a supply-chain house rule rather than a defect in the adopter's workflow (§The install disposition), and the workflow `init` seeds does not arm it over the adopter's own.

§check-action-permissions' disposition line gains the same clause, citing §check-action-pinning.

### (5) The release declaration surface and the mirrors follow

`.workflow/release-declarations.md` gains a Behavior-changes bullet {mechanical}:

> - **install disposition** — `check-manifest-temporal`, `check-manifest-count`, `check-fence-command-head`, `check-comment-tier`, `check-spec-dod-singleton`, `check-spec-derivable-section`, `check-spec-embedded-source`, `check-amendment-queue`, `check-path-dialect`, `check-tree-terms`, `check-commit-msg` and `check-brevity` are no longer registered at install, and `check-knob-default-coupling` never is. The twelve hold your content to house rules rather than to defects, and the last can only skip in a vendored tree, whose kit SPECs it reads are withheld. `update` drops them from a `gates.list` you never edited; to keep one of the twelve, pass `--with-gate <name>`.
> - **`CANON_KIT_SPEC_POINTER_TITLE_ONCE`** — `check-spec-pointer`'s one-title-per-file rule is off by default; its dangling-citation check is unchanged. Set the knob `on` in `canon-config.knobs` to keep the rule.

`docs/gate-sdk/SPEC.md`, `docs/installer/SPEC.md`, `docs/canon-kit/SPEC.md` and `docs/companion/SPEC.md` are regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in the commit landing the last of deltas 1, 3, 4, 6, 7, 8, 9, 10 and 11.

### (6) The agent-file predicate drops context-kit

`needs_agent_file` in `native/src/installer/recipe.rs` matches doctrine-kit alone {mechanical}, since context-kit no longer starts with a gate reading the file. The seeded body in `native/src/installer/init.rs` keeps its `## Shared conventions` heading, and its comment's "the gate init registers" becomes "the gate an adopter registers with `--with-gate`". **Not yet applied:** installer/SPEC.md §What init seeds' closing sentence of the agent-file paragraph, "The agent-file *membership* predicate asks a different question, whether the file must *exist* for a kit's starting gates, which context-kit answers yes to while writing nothing into it.", loses its last clause: "…whether the file must *exist* for a kit's starting gates."

### (7) The hook-refusal legs plant a defect a starting gate refuses

`installer/consumer-smoke/run-smoke.sh` (the `bash`-less arm) and `installer/consumer-smoke/run-smoke.ps1` (each profile's hooks step) plant, in place of the home-directory path, a root `*.sh` whose `set -o pipefail` pipeline feeds an array expansion into `grep -q`, and keep reading the refusing gate off the hook's `pre-commit: <gate> failed` line and holding it to the consumer's registry {mechanical}. Their `# spec:` comments' "which gate-sdk's zero-config message-pattern seed names" becomes "which a gate-sdk starting gate refuses". The PowerShell plant is written with `[IO.File]::WriteAllText` and a `\n` line ending, as the current plant is.

`.github/workflows/gates.yml`'s Windows step that commits through the generated hooks with no `bash` on `PATH` (a `starter` consumer) plants `hooks-violation.sh`, a `git rev-parse --show-toplevel` root, and matches `check-path-dialect failed`. It plants the same pipeline instead and matches `check-pipe-membership failed`.

**Not yet applied:** installer/SPEC.md §The consumer smoke, in the `bash`-less arm's third bullet and the PowerShell driver's hooks bullet, "planting a home-directory path" and "on a planted home-directory path" become "planting a shell pipeline that can lose its match" and "on a planted shell pipeline that can lose its match".

### (8) The leaked-builder-path defence keeps its second hold

gate-sdk/SPEC.md's build-native section rests its sufficiency argument on two holds, the second being that "the installer smoke runs a real `init` and a consumer battery over the result, where §check-tree-terms reads the committed binary". No profile registers `check-tree-terms` after delta 2, so the battery no longer reads it. The consumer smoke's profile loop keeps the hold by name {mechanical}: after each profile's green battery, it runs the placed binary's `--run --only check-tree-terms` in the consumer (a sole name the registry does not hold resolves against the kit's `checks/`, §run-gates) and requires it green, so the committed binary is still scanned against the seeded patterns. The PowerShell driver's battery step does the same.

**Not yet applied:** that sentence becomes "…and the installer smoke runs a real `init` and, over the committed result, §check-tree-terms by name, since no profile registers it". §check-tree-terms' "binary arm live wherever a consumer tracks the gate binary, as the installer smoke's artifact legs do" becomes "…as the installer smoke's by-name run over each installed consumer does".

### (9) The prose around the moved members follows

Two surfaces describing the prior flow follow the move {mechanical}. **Not yet applied.**

- installer/SPEC.md §What init seeds, the paragraph opening "**A disposition change reaches trees already installed.**", gains after its first sentence: "A gate moving the other way leaves an unedited `gates.list` at that run, and one the adopter edited keeps it, reported as still registered."
- `context-kit/smoke/install.sh`'s `no-port` comment drops the clause "this kit shipping check-brevity zero-config, so a crate table ADDS violations rather than removing them" from its leg-2 ground, since context-kit ships no `zero-config` member once delta 2 lands and assertion B no longer reads the roster for it.

### (10) The companion's tested claim follows `check-spec-pointer`'s split

The companion's install lines stay as they are, and OpenSpec's planted `check-spec-pointer` defect becomes a dangling citation {design-bearing}. An earlier revision of this delta added `--with-gate check-spec-pointer` to each line, on an operator direction relayed by the lead on 2026-10-01; delta 11's later direction keeps the gate registered at install, so the flag is owed nowhere and no line carries it.

**Why the defect changes.** companion/SPEC.md §The tested claim proves each toolkit's recipe against `check-spec-pointer` "on `prose` for both toolkits", and the consumer smoke's companion arm requires `FAIL: check-spec-pointer` on each toolkit's planted defect (`installer/consumer-smoke/run-smoke.sh:1039`, `:1103-1114`). Spec Kit's defect is already a dangling citation (`companion/fixtures/speckit/defects/check-spec-pointer/specs/001-release-notes/plan.md`, a citation of `§Rollout Plan`, which `spec.md` lacks), so it reds at the knob's default. OpenSpec's is a scenario title carried twice in one spec (`companion/fixtures/openspec/defects/check-spec-pointer/openspec/specs/notes/spec.md`), which reds only with the title rule on. Arming the rule in the OpenSpec recipe would opt every adopter of it into a house rule the toolkit's own specs often break (companion/SPEC.md §The OpenSpec recipe), so the recipe does not, and the planted defect becomes a citation naming a requirement the capability spec lacks.

- `companion/fixtures/openspec/defects/check-spec-pointer/` replaces its duplicated title with a repo-relative `openspec/specs/notes/spec.md §Requirement: <a name the spec lacks>` citation in the capability spec's prose, in the form companion/SPEC.md's one-unchecked-form paragraph says the fixtures use.
- The arm needs no edit: its claimed-gate roster and its fixture directories keep their names.

**The three recipe lines stay.** `GATE_SDK_PRUNE_EXTRA_DIRS = .specify` (`check-comment-tier`, `check-path-dialect`), `CANON_KIT_FENCE_PROGRAMS_EXTRA[] = Task:` (`check-fence-command-head`) and `CANON_KIT_TEMPORAL_EXEMPT_PATHS[] = openspec/changes/**` (`check-manifest-temporal`) answer reds from gates the install no longer registers, and keep a toolkit tree green for an adopter who registers them.

**Not yet applied:** companion/SPEC.md §Recipes' rule "**Each line answers a red, or arms a gate that registers disarmed.**" becomes "**Each line answers a red, answers a red of a house rule the adopter registers, or arms a gate that registers disarmed.**", and its next sentence gains after "reds a gate on one of the toolkit's own idioms until the line is there": "or would, once the adopter registers that house rule with `--with-gate`". §The tested claim's bullet "a dangling section citation, or a title carried twice in one spec, by `check-spec-pointer`" becomes "a dangling section citation, by `check-spec-pointer`". §The OpenSpec recipe's convention paragraph ("**One convention, and no dropped gate for it.**") becomes: "**One convention, left to the adopter.** OpenSpec specs often repeat a scenario title under two requirements, and a citation of the second title binds to the first. `check-spec-pointer`'s one-title-per-file rule reds the repeat when `CANON_KIT_SPEC_POINTER_TITLE_ONCE` is `on`, a house rule the recipe does not arm; the OpenSpec page states the convention and the knob line." §The fixtures' "OpenSpec's `check-spec-pointer` defect is a scenario title carried twice in one spec" becomes "OpenSpec's `check-spec-pointer` defect is a citation of a requirement its spec does not carry". docs/openspec.md's "Keep each title unique within a spec" paragraph keeps its advice and replaces "so `check-spec-pointer` reds the repeat" with "so set `CANON_KIT_SPEC_POINTER_TITLE_ONCE = on` in `canon-config.knobs` to have `check-spec-pointer` red the repeat".

### (11) `check-spec-pointer`'s title rule goes behind a knob, off by default

canon-kit gains `CANON_KIT_SPEC_POINTER_TITLE_ONCE`, `off` (default) or `on`, a scalar row in `native/src/knobs/canon_kit.rs` beside `CANON_KIT_COMMENT_ACTIONS`, whose `off`/`on` shape it takes {design-bearing}. `native/src/gates/spec_pointer.rs` runs the one-title-per-file check (the finding at `:128`) only under `on`, and names the rule on its clean line only then; its `help:` line's clause "and a title carried twice in one file is renamed apart" is printed only under `on`. The member's declared knob list in `native/src/gates/mod.rs` (`:1051` onward) gains the name. The dangling-directive and dangling-citation passes and the lead-clause bound are unchanged at every value, and so is a limit the amendment states rather than fixes: a bare `§N` in adopter prose reds as a dangling citation (run at align, Producers and consumers point 7), narrowing it being a separate unit.

The fixture pair: `bad/scripts/canon-config.knobs` sets the knob `on`, so `bad/expect.txt`'s `heading title carried twice in one file` line stands; `good/` gains a manifest file carrying one title twice, clean at the default.

This repository sets `CANON_KIT_SPEC_POINTER_TITLE_ONCE = on` in `scripts/canon-config.knobs`, since docs/site-architecture.md's section-names rule rests on the rule holding "over every page and README".

**Not yet applied:** in canon-kit/SPEC.md §check-spec-pointer, the paragraph "**A title names one section per file.**" becomes:

> **A title names one section per file, where the consumer turns the rule on.** Under `CANON_KIT_SPEC_POINTER_TITLE_ONCE=on`, two headings in one manifest file whose qualifier-stripped text is equal are red, at any levels, and the finding names the file, both line numbers and the title. A resolver takes the first match, so a pointer meaning the second binds to the first from the day it is written. The rule is off by default because ordinary prose repeats a title under two sections without being wrong (gate-sdk/SPEC.md §The install disposition); a citation is then held to the first match alone. Honest limit, at every value of the knob: the unqualified citation form reads a bare section-number mark (`§3`, `§2.1`) as a heading citation and reds it when no heading carries that text, so a document using `§` as a section number reds though it is not wrong.

The calibration paragraph's "so no new config knob" becomes "and one knob of its own, `CANON_KIT_SPEC_POINTER_TITLE_ONCE`", and §Layout and configuration gains, after the `CANON_KIT_COMMENT_*` bullet: "- `CANON_KIT_SPEC_POINTER_TITLE_ONCE` — `off` (default) or `on`: `on` arms `check-spec-pointer`'s one-title-per-file rule (§check-spec-pointer)."

## Producers and consumers

1. **The re-drawn definition.** No new state, event or interface: the vocabulary keeps its three values. Its readers are the kit authors choosing a descriptor's line, `check-install-disposition` assertion A (the closed vocabulary, unchanged), and the consumer smoke's registration accounting, which reports a `zero-config` declaration its probe contradicts (§Consumer smoke). The definition's reach into prose is the roster below.
2. **The thirteen descriptor lines.** Producer: delta 2. Consumers, each reading the line and none holding a gate name:
   - the installer's recipe derivation (`recipe_gates`), the whole of a fresh consumer's registry (installer/SPEC.md §What init seeds). A fresh `starter` registers 12 gates where it registered 15, `prose` 29 where 41, `delegation` 23 where 27, `full` 41 where 54 (the probe's counts less the moved members per profile);
   - `init` and `update` on an installed tree, which rewrite an unedited `gates.list` from the derivation and so drop the moved members (the Behavior-changes bullet, delta 5);
   - `--with-gate`, which refuses a `never` member (installer/SPEC.md §Selecting kits and gates), so `check-knob-default-coupling` is out of an adopter's reach by construction while the twelve stay one flag away;
   - `check-install-disposition` assertion B, which holds only `zero-config` members to the kit's smoke roster: moving a member out removes an obligation and reds nothing, and every moved member stays in its kit's `smoke/install.sh` roster, which the smoke may register beyond `init`'s;
   - `doctor`'s floor, which moves nothing: its `bash` audience derives from kit-root shebangs and its registered audience from registered members' programs (`native/src/toolfloor.rs`), and the thirteen are compiled members owing no program beyond the adopter floor.
3. **The agent-file predicate.** `needs_agent_file` in `native/src/installer/recipe.rs:145` answers whether a kit's starting gates need the agent file, and names `context-kit` for `check-brevity`, its one starting gate. With `check-brevity` `on-surface`, context-kit starts with no gate, so delta 6 drops it from the predicate. doctrine-kit still seeds the file wherever it is selected, and the seeded body keeps its `## Shared conventions` heading, the section `check-brevity` reads by default, so a `--with-gate check-brevity` on a seeded file is green at once.
4. **The hook-refusal legs of the consumer smoke.** The `bash`-less arm and the PowerShell driver each commit a planted home-directory path and require the installed pre-commit hook to refuse it, the refusing gate read off the hook's line and held to the registry (installer/SPEC.md §The consumer smoke; `installer/consumer-smoke/run-smoke.sh:1503-1512`, `installer/consumer-smoke/run-smoke.ps1:188-200`). `check-tree-terms` is the only member that refuses it, and it leaves every profile's registry, so both legs would red on a commit that lands. Delta 7 re-plants a defect a `starter` member refuses at pre-commit, **run at authoring**: a fresh `starter` install with the three moved `gate-sdk` members unregistered, hooks installed, refuses a staged root `plant.sh` whose `set -o pipefail` pipeline feeds an array expansion into `grep -q`, the hook printing `pre-commit: check-pipe-membership failed`. `check-pipe-membership` spawns only `git` (its registry row, `native/src/gates/mod.rs:2447-2453`), so the `bash`-less arm can run it. The CI workflow's Windows no-`bash` hook step reads `check-path-dialect` the same way over a `starter` consumer, and takes the same plant.
5. **Readers whose red condition moves.** No reader reds on finding none over the registry: the lattice monotonicity assertion is over containment, which a uniform removal keeps, since every moved member leaves every profile that carried it. The per-profile gate counts are pinned in no assertion: the smoke and the demo read the `All <N> gates passed` token with `N` unbound. `docs/install.md`'s commit-cost figure is a measured claim at `full`, re-measured at release. An installed tree that kept a `commit-msg` hook `init` generated for `check-commit-msg` keeps the file after the member leaves; it runs no gate, and the next `--emit git-hooks --write` is the adopter's.
6. **Every member's satisfying value** (point 6), the 54 enumerated by `grep -l '^# install: zero-config' */checks/*.gate`:
   - **`on-surface`, a house rule over adopter content (12):** canon-kit `check-manifest-temporal`, `check-manifest-count`, `check-fence-command-head`, `check-comment-tier`, `check-spec-dod-singleton`, `check-spec-derivable-section`, `check-spec-embedded-source`, `check-amendment-queue`; gate-sdk `check-path-dialect`, `check-tree-terms`, `check-commit-msg`; context-kit `check-brevity`.
   - **`never`, input withheld in a vendored tree (1):** canon-kit `check-knob-default-coupling`.
   - **`zero-config`, a defect over adopter content (7):** canon-kit `check-md-refs`, `check-docs-cmd`, `check-tracking-claim`, `check-spec-fence-balance`, `check-pendency-contradiction`, `check-spec-pointer` (its house rule off by default, delta 11); gate-sdk `check-pipe-membership`.
   - **`zero-config`, a methodology construct only (5):** canon-kit `check-fence-run`, `check-knob-citation`, `check-todo-task-liveness`, `check-amendment-update-target`, `check-amendment-retired-spelling`.
   - **`zero-config`, knob-armed with an empty default (9):** canon-kit `check-citation-link`, `check-docs-page-repeat`, `check-docs-restatement-parity`, `check-task-path-claim`, `check-task-label-resolution`, `check-provenance-seam`; gate-sdk `check-portability-floor`, `check-projection-roster`; site-kit `check-docs-highlight-coverage`.
   - **`zero-config`, over `init`-written, kit-shipped or gate-family surfaces (the remaining 20):** gate-sdk `check-gate-exemption-tasks`, `check-gate-output`, `check-gate-fail-closed`, `check-gate-fixture-coverage`, `check-graph`, `check-install-disposition`, `check-front-end-fail-open`, `check-kit-roots-dialect`, `check-packed-links`; queue-kit `check-queue-hygiene`, `check-queue-wrap`, `check-tag-lead-line`, `check-task-names`, `check-task-conservation`, `check-queue-prose-precondition` (the seeded queue); evidence-kit `check-evidence-manifest`, `check-evidence-baseline`; delegation-kit `check-gate-tamper`; doctrine-kit `check-doctrine-registration`; guard-kit `check-door-binding`.
7. **`CANON_KIT_SPEC_POINTER_TITLE_ONCE`.** Producer: canon-kit's knob table, a consumer's `canon-config.knobs` or its overlay. Enabling configuration: this repository's `scripts/canon-config.knobs` sets it `on`, and `bad/` of the fixture pair does. Consumer: `check-spec-pointer` alone, at its manifest-heading pass, read once per run. Roster-holding readers of the new name: `check-knob-citation` resolves the SPEC's citation of it once the row lands in the same commit; `check-knob-default-coupling` couples the SPEC's stated `off` to the row; the member's declared knob list is `check-graph`'s admissibility roster, and the descriptor's `couples=` gains nothing, a static knob's file being a derived couple. **Red condition moved:** at the default, `check-spec-pointer` no longer reds a repeated title; it reds on no count and no floor, so the narrower verdict only removes findings. docs/site-architecture.md's section-names paragraph, the one prose reader relying on the rule, keeps it through this repository's `on`. A prose install's first battery still holds the adopter's citations and pointers to resolving, while a README repeating a title, which the probe reddened, installs green. **Run at align:** a scratch repository registering `check-spec-pointer` alone, holding a README with the prose "See §3 for terms" under a heading `## 3. Terms` (and, separately, under `## Terms`), reds "§heading in no governed file" on both: a bare section-number mark in ordinary adopter prose is read as a heading citation and does not resolve, so the dangling-citation arm is a defect only for content that uses `§` as the citation mark the gate reads.

## Existing sections updated

Roster produced by `grep -rn 'zero-config\|on-surface' --include='*.md' .` over the tracked tree less the generated mirrors, `grep -n 'home' installer/consumer-smoke/run-smoke.sh installer/consumer-smoke/run-smoke.ps1`, reading `native/src/installer/recipe.rs` and `init.rs` at the agent-file seed, and a read-only reader survey over the installer, the consumer smoke, `doctor`, the kit READMEs and docs pages.

- `gate-sdk/SPEC.md` — §The install disposition (delta 1); §check-action-pinning and §check-action-permissions (delta 4).
- `canon-kit/checks/check-manifest-temporal.gate`, `canon-kit/checks/check-manifest-count.gate`, `canon-kit/checks/check-fence-command-head.gate`, `canon-kit/checks/check-comment-tier.gate`, `canon-kit/checks/check-spec-dod-singleton.gate`, `canon-kit/checks/check-spec-derivable-section.gate`, `canon-kit/checks/check-spec-embedded-source.gate`, `canon-kit/checks/check-amendment-queue.gate`, `canon-kit/checks/check-knob-default-coupling.gate`, `gate-sdk/checks/check-path-dialect.gate`, `gate-sdk/checks/check-tree-terms.gate`, `gate-sdk/checks/check-commit-msg.gate`, `context-kit/checks/check-brevity.gate` — the `# install:` line (delta 2).
- `installer/SPEC.md` — §Profiles (delta 3); §What init seeds (delta 6); §The consumer smoke (delta 7).
- `installer/profiles.list` — the `starter` and `prose` comments (delta 3).
- `.workflow/release-declarations.md` — the Behavior-changes bullet (delta 5).
- `docs/gate-sdk/SPEC.md`, `docs/installer/SPEC.md`, `docs/canon-kit/SPEC.md`, `docs/companion/SPEC.md` — regenerated mirrors (delta 5).
- `native/src/installer/recipe.rs`, `native/src/installer/init.rs` — the agent-file predicate and the seed's comment (delta 6).
- `installer/consumer-smoke/run-smoke.sh`, `installer/consumer-smoke/run-smoke.ps1` — the planted defect (delta 7) and the by-name `check-tree-terms` run (delta 8).
- `.github/workflows/gates.yml` — the Windows no-`bash` hook step's plant and match (delta 7).
- `gate-sdk/SPEC.md` — §build-native's sufficiency argument and §check-tree-terms' binary-arm sentence (delta 8).
- `installer/SPEC.md` — §What init seeds' disposition-change paragraph (delta 9).
- `context-kit/smoke/install.sh` — the `no-port` comment's leg-2 clause (delta 9).
- `companion/SPEC.md` — §Recipes, §The OpenSpec recipe, §The tested claim and §The fixtures (delta 10).
- `docs/openspec.md` — the title-convention paragraph (delta 10).
- `companion/fixtures/openspec/defects/check-spec-pointer/openspec/specs/notes/spec.md` — the planted dangling citation (delta 10).
- `canon-kit/SPEC.md` — §check-spec-pointer and §Layout and configuration (delta 11).
- `native/src/knobs/canon_kit.rs`, `native/src/gates/spec_pointer.rs`, `native/src/gates/mod.rs` — the row, the gated rule and the declared knob (delta 11).
- `canon-kit/gate-tests/check-spec-pointer/` — `bad/scripts/canon-config.knobs` and a `good/` duplicate title (delta 11).
- `scripts/canon-config.knobs` — this repository's `on` (delta 11).
- `TASK-QUEUE.md` — the two paired entries move to Done at merge (all deltas).

## Retired spellings

- None — the disposition vocabulary keeps its three values; members change value, and no name is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
