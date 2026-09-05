# SPEC amendment: scratch-run-cut

The port disposition of **`guard-kit/bin/scratch-run.sh` (50 lines), the one owed file declaring
guard-kit/SPEC.md §scratch-run**: it ports to the `--scratch-run` bridged arm, and the settings
change it forces is a grant **removal**, not the addition this section has priced it as. A
stated-contract cut under the port-only run (TRAJECTORY.md §PRIORITY DIRECTIVE), hosted on
`scratch-run-port-blocker-unrecorded` — the entry whose whole subject is this file's unrecorded
blocker — and packaged by the lead as the third of this iteration's three.

**`scratch-run-port-blocker-unrecorded` rides inside this cut and is promoted with it.** It is not
a sibling unit and gets no amendment of its own: the entry exists because this file's
operator-class blocker had no queue entry, and deltas 5 and 6 are its whole deliverable.

**The composer's precondition was run at this stage rather than inherited.**
`bash gate-sdk/bin/run-gates.sh --emit port-blockers --group` trails *108 member(s) scanned, 0
group(s) formed, … 0 still owed, 0 takeable at this cut* — no takeable group, the budget arm's
stated precondition (gate-sdk/SPEC.md §The first cohort). The selection ground is the **owed
column** of `--emit port-blockers --tree` — *92 file(s) scanned, 66 declared no-port, 0 temporarily
held, 26 owed* — where this file reads `owed lines=50`.

## The blocker this cut was priced against does not exist, and the correction is the cut's centre

§scratch-run has ruled since it was written that *re-implementing the runner on another substrate
is a permission ADDITION rather than a migration, and that is what prices the work*
(`:1316-1324`). `scratch-run-port-blocker-unrecorded` restates it — *this grant names its own fixed
path by design and cannot ride the front-end grant*. Both were written against a **relocated-path**
assumption, and the forced-family rule does not produce one. Probed at this stage:

- gate-sdk/SPEC.md §The non-gate arm's forced-family test rules that *the family choice is forced
  for any tool that needs configuration at all*, since a hardcoded top-level flag *resolves
  platform defaults and silently ignores every consumer override*. This runner reads
  `GATE_SDK_TMP_DIR`, defaulted at `gate-sdk/lib/gate.sh:48`. The bridged family is therefore
  **forced**, not chosen.
- A bridged arm is reached as `bash gate-sdk/bin/run-gates.sh --scratch-run <script>`, and
  `.claude/settings.json:12` already grants `Bash(bash gate-sdk/bin/run-gates.sh *)` — the
  end-wildcard covering every bridged arm.

So the port owes **no addition** and forces a **removal**. **Ruled 2026-09-05 (operator,
in-session prompt, lead-relayed)** on this stage's escalation, which put four options and their
costs; the operator selected the bridged, removal-only shape by name, with the retirement below
named in the question. Two things the ruling does not license, stated because this option is the
one most likely to be over-read: it is **not** a general licence over `.claude/settings.json` —
the 2026-08-22 bar stands for every edit outside a ruled cut's forced removal — and it is **not**
a finding that consumer opt-outs are cheap. It is one retirement on its own grounds, and a later
cut proposing another owes its own escalation.

**What the ruling supersedes, recorded rather than left dangling.** The 2026-09-05 consult
pre-authorization — *build lands the ported arm with the old grant in place and records the
one-line grant diff; the operator applies it out of band* — is **re-scoped by the operator's own
later ruling the same day**: there is no diff to record and nothing to apply. That is a re-scoping
by the same authority, not a reversal by this session, and delta 6 lands it on the entry and in
TRAJECTORY.md's ruling record.

## What changes

### (1) The cut is the one owed file declaring §scratch-run, and it does not discharge guard-kit

`bin/scratch-run.sh` reads `owed lines=50` and is the **only** owed file whose `# spec:` pointers
bind `## scratch-run` {mechanical}. Its reach and its section bound coincide, and no second section
is rewritten by construction — with the one stated exception every budget batch moves,
gate-sdk/SPEC.md §The first cohort's *a budget batch records only findings* rule (its own worked
instance is `§run-gates`' stub cut, the precedent this cut's finding below follows).

**It does not discharge guard-kit**, and the amendment says so because a 50-line member reads like
a whole-kit finish. The kit's owed column keeps `bin/compare-settings-allow.sh` (115) and
`bin/run-guard-tests.sh` (101) on their own separate grounds, and `lib/guard.sh` (1244) leaves it
by `# no-port:` declaration rather than by port. This cut settles one section.

### (2) The arm is bridged and `Arm::Run`, and both halves of that are forced rather than chosen

{design-bearing} The **family** is forced by the test above: the runner resolves
`GATE_SDK_TMP_DIR`, so a top-level flag would resolve `.tmp` and ignore every consumer override —
which §The non-gate arm calls *not a calibration between two workable shapes but the difference
between working and appearing to*. It joins `BRIDGED_ARMS` as `--scratch-run`.

**The variant is forced too, on two independent grounds**, and `native/src/emit/mod.rs` rules the
variant a return shape. `Arm::Emit` collapses every outcome to `{0, 2}` at
`native/src/main.rs:485-495`; this runner's whole contract is that it **passes the child's exit
code through verbatim**, and a child exits any code it likes. And `Arm::Emit` returns a `String`
the dispatcher prints at the end, where the runner must let the child's stdout and stderr reach the
terminal **as they are produced** — the echo is a transcript artifact, so buffering it behind the
child's completion would defeat what the echo is for. Either ground alone decides it.

**The spelling is `--scratch-run`, not `--emit-scratch-run`.** The `--emit-` prefix is
load-bearing rather than house style (§The non-gate arm), keying the front end's
`--emit <name>` operand composition; this member is a runner in the shape `--wait-probe`,
`--upgrade-smoke` and `--run-validate` already take, and those are spelled bare.

**Its declared roster is `GATE_SDK_TMP_DIR` and nothing else.** In particular it does **not**
acquire `GUARD_KIT_SCRIPT_INTERPRETERS`: guard-kit/SPEC.md:1928 states the runner *deliberately
does not read it*, on §scratch-run's own ground — the guard has a command string and needs a
roster, the runner has the **file** and reads its own shebang, *two mechanisms, each exact for the
input it actually has*. A port that declared the knob because it was in the kit's namespace would
retire that reasoning by accident.

### (3) `proc::run_to` is the exec half exactly, and the shebang classifier and the containment predicate are written new

{design-bearing} The reuse and the residue are both stated, because a reader sizing a 50-line
member should know which half is free.

**Free.** `proc::run_to(program, args, &Sink::Inherit)` (`native/src/proc.rs:544`) is the
passthrough: under `Sink::Inherit` the child's stdout and stderr are not redirected and the return
is the child's own exit code (`:576`), with `exit_code` (`:620`) mapping a signal death to
`128 + n`. `Err` is a spawn failure only. No new spawn primitive is minted, and
`native/src/proc.rs` stays the crate's one spawn site.

**Written new, because the crate has neither.** A **shebang classifier**: the only `#!` handling in
the crate is `walk.rs:58` and `gates/comment_tier.rs:232`, and both merely *skip* line one. The
port reads line one, and where it begins `#!` resolves the interpreter word — resolving the
`/usr/bin/env <interp>` spelling to the same answer — refusing at exit 2 with the bash-only message
**before the echo**, so a refusal still prints no body and stays distinguishable from a child that
exited 2. A target with **no** shebang is unaffected. And a **containment predicate**: there is no
shared `is_inside(root, path)` in the crate; every site spells its own prefix comparison.

**The containment test keeps symlink resolution, and that is a cost taken rather than a reuse.**
§scratch-run rules that the test *reads the resolved path, never the spelling*, so the primitive is
`walk::canonicalize` (`walk.rs:339`) — the crate's **only** `std::fs::canonicalize`, whose own
`# spec:` at `:336-338` names gate-sdk/SPEC.md §The crate's crosser and *what the two callers may
assume*. This cut makes a third, and it inherits that section's dialect obligation: the helper
hands its answer back unconverted, so the comparison runs on two answers from the same producer,
never on one canonicalized path against one composed string.

**The lexical alternative is refused on security, not on cost.** `abs_against` + `normalize_abs` +
a prefix compare is the crate's ordinary shape and it handles a `..` traversal — but it does not
follow a **symlink** out of the scratch dir, so a symlink planted inside `.tmp` would pass a test
the shell form fails. That is a narrowing of a fail-closed control, which the section's own
sentence forbids.

**The hardcoded `bash` interpreter survives the port unchanged and unwidened.** §scratch-run
refuses a second interpreter *outright rather than costed*, because it would convert a grant for
"run bash on a reviewed body" into one for "run anything on a reviewed body" with no settings edit.
That argument binds the crate arm identically and is stated in the module rather than inferred.

### (4) A latent config divergence closes, and the test harness's driving mechanism survives it

{design-bearing} `scratch-run.sh` **sources nothing** — the whole 50-line file reads
`${GATE_SDK_TMP_DIR:-.tmp}` straight from the process environment — so a consumer who sets that
knob in their gates-dir config today gets a runner that ignores it. Through the bridge the knob is
resolved properly, which makes the port a **fix** rather than a translation, and this delta is
where that is stated rather than discovered.

**The fix does not break the way the knob is driven under test.** `gate.sh:48` is
`[[ -v GATE_SDK_TMP_DIR ]] || GATE_SDK_TMP_DIR=".tmp"`, so an environment-set value still wins, and
the bridge resolves a member's knobs by sourcing the kit library in a subshell that **inherits the
caller's environment** (§The non-gate arm's `LIFECYCLE_KIT_CONFIG_FILE` near miss states exactly
this). So `guard-kit/gate-tests/scratch-run.test.sh`'s per-invocation `GATE_SDK_TMP_DIR="$scratch"`
keeps working through the arm. Named because the opposite conclusion — that a bridged knob
overrides its caller — is the natural one and is wrong.

### (5) The permission-ADDITION paragraph is narrowed, not deleted, and the consumer opt-out is retired knowingly

{design-bearing} Two sentences of §scratch-run are the ones this cut's finding reaches, and they
take **different** dispositions, which is the point of separating them.

**`:1316-1324`, the ADDITION paragraph — narrowed.** Its **structural** claim survives untouched: a
compiled or relocated arm is a different command string, and a permission set is matched rather
than versioned, so the old entry cannot be edited into the new one without a window where neither
runs. What does not survive is its **pricing conclusion** — *that is what prices the work*. Under
the forced family the arm's new command string is the **battery front end's**, so any addition a
consumer owes is the front-end grant, which buys every bridged arm at once and is not a decision
about scratch execution at all. Probed rather than assumed: `.claude/settings.json:96,114,134,138`
and `:147` are harness hook and statusline `command` entries, which take no permission decision, so
what demands the wildcard form is every **agent-side** arm — `--only`, `--enter-stage`, each
`--emit`, i.e. every lifecycle stage template. A consumer running stages already holds it; one
running only the generated pre-commit hook may not, and owes the front-end grant rather than this
tool's. The paragraph keeps its mechanism and loses the conclusion the port falsified.

**`:1296-1297`, the opt-out — retired, and the sentence is rewritten rather than deleted.** Today
the section tells a consumer unwilling to move review downstream to *simply not add it: the tool
still runs without it, and its runs still take that decision.* Post-port that is false: the runner
is reachable wherever the battery front end is, and a consumer cannot decline it without declining
the battery. The replacement states the narrowing in the same voice — what the consumer can no
longer separate, and that the **compensating control is unchanged**, since the echo lives in the
code and never lived in the grant. **This is a taken cost, not a discovery**: the operator ruled
with it named in the question (2026-09-05, in-session prompt, lead-relayed), and a later reader
meeting the retired sentence needs to find the ruling and not an accident.

**The refused reading, recorded because it is one clause away.** This does *not* establish that a
port may retire a consumer guarantee whenever a family rule forces its hand. The retirement was
escalated, not decided in the cut, and the ruling is one instance.

### (6) The settings grant drops in the deleting commit, and the two superseded pre-authorizations are disposed of

{mechanical} `.claude/settings.json:32`, `"Bash(bash guard-kit/bin/scratch-run.sh *)"`, names a path
this cut deletes and drops **in the same commit as the delete**. The licence is
`native-gate-port-remaining-corpus` ruling (2)'s **2026-08-29 base** — *removing a grant whose
target a RULED PORT CUT DELETES is OUTSIDE the 2026-08-22 bar — a pure narrowing — and build drops
the dead lines IN THE SAME COMMIT AS THE DELETE* — whose last clause is itself the in-cut edit, so
the 2026-09-05 "decommissioning bash gates" widening is **not construed and not relied on** (ruled
(i) by the lead on its own authority, 2026-09-05). guard-kit/SPEC.md:1323-1324 states the same rule
from the kit side. **No addition is owed.**

**Two pre-authorizations are superseded and each is recorded where it stands**, so a later reader
meets *superseded, nothing owed* rather than an approval with no matching step: the 2026-09-05
consult pre-authorization on `scratch-run-port-blocker-unrecorded`, and the out-of-band settings
approval relayed the same day. Both are re-scoped by the operator's own later ruling, and are
recorded as **the operator's re-scoping** rather than as this session's judgment.

**"Recorded where it stands" means replaced, not annotated, on `TRAJECTORY.md`'s own side of this
— its rule at `:56-59` forbids leaving a superseded sentence beside its corrector.**
`TASK-QUEUE.md`'s matching entry already carries an append-then-annotate pair (a PRE-AUTHORIZED
bullet and a SUPERSEDED bullet beside it) — correct for that file, under queue-kit's own
convention, and already landed in the current tree — but that is not a shape to copy onto
`TRAJECTORY.md`, whose entire 2026-09-05 scratch-runner paragraph a build session rewrites in
place. See the "Existing sections updated" entry below for the rule citation and the full scope of
what changes.

### (7) The tests split on the kfric precedent — unit cases in-crate, seam cases through the bridge

{mechanical} `guard-kit/gate-tests/scratch-run.test.sh` is this member's oracle, run by gate-sdk's
runner (guard-kit/SPEC.md §Layout and configuration). It splits the way the 2026-09-03 kfric port
split its own, which `drift-kit/smoke/install.sh:590` states in as many words — *the grammar cases
are pinned in the ported module's own `#[cfg(test)]` tests, where `check-crate-arms` runs them*.

- **In-crate** `#[cfg(test)]`: shebang classification across its cases (`bash`, `sh`, no shebang,
  `/usr/bin/env <interp>`, a non-bash interpreter) and the containment predicate including the
  traversal spelling. These are pure functions over inputs and want no process.
- **Staying in `scratch-run.test.sh`, re-pointed at the arm**: echo-precedes-exec **ordering**,
  args and exit-code passthrough, the out-of-scratch and traversal refusals, the absent-target and
  missing-argument exits, and the refusal-prints-no-body discriminator. Every one of these is a
  property of the **seam** — the front end resolving the arm, the bridge supplying the knob, and a
  real child process — which a crate unit test cannot see. `RUN` (`:8`) moves from
  `guard-kit/bin/scratch-run.sh` to the front-end form; the per-invocation
  `GATE_SDK_TMP_DIR="$scratch"` is unchanged by delta 4.

**No assertion is dropped in the split**, and a build session re-greps the file before editing
rather than working from this list, because the suite decides.

### (8) Rule 23's steer prints the arm's command string

{mechanical} A delta rather than only an update target, because it is an edit to a `no-port`
library's message text and a build batch needs it owned. `guard-kit/lib/guard.sh:1168` composes the
runner path and `:1170` and `:1172` embed it in two block messages as
`'bash $runner <script> [args…]'`. Both compose the front-end form instead, deriving the front
end's location from the root the library already computes rather than hardcoding it. The messages'
substance is unchanged — the compensating control, the inline-body carve-out, and the
`!<command>` escape hatch all stay — and `guard-kit/templates/close-triage.md:49-50` re-points to
match.

## Producers and consumers

This cut introduces **no new knob, no new log and no new file on any governed surface**, and adds
no field to anything. It moves one producer into the crate, changes the command string one steer
prints, and removes one settings line. The survey below is over the **whole component set**: every
tracked file was grepped for `scratch-run`, with stderr left open.

**The interface whose producer moves: the runner's invocation.**

- **Producer, before:** `guard-kit/bin/scratch-run.sh`, reading `GATE_SDK_TMP_DIR` from the raw
  process environment and sourcing no config.
- **Producer, after:** the `--scratch-run` bridged arm, reached through
  `gate-sdk/bin/run-gates.sh`, with `GATE_SDK_TMP_DIR` resolved by `gate_knob_env` from
  `gate-sdk/lib/gate.sh` and the consumer's gates-dir config (delta 4). Its enabling config is not
  new and is not test-only: the knob ships with a working default and every consumer vendoring
  gate-sdk resolves it.
- **Consumers, named:** a **session**, reached through the front end — which §The non-gate arm
  admits by name as a caller (*a session reaching a mode through the front-end counts exactly as a
  stage step does*), and which is this member's only caller class, since nothing in tree invokes
  the runner but its own test.

**The named callers, surveyed, with what this cut owes each.**

| caller | site | what this cut owes it |
| --- | --- | --- |
| rule 23's two block messages | `guard-kit/lib/guard.sh:1168,1170,1172` | the steer's printed command; delta 8's first target |
| the close-triage template | `guard-kit/templates/close-triage.md:49-50` | it names the shell path as *the sanctioned form already*; re-pointed |
| the member's own oracle | `guard-kit/gate-tests/scratch-run.test.sh:8` | delta 7 |
| the settings grant | `.claude/settings.json:32` | delta 6, removal |
| the front-end grant | `.claude/settings.json:12` | **nothing** — it already covers the arm, which is the finding |

**The one caller whose change is a cross-kit coupling, named because it is new.**
`guard-kit/lib/guard.sh:1168` derives the runner's path from the **guard library's own location**
(`${runner%/lib/guard.sh}/bin/scratch-run.sh`), so today a guard-kit library names only a guard-kit
sibling. After this cut it must name **gate-sdk's front end**, which is a path coupling guard-kit
does not have. It is admissible on a ground already in the tree rather than a new one: guard-kit
already depends on gate-sdk for `GATE_SDK_TMP_DIR` — the scratch dir the whole rule is written
about — and `lib/guard.sh` is `# no-port:` permanently, so the coupling has one holder and no
parity obligation. The derivation is kept rather than replaced by a literal: the front end's
location is derived from the same root the library already computes, so a relocated vendor tree
still prints a path that resolves.

**No field, message or state is added**, so the field-reader obligation is vacuous and is recorded
as such rather than omitted. The one *observable* this cut changes is the string rule 23 prints,
whose reader is the session that just tripped the rule, at the transition where the guard blocks.

## Existing sections updated

- `guard-kit/SPEC.md` §scratch-run, the **opt-out sentence** (`:1296-1297`) — retired and rewritten
  to state the narrowing honestly (delta 5). The compensating control's own paragraph above it is
  unchanged and the rewrite says so: the echo lives in the code, never in the grant.
- `guard-kit/SPEC.md` §scratch-run, the **permission-ADDITION paragraph** (`:1316-1324`) —
  narrowed, keeping its structural claim and losing its pricing conclusion (delta 5).
- `guard-kit/SPEC.md` §scratch-run, the **fail-closed-on-reach paragraph** (`:1349-1360`) — its
  *resolved path, never the spelling* rule is what forces `walk::canonicalize`, and it gains that
  mechanism plus the refused lexical alternative (delta 3). Its closing sentence — *the scratch dir
  comes from gate-sdk's existing `GATE_SDK_TMP_DIR` and no kit knob is added* — stays true and
  gains delta 4's correction, that the knob is now actually resolved.
- `guard-kit/SPEC.md` §scratch-run, the **bash-only paragraphs** (`:1299-1314`, `:1332-1343`) — the
  hardcoded interpreter and the shebang refusal port unchanged; the *why widening is refused
  outright* argument is re-attributed to the arm (delta 3).
- `guard-kit/SPEC.md` §scratch-run — gains a **port-owed residue paragraph**, the shape
  drift-kit/SPEC.md §The knowledge-friction loop established at `:467-477` (all deltas): this
  section's owed set is empty and no later cut is sequenced against it.
- `guard-kit/SPEC.md` §Layout and configuration, the `bin/scratch-run.sh` and
  `gate-tests/scratch-run.test.sh` rows (`:1833`, `:1838`) — the file leaves the layout and the test
  splits (deltas 1, 7).
- `guard-kit/SPEC.md` §Layout and configuration, the `GUARD_KIT_SCRIPT_INTERPRETERS` bullet
  (`:1918-1929`) — its *`bin/scratch-run.sh` deliberately does not read it* clause keeps its
  reasoning and names the arm instead of the script (delta 2).
- `guard-kit/SPEC.md` §The generic ruleset, rule 23 — the steer's printed command and its derived
  path (delta 8's target below; the rule's own predicate is untouched).
- `gate-sdk/SPEC.md` §The non-gate arm — the roster gains `--scratch-run` with its dated
  attribution, in the `Arm::Run` group beside `--wait-probe` and `--upgrade-smoke`, and the class
  gains its first member whose port **removes** a grant naming its own path rather than relocating
  one (deltas 2, 6).
- `gate-sdk/SPEC.md` §The crate's crosser — its two-caller statement about `walk::canonicalize`
  becomes three, and the third's ground is recorded with it (delta 3).
- `gate-sdk/SPEC.md` §The first cohort — under its own rule, *a budget batch adds a section only
  where it has a finding to record*, this cut adds one paragraph beside the sibling pair's, in the
  shape its worked instance (`§run-gates`' stub cut) already took: a member priced as blocked by a
  *permission* cost should have that cost re-derived against the forced family before it is
  composed, because the family decides the command string and the command string decides whether
  any grant is owed (all deltas motivate it, delta 5 is its subject). This corrects the amendment's
  own earlier citation of "§The port disposition" for this finding: that heading
  (`gate-sdk/SPEC.md:7979`, nested under §Consumer smoke) is a differently-scoped, already-occupied
  section and is not this cut's to edit.
- `TRAJECTORY.md` §The closed rulings — its 2026-09-05 paragraph *the scratch runner's port is
  pre-authorized: build prepares the grant addition, the operator applies it* is **replaced in
  place**, never appended to: TRAJECTORY.md's own completion-time rule (`:56-59`) is explicit that
  *a fact that has aged is corrected where it stands... a superseded sentence is never left standing
  beside the sentence that corrects it*, so a build session must rewrite the whole paragraph (its
  substance is entirely superseded, not only its lead sentence — the grant diff, the operator
  applying it out of band, and the shell path staying live in the meantime are all moot under the
  removal-only disposition) to state the current fact and delete the rest, rather than copying the
  append-then-annotate idiom `TASK-QUEUE.md`'s own `scratch-run-port-blocker-unrecorded` entry uses
  for its own PRE-AUTHORIZED/SUPERSEDED bullet pair — that idiom is queue-kit's own convention for a
  ruled entry and is not TRAJECTORY.md's, whose rule this delta names precisely so a build session
  does not import the wrong one. The `TASK-QUEUE.md` side of this correction is **already done** in
  the current tree — the entry already carries both the original bullet and its superseding one —
  so only the `TRAJECTORY.md` paragraph remains outstanding, and delta 6's own DoD line is scoped
  to that one target, named as a target rather than edited at authoring time, because the ruling record is not this
  stage's to rewrite unaudited.

## Definition of Done

- [ ] **Causal completeness** — the runner's invocation has one named producer (the bridged arm)
      and one named consumer class (a session through the front end); no field, message or state is
      added, so the field-reader obligation is vacuous and recorded as such; `GATE_SDK_TMP_DIR` is
      the sole declared knob and is actually resolved.
- [ ] **The oracle decides, not the roster** — `guard-kit/gate-tests/scratch-run.test.sh` green
      through the arm, the crate's lint and test arms green through `check-crate-arms`,
      `bash gate-sdk/bin/build-native.sh` run, the guard-kit fixture suite green, and the tree
      re-grepped for `scratch-run.sh` before the cut is called done.
- [ ] **No assertion was dropped in the test split** — every case in the shell file before the cut
      is either still there or has a named in-crate counterpart, checked case by case.
- [ ] **The containment test still refuses a symlink out of the scratch dir** — asserted, not
      assumed, since that is the property the lexical alternative would have lost.
- [ ] **The grant removal lands in the deleting commit** — `.claude/settings.json:32`, under ruling
      (2)'s 2026-08-29 base and guard-kit/SPEC.md:1323-1324, with no addition owed and the
      2026-09-05 widening not construed.
- [ ] **Both superseded pre-authorizations are disposed of** — the `TASK-QUEUE.md` entry side is
      **already done** in the current tree (checked, not assumed); only `TRAJECTORY.md`'s paragraph
      remains, replaced in place per its own completion-time rule (`:56-59`), never appended to, as
      the operator's own re-scoping rather than as a session's judgment.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); §scratch-run reads as one document to a reader who never saw this
      amendment, with the retired opt-out preserved as a stated, ruled cost.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls guard-kit/SPEC-*.md`).
- [ ] **The terminal move is `## Done`** — `scratch-run-port-blocker-unrecorded`'s deliverable is
      this one recording and this one port, not a corpus, so it does not demote.
- [ ] **Removals propagated** — every spec grepped for the names this change retires (the shell
      runner's path, the permission-addition pricing, the consumer opt-out); nothing dangles.
- [ ] **The oracle re-read, not the arithmetic trusted** —
      `bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree` re-run at the landing commit and
      its owed count recorded.
- [ ] **Gaps filed** — any cross-component gap found during the work filed to the committed gap
      inbox, and any fact re-derived off a non-owning surface stamped with `--emit kfric`.
