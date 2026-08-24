# SPEC amendment: worktree-residue

An unreaped agent worktree stops being a silent second copy of the repository.
The residue is **declared** rather than claimed away, it is **unreachable** to
the two walkers that could still read it as tree content, and it **blocks the
iteration boundary** rather than waiting to be noticed by accident. Nothing here
reclaims a worktree on the harness's behalf, and nothing here mutates a
repository from a decision surface.

**What is settled, and what this amendment refuses to depend on.** Four firings
are recorded on the queue entry, every one the identical shape: a locked
worktree at a stale stamp with an **empty** `git status --porcelain` inside, on a
read-only sweep that returned normally, each reaped by hand. The entry also
carries a hypothesis — that reclamation is tied to the *dispatching* session's
lifetime rather than the child's — that is cheap to test and **still unrun**.

Every delta below is **hypothesis-independent**, and that is the property that
makes the unit takeable now rather than after the experiment. A boundary that
refuses on residue refuses whatever caused the residue; a walker that cannot
reach a second copy cannot reach it whichever session stranded it; a claim
corrected to best-effort is correct under every cause. The experiment stays worth
running and is filed rather than blocked on — its value is telling a *dispatch
protocol* fix from a *sweep*, which is a later unit's question.

**The mechanism is the harness's and this repository does not own it.** Nothing
in tree can make the harness reclaim what it did not. So the four shapes the
entry weighs are not four fixes to one defect; three of them address *different*
halves and one addresses a false sentence. This amendment takes the false
sentence, the correctness half and the accumulation ceiling, and refuses the
fourth.

**The correctness half is measured, not argued.** A census over this tree's whole
reader set — every `scripts/gates.list` member, every native registry entry,
every kit `bin/` and `lib/` tool, and every consumer-config glob array — finds
**two** readers that would descend into `.claude/worktrees/<id>/` and read a
second copy of a governed file: `context-kit/bin/md-index.sh` and
`context-kit/bin/pub-index.sh`, each walking the repository root behind its own
hand-rolled `-not -path` list. Everything else is pruned, immune by tracking (an
ignored copy is invisible to a `git ls-files` corpus by construction), or immune
by glob. The claim the entry carries — that "the exclusion of `.claude/worktrees/`
is per-caller rather than central" — is therefore **true only in a narrower
sense**, and the narrower sense is the finding: gate-sdk's prune set is genuinely
one array read by all three walk adapters and bridged into the compiled substrate,
and it is context-kit's two index tools that hold private, stale copies of a
similar list. Both file:line sites were read directly at this stage rather than
taken from the sweep's report.

## What changes

### (1) `.gitignore`'s auto-clean claim is corrected, and the residue is declared

The ignore rule's comment stops asserting that the directory is auto-cleaned and
states the honest behaviour plus where residue is caught. **Mechanical.**

Today it reads that harness-managed worktrees are "in-repo, untracked and
auto-cleaned, so an in-flight agent would otherwise dirty the tree and abort
every clean-tree precondition". The first half is why the rule exists and is
unchanged; **auto-cleaned** is falsified by every one of the four firings and
becomes *best-effort auto-cleaned*, with a pointer to the boundary refusal delta
3 lands. This is the one option the entry costs at nothing, and it is taken
**alongside** the others rather than instead of them: a declared residue that
nothing catches is still an unbounded accumulation.

### (2) The "confinement is its own detector" biconditional is retired

delegation-kit's ground for isolation stops resting on a detector the firings
falsify, and rests on confinement alone — which was always sufficient.
**Design-bearing.**

§The delegation model currently argues that "the isolation is not purely a cost —
the harness auto-cleans a worktree left unchanged, so the worktree survives **iff**
the agent was not in fact read-only, which makes one mechanism both the
confinement and its own detector." The biconditional is false in the direction
that matters: four surviving worktrees each had an empty porcelain inside, so
**survival does not imply a write**. A detector whose positive is unsound trains
its reader to investigate nothing, which is worse than no detector — the reader
who finds a surviving worktree and infers a stray write goes looking for a change
that is not there, twice, and stops looking the third time.

**What replaces it is a weaker true statement and no new mechanism.** Auto-clean
is best-effort; a surviving worktree means *either* the child wrote *or* the
reclamation did not fire, and telling the two apart is one `git status
--porcelain` inside it. Isolation's case is unchanged and does not need the
detector: the template's **A read-only claim is made by isolation, not by
sentence** rule rests on write confinement, and confinement is a property of the
mechanism rather than an observation about its cleanup.

**The shape of this error is already recorded one section away, and the two
belong together.** §The delegation model's `worktree.baseRef` paragraph rules
that "repeated confirmation of a behavior is not evidence about its mutability",
after a false universal reached shipped doctrine on seven attestations. This is
the same failure on the same surface from the other side: repeated *successful*
auto-cleans confirmed the detector until they did not, and each success made the
mechanism feel more settled. The correction is filed beside that one so a reader
meets the general lesson once with two instances, rather than twice as two
unrelated repairs.

### (3) The iteration boundary refuses on a linked worktree

`bin/enter-stage.sh`'s iteration-boundary entry gains a fourth refusal, on the
same contract as the three it already carries. **Design-bearing.**

**The failure is a missing trigger, not a missing reap.** Every one of the four
firings was reaped by hand within a minute of being found; what was missing each
time was anything that *told* a session there was residue. Three of the four were
found by accident. So the deliverable is the trigger, and the reap stays a
session act — which is also what keeps this kit from removing a directory whose
liveness it cannot establish.

**Contract, matching the boundary refusals already in that file.** At the
**iteration boundary** only — the first stage's entry, where the reset already
truncates, wipes and re-headers — `enter-stage.sh` reads `git worktree list`. If
any **linked** worktree exists (any entry beyond the main checkout), the entry is
**refused, nothing written**, naming each worktree's path and the reap it owes;
`--simulate` reports the same refusal and writes nothing either. This is byte for
byte the shape of the non-empty-`## Lessons Learned` refusal and the untriaged-gap-
inbox refusal beside it, which is why no new gate is minted: the boundary
precondition family already has a home, and enforcement-first's own clause points
at the existing mechanism before a new one.

**Off `git worktree list`, never off `git status`.** The rule is already ruled
elsewhere in this tree (gate-sdk/SPEC.md §The first cohort's reaping paragraph):
a gitignored worktree leaves the status clean while the worktree still stands, so
a status-derived check reports success on exactly the state it exists to catch.
The `.gitignore` rule delta 1 corrects is what makes the status blind, so the two
deltas are two halves of one fact.

**Every linked worktree, and no path knob.** The obvious spelling is a knob
naming the residue directory, and it is refused: a kit default naming
`.claude/worktrees` is a kit literal carrying one harness's layout, the provenance
seam crossed for no gain. The predicate is instead a property of the boundary —
**at an iteration boundary no linked worktree should be live**, an in-flight
dispatch being something that must not straddle a boundary and everything else
being residue. That reaches the harness's worktrees, a leaked `upgrade-smoke.sh`
per-ref worktree, and any future producer, without naming any of them. A consumer
with a standing long-lived worktree turns the check off with one boolean knob,
`LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK` (default `1`, the shape
`CANON_KIT_SCAN_KIT_ROOTS` already sets for this tree's other boolean-shaped
knobs), rather than teaching the kit its paths.

**Two honest bounds, stated rather than banked.** *First*, the ceiling is
**per iteration**, not per dispatch: residue still accumulates within an
iteration, and this unit buys a bound rather than a sweep. That is the cost the
entry prices as "cheap, late" and it is accepted with its eyes open, because the
fourth firing was found in-session by the dispatching session — a residue that
survives to the boundary is exactly what a boundary check is for. *Second*, the
check **misses a dangling branch ref** whose worktree is already gone: `git
worktree list` cannot see one, and the branch-name pattern that could is one
harness's vocabulary, so a kit literal spelling it would publish that harness's
layout. The refusal's guidance therefore names the branch half explicitly so the
reaping session removes the ref with the directory, which is the entry's recorded
property (i) discharged by instruction rather than by scan.

### (4) context-kit's two index walkers read one prune array

`CONTEXT_KIT_PRUNE_DIRS` becomes the kit's single traversal-exclusion set, read by
both index tools, replacing two private literals that had drifted.
**Design-bearing.**

**The two sites, read at this stage.** `md-index.sh` excludes `*/target/*`,
`*/.git/*` and `*/node_modules/*` in the `find` at its walk; `pub-index.sh` holds
a `PRUNE` array of the same three plus `*/dist/*` and `*/build/*`. Neither carries
`worktrees`, so both descend into a second full copy of the repository and index
it as tree content — for `md-index` that is every governed markdown surface
twice, which is precisely the "a second live copy a later grep, a gate walk, or an
audit sweep can reach and read as the tree" harm the entry names.

**One array, defaulted to the union of what the two carried plus the leaf**:
`.git`, `node_modules`, `target`, `dist`, `build`, `worktrees`. The match is on
the **leaf basename**, the same rule and the same reasoning gate-sdk/SPEC.md
§lib/gate.sh already fixed for its own set: pruning the parent also passes but
loses coverage silently, because the governed markdown under it is reached by
explicit globs no prune touches. `md-index.sh` sources the consumer config seam to
read the knob — it does not today, while `pub-index.sh` already does through the
guarded `context-config.sh` lookup, so the seam read is copied from its sibling
rather than invented.

**`.tmp` and `gate-tests` are deliberately NOT added, and the omission is the
design rather than an oversight.** Both sit in gate-sdk's prune set and neither
belongs in this one: this unit's subject is the second copy of the repository, and
adding either is a corpus narrowing with its own readers — the index-tests
goldens, and a session that legitimately wants a fixture corpus indexed. A
narrowing bought for free is still a narrowing, and causal-completeness point 5 is
about exactly the free-looking ones. Whether the two sets should converge is
recorded as an open question and filed, not taken.

**The two arrays are not a duplication to collapse, and the reason is
kit-standing.** A hard read of `GATE_PRUNE_DIRS` would make an advisory index tool
fail in a tree that vendored context-kit without gate-sdk, and the sets are
deliberately **not** identical — this one omits two members and carries two
(`dist`, `build`) that gate-sdk's does not. Two arrays with two corpora that share
one leaf is not one fact spelled twice. This tree therefore assigns
`CONTEXT_KIT_PRUNE_DIRS` explicitly in `scripts/context-config.sh` rather than
deriving it from the gate library, which also keeps a `bin/` tool's traversal from
depending on a gate library being sourceable.

### (5) The guard-side dispatch sweep is refused, with grounds

The entry's second shape — a sweep in the dispatch guard — is ruled out in
delegation-kit's own prose so it is not re-proposed as the obvious earlier fix.
**Design-bearing.**

**A verdict surface may not own lifecycle.** The dispatch guard's contract is to
*decide* — allow or block a dispatch — and a guard that removes directories and
deletes refs on the way to a verdict has become a lifecycle owner whose failures
are invisible at the only place anyone looks (the verdict). That is the cost the
entry itself names as "the guard would own lifecycle it does not today", stated
here as a rule rather than as a price.

**And the timing is wrong in the one direction that cannot be recovered.** A
dispatch-time sweep runs while sibling dispatches are in flight, so its predicate
would have to establish that a worktree belongs to no live child — which the
attested firings make impossible from the outside: they were **locked** and
**stale-stamped** and **clean**, exactly the signature a live child's worktree
also carries, so lock is not a liveness signal here. A sweep that guesses wrong
destroys a running sibling's checkout. The boundary in delta 3 has the opposite
property: it takes no destructive action at all, and it runs at the one moment
whose whole definition is that the previous iteration's work is finished.

## Producers and consumers

**The boundary worktree refusal (new event, delta 3).**
*Producer:* `lifecycle-kit/bin/enter-stage.sh`, in the iteration-boundary arm
only, after the existing Lessons and gap-inbox refusals and before the boundary
truncation — the same position in the same ordered run of preconditions, so a
refusal's ordering relative to the reset is unchanged. Its enabling configuration
is `LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK`, the boolean knob delta 3 mints,
defaulted **on** (`1`), so the producer is reachable
in every consumer that does not turn it off; a consumer that never dispatches an
isolated agent has an empty `git worktree list` and the check is vacuous rather
than absent.
*Consumer:* the entering stage session, at the boundary entry, by exit status and
by the refusal text on stderr; and in `--simulate`, the same session at the
pre-flight, by the same text. No gate reads it — `check-stage-entry`'s three
assertions are unchanged, and the boundary-precondition family has deliberately
never had a gate sibling (lifecycle-kit/SPEC.md §check-stage-entry states the
predecessor-map omission for the same class of reason).
*Every field of the refusal has a named reader:* the worktree **path** by the
session performing the reap; the **branch ref** guidance by that same session at
the same transition, which is the only reader property (i) of the entry gets.

**`CONTEXT_KIT_PRUNE_DIRS` (new knob, delta 4).**
*Producer:* context-kit's config defaults, overridden by the consumer's
`context-config.sh`; in this tree, assigned explicitly in
`scripts/context-config.sh`, which is a file both tools already resolve through
the guarded seam lookup `pub-index.sh` carries and `md-index.sh` gains.
*Consumers, two, each at a named transition:* `md-index.sh` at its `find`
invocation, and `pub-index.sh` at its `find` invocation inside `emit_lang`. Both
are advisory `bin/` tools that join no registry, so there is no third reader and
none is implied — context-kit/SPEC.md §Index-first reading states the
no-`gates.list` property for all three tools already.
*The knob joins the kit's knob roster with its readers*, never ahead of them.

**Retired state (delta 2).** The auto-clean **detector** had exactly one
consumer — a parent reading a surviving worktree as evidence of a write — and
that consumer is removed with it. Nothing else read it: the reaping boundary
already sends the parent to `git worktree list` rather than to the signal, which
is what makes the retirement a deletion rather than a replacement.

**Red conditions named, because delta 4 narrows a corpus.** Point 5 binds and the
readers are enumerated by their red condition, not their subject:

- **`context-kit/index-tests/` goldens** — red condition: *emitted rows differ
  from the recorded golden*. **Not monotone** under a corpus narrowing in either
  direction: a golden asserting an exact row set breaks if a file leaves the walk
  just as surely as if one joins it. Cleared by **running** the index tests inside
  the unit, never by inspection, and this is the delta's one real verification
  cost. The narrowing is chosen to be inert here — `worktrees`, `dist` and `build`
  are absent from the fixture corpus — but "chosen to be inert" is a prediction and
  the runner is the oracle.
- **`check-comment-tier`** — red condition: *a full-line comment on a governed
  source that is not a recognised directive*. Deltas 1 and 4 both write comment
  lines (the corrected `.gitignore` note, the knob's `comment-tier-exempt:`
  rationale in the consumer config). Not monotone in the safe direction, since the
  deltas add comments; cleared by writing each as a recognised class with a
  non-empty payload, on the same terms every other line in `scripts/context-config.sh`
  carries.
- **`check-knob-citation` and `check-knob-default-coupling`** — red conditions:
  *a knob mentioned in prose that no config defines*, and *a default stated in
  prose that differs from the code's*. **Monotone** in the knob set: deltas 3
  and 4 each add a knob (`LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK`,
  `CONTEXT_KIT_PRUNE_DIRS`) **and** its SPEC roster entry **and** its default in
  the same unit, so neither reader acquires an unpaired subject. Named because a
  knob minted in a SPEC ahead of its assignment is the standard way to red both
  at once.
- **`check-stage-entry`** — red conditions are its three assertions
  (predecessor-stamp ordering, drain-entry queue-empty, audit-trigger).
  **Untouched**: delta 3 adds a refusal to `enter-stage.sh` and no assertion to
  this gate, and the gate's corpus is the state file and the queue rather than the
  worktree list. Named because a new boundary precondition looks like it must have
  a gate sibling, and lifecycle-kit/SPEC.md §check-stage-entry rules that it does
  not.
- **`enter-stage.sh`'s own smoke coverage** — red condition: *a boundary entry
  behaving differently from its recorded end-to-end expectation*. Monotone only if
  the smoke tree has no linked worktree; it does not, but the refusal is a new
  branch in a scripted path, so the smoke is **run** rather than reasoned about.

## Existing sections updated

- **delegation-kit/SPEC.md §The delegation model**, the isolation's-second-purpose
  paragraph — the auto-clean biconditional and the "one mechanism both the
  confinement and its own detector" claim are retired and replaced with the
  best-effort statement plus the one-command disambiguation; the parallel with the
  `worktree.baseRef` false universal one paragraph down is drawn (delta 2).
- **delegation-kit/SPEC.md §The delegation model** — the dispatch-time guard sweep
  is recorded as refused with both grounds, so the earlier-is-better instinct meets
  the ruling rather than the silence (delta 5).
- **delegation-kit/templates/agent-execution.md**, the **Isolation charges three
  harness costs** rule's second cost — "Gitignore the path, and reap agents at the
  boundary with `git worktree list` rather than off `git status`" gains the fact
  that the reap is now *enforced* at the iteration boundary, and the branch-ref
  half that a `worktree remove` leaves behind (deltas 2 and 3).
- **lifecycle-kit/SPEC.md §bin/enter-stage.sh** — the boundary refusal roster
  gains its fourth member: the predicate, its position in the ordered run, the
  `--simulate` behaviour, the knob, and both stated bounds (per-iteration ceiling,
  dangling branch refs) (delta 3).
- **lifecycle-kit/SPEC.md §Layout and configuration** — `LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK`
  joins the roster with its default (delta 3).
- **context-kit/SPEC.md §Index-first reading** — both tools' prune-set sentences
  ("skipping `.git/`, `node_modules/`, and build dirs"; "the prune set is `.git`,
  `target`, `node_modules`, `dist`, `build`") are replaced by the one knob, the
  leaf-basename rule is stated, and the deliberate omission of `.tmp` and
  `gate-tests` is recorded (delta 4).
- **context-kit/SPEC.md §Layout and configuration** — `CONTEXT_KIT_PRUNE_DIRS`
  joins the knob roster with its default (delta 4).
- **`.gitignore`**, the `.claude/worktrees/` rule's comment — the auto-clean claim
  corrected and the boundary refusal named (deltas 1 and 3).
- **`scripts/context-config.sh`** — this tree's explicit assignment, with the
  rationale comment the file's convention requires (delta 4).
- **`context-kit/bin/md-index.sh`** and **`context-kit/bin/pub-index.sh`** — the
  seam read and the walk's exclusion source (delta 4).
- **`docs/delegation-kit/SPEC.md`** — the on-site mirror of the delegation-kit/SPEC.md
  edits above, regenerated by its own arm (deltas 2 and 5).
- **`docs/lifecycle-kit/SPEC.md`** — the on-site mirror of the
  lifecycle-kit/SPEC.md edits above, regenerated by its own arm (delta 3).
- **`docs/context-kit/SPEC.md`** — the on-site mirror of the context-kit/SPEC.md
  edits above, regenerated by its own arm (delta 4). All three mirrors are the
  same generic projection the owed-column amendment's sibling unit names for
  gate-sdk/SPEC.md (docs/site-architecture.md §Generated projections and their
  freshness gates): `bash
  gate-sdk/bin/run-gates.sh --emit docs-mirror --write`, byte-gated by
  `check-docs-mirror-fresh`.
<!-- update-target-exempt: a no-change confirmation the ruling produces, owned by no delta -->
- **gate-sdk/SPEC.md §lib/gate.sh**, the prune-set bullet — **unchanged**, and
  listed so the build confirms rather than assumes it: the census found that array
  genuinely central and correctly bridged, so nothing about the gate substrate's
  prune moves and the leaf-basename rule is cited from there rather than restated.
<!-- update-target-exempt: a no-change confirmation, owned by no delta by construction -->
- **guard-kit's dispatch guard** — **unchanged**: delta 5 refuses giving it a
  sweep, so its contract, its assertions and its source are untouched. Listed
  because a reader of delta 5 will look for the guard edit that is deliberately
  absent.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`), discharged at the iteration
      rather than at the commit.
- [ ] **The narrowing's non-monotone reader is RUN, not inspected** — the
      context-kit index tests and `enter-stage.sh`'s smoke both execute inside the
      unit; a prediction that the narrowing is inert does not clear them.
- [ ] **The refusal is exercised on a real linked worktree** — a boundary entry is
      simulated with one present and one absent, since a refusal branch that no
      run reaches is a refusal nobody has seen work.
- [ ] **Removals propagated** — grepped every spec and template for the retired
      auto-clean claim and for the detector argument that rested on it (delta 2),
      and for either tool's prune-set enumeration (delta 4); nothing dangles.
- [ ] **The on-site mirrors are regenerated, never assumed** — this unit edits
      three kits' SPEC.md (delegation-kit, lifecycle-kit, context-kit), each with
      a byte-gated `docs/<kit>/SPEC.md` mirror; `bash gate-sdk/bin/run-gates.sh
      --emit docs-mirror --write` runs before `check-docs-mirror-fresh` is
      trusted green.
- [ ] **Gaps filed** — the unrun falsification experiment (dispatch, kill the
      parent, look), and the open question of whether context-kit's and gate-sdk's
      prune sets should converge, are filed rather than flagged-and-skipped.
