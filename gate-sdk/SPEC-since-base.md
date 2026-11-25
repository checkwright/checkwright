# SPEC amendment: since-base

Pairs `stale-test-literals` and `migration-guard`. Two of the adopter's checks judge a branch against the point it left its base, never the tree alone: a source string the branch removed while a test still asserts it, and a database migration the branch edited, reordered or made destructive. This amendment adds the shared base resolution, a listing arm for stale test literals, and check-migration-guard.

**What the tree does today**, each bullet with the probe that settled it:

- No content gate reads a since-base corpus. check-gate-tamper's `--range <base>..<head>` is the one range reader, over gate sources (`grep -n 'range' native/src/gates/gate_tamper.rs`).
- No knob names a base ref (`grep -n 'BASE_REF' native/src/knobs/*.rs` finds none).

## What changes

### (1) The base {design-bearing} {user-facing: the adopter input on both entries — a base ref, the working tree included}

**Not yet applied**: §Layout and configuration's rule rosters gain `GATE_SDK_BASE_REF` (default `main`). A new section, §The since-base corpus:

> A since-base reader takes `--base <ref>`, else `GATE_SDK_BASE_REF`, and compares the merge base `git merge-base <ref> HEAD` against the **working tree**, so an uncommitted edit counts. A ref that does not resolve, or shares no merge base with `HEAD`, refuses at exit 2; outside a work tree is exit 2; a git failure reading a resolved commit is exit 3.

- **Why the working tree.** The adopter runs both checks before it commits, so the change under review is not yet a commit.
- **Why a knob default.** A branch name is a consumer's convention; `main` is the common default and a consumer whose base is another branch binds it.

### (2) The stale-literals arm {design-bearing} {user-facing: the adopter input on `stale-test-literals`}

**Not yet applied**: §The since-base corpus continues:

> **`--emit stale-literals [--base <ref>] [--tools <dir>]`**, an `Arm::Emit` member (§The non-gate arm), lists test lines that may assert a string the branch removed. It judges nothing: it exits 0 whatever it lists, 2 or 3 only on its own failures.
>
> - **A literal** is a double-quoted string or a Rust raw string (`r#"…"#`, `br#"…"#`) in any file, and a single-quoted string too in a `.sh` or `.py` file, outside comments (`//` and `/* */` in a `.rs` file, `#` in a `.sh` or `.py` file). A literal is split into **fragments** at each placeholder (`{…}`, `$x`, `${…}`, `$(`, `%s`) and each escape, and a fragment counts at 8 or more characters holding at least 4 ASCII letters.
> - **Test code** is §The test-code classifier's.
> - **A changed fragment** is one on a non-test line the branch removed, unless some non-test line in the working tree still holds it, or the branch adds it as many times as it removes it, so a moved or re-wrapped string is no change.
> - **It lists** each test line present at the merge base and unchanged since, whose literal overlaps a changed fragment, either containing the other, as `LITERAL <test>:<line> (was <src>:<line>)`. A test line the branch added, and a literal that is an assertion's message (the argument of `.expect(` or `panic!(`, or an `assert…!` argument after its operands), are never listed.
> - `--tools <dir>` adds a test suite outside the repository: every file under it is test code at its current content.
>
> Its caller is a session reviewing a branch before it merges, the adopter's among them; a session reaching an arm through the front end counts as a stage step does.

- **Why an arm and not a gate.** The output is a worklist for a person and the adopter wants no verdict from it: a gate that never reds is a report, and §The non-gate arm is that class.
- **Honest limit.** Overlap is textual: a test asserting a reformatted message, or one built from fragments shorter than the floor, is not listed, and a common phrase overlaps unrelated tests.

### (3) check-migration-guard {design-bearing} {user-facing: the adopter input on `migration-guard`}

**Not yet applied**: §Layout and configuration's rule rosters gain `GATE_SDK_MIGRATION_DIR` (default empty, which disables the gate) and `GATE_SDK_MIGRATION_DESTRUCTIVE` (default `<gates-dir>/migration-destructive.list`, **optional**). A new section, §check-migration-guard:

> `checks/check-migration-guard.gate` (`precommit`, binary-dispatched). Invariant: a branch only appends migrations, in order, and none it adds is destructive unless declared. Its corpus is the files in `GATE_SDK_MIGRATION_DIR` at the merge base and in the working tree (§The since-base corpus). Every file there is named `NNNN_<name>.sql`, four digits; any other file is unparseable input at exit 3, naming it.
>
> - **Order.** A file added since the merge base is numbered above the merge base's highest: `<path>: [MIGRATION-ORDER] <added> <= <highest>`.
> - **Append only.** A file present at the merge base is byte-identical in the working tree; an edited, deleted or renamed one reds `<path>: [MIGRATION-EDITED]`.
> - **Not destructive.** In an added file, each statement (text up to its `;`), with `--` and `/* */` comments and quoted strings stripped and ASCII upper-cased, is matched against each live ERE of the destructive list; a match reds `<path>:<line>: [MIGRATION-DESTRUCTIVE]` at the statement's first line. A comment `-- migration-ok: <reason>` on any line of the statement clears it; an empty reason is itself refused.
>
> The kit ships `templates/migration-destructive.list` holding `DROP (TABLE|INDEX|VIEW|TRIGGER)`, `ALTER TABLE .* DROP` and `ALTER TABLE .* RENAME`. An absent list turns the destructive half off and the clean line says so. With the knob empty it prints `not applicable: no migration directory configured`; it declares `# install: on-surface` and `# armed-by: GATE_SDK_MIGRATION_DIR`.
>
> Its pair needs a history, so the cases are built in `check-migration-guard.test.sh` in a sandbox repository: an edit, a delete, a rename, a misnumbered addition, a destructive statement with and without a reason, a stray file at 3, and an unresolved base at 2. The `good/`+`bad/` pair holds the not-applicable clean and a tree whose base is its own `HEAD`.

- **Why the statement list is a file.** Which statements a project treats as destructive is its schema policy; the template is the common floor (the policy-as-choice rule).
- **Honest limit.** A deleted file's record names a path with no file, which §run-gates places at the registry line; and a destructive effect spelled outside the list, an `UPDATE` without `WHERE` say, passes.

## Producers and consumers

- **The base.** Producer: `--base` or `GATE_SDK_BASE_REF`. Consumers: the stale-literals arm and check-migration-guard (deltas 2 and 3).
- **The stale-literals arm.** Producer: the front end on a session's call. Deployed caller: the orchestrator adopter's pre-merge review *(inferred: its configuration is out of tree, lead-relayed)*, and any session here reviewing a branch. Its arm-table row owes a test module (§The non-gate arm).
- **check-migration-guard.** Producer: the battery, armed where `GATE_SDK_MIGRATION_DIR` is set. Deployed: the orchestrator adopter *(inferred, as above)*; this repo has no migrations, so the gate is registered and not applicable here, and `doctor` names it disarmed. Consumers: the committing session and the SARIF log.
- **Roster readers of the new members:** `gates::REGISTRY`, the arm table `ARMS`, `check-readme-roster` (gate-sdk/README.md's roster block), `check-install-disposition`, `check-gate-substrate-parity`, `scripts/gates.list`, the knob rows in `native/src/knobs/gate_sdk.rs`, and `templates/migration-destructive.list` through the payload and `init`'s config-seam plan.

## Existing sections updated

- gate-sdk/SPEC.md §Layout and configuration, rule rosters — the three knobs (deltas 1 and 3).
- gate-sdk/SPEC.md §The since-base corpus, new (deltas 1 and 2).
- gate-sdk/SPEC.md §check-migration-guard, new (delta 3).
- `native/src/knobs/gate_sdk.rs` (deltas 1 and 3); the arm's module and `ARMS` row (delta 2); `native/src/gates/migration_guard.rs`, `checks/check-migration-guard.gate`, its pair and test, `templates/migration-destructive.list`, `gates::REGISTRY`, gate-sdk/README.md's roster block and `scripts/gates.list` (delta 3).
- `docs/gate-sdk/SPEC.md`, the generated mirror (all deltas).

## Retired spellings

- None — a new arm, a new gate and three knobs; nothing existing is renamed or removed.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
