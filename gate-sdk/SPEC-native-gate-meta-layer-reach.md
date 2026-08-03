# SPEC amendment: native-gate-meta-layer-reach

Queue entry: **`native-gate-meta-layer-reach`**, narrowed to the
`check-reads-couples` half alone; the tamper half is
**`gate-tamper-roster-native-reach`** and stays deferred. Companion to
**`native-gate-vendoring-model`** (how a compiled gate arrives) and
**`gate-payload-disclosure-ruling`** (what it discloses). This one gives the
meta-layer a binary-side reach for the one assertion a port cannot carry.

The iteration's envelope is **unblock, not port**. This amendment builds the
mechanism and proves it against the crate's reference-only implementation and a
hermetic fixture. It lands no `.gate` member.

## The wall, stated correctly

`check-reads-couples` refuses — exit 2 — on any registry member resolving to a
`.gate`, unconditionally (`gate-sdk/checks/check-reads-couples.sh`:105-115), and
says so in its own help: *"There is deliberately no descriptor-level exemption: a
port that could opt out of this in a sentence would end the assertion it must
replace."* The opt-out the live port briefly shipped was removed with the port.

So this is a hard wall, not a leaky one. Nothing accumulates silently behind it;
a second port simply cannot happen. The refusal is right — the shell walk parser
finds zero walks in a binary gate and would print `clean`, which is the worst
vacuity available at the seam — and the way past it is to give the gate an
answer, never to give the port an exemption.

## What changes

### 1. The substrate answers what it reads {design-bearing}

`native/src/main.rs` gains a `--reads <name>` arm beside its `--list` arm
(`main.rs`:21-26), on the same shape: a flag the multi-call binary handles before
subcommand lookup.

Output grammar, one line per walk root:

- a repo-relative directory path, for a root the gate declares; or
- a single `?`, for a root the gate cannot declare statically.

Nothing else — no count line, no header. The count is derivable from the lines,
and a transcribed total would be a second source for it.

An unknown name exits 2 with the existing no-such-subcommand help
(`main.rs`:30-40), so a descriptor naming a subcommand the binary does not carry
cannot read as "reads nothing".

### 2. Declared roots are registry data, verified against executed behavior {design-bearing}

A read-set the implementation merely asserts about itself is the unbound
self-declaration §check-reads-couples already refused. What makes this one
different is that it is **held to what the code does**, by two crate-side
assertions.

- `native/src/gates/mod.rs`'s `REGISTRY` gains a roots field per member
  (`&'static [&'static str]`), beside the name and the function pointer it
  already carries. `--reads` prints it; `--list` is unchanged.
- `native/src/walk.rs` becomes the crate's **only** sanctioned filesystem walk,
  and gains a recording mode that captures the roots it is invoked with.
- **Unit test A — the declaration covers what runs.** Each registry member is run
  over its own `gate-tests/<name>/{good,bad}/` case dirs with recording on, and
  the observed root set must be a subset of the declared one.
- **Unit test B — the recorder cannot be bypassed.** No module outside `walk.rs`
  names a filesystem-walk API (`read_dir`, `ReadDir`, and any vendored walker),
  because a direct walk would be invisible to the recorder and unverify test A.
  The precedent is the crate's existing unit test that reads
  `gate-sdk/lib/gate.sh` to hold a literal against drift (`native/src/walk.rs`:69)
  and the §Meta-gate conservation `check-knob-default-coupling` row that
  established executed assertions as the answer where a static gate would be
  vacuous.

**The honest limit, stated rather than discovered.** Test A's coverage is the
fixture corpus. That is bounded by a contract rather than by luck — every ported
gate carries a `good/`+`bad/` pair (§Fixture-pair discipline), enforced by
`check-gate-fixture-coverage` — but a walk reachable only on an input no case
exercises is observed by nothing, and the declaration for it rests on the author.
The `?` line exists for exactly that case, and a gate whose author cannot bound
a root declares `?` rather than guessing.

**Where that verification runs, restated under the trajectory pivot.** Both unit
tests are `cargo test`, so they run in this repo and in CI and **never in a
consumer tree** — under **`native-gate-vendoring-model`** a consumer installs a
prebuilt binary and receives no crate source, so there is nothing there to run
them against and nothing there to edit. That is not a weakening: it makes the
division explicit. The declaration is held to executed behaviour **upstream**,
and the consumer's own independent check is the fixture pair, which
**`gate-payload-disclosure-ruling`** rules shipping-side for exactly this reason.

### 3. `check-reads-couples` consumes the report instead of refusing {design-bearing}

For a member resolving to a `.gate`, the gate invokes
`"$GATE_SDK_NATIVE_BIN" --reads "<name>"` and then runs its **existing** logic
unchanged: for each resolvable root, enumerate the tracked files under it and
assert each matches at least one expanded couple under the manifest's own glob
semantics. A `?` line increments the same skipped-and-counted honesty counter the
shell arm already reports in its clean line.

Nothing about the coverage assertion changes. What changes is only where the
walk roots come from — a shell parse for a `.sh` member, the substrate's own
report for a `.gate` one.

The refusal survives, narrowed to the two cases where the gate still cannot see:

- the binary is absent or not executable while a `.gate` member is registered —
  exit 2, the same fail-closed shape `check-gate-substrate-parity` assertion B
  uses for the same condition;
- `--reads` exits non-zero for a declared member — exit 2.

**There is still no descriptor-level exemption**, and the help text saying so
stays. A port ends the assertion by answering it, never by opting out of it.

Its `# graph:` `couples=` gains the crate's source glob, so an edit to a gate's
implementation re-fires the gate that checks its reads. Without that the
assertion would be live and unreachable — coupled to shell sources only, on a
rule whose subject moved. The couple fires **in this repo**, where the
implementation is tracked; in a consumer tree the crate source is absent by
ruling, so there is no edit for it to catch and a glob matching nothing is the
correct outcome rather than a hole.

**This delta invokes a binary and does not care where it came from.** Nothing
here assumes a locally built one: the invocation is through
`GATE_SDK_NATIVE_BIN`, the same knob every other binary reader uses, and a
prebuilt artifact placed by the installer answers `--reads` identically to one
`cargo` produced. The pivot from build-from-source to prebuilt-per-platform
leaves this amendment's mechanism untouched.

### 4. The read-set does not live in the descriptor, and that refusal is the design {design-bearing}

A `# reads:` line in the `.gate` descriptor is the obvious cheap shape and it is
refused. Nothing would hold it to the implementation: it is the removed
`# reads-couples-exempt:` opt-out with more words, and the descriptor's own rule
is that it carries no field that lacks a reader (§The `# graph:` manifest) —
here the would-be reader could not verify what it read.

**The interface is substrate-neutral; its placement is not settled here.** The
contract is *"a declaration path's substrate answers what it reads"* — shell
answers by parse, the binary answers by `--reads`. Whether a later authoring SDK
relocates that contract to a substrate-neutral surface stays
**`gate-authoring-sdk-surface`**'s question, narrowed here and deliberately not
answered: this amendment adds one substrate's answer without adding a shape that
would have to be unbuilt to generalize.

### 5. The conservation table and the prerequisite are restated {mechanical}

§Meta-gate conservation for the binary substrate's `check-reads-couples` row
states the binary-side equivalent as the disposition, in place of the
refuse-until-one-exists text. §Porting a gate to the binary substrate's
second-port-prerequisite paragraph records this half as satisfied.

### 6. The mechanism is proved without a port {design-bearing}

`gate-sdk/gate-tests/check-reads-couples.test.sh` already drives the refusal on a
bare descriptor and on one claiming the removed exemption, alongside the shell
arm so the refusal cannot pass for a parse failure. It gains the consumption
path: a descriptor whose reported roots are covered by its couples (clean), one
whose roots are not (a finding), one reporting `?` (counted, clean), and the two
surviving refusals. Hermetic, driven against a stub binary and against the real
one, and needing no `.gate` member in any kit.

The crate's `check-action-pinning` **reference-only** disposition gains a second
stated justification: it is the one member unit tests A and B can assert over, so
retiring it would leave both green over an empty registry — the vacuity the
disposition table exists to refuse.

## Producers and consumers

**`--reads` report (new interface).**
Producer: `native/src/main.rs`'s `--reads` arm, reading `REGISTRY`'s roots field.
Reachable at every `check-reads-couples` run against a `.gate` member, and in
this repo at every `run-gate-tests.sh` execution of the new cases. Its enabling
config is `GATE_SDK_NATIVE_BIN`, already live at its default and already the
knob every other binary reader uses — no new configuration is introduced, so
there is no path where the producer exists and nothing is set to reach it.
Consumer: `gate-sdk/checks/check-reads-couples.sh`, by process invocation, at the
point it today refuses.

**Its two line kinds each have a named reader at a named transition.** A path
line is read by the gate's tracked-file enumeration, at the per-root coverage
loop. A `?` line is read by the gate's skip counter, at the clean-line
parenthetical — the same reader the shell arm's unresolvable roots already have.
There is no third line kind, because there is no third reader.

**Roots field on `REGISTRY` (new state).**
Producer: each gate module's declaration, written when the gate is authored.
Consumers: the `--reads` arm (above) and unit test A, which compares it against
the recorder's observation. A member added without roots fails to compile, so the
field cannot be silently omitted.

**Recording mode on `walk.rs` (new state, test-scoped).**
Producer: `walk.rs`'s walk entry point when recording is on. Consumer: unit test
A, at the assertion. It is deliberately test-scoped — a production recorder would
be state with no reader.

**Existing integration this changes.** `check-reads-couples`'s refusal arm is a
live, specified behavior with a reader — §check-reads-couples' own text and
§Meta-gate conservation's row, both updated here. The gate's `couples=` change is
a producer-side edit whose consumer is `check-graph`'s coupling projection and the
generated pre-commit hook, both of which regenerate from the manifest.

## Existing sections updated

Owned by this amendment, each named with the delta that claims it:

- gate-sdk/SPEC.md §check-reads-couples — the refusal arm becomes the
  consumption arm plus two narrowed refusals, describing the `--reads` report
  it consumes; the no-exemption paragraph stays and the counted-zero paragraph
  is restated against the new cases (deltas 1, 3, 4, 6).
- gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate — the
  `check-reads-couples` row states the binary-side equivalent: the `--reads`
  interface and the registry roots field held to executed behavior by unit
  tests A and B; and the reference-only disposition's second justification
  (deltas 1, 2, 5, 6).
- gate-sdk/SPEC.md §Porting a gate to the binary substrate — the second-port
  prerequisite (delta 5).
- gate-sdk/SPEC.md §The `# graph:` manifest — the descriptor's field roster is
  restated as closed, with the refused `# reads:` field named as refused rather
  than absent (delta 4).
- gate-sdk/SPEC.md §check-gate-substrate-parity — assertion D's manifest-class
  partition is unchanged and stays unchanged deliberately: a roots declaration is
  implementation data, not manifest-class, so it belongs in the crate and not in
  the descriptor (delta 4).

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
- [ ] **No port lands.** No `.gate` member is added to any kit; the mechanism is
      proved on the reference-only implementation and the hermetic fixture.
