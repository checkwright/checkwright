# SPEC amendment: installer-smoke-driver

**The installer's consumer smoke becomes one compiled driver, the gate binary's `--installer-smoke` arm, run on every CI leg, and both `installer/consumer-smoke/run-smoke.sh` and `run-smoke.ps1` are deleted.** Today the bash driver runs the whole suite on the four unix legs, and the PowerShell driver runs five coverage classes on the native Windows legs (installer/SPEC.md §The consumer smoke, *The PowerShell driver carries native Windows*). The two are held in step by nobody, and eighteen arms never run on native Windows. The driver also takes over the nested batteries' environment, which the bash driver passes down whole. It pairs `compiled-consumer-smoke-driver`.

**Its reach extends past the installer into evidence-kit.** The validate parser, `--emit parse-smoke-log`, is evidence-kit's and ships to every adopter, and delta 5 widens its contract with a log-head roster form. The selected unit set did not name that reach; the operator chose the form with it stated (direction of 2026-10-02, lead-relayed, not a ruling), so the release declaration names the new form.

**Measured at authoring** (a read-only survey of both drivers and `.github/workflows/gates.yml`, and the probes named here):

- **The bash driver** prints 27 parsed arm headers, `build` through `artifact arm`, then its `INSTALLER-SMOKE: clean` marker; `.workflow/validate-baseline.txt` carries one `installer_smoke <arm> pass` row per header. It spawns `git`, `npm`, `tar`, `jq`, `sh`, `bash`, `cargo` and `rustc`, the host hasher, and floor utilities. Its unix-only constructs are the symlink farms of the `jq`-less and `bash`-less arms and the `sha256sum` mask, the `#!/usr/bin/env bash` shim masks of the download and toolchain-free arms, the `sh bin/checkwright.sh` entries, the mode-bit checks and the foreign-architecture fallback cases' exec premise.
- **The PowerShell driver** takes six parameters the workflow's Windows pack step fills, drives `bin/checkwright.ps1` alone, prints its own `RUN-SMOKE-PS1:` header grammar, which no parser reads, and covers init, battery, hooks, a one-hop `update` and uninstall.
- **Nested invocations inherit the invoker's environment whole.** No `unset`, `env -u` or `env -i` reaches a nested call. The driver exports `GATE_SDK_NATIVE_TARGETS_FILE` to every child when it steers, and `HOME` is not isolated. An exported knob redirects a nested gate: `EVIDENCE_KIT_MANIFEST_FILE=.tmp/no-such-manifest.md bash gate-sdk/bin/run-gates.sh --only check-evidence-manifest` prints `manifest not found: .tmp/no-such-manifest.md`, so a kit knob in the environment outranks the knob file the scratch consumer's `init` wrote.
- **Both bootstraps print the same refusal phrases**: each of `checkwright.sh` and `checkwright.ps1` carries six lines matching the artifact arm's five phrases (`grep -c`), so the driver's per-bootstrap expectations are one set.
- **The PowerShell bootstrap's detector is three functions**, `Get-HostShape`, `Get-HostTarget` and `Get-FallbackTarget`, and `Get-HostTarget` calls `Get-HostShape`. The POSIX one's is four, `host_arch`, `host_shape`, `target_of_host` and `fallback_of_target`, which `host-target.sh` extracts together.
- **Every CI leg already runs the producer's upload.** Each install-smoke leg places the producer's artifact as the tree's own gate binary before the smoke runs (`.github/workflows/gates.yml`, the *normalize the producer's upload* steps), so its pack call sites dispatch to uploaded bytes. Only `publish.yml`'s `pack:` job builds the binary it packs with.

## What changes

### (1) The driver is the `--installer-smoke` arm {design-bearing} {user-facing: operator direction 2026-10-02, lead-relayed (not a ruling): the driver ships as an `Arm::Run` arm of the shipped gate binary}

An `Arm::Run` member of the arm table, implemented under `native/src/emit/installer_smoke/` as one module per arm family (the preflight and roster, staging and packing, the per-profile loop and its report, the companion legs, the masked arms, the upgrade family, the move, seam, narrowing and selection arms, the artifact arm). Its exit statuses are the bash driver's: 0 clean, 1 a finding about the payload or the tree it wrote, 2 a harness precondition. Its unit-test module pins its argument grammar, the scrub of delta 4, the line reader of delta 6 and the host-spelling table of delta 3.

- **The tree is the current directory's git toplevel**, the tree whose knob files the binary reads, so the tree packed and the tree configuring the run are one by construction. Every spawn the driver makes into that tree names it as its working directory. The `PACK:` line names the root.
- **It reads three environment values undeclared**, `INSTALLER_SMOKE_TMP_DIR`, `INSTALLER_SMOKE_ARTIFACTS_DIR` and `INSTALLER_SMOKE_TARBALL_OUT`, since no static kit owns the prefix, with the meanings installer/SPEC.md §The consumer smoke gives them. It declares `GATE_SDK_NATIVE_TARGETS_FILE`, whose caller-set value decides whether it steers.
- **Its callers** are the five install-smoke legs, the `installer_smoke` validate suite's run command, and a contributor's local run, `bash gate-sdk/bin/run-gates.sh --installer-smoke`. It needs a checkout, as `--run-demo` does, since it packs the tree it runs in.
- **It packs by spawning the running binary's `--pack-installer --root <root>`** with the root as working directory, naming both decisions as installer/SPEC.md §The packer requires of every pack call site, its steered roster and pack scratch in that child's environment, never by an in-process call, so a pack's environment and refusal stay its own.
- **It is a network spawner** (`npm pack`), so it is a bare-flag member outside the fence-safe set, and it joins the crate's network-spawner roster that a unit test holds disjoint from that set.

**Not yet applied.**

### (2) The driver stages the binary it runs as and builds nothing {design-bearing}

The binary running the assertions is now part of the subject, so a mid-run rebuild would pack a binary other than the one judging it. So the `build` arm keeps its header and changes its act. Without the artifact hand-off, it asserts that the running binary's baked source stamp equals the tree's, refusing at exit 2 with the remedy *run `bash gate-sdk/bin/build-native.sh`, then the smoke* when they differ. It then copies the running binary (`std::env::current_exe`) into the host's artifact directory, on a unix host drops its executable mode as the artifact transport does, and emits its digest sidecar with the crate's own hasher. With the hand-off, it stages the handed bytes and sidecar unchanged, as today. Either way the host triple is the host bootstrap's detector's (delta 3), and a running binary built for another triple is refused naming the hand-off as the remedy.

The preflight so loses `cargo`, `rustc` and `jq`. The driver parses the manifest and `package.json` with the crate's JSON reader, so `jq` is needed only where an arm masks it from the verbs. It requires `git`, `npm` and `tar`, plus `sh` on a unix host or `pwsh` on Windows, refusing at exit 2 on any absent one, and the clean tree as today. **Not yet applied.**

### (3) Every arm runs on every leg, through the host's bootstrap {design-bearing} {user-facing: the entry's deliverable, one compiled driver on every leg}

The arms run in the bash driver's order under its 27 header names, byte-identical up to each parenthetical, so the baseline rows hold. Each arm drives **the host's bootstrap**. On a unix host the npm-installed `node_modules/.bin/checkwright` and the extracted package's `sh bin/checkwright.sh` are driven where the bash driver drives them. On native Windows the same packages, the npm-installed one included, are driven through their `bin/checkwright.ps1` under `pwsh -NoProfile -File`, as the PowerShell driver drives it today, so the install arm's `npm install --offline` still runs and its `.bin` entry is not the route under test there. The documented Windows PowerShell route, `powershell -NoProfile -ExecutionPolicy Bypass -File` (docs/install.md §Windows), stays the Windows leg's page-install step's. The host triple comes from the host bootstrap's own detector: `installer/consumer-smoke/host-target.sh` on unix, and on Windows `Get-HostShape`, `Get-HostTarget` and `Get-FallbackTarget` extracted from `bin/checkwright.ps1` through PowerShell's parser and run by `pwsh`, so neither leg holds a mapping of its own.

Per-host spellings, each the same claim:

| construct | unix host | native Windows |
| --- | --- | --- |
| reach mask (download, toolchain-free arms) | a `PATH`-first directory of shims that exit non-zero naming themselves | the directories holding the masked program dropped from `PATH`; the program must then resolve to nothing |
| absence mask (`jq`-less, `bash`-less, `sha256sum`) | a link farm of every program on a directory that holds the masked one, less it | the directories holding the masked program dropped from `PATH`, and any directory holding a program the arm needs re-added alone, as Git's `cmd` for `git` |
| mask proof | the masked program resolves to the shim or to nothing, and a control program resolves and runs | the same |
| executable bit on a printed follow-up target | asserted where no interpreter precedes it | not asserted: the host has no mode bit |
| `PATH` separator | `:` | `;` |

Two cases skip on a stated reason rather than red, printing the reason:

- **The foreign-architecture fallback cases** need a preferred artifact that verifies and cannot start. The driver proves that premise by running the artifact's `--help` first; one that starts, as an x64 build does under emulation on arm64 Windows, is no stand-in, and the three cases skip, as on a host with no such hand-off.
- **The `shasum` fallback** is the POSIX bootstrap's step-4 branch. The PowerShell bootstrap hashes in-process, so on Windows it skips, naming that.

**Inferred, cannot run before build:** that on the Windows runners each masked program's directories hold no program an arm needs beyond the ones it re-adds, and that `pwsh` can extract and run the PowerShell detector's functions — no native Windows host is reachable before the mid-iteration push runs the arm there.

**Not yet applied.**

### (4) Nested invocations run under a scrubbed environment {design-bearing}

Every spawn whose working directory is a scratch consumer, an extracted package or a throwaway copy of either receives the invoking environment **less every variable named under a static kit's prefix**, plus the values the arm sets for that call (a masked `PATH`, `DEMO_TMP_DIR`). The prefix set is derived from the crate's static knob tables, never listed, and a unit test holds it equal to their prefixes. So an exported `EVIDENCE_KIT_*_FILE`, `EVIDENCE_KIT_LOCK_FILE`, `GATE_SDK_TMP_DIR`, `GATE_SDK_NATIVE_BIN`, `GATE_SDK_ROOT` or steered `GATE_SDK_NATIVE_TARGETS_FILE` no longer redirects a scratch consumer's gates, and each nested run reads the knob files its own `init` wrote. A pack spawn runs in the source tree with the invoking environment and its steering, since that tree's configuration is the one being packed. `HOME` and git's global configuration are left as they are: each scratch consumer sets its own identity and maintenance keys locally. **Not yet applied.**

### (5) The log declares the roster the validate parser reads {design-bearing} {user-facing: operator direction 2026-10-02, lead-relayed (not a ruling): the additive `smoke-roster:` log-head form of `--emit parse-smoke-log`, chosen knowing it widens a shipped evidence-kit contract the selected set did not name}

`--emit parse-smoke-log` derives an arm roster from a driver script's `printf` headers, which a compiled driver has none of. It gains a second input form: invoked with the log alone, it reads the roster from the log's own `smoke-roster: <name>` lines, which must precede the first header, in order, the last naming the completion marker. Every other rule of evidence-kit/SPEC.md §Layout and configuration's parser paragraph holds unchanged: a header's name up to its parenthetical, the first reach, the fail-fast attribution, and exit 2 on fewer than two names. The driver prints its roster in that form as its first output, from the same table its headers are printed from, so the two cannot differ. `scripts/evidence-config.knobs` becomes:

```
EVIDENCE_KIT_PARSER_installer_smoke = bash gate-sdk/bin/run-gates.sh --emit parse-smoke-log
EVIDENCE_KIT_RUN_installer_smoke = bash gate-sdk/bin/run-gates.sh --installer-smoke
```

The parser value is then the driver-less form `scripts/gate-tests/evidence-parser-values.test.sh` uses as its negative control D, so that test is rewritten with it. Its fixture smoke log opens with `smoke-roster:` lines, so arm B's configured value still yields the scenarios it baselines. Control D becomes a log carrying no roster lines, which the configured value refuses with no scenario produced, its expected refusal text being the log-only form's rather than the missing positional's. **Not yet applied.**

### (6) The manifest-disagreement report keeps its readings and drops the bash instrument {design-bearing}

installer/SPEC.md §The consumer smoke's failure-path report is restated for a compiled reader. **Not yet applied.**

- **Kept:** the three bounded samples and their dedup on the whole tuple; the coincidence check's three outcomes; `want` and `got` as held operands and `reread`, `own` and `raw` as re-reads; the octet dump beside each value's length; the shape test's verdict and the first offending byte's index; the exit-2 refusal of a malformed operand after the report; the porcelain, log, `core.*` configuration with origins and `check-attr` witnesses; the artifact digest control, computed with the crate's hasher; the no-normalization rule.
- **Retired, with their table rows:** `wantalt`, whose subject, the `jq` line render, the tab and `read`'s split, no longer exists, since the driver parses the lock with the crate's JSON reader; the `printf '%q'` rendering and its row; the two-matcher decomposition of the shape test and its row, whose ground was bash's glob and ERE engines; the whole-line `read` and its tab-collapsing argument; the two raw stream lines.
- **Restated:** the terminator owner becomes one line reader over every child's line-oriented stdout, `git hash-object --stdin-paths` among them. It drops one trailing carriage return per line, counts them and declares the count on the green path as on the red, exactly one per line, so a doubled one still fails the shape test. Its synthetic CRLF self-test becomes a unit test of that reader rather than a silent preflight.

### (7) The callers move to the arm {mechanical}

**Not yet applied.**

- **`.github/workflows/gates.yml`:** the four unix legs run `bash gate-sdk/bin/run-gates.sh --installer-smoke` where they ran the script, with their environment unchanged; the baseline Linux leg keeps its `tee`, status capture and `--diff-baseline installer_smoke`. `install-smoke-pwsh-windows` runs `gate-sdk/bin/run-gates.ps1 --installer-smoke` under `pwsh` with `INSTALLER_SMOKE_TMP_DIR`, `INSTALLER_SMOKE_ARTIFACTS_DIR` and a steered `GATE_SDK_NATIVE_TARGETS_FILE`, as the macOS legs set them. Its pack step's second, next-patch pack and the `PWSH_UPGRADE_*` values only the deleted driver read are removed; its first pack stays for the steps that read `PWSH_PKG`. Its `timeout-minutes` is widened to cover the whole suite and narrowed only after a measured run, on §The consumer smoke's widen-before-narrow rule.
- **`README.md`**'s contributor line names `bash gate-sdk/bin/run-gates.sh --installer-smoke`.
- **`.claude/settings.json`**'s allow entry for the deleted script is replaced by one for the arm. This edit is applied on the operator's behalf, never by the build session, which prepares the diff and routes it to the lead (guard-kit/SPEC.md §compare-settings-allow).
- **`guard-kit/guard-tests/cases.tsv`**'s rows whose specimen command names the script name the arm instead, each row keeping its decision.
- **`native/src/emit/parse_smoke_log.rs` and `native/src/emit/foreign_shells.rs`** test fixtures name a neutral driver file and command.
- **`native/targets.list`**'s comment names the arm as the roster's smoke owner.
- **`installer/consumer-smoke/host-target.sh`**'s header states its own port disposition: it is the unix legs' and the driver's extraction of the POSIX detector, and ships in no payload.

**Inferred, cannot run before build:** the whole suite's duration on the Windows runners, which sets that timeout — only the leg's first run of the arm measures it.

### (8) Both scripts are deleted after a recorded parity run {mechanical}

The batch that deletes `run-smoke.sh` and `run-smoke.ps1` first runs the bash driver and the arm on one Linux tree and records, in its commit message, that both exit 0 and print the same header roster, any differing parenthetical named. The comparison is the evidence, not an arm: a parity arm whose second holder is deleted could only skip (gate-sdk/SPEC.md §The non-gate arm). The PowerShell driver has no Linux run to compare; the Windows leg's first green run of the arm is its parity evidence. **Not yet applied.**

### (9) The owning sections state the compiled driver {design-bearing}

**Not yet applied.**

- **installer/SPEC.md §The consumer smoke:** the opening names the arm; *The port disposition* paragraph is deleted, since a compiled driver owes none; the tree-selection and two-mechanisms paragraphs become delta 1's one sentence; the preflight and *What it costs to run* follow delta 2; *The PowerShell driver carries native Windows* is deleted and replaced by delta 3's host paragraph and table; delta 4 gains a paragraph; the report passages follow delta 6; *Who reads the 2* names the log-declared roster; the cargo, rustc and hand-off paragraph follows delta 2; and the CI paragraphs name the arm. The section's prose otherwise awaits `installer-smoke-brevity`, which passes it as this unit leaves it.
- **installer/SPEC.md §The packer**, the ROUTE 1 paragraph's lead and first two sentences: ***The publishing caller builds the binary it dispatches to: ROUTE 1.** `--installer-smoke` packs with the binary it runs as: a contributor's build, or on a CI leg the producer's upload placed as the tree's gate binary. Those legs publish nothing.* Its `publish.yml` sentence stays, and the refusal paragraph after it keeps binding the job that assembles and stamps the published tarball.
- **gate-sdk/SPEC.md §The non-gate arm**, *The heaviest.* gains the arm's spawn set: `git`, `npm`, `tar`, `sh` or `pwsh` for the host bootstrap, the running binary for each pack, `shasum` as the fallback's control, and whatever the vendored kits' installers and the scratch consumers' batteries spawn. The fence-safe paragraph's roster of the crate's network spawners gains `--installer-smoke` (`npm`).
- **gate-sdk/SPEC.md §git-hook**, the closing sentence's list of parsers of the hook strings drops `run-smoke.ps1`. The `bash`-less arm it names runs in the compiled driver on every leg.
- **gate-sdk/SPEC.md §The program roster** and `native/src/programs.rs`: `sh` and `shasum` join as `contributor` members.
- **evidence-kit/SPEC.md §Layout and configuration**, the parser paragraph gains delta 5's second form, and its installer-smoke sentences say the roster is the log's.
- **context-kit/SPEC.md**'s `jq:1.5::contributor` line names `guard-kit/smoke/install.sh` alone.

## Producers and consumers

- **The arm's verdict and log.** Producer: `--installer-smoke`, on each install-smoke leg on every push, at validate through the suite's run command, and locally. Consumers: each leg's status (the four non-baseline legs), the baseline Linux leg's `--diff-baseline installer_smoke` over the parsed log, `--run-validate`'s evidence row, and the release path's install-page step, which reads the tarball the hand-out writes.
- **The roster lines.** Producer: the arm, first, from its header table. Consumer: `parse-smoke-log`'s log-only form. Each field's reader: the names are the scenario set, and the last is the completion marker.
- **The scrubbed environment.** Producer: the arm, per nested spawn. Consumers: the scratch consumers' verbs, batteries and hooks, which resolve their knobs from their own knob files. Red condition: none new; a nested gate that read a redirected file was a false verdict, now gone.
- **Roster-holding readers of the minted names.** `--installer-smoke` joins the arm table, the binary's printed arm roster, the unit test resolving each row's file to a test module, the network-spawner disjointness test, and gate-sdk/SPEC.md's spawn-set paragraph. `sh` and `shasum` join `programs.rs`. The suite's baseline rows are unchanged, and `kit-log-declaration-transport`'s arm adds one in its own commit.
- **Narrowed corpora (point 5).** The preflight drops `cargo`, `rustc` and `jq`, so no refusal fires for them: a reader asserting a toolchain-free host is unaffected, and nothing counts the preflight's members. `wantalt` and the `%q` and two-matcher rows leave the report, which decides no verdict except through the shape test's refusal, which stays.

## Existing sections updated

Probes: `git grep -l "run-smoke.sh\|run-smoke.ps1\|RUN-SMOKE-PS1"` over the tracked tree (20 files, none under `docs/posts`), and `git grep -n "consumer-smoke/"`, `git grep -n "wantalt\|read_stream"` and `git grep -n "parse-smoke-log"`.

- `installer/SPEC.md` §The consumer smoke and §The packer (deltas 1, 2, 3, 4, 5, 6 and 9).
- `gate-sdk/SPEC.md` §The non-gate arm, §The program roster and §git-hook (delta 9).
- `evidence-kit/SPEC.md` §Layout and configuration (deltas 5 and 9).
- `context-kit/SPEC.md` — the `jq` audience line (delta 9).
- `native/src/emit/installer_smoke/` — new (deltas 1, 2, 3, 4, 5 and 6).
- `native/src/emit/mod.rs` — the arm-table row (delta 1).
- `native/src/emit/parse_smoke_log.rs` — the log-only form and its tests (deltas 5 and 7).
- `native/src/emit/foreign_shells.rs` — the test command (delta 7).
- `native/src/programs.rs` — `sh` and `shasum` (delta 9).
- `native/targets.list` — the comment (delta 7).
- `scripts/evidence-config.knobs` — both values (delta 5).
- `scripts/gate-tests/evidence-parser-values.test.sh` — the roster-bearing fixture log and the rewritten control D (delta 5).
- `.github/workflows/gates.yml` — the five legs (delta 7).
- `README.md` — the contributor line (delta 7).
- `.claude/settings.json` — the allow entry, prepared for the operator (delta 7).
- `guard-kit/guard-tests/cases.tsv` — the specimen commands (delta 7).
- `installer/consumer-smoke/host-target.sh` — its header (delta 7).
- `installer/consumer-smoke/run-smoke.sh` — deleted (delta 8).
- `installer/consumer-smoke/run-smoke.ps1` — deleted (delta 8).
- `.workflow/release-declarations.md` — the evidence-kit parser form (delta 5).
- `docs/installer/SPEC.md` — the generated mirror (all deltas).
- `docs/gate-sdk/SPEC.md` — the generated mirror (delta 9).
- `docs/evidence-kit/SPEC.md` — the generated mirror (deltas 5 and 9).
- `docs/context-kit/SPEC.md` — the generated mirror (delta 9).
- `docs/check-graph.html` — regenerated for the deleted scripts' node (delta 8).

## Retired spellings

- `run-smoke.sh` — the bash driver (deltas 7, 8 and 9).
- `run-smoke.ps1` — the PowerShell driver (deltas 7, 8 and 9).
- `RUN-SMOKE-PS1` — the PowerShell driver's verdict prefix (delta 8).
- `wantalt` — the report's `jq`-channel control (delta 6).
- `read_stream` — the bash terminator owner, restated as the driver's line reader (delta 6).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — the workflow steps and README line carry commands, not grounds.
- [ ] **Merged with no information lost** — §The consumer smoke reads as one section for the compiled driver.
- [ ] **Amendment deleted** — this file removed on merge; no root-level amendment of this unit remains (`ls SPEC-*.md`).
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green over the declarations above.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Parity recorded** — delta 8's comparison is in the deleting commit's message.
- [ ] **The entry is done** — `compiled-consumer-smoke-driver` moves to Done when every install-smoke leg has run the arm green on the mid-iteration push, by build's remote-oracle rule, before the drain stage. A held platform's leg reports rather than binds, as §The consumer smoke rules.
