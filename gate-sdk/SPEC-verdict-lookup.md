# SPEC amendment: verdict-lookup

Pairs `verdict-lookup`. An outside work engine wants to skip a battery run whose verdict it already holds. To do that it needs a verdict stored under a key naming what was judged, and a way to ask for it. This amendment has `--run` write a judged verdict into a store the consumer names. A new arm, `--verdict-lookup`, answers a key with the stored verdict or with `absent`. The key is the three identity fields `verdict-identity-fields` adds: the tree object id, the binary's version and the gate-set digest. The arm's statuses and the run's `error` verdict are the ones `exit-code-tool-error` adds, so this amendment lands after both.

**What the tree does today**, each bullet with the probe that settled it:

- No surface stores a verdict, and no arm reads one back. The SARIF log is written to one path and overwritten each run (`git grep -n -i 'verdict.store\|verdict-lookup' -- native/src gate-sdk/SPEC.md` finds no producer).
- gate-sdk sets six names in the environment only, and a knob file naming one is refused (gate-sdk/SPEC.md §Layout and configuration, the paragraph opening **gate-sdk's knobs live in the crate's table**; `grep -n 'env_only' -A12 native/src/knobs/gate_sdk.rs`).
- The non-gate arm rule has an arm print for a person by default and gain JSON only through a `--json` option (gate-sdk/SPEC.md §The non-gate arm, the paragraph **A machine form is minted per arm**).
- A bare-flag arm is an `Arm::Run` row, since an arm's spelling and its variant are one decision (gate-sdk/SPEC.md §The non-gate arm, its first bullet).

## What changes

### (1) The store and the run's write {design-bearing} {user-facing: the entry's deliverable — a judged verdict kept under its key}

**Not yet applied**: gate-sdk/SPEC.md §run-gates gains, after the paragraph on `GATE_SDK_SARIF_RESULTS`:

> **`GATE_SDK_VERDICT_STORE` keeps judged verdicts for `--verdict-lookup`.** It is a reporting knob and environment-only, like the SARIF log's path. Unset or empty, nothing is stored. Set, it names a directory by absolute path, and a relative one is refused at exit 2 before the first member runs. A run stores its verdict where four things hold: its scope is `registry`, its verdict is `pass` or `fail`, and its provenance block carries `treeId`, `tool.version` and `gateSet`. An `error` run, a selection and a modified worktree store nothing. The record is one object, keys sorted and no insignificant whitespace: `contract`, `1.0.0` under this section's verdict-contract rule, `verdict`, and `provenance`, the run's provenance block whole. Its file is the SHA-256, in lowercase hex, of the newline-terminated lines `verdict-key v1`, the `treeId`, the `tool.version` and the `gateSet`, followed by `.json`. The arm creates the directory where it is missing. It writes the record to a temporary file in that directory, then renames it over the key, so a reader never sees a partial record and a rerun of the same key replaces it. The write follows the SARIF log. A store the arm cannot write is exit 3, naming it, after the verdict lines print.

- **Why the key is the three fields.** `treeId` is the content judged, `gateSet` the gates and their resolved files, and `tool.version` the binary that ran the compiled ones. A rerun with all three equal judges the same thing, so its verdict is the stored one. The key's honest limits are the fields' own: a knob set in the invoking environment and an untracked file a member reads are in none of them (gate-sdk/SPEC.md §run-gates, the `treeId` and `gateSet` rows).
- **Why only a judged registry run.** A selection judges part of the gate set and would answer for the whole. An `error` run judged nothing, and stored it would be served until the gate set changed. A modified worktree has no tree object to key.
- **Why a digest as the file name.** `tool.version` is the consumer's release stamp, and a digest keeps any stamp to one fixed, portable file name. The record's own provenance carries the three fields, so the lookup checks them (delta 2).
- **Honest limit.** The store is never pruned. Every key a run judged stays until the consumer removes it, since the arm cannot tell an unused key from a key a later lookup wants.

**Not yet applied**: gate-sdk/SPEC.md §Layout and configuration's knob roster gains, after `GATE_SDK_SARIF_FILE`, "`GATE_SDK_VERDICT_STORE` (default unset = none kept): the directory `--run` keeps judged verdicts in and `--verdict-lookup` reads (§run-gates). Environment-only, because it is how the battery reports." The knob-table paragraph's "Six names are set in the environment only" becomes "Seven names", and its list of execution settings gains `GATE_SDK_VERDICT_STORE`. §The knob file's "gate-sdk lists six" becomes "gate-sdk lists seven", its "the four execution settings" becomes "the five execution settings", and that list gains the knob the same way.

### (2) The lookup arm {design-bearing} {user-facing: the entry's deliverable — the arm the adopter calls}

**Not yet applied**: gate-sdk/SPEC.md §run-gates gains, after delta 1's paragraph:

> **`--verdict-lookup <tree> <kit-ver> <digest>` answers a key from the store.** It is an `Arm::Run` row, spelled as its caller names it, and it runs with no repository. `<tree>` takes `treeId`'s pattern, `<kit-ver>` is a `tool.version` value, a non-blank token of ASCII letters, digits, `.`, `+` and `-`, and `<digest>` takes `gateSet`'s pattern. It prints one document on stdout, keys sorted and no insignificant whitespace, with no `--json` option: `contract`, `1.0.0` under this section's verdict-contract rule, and `verdict`, the stored run's `pass` or `fail`, or `absent` where the store holds no record for the key.
>
> | Case | Status | Document |
> | --- | --- | --- |
> | a record found, a `fail` verdict included | 0 | `verdict` the stored one |
> | no record, or no store directory yet | 0 | `verdict` `absent` |
> | an operand of the wrong count or shape | 2 | the error document, `usage` |
> | `GATE_SDK_VERDICT_STORE` unset, empty or relative | 2 | the error document, `refused` |
> | a store or record that cannot be read, or a record that does not parse, carries another contract major or whose provenance names another key | 3 | the error document, `input` |
>
> The arm never exits 1, because a stored `fail` is an answer and not a finding against the caller. **The goldens** are `docs/contracts/verdict-lookup/v1/pass.json`, `absent.json` and `error.json`, the last an unreadable record at exit 3. A unit test in the arm's module byte-compares each, and they are published at `/contracts/verdict-lookup/v1/`.

- **Why a found `fail` exits 0.** The caller asked whether a verdict exists. A stored failure is that verdict, and the caller reads it in `verdict`. A non-zero status would mean the lookup itself did not answer.
- **Why the record's provenance is checked.** The file name is a digest, and a record copied or edited under another key would answer for a key it never judged. Comparing the record's three fields to the operands costs one parse, which the arm makes anyway to read `verdict`.
- **Why an unset store is a refusal.** The caller asked a store that is not configured. Answering `absent` would claim the key was never judged, and a caller trusting it would rerun every time without learning why.
- **Why the arm reads no repository.** The key carries everything the lookup needs, so the caller can ask before it checks a tree out.

**Not yet applied**: gate-sdk/SPEC.md §The non-gate arm's sentences "An arm prints for a person by default. It gains a JSON form only where a named caller reads the result, and the form is a `--json` option on that arm, never a sibling arm" are rephrased to:

> An arm prints for a person by default, and gains a JSON form only where a named caller reads the result. Where it has a person reader, the form is a `--json` option on that arm, never a sibling arm, so the read, the checks and the exit statuses stay one implementation. An arm minted for a machine caller alone, with no person reader, prints its document by default and takes no option: `--verdict-lookup` is the one such arm.

The same paragraph's member sentence gains `--verdict-lookup` (§run-gates).

### (3) The arm's registration and tests {mechanical}

The arm takes a row in the arm table (`native/src/emit/mod.rs`) and a `spec:` binding to §run-gates. Its declared knob is `GATE_SDK_VERDICT_STORE`, which `native/src/knobs/gate_sdk.rs` lists among its environment-only names. The store's key and record live in a new module, `native/src/verdict_store.rs`, which the run's write and the arm's read share, so the key is computed in one place.

The front-end test `gate-sdk/gate-tests/verdict-store.test.sh` runs on a fixture registry in a scratch repository with the store set. It runs the battery on a clean tree and reads the key from the run's `--json` provenance. It asserts that `--verdict-lookup` answers the run's verdict, that a second digest answers `absent`, and that a run on a modified tree stores nothing. It also asserts that an unset store exits 2 with the `refused` document.

## Producers and consumers

- **The store's records.** Producer: the `--run` arm, where `GATE_SDK_VERDICT_STORE` is set. That is reachable in every deployed configuration, since the knob is the consumer's to set. Consumer: `--verdict-lookup`.
- **The lookup document.** Producer: `--verdict-lookup`. Consumer: the outside work engine, which reads `verdict` and branches on the status *(inferred: the adopter's reader is out of tree, its contract adopter-stated)*. In tree: the goldens' unit test and `verdict-store.test.sh`.
- **The knob.** Producer: the consumer's environment. Readers: the `--run` arm, for the write and the refusal of a relative path, and `--verdict-lookup`. The adopter sets it to a path outside any work tree *(inferred, adopter-stated)*.
- **Point 2, roster-holding readers.** `check-docs-cmd` counts every environment-only name among its static names, so it reads the new name from the knob table with no edit. The knob-file derivation leaves environment-only names out, so no knob file gains a line.
- **Point 4, every field's reader.** `contract` and `verdict` are the document's two keys, and the outside engine reads both. The record's keys are `contract`, `verdict` and `provenance`, and the lookup reads each: `contract` to refuse a record of another major, `verdict` to answer, and the provenance block's three key fields to check the record against the operands. The rest of the block rides whole, as every embedding of it does, so the record needs no shape of its own.
- **Point 5, narrowing.** A relative `GATE_SDK_VERDICT_STORE` refuses a run, and that is the only new red condition. It reaches only a consumer who set the new knob. No existing configuration narrows.
- **Point 6, the corpus.** The lookup's five cases are each held, three by the goldens and two by `verdict-store.test.sh`'s refusal and `absent` legs. The run's storing conditions are each held by a leg: a clean registry run stores, and a modified tree stores nothing. A selection and an `error` run are held by the module's unit test.

## Existing sections updated

Roster probe: `git grep -n 'Six names\|gate-sdk lists six\|GATE_SDK_SARIF_FILE. (default\|A machine form is minted per arm\|GATE_SDK_SARIF_RESULTS. selects' -- gate-sdk/SPEC.md native/src/knobs/gate_sdk.rs`, over the tracked tree.

- `gate-sdk/SPEC.md`: §Layout and configuration's knob roster and knob-table paragraph (delta 1), §The knob file (delta 1), §run-gates (deltas 1 and 2), and §The non-gate arm (delta 2).
- `native/src/verdict_store.rs`: new, the key and the record (delta 3).
- `native/src/runner.rs`: the store write and the relative-path refusal (delta 1).
- `native/src/emit/mod.rs`: the arm's row (delta 3).
- `native/src/knobs/gate_sdk.rs`: the environment-only name (delta 3).
- `docs/contracts/verdict-lookup/v1/pass.json`: new (delta 2).
- `docs/contracts/verdict-lookup/v1/absent.json`: new (delta 2).
- `docs/contracts/verdict-lookup/v1/error.json`: new (delta 2).
- `gate-sdk/gate-tests/verdict-store.test.sh`: new (delta 3).
- `docs/gate-sdk/SPEC.md`: the generated mirror (all deltas).
- `docs/llms-full.txt`: regenerated (all deltas).

## Retired spellings

- `Six names are set in the environment only` — the count becomes seven (delta 1).
- `gate-sdk lists six` — the count becomes seven (delta 1).

## Definition of Done

- [ ] **Causal completeness**: every point of canon-kit/SPEC.md §The causal-completeness check holds for the store, the knob and the lookup document.
- [ ] **Merged with no information lost**: each addition rephrases the canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted**: this file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated**: `## Retired spellings` holds.
- [ ] **Goldens held**: the three goldens byte-compare under the arm's unit test, and `verdict-store.test.sh` is green.
- [ ] **Entry moved**: `verdict-lookup` moves to Done before the drain stage, in the commit that lands the merge.
- [ ] **Gaps filed**: cross-component gaps discovered during the work are filed as debt tasks.
