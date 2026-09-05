# SPEC amendment: hermetic-cut

The port disposition of **`gate-sdk/lib/test-hermetic.sh` (52 lines), the one owed file
declaring gate-sdk/SPEC.md §lib/test-hermetic.sh**: the second producer that has kept the file
undeclarable is **removed**, the defect that second producer carries is fixed by the same
edit, and the file then takes a `# no-port:` declaration on a ground stated here for the first
time. A stated-contract cut under the port-first run (TRAJECTORY.md §PRIORITY DIRECTIVE),
hosted on its own per-cut feature entry and packaged by the lead as one of this iteration's
four.

**`hermetic-bin-suffix-pin-placement` rides inside this cut** and is promoted with it, under
`native-gate-port-remaining-corpus`' ruling **(6)** — *an entry whose discharge is an owed
file's stated precondition rides inside that cut* — and under the operator's own 2026-09-05
ruling that it blocks nothing. It is not a sibling unit and gets no amendment of its own: its
whole deliverable is delta 1 here. `kit-library-port-residue` states the same sequencing from
the other side — "**its disposition waits on the defect below**" — so this amendment
discharges that wait rather than reopening it.

**The composer's precondition was run at this stage rather than inherited.**
`bash gate-sdk/bin/run-gates.sh --emit port-blockers --group` trails *0 group(s) formed …
0 takeable at this cut*. The selection ground is the **owed column** of
`--emit port-blockers --tree` — *89 file(s) scanned, 66 declared no-port, 0 temporarily held,
23 owed* — where this file reads `owed lines=52`.

**This cut's disposition is a declaration and not a port, and that is the one thing in this
amendment a reader should check before build acts on it.** The cut still subtracts the file
from `--tree`'s owed column, which is the completion predicate's own arithmetic
(§The kit-library port disposition: the arm "reclassifies each member `owed` → `no-port`, so
the completion predicate TRAJECTORY.md states over that arm's owed count subtracts them"), so
the port's progress metric is unaffected. What differs from its three sibling cuts is the
*character* of the delivery, and delta 2 states the ground in full rather than assuming it.

## What changes

### (1) The second `GATE_SDK_NATIVE_BIN` default is removed, which is also the defect's fix

`lib/test-hermetic.sh:14` spells its own default for a bridged knob:
`export GATE_SDK_NATIVE_BIN="${GATE_SDK_NATIVE_BIN:-${_th_root}/native/target/release/checkwright-gates}"`
{design-bearing}. §lib/test-hermetic.sh already records what is wrong with it and what is not:
pinning the knob **absolute** is documented and deliberate, since a bespoke test runs its gate
from a sandbox cwd where the repo-relative default resolves to nothing; what is undefended is
that this spelling **omits the executable-suffix helper** `lib/gate.sh`'s own default appends,
so on a Windows host every bespoke test pins the knob to a suffix-less path that cannot exist.
That is the Deferred entry `hermetic-bin-suffix-pin-placement`.

**The fix is not to append `gate_exe_suffix` here.** Doing so would leave two spellings of one
default agreeing by hand, which is the duplication the kit-library class ruling rests on
refusing and the reason this file was left undeclared in the first place.

**The ruling: resolve through the accessor and absolutize the answer — the shape the runner
beside it already uses.** `bin/run-gate-tests.sh:24-28` reads `gate_native_bin`, tests the
answer for a leading `/`, and absolutizes it against the invoker's root when it is relative.
That is one producer and one consumer: the default is computed in `lib/gate.sh` and nowhere
else, the suffix rides along because the accessor appends it, and what this file adds is an
**absolutization**, not a default. The line becomes that shape, with `lib/gate.sh` sourced
ahead of it rather than lazily inside `gate_run`.

**Two facts make the change safe and both are measured rather than argued.** Under the
runner the pin is already a no-op — `bin/run-gate-tests.sh:24-28` exports an absolutized value
into the environment every `*.test.sh` inherits, and line 14's `${…:-…}` therefore never
fires; the second default only ever fires for a standalone `bash <name>.test.sh` invocation,
which is exactly the case the absolutization must keep working. And the ordering §lib/test-hermetic.sh
fixes is preserved: this library is sourced at the top of every `gate-tests/*.test.sh`,
**before** `lib/gate.sh` would otherwise be sourced, so its value is already set when
`lib/gate.sh`'s own guarded assignment runs and that assignment stays the no-op it is today.

**The paragraph in §lib/test-hermetic.sh that describes the defect is deleted, not annotated.**
TRAJECTORY.md's authoring rule is that a ruling whose subject is finished is deleted outright
and a fact that has aged is corrected where it stands; an *unless* paragraph explaining a
hazard that no longer exists is two readings of one fact.

### (2) The file is declared `# no-port:`, on a ground this section states rather than inherits

With the second producer gone the disposition is available, and it is a **declaration**
{design-bearing}. The ground is stated in full because no existing class reaches this file and
a declaration citing a class that does not reach it would mis-size the predicate with nothing
red to catch it.

**Why the existing class does not reach it, restated from the class's own text.**
§The kit-library port disposition rules a kit `lib/*.sh` permanently shell when it is the
config bridge's **sole resolver** for its kit's knobs, and it is explicit that position is not
the test: "a member sourced into the resolution subshell that computes no knob contributes
nothing to the bridge and is **not** held by this ground". This file resolves no knob. So it
needs its own ground, exactly as `context-kit/lib/pub-lang/`'s extractors did.

**The ground, in two independent limbs.**

- **Its API is three shell functions, and a binary arm cannot be sourced into bash.**
  `gate_env`, `gate_run` and `gate_arm_run` are called *inside* the caller's own shell — that
  is the whole reason `gate_env` exists rather than an `env` prefix, which §run-gate-tests
  states: "`env` cannot invoke a shell function and — more to the point — a bridged knob is
  resolved when the **argv** is built, so an override set around the binary would arrive after
  the value it was meant to change had already been read". Measured this session over the
  tracked tree: **92 of 94 `gate-tests/*.test.sh` source this library**, 58 call `gate_run`,
  33 `gate_env`, 7 `gate_arm_run` and 6 `gate_native_bin` through it. There is no in-crate arm
  that a `source` line can name.
- **What the two composing functions compose is the bridge itself.** `gate_run` calls
  `gate_command` and `gate_arm_run` calls `gate_native_bin` and `gate_knob_env`, all from
  `lib/gate.sh` — which §The kit-library port disposition calls "the second producer squared"
  and which is permanently shell. Once delta 1 lands, this file's relationship to the bridge is
  character-for-character the one `gate-sdk/lib/consumer-smoke.sh` declares on under §Consumer
  smoke *The port disposition* leg 1: "sourced into its callers' own shell and resolves
  `GATE_SDK_NATIVE_BIN` through `lib/gate.sh`'s accessor … so it sits inside the bridge rather
  than beside it".

**Three alternatives are refused, each on its own ground.**

- **An `--emit-test-hermetic` arm emitting the export lines, `eval`'d by the library**, is
  refused because it makes the hermetic bootstrap depend on the binary being present. A
  consumer vendoring the shell library on an uncovered platform would lose hermeticity
  silently, and the bootstrap's own §Testing role is to be the *first* act of every bespoke
  test — before anything has established that a binary exists.
- **Moving the environment pinning into the ported `--run-gate-tests` arm**, so the library
  keeps only its functions, is refused because it narrows the contract: `check-test-hermetic`
  requires the source line as each test's first act precisely so a standalone
  `bash <name>.test.sh` is hermetic too, and a runner-side pin reaches only tests the runner
  spawns. It also trades a stated contract for an implicit one, which is what the
  gate exists to prevent.
- **Leaving the file undeclared and owed** is refused because the ground now holds and the
  reason it was withheld has been removed. §lib/test-hermetic.sh's own sentence — "it is owed
  to the port and is **deliberately not declared**" — names the second producer as the
  withholding reason, so the sentence discharges with delta 1 rather than surviving it.

**What reopens it**, on §Consumer smoke *The port disposition*'s terms and not as a permanence
claim: the ground dissolves if `lib/gate.sh` ever admits a second bridge producer, and it
dissolves for this file if the bespoke `*.test.sh` corpus stops being bash — a member outside
the bridge whose callers are compiled is not held by either limb, and its disposition would
then be whatever this section could state for it.

### (3) `kit-library-port-residue`'s roster loses this member, as a delivery and not an unblocking

The residue entry carries `gate-sdk/lib/test-hermetic.sh` in its still-owed roster with the
sentence "its disposition waits on the defect below" {mechanical}. This cut settles the
disposition, so the member **leaves the roster outright** and the entry's paragraph records a
delivery.

**That distinction is §Porting a gate to the binary substrate's and it is stated because the
two readings differ.** A host that discharges a *blocker* corrects its roster to *unblocked
and takeable*; a host that delivers an *increment* removes the member. This is the second
kind: nothing about the member is left owed after this cut. The entry's `hermetic-bin-suffix-pin-placement`
sentence goes with it, that entry reaching Done at build.

### (4) §The non-gate arm and §The port disposition record a declaration, not an arm

No bridged-arm row is added and the class roster is untouched {mechanical}. What changes in
gate-sdk/SPEC.md is §The port disposition's owed-corpus prose: after this cut and its sibling
`SPEC-gate-tests-cut.md`, gate-sdk's owed column holds `lib/inject.sh` alone, behind
`doctrine-kit/bin/install-doctrine.sh` under the sequencing stated at
doctrine-kit/SPEC.md §install-doctrine.

**The declaration's text is the file's own `# no-port:` header**, carrying both limbs of
delta 2's ground and ending "Structural, not a sizing judgment" — the sentence every
declaration in this tree ends on, which is what stops a later reader re-opening a declared
file as a sizing question.

## Producers and consumers

**No new state, event or interface is introduced.** This cut removes a producer and adds a
declaration; it mints nothing. The causal-completeness points are therefore answered about the
value whose producer set *changes*.

**`GATE_SDK_NATIVE_BIN`'s producer set narrows from two to one.**
*Producer before* — `gate-sdk/lib/gate.sh:105` (the accessor's default, suffix appended by
`gate_exe_suffix`) **and** `gate-sdk/lib/test-hermetic.sh:14` (a hand-spelled absolute
default, no suffix). *Producer after* — `gate-sdk/lib/gate.sh:105` alone; this file becomes a
consumer that absolutizes the accessor's answer.
*Consumers, each named with the transition at which it reads* — `gate_command`
(`lib/gate.sh:443`), when it builds a `.gate`-declared member's argv; `gate_native_bin`, read
by `bin/run-gate-tests.sh:24` at startup and by six `gate-tests/*.test.sh` directly;
`gate_arm_run` (`lib/test-hermetic.sh:34`), when a bespoke test drives a bridged arm; and
`bin/run-gates.sh`'s `exec_arm`, before every arm dispatch. Every one of them reads the
resolved value, none of them re-derives it, and none of their code changes.

**The enabling configuration is actually set everywhere it must be**, which point 1 of the
check requires and which is the half a hand-spelled default hides: after delta 1 the absolute
value reaches a bespoke test by two routes, the runner's process-wide export
(`bin/run-gate-tests.sh:24-28`) and this library's own absolutization at source time, and the
second is what covers the standalone invocation the first cannot reach.

**This delta set narrows a corpus — the producer set — so point 5 binds and each affected
reader's RED condition is enumerated rather than its subject.**

- **`check-test-hermetic`** reds when a `<kit-root>/gate-tests/*.test.sh` neither sources this
  library nor carries the marker. Its verdict is monotone in the violation set and no test file
  is added or removed, so it is clearable by inspection — but it is named because it is the
  gate that would catch a botched delta 1 that broke the source line.
- **`check-knob-default-coupling`** reds when a knob's default is spelled in two places
  without a machine holding them equal. Its red condition is a *found duplication*, so
  removing one spelling can only reduce its violation set: monotone, clearable.
- **`check-gate-binary-fresh`** reds when the binary's baked source stamp disagrees with the
  shell library's computation of the same thing. Its input is `lib/gate.sh`, untouched here;
  it is named because a reader will ask whether removing a second spelling of the binary's
  *path* touches the stamp, and it does not.
- **`run-gate-tests`' own pair loop and the 94 bespoke tests** are the executed oracle and the
  one that is **not** clearable by inspection: a test that resolves the binary to a path that
  does not exist fails as a `HARNESS:` line rather than as a missing pin, so the delta is
  cleared by the full fixture battery running green from the repo root **and** by one bespoke
  test run standalone from a non-root cwd. The second run is the assertion the first cannot
  make, because the runner's export masks the line this delta edits.

## Existing sections updated

- **gate-sdk/SPEC.md §lib/test-hermetic.sh** — the binary pin restated as an absolutization of
  the accessor's answer; the deliberately-not-declared paragraph and its defect clause deleted
  rather than annotated; the declaration's two-limb ground stated with its three refused
  alternatives and its reopening condition (deltas 1 and 2).
- **gate-sdk/SPEC.md §The port disposition** — the owed-corpus prose every cut moves; gate-sdk's
  owed column after this cut and its sibling (delta 4).
- **gate-sdk/SPEC.md §The kit-library port disposition** — its honest-limit paragraph names
  the libraries "that ride the glob resolving nothing" as owed still, "each for its own reason,
  and each names the entry that owns its port in its own section"; this member leaves that set
  by taking a stated ground of its own, and the paragraph records it as the class's first
  member to do so (delta 2).
- **gate-sdk/SPEC.md §Consumer smoke, *The port disposition*** — leg 1's ground now has a
  second file standing on the same shape, cited rather than restated (delta 2).
- **gate-sdk/SPEC.md §run-gate-tests** — the sentence describing how a bespoke test reaches
  its gate names the binary as "pinned absolute"; it stays true and gains the word that makes
  it precise, *absolutized* rather than *defaulted* (delta 1).
- **`gate-sdk/lib/test-hermetic.sh`** — the file itself: line 14's shape, the `lib/gate.sh`
  source ordering, its two `spec:` comments, and the new `# no-port:` header (deltas 1, 2).
- **TASK-QUEUE.md `kit-library-port-residue`** — the member leaves the roster as a delivery;
  the `hermetic-bin-suffix-pin-placement` sentence goes with it (delta 3).
- <!-- update-target-exempt: a generated projection with its own freshness gate and regen command, rostered in docs/site-architecture.md §Generated projections and their freshness gates; it stales on the line-count change alone and no delta owns its content --> `docs/footprint.md` and `docs/value.md`'s rollup block.

## Definition of Done

- [ ] **Causal completeness** — the narrowed producer set's every consumer is named with the
      transition at which it reads, and each affected reader's red condition is enumerated
      rather than its subject.
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section (not appended); the merged spec reads as one coherent document a
      reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather than at this commit while
      the sibling gate-sdk amendment is in flight.
- [ ] **Removals propagated** — grepped every spec and entry for the deleted defect paragraph's
      subject and for the second default; nothing dangles, and
      `hermetic-bin-suffix-pin-placement` reaches Done.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
