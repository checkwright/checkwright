# SPEC amendment: multi-operator-semantics

The contributor altitude of the coordination map — a second operator of the
methodology running their own concurrent iteration on the same repo. The
other altitudes are already ruled and stay untouched: sub-agents within one
session (delegation-kit's serialize-or-worktree rules) and sessions within
one iteration (the lead, lifecycle-kit/SPEC.md §templates/lead.md — one live
iteration, stages serialized through the flip+stamp protocol). Fork
contributors are out of scope: an outside PR never flips the header or
stamps state — it only has to pass the battery in CI, and the
branch-protection/ruleset story rides **`hosted-attestation-service`**.

## What changes

- **The topology ruling — the state surfaces stay single-writer; concurrency
  is git topology.** The header line, the evidence file, and every
  boundary-truncated surface are *iteration-scoped*: an iteration owns
  exactly one branch (its home branch) and every flip and stamp lands there.
  One live iteration per branch — the second concurrent operator cuts a
  branch at their scope entry; the integration branch (master here) is the
  degenerate single-operator home, which is why this repo's own dogfood
  changes nothing. Branch naming is prose guidance (name the branch after
  the iteration), not mechanism — no knob, no gate, demand-gated until a
  pilot shows ambiguity. Ruled out: per-operator state files or stamp
  attribution fields (multi-writer surfaces, a new stamp grammar — the
  deviation-transitions doctrine of composing existing mechanism bars both;
  operator attribution already rides the git author on every flip commit);
  a lock or lease on the integration branch (adds state the kit refuses to
  own, and git already provides the isolation).
- **The merge-supersede rule, derived.** At any branch merge, the
  iteration-scoped surfaces resolve wholesale to the *arriving* (checked-out)
  iteration's version — the other side's content is per-iteration scratch
  the boundary doctrine already declares dead (git history is the permanent
  audit trail). The supersede set is **derived, never maintained**: it is
  exactly what `bin/enter-stage.sh` truncates at the iteration boundary —
  `LIFECYCLE_KIT_STATE_FILE`, `LIFECYCLE_KIT_LESSON_EVIDENCE_FILE`, and the
  `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` members (this repo: the evidence-kit
  manifest wired in `scripts/lifecycle-config.sh`). The queue file is
  deliberately *not* in the set: its body (backlog sections, lessons) is
  shared content that merges like any prose, and only its header line is
  iteration-scoped — resolved by hand to the arriving iteration, with a
  wrong resolution going red at the next commit because `check-stage-evidence`
  requires header↔stamp agreement and the state file took the arriving side
  by driver. Held-constant baselines and append-across-iterations evidence
  (release-sweep) keep normal merge semantics: their conflicts are real
  disagreements. A consumer with a *tracked* shared append log points it at
  git's built-in `union` driver themselves — sanctioned shape, no kit
  mechanism. (Re-verified against the tree: this repo's friction logs are
  gitignored per-checkout scratch — they never merge and need no rule; the
  queue entry's "committed scratch logs" premise was stale.)
- **`.gitattributes` (repo root) — the supersede rule mechanized.** Each
  supersede-set path carries `merge=iteration-scoped`; the driver definition
  (`git config merge.iteration-scoped.driver true` — keep ours) is per-clone
  config installed by `bin/install-lifecycle.sh` as a step beside its
  registration block, the `install-hooks.sh` opt-in class. Honest limit: on
  a clone without the driver installed the attribute is inert and the file
  conflicts normally — the SPEC rule then governs the hand resolution, so
  the uninstalled path degrades to judgment, never to silence.
- **`checks/check-merge-attrs.sh` (lifecycle-kit, new gate)** — invariant:
  bidirectional parity between the derived supersede set and the
  `merge=iteration-scoped` lines in the consumer's `.gitattributes`. The
  reverse direction is the safety edge: an ours-driver attribute on a path
  *outside* the derived set silently discards merge content on a real
  surface, so a smuggled line is red, not config. Ships the skeleton-derived
  contracts: output/fail-closed/self-lint, a `good/`+`bad/` fixture pair,
  registration in `scripts/gates.list`; a `gate-tests/` scenario drives a
  real two-branch merge in a sandbox repo and asserts the driver resolves
  the state file to the arriving side. Writer/asserter split: the installer
  emits the attribute block (marker-bounded, `lib/inject.sh`), the gate
  verifies it — the `gen-pre-commit.sh` ↔ `check-graph` precedent.
- **The close-merge protocol** (joins §Deviation transitions as the
  concurrent-close shape): iteration boundaries serialize on the integration
  branch. The closing operator reconciles *on the iteration branch* — merge
  the integration branch in; the driver resolves the iteration-scoped
  surfaces, humans resolve content conflicts — re-runs the full battery
  green, then lands fast-forward-only on the integration branch. The
  integration branch never hosts a conflict resolution, so "arriving
  iteration" is always well-defined (ours on the iteration branch), and
  every merged tree passed the battery post-reconcile.
- **Who may flip, restated at this altitude:** unchanged — the arriving
  stage session flips, and only on its own iteration's home branch. A
  session never flips a branch whose iteration it is not driving;
  cross-iteration discoveries (a lesson, a deferred filing) land on the
  discoverer's own branch and reconcile at merge.

## Producers and consumers

- **`merge=iteration-scoped` attribute lines** — producer:
  `bin/install-lifecycle.sh`'s attribute step (idempotent marker block,
  re-run after a `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` change; enabling config
  is this repo's own `.gitattributes`, committed by the build). Consumers:
  git's merge machinery at every branch merge, and `check-merge-attrs`,
  which re-derives the set from `lib/stages.sh` config and byte-checks
  parity each commit. Every line has both readers at a named transition
  (merge time; pre-commit).
- **`merge.iteration-scoped.driver` git config** — producer:
  `bin/install-lifecycle.sh` (per-clone, opt-in — the same deployment class
  as `install-hooks.sh`, so a deployed configuration that sets it exists
  the moment a consumer runs the installer). Consumer: git's merge
  machinery when an attributed path needs a three-way merge. Not readable
  by a pre-commit gate (per-clone state); its absence degrades to a normal
  conflict under the SPEC rule — recorded honest limit, not a silent hole.
- **The close-merge protocol** — producer: the closing operator's close
  session (the close skill's exit prose points at the SPEC section when a
  home branch differs from the integration branch). Consumer: the
  integration branch's battery — the reconcile commit re-fires every
  queue/state-coupled gate, which is what makes the header hand-resolution
  enforceable (`check-stage-evidence` header↔stamp agreement).
- No new stamp grammar, no new queue tag, no new evidence file — the design
  adds no state surface, which is its causal-completeness core: every
  existing producer/consumer pair (enter-stage, the stage gates, the drift
  report) keeps working per-branch unmodified.

## Existing sections updated

- lifecycle-kit/SPEC.md §The state machine — the single-writer statement
  gains the branch-scoped qualification and a pointer to the new
  §Multi-operator semantics section (the merged home of this amendment).
- lifecycle-kit/SPEC.md §Deviation transitions — gains the close-merge
  shape beside abandon/split/reopen (same "composes existing mechanism"
  framing).
- lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — the attribute-block and
  driver-config steps join the installer contract; `smoke/install.sh`
  extends to cover them.
- lifecycle-kit/SPEC.md §Layout and configuration — `.gitattributes` joins
  the consumer-surface roster; no new knob (the supersede set derives from
  existing knobs).
- docs/orchestration.md §What is built, and what is roadmap — the
  multi-operator bullet flips from deferred to landed, citing the SPEC
  section; the three-altitude boundary note — already relocated out of the
  queue entry into this amendment's opening paragraph at scope's promotion —
  reaches the SPEC section by the merge itself (a design ruling leaves the
  queue).
- queue-kit/SPEC.md is *not* edited: the queue file's merge behavior is
  ruled here because the header line is lifecycle-kit's governed surface;
  queue-kit's body grammar is untouched. drift-kit/SPEC.md is *not*
  edited: the friction logs are gitignored per-checkout scratch (§The
  knowledge-friction loop already states the pattern) and never merge.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as
      one coherent document a reader who never saw the amendment can use
      alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks (a build-time causal gap is resolved that session, not
      deferred).
