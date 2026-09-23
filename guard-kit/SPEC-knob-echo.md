# SPEC amendment: knob-echo

A session that runs the battery or an arm often prefixes the call with the knob it already has. The prefix takes three shapes, `export GATE_SDK_NATIVE_BIN=native/target/release/checkwright-gates && …`, `GATE_SDK_NATIVE_BIN=… <cmd>` or `env GATE_SDK_NATIVE_BIN=… <cmd>`. The value is the one the knob already resolves to, so the prefix buys nothing, and every one of the three shapes falls off the permission matcher's path. Rule 6 catches only a standalone assignment ending its segment (`guard-kit/lib/guard.sh:429`), so nothing steers these three.

**The ruling: a fourth arm of rule 2, not a new rule and not a prose fix.**

- **The arm.** It blocks an `export`, a leading assignment or an `env` assignment of a **statically owned kit knob** whose value is the one that knob already resolves to, or names the same file. It steers to the command without the prefix and names the resolved value.
- **Why rule 2.** Rule 2 is already the ruleset's home for *a prefix spelling that falls off the match path, steered to the spelling that stays on it*. As an arm it renumbers nothing: `check-guard-registration` holds items `1..n` to dispatch order, and a new item would push every "rule N" cross-reference after it.
- **Why not prose.** The entry's alternative was a prose fix: sessions read "the gate binary at `GATE_SDK_NATIVE_BIN`" as needing the variable set. That is refused as the fix. The resolved path is already printed into every session's start context, and the prefix still recurred, so a prose change alone steers nothing at the call.
- **An override stays the harness's decision.** A value that differs from the resolved one is config-via-env (gate-sdk/SPEC.md §The knob file: the environment beats the file on purpose), so the arm falls through.
- **Env-only settings are out.** An execution setting such as `GATE_SDK_JOBS` is no static knob, so `--emit knob-values` refuses it and the arm never fires on it.

**Why "names the same file" and not byte equality alone.** The two differ in harm. Exporting `./native/target/release/checkwright-gates` instead of the default `native/target/release/checkwright-gates` points at the same binary. But the hook emitter bakes the knob's raw text into the generated hooks, so that export reds `check-graph` on the battery it prefixes (measured below). A redundant spelling of a path value is the steer's strongest case, not a false positive.

**Measured at authoring (2026-09-23)**, by the research dispatch over probe payloads piped into `bash scripts/bash-guard.sh` with `GUARD_KIT_LOG` pointed at scratch:

- **The unsteered shapes.** `export GATE_SDK_NATIVE_BIN=native/target/release/checkwright-gates && native/target/release/checkwright-gates --run` exits 0 with no output. So does the leading-assignment shape, `env GATE_SDK_NATIVE_BIN=… ./native/…`, `GATE_SDK_JOBS=1 native/… --run`, and `export FOO=bar && ls`.
- **What rule 6 catches.** `FOO=bar; ls` exits 2 on rule 6's assignment block, and `ls $FOO` on its expansion block.
- **Resolution.** `native/target/release/checkwright-gates --emit knob-values GATE_SDK_NATIVE_BIN` prints `scalar native/target/release/checkwright-gates`. `--emit knob-values GATE_SDK_JOBS` and `--emit knob-values FOO` each exit 2 with a not-a-knob refusal.
- **The byte difference.** `GATE_SDK_NATIVE_BIN=./native/target/release/checkwright-gates native/target/release/checkwright-gates check-graph` exits 1 with `scripts/git-hooks/pre-commit is stale`. The byte-identical default exits 0.
- **The evidence corpus.** The filed "15 of 231 prompting calls" figure's corpus is gone from `.workflow/prompt-friction.log`, and `TASK-QUEUE.md` is its only record. The rule rests on the probes above, not on that count.

## What changes

### (1) Rule 2 gains arm (d) {design-bearing}

**Not yet applied.** In `guard-kit/lib/guard.sh` `guard_rule_git_c_root`, a fourth arm reads each segment of the `sq dq hd` skeleton the rule already declares. It recognizes three prefix shapes:

- a segment that is `export NAME=value`, standalone or followed by `&&`;
- a leading run of `NAME=value` words before a command word;
- `env` followed by `NAME=value` words.

For each assigned `NAME` it spawns `--emit knob-values NAME` through the library's existing binary read, `gate_knob_values`'s door. It fires when all four hold:

- the knob answers;
- the knob is a scalar;
- the assigned value is byte-equal to the resolved value, or both name one existing file after resolution against the repo root;
- the value carries no quote or expansion. Rule 6 owns those, so this arm defers to it.

A refusal, a spawn failure or an indexed knob falls through. The block message names the knob, its resolved value and the command with the prefix removed. When the spellings differ, it adds that a respelled path changes text the generated hooks bake.

In guard-kit/SPEC.md §The generic ruleset, rule 2's arm list gains:

> - **(d) An exported or assigned kit knob whose value is the one it already resolves to** — `export NAME=value`, a leading `NAME=value` run, or `env NAME=value`, where `NAME` is a knob a static kit table owns (the binary's `--emit knob-values` answers for it) and the value is byte-equal to its resolved value or names the same file. Steered to the command without the prefix, the message naming the resolved value. A differing value is a configured override and falls through to the harness; a name no static table owns, an environment-only execution setting included, is never read as a knob. A respelled path is the arm's sharpest case: it names the same file and still changes the text the generated hooks bake, so the battery it prefixes reds `check-graph`.

The "Why a steer and not a grant" paragraph's "arms (b) and (c)" becomes "arms (b), (c) and (d)".

### (2) The decision table carries the arm {mechanical}

**Not yet applied.** `guard-kit/guard-tests/cases.tsv`'s `# rule 2` section gains firing and non-firing rows over a knob whose resolved value is the same in the test sandbox and at its default. The sandbox exports `GATE_SDK_NATIVE_BIN` as an absolute path (guard-kit/SPEC.md §Testing), so that knob's default spelling is not its resolved value there.

- **Fires:** an `export` of a knob at its resolved value followed by `&&`, the same as a leading assignment, and the same after `env`.
- **Falls through:** the same knob at a differing value, an environment-only setting (`GATE_SDK_JOBS=1 …`), and a non-knob name (`FOO=bar ls`).

Every existing row whose command carries an assignment or export prefix is re-derived under the new arm, because §Testing rules the table non-monotone. The same-file case, a `./`-spelled default, is driven from `guard-kit/gate-tests/guard-config-knobs.test.sh`, the lane for values the table's sandbox cannot hold.

## Producers and consumers

- **Arm (d).**
  - Producer: the bash guard, on every Bash call whose skeleton carries one of the three prefix shapes.
  - Consumer: the session, through the block message.
  - Reads: the binary's `--emit knob-values`, a spawn only on a prefixed call.
- **Roster-holding readers.**
  - `check-guard-registration` holds items to functions and dispatch order. Arm (d) adds neither an item nor a function.
  - `--emit scan-prompts` ranks fall-throughs from the friction log, so a prefixed call now blocked stops appearing as a prompt. That is the effect sought. Because the definition moved, the KPI series needs a discontinuity note (guard-kit/SPEC.md §scan-prompts records each definitional step).
- **Point 5.** No corpus narrows.
- **Point 6.** Not obliged.

## Existing sections updated

Roster from `grep -n "guard_rule_git_c_root\|arms (b) and (c)\|# rule 2" guard-kit/SPEC.md guard-kit/lib/guard.sh guard-kit/guard-tests/cases.tsv`, run 2026-09-23.

- `guard-kit/lib/guard.sh` (delta 1).
- `guard-kit/SPEC.md` §The generic ruleset, rule 2 and its steer paragraph (delta 1).
- `guard-kit/SPEC.md` §scan-prompts, the KPI discontinuity note (delta 1).
- `guard-kit/guard-tests/cases.tsv` and `guard-kit/gate-tests/guard-config-knobs.test.sh` (delta 2).
- `.workflow/release-declarations.md`, one Tightened gates bullet (delta 1): the bash guard steers an `export`, leading assignment or `env` prefix that sets a kit knob to the value it already has.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/guard-kit/SPEC.md`.

## Retired spellings

- None — no delta renames or deletes a spelling.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for arm (d).
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls guard-kit/SPEC-*.md`).
- [ ] **Entry moved.** `knob-default-export-unsteered` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
