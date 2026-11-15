# SPEC amendment: gate-run-predicate

Pairs `gate-run-attestation-predicate`. Today a validate run can be read only in this kit's own manifest format. No attestation verifier or transparency log reads that format, and nothing in it ties a run to the commit it tested or to the command that produced its counts. This amendment publishes a gate-run predicate for an in-toto Statement whose subject is the commit, and has `--run-validate` write one Statement per run for a signer to wrap. It adds a verify arm that checks a Statement against the tree and its commit, and imported runs: a Statement produced on another host or in CI, verified without re-running and recorded in the manifest. The manifest moves to a second major version that carries the commit and the command binding, beside the record `recorded-test-mutation-control` adds. The predicate embeds the provenance block `machine-verdict-contract` defines (gate-sdk/SPEC-verdict-contract.md, delta 1). Any producer may write the Statement, so an external orchestrator that runs its own suites can hand one to the verify arm.

**What the tree does today**, read from `native/src/emit/run_validate.rs`, `native/src/evidence.rs`, `native/src/gates/evidence_manifest.rs`, `native/src/emit/trajectory.rs`, `native/src/installer/recipe.rs`, `native/src/emit/mod.rs`, `native/src/runner.rs`, `scripts/evidence-config.knobs` and `.workflow/validate-evidence.txt`:

- **The manifest is v1 and binds neither commit nor command.** `evidence::MANIFEST_CONTRACT` is `evidence-manifest v1`. `run_validate.rs` writes the eight-field line `<key> <suite> sha256=<log-hash> pass= fail= ignore= verdict= <date>` and nothing else, and §Evidence manifest already rules that binding the command owes a new format version, never a reinterpretation of `sha256=`.
- **`check-evidence-manifest` accepts one header and one shape.** `evidence_manifest.rs` compares the first line against `# contract: evidence-manifest v1` and reds any line that is not exactly eight fields.
- **The fold preserves whatever header it finds.** `fold` drops this key's superseded lines, copies every other line through and publishes by rename. It never reads the header, and neither does the boundary truncation (`enter_stage.rs` keeps the `# contract:` line it finds).
- **The trajectory extractor reads past manifests out of history.** `trajectory.rs`'s `is_attestation` takes a line whose third field starts `sha256=`, and it sums the value after the line's last `fail=` (`rfind`). Both readers see only the first eight fields, so the fields this amendment appends leave them unchanged.
- **The suite command is word-split.** `run_validate.rs`'s `spawn` splits the resolved `EVIDENCE_KIT_RUN_<suite>` value on whitespace and spawns the words with no shell. The effective parser value comes from `evidence::parser_for`.
- **`init` seeds a v1 header.** `recipe.rs` seeds `.workflow/validate-evidence.txt` as `# contract: evidence-manifest v1` when the file is absent, under installer/SPEC.md §What init seeds' create-once discipline. So an upgraded adopter keeps its v1 file.
- **The crate already parses JSON** (`serde_json` is its one non-platform dependency), and it carries no base64 decoder.
- **This tree's manifest holds its header alone** at this iteration's start.

## What changes

### (1) The gate-run predicate {design-bearing} {user-facing: operator direction 2026-10-25, lead-relayed (not a ruling), advancing the hosted-attestation track whose first tree-side unit this is; widened by operator direction 2026-11-15, lead-relayed (not a ruling): a published schema, the provenance block, one manifest major version}

evidence-kit/SPEC.md gains a section, **The gate-run predicate**, after §Evidence manifest. It is the one definition of the Statement this kit writes and reads. **Not yet applied.**

**The Statement** is an in-toto Statement v1, with `_type` `https://in-toto.io/Statement/v1` and `predicateType` `https://checkwright.dev/gate-run/v1`. Its `subject` holds one resource descriptor, `{"digest": {"gitCommit": "<hex>"}}`, the commit the run tested, 40 or 64 lowercase hex characters as in-toto's digest set defines `gitCommit`. A Statement serializes as one JSON object with sorted keys and no insignificant whitespace, so the same run gives the same bytes.

**The predicate** carries these fields. Each is required unless the table says otherwise.

| Field | Holds |
| --- | --- |
| `contract` | the predicate's version, a Semantic Versioning 2.0 string; this change ships `1.0.0` |
| `run` | the run key (§The evidence adapters): the iteration, else `EVIDENCE_KIT_RUN_ID` |
| `provenance` | the provenance block, gate-sdk/SPEC.md §run-gates' one definition, embedded whole and never restated here |
| `suites[]` | one object per suite the run ran, in configured-suite order |
| `suites[].name` | the suite |
| `suites[].command.sha256` | the command digest (below) |
| `suites[].log.sha256` | the captured log's digest, the manifest line's `sha256=` |
| `suites[].counts` | `{pass, fail, ignore}`, integers |
| `suites[].verdict` | `clean` or `new-failures` |
| `suites[].date` | the line's `YYYY-MM-DD` |
| `suites[].tree` | `clean` or `modified`, the worktree state when the suite started |
| `suites[].origin` | `local`, or `imported` for a line delta 5 recorded |
| `suites[].binding.sha256` | the binding digest (below) |

**The command digest** is the SHA-256 of two lines: `run <words>` and `parser <words>`, each newline-terminated. `<words>` is the suite's resolved `EVIDENCE_KIT_RUN_<suite>` value, then its effective parser value, each split on whitespace and re-joined with single spaces, which is the argv the spine spawns. The pre-hook is left out, since it prepares the tree and produces no count. The value is hashed as resolved, so a host that spells the same command differently, such as an absolute `GATE_SDK_NATIVE_BIN`, binds differently. The verify arm reports that as a command mismatch, never as a pass.

**The binding digest** is the SHA-256 of the newline-terminated lines `gate-run-binding v1`, `<suite>`, `<command digest>`, `<commit>`, `<pass> <fail> <ignore>`, `<verdict>` and `<log digest>`. It joins the command, the source commit and the counts in one value that a hash-only log entry can carry. The predicate carries it, but the manifest line does not, because every input is already on the line and the digest is derived from them, never stored.

**The predicate's version follows the verdict contract's rule** (gate-sdk/SPEC.md §run-gates, *The verdict contract's version*): the same major, minor and patch triggers, and the same reader's half. The major is also spelled in `predicateType`, so a `v2` predicate is a new type URI, which a generic in-toto policy can match without parsing `contract`. A reader refuses a type URI or a `contract` major it does not know, and ignores fields it does not know.

**The kit emits and never signs.** The DSSE envelope, its `payloadType` `application/vnd.in-toto+json`, and the signature are the signer's. A signature the kit made would be made by the same process the committing agent runs, which is exactly the identity an attestation has to be independent of, and holding a key would break the adopter constraints (gate-sdk/SPEC.md §The adopter constraints). A keyless, identity-bound signer in the consumer's CI wraps the Statement as it stands. A signer that builds its own Statement takes the `predicate` member alone *(inferred: no signer was run against a Statement; the signer side is outside the tree)*.

**Honest limits.** An unsigned Statement proves its binding is consistent, never that the run happened: anyone holding the fields can recompute every digest. That proof is the signer's. A `tree: modified` suite ran on content that is not the subject commit, and the Statement says so rather than hiding it. The command digest binds the argv and not the programs it names, so a binary that changed under the same argv is visible only through `provenance`.

### (2) The evidence manifest's second major version {design-bearing} {user-facing: operator direction 2026-11-15, lead-relayed (not a ruling): the binding is one manifest major version beside the mutation-control record, with a migration note}

§Evidence manifest's opening paragraph and its first bullet are re-phrased. **Not yet applied.**

- **The header is `# contract: evidence-manifest v2`.** The `v<n>` marker names the major alone, since the manifest has no minor. A line kind every v2 reader skips can be added without moving it.
- **The third field names the line's kind.** A field starting `sha256=` makes a **suite line**, which this section defines. A field starting `control=` makes a **control line**, which `recorded-test-mutation-control` defines. Any other third field is a grammar error.
- **A suite line is twelve fields.** The first eight are v1's, unchanged in order and spelling: `<run-key> <suite> sha256=<log-hash> pass=<n> fail=<n> ignore=<n> verdict=<clean|new-failures> <date>`. Then come `commit=<hex>`, `tree=<clean|modified>`, `cmd=<command-digest>` and `origin=<local|imported>`. `<hex>` is 40 or 64 lowercase hex characters, or `-` on an unborn branch, and `tree=` is `-` exactly when `commit=` is. `<command-digest>` is 64 lowercase hex characters (§The gate-run predicate).
- **`commit=` and `tree=` are read per suite**, after its pre-hook and before its run, by `git rev-parse HEAD` and `git status --porcelain --untracked-files=no`, the reads the provenance block makes. A pre-hook that rewrites a tracked file therefore shows as `tree=modified` on that suite's line alone.
- **The first eight fields stay in place on purpose.** The trajectory extractor reads historical manifests, v1 and v2 alike, through their first eight fields (drift-kit/SPEC.md §The published-evidence extractor). Appending keeps it correct with no edit.

**Migration, the note this major owes.** The writer writes v2 only. `--run-validate`, finding a v1 header at its fold, rewrites the header to v2 and drops every v1 data line, since a v1 line can carry neither a commit nor a command. So a partial run straight after an upgrade leaves the suites it did not run without a line, and §check-evidence-manifest's close-entry assertion asks for a full run before close. That is the right answer, because those suites have no v2 evidence. `check-evidence-manifest` reads both majors, each under its own grammar: a v1 file whose lines are v1 stays green, so an adopter upgraded by `init` is not red before its next run. `init` seeds new trees at v2. The boundary truncation needs no change, because it keeps whatever header it finds, and that header is v2 once the writer has upgraded the file. **Not yet applied.**

### (3) `--run-validate` writes the Statement {design-bearing} {user-facing: operator direction 2026-11-15, lead-relayed (not a ruling): a published predicate whose provenance block is the verdict contract's}

§bin/run-validate.sh gains a paragraph, **The Statement**. **Not yet applied.**

- **`EVIDENCE_KIT_STATEMENT_FILE`** names where the run writes its Statement. The default is empty, and empty writes none, so a consumer selects the output by naming a path, and the empty default is its off. Writing at run time, rather than from an emitter reading the manifest later, is what lets `provenance` describe the binary, the kits and the tree that actually ran.
- **One Statement per invocation, covering the suites it ran.** Its subject is `HEAD` read at the run's start, after the claim, and its provenance block is read at the same point. A run on an unborn branch writes no Statement and says so on stderr. Its exit stays the run's own.
- **Built before the fold, published after it, inside the claim.** The Statement is staged under the tmp dir before the manifest fold, so a staging failure aborts with nothing written, at the guards' exit 2. The manifest publishes first and the Statement second, each by rename. A failed second rename exits 2 and names the published manifest, so a reader knows which of the two exists. The staged file joins the claim's destructor, as the batch file does.
- **A `new-failures` run still writes its Statement.** It attests what happened, and its verdicts are inside it.
- **The declared knob roster** gains `EVIDENCE_KIT_STATEMENT_FILE` and the gate-sdk rows the provenance block reads, and the paragraph's count is re-phrased to match.

**Sibling dependency, both acts.** The provenance block is `machine-verdict-contract`'s `native/src/provenance.rs` (gate-sdk/SPEC-verdict-contract.md, delta 1), and the set record orders that unit first. If it lands in an earlier batch, or earlier in the same batch, this delta calls that module. If it has not landed when this delta's batch is cut, this delta waits for it and never ships a second definition of the block. The predicate has no form without `provenance`.

### (4) The verify arm {design-bearing} {user-facing: operator direction 2026-11-15, lead-relayed (not a ruling): a verify arm that checks a Statement against the tree and its commit}

evidence-kit/SPEC.md gains a section, **verify-gate-run**, after §bin/diff-baseline.sh. **Not yet applied.**

**`--verify-gate-run [<file> | -]`** is an `Arm::Run`. It reads one Statement, or one DSSE envelope whose `payloadType` is `application/vnd.in-toto+json` and whose base64 `payload` is a Statement, from the file, from stdin for `-`, or, given no positional, from the stdout of the command `EVIDENCE_KIT_STATEMENT_CMD` names. That is the command-knob seam (gate-sdk/SPEC.md §The knob file), so a Statement stored outside the tree is fetched by a command the consumer owns. With no positional and an empty knob, the arm refuses at exit 2, naming both. It checks, in order:

1. **Shape.** `_type`, `predicateType`, and a `contract` major this binary knows. A failure is exit 2: the input is not a Statement this reader can judge.
2. **The subject.** The `gitCommit` names a commit in this repository, and it is `HEAD` or an ancestor of it (`git merge-base --is-ancestor`). `provenance.commit` equals it.
3. **Each suite's binding.** `binding.sha256` recomputes from the suite's own fields.
4. **Each suite's command.** `command.sha256` equals the digest of the suite as this tree resolves it. A suite this tree does not configure is a contradiction.
5. **Each suite's tree.** `tree` is `clean`.

Exit 0 means every check held. Exit 1 means a contradiction, with one line per finding naming the suite and the field. Exit 2 means the Statement could not be read or judged. The verdict line names the subject, the suite count, and how many commits `HEAD` is ahead of the subject, so a reader that wants a run of `HEAD` itself reads `ahead=0`. **A DSSE signature is never checked:** the line says `signature not checked`, because a signature is checked by the signer's verifier against the signer's identity policy, which this kit does not hold (delta 1). The arm writes nothing.

**Any producer may write the Statement.** The arm judges its fields and never asks who wrote it, so a Statement from an external orchestrator, from CI or from another host verifies on the same terms as this tree's own. That is why the schema is published (delta 6).

### (5) Imported runs {design-bearing} {user-facing: operator direction 2026-11-15, lead-relayed (not a ruling): a Statement produced on another host or in CI, bound by a hash of the command, the source commit and the counts, verified without re-running}

evidence-kit/SPEC.md gains a section, **import-gate-run**, after verify-gate-run. **Not yet applied.**

**`--import-gate-run [<file> | -]`** is an `Arm::Run` that takes the verify arm's input forms and runs its checks. On exit 0 from those checks it records each suite as a manifest line with `origin=imported`, under this tree's current run key, with the Statement's counts, verdict, date, log digest, commit, tree and command digest. It writes the manifest as the spine does: it claims the producer-liveness lock (§The producer-liveness lock), folds by the spine's supersede rule and publishes by rename, so an import and a validate run can never race. Exit 0 means recorded. Exit 1 means a verify contradiction, with nothing written. Exit 2 means a refusal, a held lock among them.

**Off unless the consumer selects it.** `EVIDENCE_KIT_IMPORTED_RUNS` takes `refuse`, the default, or `accept`. Under `refuse` the arm exits 2 before reading, naming the knob. An imported line counts toward §check-evidence-manifest's close-entry green block like a local one. A consumer that never selects `accept` therefore never has one, which is why the selection sits on the import and not on the gate.

**Honest limit.** An imported line is as good as the Statement it came from. Unsigned, it proves that its command, commit and counts agree with each other and with this tree's configuration, and never that the run happened. A consumer accepting imports verifies the signature with its signer's tooling before it imports. The line's `origin=imported` keeps that visible to every later reader.

### (6) The published schema and the golden {mechanical}

`docs/contracts/gate-run/v1/` holds `schema.json`, a JSON Schema (draft 2020-12) for the Statement with the predicate's fields, and `statement.json`, a Statement rendered from a fixed manifest batch and a fixed provenance block. The site publishes them at `/contracts/gate-run/v1/`, beside the verdict contract's goldens. A crate integration test, `native/tests/gate_run_contract.rs`, renders the golden and byte-compares it. It also reads `schema.json` and asserts that the predicate's property names and `required` set equal the field set the renderer writes, so neither file moves without the other. Like the verdict goldens, these are fixtures held by a test, never a regenerated projection. **Not yet applied.**

### (7) The readers {mechanical}

Each **Not yet applied.**

- **`check-evidence-manifest`** (§check-evidence-manifest): assertion (B) reads the header's major and applies that major's grammar: v1's eight fields, or v2's twelve-field suite line and the line-kind rule. Its help line names both. Assertion (A) takes a `clean` suite line whatever its `origin=`. `evidence::MANIFEST_CONTRACT` becomes the v2 marker, with the v1 marker kept as the one legacy name the gate accepts.
- **The fold** (`run_validate.rs`): it writes the twelve-field line and performs the v1-to-v2 rewrite delta 2 states.
- **`init`'s seed** (`native/src/installer/recipe.rs`): it seeds `# contract: evidence-manifest v2`.
- **The arm table and its rosters** (`native/src/emit/mod.rs`, gate-sdk/SPEC.md §The non-gate arm): the two new `Arm::Run` members join the forced-out list, each by the distinction it carries (verify: 1 a contradiction and 2 an unreadable Statement; import: 1 nothing recorded on a contradiction and 2 a refusal). Both join the network spawners, and `no_network_spawning_arm_is_fence_safe`'s list, because `EVIDENCE_KIT_STATEMENT_CMD` names a consumer command that may fetch. Both join the program roster's *changeable by a consumer* sentence, spawning `git` and whatever that knob names. `native/src/runner.rs`'s `USAGE` gains a line and a paragraph for each.
- **The knob table**: `native/src/knobs/evidence_kit.rs` gains `EVIDENCE_KIT_STATEMENT_FILE` (scalar, default empty), `EVIDENCE_KIT_STATEMENT_CMD` (a command knob, default empty) and `EVIDENCE_KIT_IMPORTED_RUNS` (`refuse` or `accept`, default `refuse`, any other value refused by the table validator). §Layout and configuration's list and `evidence-kit/templates/evidence-config.knobs` gain them.
- **§Evidence manifest's wire-format sentences**: *the versioned wire format the deferred hosted-attestation service consumes as its attestation payload* is re-phrased to name the Statement (§The gate-run predicate) as the payload, with the manifest as the record it is built from.
- **§Producers and consumers**: the evidence-line bullet re-phrases its *forward, by the hosted-attestation payload* to name the Statement writer, the verify arm and the import arm, and it gains bullets for the Statement, the command digest and the imported line.
- **`evidence-kit/README.md`**: the header literal moves to v2, and the tool sentence names the two arms.
- **`evidence-kit/smoke/install.sh`**: it seeds the v2 header, and it adds one arm writing a Statement through `EVIDENCE_KIT_STATEMENT_FILE` and verifying it at exit 0.
- **This tree's `.workflow/validate-evidence.txt`**: its header moves to v2 in the commit that lands the writer, since it holds no data line.

### (8) Cases, the release declaration and the mirrors {mechanical}

Each **Not yet applied.**

- **`gate-tests/check-evidence-manifest.test.sh`** and the gate's `good/`/`bad/` pair: the pair moves to v2. The test adds a v1 file that is green, a v1 header over a v2 line that is red, an unknown third field that is red, a malformed `commit=`, `tree=`, `cmd=` or `origin=` that is red, and an imported `clean` line satisfying (A).
- **The other seeding cases**: `pre-hook.test.sh`, `producer-lock.test.sh`, `run-member-door.test.sh` and `run_validate.rs`'s fold tests take the v2 header. One fold test asserts the v1-to-v2 rewrite.
- **Crate unit tests**: the command digest's whitespace normalization, the binding digest over a fixed line, DSSE unwrapping, each verify check's red, an import under `refuse`, and `trajectory.rs`'s `is_attestation` and fail sum over a v2 line.
- **The release declaration** (`.workflow/release-declarations.md`). Under `## Behavior changes`: the evidence manifest moves to `evidence-manifest v2`; a run can write an in-toto Statement under `https://checkwright.dev/gate-run/v1`; and the new `--verify-gate-run` and `--import-gate-run` arms. Under `## Knob changes`: the three knobs. Under `## Upgrading`: the next `--run-validate` rewrites a v1 manifest at v2 and drops its v1 lines, so run the full roster before a close.
- **The regenerated surfaces**: the mirrors `docs/evidence-kit/SPEC.md`, `docs/evidence-kit/README.md` and `docs/gate-sdk/SPEC.md`, and the graph artifact where a descriptor moved, each by the command docs/generated-projections.md rosters for it.

## Producers and consumers

- **The Statement** (deltas 1 and 3). Producers: `--run-validate`, under `EVIDENCE_KIT_STATEMENT_FILE`, and any outside producer that writes the published shape. Consumers: the verify and import arms, a signer, and an attestation verifier or log.
- **The command digest and the binding digest** (delta 1). Producers: the spine, at each suite line, and the Statement writer. Consumers: the verify arm's checks 3 and 4, and the import arm through them.
- **The v2 suite line** (delta 2). Producer: the spine and the import arm. Consumers: `check-evidence-manifest` (A and B), the trajectory extractor through its first eight fields, the Statement writer, and the merge-supersede rule, which is unchanged. Every new field has a reader. `commit=` is read by the Statement's subject and by the import's ancestry check. `tree=` is read by verify check 5. `cmd=` is read by verify check 4, and `origin=` by every reader that must not mistake an imported line for a local one.
- **`EVIDENCE_KIT_STATEMENT_CMD`** (delta 4). Producer: consumer config. Consumers: the verify and import arms.
- **`EVIDENCE_KIT_IMPORTED_RUNS`** (delta 5). Producer: consumer config. Consumer: the import arm.
- **The schema and the golden** (delta 6). Producer: the commit that changes the predicate. Consumers: `native/tests/gate_run_contract.rs`, and outside producers and readers through the site.

The manifest's readers were found by `git grep -n "evidence-manifest v1"` and `git grep -ln "validate-evidence"` over the whole tracked tree, and the line parsers by `git grep -n "verdict=\|\"sha256=\|pass=" -- native/src`. That located the gate, the fold, the trajectory extractor, the installer seed, the kit's tests and smoke, and the README. The network-spawner roster was found by `git grep -n -i "network spawner\|NETWORK_ARMS" -- native/src gate-sdk/SPEC.md`.

## Existing sections updated

- `evidence-kit/SPEC.md` §Layout and configuration (delta 7), §Evidence manifest (deltas 2 and 7), the new §The gate-run predicate (delta 1), §bin/run-validate.sh (delta 3), the new §verify-gate-run and §import-gate-run (deltas 4 and 5), §check-evidence-manifest (delta 7), §Producers and consumers (delta 7).
- `gate-sdk/SPEC.md` §The non-gate arm, its forced-out list, its network spawners and its program roster (delta 7).
- `native/src/evidence.rs`, `native/src/emit/run_validate.rs`, `native/src/gates/evidence_manifest.rs`, the two new arm modules, `native/src/emit/mod.rs`, `native/src/runner.rs`, `native/src/knobs/evidence_kit.rs`, `native/src/installer/recipe.rs` and `native/src/emit/trajectory.rs`'s tests, with their cases, and the new `native/tests/gate_run_contract.rs` (deltas 3, 4, 5, 6, 7, 8).
- `evidence-kit/README.md`, `evidence-kit/templates/evidence-config.knobs`, `evidence-kit/smoke/install.sh`, the kit's `gate-tests/`, `docs/contracts/gate-run/v1/`, `.workflow/validate-evidence.txt`, `.workflow/release-declarations.md`, and the regenerated surfaces delta 8 lists (deltas 6, 7, 8).

## Retired spellings

- None — no knob, arm or field is renamed. The v1 header stays a name the gate accepts, so it is not retired.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls evidence-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The local oracle** — all green: `bash gate-sdk/bin/build-native.sh`, the battery, the crate's tests including `gate_run_contract`, the fixture suites of evidence-kit, gate-sdk and drift-kit, `--run-consumer-smoke`, and `--upgrade-smoke`, which carries a v1 manifest across the upgrade. The entry's Done move lands before the drain stage, in the build batch that lands it.
