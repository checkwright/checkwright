# SPEC amendment: consumer-toolchain-floor-filter

The toolchain roster is one flat array with no audience axis, so every reader
that gates on it gates on **all** of it. `installer/lib/doctor.sh` iterates
`PROBE_SET` whole, `render()` sets `FAILED=1` on an `absent` member, and
`installer/lib/init.sh` runs doctor as its last precondition and dies on a
non-zero status. `cargo:1.56` is in that array. **A machine with no Rust
toolchain therefore cannot install Checkwright at all**, and cannot `update`
either (`installer/lib/update.sh` execs `init.sh`) — which is objective 5
(*non-technical adopters are a design constraint*) violated verbatim against the
preview cohort, and it contradicts a claim already published on the front door:
`docs/install.md`'s cargo bullet says the tool is *a contributor requirement with
no install-time role at all … so no install path asks you for Rust*.

The docs are right and the code is wrong. The demand is inherited rather than
intended: `init` never invokes cargo, verifies with a SHA-256 digest compare, and
places with a copy, so the toolchain has **no functional tie to anything the
install path does**. It is worse than vestigial — gate-sdk's starting registry
(`installer/lib/common/recipe.sh`) contains no `.gate` descriptor at all, so the
refusal blocks an install that would not have dispatched to the binary on any
profile.

This amendment gives the roster the axis its readers already need, so the
published claim becomes a property of the mechanism rather than a sentence
nobody enforces.

## What changes

**1. The roster grammar gains a fourth positional field: the audience.**
*(design-bearing)* `context-kit/lib/toolfloor.sh` today parses
`<name>[:<min-version>[:<impl-token>]]`, empty meaning unconstrained on that
axis. It becomes `<name>[:<min-version>[:<impl-token>[:<audience>]]]`, with the
same emptiness rule: **an empty or omitted audience means every audience**, so
every element but one is untouched and `awk`, `awk:`, `awk::` and `awk:::` still
parse to one member. `tool_floor_parse` sets a new `TOOL_FLOOR_AUDIENCE`.

The value set is **closed and kit-owned**, exactly as `tool_floor_check`'s
verdict set is: the only declarable value is `contributor`, meaning *a
contributor-side floor with no install-time role*. The unmarked case is not
spelled — declaring `consumer` on six members to say nothing would be a roster
maintained against itself. A new predicate `tool_floor_consumer_side` answers the
one question a consumer-side reader asks, so no reader re-implements the emptiness
rule.

This is a grammar change rather than a filter because the alternative — a
hard-coded `cargo` exception in `doctor.sh` — is a literal that de-literalization
forbids and that re-fires the day a second contributor-only member lands. A
second parallel array was also refused: two rosters are two owners, and the docs
parity render would then have to reconcile both.

**2. `cargo` is declared contributor-audience; nothing else is reclassified.**
*(mechanical)* `cargo:1.56` becomes `cargo:1.56::contributor`. Every other
member's audience stays empty. **Reclassifying any other member is out of this
amendment's envelope** — `shellcheck` in particular backs a meta-gate a vendored
consumer battery runs, and demoting it would be a separate contract call needing
its own evidence.

**3. `doctor.sh` gates on the consumer subset, and omits the rest entirely.**
*(design-bearing)* Its loop filters on `tool_floor_consumer_side`. A
contributor-audience member is not probed, not rendered, and cannot set `FAILED`.
It is omitted rather than reported-as-informational on purpose: doctor is the
adopter's verb, and showing an adopter a tool they do not need is an invitation
to install it — the same failure objective 5 names, one step softer. The
explanation already has an owner in `docs/install.md`'s bullet, so omitting it
here restates nothing.

The consequence to state plainly, because `init` reads this exit status as a
precondition: **`DOCTOR: clean` stops being a claim about the machine and becomes
a claim about the machine as a consumer.** That is the intended narrowing.

**4. `env-probe.sh` keeps reading the whole roster, and labels the audience.**
*(mechanical)* The contributor-side reader is where a contributor-side floor
belongs, so its report is unfiltered; each contributor-audience member is marked
as such in its line so a probed `ENV.local.md` distinguishes *below your floor*
from *below the floor you do not have*.

**5. The audience is published and held in parity.** *(design-bearing)*
`scripts/check-install-toolchain.sh` extends whole-element parity to the new
field, and the `docs/install.md` toolchain bullet renders it. The token needs a
form the gate's positional awk cannot confuse with an impl token, exactly as `≥`
already disambiguates the min-version token from it: the audience renders with a
leading `@` in both the bullet parenthetical and the gate's comparison key —
`` - `cargo` (≥ 1.56, @contributor) ``. Publishing it is not decoration. An
audience mis-declaration is the one new silent failure this change makes
possible — marking `git` contributor-only would drop it from every consumer's
floor with no symptom until a gate fails — so the declaration lands on a gated
surface or it is ungoverned.

**6. The oracle that would have caught this ships with the fix.**
*(design-bearing)* The defect survived because the consumer smoke cannot observe
it: `installer/consumer-smoke/run-smoke.sh`'s preflight *requires* `cargo` and
`rustc` on `PATH` before it will run at all — legitimately, because its artifact
arm builds the crate it packs — so every `init` and `doctor` it drives runs on a
machine that has them.

The mechanism to fix that is **already in the file and already proven**: the
Node-free arm builds failing shims for `node npm npx` into a mask directory,
prepends it to `PATH`, and asserts the mask took before using it. The amendment
points that same mechanism at `cargo` and `rustc` for a new toolchain-free arm
running `doctor` and a full `init`, asserting both succeed and that the vendored
battery then runs. The host preflight is unchanged — the artifact arm still needs
the real tools, and masking is per-arm, which is why this arm can exist without
weakening the smoke's own requirements.

Reusing the existing mask rather than adding a knob is the point: `INSTALLER_SMOKE_TMP_DIR`
stays the smoke's only knob, and a knob that suppressed a roster member would be
a second, test-only audience axis whose production behavior no adopter ever
exercises. A masked `PATH` is what a toolchain-free machine actually is.

**7. `docs/install.md`'s cargo prose stays as written.** *(mechanical)* It
already states the contract this amendment implements; only the parenthetical
changes. The sentence *no install path asks you for Rust* moves from aspiration
to description, which is the whole point — and TRAJECTORY.md's *what the
objectives are not* clause is satisfied for this one member rather than in
general.

## Producers and consumers

The new interface is the **audience field** and the `tool_floor_consumer_side`
predicate over it. There is no new event or message, so the causal chain is a
declaration and its readers.

- **Producer** — the roster literal in `context-kit/lib/toolfloor.sh`, parsed by
  `tool_floor_parse` on every element every reader iterates. Its enabling
  configuration is the file itself, which every reader already sources or parses
  today; nothing new must be set anywhere for the field to be live, and delta 2
  sets it on the one member that needs it, so the producer is reachable in the
  shipped configuration rather than in tests only.
- **Consumer 1 — `installer/lib/doctor.sh`**, by sourcing the payload's copy of
  the roster (`$INSTALLER/payload/context-kit/lib/toolfloor.sh`) and calling
  `tool_floor_consumer_side` per element in its existing loop. Read at the
  transition where doctor decides whether to probe and render a member.
- **Consumer 2 — `installer/lib/init.sh`**, transitively and by exit status
  only: it reads doctor's rc at its last precondition. It gains no knowledge of
  the field, which is deliberate — the audience is resolved once, in the verb
  that owns the verdict.
- **Consumer 3 — `context-kit/bin/env-probe.sh`**, by sourcing the same library
  and reading `TOOL_FLOOR_AUDIENCE` after `tool_floor_parse`. Read at the
  transition where it renders a member's line into `ENV.local.md`.
- **Consumer 4 — `scripts/check-install-toolchain.sh`**, by its own positional
  parser (it parses rather than sources, because a fixture path is untrusted
  input — that existing rule is unchanged and the new field is parsed the same
  way). Read at the transition where it builds each element's comparison key.
- **Consumer 5 — `docs/install.md`**'s toolchain marker block, rendered from the
  roster and held against it by consumer 4.
- **Consumer 6 — `context-kit/index-tests/toolfloor-cases.sh`**, the grammar's
  own fixture reader, which pins the parse. Read at the transition where a case
  asserts what an element parses to; it gains cases for the fourth field's
  present, empty and omitted forms, since the emptiness rule is the part a
  reader is most likely to get wrong.

Every field this amendment adds is one field with six named readers. No field is
added that a reader does not use, and none is populated at a transition where it
is not read: `init` is the one component in the chain that never reads it.

**Why the drift was silent, stated because the fix must close it.**
`check-install-toolchain` compares two textual rosters — the page and the array —
and has no awareness of `doctor.sh`. So the surface that *enforces* the floor was
never held against either of the two that *declare* it, and the page could
promise "no install path asks you for Rust" beside code that did. Delta 3 closes
the gap by construction rather than by a third parity check: after it, doctor's
subset is computed from the declared field instead of being a second, implicit
declaration that could disagree.

## Existing sections updated

- **`context-kit/SPEC.md` §bin/env-probe** — owns the roster grammar, the floor
  predicate, and the per-member roster notes (align correction: the amendment as
  drafted misattributed the per-member notes to §Layout and configuration, which
  carries only a directory tree and a knob roster with no per-member prose; the
  `cargo` note is under this section, at the roster's constrained-members list).
  Gains the fourth field, its emptiness rule, the closed audience value set, and
  `tool_floor_consumer_side` (deltas 1, 4). The `cargo` note — already the place
  *a contributor-side floor, never a runtime one* is ruled, along with the
  two-tier commit-time reading and the observation that a consumer tree receives
  a prebuilt binary and never a crate — becomes the grammar's rationale rather
  than an unenforced aside: it gains the sentence that the reading is now
  declared on the element and read by name (delta 2). *(Owns deltas 1, 2, 4.)*
- **`installer/README.md` §doctor** — describes the verdict as the toolchain
  contract. Updated to say which toolchain: the consumer-audience subset, and
  that `DOCTOR: clean` is a claim about the machine as a consumer.
  *(Owns delta 3.)*
- **`installer/README.md` §init** — describes doctor as the last precondition
  and the refusal it produces, and **already contradicts itself**: it asserts
  both that `init` "asks for no toolchain and cannot fail on one" and that a
  below-contract toolchain blocks before any partial install. The first claim is
  the one this amendment makes true; the section is rewritten so the two
  sentences state one contract — no toolchain is asked for beyond what the
  vendored battery itself runs, and *that* is what blocks. *(Owns delta 3.)*
- **`installer/README.md` §The consumer smoke** — owns what the smoke asserts and
  its one knob. Gains the toolchain-free arm, states that masking is per-arm and
  the host preflight is unchanged, and records that the mask mechanism is the
  existing Node-free one rather than a new facility. *(Owns delta 6.)*
- **`docs/install.md`** — the toolchain marker block's cargo bullet gains the
  audience token; the surrounding prose that explains the render's fields gains
  the new one. *(Owns deltas 5, 7.)*
- **`docs/site-architecture.md` §Generated projections** — the install-page
  toolchain block is a rostered generated projection; its element grammar
  changed, so its entry is checked and updated where it spells the grammar.
  *(Owns delta 5.)*

No section is listed here that no delta claims.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The regression oracle is red before it is green** — delta 6's smoke case
      is confirmed to fail against the pre-fix `doctor.sh` and to pass after,
      because a case that never saw the defect is a case that cannot hold it out.
