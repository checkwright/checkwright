# SPEC amendment: survey-corpus

A survey block's `corpus` is spliced verbatim into its witness, `git diff --quiet <rev>..HEAD -- <corpus>`, and nothing checks what was spliced. lifecycle-kit/SPEC.md §The survey record already says the field is a pathspec and that prose there passes vacuously; neither `--emit file-survey` nor `check-survey-record` holds it. So a block whose corpus is prose certifies "unchanged" for a corpus nothing measured, and the citing session cites it.

**The ruling: the corpus is a grammar both the arm and the gate hold.** It is either the literal `none` or a list of shell-quoted git pathspecs, each matching at least one path in the tree at the block's `rev`. The arm refuses anything else before it appends. The gate asserts the grammar on every run and the per-pathspec match wherever it probes `rev`. `none` is for a survey over no tree corpus — a remote run, an untracked log, git history — whose witness is the oracle re-run alone.

**Refused: accepting prose after a separator.** Letting a corpus carry `<pathspecs> — <scoping prose>` and splitting on the dash keeps the record's old habit and adds a parse rule a raw append can still get wrong. Scoping prose already has a home, `finding`.

**Refused: probing at HEAD rather than at `rev`.** A survey over a file a later commit deleted would red at the next battery run although the witness is still sound: `git diff` over a deleted path reports the deletion.

**Not folded in: `survey-oracle-liveness-unasserted`** (an oracle may name a wiped path). The oracle is a command line, and knowing which paths it reads means running it. A dead oracle already fails its witness: the re-run does not reproduce the verdict and the session dispatches the delta, which is the status quo. That entry stays in the icebox.

**Measured at authoring (2026-09-26):**

- **The vacuous pass.** `git diff --quiet c279d4fa..HEAD -- the survey of prose` exits 0.
- **The resolution probe discriminates, magic included.** `git ls-files --error-unmatch --` exits 0 on `TASK-QUEUE.md gate-sdk/SPEC.md`, on `.`, on `':!docs'`, on `'*.gate'` and on `':(glob)**/*.sh'`, and exits 1 on `the survey`, naming each unmatched word.
- **The corpus values the record has carried.** `git log -p --format= -- .workflow/survey-record.md | grep -E "^\+- corpus:" | sort -u` prints 363 distinct values. Many carry an em-dash tail, a comma-separated list, a parenthetical count, a section heading (`TASK-QUEUE.md ## Deferred`) or a non-tree subject (runner labels, a `.metric/` log, git history), and some carry an unquoted glob (`*.gate`, `*/SPEC.md`) that the shell expands before git sees it. The record is boundary-truncated, so no block from a closed iteration is re-read. The one block on disk now, `TASK-QUEUE.md gate-sdk/SPEC.md`, satisfies the grammar.
- **The field's readers.** `grep -rn "corpus" native/src --include=*.rs` lists the ones that parse the field. `native/src/gates/survey_record.rs` is the gate. `native/src/emit/file_survey.rs` is the writer and prints the witness hint. `native/src/emit/cite_survey.rs` renders the field verbatim. `native/src/emit/enter_stage.rs` names the witness in prose only.

## What changes

### (1) The corpus grammar {design-bearing}

**Not yet applied.** In lifecycle-kit/SPEC.md §The survey record, the grammar block's line `- corpus: <git pathspec the survey covered>` becomes:

```
- corpus: <the git pathspecs the survey covered, shell-quoted, or the literal `none`>
```

The paragraph "**`corpus` is spliced verbatim into the composed witness, so it is a pathspec and nothing else.** Scoping prose belongs in `finding`. Prose in this field does not error: `git diff` accepts the words as pathspecs matching nothing and exits clean, so the witness certifies a corpus it never read." becomes:

> **`corpus` is spliced verbatim into the witness, so it has a grammar.** It is either the literal `none` or one or more git pathspecs separated by spaces. A pathspec carrying whitespace, a quote, a shell metacharacter, a glob character or pathspec magic is single-quoted, since the witness runs in a shell and must receive the pathspec rather than its expansion. Every pathspec matches at least one path in the tree at `rev`. Scoping prose belongs in `finding`. The grammar exists because prose here does not error: `git diff` takes the words as pathspecs matching nothing and exits clean, so the witness certifies a corpus it never read. `none` names a survey over no tree corpus, such as a remote run, an untracked log or git history, whose witness is the oracle re-run alone. A `none` corpus beside `oracle: none` is a note (the honest limit below).

In the witness, step 1 "**Corpus still?** `git diff --quiet <rev>..HEAD -- <corpus>` — clean means no commit since the survey touched anything it covered." becomes:

> 1. **Corpus still?** `git diff --quiet <rev>..HEAD -- <corpus>` — clean means no commit since the survey touched anything it covered. A `none` corpus skips this step.

### (2) The capture arm refuses a corpus outside the grammar {design-bearing}

**Not yet applied.** In `native/src/emit/file_survey.rs`, `emit` validates the corpus after the arity check and before stamping or writing anything. It tokenizes the corpus by the grammar (delta 1) and refuses at exit 2, printing the usage line, when:

- a bare word carries a character the grammar requires quoted, or a single quote is left open;
- `none` appears beside another word;
- any pathspec matches no path tracked at HEAD, which is the `rev` the arm stamps. The refusal names every unmatched pathspec, and adds that scoping prose belongs in `finding` and that a survey over no tree corpus takes `none`.

A git failure while resolving is a refusal too, so the arm never files a block it could not check. The witness hint printed on stderr omits the diff for a `none` corpus and prints the oracle re-run alone.

In lifecycle-kit/SPEC.md §The survey record, the paragraph opening "**The affordance.** `bash gate-sdk/bin/run-gates.sh --emit file-survey`" carries "exit 2 on a missing or empty argument,", which becomes the text below. The `--emit cite-survey` paragraph carries the same phrase and keeps it. The same sentence's repo-root clause is `isolated-capture`'s to edit (lifecycle-kit/SPEC-isolated-capture.md, delta 2). The two spans are disjoint, so either lands first.

> exit 2 on a missing or empty argument or on a corpus outside the grammar above, naming each pathspec that matched nothing,

Unit tests in the module cover each refusal and the accepted forms: a quoted glob, an exclude pathspec, `.`, and `none`. The resolution cases run in a sandbox repository.

### (3) The gate holds the grammar and probes each pathspec {design-bearing}

**Not yet applied.** In `native/src/gates/survey_record.rs` the `corpus` assertion widens past non-empty. In **both** modes the value must parse by the grammar (delta 1). In **bare** mode, where the gate already probes `rev`, each pathspec must also match at least one path in `rev`'s tree. That match is the probe the arm ran at filing, repeated against the commit the block names. The probe runs only once `rev` has resolved, so a block whose `rev` names nothing reports that finding alone. The help line names the grammar and `none`.

In lifecycle-kit/SPEC.md §check-survey-record, the invariant's "`corpus` non-empty;" becomes:

> `corpus` parsing by the grammar §The survey record states, and in bare mode each of its pathspecs matching a path in `rev`'s tree;

The paragraph "**Bare drives the configured record with the full assertion set; an explicit file argument drives it hermetically — grammar only, with no rev-existence probe.**" becomes:

> **Bare drives the configured record with the full assertion set; an explicit file argument drives it hermetically — grammar only, with neither the rev-existence probe nor the corpus-match probe.**

The fixture sentence's bad case ("the bad case a short sha, an empty oracle, …") gains "a corpus carrying an unquoted parenthetical". The test-script sentence ("plus `gate-tests/check-survey-record.test.sh` for the half the pair cannot hold, …") gains "both arms of the corpus-match probe (a pathspec matching a path at `rev`, and one matching none)".

The bad fixture `lifecycle-kit/gate-tests/check-survey-record/bad/record.md` gains a block whose corpus is `TASK-QUEUE.md (all sections)`, and its `expect.txt` gains the finding line. `lifecycle-kit/gate-tests/check-survey-record.test.sh` gains the two probe cases.

## Producers and consumers

- **The grammar** (delta 1). Producer: the filing session, through the arm, or through a raw append that the gate then holds. Consumers: the arm's validation at filing (delta 2); `check-survey-record` at every battery run (delta 3); the citing session's witness step 1, which runs the corpus or skips it for `none`; and `--emit cite-survey`, which renders the field verbatim and needs no change.
- **The `none` literal.** Read by the witness step to skip the diff, by the arm to omit the diff from its hint, and by the gate as the one legal word that matches no path. Nothing produces it by default. A filer chooses it.
- **The arm's refusal** (delta 2). The filing session reads it on stderr at exit 2 and nothing is appended. Enabled wherever the kit is vendored; no config.
- **The gate's two new findings** (delta 3). The committing session reads them at pre-commit, before the block lands, and corrects the uncommitted block. Append-only binds a committed block, so an uncommitted one is still the filer's to fix. Only a bypass lands a bad block, and there the red stands until the boundary truncation.
- **Point 5, the gate's red condition.** The corpus assertion narrows what is legal, so it can red a block that passed before. The record on disk holds one block and its corpus parses and matches (measured above). No reader reds on finding none or asserts a count over this field.
- **Point 6.** Not obliged over an enumerable corpus. The historical values are boundary-truncated and are never re-read.

## Existing sections updated

Roster from `grep -rn "corpus" lifecycle-kit/SPEC.md` for the survey sections, `grep -rn "corpus" native/src --include=*.rs`, and `ls lifecycle-kit/gate-tests | grep survey`, run 2026-09-26.

- lifecycle-kit/SPEC.md §The survey record: the grammar block, the corpus paragraph and witness step 1 (delta 1), and the affordance paragraph (delta 2).
- lifecycle-kit/SPEC.md §check-survey-record: the invariant, the bare-and-hermetic paragraph and the contracts paragraph (delta 3).
- `native/src/emit/file_survey.rs` (delta 2).
- `native/src/gates/survey_record.rs` (delta 3).
- `lifecycle-kit/gate-tests/check-survey-record/bad/record.md`, its `expect.txt`, and `lifecycle-kit/gate-tests/check-survey-record.test.sh` (delta 3).
- The on-site mirror of `lifecycle-kit/SPEC.md`, regenerated by `check-docs-mirror-fresh`'s printed command (deltas 1, 2 and 3).
- `.workflow/release-declarations.md`: one Behavior changes bullet naming `--emit file-survey` and `check-survey-record` (deltas 2 and 3). A survey's `corpus` is now shell-quoted git pathspecs, each matching a tracked path, or `none`. The arm refuses anything else and the gate reds on it.

## Retired spellings

- None — no delta renames or deletes a spelling.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness checklist holds for the grammar, the `none` literal, the arm's refusal and the gate's findings.
- [ ] **Instruction surfaces: instruction only.** No template changes. The grounds sit in the SPEC text of deltas 1 and 3.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Entry moved.** `survey-witness-composed-from-unvalidated-corpus` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
