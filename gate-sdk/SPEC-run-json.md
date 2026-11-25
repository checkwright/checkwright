# SPEC amendment: run-json

Pairs `orchestrator-arm-json`. An outside work engine runs the battery and needs its verdict in a form it can read without parsing the human transcript. Today the SARIF log is the only machine form, and it needs a path knob and a second read. This amendment gives `--run` a `--json` option. With it, stdout carries one document: the run's verdict, its scope, its provenance, one entry per member in gate-set order, and the declared omissions. Findings stay in the SARIF log. The document carries the run's three-way verdict and the error document that `exit-code-tool-error` adds, and its write follows the verdict store that `verdict-lookup` adds, so this amendment lands after both.

**What the tree does today**, each bullet with the probe that settled it:

- `--run` takes no `--json`. A leading `--json` already reaches the arm through the front end's dash passthrough and takes the arm's unrecognized-option refusal (`bash gate-sdk/bin/run-gates.sh --json`).
- The arm defaults its own gates dir where argv names none, so a selector the front end does not rewrite still finds the configured registry (`grep -n 'unwrap_or_else(|| configured_dir' native/src/runner.rs`).
- The transcript, the summary and the omission lines go to stdout (`grep -n 'stdout()' native/src/runner.rs`).
- The non-gate arm's machine-form rule admits one member, `--verify-gate-run` (gate-sdk/SPEC.md §The non-gate arm, the paragraph **A machine form is minted per arm**).
- An omission's reason is one whitespace-free token, the fourth field of its `# omitted:` line (`grep -n 'pub fn omissions' -A12 native/src/registry.rs`).

## What changes

### (1) The option and its streams {design-bearing} {user-facing: the entry's deliverable — the battery's verdict on stdout as one document}

**Not yet applied**: gate-sdk/SPEC.md §run-gates gains, after the paragraph on `GATE_SDK_SARIF_RESULTS`:

> **`--json` prints the run's verdict as one document on stdout.** It is the arm's leading option, before any selector: `--run --json`, `--run --json --only <name>…` or `--run --json --for <path>…`. Under it the transcript, the summary and the omission lines go to stderr unchanged, so stdout carries the document alone. The exit status is the run's verdict as without the option. The document is written last, after the SARIF log and the verdict store where each is set, and on exit 2 or 3 it is the error document (§The non-gate arm). A log or store the arm cannot write is therefore an error document at exit 3, and no run document is printed.

- **Why a leading option.** `--only` takes every token up to `--`, and `--for` takes the rest of argv. An option after either would be read as a gate name or a path.
- **Why the front ends are unchanged.** A leading dash token passes through the stub untouched and reaches the arm (§run-gates' residual grammar), and the arm defaults its gates dir itself. `--run-front-end-parity`'s corpus needs no case for it.
- **Why the transcript moves to stderr rather than going silent.** A person watching a run still sees red members, and the document stays parseable whole. The SARIF log stays the findings' home, so the document repeats none of the captured output.

### (2) The document {design-bearing} {user-facing: the entry's deliverable — the shape the adopter reads}

**Not yet applied**: the paragraph of delta 1 continues:

> The document is one object, keys sorted and no insignificant whitespace, under §The non-gate arm's rule. Its keys:
>
> | Key | Holds |
> | --- | --- |
> | `contract` | the document's version, `1.0.0`, versioned under this section's verdict-contract rule |
> | `verdict` | the run's verdict: `pass`, `fail` or `error` |
> | `scope` | `registry`, `only` or `for`, the SARIF run's `properties.scope` word |
> | `provenance` | the provenance block (below), whole |
> | `members` | one object per selected member, in gate-set order |
> | `omitted` | one object per declared omission in a bare run, in registry order; empty under `--only` and `--for` |
>
> A `members` object carries `name`, `kind` and, where the member ran, `exit`, its status. `kind` is one of `pass`, `notApplicable`, `fail` and `error`. `pass` and `notApplicable` are a green member, told apart as its SARIF result's kind is. `fail` is a member that exited 1. `error` is a member that exited with any other non-zero status, resolved nowhere, or could not be started. An `error` member also carries `error`, `refused` for an exit 2 or a member that resolved nowhere and `tool` otherwise. A member that could not be started carries `exit` 3, as its dispatch tail does, and one that resolved nowhere carries none. An `omitted` object carries `name` and `reason`.
>
> **The value sets are the contract's.** `verdict`, `scope`, `kind` and `error` each take only the words above, and a word outside them is a contract break, not free text. `reason` takes the registry's own grammar: one token holding no whitespace, the consumer's to choose, since the omission class is reason-agnostic (above). A reader branches on `kind` and on `reason`'s presence, and reads a reason's word as its own registry wrote it.

- **Why gate-set order.** The adopter keys a diff on member position as well as name. Gate-set order is the order `gateSet` digests and the SARIF log writes, so the three surfaces agree.
- **Why `error` beside `fail`.** The SARIF log has no error kind and writes an errored member as `fail`. The document carries the run's three-way verdict, so each member carries the kind that composed it. The SARIF log keeps its kinds, and its contract takes no bump.
- **Why `reason` is a grammar and not a word list.** The adopter asked for every value from a declared set. A closed list of reasons would make the runner judge a consumer's cause, which the omission class refuses by design. The contract declares the grammar, so a reason of another shape is still a contract break. The consumer who writes the registry holds its own word list.

**Not yet applied**: gate-sdk/SPEC.md §The non-gate arm's sentence "`--verify-gate-run` is the member (evidence-kit/SPEC.md §verify-gate-run)." becomes "The members are `--verify-gate-run` (evidence-kit/SPEC.md §verify-gate-run) and `--run` (§run-gates)."

### (3) The goldens {mechanical}

**Not yet applied**: the paragraph of delta 1 ends:

> **The goldens** are `docs/contracts/run/v1/pass.json`, a registry run with a `pass` member, a `notApplicable` member and one omission; `fail.json`, an `only` run with a `fail` member and an `error` member; `error.json`, a `for` run whose verdict is `error`, with a member that resolved nowhere and one that could not be started; and `refusal.json`, the error document. Each is rendered from a fixed provenance block. A unit test in the module that renders the document byte-compares each, so they are fixtures a test holds, never a regenerated projection. They are published at `/contracts/run/v1/`.

The front-end test `gate-sdk/gate-tests/run-json.test.sh` runs `--json` through `bin/run-gates.sh` on a fixture registry with one passing and one failing member. It asserts that stdout parses as one document whose `verdict` is `fail`, that stderr carries the failing member's banner, and that the status is 1.

## Producers and consumers

- **The `--run --json` document.** Producer: the `--run` arm under `--json`. It is reachable in every deployed configuration, since it needs no knob and the stub passes the option through. Consumer: the outside work engine, which reads `verdict`, `scope`, `members` and `omitted` *(inferred: the adopter's reader is out of tree and its need adopter-stated)*. In tree: the goldens' unit test and `run-json.test.sh`.
- **The transcript under `--json`.** Producer: the arm. Consumer: the person at the terminal, on stderr. No in-tree caller passes `--json`, so no transcript reader moves.
- **Point 2, roster-holding readers.** `--emit parse-gates-log` and the overhead meter read the transcript's prefixes from wherever their caller captured it, and every in-tree caller runs without `--json`. `--run-front-end-parity`'s corpus holds the residual grammar, which this adds no form to. The SARIF log's kinds and contract are unchanged.
- **Point 3, sections updated.** §run-gates gains the option, the document and its goldens, and §The non-gate arm's member roster names `--run`. The usage text in `native/src/runner.rs` gains the option.
- **Point 4, every field's reader.** `contract`, `verdict`, `scope`, `members`, `omitted` and `provenance` are each read by the adopter, which accepted this document *(inferred, as above)*. `members.exit` and `members.error` are read by the adopter to tell a refused member from a broken one. The goldens' test reads every key.
- **Point 5, narrowing.** `--run` gains an option and refuses nothing it accepted before. No red condition narrows.
- **Point 6, the corpus.** The members of the `kind`, `error` and `scope` sets are enumerated in delta 2, and each golden of delta 3 carries at least one. `pass.json` and `fail.json` carry two kinds each, and `error.json` carries `error` under both `refused` and `tool`.

## Existing sections updated

Roster probe: `git grep -n 'is the member (evidence-kit\|GATE_SDK_SARIF_RESULTS. selects\|usage: --run' -- gate-sdk/SPEC.md native/src/runner.rs`, over the tracked tree.

- `gate-sdk/SPEC.md`: §run-gates (deltas 1, 2 and 3) and §The non-gate arm's member roster (delta 2).
- `native/src/runner.rs`: the option's parse, the stream routing, the document's rendering and its golden test, and the usage text (deltas 1, 2 and 3).
- `docs/contracts/run/v1/pass.json`: new (delta 3).
- `docs/contracts/run/v1/fail.json`: new (delta 3).
- `docs/contracts/run/v1/error.json`: new (delta 3).
- `docs/contracts/run/v1/refusal.json`: new (delta 3).
- `gate-sdk/gate-tests/run-json.test.sh`: new (delta 3).
- `docs/gate-sdk/SPEC.md`: the generated mirror (all deltas).
- `docs/llms-full.txt`: regenerated (all deltas).

## Retired spellings

- None — the option and its document are new, and no name or value is removed or renamed.

## Definition of Done

- [ ] **Causal completeness**: every point of canon-kit/SPEC.md §The causal-completeness check holds for the document.
- [ ] **Merged with no information lost**: each addition rephrases the canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted**: this file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated**: `## Retired spellings` holds.
- [ ] **Goldens held**: the four goldens byte-compare under the renderer's unit test, and `run-json.test.sh` is green.
- [ ] **Entry moved**: `orchestrator-arm-json` moves to Done before the drain stage, in the commit that lands the merge.
- [ ] **Gaps filed**: cross-component gaps discovered during the work are filed as debt tasks.
