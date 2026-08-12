# SPEC amendment: parity-vendor-scope

`check-gate-substrate-parity` assertion B equates a **whole-binary** subcommand
roster against a **partially-vendored** descriptor set, so the equality is
unsatisfiable in any consumer that vendors a subset of the kits the shared binary
carries. Discharges `substrate-parity-partial-vendor-scope`.

**The mechanism, probed directly at HEAD rather than taken from the entry.**
`checkwright-gates --list` reports 18 subcommands spanning three kits — gate-sdk
2, canon-kit 8, queue-kit 8 — and the descriptor set on disk matches it exactly.
A consumer vendoring gate-sdk alone receives the same binary and two descriptors,
so assertion B's roster half reports sixteen subcommands nothing declares. The
default consumer-smoke run vendors every kit and is green, which is why
`.workflow/validate-baseline.txt` holds `consumer_smoke pass` while the subset
invocation reds — the two are consistent, and the baseline is not stale.

**It grows with the port.** Every cohort that lands moves more subcommands into
the binary while a subset-vendoring consumer's descriptor set stays where it is,
so the gap widens by the size of each cohort — including
`SPEC-kit-roots.md`'s five, four of which are registered in gate-sdk's own smoke
install and therefore *dispatch* in exactly the scratch tree that reds.

## What changes

### (1) The alternative is refused first, because it is the expensive one to discover late — **design-bearing**

The entry names a second candidate: build the binary **per-vendoring** rather than
shipping it whole. It is refused, and not on taste. Criterion 5's install model is
already closed and ruled (§Porting a gate to the binary substrate, criterion 5;
TRAJECTORY.md): *the payload carries a prebuilt binary **per declared target**,
built by the release and never from a working tree*. A per-vendoring binary makes
the artifact set the product of targets and kit subsets — every combination of the
kits an adopter might choose — which no release can enumerate and no digest roster
can own, and it re-imports the build-time coupling the reverted port removed. The
refusal is recorded here rather than left implicit because the option reads
attractive at first glance and costs a session to re-cost.

### (2) Assertion B's roster half restricts to the vendored kits — **design-bearing**

The equality was always meant to catch a **stranded implementation** — a
subcommand no descriptor dispatches to, which is dead code or the residue of a
half-finished port. In a subset vendoring, a subcommand belonging to a kit the
consumer never took is neither: it is out of scope, and the assertion has no
business speaking for it.

The roster half becomes: **for each subcommand, if its owning kit is present in
`gate_kit_roots`, a descriptor for it must exist**; a subcommand whose owning kit
is absent is **out of scope, counted, and declared on the clean line**. Three
properties are held deliberately:

- **The other direction stays unrestricted.** A `.gate` descriptor the resolve
  dirs carry with no subcommand behind it is red regardless of kit — that
  direction is what catches a gate that cannot run, and a vendored descriptor is
  by definition in scope.
- **The `reference-only` allowance is untouched**, and it now composes: an
  in-scope subcommand with no descriptor is still checked against the conservation
  section exactly as today.
- **The half does not go dark.** The roster half runs whenever the binary is
  *readable* — never gated on descriptor count or on the registry — which is the
  correction the reverted port paid for and which this amendment must not undo.
  Out-of-scope subcommands are counted and printed, so an emptied scope is visible
  rather than silent, in the shape the zero-descriptor clean line already uses.

### (3) The owner is registry data held to executed behavior — **design-bearing**

The binary must say which kit owns each subcommand, because the descriptor that
would have said so is precisely what a subset vendoring lacks.

**`--list` gains a second, tab-separated column: `<subcommand>\t<owning-kit>`**,
the kit's directory basename as it appears under `gate_kit_roots`. The declaration
lives in the crate's dispatch registry beside each member's walk roots and knobs —
an entry a member cannot compile without — so it is the same
*registry-data-held-to-executed-behavior* shape as `--reads` and `--knobs`, not an
unbound self-declaration. A crate unit test holds it to the tree: for every
registered subcommand, `<owner>/checks/<name>.gate` exists. That test is
`cargo test`, so it runs here and in CI and **never in a consumer tree**, which is
the same division §Meta-gate conservation states for the `--reads` tests and is
recorded rather than discovered.

**A second column rather than a fifth flag, and the reason is version skew.** The
gate ships in a kit and the binary ships in the payload, and the two version
independently — `init` places a released binary while a consumer may vendor a
newer kit, and the upgrade smoke drives exactly that across two hops. A fifth
top-level flag an older binary does not recognise answers non-zero, which
§Fail-closed contract makes exit 2: every such consumer's battery would die on a
flag rather than on a finding. A second column degrades instead of failing — an
older binary prints one column, the gate reads no owner, and **the assertion falls
back to today's unrestricted equality**, declaring on its clean line that the
restriction was unavailable. That is a return to the current behavior, never a
false green, and it is the honest verdict for a tree whose binary cannot answer
the question.

The one-column fallback is a **stated residual**: a real adopter on a subset
vendoring with a pre-column binary still reds until the binary is upgraded. It
does not reach the consumer smoke, which builds the binary from the crate on every
run, and it is bounded by a version rather than open-ended.

The top-level flag roster (`--list`, `--reads`, `--knobs`, `--source-stamp`) is
unchanged; §check-gate-substrate-parity's note that those flags are outside the
roster by construction still holds, and the second column is roster **data**, not
a new roster member.

### (4) The configuration gets an oracle at commit time — **mechanical**

The second, separable finding in the entry: nothing at commit time or in CI runs
the subset invocation, which is why the red survived two iterations unnoticed.
The answer is **not** a second consumer-smoke leg — that suite vendors, installs
and drives a whole scratch tree, and buying it twice to reach one assertion is the
cost the fixture split exists to avoid.

Instead the configuration joins the bespoke
`gate-sdk/gate-tests/check-gate-substrate-parity.test.sh`, which already holds the
descriptor configurations a single fixture pair cannot: a sandbox carrying
**descriptors for one kit and a fake binary reporting two kits' subcommands**, in
which the gate must run clean, and its near miss — the same sandbox where the
in-scope kit is missing a descriptor — which must still red. That test is in the
per-kit fixture-runner battery, so the assertion acquires an oracle that runs on
every commit at no measurable cost.

The test's existing fake binaries print a bare name per line; they gain the owner
column, and one keeps the single-column spelling so delta (3)'s fallback is
exercised rather than asserted.

### (5) The shared root with `consumer-smoke-subset-accounting-verdict` is named, not merged — **design-bearing**

That entry is the **registration accounting** reddening under a kit subset; this
is **substrate parity**. Different assertion, different fix, one shared root: a
consumer-smoke assertion equating a whole-roster fact in a subset-vendored tree.
Weighed and **not unified**, because the two derive scope from different things —
this one from a subcommand's owning kit as the binary reports it, that one from
the vendored kits' `checks/` directories, which it already reads. A shared rule
would have to be parameterised over both and would be a third thing to keep true.
What transfers is the **predicate**, stated once here so the sibling can cite it:
*an assertion over a whole-roster fact states its scope in terms of what the tree
vendored.* Recorded so a later session weighing unification finds it costed rather
than re-costing it.

## Producers and consumers

One new field on one existing interface, and one new consumer of it.

- **New field: `--list`'s owning-kit column.**
  - **Producer** — the crate's dispatch registry (`native/src/gates/mod.rs`),
    emitted by `main.rs`'s `--list` arm. Its enabling config is none: `--list` is
    a top-level flag with no knob and no bridge, reached on every invocation of
    the built binary, so the producer is live in the real configuration rather
    than under test.
  - **Consumer, and the transition where it is read** — `check-gate-substrate-parity.sh`,
    assertion B's roster half, at the point it iterates the subcommand list; it
    reads the column to decide whether a subcommand is in scope, and it reads the
    tree's `gate_kit_roots` (already derived there for `RESOLVE_DIRS`) to decide
    whether that kit is vendored.
  - **The field has exactly one reader and it is named above.** No other caller of
    `--list` exists: the only two consumers in the tree are this gate and the
    bespoke test's fake binaries, which are producers of it. The field is not
    added to `--reads`, `--knobs` or `--source-stamp`, which answer per-gate
    questions and have no roster to scope.
  - **Held to executed behavior** by the crate unit test in delta (3) — the
    declaration is not trusted on its own, which is what separates it from the
    self-declaration `check-reads-couples` exists to refuse.

**Each reader's red condition, enumerated — this delta NARROWS what assertion B
speaks for**, and a narrowing is where clearing by inspection is wrong (§The
causal-completeness check, point 5):

- **Assertion B's roster half** reds on an in-scope subcommand with no descriptor
  and no `reference-only` disposition. Narrowing the in-scope set can only remove
  findings, which is the intent — but its **clean line counts**, and a count that
  silently drops to zero is the vacuity this gate exists to refuse. So the
  out-of-scope count is printed beside the in-scope one, and the fallback state is
  printed as its own word. Not monotone, therefore checked: the new bespoke case
  and its near miss are what check it.
- **Assertion B's descriptor half** is untouched and still reds on a descriptor
  naming no subcommand — verified rather than assumed, because the obvious
  implementation restricts one loop and accidentally restricts both.
- **Assertions A, C, D, E, F** read no part of `--list` and are untouched. E's
  descriptor-set derivation is from disk, not from the roster, and stays so.
- **`check-gate-binary-fresh`** reads declaration paths as a set to decide whether
  the binary is load-bearing; it does not read `--list` and takes no change.
- **`run-gate-tests.sh`** dispatches the bespoke test substrate-blind; the added
  case is a sandbox with a fake binary, so it introduces no dependency on a built
  artifact — the property that keeps the bespoke test runnable in a tree with no
  crate.
- **`check-gate-output`** reds on a member whose declaration lacks the `: clean` /
  `help:` strings. `check-gate-substrate-parity` **stays a shell gate** — a gate
  that audits the port may not consume the substrate it audits — so its
  declaration path still holds both strings and the clean-line rewrite must keep
  them.

## Existing sections updated

- **gate-sdk/SPEC.md §check-gate-substrate-parity** — assertion B's roster half
  gains the scope rule, the owner column, the fallback and its residual (deltas
  2–3); the "why those configurations are held in fixtures" paragraph gains the
  new bespoke case (delta 4); the clean-line description gains the two counts and
  the fallback word.
- **gate-sdk/SPEC.md §Consumer smoke** — the subset invocation is recorded as a
  configuration the bespoke test now covers, so a later reader does not read the
  smoke's silence on it as coverage.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — the
  `check-gate-substrate-parity` row's *retained by construction* disposition is
  unchanged and verified so; the new crate unit test is named beside the `--reads`
  tests in the paragraph that states where they run.
- **`native/src/main.rs`'s `--list` arm and `native/src/gates/mod.rs`'s registry**
  are implementation, not spec surface, and are named here only as the delta's
  producer.

No section outside `gate-sdk/SPEC.md` changes. `consumer-smoke-subset-accounting-verdict`
is a queue entry and is left standing; delta (5) is what it gains.

## Definition of Done

- [ ] **Causal completeness** — the one new field has a named producer reachable
      in the real configuration, exactly one named reader at a named transition,
      and a crate unit test holding the declaration to the tree; every reader's
      red condition under the narrowing is enumerated above, and the non-monotone
      one is checked by a new fixture case rather than by inspection.
- [ ] **The subset configuration has an oracle that runs at commit time** — the
      bespoke case and its near miss both in the fixture-runner battery, and the
      near miss reds before the fix and only there.
- [ ] **The fallback is exercised, not asserted** — one fake binary keeps the
      single-column spelling.
- [ ] **Merged with no information lost** — the assertion-B bullet still reads as
      one bullet, not a bullet plus an appendix.
- [ ] **Amendment deleted** — this file removed on merge; the none-remain half is
      discharged at the iteration, since sibling amendments are in flight for
      gate-sdk.
- [ ] **Removals propagated** — grep for callers assuming `--list` is one column.
- [ ] **Gaps filed** — anything found in the crate registry's owner declaration
      filed through `lifecycle-kit/bin/file-gap.sh`.
