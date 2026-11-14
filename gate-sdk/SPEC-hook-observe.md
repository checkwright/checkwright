# SPEC amendment: hook-observe

Pairs `hook-observe-mode`. A consumer adopting a blocking hook in production wants a log-only period before it enforces, so it can count the refusals the hook would have issued and judge them before any session is refused. Today the only way to do that is to wrap the arm in a script of one's own that discards its status, and every consumer would write that wrapper again. This amendment gives `--hook` one registration-level flag, `--observe`, under which any member runs, its would-be decision is logged, and the call proceeds.

**What the tree does today**, read from `native/src/hook/mod.rs`, `native/src/hook/antigravity.rs`, `native/src/emit/enforcement_map.rs`, `native/src/gates/settings_paths.rs`, `native/src/gates/plugin_parity.rs` and `gate-sdk/bin/run-gates.sh`:

- **The grammar has one optional token.** `hook::run` reads `--hook <member>`, then `binding_operand` reads `--binding <name>` where it is the next token. Any other trailing token is ignored, so a misspelled flag runs the member enforcing, with no word.
- **The members print where they decide.** Each `HookFn` writes its envelope or its block text itself and returns its status. No member returns a verdict a caller could inspect.
- **The adapter already self-spawns.** `antigravity::run` re-runs `--hook <member>` as a child of `proc::self_image()` through `proc::run_with_stdin`, and reads the child's status, stdout and stderr (§The harness-integration arm).
- **The registration parser returns the member alone.** `hook::registration` answers `NotAMember`, `Member(name)` or `NoOperand`. Its three readers see no token after the member.
- **The front end forwards the whole argv.** `exec_arm` `exec`s the binary with `"$@"`, so a trailing token reaches the arm unchanged.

## What changes

### (1) The grammar and the observed run {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): one `--observe` flag, `--hook <member> [--observe] [--binding <name>]`; under it the arm is silent and exits 0}

**The grammar.** `--hook <member> [--observe] [--binding <name>]`, in that order, each at most once. One parser reads the trailing tokens for the arm and for the registration readers (delta 3). A token outside the grammar is refused where and as an unknown member is refused: exit 2 naming the token and printing the grammar, and under §The harness-integration arm's no-tree decline, the no-tree notice. A misspelled flag therefore blocks loudly rather than enforcing in silence, and `check-settings-paths` catches it at the commit (delta 3).

**The observed run.** Under `--observe`, `hook::run` does not call the member. It spawns `proc::self_image()` as `--hook <member>`, in the directory the arm stands in, with the payload bytes on stdin, through `proc::run_with_stdin`, the adapter's spawn. It reads the child's status, stdout and stderr, appends one line to the observe log (delta 2), writes nothing on stdout or stderr, and exits `0`.

**Grounds.** The members print at many sites, the reason the adapter self-spawns. Refactoring every member to return a verdict would touch each member's every exit and buy nothing outside this mode. The child keeps every member, envelope and spelling as its single copy. It costs one process per observed call, and nothing on an enforcing registration.

**With a binding.** `--hook <member> --observe --binding <name>` runs the adapter as today, and the adapter carries `--observe` into its child's argv. The child observes on the translated payload, logs the shipped binding's decision, and exits 0 with nothing on stdout, which the adapter reads as no decision. One site logs, whichever binding reached it.

**The would-be decision** is read off the child:

| child | `would=` | `reason=` |
| --- | --- | --- |
| exit 2 | `block` | stderr's first line |
| exit 0, stdout empty | `none` | `-` |
| exit 0, a context envelope | `advise` | the context text's first line |
| exit 0, an allow envelope | `grant`, or `rewrite` where it carries the changed input | the decision reason's first line |
| exit 0, a turn-end block decision | `block` | the envelope's reason's first line |
| exit 0, the notice field alone | `notice` | the notice text's first line |
| any other status, unparseable stdout, or no spawn | `error` | stderr's first line, or the spawn error |

The envelopes are read through `native/src/hook/binding.rs`, the one copy that builds them (§The hook contract). A member's own decline, on a knob it cannot resolve, therefore logs `notice` with the decline's text.

**Every member takes the flag.** The flag is grammar at the dispatcher and reads no member's rule, so it is not limited to the blocking members. Observing a member that advises or grants withholds that effect too: an observed `shell-guard` grants nothing, so a trial may show permission prompts the enforcing guard would have answered. That is a stated limit of a trial, not a defect.

**What still speaks.** The arm's own notices are unchanged by the flag: the no-tree notice, and the front end's notice on an absent binary, which runs no arm to read the flag. Each reports that nothing ran, so nothing is logged and the trial is not silent about it.

§The harness-integration arm gains an **`--hook <member> --observe`** paragraph after the binding paragraph, carrying the grammar, the observed run, the table and the limits. Its *argv* channel bullet names the grammar, and §run-gates' `--hook <member>` paragraph names the trailing tokens it forwards. The arm's help line in `native/src/runner.rs` prints the grammar. **Not yet applied.**

### (2) The observe log {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): the dispatcher logs to `GATE_SDK_HOOK_OBSERVE_LOG`, default `<GATE_SDK_TMP_DIR>/hook-observe.log`, one line per call carrying the member, the would-be decision and the reason's first line}

**The knob.** `GATE_SDK_HOOK_OBSERVE_LOG`, a derived scalar row in gate-sdk's table, default `${GATE_SDK_TMP_DIR:-.tmp}/hook-observe.log`. The `--hook` arm's declared roster carries it beside `EVERY_HOOK_KNOB`, because the dispatcher reads it, not a member (§The non-gate arm, *a dispatching arm declares its callee's reads beside its own*). §Layout and configuration gains its bullet after `GATE_SDK_TMP_DIR`'s.

**The path.** The value is read through `walk::capture_path`, as the turn-end hook's log is, so a session in a linked worktree logs into the main checkout's file rather than a worktree that is removed with the trial's record. Under the tree-free mark (gate-sdk/SPEC-hook-tree-free.md delta 2) it passes `hook::tree_free_path`, so a relative value declines there.

**The line.** One line per observed call, appended as one write of the whole line on an append-mode handle, so concurrent hooks interleave whole lines:

`ts=<UTC, ISO 8601> member=<name> event=<event name or -> session=<session id or -> would=<decision> status=<child status or -> reason=<first line or ->`

`reason=` is last, so a reason carrying spaces needs no quoting and a reader splits the line at most six times. `event` and `session` come from the payload, which the dispatcher parses once for them and hands on as bytes. The line carries no transcript path and no tool input: a log a trial leaves behind holds no command text a session typed.

**A log that cannot be written declines.** A value that cannot resolve, a relative one under the tree-free mark, or a failed open or write each decline through `hook::decline`, the decline every unresolvable knob takes (§The harness-integration arm), naming the knob and the cause. The call still proceeds at exit 0. **Grounds:** a trial whose log silently records nothing would be read at its review as a hook that never fired, the silent fail-open the decline exists to make loud. This is the one way an observed call speaks.

**Not yet applied.**

### (3) The registration readers {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): the enforcement map reads an observed registration as not enforced}

**The parser.** `hook::registration` reads the trailing tokens with the arm's own parser (delta 1). `Registration::Member` carries the member, an `observe` flag and the binding where one is named, and a new `Registration::Malformed` carries the member and the first token outside the grammar. §The harness-integration arm's registration-grammar paragraph names both. **Not yet applied.**

**`check-settings-paths`** reds a `Malformed` registration: `— hook registration outside the --hook grammar: <token>`, its `help:` line printing the grammar. An observed registration of a known member resolves clean. context-kit/SPEC.md §check-settings-paths' member paragraph gains the sentence, and its fixture pair gains a malformed registration in `bad/` and an observed one in `good/`. **Not yet applied.**

**The enforcement map** rows an observed registration under a new class, **Observed hooks**, with the guard columns (surface, event, intercepts), whatever its event, and credits it to the member's owner as today. It never rows it under Guards or Session warnings, because an observed hook binds nothing and the page's classes are ordered by how hard each binds. The page's taxonomy gains its bullet last: an **observed hook** is a registered hook run under `--observe`, which logs the decision it would have made and refuses nothing, owner gate-sdk/SPEC.md §The harness-integration arm. A `Malformed` registration keeps the path attribution an unknown operand takes. The value rollup reads the section titles off the emission, so the new class reaches it with no edit there. §enforcement-map's settings-file bullet and guard-row paragraph name the class. **Not yet applied.**

**`check-plugin-parity`'s rendering** carries `--observe` into the composed command where the template's registration carries it, after the member and before `--binding antigravity` on the adapter's rendering, and fails at exit 2 on a `Malformed` template registration, as it does on a template hook that is no member registration. plugin/SPEC.md §The guards' composition bullets say so. **Not yet applied.**

### (4) The no-knob line {design-bearing} {user-facing: operator direction 2026-11-14, lead-relayed (not a ruling): the no-knob line is amended, and the operator was shown that this reverses it as written}

delegation-kit/SPEC.md §The turn-end liveness hook's "**There is no knob, and unwiring the hook is the valve.**" is re-phrased. What it protects stays: no knob turns a refusal off for a dispatch, and no per-dispatch override exists. The line names the two consumer-side levers, each a registration or configuration choice that binds every turn end alike: the wait set (`DELEGATION_KIT_STOP_WAIT`), and the `--observe` registration, a trial that enforces nothing until it is removed. Unwiring stays the valve that removes the hook. The act depends on whether `liveness-hook-false-refusals` lands in the same build batch. Each **Not yet applied.**

- **With it in the batch:** this delta writes the observe half into the line that entry's amendment re-phrases (delegation-kit/SPEC-resume-wait.md delta 8), so the line lands once, naming both levers.
- **Without it:** the line names `--observe` alone beside unwiring. That entry's later landing adds the wait set.

§The delegation model's D1 paragraph, "the valve is **unwiring the hook**, never a knob", gains one clause: a registration under `--observe` is a trial of the whole guard, not an override of one dispatch, so it is not the honour system the paragraph refuses. delegation-kit/README.md step 6's "There is no knob: unwiring is the valve" is re-phrased as the line is, naming `--observe` and the log's knob.

### (5) Cases, the release declaration and the mirrors {mechanical}

Each **Not yet applied.**

- **The arm's cases** (`hook/mod.rs` tests, and a gate-sdk gate test running the built binary on a payload in a scratch repository):
  - `subagent-stop-liveness --observe` on a payload with a running own `shell` task: exit 0, empty stdout and stderr, one log line with `would=block` and the refusal's first line.
  - The same on a payload with no live task: `would=none`.
  - `agent-dispatch-guard --observe` on a `fork` dispatch, which D1 blocks unconditionally: exit 0, silent, `would=block`.
  - `agent-dispatch-guard --observe --binding antigravity`'s adapter path, through the translation unit test: the child argv carries `--observe`, and the adapter answers no decision.
  - An unwritable log: exit 0, the decline envelope on stdout naming `GATE_SDK_HOOK_OBSERVE_LOG`.
  - A trailing token outside the grammar, a repeated `--observe`, and `--binding` before `--observe`: each exit 2 naming the token.
  - Under the tree-free mark: an absolute log, logged; the default log, declining.
  - The decision table's rows, each through a stub child in the unit lane.
- **The readers' cases**: the parser's `Member` and `Malformed` answers; `check-settings-paths`' fixture pair (delta 3); the enforcement map's Observed hooks section and its taxonomy bullet; the plugin rendering's carried flag and its `Malformed` refusal.
- **The release declaration** (`.workflow/release-declarations.md`):
  - Under `## Behavior changes`: `--hook` takes `--observe`, which logs the member's would-be decision and exits 0, and refuses a trailing token outside `--hook <member> [--observe] [--binding <name>]` at exit 2, where it ignored the token and ran the member enforcing.
  - Under `## Knob changes`: `GATE_SDK_HOOK_OBSERVE_LOG`, default under the scratch dir.
  - Under `## New and tightened gates`: `check-settings-paths` and `check-plugin-parity` red a registration outside the `--hook` grammar, which they read as clean before.
- **The regenerated surfaces**: `docs/enforcement.md`, and the mirrors `docs/gate-sdk/SPEC.md`, `docs/delegation-kit/SPEC.md`, `docs/delegation-kit/README.md`, `docs/context-kit/SPEC.md` and `docs/plugin/SPEC.md`, each by the command docs/generated-projections.md rosters for it.

## Producers and consumers

- **The `--observe` token** (delta 1). Producers: a consumer's registration, and the plugin's rendering where its template carries it. Consumers: `hook::run`, `antigravity::run`'s child argv, and the registration parser.
- **The observe log** (delta 2). Producer: `hook::run` under `--observe`. Consumer: the consumer reviewing its trial. No crate reader parses it.
- **The `Member` fields and `Malformed`** (delta 3). Producer: `hook::registration`. Consumers: `check-settings-paths`, the enforcement map's `hook_row` and `hook_sections`, and `check-plugin-parity`'s two renderers.
- **The Observed hooks class** (delta 3). Producer: the enforcement map. Consumers: the page, `check-enforcement-fresh`, and the value rollup through `enforcement_map::measure`.

The registration readers were found by `grep -rln "Registration::\|hook::registration" native/src`, the enforcement map's library consumers by `grep -rn "enforcement_map::" native/src`, the grammar's spellings by `grep -rn -- "--hook <member>"` outside `docs/`, and the no-knob lines by `grep -rn "is the valve\|no knob" delegation-kit/`.

## Existing sections updated

- `gate-sdk/SPEC.md` §The harness-integration arm (deltas 1 and 3), §Layout and configuration (delta 2), §run-gates (delta 1), §enforcement-map (delta 3).
- `context-kit/SPEC.md` §check-settings-paths (delta 3); `plugin/SPEC.md` §The guards (delta 3).
- `delegation-kit/SPEC.md` §The turn-end liveness hook and §The delegation model's D1 paragraph, and `delegation-kit/README.md` step 6 (delta 4).
- `native/src/hook/mod.rs`, `native/src/hook/antigravity.rs`, `native/src/knobs/gate_sdk.rs`, the `--hook` row in `native/src/emit/mod.rs`, `native/src/emit/enforcement_map.rs`, `native/src/gates/settings_paths.rs`, `native/src/gates/plugin_parity.rs` and `native/src/runner.rs`, with their tests, and the `check-settings-paths` fixture pair (deltas 1 to 3 and 5).
- `.workflow/release-declarations.md`, and the regenerated surfaces delta 5 lists.

## Retired spellings

- None — `Registration::Member` keeps its name and gains fields, and the no-knob line is re-phrased rather than renamed.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The local oracle** — `bash gate-sdk/bin/build-native.sh`, the battery, the crate's tests and the fixture suites of gate-sdk, context-kit and delegation-kit, and the repo's own `scripts/gate-tests` suite that holds `check-plugin-parity`'s pair, green. No remote run observes this entry, so its Done move lands before the drain stage, in the build batch that lands it.
