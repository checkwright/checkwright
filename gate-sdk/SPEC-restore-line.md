# SPEC amendment: restore-line

Pairs `restore-line-fidelity`. The restore line is the command `--install-hooks`' receipt and `uninstall` print for an adopter to set `core.hooksPath` back to what it was before the opt-in (§install-hooks, *The restore is printed, never written*). It is read off the chain record, and it promises that a recorded value pastes as itself. This amendment closes the ways it can advise a value other than the one the clone had: a record that cannot be read, a value the record cannot tell from no value, and a value the PowerShell form may not carry through Windows PowerShell 5.1.

**What the tree does today**, probed at spec on git 2.55 in a scratch repository and read off `native/src/`:

- **An unreadable record is advised away.** `hooks_line` (`native/src/installer/uninstall.rs`) reads the record with `read_chain(d).ok().flatten()`, so any read error becomes no record, and the line printed is `git config --unset core.hooksPath`. `uninstall` then removes the hooks directory and the record with it, so the value it held is gone. The placed hooks themselves refuse on the same record (§git-hook), and `--install-hooks`' re-run and `--refresh` propagate the error, so `uninstall` is the one reader that reads past it.
- **An empty value is a third state.** With a local `core.hooksPath` set to the empty string, `git rev-parse --git-path hooks` answers `./` and a commit runs no hook from git's default directory; unset, the same commit runs it. The record's `restore=` line writes the empty value and no value alike as an empty field, so the restore line for either is `git config --unset core.hooksPath`, which turns the default directory's hooks on. `prior` (`native/src/emit/install_hooks.rs`) reads an empty effective value as unset too, so the opt-in chains the default directory's hooks, which git was not running.
- **The PowerShell form single-quotes for PowerShell's parser**: `shell_quoted` (`native/src/emit/hook_launcher.rs`) doubles each single quote, typographic ones included, and passes every other character through.

**Inferred, cannot run before build:** Windows PowerShell 5.1's legacy native-argument passing hands git a value carrying a double quote without it, `a"b` arriving as `ab`, and hands a value that holds whitespace and ends in a backslash on with a double quote in place of that backslash, where PowerShell 7.3 and later pass both intact — inferred off Microsoft's about_parsing; no host here runs Windows PowerShell, and the probe step delta 3 adds is the subject, read off the mid-iteration push's Windows run

## What changes

### (1) `uninstall` reports an unreadable chain record rather than advising it away {mechanical} {user-facing: the entry's deliverable, "report the unread chain", in the report-and-proceed form, oracle decision 2026-10-25, lead-relayed (not a ruling): the removal runs, so the install stays reversible}

`hooks_line` returns the `read_chain` error instead of folding it into no record. `uninstall` then runs its removal as today, and in place of a restore command it prints the record's own sentence (`cannot read the chain record <path> (<cause>)`) and says the prior `core.hooksPath` is unknown, so it advises no command: the adopter sets `core.hooksPath` back, or unsets it, themselves. It prints no `git config --unset core.hooksPath`, the line that turned an unknown value into a claim of none. `--dry-run` prints the same report in its plan. The status is unchanged: the removal it trails has run, as a cache deletion that fails is reported and refuses nothing (installer/SPEC.md §uninstall). Only a clone whose `core.hooksPath` names the hooks directory reads the record, as today. **Not yet applied.**

### (2) An empty `core.hooksPath` is refused at the opt-in {design-bearing} {user-facing: oracle decision 2026-10-25, lead-relayed (not a ruling): refuse an empty effective value, the refusal naming its remedy}

A bare `--install-hooks` run refuses at exit 2, placing nothing, an effective `core.hooksPath` set to the empty string, beside the line-break refusal the chain record bullet already states. `prior` tells a value that is set and empty, `git config --get` exiting 0 with an empty value, from one that is unset, exit 1. The refusal names the value as empty and says git finds no hook under it. Its `help:` is the remedy: unset it with `git config --unset core.hooksPath` at the scope that set it, or set the directory meant, then re-run. So the empty `restore=` field keeps its one meaning, no local value, and the prior directory is never misread as git's default. The record grammar is unchanged, and a record written before this change over an empty value stays as it is, the honest limit below. **Not yet applied.**

### (3) The PowerShell restore form, against Windows PowerShell 5.1 {design-bearing} {user-facing: oracle decision 2026-10-25, lead-relayed (not a ruling): probe on the granted mid-iteration push, then refuse the value classes 5.1 alters; refusing over a stop-parsing spelling is this stage's calibration}

**The probe, built first.** A step pair in the `install-smoke-pwsh-windows` job of `.github/workflows/gates.yml`, *the restore line pastes as itself under Windows PowerShell 5.1*, one under `shell: powershell` and one under `shell: pwsh`. Each runs, in a scratch repository, the restore command for three values, `a"b`, `C:\probe dir\` and the control `a b'c`, spelled as `shell_quoted`'s PowerShell form yields them. A crate unit test holds `shell_quoted` to those same three spellings, so the step pastes what the binary prints without the binary having to resolve a directory the value names. After each command the step reads the value back with `git config --local --get core.hooksPath` and prints the value set, the host and what came back. The mid-iteration push runs it, and the act below is chosen on its output. Applied: the step pair reports and asserts nothing until the act lands.

**The act, by the probe's result.** Each **Not yet applied.**

- **Where Windows PowerShell 5.1 returns a value other than the one set**, as inferred above: on native Windows a bare run refuses at exit 2, placing nothing, a local value of each class the probe showed altered, read before the record is written. The refusal names the value and the class. Its `help:` says to respell it: Windows admits no double quote in a file name, so such a value names no directory there, and a trailing backslash names the same directory without it. The step stays as the witness, asserting each refused class refused and the control value read back unchanged under both hosts.
- **Where every value comes back unchanged under both hosts**: no code changes, and the inferred claim goes from the SPEC's honest limits. The step stays as the witness, asserting all three values read back unchanged.

Off Windows the POSIX form is unchanged, and no value is refused on these grounds. **Refused rather than respelled:** a stop-parsing (`--%`) spelling would pass the raw text through both hosts, but it expands `%NAME%` environment references and quotes by the C runtime's rules, a second dialect for one value class that no Windows directory name needs.

### (4) The SPEC sections {design-bearing}

Each **Not yet applied.**

- gate-sdk/SPEC.md §install-hooks, *The chain record* bullet: the line-break refusal becomes one sentence naming every value the record or its restore line cannot carry: a line break, an effective value set to the empty string and, on native Windows, the classes delta 3's probe showed altered. Each refuses at exit 2 before anything is placed. The empty value's ground is stated with it: git finds no hook under it, which no field of the record can tell from no value (deltas 2 and 3).
- gate-sdk/SPEC.md §install-hooks, *The restore is printed, never written*: where the record cannot be read, `uninstall` reports it and that the prior value is unknown, advising no command (delta 1). The PowerShell sentence states what the probe measured, on the Windows leg step it names (delta 3).
- gate-sdk/SPEC.md §install-hooks, *Honest limits*: a record written before the empty-value refusal over an empty value restores as no value (delta 2).
- gate-sdk/SPEC.md §install-hooks, the prior hooks directory's unit cases: an empty effective value refused with nothing placed (delta 2); the `shell_quoted` spellings of the three probe values (delta 3); in `uninstall`'s module, an unreadable record reported with no restore command (delta 1).
- installer/SPEC.md §uninstall, *The placed hooks are removed*: a record that cannot be read is reported with the prior value unknown and no command advised, the removal running as for any other clone (delta 1).
- The release declaration surface gains a `behavior` bullet: `--install-hooks` refuses an empty `core.hooksPath`, and on native Windows the classes the probe showed altered; `uninstall` reports an unreadable chain record in place of a restore line (deltas 1, 2 and 3).

## Producers and consumers

- **The unread-record report** (delta 1). Producer: `read_chain`'s error, at `uninstall` on a clone opted in. Its trigger: the record exists and cannot be read or carries no `dir=` line. Consumers: the adopter, through stdout beside the removal report; the unit test. The status does not move, so no caller of `uninstall` changes. The two other `read_chain` callers already refuse, so no reader changes there.
- **The empty-value refusal** (delta 2). Producer: `prior`, at a bare run, before `record_text`. Red condition: an effective value set and empty. Consumers: the adopter; the unit test; the receipt's `Disable with:` line and `uninstall`'s line, which no longer meet the value. The `native-git-hooks.test.sh` cases set no empty value, so none newly refuses.
- **The probe step and the Windows refusal** (delta 3). Producer of the measurement: the Windows leg on the mid-iteration push. Producer of the refusal, where it lands: a bare run on native Windows. Consumers: the build session choosing the act; the SPEC sentence of delta 4; the adopter. The step's three values each have a reader: the two probe classes decide the act, and the control holds the form that already works.
- **The push.** The probe needs the mid-iteration push the entry's push need names, and it is the only push this unit adds. The act lands after that run is read, held locally by the crate's unit tests: the refusal's predicate is host-independent, as the Windows chain refusal's lines already are. The closing push's run re-runs the step as the landed act's witness, so a step assertion that run reds is close's to repair, not a further push of this unit's.

## Existing sections updated

- `native/src/installer/uninstall.rs`, `hooks_line` and its caller, and its unit tests (delta 1).
- `native/src/emit/install_hooks.rs`, `prior` and the refusals ahead of `record_text`, and its unit tests (deltas 2 and 3).
- `.workflow/release-declarations.md`, a `behavior` bullet (delta 4).
- `native/src/emit/hook_launcher.rs`, the `shell_quoted` unit test holding the three probe spellings (delta 3).
- `.github/workflows/gates.yml`, the `install-smoke-pwsh-windows` job's new step pair (delta 3).
- `gate-sdk/SPEC.md` §install-hooks (deltas 1, 2, 3 and 4).
- `installer/SPEC.md` §uninstall (deltas 1 and 4).
- `docs/gate-sdk/SPEC.md` and `docs/installer/SPEC.md`, the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).

The record's readers were found by `grep -rn "read_chain\|restore_line\|hooks_line" native/src`, the spec surfaces by `git grep -n -i "unset core.hooksPath\|Disable with"`.

## Retired spellings

- None — no delta retires a name; the refusals and the probe step are new.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The local oracle** — `bash gate-sdk/bin/build-native.sh`, the battery, the crate's tests and gate-sdk's fixture suite green.
- [ ] **The remote oracle** — the mid-iteration push's `install-smoke-pwsh-windows` legs run delta 3's step pair, and its output is read before the act lands. The Done move lands before the drain stage, once the act has landed green on the local oracle, never in the commit that lands the probe.
