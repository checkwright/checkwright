# SPEC amendment: eighth-cohort

The eighth port cohort — **the first one selected by running the selection rule
rather than by reading candidates off the tree.**

**This amendment specifies the cohort's *derivation*, not its membership, and
that is deliberate.** Its members are selected from
`bash gate-sdk/bin/port-blockers.sh --group`, the arm `SPEC-port-group.md` adds
and which does not exist yet. Naming members here would mean picking them the
way the previous seven were picked — by reading — which is the practice
`port-corpus-grouping-census-unbought` exists to end. The precedent for
specifying a derivation instead of a roster is this spec's own: §The first
cohort, and the rule that selects the next already rules that "**A primitive's
*remaining* consumers are derived, never recorded here**", on the ground that a
recorded roster rots at every cohort.

**The sequential dependency is a build-ordering constraint, and it is stated
because the two units share a surface and read as parallelizable.**
`SPEC-port-group.md` must land — the arm built, its output read — before this
unit's selection step can run. The two cannot be batched as parallel siblings
even though both touch `gate-sdk/bin/port-blockers.sh` and
`gate-sdk/SPEC.md`.

## What changes

### 1. The cohort is selected by running the tool, and the run is recorded

The selecting session runs `--group`, takes **the largest decidable group whose
members clear the criteria**, and records the tool's output as the evidence that
the selection rule was applied. **[design-bearing]**

What "recorded" means here is narrow and worth stating, because the tool is
advisory and its output is not stable across tree changes: the cohort's spec
section records **the members it selected, the group key they shared, the group's
size, and the undecidable count at selection time** — not the whole report. The
undecidable count is the load-bearing one: it is the bound on the claim that this
group was the *largest*, and a cohort recording the members without it asserts
the ordering rule was satisfied while discarding the only evidence of how well.

### 2. `scripts/`-declared gates are excluded from this cohort

A member whose declaration resolves under `scripts/` is **out of scope for the
eighth cohort**, however the grouping ranks it. **[design-bearing]**

The ground is that their **destination is unruled**.
`consumer-gate-port-disposition` records the fork — port into `native/` and ship
another project's repo rules in every adopter's binary; stand up a second,
consumer-owned crate; or justify 13 shell residues one by one — and that entry
is deferred and unpromoted this iteration, with the seam ruling explicitly left
open. Those 13 gates are 21% of what the port has left, so a grouping run over
the whole registry can plausibly surface one of them at or near the top. A
cohort that took such a group would stall mid-build on a question that wants an
operator rather than a builder, which is the failure mode criterion 7 exists to
prevent, arriving through the seam instead of through a dependency.

The exclusion is **scoped to this cohort and is not a ruling on the seam** — it
does not choose among the three ways out, and it does not make `scripts/` a
protected category, which TRAJECTORY.md §PRIORITY DIRECTIVE forbids. It defers
those members to the cohort that follows the seam ruling.

### 3. What the cohort inherits unchanged, cited rather than restated

The per-member port procedure is §Porting a gate to the binary substrate; the
payload rule is §Consumer payload; the criteria are §The port-candidate
criteria. **[mechanical]** This amendment adds nothing to any of them and
restates none of them.

Two standing obligations that bite per cohort and are named so the build session
does not rediscover them: parity is proved **while both implementations still
exist**, since §check-gate-substrate-parity assertion A forbids a descriptor and
a script coexisting in one resolve dir; and the shell original is **deleted** for
a ported member rather than left running beside it.

### 4. Criterion 5's aggregate price is measured, not reasoned

The cohort records the **binary-less residual** — the roster of members a
consumer whose payload carries no artifact for its host loses, and its count —
measured with `installer_smoke`'s binary-less leg, with this amendment's ruling
on whether the roster it grew is acceptable. **[design-bearing]**

Criterion 5 is explicit that "**N members each individually runnable is not a
discharge**", and that a cohort may not land **unpriced**. The measurement
ordering it fixes is inherited rather than rediscovered: the leg packs a payload
stamped with a commit and refuses a dirty worktree, so the measurement runs
**after this cohort's own commit**, from a clean checkout of it.

**A cross-unit interaction the build session must not miss.** This iteration
also carries `installer/SPEC-pack-root.md`, which changes how the packer
resolves the tree it packs. Criterion 5's own clause — "**When the instrument
rides the same iteration as the cohort, the measurement waits for it**" — binds
here on its face: the instrument's invocation contract is being repaired in the
same iteration as the cohort it must measure. The residual is therefore measured
**after** `SPEC-pack-root.md` lands, and the clean-checkout instruction that
clause carries ("reached by cwd, not by path") is the one that amendment
retires — so a session following the pre-repair instruction and a session
following the post-repair one will do different things. Measure after, and follow
the repaired contract.

### 5. The acceptability judgment is this amendment's to make, and is made in advance

If the measured residual grows by **zero**, the cohort lands on that finding.
If it grows, the cohort lands **only** with one of criterion 5's three designed
answers named in the cohort's spec section: restore the class shell-side, make it
binary-gated by a declaration the adopter receives, or accept and document.
**[design-bearing]**

Deciding the *rule* in advance and the *instance* at build is the split criterion
5 already draws — it says the number is machine-derived and machine-asserted
complete, while "what stays prose is the **judgment**". This delta fixes the
judgment's form so that a build session facing a grown roster has a decision
procedure rather than an open question, without pretending to know a number that
has not been measured.

### 6. The terminal queue move is a demotion, not a Done move

On completion, `native-gate-port-remaining-corpus` drops its `[spec:]` tag and
**returns to the deferred section** under `[design-pending]`, with its
`[roadmap: now/reliability]` tag intact. **[mechanical]**

This is canon-kit/SPEC.md §Merging an amendment step 4's corpus branch: the
entry's deliverable is the whole corpus and this amendment delivers one
increment. A Done move would assert a finished port with members still unported
**and** silently drop the item from the public roadmap projection, which reads
`[roadmap:]` off live entries. It is stated because that contract has no gate
behind either half — the wrong terminal move reds nothing and is found only by a
later reader. The entry has taken this exact demotion at each of the last five
cohorts.

## Producers and consumers

**This amendment introduces no new state, event, field or interface.** A port
cohort re-implements existing rules on the compiled substrate; the descriptors,
the dispatch seam, the manifest format and the parity harness all exist and are
unchanged by it. The causal-completeness check is therefore discharged against
the two things the cohort *does* produce:

- **Each ported member's `.gate` descriptor.** Producer: the build session,
  writing it beside the deleted `.sh`. Consumer: `gate_resolve` at dispatch,
  reaching `gate_command` and the binary subcommand — the mechanism §Porting a
  gate to the binary substrate specifies, unchanged. Every descriptor's reader
  already exists, which is what makes a port a port rather than a feature.
- **The recorded selection evidence and residual** (deltas 1 and 4). Producer:
  the build session, into the cohort's spec section. Consumer: **the session
  cutting the ninth cohort**, which reads the undecidable count to know how much
  of the corpus the eighth cohort's "largest" claim was blind to, and reads the
  residual roster to know what value class is already thin. This is the reader
  that makes recording it worth doing; without a named next-cohort reader these
  are ceremony.

**This delta narrows corpora, and the narrowing is real.** Deleting each ported
member's `.sh` removes files from every gate-source walk in the tree. The red
conditions, not the subjects:

- **`check-shellcheck`** — reds when ShellCheck reports a finding, and separately
  reds at exit 2 when its target set is **empty** (`check-shellcheck.sh:41-43`,
  "no `*.sh` found under: … — nothing to lint"). That is a *reds-on-finding-none*
  condition, the first of the three non-monotone shapes point 5 names. It is
  clear here only because the walk spans every kit's `lib/`, `bin/`, `checks/`
  and `templates/` and cannot empty from one cohort of `checks/` deletions — a
  ground that must be re-checked, not inherited, by any cohort large enough to
  empty a kit's `checks/`.
- **`check-gate-fixture-coverage`** — its subject is a gate's fixture pair. A
  ported member's pair is **retained**, so the deletion of the `.sh` must not
  orphan it. Enumerated because "we deleted the script" and "we deleted the
  gate's coverage" are one action away from each other.
- **`check-gate-substrate-parity`** — assertion A reds on a descriptor and a
  script **coexisting** in one resolve dir. Its red condition is therefore
  *satisfied* by the narrowing and violated by a half-done one, which is the
  inverse of the reflex.
- **`check-graph`, `check-enforcement-fresh`, and the generated pre-commit hook**
  — red on staleness against a changed manifest set. Every ported member changes
  the declaration the manifest is read from, so all three stale by construction
  and are regenerated with the cohort; each prints its own regen command on red.
- **`check-settings-paths`** (this iteration's `context-kit/SPEC-settings-paths.md`)
  — **reds on a committed allow entry naming a deleted `.sh`.** This cohort is
  precisely the event that creates such entries: the two live dead entries were
  left by the two previous cohorts. So this cohort will create more, and the new
  gate will red on them, and the fix is an **operator** settings edit the build
  session may not make. The interaction is named here because the two units are
  in the same iteration and neither entry mentions the other.

## Existing sections updated

- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** —
  owned by deltas 1, 2, 4, 5. It gains a new cohort subsection recording the
  members, the shared group key, the group size, the undecidable count at
  selection, the `scripts/` exclusion and its ground, and the measured residual
  with its acceptability ruling. The exclusion in delta 2 is recorded **in the
  cohort's own subsection**, not in the selection rule, so it does not read as a
  standing amendment to the rule.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — owned
  by delta 1. Assertion C's disposition table is keyed on members whose re-run
  trigger reaches a gate declaration; each newly ported member is re-derived
  against it, and any member the derivation newly selects owes a row.
- **`scripts/gates.list` and `native/`** — owned by delta 3: one subcommand and
  one descriptor per ported member, per the standing procedure.
- **`.workflow/validate-baseline.txt`** — owned by delta 4. It carries the held
  `installer_smoke fail` row that keeps an unpaid aggregate price visible; the
  row's disposition is revisited when this cohort's price is paid.

## Definition of Done

- [ ] **Causal completeness** — no new state or interface; each ported
      descriptor's producer and consumer are the existing dispatch path, and the
      recorded selection evidence has a named next-cohort reader. Every narrowed
      corpus's reader has its **red condition** enumerated, with the
      reds-on-empty and inverse-condition cases named rather than cleared by the
      "a narrower corpus can only remove violations" reflex.
- [ ] **Selection evidence recorded** — members, group key, group size and
      undecidable count, from an actual `--group` run.
- [ ] **Criterion 5 priced** — residual measured with the binary-less leg, after
      this cohort's commit and after `installer/SPEC-pack-root.md` lands, from a
      clean checkout; acceptability ruled per delta 5.
- [ ] **Parity proved while both implementations existed** — per member, before
      the `.sh` is deleted.
- [ ] **Merged with no information lost** — the cohort subsection integrated
      into §The first cohort, and the rule that selects the next.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      gate-sdk (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather than
      at the commit while `SPEC-port-group.md` is in flight.
- [ ] **Terminal move is a demotion** — `native-gate-port-remaining-corpus`
      returns to deferred under `[design-pending]` with `[roadmap:]` intact, not
      to `## Done`.
- [ ] **Gaps filed** — cross-component gaps found during the work filed as debt
      tasks.
