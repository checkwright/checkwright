# SPEC amendment: scratch-registers-nothing

The upgrade-smoke arm builds each ref's binary in a linked worktree of the host
repository, so a run killed from outside leaves a registration in the host's
`.git/worktrees/`. That registration outlives the scratch base's reclaimers and
makes the next iteration-boundary entry refuse. This amendment removes the
registration rather than guarding it. The per-ref checkout becomes a shared local
clone inside the scratch base, and gate-sdk states the rule the change instances:
a member writing scratch registers nothing outside the scratch base.

## What changes

### (1) The per-ref binary builds in a shared clone, never a linked worktree

`ref_binary_tree` (`native/src/emit/upgrade_smoke.rs`) stops adding a detached
worktree. **{mechanical}** The build still needs a git checkout, so it takes one this way:

1. Resolve the ref to a commit id in the host:
   `git -C <repo> rev-parse --verify -q <ref>^{commit}`. The clone's own refs
   differ from the host's (its `HEAD` is the host's default branch, not a
   detached host `HEAD`), so a ref name is never resolved inside the clone.
2. Clone it: `git clone -q --shared --no-checkout <repo> <work>/checkout-<label>`.
3. Check it out: `git -C <work>/checkout-<label> checkout -q --detach <commit>`.

The checkout path stays the same, so every later reader of the built binary's
path is unchanged. A failure at any of the three steps is exit 2. It reuses the
existing `FAIL(env)` line with its noun corrected: "could not check out the <label>
ref (<ref>)".

`Scratch` loses its `worktrees` field, and its `Drop` loses the
`git worktree remove --force` loop, because each clone lies under `work`, which
`Drop` already removes. `repo` stays only while `ref_binary_tree` reads the clone
source through it. The two code comments that call `Drop` "the trap" and say
"detached worktree" are rewritten to match (update targets below).

**Probed at authoring, 2026-09-21.** On this repository, the clone took 0.009 s and
the detached checkout 0.083 s. `git worktree list` in the host was unchanged
afterwards, and the clone's `objects/info/alternates` named the host's object
store. `cargo build --release --offline --manifest-path <clone>/native/Cargo.toml`
succeeded, which means `native/build.rs`'s `git ls-files` stamp runs inside a
clone as it does inside a worktree.

**Replacement text for gate-sdk/SPEC.md §upgrade-smoke** — **Not yet applied.**
It replaces the paragraph that opens "A ref's binary comes from a detached
worktree at that ref":

> **A ref's binary comes from a git checkout of that ref, never from the archive
> its kits come from.** `native/build.rs` stamps the crate's source by running
> `git ls-files` and panics, saying why, when that fails: the crate builds inside
> its own git checkout by construction and is never vendored. An archive-and-build
> therefore dies in the build script rather than in the compiler, a failure that
> reads as a broken tag. The checkout is a **shared local clone** under the
> scratch base (`git clone --shared --no-checkout`, then a detached checkout of
> the commit the ref resolves to *in the host*). It is never a linked worktree,
> because a linked worktree registers itself in the host's `.git/worktrees/`, and
> a registration outlives the process that made it. Rust runs no destructor on a
> signal, so a run killed from outside would leave the registration behind, and
> the iteration-boundary entry refuses on any linked worktree
> (lifecycle-kit/SPEC.md §bin/enter-stage.sh). A clone registers nothing: a killed
> run leaves only scratch, which the scratch reclaimers already own. Its
> `native/target/` is scratch as well, so the host's build output, which
> `check-gate-binary-fresh` judges, is untouched.
>
> **The two alternatives were priced and declined.** A signal handler covers
> SIGTERM and SIGINT but never SIGKILL. It would also be the crate's first
> process-global signal writer (§lib/gate.sh's process-global rule, which states
> the crate spells none), and on Windows only a console control handler exists.
> A reaper at the boundary would reverse lifecycle-kit's ruling that the reap is
> a session act. The clone leaves nothing for either one to handle.

The cold-build measurement paragraph that follows the replaced paragraph is kept
unchanged. Its sentence "The worktree build shares the host's cargo home" becomes
"The clone build shares the host's cargo home".

In the knob list, the scratch-base bullet is rewritten to:

> - Scratch base is the existing `GATE_SDK_TMP_DIR` knob; the extracted trees, the
>   per-ref clones and the consumer are created under it and removed on every
>   ordinary exit, and a killed run leaves nothing outside it.

In the declined-`tar` paragraph, "replacing `git archive | tar -x` with a per-ref
detached worktree would drop it, and the machinery is present since the binary
build adds worktrees per ref" becomes "replacing `git archive | tar -x` with a
per-ref checkout would drop it, and the machinery is present since the binary
build checks out each ref".

### (2) A member writing scratch registers nothing outside the scratch base

gate-sdk's `GATE_SDK_TMP_DIR` knob entry (§Layout and configuration) gains the
rule that delta 1 instances. **{mechanical}** **Replacement text** — **Not yet applied**:

> `GATE_SDK_TMP_DIR` (default `.tmp`; the battery runner declares it beside every
> member writing scratch, and such a member keeps all of its residue inside it —
> it registers nothing in a repository outside it, so a killed run's leftovers are
> scratch the reclaimers own, §upgrade-smoke)

**Its machine side is a crate unit test**, in the roster shape of `knobenv.rs`'s
`no_module_outside_this_one_writes_the_environment`. It lives in `native/src/proc.rs`'s
test module, beside the spawn roster, and is named
`no_crate_source_adds_a_linked_worktree`. It scans every `.rs` file under
`native/src/` and reds on any file containing the string literal `"worktree"` with
the literal `"add"` as the next non-whitespace, non-comma token (a line break
counts as whitespace, so a rustfmt wrap between the two is caught). Like its model,
it asserts first that the scan found at least one source file. The failure message
names the file and points at §upgrade-smoke's shared-clone shape.

**Honest limit:** the scan sees that one spelling. A worktree added through a
variable holding `"add"`, or through `bash -c`, evades it. The scan is a floor,
never a proof, and it is stated that way where it is merged.

**The ruling this rule makes for the other scratch producers.** Every other crate
scratch producer (the `impl Drop` roster below) creates only directories under
the scratch base. So a signal leaves only scratch behind, and none of them owes a
signal guard. That includes `--run-consumer-smoke`'s `Teardown`, the twin the
queue entry asked about, and `check-kit-roots-dialect`'s base, whose
ordinary-exit leak is its own debt entry.

## Producers and consumers

- **The shared clone (delta 1).** *Producer:* `ref_binary_tree`, which is reached
  whenever a ref's vendored kits carry a `*.gate`: every ref from the first tag
  shipping `native/` on, and `TO` at `HEAD` in the `upgrade` validate suite
  (`scripts/evidence-config.knobs`). *Consumers:* `cargo build` inside the clone
  (build.rs's `git ls-files`, probed), and the phase runs that exec the binary at
  `<work>/checkout-<label>/native/target/release/…`, a path this delta leaves
  unchanged. *Roster-holding reader of the surface it leaves:* the
  `--enter-stage` linked-worktree scan (`enter_stage.rs` `worktree_scan`). This
  delta narrows that reader's corpus by one producer. **Red condition (point 5):**
  the boundary refusal reds when it finds *at least one* linked worktree, which is
  monotone in the set, so removing a producer can only remove refusals. Its fixture
  `lifecycle-kit/gate-tests/boundary-worktree-refusal.test.sh` adds its own
  worktrees and never depends on this arm. Probe:
  `grep -n "worktree" lifecycle-kit/gate-tests/boundary-worktree-refusal.test.sh`.
- **The field removed from `Scratch` (delta 1).** Probe:
  `grep -n "env.repo\|env.worktrees\|self.repo\|self.worktrees" native/src/emit/upgrade_smoke.rs`
  returned four lines. `worktrees` is written by the `worktree add` push (:534) and
  read only by the `Drop` loop (:71), and both go with it. `repo` is read by that
  loop (:74) and by the `worktree add` spawn (:523), which becomes the clone source.
- **The rule and its unit test (delta 2).** *Producer:* the test runs under
  `cargo test`, which `check-crate-arms` runs. *Consumer:* a contributor adding a
  `worktree add` spawn, who meets the red. *Corpus enumerable at authoring
  (point 6):* `grep -rn '"worktree"' native/src` matched four files. Each one's
  satisfying value:
  - `upgrade_smoke.rs` — delta 1 removes the `add` spawn. Its remaining
    `"worktree"` spellings disappear with the `Drop` loop.
  - `enter_stage.rs` — `"worktree", "list"`, not `add`.
  - `scan_prompts.rs` — `"worktree", "reflog"` in a word list, not `add`.
  - `hook/dispatch.rs` — `"worktree"` compared against an isolation string, with
    no `"add"` following it.
- **Scratch-producer roster (delta 2's ruling).** Probe:
  `grep -rn "impl Drop for" native/src`. It found 14 impls: `proc.rs` `Piped`
  (a pipe, not scratch), and scratch structs in `upgrade_smoke`, `run_guard_tests`,
  `run_validate` (`Claimed`, a lock claim), `rewrite` (test), `front_end_parity`,
  `agents_md_smoke`, `pack_installer`, `stop_liveness` (test), `run_index_tests`,
  `knobs/mod.rs` (test), `demo`, `enter_stage`, and `run_consumer_smoke`. Only
  `upgrade_smoke` spawns a `worktree add` (probe: `grep -rn '"worktree"'` above),
  so the rule obliges no other member.

## Existing sections updated

Roster probe: `git grep -n "trap-removed\|detached worktree\|per-ref worktree"`
over the tracked tree, run at authoring.

- `gate-sdk/SPEC.md` §upgrade-smoke — the "A ref's binary comes from a detached
  worktree" paragraph, the "worktree build shares" sentence, the scratch-base knob
  bullet, and the declined-`tar` paragraph's worktree clause (delta 1).
- `gate-sdk/SPEC.md` §Layout and configuration — the `GATE_SDK_TMP_DIR` knob
  entry (delta 2).
- `native/src/emit/upgrade_smoke.rs` — the `Scratch` comment ("trap-removed …
  `Drop` is that trap"), the `ref_binary_tree` comment ("detached worktree"), and
  the `FAIL(env)` line (delta 1).
- `native/src/proc.rs` — the new unit test (delta 2).
- `lifecycle-kit/SPEC.md` §bin/enter-stage.sh — the boundary refusal's example
  list drops "a leaked per-ref worktree", which no kit producer makes anymore.
  This changes wording only, not a contract (delta 1).
- `docs/gate-sdk/SPEC.md` — the generated on-site mirror, regenerated with
  `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).
- `docs/lifecycle-kit/SPEC.md` — the same mirror of lifecycle-kit's SPEC, from the
  same regen (delta 1).

## Retired spellings

- `trap-removed` — §upgrade-smoke's claim that `Drop` is the shell form's trap,
  which it never was on a signal (delta 1).
- `detached worktree` — the per-ref checkout's mechanism (delta 1).
- `per-ref worktree` — the same mechanism, in the knob bullet and the lifecycle-kit
  example (delta 1).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Oracle** — the `upgrade` validate suite (`--upgrade-smoke`) passes, and
      `git worktree list` in the host shows only the main checkout both during a
      run and after it. A run killed with SIGKILL mid-build leaves `git worktree
      list` unchanged.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template, agent definition or shim carries no grounds; a delta places them.
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines; the merged spec reads as one document.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every retired spelling above is gone from the
      tracked tree (the queue's history prose aside), checked by
      `check-amendment-retired-spelling`.
- [ ] **Entry moved** — the paired queue entry moves to Done before the drain
      stage, in the merge commit.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks.
