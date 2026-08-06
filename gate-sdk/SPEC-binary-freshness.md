# SPEC amendment: a freshness oracle for the gate binary

Pairs with the queue entry `native-binary-freshness-ungated`.

A compiled gate has no freshness oracle. `check-gate-substrate-parity` assertion
B diffs the `.gate` descriptor set against the binary's `--list` roster — set
membership only, never the binary's content against the source it was built
from. `gate_command` dispatches a `.gate`-declared member straight to the
prebuilt binary with no rebuild and no freshness check, so editing a gate's Rust
source, skipping the rebuild, and committing runs the descriptor-named gate
**against the stale binary**, where it passes on the old implementation. A gate
reporting clean on code that is not what is committed is the vacuous green the
whole battery exists to refuse.

The hole is inert today — no gate dispatches to the binary — and re-arms at the
first commit of the second port. `native-gate-binary-port` lands that port in
this iteration, so this amendment and that one ship together or the cohort ships
the hole.

## Why the entry stayed design-pending, and what settles it

The entry recorded the blocker exactly: the cheap form (the binary's mtime
against its sources) is wrong on a fresh clone and on any checkout that reorders
timestamps, and a content-derived form needs a build stamp the crate does not
emit today. Both halves stand. What was missing was an algorithm the crate can
carry.

**The crate declares no dependencies at all** (`native/Cargo.toml`'s
`[dependencies]` is empty; `walk.rs` and the one gate module are std-only). So a
content digest is not free: adding `sha2` puts a first dependency into the
artifact every adopter's machine carries, and hand-rolling SHA-256 puts a
cryptographic primitive into a crate whose whole point is being small and
auditable-by-fixture. Hashing is also the classic place two implementations
drift — a Rust digest and a shell digest over "the same" input set are one
canonicalization disagreement away from a permanent false red.

**The ruling: git is the hasher.** git is already the sole runtime dependency
the trajectory commits to, shelled out rather than embedded
(TRAJECTORY.md §The closed rulings), and it computes content-addressed hashes as
its native operation. Both sides of the comparison call the *same tool with the
same arguments*, so there is one algorithm rather than two implementations of
one algorithm — the crate gains no dependency, no crypto code, and no second
canonicalization to keep in step.

## What changes

Each delta carries its `{mechanical | design-bearing}` work class.

### 1. `native/build.rs` — the source stamp {design-bearing}

A build script computes the crate's **source stamp** at compile time and bakes it
into the binary as a compile-time constant.

The stamp is git's own content identity for the crate's tracked source set:

- The input set is **derived, never maintained** — every tracked file under the
  crate root, as `git ls-files` reports it. Neither side carries a roster of
  which files matter, so neither side can carry a stale one. The honest cost is
  named rather than discovered: a `native/targets.list` edit changes the stamp
  and asks for a rebuild although it changes no code. That is rare, harmless, and
  strictly better than a hand-maintained input roster that silently omits the
  file that mattered.
- For each input, git's blob hash of the **worktree** content, joined with the
  repo-relative path into one sorted listing, and that listing hashed again by
  git. Worktree rather than index, because the worktree is what `cargo` compiled;
  an index-derived stamp would go green on a build made from unstaged edits.
- Emitted with `cargo:rerun-if-changed=` for every input, so any edit to any of
  them re-runs the script and re-stamps the binary.

**Its honest limit, stated rather than discovered:** an **untracked** new source
file is invisible to `git ls-files`, so it is compiled but not stamped. This is
bounded by the same fact that makes the stamp worth having — an untracked file is
not in the commit either, so the state the gate governs is exactly the state the
stamp describes. A file added and staged in the same commit is tracked at the
gate's read and is covered.

Design-bearing: the worktree-versus-index choice, the derived input set, and the
untracked-file boundary are the whole of what makes the stamp mean what it
claims.

### 2. A `--source-stamp` arm on the binary {mechanical}

The binary prints its baked stamp on `--source-stamp`, one line, exit 0 — the
same top-level shape `--list` and `--reads` already take.

**It is a flag, not a registry member, and that distinction is load-bearing:**
`check-gate-substrate-parity` assertion B equates the `.gate` descriptor set with
the `--list` roster in both directions, so a stamp arm that leaked into `--list`
would read as a subcommand with no descriptor and red as a stranded
implementation. The arm is handled beside `--list` and `--reads` in `main.rs`'s
top-level dispatch and never enters the gate registry.

Mechanical: given the constant from delta 1, this is a match arm and a `println!`,
verified by re-running the crate's existing test leg.

### 3. `check-gate-binary-fresh` — the oracle {design-bearing}

A new gate-sdk gate. **Invariant:** whenever a `.gate` descriptor makes the
binary load-bearing, the binary was built from the source now in the tree.

- **Trigger coupling.** The gate derives the `.gate` descriptor set across the
  resolve dirs. **Zero descriptors ⇒ report zero and exit clean** — no gate
  dispatches to the binary, so nothing can run stale, which is precisely the
  entry's own "zero cost while no gate dispatches" accounting made mechanical.
  This is the coupling shape `check-gate-substrate-parity` assertion F already
  uses for the target roster, reused rather than re-invented: a consumer with no
  crate is not a consumer with a defect.
- **With ≥1 descriptor**, an absent or non-executable binary is **exit 2**, not a
  violation — the §Fail-closed contract, and the same verdict `gate_command`
  already gives, since "cannot verify" and "verified fresh" must not share an
  exit code.
- **The comparison.** Run `<binary> --source-stamp`; compute the tree-side stamp
  with the identical git invocation delta 1 specifies; red on a mismatch. The
  failure text names the rebuild command, so the remedy is in the output rather
  than in a reader's memory.
- Knobs: `GATE_SDK_NATIVE_BIN` and `GATE_SDK_NATIVE_CRATE`, both existing — the
  gate introduces none.
- `# graph:` manifest: `tier=precommit`, coupling the crate root **recursively**
  (the source tree is the subject) plus `kit:checks/*.gate` (the descriptor set
  is the trigger).
- Fixture pair: hermetic, with a two-argument form steering the descriptor dir
  and the binary path onto fixture copies — the shape
  `check-gate-substrate-parity` already ships. The `bad/` case is a stub binary
  printing a stamp for a source state the fixture tree no longer carries; the
  `good/` case is the matching one. The near-miss the `bad/` pair must not
  collapse into is the zero-descriptor tree, which is a *clean* case and is
  covered by this repo's own battery today.

**Why a new gate rather than a seventh assertion on
`check-gate-substrate-parity`.** The parity gate's couples are `kit:checks/*` and
`native/*` — the crate root **non-recursively**, deliberately, so that a
relocation of the crate re-fires it while ordinary crate edits do not. A
freshness assertion needs `native/src/**` recursively, so folding it in would
re-couple the auditor to the implementation tree it currently reads only by name,
and would fire the six-assertion parity gate on every Rust edit to buy a trigger
for one of them. The subjects differ too: parity audits what a port *declares*,
this audits whether a build is *current*. Keeping them apart also keeps parity's
own rule intact — it stays a shell gate that never depends on the substrate it
audits for its declaration reading, while this gate depends on the binary
answering by construction, which is a dependency the parity gate should not
acquire.

### 4. §Meta-gate conservation gains a disposition row {mechanical}

`check-gate-binary-fresh` couples `kit:checks/*.gate`, which covers the
declaration path of a registry member — so it is **substrate-sensitive by the
derivation `check-gate-substrate-parity` assertion C performs at runtime**, and
a member the conservation section does not name is red. The row is owed by that
gate, not by review, and landing this gate without it is a red at the first
battery run.

Disposition: **Retained by construction** — it reads declaration paths as a *set*
to decide whether the binary is load-bearing and never reads a gate's source, so
a port is its trigger rather than its blind spot; a ported member is exactly the
case that switches it on.

Mechanical: the derivation names the member and the disposition text follows the
table's existing pattern; executing it is an edit and a battery run.

### 5. The roster question the entry raises, corrected {design-bearing}

The entry states that docs/site-architecture.md §Generated projections "names no
entry for the compiled binary, though the repo's derivation-first rule requires
every generated projection to be freshness-gated and rostered there."
Re-verified against the tree, that requirement is not what the roster says. Its
own admission rule closes the section: *a derived surface earns a row here only
when it has a reader who cannot run the emitter* — a public page, or a file a
fresh clone needs before its tooling works — and derivation-first is satisfied by
deriving on demand otherwise.

The binary fails that test in both directions. It is **not committed** at all
(`target/` is gitignored), so there is no tracked copy for a freshness gate to
byte-compare; and its readers in this repo can run the emitter, which is `cargo
build`. A consumer is not a counter-example: a consumer never receives the crate
source and never builds, and the artifact they do receive is held by a published
digest verified before it is written (gate-sdk/SPEC.md §Consumer payload), which
is a different guarantee with a different mechanism.

**So: no row on that roster.** What the binary earns instead is a
**standing-absence** line in the same section's closing paragraph, beside
`queue-kit/bin/queue-index.sh` and `bin/queue-edges.sh` — the two instances
already recorded there so that their absence reads as a ruling rather than an
oversight. The line names `check-gate-binary-fresh` as where the obligation is
discharged, so a later reader asking "why is the binary not rostered?" finds the
answer and the oracle in one place.

Design-bearing: this reverses a premise the queue entry argued from, on the
roster's own stated rule, and the correction has to be written so that the next
session does not re-derive the same wrong requirement.

### 6. The new-gate fan-out {mechanical}

The on-site SPEC mirror, `docs/enforcement.md`, `docs/footprint.md`,
`docs/value.md`'s rollup block, `docs/check-graph.html`, and the generated hooks
(this is a `precommit`-tier gate) — the roster docs/site-architecture.md
§Generated projections assembles for exactly this. Footprint and rollup
regenerate **after** `git add`. Beside those, gate-sdk's kit-landing checklist:
the SPEC section, the `good/`+`bad/` pair, gate-sdk's README gate-roster block,
and registration in `scripts/gates.list`.

## Producers and consumers

**The source stamp (new state).**
*Producer:* `native/build.rs`, run by `cargo` on every build of the crate, with
`cargo:rerun-if-changed=` covering every tracked crate input so the producer
cannot be skipped while an input moves. Its enabling configuration is `cargo`
itself — nothing to set, nothing a deployment can leave unset, which is the
failure mode the causal-completeness check asks about. It is reachable in this
repo, in CI's crate build/clippy/test legs, and on the publish workflow's
per-target build legs, so every artifact that reaches a payload carries a stamp.
*Consumer:* the binary's `--source-stamp` arm (delta 2) is the only reader of the
baked constant, and `check-gate-binary-fresh` (delta 3) is the only reader of
that arm's output.

**`--source-stamp` (new interface).**
*Producer:* `native/src/main.rs`'s top-level flag dispatch, beside `--list` and
`--reads`. Not a registry member — stated as a delta rather than left implicit,
because leaking it into `--list` reds `check-gate-substrate-parity` assertion B.
*Consumer:* `check-gate-binary-fresh`, by process invocation, reading one line of
stdout and the exit status. No other component invokes it; nothing parses it into
fields.
*Every field has a named reader:* the arm emits exactly one value, the stamp, and
it is read at exactly one transition — the gate's comparison against the
tree-side computation. There is no second field, and none is added: a build
timestamp or a version string would have no reader and would invite the mtime
comparison this design refuses.

**`check-gate-binary-fresh` (new interface).**
*Producer:* registration in `scripts/gates.list` puts it in the battery, and its
`tier=precommit` manifest puts it in the generated hook, so a commit runs it
under the descriptor coupling above. Registration is a delta here, not an
assumption.
*Consumer:* the committing operator, through the output contract — the red names
the mismatching stamps and the rebuild command. Its second consumer is
`check-gate-substrate-parity` assertion C, which requires its disposition row and
reds until delta 4 lands.

## Existing sections updated

- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — owned
  by delta 4. The disposition table gains the `check-gate-binary-fresh` row.
- **gate-sdk/SPEC.md — new §check-gate-binary-fresh** — owned by delta 3. The
  gate's contract section, which its `# spec:` pointer binds to.
- **gate-sdk/SPEC.md §Porting a gate to the binary substrate, §What is retained**
  — owned by delta 3. That subsection currently states that a second port waits
  on no build and no further ruling. It stays true and gains one clause: what a
  second port now also *carries* is the freshness oracle, because a live
  descriptor is what arms the stale-binary path. Without this the section reads
  as though a port needs nothing, which was accurate only while the hole was
  inert.
- **gate-sdk/SPEC.md §Layout and configuration** — owned by delta 3, only to
  record that the new gate reads the existing `GATE_SDK_NATIVE_BIN` and
  `GATE_SDK_NATIVE_CRATE` knobs. No knob is added; the entry exists so the knob
  roster's reader list stays true.
- **gate-sdk/SPEC.md §check-gate-substrate-parity** — owned by delta 2. One
  clause in assertion B's text recording that the binary's top-level flags are
  outside the `--list` roster by construction, so the roster equality is over
  subcommands alone. The assertion's behavior does not change; what changes is
  that a reader adding a third flag knows it is not a parity violation.
- **docs/site-architecture.md §Generated projections and their freshness gates**
  — owned by delta 5. The closing standing-absence paragraph gains the gate
  binary, with the reason and the oracle that discharges the obligation.
- **CLAUDE.md §Housekeeping** — owned by delta 5. The `native/` bullet states
  that `native/target/` is gitignored and never committed; it gains the pointer
  to where build currency is enforced, one clause, no restatement of the
  mechanism.

## Cross-component notice

This amendment changes **gate-sdk**'s contracts (a new gate, a conservation-table
row, two spec sections) and **native**'s (a build script, a compile-time
constant, a new top-level arm), and edits the consumer docs surface and the
root manifest. It is a cross-component amendment on `check-stage-entry`
assertion C's own test, so the audit stage is owed before build entry.

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
- [ ] **This delta adds no crate dependency** — `native/Cargo.toml`'s
      `[dependencies]` gains nothing from the stamp work. A digest crate would be
      the artifact's first, and the whole point of the git-as-hasher ruling is
      that none is needed. Stated of this delta rather than of the tree, so that
      a sibling unit adding one for its own reason does not falsify a checklist
      item it never owned.
- [ ] **One algorithm, not two** — the stamp computation is the same git
      invocation on both sides, verified by a case where the two agree over a
      tree neither side special-cases.
- [ ] **The stale case reds** — proved by executing it, not by reading the gate:
      edit a crate source file, do not rebuild, and confirm the gate reds with a
      live descriptor present.
- [ ] **Ships with the cohort** — this and `native-gate-binary-port` land in the
      same iteration, so no commit exists where a descriptor dispatches to a
      binary nothing holds fresh.
