# SPEC amendment: hooks-cut

The port disposition of **`gate-sdk/bin/install-hooks.sh` (59 lines), the one owed
file declaring §install-hooks**, onto the binary substrate as a bridged `Arm::Run`
member spelled `--install-hooks`. This is a stated-contract cut under the port-only
run (TRAJECTORY.md §PRIORITY DIRECTIVE), composed at scope and packaged by the
**lead on its own authority**, 2026-09-04, over the resume channel; it did not reach
the operator, and that is stated because a packaging ruling recorded without its
authority reads later as more settled than it is.

**Measured at this HEAD rather than carried from scope's survey**: the port oracle's
`--tree` arm reads 98 files scanned, 64 declared `no-port`, 0 temporarily held,
**34 owed**. The selection ground is that owed column and never the registry
`--group` read (gate-sdk/SPEC.md §port-blockers). The ported member is not a gate: it
is in no `gates.list`, carries no `.gate` descriptor, and owes no fixture pair.

**This cut carries the batch's drop-first permission, and it is written here so a
build session holds it without escalating.** The lead ruled 2026-09-04, on its own
authority, that if the build window proves tighter than the (unmeasured, structural)
reasoning assumed, **this cut parks and the other three land unchanged**. The
property behind that is gate-sdk/SPEC.md's *no joint proof* — dropping a member
mid-batch invalidates nothing — reached here by analogy rather than by its own terms,
since that paragraph is written over registry gates with descriptors and fixture
pairs and none of these four is one. What is true on these cuts' own facts is
narrower and holds: this cut shares **nothing** with the other three — no twin, no
library, no kit, no gate — so parking it strands no work any of them created.

**Its convergence onto a landed precedent is real but partial, and the amendment says
which half.** `native/src/emit/install_lifecycle.rs:87-101` already names and
implements this member's **class** — its `register_driver` step is documented in its
own directive as "the `install-hooks.sh` per-clone opt-in class", degrading to a
printed skip on stderr at exit 0 outside a repo. That half is convergence and its
deltas are mechanical. The other half is not: the apply-and-verify rung reaches
another gate through the registry, and moving it in-process would delete a
consumer-facing seam §install-hooks asserts by name. Delta (4) is that ruling, and it
is the reason this cut's delta set is **not** all-mechanical.

## What changes

### (1) The cut is the one owed file declaring §install-hooks, and its directive reach coincides with its bound

`bin/install-hooks.sh` is the **only** owed file declaring `### install-hooks`
{design-bearing}. All four of its `# spec:` blocks — `:2`, `:31-33`, `:34-36` and
`:48-50` — bind that one section, so unlike the sibling evidence cut this amendment's
reach and its section bound coincide and no second section is rewritten by
construction.

**It does not discharge the kit, and the amendment says so because gate-sdk is the
one kit where a reader would assume otherwise.** The kit's owed column keeps
`bin/run-gates.sh` and `bin/build-native.sh` — the two operator-class blockers whose
disposition a scheduled consult reaches — plus `bin/run-gate-tests.sh`,
`bin/gen-pre-commit.sh`, `bin/run-consumer-smoke.sh`, `lib/inject.sh` and
`lib/test-hermetic.sh` on their own separate grounds. This cut settles one section
and nothing else.

### (2) `--install-hooks` is a bridged `Arm::Run` with its own front-end case arm

The opt-in lands as `native/src/emit/install_hooks.rs` and registers as one
`BRIDGED_ARMS` row beside `--install-lifecycle` (`native/src/emit/mod.rs:470-474`),
with its own `case` arm in `gate-sdk/bin/run-gates.sh` beside that member's
(`run-gates.sh:197-202`) {design-bearing}.

**The family is forced by the exit contract.** `native/src/main.rs:483-491` maps
`Arm::Emit` onto `exit(0)`/`exit(2)`, a family that can never return **1** — and 1 is
precisely what this member propagates: §install-hooks rules that `check-identity`'s
"exit status surfaces through this script's", and that gate's contract is 0 clean,
**1** a wrong-identity or wrong-remote mapping, 2 an uninterpretable manifest. The
member's own guard — a nonexistent hooks dir — is a further 2. So the contract is
three-state and every code is load-bearing, which is the same sentence
`--enter-stage`'s own row already carries. `Arm::Run` (`native/src/main.rs:493`)
passes the arm's `i32` through verbatim, and `exec_arm` ends in a true `exec`
(`run-gates.sh:127`), so the status survives the front-end.

**It emits no document**, which independently excludes the `--emit-` family, and it
takes its own top-level spelling for its own contract as `--install-lifecycle`,
`--enter-stage`, `--usage-verdict`, `--upgrade-smoke`, `--lesson-sink` and
`--wait-probe` do.

**And it declines the `--install <op>` family**, on that family's own stated terms
and for the same reason `--install-lifecycle` did: `--install` is deliberately
unbridged because its caller is the installer bootstrap, which may not be assumed to
be a POSIX shell, so every value it needs arrives as argv. This member resolves
`GATE_SDK_HOOKS_DIR` and the check-dir roster from kit config, so it cannot live
there. The name collision is what makes this worth stating — it is the first place a
reader looks.

### (3) The declared knob roster is two names, and both are already bridged

`GATE_SDK_HOOKS_DIR` and `GATE_SDK_GATES_DIR` {mechanical}. Neither is minted:
`GATE_SDK_HOOKS_DIR` is declared and defaulted at `gate-sdk/lib/gate.sh:63` as
`<gates-dir>/git-hooks`, and it is a **proven-live bridged knob today** — declared on
two registry rows (`native/src/gates/mod.rs:1319-1325`, `:1411-1424`) and baked twice
into the generated hook (`scripts/git-hooks/pre-commit:105`, `:165`), read in-crate
through `walk::knob_scalar`. `GATE_SDK_GATES_DIR` is what the check-dir roster of
delta (4) is built from, and the crate resolves it inline at several existing sites
rather than through an accessor, which is the shape this arm follows.

The forced-family test settles the registration: both defaults live in a `no-port`
library the bridge sources, so a hardcoded top-level flag would resolve platform
defaults and ignore every consumer override.

**`GATE_KIT_ROOTS_HERE` is not declared and must not be**, which is worth one
sentence because the check-dir roster looks like it needs it: `kit_roots` is
transported rather than re-derived by standing crate invariant
(gate-sdk/SPEC.md §The non-gate arm), so the arm reads it the way every other member
does and declaring it a second time would be a second producer.

### (4) The apply-and-verify rung resolves through the registry, and an in-process call by name is REFUSED

This is the cut's central seam ruling and the reason its delta set is not
all-mechanical {design-bearing}.

**The hazard the shell rung was written against dissolves, and the amendment records
that first so the ruling below is not read as caution.** §install-hooks' own ground
for reaching the gate through `gate_command` is that "a descriptor interprets to
success" — running `bash` on a `.gate` file, whose whole content is comment lines,
exits **0**, so a rung spelled that way would not crash but *pass*, retiring the
verification it exists to perform with no diagnostic anywhere. That failure is
attested at gate-sdk/SPEC.md:5432-5444: this very rung "passed silently on a
descriptor" at `check-identity`'s own port, and it was found by grepping the tree, not
by any gate. In-crate the hazard has no analogue at all — there is no path to
interpret, `check-identity` is compiled in (`native/src/gates/identity.rs:190`,
`pub fn run(_args: &[String]) -> i32`), and `gates::lookup`
(`native/src/gates/mod.rs:1720-1725`) hands back the function itself.

**Which is exactly why the tempting move is refused.** Calling
`gates::lookup("check-identity")` — or the module's `run` directly — would resolve
**the crate's** member and ignore a consumer's own. §install-hooks asserts the
opposite in its own words: the gate is "resolved through the registry, so a consumer
shadow wins". The registry resolves consumer-first with kit shadowing, so a consumer
shipping its own `check-identity` — a `.sh` in its gates dir, or a `.gate` shadowing
the kit's — has that one run today. An in-process call by name would silently stop
honouring it. That is a **narrowing of an extension point**, which
`native-gate-port-remaining-corpus`'s ruling (1) forbids outright: a cut narrows the
port, never a consumer-facing seam.

**So the ported rung keeps the resolution and moves only the invocation.** It builds
the check-dir roster — the gates dir first, then each kit root's `checks/`, the
consumer-first order `gate_check_dirs` (`gate-sdk/lib/gate.sh:579-585`) produces —
and resolves the member against it with `registry::resolve`
(`native/src/registry.rs:22-32`) over `registry::resolve_dirs` (`:36-42`). What it
does next is decided by what it found, and the three branches are the contract:

- **a `.gate` declaration** — dispatched in-process by name, which is the branch
  where the compiled form is genuinely simpler than the shell one and where the
  descriptor-interprets-to-success hazard is gone rather than guarded against;
- **a `.sh` declaration** — a consumer shadow, **spawned** as the shell form spawns
  it, because that file is the consumer's rule and the arm is not entitled to
  substitute its own;
- **no declaration in any dir** — a silent skip at the member's own status, which
  §install-hooks already rules a consumer without the gate is entitled to.

**The status-1 branch is preserved for its stated cause, not for symmetry.**
`gate_command` returns 1 for "no such member here" and something else for a dispatch
that could not be built; the section rules the first a skip and the second a failure
of the opt-in. Both readings survive: an unresolvable member skips, and a resolved
member that cannot be invoked fails the opt-in rather than being waved through.

**`gate_command` itself gains no crate twin and needs none**, and that is stated
because its absence would otherwise read as an omission: its whole job is composing a
self-invoking argv with a resolved knob environment, and a caller inside the binary
already has both. The twin the arm needs is the **resolution**, which exists.

### (5) The `chmod`, the two `git config` writes and the listing cross unchanged

Every one of them, and each with its crate answer already in tree {mechanical}:

- **`chmod +x "$HOOKS_DIR"/*`** becomes an iteration over the directory calling the
  per-file setter `native/src/install.rs:163-172` already ships —
  `PermissionsExt`, `perms.set_mode(perms.mode() | 0o111)` — and keeps the shell
  form's tolerance: a failure to chmod one entry does not fail the opt-in, because
  §check-hook-exec-bit rules that this per-clone `chmod` "cannot repair a wrong
  committed [exec] mode" and is therefore convenience rather than an assertion.
- **`git config core.hooksPath <hooks-dir>`** and the conditional
  **`git config blame.ignoreRevsFile .git-blame-ignore-revs`** become `proc::run`
  calls, the shape `native/src/emit/install_lifecycle.rs:91,98` already uses for
  exactly this class of per-clone git-config write. The `.git-blame-ignore-revs`
  guard stays a file-existence test, so a consumer without that file gets the same
  one-line output it gets today.
- **The `ls | sed` active-hooks listing** becomes a `read_dir` and a formatting loop.
  Its output is the opt-in's receipt and its exact shape is preserved, because it is
  the only place a session sees which hooks the wiring just enabled.

**`cd "$(git rev-parse --show-toplevel)"` retires rather than porting.** The shell
form re-roots itself because `GATE_SDK_HOOKS_DIR`'s default is repo-relative and the
script may be invoked from a subdirectory. The arm is reached through the front-end,
which resolves the knob before the exec, so the arm receives a resolved value rather
than a relative one it must anchor. **The non-repo case keeps a defined behaviour**:
the two `git config` writes fail outside a repository, and the arm reports that on
stderr rather than crashing — the same soft degradation
`install_lifecycle.rs:90-97` already rules for its own driver step, whose recorded
honest limit "depends on it failing soft".

**No off-floor program leaves the set, because there was none.** Probed: `dirname`,
`pwd`, `git`, `chmod`, `ls` and `sed` are all on `GATE_SDK_PROGRAM_FLOOR`
(`gate-sdk/lib/gate.sh:182-185`). Criterion 7 is not a driver for this member in
either direction, and the cut claims no floor dividend — stated so a later reader
does not infer one from the deletion of a script.

### (6) The argv-shape split binds trivially, and the cut answers nothing it is not asked

`install-hooks.sh` takes **no positional argument at all** {mechanical}. So the shape
refusal and the `--` escape have no free text to bind on, and the `-h`/`--help` arm
retires to the front-end as it does for every member of the class.

**This cut does not answer `bin-tool-help-arm-absent-tree-wide`'s open question**,
and says so: that entry holds its corpus open precisely because whether the
contract's behaviours bind a **no-positional** tool is unstated, and this member is
one of those. The port makes the question moot for this one file by moving usage to
the front-end; it rules nothing about the others. **A census note is owed at build**
— four of this iteration's cut members sit on that census — and the entry's
derivation is not reproducible as written: it states 20 paths / 17 shipped tools
measured 2026-09-04, while the obvious spelling of its own command returns 19 at this
HEAD, three of them fixtures, so 16 shipped. The pattern is unstated, so the
difference is unattributable rather than a proven staleness; build reproduces the
census with the entry's own pattern before moving the number.

### (7) The payload carries this member wholesale, and the port changes nothing there

`scripts/pack-installer.sh:124-133` packs every `gate_kit_roots_rel` member verbatim,
and `gate_kit_roots` always includes the gate-sdk root unconditionally
(`gate-sdk/lib/gate.sh:546-553`), so `install-hooks.sh` rides the payload today at
`payload/gate-sdk/bin/install-hooks.sh` and the binary rides it as the binary
{mechanical}. `installer/` carries **no separate copy** — probed — so the port lands
in the payload with zero installer mechanism changes.

**Two installer surfaces name the invocation and neither is reached by any gate.**
`installer/lib/init.sh:414` *prints* the follow-up command with `printf` and never
runs it; `installer/README.md:183` narrates the same, and `:930-935` narrates the
`uninstall` path's "the hook opt-in is reported, not rewritten". Both are re-spelled
by hand. The exposure is real and measured: `installer/consumer-smoke/` carries
**zero** references to this member — probed — so a stale printed string there is
caught by nothing, in either substrate, and would be discovered by an adopter
following a command that no longer exists.

### (8) Every path-bearing surface moves in the deleting commit, and three gates red

The roster is probed and split by what a gate catches {mechanical}.

**The gate-caught set, and it is the largest of this iteration's four cuts:**

- **`check-settings-paths`** — `.claude/settings.json:26` carries
  `Bash(bash gate-sdk/bin/install-hooks.sh)`, an **exact-argv** grant with no
  wildcard, and that gate's predicate takes the literal `.sh` token out of a
  `Bash(<command>)` entry. Deleting the file without deleting the grant reds it.
  **The contrast with this cut's own precedent is the point**: the
  `--install-lifecycle` port's settings diff was **empty**, because that script had
  no dedicated grant and was always reached through the wildcard. Here the removal is
  forced rather than optional, it is inside
  `native-gate-port-remaining-corpus`'s ruling (2) carve-out as a narrowing forced by
  a ruled cut, and it lands in the same commit as the delete. **No addition is owed**:
  `.claude/settings.json:12` already grants `Bash(bash gate-sdk/bin/run-gates.sh *)`.
- **`check-docs-cmd`** — three **fenced** invocations resolve to this path and each
  reds on the delete: `gate-sdk/README.md:78`, `docs/gate-sdk/index.md:31`, and
  `docs/install.md:238`. Each becomes
  `bash gate-sdk/bin/run-gates.sh --install-hooks`.
- **`check-graph`** — `gate-sdk/bin/gen-pre-commit.sh:158,242` carry the invocation
  inside the template string baked verbatim into the generated hooks
  (`scripts/git-hooks/pre-commit:10`, `scripts/git-hooks/commit-msg:10`), so editing
  the template without regenerating reds the byte-freshness assertion.
- **`check-docs-mirror-fresh`** — the `docs/` mirrors of every edited kit README and
  SPEC move in lockstep.

**The silent set, fixed by hand because nothing reds:** `CLAUDE.md:61`;
`README.md:144`; `installer/README.md:183` and `:930-935`;
`lifecycle-kit/README.md:93` and its mirror; `gate-sdk/README.md:28`;
`docs/install.md:367,426`; `gate-sdk/SPEC.md`'s prose at `:14134`, `:14271`,
`:15774`; `installer/lib/init.sh:414`;
`.github/ISSUE_TEMPLATE/install-failure.yml:32`;
`gate-sdk/templates/gates-workflow.yml:3`; and `TASK-QUEUE.md:814`, which the
manifest set excludes.

**`gate-sdk/SPEC.md §install-hooks` itself** is rewritten as the arm's section per
deltas (2) through (6). The heading **stays**: three other sections of this SPEC cite
it by name — §check-identity at `:14129-14138`, §check-hook-exec-bit at
`:14271-14272`, §templates/gates-workflow.yml at `:15773-15774` — and deleting the
heading would dangle all three where deleting only the script dangles none.

### (9) Criterion 2's discharge is the both-substrates comparison, bought once, before the delete

This member ships no `good/`+`bad/` fixture pair and owes none — it is not a gate
{design-bearing}. **And the new-gate fan-out does not apply**, which is worth stating
because the roster in docs/site-architecture.md is written for a `.gate` member with a
fixture pair: this port adds no descriptor, no `gates.list` entry and no fixture
directory, so the wide new-gate checklist is not triggered and only delta (10)'s
narrower set is.

The comparison this cut owes, run in one session with both implementations present,
in a scratch clone so the `git config` writes are observable and disposable:

- the **printed receipt**, byte for byte — the `Installed: core.hooksPath = …` line,
  the conditional `Installed: blame.ignoreRevsFile = …` line in both its present and
  absent forms, the `Verifying git identity (check-identity)…` banner, the
  `Active hooks:` listing with its two-space indent, and the closing
  `Disable with:` line;
- the **git config state after each run** — `core.hooksPath` and, where the file
  exists, `blame.ignoreRevsFile`;
- the **file modes** under the hooks dir, before and after;
- the **exit status** at each of: a clean opt-in (0), an identity mismatch (1), a
  malformed identity manifest (2), an absent hooks dir (2), and a consumer whose
  check dirs resolve `check-identity` nowhere (the identity rung's skip, status
  unchanged from the wiring's own);
- **delta (4)'s three resolution branches**, each proved rather than reasoned: a
  `.gate` member dispatched in-process, a consumer `.sh` shadow **spawned** and its
  status propagated, and an unresolvable member skipped. The middle branch is the one
  the port could silently lose, so it is the one the comparison exists for.

A comparison a session runs before a delete is evidence; an arm that can only skip
after it is not (gate-sdk/SPEC.md §The non-gate arm).

### (10) The regeneration fan-out this cut stales

Deleting one owed `.sh` moves `scripts/measured-claims.sh`'s `tree-shell-owed` key
(`:42`) from **34 to 33** {mechanical}. docs/site-architecture.md §Generated
projections and their freshness gates names this exact mechanism at `:147-150` — a
tree edit that *moves* a measured claim stales the generated hooks, whose baked
invocation carries `check-measured-claim`'s resolved values — and this cut trips it
twice over, since delta (8) also edits the hook template itself. The regen is
`bash gate-sdk/bin/gen-pre-commit.sh --write` then
`bash gate-sdk/bin/run-gates.sh --emit graph > docs/check-graph.html`; the SPEC and
README edits stale their `docs/` mirrors and the crate change stales the binary.
`check-graph`, `check-docs-mirror-fresh`, `check-gate-binary-fresh` and
`check-measured-claim` are the reds, all discharged in the landing commit.

**Two keys that do not move, probed rather than assumed:** this member is not a
`gates.list` entry, so the registry-scoped loop at `measured-claims.sh:19-23` never
sees it and both `ported-gate-members` and `gate-substrates` are unaffected. **And no
`measured:` marker binds `tree-shell-owed` in any tracked `.md`** — probed; the live
markers are `ported-gate-members=108` at `docs/install.md:212` and
`gate-substrates=native` at four sites — so `check-measured-claim` stays green from
the marker side and the staleness is the baked hook alone.

**`check-install-claim` is cleared by probe rather than by argument**, because a
narrowing that removes a declaration's sole instance flips a zero-count gate red and
that is the first thing to check. Its singleton is
`<!-- install-primary: tarball -->` at `docs/install.md:222`, a claim about the
distribution **transport** and not about this member; and its `couples=scripts/*.sh`
does not even retrigger on a `gate-sdk/bin/` edit.

### (11) The deleting commit is one unit, and its template is one commit old

The whole fan-out lands together {mechanical}: the settings grant, every README and
SPEC citation, every `docs/` mirror, the three fenced invocations, the hook template
and the regenerated hooks, the two installer strings, the crate module and the
binary. The template is `--install-lifecycle`'s own deleting commit, one iteration
back — twenty files, settings through smoke through the hook's baked measured value,
in one motion — and this cut differs from it in exactly one respect, the settings
grant that port did not have to remove.

## Producers and consumers

**New interface: the `--install-hooks` arm.**
*Producer* — `gate-sdk/bin/run-gates.sh`'s new `case` arm, resolving the two declared
knobs through `gate_knob_env` and `exec`ing the binary (`run-gates.sh:114-128`). Its
enabling config is not test-only: `GATE_SDK_HOOKS_DIR` resolves to
`scripts/git-hooks` in this tree today and is baked into the generated hook twice.
*Consumers* — a session opting a fresh clone in, which is the member's only caller
and always has been (§install-hooks rules it a one-time per-clone opt-in); the
installer's `init`, which **prints** the command for a human to run and never invokes
it (`installer/lib/init.sh:414`); and `CLAUDE.md`'s per-clone opt-in line. Each reads
the arm's **exit status** and its printed receipt; nothing machine-reads the receipt.

**New interface: the arm's registry resolution of `check-identity`.**
*Producer* — the arm itself, at opt-in time, building the check-dir roster from
`GATE_SDK_GATES_DIR` and the transported kit roots and calling `registry::resolve`.
*Consumer* — the resolved member, reached by one of delta (4)'s three mechanisms: an
in-process call for a `.gate`, a spawn for a consumer `.sh` shadow, or nothing at all
where the member resolves nowhere. The value it produces is a **status**, and its one
reader is the arm's own exit code, which §install-hooks rules surfaces through to the
caller.

**New state: none.** The member writes two git-config keys and file modes, all of
which it wrote before, and no new file, record format or field.

**Each reader's RED condition, not merely its subject** — binding because delta (8)
*narrows* a corpus (one `.sh` file and one settings grant leave the tree):

- `check-settings-paths` — reds on a `Bash(<command>)` grant naming a `.sh` path that
  does not resolve. Its red condition is a **count of unresolvable grant paths**, so
  the narrowing *adds* a violation rather than removing one. Not clearable by
  inspection; discharged by deleting line 26 in the same commit.
- `check-docs-cmd` — reds on a fenced invoked repo-relative `.sh` path that does not
  resolve. Also a violation count that the narrowing *raises*, at three sites.
  Discharged by the three edits.
- `check-graph` — reds on a byte difference between the committed hooks and the
  emitter's output. A **byte equality**, not monotone; both sides move and the gate is
  re-run.
- `check-docs-mirror-fresh` — reds on a kit surface diverging from its `docs/` mirror.
  Pairwise equality, not monotone; re-run.
- `check-hook-exec-bit` — reds on a committed hook whose index mode lacks the exec
  bit. Its subject is the **committed** mode, which this member's per-clone `chmod`
  cannot reach in either substrate (§check-hook-exec-bit), so the narrowing does not
  reach it. Safe to clear by inspection, and cleared.
- `check-identity` — reds on an identity/remote mismatch. Unchanged by this cut in
  substance; but delta (4) changes **how** it is reached, so the comparison in
  delta (9) exercises it at all three resolution branches rather than reasoning about
  it.
- `check-comment-tier` — corpus reaches `*.rs`; the four directives move into it, so
  it is re-run rather than inspected.
- `check-measured-claim` — reds on a **value disagreement** for `tree-shell-owed`, so
  the narrowing reds it by construction and delta (10) regenerates.
- `check-install-claim` — reds on a **zero count** of install-primary declarations,
  which is exactly the non-monotone shape a narrowing can flip. Probed and cleared:
  its singleton is about the distribution transport and its couples set does not
  reach this file (delta 10).

## Existing sections updated

- **`gate-sdk/SPEC.md §install-hooks`** — rewritten as the arm's section: the
  spelling, the family choice, the declared roster, the registry-resolution ruling
  and its three branches, the retained steps, and the no-positional argv reading
  (deltas 2, 3, 4, 5, 6). The heading stays.
- **`gate-sdk/SPEC.md §The non-gate arm`** — the class roster gains
  `--install-hooks`, recorded as the first member that **resolves another member
  through the registry**, and as the instance where an in-process call by name was
  refused to keep a consumer shadow winning (deltas 2, 4).
- **`gate-sdk/SPEC.md §check-identity`** — its sentence that `install-hooks.sh` runs
  the gate at opt-in is re-spelled to the arm, and the "reaches the gate through
  `gate_command` rather than by interpreting the resolved declaration path" clause is
  replaced by delta (4)'s three-branch resolution, with the attested
  descriptor-interprets-to-success failure kept as the ground rather than dropped
  with the mechanism (delta 4).
- **`gate-sdk/SPEC.md §check-hook-exec-bit`** — the sentence naming this member's
  per-clone `chmod` re-spelled; its stated limit is unchanged (delta 5).
- **`gate-sdk/SPEC.md §templates/gates-workflow.yml`** — the "a clone that never ran
  `install-hooks.sh`" clause re-spelled (delta 8).
- **`gate-sdk/README.md`** — the fenced invocation at `:78` and the prose at `:28`
  (delta 8).
- **`gate-sdk/bin/gen-pre-commit.sh`** — the template string at `:158` and `:242`,
  and the regenerated hooks with it (deltas 8, 10).
- **`gate-sdk/bin/run-gates.sh`** — the new `case` arm, its usage line and its help
  text (delta 2).
- **`CLAUDE.md`** — the per-clone opt-in line at `:61` (delta 8).
- **`README.md`** — the mention at `:144` (delta 8).
- **`lifecycle-kit/README.md:93`** — the mention (delta 8).
- **`docs/install.md`** — the fenced invocation at `:238` and the prose at `:367`
  and `:426` (delta 8).
- **`docs/gate-sdk/index.md:31`** — the fenced invocation (delta 8).
- **`installer/README.md`** — `:183` and the `uninstall` narrative at `:930-935`;
  **`installer/lib/init.sh:414`** — the printed follow-up command (delta 7).
- **`.claude/settings.json`** — line 26 deleted in the deleting commit (delta 8).
- **`.github/ISSUE_TEMPLATE/install-failure.yml:32`** and
  **`gate-sdk/templates/gates-workflow.yml:3`** — the two non-markdown mentions
  (delta 8).
- **The generated projections** — `scripts/git-hooks/pre-commit`, `commit-msg`,
  `docs/check-graph.html`, the `docs/` SPEC and README mirrors, and the gate binary
  (delta 10).
<!-- update-target-exempt: TASK-QUEUE.md's entry body is outside the governed manifest set and its census note is build's filing obligation under delta (6), not a spec surface this amendment edits -->
- **`TASK-QUEUE.md` `bin-tool-help-arm-absent-tree-wide`** — named as owed a census
  note at build, not edited here.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; the resolution's one produced value,
      a status, has a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section, not appended; §check-identity's attested
      descriptor-interprets-to-success ground survives the mechanism it justified.
- [ ] **Amendment deleted** — this file removed on merge; none remain for gate-sdk,
      which this component can satisfy at its own commit because no sibling amendment
      is in flight for it.
- [ ] **Both-substrates comparison bought before the delete** — delta (9)'s
      procedure run in the deleting session, in a scratch clone, including all three
      of delta (4)'s resolution branches.
- [ ] **Removals propagated** — every surface in delta (8) edited, the settings grant
      deleted in the same commit as the file, both installer strings re-spelled, and
      every spec grepped for names this change retired.
- [ ] **Gaps filed** — the installer's uncovered printed-command surface (delta 7)
      and the unreproducible help-arm census derivation (delta 6) filed to the gap
      inbox; a build-time causal gap is resolved that session, not deferred.
