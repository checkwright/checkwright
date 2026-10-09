# SPEC amendment: step-order

§check-crate-arms states that every workflow job calling the toolchain takes the retrying composite action ahead of its first such call, and nothing holds the statement: the class was met twice, each time by review and a grep. This amendment adds the holder.

## What changes

### (1) `check-action-step-order`: a job spelling a listed program takes the listed action first

A new section `### check-action-step-order` in gate-sdk/SPEC.md, after §check-action-job-ref, stating the member below {design-bearing}.

- **Invariant.** In every Actions-shaped YAML file carrying a top-level `jobs:` key, a job whose `run:` bodies spell a listed program has a step on the listed action ordered before the job's first such spelling.
- **The two values are the declaring tree's.** `GATE_LOCAL_STEP_ORDER_ACTION` (scalar) is the action, compared with a step's `uses:` ref up to any `@`. `GATE_LOCAL_STEP_ORDER_PROGRAMS` (indexed) is the program set. Both are declared on the member's descriptor (§The declaration cohort), so the crate carries neither an action path nor a program name. Either one empty is a clean skip whose clean line names the empty knob, a different sentence from a nothing-found clean.
- **A spelling is a whole word, anywhere on a body line.** A listed program counts wherever it stands as a whole word on a line of a `run:` body, under the word class §check-action-gh-repo's detector uses and applied on both sides of the token. A line whose first non-blank character is `#` is not read. Command position is deliberately not required: that roster misses a call behind `if`, `while`, `exec`, an environment prefix or a wrapper, each of which still fetches. A word in an argument or a string therefore arms the job, the over-detecting direction, and the body's shell is not consulted, since the fetch happens under any of them.
- **The order is by line.** The job's first step on the action satisfies it when its line precedes the job's first spelling. A step on the action after that spelling, in another job, or commented out, satisfies nothing.
- **Scan set and subject are §check-action-gh-repo's.** The derived walk with the shared prune set and the optional scan-root positional; a `runs:`-shaped composite action is skipped and counted, since it has no job and the action being ordered is itself one. A job spelling no listed program is inert and counted. A reusable-workflow call has no `steps:` and is inert.
- **Red** (exit 1): one finding per job, `<file>:<line>: job <id> spells <program> with no earlier step on <action>`, the line being the first spelling's. The `help:` line names the remedy, a step on the configured action ahead of the job's first listed program. **Clean** reports the armed jobs, the inert jobs, the Actions-shaped files and the composite actions skipped. **Exit 2** on an unreadable walked file and a scan root that is not a directory.
- **No valve.** Every finding is discharged by one step, the ground §check-action-run-shell gives for minting none. The cost is stated: a job that only mentions a listed word pays one step on the action or rewords the line.
- **Honest limits.** A call reached through a script a step runs is outside the read, since the member reads workflow text and follows no path. So is a call inside a composite action or a called workflow, each held where a job of its own would be. An `if:`-conditioned step on the action counts as taken. The member holds order, never that the action fetches or retries.
- **It is the declaring tree's own gate and does not ship.** The sibling `check-action-*` members are kit mechanism because a kit ships a workflow template carrying their subject. No kit template calls a toolchain, and which action precedes which program is one tree's practice, so the rule is generic and its two values and its registration are the consumer's. Tier `precommit`; the `# graph:` couples the workflow files, `dir=one valve=none`. Born native, declared by `scripts/check-action-step-order.gate`, rule in `native/src/gates/action_step_order.rs`.
- **The pair** is its oracle. `good/` holds a job with the action ahead of its first spelling, a job spelling nothing, a composite action spelling a program, a spelling on a comment line, and a listed program as part of a longer word. `bad/` holds a job with no step on the action, one whose step follows the spelling, one whose only step on the action sits in another job, and a spelling behind `if`. Each case declares the two knobs on its own descriptor. A bespoke test holds the two empty-knob cleans, which a one-verdict case cannot.

### (2) The job-partitioned walk emits what the member reads

`native/src/actions.rs` gains the two events delta 1 consumes, a step's `uses:` ref with its line and a caller-listed word's spelling with its line, and §check-action-gh-repo's paragraph on the walk names this member as its third consumer {design-bearing}. The widening leaves `check-action-gh-repo`'s and `check-action-permissions`' findings, counts, clean lines and exit codes byte-identical, the condition §check-action-permissions set on its own widening. The word list is the caller's argument, as the valve marker's spelling is, and a caller passing none receives no such event.

### (3) §check-crate-arms names its holder

In gate-sdk/SPEC.md §check-crate-arms, the sentence stating that every job that calls the toolchain takes the action as a step ahead of its first such call gains its holder, and the section's Honest limit gains the script-reached call {mechanical}. **Not yet applied.** Replacement for the sentence:

> It has one spelling, the repo-local composite action `.github/actions/toolchain`, which retries a failed call within a bound, and every job that calls the toolchain takes it as a step ahead of its first such call, which `check-action-step-order` holds over this tree's action and program set (§check-action-step-order).

Addition to the Honest limit paragraph, since no existing sentence there carries it:

> A toolchain call a step reaches through a script is outside that member's read, so a job whose first call sits in a script still owes the step by review.

### (4) The member is declared, bound and registered in this tree

`scripts/check-action-step-order.gate` carries the manifest, the `# spec:` pointer and the two `# knob:` declarations, `scripts/gates.list` registers the member beside its siblings, and the registry row declares both knobs and the walk root {mechanical}. This tree binds the action to `./.github/actions/toolchain` and the programs to `cargo`, `rustc`, `rustup`, `rustdoc`, `rustfmt` and `clippy-driver`.

## Producers and consumers

- **The verdict.** Producer: the member, run by the pre-commit hook on a staged workflow file, by the full battery and by CI. Consumer: the committing session, through the output contract; the `--run-gate-tests` arm through the pair. The enabling configuration is the descriptor's two knob lines, which this tree sets, so the member is live here and not only in its fixtures.
- **The two knobs.** Producer: the descriptor's `# knob:` lines, the only layer a `GATE_LOCAL_` name has besides the environment. Reader: the member alone, each on the path that walks. No knob file names either, since `<gates-dir>/*.knobs` refuses the prefix.
- **The two walk events.** Producer: `actions::walk_file`. Reader: delta 1's member. The two existing consumers match on the events they already read and ignore the rest.
- **Roster-holding readers of the minted gate name**, by probe `git grep -l check-npm-publish-spec`, a consumer-declared sibling, over the tracked tree: `scripts/gates.list` and the crate registry (delta 4), and the generated `docs/check-graph.html` and `docs/enforcement.md`, regenerated by their own freshness gates' commands. `check-readme-roster` and `check-install-disposition` sweep kit roots only and name no consumer-declared member. In an adopter's tree `check-gate-substrate-parity` assertion I scopes a consumer-declared member out, so no adopter registers or declares it.
- **Every member's satisfying value (point 6).** The obliged corpus is this tree's workflow jobs. Probe: an awk pass over `.github/workflows/*.yml` printing, per job, the line of its first step on the action and the line of its first whole-word spelling of delta 4's six programs. Nine jobs spell one, all in `gates.yml`, and each has the step on an earlier line: `gates`, `install-smoke-pwsh-windows`, `native-artifacts-roster`, `native-artifacts`, `crate-tests-windows`, `crate-tests-unix`, `install-smoke-sh-macos`, `install-smoke-sh-macos-intel` and `install-smoke-sh-linux-arm64`. `build` and `pack` in `publish.yml` carry the step and spell none, their calls sitting behind a script, the limit delta 1 states. The remaining seven jobs spell none and are inert. So the member lands green with no workflow edit. *Inferred, not run:* the probe's word boundary approximates the walk's, so the member's own first run is the oracle.

## Existing sections updated

- `gate-sdk/SPEC.md` §check-action-job-ref — the new section follows it (delta 1).
- `gate-sdk/SPEC.md` §check-action-gh-repo — the walk paragraph names the third consumer (delta 2).
- `gate-sdk/SPEC.md` §check-crate-arms — the holder and the limit (delta 3).
- `native/src/actions.rs`, `native/src/gates/action_gh_repo.rs`, `native/src/gates/action_permissions.rs` — the widened event stream and its two unchanged readers (delta 2).
- `native/src/gates/mod.rs` — the registry row (delta 4).
- `scripts/gates.list`, `scripts/check-action-step-order.gate`, `scripts/gate-tests/check-action-step-order/` — registration, declaration and the pair (deltas 1 and 4).
- `docs/gate-sdk/SPEC.md`, `docs/check-graph.html`, `docs/enforcement.md` — generated, stale once any of these lands (all deltas).

## Retired spellings

- None — every delta adds a name or a sentence and none removes or renames one.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Siblings unchanged** — `check-action-gh-repo`'s and `check-action-permissions`' fixture suites pass with their `expect.txt` files untouched.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration.
- [ ] **Entry moved** — `toolchain-action-order-ungated` moves to Done in the merge commit, which lands before the drain stage.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
