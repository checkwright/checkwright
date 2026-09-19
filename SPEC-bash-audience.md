# SPEC amendment: bash-audience

The `bash` element of the probe roster carries a hand-held kit list —
`bash:4.3::context-kit+delegation-kit+drift-kit+guard-kit+lifecycle-kit`, spelled
at `native/src/toolfloor.rs:8`, `native/src/programs.rs:48`, context-kit/SPEC.md's
roster and docs/install.md's requirements. Rung 4a (`floor-bash-hooks-front-end`,
merged into context-kit/SPEC.md §bin/env-probe) enumerated it by hand and filed
its own honest limit: *"A kit that later ships a bash surface without joining the
list is under-declared, and nothing reds."*

**The limit is already realized, and the entry's framing of it as latent is
wrong.** Probed per kit root over the tracked tree (`git ls-files <kit> | grep
'\.sh$'` minus `gate-tests/` and `smoke/`; bash shebangs in those; `git grep -n
'bash ' -- <kit>/templates <kit>/lib <kit>/bin <kit>/README.md`), the declared
list matches **neither** candidate predicate:

- Under *ships a bash script*, only **context-kit** (`templates/session-context.sh`),
  **drift-kit** (`templates/kpi-deprecated-surface.sh`) and **guard-kit**
  (`lib/guard.sh`, `templates/bash-guard.sh`) qualify. The list
  **over-declares** delegation-kit and lifecycle-kit, neither of which tracks one
  `.sh` outside its tests.
- Under *instructs a reader or a harness to run bash*, **all eleven** kit roots
  qualify. The list **under-declares** canon-kit, doctrine-kit, evidence-kit,
  queue-kit and site-kit.

Rung 4a cleared evidence-kit and queue-kit on the ground that they appear in the
probe only through the header comment of their config templates, which it called
reference text rather than a spawn. That premise does not survive a wider probe:
`queue-kit/README.md:75-86` is a twelve-line block of
`bash gate-sdk/bin/run-gates.sh` commands a reader is told to run, and
`evidence-kit/README.md:82` is another. The clearance rested on a corpus that
excluded READMEs — which is exactly the entry's observation that nothing
re-derives the list.

**So the fix is not only a derivation; it is a correction with a sequencing
constraint.** The sibling unit `binary-door-wide-sweep` removes the
*instructs-bash* reach from every kit surface. Once it lands, the two predicates
**converge**, and both name the same three kits: context-kit, drift-kit,
guard-kit. That convergence is what makes a derivation cheap and honest — while
the two predicates disagree, any derivation has to pick one and defend it, and
either pick publishes a floor the other reading contradicts. This amendment
therefore derives the *ships-a-bash-surface* reach and lands **after** the door
sweep, when it is the only reading left.

It is a root-level amendment because it spans context-kit (the roster's owning
SPEC), `native/` (the roster literal, the predicate and the new derivation) and
`docs/` (the rendered requirements page).

**The tree does not already do this.** `git grep -n
"context-kit+delegation-kit+drift-kit+guard-kit+lifecycle-kit"` returns the four
hand-held sites; no code derives the set.

## What changes

**Batching.** Deltas 1 and 2 land in one commit — `check-install-toolchain` reds
while the roster literal and the page's bullet disagree. Delta 3 lands with them,
because the derivation is what makes delta 1's value defensible rather than a
second hand-list. Delta 4 rides with all three; it proves them.

### (1) The bash audience is derived, not listed {design-bearing}

**Not yet applied.** `toolfloor` gains a **derived audience**: an element whose
audience field is the literal `::derived` takes its kit list from a predicate the
crate evaluates over the vendored kit roots, rather than from a spelled list. The
`bash` element becomes `bash:4.3:::derived`.

**The predicate, stated as the one a reader can check.** A kit root is in the
bash audience when the kit **ships a file the adopter's host will run with
bash** — a tracked file under that root, outside `gate-tests/` and `smoke/`,
that either carries a bash shebang or is named in a shipped settings template
as a command a harness spawns. Prose that tells a reader to *type* a bash command
is deliberately **not** the predicate: after the door sweep no kit ships such
prose, and admitting it would re-open the disagreement this delta exists to
close. The exclusion of `gate-tests/` and `smoke/` is the contributor boundary the
roster already draws with its `contributor` audience value, reused rather than
restated.

**The derivation walks `walk::kit_roots()`, not `kit_roots_rel()`** — it opens
files, so it takes the repository-path spelling. This is the dialect rule the
sibling unit `kit-roots-rel-filesystem-readers` states; naming it here keeps this
delta from adding a nineteenth site to that unit's roster.

**Measured value at this amendment's authoring:** `{context-kit, drift-kit,
guard-kit}`. gate-sdk is **not** a member: it is the floor-holder whose own
`bin/` and `lib/` the roster's `contributor` and door surfaces already account
for, and admitting it would make the audience unconditional, which is the floor
the adopter constraints refuse (gate-sdk/SPEC.md §The adopter constraints, *the
floor is git*).

**Why `:derived` and not a computed default written back into the literal.** A
written-back value is a copy with a freshness gate, and the Derivation-first rule
prefers deriving to maintaining-plus-gating. The audience is read at probe time
by a process that already has the kit roots in hand, so there is nothing to cache.

### (2) doctor, the page and the env-probe arm render a derived audience {design-bearing}

**Not yet applied.** Each reader of the audience field renders a `::derived`
element by evaluating the predicate first, so none of them learns a new shape:

- **`toolfloor::owed`** resolves `::derived` to its kit list and then takes the
  existing kit-list arm of the owed-predicate unchanged (context-kit/SPEC.md
  §bin/env-probe, *The owed-predicate* — the union of the named kits).
- **`check-install-toolchain`** reads the rendered parenthetical, so
  docs/install.md's `bash` bullet renders the **derived** list verbatim and the
  page stops carrying a hand-held one.
- **doctor**'s installed, bare and undecided reports name the derived kits.
- **the env-probe arm**'s `<audience>-only` rendering marks the same set.

**Point 5 — the red condition each reader carries.** `check-install-toolchain`
reds when the page's rendered list and the derived list differ, which is now a
comparison against a derivation rather than against a literal, so the gate stops
being satisfiable by editing both sides. The closure test
`every_audience_value_is_closed_over_the_kit_roots` (`toolfloor.rs`) reds when the
derivation names a kit root that does not exist — the direction that a hand-list
could violate and a derivation cannot, kept because the *predicate* can still be
wrong even when the roots are right.

### (3) The correction is stated where the floor is declared {design-bearing}

**Not yet applied.** context-kit/SPEC.md §bin/env-probe's audience-axis paragraph
gains the derived value beside the kit-list one, and states the predicate above as
the contract a kit is measured against — so a kit author reads what puts their kit
on the floor, rather than discovering it from a doctor report.

docs/install.md §Requirements' `bash` bullet renders the derived list. Its macOS
paragraph — which says the remedy block is needed only where a selected kit owes
`bash` — is unchanged in substance and re-worded only where it names the audience.

**This delta narrows the declared floor for two kits and widens it for none.**
delegation-kit and lifecycle-kit leave the bash audience. Under the declaration
grammar the adopter constraints name (*"a user-facing surface states a floor as
reached only once it is"*), a narrowing is the safe direction: a host that met
the old declaration meets the new one. It is stated rather than slipped in,
because a reader whose profile carried delegation-kit was told they owed `bash`
and now is not.

### (4) The fixture pair exercises a kit that joins and a kit that leaves {mechanical}

**Not yet applied.** `check-install-toolchain`'s existing fixture pair gains a
`good/` tree whose derived audience is two kits and a `bad/` tree where one kit
root ships a bash-shebang file the page's rendered bullet does not name — the
under-declaration this unit exists to make impossible, exercised as the gate's
own red rather than argued in prose.

Mechanical: the fixture shape and the runner are gate-sdk/SPEC.md
§Fixture-pair discipline's, and the assertion is delta 2's.

## Producers and consumers

- **The `::derived` audience value (delta 1).** Producer: `PROBE_SET`'s `bash`
  element in `native/src/toolfloor.rs`, and the twin literal in
  `native/src/programs.rs`, which delta 1 collapses onto the same holder so the
  two cannot disagree. Its enabling configuration is the vendored kit set itself —
  `GATE_SDK_ROOT` and `GATE_SDK_KIT_DIRS`, which every install sets — not a new
  knob and not a test fixture.
- **The derivation (delta 1).** Consumers, each at a named transition:
  `toolfloor::owed` at a probe; `check-install-toolchain` at a battery run;
  doctor's three reports at an install inspection; the env-probe arm when a
  session seeds `ENV.local.md`; the closure test at `cargo test`.
- **The rendered page bullet (deltas 2 and 3).** Producer: the env-probe
  rendering. Consumer: `check-install-toolchain`, which compares it to the
  derivation; and the adopter, who reads it.
- **The predicate as contract (delta 3).** Reader: a kit author adding a shell
  surface, and the fixture pair of delta 4, which is the executable statement of
  it.

**Point 6 — every kit root has a satisfying value.** The predicate is total over
`walk::kit_roots()`: each root either tracks a qualifying file or does not.
Measured at authoring, context-kit, drift-kit and guard-kit do; canon-kit,
delegation-kit, doctrine-kit, evidence-kit, gate-sdk, lifecycle-kit, queue-kit and
site-kit do not. gate-sdk is the one root narrowed past, on the ground stated in
delta 1.

## Existing sections updated

Rosters produced by `git grep -n
"context-kit+delegation-kit+drift-kit+guard-kit+lifecycle-kit"` (four sites), by
the per-kit bash-reach probe named in this amendment's opening, and by reading
context-kit/SPEC.md §bin/env-probe and docs/install.md §Requirements.

- `context-kit/SPEC.md` §bin/env-probe: the audience-axis bullet, the
  owed-predicate sentence, and the `bash:4.3` forcing-construct paragraph, which
  names the nameref site rather than an audience (deltas 1, 2 and 3).
- `native/src/toolfloor.rs` — `PROBE_SET`, `audience_kits`, `owed`, and the
  closure test (deltas 1 and 2).
- `native/src/programs.rs` — the `BASH` row's audience literal, collapsed onto
  `toolfloor`'s holder (delta 1).
- `native/src/gates/install_toolchain.rs` and
  `native/src/installer/doctor.rs` — the rendering readers (delta 2).
- `docs/install.md` §Requirements: the `bash` bullet and the macOS paragraph
  (delta 3).
- `installer/SPEC.md` §doctor, its undecided-member line (delta 2).
- `context-kit/checks/`'s or gate-sdk's `check-install-toolchain` fixture pair
  (delta 4).
- `.workflow/release-declarations.md` §Behavior changes, appended by the landing
  session: one bullet that `bash` is no longer owed by delegation-kit or
  lifecycle-kit (delta 3).
<!-- update-target-exempt: generated mirrors, regenerated by their freshness gates' printed commands, never hand-edited -->
- `docs/context-kit/SPEC.md`, `docs/installer/SPEC.md`.

## Blocked on

`binary-door-wide-sweep` (`SPEC-door-binding.md`). Until its deltas 1, 2 and 4
land, the *instructs-bash* reach makes all eleven kits qualify under the reading
this amendment declines, and delta 1's measured value would be wrong on the day it
landed. This is a hard ordering constraint, not a preference.

## Retired spellings

- `context-kit+delegation-kit+drift-kit+guard-kit+lifecycle-kit` — the hand-held
  bash audience, replaced by the derived one at all four of its sites (delta 1).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only** — docs/install.md's bullet
      carries no grounds; delta 1 places them in context-kit/SPEC.md.
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved** — `bash-audience-hand-held` moves to Done in the merge
      commit, a stage before the drain stage.
- [ ] **Nothing hand-lists the audience** — `git grep` for the retired literal
      returns no tracked site outside this amendment's own history.
- [ ] **The under-declaration is impossible, not merely absent** — delta 4's
      `bad/` fixture reds on a kit root shipping an undeclared bash surface.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` runs the block
      above against the tracked tree.
- [ ] **Gaps filed** — any cross-component gap build discovers is resolved that
      session.
