# SPEC amendment: workflow-decoupling

Pairs `verify-workflow-decoupling`. The entry's operator direction (2026-10-24, lead-relayed, not a ruling) settles its open question: the gate binary is substrate both halves share. On that basis this amendment states the decoupling where an adopter reads it. It also mints the `workflow` profile the weighing produced: `starter` plus the smallest set of workflow kits on which `prove` holds. That outcome is the lead's decision of 2026-10-24 on this stage's escalation. `personal-adoption`'s amendment, `SPEC-personal-overlay.md`, is this statement's first consumer: the halves part on tracking, and that amendment builds the verification half as an untracked overlay.

**What the tree does today**, read at spec:

- The fit page refutes the decoupling outright. docs/choosing.md §What you install says "The workflow does not run without the checks, since every profile carrying lifecycle-kit carries gate-sdk", and §When it does not fit ends "A team wanting the lifecycle workflow without the gates finds no profile that ships it". The entry's premise that the front door "neither claims nor refutes" it is stale.
- docs/kits.md says "Read in this order: each kit assumes the machinery of the ones above it", and docs/index.md §The kits repeats the clause. The probe below falsifies it as a dependency claim: every kit installs beside gate-sdk alone.
- Each of the ten kits other than gate-sdk, installed as `init --profile starter --with-kit <kit>` from this tree's pack into a fresh repository, exits 0 with a green battery. Run at spec over `ls -d */checks | grep -v gate-sdk`, the payload's kit roots.
- `init --profile delegation --without-gate check-pipe-membership --without-gate check-portability-floor` exits 0 with a green 19-gate battery. Those two are gate-sdk's starting gates that read the adopter's own shell. So a selection already takes the workflow without gate-sdk's tree-reading checks.
- The stage arm runs its gates off the vendored kits, not the registry. `preflight_gate` in `native/src/emit/enter_stage.rs` resolves its gate from each kit root's `checks/`, never from `gates.list`.
- `prove` holds on `{gate-sdk, lifecycle-kit, evidence-kit}` (16 registered gates) and on that set plus queue-kit. It is not held on `starter`, on `{gate-sdk, lifecycle-kit, queue-kit}` or on `{gate-sdk, queue-kit, evidence-kit}`. Probed at spec through the same pack.

## What changes

### (1) §Profiles states the two halves and their substrate {design-bearing}

`installer/SPEC.md` §Profiles gains a paragraph after the `full` bullet and before **The payload's reserved sibling is not a kit.** **Not yet applied.**

> **The profiles split two halves that run on one substrate.** The gate binary gate-sdk ships is that substrate: the runner, the registry, the git hooks, and the arms the workflow calls. `--enter-stage` resolves the gates it runs from the vendored kits' `checks/`, whatever `gates.list` holds. So every profile carries gate-sdk, and a `--without-kit gate-sdk` is refused (§Selecting kits and gates). **Verification** is the registered battery over the project's tracked content. It ships without the workflow in `starter` and `prose` and under every payload recipe. **The workflow** is the stage machine and the process record it commits: stage stamps, the task queue, recorded test runs. It ships without canon-kit and site-kit in `workflow` and `delegation`. Both carry `starter` and so its gates, gate-sdk's own checks over the tree among them, and a selection drops any of those an adopter does not want. **The halves part on tracking.** Verification reads what the project tracks, wherever the kits sit, so it runs from an untracked install too. The workflow commits its own record, so it needs a tracked one.

The last two sentences depend on whether `personal-adoption` lands in the same build batch. If it does, the sentence "so it runs from an untracked install too" ends ", the personal install (§The personal install)" instead, and that amendment's delta 6 adds the section it cites. If it does not, the sentences land as quoted, and `personal-adoption`'s merge adds the citation.

### (2) The `workflow` profile {design-bearing} {user-facing: the lead's decision of 2026-10-24 on this stage's escalation, taken under the entry's operator direction of that date, which leaves the weighing to spec}

A fifth profile, `workflow`, is minted. It is `starter` plus the smallest set of workflow kits on which `prove` holds: lifecycle-kit and evidence-kit. The probe above shows that set is the minimum. It fits the lattice: it contains `starter`, sits inside `delegation`, and is an alternative to `prose`. It is not verification-free and is never described as such. `starter` is the lattice's bottom, and no profile drops a gate-sdk `zero-config` member (§What init seeds), so `workflow` carries gate-sdk's starting gates, its checks over the adopter's own shell among them. Only a selection removes those, per install. §The consumer smoke derives its per-profile pass and its lattice assertions from `profiles.list`, so the row adds a pass and no code. No crate source names a profile row: `grep -rn '"delegation"' native/src` finds none. Each edit is **Not yet applied.**

- **`installer/profiles.list`**, a row group between `starter`'s and `delegation`'s:

  ```
  # workflow — starter, the substrate and starting gates every profile carries,
  # plus the smallest set of workflow kits on which `prove` holds: lifecycle-kit,
  # the stage machine whose stamps make a done claim, and evidence-kit, whose
  # manifest gate catches one made with no recorded test run. starter's gates
  # come with it, gate-sdk's checks over the adopter's own shell among them,
  # since no profile sits below starter; a selection drops them per install.
  # queue-kit is left out: the claim is caught without it, the queue skeleton
  # init seeds for lifecycle-kit carries the sections the claim is staged in,
  # and delegation adds the kit.
  workflow	gate-sdk
  workflow	lifecycle-kit
  workflow	evidence-kit
  ```

- **`installer/SPEC.md` §Profiles**:
  - The opening sentence's list becomes `` `starter`, `workflow`, `delegation`, `prose` or `full` ``. The next sentence becomes: "They are not rungs on one ladder, since `prose` contains neither `workflow` nor `delegation` and answers a different question about what your repository is."
  - A bullet after `starter`'s: "- **`workflow`** is `starter` plus the smallest set of workflow kits on which `prove` holds: lifecycle-kit's stage machine and evidence-kit's recorded test runs. `starter`'s gates come with it, gate-sdk's own checks over your tree among them."
  - `delegation`'s bullet opens "**`delegation`** adds to `workflow` every kit whose subject is the agent session itself:", and the rest of the bullet is unchanged.
  - The rosters sentence opens "`starter`, `workflow`, `delegation` and `prose` are rosters in `profiles.list`".
- **`installer/SPEC.md` §The consumer smoke**, **What it costs to run.**: "That covers four profiles × install + battery + value arm" becomes "That covers each profile's install, battery and value arm". The re-measure clause stays.
- **`installer/README.md` §Choosing a profile**:
  - The opening line's list gains `` `workflow` `` after `` `starter` ``, and "Not four rungs on one ladder" becomes "Not rungs on one ladder".
  - A bullet after `starter`'s: "- **`workflow`** — `starter` plus the stage machine and the recorded test runs, the least on which `prove` catches a done claim; it keeps `starter`'s gates."
  - The rosters sentence opens "`starter`, `workflow`, `delegation` and `prose` are rosters".
- **`docs/install.md` §Choosing a profile**: a bullet after `starter`'s, "- `workflow` — adds the stage machine and the recorded test runs to it, the least on which `prove` catches a done claim;". The page-length dependence is `SPEC-personal-overlay.md` delta 8's, measured with this bullet in place.

docs/kits.md names no profile (delta 4) and takes no edit for this one.

### (3) The fit page states the decoupling {mechanical} {user-facing: the entry's operator direction of 2026-10-24, "the binary is substrate both halves share, and spec authors the statement"}

`docs/choosing.md`. **Not yet applied.**

- §When it does not fit: its last sentence, "A team wanting the lifecycle workflow without the gates finds no profile that ships it, as the next section shows.", is deleted.
- §What you install: after "The `starter` profile is gate-sdk alone.", a sentence: "The `workflow` profile is `starter` plus lifecycle-kit and evidence-kit, the least on which a done claim is caught."
- §What you install: the sentences from "The checks run without the workflow" to the end of the paragraph are replaced:

  > The checks and the workflow install without each other, on one base. gate-sdk's binary is that base: it runs the checks, places the git hooks and carries the commands the workflow's stages run, so every profile carries gate-sdk. The checks run without the workflow: `starter` and `prose` carry no lifecycle-kit, and the companion recipes gate another toolkit's workflow. The workflow runs without the document and site checks, since `workflow` and `delegation` carry neither canon-kit nor site-kit. Both build on `starter`, so gate-sdk's own starting checks come with them, and `--without-gate` drops any of those. The two part on what they write. The checks read what your repository tracks, and the workflow commits its own record: the stage stamps, the task queue and the recorded test runs. Under `delegation`, lifecycle-kit's checks enforce nothing until the files each one reads exist.

### (4) The kit roster's reading order stops claiming a dependency chain {mechanical} {user-facing: as delta 3}

Each **Not yet applied.**

- `docs/kits.md` §The kits: the line "Read in this order: each kit assumes the machinery of the ones above it." becomes:

  > Start with gate-sdk. Its binary is the runner, the registry, the git hooks and the commands the other kits call, so every install carries it. The rest follow in reading order, not dependency order: each installs beside gate-sdk alone.

- `docs/index.md` §The kits: the paragraph becomes:

  > Each kit has its own page under [Kit Reference](kits.md) in the navigation, which glosses every kit in one line. gate-sdk is the base every other kit runs on, and beyond it the checks and the workflow install without each other ([What you install](choosing.md#what-you-install)).

## Producers and consumers

- **The statement** (deltas 1, 3 and 4) is prose. Its producer is this amendment's merge, and its readers are an adopter choosing a profile and an evaluating agent reading `docs/llms-full.txt`. Its claims rest on the probes above, which build re-runs only if the payload's kit set or the profile rows move before the merge.
- **Roster-holding readers of the surfaces these deltas touch**, each with its red condition:
  - `check-md-refs` reds a link resolving to no file or heading. Delta 4's new link targets `choosing.md#what-you-install`, a heading the page carries today.
  - `check-docs-page-repeat` reds a sentence of eight words or more, or a link target, that appears twice on one page. Delta 3's sentences are new to docs/choosing.md. Delta 4 drops the docs/index.md sentence that repeated docs/kits.md's clause across pages, which the gate does not read anyway.
  - `check-docs-page-length` reds a page above 1500 words. docs/choosing.md is at 781 by `wc -w`, docs/index.md at 733 and docs/kits.md at 160.
  - `check-front-door-verbs` reads docs/index.md, a front-door page. Delta 4's paragraph carries no route, verb or arm span. Delta 1's `--enter-stage` is in installer/SPEC.md, which is no front-door page.
  - `check-llms-fresh` reds `docs/llms-full.txt` until it is regenerated: it carries docs/index.md, docs/choosing.md and docs/kits.md in full.
  - `check-docs-mirror-fresh` reds `docs/installer/SPEC.md` until it is regenerated (delta 1).
- **The `workflow` row** (delta 2). Producer: `profiles.list`, read by the profile module behind the invoke. Consumers, each with its red condition:
  - `init --profile workflow`, and `init --help`'s profile list, derived from the rows;
  - the consumer smoke's lattice assertions. They red when a named kit does not resolve, when the bounds are not unique, or when moving up a comparable pair drops a gate. `workflow` ⊂ `delegation` ⊂ `full` and `starter` ⊂ `workflow` hold by the rows, and the registry is the union derivation over the kit set (§What init seeds), so containment holds for the registries too. Its per-profile pass reds as every profile's does. Its queue post-condition expects a queue file, which the probe's install carried;
  - `check-front-door-verbs`' profile set: the first column of `profiles.list`, read at HEAD and at the pinned tag (§The front door's verbs). No delta writes `--profile workflow` on a front-door page, so nothing new is advertised. A later page that does is admitted while a close's disposition is a release;
  - `SPEC-personal-overlay.md` delta 1's process-record refusal, which refuses `workflow`, since both of its added kits commit a record. That refusal's derived `help:` still names `starter` and `prose`.

## Existing sections updated

- `installer/SPEC.md` §Profiles (deltas 1 and 2), and §The consumer smoke's **What it costs to run.** (delta 2).
- `installer/profiles.list`, `installer/README.md` §Choosing a profile and `docs/install.md` §Choosing a profile (delta 2).
- `docs/choosing.md` §When it does not fit and §What you install (delta 3).
- `docs/kits.md` §The kits and `docs/index.md` §The kits (delta 4).
- `docs/llms-full.txt`, regenerated with `bash gate-sdk/bin/run-gates.sh --emit llms --write` (deltas 2, 3 and 4).
- `docs/installer/SPEC.md`, the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`, `docs/installer/README.md` among it (deltas 1 and 2).

The surfaces were found by `git grep -n -i "machinery of the ones\|without the checks\|without the gates\|workflow without\|without the workflow\|decoupl"` over the tracked tree, with the queue, the survey record and the generated `docs/llms-full.txt` set aside.

## Retired spellings

- None — no delta retires a name. Delta 4 rewrites a clause, and delta 3 deletes a sentence that no other surface quotes.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The local oracle** — the battery green, with the two regenerated projections fresh.
- [ ] **The Done move** lands before the drain stage, with the merge.
