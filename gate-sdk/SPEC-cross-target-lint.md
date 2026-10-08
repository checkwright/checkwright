# SPEC amendment: cross-target-lint

`check-crate-arms` gains an opt-in fourth arm that runs a consumer-named lint command, so a host that can lint the crate for a target it does not build catches a target-only lint before the push.

## What changes

### (1) `GATE_SDK_CRATE_TARGET_LINT_CMD` names a fourth arm {design-bearing} {user-facing: lead decision on an operator direction, 2026-10-08, on the queue entry — a contributor-side catch before the push that runs on a Linux host with no Windows machine and no operator step; whether it is a container arm and how it degrades is this stage's}

A **command knob**: an indexed argv, one element per word, spawned directly with no shell from the repository toplevel, so it takes no environment override and a command needing shell syntax names its interpreter and command string as elements. Default empty, which adds no arm, so every host that does not arm it runs the three arms as today.

Where it is set, the arm runs after the fixture arm and unconditionally on the other three's verdicts, under the gate's existing predicate, with git's repository locators stripped as for every spawn. Its merged output is relayed on failure as each arm's is.

- **A non-zero exit is the gate's failure.** The command is a lint at deny level, and its red is a finding.
- **A command that cannot be spawned is exit 2**, naming the knob. A host that armed a lint it cannot start is a machine that cannot answer, and a skip there would read as a clean target. This keeps the section's rule that no gate branches on a missing program at run time: the opt-in is the knob, set once, never a probe on each run.
- **In a linked worktree the arm is never spawned**: the cache-or-refuse rule the section states for the other arms covers it unchanged.

**The kit names no target, no container and no toolchain manager.** What a host can lint beyond its own target is a fact about that host: one carrying a toolchain manager and the target's library names cargo directly, and one carrying neither names a container that does. Both are one argv to the gate. So the mechanism is a command and the choice is config, and since the fact is per machine the value usually lives in the gitignored knob overlay (§The knob file), where CI and a fresh clone do not read it. CI needs no copy: its own target legs are the oracle this arm runs ahead of.

**Two shapes were weighed and refused.** *A built-in cross-target clippy that runs when the target is found installed* is the run-time presence probe the section refuses, and it would do nothing on a host with no toolchain manager, the build host's own state. *A pre-push hook* has no tier to ride: the battery's tiers are commit-time and audit-time, and a third hook is a second enforcement surface for one member.

### (2) The cache key covers what the arm reads {design-bearing}

While the knob is set, the source-stamp cache's key also takes the knob's argv and the bytes of the root toolchain file. A record written before the host armed the arm then misses, so arming is never answered by an older green; and a moved pin misses on a host whose own cargo does not read the file, where the command's toolchain may. With the knob empty the key is what it is today, so no existing record is invalidated by this amendment landing.

**Honest limit.** The key cannot see inside the command: a container image that moved under an unchanged argv, or a toolchain the command fetched anew, changes the lint with no miss. The next crate edit re-runs it.

### (3) §check-crate-arms' limits and program roster say what the arm changes {mechanical}

- The sentence that only a Windows leg compiles the crate's `cfg(not(unix))` code, and the honest limit that a lint firing only there arrives with the push, each gain the qualifier: on a host that has not armed the target-lint arm.
- The compile-only Windows check stays no gate and its paragraph is unchanged: it needs a toolchain manager and network on every host, which the contributor floor does not carry, where this arm asks them of the one host that opted in.
- The declared-programs paragraph gains the program the knob names, declared as the fixture arm's runner is, so the observed-within-declared unit test holds while the knob is set.
- The arm count in the section's cache and worktree paragraphs reads four where the knob is set.

### (4) This repo arms it through a container, in the overlay {design-bearing}

On the build host the gitignored gate-sdk overlay names a container run of the stock Rust image: the tree mounted read-only and marked a safe directory, the pinned toolchain resolved from the root toolchain file by the image's own toolchain manager, the Windows MSVC target added, and `cargo clippy --release --all-targets` at `-D warnings` for that target against the crate's manifest. The toolchain home, the cargo home and the target directory are named volumes, so a second run compiles incrementally and fetches nothing. Scope ran that recipe clean on this host with a container runtime present and no toolchain manager on `PATH`; `command -v docker rustup` re-read at authoring returns the runtime alone.

The build session writes the overlay lines and reports them, since no commit carries them, and no operator step follows. The recipe's exact argv is build's to settle against the two witnesses in the Definition of Done.

## Producers and consumers

- **`GATE_SDK_CRATE_TARGET_LINT_CMD`** — producer: a host's knob overlay; this repo's build host sets it (delta 4), and the crate's cases set it to a stub. Consumer: `check-crate-arms`, which spawns the argv and folds it into the cache key. Every element is read, as argv.
- **The arm's verdict** — producer: the command's exit status. Consumer: the committing session, through the gate's report.
- **The cache record** — existing; its key gains two inputs (delta 2), read by the same lookup in the main checkout and in a linked worktree.
- **Roster-holding readers of the minted knob name** — the crate's knob table and rendered roster, the kit's knob template, the kit SPEC's knob list, the two spawn rosters gate-sdk/SPEC.md keeps, and the release declarations, each an update target below.
- **A narrowed corpus** — none. **An obligation on every member of a corpus** — none: the gate's fixture cases leave the knob unset and run the three arms as today.

## Existing sections updated

Produced by a read of gate-sdk/SPEC.md §check-crate-arms and `git grep -n 'GATE_SDK_CARGO_TARGET_DIR\|REFRESH_CMD' -- gate-sdk/SPEC.md`.

- `gate-sdk/SPEC.md` §check-crate-arms — the fourth arm beside the fixture arm's paragraph (delta 1), the cache paragraphs (delta 2), the Windows-leg sentence, the honest limit, the declared-programs paragraph and the arm counts (delta 3); §Layout and configuration — the knob bullet (delta 1); the crate's network-spawner roster and its consumer-changeable spawn list, which today name arms alone, each gaining this gate's command where set (delta 1).
- `native/src/gates/crate_arms.rs` — the arm, the key and the declared program (deltas 1, 2 and 3); `native/src/knobs/gate_sdk.rs` — the row (delta 1).
- The crate's cases under a stub command: a zero exit clean, a non-zero exit red with the output relayed, an unresolvable program at exit 2 naming the knob, an empty knob spawning nothing, and a changed argv missing the cache (deltas 1 and 2).
- `gate-sdk/templates/gate-sdk-config.knobs` and `gate-sdk/README.md` where they list knobs (delta 1).
- `.workflow/release-declarations.md` — a row for the knob (delta 1).
- `docs/gate-sdk/` and every other generated projection of the surfaces above — regenerated, never hand-edited (all deltas).

## Retired spellings

- None — no delta retires a name; the three arms and their cache are unchanged while the knob is empty.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Seen red on this host** — with the overlay armed, a seeded Windows-only use of an API newer than the crate's floor under `cfg(not(unix))` reds the gate at commit time and names the lint; the seed is removed in the same session and never committed.
- [ ] **Seen green offline** — after one run has filled the volumes, a run with the container's network disabled is green on the unseeded crate.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`), which the iteration's last gate-sdk batch discharges.
- [ ] **Queue entry done** — `--queue done windows-cfg-msrv-lint-local` in the merge commit, before the stage that drains the queue.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
