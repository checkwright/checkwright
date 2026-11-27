# SPEC amendment: child-locale

Pairs `parsed-output-c-locale`. Every child the crate builds runs in the invoking locale today, so a localized host can hand a gate git's answer in a wording no matcher expects. The binary pins `LC_ALL=C` on each child it runs as a roster tool, the class whose output it reads, and leaves the invoking locale to three kinds of child: a program the consumer names, a roster interpreter, and the binary's own image. The rule is keyed on the program's identity, never on the face that runs it.

**What the tree does today**, each bullet with the probe that settled it:

- `proc::command` builds every child with the inherited environment and adds only the test build's git isolation (`grep -n 'fn command' -A5 native/src/proc.rs`). Every face but the wait probe's exempt `exec` reaches it (`grep -n 'command(' native/src/proc.rs`).
- One shipped site sets a locale on a child: canon-kit's fence run, whose fixed isolated environment already carries `LC_ALL=C` (`native/src/gates/fence_run.rs`; canon-kit/SPEC.md §check-fence-run). Its child is `bash`, so under this rule its own pair keeps it there. The other hits are test scope or comments (`git grep -n 'LC_ALL\|"LANG' -- native/src`).
- `Program` already tells the classes apart. A roster member is `Identity::Member`, and every program named outside the crate, a knob's command, a gate declaration or a chained hook, is `Identity::Consumer` through `Program::consumer` (`grep -n 'enum Identity' -A12 native/src/programs.rs`; `git grep -n 'Program::consumer' -- native/src`). The battery's shell gates are consumer programs under `GATE_DECLARATION` (`grep -n 'GATE_DECLARATION' native/src/registry.rs`).
- The whole battery under `LC_ALL=C`, a superset of any per-child pin, is 167 of 168 green on this host. The red is check-docs-render-fidelity at exit 3: its consumer-named renderer (`SITE_KIT_RENDERER_BATCH`, a Ruby kramdown batch) dies with `invalid byte sequence in US-ASCII`, since Ruby takes its default encoding from the locale. Every kit's fixture suite and `scripts/gate-tests` under the same superset is clean but site-kit's, red for the same renderer (`LC_ALL=C bash gate-sdk/bin/run-gates.sh --run`; `--run-gate-tests <kit>/gate-tests <kit>/checks` per kit, this host, git 2.56.1, `LANG=C.UTF-8`).
- *Inferred:* GNU gettext ignores `LANGUAGE` when the locale is `C`, and Git for Windows' gettext honours `LC_ALL`. This host has no translated git catalog, so no localized wording was observed.

## What changes

### (1) A roster tool's child runs in the C locale {design-bearing} {user-facing: a roster tool the binary runs, git first, now answers in the C locale whatever the invoking one; its messages relayed to a person read in English, and a character range or class it matches is byte-wise (adopter direction 2026-11-27, lead-relayed; the identity rule and the consumer's locale left alone a lead decision of 2026-11-27 under the operator's delegation)}

**Not yet applied** — gate-sdk/SPEC.md §Fail-closed contract, a paragraph and its grounds after the **Widening `proc.rs`** sentence, before `Completed::failure_report`:

> **A child the crate parses runs in the C locale.** `proc::command`, the one constructor every face calls, sets `LC_ALL=C` on a child whose program is a roster member, the class whose output the crate reads: git's records and its stderr wording, a tool's message. Three kinds of child keep the invoking locale. A **consumer program**, any `Program::consumer`, is a gate or a command its author owns, and the crate reads it only through a contract that author keeps. A **roster interpreter**, `bash`, `sh`, `dash`, `pwsh` or `powershell`, runs a script, and the script's author owns its locale. The **binary's own image** prints no localized text and applies the rule to each child it builds. The pin is set before a face applies its caller's environment, so a caller's own `LC_ALL` or unset list wins. The isolated face re-applies it after clearing the environment, beside the git isolation.
>
> - **Why by identity and not by face.** A face says how a child's output is read, not whose program it is. A capture rule would pin the consumer's renderer that check-docs-render-fidelity reads, which takes its encoding from the locale and fails under C. It would also judge a consumer gate in the battery in another locale than the same gate run by hand. A roster tool whose output is only relayed, git under a forwarding hook say, takes the pin too, since one predicate the type carries costs English text and never a verdict.
> - **Why the consumer keeps its locale.** The consumer's interpreters read UTF-8 by the locale, Ruby, Python and a `grep` class among them, so a pin would break the gate rather than steady it. The runner reads a gate only through the output contract, the clean line and the records, which no locale changes.
> - **Why no knob.** The pin is what makes a parsed answer mean one thing on every host, a correctness floor rather than a calibration, so no consumer value would be a policy.
> - **A sandbox case holds it end to end.** `gate-tests/child-locale.test.sh` exports a non-C `LC_ALL` and puts a recording `git` first on `PATH`: a native gate's git records `C`, and a consumer shell member prints the exported value.
> - **Honest limit.** A consumer's shell gate that parses git spawns that git itself, outside the crate, in the invoking locale; its script sets the locale it parses in. So does a script a roster interpreter runs for the crate.

`native/src/programs.rs`: an interpreter mark on the roster's member rows, read by one predicate on `Program` that answers whether the child takes the pin: a member that is neither an interpreter nor the binary's image. `native/src/proc.rs`: `command` takes the `Program` beside its spawn target, and every face passes it.

### (2) The witnesses and the declaration {mechanical}

- **Unit tests** in `native/src/proc.rs`, over `Command::get_envs` on the built command: a roster tool carries `LC_ALL=C`; a consumer program, an interpreter and the binary's image carry none; a caller's own `LC_ALL` replaces the pin; the isolated face carries it after its clear.
- **A sandbox case**, `gate-sdk/gate-tests/child-locale.test.sh`, POSIX-only on the hermetic harness: with a non-C `LC_ALL` exported and a `git` wrapper first on `PATH` that records its `LC_ALL` and execs the real git, a native gate's spawn records `C`, and a consumer shell member printing its `LC_ALL` prints the exported value.
- `.workflow/release-declarations.md`: *Behavior changes* gains the pin, a roster tool's messages now in English and its matching byte-wise, the consumer's programs unchanged.

## Producers and consumers

- **The pin.** Producer: `proc::command`, for every face. Consumers: each roster tool's child, which reads `LC_ALL` as setlocale and gettext do; the crate's parsers of that child's output, which receive one wording on every host.
- **The interpreter mark.** Producer: the roster rows in `native/src/programs.rs`. Consumer: the pin predicate alone.
- **Point 2, roster-holding readers.** The roster's existing test that every member carries one audience reads its rows, and the mark adds a field it does not read. No knob roster gains a row.
- **Point 5, narrowing.** The pin narrows the environment a roster child sees. Red condition: a crate parser matching a localized wording would stop matching; the probe over the crate found no locale-dependent matcher, and the whole-battery superset run is green but for the consumer renderer this rule leaves alone.

## Existing sections updated

Roster probe: `git grep -n 'invoking environment\|LC_ALL' -- gate-sdk/SPEC.md canon-kit/SPEC.md native/src`, and `grep -n 'command(' native/src/proc.rs`, over the tracked tree.

- `gate-sdk/SPEC.md` — §Fail-closed contract's paragraph and grounds (delta 1), and its sandbox-case bullet (delta 2). The passages carrying the invoking environment unchanged, at the hermetic harness and the `--with-foreign-shells` arm, speak of the binary's own process, which keeps the locale, and stay as they are.
- `native/src/proc.rs` — `command`, every face's call, the isolated re-pin (delta 1), and the unit tests (delta 2).
- `native/src/programs.rs` — the interpreter mark and the predicate (delta 1).
- `gate-sdk/gate-tests/child-locale.test.sh` — the sandbox case (delta 2).
- `.workflow/release-declarations.md` — the behavior change (delta 2).
- `docs/gate-sdk/SPEC.md` — generated mirror (all deltas).
- `docs/llms-full.txt` and `docs/requirements.md`'s install-size block — generated, each regenerated by the command its freshness gate prints on red (all deltas).

## Retired spellings

- None — no delta of this amendment retires a spelling; `command` keeps its name and gains a parameter.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the pin and the interpreter mark.
- [ ] **Instruction surfaces: instruction only** — no template or shim gains text.
- [ ] **Merged with no information lost** — the paragraph joins §Fail-closed contract as one passage beside the funnel it rides.
- [ ] **Amendment deleted** — this file removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` holds.
- [ ] **Every reached suite run** — `proc.rs` is shared by every kit's gates, so the battery and every suite README.md §This repo, governed rosters runs green: each kit's fixture suite, `scripts/gate-tests`, `--run-guard-tests`, `--run-index-tests`, the crate's unit tests and the smokes.
- [ ] **Release declared** — `.workflow/release-declarations.md` carries the behavior change.
- [ ] **Entry moved** — `parsed-output-c-locale` moves to Done before the drain stage, in the commit that lands the merge.
- [ ] **Gaps filed** — any cross-component gap discovered during the work filed as a debt task.
