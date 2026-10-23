# SPEC amendment: npx-windows-route

Pairs `npx-native-windows-route`. On native Windows, `npx checkwright init` fails before the bootstrap runs: npm's `.cmd` shim takes the interpreter from the `checkwright` bin's `#!/bin/sh` line and runs that absolute path, which no native Windows host carries. The entry's operator direction (2026-10-24, lead-relayed, not a ruling) picks shape (B): the bin's line becomes `#!/usr/bin/env sh`, so every shim runs `sh` from `PATH`, which the install page's `windows-remedy` block already supplies, and one `npx` line holds on every host. The condition is a running witness on `install-smoke-pwsh-windows`. If that witness reds, the fallback is shape (A): the Windows line names the PowerShell entry and the page states PowerShell 7. This amendment gives (B)'s mechanism, the witness, and (A) authored in full as delta 9, so a red witness can be acted on without designing anything.

**What the tree does today**, read at spec:

- npm's shim generator (`cmd-shim`, read from npm 11.16.0's `lib/index.js`) matches the first line against `^#!\s*(?:\/usr\/bin\/env\s+(?:-S\s+)?(...))?([^ \t]+)(.*)$`. `#!/bin/sh` yields the program `/bin/sh`, and the `.cmd` shim runs `"/bin/sh"` when `%dp0%\/bin/sh.exe` is absent. That is cmd's `The system cannot find the path specified.` from v0.32.0's front-door Windows legs. `#!/usr/bin/env sh` yields `sh`, and each of the three shims then runs `sh` (the `.cmd` and `.ps1` shims resolve it on `PATH`, with `.exe`), passing the script's path after it. The `.cmd` shim passes that path as `"%dp0%\..\checkwright\bin\checkwright.sh"`, every separator a backslash.
- `npx` on Windows runs a bin through `ComSpec`, falling back to `cmd` (`libnpmexec`'s `scriptShell`), so the `.cmd` shim is the route `npx checkwright` takes.
- Only npm's bin link reads the bootstrap's shebang. Every other route names `sh` itself: `docs/install.sh`, the `unix-install` block, the CI action through `install.sh`, the gates leg's POSIX-bootstrap step, and the consumer smoke's extracted entry. No crate code reads the line. This is the survey filed at this stage, whose witness is in the survey record.
- `shellcheck -S warning` reads `#!/usr/bin/env sh` as POSIX sh, and passes the bootstrap carrying that line. A `[[ ]]` control under the same line raises SC3010.
- No run drives the `.cmd` route. The consumer smoke's Windows legs drive the npm-installed package through `bin/checkwright.ps1` (§The consumer smoke), and the gates leg runs `sh <path>` from Git Bash, which sets `MSYSTEM`, so the bootstrap's `MSYS*` arm (the `uname` a bare `usr\bin\sh.exe` reports) has never run.

## What changes

### (1) The npm bin's interpreter line {mechanical} {user-facing: the entry's operator direction of 2026-10-24, shape (B), one `npx` line on every host}

`installer/bin/checkwright.sh`'s first line becomes `#!/usr/bin/env sh`. Nothing else in the file moves for this delta. **Not yet applied.**

### (2) Step 1 respells a drive-rooted `$0` {design-bearing} {user-facing: as delta 1, the mechanism (B)'s route runs on}

`installer/bin/checkwright.sh`, between `SELF="$0"` and the `while [ -L "$SELF" ]` loop, gains a `case` on `$SELF`. A value opening with an ASCII letter, `:` and `\` is respelled with every `\` turned to `/` (`tr`, a floor member the file already runs). The letters are spelled out as `gate_path_rooted` spells them, since a bracket range is locale-dependent. A `# spec: installer/SPEC.md §Implementation` directive above it names the producer: npm's `.cmd` shim hands `sh` a backslash-separated path. No other value is touched. A POSIX path never opens with a drive letter, and a relative one keeps any backslash it carries. That was probed with the `case` and `tr` above over `C:\…\.bin\\..\checkwright\bin\checkwright.sh`, `/usr/lib/node_modules/checkwright/bin/checkwright.sh` and `./we\ird/bin/checkwright.sh`. Step 1 then resolves the payload from separators every POSIX runtime reads, rather than relying on how MSYS's `dirname` and `cd` read a backslash. **Not yet applied.**

**Inferred, not run:** Git for Windows' `usr\bin\sh.exe`, started by npm's `.cmd` shim with no `MSYSTEM` set, runs the respelled bootstrap to the verified artifact: `cd` resolves a `C:/` path, `uname -s` matches the `MSYS*` arm, and `sha256sum` resolves from `usr\bin` — the `install-smoke-pwsh-windows` step delta 5 adds, on the closing push's `gates` run

### (3) §Layout's bootstrap bullet {mechanical}

Replacement for `installer/SPEC.md` §Layout, the bullet opening ``- `bin/checkwright.sh` ``. **Not yet applied.**

> - `bin/checkwright.sh` — the POSIX sh bootstrap (§The install boundary) and the package's `bin` entry. **Its `#!/usr/bin/env sh` line is load-bearing**: npm's shim generator takes the interpreter from it, and a bare program name is the form all three of npm's shims resolve on `PATH`. An absolute interpreter path is a literal the `.cmd` shim cannot resolve on native Windows. So `npx checkwright` is one line on every host. On native Windows the shim starts the `sh` Git for Windows carries, which the install page's `PATH` block puts on `PATH`, and step 1 respells the backslash path the shim hands it (§Implementation). `install-smoke-pwsh-windows` runs that line from Windows PowerShell 5.1 through the `.cmd` shim, against the packed tarball, with every `sh` stripped from `PATH` until the page's block puts Git's back.

### (4) The other passages naming `/bin/sh` as the bootstrap's interpreter {mechanical}

Each a replacement in `installer/SPEC.md`. The survey above found them. **Not yet applied.**

- §Implementation, the sentence opening **The repository's script linter governs the POSIX one**, ends "in the dialect its `sh` shebang selects."
- §Implementation, the paragraph opening **The POSIX half is run by the host's own `/bin/sh`**, through **so its green is a strict-shell proof.**:

  > **The POSIX half runs under the `sh` the host's `PATH` resolves.** Every route names `sh` rather than a path: the page's lines pipe or pass the script to it, and npm's shims take it from the `#!/usr/bin/env sh` line (§Layout). So the unix install floor is git, a POSIX `sh` and the base userland the bootstrap reads, with no `bash`. POSIX sh is the subset of bash the OS's `sh` also runs, the reading of the bash-for-Linux-and-macOS policy this bootstrap rests on. The `gates` workflow's Linux leg prints what `sh` resolves to, `dash` there, so its green is a strict-shell proof.

- §Implementation, the bullet opening **The payload directory comes from `$0`.**, its first two sentences:

  > **The payload directory comes from `$0`.** The bootstrap is executed, never sourced, so `$0` is its path. A drive-rooted `$0`, the backslash path npm's `.cmd` shim hands `sh` on native Windows, is first respelled with forward slashes, a separator every POSIX runtime reads, and npm's `bin` link is resolved with plain `readlink`.

- §Implementation, the **No `local`.** bullet: "a `/bin/sh` that is not dash" becomes "an `sh` that is not dash". In the section's closing paragraph both `/bin/sh` become `sh`: "macOS's `sh` is bash in POSIX mode" and "whose `sh` is dash".
- §Requirements, the **What runs without `bash`.** bullet: "is POSIX sh, run under `/bin/sh`, so" becomes "is POSIX sh, run under the `sh` on `PATH`, so".
- §The consumer smoke, the **`bash`-less arm** paragraph: "`/bin/sh` is not masked: the unix bootstrap runs under it (§Implementation)." becomes "`sh` is not masked: the bootstrap runs under it (§Implementation)."
- §The consumer smoke, the paragraph opening **Every arm runs on every leg, through the host's bootstrap.**: the clause "so the install arm's `npm install --offline` still runs while its `.bin` entry is not the route under test there." becomes "so the install arm's `npm install --offline` still runs, and its `.bin` entry is driven by the PowerShell leg's `npx` step instead (§Layout)."

### (5) The Windows witness step {design-bearing}

`.github/workflows/gates.yml`'s `install-smoke-pwsh-windows` gains a step after `the POSIX bootstrap installs the lattice minimum on this Windows host`, named for its claim, such as `the page's npx line installs through npm's cmd shim`. It carries `if: matrix.part == 'bootstrap'`, `shell: powershell` (Windows PowerShell 5.1, the page's Windows floor) and a `# spec: installer/SPEC.md §Layout` directive. **Not yet applied.** In order, each a named failure:

1. **Every `sh` leaves `PATH`.** Every directory carrying `sh.exe` is stripped, the system directory excepted, by the PATH-strip loop the `commit through the placed hooks with no bash on PATH` step uses. Git's `cmd` directory is kept or added, so the page's block can ask `git` for its root. The step asserts that no `sh` resolves and that `git` and `npx` both do.
2. **The page's `windows-remedy` block runs verbatim.** It is extracted from `docs/install.md` as the `windows-install` step extracts it. The step asserts that `sh` then resolves, under the Git root `git --exec-path` names.
3. **The page's line installs.** In a fresh consumer (`git init`, a smoke identity), `npx` resolved as an application, as `scripts/ci-front-door.ps1` resolves it, runs `--yes --package=<PWSH_TARBALL> -- checkwright init --profile <minimum>`. `<minimum>` is the lattice minimum, read from the package's `profiles.list` as the POSIX-bootstrap step reads it. The step asserts exit 0, a `checkwright.lock`, and a lock whose `artifact.target` equals `PWSH_TARGET`. On a red it prints the run's output whole.

The `--package=<tarball>` form keeps the registry out of the witness. The command line is the page's, and its bin is the packed one. Short of a tag no published package carries delta 1, so this is the only reachable form. The leg's `timeout-minutes` is widened, before the push, if the step's cost is not covered by its headroom.

### (6) The Linux leg names the `sh` the bootstrap runs under {mechanical}

`.github/workflows/gates.yml`, the step `name the shell the unix bootstrap runs under`: its `run:` becomes `readlink -f "$(command -v sh)"`. The `# spec:` directive above it ends `this names the sh on PATH it runs under`. **Not yet applied.**

### (7) The install page's Node section {mechanical} {user-facing: as delta 1, the remedy being the entry's "which the page's `windows-remedy` block already supplies"}

Replacement for `docs/install.md` §With Node, its whole paragraph. **Not yet applied.**

> `npx checkwright init` runs the same `init` from the npm package, which carries npm's provenance attestation as the tarball carries a GitHub build attestation ([installer/SPEC.md](installer/SPEC.md#the-dependency-boundary)). The package's command is the POSIX bootstrap, so on Windows run the `PATH` block above first, which puts the `sh` npm starts it with on your `PATH`. `init` verifies the gate binary with `sha256sum` or `shasum`, and refuses without one. It needs Node 8.2 or later ([Requirements](#requirements)).

The sentence adds about 25 words to a page `check-docs-page-length` holds at 1500. **It depends on whether `prove-front-door-restoration` lands in the same build batch.** If it does, that entry's cut-to-fit is measured with this sentence in place, and whichever of the two commits lands second runs the gate on the joint page. If it does not, this delta lands alone, and the gate's verdict on the page with this sentence is the oracle. If it reds, a clause elsewhere on the page is cut, never this sentence.

### (8) The npm README and the Requirements row {mechanical} {user-facing: as delta 7}

Each **Not yet applied.**

- `installer/README.md` §Quick start, the paragraph opening **With Node already present**, its first sentence becomes: ``With Node already present the same install is one command, `npx checkwright init` — same payload, same `init`, same `checkwright.lock`; only the fetch differs, and on Windows the [`PATH` block](https://checkwright.dev/install.html#windows) runs first.``
- `docs/requirements.md`'s prerequisites block, the `Git for Windows` row's last cell becomes: ``it supplies `git`, and once the install page's `PATH` block has run, the `sh` and `sha256sum` `npx` runs the bootstrap with; `bash` only where a row above owes it``.

### (9) The fallback, if the witness reds {design-bearing} {user-facing: the entry's operator direction of 2026-10-24, "(A), with PowerShell 7 stated on the page"}

**Contingent. It is built only if delta 5's step reds on the closing push's `gates` run, and is not merged with deltas 1 to 8.** The executor is close, as its red-push hotfix inside its push budget. Close reads this delta from the merging commit's parent: `git show <merge commit>^:installer/SPEC-npx-windows-route.md`. Deltas 1, 3 and 4 stand, since they are correct on every unix host and leave npm's route under the `PATH` `sh` that every other route already uses. Delta 2 is reverted, because nothing then hands the bootstrap a drive-rooted path. Deltas 5, 7 and 8 are replaced by the following, each **Not yet applied**:

- **The route.** Windows spells `npx -p checkwright checkwright-pwsh init`. A bare `npx checkwright-pwsh` would fetch a package of that name. npm's shim starts `pwsh` from that bin's `#!/usr/bin/env pwsh` line, so the route needs PowerShell 7.
- **`docs/install.md` §With Node**: delta 7's Windows sentence becomes ``On Windows, `npx -p checkwright checkwright-pwsh init` runs the PowerShell bootstrap instead; npm starts it with `pwsh`, PowerShell 7, which Windows does not carry by default.`` The page-length dependence on `prove-front-door-restoration` is delta 7's.
- **`installer/README.md`**: delta 8's Windows clause becomes ``on Windows, `npx -p checkwright checkwright-pwsh init`, which needs PowerShell 7``.
- **`docs/requirements.md`'s prerequisites block** gains a row after `Node`: `` | PowerShell 7 (`pwsh`) | any | optional: only to install with npx on Windows | npm starts the package's PowerShell bootstrap with `pwsh`, the name its first line carries | ``. The `Git for Windows` row keeps its present cell.
- **`installer/SPEC.md` §Layout**: delta 3's bullet keeps its first two sentences and ends ``On native Windows the `.cmd` shim's `sh` does not reach a running bootstrap (the PowerShell leg's witness), so a native Windows adopter is sent to `checkwright-pwsh` below, with PowerShell 7.`` The `bin/checkwright.ps1` bullet gains ``The route is `npx -p checkwright checkwright-pwsh`, since npx resolves its first operand as a package name.``
- **`check-front-door-verbs`**: `ROUTES` in `native/src/gates/front_door_verbs.rs` gains `npx -p checkwright checkwright-pwsh`, so the verb after it is advertised and held to the pinned set. The unit test that holds `npx checkwright-pwsh init` advertising nothing stays, and a test beside it holds `npx -p checkwright checkwright-pwsh init` advertising `init`. `scripts/gate-tests/check-front-door-verbs/good/` gains a page site of the new route. `installer/SPEC.md` §The front door's verbs, **Routes.**, lists the route after `npx checkwright`.
- **`scripts/ci-front-door.ps1`**, the package route: `npx --yes -p "$Package@$Version" checkwright-pwsh init`. RELEASING.md §The front-door rehearsal's package-route bullet gains ``(on the Windows legs, `-p checkwright@<version> checkwright-pwsh`)``.
- **Delta 5's step** keeps its consumer and its assertions. It strips nothing, asserts that `pwsh` resolves, and runs `--yes --package=<PWSH_TARBALL> -- checkwright-pwsh init --profile <minimum>`.

## Producers and consumers

- **The interpreter name** (delta 1). Producer: npm's shim generator, when the package is installed or fetched by `npx`. It reads the first line once per install, in every deployed configuration, since every npm install of the package runs it. Consumers:
  - the `.cmd` shim, the `.ps1` shim and the extension-less shim, each resolving `sh` on `PATH`;
  - the kernel on a unix host, which runs the bin link's `#!/usr/bin/env sh` through `env`;
  - `check-shellcheck`, which reads the dialect from the line. Probed above: it reds on a bash construct, as it did under `#!/bin/sh`.
- **The respelled path** (delta 2). Producer: the `.cmd` shim's argv on native Windows, the one route that hands the bootstrap a drive-rooted backslash path. The extension-less shim hands a mixed path whose last separator is `/`, which passes through unchanged. Consumers: step 1's `readlink` loop, its `dirname` and its `cd`. The walk test holding `gate_path_rooted` byte-equal to gate-sdk's library reads that function's body alone, which the delta leaves as it is. `check-install-platforms` extracts `target_of_host` and `fallback_of_target` by shape, and the new `case` sits outside both.
- **The witness** (delta 5). Producer: the `gates` workflow on every master push, on each `pwsh_parts` entry whose `part` is `bootstrap`. That is a deployed configuration, since the closing push triggers it. Consumers:
  - close's push watch, which reads every run of the push to green before the tag (`.claude/commands/close.md`, `push-budget`);
  - delta 9's trigger, a red on this step.

  Its red conditions are a `sh` resolving after the strip, `git` or `npx` not resolving, no `sh` after the page's block, a non-zero `npx` exit, no lock, and a lock target other than the leg's. Each is a named line. A green says the `.cmd` shim, Git's `sh`, step 1, selection and `init` all ran on that host.
- **The page's precondition** (delta 7). Its reader is a Windows adopter on the Node route. It also has these roster-holding and readers' verdicts:
  - `check-front-door-verbs` reads the `npx checkwright` route's verb, `init`, which is unchanged and is in the pinned set, so no new advertisement arises.
  - `check-docs-page-length` reds a page above 1500 words. That is delta 7's sibling dependence.
  - `check-md-refs` reds a link resolving to no file. The paragraph's links are unchanged.
  - `check-llms-fresh` reds `docs/llms-full.txt` until it regenerates.
- **The README and the Requirements row** (delta 8):
  - `check-md-refs` and the docs mirror's freshness read the README. Its new link is an absolute URL to the install page's `#windows` anchor, the form its other install-page links take.
  - `check-install-platforms` arm F holds that every prerequisites row states a Minimum and that every declared system has a row. The row keeps its Minimum and Needed cells, so arm F's inputs are unchanged.
- **The Linux leg's print** (delta 6). It has no machine reader. It is the evidence the §Implementation sentence cites.
- **The release.** No declaration-bearing section takes a bullet. Nothing in a consumer's tree changes or reconciles, since the bootstrap stays in the package and is never vendored, and `check-release-change-declared` reads neither a removed kit tool nor a claimed template here. So the iteration's patch release stays a patch.
- **Delta 9's consumers**, if built:
  - `check-front-door-verbs` reds an advertised verb outside the pinned set on the new route. `init` is in it.
  - `check-install-platforms` arm F reds the new row if a cell is empty. Every cell is filled.
  - The gate's fixture pair reds if `good/` stops matching its expected output, which is regenerated in the same commit.

## Existing sections updated

- `installer/bin/checkwright.sh` (deltas 1 and 2).
- `installer/SPEC.md` §Layout (delta 3), §Implementation, §Requirements and §The consumer smoke (delta 4).
- `.github/workflows/gates.yml`, `install-smoke-pwsh-windows` (delta 5) and the Linux leg's shell-naming step (delta 6).
- `docs/install.md` §With Node (delta 7).
- `installer/README.md` §Quick start and `docs/requirements.md`'s prerequisites block (delta 8).
- `docs/llms-full.txt`, regenerated with `bash gate-sdk/bin/run-gates.sh --emit llms --write` (deltas 7 and 8).
- `docs/installer/SPEC.md` and `docs/installer/README.md`, the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).
- Contingent on delta 9 alone: `native/src/gates/front_door_verbs.rs`, `scripts/gate-tests/check-front-door-verbs/good/`, `scripts/ci-front-door.ps1`, RELEASING.md §The front-door rehearsal, and `installer/SPEC.md` §The front door's verbs (delta 9).

The `/bin/sh` passages are the survey's: `grep -n '/bin/sh' installer/SPEC.md .github/workflows/gates.yml` and `git grep -n 'bin/checkwright.sh\|checkwright\.sh'` over the tracked tree, the docs mirrors and dated posts set aside. The `npx` front-door surfaces were found by `git grep -n npx` over the front-door pages `check-front-door-verbs` lists. `README.md` and `docs/index.md` carry the bare line, which (B) makes true on every host, and `docs/maintenance.md` names `npx checkwright <verb>` generically. None of the three changes.

## Retired spellings

- None — no delta retires a name. The `#!/bin/sh` line delta 1 replaces is a file's first line that no other surface spells as this file's, and the `/bin/sh` passages delta 4 rewrites stay true of the other `#!/bin/sh` scripts they never named.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls installer/SPEC-*.md`). Delta 9 is not merged: close reads it from the merging commit's parent if the witness reds.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The local oracle** — the battery and the `installer_smoke` suite green. The unix legs' npm-installed `node_modules/.bin/checkwright` runs delta 1's line through `env`.
- [ ] **The Done move** lands before the drain stage, with the merge of deltas 1 to 8. The witness is the release's precondition, not this entry's: the set record places it on the closing push, before the tag, and close's push watch reads it there. A red there is delta 9's trigger.
