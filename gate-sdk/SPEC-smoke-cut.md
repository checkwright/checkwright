# SPEC amendment: the consumer-smoke contract cut, and the `smoke/` class disposition

The cut governed by `native-gate-port-remaining-corpus` under the 2026-08-28
stated-contract composer, selecting `gate-sdk/SPEC.md §Consumer smoke` by
operator ruling of 2026-08-30 (channel: lead-relay). The corpus entry demotes
rather than completes; the class entry
`kit-smoke-port-disposition-cohort` is what this amendment promotes, because the
ruling this cut owes is a **class disposition for all eleven kits**, not a
per-file sizing.

**The census, re-derived at this session rather than carried.** Corpus witness on
scope's 2026-08-30 survey passed unchanged — `**/*.sh` byte-identical since
`18408035`, oracle re-run identical at 139 scanned / 15 no-port / 1 held / 123
owed / 14214 owed lines. Grouping the owed list by each file's own `# spec:`
line ranks §Consumer smoke first on both axes: **17 owed files, 1572 owed
lines**, 14% of the owed file count in one section.

**Two premises this cut was scoped from are false, and both are corrected here
rather than worked around.**

- **The cut reaches eight kits, not ten or eleven.** The 17 are: canon-kit,
  doctrine-kit, evidence-kit, gate-sdk, lifecycle-kit, queue-kit and site-kit
  pairs (`smoke/install.sh` + `smoke/violation.sh`, 14 files), guard-kit's
  `smoke/install.sh` alone (it ships no `violation.sh`), and the two gate-sdk
  harness members `bin/run-consumer-smoke.sh` and `lib/consumer-smoke.sh`. The
  cohort entry's "ten kits' pairs plus two harness members" is arithmetically
  impossible — ten pairs is twenty files.
- **No member of this class carries a `# no-port:` declaration, and the class has
  zero precedent in either direction.** The cohort entry cites
  `gate-sdk/smoke/install.sh` as "already carrying `# no-port: the adoption
  bootstrap runs before any binary exists` on one of its arms". It does not: that
  string sits at `gate-sdk/smoke/install.sh:216`, inside a **quoted heredoc**
  writing a fixture file (`pbtree/permanent.sh`) into the scratch consumer to
  exercise the port-blockers oracle's own declaration parser. The oracle agrees —
  it reports that file `owed`, and `walk.rs` reads a declaration only from a
  file's header. The entry's earlier sentence ("zero precedent in either
  direction") is the true one; the later sentence contradicts it and is deleted.

**The smoke class crosses four stated contracts, not one and not three.** Seventeen
files answer to gate-sdk §Consumer smoke; context-kit's three and delegation-kit's
two answer to their own §Testing sections; drift-kit's `smoke/install.sh` answers
to drift-kit/SPEC.md §Testing and is `# port-until:`-held against this very entry.
A stated-contract cut reaches the seventeen alone, and the ruling below is written
to reach the other five files by its *ground* rather than by its scope.

## What this cut is not

**It is not a size judgment, and the disposition below must not be read as one.**
The composer selects by stated contract; the criteria relaxation is closed at
gate-sdk/SPEC.md §The port-candidate criteria as an ordering signal, never an
eligibility screen. What decides this class is what its members *are*, measured
below.

**It does not reach the harness's out-of-cut callers.** `lib/consumer-smoke.sh`
is sourced by three shell callers: `bin/run-consumer-smoke.sh` (in cut),
`bin/upgrade-smoke.sh` (gate-sdk/SPEC.md §upgrade-smoke) and
`context-kit/smoke/agents-md.sh` (context-kit/SPEC.md §Testing). The last two are
other stated contracts' and stay untouched.

**It does not retire the opacity ground.** §Consumer payload rules withholding a
gate's predicate a goal. The honest limit of this cut is stated in delta 1 rather
than answered by it.

## What changes

### (1) The `smoke/` class is declared structurally unportable, ruled for all eleven kits

The class ruling `kit-smoke-port-disposition-cohort` owes: **every `smoke/`
script and both consumer-smoke harness members are `# no-port:`**, on four
measured legs, none of them size. {design-bearing}

**Leg 1 — the config bridge, and it is the load-bearing one.** The harness's
registration accounting probes each unregistered gate through `gate_command`
(§lib/gate.sh), which builds the bridge by sourcing each owning kit's `lib/*.sh`.
§lib/gate.sh rules **exactly one place a knob's value is computed**, so a
crate-side probe is the second producer criterion 6 refuses. Nor can the probe be
delegated back to a bash front-end: `run-gates.sh --only` resolves against the
registry, and the accounting's entire subject is gates that are **not** in it —
which is why the shell harness calls `gate_command` directly rather than the
front-end. This is the same structural ground `bin/gen-pre-commit.sh` is already
declared on, reached from the opposite direction: that one bakes a resolved knob,
this one resolves a knob for a member no registry names.

**Leg 2 — a `smoke/install.sh` is an executable recipe by stated contract, and
porting it is non-monotone for a live reader.** §Consumer smoke already declined
to derive the registration from the README roster, "on a boundary, not on merit:
it would turn `smoke/install.sh` from an executable install recipe into a
derivation over a doc". A crate table is the same boundary crossed harder. It is
also read *as text*: `check-install-disposition` assertion B reds on a kit that
ships a `zero-config` gate and no `smoke/install.sh`
(`native/src/gates/install_disposition.rs`, the `!Path::new(&smoke).is_file()`
finding), and its `smoke_registers` arm greps the script body for each gate name.
Deleting or de-textualizing the scripts therefore **adds** violations rather than
removing them.

**Leg 3 — the class costs an adopter no interpreter dependency, which is the
objective the port serves.** A `smoke/` directory is kit-authored content
(gate-sdk/SPEC.md's kit-authored roster names `smoke/` explicitly) and vendors to
an adopter with its kit, but is **executed by no adopter path**: the
`: "${SMOKE_KIT_ROOT:?…}"` entry-point guard refuses a bare invocation, and the
only callers in existence are the kit repo's own validate suites
(`EVIDENCE_KIT_RUN_consumer_smoke`, `EVIDENCE_KIT_RUN_upgrade`,
`EVIDENCE_KIT_RUN_agents_md_smoke`). It ships inert.

**Leg 4 — the envelope, measured class-wide rather than argued.** Of the 1349
lines across the eight in-cut `smoke/install.sh` files, **1028 (76%) sit in four
files that drive and assert on the kit's own `bin/` tools** — lifecycle-kit 471,
gate-sdk 295, doctrine-kit 203, evidence-kit 59 — against tools owned by seven
*other* stated contracts, every one still owed: lifecycle §bin/enter-stage.sh
(617), §bin/session-id.sh (75), §bin/install-lifecycle.sh (36), §The survey record
(70), doctrine §install-doctrine (194), evidence §bin/run-validate.sh (124),
gate-sdk §run-gates (328). The remaining four files (canon-kit 58, guard-kit 26,
queue-kit 25, site-kit 35 — 144 lines, zero behavioural assertion) are pure
recipe and fall to legs 2 and 3. This is the envelope hazard the drift-kit
instance showed, now measured across the class instead of inferred from one
member.

**The honest limit, stated because this ruling does not answer it.** The smoke
corpus is a worked, cleartext catalogue of exactly what reddens each gate, and it
vendors to every adopter. That is the analysis surface §Consumer payload's opacity
ruling wants raised, and it is the one live argument for porting this class. This
declaration leaves it standing rather than refuting it; a session that wants to
answer it is answering a §Consumer payload question, not a §Consumer smoke one.

**A standing ruling names this exact corpus, and it is reconciled here rather
than reversed — flagged at align, re-put to the operator with the passage
below in front of them, and re-affirmed 2026-08-30 (the entry's existing
`ruled: kit-smoke-port-disposition-cohort operator 2026-08-30 lead-relay`
line is this ruling too — same authority, date and channel, so no second line
is owed under queue-kit/SPEC.md §The tag algebra's sharing rule).** TRAJECTORY.md
§The closed rulings, 2026-08-28 ("The port's completion predicate is literal"),
corrected the carve-out in place to read: "Kit `smoke/` suites and kit-resident
test runners ride the installer payload with their kit roots and land committed
in adopter trees, so they are **kit mechanism on the claim like any owed file**;
the residue genuinely shipping to no adopter — `demo/`, `installer/consumer-smoke/`,
the declared `scripts/` class — takes a per-file disposition when reached." Two
paragraphs later it refuses "a contributor-side `# no-port:` class" for that same
corpus, on two counts: the field "declares permanence while the carve-out's own
text says those files do port," and its offered ground, "ships to no adopter,"
"was measured false for 31 of the disputed 33 files (kit `smoke/` suites and
kit-resident runners ride the payload...)." Both counts are live objections to
*this* declaration and are answered on the text, not argued around:

- **The "when reached" clause is not scoped to the three named examples.** The
  ruling's own construing sentence — "each file it covers, when reached, ports,
  leaves the tree with the surface it drives, or takes a per-file declared
  disposition under the case-by-case residue rule below" — restates over "each
  file [the carve-out] covers," and the carve-out it construes covers
  "contributor-side tooling and the test harness" as a whole, where kit `smoke/`
  suites sit. The three-name list (`demo/`, `installer/consumer-smoke/`, the
  `scripts/` class) belongs to the *earlier*, 2026-08-23 sentence being corrected,
  not to this restatement. The "case-by-case residue rule below" it names is the
  2026-08-09 PRIORITY DIRECTIVE: "Surviving shell is residue justified case by
  case, never a protected category." Four measured legs, each individually cited
  on its own file (delta 2), is that case-by-case reasoning applied to a class
  whose members happen to share legs — not a categorical "it is a smoke harness"
  exemption, which is exactly what "never a protected category" refuses.
- **The refused ground is not this ground.** "Ships to no adopter" is refuted by
  the same 31-of-33 measurement leg 3 **concedes rather than disputes** — "vendors
  to an adopter with its kit" is leg 3's own opening clause. What leg 3 actually
  rests on is narrower and untouched by that measurement: *executed by no adopter
  path* (the `SMOKE_KIT_ROOT` entry-point guard, the three `EVIDENCE_KIT_RUN_*`
  callers). Legs 1, 2 and 4 do not use the shipping question at all. "No mechanism
  is missing — a cause is free text — only a standing ruling that would make one
  true, and none does," the 2026-08-28 ruling closes; four measured, individually-
  cited structural grounds are that ruling now existing.

**The counterweight is real and it was ruled against, not absent.** The 2026-08-28
text's contrast — kit `smoke/` "on the claim like any owed file" against a residue
that "takes a per-file disposition" — genuinely reads, on its own, as putting this
corpus on the must-port side, and a seventeen-file sweep is class-shaped even
when every line cites its own ground. A later reader meeting that passage alone
must land on this ruling, not re-open the question it already answers.

### (2) Seventeen `# no-port:` declarations, each stating the class ground

Each of the seventeen gains a header `# no-port:` line naming the leg its own
membership rests on and citing the new §Consumer smoke subsection delta 3 adds —
stated rather than cited-by-example, the discipline the drift-kit cut's own four
declarations established for a class that has never been swept. The eleven
per-kit scripts cite legs 2 and 3; the two harness members cite leg 1. {mechanical}

`drift-kit/smoke/install.sh`'s `# port-until: kit-smoke-port-disposition-cohort`
hold **releases in the same commit** and converts to a `# no-port:` on leg 3,
which is what makes the ruling reach its fourth contract by ground.
`context-kit/smoke/{install.sh,violation.sh}` and
`delegation-kit/smoke/{install.sh,violation.sh}` convert the same way; `context-kit/smoke/agents-md.sh`
does **not** — it is a validate suite driver, not a `smoke/` install or violation
recipe, and it stays owed under context-kit §Testing. {mechanical}

### (3) §Consumer smoke gains the disposition, in its own subsection

A new subsection **"The port disposition"** under §Consumer smoke states the
ruling, its four legs, its honest limit, the reach-by-ground clause, **and the
TRAJECTORY.md §The closed rulings reconciliation** (delta 1) — so a later cut
meeting a `smoke/` file, or a reader meeting the 2026-08-28 "on the claim"
passage alone, reads the class ruling and its answer off the contract that
owns the class rather than off a declaration line or a queue entry.
{design-bearing}

It also states the **one condition that reopens it**: leg 1 dissolves if
§lib/gate.sh ever admits a second bridge producer, and legs 2 and 4 dissolve if
`check-install-disposition` assertion B stops reading the script as text and the
`bin/` tools the four envelope files drive are themselves ported. Written as a
reopening condition rather than as a permanence claim, because a `# no-port:` is
the permanent tier and a class ruling owes the reader what would falsify it.

### (4) The cohort entry's two false premises corrected, and the class ruled

**Landed at spec, so build does not re-do it:** `kit-smoke-port-disposition-cohort`
has already lost the ten-kits arithmetic and the false "already carries
`# no-port:`" sentence, gained the eight-kit member roster and the four-leg
disposition in compressed form, and carries `ruled:
kit-smoke-port-disposition-cohort lead 2026-08-30 own-authority` — the class
ruling is the **lead's own**, distinct from the operator's 2026-08-30 selection
of the cut, and the two must not be collapsed into one authority.

**What build still owes on this entry:** the terminal move. It moves to `## Done`,
not a demotion — its deliverable is *one ruling for the class rather than one per
cut*, which the merge completes rather than increments, so the corpus-entry
demotion branch of §Merging an amendment step 4 does not apply to it. Verify that
against the merged text rather than taking it from here: if the ruling leaves
residue the entry must instead demote with the `[spec:]` tag dropped.
{design-bearing}

`native-gate-port-remaining-corpus` demotes as the corpus entry it is, recording
the cut's oracle move. It sits at **0 lines of headroom** (probed, not inferred:
`check-queue-entry-budget`'s headroom line), so that record lands under
queue-kit/SPEC.md §check-queue-entry-budget's mandated-write reliefs —
compression by answering, or the self-served relocation onto an entry that
already owns the subject. {mechanical}

### (5) The fan-out this cut stales

No generated projection reads a `smoke/` script's body, so the fan-out is the
declaration-parity surfaces alone: the port oracle's counts move 123 → 106 owed
and 15 → 32 declared no-port (17 in-cut plus the four out-of-contract conversions
in delta 2, less drift-kit's one released hold, which moves from held to
no-port). `docs/` mirrors and `.workflow/` projections that carry an owed count
regenerate. Every count above is re-derived at build rather than transcribed from
here. {mechanical}

## Producers and consumers

This cut introduces **no new state, event, message or field**. It introduces one
new *interface-shaped* thing — a `# no-port:` declaration on seventeen files —
and one new prose contract, the §Consumer smoke subsection. Causal completeness
is therefore discharged against the existing declaration machinery.

**Producer.** The `# no-port:` header field is produced by the build session
writing the header line. Its enabling configuration is nothing: the field is read
unconditionally from a file's header by `native/src/walk.rs`'s `header_field`,
with no knob gating it, so the producer is reachable in the only configuration
that exists.

**Consumers, three, each named with the transition it reads at.**
`--emit port-blockers` (`native/src/emit/port_blockers.rs`) reads it at every
oracle run and reclassifies the file `owed → no-port`.
`check-gate-substrate-parity` assertion G reads the declaration set at every
battery run **over its own registered-gate corpus alone**, which none of the
seventeen is a member of — named for completeness of the field's reader roster,
not as a live consumer of this cut's additions (§Red conditions below).
`check-comment-tier` reads `no-port:` as a directive-tier tag
(`native/src/gates/comment_tier.rs`'s tag array) at every battery run, which is
what keeps a seventeen-line declaration sweep from reading as seventeen new
comments.

**Every field has a named reader.** The field has exactly one payload — the
cause — and its reader is the port-blockers emitter, which prints it. **Correction
at align:** the sentence this replaces claimed `check-gate-exemption-tasks` reds
an empty cause; verified against the source, it does not — its `tree_texts`
loop matches `walk::Disposition::PortUntil` only and `continue`s past every
other disposition, including `NoPort`, and its `exception-list:` array
handling (`Kind::Permanent`) accepts a bare `# permanent:` with no reason at
all. No gate validates `no-port:` cause non-emptiness outside
`check-gate-substrate-parity` assertion G's own registered-gate corpus (below),
which this cut's seventeen additions are not members of. Filed as a gap rather
than answered here. No new field is added.

**Red conditions, enumerated because delta 2 narrows a corpus** (seventeen files
leave the `owed` set) and §The causal-completeness check point 5 binds:

- `--emit port-blockers --tree` — reds never; it is an emitter. Its *count*
  falls, which is the intended move and the operator-ruled completion predicate.
  **Monotone, clearable by inspection.**
- `check-gate-substrate-parity` assertion G — **does not read any of the
  seventeen at all, verified against the source rather than assumed from the
  field's reader roster.** Its declaration loop (`native/src/gates/gate_substrate_parity.rs`,
  assertion A's loop) walks `gates.list`'s registered members resolved to their
  own `checks/` declaration path; none of the seventeen is a registered gate,
  so `count_field` never opens one of these files. The malformed shapes (a
  `no-port:` with an empty cause, a `port-until:` naming no slug, both fields at
  once, more than one of either, or either field on a `.gate` descriptor) are
  real red conditions of that assertion, but over its own registered-gate
  corpus, not this one — the fixture at `gate-sdk/smoke/install.sh`'s `pbtree`
  block exercises them against synthetic tree-walk fixtures, a different
  reader (`--tree`) entirely. **Vacuously monotone: not merely well-formed but
  structurally unreachable.**
- `check-comment-tier` — reds on a comment carrying no directive tier. A
  `# no-port:` line is in its exempt tag array, so seventeen additions are
  invisible to it. **Monotone.**
- `check-install-disposition` assertion B — **the non-monotone reader, and the
  reason this cut declares rather than ports.** It reds on `zero-config`-declaring
  kit with no `smoke/install.sh`, and on a `smoke/install.sh` whose text does not
  name that gate. Under *this* disposition its corpus is unchanged — no script is
  deleted and no body is rewritten — so it cannot flip. Enumerated anyway, because
  it is the reader a port-shaped disposition would have reddened, and recording
  that is what makes the ruling checkable rather than asserted.
- `check-smoke-entry-guard` — `continue`s past an absent script and prints a
  clean line carrying its `swept` count, so a deleted corpus would pass
  **vacuously** rather than red. Under this disposition `swept` is unchanged.
  Recorded because a vacuous pass is the failure mode this reader has and no
  count outside its own clean line would have shown it.
- `check-gate-exemption-tasks` — reds on a `port-until:` whose named slug is not a
  live queue entry. Delta 2 **releases** drift-kit's hold, so the one live
  `port-until:` in the tree disappears; the gate reds on a *dangling* slug, never
  on finding none. **Monotone.**

**The existing integration prose that describes the prior flow** is §Consumer
smoke itself, which today says nothing about the class's port status, and the
cohort queue entry, which says two false things about it. Delta 3 updates the
first; delta 4 updates the second.

## Existing sections updated

- `gate-sdk/SPEC.md` §Consumer smoke — gains "The port disposition" subsection
  (delta 3), and its `smoke/` per-kit contract paragraph gains the sentence that
  the scripts are permanently shell (delta 1).
- `gate-sdk/SPEC.md` §The `# graph:` manifest — the `# no-port:` field's holder
  set grows by seventeen; the section states the field's grammar and its
  reader roster, neither of which changes (deltas 1 and 2).
- `gate-sdk/SPEC.md` §Porting a gate to the binary substrate — §The non-gate arm's
  class gains no member, and the paragraph that would have gained one records why
  the consumer smoke is not one: its accounting probes through `gate_command`
  (delta 1).
- `drift-kit/SPEC.md` §Testing — drift-kit's `smoke/install.sh` hold releases and
  its disposition becomes the class ruling's (delta 2).
- `context-kit/SPEC.md` §Testing — the two `smoke/` recipes take the class
  disposition; `smoke/agents-md.sh` explicitly does not (delta 2).
- `delegation-kit/SPEC.md` §Testing — the two `smoke/` recipes take the class
  disposition (delta 2).
- `TASK-QUEUE.md` — `kit-smoke-port-disposition-cohort` corrected and ruled;
  `native-gate-port-remaining-corpus` demoted with the oracle move recorded
  (delta 4).
- Generated projections carrying an owed or declared count regenerate (delta 5).
  <!-- update-target-exempt: the roster is derived at regen time by
  docs/site-architecture.md §Generated projections; enumerating it here would be a
  second copy that drifts against that roster -->

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition; every reader under the narrowing has its
      **red condition** enumerated, not merely its subject.
- [ ] **Merged with no information lost** — the disposition, its four legs, and
      the TRAJECTORY.md reconciliation integrated into §Consumer smoke as a
      subsection, not appended; the reopening condition lands with the ruling.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      gate-sdk (`ls gate-sdk/SPEC-*.md`).
- [ ] **Every declaration carries a non-empty cause** — verified by reading each
      of the seventeen headers directly. `check-gate-substrate-parity` assertion
      G reds on exactly these malformed shapes but only over its own
      registered-gate corpus (§Red conditions above, corrected at align); none
      of the seventeen is a registered gate, so no oracle exercises this
      obligation here. The coverage gap this exposes — no gate validates a
      `no-port:` cause tree-wide — is filed to the gap inbox rather than
      answered by this cut.
- [ ] **The oracle move is measured, not asserted** — `--emit port-blockers
      --tree` re-run and its four counts read off the trailer; `--emit
      port-blockers --group` (the registry arm) run beside it, since only
      `--tree` is the predicate and both are owed on every cut.
- [ ] **Removals propagated** — no surface still calls the class undecided;
      `kit-smoke-port-disposition-cohort`'s "zero precedent" framing retired
      wherever it is cited.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      through `lifecycle-kit/bin/file-gap.sh` (a build-time causal gap is
      resolved that session, not deferred).
