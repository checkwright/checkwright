# SPEC amendment: windows-hook

guard-kit's `PreToolUse(Bash)` hook is bash and jq, and guard-kit/SPEC.md says
nothing about native Windows. This amendment rules that Git for Windows' bundled
bash serves the hook there, and that the kit ships no second implementation. It
gives that claim an oracle, which is the guard's decision table run under Git
Bash on the Windows leg. It also states one honest limit: on Windows the harness
carries a second shell tool, `PowerShell`, and a `Bash`-matched hook never sees
its calls. A guard over PowerShell-grammar commands is filed as a Deferred entry
rather than designed here.

The substrate and the limit follow operator direction, 2026-09-18, relayed by the
lead: substrate (i) was taken, and the PowerShell-tool bypass is to be stated as
an honest limit with a costed Deferred entry. That stamp is provenance for this
file and the queue entry only. The SPEC text below states the rule and its
engineering grounds undated.

**Probed at authoring, 2026-09-18, at `790d9eb0`:**

- `grep -n -i windows guard-kit/SPEC.md` prints nothing.
- `guard-kit/templates/settings-hooks.json` wires three hook commands. All
  three are spelled `bash …`: `bash scripts/bash-guard.sh` and two
  `bash gate-sdk/bin/run-gates.sh --hook …` arms.
- The harness vendor's hooks guide (code.claude.com/docs/en/hooks-guide) says a
  command hook without `args` is spawned with "`sh -c` on macOS and Linux, Git
  Bash on Windows, or PowerShell when Git Bash isn't installed". The tools
  reference (code.claude.com/docs/en/tools-reference) says "there is a separate
  PowerShell tool with the tool name `PowerShell`". On Windows with Git Bash it
  is "on by default", `CLAUDE_CODE_USE_POWERSHELL_TOOL=0` turns it off, and the
  docs say to "Match `Bash|PowerShell` in hooks that inspect shell commands".
  Both pages were fetched in this session.
- The guard decision table runs only on the Linux `gates` job
  (`grep -n run-guard-tests .github/workflows/gates.yml` prints line 71).
- Git for Windows ships no `jq` (run 35390249229's Windows probe resolves `jq`
  from Chocolatey). `SPEC-remedy-persistence.md` puts `jq` into the Windows
  remedy block, and that is the floor this hook's jq use rests on.
- The generated git hooks already take this shape: gate-sdk/SPEC.md
  §gen-pre-commit rules them one bash implementation that git runs under Git for
  Windows' own shell, and a MinGit host is not claimed.

## What changes

### (1) A new guard-kit/SPEC.md section, §The hook on native Windows {design-bearing}

**Not yet applied.** Insert after §Consumer rules:

> ## The hook on native Windows
>
> On native Windows the hook runs under Git for Windows' bundled bash, and the
> kit ships no second implementation. The ground is what the hook guards. It
> inspects calls to the harness's `Bash` tool, and the harness runs that tool
> only through Git Bash, so any session that has a call to guard already has
> the shell the guard needs. The harness also runs a command hook under Git Bash
> when it is installed. So the committed wiring (`templates/settings-hooks.json`)
> is the same on every host, which is what a settings file shared across a
> team's machines requires. A host-keyed hook command could not be committed at
> all. The ruleset models bash quoting, heredocs and the harness's per-segment
> matching of bash commands (§The generic ruleset). A PowerShell twin would
> re-implement a bash reader in PowerShell to guard bash commands, and it would
> double every consumer rule on a seam whose interface is shell functions
> (§Consumer rules). A native hook front would port the ruleset and delete that
> extension point, which the library's permanent-shell ground refuses (§The
> guard framework). Both are refused.
>
> The hook's floor on that host is the install page's: Git for Windows' bash
> plus `jq`, which Git for Windows does not ship. A host carrying MinGit or no
> Git Bash is not claimed. On such a host the harness runs hook commands under
> PowerShell, where a bare `bash` reaches the system directory's WSL launcher.
> `checkwright doctor` refuses that host before any hook is wired.
>
> **The honest limit: the harness's `PowerShell` tool is not guarded.** On
> Windows the harness carries a second shell tool, named `PowerShell`, which is
> on by default beside `Bash`. A `Bash`-matched hook never sees its calls, and
> the ruleset could not read them anyway, because every rule is a claim about
> bash grammar. So on a Windows host, a command the agent sends through that
> tool reaches the harness's own permission path unguarded and unlogged, and the
> friction loop does not measure it. Widening the matcher to `Bash|PowerShell`
> is refused, because it would run bash-grammar rules over PowerShell commands.
> A guard over PowerShell-grammar commands is separate work, not this kit's
> today. A consumer who wants every shell call guarded turns the tool off with
> the harness's own switch (`CLAUDE_CODE_USE_POWERSHELL_TOOL=0`). That is the
> consumer's call, and the kit does not recommend it.
>
> **The oracle** is the decision table (§Testing) run under Git Bash on the
> binding native-Windows install-smoke leg. Every generic rule's firing and
> non-firing case then executes on the substrate this section claims.

**Verified at align, 2026-09-19, at `902e4e1e`:** no Windows host is reachable from this session, so the named run could not be made; the implementing lines are read instead. `bash` is `SYSTEM_DIR_HOMONYMS`'s one `Refuse` member (`native/src/proc.rs`), so `resolve_floor_tool` (`#[cfg(windows)]`, same file) never accepts a `bash` reachable only through the system directory and falls back to the unresolved bare name; `doctor.rs::probe_banner` still finds it "on path" first through the naive, non-skipping `proc::on_path`, so it spawns that unresolved name, which the OS's own search still lands on the system directory's WSL-launcher stub. On a host with no Git Bash that stub produces no bash version banner, so `toolfloor::check` reads `Verdict::Uncomparable`, and `doctor.rs::render_member` counts that toward `failed` exactly as it counts `Absent` — printing "could not be compared against the floor" rather than "NOT FOUND", but reaching the same `DOCTOR: below contract` exit the section claims.

### (2) The Windows leg runs the decision table {design-bearing}

`install-smoke-windows` gains the step
`bash gate-sdk/bin/run-gates.sh --run-guard-tests` (shell `bash`). It goes after
the step that places the producer's artifact as this tree's gate binary, beside
the front-end parity step. It is binding like the rest of the leg. Its header
comment cites §The hook on native Windows as the claim it measures.

A row that diverges only on this host is a defect in the guard or its library
under Git Bash. Build fixes it in `lib/guard.sh` or the template if the fix
stays substrate-neutral. If the fix would need a Windows branch in a rule, it is
filed and escalated, not patched.

**Inferred, cannot run before build:** every row of `guard-tests/cases.tsv` and the background and escalation tables classifies identically under Git Bash on `windows-latest` — the leg's first run of this step.

### (3) The wiring template and the install page state the substrate {mechanical}

**Not yet applied.** `guard-kit/templates/settings-hooks.json`'s `"//"` note
gains one sentence: "On native Windows the harness runs these commands under Git
for Windows' bash, so the same wiring serves every host; the `PowerShell` tool's
calls are not guarded (SPEC §The hook on native Windows)."

**Not yet applied.** In docs/install.md §Requirements' `bash` bullet, "Git for
Windows' bundled bash is what serves guard-kit's hook there." becomes "Git for
Windows' bundled bash serves guard-kit's hook there, because the harness runs
both its `Bash` tool and its hook commands under that shell."

## Producers and consumers

- **The section's claim.** The producer is this delta's text. The consumers are
  an adopter wiring the hook on Windows, through the template note and the
  install page, and delta 2's step, which is the claim's oracle.
- **The new CI step.** The producer is the `install-smoke-windows` job. Its
  exit status is read by the job's conclusion. The runner already has the
  binary the arm needs (the normalize step places it at `gate_native_bin`) and
  `jq` (probe above). The evidence-kit suite that §Testing says reads the
  arm's verdict runs on the validate host, not on this leg, so it is
  unaffected.
- **The honest limit.** It has no programmatic reader. The Deferred entry
  `guard-powershell-tool-unguarded` carries its cost.

## Existing sections updated

- guard-kit/SPEC.md: the new §The hook on native Windows (delta 1). The
  section-level reader rosters that list guard-kit headings pick it up through
  the generated mirror.
- `.github/workflows/gates.yml` `install-smoke-windows`: the new step (delta 2).
- `guard-kit/templates/settings-hooks.json`: the `"//"` note (delta 3).
- docs/install.md §Requirements: the `bash` bullet (delta 3).
- `docs/guard-kit/SPEC.md`: the generated mirror, regenerated (all deltas).

## Retired spellings

- None — no delta retires a spelling; the wiring's commands are kept as they are on every host.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for the section's claim and its oracle.
- [ ] **Instruction surfaces: instruction only** — the template note states the
      substrate and cites the section for the grounds.
- [ ] **Merged with no information lost** — the new section carries the grounds
      undated. No operator stamp crosses into the SPEC.
- [ ] **The Windows leg's decision-table step green.**
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      guard-kit (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — nothing retired.
- [ ] **Gaps filed** — a Git-Bash-only divergence that needs a Windows branch is
      filed, not patched.
