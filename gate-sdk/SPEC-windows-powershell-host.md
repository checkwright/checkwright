# SPEC amendment: windows-powershell-host

gate-sdk/SPEC.md §run-gates says the front-end's PowerShell twin runs under
Windows PowerShell 5.1 and PowerShell 7. The oracle holding the twin,
`--run-front-end-parity`, spawns only `pwsh`, which is PowerShell 7. So the 5.1
half of the claim has never run. This amendment keeps the claim and makes the
oracle run it: on a Windows host the arm compares the bash stub against the twin
under both PowerShell hosts. Narrowing the claim to PowerShell 7 was the other
option, and it is refused because Windows PowerShell 5.1 is the only PowerShell
a stock Windows host ships. A native-Windows adopter who has installed nothing
has 5.1 and no 7. The installer amendment `installer/SPEC-windows-follow-up.md`
has `init` print a `powershell` command for exactly that adopter, so it depends
on this amendment landing first.

**Probed at authoring, 2026-09-18, at `59810bb3`:**

- `native/src/emit/front_end_parity.rs` builds each case's twin transcript from
  `programs::PWSH` alone (`grep -n PWSH native/src/emit/front_end_parity.rs`
  prints its lines 228 and 263), and refuses with exit 2 when `bash` or
  `pwsh` does not resolve.
- `programs::PWSH` is the roster's one PowerShell member (`"pwsh",
  "contributor"`, `native/src/programs.rs:64`). gate-sdk/SPEC.md §The program
  roster states that only this arm spawns it.
- The arm runs at two sites (`grep -n run-front-end-parity
  .github/workflows/gates.yml` prints lines 77 and 477). The first is the Linux
  `gates` job, whose Ubuntu image carries `pwsh` and cannot carry Windows
  PowerShell. The
  second is the binding `install-smoke-windows` leg.
- `gate-sdk/bin/run-gates.ps1` already carries a 5.1-specific construct: its
  line 113 spells a dash by code point because 5.1 reads a BOM-less script in
  the ANSI code page. So the twin was written for 5.1, and only the run is
  missing.
- This host has no PowerShell. Nothing in this amendment can run here, and its
  oracle is the Windows leg.

## What changes

### (1) The parity arm runs the twin under every PowerShell host the platform ships {design-bearing}

The roster gains `programs::POWERSHELL = "powershell", "contributor"`, the
Windows PowerShell 5.1 host. The arm's host set depends on the platform:

- **On a Windows host** it runs every case under both `pwsh` and `powershell`.
  If either does not resolve, the arm exits 2 (could not run), as it already
  does for a missing `pwsh`. A Windows host with no 5.1 is not one the claim
  covers, and a check that skipped it would pass without running.
- **On any other host** it runs `pwsh` only. Windows PowerShell does not exist
  off Windows, so there is nothing to skip. The Linux `gates` job keeps exactly
  the coverage it has.

Each case compares the bash transcript with every twin transcript, using the
existing comparison and CRLF normalization. A divergence prints the case, the
host that diverged (`pwsh` or `powershell`), both transcripts and the first
differing line. The clean line names both hosts on a Windows run, so the log
shows that 5.1 ran. Both hosts get the same argv prefix,
`-NoProfile -NonInteractive -File <twin>`. The test uses `cfg!(windows)` rather
than `#[cfg(windows)]` so the new row stays referenced on every target. Without
that, the crate's deny-warnings lint would red a Linux build over a row nothing
spawns.

A divergence the first 5.1 run finds is a defect in the twin, and build fixes it
in the twin. The SPEC already claims 5.1, so a 5.1 construct fix stays inside
this amendment. Narrowing the claim does not.

**Inferred, cannot run before build:** the twin is byte-identical to the stub under Windows PowerShell 5.1 on `windows-latest` — only the leg's first run of this delta settles it, and a divergence found there is fixed in the twin in the same unit.

**Not yet applied.** In gate-sdk/SPEC.md §run-gates, replace the sentence
opening "It exits 2 when the check could not run:" and the sentence after it
("It runs on the binding Windows install-smoke leg …") with:

> It runs the twin under every PowerShell host the platform ships: `pwsh` and
> Windows PowerShell 5.1 (`powershell`) on a Windows host, and `pwsh` alone
> elsewhere, since 5.1 exists only on Windows. It exits 2 when the check could
> not run: `bash` or one of those hosts does not resolve, the sandbox cannot be
> built, or the bash stub itself does not do what a case names, so two halves
> failing the same way cannot pass as parity. It runs on the binding Windows
> install-smoke leg, the one site that measures 5.1, and on the `gates` job,
> never in the battery, whose host may carry no PowerShell.

**Not yet applied.** In gate-sdk/SPEC.md §The program roster, the closing
sentence about `pwsh` becomes: "`pwsh` and `powershell` are `contributor`
members: only `--run-front-end-parity` spawns them (§run-gates), on CI legs, so
no adopter host needs either and the floor-or-probe relation does not bind
them."

## Producers and consumers

- **`programs::POWERSHELL`.** The producer is the roster row. Its one consumer is
  the parity arm's Windows host set. The roster's unit tests read every member's
  audience. As a `contributor` member it is on neither
  `GATE_SDK_PROGRAM_FLOOR` nor `PROBE_SET`, which is the relation `PWSH`
  already satisfies. No floor, `doctor` or env-probe reader sees it
  (`grep -n 'fn probed' -A3 native/src/programs.rs` lists the eight probed
  members, and neither PowerShell host is among them).
- **The per-host divergence label.** The only reader is the CI log reader. No
  program parses the arm's stdout: both call sites read only the exit status
  (`gates.yml` lines 77 and 477).
- **Exit 2 on a missing `powershell` on Windows.** The binding leg's step
  status reads it. The leg already runs on a host that ships 5.1.

## Existing sections updated

- gate-sdk/SPEC.md §run-gates: the host-class sentence and the parity
  paragraph's exit-2 and run-site sentences (delta 1).
- gate-sdk/SPEC.md §The program roster: the `pwsh` closing sentence (delta 1).
- `native/src/emit/front_end_parity.rs` and `native/src/programs.rs` (delta 1).
- `.github/workflows/gates.yml`: the comment above the Windows leg's parity
  step (line 471) gains a clause saying that step is the one run measuring 5.1
  (delta 1).
- `docs/gate-sdk/SPEC.md`: the generated mirror, regenerated (all deltas).

## Retired spellings

- None — no delta of this amendment retires a spelling; the claim is kept and the oracle widened.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for the new roster row and the host set.
- [ ] **Instruction surfaces: instruction only** — no template text changes.
- [ ] **Merged with no information lost** — the §run-gates sentences re-phrased,
      not appended to.
- [ ] **The Windows leg ran the twin under 5.1 green** — the leg's parity step
      log carries a clean line naming both hosts.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      gate-sdk (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — nothing retired.
- [ ] **Gaps filed** — any 5.1 construct the run finds outside the twin is filed.
