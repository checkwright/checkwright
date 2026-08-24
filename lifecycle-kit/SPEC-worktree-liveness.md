# SPEC amendment: worktree-liveness

Closes `agent-worktree-boundary-disposition`. The iteration boundary refuses on
registered agent worktrees, but nothing reaps them and nothing surfaces one
mid-iteration.

**The entry's stated blocker no longer holds, and that is why this unit is
authorable at all.** Its design question was: "a worktree has no analogous
signal, so *is anyone still working in this one* is unanswerable today.
Inventing that signal is the design work here — the config line is not." The
upstream experiment `worktree-reclamation-cause-falsification` ran at this spec
stage and found the signal **already exists and is published by git**. Nothing
is invented below; a signal that was being discarded is read.

## What changes

### (1) A linked worktree gains a liveness class, read from its git lock

The boundary check stops treating every linked worktree as one undifferentiated
population and classifies each as **live** or **orphaned**, from the lock reason
`git worktree list --porcelain` already prints. **{design-bearing}**

**The mechanism, established by experiment at this spec stage rather than
hypothesised.** A worktree-isolated dispatch's entry in
`git worktree list --porcelain` carries a `locked <reason>` line, and the reason
is a liveness record: it names the holding process's **pid** and its **start
time**. The same string is on disk at `.git/worktrees/<name>/locked`. The
holder's start time was verified equal to that process's own
`/proc/<pid>/stat` field 22, so the reason is not decorative — it is a record
with a PID-reuse guard, strictly richer than the `pid=<n> run=<key>` grammar
evidence-kit already reads for backgrounded shell producers. Re-derive by
dispatching an isolated agent and reading the porcelain output while it runs.

**The predicate is one this project already owns.** Liveness is
`evidence::pid_alive` / `ek_pid_alive` (evidence-kit/SPEC.md
§check-producer-liveness) — the same probe the `.run` path uses, so no second
PID predicate is minted and nothing about how liveness is decided is seconded.

**The lock-reason format is consumer config, never a kit literal, and that is
the same ruling this check already made once.** §bin/enter-stage.sh states that
"no knob names a residue directory, because a kit default spelling one harness's
layout would publish it". A lock **reason string** is that same harness's
vocabulary one field over, so it takes the same disposition — the
`check-graph` / `scripts/graph-vocab.sh` pattern CLAUDE.md §The provenance seam
names for exactly this. The kit ships the mechanism (read the porcelain, apply
the pattern, probe the pid) and the consumer ships the pattern.

New knob **`LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE`** — a POSIX ERE with **one**
capture group, matched against a worktree's lock reason, whose group is the
holder's pid. **Default empty**, and an unconfigured consumer therefore sees
**exactly today's behaviour**: every linked worktree is unclassified and the
boundary refuses on all of them, which is what makes this delta additive rather
than a change of verdict for anyone who does not opt in.

The classification, with the knob set:

| observation | class |
| --- | --- |
| locked, reason matches, captured pid alive | **live** |
| locked, reason matches, captured pid dead | **orphaned** |
| locked, reason does not match the pattern | **unclassified** |
| not locked at all | **orphaned** |

**One capture group and not two, and the omission is argued rather than
overlooked.** The start-time field is present and would guard against PID reuse,
and it is deliberately **not** taken. Two grounds. First, parity: the `.run`
record grammar carries no start-time guard either, so taking one here would make
the worktree path stricter than the record path for no stated reason. Second,
and decisively, the **error direction is already safe**: a stranded worktree
whose recorded pid has been reused classifies **live**, and a live
classification refuses and tells the session to wait — it never authorises a
removal. Dropping the guard errs toward refusing, which is the direction a
fail-closed boundary wants. The strengthening is available and is filed rather
than banked.

### (2) The refusal reports the class per path and names the remedy each class actually needs

The boundary refusal keeps its exit code and its refusal set and changes what it
**says**: one line per linked worktree carrying its class, and a remedy matched
to that class. **{design-bearing}**

**The refusal set does not narrow, and this is stated because it is the first
thing a reader will check.** Both classes still refuse. A **live** worktree
refuses because §bin/enter-stage.sh's predicate is a property of the boundary —
an in-flight dispatch must not straddle one — and an **orphaned** worktree
refuses because it is residue that must be cleared before the boundary is
crossed. What was one message for two situations becomes two, and the guidance
stops being uniform advice a session had to re-derive per path:

- **live** — name the holding pid and say *wait*; do not remove. The existing
  help text's `--force` suggestion is actively wrong for this class and stops
  being offered for it.
- **orphaned** — say the holder is gone, so the lock is a statement of fact that
  has become false. This is the class `--force` exists for, and it is now named
  only where it is true.
- **unclassified** — today's message verbatim, which is also the whole
  behaviour an unconfigured consumer sees.

**Why the reap still stays a session act, and the old ground is replaced rather
than repeated.** §bin/enter-stage.sh's stated ground was "this kit does not
remove a directory whose liveness it cannot establish". That ground has been
**discharged** — liveness can now be established — and the reap stays manual on
a *different* ground that the discharge does not touch: the entry's own ground
(i), that a worktree can hold commits existing nowhere else, so a wipe is
destructive rather than merely wasteful. Liveness answers *is anyone working
here*; it does not answer *would removing this lose anything*. Restating the
retired ground would leave the SPEC asserting something the experiment
falsified, so the section carries the new one.

**The loss question is answerable too, and the refusal answers it rather than
leaving it to the session.** For an orphaned path the refusal additionally
reports whether the worktree's tree is dirty and whether its branch carries
commits unreachable from the main checkout's HEAD — two git reads, no vendor
vocabulary, and precisely the two facts the 2026-08-18 probe had to establish by
hand for four paths. A path reported clean **and** commitless is the case where
removal is lossless by construction, which is what a session needs told rather
than re-derived.

### (3) An orphaned worktree is surfaced mid-iteration, advisory, closing the entry's honest limit

The worktree read runs at **every** stage entry rather than at the iteration
boundary alone; away from the boundary it is **advisory** and reports orphaned
paths only. **{design-bearing}**

This is the entry's stated honest limit — "the refusal bounds accumulation per
iteration and no gate reads it, so residue inside an iteration is still
unsurfaced" — and it was unclosable before delta 1. An unclassified mid-iteration
report would have been useless or harmful: a live worktree mid-iteration is the
**normal** state whenever a lead has a dispatch in flight, so a report that
could not tell live from orphaned would either cry wolf at every stage entry or
refuse legitimate in-flight work. With the classification it is safe and cheap.

Two properties keep it from becoming a second refusal:

- **Advisory, never a refusal, away from the boundary.** Nothing about a
  mid-iteration orphan justifies refusing a stage entry, and the boundary
  refusal already bounds accumulation per iteration.
- **Orphaned only.** A live worktree mid-iteration is reported nowhere,
  because there is nothing for the entering session to do about it.

With the knob unset there is nothing classifiable, so an unconfigured consumer
sees no new output at all — the same additivity delta 1 has.

**No gate reads this, and that is precedent rather than an omission.**
§check-stage-entry records that the boundary-precondition family has
deliberately never had a gate sibling, and that reasoning is unchanged: these are
properties of a *moment* rather than of the tree, and a gate over tree state
cannot see a worktree at all.

### (4) This repo's consumer config supplies the pattern

`scripts/lifecycle-config.sh` sets `LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE` to the
pattern matching this harness's lock reason, with a `comment-tier-exempt:`
rationale naming the seam, exactly as the other consumer-vocabulary knobs in
that file and in `scripts/canon-config.sh` do. **{mechanical}**

## Producers and consumers

**The liveness class (delta 1)** — a derived value, not stored state.

- *Producer:* `lifecycle-kit/bin/enter-stage.sh`, in the block at lines 301-316
  that already parses `git worktree list --porcelain`. It reads one further
  porcelain field (`locked`) from a command it already runs, applies the
  configured pattern, and calls the existing pid predicate. **Enabling config
  actually set:** `LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE` is set by delta 4 in this
  repo's own `scripts/lifecycle-config.sh`, so the classified path is live here
  and not only in fixtures; every other consumer keeps the empty default and the
  unclassified path, which is the shipped behaviour today.
- *Consumer:* the **entering session**, through the refusal text (delta 2) at
  iteration-boundary entry and through the advisory (delta 3) at every other
  stage entry. There is no second consumer and no file: the class is computed,
  printed and discarded, so no new state is persisted and nothing new must be
  reclaimed, tiered or swept.
- *Named reader for every field:* the class has exactly two fields and both are
  printed and read at the transitions above — the **class token**, read by the
  session to choose between waiting and reaping, and the **captured pid**, read
  by the session to identify the holder it is waiting on. The loss report of
  delta 2 adds two more, **dirty** and **unreachable-commits**, whose reader is
  the same session at the same transition, deciding whether removal is lossless.
  No field is carried that this list does not name a reader for; in particular
  the lock reason's **start time** is matched by no capture group and is
  therefore not read, argued at delta 1.

**The knob (deltas 1 and 4).**

- *Producer:* `lifecycle-kit/lib/stages.sh`, which resolves and validates every
  `LIFECYCLE_KIT_*` knob; this one takes the empty default at line ~70 alongside
  `LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK` and a validator arm rejecting a
  pattern that is not a valid ERE or that declares no capture group — a
  malformed pattern is a fail-closed config refusal, never a silent
  everything-unclassified.
- *Consumer:* `bin/enter-stage.sh` at both transitions above.

**Narrowing check (canon-kit/SPEC.md §The causal-completeness check, point 5).**
No delta narrows a corpus. Delta 1 is additive with an empty default, delta 2
changes message text at an unchanged refusal set, delta 3 adds output at
transitions that produced none, delta 4 sets a knob. Each affected reader's
**red condition** is enumerated anyway, because the point-5 rule is about the
argument being made rather than about the author's confidence in it:

- `lifecycle-kit/gate-tests/enter-stage.test.sh` — reds when an entry's exit
  code, written state or emitted text differs from the asserted one. **Not
  monotone** (it asserts exact text on the refusal path), and it is the one
  reader this amendment must move: the boundary-refusal case's expected text
  changes under delta 2, and new cases are owed for each class and for the
  empty-knob default.
- `check-stage-entry` — reds on its three assertions (predecessor stamp,
  drain-entry queue-empty, audit trigger). **Monotone and cleared by
  inspection**: no delta touches a stamp, a queue section or an amendment
  corpus, and §check-stage-entry already records that this gate does not read
  the boundary-precondition family.
- `check-stage-evidence` — reds on stamp grammar or name-axis disagreement.
  **Monotone and cleared by inspection**: no delta changes what is written to
  `.workflow/WORKFLOW-STATE.txt`, and both new output paths write nothing.
- `check-knob-citation` — reds on a knob defined and never cited in its owning
  SPEC's knob roster. **Not monotone** — its red condition is a *zero* count for
  a knob's citation, so a new knob reds it until §Layout and configuration
  carries the roster entry. That entry is an update target below.
- `check-knob-default-coupling` — reds when a knob's default in code and its
  default as stated in the SPEC disagree. **Not monotone** for the same reason;
  the empty default must be stated in the roster entry, not merely implied.
- `check-comment-tier` — reds on a non-directive comment. **Monotone** with
  respect to this change, but delta 4's config line carries a
  `comment-tier-exempt:` reason as its sibling vocabulary knobs do, so the
  exemption is authored rather than discovered at commit.

## Existing sections updated

- `lifecycle-kit/SPEC.md` §bin/enter-stage.sh, the linked-worktree paragraph —
  the classification, the two-class refusal text, and the mid-iteration
  advisory; and the sentence "this kit does not remove a directory whose
  liveness it cannot establish" is **replaced** rather than kept, since the
  experiment discharged its premise and the manual reap now stands on the
  loss ground instead (deltas 1, 2 and 3).
- `lifecycle-kit/SPEC.md` §bin/enter-stage.sh, the two stated bounds — the
  per-iteration ceiling bound is narrowed by delta 3, which surfaces
  within-iteration residue; the dangling-branch-ref bound is unchanged and is
  re-stated as still open (delta 3).
- `lifecycle-kit/SPEC.md` §Layout and configuration — the knob roster gains
  `LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE` with its empty default and the seam
  reason for the default being empty, beside
  `LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK` (deltas 1 and 4).
- `lifecycle-kit/SPEC.md` §lib/stages.sh — the config validator's roster gains
  the pattern's fail-closed arm (delta 1).
- `delegation-kit/SPEC.md` §The delegation model — the section that owns the
  serialize-or-worktree rules and the ground for refusing a reap in the dispatch
  guard; it gains the experiment's result, which is what makes a *boundary*
  reaper designable where a dispatch-guard one was refused (deltas 1 and 2).
- `evidence-kit/SPEC.md` §check-producer-liveness — the pid predicate gains a
  second named caller outside the `.run` path, so its reuse is recorded where
  the predicate lives rather than only where it is used (delta 1).
<!-- update-target-exempt: a confirmation re-read owned by no delta — the deltas add no projection and no generated surface, and claiming this bullet for one of them would assert a write to a roster that takes none -->
- `docs/site-architecture.md` §Generated projections — re-read at merge to
  confirm no generated projection reads the boundary check's output.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`), discharged at the iteration
      rather than at this commit, a sibling amendment being in flight for it.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. The retired **liveness-cannot-be-established**
      ground is the specific removal to chase: it must survive nowhere.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Fixture pair moved with the text** — `enter-stage.test.sh` carries a
      case per class plus the empty-knob default, per the narrowing analysis.
- [ ] **Re-derived, not cited** — the lock-reason format and the start-time
      equality re-probed against a live dispatch at build, not carried from this
      file.
