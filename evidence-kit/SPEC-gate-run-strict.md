# SPEC amendment: gate-run-strict

Pairs `verify-gate-run-strictness`, and rules the question `evidence-blank-semantics` leaves open. `--verify-gate-run` is the gate-run predicate's reader, and an adopter reading it as a merge predicate meets three gaps. A Statement of an older commit verifies, so the exit says nothing about `HEAD`. A suite whose verdict is `new-failures` verifies. And a tree holding untracked files reads `clean`. Per the entry's deliverable, each becomes a selection a consumer makes, off among them. The first two are words of one knob and the third is a knob of its own. The same reader admits a whitespace-only provenance string that gate-sdk's provenance block says is never blank, so this amendment also rules what blank means.

**What the tree does today**, read from `native/src/emit/verify_gate_run.rs`, `native/src/emit/run_validate.rs`, `native/src/provenance.rs`, `native/src/evidence.rs` and `docs/contracts/gate-run/v1/schema.json`:

- **The subject's distance is printed, never judged.** `judge` counts `HEAD`'s commits ahead of the subject into `ahead`, and `report` prints it on the verdict line. No finding reads it, so a subject any number of commits behind `HEAD` exits 0.
- **A suite's verdict is never judged.** `field_findings` reads `verdict` only as an input to `binding_digest`, so a consistent `new-failures` suite exits 0.
- **The tree read excludes untracked files.** `head_state` takes `tree=` from `provenance::worktree_clean`, which runs `git status --porcelain -z --untracked-files=no`. The verify arm's check 5 reds a suite whose `tree` is `modified`, but an untracked file never makes it so.
- **Blank is zero characters.** `provenance_shape` refuses an empty `name`, `root` or optional string by `str::is_empty`, the schema holds each with `minLength: 1`, and the producer's `present` filters by `is_empty`. So `" \t"` passes all three.
- **Neither contract has shipped.** The last release tag holds neither: `git show v0.36.0:docs/contracts/gate-run/v1/schema.json` names no such path, and `git show v0.36.0:gate-sdk/SPEC.md | grep -c verdictContract` counts 0, so the gate-run predicate `1.0.0` and the verdict contract `1.0.0` are unreleased. A correction lands inside `1.0.0` and moves neither version.

## What changes

### (1) The verify arm's requirements are a consumer selection {design-bearing} {user-facing: operator direction 2026-11-24, lead-relayed (not a ruling), on the set record and the paired entry's deliverable: a strictness knob set, off among them, covering ahead equal to zero and every bound suite passing}

A new scalar knob, `EVIDENCE_KIT_VERIFY_REQUIRE`, holds a space-separated set of requirement words. The empty default requires nothing beyond checks 1 to 5. **Not yet applied.**

- **`head`** requires the subject to be `HEAD` itself. Where it is an ancestor `<n>` commits behind, the arm adds the finding `subject <commit> is <n> commit(s) behind HEAD (EVIDENCE_KIT_VERIFY_REQUIRE: head)`. A subject that is no ancestor already carries check 2's finding, and gains no second one.
- **`verdict`** requires every suite's `verdict` to be `clean`. Each other suite adds the finding `suite '<suite>' verdict: new-failures (EVIDENCE_KIT_VERIFY_REQUIRE: verdict)`.
- **A requirement's finding is a contradiction**, exit 1 like every check after shape. The arm runs checks 1 to 5 first, and the verdict line is unchanged, so `ahead=` still prints for a reader that parses it.
- **The verify arm alone reads it.** `--import-gate-run` shares `judge` and leaves the requirements out. An import records an older or failing run as history, and `check-evidence-manifest`'s close-entry green block already judges each line it records. So the requirement check sits in the verify arm's `run`, after `judge`, and never inside `judge`.
- **An unknown word is a broken config.** The kit's table validator refuses a word other than `head` or `verdict` at exit 2, naming the knob and the two words.
- **Why a set and not two booleans.** A merge predicate is usually both requirements at once, and one set-valued knob is one line in the consumer's knob file. It is also one environment override, so a CI step can tighten a single call without editing the file. A further requirement joins as a word, without a new knob.

Replacement text for §verify-gate-run's closing check and exit paragraph. **Not yet applied:**

> 5. **Each suite's tree.** `tree` is `clean`.
> 6. **The consumer's requirements.** Each word in `EVIDENCE_KIT_VERIFY_REQUIRE` adds a check: `head`, that the subject is `HEAD` itself, and `verdict`, that every suite's `verdict` is `clean`. Empty, the default, adds none.
>
> Exit 0 means every check held. Exit 1 means a contradiction, with one line per finding naming the suite and the field, or the requirement it failed. Exit 2 means the Statement could not be read or judged, a `HEAD` naming no commit among the causes. The verdict line names the subject, the suite count, and how many commits `HEAD` is ahead of the subject (`ahead=-` where the subject is no ancestor). A reader wanting a merge predicate selects `head verdict` instead of parsing it.

The paragraph's remainder, from *A DSSE signature is never checked*, is unchanged.

The roster sentence at the end of §verify-gate-run gains `EVIDENCE_KIT_VERIFY_REQUIRE`, and §import-gate-run's *takes the verify arm's input forms and runs its checks* becomes *takes the verify arm's input forms and runs checks 1 to 5*, with the two halves of its roster sentence re-cut so the import's roster stays without the knob.

### (2) The producer may count untracked files {design-bearing} {user-facing: operator direction 2026-11-24, lead-relayed (not a ruling), on the set record and the paired entry's deliverable: a strictness knob, off among them, covering untracked files counted}

A new scalar knob, `EVIDENCE_KIT_TREE_UNTRACKED`, takes `ignore`, the default and today's read, or `count`. Under `count`, `--run-validate`'s per-suite `tree=` read is `modified` when the worktree holds an untracked file that is not ignored, outside the run's own writes. **Not yet applied.**

- **The read.** `git status --porcelain -z --untracked-files=all`. An untracked path under `EVIDENCE_KIT_TMP_DIR`, or equal to `EVIDENCE_KIT_STATEMENT_FILE`, is the run's own write and never counts. Otherwise a run would dirty the tree it is reading, and its next run would read `modified`. An ignored path never counts, so the consumer's `.gitignore` sets the boundary.
- **The producer applies it, never the verifier.** The Statement records `tree`, and check 5 already reds `modified`. So the strictness lands where the tree is read, and a verifier gains it by reading a Statement produced under `count`.
- **The provenance block is unchanged.** Its `worktree` stays gate-sdk's tracked-only read, which every SARIF log shares (gate-sdk/SPEC.md §run-gates). A run under `count` can therefore carry a `clean` block beside a suite whose `tree` is `modified`, and the suite's field is the one verify judges.
- **Any other value is a broken config**, refused at exit 2 by the table validator, naming the knob and both values.
- **Honest limit.** The Statement does not record which policy produced its `tree` values. So a verifier cannot tell a `clean` read that counted untracked files from one that ignored them. A consumer whose predicate depends on the count binds the knob in its tracked knob file, where producer and verifier read the same value.

Replacement text for §Evidence manifest's `commit=` and `tree=` bullet. **Not yet applied:**

> - **`commit=` and `tree=` are read per suite**, after its pre-hook and before its run, by `git rev-parse HEAD` and `git status --porcelain --untracked-files=no`, the reads the provenance block makes (gate-sdk/SPEC.md §run-gates, *The provenance block*), so untracked files are left out. Under `EVIDENCE_KIT_TREE_UNTRACKED=count` the tree read is `git status --porcelain --untracked-files=all` instead, and an untracked path that is not ignored, and that is neither under `EVIDENCE_KIT_TMP_DIR` nor the Statement's path, reads `modified`. A pre-hook that rewrites a tracked file therefore shows as `tree=modified` on that suite's line alone. The fold follows every read, so a run from a clean tree reads `clean`, but a rerun over an uncommitted fold reads `modified` on every line, truly, since the gates suite reads the tracked manifest. Commit the fold before a rerun meant to record `clean`.

The Statement table's `suites[].tree` row reads *the worktree state when the suite started, under `EVIDENCE_KIT_TREE_UNTRACKED`*. §The gate-run predicate's honest limits gain the limit above, and §bin/run-validate.sh's declared knob roster gains the knob.

### (3) Blank means empty or whitespace-only {mechanical} {user-facing: the evidence-blank-semantics entry's deliverable, carried from the 2026-11-16 review into this iteration's set record (operator direction 2026-11-24, lead-relayed, not a ruling), which delegates the choice to this stage: rule whether blank means zero characters or whitespace-only}

A provenance string is **blank** when it holds no character outside ASCII whitespace: space, tab, line feed, form feed and carriage return. The empty string is one case of it. Every reader and the producer apply that one definition. **Not yet applied.**

- **Why whitespace-only is blank.** A field exists to name something, and `" \t"` names nothing, just as `""` does. A reader that admits it reports provenance it does not have. The value set is ASCII on purpose. Rust's `char::is_whitespace` and ECMAScript's `\s` disagree at U+0085 and U+FEFF. The five ASCII characters are the one set the crate, the schema and a third-party validator all read alike.
- **The reader.** `provenance_shape` refuses a blank `name`, `root` or optional string, in place of an empty one, through one helper, `evidence::blank`, which every refusal site calls. The refusal wording becomes *absent, blank or not a string*.
- **The producer.** `provenance::present` filters a blank value to absent. So a build stamp or lock field holding only whitespace leaves its field out, never writing it blank.
- **The schema.** Each provenance string that carries `minLength: 1` also gains `"pattern": "[^\\t\\n\\f\\r ]"`, an unanchored match requiring one non-blank character. `the_published_golden_and_schema_match_the_renderer` asserts that pattern beside `minLength`.
- **The definition is the provenance block's.** installer/SPEC.md §The manifest also says *never null or blank* of its own writer's keys. That sentence is not a reader of this definition, and the amendment leaves it untouched. The block's lead cites the manifest for the absence convention alone.
- **No version moves.** Both contracts are unreleased (above), so the correction narrows `1.0.0` before any reader of it exists. After a release, the same narrowing would be a major under §The predicate's version.

Replacement text for gate-sdk/SPEC.md §run-gates' provenance-block lead. **Not yet applied:**

> **The provenance block** says which binary and which kits produced a verdict, and on what tree. It is this section's one definition, and a reader embedding it cites it. The run's `properties.provenance` carries it whole, as a JSON object with its keys sorted. A field the run cannot fill is **absent**, never null or blank, after the manifest's convention (installer/SPEC.md §The manifest). **Blank** means holding no character outside ASCII space, tab, line feed, form feed and carriage return. `native/src/provenance.rs` is its one producer.

## Producers and consumers

- **`EVIDENCE_KIT_VERIFY_REQUIRE`** (new knob) — producer: the consumer's knob file or the environment, resolved by the evidence-kit table with an empty default. Consumers: the kit's table validator, which checks its words at every resolution, and the verify arm's requirement check, which reads it after `judge`. No deployed configuration sets it in this repo. Its enabling configuration is an adopter's merge-predicate step, which this unit's smoke and gate-test cases stand in for.
- **The requirement findings** (new event) — producer: the verify arm, per word whose condition fails. Consumer: the caller through exit 1 and the finding lines, the same channel checks 2 to 5 use.
- **`EVIDENCE_KIT_TREE_UNTRACKED`** (new knob) — producer: the knob file or the environment, default `ignore`. Consumers: the table validator, and `--run-validate`'s per-suite tree read. The verify arm reads its effect through the Statement's `tree` and never reads the knob itself. Readers of the `tree=` value it shapes: `check-evidence-manifest`, for shape only (`clean`, `modified` or `-`, which this delta does not widen), the Statement renderer, and the verify and import arms (probe: `git grep -n "tree=" -- native/src` and `git grep -n '"tree"' -- native/src docs/contracts`).
- **The blank definition** (narrowed interface) — producer: `provenance::present` for this binary's own blocks. A third-party producer writes the block by hand, which is why the schema carries the pattern. Consumers: `evidence::provenance_shape`, behind both the verify and the import arm, and the schema's third-party validators.
- **Point 5 (narrowing).** Delta 3 narrows the accepted provenance strings. The red condition is that `provenance_shape` refuses a blank string at exit 2 on both arms, where it refused only an empty one, and a schema validator rejects one. Delta 2 narrows nothing by default. Under `count`, a tree that read `clean` may read `modified`, and check 5 reds it. Delta 1 narrows nothing by default, and under each word it reds the case that word names.
- **Point 6 (every member).** The members are this binary's own provenance producers, each through `present`. In `provenance::collect` those are `tool_version`, `source_stamp`, `target` and each kit's `version` (probe: `git grep -n "present(" -- native/src/provenance.rs`). Each is a build-time stamp or a lock field, and none is whitespace-only on any build this tree makes, so each keeps its value. The golden `docs/contracts/gate-run/v1/statement.json` carries no blank string, so it stays byte-identical.

## Existing sections updated

Roster from `git grep -n -e "verify-gate-run" -e "import-gate-run" -e "untracked-files=no" -e "never null or blank" -e "never blank" -e "minLength" -e "EVIDENCE_KIT_IMPORTED_RUNS"` over the tracked tree, the generated mirrors and the queue excluded.

- `evidence-kit/SPEC.md` §Layout and configuration, which gains both knobs and the validator's two refusals (deltas 1 and 2); §verify-gate-run, with check 6, the exit paragraph and its roster (delta 1); §import-gate-run, with *checks 1 to 5* and its roster (delta 1); §Evidence manifest's `tree=` bullet, the Statement table's `suites[].tree` row, §The gate-run predicate's honest limits and §bin/run-validate.sh's declared knob roster (delta 2); §Producers and consumers' verify-arm bullet (deltas 1 and 2).
- `docs/evidence-kit/SPEC.md` — the generated mirror, regenerated by `--emit docs-mirror --write` (all deltas).
- `gate-sdk/SPEC.md` §run-gates' provenance-block lead (delta 3).
- `docs/gate-sdk/SPEC.md` — the generated mirror (delta 3).
- `native/src/knobs/evidence_kit.rs` — two scalar rows, `EVIDENCE_KIT_VERIFY_REQUIRE` defaulting empty and `EVIDENCE_KIT_TREE_UNTRACKED` defaulting `ignore`, and the validator's two refusals with a unit case each (deltas 1 and 2).
- `native/src/emit/verify_gate_run.rs` — `KNOBS` gains the knob, `run` applies the requirements after `judge`, and unit cases cover each word's red and its clean (delta 1).
- `native/src/emit/run_validate.rs` — `KNOBS` gains the knob, and `head_state` takes the policy plus the two excluded paths, under a unit case for each exclusion (delta 2).
- `native/src/evidence.rs` — the `blank` helper, `provenance_shape`'s refusals, a unit case refusing `" \t"` in a required and an optional field, and the schema test's pattern assertion (delta 3).
- `native/src/provenance.rs` — `present` filters blank, and `every_optional_field_is_absent_when_unfilled` gains a whitespace-only input (delta 3).
- `docs/contracts/gate-run/v1/schema.json` — the pattern on each provenance string carrying `minLength: 1` (delta 3).
- `native/src/runner.rs` — the `--verify-gate-run` help paragraph names the requirement knob (delta 1).
- `evidence-kit/gate-tests/verify-gate-run-strict.test.sh` — new. A scratch repository's own run writes a Statement. Then, per requirement, a passing and a red case: `head` clean at the subject and red one commit later, `verdict` clean on a passing suite and red on a failing one, and `count` reading `clean` with only an ignored or tmp-dir untracked file and `modified` with another. Each red case asserts exit 1 and its finding line (deltas 1 and 2).
- `evidence-kit/smoke/install.sh` — the Statement block verifies once more under `EVIDENCE_KIT_VERIFY_REQUIRE="head verdict"`, the installed-adopter shape of the merge predicate (delta 1).
- `evidence-kit/README.md` — the verify arm's sentence names the requirement knob (delta 1).
- `.workflow/release-declarations.md` — *Knob changes* gains both knobs. *Behavior changes*' verify and import bullet reads *never blank, a whitespace-only string included* in place of *never blank* (all deltas).

## Retired spellings

- None — every delta adds a knob, a check or a definition, and no name is removed. The phrase *never blank* survives, and its meaning is pinned in place by delta 3.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for both knobs, the requirement findings and the blank definition.
- [ ] **Instruction surfaces: instruction only** — no template, agent definition or shim is touched.
- [ ] **Merged with no information lost** — each replacement re-phrases the passage it refines; the merged specs read as one document.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls evidence-kit/SPEC-*.md`).
- [ ] **Removals propagated** — none declared; `check-amendment-retired-spelling` stays clean.
- [ ] **Entry moved** — `verify-gate-run-strictness` and `evidence-blank-semantics` to Done with `--queue done` in the build batch's merge commit, before the drain stage. Their oracle is local, the evidence-kit fixture suite and the battery, so no remote run gates the move.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
