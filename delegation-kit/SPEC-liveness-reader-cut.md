# SPEC amendment: liveness-reader-cut

The port disposition of **`scripts/producer-liveness-reader.sh` (26 lines), the one
owed file this repo holds behind delegation-kit/SPEC.md §The turn-end liveness
hook**: it leaves the tree, and the hook's default reader becomes the compiled
`check-producer-liveness` reached through the binary's own executable. A
stated-contract cut under the port-only run (TRAJECTORY.md §PRIORITY DIRECTIVE),
selected by `native-gate-port-remaining-corpus`'s composer and packaged by the lead.

**Measured at this HEAD rather than carried from scope's survey.** The port oracle's
`--tree` arm reads *94 file(s) scanned, 64 declared no-port, 0 temporarily held, **30
owed*** — 3956 lines. The selection ground is that owed column and never the registry
`--group` read, which this stage ran and which trails *108 member(s) scanned, 0
group(s) formed, 0 undecidable, 108 already ported and excluded, 0 permanently shell
and excluded, 0 temporarily held and excluded; 0 still owed, 0 takeable at this cut*
(gate-sdk/SPEC.md §port-blockers, §The first cohort). The ported file is not a gate:
it is in no `gates.list`, carries no `.gate` descriptor, and owes no fixture pair.

**The ruling this cut executes is landed, and the amendment says which half is
already written.** `kit-knob-consumer-adapter-convention`'s 2026-09-05 operator ruling
in consult — *"the liveness hook reaches its gate through its own executable, so the
knob is an override over a working default"* — is already integrated at
delegation-kit/SPEC.md §The turn-end liveness hook, in the paragraph beginning *"That
ruling's premise has lapsed, and the port disposition follows from the lapse."* So the
**contract** is settled prose and this amendment does not re-rule it. What is not
written anywhere is the cut: which code path produces the default, what happens to
the two surfaces whose scenarios the default invalidates, and what becomes of the
main-checkout resolution the deleted script performed. Those are deltas (2) through
(6), and (4) is a **measured break** rather than a tidy-up.

**Its dependency on the sibling front-end cut is one-directional and stated.** The
hook is registered in this repo as `bash gate-sdk/bin/run-gates.sh --hook
subagent-stop-liveness`, so this cut's behavior is reached through the front end the
`run-gates-stub-cut` amendment reduces to a stub. Nothing here changes that argv, and
nothing here depends on the stub landing first: `--hook` is a bridged arm today and a
bridged arm after, and the two cuts touch disjoint code. The dependency is recorded so
a build session batching both does not go looking for an ordering constraint that is
not there.

## What changes

### (1) The cut is one owed file, and its owning section is delegation-kit's rather than this repo's

`scripts/producer-liveness-reader.sh` reads `owed lines=26` and is the **only** owed
file whose subject is the turn-end liveness hook {design-bearing}. It is a
`scripts/`-resident consumer adapter, so the naive reading is that CLAUDE.md
§Housekeeping or `scripts/delegation-config.sh`'s own class ruling governs it. Both
are wrong and the correction is worth one delta, because a later selector reaching
this file again would make the same wrong turn.

**It is not config-and-vocabulary.** The 2026-08-24 operator ruling that declares this
repo's `scripts/` config class covers files whose content is a knob roster or a
vocabulary — `scripts/delegation-config.sh:3` declares on exactly that ground. This
file holds no knob and no vocabulary: it is an *executable* whose whole body is a
resolution and an exec, and it is the **value** of a knob rather than a place one is
set. So the class ruling does not reach it, which is why it reads `owed` today while
its sibling `scripts/delegation-config.sh` reads `no-port`.

**And it is not §Housekeeping's.** `cut-boundary-section-legality-unruled`'s ruling (2)
of 2026-09-05 — *"An always-loaded manifest is never a cut boundary: a `# spec:`
pointing at CLAUDE.md is mis-homed"* — forecloses that reading directly. The file's
subject is one kit's hook contract, so its owning section is
delegation-kit/SPEC.md §The turn-end liveness hook, which already names it twice.

### (2) The default reader is the running executable, spawned; an in-process call is refused

`native/src/hook/stop_liveness.rs:45` resolves `DELEGATION_KIT_LIVENESS_CMD` through
`walk::knob_scalar` and hands the value to `read_liveness` (`:74`), which at `:177-180`
returns `None` — and therefore `verdict=unavailable` (`:84`) — whenever the value is
empty or names no file {design-bearing}. That empty-means-nothing branch is what this
delta replaces: an **empty value resolves to `std::env::current_exe()`**, spawned with
the gate name and the run dir as its argv, exactly as an override path is spawned with
the run dir as its only argument.

**The change is required rather than optional, and the module says so about itself.**
`read_liveness`'s own comment at `:182-183` reads *"the liveness reader stays external
and stays spawned: it is a member of the `scripts/` owed cohort, **not of this cut**"* —
written at a cut this one supersedes. So the compiled default does not exist today, and
deleting `scripts/producer-liveness-reader.sh` while blanking the knob and *relying on*
the kit default would reproduce the attested regression exactly: 77 `verdict=unavailable`
firings inside one 46-minute window, over a green battery, the last time a port deleted
this knob's target (`kit-knob-consumer-adapter-convention`, TASK-QUEUE.md:213-217).
Delta (5)'s deletion is therefore **not landable without this delta in the same commit**,
and that ordering is the one hard constraint this cut carries.

**The `bash` prefix drops with it, and that is compliance rather than a second change.**
The spawn is `proc::run_bounded("bash", &[cmd, run_dir], READER_BOUND_SECS)` (`:184`) —
an interpreter word. §The turn-end liveness hook already rules the override *"a path
executed directly with the scratch dir as its only argument, **no interpreter word**"*
and states what that relies on: *"a consumer override that is a shell script names its
interpreter in its own shebang, which is what dropping the `bash` prefix relies on."* The
code has not caught up with its own section. It drops here because the default **cannot**
tolerate it — `current_exe()` is an ELF binary and `bash <binary> <dir>` does not run it —
so the prefix is not merely non-compliant after this delta, it is broken.

**Spawning rather than calling in-process, and the ground is the seam rather than
caution.** `check-producer-liveness` is compiled into this binary and
`gates::lookup` would hand back the function, so an in-process call is available and
looks free. It is refused because the hook's whole predicate is *the reader's exit
class* (§The turn-end liveness hook's six-arm table), and that table is written over a
**child process's** status with `None` as its no-reading arm. An in-process call
produces an `i32` and can never produce `None`, so the default and the override would
travel two code paths with two arm sets, and the `unavailable` and `error` rows would
become unreachable for the default while staying reachable for an override. One path
with two values of one argv keeps every row of that table true of both, which is the
property the table's readers — the close-stage log reader and this hook's own refusal
text — depend on.

**It also keeps the extension point whole**, which `native-gate-port-remaining-corpus`
ruling (1) requires of every cut: the consumer's way to substitute its own reader is
the knob, and the knob's contract is *a path executed directly with the scratch dir as
its only argument, no interpreter word*. Nothing about that changes, and the default
is not a special case of it — it is the same spawn with a path the binary derives
instead of one a consumer names.

**`unavailable` is not retired, and the amendment says where it survives**, because
the obvious reading of a working default is that no firing can lack a reading again.
An **override** that names a path which does not exist, is not executable, or cannot
be spawned still yields `None`, so the row keeps its producer; what it loses is the
*unset knob* as a routine producer, which is precisely the fake-default degradation
§The turn-end liveness hook records as the reason the knob had no default at all.

### (3) The main-checkout resolution the deleted script performed is not re-implemented

`scripts/producer-liveness-reader.sh` satisfied §The turn-end liveness hook's
worktree-resolvability requirement by resolving the configured binary against the main
checkout when the configured path was absent and the cwd was a linked worktree,
deriving the main checkout from `git rev-parse --git-common-dir` {design-bearing}. The
crate acquires **none** of that.

**The requirement is discharged by construction rather than waived**, and the section
already says so: *"The worktree-resolution requirement below is discharged by
construction for the default: the executable that is running is, by definition,
present."* A worktree-isolated dispatch fires this hook through a binary that is
already running, so there is no absent path to resolve and no git query to make. The
`unresolved` arm keeps its producer for the consumer whose **override** is a
compiled-binary path that a fresh worktree does not carry — the section's own words,
*"the arm stays reachable once a consumer's reader takes the worktree-resolvability
requirement below, and the row is not retired"*.

**Not re-implementing it is a narrowing of behavior and it is deliberate**, so it is
stated rather than left to be discovered: today a consumer whose knob names a
compiled-binary path gets that path repaired inside a worktree by this repo's adapter;
after this cut the same consumer's override gets no repair, and the requirement is
theirs. The repo that had the adapter is the repo that no longer needs one.

### (4) `delegation-kit/smoke/install.sh`'s allowing scenario is invalidated by the default and must be re-authored

This is the cut's one **measured** break and it is not a lint {design-bearing}.
`delegation-kit/smoke/install.sh:63-81` seeds `$sp/smoke.run` with the record
`pid=1 run=smoke`, sets `DELEGATION_KIT_LIVENESS_CMD=""`, fires the hook, and asserts
`verdict=unavailable records=1 runs=smoke decision=allow` at **exit 0**. Its `# spec:`
line at `:63` states the premise verbatim: *"the knob is emptied, so the firing holds
no reading (verdict=unavailable) whatever the run dir carries"*.

Under delta (2) that premise is false in the worst direction. The empty knob resolves
to the compiled reader, the compiled reader reads `pid=1`, and PID 1 is init and is
**live** on every host this smoke runs on — so the firing becomes `verdict=red
live=yes decision=refuse` at **exit 2** and the smoke fails on its own allowing arm.
The failure would present as a hook regression rather than as a stale scenario, which
is why it is a delta and not a build-time surprise.

**The re-authored scenario keeps the arm and changes the route to it.** The allowing
arm wanted a firing that allows; the way to buy one under a working default is a run
dir the reader can read and clear — a record naming a **dead** PID, which reads
`green`, or an empty run dir, which reads `green` over zero records. `unavailable` is
reachable only through an unresolvable **override** now, so a scenario that wants that
verdict names a path that does not exist. Which of the two the smoke asserts is
build's to settle against the grammar the section fixes; both are in contract and
neither needs a new knob.

### (5) `scripts/delegation-config.sh`'s knob assignment and its `# spec:` both go

Three lines leave: the `# spec:` at `:10`, the shellcheck directive at `:11`, and the
guarded assignment at `:12` naming the deleted path {mechanical}. The `# spec:` is not
a comment to relocate — its whole subject is why *this repo's* value must be guarded
rather than assigned, and with no value there is nothing to guard. The file's own
`# no-port:` cause at `:3` is untouched: it declares on the config-seam class and this
delta removes one knob from the file rather than changing what the file is.

**The knob's kit-side default at `delegation-kit/lib/delegation.sh:40` stays exactly
as it is** — `[[ -v DELEGATION_KIT_LIVENESS_CMD ]] || DELEGATION_KIT_LIVENESS_CMD=""`
— and that is a finding rather than an omission. The kit still ships no *path*; what
changed is what the empty value **means** on the reading side, which is delta (2)'s
subject and lives in the crate. Teaching the bridge to spell the binary's own path
would put a second producer where §lib/gate.sh rules there is one.

### (6) The negative control moves from the dead default to the unresolvable override

`scripts/gate-tests/subagent-stop-reader.test.sh` exists to prove that the reader this
repo configures *actually answers* — its own header names the failure it was written
against, a port that deleted the reader's target and left the hook logging
`verdict=unavailable` on every firing under a green battery {design-bearing}. The suite
splits three ways at this cut and the split is the delta:

- **The negative control survives with its subject intact.** The muted-reader arm
  (`:85`, `DELEGATION_KIT_LIVENESS_CMD="$mute"`) still produces `unavailable`, because
  under delta (2) an unresolvable *override* is that verdict's remaining producer.
- **The positive arm re-points from a path to an absence.** Its premise — that the
  configured value is `scripts/producer-liveness-reader.sh` — is gone, so the arm that
  proves the default answers now fires with the knob **unset** and requires a verdict
  from the compiled reader.
- **Arms D and E are rewritten or removed, not re-pointed** (`:101-121` and `:126-129`).
  They `exec` the deleted file **directly by path** rather than through the knob, and
  their subject is that file's worktree resolution and its broken-main-checkout-binary
  behavior — delta (3)'s two properties. With the resolution discharged by construction
  there is no code path left for them to drive, so the honest disposition is removal
  rather than a re-point onto a default that cannot fail the way they assert.

**The suite is this cut's own oracle and is run before the deletion, not after.** Its
whole reason for existing is the regression delta (2) is written to prevent, so the
order at build is: land the crate default, run this suite green with the knob unset,
then delete.

### (7) The four sections of §The turn-end liveness hook that narrate the adapter are rewritten in place

Not appended to {mechanical}. §The turn-end liveness hook currently reads as a
narrative — a ruling, its lapse, and a consumer adapter that is about to stop existing
— and three of its paragraphs name the deleted script or its front end in the present
tense. Each is rewritten to the post-cut state with its *ruling record* preserved:
what the kit refused and why it refused it stays, because §The turn-end liveness
hook's own text turns on that history; what changes is every sentence that describes
this repo's tree as it will not be. The specific targets are enumerated under
*Existing sections updated* below.

## Producers and consumers

This cut introduces **no new state, no new event, no new field and no new knob**. It
changes the *producer* of one existing value and deletes one adapter. The
causal-completeness survey is therefore over the readers of that value across the
whole component set, and it was run by grepping every tracked file for
`DELEGATION_KIT_LIVENESS_CMD` and for `producer-liveness-reader` with stderr left
open.

**The value whose producer changes: the liveness reader's argv.**

- **Producer, before:** `scripts/delegation-config.sh:12` sets the knob to
  `scripts/producer-liveness-reader.sh`; the config bridge resolves it; that script
  resolves the gate binary (repairing a worktree path against the main checkout) and
  execs it by gate name.
- **Producer, after:** `native/src/hook/stop_liveness.rs`. Where
  `walk::knob_scalar("DELEGATION_KIT_LIVENESS_CMD")` yields the empty string, the arm
  derives `std::env::current_exe()` and spawns it with the gate name and the run dir.
  Where it yields a value, the value is spawned unchanged — the existing path, kept.
- **Enabling config, and the point of the cut:** there is none to emit. The default
  needs no knob set anywhere, which is what makes it a default rather than a second
  configuration this repo happens to hold.

**Consumer:** `read_liveness` in the same module, which maps the child's status onto
the six-arm verdict table (`stop_liveness.rs:83-94`). Its mapping is unchanged — that
is delta (2)'s stated requirement, not a coincidence.

**Every reader of the knob, surveyed across the whole tree, with its disposition:**

| reader | site | disposition |
| --- | --- | --- |
| the arm that resolves it | `native/src/hook/stop_liveness.rs:45` | gains the default branch, delta (2) |
| the arm's declared roster | `native/src/hook/mod.rs:43` | unchanged — the knob survives as the override |
| the kit-side default | `delegation-kit/lib/delegation.sh:40` | unchanged, delta (5) |
| this repo's value | `scripts/delegation-config.sh:10-12` | deleted, delta (5) |
| the kit's consumer smoke | `delegation-kit/smoke/install.sh:63-81` | re-authored, delta (4) |
| this repo's negative control | `scripts/gate-tests/subagent-stop-reader.test.sh:20,85` | re-pointed, delta (6) |
| the knob roster prose | `delegation-kit/SPEC.md:3104`, `:3153` | re-worded, below |

**Every reader of the deleted script, same survey:** `scripts/delegation-config.sh:12`
(delta 5), `delegation-kit/SPEC.md:1173`, `:1187` and `:1208-1211` (below),
`TASK-QUEUE.md:210` and `:7145` (prose on answered entries, left as the historical
record they are — a queue entry narrating what was true when it was filed is not a
stale citation), and `scripts/gate-tests/subagent-stop-reader.test.sh` (delta 6). No
other tracked file names it.

**Named reader for the one thing that could look like a new field.** The default's
argv is not a new interface: it is the *existing* reader argv — `<reader> <run-dir>` —
with a different first token, read by the same `read_liveness` at the same transition.
Nothing is added that would need a reader, which is why this section names none.

## Existing sections updated

- `delegation-kit/SPEC.md` §The turn-end liveness hook, the paragraph beginning **"The
  kit ships no default reader"** (`:1154-1174`) — its closing two sentences name
  `scripts/producer-liveness-reader.sh` and *"the front end its whole pre-flight
  roster already uses"* in the present tense, both of which this cut deletes
  (deltas 1, 5). The refusal it records — teaching the knob to resolve a *name*, and
  why — stays, because the paragraph after it is the lapse of that refusal's premise
  and reads as a non-sequitur without it.
- `delegation-kit/SPEC.md` §The turn-end liveness hook, the paragraph beginning **"That
  ruling's premise has lapsed"** (`:1176-1189`) — moves from the future tense of a
  scheduled cut (*"This repo's shell reader and the front end it exec'd leave the tree
  with that cut"*) to the present tense of a landed one, and gains the one thing it
  does not state: that the default is **spawned** rather than called in-process, and
  the seam ground for it (delta 2).
- `delegation-kit/SPEC.md` §The turn-end liveness hook, the paragraphs on the
  worktree-resolvability requirement (`:1191-1224`) — *"This repo's own reader
  satisfies it by resolving the configured binary against the main checkout … deriving
  the main checkout vendor-neutrally from `git rev-parse --git-common-dir`"* describes
  a file that no longer exists. The **requirement** on a consumer's adapter stays
  unchanged and so does the cost paragraph beside it; what is rewritten is the worked
  example, which becomes the default's own discharge (delta 3).
- `delegation-kit/SPEC.md` §Layout and configuration, the `DELEGATION_KIT_LIVENESS_CMD`
  entry (`:3104`) and the registration prose at `:3153` — *"the liveness reader the
  probe invokes in set mode"* becomes the override over a stated default, so the
  roster line states what an unset knob now means (deltas 2, 5).
- `delegation-kit/smoke/install.sh:63` — the `# spec:` line whose text *is* the
  invalidated premise (delta 4). It is an update target rather than a code change
  alone because the sentence is a contract citation, and leaving it while changing the
  code beneath it is the restatement-drift CLAUDE.md's comment doctrine bars.
- `scripts/gate-tests/subagent-stop-reader.test.sh:1-3` — the header stating what the
  arm *"defaults to"* (delta 6).
- `native/src/hook/stop_liveness.rs:182-183` — the `// spec:` comment asserting that the
  reader *"stays external and stays spawned … not of this cut"* (delta 2). It is a
  contract citation bound to the line beneath it, so it is rewritten with the branch it
  annotates rather than left to contradict the section it cites.
- `gate-sdk/SPEC.md` §The port disposition — the owed-corpus prose that this cut moves
  by one file (all deltas). Per §The first cohort's budget-arm rule, *"a budget batch
  adds a section to this SPEC only where it has a finding to record"*, and this cut's
  finding is delta (1)'s class correction: a `scripts/`-resident consumer **adapter**
  is homed at the kit section whose contract it implements, not at this repo's
  config-and-vocabulary class and not at an always-loaded manifest.

<!-- update-target-exempt: the deleted file's own header is removed with the file, so no delta can claim it as a surviving target -->
- `scripts/producer-liveness-reader.sh` — deleted whole, header and `# spec:` with it.

## Definition of Done

- [ ] **Causal completeness** — the default reader has a named, reachable producer
      (`stop_liveness.rs`'s empty-knob branch) and a named consumer (`read_liveness`'s
      exit-class mapping); no new field is added, and every existing reader of the knob
      and of the deleted script is dispositioned in the table above.
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section (not appended); §The turn-end liveness hook reads as one
      document to a reader who never saw this amendment, with its refusal record intact
      and no sentence describing a file the tree no longer holds.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls delegation-kit/SPEC-*.md`).
- [ ] **The default lands before the deletion, in the same commit** —
      `scripts/gate-tests/subagent-stop-reader.test.sh` green with
      `DELEGATION_KIT_LIVENESS_CMD` **unset** before `scripts/producer-liveness-reader.sh`
      is removed. Delta (5) is not landable without delta (2); the attested cost of the
      other order is 77 silent `unavailable` firings under a green battery.
- [ ] **Removals propagated** — every tracked file grepped for
      `producer-liveness-reader` with stderr open; nothing dangles outside the queue's
      own historical record. `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`
      re-run for the `docs/delegation-kit/SPEC.md` mirror.
- [ ] **The oracle re-read, not the arithmetic trusted** —
      `bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree` re-run at the landing
      commit and its owed count recorded; the count is the predicate, and this cut's
      claim on it is one file and 26 lines.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed to the
      committed gap inbox (a build-time causal gap is resolved that session, not
      deferred).
