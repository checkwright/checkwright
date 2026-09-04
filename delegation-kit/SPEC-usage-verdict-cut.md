# SPEC amendment: usage-verdict-cut

The port disposition of **`delegation-kit/bin/usage-verdict.sh` (144 lines), the one
owed file declaring §usage-verdict**, onto the binary substrate as a bridged
`Arm::Run` member. This is a stated-contract cut under the port-only run
(TRAJECTORY.md §PRIORITY DIRECTIVE), composed at scope and ruled by the **lead on
its own authority**, 2026-09-04, over the resume channel; it did not reach the
operator, and that is stated because a composition ruling recorded without its
authority reads at the post-port triage as more settled than it is.

**Measured at this HEAD rather than carried from scope's survey**: the port
oracle's `--tree` arm reads 101 files scanned, 64 declared `no-port`, 0
temporarily held, **37 owed**. This cut takes one of that column, and **no ported
member is a gate** — delegation-kit/SPEC.md §Testing already rules that
`usage-verdict` "does not fit the gate contract (a three-state verdict, not a
clean/violation pair)", so this member's whole port surface is the non-gate class.

**The selection ground is the `lines=`-is-a-floor paragraph**, gate-sdk/SPEC.md
§The first cohort, and the rule that selects the next: *`lines=` is a floor on a
port's size and never a ranking of it*, and **cost behind a spawned tool is
invisible to the column on the same terms**. The 144-line column is what a
composing session sees; three costs it cannot see are what selected this member,
and each is measured below rather than argued:

- `native/src/hook/budget.rs:21` spawns `bash <path>` on the **live production
  hook path** — the `PreToolUse(Agent)` budget guard, fired at every dispatch on
  an adopter's machine — and the module's own comment at `:10-12` says the tool
  "is a member of the kit-`bin/` owed cohort, **not of this cut**". The crate has
  been waiting for this cut by name.
- `native/src/usage_tests.rs:73` carries the second waiting sentence, and it is a
  due date rather than a description: "**§usage-verdict's own cut is where the
  crate-floor question comes due**".
- `gate-sdk/SPEC.md:12715-12718` records that this one file's `# exit:` header is
  `check-assertion-strength`'s **whole live vocabulary**. Deleting it takes that
  gate's reach to zero, measured at this HEAD as `2 call(s)` → `0`. Delta (10) is
  that cost, and no line count could have shown it.

**Read the second and third of those as the paragraph's own point.** A ranking by
`lines=` sees a 144-line singleton under-filling a window that carried 192 last
cut. What the column cannot see is that one *other* kit's gate derives its entire
enforcement vocabulary from this file's header, and that this member's own test
module names this cut as the event a deferred question is owed at.

## What changes

### (1) The cut is the one owed file declaring §usage-verdict, and taking it discharges the section whole

`bin/usage-verdict.sh` is the **only** owed file declaring `## usage-verdict` or
either of its two `###` subsections {design-bearing}. Probed rather than assumed:
the tracked non-`docs/` files citing `delegation-kit/SPEC.md §usage-verdict`,
`§The usage.txt contract` or `§The statusline arm` are this file, `smoke/install.sh`
and `scripts/delegation-config.sh` — both `no-port` on class rulings that are not
this cut's — and four crate modules already in-crate
(`native/src/hook/{budget,poll,statusline,usage}.rs`) plus
`native/src/usage_tests.rs`. The two sibling owed files in this kit declare
**other** sections: `bin/usage-trend.sh` declares §Trend reporter and
`bin/wait-probe.sh` declares §bin/wait-probe, so the stated-contract composer's
*one section, the one amendment* is satisfied by a clean singleton rather than by
a subset ruling.

**This cut therefore *discharges* the section, and that is the contrast with the
cut before it.** §scan-prompts could not be discharged: its behaviour is composed
out of a permanently-shell library, so its contract stays half in shell while that
disposition stands. Here both reference producers — the `--statusline` push
producer and the `--usage-poll` poll producer — are **already** in-crate, and the
verdict is the last shell holder in the section's tree. After this cut
§usage-verdict's whole contract is compiled, and the section says so rather than
leaving a reader to derive it.

**What that does *not* discharge is the kit**, and the amendment states the
remainder so no reader reads a discharged section as a discharged kit:
`bin/usage-trend.sh` (120) and `bin/wait-probe.sh` (184) stay owed on their own
sections, and `lib/delegation.sh` stays permanently shell as the config bridge's
sole `DELEGATION_KIT_*` resolver.

### (2) `--usage-verdict` — a bridged `Arm::Run`, and the shape is forced by the exit contract rather than chosen

The rule lands as `native/src/hook/verdict.rs` and registers as one
`BRIDGED_ARMS` row, `("--usage-verdict", Arm::Run(crate::hook::verdict::run),
crate::hook::verdict::KNOBS)`, with its own `case` arm in
`gate-sdk/bin/run-gates.sh` beside `--usage-poll` {design-bearing}.

**It cannot be an `--emit-` member, and this is arithmetic on the dispatcher
rather than a style preference.** `native/src/main.rs:400-417` maps
`Arm::Emit` onto `exit(0)` for `Ok` and `exit(2)` for `Err` — the family collapses
to `{0, 2}` and can never return 1. `usage-verdict`'s published contract is **0**
OK / RESET-OK, **1** PAUSE, **2** STALE, and the 1 is the whole blocking signal
the budget guard grades on. `native/src/emit/mod.rs:445-448` already states the
same conclusion for the sibling that met it first: `--upgrade-smoke` is "an
`Arm::Run` because its contract is the 1-versus-2 split of its exit status, which
an emitting arm collapses." So the `--emit-` prefix — which gate-sdk/SPEC.md
§The non-gate arm rules load-bearing rather than house style, because the
front-end composes it from an `--emit <name>` operand — is unavailable here, and
the member takes its own spelling with its own case arm, as `--lesson-sink`,
`--upgrade-smoke` and `--install-lifecycle` do.

**The status survives the front-end verbatim, probed rather than assumed.**
`gate-sdk/bin/run-gates.sh`'s `exec_arm` ends in `exec`, which replaces the shell
process image, so the binary's status *becomes* the front-end's. Its own
`ARM_UNAVAILABLE_STATUS` is reachable only on the two pre-dispatch failure paths
inside `exec_arm` — an absent binary and a refusing config bridge — and touches no
returned code.

**It is *not* a harness-integration arm**, and the ground is that subsection's own
definition rather than a taxonomy call. gate-sdk/SPEC.md §The harness-integration
arm scopes the class to an arm "whose named caller is the **coding harness** rather
than a battery, a stage step, a regen command or **a gate reaching it in
process**". After delta (3) the budget guard reaches this rule *in process*, and
the arm's remaining callers are a session brief and a smoke — precisely the
reading that section already records for `--upgrade-smoke`, "and a session, so it
is not a harness-integration arm either".

**Ten declared knobs, and no knob default moves in this cut.** The roster is
`DELEGATION_KIT_USAGE_FILE`, `_CRED_FILE`, `_PAUSE_PCT`, `_PAUSE_PCT_7D`,
`_STALE_AGE`, `_LOGIN_WINDOW`, `_REFRESH_CMD`, `_REFRESH_MIN_AGE`,
`_USAGE_HISTORY` and `_FAN_WIDTH` — ten names, of which `_USAGE_FILE` and
`_CRED_FILE` already ride an existing arm's roster (`--statusline`,
`--usage-poll`) and the rest join one for the first time. All are already defined
with defaults in `delegation-kit/lib/delegation.sh:21-30`, with the numeric ones
validated at `:77-89`, so gate-sdk/SPEC.md §The non-gate arm's rule that "a
default the deleted shell driver held inline moves into the owning kit's library
in the same cut" has nothing to move: the driver held none inline, it read them
all off that library. That matters because the bridge **refuses the whole
environment** for a declared knob the owning kit's `lib/` does not define, and on
the hook path delta (3) creates, that refusal is a decline.

### (3) The budget guard retires its spawn — the dividend the crate's own comment was waiting for

`native/src/hook/budget.rs` stops resolving a path and stops spawning
`bash` {design-bearing}. `hook::verdict` exposes the rule as
`verdict(args) -> (String, i32)` — the line and the status — and `run(args)`
prints the line and returns the status, so **one function has two callers**: the
`--usage-verdict` arm and the guard, which calls it directly and grades the `i32`
on the same `code == 1` branch it grades today.

Three things retire with the spawn, and each is named because each is a behaviour
a reader would otherwise look for:

- **The merged-stream capture.** `:18-20` records that the shell form captured
  `2>&1` "so the verdict a caller quotes is whatever the binary said on either
  stream". In process there is one `String` and no stream question.
- **The spawn-that-never-ran branch.** `:26` maps a spawn error onto code 127 and
  the advise arm. An in-process call has no such failure mode; what replaces it is
  the arm-unavailable path, which the front-end already owns and which
  gate-sdk/SPEC.md §The harness-integration arm already rules **fails open** for a
  member gating a tool call.
- **The comment at `:10-12`**, which says the tool "is a member of the kit-`bin/`
  owed cohort, not of this cut". It becomes false at the landing and is rewritten
  rather than left; a comment that documents the opposite of the code beneath it
  is the defect `check-comment-tier` exists against, and this one is load-bearing
  prose a later port reader would take at face value.

**`native/src/hook/mod.rs:24-29`'s `HOOKS` row swaps its knob slice** from the
single `DELEGATION_KIT_VERDICT_BIN` to the ten of delta (2), because the rule now
runs inside the hook process and its inputs must be bridged there. The member's
roster and its kit library's defaults stay one change rather than two, which is
the property that rule exists for.

### (4) `DELEGATION_KIT_VERDICT_BIN` retires, and the seam it is *not* is stated because the name reads like one

The knob leaves `lib/delegation.sh:36`, §Layout and configuration's roster, the
`HOOKS` row and the two SPEC sentences that name it {design-bearing}. This is the
seam ruling this cut owes, and it is taken on an on-point governed precedent
rather than on analogy.

**The precedent is `GUARD_KIT_LIB`.** gate-sdk/SPEC.md §The harness-integration
arm rules that knob out of a ported member's slice "because it named the shell
library the member sourced for its primitives and **a compiled member sources
nothing**". `DELEGATION_KIT_VERDICT_BIN` is that shape with one word changed: it
names the shell binary the member **spawned**, and a compiled member spawns
nothing. There is no path left for it to point at.

**It is not an extension point, and the check is the knob's own documented
purpose rather than its shape.** delegation-kit/SPEC.md:2785-2786 states why it is
a knob: "a consumer may vendor the kit elsewhere". That is *relocation of a
vendored kit file*, not a slot for a consumer's own verdict tool — so
`native-gate-port-remaining-corpus`' ruling (1), which protects an extension
point's resolution, direct execution and env contract, is not engaged; and the
operator's 2026-09-03 seam ruling reaches it from the other side, a file this repo
names as a knob's **value** being what the seam resolves rather than the seam
itself. Nothing narrows: every one of the ten knobs a consumer can actually tune
the verdict with survives, and the arm is reachable through the front-end from any
vendored layout.

**The refused alternative, recorded because it is the tempting one.** Keeping the
knob as an optional override — empty meaning in-process, non-empty meaning spawn —
would preserve a slot no shipped configuration uses and would mint a **second
producer of the verdict**, which is the duplication criterion 6 refuses and which
would leave two implementations of one decision table with nothing holding them
equal.

**The cost is honest and is a published-surface removal.** An adopter who set this
knob to relocate a vendored path finds it gone; the remedy is that no relocation
is needed, the binary being resolved by `gate_command` through
`GATE_SDK_NATIVE_BIN`. The removal lands in §Layout and configuration's roster in
the same commit as the delete, so no sentence survives naming a knob nothing reads.

### (5) The two positionals port unchanged, and the naive read deletes them

`$1` (the usage file) and `$2` (the credentials file) arrive as the arm's argv and
keep overriding `DELEGATION_KIT_USAGE_FILE` and `DELEGATION_KIT_CRED_FILE`
{design-bearing}. Stated as a delta because gate-sdk/SPEC.md's distinguishing test
is applied far more often than it binds, and applied carelessly it deletes these.

The test: an argument is unportable when it redirects something `gate_command` has
**already resolved** before the exec; an argument the **rule itself** consumes,
arriving as argv into the subcommand, ports unchanged. These are the second kind —
the file-positional-falling-back-to-a-knob shape that `check-amendment-queue` and
`check-evidence-manifest` already carry through their own ports. Their live
readers are `delegation-kit/smoke/install.sh:30` and `:53`, which inject a crafted
snapshot, and the crate decision table, which injects one per case; deleting them
would delete the test seam the whole coverage rests on.

### (6) The argv-shape refusal and the `--` escape cross the port; the `-h`/`--help` arm retires to the front-end

The arm refuses a first positional beginning with `-` that is not a recognized
option — usage on stderr, exit 2 — and honours `--` as the end of option
processing {design-bearing}. gate-sdk/SPEC.md §The bin/-tool contract rules that
split explicitly: "the shape refusal and the `--` escape cross the port, the
`-h`/`--help` arm retires to the front-end".

**The shell form has none of the three today, and what that costs is not
theoretical.** `usage-verdict.sh` assigns `$1` straight into `USAGE_FILE`, so
`usage-verdict.sh --help` reads `--help` as a snapshot path, finds it unreadable,
and prints `usage-verdict: cannot read --help … -> STALE` at exit 2. The budget
guard routes exit 2 to **advise** — so a mistyped invocation of the tool that
gates delegation spend reports budget-unknown and waves the dispatch through,
under a line that looks like a real reading. That is the same silent-reader harm
`--emit-scan-prompts` was recorded as the clause's first instance of, reached
through a different door: not a wrong number written into a trend, but a
non-reading dressed as one at the exact decision point the tool exists to hold.

**No live caller is affected, probed rather than assumed**: of the four
invocations in tree, two pass crafted file paths (`smoke/install.sh:30`, `:53`),
one passes none (`scripts/session-context.sh:60`), and the fourth is the crate
test. None passes a `-`-prefixed positional.

### (7) `stat` leaves the tree and the mtime read goes in process

The credentials-file mtime — the whole login-window input — is read with
`std::fs::metadata(path).modified()` rather than `stat -c %Y`
{mechanical}. `native/src/emit/session_id.rs:62-67` already carries that idiom, so
this is an existing crate pattern rather than a new one.

**Measured, and it is a criterion-7 dividend the column also could not see**:
`stat` is **not** on `GATE_SDK_PROGRAM_FLOOR` (`gate-sdk/lib/gate.sh:181-185`),
and `bin/usage-verdict.sh` is the only tracked shell file in the tree that names
it. The delete therefore removes the tree's last off-floor `stat` spawn, and the
arm declares no `--needs` for it. The five on-floor programs the tool also spawns
— `date`, `awk`, `tail`, `mkdir` and `bash -c` for the refresh command — are each
independently named by dozens of other tracked files, so the floor array does not
move; `bash -c` survives inside the arm as the refresh command's launcher, which
is the seam `DELEGATION_KIT_REFRESH_CMD` *is*.

### (8) The decision table's subject moves in process, and its hermeticity mechanism changes with it

`native/src/usage_tests.rs` stops spawning `bash delegation-kit/bin/usage-verdict.sh`
and calls `hook::verdict::verdict` directly {design-bearing}. delegation-kit/SPEC.md
§Testing already licenses either form — "a runner retires into the crate when its
*cases* can be driven from there — the subject reached in process **or** spawned"
— so what this delta owes is not the licence but the consequence, which is not
obvious and is the subtlest part of the cut.

**The existing strip becomes structurally inert, and re-pointing it is not
optional.** Today `subject()` derives the whole `DELEGATION_KIT_*` namespace from
the running process's own variables and removes it from the **child's**
environment, under a real parent-side poison export, so a broken strip fails the
table loudly. A compiled member never reads those names: it reads
`GATE_SDK_KNOB_DELEGATION_KIT_*` through `walk::knob_scalar`, which the config
bridge sets. Left as written, the strip would remove names the subject could not
have read and the poison would prove nothing — the exact ship-green-and-vacuous
failure §Testing names as the one way this coverage can be lost.

**So the three properties §Testing pins are re-expressed on the substrate that now
carries them, each keeping its stated job:**

- the strip is the whole **`GATE_SDK_KNOB_*`** namespace, still derived at run
  time from the running process's own variables and never a hardcoded list;
- the poison stays a **real** value in the running process — now
  `GATE_SDK_KNOB_DELEGATION_KIT_PAUSE_PCT` — so a broken strip still fails the
  table rather than passing it;
- the write goes through `knobenv`'s serializing guard, the crate's sole writer of
  the process-global environment, which the in-process form makes *more* load
  bearing rather than less: the subject now shares that environment with the test.

**One property is genuinely lost, and it is named with the two holders that keep
it rather than waved past.** With no child there is no bridge run, so the table no
longer proves that the defaults it encodes are the ones `lib/delegation.sh`
actually ships. Two existing holders carry that link and neither is minted here:
`check-knob-default-coupling` holds every literal default site against the owning
SPEC, and `smoke/install.sh` — permanently shell, and staying so — drives a real
front-end invocation through the real bridge under its hermetic prelude. The
per-case expectations stay **literal expectations of the kit defaults** exactly as
§Testing already requires, which is what keeps the table asserting what the
defaults *should* be rather than agreeing with whatever they are.

**The refused alternative is spawning the front-end from the crate test.** It
would keep the bridge in the loop, and it is refused because it re-introduces the
`bash` spawn this cut exists to delete, one process further out, while the two
holders above already cover what it would buy.

**Everything else in the module survives unchanged**: the single clock reading for
the whole run, `cases.tsv`'s eleven columns read off disk rather than transcribed
into Rust literals, the beside-the-table refresh stub triple, the five roll-witness
cases and the not-self-witnessing sixth, and the `width=`-with-trailing-space
match. The `the_kits_trend_fixture_reports_its_segments` test is **untouched**: its
subject is `bin/usage-trend.sh`, which declares §Trend reporter and stays owed.

### (9) The crate-floor question §Testing dates to this cut is answered — and the answer is no, with a measured ground

`usage_tests.rs:72-74` and delegation-kit/SPEC.md §Testing both say the
`touch -d "@<epoch>"` mtime set "stays unix-bound for as long as §usage-verdict
does, and the floor question comes due at that member's own cut, when the bash
spawn goes and the mtime set is the last unix-ism left" {design-bearing}. The bash
spawn goes at delta (8). The mtime set **stays**, and both halves of why are
probed rather than reasoned:

- **The API is out of reach at the declared floor.** `std::fs::File::set_modified`
  and `FileTimes` stabilised in Rust 1.75; `native/Cargo.toml:5` pins
  `rust-version = "1.71"`. gate-sdk/SPEC.md has already refused exactly this trade
  once, on `trim-paths`: "taking it would raise the toolchain floor, which runs
  against the objective that exists to collapse that floor rather than raise it".
  The other exit is a dependency, against a crate carrying exactly one
  (`serde_json`).
- **The premise is false at the tree, which is the more useful half.** The clause
  assumes the mtime set is *this member's* last unix-ism.
  `native/src/emit/session_id.rs:179-191` already spawns the identical
  `touch -d @<epoch>` in an **already-ported** member's tests, with no shell
  subject anywhere in it. The idiom is a crate-wide test-harness property, not a
  residue of this member's shell subject, and it would survive this cut whatever
  the floor said.

So the cut **corrects** the sentence rather than discharging it: §Testing's
paragraph and the comment at `usage_tests.rs:72-74` stop dating the question to
this cut and name what actually holds it — the crate's MSRV against a 1.75 API,
with a second holder already in `session_id.rs`. Recording it this way is the
point: a question answered *no* with its ground written down does not come back as
an open item at the next member's cut.

### (10) `check-assertion-strength` loses its entire live reach, and the disposition is accept-and-declare

The delete takes that gate from two covered call sites to zero, and the amendment
costs it rather than letting it be found later {design-bearing}. Measured at this
HEAD: `ASSERTION-STRENGTH: clean (104 script(s) scanned; 2 call(s) to a script
with a declared exit contract)`. Both calls are `smoke/install.sh:30` and `:53`,
both resolving `bin/usage-verdict.sh` through the gate's own-kit-bin convention;
after the cut they name the front-end, which is not that shape.

gate-sdk/SPEC.md:12715-12718 already states the exposure in advance: of the two
declaring scripts, `usage-trend.sh` "names its codes in prose with no uppercase
token and so yields an empty map, leaving `usage-verdict.sh`'s `PAUSE`→1 and
`STALE`→2 as the whole live vocabulary".

**The disposition is accept and declare, on that gate's own stated design.**
§check-assertion-strength rules its reach **opt-in and never widened** — "a callee
declaring no `# exit:` header is simply out of reach" — and makes the call count
the visible surface precisely "so the reach stays visible rather than implied". A
zero reading is that design reporting honestly, not a broken gate; and the
substantive half of what is lost is smaller than the count suggests, because the
`:53` guard's message names `OK`, which binds to code 0 and which the gate's own
skip rule already excludes, while the `:30` guard already compares `-ne 1`
explicitly.

**Two alternatives are refused with their grounds.** *Widening the callee
resolution to reach a compiled arm* would need the arm's exit contract published
on a surface the gate can read, which mints a name and a producer — a gate-sdk
unit of its own, taken deliberately rather than inside a port cut. *Leaving the
`# exit:` header behind in a stub shell file* keeps a gate green by keeping a dead
file, which is the shape the port exists to end.

**The residue is filed rather than flagged**, through the committed gap inbox, so
close disposes of it under the standing rules rather than this amendment
pre-empting them. What is filed is the condition, not a remedy: a gate whose live
reach is zero, with the two candidate widenings above and their costs named.

### (11) Criterion 2's discharge is the both-substrates comparison, bought once, before the delete

This member ships no `good/`+`bad/` fixture pair and owes none — it is not a gate
{design-bearing}. delegation-kit/SPEC.md §Testing already rules what stands in its
place for exactly this kit's non-gate runners: "criterion 2's discharge for a
non-gate member is the both-substrates comparison bought once at port time", with
the cases "driven through the shell runners and the crate module together and
their verdicts and exit codes compared, *before* either shell file was deleted".

The comparison this cut owes, stated as a procedure so build does not invent one:
every row of `usage-tests/cases.tsv`, both beside-the-table sets (the refresh stub
triple and the six roll-witness cases) and the fan-width case are run against the
shell subject and the compiled rule in the same session, and **both the verdict
line and the exit code** are compared per case. The comparison is evidence taken
while both implementations exist; nothing machine-held keeps them agreeing after,
which is why the shell file is deleted rather than left running beside the crate
test.

**The live-tree arm is not available here and saying so is not a loss**: this
member has no live tree corpus — its input is a crafted snapshot — so the case set
*is* the corpus, and the comparison over it is complete rather than demoted.

### (12) Every path-bearing surface moves in the deleting commit

The roster is probed rather than assumed {mechanical}:

- `.claude/settings.json:49` — **exactly one** grant names the deleted path,
  `Bash(bash delegation-kit/bin/usage-verdict.sh)`, and it is in
  `check-settings-paths`' scope, so it reds on the delete without its removal.
  Removing a grant whose target a ruled port cut deletes is outside the 2026-08-22
  bar under the operator's 2026-08-29 settings-grant carve-out, and the removal
  lands in the same commit as the delete. **No replacement grant is needed**,
  probed: `Bash(bash gate-sdk/bin/run-gates.sh *)` is already granted. The
  `--hook agent-budget-guard` dispatch entry is untouched — what changes is behind
  it, not the command it names.
- `scripts/session-context.sh:58-60` — the SessionStart budget line. `VERDICT_BIN`
  and its `-f` existence test go; the call becomes the front-end arm. The
  precedent for the spelling is one file away, `scripts/delegation-config.sh:18-20`,
  where `DELEGATION_KIT_REFRESH_CMD` already reads
  `bash gate-sdk/bin/run-gates.sh --usage-poll` under a comment recording that
  "the poller is an arm now, so the refresh command dispatches it through the
  front-end rather than naming a template path the port deleted".
- `delegation-kit/smoke/install.sh:30` and `:53` — both invocations. The file
  carries `# no-port:` at `:4`, which exempts the smoke from porting and not its
  call sites from naming a path that stops existing.
- `delegation-kit/README.md:81-82` — the two-line invocation roster, both forms.
- `delegation-kit/templates/agent-execution.md:406` and `:430` — the resident
  dispatch protocol, including the `watch -n 30` polling line. This is the
  highest-traffic prose surface in the roster: every stage session loads it.
- `delegation-kit/SPEC.md` — `:2657` (the layout tree), `:2783-2790` (the retiring
  knob, delta 4), `:2842-2843` (the guard's resolution sentence), plus
  §usage-verdict, §Testing and §Layout and configuration themselves.
- `native/src/hook/budget.rs` — `:10-12`, `:14`, `:18-27` (delta 3).
- `native/src/hook/mod.rs:24-29` — the `HOOKS` knob slice (delta 3).
- `native/src/usage_tests.rs` — `:1`, `:49-53`, `:72-87`, `:96-145`, `:183`
  (deltas 8, 9).
- `delegation-kit/lib/delegation.sh:2` and `:36` — the header's tool list and the
  retiring knob's default (delta 4).
- `gate-sdk/SPEC.md` — §The non-gate arm's class roster, §The bin/-tool contract's
  crossing clause, and §check-assertion-strength's live-vocabulary paragraph
  (deltas 2, 6, 10).
- The `docs/` mirrors of every edited SPEC and README, which are generated and are
  delta (13)'s.
- **Not on this list, named so the omission is legible:**
  `gate-sdk/gate-tests/check-template-copy-parity/{good,bad}/scripts/agent-budget-guard.sh:6`
  each carry `${DELEGATION_KIT_VERDICT_BIN:-delegation-kit/bin/usage-verdict.sh}`
  and look like knob-default sites this cut must move. They are not, **probed**:
  `gate-tests` is a member of `GATE_PRUNE_DIRS` (`gate-sdk/lib/gate.sh:36`), which
  `walk::find_files` applies before any per-gate filter, so
  `check-knob-default-coupling` never reaches them — its clean line at this HEAD
  reads 57 kit source files, not a tree-wide count. They are fixture data for a
  different gate's copy-parity assertion and stay as written.

### (13) The regeneration fan-out this cut stales

Deleting one owed `.sh` moves `measured-claims.sh`'s `tree-shell-owed` key, read
off `--emit port-blockers --tree`'s trailer at `scripts/measured-claims.sh:39-42`
{mechanical}. docs/site-architecture.md §Generated projections rules that a tree
edit moving a measured claim stales the generated `pre-commit` and `commit-msg`
hooks, because the baked invocation carries `check-measured-claim`'s resolved
values, and `docs/check-graph.html` with them. The SPEC and README edits stale
their on-site mirrors, and the crate change stales the binary. All are rostered
with their regen commands in that section and are discharged in the landing
commit; `check-graph`, `check-docs-mirror-fresh` and `check-gate-binary-fresh` are
the reds.

**Two surfaces a reader expects here and which are *not* staled, probed rather
than assumed**: `docs/footprint.md`, whose own text excludes scripts under `bin/`
from its set; and `docs/enforcement.md` / `docs/value.md`, whose trigger is a
change to the class registry — this cut registers no gate, adds no `kpis.list`
entry and changes no `.claude/settings.json` hooks roster shape. No prose marker
pins `tree-shell-owed` either: the three live `measured:` markers name
`gate-substrates` and `ported-gate-members`, so `check-measured-claim` stays green
from the marker side and the staleness is the baked hook alone.

### (14) The criterion-5 residual is smaller than it looks, and the reason is that the guard was already binary-gated

Criterion 5 asks what a consumer whose payload carries no artifact for its host
still has after the cut {design-bearing}. This cut registers no gate, so the
binary-less leg's omitted-member roster and its count do not move at all.

**The verdict's own loss on such a host is real but was already paid.**
`agent-budget-guard` is a hook member that moved into the binary at an earlier
cut, so on an artifact-less host the guard already cannot run and already declines
under the fail-open rule; what this cut adds to that host's loss is the two shell
call sites — the SessionStart brief line and the smoke's assertion — not the
enforcement. The honest statement is that such a consumer dispatches with no
budget verdict in context and no per-dispatch block, which was true before this
cut for the blocking half and becomes true for the advisory half; it is worse, it
is not nothing, and it resolves the moment that platform has a published artifact
rather than by anything this cut could do.

### (15) The queue entry is promoted at exactly zero headroom, and the demotion is priced now

`native-gate-port-remaining-corpus` is promoted by swapping `[design-pending]` for
`[spec: SPEC-usage-verdict-cut.md]` on its lead line {mechanical}. Both numbers
are measured, not estimated:

- The gate's own headroom line reads
  **`native-gate-port-remaining-corpus: 0 lines of headroom`** against the 50-line
  cap. The promotion is therefore **line-neutral by necessity**, and this cut's
  record lands in delegation-kit/SPEC.md §usage-verdict — the entry's own rule that
  each closed cut's record lives in the contract section that cut selected — never
  on the entry.
- The lead line is 83 columns; less `[design-pending] ` it is a 66-column base.
  The **bare-basename** ref costs 34 columns, for exactly **100** against
  `QUEUE_KIT_WRAP_BUDGET=100`, which `native/src/gates/queue_wrap.rs:66` compares
  with `<=`. Measured with `printf | wc -c`, not counted by eye: the fit has zero
  columns to spare, and the repo-relative spelling
  `delegation-kit/SPEC-usage-verdict-cut.md` would cost 49 and overflow at 115.
  canon-kit/SPEC.md §check-amendment-queue admits either form; only one fits, and
  it is chosen for that reason rather than by house style.

The entry **demotes** at build and never reaches `## Done`, which its own body
rules. The demotion re-prices it against `check-queue-entry-budget`, unlike a Done
move, so at zero headroom the demoting commit may add no line to it — the
constraint build meets, stated here so build does not discover it.

## Producers and consumers

The amendment introduces **one interface** — one bridged flag — and **no new
state, no new event, no new field, and no new knob**. It **retires** one knob
(delta 4). Every knob named is already shipped and already defaulted in
`lib/delegation.sh`, which stays permanently shell as the bridge's sole
`DELEGATION_KIT_*` resolver, so no default moves.

- **Producer** — `native/src/emit/mod.rs`'s `BRIDGED_ARMS` table, one row
  (`"--usage-verdict"`, `Arm::Run(crate::hook::verdict::run)`,
  `crate::hook::verdict::KNOBS`), plus one `case` arm in
  `gate-sdk/bin/run-gates.sh` — required, unlike an `--emit-` member, because the
  front-end composes only the `--emit-<name>` spelling and this member cannot take
  it (delta 2). The enabling config is the table row itself: `--knobs` publishes
  the roster and `gate_command` resolves it before the exec, so nothing is
  configured per install.
- **Consumer, through the arm** — `scripts/session-context.sh`, at the SessionStart
  transition, which reads the verdict **line** off stdout and prints it into the
  session brief; and `delegation-kit/smoke/install.sh` at `:30` and `:53`, which
  reads the **exit status** at install-verification time. The channel is live
  wherever delegation-kit is vendored, all ten knobs carrying shipped defaults.
- **Consumer, in process** — `native/src/hook/budget.rs`, which calls
  `hook::verdict::verdict` directly and receives `(String, i32)`. The transition is
  every `PreToolUse(Agent)` fire: the `i32` routes block-on-1 / advise-otherwise
  and the `String` is relayed verbatim as the block text or as
  `additionalContext`. This is §The non-gate arm's *a gate calling it in-process*
  caller shape, which `--emit-close-surfaces` established.
- **Consumer of the declared roster** — `gate_command`, which resolves the ten
  knobs by sourcing `delegation-kit/lib/delegation.sh` and refuses the whole
  environment for a knob it does not define. All ten are defined there today at
  `:21-30`, which is why this cut moves no default.
- **Consumer, in test** — `native/src/usage_tests.rs`, calling the same
  `verdict(args) -> (String, i32)` the guard calls, per case, under the re-pointed
  strip of delta (8).

**The arm has a caller that is not its own fixture**, which is §The non-gate arm's
third property: two of them, a consumer-side session brief and a kit smoke, plus
the in-process guard.

**The arm's spawned-program set is one program, and it is the seam rather than an
implementation detail.** The shell tool spawns `date`, `stat`, `awk`, `tail`,
`mkdir` and `bash -c`; the compiled arm spawns **`bash -c` alone**, and only when
`DELEGATION_KIT_REFRESH_CMD` is non-empty — the knob *is* a command seam, so its
launcher survives the port by the same reasoning that kept `curl` external and
spawned under `--usage-poll`. Recorded in prose because a `BRIDGED_ARMS` row
carries no requirement element and `--needs` answers about registry members only.

**One corpus is narrowed, and its readers' red conditions are enumerated rather
than their subjects** (canon-kit/SPEC.md §The causal-completeness check, point 5).
The narrowing is the deletion of one file from the tracked `*.sh` tree:

- `check-settings-paths` — reds on a literal repo-relative `.sh` grant that **does
  not resolve**, so its verdict is *not* monotone under this narrowing: removing
  the file **adds one** violation. Cleared by delta (12), in the same commit, not
  by inspection.
- `check-assertion-strength` — its red condition is a guard whose message names a
  discriminable token while comparing no status to that token's code. Under this
  narrowing it can only find fewer, so it is monotone and stays green — but its
  *reach* goes to zero, which is a coverage loss its clean-line count exposes and
  which no red will ever report. Delta (10) owns it.
- `check-measured-claim` — `tree-shell-owed` moves by one. No governed sentence
  pins it (the three live markers are checked, not assumed), so the reader that
  moves is the baked hook invocation, cleared by delta (13).
- `check-graph` / `check-docs-mirror-fresh` / `check-gate-binary-fresh` — red on a
  stale hook, artifact, mirror or binary, and non-monotone for the same
  baked-value reason. Cleared by delta (13).
- `check-knob-default-coupling` — named because delta (4) removes a knob's default
  site and a reader assumes fixture literals must follow. Probed: the corpus is
  pruned of `gate-tests`, so it does not reach them, and the removal of one
  literal site can only remove comparisons. Monotone, cleared by inspection.
- `check-gate-fixture-coverage` — named because a reader assumes a deleted tool
  moves it, and it does not: no `.gate` descriptor and no fixture pair is owed or
  removed here, the member never having been a gate.
- `check-exec-bit`, `check-shellcheck`, `check-comment-tier`, `check-path-dialect`
  — monotone in the scanned `.sh` set; each can only lose findings. Cleared by
  inspection. `check-comment-tier` additionally *gains* the new Rust module's
  `// spec:` comments, which is an obligation rather than a risk.
- `check-crate-arms` — gains rather than loses: one new module and its
  `#[cfg(test)]` coverage join the lint and test arms it runs at every commit.

**Cross-component signal: this amendment's component set is three** —
delegation-kit (§usage-verdict, §Testing, §Layout and configuration), gate-sdk
(§The non-gate arm, §The bin/-tool contract, §check-assertion-strength) and the
crate's hook layer, which is not a kit but whose `HOOKS` roster and budget member
change contract here — so `check-stage-entry` assertion C fires and the **align
stamp is demanded at the build stage's entry**. Stated here so the build session is
not the one that learns it, and it is the reason this session recommends the audit
stage next.

## Existing sections updated

- `delegation-kit/SPEC.md §usage-verdict` — the tool is restated as its arm: the
  invocation form, the ten declared knobs, the `Arm::Run` shape and why the
  `--emit-` family is unavailable, the new argv-shape refusal and `--` escape, the
  two surviving positionals, and the in-process caller beside the front-end one
  (deltas 2, 3, 5, 6). The `# exit:` paragraph at `:2218-2228` is rewritten: the
  mapping stands, but its *machine reader* is gone, and the paragraph says so and
  points at delta (10)'s record rather than continuing to describe a header as "a
  surface with a consumer". **This section records the cut**, per the composer
  entry's own rule that each closed cut's record lives in the contract section
  that cut selected — and it records the section as **discharged**, the first in
  this kit (delta 1).
- `delegation-kit/SPEC.md §Testing` — the decision-table paragraph's three pinned
  properties are re-expressed on the in-process subject: the strip becomes the
  `GATE_SDK_KNOB_*` namespace, the poison moves with it, and the
  "**The subject stays shell and is spawned**" bullet is replaced by what now
  holds the bridge-side link — `check-knob-default-coupling` and the smoke
  (delta 8). The `touch -d "@<epoch>"` sentence stops dating the floor question to
  this cut and names the MSRV that actually holds it, with `session_id.rs` as the
  second holder (delta 9). The criterion-2 discharge sentence gains this member's
  own comparison (delta 11). The `usage-trend` paragraph is untouched.
- `delegation-kit/SPEC.md §Layout and configuration` — `bin/usage-verdict.sh`
  leaves the tree block; `DELEGATION_KIT_VERDICT_BIN` leaves the knob roster with
  its ground (delta 4); the `agent-budget-guard` registration paragraph at
  `:2838-2847` stops saying the hook "resolves `bin/usage-verdict.sh` at its
  vendored path" and states the in-process call instead, keeping the wiring line
  unchanged because the wiring does not change (deltas 3, 4).
- `delegation-kit/README.md` — the two-line invocation roster becomes the arm's,
  since §The non-gate arm puts a bridged arm's discoverability in the front-end's
  help and the owning kit's README (delta 12).
- `delegation-kit/templates/agent-execution.md` — the two invocation lines,
  including the `watch -n 30` polling form (delta 12).
- `gate-sdk/SPEC.md §The non-gate arm` — the class roster gains `--usage-verdict`,
  recorded as the first member forced out of the `--emit-` family by a
  **three-state exit contract on a decision a hook grades**, beside
  `--upgrade-smoke`'s 1-versus-2 split; and the spawned-program prose gains this
  arm's single program with the reason it survives (deltas 2, 7).
- `gate-sdk/SPEC.md §The bin/-tool contract` — the crossing clause gains a second
  reader instance and the ground that distinguishes it from the first: here the
  absorbed argument does not write a wrong number into a trend, it produces a
  **non-reading dressed as a reading** at the decision point a hook grades, which
  routes to advise (delta 6).
- `gate-sdk/SPEC.md §check-assertion-strength` — the live-vocabulary paragraph is
  rewritten from *thin* to *empty*, with the count named as the surface that shows
  it, the two candidate widenings and their costs recorded, and the filed residue
  cited (delta 10).
- `gate-sdk/SPEC.md §The first cohort, and the rule that selects the next` — the
  `lines=`-is-a-floor paragraph's spawn-invisibility clause gains this cut as its
  worked instance on a **third** axis: not a hidden interface set and not a
  permanently-shell composition, but a spawned tool whose header is another kit's
  gate's entire enforcement vocabulary (delta 10, and the title prose above).
- `TASK-QUEUE.md`, the `native-gate-port-remaining-corpus` entry — promoted with
  `[design-pending]` swapped for this amendment's `[spec:]` ref; it **demotes** at
  build and never reaches `## Done`, which its own body already rules. At zero
  headroom the swap is line-neutral and the demoting commit may add no line
  (delta 15).
- The generated projections this cut stales — the on-site SPEC and README mirrors,
  the generated `pre-commit`/`commit-msg` hooks, `docs/check-graph.html`, and the
  gate binary itself. All are rostered with their triggers and regen commands in
  `docs/site-architecture.md` §Generated projections (delta 13).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named reader
      at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone, and
      the merge holds CLAUDE.md §The provenance seam: the dated rulings and
      authorities above land in git history, never in a kit SPEC's prose.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`), the none-remain half discharged at
      the iteration rather than at the commit.
- [ ] **Removals propagated** — grepped every spec, skill, template, README, smoke
      script, gate-test, settings file and committed workflow surface for the
      deleted path and for `DELEGATION_KIT_VERDICT_BIN`; nothing dangles.
- [ ] **The both-substrates comparison was taken before the delete** — every
      `cases.tsv` row, both beside-the-table sets and the fan-width case run
      against the shell subject and the compiled rule in one session, **verdict
      line and exit code compared per case**, and the comparison recorded. A
      comparison a session runs before a delete is evidence; one attempted after
      it is nothing (delta 11).
- [ ] **The three-state status survives end to end** — a PAUSE reading exits 1
      through `bash gate-sdk/bin/run-gates.sh --usage-verdict`, proved by running
      it, not by reading `exec`.
- [ ] **The re-pointed strip is proved, not asserted** — the `GATE_SDK_KNOB_*`
      poison fails the table when the strip is broken, demonstrated by a negative
      control in the landing session (delta 8).
- [ ] **The retired knob leaves no reader** — no surface names
      `DELEGATION_KIT_VERDICT_BIN`, and the `HOOKS` row carries the ten knobs the
      in-process rule needs, proved by a hook fire rather than by inspection: a
      bridge that refuses one undefined knob declines the guard (delta 4).
- [ ] **The argv-shape refusal has a firing and a non-firing case** — a
      `-`-prefixed first positional refuses at exit 2 on stderr; the same token
      after `--` is taken as a path (delta 6).
- [ ] **`stat` left the tree** — no tracked shell file names it, taken as a grep
      rather than inferred from the delete (delta 7).
- [ ] **The reach loss is recorded and filed** — §check-assertion-strength states
      the empty vocabulary and the residue is in the gap inbox; the gate's clean
      line is re-read after the delete and the new count written into that record
      (delta 10).
- [ ] **The section is declared discharged, and truthfully** — §usage-verdict
      states that its whole contract is in-crate, and the `--tree` roster carries
      no `delegation-kit/bin/usage-verdict.sh` row, taken as a per-file roster diff
      and not as a trailer delta (delta 1).
- [ ] **The regeneration fan-out is discharged in the landing commit** — the
      generated hooks, the graph artifact, the SPEC and README mirrors, and the
      gate binary (delta 13).
- [ ] **The demotion fits** — the entry returns to Deferred at build with no line
      added, against a measured zero headroom (delta 15).
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not deferred).
