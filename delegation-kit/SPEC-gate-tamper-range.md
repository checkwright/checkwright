# SPEC amendment: gate-tamper-range

Pairs `gate-tamper-range-arm`. `check-gate-tamper` gains a range form that judges each commit of a handed-back range by the same two assertions its index form judges a staged commit by. A supervisor receiving an agent's commits runs it once over the range they span, through the direct form, and reads the verdict from the exit and the gate's lines. The index form, the battery's and the pre-commit hook's, is unchanged.

**What the tree does today**, each bullet with the probe that settled it:

- The rule accepts `--fixture <dir>` and no other argument, and refuses any other as `unknown argument` at exit 2 (`grep -n 'unknown argument\|"--fixture"' native/src/gates/gate_tamper.rs`).
- The live corpus is the index alone: the staged names from `git diff --cached -z --name-only`, and each staged gate file's exemption set read as `:<f>` against `HEAD:<f>` (`grep -n 'diff", "--cached\|format!(":{}"\|HEAD:{}' native/src/gates/gate_tamper.rs`).
- The live verdicts the fixture pair cannot reach are pinned by `delegation-kit/gate-tests/check-gate-tamper.test.sh` in throwaway repositories, and `--run-gate-tests` runs every `*.test.sh` in a kit's tests dir (`grep -n 'case_run "\$d"' delegation-kit/gate-tests/check-gate-tamper.test.sh`; `grep -n 'test.sh' native/src/emit/run_gate_tests.rs`).
- delegation-kit/README.md says the gate is invoked directly with `--fixture <dir>` only for testing (`git grep -n 'invoke it directly' -- delegation-kit/README.md`).

## What changes

### (1) The range form: argv, verdict and lines {design-bearing} {user-facing: the entry's deliverable — a range form with a machine verdict, the index form unchanged}

`checkwright-gates check-gate-tamper --range <base>..<head>` judges a range of commits instead of the index. The verdict is the gate contract's exit and lines, the one machine verdict every gate already speaks. No second result format is minted: the gate's direct form is how an outside caller reads a gate, exit code and clean line.

**Not yet applied** — replaces the paragraph of delegation-kit/SPEC.md §Verify after every agent commit that opens "`checks/check-gate-tamper.gate` (`precommit`, binary-dispatched)":

> `checks/check-gate-tamper.gate` (`precommit`, binary-dispatched). It reads one of three corpora, each a selector of the rule's own input that no knob resolves, so none is the arrives-too-late shape gate-sdk/SPEC.md §The non-gate arm deletes.
>
> - **The index**, with no argument: the staged commit, from `git diff --cached`. The battery and the pre-commit hook run this form.
> - **A range**, with `--range <base>..<head>`: every commit `<head>` reaches and `<base>` does not, each judged as the commit it is. A supervisor receiving an agent's commits runs it through the direct form, `checkwright-gates check-gate-tamper --range <base>..<head>`, since a commit made with its hooks bypassed was never judged at its own pre-commit. Each finding's head line names its commit as `in commit <abbreviated id>`. The clean line reads `GATE-TAMPER: clean (<n> commit(s) in <base>..<head>; <k> merge(s) not judged; <m> registered gate file(s) covered; …)`, and an empty range is clean with `0 commit(s)`.
> - **A fixture**, with `--fixture <dir>`: the injected `staged-files` and `added-exemptions` lists of the fixture pair. A trailing `--fixture` with no directory is refused.
>
> Exit 2 refuses a range with an empty side or spelled other than `<base>..<head>`, either side naming no commit, `--range` beside `--fixture`, and any other argument.

- **Why per commit, never the range's net change.** Both assertions are about a commit's shape: a gate edit isolated in its own commit, and an exemption never excusing the commit it lands with. A range's net diff merges a legitimate gate-only commit with the product commit beside it and reds a correct history, and it hides an exemption added in one commit and used in the next from neither assertion. Judging each commit as its pre-commit would have reads the range exactly as an unhooked agent should have been read.
- **Why `<base>..<head>` alone.** A bare revision would judge the whole history reachable from it, a silent widening the caller did not ask for, and a symmetric `...` range judges commits on the base's side too. The two-dot form is the one a supervisor holds after a hand-back: the commit it dispatched from and the commit it received.
- **Why no JSON.** A gate's verdict is its exit and its lines on every path (gate-sdk/SPEC.md §Fail-closed contract), and the direct form is how a caller reaches one gate (gate-sdk/SPEC.md §run-gates). A JSON form is minted where a named reader parses fields no line carries; this one reads a verdict and a commit id, both on the lines.

### (2) The per-commit derivation and its honest limits {design-bearing}

The range's commits are `git rev-list --reverse --no-merges <base>..<head>`, oldest first, and its merges are counted by the same read with `--merges`; both read object ids, never names. Each changed set is a name listing, so it is read NUL-terminated through the crate's listing reader (gate-sdk/SPEC.md §Fail-closed contract, `native/src/listing.rs`). For each commit `<c>` the two lists the index form reads are derived against its first parent:

- **The changed set** is `git diff-tree -r -z --no-commit-id --name-only <c>^ <c>`, and a root commit is diffed against the empty tree.
- **The added exemptions** are each changed gate file's set at `<c>:<f>` against `<c>^:<f>`, with `added_between`'s existing reading: a file absent at the parent adds its whole set, and a file the commit deletes reads as an empty set rather than a failure.

The gate-file and meta-path unions are built once, from the working tree's registry and knobs, as the index form builds them. A listing git cannot produce refuses at exit 2: the range form judges what the caller named, and degrading a failed listing to an empty range would print a clean verdict over commits it never read.

**Not yet applied** — appended after the paragraph opening "The union is built in both modes" in delegation-kit/SPEC.md §Verify after every agent commit, which is re-phrased to "The union is built in every mode":

> **A range is judged commit by commit against each commit's first parent.** A root commit is diffed against the empty tree. The unions are the working tree's, so a gate registered only inside the range is covered by the declared globs alone, as a declaration staged beside its own registration is. **Honest limits:** a merge commit is not judged, since its diff against its first parent carries every commit it merges; the commits it brings in are in the range and are judged one by one, so what escapes is a merge's own resolution edits, and the clean line counts the merges it skipped. A range that rewrites the registry or the meta-path knobs is judged under the working tree's values, not each commit's own.

- **Why first parent and `--no-merges`.** Diffing a merge against its first parent would charge every merged commit's paths to one commit and red a merge of two clean branches. The merged commits are each in `<base>..<head>` and are judged on their own, so skipping the merge loses only its own resolution edits, which the clean line's merge count makes visible.
- **Why the working tree's unions.** Rebuilding the registry at every commit would re-run the resolution the battery owns once per commit, for the narrow case of a range that edits the registry, which assertion A already isolates as a meta-layer commit.

### (3) The range form's pair: good and bad range cases in the live harness {mechanical}

The range derivation runs in no fixture case, by the injection property §Testing already records for the live arm: `--fixture` supplies the lists directly. Its pair therefore lives beside the live verdicts, as cases of `delegation-kit/gate-tests/check-gate-tamper.test.sh`: a throwaway repository, a commit series, and the gate run with `--range` inside it.

**Not yet applied** — appended to the bullet list under "**The tamper pair cannot reach the live arm**" in delegation-kit/SPEC.md §Testing, whose lead sentence re-phrases "the live arm" to "the live and range arms", and "It pins the four verdicts the pair cannot" to "It pins the verdicts the pair cannot":

> - a range whose commits each hold the shape is clean, a gate-only commit beside a product-only commit among them, where the same two changes in one commit red;
> - a range carrying one co-staged gate edit, or one exemption added beside the file it excuses, exits 1 naming that commit;
> - a merge in the range is counted and not judged, and the commits it brings in are;
> - a malformed range, an unknown revision and `--range` beside `--fixture` exit 2.

The `delegation-kit/SPEC.md §Layout and configuration` tree line `gate-tests/check-gate-tamper.test.sh  # the live arm, over throwaway git repos` reads `# the live and range arms, over throwaway git repos`.

### (4) The two pointers that name the gate's argv {mechanical}

**Not yet applied** — delegation-kit/README.md's sentence "`check-gate-tamper` runs from the pre-commit hook and the battery; invoke it directly with `--fixture <dir>` only for testing." becomes:

> `check-gate-tamper` runs from the pre-commit hook and the battery. A supervisor checks a range of commits it received with `checkwright-gates check-gate-tamper --range <base>..<head>`; `--fixture <dir>` is for testing.

**Not yet applied** — gate-sdk/SPEC.md §The non-gate arm's input-corpus-selector bullet names `check-gate-tamper`'s `--fixture <dir>` and `--range <base>..<head>`.

## Producers and consumers

- **The `--range` argument.** Producer: a supervisor, a lead or an outside orchestrator holding a hand-back's base and head, through the direct form; no deployed configuration sets it, since the battery and the hook keep the index form. Consumer: `rule` in `native/src/gates/gate_tamper.rs`, which selects the range corpus.
- **The per-commit changed set and added exemptions.** Producer: the range derivation of delta 2, one pair per non-merge commit. Consumer: the same assertion A and B code the index form runs, once per commit.
- **The `in commit <id>` head-line clause.** Producer: the range form's findings. Consumer: the caller reading the exit-1 output, who must know which commit to split or rewrite; it is the only field that tells a range's findings apart.
- **The clean line's `<n> commit(s) in <range>` and `<k> merge(s) not judged`.** Consumer: the caller confirming the range it named was the range read; the merge count is the honest limit made visible at the verdict, read by the same caller.
- **Point 2, roster-holding readers of a surface a name lands on.** The arg lands on the gate's argv, whose rosters are gate-sdk/SPEC.md §The non-gate arm's selector bullet (delta 4) and delegation-kit/README.md's invocation sentence (delta 4). gate-sdk/SPEC.md §Fail-closed contract's degrading-caller roster (`check-gate-tamper`'s staged read) is not joined: the range read refuses on a failed listing and degrades nothing.
- **Point 5, narrowing.** None: the index and fixture forms keep their corpus and verdicts.
- **Point 6, every member.** The range form obliges every commit in the range; a merge commit has no satisfying reading under per-commit shape, so it is narrowed out by name and counted (delta 2).

## Existing sections updated

Roster probe: `git grep -n -- '--fixture' -- delegation-kit/ gate-sdk/SPEC.md ':!*/gate-tests/*'` and `git grep -n 'check-gate-tamper.test.sh' -- delegation-kit/`, over the tracked tree.

- delegation-kit/SPEC.md §Verify after every agent commit — the corpus paragraph and the union paragraph (deltas 1 and 2).
- delegation-kit/SPEC.md §Testing — the live-arm list (delta 3).
- delegation-kit/SPEC.md §Layout and configuration — the test-harness tree line (delta 3).
- delegation-kit/README.md — the invocation sentence (delta 4).
- gate-sdk/SPEC.md §The non-gate arm — the input-corpus-selector bullet (delta 4).
- native/src/gates/gate_tamper.rs — the argv, the range derivation and the commit-naming lines, with unit tests for the range spelling's refusals (deltas 1 and 2).
- delegation-kit/gate-tests/check-gate-tamper.test.sh — the range cases (delta 3).
- docs/delegation-kit/SPEC.md, docs/delegation-kit/README.md, docs/llms-full.txt — the generated mirrors (all deltas).

## Retired spellings

- None — the range form adds an argument and retires no name; the index form's argv and lines are unchanged.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — the README sentence carries the invocation, never the grounds.
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` holds.
- [ ] **Entry moved** — `gate-tamper-range-arm` moves to Done before the drain stage, in the commit that lands the merge.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
