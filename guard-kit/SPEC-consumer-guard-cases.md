# SPEC amendment: consumer-guard-cases

**A consumer's guard rules have no verification lane, and this repository's three most destructive guards are the ones nothing runs.** guard-kit/SPEC.md §Consumer rules names the gap and leaves it open: the generic lane carries the decision table, and "a consumer's command is reachable by no table in this kit, so it ships hand-verified". `scripts/guard-rules.sh`, this repository's rule command, blocks `git commit --no-verify` (and `-n`), any command naming the harness's temporary path prefix `/tmp/claude-`, and `git clean` with `-x` or `-X`. No row in any table exercises them. `guard-kit/gate-tests/consumer-rules.test.sh` tests the member's consumer-stage *protocol* with a scratch rule of its own, and never runs a consumer's command. An adopter writing a rule has no way to prove it fires.

**The arm already has everything the lane needs but the table.** `--run-guard-tests` builds a payload per row, classifies a verdict with a five-way ladder, and fails on a mismatch in either direction (§Testing). What it lacks is a row source the consumer owns and a subject the consumer wrote.

**The subject is the consumer's command, spawned directly, and not the member.** Two alternatives were weighed and refused:

- **The member run with the consumer's knob file.** A consumer's command is written for its repository, and this repository's is `bash scripts/guard-rules.sh`, a cwd-relative path that sources `gate-sdk/lib/gate.sh`. It runs only at the repository root, where the kit lane's sandbox is not. Run at the root, the member's non-firing verdict is whatever the generic ruleset makes of the live tree (its liveness records, its tracked set, its settings file), which a table row cannot pin. A non-firing row would then assert the generic ruleset's verdict rather than the consumer's silence.
- **A second table grammar with a column for the expected generic verdict.** It would couple every consumer's table to the kit's rule changes, so a kit release could red a consumer's lane for a rule the consumer never wrote.

Spawning the command directly asserts exactly what the consumer owns: its decision, or its silence. Placement ahead of the generic ruleset stays `consumer-rules.test.sh`'s subject, where it already is.

**Measured at authoring.** Ten proposed rows were fed to `bash scripts/guard-rules.sh` as Bash payloads from the repository root, with `GATE_SDK_NATIVE_BIN` exported and the verdict classified on the ladder. All ten matched: five `block` and five `fallthrough`, among them a quoted `--no-verify` and a heredoc body naming `git clean -x`, which the rule's `sq dq hd` view leaves inert.

## What changes

### (1) `GUARD_KIT_CONSUMER_CASES` names the consumer's case table

guard-kit gains the scalar knob `GUARD_KIT_CONSUMER_CASES`, a repository-relative path to a table in `cases.tsv`'s grammar, default empty {mechanical}. **Not yet applied.** Its row joins `native/src/knobs/guard_kit.rs`'s table, and its bullet joins §Layout and configuration's roster after `GUARD_KIT_CONSUMER_RULES_CMD`'s:

> - `GUARD_KIT_CONSUMER_CASES` — the consumer's own decision table (§Testing, the consumer lane), a repository-relative path in `cases.tsv`'s grammar; default empty, which runs no consumer lane. The kit ships no table, because the rows test the consumer's rules.

`templates/guard-config.knobs` gains the commented example `# GUARD_KIT_CONSUMER_CASES = scripts/guard-rules-cases.tsv` under a `spec:` line citing §Testing, beside the rule-command example it pairs with.

### (2) `--run-guard-tests` runs the consumer lane

After the kit's four tables, the arm reads `GUARD_KIT_CONSUMER_CASES` and, when it is set, runs the consumer lane {design-bearing}. **Not yet applied.** §Testing gains the lane as a paragraph after the sandbox paragraph:

> **The consumer lane** runs a consumer's own rows against its own rule command, and only where `GUARD_KIT_CONSUMER_CASES` names a table. Each row is fed as a `Bash` payload to the argv `GUARD_KIT_CONSUMER_RULES_CMD` names, spawned directly and with no shell, as the member spawns it (§Consumer rules). The working directory is the arm's own, which is the repository root through the front end or a root invocation, since a consumer's command is written for its repository. The environment is inherited, with `GATE_SDK_NATIVE_BIN` set to the running binary's absolute path as for the kit's cases. `@ROOT@` becomes that root, and `@NL@` a newline. The verdict is read on the same ladder, so exit 2 is `block`, empty stdout is `fallthrough`, and a fault is `exit<rc>` or `unknown`, a mismatch whatever row expected it. The subject is the consumer's command and not the member: the generic ruleset's verdict at a live root is the tree's, which a row cannot pin, and the member's placement of the consumer stage is `consumer-rules.test.sh`'s subject. A table set with no rule command, or naming no readable file, is exit 2, the harness-precondition code. Rows are fed as `Bash` payloads only, so a consumer rule written for PowerShell is not reached by this lane.

The arm's declared knob roster becomes `GATE_SDK_KIT_DIRS`, `GUARD_KIT_CONSUMER_RULES_CMD` and `GUARD_KIT_CONSUMER_CASES`. §Testing's roster paragraph ("The declared roster is `GATE_SDK_KIT_DIRS` and nothing else…") is re-phrased. The guard's own knobs stay undeclared, because the spawned member resolves them. The two consumer knobs are read by the arm itself, which spawns the consumer's command without the member. `GUARD_KIT_LOG` stays overridden and undeclared. The summary line reports the consumer lane's row count beside the kit tables', and reports no consumer row count where the knob is empty. The consumer lane's mismatches print in the kit lane's form, and a mismatch in either lane makes the exit 1.

### (3) guard-kit/SPEC.md names the lane where the gap was named

§Consumer rules' placement paragraph and §Writing a consumer rule name the lane {mechanical}. **Not yet applied.**

- **§Consumer rules**, the sentences "The consumer lane carries none: a consumer's command is reachable by no table in this kit, so it ships hand-verified and its narrowing cannot be measured. … That a consumer's own rules have no verification lane at all is a real gap and a separate one; it is named here rather than solved, since solving it designs a testing lane for consumer commands, and the command form does not close it." become: "The consumer lane carries none the kit can hold: a consumer's cases are its own table, run by the same arm (§Testing), and no kit check counts them against its rules."
- **§Writing a consumer rule** gains, after the ordering disciplines: "Prove a rule with a firing and a non-firing row in the table `GUARD_KIT_CONSUMER_CASES` names; `--run-guard-tests` runs them against your command (§Testing)."

### (4) This repository's three rules get their rows

`scripts/guard-rules-cases.tsv` is created, and `scripts/guard-config.knobs` sets `GUARD_KIT_CONSUMER_CASES = scripts/guard-rules-cases.tsv` {mechanical}. **Not yet applied.** The table carries the ten measured rows, a firing and a non-firing case per rule and more:

- `block` — `git commit --no-verify -m x`, `git commit -n -m x`, `ls /tmp/claude-0/scratch`, `git clean -fdx` and `git clean -X -n`
- `fallthrough` — `git commit -m 'drop --no-verify from the docs'`, `git commit -m x`, `ls .tmp/scratch`, `git clean -fd`, and `git commit -F - <<'MSG'@NL@never run git clean -x here@NL@MSG`

The temporary-path rows carry the harness's generic prefix with a synthetic suffix and no account or session name, the spelling `scripts/guard-rules.sh` already carries.

**Inferred, cannot run before build:** the consumer lane passes on the Windows leg's `--run-guard-tests` step, where the rule command's `bash` argv resolves on that runner's `PATH` — the lane does not exist until build lands it.

## Producers and consumers

- **`GUARD_KIT_CONSUMER_CASES`.** Producer: the consumer's knob file, which this repository sets in delta 4. Consumers: the `--run-guard-tests` arm, and the roster-holding readers of a knob name: `native/src/knobs/guard_kit.rs`'s table, `--emit knob-roster`, `check-knob-citation`, `check-knob-default-coupling` (the bullet's "default empty"), and the template's example line.
- **The consumer lane.** Producer: the arm, wherever it runs. That is CI's "guard-kit decision table" step on Linux and on the Windows leg, and `EVIDENCE_KIT_RUN_guard_tests` in this repository's evidence config. Both run it at the repository root, so the lane runs wherever the kit lane runs. Consumer: the exit status those callers read, and the mismatch lines a person reads.
- **The table's fields.** The decision column is read by the ladder comparison, and the command column by the payload builder, after the two substitutions. There are no other fields.

## Existing sections updated

- `guard-kit/SPEC.md` — §Consumer rules and §Writing a consumer rule (delta 3); §Testing, the roster paragraph and the new lane paragraph (delta 2); §Layout and configuration, the knob bullet (delta 1).
- `native/src/knobs/guard_kit.rs` — the knob row (delta 1).
- `native/src/emit/run_guard_tests.rs` — `KNOBS` and the lane (delta 2).
- `guard-kit/templates/guard-config.knobs` — the example line (delta 1).
- `scripts/guard-config.knobs` and `scripts/guard-rules-cases.tsv` — this repository's value and table (delta 4).
- `docs/guard-kit/SPEC.md` — the generated mirror (all deltas).

The roster came from `git grep -n 'GUARD_KIT_CONSUMER_RULES_CMD'` for the readers a guard-kit consumer knob reaches, and `git grep -n 'run-guard-tests'` for the lane's callers.

## Retired spellings

- None — the re-phrased §Consumer rules sentence names the gap in prose no other surface quotes.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the knob, the lane and the table.
- [ ] **Instruction surfaces: instruction only** — the template's example line carries a pointer and no grounds.
- [ ] **Merged with no information lost** — §Testing reads as one runner with two lanes, and the refused alternatives above land in §Testing's lane paragraph as its grounds.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — the gap sentence is gone from §Consumer rules.
- [ ] **Gaps filed** — a cross-component gap found during the work filed with `--emit file-gap`.
- [ ] **The entry moves** — `consumer-guard-rule-coverage` moves to Done in the landing commit, a stage before the drain stage.
