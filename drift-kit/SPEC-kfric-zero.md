# SPEC amendment: kfric-zero

Closes `kfric-empty-log-ambiguity`. An empty knowledge-friction log is read as
*no friction* and is equally consistent with *no capture*, and the surface that
licenses the wrong reading is drift-kit's own: §The knowledge-friction loop step
3 says the KPI "trends the per-iteration count; it falls as the tier contract's
holes fill". That sentence is unsupported — a count that only ever counts what a
session chose to stamp cannot distinguish a filled hole from an unstamped one —
and it is the one a reader of the KPI table is entitled to rely on.

**The entry's fork is ruled to its third horn, and the other two are refused with
cause rather than left open.** *A corroborating signal* is cheap and weak, and
weak in the direction that matters: the prompt log records history archaeology
without recording what was re-derived, so it can suggest an unstamped event and
never identify one. *A per-stage capture prompt* is precisely the
standing-instruction tax context-kit's brevity machinery rejects, and drift-kit
already refuses its heavier sibling — the transcript scan — on a related ground.
What remains is the contract call the entry names: **whether the KPI is meant to
be trusted at zero.** It is not, and this amendment says so on the surfaces that
currently imply otherwise.

**The reader-side half is the whole of this unit, and the boundary is stated
rather than assumed.** `kfric-obligation-residency` is the writer-side twin —
the capture obligation never reaching a session that has no template carrying it
— and `recurrence-obligation-residency` is its structural sibling. Neither is in
this iteration's ruled unit set, and nothing here serves the writer. A reader
meeting the merged spec will find the honest limit stated and the writer's
carrier still missing; that is the correct reading of the tree, not an omission
this amendment hid.

## What changes

### (1) The trend claim is retired, and what replaces it is a reading rule

§The knowledge-friction loop step 3's "it falls as the tier contract's holes fill"
is **deleted**, not softened. In its place: the count is a **capture** count, so
it moves with two independent things — how much friction occurred and how much of
it a session stamped — and a **fall is not attributable to either**. A **zero
reading is not evidence of zero friction**; it is the reading an iteration
produces when nobody captured, and it is also the reading an iteration produces
when nobody re-derived, and the log cannot tell a reader which.
**{design-bearing}**

Retiring rather than hedging, because the sentence's failure is not that it
overclaims by a margin. Its subject — *the tier contract's holes* — is not what
the log measures at all, and a hedge on a claim about the wrong quantity leaves
the wrong quantity in the reader's head.

**The direction of the error is stated because it is the expensive half.** The
KPI reads **best** exactly where it is **least** trustworthy: an iteration where
capture discipline collapsed is indistinguishable from one where the tier
contract is complete, and the first is the one that needs acting on. A metric
whose failure mode looks like success is worse than an absent metric, which is
what makes this a contract defect rather than a precision complaint.

### (2) The zero reading carries its own limit at the point of reading

`drift-kit/kpis/kpi-knowledge-friction.sh` emits a **distinct line for a zero
count**, naming the non-inference in the value itself rather than leaving it to a
reader who has read §The knowledge-friction loop. The non-zero line keeps its
existing `(lower bound)` qualifier, which is true and stays. **{design-bearing}**

Three states, three lines, and the third is the new one:

| log state | line | what it asserts |
| --- | --- | --- |
| absent | `n/a (no knowledge-friction log)` | this tree runs no capture loop |
| present, non-empty | `<N> re-derivation(s) logged this iteration (lower bound)` | at least `N` occurred |
| present, empty | `0 logged — not evidence of zero friction; no capture floor exists` | nothing at all |

The third row is the amendment's point. Today an empty log yields the second
row's sentence with `0` in it, which reads as a measurement of zero and is a
measurement of nothing.

**`--trend`'s grammar does not move, and that is deliberate.** It keeps emitting
`kfric <N>` with `kfric 0` for the empty log. A trend consumer plots a series,
and changing a series' grammar for one of its values breaks comparability across
the change — the same reasoning §Bundled KPIs already applies to
`kpi-incident-recurrence`, where "a series spanning such a change is two series,
not one". The limit belongs on the human-read line, where a reader can act on it,
and not in the series, where it would only make the history unreadable.

### (3) The two refused alternatives become named non-targets

§The knowledge-friction loop records that a **corroborating signal** and a
**per-stage capture prompt** were weighed and refused, with the ground for each,
so the next session meeting the empty log does not re-open a settled call as if
it were an oversight. **{design-bearing}**

- *Corroborating signal* — refused for weakness, not for cost. The prompt log
  records that a session read history; it does not record **what fact** was
  re-derived, so it can raise a suspicion and can never resolve one. A signal
  that cannot identify an instance cannot correct a count.
- *Per-stage capture prompt* — refused on the surface that already owns the
  refusal. Step 2 of the loop states the equivalent rule for the *remediation*
  side ("Never a standing session-start instruction: that converts one
  re-derivation into a permanent per-session tax"), and the same reasoning binds
  the capture side. Recorded here because the rule currently sits under triage
  and reads as a triage-only rule.

**A third non-target, because it is the one this amendment is most likely to be
mistaken for.** Nothing here gives the loop a **floor** — an independent signal
that capture happened — and no cheap one exists. The honest consequence is that
the KPI stops asserting something it cannot support; it does not start supporting
it. That is the bound this claim ships with, and the entry's cost field is
unchanged by this unit: the one KPI measuring the tier contract's completeness
still reads best exactly when nobody is capturing, and now says so.

### (4) The lag tier's own definition separates *undercount* from *non-evidence*

§Bundled KPIs' `kpi-knowledge-friction` bullet currently reads "Lag by
construction: only what a session *noticed and logged* is counted, so the value
lower-bounds the real rate." That is true and it is not the whole obligation: a
lower bound of zero bounds nothing, so the lag label alone does not warn a reader
off the one reading that is wrong. The bullet gains the degenerate case
explicitly. **{design-bearing}**

Stated as its own delta because the lag section is where a reader comparing KPIs
looks, and because the same structure holds for `kpi-incident-recurrence`
directly below it — which already says "a recurrence nobody files is uncounted,
exactly `kpi-knowledge-friction`'s structure". That cross-reference makes this
bullet load-bearing for a second metric, so leaving the correction only in §The
knowledge-friction loop would leave the sibling's citation pointing at the old
reading.

## Producers and consumers

**The zero-state report line (delta 2)** is the only new interface.

- *Producer:* `drift-kit/kpis/kpi-knowledge-friction.sh`, on the branch where the
  log exists and its non-blank line count is zero — the branch that today falls
  through to the shared report line. **Enabling config actually set:** none is
  added. The knob it reads, `DRIFT_KIT_KNOWLEDGE_LOG`, already resolves to
  `${GATE_SDK_WORKFLOW_DIR:-.workflow}/knowledge-friction.log` in this tree, the
  log already exists there, and it already reads empty at an iteration's open, so
  the new branch is live on the next KPI run rather than reachable only in a
  fixture.
- *Consumer:* the KPI **report reader** — the close-stage session running the
  drift report, and any human reading the emitted table. It is read at the close
  transition, where the decision the line informs is whether to treat the tier
  contract as complete.
- *Named reader for every field:* the line carries one value, the count, whose
  reader is above. No field is added to `--trend`'s output, and that is the
  point of the delta's second half — the series consumer's parse is untouched, so
  no consumer must learn a new token to keep working.

**Deltas 1, 3 and 4 introduce no state, event or interface.** They are prose
obligations on surfaces that already exist, and their consumer is the reader of
those surfaces: a session reading §The knowledge-friction loop before capturing
or triaging, and a session reading §Bundled KPIs before acting on a number.

**Narrowing check (canon-kit/SPEC.md §The causal-completeness check, point 5).**
No delta narrows a corpus. The readers that move are enumerated by red condition
because two of them assert over the emitted strings:

- `drift-kit`'s KPI behavioural coverage — reds when an emitter's observed line
  differs from the asserted one. **Not monotone** (exact match), and it must move
  in the same unit: an empty-log arm asserting today's shared sentence goes red
  on delta 2. Listed as an update target.
- `check-measured-claim` / `check-unmarked-claim` — red on a governed prose
  surface asserting a measured quantity with no evidence marker, and on an
  unmarked claim. **Not monotone** (both hold minimum-marker conditions). Delta 1
  **deletes** an unsupported claim and deltas 1, 3 and 4 add prose to a governed
  SPEC, so both gates are live over the merged text and the wording must satisfy
  them at merge rather than at this file, which the manifest globs do not scan
  (canon-kit/SPEC.md §The amendment lifecycle).
- `check-knob-citation` — reds on a knob mentioned in a governed SPEC without its
  citation. **Not monotone.** `DRIFT_KIT_KNOWLEDGE_LOG` is named in the producer
  paragraph above and must carry its §Layout citation once that paragraph merges.
- `check-close-surfaces` — reds on a capture-tier declaration naming no
  `reclaim=`. **Monotone here and cleared by inspection**: the log's
  `close-surface:` declaration and its reclaim command are untouched by every
  delta.
- `kpi-incident-recurrence` — not a gate; named because delta 4 edits the bullet
  it cites, and its own text must still read true afterwards.

## Existing sections updated

- `drift-kit/SPEC.md` §The knowledge-friction loop, step 3 — the trend sentence
  is deleted and the reading rule replaces it; the "Detection is the loop;
  elimination is a tiering edit" clause survives and is re-read against the new
  neighbour (delta 1).
- `drift-kit/SPEC.md` §The knowledge-friction loop — the two refused
  alternatives and the declared absence of a capture floor, placed with the
  loop's other design refusals rather than at step 3, since they are refusals
  about the loop and not about the aggregate (delta 3).
- `drift-kit/SPEC.md` §The knowledge-friction loop, step 2 — the
  no-standing-instruction rule is stated as binding the **capture** side too, not
  only remediation; it is currently written as a triage rule and is cited as a
  general one (delta 3).
- `drift-kit/SPEC.md` §Bundled KPIs, the `kpi-knowledge-friction` bullet — the
  lower-bound clause gains its degenerate case, and the `kpi-incident-recurrence`
  bullet's citation of "exactly `kpi-knowledge-friction`'s structure" is re-read
  so it still names what it now points at (delta 4).
- `drift-kit/kpis/kpi-knowledge-friction.sh` — the emitter's own branch and its
  `# spec:` pointer, which must keep binding to the section that now owns the
  three-state contract (delta 2).
- `drift-kit`'s KPI behavioural coverage — the empty-log arm moves with delta 2
  or the emitter's new line is asserted by nothing (delta 2).
<!-- update-target-exempt: owned by no delta — a consistency re-read of a sibling entry's boundary claim, which this unit deliberately does not change -->
- `TASK-QUEUE.md`'s `kfric-obligation-residency` entry — no content change; its
  "**DISTINCT from `kfric-empty-log-ambiguity`**" paragraph is re-read at merge to
  confirm the boundary it draws still holds once the reader-side half has landed
  and the writer-side half has not.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. The retired **claim** is prose, so the grep is
      over the sentence's readers rather than over an identifier.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The three states are observed, not reasoned** — the emitter is run
      against an absent, an empty and a non-empty log at build, and its three
      lines read off the run.
