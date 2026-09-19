# SPEC amendment: posix-bootstrap

The unix install bootstrap `installer/bin/checkwright.sh` is a bash script
(`#!/usr/bin/env bash`), and every documented and smoke invocation of it spells
`bash`. So a starter or prose install on Linux or macOS still reaches `bash`,
though since rung 4a the battery and the generated hooks do not. This amendment
ports the bootstrap to POSIX sh, run by the OS's own `/bin/sh`. The unix install
floor becomes git, `/bin/sh` and the base userland the bootstrap already reads.
The five install steps, their three selection outcomes, their refusal messages
and the PowerShell twin do not change. Only the language of the unix half does.

This is the reading of TRAJECTORY.md objective 6 the generated hooks already
rest on (gate-sdk/SPEC.md §gen-pre-commit): POSIX sh is the subset of bash that
the OS's own `/bin/sh` runs as well, so a POSIX script keeps the "bash for Linux
and macOS" policy and drops `bash` from the floor.

**Probed at authoring, 2026-09-19, at `ad651168`:**

- The bootstrap uses seven constructs POSIX sh lacks: `BASH_SOURCE`, `[[ ]]`,
  `local`, `set -o pipefail`, arrays, process substitution (`< <(find …)`) and
  `find -maxdepth`. `shellcheck -S warning` on a `#!/bin/sh` file carrying the
  first six reports SC3028, SC3010, SC3014, SC3043, SC3040, SC3030, SC3024,
  SC3054 and SC3001, all at warning level — `[[ ]]`'s `==` pattern test takes
  SC3014 beside SC3010, and the array construct's declaration, `+=` and
  `${arr[@]}`/`${arr[0]}` references each take their own code (SC3030, SC3024,
  SC3054) beside one another. `find -maxdepth` is not a shellcheck finding.
- `check-shellcheck` already lints `installer/bin`
  (`GATE_SDK_LINT_EXTRA_DIRS` in `scripts/gate-sdk-config.knobs`), passes
  `-S warning` (`native/src/gates/shellcheck.rs:74`) and no `-s`. So shellcheck
  takes the dialect from the shebang, and the gate lints a `#!/bin/sh` bootstrap
  as POSIX sh with no change to the gate.
- npm's `cmd-shim` reads the shebang's program token (`lib/index.js:24,56`).
  `/bin/sh` works in the extension-less shim, which MSYS runs. The `.cmd` shim
  gets an unresolvable `"/bin/sh"`. A native-Windows adopter is sent to the
  PowerShell entry `checkwright-pwsh` (§Layout), so no supported path goes
  through that shim.
- Every smoke and page invocation spells `bash`:
  `installer/consumer-smoke/run-smoke.sh` lines 917, 933, 964, 1254, 1302, 1329,
  1331, 1340 and 1506. The bash-less arm runs `"$BASH" "$DL_ENTRY"` (line
  1132). Also `docs/install.md:341`, `installer/README.md:53` and
  `installer/SPEC.md:46`. Found with `git grep -n "checkwright\.sh"` over the
  tracked tree, plus tracing line 964's `ENTRY=(bash "$DL_ENTRY")` array to
  `DL_ENTRY`'s assignment (line 951): the literal grep alone gives 917, 933,
  951, 952, 1254, 1302, 1329, 1331, 1340 and 1506, and 951-952 are the
  assignment and its existence check rather than invocation sites.

## What changes

### (1) The unix bootstrap is POSIX sh {design-bearing}

`installer/bin/checkwright.sh` keeps its name, its mode, its `bin` entry, its
function names and the five steps in order. It starts `#!/bin/sh` and uses only
POSIX shell, which `check-shellcheck` enforces through the shebang. Each
construct POSIX lacks is replaced like this:

- **Step 1, the payload directory.** `$0` replaces `BASH_SOURCE[0]`. The
  bootstrap is executed and never sourced, so `$0` is the script's path, and the
  npm link is still resolved with plain `readlink`. The script runs `unset CDPATH`
  before its first `cd`, because a POSIX `cd` into a relative path consults
  `CDPATH` and prints the directory it picks, which corrupts the `$( )` holding it.
- **Step 3, selection.** A glob over `"$src"/*` replaces `find -maxdepth 1 -type f
  ! -name '*.sha256' | sort`. It counts regular files not ending in `.sha256` and
  keeps the one it found. There is no array, no process substitution and no
  `find` primary to disagree between GNU and BSD. The glob skips dotfiles, so a
  stray dotfile beside the binary is no longer counted as a second artifact.
- **Pattern tests.** `case` replaces every `[[ … == pattern ]]` (the
  `*-linux-gnu` test, the absolute-link test and step 5's dash test), and `[ ]`
  replaces the rest.
- **No `local`.** POSIX has none, and a `/bin/sh` that is not dash, bash or a BSD
  sh may lack it. The functions share the script's scope, so each working name is
  unique within the file.
- **No `pipefail`.** No pipeline's verdict needs it. The libc probe reads `grep`'s
  status, the roster test reads the last `grep`'s, and step 4 reads what it
  captured. A hasher that fails leaves `got` empty, and the empty-`want`/unequal
  test already refuses that.

Some things do not change. `target_of_host` still emits each triple as the sole
single-quoted operand of a `printf`, so `check-install-platforms` extracts it
unchanged. Every `die` message and remedy, every exit status, the
`sha256sum`-then-`shasum -a 256` order and step 5's argv rule stay byte for byte.
The file's `# no-port:` declaration still cites §The install boundary.

**Not yet applied.** installer/SPEC.md §Implementation, first paragraph: its
opening word, "Bash, up to the boundary §The install boundary rules", becomes
"POSIX sh, up to the boundary §The install boundary rules"; "one bash, one
PowerShell" becomes "one POSIX sh, one PowerShell". In its second paragraph,
"governs the bash one" becomes "governs the POSIX one, in the dialect its
`#!/bin/sh` selects".

**Not yet applied.** installer/SPEC.md §Layout: the first bullet reads
"`bin/checkwright.sh` — the POSIX sh bootstrap (§The install boundary), run by the
host's own `/bin/sh`, and the package's `bin` entry." The second bullet's "the
shim npm writes for the bash target" becomes "the shim npm writes for the sh
target".

**Not yet applied.** installer/SPEC.md §The install boundary:

- "rather than twice, in bash and in the PowerShell half" becomes "rather than
  twice, in POSIX sh and in the PowerShell half".
- The two standing obligations become "**add no new shell-only install step**,
  and **assume no POSIX shell on a host the PowerShell half serves**". The POSIX
  half is now itself a POSIX shell script, so the unqualified phrasing would
  contradict the section.
- In the legs paragraph, "every one of them drives the *bash* half, the Windows
  one through Git-for-Windows bash" becomes "every one of them drives the *POSIX*
  half, the Windows one through Git for Windows' `sh`".

**Not yet applied.** installer/SPEC.md §The gate binary:

- "in bash and in PowerShell" becomes "in POSIX sh and in PowerShell".
- The **bash** labels in the resolution paragraph and in the extraction-shape
  bullet become **POSIX sh**.
- "the bash half's" becomes "the POSIX half's" in the `MINGW*` sentence and in
  the hasher-resolution paragraph.
- "the bash arm's" becomes "the POSIX arm's".
- "the bash half declared-and-proceeded" becomes "the unix half
  declared-and-proceeded". That sentence is history, so it names the half by
  platform rather than by language.

### (2) Every invocation of the bootstrap spells `sh` {mechanical}

The adopter-facing command in docs/install.md §Quick start and in
`installer/README.md` changes from `bash "$cw/package/bin/checkwright.sh" init` to
`sh "$cw/package/bin/checkwright.sh" init`. installer/SPEC.md §Requirements
changes from `bash package/bin/checkwright.sh init` to
`sh package/bin/checkwright.sh init`. Each `bash "…/package/bin/checkwright.sh"`
in `installer/consumer-smoke/run-smoke.sh`, including the `ENTRY=(bash …)`
arrays, becomes `sh "…"`, so the smoke runs the bootstrap exactly as the page
tells an adopter to. The npm-transport arms keep `ENTRY=("$CW")`, the linked
`bin` entry, which now runs through its shebang's `/bin/sh`.

This is what makes the smoke the running oracle for the port. On a Linux leg,
`sh` is the distribution's `/bin/sh`.

**Inferred, cannot run before build:** `/bin/sh` on the `install-smoke` job's `ubuntu-latest` runner is dash, a shell with no bash extensions, so every arm there runs the bootstrap under a strict POSIX shell — no runner is reachable from the authoring session; the probe line this delta adds to that job's smoke step, `echo "/bin/sh is $(readlink -f /bin/sh)"`, settles it on the first pushed run.

macOS's `/bin/sh` is bash 3.2 in POSIX mode, a lenient oracle, so there the
claim rests on the Linux leg and on `check-shellcheck`.

### (3) The bash-less arm runs the bootstrap through the farm's `sh` {mechanical}

The bash-less arm's entry becomes `ENTRY=(sh "$DL_ENTRY")`, resolved on
`$BASH_PATH`. Before the first verb, the arm asserts that `sh` resolves under
that `PATH`, in the same way it proves the farm's `git` runs. A farm that dropped
`sh` would otherwise fail every step for a reason that is not bash. After this
delta, no step of the arm (bootstrap, binary, follow-up commands or hooks) runs
with `bash` reachable. The arm then proves the whole starter/prose path, install
included, needs no `bash`.

**Not yet applied.** installer/SPEC.md §The consumer smoke, the bash-less arm's
paragraph: "The unix bootstrap is itself a bash script (§The install boundary), so
the arm runs it through the harness's own interpreter named by absolute path;
nothing it or the binary spawns may find `bash` on `PATH`." becomes "The arm runs
the unix bootstrap as the page does, through `sh` resolved on the farm, so nothing
from the bootstrap to the hooks can find `bash` on `PATH`."

### (4) The adopter surfaces stop saying installing needs bash {mechanical}

**Not yet applied.** docs/install.md §Requirements, the `bash` toolchain bullet.
The two sentences from "On every profile, though, the unix install bootstrap" to
"porting the bootstrap to POSIX sh is what removes it." are replaced by: "The unix
install bootstrap (`installer/bin/checkwright.sh`) is POSIX sh too, run by the
OS's own `/bin/sh`, so installing a starter or prose profile reaches no `bash`
either."

**Not yet applied.** docs/install.md §Where this is heading: "a PowerShell half
ships beside the bash one" becomes "a PowerShell half ships beside the POSIX sh
one". "the bash one under Git for Windows" becomes "the POSIX one under Git for
Windows' `sh`".

**Not yet applied.** context-kit/SPEC.md's `bash` roster element: the closing
sentence "The unix install bootstrap is a bash script on every profile, but it
runs before any reader of this roster and is no kit's surface, so it is stated on
the install page rather than here." is deleted. The bootstrap reaches no `bash`,
so the carve-out has nothing left to explain.

**Not yet applied.** `.workflow/release-declarations.md` `## Behavior changes`
gains: "**`installer/bin/checkwright.sh`** — the unix bootstrap is POSIX sh
(`#!/bin/sh`) and no longer needs `bash`. Run it as
`sh package/bin/checkwright.sh <verb>`. `bash …` still works, since bash runs
POSIX sh."

### (5) Comments that name the bash half are re-worded {mechanical}

The header directive of `installer/bin/checkwright.sh`. The step-4 directive of
`installer/bin/checkwright.ps1` ("the bash one" becomes "the POSIX one"). The
header comments of the two `check-install-platforms` fixture stand-ins. The
`install-smoke-powershell` step comment in `.github/workflows/gates.yml` ("the
bash half's own smoke leg").

The `install-smoke-macos` probe step's `find -printf` measurement, with its two
comment lines, is deleted. It measured whether the bootstrap's refusal of that
primary was needed, and delta 1 removes `find` from the bootstrap, so the
measurement has no reader.

## Producers and consumers

- **The POSIX bootstrap.** The producer is the file itself, shipped in the
  package's `bin/` directory (`installer/package.json` `files`). Its consumers are
  the adopter's `sh …` command, npm's `bin` link, which executes through the
  shebang, every smoke `ENTRY` (delta 2) and the bash-less arm (delta 3).
  `check-shellcheck` reads its shebang for the dialect. `check-install-platforms`
  reads `target_of_host`, whose shape is unchanged. `check-portability-floor`
  reads the file as part of `GATE_SDK_PORTABILITY_PATHS` and loses no construct
  it held. Roster-holding readers of the path: `scripts/core-files.list:45`, the
  `check-install-platforms` descriptor's `couples=` and the generated
  pre-commit hook's `staged_matches` line. All three name the path and not the
  language, so none changes (`git grep -n "checkwright\.sh"`).
- **The `sh`-resolves assertion (delta 3).** It is produced and read inside the
  arm. A red names the farm.
- **The dash probe line (delta 2).** It is read by the build session on the
  first pushed run, and then by any later reader of the leg's log. It sets no
  verdict.
- **Point 5 (narrowing):** delta 5 deletes one probe step's measurement. The step
  is reporting-only (`set +e`, `exit 0`), so no verdict reads it.

## Existing sections updated

The roster's probe: `git grep -n -i -E "bash (half|bootstrap)|checkwright\.sh"` over
the tracked tree at `ad651168`, plus `grep -n bash installer/SPEC.md` for the
design record's own labels.

- `installer/bin/checkwright.sh`: the port and its header directive (deltas 1 and 5).
- `installer/SPEC.md`: §Implementation, §Layout, §The install boundary and §The gate binary (delta 1); §Requirements (delta 2); §The consumer smoke, the bash-less arm (delta 3).
- `installer/consumer-smoke/run-smoke.sh`: every `bash …/checkwright.sh` site and the bash-less arm's entry, its `sh` assertion and its directive (deltas 2 and 3).
- `installer/README.md`: the quick-start line (delta 2).
- `docs/install.md`: §Quick start (delta 2); §Requirements' `bash` bullet and §Where this is heading (delta 4).
- `context-kit/SPEC.md`: the `bash` roster element's closing sentence (delta 4).
- `.workflow/release-declarations.md`: the Behavior-changes bullet (delta 4).
- `installer/bin/checkwright.ps1`: the step-4 directive (delta 5).
- `scripts/gate-tests/check-install-platforms/good/checkwright.sh` and its `bad/` twin: header comments (delta 5).
- `.github/workflows/gates.yml`: the `install-smoke` job's `/bin/sh` probe line (delta 2); the `install-smoke-powershell` comment and the `install-smoke-macos` probe step (delta 5).
- `docs/installer/SPEC.md`, `docs/installer/README.md`, `docs/context-kit/SPEC.md`: the generated mirrors, regenerated (all deltas).

## Retired spellings

- `bash half` — the unix bootstrap's language label in the design record and its comments (deltas 1 and 5).
- `bash bootstrap` — the same label's noun form, in the design record and the fixtures' headers (deltas 1 and 5).

## Definition of Done

- [ ] **Causal completeness**: every point of SPEC §The causal-completeness
      check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only**: replacement text for a
      template, agent definition or shim carries no grounds, and a delta places
      them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost**: each addition re-phrases the
      canonical-spec text it refines rather than appending to it, and the merged
      spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted**: this file is removed on merge, and none remain for the
      component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated**: every name this change retired is declared in
      `## Retired spellings` above, and `check-amendment-retired-spelling` runs
      each declaration against the whole tracked tree, not against the specs
      alone.
- [ ] **Gaps filed**: cross-component gaps found during the work are filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Proved on a pushed run**: the ubuntu leg's probe line names dash, and the
      bash-less arm is green on Linux and on both macOS legs.
