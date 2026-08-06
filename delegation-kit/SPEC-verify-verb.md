# SPEC amendment: the verify verb and evidence-producer check routing

Pairs with the queue entry `validate-verb-collision-and-check-routing`.

Two coupled defects with one root: the delegation discipline verb collides with
the `/validate` stage noun, and that collision misroutes the lead's
post-delegation check onto the evidence producer.

## (1) The verb collision

The lifecycle stage `/validate` shares its term with the delegation discipline
"validate after every agent commit". The verb is performed right *before* the
stage, so completing the lead-side check reads as completing the stage — a lead
conflated them and nearly skipped `/validate`, jumping build to close.

**Ruled: rename the discipline verb to `verify`.** Renaming the stage is rejected
as invasive for a confusion the verb rename resolves, and the stage name is
load-bearing across stamps, gates, skills and evidence artifacts.

`verify` rather than `re-check` because the tree has already converged on it
without being told to: the lead template describes the lead as having "no
verification discipline", the delegation protocol speaks of "post-commit
verification", and the queue entry that files this defect calls the rule "the
post-delegation verify discipline". The rename therefore ratifies existing usage
instead of minting a third word.

### The noun stays — `validate battery` is not renamed

This is a real fork the queue entry does not settle, so it is ruled here.
**Rename the verb; keep the noun.**

- The confusion the entry documents is between the *stage* and the *act*. Nobody
  skips a stage because a command set is called the validate battery; the battery
  genuinely is the suite set the `/validate` stage runs, so the name is accurate
  rather than colliding.
- `validate-battery` is a **binding slot name** in the consumer-binding grammar,
  so renaming it breaks every consumer's binding shim and is read by
  `check-skill-binding`. That is a large, gated blast radius bought for no
  reduction in the confusion actually attested.
- The entry's own scope bolds the word **verb**. Keeping the noun is inside that
  envelope, not a narrowing of it.

The result reads coherently: *you verify an agent's commit by re-running the
relevant gates and the consumer's validate battery.* The act and the artifact
have different names, which is the distinction that was missing.

### The rename runs off a roster, because the gate does not back it

Enumerated here so the sweep stays mechanical, and because **two of the four
non-owner citation sites are invisible to `check-spec-pointer`** — a mechanical
grep-and-rename is not backstopped, and assuming it is would leave a dangling
citation nothing reds on:

| site | gate-checked |
|---|---|
| `delegation-kit/SPEC.md` §Validate after every agent commit (the heading, owner) | — |
| `delegation-kit/checks/check-gate-tamper.sh` `# spec:` directive | yes |
| `docs/orchestration.md` §citation | yes |
| `delegation-kit/templates/agent-execution.md` §citation | **no** — a slot-bearing template self-excludes from the governed manifest |
| `CONTRIBUTING.md` §citation | **no** — the citation is markdown-link-wrapped, a shape the prose extractor does not match |

Plus the in-prose verb uses: `agent-execution.md` (the serialization bullet, the
serialize-by-completion sentence, the rule's own bullet lead-in),
`lifecycle-kit/templates/lead.md` (the dispatch-mechanics sentence, the sibling
-batch sentence, the acceptance-boundary sentence), `lifecycle-kit/SPEC.md`'s two
paraphrases of those, `CLAUDE.md`'s compound "validate-after-commit set", and
`README.md`'s protocol summary. The docs mirror is regenerated, never hand-edited.

The second unchecked site is a **gate defect, not this amendment's to fix** — the
prose extractor misses link-wrapped citations regardless of this rename. It is
filed to the gap inbox rather than folded in here.

## (2) The check-routing carve-out

The post-delegation check carves out no case for when the delegated unit's own
output **is** the evidence. There the naive "re-run the battery to check it" is
wrong: re-running the producer mutates or duplicates the committed evidence
rather than verifying it.

**The generic rule, which is delegation-kit's:** a verify step must not run the
producer of the artifact it is verifying. Where a dispatched unit's deliverable
is an evidence artifact, the verify is a **read** of the committed artifact,
never a re-execution of what wrote it. Re-running is safe and idempotent exactly
when the unit's output is *work* and the check's output is *a verdict about that
work*; it stops being safe the moment those coincide.

**The lifecycle instance, which is lifecycle-kit's**, because only it knows a
stage roster: when the dispatched stage is the evidence-producing stage, the
lead's verify is a read of the committed evidence manifest. This is the seam —
delegation-kit cannot name a stage without shipping one consumer's lifecycle
vocabulary as everyone's, and lifecycle-kit cannot state the generic discipline
without duplicating delegation-kit's rule.

**The two wrong re-runs are not equally harmful, and the wording must split
them.** A whole-gate battery writes nothing under the workflow directory, so
re-running it is inert on the evidence and merely wasted work. The evidence
producer is the sole writer of the manifest, so re-running *it* is destructive.
The routing defect is real for both; the corruption risk belongs only to the
second. Collapsing them would either overstate the harm of a wasteful re-run or
understate the harm of the destructive one.

## The honest limit, and why it is stated rather than left implicit

The prose fix **installs no oracle**, and recurrence is what establishes that
this matters: the defect fired in two consecutive iterations and an operator
caught it both times. The check class that would catch it is a gate over a lead's
*tool invocations*, and no scanner is buildable — a lead's choice of command
leaves no tracked artifact to read.

The nearest buildable proxy is `evidence-kit/SPEC-liveness-lock.md`, and its
coverage boundary must be stated precisely or the limit reads as closed when it
is not: the lock covers the **concurrent** case, a producer still running when
the next entry is stamped. It does not cover a **sequential** re-run — a lead
re-running the producer after it has cleanly exited holds no lock and reds
nothing. So detection for this defect stays human, and that belongs in the
binding rather than in a queue entry nobody reads at build time.

## What changes

Each delta carries its `{mechanical | design-bearing}` work class.

1. **The verb rename sweep** — {mechanical}
   Every site in the roster above, in one pass. Mechanical *because* the roster
   is enumerated here: the judgment (which occurrences are the discipline verb,
   which are the stage or an artifact name) is spent in this amendment, leaving
   execution to a fixed list plus the two gates that do fire.

2. **`delegation-kit/SPEC.md` gains the generic no-producer-re-run rule** —
   {design-bearing}
   Stated in the renamed §Verify after every agent commit, with the
   safe-versus-destructive split and the stated limit. Design-bearing: it is new
   governed vocabulary — the work-produced/evidence-produced distinction ships
   nowhere in the tree today — and the seam ruling about what delegation-kit may
   say without naming a stage is part of the delta.

3. **`lifecycle-kit/templates/lead.md` gains the lifecycle instance** —
   {design-bearing}
   The evidence-producing stage is verified by reading its committed evidence,
   never by re-running its producer, with the harm asymmetry named. Placed with
   the post-delegation verify discipline. Design-bearing for the same reason as
   delta 2's seam half: it must apply the generic rule without restating it.

4. **`lifecycle-kit/SPEC.md` records the stated limit** — {mechanical}
   That this routing rule is prose-only and human-enforced, and that the liveness
   lock covers the concurrent case only. Mechanical: the ruling is made above, and
   landing it is transcription into the section that already holds the kit's
   honest limits.

## Producers and consumers

This amendment introduces a **renamed governed name** and a **new rule**, not
runtime state.

- **The renamed name** — `delegation-kit/SPEC.md` §Verify after every agent
  commit. Producer: that heading, the rule's single owner. Consumers: the four
  citation sites in the roster, each of which must be updated in the same commit;
  two are read by `check-spec-pointer`, and the other two are read only by this
  roster, which is why the roster exists.
- **The no-producer-re-run rule** — Producer: the renamed section, unconditional
  kit mechanism with no knob. Consumers: (i) any supervising session choosing a
  post-commit check, at the moment the dispatched unit returns; (ii) the lead,
  via the lead template's lifecycle instance, at the same transition; (iii) a
  reviewer assessing whether a performed check was the right one.
- **Every distinction has a named reader.** The work-produced/evidence-produced
  split is read by the supervisor when selecting the check. The
  wasteful/destructive split is read by the same actor when it has already run
  the wrong one and must decide whether the evidence needs restoring — a reader
  that a collapsed wording would leave unserved.

No new field, artifact or message is introduced, so no field-reader roster is
owed.

## Existing sections updated

- `delegation-kit/SPEC.md` §Validate after every agent commit — renamed, and
  gains the carve-out plus the stated limit. Owned by deltas 1–2.
- `delegation-kit/templates/agent-execution.md` — the rule's bullet lead-in, its
  §citation, and the two in-prose verb uses in the serialization bullets. Owned
  by delta 1.
- `delegation-kit/checks/check-gate-tamper.sh` — its `# spec:` directive citation.
  Owned by delta 1.
- `lifecycle-kit/templates/lead.md` §The lead model and §Economics — the
  in-prose verb uses, plus the lifecycle instance of the carve-out. Owned by
  deltas 1 and 3.
- `lifecycle-kit/SPEC.md` — its two paraphrases of the lead template, plus the
  stated limit. Owned by deltas 1 and 4.
- `CLAUDE.md` §Agent execution — the compound "validate-after-commit set". Owned
  by delta 1.
- `README.md` and `CONTRIBUTING.md` — the protocol summary and the §citation.
  Owned by delta 1.
- `docs/orchestration.md` — its §citation and verb uses; the generated kit mirror
  under `docs/<kit>/` is **regenerated, never hand-edited**. Owned by delta 1.

## Out of the envelope, stated so build does not drift into it

- **The dispatch precondition is not this amendment's ground.**
  `lifecycle-kit/SPEC-dispatch-signal.md` governs the moment *before* a dispatch
  (how the lead knows stage N is over); this one governs the moment *after* (which
  check is safe to run on which kind of stage). They land in the same template
  and must not be merged.
- **The `check-spec-pointer` prose blind spot** found while sizing this rename is
  filed to the gap inbox, not fixed here.
- **`validate battery` and every artifact name** — `run-validate`, the evidence
  manifest, the `EVIDENCE_KIT_*` knobs, the `/validate` stage and its stamps —
  are explicitly out of the rename. A sweep that touches one of them has
  exceeded this amendment.

## Definition of Done

- [ ] **Causal completeness** — the renamed name and the new rule each have a
      named producer and named consumers; each distinction has a named reader at a
      named transition.
- [ ] **Rename roster fully executed** — every site in the roster updated in the
      same commit, including the two no gate will catch; `check-spec-pointer`
      green is necessary and *not* sufficient, and the DoD says so because
      treating it as sufficient is the specific way this delta fails.
- [ ] **Nothing renamed that was excluded** — the stage name, `validate battery`,
      and every artifact/knob name are untouched.
- [ ] **Merged with no information lost** — the seam split (generic rule in
      delegation-kit, stage instance in lifecycle-kit) survives the merge; a
      merged form that names a stage in delegation-kit has lost it.
- [ ] **Amendment deleted** — this file removed on merge; `ls delegation-kit/SPEC-*.md`
      returns none.
- [ ] **Removals propagated** — grepped every spec for names this change retired;
      nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks.
