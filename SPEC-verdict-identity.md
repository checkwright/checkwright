# SPEC amendment: verdict-identity

Pairs `verdict-identity-fields`. The battery's SARIF log is the verdict an outside work engine reads, and the engine needs to key it and diff it. To key it, the engine needs the content that was judged and the gates that judged it. To diff it, each failure needs an identity that survives a rerun. This amendment adds a tree object id and a gate-set digest to the provenance block, so the gate-run Statement takes both through its embedded block. It also adds a fingerprint to each fail result. The verdict contract and the gate-run predicate each take a minor bump.

The ask's fourth field, a rule id beneath a member with several rule classes, is not in this amendment. No member in the tree prints a class token on its finding lines today, so the field would have no producer, and the adopter needs no in-tree rule id. It rides the per-pattern rule ids of the adopter-content-checks entry, their first producer.

**What the tree does today**, each bullet with the probe that settled it:

- The provenance block carries `tool.*`, `kits[]`, `commit` and `worktree`, and no tree object id (`grep -n 'insert\|put(' native/src/provenance.rs`; gate-sdk/SPEC.md §run-gates, the provenance table).
- No surface digests the gate set that produced a verdict (`git grep -n -i 'gate.set\|gateSet' -- native/src gate-sdk/SPEC.md` finds no producer).
- `native/src/sarif.rs` writes no `fingerprints` or `partialFingerprints` (`grep -c -i fingerprint native/src/sarif.rs` answers 0). A result carries `ruleId`, `ruleIndex`, `kind`, `level`, `message` and `locations` (`grep -n 'fn result' -A9 native/src/sarif.rs`).
- The verdict contract is `1.0.0` (`grep -n 'CONTRACT' native/src/sarif.rs`). The gate-run predicate's `contract` is `1.0.0`, and its schema types the embedded block with `dependentRequired` tying `commit` to `worktree` (`grep -n 'dependentRequired\|"contract"' docs/contracts/gate-run/v1/schema.json`).
- `--run-validate` builds its Statement's block through the same `provenance::collect` that `--run` uses, so a field added there reaches the Statement with no second producer (`grep -n 'provenance::collect' native/src/runner.rs native/src/emit/run_validate.rs`).
- `--verify-gate-run` check 1 types the block field by field and check 2 compares `provenance.commit` to the subject (`grep -n 'fn provenance_shape' -A50 native/src/evidence.rs`).

## What changes

### (1) The tree object id {design-bearing} {user-facing: the entry's deliverable — a verdict keyed to the content it judged}

The provenance block gains `treeId`, the tree object `HEAD` names. It is present only where `worktree` is `clean`, because a modified worktree's judged content is no tree object at all. A reader keying a verdict to content keys it to `treeId`. That key survives a rebase or an amend that leaves the content alone, where `commit` would change.

**Not yet applied**: gate-sdk/SPEC.md §run-gates' provenance table gains a row after `worktree`:

> | `treeId` | the tree object `HEAD` names, in `commit`'s format | `git rev-parse HEAD^{tree}`; absent unless `worktree` is `clean`, since a modified worktree's content is no tree object |

The opening sentence of **The provenance block** is rephrased to "says which binary, which kits and which gates produced a verdict, and on what tree".

- **Why absent rather than the index's tree.** `git write-tree` would name the index's content, but it writes an object into the repository. A verdict run that writes into `.git` breaks the battery's leave-no-trace invariant (§run-gates, *The projection-ordering constraint*). The index is also not the worktree a member reads.
- **Honest limit.** The key follows `worktree`'s tracked-only read. An untracked file that a member reads is outside `treeId`, as it is outside `worktree`.

### (2) The gate-set digest {design-bearing} {user-facing: the entry's deliverable — a verdict keyed to the gates that judged it}

The provenance block gains `gateSet`, a digest of the gates that judged. Only `--run` fills it, so a `--run-validate` Statement, which selects no gates, carries none.

**Not yet applied**: gate-sdk/SPEC.md §run-gates' provenance table gains a row after `treeId`:

> | `gateSet` | the digest of the gate set that judged: SHA-256, 64 lowercase hex characters, over the newline-terminated lines `gate-set v1`, the run's `scope` word, `tool.sourceStamp` (`-` where absent), then one line per selected member in registry order: `<name> <file> <argv>`, where `<file>` is the SHA-256 of the file the member resolved to (its `.gate` descriptor or its script, `-` for a member resolving nowhere) and `<argv>` is the SHA-256 of its forwarded arguments, each NUL-terminated | the `--run` arm's selection and resolution; absent from every other producer |

- **Why the source stamp.** A compiled member's logic lives in the binary, and its descriptor does not carry it. Two binaries with the same descriptors but different crates are different gate sets.
- **Why the scope word and the argv.** A `--for` run hands a staged member its matching paths. An `--only` run may forward an argument vector. Either way the same members judge a different corpus, so the selection is part of the set's identity.
- **Why the resolved file and not the name.** A consumer's shadowing script, or a descriptor edited to a different command, changes what judges under an unchanged name.
- **Honest limit.** A knob value set in the invoking environment is in neither `treeId` nor `gateSet`. A knob file the tree tracks is in `treeId`.

### (3) A fail result's fingerprint {design-bearing} {user-facing: the entry's deliverable — a failure's identity across reruns}

Each `fail` result carries `fingerprints["checkwright/v1"]`, so a reader can tell a rerun's surviving failures from new ones.

**Not yet applied**: gate-sdk/SPEC.md §run-gates' result list, after the bullet that opens "**A red or unresolved member with no such line**", gains:

> - **A fail result's identity** is `fingerprints["checkwright/v1"]`, the SHA-256 of the newline-terminated lines `checkwright-result v1`, its `ruleId`, its location's `uri`, its text and its ordinal. A finding-line result's text is its line after the `<path>:<line>:[<col>:]` prefix, with leading whitespace stripped, so a finding that moves to another line keeps its identity. A registry-line result's text is empty, because its captured block can carry the run's own scratch paths and timings. The ordinal counts, from 0, the run's earlier results with the same rule, `uri` and text, so two identical findings in one file stay two. A `pass` or `notApplicable` result carries none, because its rule id is its identity. **Honest limit:** a finding whose text embeds a volatile value, such as a count or another site's line, moves its fingerprint, and a finding inserted before an identical one shifts the ordinals after it.

- **Why `fingerprints` and not `partialFingerprints`.** SARIF reserves `partialFingerprints` for inputs a result-management system combines into its own identity, and code scanning computes its own from those. This identity is computed whole by the tool, which is what `fingerprints` holds. The key carries a version, so a change of recipe is a new key and never a silent change of meaning.

### (4) The two contracts' versions, goldens and verifier {mechanical}

**Not yet applied**: in gate-sdk/SPEC.md §run-gates, *The verdict contract's version*:

- "`1.0.0` today" becomes "`1.1.0` today".
- The contract's enumeration gains "a fail result's fingerprint" after "the result kinds and levels".
- A note follows the note for `1.0.0`: "The note for `1.1.0`: the provenance block gained `treeId` and `gateSet`, and a fail result gained its fingerprint."

*The contract's goldens* is rephrased so the fixed provenance block carries `treeId` and `gateSet`. `red.sarif` and `failures.sarif` then move with their fail results' fingerprints, and `green.sarif` and `omitted.sarif` with the block alone. `CONTRACT` in `native/src/sarif.rs` becomes `1.1.0`.

**Not yet applied**: in evidence-kit/SPEC.md §The gate-run predicate, the `contract` row becomes "`1.1.0` at this major". `docs/contracts/gate-run/v1/schema.json` types the block's two new fields:

- `treeId` takes `commit`'s pattern, requires `commit`, and is admitted only where `worktree` is `clean`.
- `gateSet` takes `^[0-9a-f]{64}$`.

`statement.json` moves with the renderer's fixed block, which gains `treeId`. A validate Statement carries no `gateSet`, so the golden carries none.

**Not yet applied**: in evidence-kit/SPEC.md §verify-gate-run:

- Check 1's sentence "the provenance block to gate-sdk/SPEC.md §run-gates' table" stands. `native/src/evidence.rs`' `provenance_shape` gains the two fields' typing, `treeId` beside a `clean` worktree only.
- Check 2 is rephrased to "`provenance.commit` equals it, and `provenance.treeId`, where present, is that commit's tree". A mismatch is a contradiction at exit 1, because a Statement whose block names another tree did not judge its subject's content.

## Producers and consumers

- **`treeId`.** Producer: `provenance::collect`, under every `--run` that writes a SARIF log and every `--run-validate` that renders a Statement. Deployed configurations that set it: this repo's `gates` job, which sets `GATE_SDK_SARIF_FILE` (`.github/workflows/gates.yml`), and the installer's CI action (installer/SPEC.md §The CI action). Consumers: `--verify-gate-run` check 2 (delta 4), the verdict-lookup key (the `verdict-lookup` entry's amendment, which lands after this one), and the outside work engine keying a verdict *(inferred: the adopter's reader is out of tree, its need lead-relayed)*.
- **`gateSet`.** Producer: the `--run` arm in `native/src/runner.rs`, from its selection and its resolved members, which it hands to `provenance::collect`. Deployed: as for `treeId`. Consumers: the verdict-lookup key, `--verify-gate-run` check 1's typing, and the outside engine *(inferred, as above)*.
- **`fingerprints["checkwright/v1"]`.** Producer: `sarif::render` on each fail result. Consumer: the outside engine diffing a rerun's failures *(inferred, as above)*. In-tree, code scanning receives the field and is not known to read it. The `failures` upload is unaffected either way.
- **Point 2, roster-holding readers.** The block's fields land on gate-sdk's provenance table, on the gate-run schema's `provenance` properties, and on `provenance_shape`'s typed field list. Each of the three is updated by delta 1, 2 or 4. The SARIF result's property set is enumerated by the verdict contract's enumeration (delta 4). `--run-consumer-smoke` reads only the contract's major (`grep -n 'verdictContract' native/src/emit/run_consumer_smoke.rs`), and a minor leaves it unchanged.
- **Point 4, every field's reader.** Each of the three fields has a reader in the tree, beside the outside engine: verify check 2 reads `treeId`, check 1 types `gateSet` and the lookup key reads both, and the golden test holds the fingerprint recipe. The fingerprint has no in-tree reader at a transition. It is minted for the adopter's named need (the entry's ask A), with the inference marked above.
- **Point 5, narrowing.** Check 2 narrows: a Statement whose `treeId` names a tree other than its subject's now reds. Every Statement this tree's `--run-validate` writes satisfies it by construction, because both reads happen at the run's start. A hand-built or foreign Statement carrying a wrong `treeId` reds, which is the point. A 1.0.0 Statement carries no `treeId` and is unaffected.
- **Point 6.** None. No enumerable corpus is obliged.

## Existing sections updated

Roster probe: `git grep -n 'provenance block\|verdictContract\|CONTRACT\|1\.0\.0\|dependentRequired\|fn provenance_shape' -- gate-sdk/SPEC.md evidence-kit/SPEC.md native/src/sarif.rs native/src/provenance.rs native/src/evidence.rs native/src/runner.rs docs/contracts`, over the tracked tree.

- `gate-sdk/SPEC.md` §run-gates: the provenance table and its opening sentence (deltas 1 and 2), the result list (delta 3), *The verdict contract's version* and *The contract's goldens* (delta 4).
- `native/src/provenance.rs`: the `tree_id` and `gate_set` fields, their reads, `block()`, and unit tests for absence under a modified worktree (deltas 1 and 2).
- `native/src/runner.rs`: the gate-set digest computed from the selection and handed to `collect` (delta 2).
- `native/src/sarif.rs`: the fingerprint on each fail result, `CONTRACT`, and the golden test's fixed block (deltas 3 and 4).
- `docs/contracts/verdict/v1/green.sarif`, `red.sarif`, `omitted.sarif` and `failures.sarif`: moved with the renderer (delta 4).
- `evidence-kit/SPEC.md` §The gate-run predicate and §verify-gate-run: the `contract` row, the sentence "Predicate `1.0.0` (§The gate-run predicate) has no control field", which becomes "The predicate at this major has no control field", and check 2 (delta 4).
- `native/src/evidence.rs`: `provenance_shape`'s two fields, check 2's tree comparison, and the golden and schema tests (delta 4).
- `docs/contracts/gate-run/v1/schema.json` and `statement.json`: the two fields' typing, the `provenance` description's field list "(tool, kits, commit, worktree)", and the moved golden (delta 4).
- `native/src/emit/verify_gate_run.rs`: check 2's finding for a tree mismatch (delta 4).
- `docs/gate-sdk/SPEC.md`, `docs/evidence-kit/SPEC.md` and `docs/llms-full.txt`: the generated mirrors (all deltas).

## Retired spellings

- None — each field is added under a minor bump, and no name or value is removed or renamed.

## Definition of Done

- [ ] **Causal completeness**: every point of canon-kit/SPEC.md §The causal-completeness check holds for each new field and the narrowed check.
- [ ] **Instruction surfaces: instruction only**: no template or shim is touched.
- [ ] **Merged with no information lost**: each addition rephrases the canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted**: this file is removed on merge (`ls SPEC-*.md`).
- [ ] **Removals propagated**: `## Retired spellings` holds.
- [ ] **Goldens moved and held**: the four verdict goldens, the gate-run schema and its Statement golden are byte-compared green by their unit tests, and `--verify-gate-run` judges this tree's own Statement clean.
- [ ] **Run once on a clean tree**: a bare `--run` with `GATE_SDK_SARIF_FILE` set on a clean worktree writes `treeId` and `gateSet`. A second run on the same tree writes the same `gateSet` and, for any red, the same fingerprints.
- [ ] **Entry moved**: `verdict-identity-fields` moves to Done before the drain stage, in the commit that lands the merge.
- [ ] **Gaps filed**: cross-component gaps discovered during the work are filed as debt tasks.
