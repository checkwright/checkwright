The `spec` (amendment-authoring) stage of an iteration — the generative half of
design, split out from `scope` on a roster that carries a dedicated authoring
stage. The ontology: **scope bounds the units; `spec` authors the amendments;
the audit stage independently verifies them.** Author the design amendment(s)
for the feature units this iteration promotes, pair each into the active queue,
and recommend the next stage. Exit condition: *<exit-condition: your spec exit
condition — every promoted feature carries a `[spec:]` ref resolving to an
amendment on disk, and each amendment's causal-completeness check is green>*.

## Trigger (spec is trigger-gated)

Run `spec` when this iteration promotes ≥1 unit that carries **authoring** —
genuine design reasoned and recorded before build. Two shapes qualify: a
**feature** (its amendment is the authoring), and a **debt unit whose design is a
ruling** — a placement, seam, or among-alternatives decision, not a mechanical
convergence on a name the spec already carries. An iteration with neither — every
unit a mechanical debt convergence — skips `spec` entirely (scope advances to the
next stage), exactly as the audit stage is skipped when no cross-component
amendment exists. The trigger is **not** the feature/debt litmus, and it is
**procedural** — scope's next-stage recommendation.

The two shapes differ in **where the authoring lands**, and only the feature
shape is gate-backstopped. A feature entry carries a `[spec: <ref>]` only once
this stage has authored the amendment file, and the bidirectional rule reds any
`[spec:]` ref resolving to no file, so a skipped `spec` cannot ship a feature
without its amendment (the same procedural-plus-one-gated-backstop shape as the
audit stage's own trigger). A **debt ruling** authors **directly into the
governed surface it converges** — the canonical spec section or skill template —
with **no amendment file and no `[spec:]` tag**, because the pairing gate reds a
`[spec:]` outside a feature section (canon-kit/SPEC.md §check-amendment-queue); it
is un-backstopped by that gate, its correctness resting on scope's routing and
the converged surface's own gates.

**First step — stamp evidence.** Run lifecycle-kit's `bin/enter-stage.sh spec`:
it appends `<iteration> spec <session-id> <date>` to
`.workflow/WORKFLOW-STATE.txt` (required by `check-stage-evidence`; the stamp
proves invocation, not faithful execution), reading `<session-id>` from
`bin/session-id.sh` (the newest transcript — never hand-picked), using
`date +%F`, and refusing (writing nothing) if `check-stage-entry` is red. `spec`
is **not** the iteration boundary, so it **appends** — only the first stage
resets the evidence file. That stamp *is* the transition — the last stamp is the
stage cursor, so nothing flips and no queue write is involved. Commit the stamp
on its own.

Because this is a **fresh session** — a fresh prompt cache, on the tier the
lead chose — the causal-completeness authoring no longer re-reads scope's whole
survey context through every authoring turn; the exploratory sweep dropped at
scope's session boundary. That drop is the point of the split.

## Session ritual

*<ritual: your spec ritual: hold the provenance seam and config-via-env
convention; author each promoted feature's amendment against the amendment
lifecycle and causal-completeness contract your canon-kit owns; name the
queue-entry grammar and `[spec:]` ref-resolution owners the promotion writes
against.>*

**Verify causal completeness before declaring an amendment ready.** Every new
state, event, or interface the amendment introduces names its **producer** (what
code path or trigger creates it — and that the producer's enabling config is
actually emitted everywhere it must be), its **consumer** (what receives it, by
what mechanism), updates any **existing integration prose** describing the prior
flow, and gives every new field a **named reader** (a field with no named reader
should be removed). Survey those readers across the **whole component set**,
never a hand-picked subset, and never silence a probe's stderr — a `2>/dev/null`
on a path grep reads a bad path as "no reader", manufacturing the false negative
that hides a cross-component reader.

**Writing the amendment *is* promoting the deferred entry.** Land the
`[spec: <ref>]` ref on the feature entry and the amendment file itself in **one
commit**, satisfying canon-kit's bidirectional rule — without the pairing,
design rationale and ruled-out alternatives get re-derived under build pressure.
scope has already run the feature/debt litmus at filing and bounded the unit
set; this stage authors, it does not re-triage.

Close by **recommending the next stage**: the trigger-gated audit stage when one
of its triggers fired this session (an amendment changing ≥2 components'
contracts is one — `check-stage-entry` will demand the audit stamp at the next
stage's entry), otherwise the build stage. A `spec` that authored a
cross-component amendment should say so rather than let the downstream entry
discover it.
