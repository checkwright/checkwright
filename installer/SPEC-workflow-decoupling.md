# SPEC amendment: workflow-decoupling

Pairs `verify-workflow-decoupling`. The entry's operator direction (2026-10-24, lead-relayed, not a ruling) settles its open question: the gate binary is substrate both halves share. On that basis this amendment states the decoupling where an adopter reads it, and weighs a workflow-only profile. `personal-adoption`'s amendment, `SPEC-personal-overlay.md`, is this statement's first consumer: the halves part on tracking, and that amendment builds the verification half as an untracked overlay.

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

> **The profiles split two halves that run on one substrate.** The gate binary gate-sdk ships is that substrate: the runner, the registry, the git hooks, and the arms the workflow calls. `--enter-stage` resolves the gates it runs from the vendored kits' `checks/`, whatever `gates.list` holds. So every profile carries gate-sdk, and a `--without-kit gate-sdk` is refused (§Selecting kits and gates). **Verification** is the registered battery over the project's tracked content. It ships without the workflow in `starter` and `prose` and under every payload recipe. **The workflow** is the stage machine and the process record it commits: stage stamps, the task queue, recorded test runs. It ships without canon-kit and site-kit in `delegation`, and a selection drops any gate-sdk gate an adopter does not want. **The halves part on tracking.** Verification reads what the project tracks, wherever the kits sit, so it runs from an untracked install too. The workflow commits its own record, so it needs a tracked one.

The last two sentences depend on whether `personal-adoption` lands in the same build batch. If it does, the sentence "so it runs from an untracked install too" ends ", the personal install (§The personal install)" instead, and that amendment's delta 6 adds the section it cites. If it does not, the sentences land as quoted, and `personal-adoption`'s merge adds the citation.

### (2) The workflow-only profile, weighed {design-bearing}

The weighing is recorded here. Its outcome waits on the lead's answer to this stage's escalation, since minting a profile adds a name adopters type. **Not yet applied** in either form.

- **What a fourth profile would be.** `workflow`, rostered `gate-sdk`, `lifecycle-kit` and `evidence-kit`, on the criterion *the smallest set on which `prove` holds*. The probe above shows that set is the minimum. It fits the lattice: it contains `starter`, sits inside `delegation`, and is an alternative to `prose`. §The consumer smoke derives its per-profile pass from `profiles.list`, so a row adds a pass to the smoke and no code to it.
- **What it cannot be.** It cannot be *verification-free*. `starter` is the lattice's bottom, and no profile drops a gate-sdk `zero-config` member (§What init seeds), so the profile carries gate-sdk's starting gates. Only a selection removes those, and only per install.
- **If the answer mints it**, this delta becomes these edits:
  - a `profiles.list` row group with its criterion comment;
  - a `workflow` bullet in §Profiles, `installer/README.md` §Choosing a profile and docs/install.md §Choosing a profile;
  - delta 3's sentence naming `delegation` names `workflow` instead.

  The amendment is revised to carry them before build takes this unit.
- **If the answer declines it**, this paragraph lands in §Profiles after delta 1's:

  > **No workflow-only profile is rostered.** The lattice's bottom puts gate-sdk's starting gates in every profile, so no profile can be free of verification. `delegation` already ships the workflow, and a selection narrows it per install.

### (3) The fit page states the decoupling {mechanical} {user-facing: the entry's operator direction of 2026-10-24, "the binary is substrate both halves share, and spec authors the statement"}

`docs/choosing.md`. **Not yet applied.**

- §When it does not fit: its last sentence, "A team wanting the lifecycle workflow without the gates finds no profile that ships it, as the next section shows.", is deleted.
- §What you install: the sentences from "The checks run without the workflow" to the end of the paragraph are replaced:

  > The checks and the workflow install without each other, on one base. gate-sdk's binary is that base: it runs the checks, places the git hooks and carries the commands the workflow's stages run, so every profile carries gate-sdk. The checks run without the workflow: `starter` and `prose` carry no lifecycle-kit, and the companion recipes gate another toolkit's workflow. The workflow runs without the document and site checks, since `delegation` carries neither canon-kit nor site-kit, and `--without-gate` drops any of gate-sdk's own starting checks. The two part on what they write. The checks read what your repository tracks, and the workflow commits its own record: the stage stamps, the task queue and the recorded test runs. Under `delegation`, lifecycle-kit's checks enforce nothing until the files each one reads exist.

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
- **Delta 2, if minted**: the readers are `check-front-door-verbs`' profile set, the first column of `profiles.list` read at HEAD and at the pinned tag (installer/SPEC.md §The front door's verbs). A `--profile workflow` advertised on a front-door page is admitted while the close's release disposition is a release, so a mint makes this iteration's close release or withdraw the advertisement. The consumer smoke's lattice assertions read the row, and a pass is added per profile.

## Existing sections updated

- `installer/SPEC.md` §Profiles (deltas 1 and 2).
- `docs/choosing.md` §When it does not fit and §What you install (delta 3).
- `docs/kits.md` §The kits and `docs/index.md` §The kits (delta 4).
- `docs/llms-full.txt`, regenerated with `bash gate-sdk/bin/run-gates.sh --emit llms --write` (deltas 3 and 4).
- `docs/installer/SPEC.md`, the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (deltas 1 and 2).
- Contingent on delta 2's mint: `installer/profiles.list`, `installer/README.md` §Choosing a profile and `docs/install.md` §Choosing a profile (delta 2).

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
