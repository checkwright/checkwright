# SPEC amendment: fail-open-set

The front-end exits with one of two statuses when the binary is absent or not
executable, and which one depends on the arm. `gate-sdk/bin/run-gates.sh` sets
`ARM_UNAVAILABLE_STATUS=2` and flips it to `0` for `--hook | --statusline`.
`gate-sdk/bin/run-gates.ps1` holds the same two names as a `-ceq` test that sets
`$unavailable = 0`.

Those two names are the **fail-open set**. gate-sdk/SPEC.md §The harness-integration
arm rules who belongs in it:

- an arm whose exit status gates a user action fails open;
- `--statusline` declines at 0 because the harness discards its status;
- `--usage-poll` keeps 2, because its caller is a timer.

The set is held in three places, and nothing holds them together. There are the
two stubs, and there is each arm's contract prose. gate-sdk/SPEC.md §run-gates
says so itself (`:10286-10293`): "It is a second source and this section says so
rather than claiming otherwise … the honest closure is a parity assertion over the
fail-open set, which the cut did not build."

The shell copy cannot be deleted. It is read exactly when the binary is absent,
so it cannot be asked of the binary. It can only be asserted. The trigger is live:
the port keeps landing non-gate arms. A harness-integration arm added without its
name in both stubs wedges a binary-less adopter at the point a hook grades a user
action.

It is a gate-sdk amendment: the stubs, the arm table, the new gate and its SPEC
sections are gate-sdk's.

The tree does not already do this. No machine-readable per-arm unavailable status
exists: the arm table in `native/src/emit/mod.rs` rows carry a name, a variant
and a knob roster, `--help` prints names only, and `--run-front-end-parity` compares
the two stubs with each other, never with the arms' contracts.

## What changes

**Batching.** Deltas 1 to 3 land in one commit. The gate (delta 3) reads the
declaration lines delta 2 writes and the set delta 1 declares.

### (1) The crate declares the fail-open set {design-bearing}

**Not yet applied.** The arm table's module declares
`FAIL_OPEN_ARMS: &[&str] = &["--hook", "--statusline"]` beside the table. A unit
test holds each name to an arm-table row, so a renamed or deleted arm reds the
crate's own tests.

**The crate's declaration is authoritative, and the stubs are its copies.** Only
the binary can hold a declaration a test can check against the arms it names. The
stubs hold a copy because they cannot ask. An arm's contract prose states *why* the
arm is in the set: the class rule in §The harness-integration arm, and the
`--statusline` paragraph. The prose does not re-list the set.

**The filing asked for a different reference set, and the change is deliberate.**
It named "the set of arms whose owning SPEC section declares status 0". Prose has
no declaration a gate can read without a grammar invented for the purpose, and a
per-arm status sentence in each section would be a third copy of the set. The
class rule stays in prose, and the list moves into the one place a test can
reach.

`--statusline`'s open question stays open. §The harness-integration arm leaves
unsettled whether it is a third branch of the rule or an instance of the fail-open
one. The declaration records its status, which both readings give as 0, and
chooses neither.

### (2) Each stub holds the set on one declaration line {mechanical}

**Not yet applied.** `run-gates.sh` spells the set as one assignment,
`FAIL_OPEN_ARMS='--hook --statusline'`, and sets `ARM_UNAVAILABLE_STATUS=0`
when the leading token is one of its words.
`run-gates.ps1` spells it as `$FailOpenArms = @('--hook', '--statusline')` and
tests membership with a case-sensitive `-ccontains`.

The transcripts are unchanged, so `--run-front-end-parity`'s corpus passes
byte-identical without an edit. Its absent-binary cases for `--hook` and
`--statusline` still exit 0, and those for every other arm still exit 2.

**Why a declaration line, not the current `case` arm.** The gate reads the set, and
reading it out of a `case` pattern or a `-ceq` chain would make the gate a parser
of two shell dialects' control flow. One assignment per stub is a line a reader
can find by its name.

### (3) `check-front-end-fail-open` holds the copies to the declaration {design-bearing}

**Not yet applied.** A new native gate, born with a `.gate` descriptor, a Rust
module and a `good/`+`bad/` fixture pair. It reads the `FAIL_OPEN_ARMS` line of
`<GATE_SDK_ROOT>/bin/run-gates.sh` and the `$FailOpenArms` line of
`<GATE_SDK_ROOT>/bin/run-gates.ps1`, and compares each name set with the crate's
`FAIL_OPEN_ARMS`. It exits:

- **0** when both stubs name exactly the declared set. The clean line names the set
  and both stubs.
- **1** when a stub names an arm the crate does not declare, or omits one it does.
  The finding names the stub, the name and the direction.
- **2** when a present stub carries no declaration line or two of them, or the line
  does not parse. The gate is fail-closed, and a stub it cannot read is not a
  clean one.

A stub that is **absent** is reported on the clean line, not red. A consumer tree
may drop the PowerShell twin, and a tree with no front-end at all has no copy to
drift.

- **Descriptor:** `# install: zero-config`, because its subject is the vendored
  `gate-sdk/bin/`, which `init` writes. Its couples are the two stubs.
- **Fixtures:** `good/` carries both stubs declaring `--hook --statusline`. `bad/`
  carries a shell stub declaring `--hook` alone, and a PowerShell stub declaring an
  extra `--usage-poll`, so both directions and both dialects red.

**Point 2, the rosters the new name lands on.** The name reaches `gates.list`, the
descriptor roster `check-gate-substrate-parity` holds, the fixture-coverage roster
`check-gate-fixture-coverage` holds, the coupling graph and the generated
pre-commit hook. Each is regenerated or registered in the same commit, and each
freshness gate prints its own regeneration command on red.

## Producers and consumers

- **`FAIL_OPEN_ARMS` in the crate (delta 1).** Producer: the arm table's module.
  Consumers: its unit test, and `check-front-end-fail-open` (delta 3).
- **The stubs' declaration lines (delta 2).** Producers: the two stubs. Consumers:
  each stub's own unavailable-status branch, and delta 3's gate.
- **The gate's verdict.** Consumers: the battery, and the pre-commit hook through
  the couples on the two stubs.
- **`--run-front-end-parity`.** Unchanged, and still what holds the two stubs'
  transcripts equal. The new gate holds their names to the crate, which the parity
  arm never did.

## Existing sections updated

Rosters produced by `grep -n "ARM_UNAVAILABLE_STATUS\|unavailable" gate-sdk/bin/run-gates.sh gate-sdk/bin/run-gates.ps1 gate-sdk/SPEC.md`
and by reading §run-gates and §The harness-integration arm.

- `gate-sdk/SPEC.md` §run-gates, the paragraph at `:10281-10293`: "nothing holds
  the two halves in lockstep" becomes the statement that `check-front-end-fail-open`
  holds both stubs to the crate's declaration (deltas 1 to 3).
- `gate-sdk/SPEC.md` §The harness-integration arm, the absent-binary rule
  (`:3813-3838`): it names `FAIL_OPEN_ARMS` as the list the rule is recorded in
  (delta 1).
- `gate-sdk/SPEC.md` gains a `### check-front-end-fail-open` section in the
  per-component roster (delta 3).
- `gate-sdk/bin/run-gates.sh` and `gate-sdk/bin/run-gates.ps1`, whose `# spec:`
  lines at the two-name test are re-phrased (delta 2).
- `native/src/emit/mod.rs` (delta 1). The new gate module and its registration in
  the crate's `REGISTRY` (delta 3).
- `gate-sdk/checks/check-front-end-fail-open.gate`, `scripts/gates.list`, the
  fixture pair under `gate-sdk/gate-tests/check-front-end-fail-open/`, and the
  generated pre-commit hook and coupling graph (delta 3).
- `.workflow/release-declarations.md` §Behavior changes: one bullet for the new
  zero-config gate, appended by the landing session (delta 3).
<!-- update-target-exempt: generated mirror of the kit SPEC, regenerated by its freshness gate's printed command, never hand-edited -->
- `docs/gate-sdk/SPEC.md`.

## Retired spellings

- None — the stubs' status variables keep their names, and the declaration lines
  are added beside them.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved** — `fail-open-arm-status-second-source` moves to Done in the
      merge commit, a stage before the drain stage.
- [ ] **Front-end parity unchanged** — `--run-front-end-parity` passes on the
      `gates` job.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` runs the block
      above against the tracked tree.
- [ ] **Gaps filed** — any cross-component gap build discovers is resolved that
      session.
