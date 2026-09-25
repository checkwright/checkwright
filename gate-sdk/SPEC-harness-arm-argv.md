# SPEC amendment: harness-arm-argv

gate-sdk/SPEC.md §The harness-integration arm states argv for `--hook` alone. Its two siblings, `--statusline` and `--usage-poll`, take `_args` and ignore them (`native/src/hook/statusline.rs`, `native/src/hook/poll.rs`), so a surplus or misspelt argument is dropped at exit 0. Every other non-gate arm refuses one: the `--emit` family through its grammar, and `--run-validate` since the close that filed this entry.

**The ruling: each sibling answers a surplus argument on the channel its caller reads.**

- **`--usage-poll` refuses.** Its callers read its status: a timer's log, the `DELEGATION_KIT_REFRESH_CMD` spawn, a session. The refusal is exit 2, before any file read or spawn, and 2 stays distinct from the fail-soft cycle's 1.
- **`--statusline` drops, and says so on stderr.** The harness discards its status and reads only its stdout. A refusal would therefore speak a status nobody reads and blank the one thing the harness shows. The stderr line reaches the one caller who can read it, a hand run.

The drop does not settle the open question the section records: whether `--statusline` is a third branch (*an arm whose status the harness ignores declines at 0*) or the fail-open branch read more widely. It rests on which channel the caller reads, and both readings agree on that.

`--help` is an argument like any other on both arms. The usage home is the front-end's `--help` paragraph each arm already holds (`native/src/runner.rs`), so no per-arm help flag is minted. That matches the `--emit` family's retirement of per-arm help this iteration (`emit-per-arm-help-residue`).

**Measured at authoring (2026-09-26):**

- `./native/target/release/checkwright-gates --usage-poll --bogus` exits 0 after a full poll cycle. The `--statusline` probe was not re-run, since an empty payload rewrites the snapshot empty. Scope's survey records it at exit 0, and no commit to `native/src` has landed since that survey's rev.
- The one harness registration is `.claude/settings.json`'s `statusLine.command`, `bash gate-sdk/bin/run-gates.sh --statusline`, which passes no argument. `delegation-kit/smoke/install.sh` runs `--usage-poll` bare. `verdict.rs`'s `refresh` spawns the consumer's `DELEGATION_KIT_REFRESH_CMD` argv and discards its status (`let _ = proc::run(...)`), so a refused refresh leaves the snapshot to age into STALE, never into a false green.
- The precedent is `native/src/emit/run_validate.rs`'s `dispatch`: the first argument is refused before configuration is read, with `takes no arguments (got: <arg>) — usage: run-gates.sh --help`.

## What changes

### (1) The section states both siblings' argv {mechanical}

**Not yet applied.** In gate-sdk/SPEC.md §The harness-integration arm, the paragraph opening "**`--statusline` and `--usage-poll` are siblings of `--hook`, not members of it.**" becomes:

> **`--statusline` and `--usage-poll` are siblings of `--hook`, not members of it, and neither takes an argument.** That arm's contract is the hook protocol: an envelope on stdout and the exit status as the harness's allow or block signal. A status-line renderer writes an ANSI line whose status the harness ignores, and a usage-snapshot refresher has no harness event at all, its callers being a refresh-command knob and the session that invokes it. Folding either into `--hook` would make its output contract "whatever the member's integration point expects", which is no contract. Each satisfies the class's properties, names its own caller, and answers a surplus argument, `--help` included, on the channel that caller reads:
>
> - `--usage-poll` refuses it at exit 2 before it reads a file or spawns `curl`, naming the argument and the front-end's `--help`. Its callers read the status, and 2 stays distinct from the fail-soft cycle's 1.
> - `--statusline` drops it, names it in one stderr line written before the payload is read, and renders as it would bare. A refusal would speak a status nobody reads and blank the line, the only thing the harness shows. The drop rests on the channel alone, so it leaves the open question above unsettled.

In `native/src/runner.rs`, the front-end `--help` paragraphs gain the argv clause: `--statusline` "takes no argument, and names one on stderr and ignores it"; `--usage-poll` "takes no argument, and refuses one with exit 2".

### (2) `--usage-poll` refuses an argument before it reads anything {mechanical}

**Not yet applied.** `poll::run` takes `args` and, as its first act, refuses a non-empty argv: one `usage-poller:` stderr line naming the first argument and pointing at `run-gates.sh --help`, then exit 2. The spelling follows `run_validate::dispatch`. No knob is resolved, no file read and nothing spawned before the check. A unit test in the module's test block calls `run` with `--bogus` and with `--help` and asserts 2 for each. Because the check comes first, the test reads no knob and spawns nothing.

### (3) `--statusline` names a dropped argument on stderr {mechanical}

**Not yet applied.** `statusline::run` takes `args`, and before `hook::read_payload` it writes one stderr line naming every argument it ignores. The text comes from a pure helper that returns `None` for an empty argv and the line otherwise. The render and the exit status are unchanged. A unit test holds the helper: `None` bare, and a line naming both tokens for `--bogus --help`. The helper is tested rather than `run`, which reads stdin.

### (4) Release declaration {mechanical}

**Not yet applied.** Under `## Behavior changes` in `.workflow/release-declarations.md`:

> - **`--usage-poll`, `--statusline`** — `--usage-poll` now refuses any argument, `--help` included, at exit 2 before it polls, where the argument was dropped and a cycle ran. `--statusline` still ignores an argument and renders as before, and now names it on stderr. Act only if a timer, a `DELEGATION_KIT_REFRESH_CMD` value or a `statusLine` command of yours passes either arm an argument: drop the argument.

## Producers and consumers

- **The `--usage-poll` refusal.** Producer: `poll::run`, reached when any caller passes an argument. That is reachable in every deployed configuration, because a timer line, the refresh knob and a session all pass argv. Consumers: a timer's log and a session read the status and the stderr line. The refresh spawn discards both, and its reader is `usage-verdict`'s staleness check, which reads the untouched snapshot as STALE once it ages.
- **The `--statusline` stderr line.** Producer: `statusline::run`, on a non-empty argv. Consumer: a hand run's terminal. The harness path makes no claim about it. No gate reads either output.
- **Fields.** Neither output adds a field. The refusal names the argument and the front-end help, and the stderr line names the ignored tokens.
- **Existing readers of these arms.** `.claude/settings.json`'s `statusLine`, `delegation-kit/smoke/install.sh`, `native/src/emit/front_end_parity.rs` (argv `--statusline`, bare) and `check-front-end-fail-open`'s pair each pass no argument, so none meets a changed behaviour. `FAIL_OPEN_ARMS` and `FENCE_SAFE_ARMS` are unchanged.
- **Point 5.** No corpus narrows.
- **Point 6.** The obliged members are the two siblings, and each one's satisfying behaviour is its bullet in delta 1.

## Existing sections updated

The roster comes from `git grep -n -- "--usage-poll\|--statusline"` over the tracked tree, and from `grep -n "_args" native/src/hook/poll.rs native/src/hook/statusline.rs`, both run 2026-09-26.

- `gate-sdk/SPEC.md` §The harness-integration arm (delta 1).
- `native/src/runner.rs`, the `--statusline` and `--usage-poll` help paragraphs (delta 1).
- `native/src/hook/poll.rs`, `run` and its test block (delta 2).
- `native/src/hook/statusline.rs`, `run`, the helper and its test block (delta 3).
- `.workflow/release-declarations.md` (delta 4).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`.

## Retired spellings

- None — the deltas add argv behaviour to two arms and retire no name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the refusal and the stderr line.
- [ ] **Instruction surfaces: instruction only.** No instruction surface is edited.
- [ ] **Merged with no information lost.** Delta 1 re-phrases the siblings paragraph rather than appending a new one.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `harness-arm-argv-unstated` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
