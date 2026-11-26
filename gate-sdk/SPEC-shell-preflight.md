# SPEC amendment: shell-preflight

Pairs `script-preflight` and `pipefail-readers`. A consumer that ships shell scripts to a batch scheduler wants each one refused before submission when it kills by a broad pattern, names a forbidden path, pipes into an early-exit reader under `pipefail`, or submits under a malformed owner id. It also wants the early-exit rule run across its whole shell tree, with a mechanical fix and a baseline for the sites it has not fixed yet. This amendment adds one shell reader shared by both, two gates over it, a fix arm, and the count baseline both this and gate-sdk/SPEC-site-count.md read.

**What the tree does today**, each bullet with the probe that settled it:

- check-pipe-membership reds one shape of the early-exit hazard: a set producer feeding `head` or a quiet `grep` under `pipefail` (gate-sdk/SPEC.md §check-pipe-membership). Its line joiner, pipeline splitter and `pipefail` test are private to its module (`grep -n '^fn' native/src/gates/pipe_membership.rs`: `sets_pipefail`, `logical_lines`, `pipelines`).
- `native/src/bashscan.rs` finds command positions for the fence and port gates and tracks no `pipefail` scope (`grep -n 'pipefail' native/src/bashscan.rs` finds none).
- No gate reads a consumer rules file over shell scripts, and no file in the tree holds a `<count><TAB><path>` baseline (`git grep -n 'baseline' -- native/src/gates` finds only evidence-kit's suite baseline, a different format).
- This repo's tracked shell outside the pruned fixtures calls no `pkill` (`git grep -l 'pkill' -- '*.sh' ':!*gate-tests*'` finds none).

## What changes

### (1) The shared shell reader {design-bearing} {user-facing: the adopter input on `pipefail-readers` — one detector, two entry points}

**Applied** (merged into gate-sdk/SPEC.md §The shell reader): a new section, §The shell reader, before §check-pipe-membership:

> `native/src/shell_lines.rs` is the one reader of shell text for the shell content gates. It joins backslash continuations and trailing `|`, `&&` and `||` into **logical lines**, each keeping its first physical line's number; splits each into its **code half and comment half** outside quotes; splits code into **pipelines** and each pipeline into stages outside quotes, a nested `( )` or `$( )` read as a pipeline set of its own; and tracks **`pipefail` scope**: on from a code line's `set -o pipefail` (or a flag word ending in `o` followed by `pipefail`) to the end of the file, `( )` or `$( )` that holds it. A here-doc body or a quoted operand fed to a command the caller names is read as **a script of its own**, its own scope off at its start and its line numbers the outer file's.
>
> **A span the reader cannot parse** — an unterminated quote, a here-doc with no terminator, an unbalanced `(`, `)` or `$(` — is reported as `<path>:<line>: [LINT-PARSE] unparseable span` and makes the reading gate exit 3, unparseable input; the rest of the file is not judged.
>
> **The early-exit detector** is the reader's one rule shared by two gates. A pipeline under `pipefail` hits when a stage after the first stops reading early: `grep` with `-q`, `-m`, `-l` (or their long forms, or a short cluster holding one); `head`; `read`; `sed` whose script ends a command with `q`; `awk` whose program holds `exit`; or a `while read` loop whose body holds `break`. It is cleared by a guarded producer, a first stage of the shape `{ <cmd> || true; }`, or by a waiver token the entry point names, written `# <token>: <reason>` on any physical line of the pipeline. A hit the fix arm can rewrite (delta 4) is marked fixable.

check-pipe-membership's `logical_lines`, `pipelines` and `sets_pipefail` move into this module and that gate calls them; its predicate and corpus are unchanged.

- **Why the reader list is kit mechanism.** Which readers stop early is shell semantics, the same in every project, as check-pipe-membership's reader set is. Which files, which waiver tokens and which reasons count are the consumer's.
- **Why one reader.** The adopter runs the same detector single-script and tree-wide; two parsers would disagree on a continuation or a quote and give one script two verdicts.
- **Honest limit.** The reader is lexical: a pipeline built through `eval`, a function called inside a pipeline, or `pipefail` set in a sourcing script and inherited by a library are outside it, the last bounded by the library rule in delta 3.

### (2) check-script-preflight {design-bearing} {user-facing: the adopter input on `script-preflight`}

**Not yet applied**: §Layout and configuration's rule rosters gain `GATE_SDK_SCRIPT_FILES` (default empty: pathspecs over the tracked tree, the shared prune dirs applied; empty disables the gate) and `GATE_SDK_SCRIPT_RULES` (default `<gates-dir>/script-rules.list`). A new section, §check-script-preflight:

> `checks/check-script-preflight.gate` (`precommit`, binary-dispatched). Invariant: no configured shell script breaks a rule in the consumer's rules file. Its corpus is the scripts given as operands, tracked or not, else `GATE_SDK_SCRIPT_FILES`. A named operand that is absent refuses at exit 2 and an unreadable script is exit 3. With no operand and the knob empty it prints `not applicable: no script configured`; it declares `# install: on-surface` and `# armed-by: GATE_SDK_SCRIPT_FILES`.
>
> **The rules file** is blocks. A line `rule <ID> <kind>` opens one, `<ID>` matching the class grammar (§Output contract); each following line indented by blanks is `<key> <value>`, the value the rest of the line. `#` lines and blank lines are skipped. Two top-level lines may precede the first block: `waiver-min <n>`, the shortest reason a waiver takes (default 1), and `body <word>…`, the commands whose here-doc or quoted operand §The shell reader reads as its own script. The file absent while a corpus is configured refuses at exit 2, unreadable is exit 3, and an unknown kind or key, a missing required key, a malformed id or an ERE the engine refuses is exit 2 naming the file and line.
>
> **The four kinds**, every command, flag, token and grammar a value:
>
> - `command` — keys `cmd` (required), `allow <ERE>`, `before <ERE>` and `waiver <token>`, each but `cmd` repeatable. Every unquoted shell word equal to `cmd` on a code line opens an invocation, its text running to the end of its simple command, quotes kept and blank runs collapsed. It is refused unless an `allow` matches that text, a `before` matches the logical line's code before the word, or a waiver clears it.
> - `forbidden-token` — keys `token <literal>` (required, repeatable) and `waiver`. A code half holding a token is refused, the record naming the token's position among the rule's tokens, never the token.
> - `early-exit` — key `waiver <token>…`. The early-exit detector, its waiver tokens the rule's.
> - `id-grammar` — keys `cmd`, `flag <flag>` (repeatable), `grammar <ERE>` and `waiver`. In each invocation of `cmd`, the value of each listed flag (the next word, or the text after `=`, unquoted) must match `grammar`; a value holding an expansion is skipped.
>
> **A waiver** is `# <token>: <reason>` on a physical line of the logical line, its reason at least `waiver-min` characters after trimming; a shorter reason is itself refused, `[<ID>] waiver reason under <n> characters`.
>
> **The record** is `<path>:<line>: [<ID>] <what>`, one per refused site in file then line order, `<what>` naming the kind's subject (the command word, the token position, the reader, the flag) and never a value from the script.

The kit ships `templates/script-rules.list`, commented out whole, one example block per kind, on `templates/portability-patterns.list`'s model.

- **Why generic kinds and not the adopter's four rules.** Each rule the adopter named is one instance of a kind: its `pkill` allowlist is a `command` block, its mount paths a `forbidden-token` block, its owner flags an `id-grammar` block. A kit literal naming its scheduler's command or its mount layout would publish both (§The provenance seam).
- **Why a waiver reason has a floor.** Its reader is the reviewer of the diff adding it, and a reason of a word or two tells that reader nothing; the floor is the consumer's.
- **Honest limit.** An `allow` is an ERE over the invocation as written, so an allowed form spelled with different quoting reds, and a command reached through a variable (`"$kill" -f …`) is no invocation.

### (3) check-pipefail-readers {design-bearing} {user-facing: the adopter input on `pipefail-readers`}

**Applied** (merged into gate-sdk/SPEC.md §check-pipefail-readers and §Layout and configuration): §Layout and configuration's rule rosters gain `GATE_SDK_PIPEFAIL_PATHS` (default empty: files and directories, a directory searched for `*.sh` under the shared prune dirs), `GATE_SDK_PIPEFAIL_LIBRARIES` (default `*lib.sh *-lib*`: basename globs of files linted with `pipefail` on from their first line), `GATE_SDK_PIPEFAIL_ALL` (default `0`; `1` lints every file in the corpus that way) and `GATE_SDK_PIPEFAIL_BASELINE` (default empty: a count baseline, §The count baseline). A new section, §check-pipefail-readers:

> `checks/check-pipefail-readers.gate` (`precommit`, binary-dispatched). Invariant: no early-exit reader is fed under `pipefail` in the configured shell tree beyond its baselined count. Its corpus is the operands, else `GATE_SDK_PIPEFAIL_PATHS`; a file is linted when a code line sets `pipefail` (from that line, §The shell reader), when its basename matches a library glob, or when `GATE_SDK_PIPEFAIL_ALL` is `1`. With the knob empty and no operand it prints `not applicable: no shell path configured`; it declares `# install: on-surface` and `# armed-by: GATE_SDK_PIPEFAIL_PATHS`.
>
> It runs the early-exit detector, its waiver token `pipefail-ok`, its reason held to a floor of 1. The record is `<path>:<line>: [PIPEFAIL-EARLY-EXIT] <reader>`, followed by ` [auto-fix]` where delta 4 can rewrite it. Under a baseline a file's hits are counted against its row (§The count baseline), and a file over its count prints every hit, since which one is new is unknowable.

- **Why a second gate and not a wider check-pipe-membership.** That gate is zero-config and kit-wide on a predicate narrow enough to hold every tree without a baseline. This one reds a whole class a tree adopts site by site, so it is consumer-armed and baselined.

### (4) The fix arm {design-bearing} {user-facing: the adopter input on `pipefail-readers` — a mechanical rewrite}

**Applied** (merged into gate-sdk/SPEC.md §check-pipefail-readers): §check-pipefail-readers continues:

> **`--emit pipefail-fix [<path>…]`**, an `Arm::Emit` member (§The non-gate arm), rewrites every fixable hit in its corpus, chosen as the gate chooses it, in place: `grep -q X` to `grep X >/dev/null`, `grep -m N …` to `grep … | sed -n '1,Np'`, and `head -n N` to `sed -n '1,Np'`. Each rewritten file keeps its mode, and the arm prints one `<path>:<line>: fixed` line per site. Any other hit is a hand fix and is left alone. An unparseable file is exit 3 with nothing written to it; an unwritable one is exit 3. Its caller is the session fixing a red, which the gate's `help:` line names.

- **Why an arm and not a gate flag.** A gate judges and never writes the tree (§run-gates' leave-no-trace invariant); a rewrite has a person reader and a status grammar of 0, 2 and 3, the `Arm::Emit` family's.

### (5) The count baseline {design-bearing} {user-facing: the adopter input on `pipefail-readers` and `reviewed-site-count` — one format}

**Applied** (merged into gate-sdk/SPEC.md §The count baseline): a new section, §The count baseline, after §The shell reader:

> A count baseline holds lines `<count><TAB><path>`, `<count>` decimal digits and `<path>` repo-relative; `#` lines and blank lines are skipped, and any other line, or a path given twice, is unparseable input at exit 3 naming the line. A reading gate names its comparison: **a ceiling** (check-pipefail-readers) passes a file at or under its count, reds one over it, and reds `<path>: [BASELINE-STALE] <found>/<count>` where the count exceeds the file's hits so the count is lowered; **an exact count** (check-site-count) reds any difference. A file with no row has a count of 0.

- **Why `<found>/<count>` in the record.** The fix is a number in the baseline, and the record carries it. **Honest limit:** that text moves the result's fingerprint whenever a count changes, the volatile-value limit §run-gates names.

## Producers and consumers

- **The shell reader.** Producer: `native/src/shell_lines.rs`. Consumers: check-script-preflight, check-pipefail-readers, the fix arm and check-pipe-membership (deltas 1 to 4); check-pipe-membership's pair re-runs, its predicate unchanged.
- **check-script-preflight.** Producer: the battery, and a consumer's submission wrapper naming one script as an operand. Deployed: this repo binds `GATE_SDK_SCRIPT_FILES` to its tracked `*.sh` and a `scripts/script-rules.list` holding one `command` rule over `pkill`, allowing `-P` with a lone quoted expansion and `-x` with a quoted literal, `waiver-min 5` — a guard on a tree several concurrent sessions share, green today since no tracked script calls `pkill`; the orchestrator adopter binds its own four rules *(inferred: its configuration is out of tree, lead-relayed)*. Consumers: the committing session, the SARIF log, the adopter's wrapper reading the status *(inferred, as above)*. Point 6: every tracked script outside the prune set must read without a `LINT-PARSE` (`git ls-files '*.sh' | grep -v gate-tests` lists 39 today); a script the reader refuses is a reader defect fixed at build, never a narrowed corpus.
- **check-pipefail-readers and the fix arm.** Producer: the battery and the session. Deployed: the orchestrator adopter *(inferred, as above)*. This repo binds no path: check-pipe-membership holds the set-producer shape here, and adopting the wider class is a calibration this unit does not make, so the gate is registered, not applicable, and named disarmed by `doctor`. Consumer of the fix arm: the session the `help:` line points at.
- **The count baseline.** Producer: the consumer's baseline file. Consumers: check-pipefail-readers (ceiling) and check-site-count (exact, gate-sdk/SPEC-site-count.md).
- **Roster readers of the new members:** `gates::REGISTRY` and the arm table `ARMS` (whose test owes the arm a test module), `check-readme-roster` (gate-sdk/README.md's roster block), `check-install-disposition`, `check-gate-substrate-parity` and `scripts/gates.list`; the knob rows in `native/src/knobs/gate_sdk.rs`; `templates/script-rules.list` through the payload and `init`'s config-seam plan (`native/src/installer/recipe.rs`).

## Existing sections updated

- gate-sdk/SPEC.md §The shell reader, new; §check-pipe-membership's helpers named there (delta 1).
- gate-sdk/SPEC.md §Layout and configuration, rule rosters — the six knobs (deltas 2 and 3).
- gate-sdk/SPEC.md §check-script-preflight, new (delta 2).
- gate-sdk/SPEC.md §check-pipefail-readers, new (deltas 3 and 4).
- gate-sdk/SPEC.md §The count baseline, new (delta 5).
- `native/src/shell_lines.rs`, `native/src/gates/pipe_membership.rs` (delta 1); `native/src/gates/script_preflight.rs`, its `.gate`, pair, test and `templates/script-rules.list` (delta 2); `native/src/gates/pipefail_readers.rs`, its `.gate`, pair and test (delta 3); the fix arm's module and its `ARMS` row (delta 4); the baseline reader (delta 5).
- `native/src/knobs/gate_sdk.rs`, `gates::REGISTRY`, gate-sdk/README.md's roster block, `scripts/gates.list`, `scripts/script-rules.list` and this repo's knob binding (deltas 2 and 3).
- `docs/gate-sdk/SPEC.md`, the generated mirror (all deltas).

## Retired spellings

- None — check-pipe-membership's helpers move module without a rename, and every other name is new.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
