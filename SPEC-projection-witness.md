# SPEC amendment: projection-witness

The generated-projections roster states each projection's staleness trigger in
English, and nothing checks the English against the emitter. The filing counted
five false statements in one roster, and four of them were in the negative
direction: a class the prose said was a trigger and is not, or the reverse.

This amendment makes two changes:

- It replaces the prose trigger with the declaration that already exists, the
  freshness gate's own `couples=`/`trigger=`.
- It adds a differential witness that tests that declaration against the
  emitter by perturbing the tree and re-running the gate.

The witness reads the `# projection:` membership that SPEC-projection-roster.md
mints, so the two share one declaration, as their entries anticipated.

**The open part the entry named, the row-to-class binding, is derived and not
written.** A freshness gate's manifest already declares the paths whose change
must re-trigger it: `trigger=`, or `couples=` where `trigger=` is absent, plus
the derived knob files (gate-sdk/SPEC.md §The `# graph:` manifest, *Every
trigger reader appends the set*). That declaration drives the generated hook, so
it is the one that matters. A prose row that restates it is a second source. The
prose restatement was the part that was wrong. So the per-row declaration the
entry asked for is the descriptor, and the row stops stating a trigger class.

**Perturbations run at this spec, in a disposable worktree (2026-09-21).**

- **Negative claim, run.** One line was appended to `scripts/bash-guard.sh`. It
  is outside `check-footprint-fresh`'s couples, and the roster says footprint
  measures no script. `--emit footprint` came back byte-identical, so the claim
  holds.
- **Coupled but inert.** One line was appended to the end of `CLAUDE.md`, which
  is inside footprint's `couples=`. The emission came back byte-identical,
  because `native/src/emit/footprint.rs` reads only the text between kit
  markers.
- **Coupled and live.** One line was inserted inside the lifecycle-kit marker
  block. That row went from `5l·~76t` to `6l·~85t`.

So `couples=` is a sound **superset** of the reads and never a minimal set. That
is the over-approximation the manifest rule sanctions: "an extra trigger runs a
green gate while a missing one would skip a red".

The witness therefore asserts only in the direction where an error is a
defect. A read **outside** the declared trigger set is a stale projection the
hook never fires on. A coupled file that moves nothing is sanctioned, so the
witness reports it and never reds on it.

**Cost, measured.** Each emitter takes 7 to 35 ms (`time --emit footprint`,
`enforcement-map`, `value-rollup`, `roadmap`). No knob redirects the tree root:
`native/src/walk.rs`'s one `std::env::current_dir()` is the root. So the witness
needs a real scratch copy. The tracked tree grouped by (first path segment,
extension) gives 134 classes, where `git ls-files` gives 1996 files and
per-directory grouping gives 1153. Eight gates times 134 classes times an edit
and a create is about two thousand gate runs. That belongs in a validate suite,
not the battery.

It is a root-level amendment because it spans gate-sdk (the arm and the
tracked-tree scratch), evidence-kit (the suite registration, which is consumer
config) and this repo's `docs/` roster.

## What changes

**Batching.** Delta 1 lands first, or with SPEC-projection-roster.md's delta 1,
because it reads `# projection:`. Deltas 2 and 3 ride with it.

### (1) `--projection-witness` perturbs outside each projection's trigger set and asserts the gate stays green {design-bearing}

**Not yet applied.** This is a new gate-sdk non-gate arm, an `Arm::Run`
(gate-sdk/SPEC.md §The non-gate arm), with a 0/1/2 exit.

1. **Members.** Every registered gate carrying `# projection:`. With none, the
   arm prints `0 projections` and exits 0, which is visible and not a silent
   pass.
2. **Scratch.** One tracked-tree scratch: the index's tracked set copied with
   `std::fs`, then `git init` and a seed commit — gate-sdk's tracked-tree scratch
   (gate-sdk/SPEC.md §Consumer smoke), already landed with `check-fence-run`, which
   this arm calls. The running binary is placed where `GATE_SDK_NATIVE_BIN`
   resolves in the scratch, through that section's `place_artifact`. It is removed
   on every exit path.
3. **Baseline.** Each member gate is spawned by name in the scratch, the arm's
   own executable with the scratch as its working directory, and it must exit 0.
   A member red at baseline is exit 2 naming it, since the projection was
   already stale and nothing can be witnessed.
4. **Classes.** Group the tracked set by first path segment and extension. For
   each member, take each class's first member in byte order that its
   **expanded trigger set** does not match. The trigger set is the one
   `run-gates --for` computes, knob files included, through
   `registry::expand_couples` and the field's one matcher.
5. **Two perturbations per class.**
   - An **edit**: append one line to the chosen file.
   - A **create**: stage a new file `<segment>/.projection-witness.<ext>`.

   After each one, the member gate runs again, and the scratch is reset with
   `git checkout -- .` and `git clean -fdq`.
6. **Verdict.** If a member gate goes **red** after an out-of-trigger
   perturbation, the projection read a file its manifest does not declare: a
   finding, exit 1. The finding names:
   - the gate;
   - the perturbed path;
   - which of the two perturbations;
   - the gate's first red line.

**The positive direction is a report, never a red.** For each member and each
class its trigger set does match, the arm also runs the edit. A gate that stays
green is printed as `coupled but inert: <gate> <path>`, on the ground stated
above. The line is for a person tightening a trigger, and no suite parses it.

**Point 5, the red condition.** Exit 1 carries one `WITNESS: FAIL` line per
finding. Exit 2 carries `WITNESS: FAIL(env)` for:
- a member red at baseline;
- a scratch that could not be built;
- a spawn failure.

The clean line counts members, classes, perturbations and inert couples. So a
run whose class set came out empty prints zeros and is not mistaken for coverage.

**Honest limits, stated in the section.**

- An append can miss a read that is confined to a marker block, which is the
  `CLAUDE.md` case above. So an out-of-trigger file read only inside markers can
  pass. The create perturbation does not share this blind spot for enumerating
  emitters.
- Classes are sampled one member each, so a read of one specific file in an
  otherwise-unread class is missed.
- Both limits under-report and never over-report. A red is always real.

### (2) The roster rows stop restating the trigger {mechanical}

**Not yet applied.** In docs/site-architecture.md §Generated projections, each
keyed projection row (SPEC-projection-roster.md delta 4) is re-phrased so it
drops its trigger-class sentence. It keeps:

- the output;
- the gate;
- the regen command;
- any grounds a reader needs, for example why the footprint's measured set is
  narrow, which is a reason and not a trigger.

The section intro says, once, that a projection's trigger is its gate's
`couples=`/`trigger=`, answered for any path by `run-gates --for <path>`, and
held by `--projection-witness`.

Mechanical: the sentences to drop are each row's "stale on …", "regenerate after
…" or "the trigger is …" clause. Seven rows, found by
`grep -n "^- \*\*" docs/site-architecture.md`.

### (3) This repo runs the witness as a validate suite {mechanical}

**Not yet applied.** `scripts/evidence-config.knobs` gains
`EVIDENCE_KIT_SUITES[] = projection_witness` and
`EVIDENCE_KIT_RUN_projection_witness = bash gate-sdk/bin/run-gates.sh
--projection-witness`. That is the `demo` suite's shape: the exit code is the
verdict, so no parser knob is needed.

Mechanical: two knob lines.

## Producers and consumers

- **`--projection-witness` (delta 1).** Producer: the arm table row. Enabling
  configuration: `# projection:` lines, which SPEC-projection-roster.md delta 4
  sets on eight gates in this repo. So the arm is live here and not dead outside
  its tests. Consumers:
  - the `projection_witness` validate suite (delta 3), which reads the exit code;
  - a person reading the `coupled but inert` lines.

  Roster-holding readers:
  - the arm table itself;
  - the `--help` listing, which derives from the table;
  - gate-sdk/SPEC.md §The non-gate arm's member prose, which gains the member;
  - `check-knob-citation` for any knob the arm declares. It declares
    `GATE_SDK_NATIVE_BIN` in its `KNOBS` for the placement, and that knob is
    already documented.
- **The tracked-tree scratch (delta 1).** It is internal, shared in-crate with
  `check-fence-run`, and no other component reads it.
- **The class grouping (delta 1).** The arm's own computation. Its one reader is
  the arm's loop.
- **The suite (delta 3).** Producer: the knob lines. Consumer: `--run-validate`,
  at the validate stage.
- **Point 6.** The member corpus is SPEC-projection-roster.md's eight gates, and
  each one's satisfying value is its own baseline-green state. That holds at HEAD,
  because the battery is green. No member is narrowed past.

## Existing sections updated

Rosters produced by the three perturbations and the timing runs in the
preamble, by `git ls-files` grouped as delta 1 step 4 states, and by reading
gate-sdk/SPEC.md §The `# graph:` manifest, §Reading a `couples=` field's reach
and §The non-gate arm.

- gate-sdk/SPEC.md: a new section for `--projection-witness` beside §check-reads-couples,
  whose static reads⊆couples half this is the dynamic complement of, and
  §The non-gate arm's roster (delta 1).
- `native/src/emit/mod.rs`'s arm table and a new arm module, which calls the shared
  tracked-tree scratch helper (delta 1).
- docs/site-architecture.md §Generated projections: the section intro and seven
  row sentences (delta 2).
- `scripts/evidence-config.knobs` (delta 3).
<!-- update-target-exempt: generated mirrors and projections, regenerated by their freshness gates' printed commands, never hand-edited -->
- `docs/gate-sdk/SPEC.md`, `docs/enforcement.md`.

## Retired spellings

- None — no name is removed. Each row's trigger sentence is re-phrased away, and
  none of them was a name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only.** The roster intro names the
      oracle and does not restate the rule.
- [ ] **Merged with no information lost.** Each addition re-phrases the text it
      refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`), and
      the sibling amendments' filename citations are repointed.
- [ ] **Entry moved.** `projection-trigger-witness` moves to Done in the merge
      commit, at a stage before the drain stage.
- [ ] **Witnessed on a known defect.** A crate test builds a scratch with a toy
      projection whose emitter reads a file its descriptor omits, and the arm
      exits 1 naming that file.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block
      above.
- [ ] **Gaps filed.** Any cross-component gap build discovers is resolved that
      session.
