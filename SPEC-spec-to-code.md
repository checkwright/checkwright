# SPEC amendment: spec-to-code

The companion checks a toolkit's specs as documents and never against the code (docs/spec-toolkits.md §What the gates catch: *None of them checks that your code does what a spec says*). This amendment adds the two spec-to-code checks the pinned toolkits' formats make possible, as generic canon-kit gates, and binds them to each toolkit's layout in its recipe:

- `check-task-path-claim`: a ticked task in a task list names only paths that exist. A ticked task is a done claim, and a path it names that is not in the tree is the "phantom completion" the done-claim failure class describes.
- `check-task-label-resolution`: a label a task list cites resolves to its definition in a document beside it, such as a user-story label naming a story the spec defines.

One queue entry pairs it: [companion-spec-to-code-gates](TASK-QUEUE.md#companion-spec-to-code-gates).

**The probe.** The pinned toolkits were installed and run (survey record, *Which spec-to-code checks do the pinned Spec Kit and OpenSpec formats make possible*):

- **Spec Kit.** A task line reads `- [ ] T005 [P] [US1] <description> in src/<path>`. The path is bare text after *in*, some tasks carry none, and the implement command ticks `[X]` while other surfaces write `[x]`. The `[USn]` label is the template's one traceability convention: it maps a task to `### User Story n - …` in the `spec.md` beside it. Tasks cite requirement IDs (`FR-…`, `SC-…`) only in the optional converge phase. `plan.md`'s source layout is a `text` fence drawing a tree. `analyze` is agent-run prose, and the one script it runs checks that three files exist.
- **OpenSpec.** A task line reads `- [ ] 1.1 <text>`, free text with no paths or IDs by convention. `validate --strict` exits 0 on a MODIFIED or RENAMED delta naming an absent requirement and on an ADDED one naming an existing requirement, printing an INFO line, and on a REMOVED one naming an absent requirement, printing nothing. `archive` refuses the first three and archives the fourth as already removed.

**The rulings.**

- **Two gates, and the toolkit keeps what it owns.** OpenSpec owns delta-to-base agreement: archive refuses every disagreement it treats as one, and tolerates an absent REMOVED by design. companion/SPEC.md §The two tiers rules that neither tier re-checks what a toolkit owns, so no gate here re-implements that check. A commit-time gate for it is filed as [openspec-delta-base-agreement](TASK-QUEUE.md#openspec-delta-base-agreement), on an operator direction lead-relayed (not a /consult ruling), with that conflict as the question it owes. `plan.md`'s tree is ruled out: it draws the structure a feature will have, so a gate would red every plan written before its implementation.
- **The mechanism is generic, and the binding is the recipe's.** Neither gate names a toolkit or a file name. A task list is whatever `CANON_KIT_TASK_LIST_GLOBS` names, and a label family is a pair of one-group EREs. Each recipe binds its toolkit's layout (gate-sdk/SPEC.md §The provenance seam).
- **Both tiers carry them.** Each gate is a canon-kit member declared `zero-config` with `# armed-by:`, on `check-citation-link`'s precedent: it asserts nothing until its knob is set, so it registers at install and a recipe line arms it. The Spec Kit extension's `prose` install and both toolkits' `full` lines therefore run them.
- **One label family is bound.** The Spec Kit recipe binds the user-story family, which every story-phase task carries. A converge task's `per FR-003` is not bound, since each claimed gate has one planted defect and the family would be untested. An adopter adds that family with two knob lines.
- **Existence, not behaviour.** A path that exists says nothing about what the file does, and the pages say so.

## What changes

### (1) `check-task-path-claim` {design-bearing}

**Not yet applied.** canon-kit/SPEC.md gains a section after §check-tracking-claim:

> ### check-task-path-claim
>
> Invariant: every ticked task in a configured task list names only paths that exist. A **task list** is a markdown file `CANON_KIT_TASK_LIST_GLOBS` matches, walked through the pruning walk. A **ticked task** is a list item whose marker, `-`, `*` or `+`, is followed by `[x]` or `[X]`. Its text is the item's first line and its indented continuation lines. Fenced code blocks are skipped, since a quoted example is not a claim. An unticked task claims nothing and is not read.
>
> A **path token** is a maximal run of `[A-Za-z0-9._/@+-]` in the task's text, inline code spans included, with a trailing `.`, `,`, `;` or `:` dropped. It must hold a `/` that is not its first character, and either end in `/` or carry a final segment with an extension, `.` then one or more alphanumerics. A token is not a path when it opens with `/` or `~`, holds `://`, `//` or a `..` segment, or touches `[`, `{`, `<` or `*` on either side, since that is a placeholder. A path token resolves from the repository root, and it holds when a file or directory is there.
>
> Reddens on each path token of a ticked task that does not hold, naming the file, the line, the token and the task's text. The `help:` names the three remedies: restore the path, correct the task, or untick it. **Valve:** `task-path-exempt: <reason>` on the task's line or the one above, in the shared exempt window (§The shared spec adapters), for a task whose path is absent by design, such as one that deleted it. The reason is mandatory.
>
> Declared by `checks/check-task-path-claim.gate`, `# install: zero-config`, `# armed-by: CANON_KIT_TASK_LIST_GLOBS`, dispatching to the binary. Its `couples=` carries `knob:CANON_KIT_TASK_LIST_GLOBS` for the list and `*` for the tree its tokens resolve against. An empty knob is the clean skip, and its clean line says no task list is configured.

`native/src/gates/task_path_claim.rs` implements it and registers in `native/src/gates/mod.rs`. `canon-kit/gate-tests/check-task-path-claim/` holds the pair. `good/` has a ticked task naming an existing file and an existing directory, an unticked task and a fenced example naming absent ones, a placeholder token, and an exempted task. `bad/` has a ticked task, in each marker case, naming an absent file.

### (2) `check-task-label-resolution` {design-bearing}

**Not yet applied.** canon-kit/SPEC.md gains a section after delta 1's:

> ### check-task-label-resolution
>
> Invariant: every label a task list cites resolves to a definition in a markdown file beside it. A **label family** is a key of `CANON_KIT_TASK_LABEL_CITES` and of `CANON_KIT_TASK_LABEL_DEFINES`. Each value is an ERE holding exactly one group, the one-group capture shape gate-sdk/SPEC.md §The POSIX ERE matcher admits. The group's text is the **label**. Across each task list, fences skipped, every match of a family's cite pattern cites its label. The line is matched whole: each leftmost-longest match is found in turn, and the group is captured within that match's span. The label resolves when some line of a `*.md` file in the task list's own directory, the task list included, matches the family's define pattern with the same group text.
>
> Reddens on each unresolved citation, naming the file, the line, the family and the label. **Valve:** `task-label-exempt: <reason>`, in the shared exempt window. Exit 2, naming the knob and family, when the two knobs' key sets differ or a pattern does not compile to the one-group shape.
>
> Declared by `checks/check-task-label-resolution.gate`, `# install: zero-config`, `# armed-by: CANON_KIT_TASK_LABEL_CITES`, dispatching to the binary, with `knob:CANON_KIT_TASK_LIST_GLOBS` and `*.md` in `couples=`. It also asserts nothing while `CANON_KIT_TASK_LIST_GLOBS` is empty, which `check-task-path-claim`'s declaration already names to `doctor`.

`native/src/gates/task_label_resolution.rs` implements it over `EreCapture` and `find_from`, with no new engine item. `canon-kit/gate-tests/check-task-label-resolution/` holds the pair. `good/` has a family whose citations resolve in a sibling file, and a citation in a fence naming an undefined label. `bad/` has an unresolved citation, plus a knob pair whose key sets differ, which exits 2 in its own case.

### (3) The knobs {mechanical}

**Not yet applied.** canon-kit/SPEC.md §Layout and configuration's knob list gains:

> - `CANON_KIT_TASK_LIST_GLOBS` — array of globs, default empty: the task lists `check-task-path-claim` and `check-task-label-resolution` read. `CANON_KIT_TASK_LABEL_CITES` and `CANON_KIT_TASK_LABEL_DEFINES` — keyed, default empty: per label family, the one-group ERE that cites a label and the one that defines it (§check-task-label-resolution).

The corpus-knob clause in that section is derived from the walks, so it reaches the glob knob with no edit. `native/src/knobs/canon_kit.rs` gains the three rows. The table validator holds the two keyed knobs to one key set and each value to the one-group shape. `canon-kit/README.md`'s gate roster gains `check-task-path-claim # needs task lists (a ticked task names only paths that exist)` and `check-task-label-resolution # needs task lists and label families (a cited label resolves beside it)`. `canon-kit/smoke/install.sh` registers both.

### (4) The recipes arm them {mechanical}

**Not yet applied.** `companion/speckit/recipe/canon-config.knobs` gains:

```text
CANON_KIT_TASK_LIST_GLOBS[] = specs/*/tasks.md
CANON_KIT_TASK_LABEL_CITES[story] = \[US([0-9]+)\]
CANON_KIT_TASK_LABEL_DEFINES[story] = ^### User Story ([0-9]+)[ ]
```

The define pattern's trailing space is a bracket, since the knob grammar trims a value's trailing blank. `companion/openspec/recipe/canon-config.knobs` gains `CANON_KIT_TASK_LIST_GLOBS[] = openspec/changes/*/tasks.md`, whose single `*` keeps `openspec/changes/archive/` out. An archived change's paths are history.

companion/SPEC.md §The Spec Kit recipe and §The OpenSpec recipe each gain a bullet per line, stating the idiom it arms: the task list's path, and for Spec Kit the `[USn]` label and the `### User Story n` heading it names.

### (5) The fixtures and the tested claim {design-bearing}

**Not yet applied.**

- **Fixtures.** Each toolkit's `layout/` gains one source file that is neither markdown nor a shell script, so no other claimed gate reads it, and a ticked task naming it: `T003` on Spec Kit and `1.1` on OpenSpec. `defects/check-task-path-claim/` on each toolkit replaces the task list with that task naming an absent path. `defects/check-task-label-resolution/` on Spec Kit replaces `tasks.md` with a task citing `[US2]`, which `spec.md` does not define. The OpenSpec layout still passes `openspec validate --all --strict`.
- **companion/SPEC.md §The tested claim** becomes a roster per toolkit. It keeps the four document classes for both, and adds *a ticked task naming a path that does not exist, by `check-task-path-claim`* for both, and *a task citing a user story its spec does not define, by `check-task-label-resolution`* for Spec Kit. The first sentence's *four defect classes per toolkit* becomes *the defect classes below*. The paragraph gains: *The toolkit keeps what it owns: OpenSpec's archive refuses a delta that disagrees with its base spec, so no gate re-checks it.*
- **The companion arm.** `installer/consumer-smoke/run-smoke.sh`'s `COMPANION_CLAIMED` gains `check-task-path-claim`, and a Spec Kit roster beside it carries `check-task-label-resolution`. The arm requires a `defects/<gate>/` directory for the shared roster and the toolkit's own. installer/SPEC.md §The consumer smoke's companion paragraph names the per-toolkit roster.

### (6) The pages {mechanical}

**Not yet applied.** `docs/spec-toolkits.md`:

- The line *These four are document hygiene: … None of them checks that your code does what a spec says.* is replaced by a section `## What the gates check against your code`:

  > Two gates read your task lists against the tree. A task you ticked that names a file or directory the repository does not have reds `check-task-path-claim`. On Spec Kit, a task labelled with a user story the spec beside it does not define, such as `[US3]` with no User Story 3, reds `check-task-label-resolution`. Both check that a path or a story exists, not that the code does what the task says.

- §What the gates catch's *four kinds of defect in your specs* stays, since it lists the document gates.
- §What is tested's *plants each of the four defects* becomes *plants each defect*.

`companion/speckit/extension.yml`'s description and `companion/speckit/README.md`'s gate sentence each add the ticked-task check, the description kept under the catalog's 200 characters. `docs/speckit.md` line 31's *to the four [the overview](spec-toolkits.md#what-the-gates-catch) names* becomes *to the ones [the overview](spec-toolkits.md) names*.

## Producers and consumers

Probe: the survey record's toolkit-format block (installs of both pins, `specify init`, OpenSpec validate and archive runs); `companion/fixtures/*/layout` read for task and story lines; `grep -n "COMPANION_CLAIMED"` over `installer/consumer-smoke/run-smoke.sh`; the `# armed-by:` census over `*/checks/*.gate`, which shows canon-kit's `check-citation-link` and `check-docs-page-repeat` as `zero-config` members armed by an empty glob knob; gate-sdk/SPEC.md §The POSIX ERE matcher for the capture shape.

- **The task-list knob** (delta 3). Producer: a recipe line, or the adopter's knob file. Consumers: both gates and `doctor`'s disarmed line, through `# armed-by:`.
- **The label families** (delta 3). Producer: the Spec Kit recipe. Consumer: `check-task-label-resolution`. Each field has a reader: the cite pattern at each task-list line, the define pattern at each sibling file's line. The validator reads both at the kit's first resolution.
- **The findings** (deltas 1 and 2). Consumer: the battery, on the pre-commit hook and in CI. The companion arm reads each red through its planted defect.
- **Red conditions for adopters.** A `prose` or `full` install gains two registered gates, both disarmed until a knob is set, so no existing tree reds. A tree applying a toolkit recipe from this release reds on a ticked task naming a missing path, and on Spec Kit on an undefined story label. A Spec Kit tree whose shipped features' tasks name files since moved reds there too. That is drift the gate exists to show, and the task list or the valve answers it.

## Existing sections updated

Roster probe: `git grep -n "check-tracking-claim"` over the tracked tree, for each place a canon-kit gate is rostered; `git grep -n "four defect\|four kinds\|the four\|COMPANION_CLAIMED"` over `companion`, `docs`, `installer`.

- `canon-kit/SPEC.md` — §check-task-path-claim and §check-task-label-resolution (new, deltas 1 and 2); §Layout and configuration (delta 3).
- `canon-kit/checks/check-task-path-claim.gate`, `canon-kit/checks/check-task-label-resolution.gate`, `canon-kit/gate-tests/` pairs and suites (deltas 1 and 2).
- `native/src/gates/task_path_claim.rs`, `native/src/gates/task_label_resolution.rs`, `native/src/gates/mod.rs` (deltas 1 and 2); `native/src/knobs/canon_kit.rs` (delta 3).
- `canon-kit/README.md`, `canon-kit/smoke/install.sh` (delta 3).
- `scripts/gates.list` — registers both, disarmed on this tree (deltas 1 and 2).
- `companion/speckit/recipe/canon-config.knobs`, `companion/openspec/recipe/canon-config.knobs` (delta 4).
- `companion/SPEC.md` — §The Spec Kit recipe, §The OpenSpec recipe (delta 4), §The tested claim (delta 5).
- `companion/fixtures/speckit/` and `companion/fixtures/openspec/` layouts and defects (delta 5).
- `installer/consumer-smoke/run-smoke.sh`, `installer/SPEC.md` §The consumer smoke (delta 5).
- `docs/spec-toolkits.md`, `docs/speckit.md`, `companion/speckit/extension.yml`, `companion/speckit/README.md` (delta 6).
- `docs/enforcement.md` and `docs/check-graph.html`, the generated gate projections, each regenerated by the command its freshness gate prints (deltas 1 and 2).
- `docs/canon-kit/SPEC.md`, `docs/canon-kit/README.md`, `docs/companion/SPEC.md`, `docs/installer/SPEC.md`, the generated mirrors, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).
- `.workflow/surface-ceiling.txt` — the grown `*/SPEC.md` and `docs/*.md` rows re-stamped with `--emit always-loaded --ceiling` in the growing commit (deltas 1, 2, 3, 5 and 6).
- `.workflow/release-declarations.md` — Tightened gates gains `check-task-path-claim` and `check-task-label-resolution`, each new and disarmed until its knob is set. Behavior changes gains a bullet led by **the Spec Kit and OpenSpec recipes**, which arm them (deltas 1, 2 and 4).

## Retired spellings

- None — the deltas add two gates, three knobs, recipe lines, fixtures and a smoke roster, and re-phrase prose; no name, path or token is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Battery green** — the full battery, canon-kit's fixture suite, `cargo test` and the installer consumer smoke green on the landing commit, and the gates workflow green on the mid-iteration push, whose Windows and macOS legs run the new native gates. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`), once that push's runs are green.
