# SPEC amendment: worktree-hook-bin

Every binary-backed harness hook is inert inside a worktree-isolated dispatch. The front end (`gate-sdk/bin/run-gates.sh`) `cd`s to the git toplevel of its cwd and resolves `GATE_SDK_NATIVE_BIN` there. Its default is the repo-relative `native/target/release/checkwright-gates`, and a linked worktree carries no build output, since `target/` is gitignored. `--hook` and `--statusline` are the fail-open set, so each `--hook` exits 0 on its "absent or not executable" line. The bash guard never reaches the front end: `guard-kit/lib/guard.sh` resolves `gate_native_bin` itself, finds nothing executable, prints its "rules did not run" advisory and exits 0. So the fork ban, the isolation claim, the budget guard, the workflow-state guard and every bash-guard rule are advisory for an isolated agent. And the read-only claim rule sends such agents to exactly that place. Measured at scope on 2026-09-23: from a scratch `git worktree add` with no `native/target`, the bash guard, the workflow-state guard and the dispatch guard each exit 0 on a payload each blocks from the main checkout.

**The ruling: on the fail-open path only, a harness-integration arm run inside a linked worktree dispatches to the main checkout's binary.** It never builds one and never widens beyond the fail-open set. One gate-sdk accessor, `gate_harness_bin`, answers which binary a harness-integration arm runs. The front end's fail-open branch and guard-kit's load both ask it, so the two resolution points cannot diverge.

- **Local first.** The spelled `GATE_SDK_NATIVE_BIN` answer, when executable, is the answer. The main checkout pays nothing, and so does any tree that built its own binary.
- **Linked worktree second.** A linked worktree is a cwd whose `git rev-parse --git-dir` differs from its `--git-common-dir`, each crossed by `cd … && pwd -P` (gate-sdk/SPEC.md §The path-dialect contract). The main checkout is the common dir's parent, and only when the common dir's basename is `.git`. A bare repository and a `--separate-git-dir` checkout do not satisfy that, and take no fallback. The binary is resolved *in the main checkout's context*: `gate_native_bin` evaluated there, so a pin held only in the main checkout's gitignored `.local.knobs` overlay is honoured. A rooted value is returned unchanged, and a relative one is joined onto the main checkout. It is used when executable.
- **Otherwise unchanged.** The local spelled answer is returned as today, so every caller's `-x` test, diagnostic text and fail-open exit stay byte-identical.

**Why this does not contradict the liveness hook's ruling, and where that ruling's ground stops.** delegation-kit/SPEC.md §The turn-end liveness hook rules a main-checkout resolution safe for `check-producer-liveness` alone, and "never in how the binary knob resolves", because gates in general compare against the source in their own tree (§check-gate-binary-fresh). That still holds here. `gate_native_bin`, `gate_command`, the battery, `--emit` and every gate dispatch keep resolving locally and keep failing closed in a worktree. The new resolution sits on the one path whose status is a harness decision rather than a verdict. On that path the ground is different. A harness hook enforces the *session's* policy, and the session's wiring is the parent's settings, whose own tool calls the main checkout's binary already guards. So dispatching the child's hooks to that binary is parity with the parent rather than a cross-tree verdict. The alternative on offer is no enforcement at all.

**Refused, with grounds.**

- **Fail closed in a worktree with no binary.** It blocks every tool call of every isolated child whenever the main checkout has no binary either, a pre-build clone or an omit-and-declare install. That is the blast radius §The harness-integration arm refuses, and it would make a worktree strictly more fragile than its main checkout on the same absence.
- **A stated refusal.** It leaves the fork ban, the isolation claim and the budget guard advisory for exactly the agents the read-only rule isolates. guard-kit's own premise also rules it out: a guard binary may not be allowed to fail open on its own absence (guard-kit/SPEC.md §The guard framework), and here a binary is present one directory up.
- **Wiring an absolute path in the settings file.** §run-gates already refuses it: a literal `command` resolves no precedence.

**Honest limits, stated with the ruling.**

- **Skew.** The main checkout's binary can be older than the worktree's tree. If the worktree's guard library or knob files name a knob that binary does not own, `gate_knob_values` refuses at exit 2 and the bash guard blocks every command in that worktree. The same skew already wedges the main checkout when its binary is stale against its own tree, so the window is main-checkout staleness, not a new one.
- **Overlays.** A worktree carries no gitignored `.local.knobs` overlay. A value the operator set only there reaches a worktree child for `GATE_SDK_NATIVE_BIN` (resolved in the main checkout's context) and for nothing else.
- **The guard's steer text.** `_guard_door` still names the worktree-relative door, which does not exist there. It is steer text, not a dispatch.

## What changes

### (1) The accessor {design-bearing}

**Not yet applied.** `gate-sdk/lib/gate.sh` gains `gate_harness_bin`, after `gate_native_bin_spelled`, implementing the three steps above. Its git queries are the shell producer roster's `--git-dir` and `--git-common-dir`, each in `cd` position, so `check-path-dialect` clears them, and the common dir's parent is string arithmetic on an already-crossed value. It spawns git only after the local `-x` test fails. It prints one line and returns 0.

### (2) The front end's fail-open branch asks it {design-bearing}

**Not yet applied.** In `gate-sdk/bin/run-gates.sh`, `exec_arm` takes its binary from `gate_harness_bin` when `ARM_UNAVAILABLE_STATUS` is `0`, and from `gate_native_bin_spelled` otherwise. `FAIL_OPEN_ARMS` is untouched and still assigned once, so `check-front-end-fail-open` reads it unchanged. `gate-sdk/bin/run-gates.ps1` mirrors the resolution in its `$unavailable -eq 0` branch, re-holding the accessor as it re-holds the other four (§run-gates). It evaluates `Get-NativeBinSpelled` with the main checkout as the location.

### (3) The bash guard asks it {design-bearing}

**Not yet applied.** In `guard-kit/lib/guard.sh`'s load, `_guard_bin` takes `gate_harness_bin` when the sourced library defines it, and `gate_native_bin` otherwise, so an older vendored library keeps today's behaviour. When the answer differs from `gate_native_bin`'s, the load exports `GATE_SDK_NATIVE_BIN` as that absolute path before its knob read, so `gate_knob_values` reaches the same binary. The library's own hermetic pin already absolutizes the accessor's answer this way (§lib/test-hermetic.sh). The unreachable-binary advisory and the refused-config block are unchanged.

### (4) The linked-worktree test proves the guards fire {design-bearing}

**Not yet applied.** `gate-sdk/gate-tests/run-gates-linked-worktree.test.sh`, hermetic per §lib/test-hermetic.sh, builds a sandbox repository vendoring this tree's `gate-sdk/bin`, `gate-sdk/lib`, `guard-kit/lib` and the bash-guard template. It commits them, runs `git worktree add` inside the sandbox (`lifecycle-kit/gate-tests/boundary-worktree-refusal.test.sh` is the precedent), and places the pinned binary at the main checkout's default path. Each child runs with `GATE_SDK_NATIVE_BIN` unset, so the default resolves. From the worktree it asserts:

1. a `fork` dispatch payload to `--hook agent-dispatch-guard` exits 2 and carries the fork-ban text;
2. a Write to `.workflow/WORKFLOW-STATE.txt` through `--hook workflow-state-guard` exits 2;
3. the bash guard on a payload it blocks exits 2, not the advisory;
4. with the main checkout's binary also removed, `--hook` exits 0 on the absent-binary line (the control);
5. `--emit knob-roster` still reports the binary absent at exit 2, so the fallback is scoped to the fail-open set;
6. an absolute pinned `GATE_SDK_NATIVE_BIN` naming nothing takes no fallback.

**Inferred, cannot run before build:** assertion 1 exits 0 against today's front end, which makes it a red-before-green witness — that depends on the sandbox layout delta 4 builds.

### (5) The twin's parity corpus covers the linked layout {design-bearing}

**Not yet applied.** `native/src/emit/front_end_parity.rs` gains cases on a hand-built linked-worktree layout:

- the worktree's `.git` file names `<main>/.git/worktrees/<wt>`, which carries `HEAD`, `commondir` and `gitdir`.

The layout is hand-built because the crate's own `proc` unit test refuses any crate source that adds a worktree. Three cases:

- a `--hook` dispatching through the main checkout's binary;
- an `--emit` still reporting absent at 2;
- both binaries absent, where `--hook` exits 0.

**Honest limit:** the hand-built layout rests on git's documented on-disk worktree format (gitrepository-layout).

### (6) The owning sections state the resolution {mechanical}

**Not yet applied.**

**gate-sdk/SPEC.md §The harness-integration arm.** In the paragraph opening "**The absent-binary behavior, which the port creates and must therefore settle.**", the sentence "*Cannot run at all* has one cause the front-end can see, an absent or non-executable binary, and it takes this branch." becomes:

> *Cannot run at all* has one cause the front-end can see, an absent or non-executable binary, and it takes this branch — after one resolution the rest of the front end never makes. Inside a linked worktree, which carries no build output, a fail-open arm dispatches to the main checkout's binary through `gate_harness_bin` (§lib/gate.sh) before it declines. A hook enforces the session's policy, and that binary already guards the parent session's own calls. Every verdict-bearing path keeps resolving locally, failing closed, so the liveness hook's ruling that a main-checkout binary is never how the binary knob resolves (delegation-kit/SPEC.md §The turn-end liveness hook) stands for everything this branch does not reach.

**gate-sdk/SPEC.md §run-gates.**

- In the front-end paragraph's residue list, "locate the binary over `GATE_SDK_NATIVE_BIN`'s pre-binary precedence" becomes "locate the binary over `GATE_SDK_NATIVE_BIN`'s pre-binary precedence, through `gate_harness_bin` for a fail-open arm".
- "the four accessors it needs" becomes "the five accessors it needs", and the list gains `gate_harness_bin`.
- In the `--run-front-end-parity` paragraph, the corpus list "each exit path, each precedence tier of `GATE_SDK_NATIVE_BIN`, each residual-grammar form, and forwarded stdin" gains "and the linked-worktree resolution on a hand-built layout".

**gate-sdk/SPEC.md §lib/gate.sh.** After the `gate_native_bin_spelled` bullet, add:

> - **`gate_harness_bin` answers which binary a harness-integration arm runs, and nothing else asks it.** It returns `gate_native_bin_spelled`'s answer when that is executable. Inside a linked worktree whose common dir is `<main>/.git`, it returns the main checkout's own resolution of the knob, rooted, when that is executable. Otherwise it returns the local answer unchanged, so every caller's absent-binary test and diagnostic are untouched. Its callers are the front end's fail-open branch and guard-kit's load (guard-kit/SPEC.md §The guard framework). It is not a knob accessor and not a second default: the knob's value and precedence are `gate_native_bin`'s, read in another tree.

**guard-kit/SPEC.md §The guard framework (`lib/guard.sh`).** In the paragraph opening "**The load fails in one of two ways**", "When the binary cannot be reached — no `gate-sdk/lib/gate.sh` beside the vendor root, or no executable at `GATE_SDK_NATIVE_BIN` —" becomes "When the binary cannot be reached — no `gate-sdk/lib/gate.sh` beside the vendor root, or no executable where `gate_harness_bin` looks, which inside a linked worktree includes the main checkout (gate-sdk/SPEC.md §lib/gate.sh) —".

**delegation-kit/SPEC.md §The delegation model.** In the **Degradation** paragraph, "The binary being absent takes the general fail-open path (gate-sdk/SPEC.md §The non-gate arm) before this member ever runs — silently, with no advisory of this guard's own." becomes "The binary being absent takes the general fail-open path (gate-sdk/SPEC.md §The non-gate arm) before this member ever runs — silently, with no advisory of this guard's own — and inside a linked worktree it is absent only when the main checkout carries none either, since the front end dispatches there first."

**delegation-kit/SPEC.md §The turn-end liveness hook.** In the paragraph opening "**Not re-implementing a resolution for the override is a narrowing**", after "…which is why it belonged in one consumer front end for one gate and never in how the binary knob resolves.", add: "The front end's fail-open branch does take a main-checkout resolution (gate-sdk/SPEC.md §The harness-integration arm), and on a different ground: it dispatches the session's policy hooks, not a gate's verdict, and it leaves the knob's resolution untouched."

## Producers and consumers

- **`gate_harness_bin`'s answer** (delta 1). Producers: the local knob resolution, and the main checkout's, reached through git's own common-dir pointer. Consumers: `exec_arm`'s fail-open branch at every `--hook` and `--statusline` firing (delta 2), and guard-kit's load at every bash-guard firing (delta 3). The PowerShell twin re-holds it (delta 2), and `--run-front-end-parity` holds the twin to the stub (delta 5).
- **The exported absolute `GATE_SDK_NATIVE_BIN`** (delta 3). Its one reader is `gate_knob_values`, inside the same guard process, at the knob read that follows.
- **Point 5.** No corpus narrows. The fail-open set is unchanged. What changes is which binary its members reach.
- **Point 6.** The obliged members are the harness-wired arms in `.claude/settings.json`: `--hook subagent-stop-liveness`, `--hook agent-budget-guard`, `--hook agent-dispatch-guard`, `--hook workflow-state-guard`, `--statusline`, and the bash guard. The first five are satisfied by delta 2 and the sixth by delta 3. `scripts/session-context.sh` is a SessionStart hook resolving its own `[[ -x ]]` door; it gates no action, and it is outside this ruling.

## Existing sections updated

Roster from `git grep -n "absent or not executable\|Cannot run at all\|The binary being absent\|The load fails in one of two ways\|never in how the binary knob resolves\|four accessors" -- '*.md' ':!docs'` and `git grep -n "gate_native_bin" -- '*.sh'`, run 2026-09-23.

- `gate-sdk/SPEC.md` §The harness-integration arm, §run-gates and §lib/gate.sh (delta 6).
- `guard-kit/SPEC.md` §The guard framework (delta 6).
- `delegation-kit/SPEC.md` §The delegation model and §The turn-end liveness hook (delta 6).
- `gate-sdk/lib/gate.sh` (delta 1).
- `gate-sdk/bin/run-gates.sh` and `gate-sdk/bin/run-gates.ps1` (delta 2).
- `guard-kit/lib/guard.sh` (delta 3).
- `gate-sdk/gate-tests/run-gates-linked-worktree.test.sh` (delta 4).
- `native/src/emit/front_end_parity.rs` (delta 5).
- `.workflow/release-declarations.md`, a Behavior changes bullet: inside a linked worktree with no binary of its own, `run-gates.sh --hook`/`--statusline` and the bash guard now dispatch to the main checkout's binary, so an isolated agent's hooks enforce rather than decline (deltas 2 and 3).
<!-- update-target-exempt: generated mirrors, regenerated by their freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`, `docs/guard-kit/SPEC.md` and `docs/delegation-kit/SPEC.md`.

## Retired spellings

- None — no delta retires a spelling; the fail-open set and every accessor keep their names.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the accessor and both callers.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved.** `worktree-hook-guards-fail-open` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Nothing retired.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
