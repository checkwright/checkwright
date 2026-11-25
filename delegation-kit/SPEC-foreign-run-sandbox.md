# SPEC amendment: foreign-run-sandbox

Pairs `foreign-run-verb`. The foreign-vendor run already takes a brief, a vendor and a model, and already returns a machine verdict with the path of the agent's output. What it lacks is the fourth input, a sandbox policy the caller names and the executor applies. This amendment adds that policy as a consumer-configured vocabulary resolved into the adapter's argv. It mirrors how a class resolves into `@MODEL@`, and it holds a resumed turn to the policy its session opened with. It also states the arm as the verb an outside caller runs.

**What the tree does today**, each bullet with the probe that settled it:

- `--foreign-run <adapter> <prompt-file> [--mode audit|sweep] [--class <class>] [--key <key>]` exists. The prompt file is the brief, the adapter names the vendor's program, and `--class` resolves the model and effort on a class-taking adapter (`grep -n 'const USAGE' native/src/emit/foreign_run.rs`; delegation-kit/SPEC.md §The foreign-vendor run).
- The result is the verdict line `foreign-run: adapter=… mode=… class=… key=… budget=… exit=… report=<path|none> patch=<path|none> -> <VERDICT>` on exit 0, 1 or 2, and `report.txt` is the agent's standard output (`grep -n '"foreign-run: adapter={}' native/src/emit/foreign_run.rs`).
- The adapter's sandbox mode is a literal word in its argv, so one adapter carries one confinement and a caller picks a policy only by picking an adapter (`grep -n 'carrying its vendor' delegation-kit/SPEC.md`). An adapter's open and resume forms substitute `@PROMPT_FILE@`, `@SESSION_ID@`, `@MODEL@` and `@EFFORT@`; `@USAGE_FILE@` and `@FAILED_RUN@` are the usage producer's (`grep -n '_TOKEN: &str' native/src/emit/foreign_run.rs`).
- Nothing checks that a resume form restates the open form's confinement. The SPEC states that as an honest limit and as a sentence on `DELEGATION_KIT_FOREIGN_RESUME` (`grep -n 'restates its confinement\|does not restate' delegation-kit/SPEC.md`).
- The arm prints the line and no JSON. The one adopter-named JSON reader is evidence-kit's `verify` (`ls docs/contracts`).

## What changes

### (1) The sandbox policy: a caller's word, a consumer's vocabulary {design-bearing} {user-facing: the entry's deliverable — the verb's sandbox policy, its vocabulary held in config}

A run names a sandbox **policy** with `--sandbox <policy>`. With no `--sandbox`, the policy is the run's mode word, `audit` or `sweep`. So a host that passes only a mode gets the confinement its consumer bound to that mode, and lifecycle-kit's host protocol is unchanged. A word of an adapter's forms may carry `@SANDBOX@`, substituted inside the word as `@MODEL@` is, with the word `DELEGATION_KIT_FOREIGN_SANDBOX` binds to that adapter and policy.

**Not yet applied** — replaces the paragraph of delegation-kit/SPEC.md §The foreign-vendor run that opens "**The executor resolves a run's class into the adapter's argv.**", re-phrased to cover both resolutions:

> **The executor resolves a run's class and its sandbox policy into the adapter's argv.** Each is substituted inside a word, as `@PROMPT_FILE@` is, so a vendor's `key=value` option takes one.
>
> - **The class.** An adapter takes a class when `DELEGATION_KIT_FOREIGN_HARNESS` names it. A word of its forms may then carry `@MODEL@` and `@EFFORT@`, with the pair its harness binds to the run's `--class` (§The tier binding).
> - **The policy.** `--sandbox <policy>` names it, and the mode word names it when the run names none. An adapter whose open form carries `@SANDBOX@` takes the word `DELEGATION_KIT_FOREIGN_SANDBOX` binds to `<adapter>/<policy>`. A policy that adapter leaves unbound is `FAILED (sandbox: <why>)` with `budget=-`, among the pre-spawn checks. An adapter carrying no `@SANDBOX@` pins its confinement as a literal: a run naming no `--sandbox` proceeds on it, and one naming a policy fails the same way. The executor cannot apply a policy the caller asked for, and running anyway would report a confinement that was never set.

The class bullets that follow the paragraph keep their text. The verdict line of §The foreign-vendor run gains `sandbox=<policy|->` after `class=`. It reads `-` where the adapter carries no token, and the line's field roster says so. The section's opening argv, the usage text and the shape-refusal paragraph add `[--sandbox <policy>]`, a policy outside `[a-z0-9-]` being a shape refusal, and `--budget` refuses `--sandbox` beside `--mode`, `--class` and `--key`.

- **Why the mode word is the default policy.** Each mode already states the confinement it needs. An audit must not write, and a sweep writes inside the clone. A consumer binding `<adapter>/audit` and `<adapter>/sweep` gets one adapter that serves both modes at the right confinement, and no host learns a second word.
- **Why a policy named on a token-less adapter fails.** `--class` on such an adapter applies nothing and proceeds, because the adapter's literal model is a cost choice. A sandbox is the run's only confinement (the confinement honest limit). A caller asking for one that the executor cannot set is told so before anything is cloned.
- **Why keyed by adapter, not by harness.** The class rows are keyed by harness because a class names a model the vendor serves under every adapter. A sandbox word belongs to one adapter's argv, and keying it by adapter needs no adapter to take a class first.

### (2) A session keeps the confinement it opened with {design-bearing} {user-facing: the entry's deliverable — the sandbox policy held across a resumed session}

The open records the policy and its resolved word, and every resume substitutes the recorded word. The validator holds the open and resume forms together, so the resume form's restatement of confinement becomes a checked fact rather than a sentence.

**Not yet applied** — in delegation-kit/SPEC.md §The foreign-vendor run, step 4's `session.txt` record becomes `adapter=<adapter> mode=<mode> class=<class|-> model=<model|-> effort=<effort|-> sandbox=<policy|-> confine=<word|-> base=<commit> turn=1 id=<id|->`. A line an earlier build wrote reads `-` for the two new fields, as for the class fields. The bullet "**A session keeps the pair it opened with.**" is re-phrased to "**A session keeps the pair and the confinement it opened with**", and its grounds apply to both. §Resuming a session's sentence on class tokens becomes:

> A class token or `@SANDBOX@` in the form takes the session's recorded model, effort or confinement word, and a form asking for one the session recorded as `-` fails before the spawn as `FAILED (class: …)` or `FAILED (sandbox: …)`.

Its line gains `sandbox=<policy|->` after `class=`, the session's recorded policy.

**Not yet applied** — the confinement bullet's second half in §The foreign-vendor run's honest limits, "A resume form that does not restate the open form's confinement runs the next turn under the vendor's default policy", becomes:

> A resume form whose adapter pins its confinement as a literal is held to restating it by nothing but its author. An adapter whose open form carries `@SANDBOX@` must carry it in its resume form too (§Layout and configuration), so a policy-taking session's every turn runs under the word it opened with.

- **Why record the word, not only the policy.** The class precedent's ground applies with more force here. Re-resolving at a resume would change a kept conversation's confinement because a binding was edited, and no line would say so. A widening is a new run under a policy bound wider, never a flag a resume adds.

### (3) The arm, stated as the verb an outside caller runs {mechanical}

**Not yet applied** — after the opening paragraph of delegation-kit/SPEC.md §The foreign-vendor run, one paragraph:

> **It is the verb a second-opinion caller runs.** The prompt file is the brief. The adapter names the vendor and its program. `--class` names the model, or the adapter's argv carries one literally. `--sandbox` names the policy. The result is the verdict line and its exit, with `report.txt` as the agent's transcript at the line's `report=` path, `stderr.txt` beside it, and a sweep's `change.patch` at `patch=`. The line is the machine result: every field a caller acts on is a `key=value` pair on it. A second format is minted only for a named reader that needs a field the line cannot carry.

- **Why no JSON result.** Each published wire contract under `docs/contracts/` has a reader that parses fields no line carries. This arm's callers read the verdict word, the exit and two paths, and the line carries all of them. A JSON form would be a published contract with no reader to hold it. And a field the line lacked would have to be added to the line anyway, since the line is what the dispatcher and the landing commit read.

### (4) The knob, its validation and its rosters {mechanical}

**Not yet applied** — delegation-kit/SPEC.md §Layout and configuration gains, after `DELEGATION_KIT_FOREIGN_RESUME`:

> - `DELEGATION_KIT_FOREIGN_SANDBOX` — each foreign adapter's sandbox vocabulary (§The foreign-vendor run): indexed, each element `<adapter>/<policy>=<word>`, adapter and policy each a name in `[a-z0-9-]`; default empty. The word is what `@SANDBOX@` becomes for a run of that adapter under that policy. The table validator refuses a malformed element, an adapter `DELEGATION_KIT_FOREIGN_ADAPTERS` does not configure, an adapter and policy bound twice, an empty word, a word carrying whitespace and a word that is one `-`, which the session record reads as absent. It holds three facts together with the adapter forms. An adapter with an element carries `@SANDBOX@` in its open form. An adapter carrying the token in its open form carries it in its resume form, where it has one. An adapter with no element carries the token in neither form. The kit ships no element, since a word names a vendor's option.

`DELEGATION_KIT_FOREIGN_RESUME`'s closing sentence "A resume form restates its confinement, since a vendor's resume form may spell its sandbox differently from its open form." becomes "A resume form restates its confinement, since a vendor's resume form may spell its sandbox differently from its open form; for a policy-taking adapter the validator holds it to carrying `@SANDBOX@`." The `DELEGATION_KIT_FOREIGN_ADAPTERS` bullet's token sentence names `@SANDBOX@` beside `@MODEL@` and `@EFFORT@`. `--foreign-run`'s and `--foreign-resume`'s declared-knob lists add `DELEGATION_KIT_FOREIGN_SANDBOX`. `delegation-kit/templates/delegation-config.knobs` gains the commented element shape beside the harness line.

## Producers and consumers

- **`--sandbox <policy>`.** Producer: the dispatcher, a host or an outside caller, at the run's argv. Where a caller names none, the mode word. Consumer: `parse` and the pre-spawn resolution in `native/src/emit/foreign_run.rs`, which looks the policy up in the adapter's rows.
- **`DELEGATION_KIT_FOREIGN_SANDBOX`.** Producer: a consumer's knob file, usually the gitignored `.local` overlay beside its adapters. The deployed configuration that sets it is this repo's own overlay: build moves a resumable adapter's literal confinement to `@SANDBOX@` with its `audit` and `sweep` elements, and runs it once live (Definition of Done). With no element every adapter is literal and nothing changes. Consumers: the run's resolution (delta 1), and the knob table validator in `native/src/knobs/delegation_kit.rs`, which holds the three cross-table facts (delta 4).
- **`@SANDBOX@`.** Producer: a word of an adapter's open or resume form. Consumer: the substitution beside `@MODEL@`'s, at the open (delta 1) and at every resume (delta 2).
- **`session.txt`'s `sandbox=` and `confine=` fields.** Producer: an `OK` open on a resumable adapter. Consumers: `--foreign-resume`, which substitutes `confine` and prints `sandbox`, and the old-line reader, which reads both as `-`.
- **The verdict lines' `sandbox=` field.** Producer: both arms. Consumer: the dispatcher reading which confinement a turn ran under, and the commit message that lands a foreign run's work, which quotes the line (lifecycle-kit/SPEC.md §The host protocol). The roster probe `git grep -n 'foreign-run: adapter\|foreign-resume: adapter' -- ':!docs/*'` over the tracked tree finds the SPEC's two line definitions and the unit tests' prefix assertions in `native/src/emit/foreign_run.rs`. No other parser reads the line.
- **Point 2, roster-holding readers.** The knob lands on the kit's roster (§Layout and configuration, `--emit knob-roster`, the knob table) and on the two arms' declared-knob lists. The token lands on the adapter-table bullet. The option lands on the usage text in `native/src/emit/foreign_run.rs` and the arm's help in `native/src/runner.rs`.
- **Point 5, narrowing.** None. A configuration with no element behaves as today, and the validator's new refusals bind only a consumer who adopts the token.
- **Point 6, every member.** Every adapter carrying `@SANDBOX@` must bind each policy its runs name. The mode words are the members every host run names, and an unbound one fails before anything is cloned, so the failure surfaces at the first run rather than as a silently unconfined one.

## Existing sections updated

Roster probe: `git grep -n 'MODEL@\|restates its confinement\|does not restate\|session.txt\|class=<class' -- delegation-kit/ native/src/emit/foreign_run.rs native/src/knobs/delegation_kit.rs native/src/runner.rs`, over the tracked tree.

- delegation-kit/SPEC.md §The foreign-vendor run — the resolution paragraph, the verdict line, the shape-refusal paragraph, the session record, the kept-pair bullet, the confinement honest limit and the declared-knob list (deltas 1 and 2), and the verb paragraph (delta 3).
- delegation-kit/SPEC.md §Resuming a session — the token sentence, the line and its declared knobs (delta 2).
- delegation-kit/SPEC.md §The keyed verdict — the `--budget` form's refused-option list (delta 1).
- delegation-kit/SPEC.md §Layout and configuration — the new knob bullet and the adapter and resume bullets' sentences (delta 4).
- delegation-kit/templates/delegation-config.knobs — the commented element shape (delta 4).
- native/src/emit/foreign_run.rs — the option, the token, the resolution, the session fields, both lines, the usage text and unit tests for each refusal and substitution (deltas 1 and 2).
- native/src/knobs/delegation_kit.rs — the row and the three cross-table refusals with their unit tests (delta 4).
- native/src/runner.rs — the arm's help text (delta 1).
- docs/delegation-kit/SPEC.md, docs/llms-full.txt — the generated mirrors (all deltas).

## Retired spellings

- None — the policy adds an option, a token, a knob and two record fields, and retires no name; a session line without the new fields still reads.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — the knob template's comment line carries the element shape, never its grounds.
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` holds.
- [ ] **Deployed and run** — this repo's overlay carries `@SANDBOX@` on a resumable adapter with `audit` and `sweep` elements, the validator passes it, and one live open and resume print the `sandbox=` field.
- [ ] **Entry moved** — `foreign-run-verb` moves to Done before the drain stage, in the commit that lands the merge.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
