# SPEC amendment: consumer-remainder

The eleventh port cohort — **the whole remainder of this repo's own gates
directory, ten members, completing `consumer-gate-port-disposition` 13 of 13 and
retiring it.**

`check-docs-kit-parity`, `check-docs-mirror-fresh`, `check-docs-nav-reachable`,
`check-install-toolchain`, `check-installer-no-deps`, `check-kit-ref-liveness`,
`check-npm-publish-spec`, `check-release-channel-parity`, `check-trajectory-fresh`
and `check-value-rollup-fresh` become subcommands of the existing multi-call
binary, and `scripts/` keeps no `check-*.sh` at all.

**Operator-ruled 2026-08-15 at scope: the REMAINDER, all ten.** The destination
was ruled 2026-08-14 (TRAJECTORY.md §PRIORITY DIRECTIVE — the port track's
sequence, its consumer clause) and is not re-opened here. What this cohort adds
to that ruling is the tranche, and the tranche's ground is **not** a member
count: `--group`'s size arm is exhausted a third consecutive time, so the live
ground is the tenth cohort's — the documented blocker-retiring override
(§The first cohort, and the rule that selects the next) — plus the fact that the
first-mover design is already paid, so every member inherits it and marginal cost
per member sits at its floor.

**The census is cited, not re-bought.** `.workflow/survey-record.md`'s
2026-08-15 scope block answers *which gates compose the eleventh cohort*; its
witness was run at this stage and **holds**: `git diff --quiet
60b9e43a..HEAD -- gate-sdk/SPEC.md scripts/ native/` is clean, and re-running the
oracle `bash gate-sdk/bin/port-blockers.sh --group` reports the same verdict the
finding was written against — 104 members scanned, 0 undecidable, 47 already
ported, 57 still shell. The only corpus movement since the recorded revision is
`TASK-QUEUE.md`, moved by the scope commits that recorded this ruling. So the
membership derivation below is the record's, and this amendment re-derives none
of it.

**Two carried claims are corrected here rather than inherited, each against a
command.** The queue entry predicted the cohort might be nine and predicted which
member carries the config-bridge question; both were predictions made before
anyone read the two scripts, and probe-before-assertion says a claim one cheap
command settles is probed before it is asserted. Deltas 1 and 6 carry the probes
and the corrections. Neither touches the ruling — the ruling is *port the
remainder*, and the remainder is what it is.

## What changes

### 1. `check-docs-kit-parity` clears criterion 7, so the tranche is ten and the fallback is spent

**The `?` was a scanner limitation and never a dependency** — mechanical, since
executing this delta is running the trace the finding already records
**[mechanical]**. `port-blockers.sh` reports
`check-docs-kit-parity ? scripts/check-docs-kit-parity.sh:22 (command-position
$WRAPPED, default unresolvable)`: the command word at that line is an array
populated at run time by `gate_command`, which no static scan can resolve. Traced
by hand, the runtime program set is `{awk, bash, the crate's own binary}` — `awk`
and `bash` are on `GATE_SDK_PROGRAM_FLOOR`, and the wrapped member
`check-kit-registration` is itself already ported, so the third element is the
binary this cohort ports *into*. Criterion 7 clears.

**Scope's fallback is therefore spent rather than declined.** Scope wrote *if a
manual criterion-7 check does not clear it the tranche is 9, and the stage that
finds it says so*. This stage ran the check, it cleared, and the tranche is
**ten**. Recorded in this shape because a later reader meeting the fallback in
the queue entry needs to know it was executed and not skipped.

### 2. The composition is a first: a compiled gate whose rule is another gate's verdict

**`check-docs-kit-parity` wraps `check-kit-registration` and must keep doing so
across the substrate boundary** — design-bearing, because the shell form's
composition has no automatic translation **[design-bearing]**. The shell form
reaches the wrapped member through `gate_command`, captures its combined output,
and re-frames it: exit 2 passes the wrapped gate's diagnostic through to stderr
unchanged, a non-zero exit becomes this gate's own headed failure on stdout, and
exit 0 falls through to the nav-block sweep. The argv indirection is the point —
`gate_command` is what lets the wrapper survive the wrapped member's own port,
and it already did.

**The compiled form calls the module, and does not spawn itself.** Both members
live in one binary, so the wrapped rule is reachable as an ordinary in-crate
call; spawning the binary from inside the binary would put a gate's own process
through `native/src/proc.rs` for no reason, and would make a gate's verdict
depend on the binary being executable from a path it happens to know. The
consequence the port owes: `kit_registration::run` returns only an exit code and
writes its report to the process's own streams, which the wrapper cannot re-frame.
So the wrapped rule gains a **capturing entry point** — the existing
`run(&[String]) -> i32` retained as the dispatch signature, with the body moved
behind a form that writes into caller-supplied sinks — and `docs_kit_parity`
calls that form. The three-way rc handling above is then reproduced exactly,
which is what criterion 2 will be asked to prove.

**Ruled out, recorded because it is the cheaper-looking road:** letting the
wrapper call `run` directly and leaving the wrapped gate's output on stdout. It
passes a `good/` fixture and diverges on every failing case, where the shell form
prints one headed report and the compiled form would print two unheaded ones.

### 3. Three members of the generated-projection freshness family port ahead of their emitters, and the cost is stated rather than absorbed

**This cohort ports `check-trajectory-fresh`, `check-value-rollup-fresh` and
`check-docs-mirror-fresh` under the operator's remainder ruling, against a
standing sequencing finding that would have held all three** — design-bearing,
because what the delta decides is whether a recorded finding is superseded or
contradicted **[design-bearing]**.

§The generated-projection freshness family states, of its six members, that *not
one of [their emitters] is ported, so every member is held for any cohort that
ports no emitter*, and that *a ported byte-comparator spawning a shell emitter
removes no shell, so it buys nothing against the dual-maintenance ground the port
rests on*. Both sentences are true and neither is repealed. What changes is their
standing: they are a **sequencing** finding, and TRAJECTORY.md §PRIORITY DIRECTIVE
rules that no port-candidate criterion survives as an eligibility gate — the
technical problems the criteria name are work the port owes, not exclusions it
may take. The operator's remainder ruling is the later and more specific
instruction, and it ports the remainder.

**So the honest accounting is written into the family's own section rather than
left implied.** The three port; their emitters do not; the dual-maintenance win
for those three is **zero** until the emitters follow, and the win this cohort
actually banks for them is the *consumer-tranche completion* — an empty
`scripts/` of check scripts — not a shrunken interpreter surface. Recording it
where the family is documented is what stops the next selector reading three
ported members as evidence that the family's hold was wrong.

**The emitters are filed, not adopted.** §The generated-projection freshness
family's own transferable conclusion is that *the cheap cohort in this family is
the emitters, not the gates*. Porting `scripts/gen-value-rollup.sh`,
`scripts/gen-docs-mirror.sh` and `drift-kit/bin/trajectory.sh` is therefore real
and identified work — and it is outside this unit's ruled scope, so it is filed as
a costed gap under scope-gated intake rather than pulled in here.

### 4. The family's first porter lands the report cap, and this cohort is it

**The `head -20` literal becomes one crate constant with its first reader in this
same commit** — mechanical, because the value, the placement and the rule are all
already settled **[mechanical]**. §The diff renderer states the rule and the
reason it is not landed early (a constant no code reads is `dead_code` under
`check-crate-arms`' clippy arm). Two of the three freshness members carry the
literal today — `scripts/check-value-rollup-fresh.sh:42` and
`scripts/check-trajectory-fresh.sh:32`, both `diff … | head -20` — so this cohort
lands the constant and both readers together, and every later family member reads
the constant instead of repeating the literal. The renderer stays uncapped:
`native/src/diff.rs`'s `normal_diff` returns every hunk and the cap is applied by
the caller, which is the separation that section already rules.

**The renderer is not spawned.** `diff` is on the program floor, so criterion 7
would clear a `Command::new("diff")` and a session under parity pressure has every
reason to reach for it; §The diff renderer refuses it and this cohort inherits the
refusal rather than re-arguing it.

### 5. `check-docs-mirror-fresh` owes a fail-closed repair, and the port is where it comes due

**Its orphan sweep silences stderr, so an unreadable tree reads as no orphans** —
design-bearing, because the repair changes the gate's verdict on a case the shell
form got wrong **[design-bearing]**. §The generated-projection freshness family
already names this as what the member owes past its emitter. In the compiled form
the defect is not merely repaired but **unrepresentable for the read half**: a
fallible read returns a `Result` that cannot be dropped (§Fail-closed contract),
and the sweep's own walk goes through the crate's single sanctioned walk
implementation, so the roots are recorded for `--reads` rather than invisible.

**This is the one place the port changes behavior, and it is declared rather than
smuggled.** Criterion 2 proves a ported member byte-identical to its shell
original; here the two forms deliberately diverge on exactly one input class — a
`<root>/docs` the process cannot read — where the shell form prints `clean` and
the compiled form refuses at exit 2. Named as the designed divergence, on the
tenth cohort's precedent for the version comparator's refusal, so a parity run
reporting it reports a pass rather than a finding.

### 6. The config-bridge question belongs to `check-kit-ref-liveness`, and it turns out to be the bridge's ordinary path

**The queue entry's attribution is corrected against a probe** — design-bearing,
because the correction changes which member carries a named open question and
what that question costs **[design-bearing]**. `consumer-gate-port-disposition`
records that `check-installer-no-deps` *carries the knob-declaring dispatch
question still owed since the zero-knob first tranche never reached the config
bridge*. Probed: `scripts/check-installer-no-deps.sh` reads **no** knob — its only
environment read is the `GATE_SDK_ROOT` bootstrap every gate script carries
identically to locate `lib/gate.sh`, which has no compiled counterpart because a
compiled gate sources nothing. Grepping the same shape across all ten members
finds exactly one business-logic knob read in the tranche:
`scripts/check-kit-ref-liveness.sh:66`, `GATE_SDK_QUEUE_FILE`.

**And that knob is the easy case, which is the substantive half of the
correction.** §The declaration cohort named the hazard precisely: the bridge
attributes a declared knob from the knob's **own name**, never from the declaring
gate's location, so a knob whose name carries no kit's prefix would resolve
against a kit that does not define it and the undeclared-knob refusal would fire
on every invocation. `GATE_SDK_QUEUE_FILE` carries gate-sdk's prefix, so it
resolves under gate-sdk's library on the bridge's ordinary path, and the member
declares it in its registry tuple's knob slot like any kit-declared member.

**So this tranche completes 13 of 13 without answering the config-bridge
question, and that is the finding rather than an omission.** The unanswered case
is a **consumer-owned** knob name declared by a consumer-declared member; no
member of the thirteen has one, so designing for it here would be designing
against no case — the same ground on which the tenth cohort declined it. What the
tranche *does* deliver is the first proof that attribution-by-name works from a
`-` owner root: a consumer-declared subcommand resolving a kit-prefixed knob
through the bridge, which was asserted by the first-mover design and never
executed. The open question is restated where it lives (§The declaration cohort)
as belonging to the first consumer-owned knob rather than to this tranche.

### 7. `check-installer-no-deps` drops `jq` for the crate's existing JSON reader

**The only genuine non-floor dependency in the tranche is retired rather than
carried** — mechanical, because the reader already exists and the rule is a field
test **[mechanical]**. The gate's whole predicate is *does this package declare
any of three dependency fields, or any of three install-time lifecycle script
keys* — a `has(key)` test over one object and one nested object, which
`serde_json` (a crate dependency since the settings cohort, §The settings cohort,
and the crate's first dependency) expresses directly. The `command -v jq` guard
and its fail-closed exit 2 go with it: a linked parser cannot be absent from
`PATH`, so the branch has no compiled counterpart. The gate's own semantics are
preserved exactly, including the one they turn on — **the field's presence is the
finding, not its emptiness**.

### 8. Every member declares its owner as `-`, its reads, and its knobs, and the first-mover design is inherited whole

**Ten registry tuples, each un-omittable by construction** — mechanical
**[mechanical]**. `native/src/gates/mod.rs`'s `GateEntry` is a five-tuple —
subcommand name, function, declared walk roots, declared knobs, declaring root —
and a member added without them fails to compile. All ten carry `-` in the owner
slot, the sentinel §check-gate-substrate-parity assertion B's owner clause rules
for a member the consumer's own gates directory declares; assertion B's scope rule
then requires a descriptor for each iff the tree is a publishing tree, the
predicate assertion F computes. Nine of the ten members' knob slots are empty and
`check-kit-ref-liveness` carries `GATE_SDK_QUEUE_FILE` (delta 6). Walk roots are
declared per member and held to executed behavior by the crate's two `--reads`
unit tests, not by review.

### 9. The substrate-sensitive set is re-derived at this cut, in both directions

**A port changes declaration paths, so it can move members into and out of the
derived set** — design-bearing, because clearing a narrowing by inspection is
exactly the reasoning canon-kit's causal-completeness point 5 rules unsound
**[design-bearing]**. §Meta-gate conservation states the derivation, states that
*where a declaration lives is not a term of it*, and states that a member the
section does not name is red. Ten `<gates-dir>/<name>.sh` paths become
`<gates-dir>/<name>.gate` in one commit, which is the widest single move the
derivation has seen.

**Enumerated by red condition, never by subject.** `check-value-rollup-fresh` is
itself in the table's *survive unchanged* row and is itself a member of this
cohort, so its row is owed an update saying it is now `.gate`-dispatched — the
shape the table already uses for `check-comment-tier`, `check-settings-paths` and
the others that ported into their own rows. Its `couples=` names
`scripts/*.sh,kit:*.sh`; after this cohort `scripts/*.sh` covers **no** registry
member's declaration path, and the member stays substrate-sensitive only through
`kit:*.sh`, which the re-derivation confirms rather than assumes.
`check-gate-fixture-coverage` and `check-shellcheck` both read `scripts/*.sh` and
both have red conditions that are **not** monotone in the violation set — the
first reds on a registered member with no pair, the second exits 2 on an empty
target set — so neither is clearable by inspection under a narrowing and both are
re-checked by running them. Every fixture pair is retained (delta 10), and
`scripts/` keeps many non-gate `*.sh`, but the point of the rule is that those two
facts are established by the oracle rather than argued.

### 10. Every fixture pair is retained and re-pointed, and two of them cannot prove parity at all

**The pairs survive; their callers move** — design-bearing, because two members'
parity has to be bought somewhere other than a fixture **[design-bearing]**. Each
member's `gate-tests/<name>/good/` and `bad/` case directories are unchanged; each
bespoke `scripts/gate-tests/<name>.test.sh` that names its gate by **script path**
is re-pointed through `gate_run`, which is the tenth cohort's own recorded lesson
about a consumer's gates directory and the reason a behavioral test survives a
substrate change. A tranche porting consumer-declared members enumerates that
directory before the cut.

**The exception the family carries.** §The generated-projection freshness family
records that five of its six members steer their pairs off the live emitter
through the emit-source positional, so a pair proves the *comparison* and never
the *emitter-executing arm*. Two of this cohort's three family members are in that
five — `check-trajectory-fresh` and `check-value-rollup-fresh` — so their criterion-2
parity is bought by a **live-tree run** with both implementations still present,
not by their pairs. `check-docs-mirror-fresh` is the sixth and is explicitly
exempt: it takes no emit-source argument, both its case arg files are `.`, and it
executes the live `gen-docs-mirror.sh` against the synthetic case tree, so its
pair already proves that arm.

**Parity is proved while both implementations still exist**, which assertion A
makes the only possible order — a descriptor and a script may not coexist in one
directory, so the comparison happens before the deletions land.

### 11. The cohort is one commit, and its generated fan-out is named before it is met

**One commit, and the projections that go stale with it** — mechanical
**[mechanical]**. §The declaration cohort's construction holds unchanged:
`check-gate-binary-fresh`'s stamp is computed from `git ls-files`, so a
partial-path commit recomputes the crate's manifest without the sources the
partial commit leaves out and no binary can match it. Splitting the modules from
the registry, the descriptors or the `.sh` deletions reds rather than staging
cleanly.

Riding that one commit: `bash gate-sdk/bin/build-native.sh`; the generated
pre-commit hook; the docs mirror and the `check-graph` artifact; the enforcement
map, the footprint and the value rollup — regenerated **after** `git add`, since a
file this commit adds has no footprint cost until it is staged; and
`docs/install.md`'s marked `ported-gate-members` literal, which
`bash scripts/measured-claims.sh` reports as **47** today and which this cohort
moves to **57**. The hook bakes that same value, which is why the regen is owed
with the doc edit rather than after it. The roster and each regen command are
docs/site-architecture.md §Generated projections and their freshness gates'; each
freshness gate prints its own command on red.

### 12. Criterion 5 is priced by measurement, with the judgment fixed in advance

**All ten are consumer-declared, so no adopter loses anything, and the measurement
still has to be taken** — mechanical **[mechanical]**. The ninth and tenth cohorts
both measured **12 members omitted and declared** on `installer_smoke`'s
binary-less leg from a clean checkout. All ten members here sit in no kit's
`checks/`, `init` can never seed them, and no adopter has ever had them — the
tenth cohort's stronger ground, which predicts **zero** growth. The measurement is
taken after this cohort's own commit and the cohort lands on the finding; a
non-zero would be a finding about the measurement as much as about the cohort. N
members each individually runnable is not a measurement, which is why the ground
does not discharge the criterion.

### 13. The queue entry's terminal move is a Done move, and this is the one cohort where it is

**Thirteen of thirteen retires the entry** — mechanical **[mechanical]**.
`consumer-gate-port-disposition`'s deliverable is thirteen members; every previous
tranche delivered an increment and demoted, under §Merging an amendment's
corpus-increment branch. This tranche delivers the last ten, so the entry moves to
`## Done` and drops its `[spec:]` tag. `native-gate-port-remaining-corpus` is a
different entry with a different corpus (57 unported of 104) and keeps demoting;
its member count moves from 47 to 57 ported.

## Producers and consumers

**This amendment introduces no new state surface, no new stamp grammar, no new
tag and no new knob.** What it introduces is ten dispatch entries, one crate
constant, one capturing entry point on an existing module, and ten descriptors.
Each is named with its producer, its consumer and the transition where it is read.

**Ten subcommands.** *Producer:* `native/src/gates/mod.rs`'s dispatch registry,
built into the binary by `bash gate-sdk/bin/build-native.sh`; its enabling
configuration is the binary's presence, resolved by `gate_command`'s existing
sentinel dispatch, which no knob gates. *Consumer:* `run-gates.sh` and the
generated pre-commit hook, at every gate run, through `gate_command` — the same
call the shell forms answered, which is why no caller changes. On a tree with no
binary the members are **omitted and declared** rather than dispatched into an
absent artifact (§The install disposition, §check-gate-binary-fresh) — the
producer's absence is a declared absence, not a broken battery.

**Ten `.gate` descriptors.** *Producer:* this cohort's commit, in
`scripts/`. *Consumers, each at a named transition:* `gate_command`'s resolution,
at dispatch; `check-gate-substrate-parity` assertions A and B, at pre-commit
(A reds on a descriptor and a script coexisting, B on descriptor/roster
disagreement scoped by the owner column); `check-graph`, reading the `# graph:`
manifest to regenerate the hook; `check-comment-tier`, `check-spec-pointer`,
`check-todo-task-liveness` and `check-deprecation-task` through the shared
`comment_surface` primitive's `*.gate` arm; `check-exec-bit`, asserting the
descriptor is **non**-executable.

**Each descriptor's fields have named readers, and one field is load-bearing in a
way a copy of the old manifest would miss.** The `# graph:` manifest's `couples=`
must name the member's own `native/src/gates/<name>.rs` — read by `check-graph` to
put the implementation in the gate's own trigger set, and read by
`check-gate-substrate-parity` assertion C's derivation, which is what makes a
ported member substrate-sensitive at all. The `# spec:` pointer is read by
`check-spec-pointer`. There is no third field: the closed field roster is what
makes the output-contract strings unreachable from the descriptor and is why
`check-gate-output` follows the rule to the implementation module.

**Three registry-tuple slots, all un-omittable.** *Producer:* the tuple, at
compile time — a member added without them fails to compile, which is the
construction that replaces a declaration nobody checks. *Consumers:* `--reads
<name>` feeds `check-reads-couples`' coverage assertion at pre-commit, and the
crate's two unit tests close the loop (every member's observed walk roots a subset
of its declared ones; no module outside the sanctioned walk implementation names a
walk API). `--knobs <name>` feeds the config bridge at every dispatch — the reader
that makes `check-kit-ref-liveness`' `GATE_SDK_QUEUE_FILE` resolve rather than
refuse. `--list`'s second column feeds assertion B's owner clause, and the crate's
owner unit test resolves each declared root to a descriptor on disk.

**One crate constant — the report cap.** *Producer:* this cohort's commit, in the
crate. *Consumers, in the same commit:* `check-value-rollup-fresh` and
`check-trajectory-fresh`'s compiled forms, each capping its stale report where the
shell form's `head -20` did. The constant is landed **with** its readers and never
ahead of them, because an unread constant is `dead_code` under the clippy arm
`check-crate-arms` runs — the rule §The diff renderer states.

**One capturing entry point on `kit_registration`.** *Producer:* the module, this
commit. *Consumer:* `docs_kit_parity`, at its first assertion, reading the wrapped
rule's exit code and its captured report to reproduce the shell form's three-way
framing. The existing `run(&[String]) -> i32` is retained unchanged as the
dispatch signature, so the registry entry, `--list` and every existing caller are
untouched — the new form has exactly one reader and would be removed if it did
not.

**Existing integration prose describing the prior flow is updated, not left to
drift** — see below. The one flow that genuinely changes is
`check-docs-kit-parity`'s: it stops reaching its wrapped member through an argv
and reaches it through the crate. `gate_command` remains the mechanism for every
*shell* caller of a ported gate, so the indirection is not retired, only
short-circuited inside the binary.

## Existing sections updated

- **§The first cohort, and the rule that selects the next** — records the
  eleventh cohort's selection: the size arm exhausted a **third** consecutive
  time, the blocker-retiring override as the live ground, and the `--group`
  verdict at the cut. Delta: the framing above.
- **A new §The consumer remainder cohort** — the cohort's own section, in the
  place the ten before it sit: members, selection evidence, the criterion-7
  resolution (delta 1), the criterion-5 measurement (delta 12), criterion 2's
  live-tree parity for the two family members whose pairs cannot buy it
  (delta 10), and the assertion-C re-derivation (delta 9).
- **§The generated-projection freshness family** — delta 3. Three members port
  ahead of their emitters under the remainder ruling; the family's own hold and
  its *buys nothing against dual maintenance* finding are recorded as superseded
  for these three rather than repealed, and what remains owed — the emitters — is
  named. The table's `check-docs-mirror-fresh` row records that its fail-closed
  repair is paid (delta 5); the carried claim about the five emit-source pairs is
  unchanged and now has two attested consumers.
- **§The diff renderer** — delta 4. The rule *the first freshness member to port
  lands the cap as one crate constant with its first reader in the same commit* is
  discharged here; the section records which cohort discharged it and names the
  constant, so a later family member reads the constant rather than the literal.
- **§Meta-gate conservation for the binary substrate** — delta 9.
  `check-value-rollup-fresh` moves out of the *survive unchanged* row's
  unqualified reading into a row recording that it is itself `.gate`-dispatched
  from this cohort, and that its `scripts/*.sh` coupling no longer covers any
  registry member's declaration path while `kit:*.sh` still does.
- **§check-gate-substrate-parity** — delta 8. Assertion B's owner clause gains
  its first *ten-member* exercise; the section records that the `-` sentinel and
  the publishing-tree scope rule are now proved over the whole consumer gates
  directory rather than over three members. No assertion changes.
- **§The declaration cohort** — the config-bridge question it named and
  deliberately did not answer is re-pointed (delta 6): it is owed by the first
  **consumer-owned** knob name, not by this tranche and not by
  `check-installer-no-deps`, and the tranche's own knob read resolves on the
  bridge's ordinary prefix path. Its two exclusions are discharged:
  `check-installer-no-deps` folds in here as its named cheapest single-gate first
  mover, and the *other ten … sequencing, not exclusion* clause is spent.
- **§The port-candidate criteria, criterion 7** — the section's existing rule that
  *whether the target of a sanctioned spawn is itself ported is a cohort-composition
  question criterion 7 does not reach* gains its worked instance: three members
  spawning unported emitters clear the criterion and are held by delta 3's
  accounting instead.
- **docs/install.md** — delta 11. The marked `ported-gate-members` literal moves
  47 → 57, and the surrounding sentence with it; the generated hook bakes the
  same value.
- **TASK-QUEUE.md** — `consumer-gate-port-disposition` moves to `## Done`
  (delta 13); `native-gate-port-remaining-corpus` records 57 of 104 ported and the
  eleventh cohort, and demotes as it always has.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named reader
      at a named transition.
- [ ] **Criterion 7 traced, not assumed** — `check-docs-kit-parity`'s runtime
      program set established by hand and recorded; the fallback declared spent.
- [ ] **Parity proved while both implementations exist** — every member's script
      and its subcommand run over every fixture case, over the live tree and over
      the edge roots, compared on stdout, stderr and exit code, **before** any
      deletion. `check-trajectory-fresh` and `check-value-rollup-fresh` bought by a
      live-tree run rather than by their pairs. Exactly one divergence is expected
      and is the designed one (delta 5); any other is a defect.
- [ ] **Assertion C re-derived at the cut, in both directions** — every member the
      derivation newly selects carries a disposition row, and every non-monotone
      reader of the narrowed `scripts/*.sh` corpus (`check-gate-fixture-coverage`,
      `check-shellcheck`) is re-checked by **running** it, never cleared by
      inspection.
- [ ] **Criterion 5 measured** — `installer_smoke`'s binary-less leg run from a
      clean checkout after this cohort's commit; the omitted-and-declared count
      recorded and compared against 12.
- [ ] **One commit** — modules, registry, descriptors, `.sh` deletions, re-pointed
      bespoke tests, hook regen, built binary and every stale projection in one
      commit; the projections regenerated after `git add`.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`). Discharged at the **iteration**, not at
      this commit, where sibling amendments are in flight.
- [ ] **Removals propagated** — grepped every spec, doc and fenced command for the
      ten deleted `scripts/check-*.sh` paths; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks; the three unported emitters filed as a costed entry rather than
      adopted mid-cohort.
