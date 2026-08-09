# SPEC amendment: dispatch-worktree-reds-the-battery

Rules the defect `dispatch-worktree-reds-the-battery` filed across four
attestations: a live agent worktree materialises `.claude/worktrees/agent-<id>/`,
a full second copy of the repo, and every tree-walking gate descends into it. The
battery — the oracle every commit in this tree requires — goes red for the whole
duration of a read-only fan-out, so delegation and verification cannot overlap.
The entry is emphatic that this is on the sanctioned path: the dispatch guard
*mandates* `isolation: worktree` for a read-only agent type, so the cost is a
serialization constraint on every delegating session rather than an
untracked-directory nit.

**The ruling: the entry's own stated blocker is false, and the fix is one word.**
The entry sharpened its design question into a concrete blocker — that a shared
exclusion root "would have to be path-shaped, not name-shaped". Measurement
refutes it. A name-shaped prune entry, `worktrees`, closes the defect completely.

## The measurement that refutes the blocker

Run at this amendment's authoring, on a real worktree
(`git worktree add .claude/worktrees/agent-probe HEAD --detach`):

| condition | battery |
| --- | --- |
| worktree live, prune set at its default | **3 of 100 FAILED** |
| worktree live, `worktrees` appended to the prune set | **All 100 passed** |

The three reds reproduce the entry's attestations exactly —
`check-comment-tier` on `reserve/crates/src/lib.rs` inside the copy, and both
`check-enforcement-fresh` and `check-value-rollup-fresh` stale because their
emitters counted the duplicated tree.

**Why the entry concluded otherwise.** It reasoned that `gate_find` prunes by
directory basename, so no prune entry can name the *path* `.claude/worktrees`
without naming the *parent* `.claude` and taking `.claude/commands` and
`.claude/agents` with it. Both halves of that are true. The step that does not
follow is the conclusion: the prune matches the **leaf** basename, and
`worktrees` is itself a leaf name. It selects `.claude/worktrees/` and nothing
else — verified: no other directory named `worktrees` exists in the tree.

**And pruning `.claude` would not have reddened either** — also measured, All 100
passed. That is the more useful half of the result, because it shows the
`.claude` variant fails in the direction a battery cannot report: `.claude` holds
only `.json` and `.md`, so `gate_find`'s shell/Rust corpus never reaches it, and
the governed `.claude/**.md` surfaces are read by **explicit globs**
(`scripts/canon-config.sh`'s agent-definition surface,
`LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS`, `check-skill-binding`'s `# graph:` manifest)
which no prune touches. So `.claude` is wrong for a **coverage** reason, silently,
where `worktrees` is right for both. Recording that distinction is the point: a
later session testing the blunt fix would see green and conclude it was fine.

## The real blocker, which the entry did not name

`GATE_SDK_PRUNE_DIRS` **replaces** the default set wholesale — `lib/gate.sh` reads
the knob or falls back, with no additive form. So a consumer wanting one extra
prune directory must copy all five kit defaults into its config, and that copy
drifts silently the day the kit default moves. Setting the knob in
`scripts/gate-sdk-config.sh` would therefore be a maintained copy of a derivable
set: a derivation-first violation, traded for the fix.

That is the design question this unit actually has to answer, and it decides
where the fix lands.

## The ruling on placement

**`worktrees` joins the kit default; the consumer config is not touched.**

Two facts settle it against the consumer-config alternative. First, the
copy-drift above: with no additive knob, consumer config cannot express "the
default plus one". Second, the provenance seam does not bar it. The seam bars
*private rule content* — term lists, coupling vocabularies, product constants. A
harness-produced directory name is neither private nor this repo's: the default
set already carries `node_modules`, `target` and `.git` (third-party tool
artifacts) and `.tmp` (this methodology's own scratch), so a tool-produced
directory that is never source is exactly the class the default already holds.
Every consumer running isolated dispatch hits this defect identically, which is
the test for a kit default rather than a knob.

The residual risk is stated rather than dismissed: a consumer whose own source
lives under a directory named `worktrees` loses coverage silently. That risk is
already carried by `target` and `.tmp` on the same terms, and the additive knob
below is what lets such a consumer opt out without re-copying the set.

## What changes

- **(D1) Append `worktrees` to `GATE_SDK_PRUNE_DIRS`' default** in
  `gate-sdk/lib/gate.sh`. One word. It reaches all three walk adapters at once —
  `gate_find`, `GATE_GREP_EXCLUDES`, `gate_path_pruned` — because all three
  derive from the same array, which is why no per-gate edit is owed.
  **mechanical** — a one-token change to a literal, verified by D3's oracle.

- **(D2) Document it in `gate-sdk/SPEC.md` §lib/gate.sh**, where the knob and its
  default are already specified, with the one-clause reason (a harness-produced
  repo copy is not source) and the coverage caveat above. **design-bearing** — the
  caveat is the part a consumer needs and the part a bare default cannot say.

- **(D3) Extend `gate-sdk/gate-tests/lib-gate.test.sh`** — its existing
  prune-set block already exercises `gate_path_pruned`, `GATE_GREP_EXCLUDES` and
  `gate_find` against the default set, so the case is one sandbox directory and
  one assertion: a file under `<sandbox>/.claude/worktrees/agent-x/` is absent
  from `gate_find`'s output while a sibling under `<sandbox>/sub/` is present.
  **mechanical** — the surrounding assertions are the template.

- **(D4) Add the additive knob `GATE_SDK_PRUNE_EXTRA_DIRS`** to `lib/gate.sh`:
  appended to the resolved set, whether that set came from the default or from
  `GATE_SDK_PRUNE_DIRS`. It is what makes D1's placement ruling honest — without
  it, a consumer disagreeing with D1 has no move but to copy the default. Named
  on the house pattern already in this repo's own gate-sdk config,
  `GATE_SDK_LINT_EXTRA_DIRS`. Default empty, so no behavior changes for anyone
  who does not set it. **design-bearing** — an additive-versus-replacing knob pair
  is a config contract, and its interaction with the replacing knob has to be
  specified rather than left to the reader.

## What this amendment deliberately does not do

**It does not relocate worktrees outside the repo.** The entry named that as the
fix that would close both halves at once and correctly called it the harness's
call, not the tree's. D1 makes it unnecessary rather than merely unavailable: the
`git status` half was already closed when `.claude/worktrees/` was gitignored at
the 2026-08-08 close, and D1 closes the gate half, which is the half that
gitignore provably did not reach.

**It does not add a shared exclusion-root mechanism, nor a roster of exclusion
paths.** The entry's design question asked "whether kit scanners owe a shared
exclusion root and where that roster lives". The answer measurement gives is that
they already have one — `GATE_SDK_PRUNE_DIRS`, read by all three walk adapters —
and it needed one more member, not a second mechanism. A path-shaped exclusion
facility would be new machinery bought to solve a problem the existing
basename-shaped one solves.

**It does not touch the `recurrence:` declaration.** The entry's body attests four
occurrences while its declaration carries one date, and a gap-inbox bullet already
routes that reconciliation through the mechanized drain at close. A hand-stamp
here would write the sanctioned writer's output by hand, which is the defect
`recurrence-drain-input-widening` names.

## The seam

D1 puts a directory name in a kit literal, so the seam question is real and the
answer is the class test: `worktrees` names a **harness artifact**, generic to
every consumer of these kits, in the same class as the `node_modules` and
`target` already there. It carries no project vocabulary, no product constant, and
nothing about what this repo builds — nothing is published by shipping it. D4 is
the seam's other half: a consumer whose tree makes the default wrong adjusts it
through config rather than by editing the kit, which is the pattern
CLAUDE.md §The provenance seam names.

## Producers and consumers

The amendment introduces one new configuration input (D4's knob) and changes one
existing default (D1). No new state, event, or message.

- **Producer of the resolved prune set** — `gate-sdk/lib/gate.sh` at source time,
  on every gate run. Its enabling config is unconditional: the array is built
  whether or not either knob is set, so the producer is on the ordinary path for
  every consumer, and D1's member is emitted everywhere the set is.
- **Consumers of the set, all three and by mechanism, not by convention** —
  `gate_find` (a `-name … -prune` clause per member), `GATE_GREP_EXCLUDES` (one
  `--exclude-dir` per member, consumed by sourcing gates), and
  `gate_path_pruned` (a path-segment predicate). Every gate that walks a tree
  reaches the set through one of these; the three reddening gates reach it
  through the first, `check-comment-tier` via `canon-kit/lib/spec.sh`'s comment
  surface.
- **Named reader of D4's new knob** — `lib/gate.sh`'s own set-resolution block,
  the single site that reads it, and the only one: the knob has exactly one
  reader by construction because the resolved array is what every other reader
  sees.
- **Consumer of D2's prose** — the author of the next gate that walks the tree,
  who needs to know the walk is already pruned and on what basis. This is a named
  reader rather than a decorative one: the entry records that the failure is
  "misattributed by construction — the red names the session's own files, not the
  dispatch", so a session meeting the red is the reader D2 has to reach.

**Existing integration prose describing the prior flow**: `gate-sdk/SPEC.md`
§lib/gate.sh states the walk adapters and names the knob; D2 updates it in place.
Two further SPEC sites mention the knob (§Layout and configuration's default
listing, and §check-graph's sanctioned-walk clause). The default listing carries
the value and is updated by D2; the check-graph clause cites the knob by name
without restating members and needs no edit.

No new field is added to any record or artifact, so the every-field-has-a-reader
obligation is vacuous here rather than skipped.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **The acceptance oracle, named per delta.** D1+D3: `lib-gate.test.sh` green,
      and it discriminates both wrong implementations — D1 absent fails the new
      assertion, D1 misspelled as a path (`.claude/worktrees`) also fails it,
      because a basename matcher cannot match a slash. D4: an assertion in the
      same test that a set from `GATE_SDK_PRUNE_EXTRA_DIRS` appends to the default
      *and* to an explicit `GATE_SDK_PRUNE_DIRS`, which is the interaction D4's
      contract turns on and the only one a single-knob test would miss. D2: no
      oracle beyond the doc gates; stated as such rather than implied covered.
- [ ] **The end-to-end witness, run once at build**: create a worktree under
      `.claude/worktrees/`, run the full battery, see 100 of 100 with it live.
      This is the defect's own reproduction and it is not a substitute for D3 —
      it cannot run in CI on a clean tree, which is exactly why D3 exists.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change retired;
      nothing dangles. Specifically: the queue entry's path-shaped-blocker claim
      does not survive into any merged spec section.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not deferred).
