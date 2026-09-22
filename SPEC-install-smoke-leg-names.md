# SPEC amendment: install-smoke-leg-names

The `install-smoke` legs in `.github/workflows/gates.yml` use one suffix slot for two axes. Four legs name a platform (`install-smoke-windows`, `install-smoke-macos`, `install-smoke-macos-intel`, `install-smoke-linux-arm64`), the baseline Linux leg names nothing, and `install-smoke-powershell` names a bootstrap while running on Windows. So a reader counting platforms off the names counts wrong, and since the second Linux leg joined the unsuffixed baseline is ambiguous too. `.github/workflows/gates.yml` and `installer/SPEC.md` §The install boundary each carry a paragraph whose job is to correct that misreading. Whether to rename is already ruled, since the aim is intuitive design; only the scheme is open.

**The ruling: bootstrap first, then platform, on every leg: `install-smoke-<bootstrap>-<platform>`.** The bootstrap segment is `sh` or `pwsh`, the names the two bootstraps already carry (`bin/checkwright.sh`, and `checkwright-pwsh` on the package's `bin` roster). The platform segment is the one each leg already carries, with the Linux baseline taking `linux`. The platform-preserving scheme was weighed and refused. It renames fewer legs, but it leaves the bootstrap implicit on every leg of a platform with one bootstrap leg, so the axis installer/SPEC.md says to count by stays the one the names hide. Bootstrap first puts that axis in the first segment. The rename's radius is the same set of files under either scheme, because each of those files names several legs already.

| today | renamed |
| --- | --- |
| `install-smoke` | `install-smoke-sh-linux` |
| `install-smoke-linux-arm64` | `install-smoke-sh-linux-arm64` |
| `install-smoke-macos` | `install-smoke-sh-macos` |
| `install-smoke-macos-intel` | `install-smoke-sh-macos-intel` |
| `install-smoke-windows` | `install-smoke-sh-windows` |
| `install-smoke-powershell` | `install-smoke-pwsh-windows` |

`pwsh` rather than `powershell` has a second reason: `install-smoke-powershell` is a substring of `install-smoke-powershell-windows`, so the retired-spelling gate could not tell a survivor from its successor.

## What changes

### (1) Rename the six job keys and every mention of them {mechanical}

**Not yet applied.** Rename each job key in `.github/workflows/gates.yml` per the table, and every tracked mention of a renamed leg, backticked or not, outside the history surfaces. The roster under §Existing sections updated names each surface.

`docs/install.md` §Requirements' platform block names `install-smoke-linux-arm64` in the `aarch64-unknown-linux-gnu` line's held precondition. The act there depends on the sibling entry `install-path-developer-first`, whose own ruling ("Where the contributor facts go", SPEC-install-per-os.md) keeps that block on `docs/install.md` and only repositions it, to `## Requirements` moved to the page's end — it never carries the block off the file:

- **If `install-path-developer-first` lands in the same build batch, or earlier,** rename the leg at the block's repositioned location, under the page's `## Requirements`.
- **If it lands in a later batch, or not at all this iteration,** rename the leg at the block's current location, mid-page.

Both branches edit `docs/install.md`; the difference is only where in the page the block sits when the rename runs.

Rename the leg names in live queue entries too, active and deferred, in the same commit. The queue is excluded from the retired-spelling gate's corpus, so no gate catches a missed queue mention. **A queue sentence naming the leg family as a whole** — as `install-smoke-leg-names-mix-two-axes`'s own opening paragraph does, with "the `install-smoke` legs … spend one suffix slot on two different axes" — is unbackticked in place, per delta 4's convention, rather than rewritten to one leg's new name: the sentence describes all six legs' pre-rename shape, and substituting a single renamed leg's spelling would misstate it. A queue mention that names one leg specifically is renamed to that leg's new spelling as usual. That commit applies the rule in `install-smoke-powershell-demo-runs-before-bash-strip`'s body as it stands then, whether or not that debt entry has landed.

### (2) Retire the two correcting paragraphs {mechanical}

**Not yet applied.** Now that each name carries its bootstrap, the paragraphs that walk back a platform count shrink to the rule.

In `installer/SPEC.md` §The install boundary, rewrite the paragraph beginning "**The two bootstraps are hand-kept, and parity is held by running, not by generation.**" as:

> **The two bootstraps are hand-kept, and parity is held by running, not by generation.** Each half is authored in its own language against the five steps above, and the oracle that holds them equal is a per-bootstrap install-smoke leg. A leg's name is `install-smoke-<bootstrap>-<platform>`, so the bootstrap a leg drives is its first segment and the count that matters is read off the names: two bootstraps, `sh` and `pwsh`. The platform segments move with the platform declaration and are not counted here. **No other leg substitutes for the `pwsh` one**, because every other leg drives the `sh` bootstrap.

In `.github/workflows/gates.yml`, delete the comment block above the PowerShell leg from "The second BOOTSTRAP's own install-smoke leg." through "No other leg substitutes for this one." and replace it with one line:

> `  # The pwsh bootstrap's leg (installer/SPEC.md §The install boundary). No other leg substitutes for it.`

Then delete the paragraph above the arm64 leg beginning "**The name is minted under the convention already in this file and that is a deliberate NON-decision.**", which records this entry's open question.

### (3) `check-action-job-ref`: a backticked leg name names a job {design-bearing}

**Not yet applied.** A new gate-sdk gate, born native, registered in `scripts/gates.list`. Add to gate-sdk/SPEC.md after §check-action-permissions:

> ### check-action-job-ref
>
> Invariant: every backticked reference to a CI job, in the governed prose and the workflow files themselves, names a job some workflow defines. A job key is renamed in one file and cited in many, and a citation to a key that is gone reds nowhere else.
>
> **The reference.** A single-backtick code span whose whole content matches one of `GATE_SDK_JOB_REF_PATTERNS`, anchored at both ends. The knob is an array of EREs, default empty, and an empty set is a clean skip whose clean line names the knob. Which job names a tree cites is its own vocabulary, so no pattern ships as a kit literal (the provenance seam). A pattern selects what counts as a job reference. A span outside every pattern is not one, so a queue slug sharing a prefix with a job family is out of reach by construction.
>
> **The job set.** Every job key under a top-level `jobs:` key of each YAML file the walk finds, read by the job partition §check-action-run-shell specifies (`native/src/actions.rs`), not re-parsed.
>
> **Corpus.** A pruned walk from the scan root (the optional positional argument, default `.`) for `*.md`, `*.yml` and `*.yaml`, taking the shared prune set, so `gate-tests/` is out and the `bad/` fixture cannot red the whole-tree run. A workflow file is read twice: once for its job keys and once for its comments' references.
>
> **Red:** one finding per reference that names no job key, as `<file>:<line>: <reference> names no job in any walked workflow`. **Exit 2:** a pattern that does not compile, and an unreadable file.
>
> **Honest limits.** A leg described in free prose ("the Windows leg") is out of reach, and so is a reference outside every pattern, a misspelled bootstrap segment included. A reference to a job that was renamed away is found only while the old name still matches a pattern. The durable catch for a rename is the renaming amendment's retired-spelling block (canon-kit/SPEC.md §check-amendment-retired-spelling), which this gate complements and does not replace.
>
> Tier `precommit`, `# install: on-surface`. The `# graph:` manifest couples `*.md`, `*.yml` and `*.yaml`. The knob is a vocabulary rather than a walk filter, so it takes no `knob:` token.

Implementation: a module in `native/src/gates/`, a `gate-sdk/checks/check-action-job-ref.gate` descriptor, a row for `GATE_SDK_JOB_REF_PATTERNS` in `native/src/knobs/gate_sdk.rs` and in gate-sdk/SPEC.md §Layout and configuration's knob roster, and a `good/` + `bad/` fixture pair under `gate-sdk/gate-tests/check-action-job-ref/`. `good/` holds a workflow defining job `leg-a` and a doc citing `leg-a`, plus a slug sharing its prefix outside every pattern. `bad/` holds the same workflow and a doc citing `leg-b`. The release declaration surface gains a Tightened-gates bullet: `check-action-job-ref` — new, opt-in.

### (4) This repo binds the leg family {mechanical}

**Not yet applied.** In `scripts/gate-sdk-config.knobs`:

```
GATE_SDK_JOB_REF_PATTERNS[] = install-smoke
GATE_SDK_JOB_REF_PATTERNS[] = install-smoke-(sh|pwsh)-[a-z0-9-]+
```

The first pattern catches a bare `install-smoke` left behind, since no job carries that key once delta 1 lands. So a mention of the family as a whole is written unbackticked, as "the install-smoke legs". The second pattern keeps every queue slug in the family out of reach, because none of them opens with a bootstrap segment. Register `check-action-job-ref` in `scripts/gates.list`.

## Producers and consumers

- **The job keys** (delta 1). Produced by `.github/workflows/gates.yml`. Consumed by GitHub Actions as check-run names, and by the readers delta 1 renames. No required status check names them: this repo's branch-protection desired state carries none, and `OPS.local.md` names no leg (measured at the entry's 2026-09-11 drain).
- **`GATE_SDK_JOB_REF_PATTERNS`** (deltas 3 and 4). Produced by the consumer's `gate-sdk-config.knobs`, set in this repo, so the producer is live outside the fixtures. Consumed by `check-action-job-ref` alone, at its reference test.
- **The job set** (delta 3). Produced by `actions::walk_file`'s `Ev::Job` events, an existing interface, now with a third consumer.
- **Roster-holding readers of the new names.** `scripts/gates.list` and `check-graph`'s generated projections (the pre-commit hook and the coupling graph, regenerated with `--emit git-hooks --write`), the gate registry in `native/src/gates/mod.rs`, the knob table, `check-gate-fixture-coverage` through the fixture pair, `check-install-disposition` through the `# install:` line, and the gate-sdk README's `gate-roster` block. Each is an update target (delta 3).
- **Point 5.** No corpus narrows.
- **Point 6.** Delta 1 obliges every tracked mention of six names. The members are enumerated by the probe below, and each member's value is its renamed spelling from the table.

## Existing sections updated

Roster from `git grep -n -E "install-smoke" -- . ':!docs/posts'`, run 2026-09-22 and read line by line. Mentions of the leg family as a whole ("an install-smoke leg") need no rename and are not listed; those that name one leg are.

- `.github/workflows/gates.yml` — the six job keys, the comments naming them, and the two paragraphs (deltas 1 and 2).
- `installer/SPEC.md` — §init's follow-up paragraph, §The install boundary's parity paragraph, and §The consumer smoke's Windows passages (deltas 1 and 2).
- `gate-sdk/SPEC.md` — §gen-pre-commit's `install-smoke-powershell` sentence (delta 1); §check-action-job-ref and the knob roster (delta 3).
- `docs/site-architecture.md` — the install-platforms and remedy-block rows (delta 1).
- `docs/install.md` — the held precondition, under the sibling condition in delta 1.
- `installer/consumer-smoke/run-smoke.sh` — the two Windows-host lines naming `install-smoke-powershell` (delta 1).
- `TASK-QUEUE.md` — live entries naming a renamed leg (delta 1).
- `scripts/gate-sdk-config.knobs` and `scripts/gates.list` (deltas 3 and 4).
- `native/src/gates/`, `native/src/gates/mod.rs`, `native/src/knobs/gate_sdk.rs`, `gate-sdk/checks/`, `gate-sdk/gate-tests/check-action-job-ref/`, `gate-sdk/README.md` (delta 3).
- `.workflow/release-declarations.md` (delta 3).
- `docs/gate-sdk/SPEC.md`, the generated mirror, regenerated (all deltas).
- `docs/installer/SPEC.md`, the generated mirror, regenerated (all deltas).

Descriptive family mentions stay as they are: `guard-kit/SPEC.md`, `native/runners.list`, `native/targets.list`, `.github/workflows/publish.yml` and `scripts/gate-sdk-config.knobs`' existing comment.

## Retired spellings

- `install-smoke-windows` — renamed `install-smoke-sh-windows` (delta 1).
- `install-smoke-macos` — renamed `install-smoke-sh-macos`; also covers `install-smoke-macos-intel`, which contains it (delta 1).
- `install-smoke-linux-arm64` — renamed `install-smoke-sh-linux-arm64` (delta 1).
- `install-smoke-powershell` — renamed `install-smoke-pwsh-windows` (delta 1).
<!-- retired-spelling-exempt: a substring of every successor name, so a survivor scan cannot tell it from them; the bare-key pattern in delta 4 catches a backticked survivor instead -->
- `install-smoke` — the baseline leg, renamed `install-smoke-sh-linux` (delta 1).

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit's causal-completeness checklist holds for the renamed keys, the knob and the gate.
- [ ] **Instruction surfaces: instruction only.** The workflow comment keeps a pointer and the rule; the grounds sit in installer/SPEC.md.
- [ ] **Merged with no information lost.** installer/SPEC.md §The install boundary reads as one document, and gate-sdk/SPEC.md gains one gate section.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved.** `install-smoke-leg-names-mix-two-axes` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Every spelling above is reconciled by `check-amendment-retired-spelling`, and the queue's live entries by hand.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
- [ ] **Observed.** The entry carries `[observed-by: gates workflow]`: a job key is only proven by a run, so the Done move waits for a watched push showing every renamed leg green under its new name.
