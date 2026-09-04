# SPEC amendment: spine-cut

The port disposition of **`evidence-kit/bin/run-validate.sh` (124 lines), the one
owed file declaring §bin/run-validate.sh**, onto the binary substrate as a bridged
`Arm::Run` member spelled `--run-validate`. This is a stated-contract cut under the
port-only run (TRAJECTORY.md §PRIORITY DIRECTIVE), composed at scope and packaged by
the **lead on its own authority**, 2026-09-04, over the resume channel; it did not
reach the operator, and that is stated because a packaging ruling recorded without
its authority reads later as more settled than it is.

**Measured at this HEAD rather than carried from scope's survey**: the port oracle's
`--tree` arm reads 98 files scanned, 64 declared `no-port`, 0 temporarily held,
**34 owed**. The selection ground is that owed column and never the registry
`--group` read, which trails zero groups and cannot answer for the tree remainder
at all (gate-sdk/SPEC.md §port-blockers). This cut takes one file of that column,
and **no ported member is a gate** — §bin/run-validate.sh already rules the tool
"Not a gate — a `bin/` tool exercised end-to-end in `smoke/`", so this member's
whole port surface is the non-gate class.

**The cost this member hides from `lines=` is the third recorded axis, composition
over a permanently-shell library** (gate-sdk/SPEC.md §The first cohort, and the rule
that selects the next). The column reads 124. The cut is that file **plus five
compiled twins** of `lib/evidence.sh` primitives, and the attested precedent for the
shape is guard-kit's ranker, "whose column saw a single-file port and whose real cut
was that file plus three twins, a parity arm and a harness". Two of the five twins
are this member's alone; three are shared with the sibling cut on
§bin/diff-baseline.sh, which **consumes** them. That sharing is a between-cuts
economy and **not** a section pairing: the two sections are separate `###` headings,
neither amendment claims a shared proof, and the parent `## Per-component contracts`
is not read here as a cut boundary — whether a `##` parent may bound a cut is the
open question `cut-boundary-section-legality-unruled` holds and a scheduled consult
will reach.

**And the economy has a direction, which prices the drop order.** While
`bin/diff-baseline.sh` stays shell it keeps sourcing the *shell* twins, so this cut
landing alone leaves a live double implementation of the parser and the diff. The
sibling cut is what retires it. A batch that drops this member strands nothing; a
batch that drops the sibling strands a duplication this member created.

## What changes

### (1) The cut is the one owed file declaring §bin/run-validate.sh, and its directive reach is three sections

`bin/run-validate.sh` is the **only** owed file declaring `### bin/run-validate.sh`
{design-bearing}. The kit's other owed member, `bin/diff-baseline.sh`, declares
§bin/diff-baseline.sh; `lib/evidence.sh` is permanently shell as the config bridge's
sole `EVIDENCE_KIT_*` resolver (gate-sdk/SPEC.md §The kit-library port disposition);
`checks/` holds one member and it is already compiled. So the stated-contract
composer's *one section, the one amendment* is satisfied by a clean singleton.

**The file's own `# spec:` directives reach further than its titular section, and
the amendment is written to that reach rather than to the heading.** Six directives,
probed rather than counted from memory: `:2` binds §bin/run-validate.sh; `:29`,
`:43` and `:55` bind §The producer-liveness lock; `:64` and `:113` bind §Evidence
manifest. A rewrite bounded to the titular section would leave two sections
describing a shell file that no longer exists, so all three are update targets
below.

**This cut discharges the section and takes the kit from two owed files to one.**
The remainder is stated so no reader reads a discharged section as a discharged kit:
`bin/diff-baseline.sh` (53) stays owed on §bin/diff-baseline.sh until its own cut
lands, and `lib/evidence.sh` (permanently shell) never enters the column.

### (2) `--run-validate` is a bridged `Arm::Run` with its own front-end case arm, and the three-state exit forces the family

The spine lands as `native/src/emit/run_validate.rs` and registers as one
`BRIDGED_ARMS` row beside the thirty-seven already there, with its own `case` arm in
`gate-sdk/bin/run-gates.sh` beside `--enter-stage` {design-bearing}.

**It cannot be an `--emit-` member, and the exit contract settles it without
appeal to taste.** `native/src/main.rs:483-491` maps `Arm::Emit` onto `exit(0)` for
`Ok` and `exit(2)` for `Err`, so that family collapses to `{0, 2}` and can never
return 1. §bin/run-validate.sh's own contract is **three-state** and the section
says so in as many words — "this is why the tool has **two** non-zero exits with
different meanings: exit 1 when a suite records `new-failures` — the verdict — and
the guards' exit 2 when the run cannot start at all". An `--emit-` spelling would
rewrite the verdict to the refusal code by construction, and nothing in the battery
would report it. That is the crossing clause gate-sdk/SPEC.md §The bin/-tool
contract records at its third reader instance, met here from a fourth direction: the
collapse would make *a suite regressed* indistinguishable from *the run could not
start*, on the one tool whose entire product is that distinction.

`Arm::Run` (`native/src/main.rs:493`) passes the arm's own `i32` through verbatim,
and `exec_arm` ends in a true `exec` (`gate-sdk/bin/run-gates.sh:127`), which
replaces the shell's process image — so the binary's status *becomes* the
front-end's and `ARM_UNAVAILABLE_STATUS` (`:113`) stays reachable only on the two
pre-dispatch failure paths.

**The spelling is its own rather than `--emit-`, and the front-end grammar is why.**
`bin/run-gates.sh` composes `--emit-<name>` from its `--emit <name>` operand, so the
two are one decision (gate-sdk/SPEC.md §The non-gate arm). This member emits no
document at all: it runs a roster of suites, writes a tracked manifest and returns a
verdict. It joins the arms that take their own spelling for their own contract —
`--lesson-sink`, `--upgrade-smoke`, `--install-lifecycle`, `--usage-verdict`,
`--wait-probe`, `--enter-stage`.

**The `bin/`-tool contract's split binds, and this member's shape half is empty.**
`run-validate.sh` takes **no positional argument at all** — it reads its whole input
from the bridged environment. So the shape refusal and the `--` escape have no free
text to bind on, and the `-h`/`--help` arm retires to the front-end as it does for
every member of the class. Stated rather than skipped, because
`bin-tool-help-arm-absent-tree-wide`'s open design question is precisely whether the
contract binds a tool taking **no** positionals, and this cut must not be read as
answering it: the port makes the question moot for this one file by moving usage to
the front-end, and rules nothing about the seventeen.

**A census note is owed at build, and this member is one of the four that move it**
— probed, `run-validate.sh` is on the census (`git ls-files '*/bin/*.sh' | xargs
grep -L -- '--help'` names it alongside the other three cut members). The entry's
own derivation is not reproducible as written: it states 20 paths / 17 shipped
tools measured 2026-09-04, while the obvious spelling of its own command returns
19 at this HEAD, three of them fixtures, so 16 shipped. The pattern is unstated,
so the difference is unattributable rather than a proven staleness; build
reproduces the census with the entry's own pattern before moving the number.

### (3) The declared knob roster, and the two prefix families it carries

The arm declares thirteen names {design-bearing}:

`EVIDENCE_KIT_SUITES`, `EVIDENCE_KIT_RUN_*`, `EVIDENCE_KIT_PARSER`,
`EVIDENCE_KIT_PARSER_*`, `EVIDENCE_KIT_BASELINE_FILE`,
`EVIDENCE_KIT_MANIFEST_FILE`, `EVIDENCE_KIT_SKIP_FILE`, `EVIDENCE_KIT_QUEUE_FILE`,
`EVIDENCE_KIT_STATE_FILE`, `EVIDENCE_KIT_TMP_DIR`, `EVIDENCE_KIT_LOCK_FILE`,
`EVIDENCE_KIT_RUN_ID`, `EVIDENCE_KIT_PRE_HOOK`.

**The forced-family test settles the registration and leaves nothing to
calibrate**: every one of those values is defined and defaulted in
`evidence-kit/lib/evidence.sh` (`:34-48`), a `no-port` file the bridge sources to
resolve them, so a hardcoded top-level flag would resolve platform defaults and
silently ignore every consumer override — the difference gate-sdk/SPEC.md §The
non-gate arm calls "between working and appearing to". Only a bridged-arm table
member is bridged.

**Two of the names are prefix families, and the family arm is precedented rather
than minted.** `_gate_knob_emit` (`gate-sdk/lib/gate.sh:245-247`) routes a declared
name ending in `*` to `_gate_knob_prefix_emit` (`:318-350`), which resolves the whole
family **inside the owning kit's already-sourced subshell** — which is what puts a
consumer config's loop-declared variables in scope, and
`scripts/evidence-config.sh:8-12` declares one `EVIDENCE_KIT_RUN_<kit>` per kit in
exactly such a loop. A prefix matching nothing is an empty family and passes.
`EVIDENCE_KIT_SUITES` beside `EVIDENCE_KIT_RUN_*` is already a live declared pair on
two arms (`native/src/emit/mod.rs:180-181`, `:197-198`) and three registry rows
(`native/src/gates/mod.rs:1248`, `:1280`, `:1642`), so this member copies a working
declaration.

**`EVIDENCE_KIT_PARSER_*` is declared nowhere in the crate today and this cut mints
its first declaration** — probed, not assumed. Its receiving half already exists:
`walk::knob_prefix` (`native/src/walk.rs:220-227`) and `walk::knob_in_family`
(`:232-237`), whose own directive already rules the semantics this member depends
on — *a prefix is a resolution set, never a roster*, because "a reader enumerating
`knob_prefix` instead would publish `EVIDENCE_KIT_RUN_ID` as a suite" (`:229-231`).
The roster is `EVIDENCE_KIT_SUITES`; the family answers *what is this suite's value*.

**The `EVIDENCE_KIT_RUN_ID` overlap is pre-existing and is not port-introduced.**
`ek_suite_cmd` computes `EVIDENCE_KIT_RUN_<suite>`, so a suite literally named `ID`
already collides with the run-id knob in the shell form. The port preserves the
collision rather than resolving it, because resolving it would be a behaviour
widening a faithful port may not take on its own authority; it is named here so a
later reader does not read the family declaration as having introduced it.

### (4) Five compiled twins, and the library's own header is the licence for them

`native/src/evidence.rs` already carries five of the kit's ten `ek_*` primitives —
`data_lines` (`:13`), `queue_iteration` (`:25`), `state_stage` (`:37`/`:56`),
`lock_read` (`:73`) and `pid_alive` (`:132`). This cut pays the five that are
missing: `ek_run_key`, `ek_suite_cmd`, `ek_parser_for`, `ek_parse` and `ek_diff`
{design-bearing}.

**The caller census is closed and was probed rather than reasoned.**
`ek_run_key` (`run-validate.sh:14`) and `ek_suite_cmd` (`:68`) have exactly **one**
production caller each — this file. `ek_parser_for`, `ek_parse` and `ek_diff` have
exactly **two** — this file (`:91`, `:89`, `:99`) and `bin/diff-baseline.sh`
(`:30`, `:40`, `:41`). Every other hit in the tree is `lib/evidence.sh`'s own
dispatch or one of the library's gate-tests sourcing it directly to test the
adapter. No third production caller exists anywhere.

**The cut also empties the caller set of the two twins already compiled before this
iteration, and that closes a duplication rather than opening one.** `ek_lock_read`
and `ek_pid_alive` (already compiled at `native/src/evidence.rs:73`/`:132`) have
exactly **one** production caller in the whole tree today — this file, at `:32`,
`:48` and `:52` — probed the same way as the five above; `bin/diff-baseline.sh`
calls neither. `evidence-kit/gate-tests/evidence-lib-parity.test.sh` exists to hold
the shell and compiled forms of these two to their classification parity, and its
own header states why: "`bin/run-validate.sh` still calls both..., so the caller set
does not empty and the duplication is permanent — which is criterion 6's *unless*
clause". This cut removes that one caller, so the premise the test's own comment
states goes false the moment it lands. §The non-gate arm's parity-arm-retirement
rule — "caller is the second holder, so the arm retires with it", already the
ground the sibling cut cites for its own three twins' library tests — applies here
on the same terms: with no production caller left in either substrate, the shell
forms of `ek_lock_read` and `ek_pid_alive` retire from `lib/evidence.sh`, and
`evidence-lib-parity.test.sh` retires with them, in this cut's own deleting commit.
Nothing needs re-pointing in its place — unlike delta (10)'s two behavioral suites,
this one has no surviving subject to re-point at, because both forms of the thing it
compared are about to have zero production callers on the shell side.

**The class ruling does not foreclose this, and the licence is written into the
library's own header rather than inferred.** `evidence-kit/lib/evidence.sh:3` rules
the *file* permanently shell as the config bridge's sole resolver, and says in the
same breath that "the parser adapters beside those defaults are a separate question
this ruling does not reach and does not foreclose". So the twins are lawful, and the
shell forms **survive** — the library is not narrowed by this cut, because the
sibling shell caller still sources them and because the file is a `no-port` member
that never enters the owed column either way.

**What each twin owes across the seam, stated so build does not re-derive it:**

- `ek_run_key` — the queue iteration, else `EVIDENCE_KIT_RUN_ID`, else a failure the
  caller turns into the guards' exit 2. `queue_iteration` is already compiled, so
  this twin is a two-branch composition over an existing primitive.
- `ek_suite_cmd` — `knob_in_family(&run_family, suite)`, per delta (3).
- `ek_parser_for` — the per-suite override else the global `EVIDENCE_KIT_PARSER`
  (kit default `exit-code`), the same family lookup against the parser family.
- `ek_parse` — the three-arm dispatch: `exit-code` yields one `<suite> pass|fail`
  line off the status; `libtest` yields one `<name> pass|fail|ignore` line per
  libtest result token; anything else is a **consumer command** and is delta (5)'s.
- `ek_diff` — four inputs (baseline, suite, observed, skip file), **exactly two**
  printed line shapes and no third: `new-failure <suite> <scenario>` and
  `recovery <suite> <scenario>`; the skip channel demotes an observed pass to `skip`
  before the pass/fail branch; the status is 1 the moment a new-failure fires and 0
  otherwise, so a recovery-only diff is clean.

### (5) Two consumer seams survive the port unnarrowed, and that is the host's ruling (1) applied twice

`native-gate-port-remaining-corpus`'s ruling (1) — a cut narrows the port, never an
extension point — binds twice here, and the second instance is the sharper one
{design-bearing}.

- **`EVIDENCE_KIT_PARSER_<suite>`.** A parser value that is neither `exit-code` nor
  `libtest` is a consumer command run against the log, and this tree's own config
  uses that seam twice — `scripts/evidence-config.sh:18` and `:22` name
  `bash gate-sdk/bin/run-gates.sh --emit parse-gates-log` and
  `--emit parse-smoke-log`. The compiled arm therefore still word-splits the
  configured value and **spawns** it, exactly as `lib/evidence.sh:110-113` does.
  Compiling the two shipped adapters' behaviour into the arm and short-circuiting
  the spawn is the refused alternative: it would silently privilege the bundled
  adapters over a consumer's own, which is the seam narrowing the ruling forbids,
  and it would make the two shipped `--emit-parse-*` arms unreachable through the
  path their own sections document.
- **`EVIDENCE_KIT_PRE_HOOK`.** The optional per-suite pre-hook is a consumer command
  word-split by design (`run-validate.sh:74-80`) and stays one. A failing pre-hook
  aborts the run at exit 2 with no evidence appended, and that ordering is contract
  rather than implementation: it is what keeps a refused run from writing a line.

**So the arm's spawned-program set is wider than a reader would guess, and it is
recorded in prose because a `BRIDGED_ARMS` row carries no requirement element and
`--needs` answers about registry members only** (gate-sdk/SPEC.md §The non-gate arm).
The set is: `bash` and whatever the configured parser and pre-hook commands name,
plus each suite's own configured run command — which in this tree includes `cargo`
(`scripts/evidence-config.sh:31`). This member therefore joins `--upgrade-smoke` and
`--emit-env-probe` on the short list whose set a consumer can change, and it is the
first whose set includes **the subject under test**.

### (6) The producer-liveness lock's writer half crosses the seam behaviour-identical, and the trap becomes a destructor

Everything §The producer-liveness lock asserts about the *writer* is preserved, and
each property is named separately because a port could lose any one of them
independently while the others held {design-bearing}:

- **The record's bytes.** One line, `pid=<n> run=<key>`, and no other field. The
  same grammar is the one a backgrounded shell child's `<key>.run` launch record
  uses, so a writer that drifted here would leave its own producer invisible to
  `check-producer-liveness` **and** to guard rule 14 while every battery stayed
  green. `check-producer-liveness.gate:1` couples `.tmp/run-validate.lock`,
  `.tmp/*.run` and the two already-compiled reader modules; it does **not** name the
  writer, so nothing static holds this and delta (11)'s comparison compares the
  record bytes for exactly that reason.
- **The claim is atomic create-exclusive, never check-then-write.** The record is
  built whole in a temp file under the scratch dir and `ln`ed into place, so the
  claim fails if the target exists. `mkdir` and `set -C` are refused by the section
  as atomic on only the first half, and that refusal crosses unchanged — the
  compiled form uses the same hard-link primitive rather than an `O_EXCL` create,
  because the section's asserted mechanism is the link and a second spelling would
  be a second contract.
- **The claim's placement.** After the preflight guards and the scratch `mkdir` — a
  run that refuses to start must not claim — and before the batch file is created.
  §bin/run-validate.sh asserts this placement rather than leaving it to the
  implementer, so it is an ordering the port carries, not an artifact of the shell
  file's line order.
- **Reclaim exactly once.** A live PID refuses immediately, naming the blocking run
  key; a dead or vanished holder is reclaimed once and retried once; a second
  failure refuses rather than loops; a lock that does not parse refuses outright.
  `ek_lock_read`'s three-status contract — 0 read, 1 no lock, 2 unparseable, the
  last **never** treated as free — is already compiled at
  `native/src/evidence.rs:73` and is the twin the reclaim branch reads.
- **Release is conditional.** Only if the record is still ours. The shell form is an
  `EXIT` trap; the compiled form is a destructor, `impl Drop`, which is the idiom
  §bin/run-validate.sh already names as the precedent's compiled spelling —
  "what the precedent carries across substrates is the reclaim-on-every-path
  property, not the mechanism", written of `--enter-stage`'s `impl Drop for Scratch`.
  So this is convergence onto a landed in-crate precedent, not a new mechanism.

**The one honest limit the port does not remove**: `Drop` does not run on a
`SIGKILL`, and neither does an `EXIT` trap. The section already rules that residual
inert — a leaked record is made harmless by the readers' PID-liveness predicate and
removed by the scratch-boundary wipe — so the property set is unchanged, and saying
so is what stops a later reader reading the substrate change as a change in
guarantee.

### (7) The manifest fold is one write after the last suite, and the awk block becomes Rust

The run accumulates its rows in a scratch batch file and touches the tracked
manifest **once**, after the whole roster has run, so no suite runs against a tree
the spine has already written to {design-bearing}. The fold supersedes this
iteration's prior line for every suite the run covered and re-appends the batch in
roster order, so a repeated run leaves the line order unchanged.

The `awk` program at `run-validate.sh:115-118` is the fold, and it becomes Rust: a
two-pass read where the batch's second field seeds the superseded set and the
manifest's lines survive unless their key and suite both match. `awk` leaves this
member's spawn set and stays on the floor with its other tracked users. The
`fail_closed` calls that guard the `awk` and the hash (`:107`, `:119`) become the
crate's own error returns feeding the guards' exit 2 — the shell idiom retires, the
verdict does not.

**The batch file is a scratch artifact and stays one.** It is created under
`EVIDENCE_KIT_TMP_DIR` and removed after the fold; the compiled form claims it in
the same destructor delta (6) uses, so a run that dies between the last suite and
the fold leaves no orphan batch where the shell form's `rm` at `:122` would have
been skipped.

### (8) `sha256sum` leaves the spawn set, `ps` does not, and the cut claims no floor dividend

`sha256sum` is **off** `GATE_SDK_PROGRAM_FLOOR` (`gate-sdk/lib/gate.sh:182-185`) and
this file spawns it once, at `:106`, to hash each suite's log for the evidence line
{design-bearing}. The crate already answers it: `native/src/sha256.rs` exposes
`file_hex` (`:90`), live at four call sites in `native/src/install.rs` (`:257`,
`:485`, `:497`, `:533`). This port is its first evidence-side consumer, and the
hash's hex encoding is byte-compatible with `sha256sum`'s first field — which
delta (11)'s comparison verifies rather than assumes, because every manifest line
already written carries the shell form's digest and a divergence would silently
supersede real history with a differently-computed value.

**The dividend stops there, and the amendment says so rather than letting a later
reader infer a floor that did not move.** `ps` is also off the floor, reached
transitively through `ek_pid_alive`'s second leg, and it **stays** off the floor
after this port: the compiled twin already in tree spawns `ps -p` itself
(`native/src/evidence.rs:146`) after trying `kill -0`, and returns a distinguished
`PsAbsent` when `ps` is not on `$PATH`. §The producer-liveness lock rules that two-leg
predicate deliberate — `kill -0` conflates "no such process" with "not yours" and
must never produce a false *free* reading — so the second leg is the rule's content
rather than incidental spelling, and criterion 7's *the program is the rule* clause
holds it. The honest reading is that this member never cleared criterion 7 and does
not clear it now; what changes is that one off-floor program leaves and the other
becomes visible in a declared prose set instead of sitting invisible in an
unregistered `bin/` script.

### (9) The six `# spec:` directives re-home onto the Rust module

They do not vanish with the file: `check-comment-tier`'s corpus reaches `*.rs` as
well as `*.sh`, so the directive budget **moves** rather than shrinking, and each
lands on the construct it directed {mechanical}.

The three that must survive as written are `:43` (the claim is built whole and
`ln`ed, so it is create-exclusive and never half-written), `:55` (a dead holder is
reclaimed exactly once; a second failed claim refuses rather than loops) and `:64`
(the run accumulates in the batch and touches the tracked manifest only after the
last suite) — each states a property the compiled form could silently lose, and each
is the invariant of a delta above. `:29` lands on the destructor, `:113` on the fold,
and `:2` on the module header.

### (10) The behavioral coverage moves rather than shrinking, and it is this member's whole parity surface

This member ships no `good/`+`bad/` fixture pair and owes none — it is not a gate
{design-bearing}. What stands in its place is two live suites, both of which drive
the tool **by literal path** and both of which are re-pointed at the front-end
spelling in the deleting commit:

- **`evidence-kit/gate-tests/producer-lock.test.sh`** — `RV="$DIR/bin/run-validate.sh"`
  at `:17`, invoked at `:37` inside a per-scenario helper, driving claim, refusal
  against a live PID this test owns, stale-PID reclaim and second-producer takeover.
  This is the writer's entire behavioral coverage and no fixture pair can hold it.
- **`evidence-kit/smoke/install.sh`** — `bash "$SMOKE_KIT_ROOT/bin/run-validate.sh"`
  at `:44`, inside the three-suite scratch tree it builds at `:26-59`, asserting the
  clean-line append, the per-suite parser override's reach, the single fold after
  the roster and the configured suite ordering. It carries its own
  `# spec: evidence-kit/SPEC.md §bin/run-validate.sh` at `:26`. It is reached by the
  `consumer_smoke` evidence suite (`scripts/evidence-config.sh:27`), so a re-point
  that broke it would red a validate suite rather than nothing.

`demo/run-demo.sh` does **not** reference this tool — probed, zero hits — and is
named because a reader would expect the adoption walkthrough to exercise the
validate spine and it does not.

### (11) Criterion 2's discharge is the both-substrates comparison, bought once, before the delete

The comparison this cut owes, stated as a procedure so build does not invent one. In
one session, with both implementations present and pointed at the same config, run
the full configured roster under each and compare {design-bearing}:

- every **evidence line** the run appends, byte for byte — the key, the suite, the
  `sha256=` digest, the three counts, the verdict and the date;
- the **manifest after the fold**, byte for byte, over a repeated run, which is what
  asserts the supersede-and-reappend ordering rather than merely the content;
- the **lock record** byte for byte, per delta (6), which no gate holds;
- the **exit status** of each of the three states — a clean roster (0), a roster with
  a baselined-pass scenario gone red (1), and each guard (2): no suites configured,
  no run key, absent manifest, a held live lock, an unparseable lock, a
  double-reclaim failure, a missing suite command, a failing pre-hook, and a parser
  that produces no result.

Two things are **not** compared for equality, and saying so is the honest half: the
log files themselves differ run to run wherever a suite prints a timing or a path,
and the `sha256=` digest differs with them. What is compared for those is the
**relation** — that both substrates hash the same log to the same digest, taken by
running each substrate's hash over one captured log rather than over two runs.

delegation-kit/SPEC.md §Testing already rules this the discharge shape for a
non-gate member: criterion 2's discharge is the both-substrates comparison bought
once at port time. A comparison a session runs before a delete is evidence; an arm
that can only skip after it is not (gate-sdk/SPEC.md §The non-gate arm).

### (12) Every path-bearing surface moves in the deleting commit, and the doc set is entirely silent

The roster is probed rather than assumed, and it is split by what a gate catches
{mechanical}.

**The gate-caught set is empty, and that is the finding rather than an aside.**
`check-docs-cmd`'s invoked-path assertion fires **only inside a triple-backtick
fence** (`native/src/gates/docs_cmd.rs:211-230`). Every one of this file's ~50 doc
mentions is outside one, verified by fence position and not by grep:
`evidence-kit/SPEC.md` and its docs mirror carry **no** fences at all;
`evidence-kit/README.md`'s fences are `:25-30` and `:73-75` while its mentions are
`:14`, `:52`, `:66`, `:68`; `installer/README.md`'s one fence is `:640-650` and its
mention `:1669`; `gate-sdk/SPEC.md`'s fences are `:900-903`, `:1656-1658`,
`:8020-8032` and `:12448-12451` while its mentions are `:3688` and `:9443`;
`README.md:98` is a table cell carrying the bare filename. So
gate-sdk/SPEC.md:1882's general claim that the gate "will correctly — not
vacuously — red on a doc still fencing a deleted `.sh` path after a port" is true in
general and **empty for this file**. `check-tracking-claim` is likewise silent: none
of the mentions carries its `is committed` / `is tracked` grammar.

**The silent set, fixed by hand because nothing reds:** `README.md:98`;
`evidence-kit/README.md:14,52,66,68`; `evidence-kit/SPEC.md`'s prose mentions;
`gate-sdk/SPEC.md:3688,9443`; `installer/README.md:1669`;
`.claude/commands/validate.md:10` (doubly silent — `.claude/commands/` is outside
`CANON_KIT_MANIFEST_FILES` by `scripts/canon-config.sh:86`);
`lifecycle-kit/templates/lead.md:83` and `templates/stages/validate.md:58`;
`native/src/gates/evidence_manifest.rs:178` (a help string no gate scans for stale
path text); and the `TASK-QUEUE.md` bodies, which the manifest set excludes. The
frozen `docs/posts/*` releases are **not** edited — a dated post records what the
tree was.

**The two surfaces that red are executions, not assertions**, and they are
delta (10)'s.

**`evidence-kit/SPEC.md §bin/run-validate.sh` itself** is rewritten as the arm's
section per deltas (1) through (8). The heading **stays**: it is cited by
`bin/diff-baseline.sh`'s own section and by `lib/evidence.sh`'s, and deleting the
heading would dangle those where deleting only the script dangles nothing.

### (13) The two settings grants are a pure removal, and no addition is owed

`.claude/settings.json:49-50` carry exactly two grants naming this path —
`Bash(bash evidence-kit/bin/run-validate.sh)` and its ` *` form — probed rather than
assumed, as `native-gate-port-remaining-corpus`'s ruling (2) requires {mechanical}.
Both are deleted **in the same commit as the file**, the window that ruling exists
to close, and both are inside its carve-out: a removal forced by a ruled cut, not
the 2026-08-22 operator-class bar.

**No addition is owed.** `.claude/settings.json:12` already grants
`Bash(bash gate-sdk/bin/run-gates.sh *)`, which reaches every bridged arm through
the front-end. Named because a reader would expect a replacement grant and there
is none.

### (14) The regeneration fan-out this cut stales

Deleting one owed `.sh` moves `scripts/measured-claims.sh`'s `tree-shell-owed` key
(`:42`), which reads the trailer of `--emit port-blockers --tree` {mechanical}. The
generated `scripts/git-hooks/pre-commit:369` bakes that key's resolved value inline
— `108`, `native`, `34` today — so the hook and `commit-msg` with it go stale and
regenerate in the landing commit; `docs/check-graph.html` regenerates with them
(docs/site-architecture.md §Generated projections and their freshness gates). The
SPEC and README edits stale their on-site mirrors and the crate change stales the
binary: `check-graph`, `check-docs-mirror-fresh` and `check-gate-binary-fresh` are
the reds, all discharged in the landing commit.

**Two keys that do not move, probed rather than assumed:** `ported-gate-members`
counts `gates.list` members resolving to a `.gate` declaration and this cut registers
no gate; `gate-substrates` stays `native` because an `Arm` adds no descriptor. No
tracked `.md` binds `tree-shell-owed` behind a `measured:` marker, so
`check-measured-claim` stays green from the marker side and the staleness is the
baked hook alone.

## Producers and consumers

**New interface: the `--run-validate` arm.**
*Producer* — `gate-sdk/bin/run-gates.sh`'s new `case` arm, which resolves the
declared roster through `gate_knob_env` and `exec`s the binary
(`run-gates.sh:114-128`); and a direct binary invocation in a tree that has already
exported the bridged environment. Its enabling config is not test-only: this repo's
`scripts/evidence-config.sh` configures the whole roster today, and every consumer
that runs the spine already sets the same knobs.
*Consumers* — the validate stage (`lifecycle-kit/templates/stages/validate.md:58`),
the lead template (`templates/lead.md:83`), the `.claude/commands/validate.md` shim,
and `evidence-kit/smoke/install.sh` through the `consumer_smoke` suite. Each reaches
it by running the front-end and reading its **exit status**, which is the mechanism:
0 clean, 1 new failures, 2 the run could not start.

**New interface: the five compiled twins in `native/src/evidence.rs`.**
*Producer* — the module itself, called in-process by the new arm; no separate
trigger and no configuration of their own beyond the arm's declared roster.
*Consumers* — this cut's arm for all five, and the sibling cut on
§bin/diff-baseline.sh for `parser_for`, `parse` and `diff`. The shell forms stay in
`lib/evidence.sh` and keep their own caller (`bin/diff-baseline.sh`) until that cut
lands, so no shell reader is orphaned by this one.

**New state: none.** The cut introduces no new file, no new record format and no new
field. The lock record, the batch file, the parsed files, the per-suite logs and the
evidence line all keep their existing shapes and their existing readers, which is
what makes delta (11)'s byte comparison the right discharge.

**Existing readers of the two artifacts this member writes, and each one's RED
condition rather than its subject** — binding because delta (12) *narrows* a corpus
(one `.sh` file and two settings lines leave the tree):

- `check-producer-liveness` — reds when a lock or `<key>.run` record names a **live**
  PID at a stage entry. Its verdict is monotone in the record set: removing the
  writer removes records, never adds one. Safe to clear by inspection, and cleared.
- `check-evidence-manifest` — reds on a manifest line that does not parse, or whose
  key or suite is unknown. Monotone in the line set; the port writes the same lines.
- `check-evidence-baseline` — reds on a suite in `EVIDENCE_KIT_SUITES` carrying **no
  baseline row**, which is a **coverage floor** and therefore *not* monotone. It is
  cleared by argument rather than by inspection: this cut changes no member of
  `EVIDENCE_KIT_SUITES` and adds none, so the floor's input is untouched.
- `check-battery-roster` — `evidence-kit/gate-tests/check-battery-roster.test.sh:6,58`
  names this path; it reds on a roster disagreement. Because its red condition is a
  **set equality** rather than a violation count, it is not monotone under a
  narrowing and is re-run rather than reasoned about.
- `check-docs-cmd` — reds on a fenced invoked `.sh` path that does not resolve. Its
  red condition is a count of unresolvable fenced paths; the count is **zero for
  this file today** (delta 12), so the narrowing can only leave it zero.
- `check-comment-tier` — reds on a full-line comment that is neither directive nor
  exempt. Not monotone under a narrowing in principle, since its corpus reaches
  `*.rs`; delta (9) moves six directives into that corpus, so the gate is re-run and
  not inspected.
- `check-measured-claim` — reds only on a bound `measured:` marker whose oracle value
  disagrees with it; it has no arm that scans an unbound key. Delta (14) probes that no
  tracked `.md` binds `tree-shell-owed` behind such a marker, so this gate has no claim
  to check here and stays green — the staleness delta (14) regenerates is the baked
  hook, caught by `check-graph`, not this gate.

## Existing sections updated

- **`evidence-kit/SPEC.md §bin/run-validate.sh`** — rewritten as the arm's section:
  the spelling, the family choice, the three-state exit, the declared roster, the
  seams, the twins and the retained spawn set (deltas 1, 2, 3, 4, 5, 8). The heading
  stays.
- **`evidence-kit/SPEC.md §The producer-liveness lock`** — the writer half restated
  in substrate-neutral terms, the trap named as a destructor in the compiled form,
  and the `SIGKILL` residual re-stated as unchanged (delta 6).
- **`evidence-kit/SPEC.md §Evidence manifest`** — the single fold's ownership moves
  to the arm; the batch file's scratch claim joins the destructor (delta 7).
- **`evidence-kit/SPEC.md §lib/evidence.sh`** — the adapter roster gains a sentence
  naming which five now have compiled twins and which shell forms survive on which
  caller; the shell forms of `ek_lock_read` and `ek_pid_alive`, already compiled
  before this iteration, are removed as their last production caller leaves
  (delta 4).
- **`evidence-kit/gate-tests/evidence-lib-parity.test.sh`** — retired in the deleting
  commit: its own header states its existence rests on `bin/run-validate.sh` keeping
  the shell/compiled duplication of `ek_lock_read`/`ek_pid_alive` alive, and this cut
  is what ends that (delta 4).
- **`evidence-kit/SPEC.md §Layout and configuration`** — the two prefix families are
  named as declared knobs, and the parser seam's survival is stated where the knob
  is defined (deltas 3, 5).
- **`evidence-kit/README.md`** — the four mentions re-spelled to the front-end form
  (delta 12).
- **`gate-sdk/SPEC.md §The non-gate arm`** — the class roster gains `--run-validate`;
  it is recorded as the first member whose spawned-program set includes the
  **subject under test**, and as the first to declare two prefix families at once
  (deltas 2, 3, 5).
- **`gate-sdk/SPEC.md §The port-candidate criteria`** — criterion 7 gains this
  member as the instance where one off-floor program leaves and a second stays
  because the section that owns it rules the second leg to be the rule (delta 8).
- **`gate-sdk/SPEC.md §The first cohort, and the rule that selects the next`** — the
  composition axis gains its second attested instance beside guard-kit's ranker: a
  124-line column whose real cut is that file plus five twins (delta 4).
- **`gate-sdk/bin/run-gates.sh`** — the new `case` arm and its usage line (delta 2).
- **`.claude/settings.json`** — lines 49-50 deleted in the deleting commit
  (delta 13).
- **`lifecycle-kit/templates/stages/validate.md` and `templates/lead.md`** — the two
  invocation mentions re-spelled (delta 12).
- **`evidence-kit/gate-tests/producer-lock.test.sh` and `evidence-kit/smoke/install.sh`**
  — re-pointed at the front-end spelling (delta 10).
- **The generated projections** — `scripts/git-hooks/pre-commit`, `commit-msg`,
  `docs/check-graph.html`, the `docs/` SPEC and README mirrors, and the gate binary
  (delta 14).
<!-- update-target-exempt: the frozen release posts under docs/posts/ record what the tree was on a dated release and are deliberately not edited by any delta -->
- **`docs/posts/2026-07-17-checkwright-v0-2-0.md` and `docs/posts/2026-08-06-checkwright-v0-22-0.md`**
  — named so a later reader does not read their stale mentions as an omission.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named reader at
      a named transition. (No new field: the cut adds none.)
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section, not appended; the merged spec reads as one coherent
      document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge. The none-remain half is
      discharged at the iteration, not at this commit: a sibling amendment is in
      flight for this component (§bin/diff-baseline.sh's cut), so only the batch
      merging the last of the two can satisfy it.
- [ ] **Both-substrates comparison bought before the delete** — delta (11)'s
      procedure run in the deleting session, with both implementations present.
- [ ] **Removals propagated** — every surface in delta (12) edited, the two settings
      grants deleted in the same commit as the file, `evidence-lib-parity.test.sh` and
      the now-uncalled shell forms of `ek_lock_read`/`ek_pid_alive` retired per
      delta (4), and every spec grepped for names this change retired.
- [ ] **Gaps filed** — the unreproducible help-arm census derivation (delta 2, shared
      with the other three cuts of this iteration) filed to the gap inbox; a
      build-time causal gap is resolved that session, not deferred.
