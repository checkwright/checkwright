# SPEC amendment: rule-id-coverage

Pairs `rule-id-coverage`. `check-spec-pointer` checks one direction only. Its calibration rules the reverse out, because flagging a requirement with no inbound pointer needs a notion of what counts as a requirement. An adopter that names its rules by id supplies exactly that notion. Its code declares each rule under an id matching a pattern, and its specs cite the ids. It asks for coverage both ways: every id the code declares is cited by exactly one spec, and every id a spec cites is declared. This amendment adds that check as a new canon-kit gate, off until a consumer declares its id patterns. It also rewrites the refusal sentence to point at the gate.

**What the tree does today**, read from `canon-kit/SPEC.md`, `native/src/gates/task_label_resolution.rs` and `native/src/knobs/canon_kit.rs`:

- **The reverse is refused in prose and nothing implements it.** §check-spec-pointer's calibration paragraph says the reverse *is ruled out and no syntax is reserved for it*. No gate reads a consumer's id pattern on code and spec together. `git grep -n -i "rule id\|rule-id" -- '*/SPEC.md' native/src` finds only a SARIF result's rule id and guard-kit's citation grammar for its own rule names, a kit-internal vocabulary.
- **A one-way analog exists.** `check-task-label-resolution` resolves each label a task list cites against a define pattern in the files beside it. It is armed by keyed knobs, a family per key, each value a one-group ERE. The table validator refuses unequal key sets and a pattern outside the one-group shape at exit 2 (`validate` in `native/src/knobs/canon_kit.rs`), and the gate skips fences and honours a `task-label-exempt:` valve in the shared exempt window.
- **The refusal was about a general notion, not about ids.** A heading is not a requirement, and a sentence is not either, so no kit-wide reverse rule holds the false-positive floor. A consumer's declared id pattern is a different input. It names exactly the tokens the consumer counts as requirements, so a reverse red over it is never a guess.

## What changes

### (1) A new gate, `check-rule-id-coverage` {design-bearing} {user-facing: operator direction 2026-11-24, lead-relayed (not a ruling), on the set record and the paired entry's deliverable: a bidirectional id-coverage check whose requirement notion is a consumer-declared pattern, off by default}

A new section, `### check-rule-id-coverage`, placed after §check-task-label-resolution. **Not yet applied:**

> Invariant: every rule id the code declares is cited by exactly one spec file, and every rule id a spec cites is declared. A consumer that names its rules by id gives this gate its notion of a requirement, which §check-spec-pointer's forward-only calibration has no general way to supply.
>
> **The two sides.** The **declaring files** are the files `CANON_KIT_RULE_ID_DECLARE_GLOBS` matches, and the **spec files** are those `CANON_KIT_RULE_ID_SPEC_GLOBS` matches, each walked through the pruning walk. A **rule family** is a key of `CANON_KIT_RULE_ID_DECLARES` and of `CANON_KIT_RULE_ID_CITES`. Each value is an ERE holding exactly one group, the one-group capture shape gate-sdk/SPEC.md §The POSIX ERE matcher admits. The group's text is the **id**. A line of a declaring file matching the family's declare pattern **declares** the id, and a line of a spec file matching its cite pattern **cites** it. Each line is matched whole, each leftmost-longest match found in turn, as in §check-task-label-resolution. Fenced code is skipped in a markdown file on either side, since a quoted example neither declares nor cites. A file matched by both glob sets is read as both.
>
> **What reds.** Per family, three findings, each naming the family and the id:
>
> - **uncited**: a declared id that no spec file cites, naming the file and line of its first declaration;
> - **cited twice**: a declared id cited by two or more spec files, naming each file and its first citing line, where `CANON_KIT_RULE_ID_HOMES` is `one`, the default;
> - **undeclared**: a cited id that no declaring file declares, naming the citing file and line.
>
> Several citations inside one spec file count as one home. Under `CANON_KIT_RULE_ID_HOMES=any`, a declared id needs at least one citing file, and the cited-twice finding never fires.
>
> **Valve:** `rule-id-exempt: <reason>`, in the shared exempt window (§The shared spec adapters), reason mandatory. On a declaring line it exempts that declaration from *uncited* and *cited twice*. On a citing line it exempts that citation from *undeclared*, and leaves it uncounted as a home.
>
> **Configuration.** All four knobs default empty, and an empty `CANON_KIT_RULE_ID_DECLARES` is the clean skip, whose clean line says no rule family is configured. Set families with either glob set empty are malformed config, and so are unequal key sets in the two keyed knobs, a pattern outside the one-group shape, or a `CANON_KIT_RULE_ID_HOMES` value other than `one` or `any`. The table validator refuses each at exit 2, naming the knob and the family (§Layout and configuration). The crate's unit tests hold that path, which a `bad/` case held to exit 1 cannot express. An unreadable file exits 2.
>
> Declared by `checks/check-rule-id-coverage.gate`, `# install: zero-config`, `# armed-by: CANON_KIT_RULE_ID_DECLARES`, dispatching to the binary, with `knob:CANON_KIT_RULE_ID_DECLARE_GLOBS` and `knob:CANON_KIT_RULE_ID_SPEC_GLOBS` in `couples=`. The pair: `good/` holds a family whose every declared id is cited once, an id cited twice within one spec file, a fenced citation of an undeclared id, an exempted uncited declaration, and a second family on the same files. `bad/` holds one finding of each kind. Tier `precommit`.

- **A new gate, not a mode of `check-spec-pointer`.** The entry leaves that choice here. `check-spec-pointer` is default-on over a fixed directive grammar, and its subject is a pointer's resolution. This gate's subject is a token set both sides must agree on, and it asserts nothing until a consumer declares its pattern. One gate holding both would be a default-on gate carrying an off-by-default arm, which §The install disposition's arming rule says takes its own declaration. It would also print one clean line for two subjects. The analog is `check-task-label-resolution`, a separate gate over a consumer's keyed one-group patterns, and this gate reuses that gate's validator shape and matching rule.
- **Why `one` is the default home count.** The asker's rule is *named in exactly one spec*. That is this project's content-tiering rule applied to ids: an id cited from two specs has two homes, and the two can drift. `any` is the selectable alternative, for a consumer whose overview spec cites ids that also have a detailed home (doctrine-kit/DOCTRINE.md, Policy-as-choice). The gate itself stays off by default, so neither value reaches a tree that has set no family.
- **Why the cite side is any file the glob names, not markdown only.** A consumer may keep its specs in another format. Fence skipping applies to markdown files alone, because only markdown has fences this gate knows.
- **Install disposition.** `zero-config`, because its subject is a knob a consumer sets, the second condition of §The install disposition, and it registers clean-skipped. Its `# armed-by:` declaration makes `doctor` name it while it is disarmed.

### (2) The refusal sentence points at the gate {mechanical} {user-facing: the paired entry's deliverable on this iteration's set record (operator direction 2026-11-24, lead-relayed, not a ruling): rewrite the refusal sentence either way}

Replacement text for the opening of §check-spec-pointer's calibration paragraph. **Not yet applied:**

> Calibration: forward direction only. The reverse, flagging a requirement with no inbound pointer, needs a notion of what counts as a requirement. A heading or a sentence is not one, so no reverse rule over this gate's corpus holds the cheap-and-false-positive-free bar. A consumer that names its rules by an id pattern supplies the notion itself, and §check-rule-id-coverage checks both directions over those ids. This gate reserves no syntax for the reverse.

The rest of the paragraph, from *Nor does the gate judge ownership*, is unchanged.

## Producers and consumers

- **`CANON_KIT_RULE_ID_DECLARE_GLOBS` and `CANON_KIT_RULE_ID_SPEC_GLOBS`** (new indexed knobs) — producer: the consumer's knob file, default empty. Consumers: the gate's two walks and the validator's emptiness check.
- **`CANON_KIT_RULE_ID_DECLARES` and `CANON_KIT_RULE_ID_CITES`** (new keyed knobs) — producer: the consumer's knob file, default empty. Consumers: the table validator, which pairs their key sets and checks each pattern's shape at every resolution, and the gate's matcher. `doctor` reads `CANON_KIT_RULE_ID_DECLARES` through `# armed-by:` (installer/SPEC.md §doctor).
- **`CANON_KIT_RULE_ID_HOMES`** (new scalar knob) — producer: the knob file or the environment, default `one`. Consumers: the validator, and the gate's cited-twice finding.
- **The three findings** (new events) — producer: the gate. Consumer: the battery and the pre-commit hook through exit 1, and the SARIF log through the runner.
- **The `rule-id-exempt:` valve** (new marker) — producer: a consumer's declaring or spec file. Consumers: the gate, through the shared exempt-window reader, and `check-comment-tier`. On a declaring line the valve is usually a code comment, and that gate blesses a site valve only by name, from its built-in roster (§check-comment-tier's site-valve bullet, and `native/src/gates/comment_tier.rs`). So `rule-id-exempt:` joins both, or a valved code comment would red the tier gate.
- **The registry row** (new member) — producer: `native/src/gates/mod.rs`. Consumers: the direct form, `--run`, `--needs`, the `.gate` descriptor's dispatch, and `check-graph`.
- **Point 5 (narrowing).** Nothing narrows by default. A tree with no rule family is clean-skipped. Under a configured family the gate reds what it names, and the delta 2 rewrite narrows no gate.
- **Point 6 (every member).** The members are every reader of the refusal sentence's claim that the reverse *is ruled out*. Its one site is §check-spec-pointer's calibration paragraph and its mirror (probe: `git grep -n "no syntax is reserved" -- ':!docs/posts'`). §check-tracking-claim cites *`check-spec-pointer`'s forward-only bar* for its own calibration, and it stays true, since `check-spec-pointer` is still forward-only.

## Existing sections updated

Roster from `git grep -n -e "check-task-label-resolution" -e "no syntax is reserved" -e "forward-only bar" -e "gh-repo-exempt"` over the tracked tree, the generated mirrors, the posts and the queue excluded. The analog's sites are the sites every new canon-kit gate lands on.

- `canon-kit/SPEC.md` §Layout and configuration gains the five knobs and the validator's refusals. The new §check-rule-id-coverage lands, with §check-spec-pointer's calibration opening (all deltas).
- `docs/canon-kit/SPEC.md` — the generated mirror, regenerated by `--emit docs-mirror --write` (all deltas).
- `canon-kit/SPEC.md` §check-comment-tier's site-valve bullet gains `rule-id-exempt: <reason>` (§check-rule-id-coverage), and `native/src/gates/comment_tier.rs`'s built-in site-valve roster gains `rule-id-exempt:` (delta 1).
- `canon-kit/README.md` and its mirror `docs/canon-kit/README.md` — a gate-roster line beside `check-task-label-resolution`'s, *needs rule families (a declared id is cited once and a cited id is declared)* (delta 1).
- `canon-kit/checks/check-rule-id-coverage.gate` — new descriptor (delta 1).
- `canon-kit/gate-tests/check-rule-id-coverage/good/` and `bad/` — new pair, each with a fixture `scripts/canon-config.knobs` binding a family (delta 1).
- `canon-kit/smoke/install.sh` — the gate joins the registration list beside `check-task-label-resolution` (delta 1).
- `native/src/gates/rule_id_coverage.rs` — new module. `native/src/gates/mod.rs` gains its `pub mod` and registry row (delta 1).
- `native/src/knobs/canon_kit.rs` — two indexed rows, two keyed rows and one scalar row defaulting `one`. `validate` gains the pairing, shape, emptiness and value refusals, each with a unit case (delta 1).
- `scripts/gates.list` — registered, as `check-task-label-resolution` is, under the dogfooding rule. It reads clean-skipped here, since this tree declares no rule family (delta 1).
- Generated projections the new member moves: `docs/enforcement.md`, `docs/check-graph.html`, `docs/llms-full.txt`, and any roster table holding canon-kit's gates. Each is regenerated by the command its freshness gate prints on red (docs/generated-projections.md) (delta 1).
- `.workflow/release-declarations.md` — *Knob changes* gains the five knobs, and *Behavior changes* gains the gate (delta 1).

## Retired spellings

- None — the gate and its knobs are additions. Delta 2 re-phrases a sentence whose claim narrows, and removes no name.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the five knobs, the three findings, the valve and the registry row.
- [ ] **Instruction surfaces: instruction only** — no template, agent definition or shim is touched.
- [ ] **Merged with no information lost** — the new section lands whole and the calibration rewrite re-phrases the passage it refines; the merged spec reads as one document.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls canon-kit/SPEC-*.md`).
- [ ] **Removals propagated** — none declared; `check-amendment-retired-spelling` stays clean.
- [ ] **Entry moved** — `rule-id-coverage` to Done with `--queue done` in the build batch's merge commit, before the drain stage. Its oracle is local, the canon-kit fixture suite, the smoke and the battery, so no remote run gates the move.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
