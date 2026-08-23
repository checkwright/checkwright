# SPEC amendment: shell-gate-tail-port

The port's last gates leave shell. Six registered members and two kit-shipped
members this tree never registers become `.gate` descriptors dispatching to the
multi-call binary; five of them are **wrappers** that spawn an external program
and refuse at exit 2 when it is absent, exactly as their shell forms do today.

**This unit is the port tail's second, operator-ruled 2026-08-23**
(TRAJECTORY.md §PRIORITY DIRECTIVE — the port track's sequence; §The closed
rulings). That ruling retired born-native exception class (a) and the
criterion-7 "sub-project" hold, and it supplied the *verdict* — every remaining
gate ports, as a wrapper where the program is the rule's own content. What it did
not supply is the **mechanism**: how a compiled member states the external
program it spawns, so §port-blockers and a reader of the payload learn it without
a shell rule to tokenize. That mechanism is delta 1, and the design it lands is
not this amendment's invention — §The `# graph:` manifest already refused the
cheap spelling and named the expensive one, sequenced against the first port that
would need it. This is that port.

**The oracle is read at this stage rather than inherited.**
`bash gate-sdk/bin/port-blockers.sh --group` reports **106 members scanned, 6
groups formed, 0 undecidable, 100 already ported and excluded, 0 permanently
shell, 0 temporarily held; 6 still owed, 6 takeable at this cut**. Every group
holds exactly **one** member, so no two owed members share a corpus derivation
and criterion 6 offers no multi-member composition at all — the budget arm is the
only composer left, which is what §The first cohort, and the rule that selects
the next already says of the size arm's exhaustion, now true of the grouping arm
too.

**And the default arm reports something sharper than the six.** Its trailer reads
**106 member(s) scanned, 102 with a requirement this report could not decide** —
every one of the 100 ported members printing
`? (binary substrate; no --needs)`. The criterion-7 roster is blind over 96% of
the corpus it scans. So delta 1 is not bookkeeping for eight members: it is what
restores an oracle the port itself blinded, one member at a time, exactly as
§The port-candidate criteria predicted when it wrote that "every port moves a
member into it until the binary answers `--needs`".

**Four premises this stage carried in are corrected against the tree, and each
correction is load-bearing rather than tidy.**

- `check-crate-arms` requires **`cargo` and `rustc`**, two rows at
  `check-crate-arms.sh:28` and `:42`. Criterion 7's prose names cargo alone
  (§The port-candidate criteria), and a wrapper declaring one of the two would
  refuse on a machine missing the other with no declaration to explain it.
- `check-docs-render-fidelity`'s requirement is **knob-derived** — two `?` rows
  for command-position `$SITE_KIT_RENDERER_BATCH` and `$SITE_KIT_RENDERER` — so
  the member's port spans **site-kit**, and the component set of this amendment
  is gate-sdk, site-kit, canon-kit and evidence-kit rather than the three the
  unit was scoped against.
- **`check-producer-liveness` carries an external program too, and the unit set
  was scoped without it.** `ek_pid_alive` tries the `kill -0` builtin and falls
  back to **`ps -p`** (`evidence-kit/lib/evidence.sh:117-122`), and `ps` is not on
  `GATE_SDK_PROGRAM_FLOOR`. So the tail holds **five** wrappers, not four. The
  member was invisible to the oracle on two independent counts — it is
  unregistered, *and* its spawn sits in a shared library rather than the gate's
  own text — and the second count is a finding
  `port-oracle-corpus-narrower-than-the-directive` inherits.
- `check-shellcheck` **does not retire.** The queue entry says it retires "with
  the last `.sh`"; that is true of *this tree's registration* and false of the
  gate. It is `install: zero-config`, an adopter cannot author a compiled gate at
  all (§The port-candidate criteria, the default's domain), so a vendoring
  consumer's shell battery is precisely the corpus it exists to lint. Delta 5
  states the distinction rather than letting a later reader delete a live kit
  gate on the entry's word.

**What this unit is not.** It does not port `gate-sdk/bin/gen-pre-commit.sh`,
so `graph-port-bash-spawn-residue`'s spawn survives it untouched — that unit's
disposition is its own and this amendment neither takes nor pre-empts it. It
does not narrow `docs/install.md`'s `awk (GNU)` requirement, which is
`interpreter-floor-gawk-residue-empty`'s and operator-class at the user-facing
edge; delta 8 only retires that floor's **last live holder** and says so, which
is the unblocking event that entry is waiting on. And it does not retire
`run-gates.sh`'s binary-less dispatch loop, which
`binary-less-dispatch-loop-retirement` owns and which delta 10 leaves standing on
its 2026-08-23 ruling.

## What changes

### (1) `--needs`: a declaration path's substrate answers what it requires

The binary gains a top-level `--needs <name>` arm beside `--reads`, backed by a
new element on the crate's registry tuple and held to executed behavior by a
crate unit test in the `--reads` one's shape. **Design-bearing.**

**The design is inherited, not chosen.** §The `# graph:` manifest refuses a
`# needs:` descriptor field on the ground it refuses `# reads:` — a
self-declaration in a data file has no reader that could verify it — and names
the replacement in the same breath: the requirement set lives in the crate's
registry, "where a unit test runs the member and compares the declaration against
what it actually did". The arm was declared **sequenced, not shipped**, with the
condition for building it stated exactly: "The first port of a member carrying an
external requirement builds it." Four of this unit's members carry one, so the
condition fires here and the amendment builds the arm rather than re-opening the
refusal.

**Not "the fifth flag" — and not "a fifth registry-tuple element" either, the
same staleness twice in one paragraph.** That section's ordinals are both
stale, verified against the tree rather than the sentence. `main.rs` already
resolves `--source-stamp`, `--list`, `--queue-parity`, `--declaration-parity`,
`--reads` and `--knobs` before the registry lookup, beside the bridged arms —
six top-level arms, not four. And `GateEntry` (`native/src/gates/mod.rs:116-122`)
is already a five-element tuple — name, `GateFn`, reads, knobs, declaring root,
the last two added after the manifest section's prose was written and never
back-read against it — so `--needs`'s element is the tuple's **sixth**, not its
fifth. The merge drops both counts rather than incrementing either, per
de-literalization: a number in prose over a churning roster is a second source
for something the source already holds.

**The report grammar: one line per requirement, and nothing else** — no count
line and no header, on §check-reads-couples' stated ground that a transcribed
total is a second source for something derivable from the lines. Three line
kinds, each with a named reader at a named transition:

- **`<program>`** — a program name the member spawns unconditionally. Read by
  §port-blockers' default arm at its per-member row, filtered against
  `GATE_SDK_PROGRAM_FLOOR` exactly as a shell member's scanned command word is,
  so an on-floor program is suppressed on both substrates by one rule.
- **`?<TAB><knob-name>`** — the member's requirement is the **command word of
  that knob's resolved value**. Read by the same arm at the same transition,
  which resolves the knob through `lib/gate.sh`'s bridge resolver — the path it
  already uses for a shell member's command-position expansion, so the two
  substrates cannot disagree about a knob's value.
- **`?`** — a requirement the registry cannot bound at all. Read by the trailer's
  undecidable counter, the reader a shell member's unresolvable expansion already
  has.

**The `?` in the knob form is deliberate reuse and not an inconsistency with
`--reads`, which spells its optional field the other way round.** There a root is
an answer and the tab field refines it; here `?` says *the registry cannot name
this literal* — which is exactly what `?` already means on both arms — and the tab
field says *where the answer is instead*. A consumer-configurable program is
genuinely unbounded in the registry, so the marker is true rather than borrowed,
and a reader who takes the tab field away is left with the honest `?` the arm
would otherwise print. Stated because the two arms' grammars are read side by
side and the asymmetry would otherwise look like drift.

**The knob form is the load-bearing one and it is not an ergonomic.** Criterion 7
already rules that a gate's requirement "need not be spelled in its source at
all", names `check-docs-render-fidelity` as the instance, and concludes that **no
literal roster is true for every consumer**. Spelling `ruby` into the crate's
registry would be a second copy of `SITE_KIT_RENDERER`'s default, which
de-literalization forbids, and it would be *wrong* for any consumer who repointed
the knob — silently, since nothing would compare the two. Carrying the knob name
keeps the value single-sourced and reuses the resolution path the bridge owns.
This is the same ruling §check-reads-couples took for a run-time-named filter
pattern, taken again on the same grounds for a run-time-named program; the
symmetry is deliberate, so the two arms' line grammars are read together.

**Two crate-side tests hold the declaration to behavior, and one of them already
exists — verified against the crate rather than assumed.** Test **A**: every
registry member is run over its own `gate-tests/<name>/{good,bad}/` cases with
spawn recording on, and the observed program set must be a subset of the declared
one. Test **B**: no module outside the crate's single sanctioned spawn wrapper
constructs a `Command` — which is **already true and already machine-held** by
`no_gate_module_constructs_a_subprocess_itself` (`native/src/proc.rs:196-220`),
which scans every `native/src/gates/*.rs` for `Command::` and fails the suite on a
hit. That is §Meta-gate conservation's `check-gate-fail-closed` row landed as
code. So the recorder hooks `proc::run` / `proc::run_with_stdin`
(`native/src/proc.rs:36,52`), the way `--reads`' recorder hooks the single
sanctioned walk implementation, and B is a re-assertion rather than new work. A
member whose registry entry omits the element fails to compile, on `--reads`'
terms.

**The subset direction is the honest one and is stated so a later reader does not
tighten it.** A's assertion is *observed ⊆ declared*, never equality: a wrapper
whose program is reached only on a branch no fixture case takes would fail an
equality test for being correctly declared. The failure direction that matters is
an **undeclared** spawn, which is what a subset test catches, and it is the same
direction §The port-candidate criteria fixes for an undeclared hold — over-count
rather than lose.

### (2) The wrapper contract: spawn, and refuse at exit 2 when the program is absent

A compiled member whose rule's content is an external program spawns it through
`proc::run` and **refuses at exit 2 when it is absent**, reproducing its shell
form's refusal *message* rather than inheriting the wrapper's generic one.
**Design-bearing.**

**This is the operator ruling's own words made executable** (§The port-candidate
criteria, criterion 7, class *the program is the rule*): "such a member ports as
a wrapper that spawns the program, and the program stays a declared dependency —
refusing at exit 2 when absent, exactly as the shell form does today." Two
consequences the ruling states and this contract holds: the dependency floor is
**not widened**, because a consumer without the program gets the refusal it gets
today; and the substrate of the wrapper does not move the floor, which is the
reading the eleven-day hold got wrong.

**Half of this is already built, and naming which half is what keeps the delta
honest.** `proc::run` already fails closed on an absent program — a spawn failure
becomes `Err("cannot run <program>: … — the check could not run; treating as
failure (not clean)")`, propagated to the crate's exit-2 path — and
`Completed::stdout()` already withholds output unless the status succeeded, so
the captured-emptiness false green is unreachable. **What is not built is message
parity.** Each shell form guards with `command -v <prog>` and prints its own
remedy; the wrapper's generic string names the program and not what to do about
it. A refusal message is a documented surface, and a session debugging its PATH
reads the message rather than the exit code.

**Corrected at build, 2026-08-23 — "what is not built is message parity" names
one of three gaps, and the other two are mechanism rather than text.** Landing the
first wrapper (delta 5) found `proc::run` unable to serve a wrapper at all, for
the very property this paragraph praises: `Completed::stdout()` withholds output
unless the status succeeded, and for a linter the **non-zero** run is the one
whose report must be printed. And `run` captures the two streams separately where
every wrapper's shell form takes a `2>&1` merge. Two faces were added to
`proc.rs` — a PATH presence probe and a merged-stream capture — and the presence
probe carries the third gap: the refusal must fire **where the shell form fired
it**, before the target derivation, or a tree with nothing to lint and no linter
reports the empty corpus instead of the missing program. All three are recorded in
§Fail-closed contract as the wrapper contract for the class, so deltas 6 and 7
inherit them rather than re-deriving them.

**The refusal is §Fail-closed contract's exit 2 and not a red.** *Cannot verify*
and *verified clean* must not share an exit code, and an absent linter is the
first case: a member that printed `clean` because shellcheck was missing is the
vacuity the whole contract exists to close.

**Parity is proved, not asserted, and the pair alone cannot prove it.** The
absent-program branch has no static representation: a committed fixture cannot
remove a program from PATH. So each wrapper's parity run carries a **constructed
scenario** on §The port-candidate criteria criterion 2's own terms — both
implementations run over the same cases with the program present, and again with
PATH scrubbed of it, comparing bytes and exit codes. That is the `# no-fixture:`
discharge shape applied to a member that *has* a pair, which criterion 2's
second worked instance already licenses: where a pair can reach the derivation it
does, and the scenario covers only the residue.

### (3) `check-install-disposition` ports first, and criterion 4 binds on it

The cheapest member — `c7=clean`, `lines=92`, no external program — becomes
`gate-sdk/checks/check-install-disposition.gate` plus a crate module, the shell
original deleted. **Design-bearing.**

**Criterion 4 binds and is discharged by widening the pair first.** Its
`couples=` covers `kit:checks/check-*.sh` **and** `kit:checks/check-*.gate`, and
its walk reads both spellings as content — it is the gate that asserts every
shipped gate declares a disposition, so every registry member's declaration path
is inside the corpus it scans. The widening the criterion prescribes is the
general one: the pair must carry **every arm of the derivation**, which for this
member means a `.sh`-declared case, a `.gate`-declared case, a mixed kit, and the
`installer/lib/common/recipe.sh` assertion-C arm whose file is *absent* in a
vendored consumer and whose absence is a skip. Widen first, then port.

**Its own port moves the corpus it scans, which is why the live-tree arm is smoke
rather than proof.** Assertion A forbids a descriptor and a script coexisting in
one resolve dir, so the cross-substrate comparison necessarily runs on the
pre-descriptor tree — a corpus this port then changes. The verdict is recorded as
**no disagreement found on the pre-descriptor tree**, never as *parity proved*,
on the demotion §The port-candidate criteria records for exactly this shape.

**Corrected at build, 2026-08-23 — the demotion above did not have to be taken,
and the executed verdict is the undemoted one.** Assertion A binds a *resolving*
name. Restoring the pre-port rule inside the resolve dir under a name outside the
`check-*` glob — so no registry member resolves to it — puts both implementations
over the **post**-descriptor corpus, the one the port actually produces. That is
what ran: byte-identical stdout, stderr and exit code across the live tree, both
fixture cases, and the two fail-closed arms no committed fixture can carry
(absent root; no positional root outside a checkout). The correction is recorded
in §The port-candidate criteria, criterion 4 with its bound — a port that also
moves a *sibling* member's corpus keeps the demotion — so a later delta reads the
true premise rather than re-deriving it.

**Also corrected: "no external program" is true of the *floor* and not of the
member.** `check-install-disposition.sh:17` runs `git rev-parse --show-toplevel`
when no positional root is given; `git` is on `GATE_SDK_PROGRAM_FLOOR`, which is
why `c7` reads `clean`. The compiled form keeps that spawn through
`fresh::toplevel()` and **declares `git`** in its registry tuple, which delta 1's
unit test A forces. Unlike delta 4's `gate_authoring_tree` case this one is
visible in the member's own text, so it is not a third instance of the
shared-library blind spot; the blind spot was checked here and is empty
(`gate_kit_roots` and `fail_closed` spawn nothing).

### (4) `check-gate-substrate-parity` ports, and ends its own `c7=?`

The auditor of the dispatch relation — `lines=459`, the largest member in the
tail — becomes a descriptor and a crate module. **Design-bearing**, and this is
the delta the retired exception class (a) was standing on.

**Its `?` is itself, and the fork that creates is ruled here rather than left to
build.** The default arm reports `check-gate-substrate-parity.sh:223
(command-position $BIN, default unresolvable)`: the program at its unresolvable
command position is the **gate binary**, spawned as `"$BIN" --list` to read the
subcommand roster assertion B compares against the descriptor set.

**Ruled: the compiled form reaches the registry in process, and the spawn is
deleted.** That is the shape §check-reads-couples and §check-gate-binary-fresh
both already took — the fourth budget batch for the latter, the sixth for the
former — so the row is *gone* rather than *answered*, and the member declares no
requirement at all.

**The alternative was weighed and is refused with cause, because it is the
reading a session arrives at first.** *Keep the spawn, because the gate audits the
binary and an auditor that asks itself proves nothing.* That is class (a)'s own
argument, and the operator retired it on 2026-08-23 with the refutation recorded
at §Meta-gate conservation: the shell auditor already trusts the binary's
`--list`, so the spawn never bought independence in the first place; an absent
binary is exit 2 under the fail-closed contract; and a **stale** binary — the only
state where in-process and spawned answers could differ — is
§check-gate-binary-fresh's red and not this member's. Keeping the spawn would
re-import a retired class's reasoning through an implementation choice, and it
would keep a `?` in a report this unit exists partly to un-blind. Recorded rather
than taken silently, because a reader finding the old `?` and no adjudication
would conclude the remainder was ignored.

**The class-(a) refutation is discharged here rather than restated.** The class
held that a compiled auditor could pass *itself* with a broken binary. The
grounds for retiring it are already recorded at §Meta-gate conservation — the
shell auditor already trusts the binary's `--list`, an absent binary is exit 2
under the fail-closed contract, and a stale one is §check-gate-binary-fresh's red
— so the port is the ruling landing, not a fresh argument. What the port owes is
narrower and concrete: assertions B, G and H all read declaration paths, and this
unit changes eight of them, so the member's own fixture pair must carry the
post-port shapes before the port and not after.

**Assertion G's corpus narrows, and that narrowing is not monotone.** G reads
`# no-port:` and `# port-until:` on `<name>.sh` declaration paths; after this
unit no such path remains in this tree. G's verdict on an empty corpus must be
**green-with-a-counted-zero**, never a red for finding none, and the pair must
carry the empty case explicitly — a reader that reds on finding none is one of
the three non-monotone shapes §The causal-completeness check point 5 names, and
this delta is exactly a corpus narrowing.

### (5) `check-shellcheck` ports as a wrapper — and does not retire

`lines=56`, `c7=shellcheck`, `install: zero-config`. It becomes a descriptor and
a crate module that spawns `shellcheck` and refuses at exit 2 when it is absent.
**Design-bearing**, because the ruling it lands is a refusal rather than a port.

**The retirement claim is corrected, and its live holder is §check-shellcheck's
own text rather than the queue.** `TASK-QUEUE.md`'s entry already reads
correctly — "DEREGISTERING here with the last `.sh` and not retiring" — so it is
not this claim's source; that reading was verified against the tree rather than
assumed. The claim that needs correcting sits in the canonical section this
delta edits: §check-shellcheck itself currently says the port leaves "the
gate's horizon" unchanged, "so it retires with the last `.sh` rather than
outliving it" — and §Meta-gate conservation's table row says "Retired with
cause — no shell exists to lint". Those are two different subjects and only one
of them is true. The **table row** is a per-member disposition: for a
`.gate`-dispatched member there is no shell file to lint, so the meta-gate
makes no assertion about it. The **gate** is `zero-config` and ships to every
adopter, and an adopter *cannot author a compiled gate* — `native/` ships no
`checks/` and no `smoke/`, so `gate_kit_roots` never selects it and `init`
never vendors it (§The port-candidate criteria, the default's domain). A
vendoring consumer's gate family is shell by construction, which is the corpus
this gate exists for.

**So what ends is this tree's registration, not the kit's gate**, and the two are
recorded as separate facts. When this tree's last `.sh` leaves, the member's
corpus here is empty and `scripts/gates.list` drops it; the kit keeps shipping it,
seeded by `init`, doing exactly the job it does today on a tree that has shell.
Deleting the gate on that stale phrasing would remove an adopter's self-lint
floor to tidy a tree it does not describe.

**Also corrected at build, 2026-08-23 — the knob is a scalar feeding an array and
had no bridgeable name.** `GATE_SDK_LINT_EXTRA_DIRS` is set only in a consumer's
config file, so `declare -p` finds it in *this* tree and would meet the bridge's
undeclared-knob refusal in any consumer that never sets it — F11's shape a third
time. It is resolved in `gate-sdk/lib/gate.sh` onto `GATE_LINT_EXTRA_DIRS`, the
distinct-array-name rule the library already applies to `GATE_PRUNE_DIRS` and
`GATE_EXEC_GLOBS`, and the resolution word-splits without pathname-expanding —
recorded in §check-shellcheck rather than left as a silent narrowing.

**Its own corpus narrows as this unit runs**, which puts it under point 5 with
delta 4: the gate's red condition is *a `.sh` under a resolve dir that fails
shellcheck at -S warning*, which is monotone in the violation set, so shrinking
the corpus can only remove findings. Stated rather than assumed, because
"a narrower corpus can only remove violations" is the first argument a narrowing
delta reaches for and it is false in general.

### (6) `check-action-run-shell` ports as a wrapper

`lines=222`, `c7=shellcheck`, `install: on-surface` — the YAML `run:` extractor,
the dialect resolution and the per-block shellcheck invocation move to a crate
module behind a descriptor. **Design-bearing.**

**The extractor is the port's real content, not the spawn.** Its two in-file
`# spec:` directives rule that the extractor stays inline (a `lib/` helper earns
its place at a second consumer and there is none) and that an absent `shell:` key
resolves to bash on GitHub's documented runner default while a dialect shellcheck
has no theory of is **skipped and counted, never linted as shell**. Both are
locality-class directives binding to a line of implementation, so both follow the
code into the module (§The `# graph:` manifest, the annotation partition) rather
than into the descriptor.

**The skipped-and-counted branch is the one a port silently loses.** It is a
count printed on the clean line, so a compiled form that simply declined to lint
an unknown dialect would pass every fixture case and print a different clean line
— the divergence class the parity run exists to catch. The pair must carry an
unknown-dialect block before the port, not after.

### (7) `check-crate-arms` ports as a wrapper, declaring both programs

`lines=78`, `c7=cargo,rustc`, `install: never`. It becomes a descriptor and a
module spawning both, carrying its source-stamp cache unchanged. **Design-bearing.**

**Both programs are declared, which is the correction this delta lands.**
Criterion 7's prose names `cargo` alone; the tree reports two rows, `cargo` at
`:28` and `rustc` at `:42`. A wrapper declaring one would refuse on a machine
carrying cargo without rustc with nothing in `--needs` to explain it, and test A
would not catch it, because A asserts *observed ⊆ declared* and an undeclared
program is exactly what that direction is for.

**Its permanence ruling is retired here rather than re-argued.** The member was
held permanently on the reading that a gate running `cargo test` over the crate
cannot live inside the crate it tests; §The port-candidate criteria already
records the refutation — cargo compiles and runs the crate's **source** afresh in
a target directory the binary never reads back, so a stale or broken installed
binary spawning it asserts nothing about itself. This delta lands that, and the
`install: never` disposition is what makes the whole question adopter-invisible.

**The cache is carried, not redesigned.** `check-crate-arms` stays whole and
caches, ruled 2026-08-23 (TRAJECTORY.md §The closed rulings), and its
source-stamp mechanics are §check-crate-arms'. The port relocates them; a port
that quietly dropped the cache would put a full `cargo test` on every commit,
which is the cost that ruling was taken to avoid.

**One hazard is pinned because the natural port loses it.** The shell form runs
**both arms even when the first fails**, so one red does not hide the other
(`check-crate-arms.sh:51`'s directive). `Command::status()?` short-circuits by
construction, so a compiled form written the obvious way reports clippy's failure
and never runs the tests.

**CORRECTION (build, 2026-08-23) — the refusal this delta predicts for an absent
`rustc` does not exist, and the declaration stands on the other ground stated
above.** *"A wrapper declaring one would refuse on a machine carrying cargo
without rustc"* is false against the tree: `rustc` is reached at exactly one site,
inside the cache-key composition
`key="$stamp $(rustc --version 2>/dev/null) $(cargo --version 2>/dev/null)"`. That
program's stderr is discarded and the substitution's emptiness is never tested, so
an absent `rustc` yields a key with an empty middle field — a cache *miss* against
any key written while it was present, never a refusal. A faithful port must not
mint one, and the compiled form does not. What actually forces both declarations
is the sentence after it: test A asserts *observed ⊆ declared*, and `rustc` is
observed. Ported on that ground.

**CORRECTION (build, 2026-08-23) — three programs are declared, not two.** `c7`
counts the **off-floor** pair, which is what `port-blockers.sh` reports; the member
also spawns `git`, twice in its own text and three more times through
`gate_native_source_stamp`, the shared-library reach the report cannot see. `git`
is on `GATE_SDK_PROGRAM_FLOOR`, so declaring it costs no criterion-7 residual — but
test A observes it, so `--needs` carries it. The same shape delta 4's `git`
correction records.

**CORRECTION (build, 2026-08-23) — the port needed one knob repair the delta does
not name.** `GATE_SDK_CARGO_TARGET_DIR` was defaulted inline at the member's read
site, where the config bridge's `declare -p` cannot see it, and no config in this
tree sets it — so the bridge's undeclared-knob refusal would have fired here rather
than only in a consumer. Resolved in `gate-sdk/lib/gate.sh` off the already-normalized
`GATE_SDK_NATIVE_CRATE`, which is the derivation §Layout and configuration had
already described. The fourth instance of the pattern in this unit, after deltas 4
and 5.

### (8) `check-docs-render-fidelity` ports, and the GNU-awk floor loses its last holder

`lines=241`, `install: on-surface`, requirement knob-derived. It becomes a
site-kit descriptor and a crate module whose `--needs` answers
`?<TAB>SITE_KIT_RENDERER_BATCH` and `?<TAB>SITE_KIT_RENDERER`. **Design-bearing.**

**This member is why delta 1 carries the knob line kind**, and the reasoning is
already ruled: the requirement "is the first element of `SITE_KIT_RENDERER`'s
default", and "because that knob is consumer config, a consumer who repoints it
changes which external program that gate requires" (§The port-candidate criteria,
criterion 7).

**Its batch knob's conditional default rides with it and must not be flattened.**
`SITE_KIT_RENDERER_BATCH` is site-kit's one conditional default — filling it for
a consumer who pinned `SITE_KIT_RENDERER` "would replace their pinned oracle with
this unpinned one and report clean against a parser build they rejected"
(`check-docs-render-fidelity.sh:35`). The bridge transports resolved values and
interprets nothing (§The port-candidate criteria, criterion 6), so the
conditional stays in `site-kit/lib/site.sh` and the compiled member receives
whatever it resolved to — including empty, which is the batch-absent branch.

**The GNU-awk floor's last live holder leaves.** Four in-file directives
(`:74`, `:154`, `:200`) rest on `BEGINFILE`, `ENDFILE` and `ARGIND` — gawk
extensions in live program text — and this is the **only** remaining registered
shell member that holds them, which
`interpreter-floor-gawk-residue-empty` established by measurement after an
earlier probe's conclusion outran its two-member corpus. This delta retires the
holder and **states that it does**; it does **not** narrow `docs/install.md`
§Requirements, whose edge is operator-class and whose decision that entry owns.
The unblocking fact is recorded here so that entry's taker reads a discharged
precondition rather than re-deriving it.

**Correction taken at the build (1) — the requirement's stated ground is false,
and the conclusion survives on the ground stated beside it.** This delta says the
requirement "is the first element of `SITE_KIT_RENDERER`'s default", quoting
§The port-candidate criteria criterion 7, which says the same. Against the tree
`port-blockers.sh` resolves it off **`SITE_KIT_RENDERER_BATCH`** at `:32`: with
the batch knob non-empty the gate takes the batch branch and never invokes the
per-document renderer, and `lib/site.sh` fills the batch knob at zero config. The
claim is accidentally true of the *program* only because both defaults begin
`ruby`; it is flatly false for a consumer who pins only the batch knob, whose gate
then requires that command and never `SITE_KIT_RENDERER`'s. The conclusion this
delta draws is unchanged and already rests on the right ground — it declares
**both** knobs, which is what a two-knob requirement needs. Corrected in
site-kit/SPEC.md §check-docs-render-fidelity and in criterion 7.

**Correction taken at the build (2) — the fifth no-bridgeable-name candidate is
not one.** `SITE_KIT_RENDERER_BATCH` was flagged as the next F11-shaped knob
repair after four consecutive hits. It is not: `site-kit/lib/site.sh` defines both
renderer knobs *and* `SITE_KIT_DOCS_DIR` under their own names, so the bridge's
`declare -p` finds each, and the batch knob's empty-array branch crosses as a
resolved-empty value the wire format already distinguishes from an absent one. No
knob repair is owed by this delta. Recorded because a fourth consecutive hit reads
as a rule, and the fifth check is what keeps it a measurement.

**Correction taken at the build (3) — the port needs one `proc.rs` face this
amendment does not anticipate, and the cause is measured.** The batch path streams
the whole docs corpus through the renderer. `proc::run_with_stdin` pipes both
directions and writes its whole input before reading, so the child fills the
stdout pipe, stops draining stdin, and both sides block — 73 pages and 2.1 MB
against a 64 KiB buffer. The shell form never meets it because a process
substitution is a concurrent reader, which §check-docs-render-fidelity already
states in terms. The face is file-backed on both ends with stderr left unmerged,
and it lands in §Fail-closed contract for the class rather than in this member.

### (9) The two unregistered kit-shipped members port, and their price is named

`canon-kit/checks/check-surface-duplication.sh` and
`evidence-kit/checks/check-producer-liveness.sh` become descriptors and crate
modules though this tree registers neither. **Design-bearing.**

**`port-blockers.sh` never counted them and never will**, because its walk is
`gates.list`. That is the whole of
`port-oracle-corpus-narrower-than-the-directive`'s thesis landing on this unit:
the port's corpus is the **kit-shipped** gate set, whether or not the authoring
tree registers a member, and an oracle that scans a registry cannot see the
difference. This unit ports them on the directive's scope; the *measurement* is
the sibling amendment's.

**Criteria 1 and 3 both genuinely fail, and neither is a screen.** Criterion 1
(registered) fails by construction — an unregistered gate proves no dispatch here.
Criterion 3 fails on its own terms rather than by the `commit-msg` proxy: both are
`tier=align-only`, which "emits into no hook, so the reason has nothing to attach
to and the criterion still names a real cost" (§The port-candidate criteria). So
neither member gets `check-graph`'s end-to-end proof that its manifest survived
the substrate change, and neither gets a dispatch proof from this tree's battery.
**What replaces both is the fixture pair plus the owning kit's smoke**, which is
the only oracle available and is named as such rather than presented as
equivalent.

**Each keeps its own contract exactly.** `check-surface-duplication` is `dir=bi`
with the `vision-introduces:` / `spec-introduces:` valves and an **exit 2 when
the glossary file is absent**, which is a fail-closed refusal a port must
reproduce rather than convert to a skip; it spawns nothing off the floor.
`check-producer-liveness` keeps its single-path mode, its set mode over `*.run`,
its exit-2-wins-over-red-wins-over-green aggregation and its liveness leg.

**`check-producer-liveness` is the tail's fifth wrapper, and nothing had noticed.**
`ek_pid_alive` (`evidence-kit/lib/evidence.sh:117-122`) tries the `kill -0`
builtin and falls back to **`ps -p`**, which is not on
`GATE_SDK_PROGRAM_FLOOR`. The pair depends on that leg: `bad/` names PID 1 because
"under `kill -0` alone, an unprivileged run reads init as dead and the case would
silently invert" (evidence-kit/SPEC.md §check-producer-liveness). So the compiled
form declares `ps` in `--needs` and refuses under delta 2 when it is absent —
and, matching the shell, only on the fallback path, since a `kill -0` that answers
never reaches it.

**Two independent reasons the oracle could never have reported this**, and both
are `port-oracle-corpus-narrower-than-the-directive`'s to carry: the member is
absent from `gates.list`, and the spawn is in a **shared library** rather than the
gate's own declaration text, which §port-blockers' command-position scan does not
follow. The second is the sharper one, because it is true of registered members
too.

**Criterion 6's *unless* clause binds, and the discharge is a comparison rather
than a deletion.** `ek_pid_alive` keeps a live shell caller after the port —
`evidence-kit/bin/run-validate.sh:52`, which asks it whether a lock's holder is
alive — so the port **creates** a dual implementation with consumers on both
sides. That is the road §The port-candidate criteria fixes for `lib/queue.sh` and
for `gate_staged_matches`: the disposition is chosen by *whether the shell caller
set empties*, it does not, so what discharges the criterion is a standing
cross-substrate comparison in the owning kit's fixture lane — one canned corpus of
PID states fed to both holders, verdicts compared byte for byte — and never a
parity proof taken once at port time, which satisfies criterion 2 and expires at
the next edit to either side.

**Its wiring is already port-safe and that is verified, not assumed.**
`check-producer-liveness` is dispatched from `LIFECYCLE_KIT_ENTRY_PREFLIGHT` at
six stage keys, and this repo's `scripts/lifecycle-config.sh:13-17` names it
through the front end `scripts/gate-exec.sh` rather than by a literal
`.sh` path — the form evidence-kit/SPEC.md §lifecycle-kit integration says "would
break at its port". The port therefore changes nothing at the entry hook, and the
kit SPEC's parenthetical stops being a warning and becomes a discharged one.

**Corrected at build, 2026-08-24 — the shell form fires no refusal on an absent
`ps`, so the port's refusal is a deliberate divergence and not parity.** Read off
`ek_pid_alive`'s text: `ps -p "$pid" >/dev/null 2>&1` is the function's *return
value*, so an absent program exits 127, the function returns 1, and the caller
reads **"not alive"** and prints a clean line. "Matching the shell" is true of the
*path restriction* — only the fallback leg reaches the program — and false of the
refusal, which the shell has none of. The conclusion survives on the ground delta
2 states beside it rather than on parity: *cannot verify* and *verified clean* must
not share an exit code, and a member printing clean because the program was
missing is the vacuity the whole contract exists to close. Measured rather than
argued — with `ps` scrubbed off `PATH` the shell form reads the fixture pair's own
`bad/` case, PID 1, as **dead** and exits 0. Two things a later reader is owed and
this paragraph carries: the port's cost (on a `ps`-less machine a lock naming a
dead PID now refuses where it printed clean, because `kill -0` fails with `ESRCH`
and the disambiguator is gone), and the fact that this is the first member of the
class where *fire where the shell form fired it* has no shell refusal to fire
against. Recorded at evidence-kit/SPEC.md §check-producer-liveness and, for the
class, at gate-sdk/SPEC.md §Fail-closed contract.

**Corrected at build, 2026-08-24 — criterion 6's *unless* clause binds on two
helpers, not one.** This delta names `ek_pid_alive` as the dual implementation the
port creates. Enumerating the caller set rather than inheriting the name finds
`ek_lock_read` dual on identical grounds: `evidence-kit/bin/run-validate.sh` calls
it at `:32` and `:48`, beside the `:52` predicate call this delta already cites.
The standing comparison therefore covers **both** readers. The lesson is
procedural and generalises past this member: before taking the machine-held road,
enumerate the shell callers of every helper the ported member touches, because the
disposition turns on whether *that* set empties.

**Corrected at build, 2026-08-24 — the compiled form declares two programs, not
one.** This delta says the port "declares `ps` in `--needs`". It declares `ps`
**and `bash`**, because the first leg is bash's `kill -0` *builtin*, which `std`
has no spelling for and this crate carries no `libc` to reach. Of the three routes
to it, only `proc::run("bash", &["-c", …])` keeps the requirement honest:
`ps -p` alone would make the program required on every call rather than on the
fallback, and `/bin/kill` would mint a second **off-floor** requirement. `bash` is
on `GATE_SDK_PROGRAM_FLOOR`, so the report still counts one program — the same
shape delta 4 recorded for `gate_authoring_tree`'s `git` and delta 7 for
`check-crate-arms`'.

**Lead-ruled at build, 2026-08-24 — this delta empties the *last* `check-*.sh`
in the tree, and `check-gate-fail-closed` refused on the empty corpus it produced.**
That member is `precommit` tier, so the unit could not land at all until it was
dispositioned, and the disposition is a third gate's user-facing semantics this
amendment settles nothing about — escalated rather than taken. **Ruled: green with
a counted zero, with the misconfiguration refusal preserved wherever it is still
meaningful.** The decisive ground is that the SPEC had already blessed the corpus
*shrinking* and only ever anticipated it shrinking, so at zero the refusal fired
because the port had succeeded — a heuristic mistaking its objective being met for
failure. The discriminator chosen at the code, and stated at gate-sdk/SPEC.md
§check-gate-fail-closed rather than here: no shell gate beside a **non-empty
descriptor set** is a finished port and is green; no declaration of **either**
spelling under the same resolved dirs is a tree that resolved no gates directory,
and stays exit 2. Delta 10's remainder accounting inherits a member whose clean
line now carries a descriptor count.

**Provenance corrected at the cut — the ruling above is the lead's, and was first
recorded as the operator's.** The operator was not consulted on it. The delta-9
session escalated correctly, and the lead ruled it under the lead's own authority
rather than relaying it onward. Commit `0f9cc492`'s message carries the same
misattribution and is immutable; it is not rewritten, and **this passage is the
correct record** — the owner doc is ground truth and a commit message answers only
what happened, never what is correct. The label is load-bearing rather than
decorative: *operator-ruled* marks what a later session may not reverse alone
(TRAJECTORY.md, how to read a ruling recorded there), so inflating a lead ruling
to an operator one freezes a decision that should stay re-rulable at the lead, and
it does so silently. The tree already carries an open entry about this failure
class, `relayed-ruling-provenance-unrecorded`; this instance is filed to the gap
inbox against it rather than written into it, since a mid-iteration queue edit
contends on a stage session's surface.

**One thing this delta does not say and a later reader will meet: the port empties
`spec_canonical_specs`' last live shell caller.** `check-surface-duplication.sh`
was it, so after this commit `canon-kit/lib/spec.sh`'s spec-corpus helpers have no
caller outside that library. gate-sdk/SPEC.md §The canonical-spec cohort rested its
criterion-6 discharge on that caller being live; the verdict survives on the *dead
twin* disposition's **undocumented-surface bound** instead, since both helpers are
documented by canon-kit/SPEC.md §lib/spec.sh. Repaired there rather than left to
delta 10, because it is staleness this delta creates and no gate catches it.

**Repaired at the cut — this delta also killed delegation-kit's turn-end liveness
probe, and the unit does not ship a dead probe.**
`DELEGATION_KIT_LIVENESS_CMD` defaulted to the declaration path this delta
deletes, so the readability test it sits behind failed on every firing and the
probe logged `verdict=unavailable` while answering nothing. **Measured, not
inferred**: the probe's own log carries 508 answered firings (471 green, 37 red)
and then an unbroken run of `unavailable` beginning at this delta's commit. That
is the mechanism the wait-primitive discipline rests on, and the detector behind a
standing defect with five recorded firings, so it is the unit's breakage to repair
rather than a bullet to leave behind. It was first filed as a knob-contract and
provenance-seam question on the premise that *every* repair turns the knob's value
from a path into a name-resolving command. **That premise is false, and the tree
had already ruled the case one caller over**: evidence-kit/SPEC.md
§check-evidence-manifest met the identical break when this same port turned a
pre-flight entry's named path into a descriptor, and discharged it with a
**consumer-side front end**, refusing to teach the kit's knob to resolve a name as
"a kit-contract change". Applied here the knob keeps its contract exactly: the
template drops its now-fake default, and this repo names its own two-line reader
reaching the gate by name through the front end its whole pre-flight roster
already uses. The wider filing stays open on what it still carries; the capability
does not wait on it. The oracle that would have caught it lands with the fix, in
the consumer that configures the reader — delegation-kit's own probe test drives a
**stub** reader by design, so it can hold every verdict arm and can never see
whether the configured one resolves.

### (10) Descriptors, registration, conservation rows and the remainder accounting

Eight descriptors, eight registry entries, the derived substrate-sensitive set
re-taken, and every roster that counts the residue re-read from a run.
**Mechanical.**

- Each descriptor carries the three lines that are copied verbatim — `# graph:`,
  `# install:`, `# spec:` — and **neither** `# no-port:` nor `# port-until:`,
  which §The `# graph:` manifest rules a descriptor never carries and
  §check-gate-substrate-parity assertion G reds on.
- The six registered members keep their `gates.list` entries; the two
  unregistered ones gain none, which is the state that made them invisible to
  the oracle and stays so deliberately.
- **The substrate-sensitive set is re-derived at the cut, never inherited.** A
  port *changes* a declaration path — `<name>.sh` becoming `<name>.gate` — which
  can move other members into or out of the derived set, and this unit changes
  eight at once. §Meta-gate conservation requires exactly one recorded
  disposition per derived member and reds one it does not name.
- The residue counts every surface states are re-read from a run rather than
  edited by arithmetic: `port-blockers.sh` both arms, and the
  `ported-gate-members` measured claim.

### (11) Criterion 5's residual is measured on both sides, and the growth is predicted at two

The binary-less leg's omitted-member roster is measured before and after, from a
clean checkout of this unit's own commit. **Mechanical.**

**The prediction is derived from the install dispositions of this unit's own
members, intersected with the measured profile's kits** — the method §The
port-candidate criteria fixes after the fifth budget batch's estimate missed for
skipping the intersection. Of the eight, exactly **two** are `zero-config`:
`check-shellcheck` and `check-install-disposition`, both gate-sdk, which every
profile carries. The other six are `on-surface` (`check-action-run-shell`,
`check-docs-render-fidelity`, `check-gate-substrate-parity`,
`check-surface-duplication`) or `never` (`check-crate-arms`,
`check-producer-liveness`), and no `init` seeds either class, so no artifact-free
`init` can lose them. **Predicted growth: two.**

**The absolute number is measured on both sides or not compared at all** — a
standing quantity, never a per-cohort delta, on the ruling the fourth budget
batch's divergence produced. And the ordering rides with it: the leg packs a
payload stamped with a commit and refuses a dirty worktree, so the measurement
runs after this unit's own commit, from a clean checkout reached **by path**.

**The judgment the criterion leaves to the cut is ruled here: accept and
declare**, on the standing terms and refusing the same two rivals. An adopter on
an uncovered platform loses the two seeded members and receives the omission
declared in their own `gates.list` rather than as a broken battery; restoring the
class shell-side reinstates the duplication this unit deletes, which
enforcement-first ranks below removal; and a binary-gated declaration is what the
omit path already is. The honest limit rides with the ruling: for a consumer on an
uncovered platform, losing `check-shellcheck` means losing the self-lint floor
over their own shell gates — the one loss in this set that lands on the corpus
delta 5 keeps the gate alive for. It shrinks as targets are published, not by
repair here.

## Producers and consumers

**`--needs <name>` (new interface, delta 1).**
*Producer:* the binary's top-level arm, resolved in `main` before the registry
lookup, printing the requirement set each registry member declares in its tuple.
Its enabling configuration is none — a top-level flag on an installed artifact,
reachable wherever `--reads` is, which §check-reads-couples' consumption path
already proves is everywhere the shell readers run.
*Consumer:* `gate-sdk/bin/port-blockers.sh`'s **default arm**, at the per-member
row where it today prints `? (binary substrate; no --needs)`. That is the arm's
only consumer and the only transition it is read at. `--group` does **not**
consume it: that arm excludes ported members from the partition entirely, so it
has no row to fill.
*Every line kind has a named reader at a named transition:* `<program>` at the
floor filter, `?<TAB><knob-name>` at the bridge resolution that precedes the floor
filter, `?` at the trailer's undecidable counter. There is no fourth line kind
because there is no fourth reader.

**The registry tuple's new element (new state, delta 1).**
*Producer:* the crate's registry, at compile time; a member added without it fails
to compile, which is `--reads`' own enforcement.
*Consumer:* the `--needs` arm, and crate unit test **A**, which runs each member
over its own fixture cases with the spawn recorder armed and asserts observed ⊆
declared. Test **B** — no `Command` construction outside the sanctioned wrapper —
is **already enforced** by the wrapper's existing unit tests (§Meta-gate
conservation, the `check-gate-fail-closed` row), so this amendment names it as the
property A rests on rather than as new work.

**The spawn recorder (new state, delta 1).**
*Producer:* `proc::run` / `proc::run_with_stdin` (`native/src/proc.rs:36,52`),
when recording is enabled by the test harness.
*Consumer:* test A alone, at the post-run comparison. It has exactly one reader
and is not reachable from a gate module, which is what keeps it from becoming a
runtime surface.

**The absent-program refusal message (new field on an existing behavior,
delta 2).**
*Producer:* each of the five wrappers, at the pre-spawn probe — `check-shellcheck`
and `check-action-run-shell` for `shellcheck`, `check-crate-arms` for `cargo` and
`rustc`, `check-docs-render-fidelity` for the renderer knob's command word, and
`check-producer-liveness` for `ps` on the fallback path.
*Consumer:* the committing session through §Output contract, `run-gates.sh`
through the exit code, and the parity scenario through the byte comparison.
Its enabling condition — the program's absence — is not representable in a
committed fixture, which is why the scenario is named in the delta rather than
left to the pair.

**The `ek_pid_alive` cross-substrate comparison (new standing oracle, delta 9).**
*Producer:* evidence-kit's fixture lane, feeding one canned corpus of PID states
to the shell helper and to the compiled member.
*Consumer:* the byte comparison at the lane's own assertion, on every run of that
lane. Its enabling configuration is the lane, which already exists; the surviving
shell caller `evidence-kit/bin/run-validate.sh:52` is the reason it is standing
rather than one-shot.

**Existing producers whose consumers must be re-verified at build**, because this
unit changes the declaration path they read and the contract is what they consume:
`gate_resolve` (which spelling it returns for eight members), `gates_list_members`
(six entries unchanged, two members never in it), `gate_manifest_field` (reading
`# graph:` out of a descriptor rather than a script for eight members),
`recipe_gates` (the installer's derived roster, which reads both declaration
spellings and must keep seeding the two `zero-config` members), and
`scripts/gate-exec.sh` (the name-resolving front end the entry-preflight wiring
depends on, delta 9).

**Red conditions named, because three deltas narrow a corpus.** Delta 4 narrows
`check-gate-substrate-parity` assertion G's corpus to **empty** in this tree —
G's red condition must be *a `<name>.sh` declaration whose declared field is
malformed or whose `# spec:` section does not state the hold*, which is monotone,
and **not** *no declaration found*, which is the reds-on-finding-none shape point
5 names. The empty case is carried in the pair. Delta 5 narrows
`check-shellcheck`'s corpus; its red condition is *a `.sh` under a resolve dir
failing shellcheck at -S warning* — monotone, clearable by inspection. Delta 8
narrows the gawk-extension corpus to empty; its would-be reader is
`check-install-toolchain`, whose red condition is a **two-way** parity between
`context-kit/lib/toolfloor.sh`'s probe roster and `docs/install.md`
§Requirements — an **exact-match** condition, therefore **not** monotone, and
therefore the reason delta 8 deliberately changes neither side. Retiring the
holder without touching either surface leaves that parity intact; narrowing one
side alone would red it, which is precisely why the narrowing is
`interpreter-floor-gawk-residue-empty`'s and not this unit's.

## Existing sections updated

- **gate-sdk/SPEC.md §The `# graph:` manifest** — the `# needs:` refusal stands
  and gains its discharge: the sequenced interface is built, both the "fifth
  top-level flag" and the "fifth registry-tuple element" ordinals are dropped
  rather than incremented or corrected to "sixth", and the paragraph stops
  saying `--needs` is unshipped (delta 1).
- **gate-sdk/SPEC.md §The port-candidate criteria, criterion 7** — the class-(i)
  ruling gains its executed form; `check-crate-arms`' requirement is corrected to
  `cargo` and `rustc`; the `check-action-run-shell` worked example and the
  `check-crate-arms` permanence paragraph move to the past tense; and the
  undecidable-count paragraph records that the count falls rather than grows for
  the first time (deltas 1, 2, 5, 6, 7, 8).
- **gate-sdk/SPEC.md §The port-candidate criteria, criterion 5** — this cut's
  residual, measured on both sides, its predicted growth of two, and the
  accept-and-declare judgment with its `check-shellcheck`-shaped limit (delta 11).
- **gate-sdk/SPEC.md §port-blockers** — the default arm's `.gate` row stops being
  `?` and becomes the `--needs` answer, with the knob line kind's bridge
  resolution; the `--group` arm is stated unchanged, and the trailer's
  six-still-owed reading is superseded by a run (deltas 1, 10).
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — the
  derived set is re-taken across eight changed declaration paths and every
  affected row gains its disposition; the `check-shellcheck` row is left as a
  per-member disposition and explicitly not read as the gate's retirement
  (deltas 5, 10).
- **gate-sdk/SPEC.md §check-gate-substrate-parity** — assertions B, G and H over
  eight new descriptors, and G's empty-corpus verdict fixed as
  green-with-a-counted-zero (deltas 4, 10).
- **gate-sdk/SPEC.md §check-shellcheck** — the port, the wrapper's refusal, and
  the ruling that this tree's deregistration is not the gate's retirement
  (deltas 2, 5).
- **gate-sdk/SPEC.md §check-action-run-shell** — the port, the inline extractor
  and the skipped-and-counted dialect branch carried as locality-class directives
  (deltas 2, 6).
- **gate-sdk/SPEC.md §check-crate-arms** — the port, both declared programs, the
  source-stamp cache carried, and the both-arms-run-even-when-the-first-fails
  hazard pinned (deltas 2, 7).
- **gate-sdk/SPEC.md §check-install-disposition** — the port, criterion 4's
  binding verdict and the four-arm fixture widening that discharges it (delta 3).
- **gate-sdk/SPEC.md §Fail-closed contract** — the absent-program refusal joins
  the exit-2 register as a compiled-path instance, and the spawn wrapper's
  recorder is named as a test-only surface (deltas 1, 2).
- **gate-sdk/SPEC.md §The install disposition** — `recipe_gates` keeps seeding
  the two `zero-config` members across their spelling change, which is the
  assertion the derived roster already makes and which delta 11's prediction
  rests on (deltas 10, 11).
- **site-kit/SPEC.md §check-docs-render-fidelity** — the port, the knob-derived
  requirement, and the retirement of the GNU-awk floor's last live holder stated
  without narrowing the published requirement (delta 8).
- **site-kit/SPEC.md §lib/site.sh** — the conditional `SITE_KIT_RENDERER_BATCH`
  default stays shell-side and its resolved value crosses the bridge, including
  the empty case (delta 8).
- **canon-kit/SPEC.md §check-surface-duplication** — the port, `dir=bi`, the two
  valves, the `align-only` tier's criterion-3 cost, and the absent-glossary exit 2
  reproduced rather than converted to a skip (delta 9).
- **gate-sdk/SPEC.md §The port-candidate criteria, criterion 6** — `ek_pid_alive`
  joins `lib/queue.sh` and `gate_staged_matches` as a worked instance of the
  *unless* clause discharged by a standing comparison, and as the second instance
  of a port **creating** the twin rather than inheriting one (delta 9).
- **evidence-kit/SPEC.md §check-producer-liveness** — the port, both modes, the
  aggregation rule, and `ps` declared as the member's external requirement rather
  than left implicit in the `ps -p` leg the pair's PID-1 case depends on
  (deltas 2, 9).
- **evidence-kit/SPEC.md §lib/evidence.sh**, on `ek_pid_alive` — the helper keeps
  its surviving shell caller and gains a compiled twin, with the standing
  comparison named as what holds the two together (delta 9).
- **evidence-kit/SPEC.md §lifecycle-kit integration** — the "would break at its
  port" parenthetical becomes a discharged one: the wiring already names the gate
  through a front end (delta 9).
- **gate-sdk/README.md** — the `port-blockers` description and the binary's
  top-level arm roster both enumerate the arms, so both gain `--needs`
  (all deltas).
- **`native/src/main.rs`'s usage line** — enumerates the top-level arms and gains
  `--needs` (delta 1).
<!-- update-target-exempt: a no-change confirmation the ruling produces, owned by no delta -->
- **TRAJECTORY.md §PRIORITY DIRECTIVE** — the tail's stated sequence is unchanged
  and the bootstrap pair remains after this unit; listed so the build confirms the
  sentence rather than assuming it.
<!-- update-target-exempt: a no-change confirmation, owned by no delta by construction -->
- **docs/install.md §Requirements** — deliberately **not** narrowed by this unit,
  and listed so the build confirms it was left alone rather than silently
  adjusted while delta 8 retires its stated ground.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather
      than at the commit, a sibling amendment being in flight for gate-sdk.
- [ ] **Removals propagated** — grepped every spec for names this change retired
      (the eight `<name>.sh` declaration paths, the "`--needs` is sequenced, not
      shipped" clause, the "fifth top-level flag" ordinal, `check-crate-arms`'
      permanence reading, and the cargo-alone requirement); nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
