# SPEC amendment: verify-gate-run-json

Pairs `non-gate-arm-json`. An orchestrator that gates a merge or a dispatch on a gate-run Statement it did not produce calls `--verify-gate-run` and reads four things off the result: the verdict, how far `HEAD` is ahead of the subject, and each suite's tree and verdict. Today it gets them by parsing a line written for a person. This amendment gives that one arm a machine form, a versioned JSON document behind a `--json` option. It also states, in gate-sdk's non-gate arm section, the general rule for minting such a form on any other arm: only for a named caller, and under the verdict contract's versioning.

**What the tree does today**, read from `evidence-kit/SPEC.md` §verify-gate-run, `gate-sdk/SPEC.md` §The non-gate arm and §run-gates, and `native/src/emit/verify_gate_run.rs`:

- **The arm prints prose only.** `Judged::report` prints `VERIFY-GATE-RUN: clean (subject <c>, <n> suite(s), ahead=<a>; signature not checked)`, or a contradiction count over one indented line per finding. The suites' own tree and verdict values are not printed at all unless a finding names them (probe: `git grep -n "signature not checked" -- native/src/emit/verify_gate_run.rs`).
- **An option is refused today.** `--verify-gate-run --json` exits 2: `takes at most one operand (got: --json)`, since `read_input` refuses any operand beginning `--` (probe: run it through the binary).
- **No non-gate arm has a machine form.** The one versioned machine-read contract the binary writes is the battery's SARIF log, `properties.verdictContract` `1.0.0`, with its version rule and published goldens under `docs/contracts/verdict/v1/` (gate-sdk/SPEC.md §run-gates, *The verdict contract's version* and *The contract's goldens*). The Statement's predicate follows that rule too (evidence-kit/SPEC.md §The gate-run predicate) (probe: `ls docs/contracts`, which lists `gate-run` and `verdict`).
- **§The non-gate arm states no rule for a machine form.** It requires a named caller per arm and mints a key or field only with its reader, but says nothing on when an arm may print JSON (probe: `git grep -n -i "json" -- gate-sdk/SPEC.md`, whose hits in that section are the harness payload and envelope only).

## What changes

### (1) `--verify-gate-run --json` {design-bearing} {user-facing: operator direction 2026-11-24, lead-relayed (not a ruling), on the set record: the JSON form narrowed to `--verify-gate-run`, the one arm an adopter has named}

§verify-gate-run's opening sentence takes the option, and a paragraph is added after the exit paragraph. **Not yet applied:**

> **`--verify-gate-run [--json] [<file> | -]`** judges one Statement against this tree and its commit, dispatched to `native/src/emit/verify_gate_run.rs`.

> **`--json` replaces the verdict line with one JSON document**, for a caller that gates on the result. The checks and the exit statuses are unchanged. On exit 0 or 1 the arm prints one object on stdout, its keys sorted and no insignificant whitespace, ending in a newline:
>
> | Key | Value |
> | --- | --- |
> | `contract` | the document's version, `1.0.0` |
> | `verdict` | `clean` on exit 0, `contradiction` on exit 1 |
> | `subject` | the Statement's `gitCommit` |
> | `ahead` | the number of commits `HEAD` is ahead of the subject, an integer; absent where the subject is no ancestor of `HEAD` |
> | `suites` | one object per suite, in the Statement's order: `name`, `tree` and `verdict`, copied from the suite |
> | `findings` | one string per finding, in the order the text form prints them; empty on exit 0 |
>
> The document's shape and meanings are the contract: its keys, their types, the value sets of `verdict`, and the order of `suites` and `findings`. A finding's text is not, and changes freely, as a SARIF message's does. The version follows the verdict contract's rule (gate-sdk/SPEC.md §run-gates, *The verdict contract's version*): a removed or renamed key, a changed meaning or a narrowed value set is a major, an added key or value a minor. A reader refuses a major it does not know and ignores a key it does not know. On exit 2 the arm prints no document, and the refusal goes to stderr as it does without the option, so a caller reads the status before it parses. The document has no signature key, because its value would be constant: this arm never checks a signature, as above.
>
> **The goldens** are `docs/contracts/verify-gate-run/v1/clean.json`, a two-suite Statement judged clean at `ahead` 0, and `contradiction.json`, a subject that is no ancestor, with a suite whose `tree` is `modified`. They are published at `/contracts/verify-gate-run/v1/`. A unit test in the arm's module byte-compares each against the renderer's output on a fixed judgment, so they are fixtures a test holds, never a regenerated projection.

- **Why an option on the arm, and not a second arm.** The checks, inputs, knobs and exit statuses are the same; only the rendering of the result differs. A sibling arm would duplicate the read and the judgment, and a second flag would need its own named caller for an output its sibling already owns.
- **Why `ahead` is absent rather than `-` or `null`.** `-` is the text form's placeholder, and an integer key that is sometimes a string forces every reader to branch on type. Absence is the provenance block's convention for a key that does not apply, as `commit` is absent on an unborn branch (gate-sdk/SPEC.md §run-gates, *The provenance block*), and the Statement embeds that block.
- **Why the suites are copied rather than judged.** The caller asked for each suite's tree and verdict, and the Statement holds both. The judgment on them is already in `findings` and `verdict`, so a per-suite `ok` key would be a second copy of it.
- **Why the findings stay strings.** The named caller reads the verdict and the suite fields, not the finding structure. A finding object would mint keys no reader reads (gate-sdk/SPEC.md §The non-gate arm, a key minted with its reader).
- **Why `--import-gate-run` keeps its text form.** It shares the judgment and the report, but no caller has asked to read its result, so the option is not minted there.

### (2) The rule for a machine form on any non-gate arm {design-bearing} {user-facing: operator direction 2026-11-15, lead-relayed (not a ruling), carried on the paired entry: a JSON form lands on an arm only where a named caller reads it, never across every arm at once}

A paragraph added to gate-sdk/SPEC.md §The non-gate arm, after *A flag spelling is minted with its implementation*. **Not yet applied:**

> **A machine form is minted per arm, with the caller that reads it.** An arm prints for a person by default. It gains a JSON form only where a named caller reads the result, and the form is a `--json` option on that arm, never a sibling arm, so the read, the checks and the exit statuses stay one implementation. The document is one object on stdout, keys sorted and no insignificant whitespace, carrying a `contract` key versioned under §run-gates' verdict-contract rule, with goldens a unit test in the arm's module byte-compares. On the arm's refusal status it prints no document. Its keys are the ones its caller reads: a key no reader reads is not minted. `--verify-gate-run` is the member (evidence-kit/SPEC.md §verify-gate-run).

- **Why per arm and not across the family.** A JSON form is a contract, and a contract with no reader is a surface that can only drift: it must stay versioned and golden-held for nobody. The class already refuses an arm without a caller, and this applies the same rule one level down, to an arm's output form.
- **Why the verdict contract's rule rather than a new one.** One version rule across the binary's machine outputs means a caller learns it once. The SARIF log and the gate-run predicate already follow it.

## Producers and consumers

- **The `--json` option** (new) — producer: the arm's argument grammar, in `read_input`'s caller, which strips the option before the operand rules apply. Consumer: the renderer that writes the document in place of `Judged::report`.
- **The document** (new contract) — producer: `--verify-gate-run --json`. Named caller: an orchestrator gating a merge or a dispatch on a Statement, which reads `verdict`, `ahead`, and each suite's `tree` and `verdict`. In-tree readers: the golden unit test, and a front-end case in `evidence-kit/gate-tests/verify-gate-run-strict.test.sh` that parses the document's `verdict` and `ahead` on a clean and on a contradicted Statement.
- **Every key has a reader.** `contract`: a reader's major check. `verdict`, `ahead`, and `suites[]`'s `tree` and `verdict`: the named caller. `suites[]`'s `name`: the same caller, to key the suite fields. `subject`: the caller, to confirm which commit was judged. `findings`: the caller's report to its own operator, as the text form's lines are.
- **The goldens** (new) `docs/contracts/verify-gate-run/v1/` — producer: the commit changing the renderer; consumers: the unit test, and outside readers through the site.
- **The non-gate arm rule** (new paragraph) — consumer: a session adding a machine form to any other arm.
- **Point 5 (narrowing).** Nothing narrows. Without `--json` the arm's output is byte-identical, and an argument list that was refused before is the only one that now succeeds.
- **Point 6 (every member).** The members are the arms sharing this arm's input grammar and report, since the option could reach them through the shared `read_input`: `--verify-gate-run` and `--import-gate-run` (probe: `git grep -n "read_input(" -- native/src/emit`). The option is added to the first alone, and `read_input` keeps refusing a `--` operand, so `--import-gate-run --json` stays a refusal at exit 2. The front-end's help text is the other member naming the arm's grammar (probe: `git grep -n "verify-gate-run \[" -- native/src/runner.rs`).

## Existing sections updated

Roster from `git grep -n -e "verify-gate-run" -e "verdictContract"` over the tracked tree, the posts, the queue and the generated mirrors excluded, narrowed to the sites stating the arm's grammar or output, or the binary's machine-read contracts.

- `evidence-kit/SPEC.md` §verify-gate-run — the opening sentence and the new paragraph (delta 1). §Producers and consumers gains the document and its goldens beside the schema and golden entry (delta 1).
- `gate-sdk/SPEC.md` §The non-gate arm — the new paragraph (delta 2). §run-gates, *The verdict contract's version*, gains one sentence naming the arms' machine forms as further users of the rule (delta 2).
- `native/src/emit/verify_gate_run.rs` — the option, the renderer, and the golden unit test (delta 1).
- `native/src/runner.rs` — the usage line and the `--verify-gate-run` help paragraph name `--json` (delta 1).
- `docs/contracts/verify-gate-run/v1/clean.json` and `contradiction.json` — new (delta 1).
- `evidence-kit/gate-tests/verify-gate-run-strict.test.sh` — the `--json` cases (delta 1).
- `evidence-kit/README.md` — the line naming the arm names its machine form (delta 1).
- Generated projections the sections move: `docs/evidence-kit/SPEC.md`, `docs/evidence-kit/README.md`, `docs/gate-sdk/SPEC.md` and `docs/llms-full.txt`, each regenerated by the command its gate prints on red (docs/generated-projections.md) (all deltas).
- `.workflow/release-declarations.md` — *Behavior changes* gains `--verify-gate-run --json`, the document at contract `1.0.0`, and its goldens' site path (delta 1).

## Retired spellings

- None — the option and the document are additions, and the text form is unchanged.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the option, the document, its goldens and the arm rule.
- [ ] **Instruction surfaces: instruction only** — no template changes; the release declaration states what a caller reads.
- [ ] **Merged with no information lost** — the paragraphs land in §verify-gate-run and §The non-gate arm, and the sentence in §run-gates; the merged specs read as one document each.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls evidence-kit/SPEC-*.md`).
- [ ] **Removals propagated** — none declared; `check-amendment-retired-spelling` stays clean.
- [ ] **Entry moved** — `non-gate-arm-json` to Done with `--queue done` in the build batch's merge commit, before the drain stage. Its oracle is local: the unit tests, the evidence-kit fixture suite, the build and the battery.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
