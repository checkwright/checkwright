# SPEC amendment: demo-cut

The port disposition of **`demo/run-demo.sh` (96 owed lines), the adoption walkthrough**: its
`# spec:` pointers are **re-homed onto gate-sdk/SPEC.md §Consumer smoke first**, and the file then
cuts as a singleton under that owner into a bridged `--run-demo` `Arm::Run`, reaching
`csmoke_place_binary` across the same spawn seam the upgrade suite and the AGENTS.md smoke already
use. A stated-contract cut under the port-first run (TRAJECTORY.md §PRIORITY DIRECTIVE), hosted on
its own per-cut feature entry.

**The re-home is the mis-homed correction this SPEC's own composer section rules, and it is
expressly not the re-pointing the 2026-09-03 ruling refused.** §Porting a gate to the binary
substrate draws that line in terms: the composer's *one specification section* means "a section of
a kit SPEC or of the owner doc a `# spec:` line names for its mechanism; an always-loaded manifest
such as a consumer's CLAUDE.md owns no mechanism, so a file whose `# spec:` points there is
**mis-homed** rather than sequenced. The correction re-homes the pointer at the doc that owns the
file's mechanism, and the file then cuts as a singleton under that owner. This is not the
re-pointing the paragraph above refuses: that one moved a file away from a SPEC section that
already owned it, and this one gives a file the owner it never had." Both halves of the test hold
here and were probed rather than argued: `demo/run-demo.sh:2` and `:11` point at
`CLAUDE.md §Housekeeping`, which describes the walkthrough and specifies no mechanism of it — it
does not so much as name `DEMO_TMP_DIR`, the knob `:11` tells the reader that section owns — while
the file's own third pointer, at `:35`, already names `gate-sdk/SPEC.md §Consumer smoke` for the
one decision it needed a spec for. The file is being given the owner it never had.

**The `# no-port:` fork is closed, and the decline is a selection inside the grant rather than an
override of it.** TRAJECTORY.md's residue-class clause says the residue shipping to no adopter
"takes a **per-file disposition** when reached" — a disjunction whose branches are a `# no-port:`
declaration and a port, not a default of the first. `installer/consumer-smoke/run-smoke.sh` took
the declaring branch; this file takes the porting one — operator, 2026-09-06, relayed by the lead
session — so no delta below re-weighs the fork.

**The composer's precondition was run at this stage rather than inherited.**
`bash gate-sdk/bin/run-gates.sh --emit port-blockers --group` trails *0 group(s) formed … 0
takeable at this cut* — no takeable group, which is the budget arm's stated precondition
(§The first cohort). The selection ground is the **owed column** of `--emit port-blockers --tree` —
*84 file(s) scanned, 67 declared no-port, 0 temporarily held, 17 owed* — where this file reads
`owed lines=96`. After the cut: 16 owed, 1930 lines.

**This cut is one of five units in a five-amendment iteration spanning gate-sdk, lifecycle-kit and
drift-kit**, so `check-stage-entry` assertion C is armed on the amendment **file count** alone
(≥2 component dirs) before any body token is read: the audit stage's stamp will be demanded at
build's entry. Stated here so the downstream entry does not discover it.

## What changes

### (1) The three `# spec:` pointers are re-homed, and that edit lands before the port

`demo/run-demo.sh:2` (the file header) and `:11` (the `DEMO_TMP_DIR` knob) move from
`CLAUDE.md §Housekeeping` to `gate-sdk/SPEC.md §Consumer smoke` {design-bearing}. `:35` already
names that owner and is untouched. The re-home is a **precondition of the cut's well-formedness**,
not tidying that rides along: until it lands, the composer's stated-contract test resolves this
file to a section that owns no mechanism, and a cut selected there would be selecting by size.

**It is ordered first and is separately landable.** Re-homing a pointer is a legal end state on its
own — the file would then be a correctly-homed owed singleton under §Consumer smoke, which is what
that section's own prose would say of it. The port is what §Consumer smoke then chooses to do with
it. Stating the order matters because the reverse order has no legal intermediate state: a port
whose deleting commit also moves the pointer never leaves a tree in which the mis-homing was
corrected, and the record would show a file ported out of a section that never claimed it.

**`check-spec-pointer` grades the header pointer and its verdict is the oracle for the re-home
alone.** The gate's subject is whether a pointer resolves; the *legality of the boundary* it
resolves to is what it cannot grade — the standing gap filed as `spec-pointer-boundary-legality`,
which names this file among its mis-pointers. So the re-home is cleared by the composer's own
sentence and by §Consumer smoke gaining the member, never by a green gate, and this delta says so
rather than letting a build session read a passing battery as clearance.

### (2) The cut takes §Consumer smoke's one owed non-class member and reopens no part of the class

§The port disposition's `no-port` class membership is **closed and enumerated**: "Every `smoke/`
install and violation recipe, and both members of this harness" — `bin/run-consumer-smoke.sh`,
`lib/consumer-smoke.sh`, and each kit root's `smoke/install.sh` and `smoke/violation.sh`
{mechanical}. `demo/run-demo.sh` is in none of those sets, and after delta 1 it is the section's
only owed file. Every class member keeps its declaration and its four measured legs untouched.

**The worked precedent for a validate-suite driver porting is inside this same kit's reach**:
context-kit's AGENTS.md smoke was a driver registered as an `EVIDENCE_KIT_RUN_<suite>` value by the
identical predicate, and it ported to `--agents-md-smoke`. This delta claims that precedent
explicitly so a later reader does not read "runs a battery in a scratch consumer" as class
membership.

### (3) The arm is `--run-demo`, an `Arm::Run`, and both halves are forced rather than chosen

The member ports as a **bridged-arm table** row — `("--run-demo", Arm::Run(demo::run), demo::KNOBS)`
in `native/src/emit/mod.rs`'s `BRIDGED_ARMS`, with the body in `native/src/emit/demo.rs`
{design-bearing}.

**`Arm::Run` is forced by the exit grammar, and this is where the natural instinct is wrong.** The
sibling port cuts in this kit have been `Arm::Emit` members admitted on the ground that they
"declare no 1 and never have", so the collapse to {0, 2} discards nothing. That ground is **false
here**: `fail()` at `:24` exits **1**, and every one of the walkthrough's six assertions reaches
it — the battery not green after install, the violation not turning it red, the wrong gate
catching it, the battery not returning to green. That 1 is the whole verdict the `demo` validate
suite reads. An `Arm::Emit` would collapse it into the dispatch-failure band and the suite would
report a broken demo as an environment error. The rule is already written for exactly this shape at
`--upgrade-smoke` — "an `Arm::Run` because its contract is the 1-versus-2 split of its exit status,
which an emitting arm collapses" — and at `--agents-md-smoke`, "an `Arm::Run` because the contract
is the verdict, which an emitting arm cannot carry".

**Table membership is forced by the forced-family test.** The member resolves `GATE_KIT_ROOTS_HERE`
(through `gate_kit_roots`, which decides *which* kits the walkthrough vendors) and
`GATE_SDK_NATIVE_BIN` (through `gate_native_bin`, twice: the scratch `.gitignore` line and the
binary placement). A hardcoded top-level flag receives neither, which §The non-gate arm calls the
difference between working and appearing to. The declared roster is those two and nothing else —
byte-identical to `agents_md_smoke::KNOBS`, which is the same pair for the same two reasons.

**No front-end edit is owed and none is legal.** `bin/run-gates.sh`'s residual argv grammar passes
every leading `-*` token through to the crate's own parser untouched, which is why
`bash gate-sdk/bin/run-gates.sh --run-demo` is reachable the moment the table row exists. Adding a
case for it would put a second spelling of the arm roster in the stub.

### (4) `csmoke_place_binary`'s spawn helper moves into `csmoke.rs` when it gains its second caller

The demo reaches the shared library for exactly one helper, `csmoke_place_binary`, and it must keep
reaching it: the helper sets its caller's `SCRATCH` and resolves `GATE_SDK_NATIVE_BIN` through
`lib/gate.sh`'s accessor, which is leg 1's own ground {design-bearing}. The ported arm therefore
takes the **duplication-absent road** both sibling arms already took — a `bash -c` that sources the
unchanged library through `native/src/emit/csmoke.rs`'s `SOURCE` prologue, supplies `SCRATCH` and
the host checkout on the way in, and reads the status back.

**The helper's spawn wrapper is today private to `upgrade_smoke.rs` and moves to `csmoke.rs` in
this cut.** §Consumer smoke already rules the shape — the spawn wrapper and the script prologue
"live in a module both arms call, never a copy each" — and that rule binds at the moment a second
arm calls the same helper, which is here. Leaving the wrapper where it is and calling it across
module boundaries would leave the producer named after one of its two callers; copying it would be
the second producer criterion 6 refuses.

**A reimplementation of the placement in Rust is refused, and not on effort.** `lib/gate.sh` rules
exactly one place a knob's value is computed; a crate-side placement would resolve
`GATE_SDK_NATIVE_BIN` a second time, which is the second producer that put `lib/consumer-smoke.sh`
in the `no-port` class in the first place. Porting the *caller* while leaving the *library* is the
whole shape of legs 1 and 2, and this cut is an instance of it rather than an exception to it.

### (5) The library's shell sourcer set empties, and three surfaces state the count it had

§Consumer smoke counts the library's sourcers and its builder's callers separately, and today reads
"What is left sourcing it is `demo/run-demo.sh` alone … so the sourcer set is **one** and the
builder's caller set is three" {design-bearing}. After this cut the sourcer set is **zero** and the
builder's caller set is unchanged at three. Two further surfaces carry the same count and both move
in the same commit: `lib/consumer-smoke.sh`'s own `# no-port:` cause, which says in terms "Its one
remaining shell sourcer is outside this cut and keeps sourcing it (demo/run-demo.sh, which takes
csmoke_place_binary alone rather than the builder)", and context-kit/SPEC.md §Testing, which records
the AGENTS.md cut leaving "`demo/run-demo.sh` alone".

**An empty sourcer set does not reopen leg 1, and saying so is this delta's point.** Leg 1's ground
is the config bridge — the library resolves a knob through `lib/gate.sh`'s accessor and therefore
sits inside the bridge rather than beside it — and that ground is a property of the library, not of
how many shells source it. The natural misreading is available and cheap: a library with no shell
sourcer left looks like a library whose only callers are compiled, which looks like a candidate for
going in-crate. It is not, and the paragraph that states the new count states the non-consequence
beside it, in the same place, so the two are read together.

### (6) `DEMO_TMP_DIR` survives the cut, read undeclared off the process environment

`:12` reads `BASE="${DEMO_TMP_DIR:-${TMPDIR:-/tmp}}"`. The ported arm keeps that resolution whole —
the knob, the `TMPDIR` fallback and the `/tmp` floor — and **declares neither name** in
`demo::KNOBS` {design-bearing}.

**Not declaring is forced, not preferred.** The config bridge resolves a declared knob by sourcing
exactly one kit's library and reading it back through `declare -p`, and partitions the declared set
by each knob's `<KIT>_` prefix. `DEMO_TMP_DIR` matches no kit prefix and is defined in no kit
library, so declaring it would meet §lib/gate.sh's undeclared-knob refusal and **fail-close the arm
on every invocation** — the exact ruling `--emit-session-id` already carries for names read straight
off the process environment and defined in no kit library. `TMPDIR` is a standard environment
variable and is on the same footing.

**Retiring the knob was weighed and is refused on a recorded ruling rather than on taste.** It is
tempting: a tree-wide `git ls-files` search returns the name in **one** file, the script's own two
lines, so it has no setter, no config seam and no documentation outside the pointer delta 1 is
already fixing. But `native-gate-port-remaining-corpus`' ruling (1) is that **a cut narrows the
port, never an extension point**, and an override a consumer of this repo may set on the command
line is an extension point however thinly used. Retiring it is a separable decision that no ruling
reaching this iteration authorizes, and folding it in here would be a port cut spending an envelope
it was not given. What the cut *does* owe it is an owner: delta 1 moves the knob's contract onto
§Consumer smoke, where a reader can find it, from a section that never mentioned it.

**Folding it onto `GATE_SDK_TMP_DIR` is refused separately and on behaviour.** That knob defaults to
the in-repo `.tmp` and is absolutized at the invoker's root, while this base must be a temp root a
scratch git repository and a full kit vendoring can be built under and torn down — and the
walkthrough's whole claim is that it runs on the consumer-smoke mechanics, whose builder reads
`${TMPDIR:-/tmp}`. Keeping `TMPDIR` keeps that parity; `GATE_SDK_TMP_DIR` would break it silently.

### (7) Two behaviour changes, named as two, and the first is a defect the port repairs

**(7a) The exit grammar becomes 0/1/2, which is what §Consumer smoke already specifies.** That
section states "Exit codes follow the gate convention (0 all hold, 1 an assertion failed, 2
usage/environment)" {design-bearing}. The shell form does **not**: it has no exit-2 path at all, and
all seven `fail()` sites at `:24` return 1 — including the two that are environment-class rather
than finding-class, `:43` (the native binary could not be placed in the scratch consumer) and `:48`
(a vendored kit's installer errored). Neither is a statement about the adoption arc; both are a
statement that the harness could not be stood up. The ported arm maps them to **2** and the five
genuine assertion failures to **1**, the three-outcome shape `agents_md_smoke.rs` already carries as
a type so that "a finding about the consumer cannot be raised on a precondition's spelling or the
reverse".

**This is a behaviour change under `EVIDENCE_KIT_PARSER=exit-code` and is named rather than folded
into the port.** The `demo` suite declares no parser, so it falls to that default and its whole
product is the status; a run that could not place the binary stops reporting as a failed walkthrough
and starts reporting as an environment failure. That is the correct report and it is not the report
the tree makes today. It is also the change delta 3's family choice exists to make possible — an
`Arm::Emit` collapses the very distinction being drawn.

**(7b) An operand is a refusal.** The shell form takes no arguments and inspects none, so
`bash demo/run-demo.sh --keep` runs the full walkthrough and ignores the word {design-bearing}. The
ported arm prints `usage: --run-demo` on stderr and exits 2 instead. Its ground is 7a: once the
2 band means "the harness could not be stood up", an operand silently swallowed is the one way a
caller believes it selected a mode and reads a verdict about a different run. The sibling arm
accepts `--keep` and this one accepts nothing, which is a difference worth stating rather than
smoothing — the demo's scratch is narrated and torn down as part of the walkthrough's own arc, so
there is no mode to keep.

### (8) The named callers, re-pointed in the same commit; three of them are one atomic edit

Every tracked surface naming the script moves to `bash gate-sdk/bin/run-gates.sh --run-demo`,
enumerated over `git ls-files` this session rather than assumed {mechanical}:

- `scripts/evidence-config.sh:25` — `EVIDENCE_KIT_RUN_demo='bash demo/run-demo.sh'` becomes the
  arm. **The suite name `demo` does not change**, so `EVIDENCE_KIT_SUITES` is untouched and the
  validate stage's own roster is unmoved. This is the knob-value re-point §Porting a gate to the
  binary substrate already sanctions in terms — "a file this repo names as a knob's **value** … is
  what the seam resolves, not the seam: no adopter edits it, and its whole documented purpose is to
  run. Porting one moves its mechanism into the binary and re-points the value" — which is also why
  the extension-point ruling (1) does not reach this file.
- `README.md:140` — **inside the `<!-- battery-roster:begin -->` block**, which
  `check-battery-roster` holds to `EVIDENCE_KIT_RUN_<suite>` in both directions: an undocumented
  suite reds assertion A and a roster line resolving to no suite reds assertion B. So the knob
  value, this roster line and the generated hook that bakes the value are **one atomic edit** —
  either half alone is red. This is the correction most likely to be missed, because the line reads
  as an ordinary cheat-sheet entry.
- `README.md:25` and `docs/index.md:27` — the two quick-start invocations, in fenced blocks.
- `README.md:30` — **a markdown link to the path**, `[`demo/run-demo.sh`](demo/run-demo.sh)`, so
  the delete makes it dangle under `check-md-refs` rather than merely reading stale.
- `scripts/root-allowlist.list:35-36` — the comment and the `demo` entry allowlisting the directory
  at the repo root. The cut empties `demo/`, so the entry is retired with the directory.
- `delegation-kit/SPEC.md:2790` — `--wait-probe`'s precedent sentence, "Its precedent for existing
  at all is `demo/run-demo.sh`, a runnable artifact a session invokes by hand." The precedent
  survives the port and reads *stronger* — the artifact is now an arm, which is what `--wait-probe`
  is — but the sentence names a path that will not exist and is rewritten with it.
- `gate-sdk/SPEC.md:11640` — "the uncovered set is the twelve files under `installer/` and `demo/`",
  a count this cut moves and a set it empties on one of its two legs.
- `CLAUDE.md §Housekeeping` — the walkthrough's always-loaded bullet, whose invocation and whose
  claim to own the file's mechanism both move under delta 1.
- `TRAJECTORY.md` — the residue-class clause offering `demo/` the per-file `# no-port:` branch. The
  fork is ruled closed, so the clause records a spent alternative and is retired rather than
  reversed.

**The `.claude/settings.json` grant drops in the deleting commit**, in-cut and with no out-of-band
step, under `native-gate-port-remaining-corpus`' ruling (2) as widened 2026-09-05.
`"Bash(bash demo/run-demo.sh)"` at `:34` is a literal `.sh` command token with no `*`, squarely
inside `check-settings-paths`' scope. **The hook will not catch it and that is why the removal is
in-cut**: the generated hook matches staged `ACMR` paths, so a *deleted* `.sh` never fires its
trigger, and what would otherwise catch the stranded grant is the whole-tree battery alone.

### (9) The parity oracle is a captured before/after comparison, and no parity arm is minted

There is no golden and none is possible: the walkthrough asserts against a live battery whose gate
count and whose violation text move between commits, so no byte-comparable expectation file exists
{design-bearing}. Its only committed record is the validate baseline's `demo demo pass` row, which
is one bit.

**The oracle is the one this SPEC already rules for exactly this situation** — a one-time
line-for-line comparison of both forms' stdout, taken in the deleting session on one tree, because
"a comparison a session runs before a delete is evidence; an arm that can only skip after it is
not". The stdout is the right subject rather than the status: `:26` says in terms that the verbose
battery roll is the demo's payload, and the four-act narration is what `README.md` and
`docs/index.md` describe to a reader as the observable behaviour. **A parity arm is refused on that
same rule**, the port deleting the only shell holder, so such an arm could only ever skip.

### (10) §The non-gate arm's roster gains the member, undated

The class roster gains `--run-demo` with its owning section named and **no landing-date cohort
label** {mechanical}, for the reason the sibling cuts state: those labels are provenance, which
`kit-spec-provenance-seam-sweep-remainder` retires, so a dated label would be landed and removed
inside one iteration.

## Producers and consumers

**New interface: the `--run-demo` bridged arm.**
*Producer* — `native/src/emit/mod.rs`'s `BRIDGED_ARMS` row, dispatched in `native/src/main.rs`; its
enabling configuration is the bridged environment `gate_knob_env` builds for it, which
`bin/run-gates.sh` resolves and execs. Reachable with no front-end edit, the residual argv grammar
passing a leading `-*` token straight through. *Consumers* — the `demo` validate suite, through
`EVIDENCE_KIT_RUN_demo`, reading the arm's **exit status** at the validate transition; and a human
reader, through stdout, at any of the four documented invocation sites.

**The verdict is the only field, and its reader is named per value.** The suite declares no
`EVIDENCE_KIT_PARSER_demo`, so it falls to `EVIDENCE_KIT_PARSER=exit-code` and the status *is* the
suite's whole product. Exit 0 is read by evidence-kit's suite runner as the suite passing and by a
session as the arc having behaved; exit 1 is read by the same runner as a **demo failure** — a
genuine finding about the adoption arc — and carries the `DEMO: FAIL — <cause>` line on stdout
naming which act broke; exit 2 is read as a precondition the harness could not meet, which under
delta 7a is where the binary-placement and installer-error paths move. No field is added beyond the
status and the narration the shell form already printed.

**The knob `DEMO_TMP_DIR` keeps its producer and gains a documented owner.** *Producer* — the
invoking environment, read directly by the arm and declared in no roster (delta 6). *Consumer* —
the arm's scratch-base resolution, at the transition before `mktemp`. Its enabling configuration is
the process environment itself, which needs nothing emitted; what it lacked was a section stating
its contract, and delta 1 supplies one.

**Existing interface whose caller-kind changes: `csmoke_place_binary`.**
*Producer* — `gate-sdk/lib/consumer-smoke.sh`, unchanged, permanently shell under leg 1. *Consumer*
— was `demo/run-demo.sh` by sourcing; becomes `native/src/emit/demo.rs` by spawn, through
`csmoke.rs`'s shared prologue, at the transition where the arm has vendored the kit roots and
before it runs the scratch battery. The enabling configuration is actually emitted:
`GATE_SDK_NATIVE_BIN` and `GATE_KIT_ROOTS_HERE` are both resolved for this arm by the same bridge
that already resolves them for `--agents-md-smoke`, so the live configuration exercises the path
rather than only a test.

**Existing interface whose producer moves: the demo's spawned battery.** *Producer* — was
`run_battery()`'s `( cd "$SCRATCH" && GATE_SDK_VERBOSE=1 bash gate-sdk/bin/run-gates.sh )`; becomes
the arm spawning the **vendored consumer's own front-end** with the same `GATE_SDK_VERBOSE=1`.
*Consumer* — the arm's own three assertions over that output. **It stays spawned and that is
load-bearing**: the subject under test is the vendored consumer's battery, not this repo's, and
calling the runner in-process would run the host's registry against the scratch tree, which is the
pairing defect `csmoke_place_binary`'s own comment records `upgrade-smoke` having met.

**The spawned-program set is `bash`, `git`, `mktemp`, `cp`, `rm` and `awk`, plus whatever the
vendored kits' installers spawn** — unchanged from the shell form, and it joins the prose paragraph
in §The non-gate arm that records those sets. It is **not** a consumer-changeable set: no knob names
a program here, which is the axis on which `--emit-always-loaded` and `--emit-env-probe` differ from
it.

**This delta set narrows two corpora, so each reader's red condition is named rather than its
subject** (§The causal-completeness check, point 5). The narrowings are the tracked shell corpus,
which loses one file, and the accepted-invocation set, which delta 7 tightens.

- `check-measured-claim` reds on a **value mismatch** between an emitted key and a surface's marker.
  Its `tree-shell-owed` key is read live off `--emit port-blockers --tree` and moves 17 → 16. No
  governed surface carries a `tree-shell-owed=` marker, verified over `git ls-files` this session,
  so no prose value is owed — but the **generated pre-commit hook bakes the resolved value**
  (`CANON_KIT_MEASURED_VALUES=…\t17`), which is why the hook regeneration below is mandatory rather
  than cosmetic.
- `check-settings-paths` reds on a **zero-resolution** for a literal `.sh` command token, which is
  why delta 8's grant removal is same-commit rather than tidy-up. Non-monotone under the narrowing:
  the narrowing *adds* this violation.
- `check-spec-pointer` reds on a pointer that **resolves to nothing**. Delta 1 moves two pointers
  onto a section that exists; deleting the file removes all three from the corpus. Monotone, clears
  by inspection.
- `check-exec-bit` reds on an **executable-bit mismatch** over its glob; its corpus loses a member.
  Monotone, clears by inspection.
- `check-graph` and the generated hooks red on a **byte mismatch** against a fresh emission. Both
  stale here on two independent triggers — the moved measured value above and the changed
  `EVIDENCE_KIT_RUN_demo` value the hook bakes verbatim — so this is non-monotone in both directions
  and clears only by regenerating and committing.
- `check-docs-mirror-fresh` reds on a **byte mismatch** against a fresh mirror; the gate-sdk,
  context-kit and delegation-kit SPEC edits all reach it. Non-monotone; regenerate.
- `check-gate-binary-fresh` reds on a **stamp mismatch** against a `git ls-files`-derived tree,
  which is why the new crate source is staged **before** `build-native.sh` runs and not after.
- `check-md-refs` reds on a **link target that does not resolve**, which is what `README.md:30`'s
  link to the deleted path becomes. Non-monotone: the narrowing *adds* this violation, and this is
  the second reader in the set with that property.
- `check-battery-roster` reds **in both directions** — assertion A on a suite whose normalized
  invocation matches no roster line, assertion B on a roster line matching no suite. Neither is
  monotone, and together they are why delta 8's three sites are one commit.
- evidence-kit's suite runner reds on a **non-zero exit** from `EVIDENCE_KIT_RUN_demo`. Its corpus
  is unchanged — the suite name does not move — so the only way this reader notices the cut is by
  the arm behaving differently, which is exactly the assertion delta 7a wants it making.
- `check-knob-citation` reds on a knob **cited in governed prose and defined nowhere**, or the
  reverse. Delta 6 keeps `DEMO_TMP_DIR` and delta 1 gives it a SPEC home, so this reader gains a
  citation where it had none. It is named because a knob newly cited in a governed manifest is
  exactly this gate's subject, and because the natural alternative — retiring the knob — would have
  been the case to check it against.

No reader in the set reds on finding none, asserts an exact count, or holds a coverage floor over
either narrowed corpus. The three non-monotone readers are named above and none clears by
inspection.

## Existing sections updated

- **gate-sdk/SPEC.md §Consumer smoke** — gains the member's port record: the re-home and its ground,
  the arm and its two forced halves, the spawn seam onto `csmoke_place_binary`, and the sourcer
  count moving to zero beside the statement that leg 1 does not reopen (deltas 1, 2, 3, 4 and 5).
- **gate-sdk/SPEC.md §The port disposition** — the class-membership paragraph gains the sentence
  distinguishing the enumerated `no-port` class from this section's one owed non-member, so a later
  reader does not have to derive non-membership from an enumeration (delta 2).
- **gate-sdk/SPEC.md §Porting a gate to the binary substrate** — the mis-homed-pointer corollary
  gains its first worked instance; the ruling is unchanged and the instance is what it lacked
  (delta 1).
- **gate-sdk/SPEC.md §Consumer smoke's exit-code sentence** — the convention the shell form never
  held: the section states 0/1/2 and gains the member's own mapping, naming which two failure sites
  are environment-class (delta 7).
- **gate-sdk/SPEC.md §The non-gate arm** — the class roster gains `--run-demo`, undated; the
  spawned-program prose gains this member's set with its not-consumer-changeable note; and the
  `DEMO_TMP_DIR` case joins `--emit-session-id`'s as a second instance of a name read off the
  process environment that may not be declared (deltas 3, 6, 7 and 10).
- **gate-sdk/SPEC.md §The port disposition's owed-corpus prose** — the count every cut moves: 17
  owed / 2026 lines becomes 16 / 1930, and what stays owed to this tree is the installer family and
  the four reachable non-installer members (delta 2).
- **gate-sdk/SPEC.md's uncovered-set sentence at §check-graph's coverage prose** — "the twelve
  files under `installer/` and `demo/`", a count this cut moves and a directory it empties
  (delta 8).
- **`gate-sdk/lib/consumer-smoke.sh`'s `# no-port:` cause** — the clause naming
  `demo/run-demo.sh` as the one remaining shell sourcer, rewritten to the empty sourcer set with the
  leg-1 ground restated as the thing that did not change (delta 5).
- **context-kit/SPEC.md §Testing** — its "leaving `demo/run-demo.sh` alone" clause, which this cut
  falsifies (delta 5).
- **delegation-kit/SPEC.md §bin/wait-probe** — the precedent sentence naming the script by path
  (delta 8).
- **`CLAUDE.md` §Housekeeping** — the walkthrough bullet: the invocation moves to the arm, and the
  section stops being cited as the file's mechanism owner (deltas 1 and 8).
- **`TRAJECTORY.md`** — the residue-class clause offering `demo/` the per-file `# no-port:` route,
  retired as a spent alternative now the fork is ruled closed (delta 8).
- **`scripts/evidence-config.sh`**, **`README.md`** (the two invocations, the linking sentence, and
  the `battery-roster` block line), **`docs/index.md`**, **`scripts/root-allowlist.list`** and
  **`.claude/settings.json`** — the invocation lines, the dangling link, the retired root entry and
  the dead grant (delta 8).
- <!-- update-target-exempt: generated projections with their own freshness gates and regen commands, rostered in docs/site-architecture.md §Generated projections and their freshness gates; each stales on a value or a byte no delta authors --> the generated `pre-commit` and `commit-msg` hooks, `docs/check-graph.html`, and the on-site SPEC mirror.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and
      a named consumer; every new field has a named reader at a named transition.
- [ ] **The re-home lands first and separately** — delta 1's pointer edit is its own commit, so the
      record shows the file correctly homed under §Consumer smoke before that section ports it.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); the merged spec reads as one coherent document a reader who never saw
      the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec, config, template, doc, crate source, allowlist
      and settings file for `run-demo`; nothing dangles.
- [ ] **The captured comparison is taken before the delete, not after** — both forms' stdout on one
      tree, line for line, in the deleting session; the four-act narration and exit 0 agree, and the
      `demo` validate suite is green in the battery. There is no arm to fall back on afterwards.
- [ ] **The three-site roster edit lands in one commit** — `EVIDENCE_KIT_RUN_demo`, `README.md`'s
      `battery-roster` block line, and the hook regeneration; `check-battery-roster` reds on either
      half alone.
- [ ] **The new crate source is staged before `bash gate-sdk/bin/build-native.sh` runs**, the tree
      `check-gate-binary-fresh` compares being `git ls-files`-derived; the hooks and the graph are
      regenerated after the delete, both measured values having moved.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
