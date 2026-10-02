# SPEC amendment: front-door-reach

**`check-front-door-verbs` invariant B reaches two advertisements it reads past today: the value after a `--profile` or `--recipe`, and a gate-binary arm a page names outside a route.** Both are held to the pinned release under the pending admission invariant B already applies to a verb and a flag (installer/SPEC.md §The front door's verbs). It pairs two queue entries that widen one invariant in one batch: `front-door-flag-operand` (deltas 2 and 4) and `front-door-arm-route-blind` (deltas 3 and 4).

**Measured at authoring**, against the hosted pin `0.31.0` (`docs/install.sh`'s `pin` line) and its tag:

- **Operands.** The front-door pages advertise `--profile prose`, `--profile full`, `--recipe speckit`, `--recipe openspec` and `--recipe openspec-lifecycle` after a route, and `--profile <profile>` and `--recipe <name>` as placeholders (`grep -ohE -- '--(profile|recipe)[ =][a-z0-9<][a-z0-9>-]*'` over the seven pages). The tag's `installer/profiles.list` rows name `starter`, `delegation` and `prose`, `full` is the profile module's derived name (`native/src/installer/profile.rs`, `DERIVED`), and the tag's `scripts/gate-sdk-config.knobs` keys `GATE_SDK_PAYLOAD_RECIPES` by `openspec`, `openspec-lifecycle` and `speckit`. Every advertised operand is in the pinned sets, so the widening reds no page today.
- **Arms.** Code spans opening with `--` on the pages are installer flags, except six: docs/install.md's `--emit env-probe`, `--measure-commit`, `--run` and `--usage-poll`, and README.md's `--run-demo` and `--install-hooks` (`grep -on '`-[^`]*`'` over the seven pages). README.md's two sit under its `## This repo, governed` section, which describes a checkout, whose binary is built from the tree and not fetched from the pin. The tag's `ARMS` table in `native/src/emit/mod.rs` carries all four of docs/install.md's arms (`--emit-env-probe`, `--measure-commit`, `--run`, `--usage-poll`), so this widening also reds no page today.
- **Where the tag's arm set lives.** No file a release carries lists the gate binary's arms as a table. The arms are the first string literal of each top-level tuple of `ARMS` in `native/src/emit/mod.rs`, and the top-level flags are `TOP_LEVEL_FLAGS` in `native/src/main.rs`; both arrays exist at `v0.31.0`. A flat scan of every `"--…"` literal in `ARMS` over-reads: at HEAD the array holds 86 tuples and 93 such literals, the surplus being flag literals inside a `Grammar::Flags` value.

## What changes

### (1) The positional form takes two trees {mechanical}

The gate reads HEAD's files and the tag's files by repo-relative path through one reader per file, a path on disk for HEAD and `git show v<pin>:<path>` for the tag, so a roster read off either side is one parse. The positional form becomes:

```
check-front-door-verbs [head pinned disposition queue page...]
```

`head` and `pinned` are directories standing for HEAD's tree and the tag's, each read at the same repo-relative paths the bare form reads: `installer/README.md`, `installer/profiles.list` and the gate-sdk knob seam file on both sides, and `native/src/emit/mod.rs` and `native/src/main.rs` on the pinned side. The bare form resolves the seam file's directory from `GATE_SDK_GATES_DIR`, which joins the gate's declared knob roster in `native/src/gates/mod.rs`. The binary's own `VERBS`, `FLAGS` and arm table stay read in process. The descriptor's graph-manifest `couples=` list gains `installer/profiles.list` and the gate-sdk knob seam file, the HEAD-side files the operand read adds, and its `# spec:` line names operands and arms beside verbs and flags.

The fixture pair's `pinned.md` moves to `pinned/installer/README.md`, its `readme.md` to `head/installer/README.md`, and each `args` file is rewritten to the new form. **Not yet applied.**

### (2) Invariant B reads the operand of `--profile` and `--recipe` {design-bearing}

After an advertised flag that takes a value, the value is an **advertised operand** of that flag: the part after the flag's `=` when the token carries one, else the next token, unless that token is a stop token, opens with `-` or `#`, or is past the walk's end. An operand is read only when it matches `[a-z0-9][a-z0-9-]*`, so a placeholder such as `<profile>` advertises none. Two flags take a roster-checked operand:

- `--profile`, against the **profile set**: the first column of `installer/profiles.list`'s rows, comments and blanks dropped, plus `full`, the name the profile module derives rather than lists (§Profiles).
- `--recipe`, against the **recipe set**: the keys of the `GATE_SDK_PAYLOAD_RECIPES[<name>]` lines in the gate-sdk knob seam file, the set the packer packs (§The packer).

Each set is read at HEAD and at the tag by the same reader. An operand in the tag's set passes. One in HEAD's set and not the tag's takes the pending admission exactly as a flag does: admitted while this iteration's disposition line is absent or names a release, red under `none` or `deferred:`. One in neither set is never admitted, since no release will carry it. A tag whose `installer/profiles.list` or seam file is absent has the empty set for that flag, the ground §The front door's verbs gives for a tag without a flag table. The kit and gate names after `--with-kit`, `--without-kit`, `--with-gate` and `--without-gate` stay unread. **Not yet applied.**

### (3) Invariant B reads a gate-binary arm named outside a route {design-bearing}

An inline code span on a front-door page whose text's first token opens with `--` is an **advertised arm**, unless that token is a flag of HEAD's `FLAGS` or the pinned flag table, which stays a flag named apart from its route. `--emit <name>` reads as the arm `--emit-<name>`, the normalization the binary applies to that spelling. README.md's `## This repo, governed` section is not read for arms, since its arms run on a binary the checkout builds. Fenced code lines are not read for arms.

The **pinned arm set** is the tag's arm table: the first string literal of each top-level tuple of `ARMS` in `native/src/emit/mod.rs`, with every literal of `TOP_LEVEL_FLAGS` in `native/src/main.rs`. The binary's arm set is `emit::arms()` with `TOP_LEVEL_FLAGS`, read in process. An arm in the pinned set passes. One the binary carries and the pin lacks takes the pending admission, and one in neither set is never admitted. A tag carrying either file with its array absent or yielding no member fails the gate closed, exit 2, since an empty arm set there is a parse that went wrong rather than a release with no arms. A unit test holds the extractor to the binary: applied to the crate's own `native/src/emit/mod.rs` and `native/src/main.rs` text, it yields exactly the in-process set, so a change to the arrays' shape reds in the commit that makes it. **Not yet applied.**

### (4) installer/SPEC.md §The front door's verbs states the widened invariant {design-bearing}

The section is rewritten as below. **Not yet applied.**

The opening sentence becomes:

> Every route on the front door resolves to the newest release, so a verb the front door advertises, each flag after it, the value a `--profile` or `--recipe` names, and a gate-binary arm a page names must be ones that release carries. `check-front-door-verbs` (this repo's `scripts/`) holds two invariants:

Invariant B's bullet gains a final sentence:

> Each advertised operand must be in the pinned profile or recipe set, and each advertised arm in the pinned arm set (below).

After the **Flags.** paragraph, two paragraphs:

> **Operands.** After an advertised `--profile` or `--recipe`, the value after its `=`, or else the next token unless it is a stop token or opens with `-` or `#`, is an advertised operand when it matches `[a-z0-9][a-z0-9-]*`, so a placeholder advertises none. A `--profile` operand is held to the profile set, the first column of `installer/profiles.list` plus the derived `full` (§Profiles). A `--recipe` operand is held to the recipe set, the keys of `GATE_SDK_PAYLOAD_RECIPES` in the gate-sdk knob seam file (§The packer). Each set is read at HEAD and at the tag by one reader, and a tag lacking the file has the empty set.
>
> **Arms.** An inline code span whose first token opens with `--` and is no flag of `FLAGS` or the pinned flag table is an advertised gate-binary arm, `--emit <name>` reading as `--emit-<name>`. README.md's `## This repo, governed` section is not read for arms: it describes a checkout, whose binary is built from the tree. The pinned arm set is the tag's `ARMS` table in `native/src/emit/mod.rs`, the first string literal of each top-level tuple, with `TOP_LEVEL_FLAGS` in `native/src/main.rs`. No file a release carries holds the arms as a table, so the source arrays are the record, and a unit test holds the extractor equal to the binary's own set over the crate's source, so a reshaped array reds in the commit that reshapes it.

**The pending admission.**'s first sentence becomes:

> A verb, flag, operand or arm B would red is admitted while HEAD carries it — the verb in `VERBS`, the pair in `FLAGS`, the operand in HEAD's profile or recipe set, the arm in the binary's arm table — and `.workflow/release-disposition.txt` carries no line for the iteration the queue header names.

and every later *verb or flag* in that paragraph reads *advertisement*.

The fail-closed paragraph gains, after its tag sentence:

> It fails closed too on a tag whose `native/src/emit/mod.rs` or `native/src/main.rs` carries no `ARMS` or `TOP_LEVEL_FLAGS` array or yields no member from one.

and its positional sentence becomes:

> Its positional form, `check-front-door-verbs [head pinned disposition queue page...]`, points it at a fixture tree: `head` and `pinned` are directories read at the paths the bare form reads at HEAD and at the tag, so the fixture holds still as the tags move.

**Honest limits.** is rewritten:

> **Honest limits.** A verb named in a code span apart from its route, as in "or `demo`", is out of reach, and so is a flag named apart from its route, as in "pass `--recipe`". A kit or gate name after a selection flag is unread. An arm in a fenced command line, or reached through a named front-end such as `run-gates.sh`, is out of reach, and a renamed checkout heading in README.md widens the arm read to that section's arms rather than narrowing it. The gate holds the release decision, not the live site: between a push carrying a new advertisement and the close's tag, the site advertises something the one-liner cannot run. The pin moves in the commit after the tag (RELEASING.md step 4), so the live one-liner reaches the new release on the push carrying that commit.

### (5) The fixture pair and the module's tests cover both reads {mechanical}

`scripts/gate-tests/check-front-door-verbs/`, in the tree form delta 1 gives it. **Not yet applied.**

- `good/` gains a pinned `installer/profiles.list`, a pinned and a HEAD seam file, and a pinned `native/src/emit/mod.rs` and `native/src/main.rs` holding a small `ARMS` and `TOP_LEVEL_FLAGS` array, one tuple of which carries a `Grammar::Flags` literal the extractor must skip. Its pages advertise a pinned operand of each flag, a pinned arm in the `--emit <name>` spelling and the bare one, an operand HEAD's set carries and the pin lacks, and an arm `emit::arms()` carries that the pinned array lacks, both admitted under the fixture's absent disposition and listed on the clean line. The clean line names its operand and arm counts and any pending operands and arms.
- `bad/` gains, under its `deferred:` disposition, a pending operand of each flag and a pending arm, each red, and an operand and an arm in neither set, red under any disposition. Its expectation lists each finding line.
- The module's unit tests pin the operand walk (`=` and next-token forms, a placeholder, a stop token, a following flag), the arm read (the `--emit` normalization, an installer flag skipped, the excluded section), and the extractor's equality with the in-process arm set over the crate's own two source files.

## Producers and consumers

- **The operand and arm findings.** Producer: `check-front-door-verbs`, at pre-commit on any coupled file and in the battery, reading the pages, HEAD's `installer/profiles.list` and seam file in the tree, the tag's copies of those two and of `native/src/emit/mod.rs` and `native/src/main.rs` through `git show`, and the binary's `FLAGS` and arm table in process. Consumers: the committer, through the finding lines and their `help:` remedies (release, or withdraw the advertisement), and the close binding's front-door step in `.claude/commands/close.md`, which acts on the same red at the release disposition.
- **The pending operands and arms on the clean line.** Producer: the same gate. Consumer: the close session reading the battery before its release disposition, as it reads the pending verbs and flags today. Each count names how many sites the read reached, which is what tells a clean run from a read that reached nothing.
- **The extractor's equality test.** Producer: the crate's test run (`check-crate-arms`). Consumer: the committer whose change to `ARMS` or `TOP_LEVEL_FLAGS` changed their shape. Red condition: the extracted set differs from `emit::arms()` with `TOP_LEVEL_FLAGS`.
- **Red conditions of the widened corpus (point 5).** The corpus only widens. The gate reds on finding a violation, never on finding none, and asserts no count or floor over the new reads, so no existing reader gains a red from the widening. The new reds are an operand or arm the pinned release lacks under a withheld disposition, one in neither set, and a tag whose arm arrays do not parse.
- **Every member's satisfying value (point 6), probed above.** The operand sites `prose`, `full`, `speckit`, `openspec` and `openspec-lifecycle` are each in `v0.31.0`'s set. The arm sites `--emit env-probe`, `--measure-commit`, `--run` and `--usage-poll` are each in `v0.31.0`'s `ARMS`. README.md's two arm spans are outside the read.

## Existing sections updated

Probe for the reader roster: `git grep -n "front-door-verbs\|pinned-readme"` over the tracked tree less `docs/posts` and `TASK-QUEUE.md`, and `git grep -n "check-front-door-verbs"` in `.claude/`.

- `installer/SPEC.md` §The front door's verbs — the opening sentence, invariant B, the Operands and Arms paragraphs, the pending admission, the fail-closed paragraph and the honest limits (deltas 2, 3 and 4).
- `native/src/gates/front_door_verbs.rs` — the tree readers and positional form, the operand walk, the arm read and extractor, the findings, the clean line and the unit tests (all deltas).
- `native/src/gates/mod.rs` — `GATE_SDK_GATES_DIR` joins the gate's declared knobs (delta 1).
- `scripts/check-front-door-verbs.gate` — the `couples=` additions and the `# spec:` line (deltas 1, 2 and 3).
- `scripts/gate-tests/check-front-door-verbs/` — the tree form and the new cases (deltas 1 and 5).
- `.claude/commands/close.md` — the front-door bullet names an operand or arm beside a verb or flag (deltas 2 and 3).
- `docs/site-architecture.md` §Generated projections and their freshness gates, the hosted-install-scripts row — `check-front-door-verbs` reads the pin as the tag whose verb and flag tables, rosters and arm table the front door is held to (deltas 2 and 3).
- `docs/check-graph.html` — regenerated for the descriptor's new couples (delta 1).
- `docs/installer/SPEC.md` — the generated mirror, regenerated (all deltas).

## Retired spellings

- `pinned-readme` — the positional operand naming one pinned README file, replaced by the `pinned` tree (delta 1).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — the close binding's bullet carries no grounds.
- [ ] **Merged with no information lost** — §The front door's verbs reads as one section.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green over the declaration above.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Both entries done** — `front-door-flag-operand` and `front-door-arm-route-blind` move to Done in the merging commit, before the drain stage: the gate reads the pinned tag from local git, so no remote run is their oracle.
