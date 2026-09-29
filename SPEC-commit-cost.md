# SPEC amendment: commit-cost

No public page states what the battery costs a commit, which is the adopter's first question about a pre-commit hook. The one timing record in the tree, `.workflow/gate-timing-baseline.txt`, times the pre-port bash battery on one Linux host (2026-08-02), and nothing reads it: `git grep` finds no reader in `native/src` or `scripts`, and every gate it timed has since ported.

This amendment adds a figure an adopter can read and reproduce. It is the pre-commit hook's own work, measured by the hook's own code, on a stated host and profile, re-measured at every release and held to the release by a gate. The unread baseline is retired.

Two queue entries pair it: [per-commit-cost-figure](TASK-QUEUE.md#per-commit-cost-figure) and [gate-timing-baseline-comparability](TASK-QUEUE.md#gate-timing-baseline-comparability).

**The rulings.**

- **The hook's own run is what is measured.** The `pre-commit` arm runs its members **serially**, stopping at the first red (gate-sdk/SPEC.md §git-hook). `--run` runs the same members on a worker pool, so the battery's wall-clock under `--run` understates a commit, and the timings file's `TOTAL` is a sum under contention. `--measure-commit` calls the hook's own selection and dispatch, so the figure cannot drift from what a commit runs.
- **The worst case, on a fresh `full` install.** Every tracked path counts as staged, so every pre-commit member whose triggers match anything runs, each staged-mode member over every file it matches. `full` is the top of the profile lattice, so every other profile's members are a subset of its members, and the figure bounds a commit in any freshly initialized repository. The reference repository is one the pinned release's one-line install has just initialized, which is the tree an adopter starts from.
- **An adopter measures their own.** The same arm, run in the adopter's repository, answers the question for that repository. The page names it, so the published figure is a reference point, never the only reading.
- **Re-derived at release, held by the pin gate.** The page's figure names the release it was measured at, and `check-install-pin` gains an invariant holding that version to the pin. The commit that moves the pin must therefore carry a new measurement. The figure is one host's, so no gate can re-run it. The gate holds its currency and nothing more, and the page says whose host it was.
- **The per-platform limits are stated, not measured.** The figure excludes the hook's process start, which runs through `sh`. On Windows that is Git for Windows' bundled sh, emulated on Arm. The start cost per operating system is [native-executable-git-hooks](TASK-QUEUE.md#native-executable-git-hooks)' third deliverable, and until it lands the page names the cost as unmeasured. A per-OS and per-arch CI table was refused at filing: CI job times measure runner hardware rather than the operating system.
- **The baseline is retired, not compared.** A comparer would compare the native battery against a bash substrate that no longer ships, and nothing would read its answer. The commit-cost figure is the timing record that has a reader.
- **The seam.** The arm is gate-sdk mechanism, since any consumer's hook can be measured. The page block, the pin invariant and the release step are this repository's own publishing, at the root.

## What changes

### (1) The `--measure-commit` arm {design-bearing}

**Not yet applied.** gate-sdk/SPEC.md gains a section after §git-hook:

*### measure-commit*

*The binary's `--measure-commit` arm times what the `pre-commit` hook would run if every tracked file were staged. It lists the tracked paths with `git ls-files -z` and passes them to the hook arm's own selection and serial dispatch (§git-hook), three times. Nothing is staged, and the tree and the index are left as they were. It prints two lines:*

```text
measure-commit: <median>ms median of 3 (<a>ms, <b>ms, <c>ms) — <m> pre-commit member(s) over <p> tracked path(s), every path staged
host: <os> <arch>, <n> logical CPU(s)
```

*The host line comes from the build's target constants and the available parallelism, with no spawn. A member that goes red ends the run at exit 1 with the hook's own failure report and no figure, because a red battery measures a failure path rather than a commit. The arm exits 2 on any operand, outside a repository, or on an unreadable registry. It is an `Arm::Run`, and its knob roster is the hook arm's. The figure is the hook's work: it excludes git's own and the hook's process start.*

***Named caller and transition**: an adopter asking what a commit costs in their own repository; and the release step that re-measures the install page's figure (RELEASING.md step 4), whose transition is the pin-moving commit.*

gate-sdk/SPEC.md §The non-gate arm, the **Fixed by the subject** bullet, gains: *`--measure-commit` spawns `git` for the tracked-path list and whatever the selected members spawn, since running them is what it measures.* Implementation: in `native/src/emit/git_hook.rs`, beside `run_over`, a `measure` entry that times `run_over(&Hook::PreCommit, &tracked)`, plus its `ARMS` row. The median of three is the middle value.

### (2) The install page's figure and its limits {mechanical}

**Not yet applied.** docs/install.md gains a subsection after the paragraph ending *see the [contributing guide](…).*, before *### Writing your own shell gates*:

*### What a commit costs*

*The pre-commit hook runs the gates your profile registers whose triggers match the staged files, one after another, and stops at the first red. The figure is the worst case on a fresh install: every file staged, in a repository the one-line install has just initialized with `full`, the profile that contains every other.*

*`<!-- commit-cost:begin -->`*

*On `<os> <arch>` with `<n>` logical CPUs, a commit staging every file ran `<m>` pre-commit gates in **`<N>` ms**, the median of three, measured at v`<X.Y.Z>`.*

*`<!-- commit-cost:end -->`*

*To measure your own repository, run the gate binary with `--measure-commit`. It stages nothing and times the same run over every tracked file. Two costs sit outside the figure. The hook starts through `sh`, which on Windows is Git for Windows' bundled sh, emulated on Arm, and its start cost is unmeasured. A gate you register yourself adds its own time.*

Build fills the angle-bracketed fields from the arm's two output lines. The pin is 0.30.0 and that release predates the arm, so the landing measurement runs the tree's own binary over a v0.30.0 `full` install, and the block says `measured at v0.30.0`. Every later measurement runs the pinned release's binary (delta 4).

### (3) Invariant D of the hosted install pin {design-bearing}

**Not yet applied.** installer/SPEC.md §The hosted install pin: the opening paragraph's *`check-install-pin` (this repo's `scripts/`) holds three invariants* becomes *four invariants*, and the section gains, after Invariant C:

*- **Invariant D — the commit-cost figure is the pinned release's.** docs/install.md carries exactly one `<!-- commit-cost:begin -->` … `<!-- commit-cost:end -->` block, holding exactly one `measured at v<X.Y.Z>` token, whose version equals the pin. So the commit that moves the pin carries a new measurement (RELEASING.md step 4). **Honest limit:** D reads the version and never the host or the figure, so a figure copied forward under a new version passes.*

The fail-closed paragraph gains *a missing or duplicated commit-cost block, and a block carrying no `measured at` token or more than one*. D is hermetic, since the pin and the page are both in the tree, so no dormancy applies. Implementation: `native/src/gates/install_pin.rs` over the `md_text` it already reads. The descriptor's `# spec:` line in `scripts/check-install-pin.gate` names (D) beside (A), (B) and (C). `scripts/gate-tests/check-install-pin/good/install.md` gains a block at the fixture's pin. The `bad/` install.md gains a block naming another version, and `bad/expect.txt` gains the D line. The fail-closed cases are a unit test in `install_pin.rs` beside `a_missing_duplicated_or_malformed_pin_fails_closed`: a missing block, a duplicated block, and a block with no `measured at` token or two, each exiting 2.

### (4) The release step {mechanical}

**Not yet applied.** RELEASING.md step 4, the paragraph opening *Tag with `git tag -a vX.Y.Z`*. After the sentence ending *which `check-install-pin` demands from the first commit after the tag*, add:

*That commit also carries the new commit-cost figure, so make it once step 5's publish watch is green. In an empty scratch repository, run the one line with `CHECKWRIGHT_VERSION=X.Y.Z` set, since the hosted pin still names the previous release until this commit moves it, and `init --profile full`; then run the installed gate binary with `--measure-commit`, and write its two lines into docs/install.md's `commit-cost` block as `measured at vX.Y.Z` (invariant D).*

### (5) The unread baseline is retired {mechanical}

**Not yet applied.** `git rm .workflow/gate-timing-baseline.txt`. Its one declaration is its own `# contract:` header, which leaves with it. No gate lists the workflow directory's members by name: `check-workflow-tiering` reads tracking and `.gitignore`. The per-commit-cost-figure and gate-timing-baseline-comparability entries' Done moves drop their bodies, which are the queue's last mentions.

### (6) The site-architecture row {mechanical}

**Not yet applied.** docs/site-architecture.md, after the prerequisites-block row: *- **The commit-cost block** — `docs/install.md`'s Requirements section carries a fourth marker block (`<!-- commit-cost:begin -->`), one sentence written from `--measure-commit`'s output at each release (RELEASING.md step 4) and held to the pin by `check-install-pin` invariant D ([installer/SPEC.md §The hosted install pin](installer/SPEC.md#the-hosted-install-pin)). It is measured rather than derived: the figure is one host's, so no gate re-runs it.*

### (7) The fixtures {mechanical}

**Not yet applied.** The median rule becomes a pure function in `git_hook.rs` with a unit test. gate-sdk/smoke/install.sh runs `--measure-commit` once in its green scratch consumer and asserts exit 0, both lines' shapes, and a `git status --porcelain` identical before and after. It also asserts that an operand exits 2. gate-sdk/SPEC.md §measure-commit closes with *Its cases are the module's unit test and the kit smoke's green-consumer run.* The invariant-D fixtures are delta 3's.

## Producers and consumers

Probes: `git grep -n 'gate-timing-baseline'` over the tracked tree; `grep -n 'dispatch_one\|fn report\|fn run_over' native/src/emit/git_hook.rs`, which shows the serial, fail-fast loop; `grep -n 'install-md\|md_text' native/src/gates/install_pin.rs`; `ls scripts/gate-tests/check-install-pin/*/`; `grep -n "pin=" docs/install.sh` (`0.30.0`); RELEASING.md step 4 read for the drain commit's contents.

- **The measurement** (delta 1). Producer: `--measure-commit`, run by an adopter or by the release step (delta 4). Consumers: the running session, and the page block the release step writes. Each output field has a reader. The median is the page's figure, and the three samples let the reader judge spread. The member and path counts are the page's `<m>` and scope, and the host line is the page's host clause.
- **The commit-cost block** (delta 2). Producer: the release step's session, from the arm's output, in the pin-moving commit. Consumers: the adopter reading the page, and `check-install-pin` invariant D, which reads only the `measured at` token.
- **Invariant D** (delta 3). Its red condition: a block version other than the pin, and exit 2 on a missing or duplicated block or token. The pin moves only in the post-tag commit, so D reds exactly when a release has not been re-measured. The gate is `precommit` tier and couples to `docs/install.md` already, so an edit to the block triggers it.
- **The retired baseline** (delta 5). The probe finds no reader in code or scripts. Its other mentions are the two queue entries and one survey-record finding, which names the entry slug rather than the file.
- **Roster-holding readers of the new names.** The `ARMS` table and the arm-knob test (delta 1). The FENCE_SAFE derivation, which the arm stays outside as a non-`--emit` `Arm::Run`. docs/site-architecture.md's block roster (delta 6).

## Existing sections updated

Roster probe: the probes above, plus `git grep -n 'prerequisites:begin'` for the block roster.

- `gate-sdk/SPEC.md` — a new §measure-commit; §The non-gate arm, the **Fixed by the subject** bullet (deltas 1 and 7).
- `native/src/emit/git_hook.rs`, `native/src/emit/mod.rs` (deltas 1 and 7).
- `docs/install.md` (delta 2).
- `installer/SPEC.md` §The hosted install pin (delta 3).
- `native/src/gates/install_pin.rs`, `scripts/check-install-pin.gate`, `scripts/gate-tests/check-install-pin/` (delta 3).
- `RELEASING.md` step 4 (delta 4).
- `.workflow/gate-timing-baseline.txt` (delta 5).
- `TASK-QUEUE.md` — the per-commit-cost-figure and gate-timing-baseline-comparability entries, whose Done moves drop their bodies (delta 5).
- `docs/site-architecture.md` (delta 6).
- `gate-sdk/smoke/install.sh` (delta 7).
- `.workflow/surface-ceiling.txt` — the grown `docs/install.md`, `docs/site-architecture.md`, `gate-sdk/SPEC.md` and `installer/SPEC.md` rows re-stamped with `bash gate-sdk/bin/run-gates.sh --emit always-loaded --ceiling`, which `check-surface-ratchet` demands with the growth (all deltas).
- `docs/gate-sdk/SPEC.md` and `docs/installer/SPEC.md`, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).

## Retired spellings

- `gate-timing-baseline.txt` — the workflow-directory file delta 5 removes (delta 5).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` above holds, checked by `check-amendment-retired-spelling`.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Measured on this host** — the landing figure is taken as delta 2 states, and `check-install-pin` is green over it. Both entries move to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`).
