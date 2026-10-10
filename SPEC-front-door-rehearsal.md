# SPEC amendment: front-door-rehearsal

Every install smoke in this tree packs the tree it runs in and installs that tarball from the author's seat (installer/SPEC.md §The consumer smoke). None installs the published artifact on a host that has never held the project, and `.github/workflows/publish.yml` runs `roster`, `build`, `pack`, `npm` and `release` and installs nothing after either channel publishes. A one-off rehearsal from a clean container found two defects no smoke had: a hooked `update` or profile move refused by `check-gate-tamper`, and `init` and `uninstall` hiding git's own failure output. This amendment makes the rehearsal a rule of the methodology, a job of the publish run, and a walked procedure for the routes no job can drive.

It sits at the repository root because it spans doctrine-kit, the release runbook and the publish workflow, and no one component owns it.

## What changes

### (1) A doctrine rule: rehearse the published front door from a clean seat

doctrine-kit/DOCTRINE.md gains an engineering-craft rule after the last one, with a `close` stage trailer {design-bearing} {user-facing: the entry's recorded direction that the front-door rehearsal belongs in the methodology; a re-vendoring consumer reads the rule and `--emit stage-rules close` prints its pointer}.

- **Doctrine, not a template slot.** The rule's reader is the session cutting a release, which the `close` trailer reaches through `--emit stage-rules`. A new slot on lifecycle-kit's close template would red every consumer's binding at upgrade (`check-skill-binding`) to carry a sentence the consumer's release procedure already has a home for: that template's `release-policy` slot.
- **The seat, the platforms, the routes and the events are the consumer's.** The rule names none. A consumer states them in the release procedure its `release-policy` slot points at; delta 4 is this repository's.
- **The trailer is `close`** because the release-disposition step is that stage's, and the rehearsal's mechanized half runs off the publish that step triggers.

Rule text. **Not yet applied.**

> 29. **Rehearse the published front door from a clean seat.** Before an audience meets a release, install the artifact as published, by each route that audience will take, on a seat that has never held the project: no checkout, no cached toolchain, no configuration of the author's. Walk a newcomer's first session there, through the first refusal and the first upgrade, and file every finding (the *Gap disposition* rule). A smoke that packs the tree and installs it from the author's seat exercises neither the published bytes nor the clean host, so it passes what a stranger's first install refuses. A route a pipeline can drive is rehearsed by the pipeline on every publish; one it cannot is walked by hand before the event. Which seats, platforms, routes and events those are is the consumer's release procedure's to name. *Under agent work:* an agent's seat is the author's, with every tool placed and configured, so its green install proves the author's path and no other. *Enforced by:* convention at the release boundary; a consumer mechanizes the drivable routes as a job of its publish pipeline. No checkwright gate reads it yet: whether a rehearsal ran on a seat that never held the project leaves nothing in the tree but the consumer's own record.
>
>     *Stages:* close

### (2) A post-publish job installs the Release on clean runners

`.github/workflows/publish.yml` gains a job, `front-door`, that needs `release` and `npm` and runs delta 3's driver once per distinct runner the target roster maps {design-bearing}.

- **The seats come from the roster.** `roster` gains a second output, the distinct runners of `native/runners.list`'s map for the targets in the roster, and the job's matrix is that list. No platform is spelled in the workflow, as for `build`.
- **The job exports no credential to a step.** Its permissions are `contents: read`, for a sparse checkout of the driver's two files and nothing else of the tree, which runs with `persist-credentials: false` and lands under a subdirectory of `RUNNER_TEMP`. Every driver step runs from a working directory outside that checkout, so the seat check reads the runner's own directory as clean. No token reaches a step's environment, so the hosted script meets the signed-out seat a stranger has.
- **The release under rehearsal is the tag's, named explicitly.** The hosted pin still names the release before it until the drain commit moves it (installer/SPEC.md §The hosted install pin), so each leg passes the tag's version through `CHECKWRIGHT_VERSION`.
- **The release before it comes from the `release` job**, as a job output: the newest published Release other than the tag's, read with that job's token before it creates or edits this one. An empty value means no earlier Release, and the driver states the upgrade leg skipped.
- **It follows both channels.** `npm` waits on its approval environment, so the job starts after that approval; a refused or failed `npm` leaves it unrun, and the watch reads that as the channel's failure it is.
- **A red leg is a finding on a published release.** Both channels have published the tag's bytes, so the tag is never re-pushed for it. The finding is filed, and a defect that stops a first install or a first upgrade is fixed and shipped as the next patch release, whose own job reads green. A leg red on a fetch that failed in transit is re-run. A leg red on a 404 for an asset name the release renamed is the drain commit's, which moves the name on the fetch surfaces (RELEASING.md §The procedure, step 4), and no finding on the release; the watch reads the fetch's failing name before it files.
- **The text is written in the shapes the action walk reads today**: step dashes indented under `steps`, and no comment after a block-scalar indicator or a quoted `uses`. The matrix-resolved `runs-on` leaves the dialect unstated, so every `run:` step names its `shell:`, `bash` on the Unix legs and `pwsh` on the Windows ones, each step guarded by `runner.os` (gate-sdk/SPEC.md §check-action-run-shell). That holds whether or not [action-walk-yaml-shapes](TASK-QUEUE.md#action-walk-yaml-shapes) lands in the same batch, so the job waits on no sibling.

### (3) The driver: one stranger's first session, per session

Two new files, `scripts/ci-front-door.sh` and its PowerShell twin `scripts/ci-front-door.ps1`, take a version and an optional previous version and run the sequence below with no other input from the tree {design-bearing}. The shell file runs the Unix legs and the PowerShell one the Windows legs, each through the one-line install its system's install page prints (installer/SPEC.md §The dependency boundary, *The one-line install*).

- **Shell, and permanently.** The subject is the line a stranger types into a shell and the bootstrap that places the compiled binary, so a compiled driver would test the artifact with itself. Each file declares `# no-port:` on the operator's direction, asked and answered in a `/lead` session and lead-relayed on 2026-10-10, that the driver stays permanently shell; offered a temporary `# port-until:` hold, a born-native driver and a `/consult` ruling, the operator chose none. The cause each file carries states that ground and names that answer with its true class, a direction and no ruling, and its date (gate-sdk/SPEC.md §The port-candidate criteria; §The `# graph:` manifest, *A cause names the ruling it rests on*), as `scripts/ci-build-artifact.sh` and `scripts/ci-macos-floor.sh` cite theirs. The cause is each file's own and never a class.
- **The seat is checked first.** A `checkwright` already resolvable on `PATH`, or a working directory inside a git work tree, is exit 2: the seat is not clean, and nothing after it would mean what it says.
- **The first session**, in a new empty repository under the runner's temp directory:
  1. the one-line install at the version exits 0 and leaves a `checkwright.lock` naming it;
  2. each command `init`'s follow-up block prints is run and exits 0, `--install-hooks` among them;
  3. a clean commit lands through the placed hooks;
  4. a commit carrying a violation of a gate the installed profile registers is refused, and the refusal names the gate;
  5. `init` at a profile other than the installed one, read from the artifact's own roster and never spelled in the driver, exits 0 with the hooks on, and a commit lands after it.
- **The first upgrade**, in a second repository, when a previous version is given: the one-line install at the previous version, its follow-up block and a commit; then the same line at the version with `update`, which exits 0 and moves the lock, and a commit lands through the hooks after it.
- **The package route**, in a third: `npx` at the version runs `init`, exits 0 and leaves the lock.
- **Output and status.** One line per step naming its route and step, the step's captured output printed whole when it fails, git's own included. Exit 0 clean, 1 on any step's finding, a fetch that failed among them, 2 on the seat check or a missing operand.

**Inferred, cannot run before build:** that the hosted scripts accept a `CHECKWRIGHT_VERSION` newer than their pin on every leg, and that the previous release's artifact installs on each roster runner — RELEASING.md §The procedure runs the first on one host at each release, and no macOS or Windows host is reachable from this tree; the build session runs the shell driver on Linux against the two newest published releases, and the other legs' first run is [front-door-job-observed](TASK-QUEUE.md#front-door-job-observed)'s.

### (4) The runbook names the rehearsal

RELEASING.md gains a section, *The front-door rehearsal*, and step 5 of §The procedure points at it {design-bearing}.

- **Step 5** scopes its *watch both jobs* and *a red publish is fixed and the tag re-pushed* sentences to the two channel jobs, `release` and `npm`, adds `front-door` to what the publish watch reads, and states delta 2's rule for a red leg in one sentence (the tag is never re-pushed for it, the finding is filed, a defect that stops a first install or upgrade ships as the next patch) with a pointer to the section.
- **The section's first half is the job's contract**: the seats, the two routes (the one-line install and the package) run as three sessions (first session, first upgrade, package), the sequence of delta 3 and its statuses, cited by `publish.yml`'s and the two drivers' `# spec:` lines.
- **Its second half is the walked rehearsal**, this repository's consumer content under delta 1's rule:
  - **When.** Before an audience-facing event and only then: a catalog submission, or an install by a partner on a machine this project does not control. A release cut for no such event takes the job alone.
  - **What.** The routes no job drives: the adoption prompt given to a coding agent, the plugin marketplace install, and the Spec Kit extension. Each is walked by an agent session, on the release the event will meet, through the same first session as delta 3.
  - **Where.** A seat with no checkout of this repository and none of its author's configuration: a fresh container, or a machine the event's audience would recognise as theirs. The runbook names the property; which container is local operations content.
  - **Record.** Every finding goes to the gap inbox with `--emit file-gap`. The queue entry of the event the rehearsal precedes takes one dated line: the release, the seat, the routes walked and how many findings were filed. A rehearsal with no line did not happen.
- **What stays unrehearsed is stated there**: a musl host and WSL, which no roster runner is, wait for the walked half. So does the seat itself: a hosted runner is clean of this project and of the author's configuration but carries its image's preinstalled tools, and the job's checkout is the driver's two files, so a seat with neither is the walked half's.

installer/SPEC.md gains two edits {design-bearing}. §The consumer smoke takes one sentence after its opening paragraph, and §The dependency boundary's *Arguments and overrides* bullet widens *installs a named older release* to *installs a named release, older or newer than the pin*, the use delta 2 and RELEASING.md step 4 make of it. **Not yet applied.**

> It installs the tree's own pack from the author's seat; the published artifact on a clean one is RELEASING.md §The front-door rehearsal's.

## Producers and consumers

- **The rule (delta 1).** Producer: doctrine-kit/DOCTRINE.md, re-vendored by a consumer's upgrade. Consumers: the close session through `--emit stage-rules close` (doctrine-kit/SPEC.md §stage-rules) and `check-doctrine-registration`'s assertion D, which requires the trailer. The always-loaded digest carries the methodology-maintenance rules alone, so CLAUDE.md §Delivery doctrine takes no line.
- **The `front-door` job (delta 2).** Producer: a tag push, the workflow's one trigger. Consumer: the closing session's publish watch (RELEASING.md §The procedure, step 5), and for its first run [front-door-job-observed](TASK-QUEUE.md#front-door-job-observed). Readers of the workflow's text, by `grep -n "check-action" scripts/gates.list`: `check-action-pinning`, `check-action-run-shell`, `check-action-run-path`, `check-action-gh-repo`, `check-action-permissions`, `check-action-job-ref` and `check-action-step-order`. `check-comment-tier` and `check-spec-pointer` read its comment lines, so each `# spec:` line the job carries cites a section delta 4 writes.
- **The two outputs (delta 2).** The distinct-runner list: produced by `roster`, read by `front-door`'s matrix. The previous version: produced by `release`, read by each leg's driver call as its second operand; empty is read as *no upgrade leg*.
- **The driver's operands (delta 3).** Version: read at every step. Previous version: read by the upgrade leg alone.
- **The walked rehearsal's line (delta 4).** Producer: the session that walks it. Consumer: the session running the event, which reads the event's queue entry before acting.
- **Every member's satisfying value (point 6).** Delta 2 obliges each distinct runner of the map to run the driver: the Linux and macOS runners take the shell file, the Windows runners the PowerShell one. A musl target maps to a glibc runner, so no leg is a musl seat, which delta 4 states.

## Existing sections updated

- `doctrine-kit/DOCTRINE.md` — the rule (delta 1).
- `docs/doctrine-kit/DOCTRINE.md` — generated mirror, stale once delta 1 lands (delta 1).
- `.github/workflows/publish.yml` — the job, the two outputs and the header's `# spec:` line (delta 2).
- `scripts/ci-front-door.sh`, `scripts/ci-front-door.ps1` — new (delta 3).
- `RELEASING.md` §The procedure — step 5: the watch's scope and the red-leg rule (deltas 2 and 4).
- `RELEASING.md` — the new section (deltas 3 and 4).
- `installer/SPEC.md` §The consumer smoke — the pointer sentence (delta 4).
- `installer/SPEC.md` §The dependency boundary, *Arguments and overrides* — `CHECKWRIGHT_VERSION` installs a named release, older or newer than the pin (delta 4).
- `docs/installer/SPEC.md` — generated mirror, stale once delta 4 lands (delta 4).
- `.workflow/release-declarations.md` — a Behavior changes bullet led by `doctrine-kit/DOCTRINE.md` *Rehearse the published front door from a clean seat*: a re-vendoring consumer receives the rule and `--emit stage-rules close` gains its pointer; nothing to do (delta 1).

## Retired spellings

- None — every delta adds; no name is removed or renamed.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical text it refines rather than appending to it; the merged surfaces read as one document a reader who never saw the amendment can use alone.
- [ ] **The shell driver run** — on Linux, against the two newest published releases, exit 0 or its findings filed.
- [ ] **The workflow's text held** — the battery green on `publish.yml`, and the mid-iteration push that carries it watched (the push line is [windows-hook-legs-unexecuted](TASK-QUEUE.md#windows-hook-legs-unexecuted)'s).
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Entry moved** — `front-door-rehearsal-rule` moves to Done in the merge commit, which lands before the drain stage. The job's first run is no part of it: a tag alone starts the job, and that reading is [front-door-job-observed](TASK-QUEUE.md#front-door-job-observed)'s.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
