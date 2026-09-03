# SPEC amendment: generator-spawn

The repair of **one spawn** — `native/src/gates/graph.rs:379`, `generator_emit`'s
`proc::run("bash", &[gen, arm])` — which is the single cause the Windows CI leg has
returned on for five consecutive rounds. Admitted as an **operator-directed hotfix**
(2026-09-03, lead-relayed), *minimal and test-and-doc-complete in one commit*, in
CLAUDE.md §Delivery doctrine's sense; unsequenced against this iteration's two port
cuts, the sequenced variant having been offered and declined. `platform-support-ci-matrix`
records the ruling and owns the round history; nothing of that history is restated here.

**The envelope is ruled and it is narrow — ruled by the lead 2026-09-03 on its own
authority.** The crate carries no bash resolver and about twenty bare `"bash"` spawn
sites; **only the one that reds is repaired here**, and a class-wide resolver is *not*
admitted. The grounds are derivable rather than preference: a resolver covering the
whole class is not *minimal*, and a feature unit beyond the hotfix is a **yield**, which
TRAJECTORY.md §PRIORITY DIRECTIVE's port-only run refuses with exactly one exception —
the operator-ruled hotfix, which this is. The residue is **filed, not carried**: the
other bare-spawn sites go to the committed gap inbox as a work-shaped finding for close
to drain. Ruling narrow is applying the operator's ruling; ruling wide would be
re-scoping it.

**Why this is a feature owing an amendment at all**, given that it repairs a defect: it
introduces a **resolution contract the crate does not carry today** — a rule about what
a spawned program name means — which is a new name on a governed surface and so a
feature by canon-kit/SPEC.md §The amendment lifecycle's new-names litmus. The litmus
was run at scope and this stage authors rather than re-triages.

**The mechanism, traced to source rather than inferred**, and already recorded on
`platform-support-ci-matrix` from round 7's instrumented run: the installed binary is a
**native Windows process**, so a bare program name is resolved by a Win32 `PATH` search
in which `%SystemRoot%\System32` precedes Git-for-Windows' `usr/bin`. `System32\bash.exe`
is the **WSL launcher** — a dispatcher into a Linux VM, not a bash — so it is what runs,
and on a runner with no distro installed it exits 1 saying
`Windows Subsystem for Linux has no installed distributions`. Assertion D reported
exactly that on both `--emit` arms, on stdout with stderr empty, which is the shape the
2026-08-31 widening was bought to make readable.

**The scope of the red is one assertion, and that bounds the repair.** Only assertion D
spawns; the rest of `check-graph` reads files in process. `bin/gen-pre-commit.sh` stays
shell on its own declared cause (§gen-pre-commit), so the spawn is permanent and cannot
be designed away by porting the generator.

## What changes

### (1) The interpreter is **resolved**, and a bare program name stops being a spawn's argument

`generator_emit` spawns a **resolved absolute path** instead of the bare name `bash`
{design-bearing}. The resolution is `PATH`-ordered exactly as the operating system's own
search is, with **one rejection**: on a Windows target a candidate resolving inside the
Windows **system directory** — `%SystemRoot%\System32` and its `SysWOW64` and `Sysnative`
views — is skipped. That directory holds the only `bash.exe` Windows itself ships and it
is the WSL launcher; nothing else the payload wants is ever found there. On every other
target the rejection is inert, because no candidate can match it, so the repair changes
no behaviour on the platforms the battery runs on today.

**The comparison is case-insensitive**, because Windows paths are, and a `PATH` entry
spelled `c:\windows\system32` is the same directory as `C:\Windows\System32`. This is
stated as contract rather than left to the implementation: a case-sensitive test would
pass on every developer machine and fail on the one host the repair exists for.

### (2) The rule lives in `proc.rs`, has exactly one reader, and the comment says so

The resolution lands beside `which` / `resolve_on_path` / `exe_candidates` in
`native/src/proc.rs`, because that module already owns `PATH` splitting, the `PATHEXT`
candidate set and the executability predicate, and a second copy in `graph.rs` would be
the duplicate criterion 6 refuses {design-bearing}. Its public face is one function whose
**`spec:` comment states three things**: the rule of delta (1), that its one reader is
`gates::graph::generator_emit`, and that **the crate's other bare-`"bash"` spawn sites
are deliberately not re-pointed by this repair** and are open work under the filed gap.
That comment is the structural guard on the envelope: a later batch meeting a resolver
in `proc.rs` will otherwise read it as the class fix and sweep the callers, which is the
widening the operator's ruling forbids.

**A location in `graph.rs` was weighed and refused**, and the refusal is the reason the
comment above has to carry the envelope instead: keeping the helper at its one call site
would make the narrowness structural, but it would mint a second `PATH` resolver in a
tree that has exactly one, and de-literalization outranks the legibility gain.

### (3) The refusal when nothing resolves is **named**, and it reaches the reader assertion D already has

When no candidate survives the rejection the function returns an error naming the cause —
no `bash` on `PATH` outside the Windows system directory — and `generator_emit` returns
it on the `Err` arm it already has {design-bearing}. That arm is §Fail-closed contract's
*spawn failure and nothing else*, so the refusal is a check-could-not-run verdict rather
than a finding, exactly as an absent `bash` is today. It costs no new channel: §check-graph
already rules that assertion D's refusal carries the generator's whole account of itself,
and this is one more account with a cause in it rather than the bare refusal that cost a
CI round.

### (4) Criterion 7 gains the distinction it was silent on, and this is the delta with reach beyond the site

`GATE_SDK_PROGRAM_FLOOR` guarantees that a program **exists** on an adopter's host; it
has never guaranteed that a **bare name resolves to it**, and criterion 7's own text
reads as though it did {design-bearing}. That silence is what cleared this spawn: the
`spec:` comment at `graph.rs:373-374` says *"criterion 7 clears the spawn because `bash`
is on the program floor"*, which was true of the floor and false of the spawn. The
distinction lands at §The port-candidate criteria, criterion 7, because that is the
criterion doing the clearing, and §lib/gate.sh's `GATE_SDK_PROGRAM_FLOOR` bullet points
at it in one sentence rather than restating it — the knob owns the set, the criterion
owns what membership entitles a spawn to assume.

**This is the transferable half of the repair and it is deliberately prose rather than a
gate.** A gate that read every spawn site and demanded a resolved path is the class fix
the envelope refuses; naming the distinction where the next porting session already
looks is what the hotfix can honestly carry.

### (5) The graph module's own `spec:` comment is corrected, not merely extended

`native/src/gates/graph.rs:373-374` currently asserts a clearance this host falsifies, so
the sentence is rewritten rather than appended to {mechanical}: the generator stays shell,
assertion D spawns it, and the interpreter is resolved because the floor's guarantee does
not reach a bare name. A comment that still states the false clearance beside the code
that stopped relying on it is the restatement CLAUDE.md's comment doctrine refuses.

### (6) The `.gate` descriptor does **not** change, and the negative is stated because the instinct is to add a couple

`gate-sdk/checks/check-graph.gate` couples `native/src/gates/graph.rs` and
`native/src/emit/graph.rs` and names no universal layer {design-bearing}. §The non-gate
arm's transitive-coupling rule *stops at the universal layers* — `walk.rs`, `proc.rs`,
`registry.rs` — and the tree is what says so: no descriptor in it names any of them. So
this repair adds `proc.rs` to no `couples=` field, and a build batch that adds one is
re-running the whole battery from the generated hook on every spawn-layer edit, which is
de-literalization inverted.

### (7) The residue is filed and named, and this delta is what stops it being carried

The crate's other bare-`"bash"` spawn sites — in `native/src/evidence.rs`,
`native/src/emit/upgrade_smoke.rs`, `native/src/runner.rs` and elsewhere — keep spawning
the bare name and are **not** touched by this commit {mechanical}. They are filed to the
committed gap inbox (`lifecycle-kit/bin/file-gap.sh`) as a work-shaped finding at this
stage, so close disposes of them rather than a build batch adopting them on its own
authority. Under the port-only run close promotes nothing, so the expected disposition is
the icebox; the filing is owed either way, and the knowledge-friction stamp scope left is
not a filing.

### (8) What this unit's completion **is**, stated because getting it wrong re-arms a known wedge

The deliverable is the **code, its tests and its documentation** — never a green Windows
leg {design-bearing}. The leg's observation belongs to `platform-support-ci-matrix`, whose
`continue-on-error` and `targets.list` consequences both wait on a first-observed-green
run and stay unexecuted here. An entry whose completion predicate is an observation it
cannot make in its own iteration is exactly the wedge
`observation-predicate-entry-cannot-drain-in-its-own-iteration` records and
`platform-support-ci-matrix` has already been demoted by once; this entry is written so
it drains at this iteration's close on evidence this iteration can hold.

## Producers and consumers

The repair introduces **no new state, event, message or field**. It introduces one new
**interface**: a resolution function in `native/src/proc.rs`. The checklist therefore
runs over that interface and over the behaviour change at the one call site.

- **Producer.** The function itself, called from `gates::graph::generator_emit` on every
  `check-graph` run that reaches assertion D — which is every run, since assertion D has
  no input-dependent guard. Its inputs are the process environment's `PATH` and, on
  Windows, `PATHEXT` and `SystemRoot`, all read through the accessors `proc.rs` already
  owns. There is no enabling configuration to set and none is minted: the rule is not a
  knob, for the reason delta (1) gives — a WSL bash cannot satisfy this gate on any host,
  so there is no legitimate consumer choice to expose, and a knob would be a governed name
  with no reader that could rationally set it.
- **Consumer.** `generator_emit`, and through it `check-graph`'s assertions on the
  pre-commit hook, the commit-msg hook and the graph artifact. It is the **only** consumer
  and delta (2) requires the comment to say so.
- **No new field.** The function returns the resolved path or a named error; both arms are
  consumed at the same call site, in the `Ok`/`Err` split `generator_emit` already has.
- **The existing integration prose is updated in this amendment**, not left to drift:
  criterion 7, the `GATE_SDK_PROGRAM_FLOOR` bullet, §check-graph's spawn paragraph and the
  module's own `spec:` comment are all named under §Existing sections updated.

**Every reader's RED condition.** This change narrows no corpus — it deletes no file,
prunes no glob and drops no declaration — so canon-kit/SPEC.md §The causal-completeness
check's point 5 does not bind in its hard form. The enumeration is given anyway, because
the point of it is to say where to look when one goes red:

- **`check-graph` itself** — reds when a hook or the artifact diverges from the
  generator's `--emit` output, and reds *differently* when the generator cannot be run at
  all. On Linux the resolved path is the same program the bare name found, so the verdict
  is unchanged and a changed verdict is a defect in the resolution rather than in the
  hook. Monotone in neither direction and therefore **run, not inspected**.
- **`check-crate-arms`** — reds on a failing `cargo clippy` or `cargo test` arm, so it is
  the reader of delta (1)'s tests. Its red condition is any failing test, which is
  monotone, and it is what makes the new pure-function tests load-bearing rather than
  decorative.
- **`check-comment-tier`** — reds on a comment that is not a directive. Delta (5) rewrites
  one comment and delta (2) adds one; both must be directives, and relocating the false
  clearance behind a `spec:` tag rather than correcting it is the blessing-a-restatement
  defect CLAUDE.md names.
- **`check-gate-binary-fresh`** — reds when the committed binary's source stamp does not
  match the tree, so it fires on this edit and is discharged by
  `bash gate-sdk/bin/build-native.sh`. It is an **equality**, not monotone, and it is the
  commit-time obligation CLAUDE.md states beside the battery.
- **`check-reads-couples` and `check-graph`'s own manifest assertions** — read `couples=`
  fields, which delta (6) does not change; both stay green *because* nothing is added.
- **`check-measured-claim` / `check-unmarked-claim`** — the §check-graph runtime paragraph
  carries measured figures. This repair changes neither the spawn count nor the measured
  medians, and the paragraph's numbers are not re-measured by it; a delta that touched
  them would owe a re-measurement rather than an edit.
- **The port oracle `--emit port-blockers`** — reports rather than reds, and its `--tree`
  owed count is **unchanged at 43 by this unit alone**: no `.sh` file is added, deleted or
  declared. The two port cuts riding this iteration move it; this one does not.

## Existing sections updated

- `gate-sdk/SPEC.md §check-graph` — the spawn paragraph gains the resolution: assertion D
  spawns a resolved interpreter rather than a bare name, the rejection rule and its
  case-insensitivity, the named refusal when nothing resolves, and the fact that this is
  the crate's only resolved spawn today (deltas 1, 3 and 5).
- `gate-sdk/SPEC.md §The port-candidate criteria`, **criterion 7** — the distinction the
  criterion was silent on: the floor guarantees the program's presence, never that a bare
  name resolves to it, with this spawn as the attested instance (delta 4).
- `gate-sdk/SPEC.md §lib/gate.sh`, the `GATE_SDK_PROGRAM_FLOOR` bullet — one sentence
  pointing at criterion 7 for what membership entitles a spawn to assume; the knob keeps
  sole ownership of the set and the criterion owns the entitlement, so neither restates
  the other (delta 4).
- `native/src/proc.rs` — the new function and its `spec:` comment, which carries the rule,
  its one reader, and the envelope's own limit (deltas 1, 2 and 7).
- `native/src/gates/graph.rs` — the call site and the corrected `spec:` comment at
  lines 373-374, whose present claim is falsified by the host this repair exists for
  (deltas 1 and 5).
- `TASK-QUEUE.md`, the new `generator-spawn-resolves-wsl-launcher` entry — filed with this
  amendment's `[spec:]` ref in the same commit, carrying both 2026-09-03 rulings (the
  operator's admission by lead-relay and the lead's own-authority envelope) and stating
  its completion predicate as the code rather than the run (deltas 7 and 8).

<!-- update-target-exempt: this entry owns the round history and the observation predicate, both of which stay untouched by design; its continue-on-error and targets.list consequences wait on a first-observed-green run, and it is at 50 of a 50-line cap so any write here would cost a compression the repair does not need -->
- `TASK-QUEUE.md`, `platform-support-ci-matrix` — deliberately unwritten.

<!-- update-target-exempt: the generator stays shell on its own declared cause and this repair neither ports it nor changes what it does; naming it here is what stops a reader taking the resolved spawn for a step toward porting it -->
- `gate-sdk/SPEC.md §gen-pre-commit` — deliberately untouched.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The attested host configuration is a test** — a `PATH` whose first entry is a
      Windows system directory holding a `bash` candidate and whose second is a
      Git-for-Windows `usr/bin` resolves to the **second**, driven through the pure
      function with `PATH`, `PATHEXT` and the existence predicate all injected, exactly as
      `resolve_on_path`'s existing tests are. Round 7's measured configuration, reproduced
      as an assertion that runs on a Linux developer machine.
- [ ] **The lowercase spelling is a test of its own** — `c:\windows\system32` is rejected
      as surely as `C:\Windows\System32`, because a case-sensitive comparison passes
      everywhere except on the one host this exists for.
- [ ] **The refusal arm is a test** — a `PATH` offering only the rejected directory yields
      the named error, not a silent fall-through to the bare name and not a panic.
- [ ] **The inert arm is a test** — a POSIX `PATH` resolves the first match unchanged, so
      the repair is proved to change nothing on the platform the battery runs on.
- [ ] **The other bare-`"bash"` spawn sites are UNCHANGED in the diff**, and the commit is
      read for that specifically rather than assumed — the envelope is the ruling, and a
      helpful sweep of the siblings is the one way this unit can fail while every gate is
      green.
- [ ] **The residue is in the gap inbox** before the commit lands, with its site count
      probed rather than carried from this amendment's approximate figure.
- [ ] **`check-comment-tier` is green on the corrected comment**, and the correction is a
      rewrite: the false clearance is not relocated behind a tag.
- [ ] **The binary is rebuilt** (`bash gate-sdk/bin/build-native.sh`) in the same commit,
      the obligation CLAUDE.md states beside the battery and which neither discharges.
- [ ] **No green Windows leg is claimed** — the entry drains on the code, and the
      observation stays `platform-support-ci-matrix`'s.
