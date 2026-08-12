# SPEC amendment: crate-arms

A new gate, `check-crate-arms`, runs the Rust crate's **lint and test arms** at
commit time. Today `.github/workflows/gates.yml` runs them and `scripts/gates.list`
does not, so a contributor who runs the full battery **plus**
`gate-sdk/bin/build-native.sh` has satisfied every documented commit-time
obligation and can still push a red CI. This amendment designs the gate and the
deduplication that comes with it. It does not restate the output contract
(§Output contract), the fixture-pair discipline (§Fixture-pair discipline), or
the `# graph:` grammar (§The `# graph:` manifest).

## The queue entry's prescribed shape names a mechanism this tree does not have

The entry's deliverable reads *"degrading to a declared skip where cargo is
absent — the same omit-and-declare shape criterion 5 rules for a missing
artifact"*. Probed at HEAD, both halves of that sentence are wrong, and the
correction is what makes the gate simpler rather than harder.

**No gate in this tree skips at runtime on a missing program.** Both existing
cases fail closed, in identical words:
`gate-sdk/checks/check-shellcheck.sh:27-32` and
`gate-sdk/checks/check-action-run-shell.sh:17-22` each print *"A gate that cannot
run is not clean (fail-closed)"* and exit 2.
`gate-sdk/bin/build-native.sh:19-24` does the same for `cargo`, under a comment
fixing the rule for this exact program: *"an absent toolchain is exit 2 naming
the floor, never a bare command-not-found from the shell."*

**Criterion 5 is not a runtime branch at all.** §The port-candidate criteria
omits a member from the *consumer's* `gates.list` at vendor time, when the
release published no binary for that host's target — a decision `init` makes
once, before any gate runs. Reading it as licence for a gate to branch on
`command -v` inside its own body imports a mechanism the criterion never
describes.

**The repair, and it removes work rather than adding it: the gate's predicate is
the crate's presence, not cargo's.** `native/` is never vendored — §Meta-gate
conservation for the binary substrate states it outright (*"`native/` ships no
`checks/` and no `smoke/`, so it is not a kit root and no consumer ever receives
the crate source — there is nothing there to run them against"*). So in a
consumer tree what is absent is the **corpus**, not the toolchain, and a gate
with no corpus is clean rather than skipped. That is a shape this tree already
has: `check-gate-binary-fresh` reports clean at exit 0 when no registered member
dispatches to a descriptor (`gate-sdk/checks/check-gate-binary-fresh.sh:57-59`),
rather than declaring a skip.

The two branches therefore separate cleanly, and neither needs a new vocabulary:

- **No `$CRATE/Cargo.toml`** — nothing to lint or test. Clean, exit 0, the
  parenthetical naming the absent crate so the green is legible.
- **Crate present, `cargo` absent** — a contributor holding the crate without the
  toolchain. Fail closed, exit 2, the same message shape `build-native.sh`
  already uses. This costs no adopter anything, because an adopter never reaches
  it, and it is already this tree's standing commitment: CLAUDE.md makes
  `build-native.sh` a commit-time requirement, so a session committing here needs
  cargo before this gate is reached.

## What changes

**(1) A new gate-sdk gate, `check-crate-arms`.** [design-bearing]
It resolves the crate through `gate_native_crate` (`gate-sdk/lib/gate.sh:190-194`,
the `GATE_SDK_NATIVE_CRATE` knob, default `native`) and runs the two arms over it.
Design-bearing rather than mechanical: the predicate ruling above, the tier
ruling below, and the fixture design are each a decision, not a transcription.
Its `# graph:` manifest:

```
# graph: couples=native/Cargo.toml,native/build.rs,native/src/*.rs,native/src/gates/*.rs dir=one valve=none tier=precommit
```

`dir=one` because the gate reads the crate and nothing in the crate reads the
gate. The couples list is the crate's tracked source, which is what makes an edit
to a ported gate's module re-run the arms that would have caught it.

**(2) The command is byte-identical to the workflow's, and the duplicate is
deleted rather than kept in step.** [design-bearing] This is the delta that
actually closes the defect. A gate running *approximately* what CI runs leaves the
same hole one flag narrower: a green battery that still permits a red CI. So the
gate runs exactly

```
cargo clippy --release --manifest-path "$CRATE/Cargo.toml" --all-targets -- -D warnings
cargo test --release --manifest-path "$CRATE/Cargo.toml"
```

— `--all-targets` included, matching `.github/workflows/gates.yml:37-42` — and
that workflow's *"native crate lint + unit tests"* step is then **removed**, since
the workflow already runs the battery and the battery now carries the member.
Enforcement-first and SSOT both point the same way here: two copies of a command
held equal by nobody is the defect this unit exists to close, so the fix removes
the duplication rather than gating it. The workflow's preceding *"build the native
gate binary"* step stays — `check-gate-binary-fresh` still needs the artifact, and
that step is not a copy of anything.

**(3) Tier is `precommit`, and the cost question is settled by measurement with
its fallback named.** [design-bearing] The tier discriminator is whether the
invariant is *restorable within the single commit that perturbs it*
(§The `# graph:` manifest): a clippy finding or a failing unit test is introduced
and repaired inside one commit, so `precommit` is the correct tier by the rule,
and it is what the queue entry asks for. The honest counter-pressure is wall
clock — `--all-targets` compiles the test targets, and clippy keeps a cache
separate from the `cargo build --release` that `build-native.sh` already warms.
The build session **measures** the added battery time on a warm tree and records
it. If it exceeds the battery's current dominant member, the fallback is
`align-only`, ruled here so the measurement does not reopen a design round: an
arm that runs in the full battery and in CI still closes the attested defect,
which was a *push* on a green battery, and only the pre-commit convenience is
lost.

**(4) Fixture pair, and the hermeticity problem it raises.** [design-bearing]
The pair lives at `gate-sdk/gate-tests/check-crate-arms/{good,bad}/`: two
minimal crates, `good/` clean, `bad/` carrying one clippy finding and one failing
test, each pointed at by its `args` file through `GATE_SDK_NATIVE_CRATE`. Both
build with **no network**, because the crate under test vendors nothing — the
property the real crate's own test asserts (`native/src/walk.rs:412-438`, the
`[dependencies]` section is empty) and the fixtures inherit by construction.
The problem to design around is that cargo writes a `target/` directory beside
whatever manifest it is given, which would have a fixture run writing in-tree and
`check-test-hermetic` reddening. The gate therefore honors a
`GATE_SDK_CARGO_TARGET_DIR` knob, defaulting to the crate's own `target/` (already
gitignored, and the warm cache `build-native.sh` fills), with the fixture `args`
overriding it into the scratch dir. Defaulting it to scratch instead is refused:
it would cold-build the real crate on every battery run, buying hermeticity the
real tree does not need at a cost the real tree pays every time.

**(5) Registration and the new-gate fan-out.** [mechanical]
`check-crate-arms` joins `scripts/gates.list`, gate-sdk's README gate-roster
block, and a `### check-crate-arms` section in gate-sdk/SPEC.md. Because it is
`tier=precommit` it lands in the generated hook. The full regen set and its
ordering constraint — the footprint and value rollups regenerate **after** the new
gate file is `git add`ed, never before — are docs/site-architecture.md §Generated
projections' to state and this delta's to execute.

**(6) CLAUDE.md's one-directional sentence is completed.** [mechanical]
It states the asymmetry one way only (*"`cargo test` does not discharge
`build-native.sh`"*) and nothing states the other, which is half of what let the
attested defect through. With the gate registered the correct sentence is shorter,
not longer: the battery now runs both arms, so the obligation is the battery plus
`build-native.sh`, and neither discharges the other.

**(7) The gate stays shell, permanently, and the residue is justified rather than
assumed.** [design-bearing] TRAJECTORY.md §PRIORITY DIRECTIVE makes surviving
shell *"residue justified case by case, never a protected category"*, so this owes
an argument and gets one on two independent grounds. First, criterion 7 in its
plainest form: the rule *is* an invocation of `cargo`, a program the payload does
not carry and objective 1 exists to keep off the floor. Second, and the stronger
one, criterion 4 at its purest: a gate that runs `cargo test` over the crate
cannot live **inside** the crate it tests, because the artifact under test would
be the artifact asserting. No later cohort should read this member as unported
work; it is a member the port does not take.

## Producers and consumers

**New interface: the `check-crate-arms` gate and its `gates.list` registration.**
- *Producer* — the battery (`gate-sdk/bin/run-gates.sh`) and the generated
  pre-commit hook, which gains a trigger block for the couples above.
  Its enabling config is emitted everywhere it must be: `GATE_SDK_NATIVE_CRATE`
  carries a kit default (`gate-sdk/lib/gate.sh:190-194`), so no consumer sets
  anything, and the branch a consumer actually takes is delta (1)'s
  absent-crate clean.
- *Consumers* — `check-graph`, `gen-pre-commit.sh`, `enforcement-map.sh` and
  `footprint.sh` read the `# graph:` manifest; `check-spec-pointer` reads its
  `# spec:` line; `check-install-disposition` reads its `# install:` line;
  `check-gate-output` reads its `: clean` and `help:` emissions;
  `check-readme-roster` reads its name in both directions;
  `check-gate-fixture-coverage` reads its fixture pair. No new field is added to
  any of those rosters — the gate is an ordinary member.

**New knob: `GATE_SDK_CARGO_TARGET_DIR`.**
- *Producer* — the consumer's gate-sdk config, or the kit default (the crate's
  own `target/`).
- *Consumer* — `check-crate-arms` alone, at invocation, passed to both cargo
  calls as `--target-dir`. Its one non-default reader is the fixture pair's
  `args`, which is what keeps a fixture run out of the tree.
- *Named reader at a named transition* — read once, when the gate builds each
  cargo argv; there is no other call site, and if the fixture design in delta (4)
  were dropped the knob would be removed with it rather than kept unread.

**Red conditions of the readers this change touches.** The delta **widens** two
corpora (a new gate file, a new SPEC section) and **narrows** one (the deleted
workflow step), so the narrowing side is the one §The causal-completeness check
point 5 binds on:

- `check-gate-output` — red on a **zero count** of a `: clean` / `help:` line in
  the member's source. Non-monotone, but this delta only adds a member, so it is
  satisfied by the gate carrying both emissions rather than by inspection of a
  narrowing.
- `check-gate-fixture-coverage` — red on a member with **neither** a `good/`+`bad/`
  pair **nor** a `# no-fixture:` opt-out: a zero-count reader, and delta (4) is
  what clears it. The opt-out is deliberately not taken — the state under test has
  a static representation, which is exactly the condition that makes a pair
  available.
- `check-readme-roster` — red in **both** directions, so gate-sdk's README gains
  the name in the same commit or reds.
- `check-graph` / `check-enforcement-fresh` / `check-footprint-fresh` /
  `check-value-rollup-fresh` — each red on a **stale projection**, which a new
  registered gate makes stale by construction; delta (5) is the discharge, and its
  ordering constraint is why the regen follows the `git add`.
- `check-action-pinning`, `check-action-gh-repo`, `check-action-run-shell` — the
  readers of the workflow file delta (2) edits. Deleting a `run:` step **narrows**
  `check-action-run-shell`'s extracted-block corpus, and its verdict is monotone
  in the violation set (it reds per offending block, never on finding none), so it
  is clearable by inspection. The other two read `uses:` refs and job-level `gh`
  context, neither of which the deleted step carries.
- `check-docs-cmd` — red on a governed doc fencing a command that does not run.
  The new gate's SPEC section and any doc naming the arms must fence a real
  invocation; this is real signal, not a formality.
- `check-manifest-count`, `check-prose-enum` — readers of the CLAUDE.md and
  gate-sdk/SPEC.md prose delta (5) and (6) touch. `check-prose-enum` reds when a
  hand list omits a declared set member, so any prose enumerating the gate roster
  gains the new name or reds; that is the intended catch.
- `check-gate-substrate-parity` assertion C — red on a substrate-sensitive member
  with no disposition row. **This gate is not substrate-sensitive**: the
  derivation tests whether a member's expanded `couples=` covers *a registry
  member's declaration path*, and `native/src/gates/*.rs` is an implementation
  module, never a declaration path. Stated rather than left silent, so a later
  reader does not add a conservation-table row the derivation never asked for.

## Existing sections updated

- **gate-sdk/SPEC.md, new `### check-crate-arms`** — owned by deltas (1), (3) and
  (7): the invariant, the two-branch predicate, the tier ruling and its
  measurement fallback, the calibration (why the crate's presence and not cargo's),
  and the permanent-shell justification.
- **gate-sdk/SPEC.md §The port-candidate criteria** — owned by delta (7). Criterion
  7's worked example is `check-action-run-shell`, named as *"the largest named
  piece of port work"*. `check-crate-arms` is a different case in the same
  criterion — a member the port does not take at all, because criterion 4 forbids
  it independently — and recording it keeps the roster's opening claim ("never an
  eligibility screen") true without implying this member is owed work.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — owned by
  the last red-condition bullet: a sentence recording that this member is outside
  assertion C's derivation, so its absence from the table is a verdict rather than
  an omission.
- **gate-sdk/README.md** — the `<!-- gate-roster:begin -->` block gains the name.
- **CLAUDE.md §Housekeeping** — owned by delta (6), the `native/` bullet's
  commit-time obligation sentence.
- **.github/workflows/gates.yml** — owned by delta (2): the lint-and-test step
  removed, the build step retained.
- **scripts/gates.list** — owned by delta (5).

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
