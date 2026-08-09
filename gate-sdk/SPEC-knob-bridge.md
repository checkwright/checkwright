# SPEC amendment: knob-bridge

Queue entry: `native-knob-bridge`. **Operator-ruled 2026-08-09**, re-scoping the
port unit after the cohort census was found to have never scored criterion 6:
this unit builds the **array-knob config bridge for the `.gate` dispatch path**
and discharges criterion 6, and **ports zero gates**. That is intended, not a
shortfall — cohort selection is deferred to a later iteration on a scoring that
covers every criterion, and no token port is added to show motion.

The finding that forced the re-scope: a consumer's kit knobs are **bash arrays**
resolved by a shell library that is sourced *in-process by each shell gate*,
after it is already the exec'd process. `gate_command`'s `.gate` branch exec's
`[<binary>, <name>]` directly, so a compiled gate sees none of them. Every
candidate cohort owes this bridge — cohort A at 0 of 10 on criterion 6, the
runner-up at 5 of 7 — which is what makes the bridge the shared substrate work
rather than either cohort's local cost.

## What changes

### Delta 1 — `--knobs <name>`, the binary's declaration of the config it reads

*Work class: **design-bearing***.

The crate registry (`native/src/gates/mod.rs`) gains a fourth tuple element per
member: the knob names that member reads. A member added without it **fails to
compile**, the same construction that makes the `--reads` roots un-omittable.

`checkwright-gates --knobs <name>` prints one knob name per line and exits 0; an
unregistered name is exit 2. It is **registry data held to executed behavior**,
the property the conservation table already blesses for `--reads` — not an
unbound self-declaration.

**This is the fourth top-level flag, and it is outside `--list`'s roster by
construction.** §check-gate-substrate-parity assertion B states that the roster
is over subcommands alone and that "a reader adding a fourth flag needs to know
it is not a parity violation" — this is that reader, and this sentence is the
answer. A flag that leaked into `--list` would red as a stranded implementation.

### Delta 2 — `gate_command` carries the resolved knobs into the invocation

*Work class: **design-bearing***.

`gate_command` **prints argv**; it is called in a command substitution, so it
runs in a subshell and cannot export into its caller. The bridge therefore rides
the argv it already emits: for a `.gate` member declaring knobs, the emitted
argv becomes

```
env
GATE_SDK_KNOB_<NAME>=<tab-joined values>
…
<binary>
<name>
```

one element per line, the existing protocol unchanged. A member declaring no
knob emits the two-element argv exactly as today, so the two live first-cohort
members are unaffected until Delta 3 gives them a declaration.

Resolution, per declared knob:

- The owning kit comes from the knob's own `<KIT>_<KNOB>` prefix — the
  convention CLAUDE.md §Conventions established in gate-sdk already states —
  mapped to a kit root through `gate_kit_roots`. Derivation-first: no roster of
  knob→kit pairs is maintained, so none can rot.
- That kit's `lib/*.sh` is sourced **in a subshell**, so a kit library's globals
  cannot leak into the dispatcher or across members.
- The resolved array is serialized **tab-joined**. Whitespace is preserved
  inside an element, which is exactly why the whitespace-separated scalar shape
  (`GATE_SDK_PRUNE_DIRS`) cannot serve: `CANON_KIT_TEMPORAL_EXEMPT_SECTIONS`
  contains `Out of scope`. A scalar knob is a one-element array; the two cases
  share one grammar.
- **An element containing a tab or a newline is exit 2, naming the knob and the
  element.** A newline would break the line-per-element argv protocol and a tab
  would break the serialization; refusing both is the fail-closed answer, and
  the limit is stated here rather than discovered by a consumer.

The `GATE_SDK_KNOB_` prefix is deliberately **not** the knob's own name: reusing
`CANON_KIT_MANIFEST_FILES` as an env scalar would collide with the existing
whitespace-scalar override convention (`GATE_SDK_PRUNE_DIRS` is exactly such a
knob), and one name meaning two grammars is the defect this prefix avoids.

`env` is an external program in the dispatch path. It is sanctioned here because
the dispatcher is already bash — this adds no dependency any caller did not
have — and because the alternative, a shell wrapper per gate, would add a bash
hop to every invocation and contradict the port's own purpose.

### Delta 3 — `walk.rs` reads the bridged knob; the duplicated default is retired

*Work class: **design-bearing***.

This is what gives the bridge a **live reader today, with zero gates ported** —
without it the bridge would be an interface with no consumer, which the
causal-completeness check removes rather than ships.

`gate-sdk/lib/gate.sh` resolves `GATE_PRUNE_DIRS` as a bash array (the
`GATE_SDK_PRUNE_DIRS` scalar override, else the literal default, then
`GATE_SDK_PRUNE_EXTRA_DIRS` appended). `native/src/walk.rs` today **duplicates
all of it**: a `PRUNE_DIRS_DEFAULT` const, its own re-implementation of the
override-and-append resolution, and a unit test
(`prune_default_equals_the_shell_libraries`) whose whole job is to hold the two
literals equal.

After this delta:

- `PRUNE_DIRS_DEFAULT` is **deleted**. `prune_dirs()` returns a `Result` and
  reads `GATE_SDK_KNOB_GATE_PRUNE_DIRS`, tab-split. The crate carries **no
  default for a bridged knob** — an absent one is an error, never a built-in
  fallback, because a fallback is precisely the second default criterion 6
  exists to refuse.
- The override-and-append resolution disappears from Rust: the shell library
  already performed it, and what crosses the bridge is the **resolved** value.
- `prune_default_equals_the_shell_libraries` is **deleted**. It is a gate over a
  duplication, and enforcement-first ranks removing the duplication above gating
  it. Deleting the test alongside the duplication is the point, not a coverage
  loss.
- Both live members declare `GATE_PRUNE_DIRS` in the registry, so both exercise
  the bridge on every battery run from the day it lands.

The crate's unit tests reach `find_files` without going through `gate_command`
(unit test A runs members in-process; unit test B scans the crate sources), so
they set `GATE_SDK_KNOB_GATE_PRUNE_DIRS` themselves. Named here because a test
that silently kept a default would re-introduce exactly the second source this
delta removes.

### Delta 4 — the generated hook bakes the knob elements, and its trigger widens

*Work class: **design-bearing*** — the delta is small, the reasoning is not.

`gen-pre-commit.sh` resolves argv through `gate_command` at **generation** time
and bakes it into `scripts/git-hooks/pre-commit`. So the `env` elements are
baked too, and a kit-config edit stales the hook.

**That is the established pattern here, not a new hazard.** The hook already
bakes a knob-derived value — the resolved `GATE_SDK_NATIVE_BIN` path sits
literally in `run_gate check-action-pinning native/target/release/…` — and the
hook's currency is held by regeneration plus its freshness gate. The bridge
extends the set of knobs that stale the hook; it does not invent the property.

What must therefore change with it: the hook's staleness **trigger** gains the
kit config files, or a config edit goes uncaught. This is a manifest edit rather
than new mechanism, and it is called out because a bridge that bakes values
without widening the trigger is a silently stale hook — the failure mode worth
more than the delta.

**The alternative weighed and refused: a generated knob projection.** A shell
tool dumping every resolved array into a tracked artifact, byte-compared by a
freshness gate, is the shape this repo already uses for the hook, the graph
artifact and the docs mirror, and it would keep values out of the hook. It is
refused because the fixture runner `cd`s into each case dir, so every one of the
hundred-odd fixture cases would need its own generated projection — a
per-fixture generated artifact is a maintenance surface far larger than the
staleness it avoids. Recorded so a later reader does not re-attempt it blind.

### Delta 5 — criterion 6 is discharged by construction, and glob semantics are committed

*Work class: **design-bearing***.

The ruling framed this as "a parity discharge holding the Rust default to the
shell default". **The bridge makes that unnecessary, and the departure is an
improvement rather than a shortcut**, so it is stated rather than quietly taken:
with the bridge there is exactly **one** place a knob's value is computed — the
kit's shell library — and the binary holds no default to drift. That is stronger
than two defaults held equal by a test, and it is why Delta 3 deletes the test
this repo already had. Criterion 6's qualification is satisfied in its strongest
form: the duplication is not machine-held, it is *absent*.

**Glob semantics, committed as the ruling asked.** The bridge itself transports
strings and interprets nothing — it has no glob matcher, because this unit ports
no corpus derivation. The commitment is for the reader that will interpret them:
the future Rust glob matcher is **`**`-capable (globstar semantics)**, matching
`shopt -s globstar` at `canon-kit/lib/spec.sh:211`. The evidence and its limit:
this consumer's arrays use no `**` today, so plain-glob semantics would pass
here — but the shell side enables `globstar`, so the config surface *permits*
one, and a plain-glob reader would silently mis-scan the first consumer who
writes one. Committing to the wider semantics costs nothing today and closes a
silent-divergence class; the porting unit inherits this commitment rather than
re-deciding it.

## Producers and consumers

**New interface 1 — `--knobs <name>`.** *Producer*: the crate's top-level
dispatch, off registry data that fails to compile when omitted. *Consumer*:
`gate_command`, which calls it for every `.gate` member before emitting argv.
Reachable in the deployed configuration: both live members are registered and
resolve to descriptors today.

**New interface 2 — the `GATE_SDK_KNOB_<NAME>` environment convention.**
*Producer*: `gate_command`, via the `env` elements it emits — and, at one
remove, `gen-pre-commit.sh`, which bakes them. *Consumers*: the crate's knob
reader; the first one is `walk.rs::prune_dirs`, reading
`GATE_SDK_KNOB_GATE_PRUNE_DIRS` (Delta 3). **Every field has a named reader**:
the convention carries exactly one field, the tab-joined value, read by
`prune_dirs` at the walk it performs. No knob is bridged that nothing reads —
the registry declaration is what makes an unread knob impossible, since the
crate declares only what its own code reads.

**Dispatch surfaces that receive the bridge, all three, verified rather than
assumed** — each already routes through `gate_command`, so none needs its own
change beyond Delta 4: `bin/run-gates.sh` (`gate_command` at its dispatch),
`bin/run-gate-tests.sh:32`, and `bin/gen-pre-commit.sh:38`.

**Red conditions of the affected readers.** This delta narrows nothing — no
corpus shrinks, no glob tightens, no file is pruned — so the point-5 narrowing
hazard does not arise. The readers that can nonetheless flip:

- `check-gate-substrate-parity` reds when a descriptor and a subcommand disagree
  (assertion B), when a substrate-sensitive member lacks a disposition
  (assertion C), or when the implementation tree carries a manifest annotation
  (assertion D). `--knobs` is a top-level flag outside the subcommand roster, so
  B is unmoved; no new gate is ported, so C's derived set is unchanged.
- `check-reads-couples` reds on an absent or non-executable binary or a non-zero
  `--reads`. It invokes the binary **without** the bridge, so `--reads` must
  keep working with no `GATE_SDK_KNOB_*` set. It does: `--reads` prints registry
  data and performs no walk, so it never reaches `prune_dirs`. This is the one
  place Delta 3's fail-closed `prune_dirs` could have broken a live reader, and
  it is why the reader is named with its red condition rather than its subject.
- `check-graph` reds when the generated hook diverges from the manifests. Delta
  4 is what keeps it honest; the hook is regenerated in the same unit.
- `check-gate-binary-fresh` reds on a stale binary. Unmoved — its predicate is
  the source stamp, not the invocation shape.
- `check-knob-default-coupling` is **not** a reader here: the conservation table
  records it as deliberately not corpus-extended to Rust, and Delta 3 removes
  the one duplication it could never reach. Its verdict on `lib/gate.sh` is
  unchanged, since the shell default stays exactly where it is.

## Existing sections updated

- **gate-sdk/SPEC.md §lib/gate.sh** — `gate_command`'s argv contract gains the
  `env`-prefixed form, the tab serialization, and the tab/newline refusal.
  Owned by Delta 2.
- **gate-sdk/SPEC.md §Layout and configuration** — the `GATE_SDK_KNOB_<NAME>`
  convention joins the knob roster as a *dispatch-time convention* rather than a
  consumer knob: no consumer sets it, and saying so is what stops it being
  documented as one. Owned by Delta 2.
- **gate-sdk/SPEC.md §check-gate-substrate-parity** — assertion B's
  "roster is over subcommands alone" paragraph names `--knobs` as the fourth
  flag, so the next reader inherits the answer instead of re-deriving it.
  Owned by Delta 1.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — the
  `check-knob-default-coupling` row records that the duplication it was written
  around (the crate's prune default against `lib/gate.sh`'s) is now **absent
  rather than test-held**, and that the executed assertion it cited is deleted
  with it. This row currently cites that unit test as its answer; leaving it
  would leave the table pointing at a mechanism that no longer exists — the
  exact defect the table's own prose calls out. Owned by Delta 3.
- **gate-sdk/SPEC.md §The port-candidate criteria, criterion 6** — records that
  the criterion is discharged by construction for bridged knobs, and carries the
  globstar commitment. Owned by Delta 5.
- **gate-sdk/SPEC.md §gen-pre-commit** — the hook bakes knob elements; the
  staleness trigger includes the kit config files. Owned by Delta 4.
- **gate-sdk/SPEC.md §The `# graph:` manifest** — the fan-out a config edit now
  stales. Owned by Delta 4.
- **docs/site-architecture.md §Generated projections** — the pre-commit hook's
  trigger line gains the kit config files. Owned by Delta 4.
- **native/src/walk.rs's `# spec:` pointer** — currently cites `gate_find`'s
  pruned walk as reading "the same knobs so the two substrates cannot scan
  different trees". After Delta 3 it reads the *resolved* value instead, which
  is a different and stronger claim. Owned by Delta 3.

**Deliberate non-updates, recorded so a reader does not go looking.**
`canon-kit/SPEC.md` is unchanged: no canon-kit gate is ported and no canon-kit
knob is bridged in this unit. `installer/README.md` §The gate binary is
unchanged: the payload ships the same artifact and the bridge lives entirely in
the dispatcher.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Zero gates ported** — `scripts/gates.list` and the descriptor set are
      unchanged; `native/src/gates/mod.rs` registers the same two members. A
      port added here is out of scope by ruling, not by oversight.
- [ ] **The bridge has a live reader** — `walk.rs::prune_dirs` reads the bridged
      value, both live members declare `GATE_PRUNE_DIRS`, and removing the
      `env` elements makes the battery fail rather than silently pass.
- [ ] **The duplication is gone, not gated** — `PRUNE_DIRS_DEFAULT` and
      `prune_default_equals_the_shell_libraries` are both deleted, and no other
      Rust default for a bridged knob replaced them.
- [ ] **The refusals are exercised** — a knob element containing a tab and one
      containing a newline each produce exit 2 naming the knob.
- [ ] **The hook is regenerated and its trigger widened**, and a kit-config edit
      is verified to stale it (`bash gate-sdk/bin/gen-pre-commit.sh --write`,
      then `check-graph`).
- [ ] **Full battery green** (`bash gate-sdk/bin/run-gates.sh`), the gate-sdk
      and canon-kit fixture suites, and `cargo test` for the crate's unit tests.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired (`PRUNE_DIRS_DEFAULT`, the deleted unit test); nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
